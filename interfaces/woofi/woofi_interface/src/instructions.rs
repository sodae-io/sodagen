use solana_pubkey::Pubkey;
use solana_cpi::{invoke, invoke_signed};
use solana_instruction::{AccountMeta, Instruction};
use solana_account_info::AccountInfo;
use solana_program_error::ProgramError;
use std::io::Read;
#[allow(unused_imports)]
use crate::*;
#[derive(Clone, Debug, PartialEq)]
pub enum WoofiProgramIx {
    CreateConfig,
    CreateWooracle(CreateWooracleIxArgs),
    SetOracleMaximumAge(SetOracleMaximumAgeIxArgs),
    SetStaleDuration(SetStaleDurationIxArgs),
    SetWooBound(SetWooBoundIxArgs),
    SetWooRange(SetWooRangeIxArgs),
    SetWooPrice(SetWooPriceIxArgs),
    SetWooCoeff(SetWooCoeffIxArgs),
    SetWooSpread(SetWooSpreadIxArgs),
    SetWooAdmin(SetWooAdminIxArgs),
    SetGuardianAdmin(SetGuardianAdminIxArgs),
    SetLendingManager(SetLendingManagerIxArgs),
    SetSuperchargerVaultWhitelist(SetSuperchargerVaultWhitelistIxArgs),
    SetWooState(SetWooStateIxArgs),
    GetPrice,
    CreatePool,
    CreateWooAmmPool,
    SetPoolAdmin(SetPoolAdminIxArgs),
    SetFeeAdmin(SetFeeAdminIxArgs),
    SetPauseRole(SetPauseRoleIxArgs),
    Pause,
    Unpause,
    SetPoolFeeRate(SetPoolFeeRateIxArgs),
    SetPoolMaxGamma(SetPoolMaxGammaIxArgs),
    SetPoolMaxNotionalSwap(SetPoolMaxNotionalSwapIxArgs),
    SetPoolCapBal(SetPoolCapBalIxArgs),
    SetPoolMinSwapAmount(SetPoolMinSwapAmountIxArgs),
    TryQuery(TryQueryIxArgs),
    Query(QueryIxArgs),
    Swap(SwapIxArgs),
    Deposit(DepositIxArgs),
    Withdraw(WithdrawIxArgs),
    ClaimFee,
    ClaimFeeAmount(ClaimFeeAmountIxArgs),
    IncaseTokenGotStuck(IncaseTokenGotStuckIxArgs),
    SetWooconfigNewAuthority,
    ClaimWooconfigAuthority,
    ClaimWooracleAuthority,
    ClaimWoopoolAuthority,
    ClaimWooammpoolAuthority,
    RepayByLendingManager(RepayByLendingManagerIxArgs),
}
impl WoofiProgramIx {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        if buf.starts_with(&CREATE_CONFIG_IX_DISCM) {
            return Ok(Self::CreateConfig);
        }
        if buf.starts_with(&CREATE_WOORACLE_IX_DISCM) {
            let mut reader = &buf[CREATE_WOORACLE_IX_DISCM.len()..];
            let maximum_age: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::CreateWooracle(CreateWooracleIxArgs {
                    maximum_age,
                }),
            );
        }
        if buf.starts_with(&SET_ORACLE_MAXIMUM_AGE_IX_DISCM) {
            let mut reader = &buf[SET_ORACLE_MAXIMUM_AGE_IX_DISCM.len()..];
            let maximum_age: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::SetOracleMaximumAge(SetOracleMaximumAgeIxArgs {
                    maximum_age,
                }),
            );
        }
        if buf.starts_with(&SET_STALE_DURATION_IX_DISCM) {
            let mut reader = &buf[SET_STALE_DURATION_IX_DISCM.len()..];
            let stale_duration: i64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::SetStaleDuration(SetStaleDurationIxArgs {
                    stale_duration,
                }),
            );
        }
        if buf.starts_with(&SET_WOO_BOUND_IX_DISCM) {
            let mut reader = &buf[SET_WOO_BOUND_IX_DISCM.len()..];
            let bound: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(Self::SetWooBound(SetWooBoundIxArgs { bound }));
        }
        if buf.starts_with(&SET_WOO_RANGE_IX_DISCM) {
            let mut reader = &buf[SET_WOO_RANGE_IX_DISCM.len()..];
            let range_min: u128 = crate::borsh_de_or_default(&mut reader)?;
            let range_max: u128 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::SetWooRange(SetWooRangeIxArgs {
                    range_min,
                    range_max,
                }),
            );
        }
        if buf.starts_with(&SET_WOO_PRICE_IX_DISCM) {
            let mut reader = &buf[SET_WOO_PRICE_IX_DISCM.len()..];
            let price: u128 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(Self::SetWooPrice(SetWooPriceIxArgs { price }));
        }
        if buf.starts_with(&SET_WOO_COEFF_IX_DISCM) {
            let mut reader = &buf[SET_WOO_COEFF_IX_DISCM.len()..];
            let coeff: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(Self::SetWooCoeff(SetWooCoeffIxArgs { coeff }));
        }
        if buf.starts_with(&SET_WOO_SPREAD_IX_DISCM) {
            let mut reader = &buf[SET_WOO_SPREAD_IX_DISCM.len()..];
            let spread: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(Self::SetWooSpread(SetWooSpreadIxArgs { spread }));
        }
        if buf.starts_with(&SET_WOO_ADMIN_IX_DISCM) {
            let mut reader = &buf[SET_WOO_ADMIN_IX_DISCM.len()..];
            let admin_authority: Vec<Pubkey> = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::SetWooAdmin(SetWooAdminIxArgs {
                    admin_authority,
                }),
            );
        }
        if buf.starts_with(&SET_GUARDIAN_ADMIN_IX_DISCM) {
            let mut reader = &buf[SET_GUARDIAN_ADMIN_IX_DISCM.len()..];
            let guardian_authority: Vec<Pubkey> = crate::borsh_de_or_default(
                &mut reader,
            )?;
            return Ok(
                Self::SetGuardianAdmin(SetGuardianAdminIxArgs {
                    guardian_authority,
                }),
            );
        }
        if buf.starts_with(&SET_LENDING_MANAGER_IX_DISCM) {
            let mut reader = &buf[SET_LENDING_MANAGER_IX_DISCM.len()..];
            let lending_managers: Vec<Pubkey> = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::SetLendingManager(SetLendingManagerIxArgs {
                    lending_managers,
                }),
            );
        }
        if buf.starts_with(&SET_SUPERCHARGER_VAULT_WHITELIST_IX_DISCM) {
            let mut reader = &buf[SET_SUPERCHARGER_VAULT_WHITELIST_IX_DISCM.len()..];
            let supercharger_vaults: Vec<Pubkey> = crate::borsh_de_or_default(
                &mut reader,
            )?;
            return Ok(
                Self::SetSuperchargerVaultWhitelist(SetSuperchargerVaultWhitelistIxArgs {
                    supercharger_vaults,
                }),
            );
        }
        if buf.starts_with(&SET_WOO_STATE_IX_DISCM) {
            let mut reader = &buf[SET_WOO_STATE_IX_DISCM.len()..];
            let price: u128 = crate::borsh_de_or_default(&mut reader)?;
            let coeff: u64 = crate::borsh_de_or_default(&mut reader)?;
            let spread: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::SetWooState(SetWooStateIxArgs {
                    price,
                    coeff,
                    spread,
                }),
            );
        }
        if buf.starts_with(&GET_PRICE_IX_DISCM) {
            return Ok(Self::GetPrice);
        }
        if buf.starts_with(&CREATE_POOL_IX_DISCM) {
            return Ok(Self::CreatePool);
        }
        if buf.starts_with(&CREATE_WOO_AMM_POOL_IX_DISCM) {
            return Ok(Self::CreateWooAmmPool);
        }
        if buf.starts_with(&SET_POOL_ADMIN_IX_DISCM) {
            let mut reader = &buf[SET_POOL_ADMIN_IX_DISCM.len()..];
            let admin_authority: Vec<Pubkey> = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::SetPoolAdmin(SetPoolAdminIxArgs {
                    admin_authority,
                }),
            );
        }
        if buf.starts_with(&SET_FEE_ADMIN_IX_DISCM) {
            let mut reader = &buf[SET_FEE_ADMIN_IX_DISCM.len()..];
            let fee_authority: Vec<Pubkey> = crate::borsh_de_or_default(&mut reader)?;
            return Ok(Self::SetFeeAdmin(SetFeeAdminIxArgs { fee_authority }));
        }
        if buf.starts_with(&SET_PAUSE_ROLE_IX_DISCM) {
            let mut reader = &buf[SET_PAUSE_ROLE_IX_DISCM.len()..];
            let pause_authority: Vec<Pubkey> = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::SetPauseRole(SetPauseRoleIxArgs {
                    pause_authority,
                }),
            );
        }
        if buf.starts_with(&PAUSE_IX_DISCM) {
            return Ok(Self::Pause);
        }
        if buf.starts_with(&UNPAUSE_IX_DISCM) {
            return Ok(Self::Unpause);
        }
        if buf.starts_with(&SET_POOL_FEE_RATE_IX_DISCM) {
            let mut reader = &buf[SET_POOL_FEE_RATE_IX_DISCM.len()..];
            let fee_rate: u16 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(Self::SetPoolFeeRate(SetPoolFeeRateIxArgs { fee_rate }));
        }
        if buf.starts_with(&SET_POOL_MAX_GAMMA_IX_DISCM) {
            let mut reader = &buf[SET_POOL_MAX_GAMMA_IX_DISCM.len()..];
            let max_gamma: u128 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(Self::SetPoolMaxGamma(SetPoolMaxGammaIxArgs { max_gamma }));
        }
        if buf.starts_with(&SET_POOL_MAX_NOTIONAL_SWAP_IX_DISCM) {
            let mut reader = &buf[SET_POOL_MAX_NOTIONAL_SWAP_IX_DISCM.len()..];
            let max_notional_swap: u128 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::SetPoolMaxNotionalSwap(SetPoolMaxNotionalSwapIxArgs {
                    max_notional_swap,
                }),
            );
        }
        if buf.starts_with(&SET_POOL_CAP_BAL_IX_DISCM) {
            let mut reader = &buf[SET_POOL_CAP_BAL_IX_DISCM.len()..];
            let cap_bal: u128 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(Self::SetPoolCapBal(SetPoolCapBalIxArgs { cap_bal }));
        }
        if buf.starts_with(&SET_POOL_MIN_SWAP_AMOUNT_IX_DISCM) {
            let mut reader = &buf[SET_POOL_MIN_SWAP_AMOUNT_IX_DISCM.len()..];
            let min_swap_amount: u128 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::SetPoolMinSwapAmount(SetPoolMinSwapAmountIxArgs {
                    min_swap_amount,
                }),
            );
        }
        if buf.starts_with(&TRY_QUERY_IX_DISCM) {
            let mut reader = &buf[TRY_QUERY_IX_DISCM.len()..];
            let from_amount: u128 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(Self::TryQuery(TryQueryIxArgs { from_amount }));
        }
        if buf.starts_with(&QUERY_IX_DISCM) {
            let mut reader = &buf[QUERY_IX_DISCM.len()..];
            let from_amount: u128 = crate::borsh_de_or_default(&mut reader)?;
            let min_to_amount: u128 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::Query(QueryIxArgs {
                    from_amount,
                    min_to_amount,
                }),
            );
        }
        if buf.starts_with(&SWAP_IX_DISCM) {
            let mut reader = &buf[SWAP_IX_DISCM.len()..];
            let from_amount: u128 = crate::borsh_de_or_default(&mut reader)?;
            let min_to_amount: u128 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::Swap(SwapIxArgs {
                    from_amount,
                    min_to_amount,
                }),
            );
        }
        if buf.starts_with(&DEPOSIT_IX_DISCM) {
            let mut reader = &buf[DEPOSIT_IX_DISCM.len()..];
            let amount: u128 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(Self::Deposit(DepositIxArgs { amount }));
        }
        if buf.starts_with(&WITHDRAW_IX_DISCM) {
            let mut reader = &buf[WITHDRAW_IX_DISCM.len()..];
            let amount: u128 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(Self::Withdraw(WithdrawIxArgs { amount }));
        }
        if buf.starts_with(&CLAIM_FEE_IX_DISCM) {
            return Ok(Self::ClaimFee);
        }
        if buf.starts_with(&CLAIM_FEE_AMOUNT_IX_DISCM) {
            let mut reader = &buf[CLAIM_FEE_AMOUNT_IX_DISCM.len()..];
            let claim_amount: u128 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::ClaimFeeAmount(ClaimFeeAmountIxArgs {
                    claim_amount,
                }),
            );
        }
        if buf.starts_with(&INCASE_TOKEN_GOT_STUCK_IX_DISCM) {
            let mut reader = &buf[INCASE_TOKEN_GOT_STUCK_IX_DISCM.len()..];
            let amount: u128 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::IncaseTokenGotStuck(IncaseTokenGotStuckIxArgs {
                    amount,
                }),
            );
        }
        if buf.starts_with(&SET_WOOCONFIG_NEW_AUTHORITY_IX_DISCM) {
            return Ok(Self::SetWooconfigNewAuthority);
        }
        if buf.starts_with(&CLAIM_WOOCONFIG_AUTHORITY_IX_DISCM) {
            return Ok(Self::ClaimWooconfigAuthority);
        }
        if buf.starts_with(&CLAIM_WOORACLE_AUTHORITY_IX_DISCM) {
            return Ok(Self::ClaimWooracleAuthority);
        }
        if buf.starts_with(&CLAIM_WOOPOOL_AUTHORITY_IX_DISCM) {
            return Ok(Self::ClaimWoopoolAuthority);
        }
        if buf.starts_with(&CLAIM_WOOAMMPOOL_AUTHORITY_IX_DISCM) {
            return Ok(Self::ClaimWooammpoolAuthority);
        }
        if buf.starts_with(&REPAY_BY_LENDING_MANAGER_IX_DISCM) {
            let mut reader = &buf[REPAY_BY_LENDING_MANAGER_IX_DISCM.len()..];
            let repay_amount: u128 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::RepayByLendingManager(RepayByLendingManagerIxArgs {
                    repay_amount,
                }),
            );
        }
        Err(std::io::Error::from(std::io::ErrorKind::InvalidData))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        match self {
            Self::CreateConfig => writer.write_all(&CREATE_CONFIG_IX_DISCM),
            Self::CreateWooracle(args) => {
                writer.write_all(&CREATE_WOORACLE_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.maximum_age, &mut writer)?;
                Ok(())
            }
            Self::SetOracleMaximumAge(args) => {
                writer.write_all(&SET_ORACLE_MAXIMUM_AGE_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.maximum_age, &mut writer)?;
                Ok(())
            }
            Self::SetStaleDuration(args) => {
                writer.write_all(&SET_STALE_DURATION_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.stale_duration, &mut writer)?;
                Ok(())
            }
            Self::SetWooBound(args) => {
                writer.write_all(&SET_WOO_BOUND_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.bound, &mut writer)?;
                Ok(())
            }
            Self::SetWooRange(args) => {
                writer.write_all(&SET_WOO_RANGE_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.range_min, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.range_max, &mut writer)?;
                Ok(())
            }
            Self::SetWooPrice(args) => {
                writer.write_all(&SET_WOO_PRICE_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.price, &mut writer)?;
                Ok(())
            }
            Self::SetWooCoeff(args) => {
                writer.write_all(&SET_WOO_COEFF_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.coeff, &mut writer)?;
                Ok(())
            }
            Self::SetWooSpread(args) => {
                writer.write_all(&SET_WOO_SPREAD_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.spread, &mut writer)?;
                Ok(())
            }
            Self::SetWooAdmin(args) => {
                writer.write_all(&SET_WOO_ADMIN_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.admin_authority, &mut writer)?;
                Ok(())
            }
            Self::SetGuardianAdmin(args) => {
                writer.write_all(&SET_GUARDIAN_ADMIN_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.guardian_authority, &mut writer)?;
                Ok(())
            }
            Self::SetLendingManager(args) => {
                writer.write_all(&SET_LENDING_MANAGER_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.lending_managers, &mut writer)?;
                Ok(())
            }
            Self::SetSuperchargerVaultWhitelist(args) => {
                writer.write_all(&SET_SUPERCHARGER_VAULT_WHITELIST_IX_DISCM)?;
                borsh::BorshSerialize::serialize(
                    &args.supercharger_vaults,
                    &mut writer,
                )?;
                Ok(())
            }
            Self::SetWooState(args) => {
                writer.write_all(&SET_WOO_STATE_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.price, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.coeff, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.spread, &mut writer)?;
                Ok(())
            }
            Self::GetPrice => writer.write_all(&GET_PRICE_IX_DISCM),
            Self::CreatePool => writer.write_all(&CREATE_POOL_IX_DISCM),
            Self::CreateWooAmmPool => writer.write_all(&CREATE_WOO_AMM_POOL_IX_DISCM),
            Self::SetPoolAdmin(args) => {
                writer.write_all(&SET_POOL_ADMIN_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.admin_authority, &mut writer)?;
                Ok(())
            }
            Self::SetFeeAdmin(args) => {
                writer.write_all(&SET_FEE_ADMIN_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.fee_authority, &mut writer)?;
                Ok(())
            }
            Self::SetPauseRole(args) => {
                writer.write_all(&SET_PAUSE_ROLE_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.pause_authority, &mut writer)?;
                Ok(())
            }
            Self::Pause => writer.write_all(&PAUSE_IX_DISCM),
            Self::Unpause => writer.write_all(&UNPAUSE_IX_DISCM),
            Self::SetPoolFeeRate(args) => {
                writer.write_all(&SET_POOL_FEE_RATE_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.fee_rate, &mut writer)?;
                Ok(())
            }
            Self::SetPoolMaxGamma(args) => {
                writer.write_all(&SET_POOL_MAX_GAMMA_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.max_gamma, &mut writer)?;
                Ok(())
            }
            Self::SetPoolMaxNotionalSwap(args) => {
                writer.write_all(&SET_POOL_MAX_NOTIONAL_SWAP_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.max_notional_swap, &mut writer)?;
                Ok(())
            }
            Self::SetPoolCapBal(args) => {
                writer.write_all(&SET_POOL_CAP_BAL_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.cap_bal, &mut writer)?;
                Ok(())
            }
            Self::SetPoolMinSwapAmount(args) => {
                writer.write_all(&SET_POOL_MIN_SWAP_AMOUNT_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.min_swap_amount, &mut writer)?;
                Ok(())
            }
            Self::TryQuery(args) => {
                writer.write_all(&TRY_QUERY_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.from_amount, &mut writer)?;
                Ok(())
            }
            Self::Query(args) => {
                writer.write_all(&QUERY_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.from_amount, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.min_to_amount, &mut writer)?;
                Ok(())
            }
            Self::Swap(args) => {
                writer.write_all(&SWAP_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.from_amount, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.min_to_amount, &mut writer)?;
                Ok(())
            }
            Self::Deposit(args) => {
                writer.write_all(&DEPOSIT_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.amount, &mut writer)?;
                Ok(())
            }
            Self::Withdraw(args) => {
                writer.write_all(&WITHDRAW_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.amount, &mut writer)?;
                Ok(())
            }
            Self::ClaimFee => writer.write_all(&CLAIM_FEE_IX_DISCM),
            Self::ClaimFeeAmount(args) => {
                writer.write_all(&CLAIM_FEE_AMOUNT_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.claim_amount, &mut writer)?;
                Ok(())
            }
            Self::IncaseTokenGotStuck(args) => {
                writer.write_all(&INCASE_TOKEN_GOT_STUCK_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.amount, &mut writer)?;
                Ok(())
            }
            Self::SetWooconfigNewAuthority => {
                writer.write_all(&SET_WOOCONFIG_NEW_AUTHORITY_IX_DISCM)
            }
            Self::ClaimWooconfigAuthority => {
                writer.write_all(&CLAIM_WOOCONFIG_AUTHORITY_IX_DISCM)
            }
            Self::ClaimWooracleAuthority => {
                writer.write_all(&CLAIM_WOORACLE_AUTHORITY_IX_DISCM)
            }
            Self::ClaimWoopoolAuthority => {
                writer.write_all(&CLAIM_WOOPOOL_AUTHORITY_IX_DISCM)
            }
            Self::ClaimWooammpoolAuthority => {
                writer.write_all(&CLAIM_WOOAMMPOOL_AUTHORITY_IX_DISCM)
            }
            Self::RepayByLendingManager(args) => {
                writer.write_all(&REPAY_BY_LENDING_MANAGER_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.repay_amount, &mut writer)?;
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
pub const CREATE_CONFIG_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct CreateConfigAccounts<'me, 'info> {
    pub authority: &'me AccountInfo<'info>,
    pub wooconfig: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CreateConfigKeys {
    pub authority: Pubkey,
    pub wooconfig: Pubkey,
    pub system_program: Pubkey,
}
impl From<CreateConfigAccounts<'_, '_>> for CreateConfigKeys {
    fn from(accounts: CreateConfigAccounts) -> Self {
        Self {
            authority: *accounts.authority.key,
            wooconfig: *accounts.wooconfig.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<CreateConfigKeys> for [AccountMeta; CREATE_CONFIG_IX_ACCOUNTS_LEN] {
    fn from(keys: CreateConfigKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.authority,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.wooconfig,
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
impl From<[Pubkey; CREATE_CONFIG_IX_ACCOUNTS_LEN]> for CreateConfigKeys {
    fn from(pubkeys: [Pubkey; CREATE_CONFIG_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            authority: pubkeys[0],
            wooconfig: pubkeys[1],
            system_program: pubkeys[2],
        }
    }
}
impl<'info> From<CreateConfigAccounts<'_, 'info>>
for [AccountInfo<'info>; CREATE_CONFIG_IX_ACCOUNTS_LEN] {
    fn from(accounts: CreateConfigAccounts<'_, 'info>) -> Self {
        [
            accounts.authority.clone(),
            accounts.wooconfig.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CREATE_CONFIG_IX_ACCOUNTS_LEN]>
for CreateConfigAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; CREATE_CONFIG_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            authority: &arr[0],
            wooconfig: &arr[1],
            system_program: &arr[2],
        }
    }
}
pub const CREATE_CONFIG_IX_DISCM: [u8; 8usize] = [201, 207, 243, 114, 75, 111, 47, 189];
#[derive(Clone, Debug, PartialEq)]
pub struct CreateConfigIxData;
impl CreateConfigIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CREATE_CONFIG_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CREATE_CONFIG_IX_DISCM)
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
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CREATE_CONFIG_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: CreateConfigIxData.try_to_vec()?,
    })
}
pub fn create_config_ix(keys: CreateConfigKeys) -> std::io::Result<Instruction> {
    create_config_ix_with_program_id(WOOFI_PROGRAM_ID, keys)
}
pub fn create_config_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CreateConfigAccounts<'_, '_>,
) -> ProgramResult {
    let keys: CreateConfigKeys = accounts.into();
    let ix = create_config_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn create_config_invoke(accounts: CreateConfigAccounts<'_, '_>) -> ProgramResult {
    create_config_invoke_with_program_id(WOOFI_PROGRAM_ID, accounts)
}
pub fn create_config_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CreateConfigAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CreateConfigKeys = accounts.into();
    let ix = create_config_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn create_config_invoke_signed(
    accounts: CreateConfigAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    create_config_invoke_signed_with_program_id(WOOFI_PROGRAM_ID, accounts, seeds)
}
pub fn create_config_verify_account_keys(
    accounts: CreateConfigAccounts<'_, '_>,
    keys: CreateConfigKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.authority.key, keys.authority),
        (*accounts.wooconfig.key, keys.wooconfig),
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
    for should_be_writable in [accounts.authority, accounts.wooconfig] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn create_config_verify_signer_privileges<'me, 'info>(
    accounts: CreateConfigAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority] {
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
pub const CREATE_WOORACLE_IX_ACCOUNTS_LEN: usize = 10;
#[derive(Copy, Clone, Debug)]
pub struct CreateWooracleAccounts<'me, 'info> {
    pub wooconfig: &'me AccountInfo<'info>,
    pub token_mint: &'me AccountInfo<'info>,
    pub wooracle: &'me AccountInfo<'info>,
    pub admin: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub feed_account: &'me AccountInfo<'info>,
    pub price_update: &'me AccountInfo<'info>,
    pub quote_token_mint: &'me AccountInfo<'info>,
    pub quote_feed_account: &'me AccountInfo<'info>,
    pub quote_price_update: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CreateWooracleKeys {
    pub wooconfig: Pubkey,
    pub token_mint: Pubkey,
    pub wooracle: Pubkey,
    pub admin: Pubkey,
    pub system_program: Pubkey,
    pub feed_account: Pubkey,
    pub price_update: Pubkey,
    pub quote_token_mint: Pubkey,
    pub quote_feed_account: Pubkey,
    pub quote_price_update: Pubkey,
}
impl From<CreateWooracleAccounts<'_, '_>> for CreateWooracleKeys {
    fn from(accounts: CreateWooracleAccounts) -> Self {
        Self {
            wooconfig: *accounts.wooconfig.key,
            token_mint: *accounts.token_mint.key,
            wooracle: *accounts.wooracle.key,
            admin: *accounts.admin.key,
            system_program: *accounts.system_program.key,
            feed_account: *accounts.feed_account.key,
            price_update: *accounts.price_update.key,
            quote_token_mint: *accounts.quote_token_mint.key,
            quote_feed_account: *accounts.quote_feed_account.key,
            quote_price_update: *accounts.quote_price_update.key,
        }
    }
}
impl From<CreateWooracleKeys> for [AccountMeta; CREATE_WOORACLE_IX_ACCOUNTS_LEN] {
    fn from(keys: CreateWooracleKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.wooconfig,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.wooracle,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.admin,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.system_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.feed_account,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.price_update,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.quote_token_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.quote_feed_account,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.quote_price_update,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; CREATE_WOORACLE_IX_ACCOUNTS_LEN]> for CreateWooracleKeys {
    fn from(pubkeys: [Pubkey; CREATE_WOORACLE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            wooconfig: pubkeys[0],
            token_mint: pubkeys[1],
            wooracle: pubkeys[2],
            admin: pubkeys[3],
            system_program: pubkeys[4],
            feed_account: pubkeys[5],
            price_update: pubkeys[6],
            quote_token_mint: pubkeys[7],
            quote_feed_account: pubkeys[8],
            quote_price_update: pubkeys[9],
        }
    }
}
impl<'info> From<CreateWooracleAccounts<'_, 'info>>
for [AccountInfo<'info>; CREATE_WOORACLE_IX_ACCOUNTS_LEN] {
    fn from(accounts: CreateWooracleAccounts<'_, 'info>) -> Self {
        [
            accounts.wooconfig.clone(),
            accounts.token_mint.clone(),
            accounts.wooracle.clone(),
            accounts.admin.clone(),
            accounts.system_program.clone(),
            accounts.feed_account.clone(),
            accounts.price_update.clone(),
            accounts.quote_token_mint.clone(),
            accounts.quote_feed_account.clone(),
            accounts.quote_price_update.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CREATE_WOORACLE_IX_ACCOUNTS_LEN]>
for CreateWooracleAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; CREATE_WOORACLE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            wooconfig: &arr[0],
            token_mint: &arr[1],
            wooracle: &arr[2],
            admin: &arr[3],
            system_program: &arr[4],
            feed_account: &arr[5],
            price_update: &arr[6],
            quote_token_mint: &arr[7],
            quote_feed_account: &arr[8],
            quote_price_update: &arr[9],
        }
    }
}
pub const CREATE_WOORACLE_IX_DISCM: [u8; 8usize] = [73, 65, 167, 4, 144, 141, 147, 32];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CreateWooracleIxArgs {
    pub maximum_age: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct CreateWooracleIxData(pub CreateWooracleIxArgs);
impl From<CreateWooracleIxArgs> for CreateWooracleIxData {
    fn from(args: CreateWooracleIxArgs) -> Self {
        Self(args)
    }
}
impl CreateWooracleIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CREATE_WOORACLE_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let maximum_age: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(CreateWooracleIxArgs {
                maximum_age,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CREATE_WOORACLE_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.maximum_age, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn create_wooracle_ix_with_program_id(
    program_id: Pubkey,
    keys: CreateWooracleKeys,
    args: CreateWooracleIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CREATE_WOORACLE_IX_ACCOUNTS_LEN] = keys.into();
    let data: CreateWooracleIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn create_wooracle_ix(
    keys: CreateWooracleKeys,
    args: CreateWooracleIxArgs,
) -> std::io::Result<Instruction> {
    create_wooracle_ix_with_program_id(WOOFI_PROGRAM_ID, keys, args)
}
pub fn create_wooracle_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CreateWooracleAccounts<'_, '_>,
    args: CreateWooracleIxArgs,
) -> ProgramResult {
    let keys: CreateWooracleKeys = accounts.into();
    let ix = create_wooracle_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn create_wooracle_invoke(
    accounts: CreateWooracleAccounts<'_, '_>,
    args: CreateWooracleIxArgs,
) -> ProgramResult {
    create_wooracle_invoke_with_program_id(WOOFI_PROGRAM_ID, accounts, args)
}
pub fn create_wooracle_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CreateWooracleAccounts<'_, '_>,
    args: CreateWooracleIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CreateWooracleKeys = accounts.into();
    let ix = create_wooracle_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn create_wooracle_invoke_signed(
    accounts: CreateWooracleAccounts<'_, '_>,
    args: CreateWooracleIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    create_wooracle_invoke_signed_with_program_id(
        WOOFI_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn create_wooracle_verify_account_keys(
    accounts: CreateWooracleAccounts<'_, '_>,
    keys: CreateWooracleKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.wooconfig.key, keys.wooconfig),
        (*accounts.token_mint.key, keys.token_mint),
        (*accounts.wooracle.key, keys.wooracle),
        (*accounts.admin.key, keys.admin),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.feed_account.key, keys.feed_account),
        (*accounts.price_update.key, keys.price_update),
        (*accounts.quote_token_mint.key, keys.quote_token_mint),
        (*accounts.quote_feed_account.key, keys.quote_feed_account),
        (*accounts.quote_price_update.key, keys.quote_price_update),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn create_wooracle_verify_writable_privileges<'me, 'info>(
    accounts: CreateWooracleAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.wooracle, accounts.admin] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn create_wooracle_verify_signer_privileges<'me, 'info>(
    accounts: CreateWooracleAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn create_wooracle_verify_account_privileges<'me, 'info>(
    accounts: CreateWooracleAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    create_wooracle_verify_writable_privileges(accounts)?;
    create_wooracle_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SET_ORACLE_MAXIMUM_AGE_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct SetOracleMaximumAgeAccounts<'me, 'info> {
    pub wooconfig: &'me AccountInfo<'info>,
    pub wooracle: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SetOracleMaximumAgeKeys {
    pub wooconfig: Pubkey,
    pub wooracle: Pubkey,
    pub authority: Pubkey,
}
impl From<SetOracleMaximumAgeAccounts<'_, '_>> for SetOracleMaximumAgeKeys {
    fn from(accounts: SetOracleMaximumAgeAccounts) -> Self {
        Self {
            wooconfig: *accounts.wooconfig.key,
            wooracle: *accounts.wooracle.key,
            authority: *accounts.authority.key,
        }
    }
}
impl From<SetOracleMaximumAgeKeys>
for [AccountMeta; SET_ORACLE_MAXIMUM_AGE_IX_ACCOUNTS_LEN] {
    fn from(keys: SetOracleMaximumAgeKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.wooconfig,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.wooracle,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.authority,
                is_signer: true,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; SET_ORACLE_MAXIMUM_AGE_IX_ACCOUNTS_LEN]> for SetOracleMaximumAgeKeys {
    fn from(pubkeys: [Pubkey; SET_ORACLE_MAXIMUM_AGE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            wooconfig: pubkeys[0],
            wooracle: pubkeys[1],
            authority: pubkeys[2],
        }
    }
}
impl<'info> From<SetOracleMaximumAgeAccounts<'_, 'info>>
for [AccountInfo<'info>; SET_ORACLE_MAXIMUM_AGE_IX_ACCOUNTS_LEN] {
    fn from(accounts: SetOracleMaximumAgeAccounts<'_, 'info>) -> Self {
        [
            accounts.wooconfig.clone(),
            accounts.wooracle.clone(),
            accounts.authority.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SET_ORACLE_MAXIMUM_AGE_IX_ACCOUNTS_LEN]>
for SetOracleMaximumAgeAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; SET_ORACLE_MAXIMUM_AGE_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            wooconfig: &arr[0],
            wooracle: &arr[1],
            authority: &arr[2],
        }
    }
}
pub const SET_ORACLE_MAXIMUM_AGE_IX_DISCM: [u8; 8usize] = [
    65, 10, 188, 33, 5, 143, 54, 161,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SetOracleMaximumAgeIxArgs {
    pub maximum_age: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct SetOracleMaximumAgeIxData(pub SetOracleMaximumAgeIxArgs);
impl From<SetOracleMaximumAgeIxArgs> for SetOracleMaximumAgeIxData {
    fn from(args: SetOracleMaximumAgeIxArgs) -> Self {
        Self(args)
    }
}
impl SetOracleMaximumAgeIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SET_ORACLE_MAXIMUM_AGE_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let maximum_age: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(SetOracleMaximumAgeIxArgs {
                maximum_age,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SET_ORACLE_MAXIMUM_AGE_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.maximum_age, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn set_oracle_maximum_age_ix_with_program_id(
    program_id: Pubkey,
    keys: SetOracleMaximumAgeKeys,
    args: SetOracleMaximumAgeIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SET_ORACLE_MAXIMUM_AGE_IX_ACCOUNTS_LEN] = keys.into();
    let data: SetOracleMaximumAgeIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn set_oracle_maximum_age_ix(
    keys: SetOracleMaximumAgeKeys,
    args: SetOracleMaximumAgeIxArgs,
) -> std::io::Result<Instruction> {
    set_oracle_maximum_age_ix_with_program_id(WOOFI_PROGRAM_ID, keys, args)
}
pub fn set_oracle_maximum_age_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SetOracleMaximumAgeAccounts<'_, '_>,
    args: SetOracleMaximumAgeIxArgs,
) -> ProgramResult {
    let keys: SetOracleMaximumAgeKeys = accounts.into();
    let ix = set_oracle_maximum_age_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn set_oracle_maximum_age_invoke(
    accounts: SetOracleMaximumAgeAccounts<'_, '_>,
    args: SetOracleMaximumAgeIxArgs,
) -> ProgramResult {
    set_oracle_maximum_age_invoke_with_program_id(WOOFI_PROGRAM_ID, accounts, args)
}
pub fn set_oracle_maximum_age_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SetOracleMaximumAgeAccounts<'_, '_>,
    args: SetOracleMaximumAgeIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SetOracleMaximumAgeKeys = accounts.into();
    let ix = set_oracle_maximum_age_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn set_oracle_maximum_age_invoke_signed(
    accounts: SetOracleMaximumAgeAccounts<'_, '_>,
    args: SetOracleMaximumAgeIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    set_oracle_maximum_age_invoke_signed_with_program_id(
        WOOFI_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn set_oracle_maximum_age_verify_account_keys(
    accounts: SetOracleMaximumAgeAccounts<'_, '_>,
    keys: SetOracleMaximumAgeKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.wooconfig.key, keys.wooconfig),
        (*accounts.wooracle.key, keys.wooracle),
        (*accounts.authority.key, keys.authority),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn set_oracle_maximum_age_verify_writable_privileges<'me, 'info>(
    accounts: SetOracleMaximumAgeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.wooracle] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn set_oracle_maximum_age_verify_signer_privileges<'me, 'info>(
    accounts: SetOracleMaximumAgeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn set_oracle_maximum_age_verify_account_privileges<'me, 'info>(
    accounts: SetOracleMaximumAgeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    set_oracle_maximum_age_verify_writable_privileges(accounts)?;
    set_oracle_maximum_age_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SET_STALE_DURATION_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct SetStaleDurationAccounts<'me, 'info> {
    pub wooconfig: &'me AccountInfo<'info>,
    pub wooracle: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SetStaleDurationKeys {
    pub wooconfig: Pubkey,
    pub wooracle: Pubkey,
    pub authority: Pubkey,
}
impl From<SetStaleDurationAccounts<'_, '_>> for SetStaleDurationKeys {
    fn from(accounts: SetStaleDurationAccounts) -> Self {
        Self {
            wooconfig: *accounts.wooconfig.key,
            wooracle: *accounts.wooracle.key,
            authority: *accounts.authority.key,
        }
    }
}
impl From<SetStaleDurationKeys> for [AccountMeta; SET_STALE_DURATION_IX_ACCOUNTS_LEN] {
    fn from(keys: SetStaleDurationKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.wooconfig,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.wooracle,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.authority,
                is_signer: true,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; SET_STALE_DURATION_IX_ACCOUNTS_LEN]> for SetStaleDurationKeys {
    fn from(pubkeys: [Pubkey; SET_STALE_DURATION_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            wooconfig: pubkeys[0],
            wooracle: pubkeys[1],
            authority: pubkeys[2],
        }
    }
}
impl<'info> From<SetStaleDurationAccounts<'_, 'info>>
for [AccountInfo<'info>; SET_STALE_DURATION_IX_ACCOUNTS_LEN] {
    fn from(accounts: SetStaleDurationAccounts<'_, 'info>) -> Self {
        [
            accounts.wooconfig.clone(),
            accounts.wooracle.clone(),
            accounts.authority.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SET_STALE_DURATION_IX_ACCOUNTS_LEN]>
for SetStaleDurationAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; SET_STALE_DURATION_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            wooconfig: &arr[0],
            wooracle: &arr[1],
            authority: &arr[2],
        }
    }
}
pub const SET_STALE_DURATION_IX_DISCM: [u8; 8usize] = [
    108, 101, 231, 96, 83, 110, 252, 46,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SetStaleDurationIxArgs {
    pub stale_duration: i64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct SetStaleDurationIxData(pub SetStaleDurationIxArgs);
impl From<SetStaleDurationIxArgs> for SetStaleDurationIxData {
    fn from(args: SetStaleDurationIxArgs) -> Self {
        Self(args)
    }
}
impl SetStaleDurationIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SET_STALE_DURATION_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let stale_duration: i64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(SetStaleDurationIxArgs {
                stale_duration,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SET_STALE_DURATION_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.stale_duration, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn set_stale_duration_ix_with_program_id(
    program_id: Pubkey,
    keys: SetStaleDurationKeys,
    args: SetStaleDurationIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SET_STALE_DURATION_IX_ACCOUNTS_LEN] = keys.into();
    let data: SetStaleDurationIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn set_stale_duration_ix(
    keys: SetStaleDurationKeys,
    args: SetStaleDurationIxArgs,
) -> std::io::Result<Instruction> {
    set_stale_duration_ix_with_program_id(WOOFI_PROGRAM_ID, keys, args)
}
pub fn set_stale_duration_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SetStaleDurationAccounts<'_, '_>,
    args: SetStaleDurationIxArgs,
) -> ProgramResult {
    let keys: SetStaleDurationKeys = accounts.into();
    let ix = set_stale_duration_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn set_stale_duration_invoke(
    accounts: SetStaleDurationAccounts<'_, '_>,
    args: SetStaleDurationIxArgs,
) -> ProgramResult {
    set_stale_duration_invoke_with_program_id(WOOFI_PROGRAM_ID, accounts, args)
}
pub fn set_stale_duration_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SetStaleDurationAccounts<'_, '_>,
    args: SetStaleDurationIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SetStaleDurationKeys = accounts.into();
    let ix = set_stale_duration_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn set_stale_duration_invoke_signed(
    accounts: SetStaleDurationAccounts<'_, '_>,
    args: SetStaleDurationIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    set_stale_duration_invoke_signed_with_program_id(
        WOOFI_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn set_stale_duration_verify_account_keys(
    accounts: SetStaleDurationAccounts<'_, '_>,
    keys: SetStaleDurationKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.wooconfig.key, keys.wooconfig),
        (*accounts.wooracle.key, keys.wooracle),
        (*accounts.authority.key, keys.authority),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn set_stale_duration_verify_writable_privileges<'me, 'info>(
    accounts: SetStaleDurationAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.wooracle] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn set_stale_duration_verify_signer_privileges<'me, 'info>(
    accounts: SetStaleDurationAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn set_stale_duration_verify_account_privileges<'me, 'info>(
    accounts: SetStaleDurationAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    set_stale_duration_verify_writable_privileges(accounts)?;
    set_stale_duration_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SET_WOO_BOUND_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct SetWooBoundAccounts<'me, 'info> {
    pub wooconfig: &'me AccountInfo<'info>,
    pub wooracle: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SetWooBoundKeys {
    pub wooconfig: Pubkey,
    pub wooracle: Pubkey,
    pub authority: Pubkey,
}
impl From<SetWooBoundAccounts<'_, '_>> for SetWooBoundKeys {
    fn from(accounts: SetWooBoundAccounts) -> Self {
        Self {
            wooconfig: *accounts.wooconfig.key,
            wooracle: *accounts.wooracle.key,
            authority: *accounts.authority.key,
        }
    }
}
impl From<SetWooBoundKeys> for [AccountMeta; SET_WOO_BOUND_IX_ACCOUNTS_LEN] {
    fn from(keys: SetWooBoundKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.wooconfig,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.wooracle,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.authority,
                is_signer: true,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; SET_WOO_BOUND_IX_ACCOUNTS_LEN]> for SetWooBoundKeys {
    fn from(pubkeys: [Pubkey; SET_WOO_BOUND_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            wooconfig: pubkeys[0],
            wooracle: pubkeys[1],
            authority: pubkeys[2],
        }
    }
}
impl<'info> From<SetWooBoundAccounts<'_, 'info>>
for [AccountInfo<'info>; SET_WOO_BOUND_IX_ACCOUNTS_LEN] {
    fn from(accounts: SetWooBoundAccounts<'_, 'info>) -> Self {
        [
            accounts.wooconfig.clone(),
            accounts.wooracle.clone(),
            accounts.authority.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SET_WOO_BOUND_IX_ACCOUNTS_LEN]>
for SetWooBoundAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; SET_WOO_BOUND_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            wooconfig: &arr[0],
            wooracle: &arr[1],
            authority: &arr[2],
        }
    }
}
pub const SET_WOO_BOUND_IX_DISCM: [u8; 8usize] = [
    250, 175, 192, 204, 161, 134, 105, 137,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SetWooBoundIxArgs {
    pub bound: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct SetWooBoundIxData(pub SetWooBoundIxArgs);
impl From<SetWooBoundIxArgs> for SetWooBoundIxData {
    fn from(args: SetWooBoundIxArgs) -> Self {
        Self(args)
    }
}
impl SetWooBoundIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SET_WOO_BOUND_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let bound: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(Self(SetWooBoundIxArgs { bound }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SET_WOO_BOUND_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.bound, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn set_woo_bound_ix_with_program_id(
    program_id: Pubkey,
    keys: SetWooBoundKeys,
    args: SetWooBoundIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SET_WOO_BOUND_IX_ACCOUNTS_LEN] = keys.into();
    let data: SetWooBoundIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn set_woo_bound_ix(
    keys: SetWooBoundKeys,
    args: SetWooBoundIxArgs,
) -> std::io::Result<Instruction> {
    set_woo_bound_ix_with_program_id(WOOFI_PROGRAM_ID, keys, args)
}
pub fn set_woo_bound_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SetWooBoundAccounts<'_, '_>,
    args: SetWooBoundIxArgs,
) -> ProgramResult {
    let keys: SetWooBoundKeys = accounts.into();
    let ix = set_woo_bound_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn set_woo_bound_invoke(
    accounts: SetWooBoundAccounts<'_, '_>,
    args: SetWooBoundIxArgs,
) -> ProgramResult {
    set_woo_bound_invoke_with_program_id(WOOFI_PROGRAM_ID, accounts, args)
}
pub fn set_woo_bound_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SetWooBoundAccounts<'_, '_>,
    args: SetWooBoundIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SetWooBoundKeys = accounts.into();
    let ix = set_woo_bound_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn set_woo_bound_invoke_signed(
    accounts: SetWooBoundAccounts<'_, '_>,
    args: SetWooBoundIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    set_woo_bound_invoke_signed_with_program_id(WOOFI_PROGRAM_ID, accounts, args, seeds)
}
pub fn set_woo_bound_verify_account_keys(
    accounts: SetWooBoundAccounts<'_, '_>,
    keys: SetWooBoundKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.wooconfig.key, keys.wooconfig),
        (*accounts.wooracle.key, keys.wooracle),
        (*accounts.authority.key, keys.authority),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn set_woo_bound_verify_writable_privileges<'me, 'info>(
    accounts: SetWooBoundAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.wooracle] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn set_woo_bound_verify_signer_privileges<'me, 'info>(
    accounts: SetWooBoundAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn set_woo_bound_verify_account_privileges<'me, 'info>(
    accounts: SetWooBoundAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    set_woo_bound_verify_writable_privileges(accounts)?;
    set_woo_bound_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SET_WOO_RANGE_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct SetWooRangeAccounts<'me, 'info> {
    pub wooconfig: &'me AccountInfo<'info>,
    pub wooracle: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SetWooRangeKeys {
    pub wooconfig: Pubkey,
    pub wooracle: Pubkey,
    pub authority: Pubkey,
}
impl From<SetWooRangeAccounts<'_, '_>> for SetWooRangeKeys {
    fn from(accounts: SetWooRangeAccounts) -> Self {
        Self {
            wooconfig: *accounts.wooconfig.key,
            wooracle: *accounts.wooracle.key,
            authority: *accounts.authority.key,
        }
    }
}
impl From<SetWooRangeKeys> for [AccountMeta; SET_WOO_RANGE_IX_ACCOUNTS_LEN] {
    fn from(keys: SetWooRangeKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.wooconfig,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.wooracle,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.authority,
                is_signer: true,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; SET_WOO_RANGE_IX_ACCOUNTS_LEN]> for SetWooRangeKeys {
    fn from(pubkeys: [Pubkey; SET_WOO_RANGE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            wooconfig: pubkeys[0],
            wooracle: pubkeys[1],
            authority: pubkeys[2],
        }
    }
}
impl<'info> From<SetWooRangeAccounts<'_, 'info>>
for [AccountInfo<'info>; SET_WOO_RANGE_IX_ACCOUNTS_LEN] {
    fn from(accounts: SetWooRangeAccounts<'_, 'info>) -> Self {
        [
            accounts.wooconfig.clone(),
            accounts.wooracle.clone(),
            accounts.authority.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SET_WOO_RANGE_IX_ACCOUNTS_LEN]>
for SetWooRangeAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; SET_WOO_RANGE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            wooconfig: &arr[0],
            wooracle: &arr[1],
            authority: &arr[2],
        }
    }
}
pub const SET_WOO_RANGE_IX_DISCM: [u8; 8usize] = [53, 126, 84, 226, 73, 140, 138, 19];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SetWooRangeIxArgs {
    pub range_min: u128,
    pub range_max: u128,
}
#[derive(Clone, Debug, PartialEq)]
pub struct SetWooRangeIxData(pub SetWooRangeIxArgs);
impl From<SetWooRangeIxArgs> for SetWooRangeIxData {
    fn from(args: SetWooRangeIxArgs) -> Self {
        Self(args)
    }
}
impl SetWooRangeIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SET_WOO_RANGE_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let range_min: u128 = crate::borsh_de_or_default(&mut reader)?;
        let range_max: u128 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(SetWooRangeIxArgs {
                range_min,
                range_max,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SET_WOO_RANGE_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.range_min, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.range_max, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn set_woo_range_ix_with_program_id(
    program_id: Pubkey,
    keys: SetWooRangeKeys,
    args: SetWooRangeIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SET_WOO_RANGE_IX_ACCOUNTS_LEN] = keys.into();
    let data: SetWooRangeIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn set_woo_range_ix(
    keys: SetWooRangeKeys,
    args: SetWooRangeIxArgs,
) -> std::io::Result<Instruction> {
    set_woo_range_ix_with_program_id(WOOFI_PROGRAM_ID, keys, args)
}
pub fn set_woo_range_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SetWooRangeAccounts<'_, '_>,
    args: SetWooRangeIxArgs,
) -> ProgramResult {
    let keys: SetWooRangeKeys = accounts.into();
    let ix = set_woo_range_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn set_woo_range_invoke(
    accounts: SetWooRangeAccounts<'_, '_>,
    args: SetWooRangeIxArgs,
) -> ProgramResult {
    set_woo_range_invoke_with_program_id(WOOFI_PROGRAM_ID, accounts, args)
}
pub fn set_woo_range_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SetWooRangeAccounts<'_, '_>,
    args: SetWooRangeIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SetWooRangeKeys = accounts.into();
    let ix = set_woo_range_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn set_woo_range_invoke_signed(
    accounts: SetWooRangeAccounts<'_, '_>,
    args: SetWooRangeIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    set_woo_range_invoke_signed_with_program_id(WOOFI_PROGRAM_ID, accounts, args, seeds)
}
pub fn set_woo_range_verify_account_keys(
    accounts: SetWooRangeAccounts<'_, '_>,
    keys: SetWooRangeKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.wooconfig.key, keys.wooconfig),
        (*accounts.wooracle.key, keys.wooracle),
        (*accounts.authority.key, keys.authority),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn set_woo_range_verify_writable_privileges<'me, 'info>(
    accounts: SetWooRangeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.wooracle] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn set_woo_range_verify_signer_privileges<'me, 'info>(
    accounts: SetWooRangeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn set_woo_range_verify_account_privileges<'me, 'info>(
    accounts: SetWooRangeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    set_woo_range_verify_writable_privileges(accounts)?;
    set_woo_range_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SET_WOO_PRICE_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct SetWooPriceAccounts<'me, 'info> {
    pub wooconfig: &'me AccountInfo<'info>,
    pub wooracle: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SetWooPriceKeys {
    pub wooconfig: Pubkey,
    pub wooracle: Pubkey,
    pub authority: Pubkey,
}
impl From<SetWooPriceAccounts<'_, '_>> for SetWooPriceKeys {
    fn from(accounts: SetWooPriceAccounts) -> Self {
        Self {
            wooconfig: *accounts.wooconfig.key,
            wooracle: *accounts.wooracle.key,
            authority: *accounts.authority.key,
        }
    }
}
impl From<SetWooPriceKeys> for [AccountMeta; SET_WOO_PRICE_IX_ACCOUNTS_LEN] {
    fn from(keys: SetWooPriceKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.wooconfig,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.wooracle,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.authority,
                is_signer: true,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; SET_WOO_PRICE_IX_ACCOUNTS_LEN]> for SetWooPriceKeys {
    fn from(pubkeys: [Pubkey; SET_WOO_PRICE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            wooconfig: pubkeys[0],
            wooracle: pubkeys[1],
            authority: pubkeys[2],
        }
    }
}
impl<'info> From<SetWooPriceAccounts<'_, 'info>>
for [AccountInfo<'info>; SET_WOO_PRICE_IX_ACCOUNTS_LEN] {
    fn from(accounts: SetWooPriceAccounts<'_, 'info>) -> Self {
        [
            accounts.wooconfig.clone(),
            accounts.wooracle.clone(),
            accounts.authority.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SET_WOO_PRICE_IX_ACCOUNTS_LEN]>
for SetWooPriceAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; SET_WOO_PRICE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            wooconfig: &arr[0],
            wooracle: &arr[1],
            authority: &arr[2],
        }
    }
}
pub const SET_WOO_PRICE_IX_DISCM: [u8; 8usize] = [103, 138, 92, 57, 66, 194, 40, 23];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SetWooPriceIxArgs {
    pub price: u128,
}
#[derive(Clone, Debug, PartialEq)]
pub struct SetWooPriceIxData(pub SetWooPriceIxArgs);
impl From<SetWooPriceIxArgs> for SetWooPriceIxData {
    fn from(args: SetWooPriceIxArgs) -> Self {
        Self(args)
    }
}
impl SetWooPriceIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SET_WOO_PRICE_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let price: u128 = crate::borsh_de_or_default(&mut reader)?;
        Ok(Self(SetWooPriceIxArgs { price }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SET_WOO_PRICE_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.price, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn set_woo_price_ix_with_program_id(
    program_id: Pubkey,
    keys: SetWooPriceKeys,
    args: SetWooPriceIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SET_WOO_PRICE_IX_ACCOUNTS_LEN] = keys.into();
    let data: SetWooPriceIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn set_woo_price_ix(
    keys: SetWooPriceKeys,
    args: SetWooPriceIxArgs,
) -> std::io::Result<Instruction> {
    set_woo_price_ix_with_program_id(WOOFI_PROGRAM_ID, keys, args)
}
pub fn set_woo_price_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SetWooPriceAccounts<'_, '_>,
    args: SetWooPriceIxArgs,
) -> ProgramResult {
    let keys: SetWooPriceKeys = accounts.into();
    let ix = set_woo_price_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn set_woo_price_invoke(
    accounts: SetWooPriceAccounts<'_, '_>,
    args: SetWooPriceIxArgs,
) -> ProgramResult {
    set_woo_price_invoke_with_program_id(WOOFI_PROGRAM_ID, accounts, args)
}
pub fn set_woo_price_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SetWooPriceAccounts<'_, '_>,
    args: SetWooPriceIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SetWooPriceKeys = accounts.into();
    let ix = set_woo_price_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn set_woo_price_invoke_signed(
    accounts: SetWooPriceAccounts<'_, '_>,
    args: SetWooPriceIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    set_woo_price_invoke_signed_with_program_id(WOOFI_PROGRAM_ID, accounts, args, seeds)
}
pub fn set_woo_price_verify_account_keys(
    accounts: SetWooPriceAccounts<'_, '_>,
    keys: SetWooPriceKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.wooconfig.key, keys.wooconfig),
        (*accounts.wooracle.key, keys.wooracle),
        (*accounts.authority.key, keys.authority),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn set_woo_price_verify_writable_privileges<'me, 'info>(
    accounts: SetWooPriceAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.wooracle] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn set_woo_price_verify_signer_privileges<'me, 'info>(
    accounts: SetWooPriceAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn set_woo_price_verify_account_privileges<'me, 'info>(
    accounts: SetWooPriceAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    set_woo_price_verify_writable_privileges(accounts)?;
    set_woo_price_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SET_WOO_COEFF_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct SetWooCoeffAccounts<'me, 'info> {
    pub wooconfig: &'me AccountInfo<'info>,
    pub wooracle: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SetWooCoeffKeys {
    pub wooconfig: Pubkey,
    pub wooracle: Pubkey,
    pub authority: Pubkey,
}
impl From<SetWooCoeffAccounts<'_, '_>> for SetWooCoeffKeys {
    fn from(accounts: SetWooCoeffAccounts) -> Self {
        Self {
            wooconfig: *accounts.wooconfig.key,
            wooracle: *accounts.wooracle.key,
            authority: *accounts.authority.key,
        }
    }
}
impl From<SetWooCoeffKeys> for [AccountMeta; SET_WOO_COEFF_IX_ACCOUNTS_LEN] {
    fn from(keys: SetWooCoeffKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.wooconfig,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.wooracle,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.authority,
                is_signer: true,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; SET_WOO_COEFF_IX_ACCOUNTS_LEN]> for SetWooCoeffKeys {
    fn from(pubkeys: [Pubkey; SET_WOO_COEFF_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            wooconfig: pubkeys[0],
            wooracle: pubkeys[1],
            authority: pubkeys[2],
        }
    }
}
impl<'info> From<SetWooCoeffAccounts<'_, 'info>>
for [AccountInfo<'info>; SET_WOO_COEFF_IX_ACCOUNTS_LEN] {
    fn from(accounts: SetWooCoeffAccounts<'_, 'info>) -> Self {
        [
            accounts.wooconfig.clone(),
            accounts.wooracle.clone(),
            accounts.authority.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SET_WOO_COEFF_IX_ACCOUNTS_LEN]>
for SetWooCoeffAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; SET_WOO_COEFF_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            wooconfig: &arr[0],
            wooracle: &arr[1],
            authority: &arr[2],
        }
    }
}
pub const SET_WOO_COEFF_IX_DISCM: [u8; 8usize] = [122, 0, 214, 59, 63, 158, 219, 0];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SetWooCoeffIxArgs {
    pub coeff: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct SetWooCoeffIxData(pub SetWooCoeffIxArgs);
impl From<SetWooCoeffIxArgs> for SetWooCoeffIxData {
    fn from(args: SetWooCoeffIxArgs) -> Self {
        Self(args)
    }
}
impl SetWooCoeffIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SET_WOO_COEFF_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let coeff: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(Self(SetWooCoeffIxArgs { coeff }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SET_WOO_COEFF_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.coeff, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn set_woo_coeff_ix_with_program_id(
    program_id: Pubkey,
    keys: SetWooCoeffKeys,
    args: SetWooCoeffIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SET_WOO_COEFF_IX_ACCOUNTS_LEN] = keys.into();
    let data: SetWooCoeffIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn set_woo_coeff_ix(
    keys: SetWooCoeffKeys,
    args: SetWooCoeffIxArgs,
) -> std::io::Result<Instruction> {
    set_woo_coeff_ix_with_program_id(WOOFI_PROGRAM_ID, keys, args)
}
pub fn set_woo_coeff_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SetWooCoeffAccounts<'_, '_>,
    args: SetWooCoeffIxArgs,
) -> ProgramResult {
    let keys: SetWooCoeffKeys = accounts.into();
    let ix = set_woo_coeff_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn set_woo_coeff_invoke(
    accounts: SetWooCoeffAccounts<'_, '_>,
    args: SetWooCoeffIxArgs,
) -> ProgramResult {
    set_woo_coeff_invoke_with_program_id(WOOFI_PROGRAM_ID, accounts, args)
}
pub fn set_woo_coeff_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SetWooCoeffAccounts<'_, '_>,
    args: SetWooCoeffIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SetWooCoeffKeys = accounts.into();
    let ix = set_woo_coeff_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn set_woo_coeff_invoke_signed(
    accounts: SetWooCoeffAccounts<'_, '_>,
    args: SetWooCoeffIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    set_woo_coeff_invoke_signed_with_program_id(WOOFI_PROGRAM_ID, accounts, args, seeds)
}
pub fn set_woo_coeff_verify_account_keys(
    accounts: SetWooCoeffAccounts<'_, '_>,
    keys: SetWooCoeffKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.wooconfig.key, keys.wooconfig),
        (*accounts.wooracle.key, keys.wooracle),
        (*accounts.authority.key, keys.authority),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn set_woo_coeff_verify_writable_privileges<'me, 'info>(
    accounts: SetWooCoeffAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.wooracle] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn set_woo_coeff_verify_signer_privileges<'me, 'info>(
    accounts: SetWooCoeffAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn set_woo_coeff_verify_account_privileges<'me, 'info>(
    accounts: SetWooCoeffAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    set_woo_coeff_verify_writable_privileges(accounts)?;
    set_woo_coeff_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SET_WOO_SPREAD_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct SetWooSpreadAccounts<'me, 'info> {
    pub wooconfig: &'me AccountInfo<'info>,
    pub wooracle: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SetWooSpreadKeys {
    pub wooconfig: Pubkey,
    pub wooracle: Pubkey,
    pub authority: Pubkey,
}
impl From<SetWooSpreadAccounts<'_, '_>> for SetWooSpreadKeys {
    fn from(accounts: SetWooSpreadAccounts) -> Self {
        Self {
            wooconfig: *accounts.wooconfig.key,
            wooracle: *accounts.wooracle.key,
            authority: *accounts.authority.key,
        }
    }
}
impl From<SetWooSpreadKeys> for [AccountMeta; SET_WOO_SPREAD_IX_ACCOUNTS_LEN] {
    fn from(keys: SetWooSpreadKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.wooconfig,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.wooracle,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.authority,
                is_signer: true,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; SET_WOO_SPREAD_IX_ACCOUNTS_LEN]> for SetWooSpreadKeys {
    fn from(pubkeys: [Pubkey; SET_WOO_SPREAD_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            wooconfig: pubkeys[0],
            wooracle: pubkeys[1],
            authority: pubkeys[2],
        }
    }
}
impl<'info> From<SetWooSpreadAccounts<'_, 'info>>
for [AccountInfo<'info>; SET_WOO_SPREAD_IX_ACCOUNTS_LEN] {
    fn from(accounts: SetWooSpreadAccounts<'_, 'info>) -> Self {
        [
            accounts.wooconfig.clone(),
            accounts.wooracle.clone(),
            accounts.authority.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SET_WOO_SPREAD_IX_ACCOUNTS_LEN]>
for SetWooSpreadAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; SET_WOO_SPREAD_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            wooconfig: &arr[0],
            wooracle: &arr[1],
            authority: &arr[2],
        }
    }
}
pub const SET_WOO_SPREAD_IX_DISCM: [u8; 8usize] = [
    235, 93, 181, 194, 129, 219, 119, 243,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SetWooSpreadIxArgs {
    pub spread: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct SetWooSpreadIxData(pub SetWooSpreadIxArgs);
impl From<SetWooSpreadIxArgs> for SetWooSpreadIxData {
    fn from(args: SetWooSpreadIxArgs) -> Self {
        Self(args)
    }
}
impl SetWooSpreadIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SET_WOO_SPREAD_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let spread: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(Self(SetWooSpreadIxArgs { spread }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SET_WOO_SPREAD_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.spread, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn set_woo_spread_ix_with_program_id(
    program_id: Pubkey,
    keys: SetWooSpreadKeys,
    args: SetWooSpreadIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SET_WOO_SPREAD_IX_ACCOUNTS_LEN] = keys.into();
    let data: SetWooSpreadIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn set_woo_spread_ix(
    keys: SetWooSpreadKeys,
    args: SetWooSpreadIxArgs,
) -> std::io::Result<Instruction> {
    set_woo_spread_ix_with_program_id(WOOFI_PROGRAM_ID, keys, args)
}
pub fn set_woo_spread_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SetWooSpreadAccounts<'_, '_>,
    args: SetWooSpreadIxArgs,
) -> ProgramResult {
    let keys: SetWooSpreadKeys = accounts.into();
    let ix = set_woo_spread_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn set_woo_spread_invoke(
    accounts: SetWooSpreadAccounts<'_, '_>,
    args: SetWooSpreadIxArgs,
) -> ProgramResult {
    set_woo_spread_invoke_with_program_id(WOOFI_PROGRAM_ID, accounts, args)
}
pub fn set_woo_spread_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SetWooSpreadAccounts<'_, '_>,
    args: SetWooSpreadIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SetWooSpreadKeys = accounts.into();
    let ix = set_woo_spread_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn set_woo_spread_invoke_signed(
    accounts: SetWooSpreadAccounts<'_, '_>,
    args: SetWooSpreadIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    set_woo_spread_invoke_signed_with_program_id(WOOFI_PROGRAM_ID, accounts, args, seeds)
}
pub fn set_woo_spread_verify_account_keys(
    accounts: SetWooSpreadAccounts<'_, '_>,
    keys: SetWooSpreadKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.wooconfig.key, keys.wooconfig),
        (*accounts.wooracle.key, keys.wooracle),
        (*accounts.authority.key, keys.authority),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn set_woo_spread_verify_writable_privileges<'me, 'info>(
    accounts: SetWooSpreadAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.wooracle] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn set_woo_spread_verify_signer_privileges<'me, 'info>(
    accounts: SetWooSpreadAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn set_woo_spread_verify_account_privileges<'me, 'info>(
    accounts: SetWooSpreadAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    set_woo_spread_verify_writable_privileges(accounts)?;
    set_woo_spread_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SET_WOO_ADMIN_IX_ACCOUNTS_LEN: usize = 2;
#[derive(Copy, Clone, Debug)]
pub struct SetWooAdminAccounts<'me, 'info> {
    pub wooconfig: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SetWooAdminKeys {
    pub wooconfig: Pubkey,
    pub authority: Pubkey,
}
impl From<SetWooAdminAccounts<'_, '_>> for SetWooAdminKeys {
    fn from(accounts: SetWooAdminAccounts) -> Self {
        Self {
            wooconfig: *accounts.wooconfig.key,
            authority: *accounts.authority.key,
        }
    }
}
impl From<SetWooAdminKeys> for [AccountMeta; SET_WOO_ADMIN_IX_ACCOUNTS_LEN] {
    fn from(keys: SetWooAdminKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.wooconfig,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.authority,
                is_signer: true,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; SET_WOO_ADMIN_IX_ACCOUNTS_LEN]> for SetWooAdminKeys {
    fn from(pubkeys: [Pubkey; SET_WOO_ADMIN_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            wooconfig: pubkeys[0],
            authority: pubkeys[1],
        }
    }
}
impl<'info> From<SetWooAdminAccounts<'_, 'info>>
for [AccountInfo<'info>; SET_WOO_ADMIN_IX_ACCOUNTS_LEN] {
    fn from(accounts: SetWooAdminAccounts<'_, 'info>) -> Self {
        [accounts.wooconfig.clone(), accounts.authority.clone()]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SET_WOO_ADMIN_IX_ACCOUNTS_LEN]>
for SetWooAdminAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; SET_WOO_ADMIN_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            wooconfig: &arr[0],
            authority: &arr[1],
        }
    }
}
pub const SET_WOO_ADMIN_IX_DISCM: [u8; 8usize] = [139, 94, 246, 31, 251, 133, 174, 104];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SetWooAdminIxArgs {
    pub admin_authority: Vec<Pubkey>,
}
#[derive(Clone, Debug, PartialEq)]
pub struct SetWooAdminIxData(pub SetWooAdminIxArgs);
impl From<SetWooAdminIxArgs> for SetWooAdminIxData {
    fn from(args: SetWooAdminIxArgs) -> Self {
        Self(args)
    }
}
impl SetWooAdminIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SET_WOO_ADMIN_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let admin_authority: Vec<Pubkey> = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(SetWooAdminIxArgs {
                admin_authority,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SET_WOO_ADMIN_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.admin_authority, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn set_woo_admin_ix_with_program_id(
    program_id: Pubkey,
    keys: SetWooAdminKeys,
    args: SetWooAdminIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SET_WOO_ADMIN_IX_ACCOUNTS_LEN] = keys.into();
    let data: SetWooAdminIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn set_woo_admin_ix(
    keys: SetWooAdminKeys,
    args: SetWooAdminIxArgs,
) -> std::io::Result<Instruction> {
    set_woo_admin_ix_with_program_id(WOOFI_PROGRAM_ID, keys, args)
}
pub fn set_woo_admin_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SetWooAdminAccounts<'_, '_>,
    args: SetWooAdminIxArgs,
) -> ProgramResult {
    let keys: SetWooAdminKeys = accounts.into();
    let ix = set_woo_admin_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn set_woo_admin_invoke(
    accounts: SetWooAdminAccounts<'_, '_>,
    args: SetWooAdminIxArgs,
) -> ProgramResult {
    set_woo_admin_invoke_with_program_id(WOOFI_PROGRAM_ID, accounts, args)
}
pub fn set_woo_admin_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SetWooAdminAccounts<'_, '_>,
    args: SetWooAdminIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SetWooAdminKeys = accounts.into();
    let ix = set_woo_admin_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn set_woo_admin_invoke_signed(
    accounts: SetWooAdminAccounts<'_, '_>,
    args: SetWooAdminIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    set_woo_admin_invoke_signed_with_program_id(WOOFI_PROGRAM_ID, accounts, args, seeds)
}
pub fn set_woo_admin_verify_account_keys(
    accounts: SetWooAdminAccounts<'_, '_>,
    keys: SetWooAdminKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.wooconfig.key, keys.wooconfig),
        (*accounts.authority.key, keys.authority),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn set_woo_admin_verify_writable_privileges<'me, 'info>(
    accounts: SetWooAdminAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.wooconfig] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn set_woo_admin_verify_signer_privileges<'me, 'info>(
    accounts: SetWooAdminAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn set_woo_admin_verify_account_privileges<'me, 'info>(
    accounts: SetWooAdminAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    set_woo_admin_verify_writable_privileges(accounts)?;
    set_woo_admin_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SET_GUARDIAN_ADMIN_IX_ACCOUNTS_LEN: usize = 2;
#[derive(Copy, Clone, Debug)]
pub struct SetGuardianAdminAccounts<'me, 'info> {
    pub wooconfig: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SetGuardianAdminKeys {
    pub wooconfig: Pubkey,
    pub authority: Pubkey,
}
impl From<SetGuardianAdminAccounts<'_, '_>> for SetGuardianAdminKeys {
    fn from(accounts: SetGuardianAdminAccounts) -> Self {
        Self {
            wooconfig: *accounts.wooconfig.key,
            authority: *accounts.authority.key,
        }
    }
}
impl From<SetGuardianAdminKeys> for [AccountMeta; SET_GUARDIAN_ADMIN_IX_ACCOUNTS_LEN] {
    fn from(keys: SetGuardianAdminKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.wooconfig,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.authority,
                is_signer: true,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; SET_GUARDIAN_ADMIN_IX_ACCOUNTS_LEN]> for SetGuardianAdminKeys {
    fn from(pubkeys: [Pubkey; SET_GUARDIAN_ADMIN_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            wooconfig: pubkeys[0],
            authority: pubkeys[1],
        }
    }
}
impl<'info> From<SetGuardianAdminAccounts<'_, 'info>>
for [AccountInfo<'info>; SET_GUARDIAN_ADMIN_IX_ACCOUNTS_LEN] {
    fn from(accounts: SetGuardianAdminAccounts<'_, 'info>) -> Self {
        [accounts.wooconfig.clone(), accounts.authority.clone()]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SET_GUARDIAN_ADMIN_IX_ACCOUNTS_LEN]>
for SetGuardianAdminAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; SET_GUARDIAN_ADMIN_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            wooconfig: &arr[0],
            authority: &arr[1],
        }
    }
}
pub const SET_GUARDIAN_ADMIN_IX_DISCM: [u8; 8usize] = [
    143, 114, 173, 208, 10, 22, 52, 9,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SetGuardianAdminIxArgs {
    pub guardian_authority: Vec<Pubkey>,
}
#[derive(Clone, Debug, PartialEq)]
pub struct SetGuardianAdminIxData(pub SetGuardianAdminIxArgs);
impl From<SetGuardianAdminIxArgs> for SetGuardianAdminIxData {
    fn from(args: SetGuardianAdminIxArgs) -> Self {
        Self(args)
    }
}
impl SetGuardianAdminIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SET_GUARDIAN_ADMIN_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let guardian_authority: Vec<Pubkey> = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(SetGuardianAdminIxArgs {
                guardian_authority,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SET_GUARDIAN_ADMIN_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.guardian_authority, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn set_guardian_admin_ix_with_program_id(
    program_id: Pubkey,
    keys: SetGuardianAdminKeys,
    args: SetGuardianAdminIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SET_GUARDIAN_ADMIN_IX_ACCOUNTS_LEN] = keys.into();
    let data: SetGuardianAdminIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn set_guardian_admin_ix(
    keys: SetGuardianAdminKeys,
    args: SetGuardianAdminIxArgs,
) -> std::io::Result<Instruction> {
    set_guardian_admin_ix_with_program_id(WOOFI_PROGRAM_ID, keys, args)
}
pub fn set_guardian_admin_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SetGuardianAdminAccounts<'_, '_>,
    args: SetGuardianAdminIxArgs,
) -> ProgramResult {
    let keys: SetGuardianAdminKeys = accounts.into();
    let ix = set_guardian_admin_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn set_guardian_admin_invoke(
    accounts: SetGuardianAdminAccounts<'_, '_>,
    args: SetGuardianAdminIxArgs,
) -> ProgramResult {
    set_guardian_admin_invoke_with_program_id(WOOFI_PROGRAM_ID, accounts, args)
}
pub fn set_guardian_admin_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SetGuardianAdminAccounts<'_, '_>,
    args: SetGuardianAdminIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SetGuardianAdminKeys = accounts.into();
    let ix = set_guardian_admin_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn set_guardian_admin_invoke_signed(
    accounts: SetGuardianAdminAccounts<'_, '_>,
    args: SetGuardianAdminIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    set_guardian_admin_invoke_signed_with_program_id(
        WOOFI_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn set_guardian_admin_verify_account_keys(
    accounts: SetGuardianAdminAccounts<'_, '_>,
    keys: SetGuardianAdminKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.wooconfig.key, keys.wooconfig),
        (*accounts.authority.key, keys.authority),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn set_guardian_admin_verify_writable_privileges<'me, 'info>(
    accounts: SetGuardianAdminAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.wooconfig] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn set_guardian_admin_verify_signer_privileges<'me, 'info>(
    accounts: SetGuardianAdminAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn set_guardian_admin_verify_account_privileges<'me, 'info>(
    accounts: SetGuardianAdminAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    set_guardian_admin_verify_writable_privileges(accounts)?;
    set_guardian_admin_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SET_LENDING_MANAGER_IX_ACCOUNTS_LEN: usize = 2;
#[derive(Copy, Clone, Debug)]
pub struct SetLendingManagerAccounts<'me, 'info> {
    pub wooconfig: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SetLendingManagerKeys {
    pub wooconfig: Pubkey,
    pub authority: Pubkey,
}
impl From<SetLendingManagerAccounts<'_, '_>> for SetLendingManagerKeys {
    fn from(accounts: SetLendingManagerAccounts) -> Self {
        Self {
            wooconfig: *accounts.wooconfig.key,
            authority: *accounts.authority.key,
        }
    }
}
impl From<SetLendingManagerKeys> for [AccountMeta; SET_LENDING_MANAGER_IX_ACCOUNTS_LEN] {
    fn from(keys: SetLendingManagerKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.wooconfig,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.authority,
                is_signer: true,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; SET_LENDING_MANAGER_IX_ACCOUNTS_LEN]> for SetLendingManagerKeys {
    fn from(pubkeys: [Pubkey; SET_LENDING_MANAGER_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            wooconfig: pubkeys[0],
            authority: pubkeys[1],
        }
    }
}
impl<'info> From<SetLendingManagerAccounts<'_, 'info>>
for [AccountInfo<'info>; SET_LENDING_MANAGER_IX_ACCOUNTS_LEN] {
    fn from(accounts: SetLendingManagerAccounts<'_, 'info>) -> Self {
        [accounts.wooconfig.clone(), accounts.authority.clone()]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SET_LENDING_MANAGER_IX_ACCOUNTS_LEN]>
for SetLendingManagerAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; SET_LENDING_MANAGER_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            wooconfig: &arr[0],
            authority: &arr[1],
        }
    }
}
pub const SET_LENDING_MANAGER_IX_DISCM: [u8; 8usize] = [
    174, 103, 65, 227, 230, 50, 204, 96,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SetLendingManagerIxArgs {
    pub lending_managers: Vec<Pubkey>,
}
#[derive(Clone, Debug, PartialEq)]
pub struct SetLendingManagerIxData(pub SetLendingManagerIxArgs);
impl From<SetLendingManagerIxArgs> for SetLendingManagerIxData {
    fn from(args: SetLendingManagerIxArgs) -> Self {
        Self(args)
    }
}
impl SetLendingManagerIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SET_LENDING_MANAGER_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let lending_managers: Vec<Pubkey> = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(SetLendingManagerIxArgs {
                lending_managers,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SET_LENDING_MANAGER_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.lending_managers, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn set_lending_manager_ix_with_program_id(
    program_id: Pubkey,
    keys: SetLendingManagerKeys,
    args: SetLendingManagerIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SET_LENDING_MANAGER_IX_ACCOUNTS_LEN] = keys.into();
    let data: SetLendingManagerIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn set_lending_manager_ix(
    keys: SetLendingManagerKeys,
    args: SetLendingManagerIxArgs,
) -> std::io::Result<Instruction> {
    set_lending_manager_ix_with_program_id(WOOFI_PROGRAM_ID, keys, args)
}
pub fn set_lending_manager_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SetLendingManagerAccounts<'_, '_>,
    args: SetLendingManagerIxArgs,
) -> ProgramResult {
    let keys: SetLendingManagerKeys = accounts.into();
    let ix = set_lending_manager_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn set_lending_manager_invoke(
    accounts: SetLendingManagerAccounts<'_, '_>,
    args: SetLendingManagerIxArgs,
) -> ProgramResult {
    set_lending_manager_invoke_with_program_id(WOOFI_PROGRAM_ID, accounts, args)
}
pub fn set_lending_manager_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SetLendingManagerAccounts<'_, '_>,
    args: SetLendingManagerIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SetLendingManagerKeys = accounts.into();
    let ix = set_lending_manager_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn set_lending_manager_invoke_signed(
    accounts: SetLendingManagerAccounts<'_, '_>,
    args: SetLendingManagerIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    set_lending_manager_invoke_signed_with_program_id(
        WOOFI_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn set_lending_manager_verify_account_keys(
    accounts: SetLendingManagerAccounts<'_, '_>,
    keys: SetLendingManagerKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.wooconfig.key, keys.wooconfig),
        (*accounts.authority.key, keys.authority),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn set_lending_manager_verify_writable_privileges<'me, 'info>(
    accounts: SetLendingManagerAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.wooconfig] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn set_lending_manager_verify_signer_privileges<'me, 'info>(
    accounts: SetLendingManagerAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn set_lending_manager_verify_account_privileges<'me, 'info>(
    accounts: SetLendingManagerAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    set_lending_manager_verify_writable_privileges(accounts)?;
    set_lending_manager_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SET_SUPERCHARGER_VAULT_WHITELIST_IX_ACCOUNTS_LEN: usize = 2;
#[derive(Copy, Clone, Debug)]
pub struct SetSuperchargerVaultWhitelistAccounts<'me, 'info> {
    pub wooconfig: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SetSuperchargerVaultWhitelistKeys {
    pub wooconfig: Pubkey,
    pub authority: Pubkey,
}
impl From<SetSuperchargerVaultWhitelistAccounts<'_, '_>>
for SetSuperchargerVaultWhitelistKeys {
    fn from(accounts: SetSuperchargerVaultWhitelistAccounts) -> Self {
        Self {
            wooconfig: *accounts.wooconfig.key,
            authority: *accounts.authority.key,
        }
    }
}
impl From<SetSuperchargerVaultWhitelistKeys>
for [AccountMeta; SET_SUPERCHARGER_VAULT_WHITELIST_IX_ACCOUNTS_LEN] {
    fn from(keys: SetSuperchargerVaultWhitelistKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.wooconfig,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.authority,
                is_signer: true,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; SET_SUPERCHARGER_VAULT_WHITELIST_IX_ACCOUNTS_LEN]>
for SetSuperchargerVaultWhitelistKeys {
    fn from(
        pubkeys: [Pubkey; SET_SUPERCHARGER_VAULT_WHITELIST_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            wooconfig: pubkeys[0],
            authority: pubkeys[1],
        }
    }
}
impl<'info> From<SetSuperchargerVaultWhitelistAccounts<'_, 'info>>
for [AccountInfo<'info>; SET_SUPERCHARGER_VAULT_WHITELIST_IX_ACCOUNTS_LEN] {
    fn from(accounts: SetSuperchargerVaultWhitelistAccounts<'_, 'info>) -> Self {
        [accounts.wooconfig.clone(), accounts.authority.clone()]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; SET_SUPERCHARGER_VAULT_WHITELIST_IX_ACCOUNTS_LEN]>
for SetSuperchargerVaultWhitelistAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; SET_SUPERCHARGER_VAULT_WHITELIST_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            wooconfig: &arr[0],
            authority: &arr[1],
        }
    }
}
pub const SET_SUPERCHARGER_VAULT_WHITELIST_IX_DISCM: [u8; 8usize] = [
    22, 236, 175, 219, 150, 205, 128, 228,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SetSuperchargerVaultWhitelistIxArgs {
    pub supercharger_vaults: Vec<Pubkey>,
}
#[derive(Clone, Debug, PartialEq)]
pub struct SetSuperchargerVaultWhitelistIxData(pub SetSuperchargerVaultWhitelistIxArgs);
impl From<SetSuperchargerVaultWhitelistIxArgs> for SetSuperchargerVaultWhitelistIxData {
    fn from(args: SetSuperchargerVaultWhitelistIxArgs) -> Self {
        Self(args)
    }
}
impl SetSuperchargerVaultWhitelistIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SET_SUPERCHARGER_VAULT_WHITELIST_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let supercharger_vaults: Vec<Pubkey> = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(SetSuperchargerVaultWhitelistIxArgs {
                supercharger_vaults,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SET_SUPERCHARGER_VAULT_WHITELIST_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.supercharger_vaults, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn set_supercharger_vault_whitelist_ix_with_program_id(
    program_id: Pubkey,
    keys: SetSuperchargerVaultWhitelistKeys,
    args: SetSuperchargerVaultWhitelistIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SET_SUPERCHARGER_VAULT_WHITELIST_IX_ACCOUNTS_LEN] = keys
        .into();
    let data: SetSuperchargerVaultWhitelistIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn set_supercharger_vault_whitelist_ix(
    keys: SetSuperchargerVaultWhitelistKeys,
    args: SetSuperchargerVaultWhitelistIxArgs,
) -> std::io::Result<Instruction> {
    set_supercharger_vault_whitelist_ix_with_program_id(WOOFI_PROGRAM_ID, keys, args)
}
pub fn set_supercharger_vault_whitelist_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SetSuperchargerVaultWhitelistAccounts<'_, '_>,
    args: SetSuperchargerVaultWhitelistIxArgs,
) -> ProgramResult {
    let keys: SetSuperchargerVaultWhitelistKeys = accounts.into();
    let ix = set_supercharger_vault_whitelist_ix_with_program_id(
        program_id,
        keys,
        args,
    )?;
    invoke_instruction(&ix, accounts)
}
pub fn set_supercharger_vault_whitelist_invoke(
    accounts: SetSuperchargerVaultWhitelistAccounts<'_, '_>,
    args: SetSuperchargerVaultWhitelistIxArgs,
) -> ProgramResult {
    set_supercharger_vault_whitelist_invoke_with_program_id(
        WOOFI_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn set_supercharger_vault_whitelist_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SetSuperchargerVaultWhitelistAccounts<'_, '_>,
    args: SetSuperchargerVaultWhitelistIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SetSuperchargerVaultWhitelistKeys = accounts.into();
    let ix = set_supercharger_vault_whitelist_ix_with_program_id(
        program_id,
        keys,
        args,
    )?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn set_supercharger_vault_whitelist_invoke_signed(
    accounts: SetSuperchargerVaultWhitelistAccounts<'_, '_>,
    args: SetSuperchargerVaultWhitelistIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    set_supercharger_vault_whitelist_invoke_signed_with_program_id(
        WOOFI_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn set_supercharger_vault_whitelist_verify_account_keys(
    accounts: SetSuperchargerVaultWhitelistAccounts<'_, '_>,
    keys: SetSuperchargerVaultWhitelistKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.wooconfig.key, keys.wooconfig),
        (*accounts.authority.key, keys.authority),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn set_supercharger_vault_whitelist_verify_writable_privileges<'me, 'info>(
    accounts: SetSuperchargerVaultWhitelistAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.wooconfig] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn set_supercharger_vault_whitelist_verify_signer_privileges<'me, 'info>(
    accounts: SetSuperchargerVaultWhitelistAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn set_supercharger_vault_whitelist_verify_account_privileges<'me, 'info>(
    accounts: SetSuperchargerVaultWhitelistAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    set_supercharger_vault_whitelist_verify_writable_privileges(accounts)?;
    set_supercharger_vault_whitelist_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SET_WOO_STATE_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct SetWooStateAccounts<'me, 'info> {
    pub wooconfig: &'me AccountInfo<'info>,
    pub wooracle: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SetWooStateKeys {
    pub wooconfig: Pubkey,
    pub wooracle: Pubkey,
    pub authority: Pubkey,
}
impl From<SetWooStateAccounts<'_, '_>> for SetWooStateKeys {
    fn from(accounts: SetWooStateAccounts) -> Self {
        Self {
            wooconfig: *accounts.wooconfig.key,
            wooracle: *accounts.wooracle.key,
            authority: *accounts.authority.key,
        }
    }
}
impl From<SetWooStateKeys> for [AccountMeta; SET_WOO_STATE_IX_ACCOUNTS_LEN] {
    fn from(keys: SetWooStateKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.wooconfig,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.wooracle,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.authority,
                is_signer: true,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; SET_WOO_STATE_IX_ACCOUNTS_LEN]> for SetWooStateKeys {
    fn from(pubkeys: [Pubkey; SET_WOO_STATE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            wooconfig: pubkeys[0],
            wooracle: pubkeys[1],
            authority: pubkeys[2],
        }
    }
}
impl<'info> From<SetWooStateAccounts<'_, 'info>>
for [AccountInfo<'info>; SET_WOO_STATE_IX_ACCOUNTS_LEN] {
    fn from(accounts: SetWooStateAccounts<'_, 'info>) -> Self {
        [
            accounts.wooconfig.clone(),
            accounts.wooracle.clone(),
            accounts.authority.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SET_WOO_STATE_IX_ACCOUNTS_LEN]>
for SetWooStateAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; SET_WOO_STATE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            wooconfig: &arr[0],
            wooracle: &arr[1],
            authority: &arr[2],
        }
    }
}
pub const SET_WOO_STATE_IX_DISCM: [u8; 8usize] = [123, 114, 129, 125, 65, 73, 165, 17];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SetWooStateIxArgs {
    pub price: u128,
    pub coeff: u64,
    pub spread: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct SetWooStateIxData(pub SetWooStateIxArgs);
impl From<SetWooStateIxArgs> for SetWooStateIxData {
    fn from(args: SetWooStateIxArgs) -> Self {
        Self(args)
    }
}
impl SetWooStateIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SET_WOO_STATE_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let price: u128 = crate::borsh_de_or_default(&mut reader)?;
        let coeff: u64 = crate::borsh_de_or_default(&mut reader)?;
        let spread: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(SetWooStateIxArgs {
                price,
                coeff,
                spread,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SET_WOO_STATE_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.price, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.coeff, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.spread, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn set_woo_state_ix_with_program_id(
    program_id: Pubkey,
    keys: SetWooStateKeys,
    args: SetWooStateIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SET_WOO_STATE_IX_ACCOUNTS_LEN] = keys.into();
    let data: SetWooStateIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn set_woo_state_ix(
    keys: SetWooStateKeys,
    args: SetWooStateIxArgs,
) -> std::io::Result<Instruction> {
    set_woo_state_ix_with_program_id(WOOFI_PROGRAM_ID, keys, args)
}
pub fn set_woo_state_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SetWooStateAccounts<'_, '_>,
    args: SetWooStateIxArgs,
) -> ProgramResult {
    let keys: SetWooStateKeys = accounts.into();
    let ix = set_woo_state_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn set_woo_state_invoke(
    accounts: SetWooStateAccounts<'_, '_>,
    args: SetWooStateIxArgs,
) -> ProgramResult {
    set_woo_state_invoke_with_program_id(WOOFI_PROGRAM_ID, accounts, args)
}
pub fn set_woo_state_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SetWooStateAccounts<'_, '_>,
    args: SetWooStateIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SetWooStateKeys = accounts.into();
    let ix = set_woo_state_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn set_woo_state_invoke_signed(
    accounts: SetWooStateAccounts<'_, '_>,
    args: SetWooStateIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    set_woo_state_invoke_signed_with_program_id(WOOFI_PROGRAM_ID, accounts, args, seeds)
}
pub fn set_woo_state_verify_account_keys(
    accounts: SetWooStateAccounts<'_, '_>,
    keys: SetWooStateKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.wooconfig.key, keys.wooconfig),
        (*accounts.wooracle.key, keys.wooracle),
        (*accounts.authority.key, keys.authority),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn set_woo_state_verify_writable_privileges<'me, 'info>(
    accounts: SetWooStateAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.wooracle] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn set_woo_state_verify_signer_privileges<'me, 'info>(
    accounts: SetWooStateAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn set_woo_state_verify_account_privileges<'me, 'info>(
    accounts: SetWooStateAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    set_woo_state_verify_writable_privileges(accounts)?;
    set_woo_state_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const GET_PRICE_IX_ACCOUNTS_LEN: usize = 4;
#[derive(Copy, Clone, Debug)]
pub struct GetPriceAccounts<'me, 'info> {
    pub wooconfig: &'me AccountInfo<'info>,
    pub oracle: &'me AccountInfo<'info>,
    pub price_update: &'me AccountInfo<'info>,
    pub quote_price_update: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GetPriceKeys {
    pub wooconfig: Pubkey,
    pub oracle: Pubkey,
    pub price_update: Pubkey,
    pub quote_price_update: Pubkey,
}
impl From<GetPriceAccounts<'_, '_>> for GetPriceKeys {
    fn from(accounts: GetPriceAccounts) -> Self {
        Self {
            wooconfig: *accounts.wooconfig.key,
            oracle: *accounts.oracle.key,
            price_update: *accounts.price_update.key,
            quote_price_update: *accounts.quote_price_update.key,
        }
    }
}
impl From<GetPriceKeys> for [AccountMeta; GET_PRICE_IX_ACCOUNTS_LEN] {
    fn from(keys: GetPriceKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.wooconfig,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.oracle,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.price_update,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.quote_price_update,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; GET_PRICE_IX_ACCOUNTS_LEN]> for GetPriceKeys {
    fn from(pubkeys: [Pubkey; GET_PRICE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            wooconfig: pubkeys[0],
            oracle: pubkeys[1],
            price_update: pubkeys[2],
            quote_price_update: pubkeys[3],
        }
    }
}
impl<'info> From<GetPriceAccounts<'_, 'info>>
for [AccountInfo<'info>; GET_PRICE_IX_ACCOUNTS_LEN] {
    fn from(accounts: GetPriceAccounts<'_, 'info>) -> Self {
        [
            accounts.wooconfig.clone(),
            accounts.oracle.clone(),
            accounts.price_update.clone(),
            accounts.quote_price_update.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; GET_PRICE_IX_ACCOUNTS_LEN]>
for GetPriceAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; GET_PRICE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            wooconfig: &arr[0],
            oracle: &arr[1],
            price_update: &arr[2],
            quote_price_update: &arr[3],
        }
    }
}
pub const GET_PRICE_IX_DISCM: [u8; 8usize] = [238, 38, 193, 106, 228, 32, 210, 33];
#[derive(Clone, Debug, PartialEq)]
pub struct GetPriceIxData;
impl GetPriceIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != GET_PRICE_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&GET_PRICE_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn get_price_ix_with_program_id(
    program_id: Pubkey,
    keys: GetPriceKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; GET_PRICE_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: GetPriceIxData.try_to_vec()?,
    })
}
pub fn get_price_ix(keys: GetPriceKeys) -> std::io::Result<Instruction> {
    get_price_ix_with_program_id(WOOFI_PROGRAM_ID, keys)
}
pub fn get_price_invoke_with_program_id(
    program_id: Pubkey,
    accounts: GetPriceAccounts<'_, '_>,
) -> ProgramResult {
    let keys: GetPriceKeys = accounts.into();
    let ix = get_price_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn get_price_invoke(accounts: GetPriceAccounts<'_, '_>) -> ProgramResult {
    get_price_invoke_with_program_id(WOOFI_PROGRAM_ID, accounts)
}
pub fn get_price_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: GetPriceAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: GetPriceKeys = accounts.into();
    let ix = get_price_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn get_price_invoke_signed(
    accounts: GetPriceAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    get_price_invoke_signed_with_program_id(WOOFI_PROGRAM_ID, accounts, seeds)
}
pub fn get_price_verify_account_keys(
    accounts: GetPriceAccounts<'_, '_>,
    keys: GetPriceKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.wooconfig.key, keys.wooconfig),
        (*accounts.oracle.key, keys.oracle),
        (*accounts.price_update.key, keys.price_update),
        (*accounts.quote_price_update.key, keys.quote_price_update),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub const CREATE_POOL_IX_ACCOUNTS_LEN: usize = 9;
#[derive(Copy, Clone, Debug)]
pub struct CreatePoolAccounts<'me, 'info> {
    pub wooconfig: &'me AccountInfo<'info>,
    pub token_mint: &'me AccountInfo<'info>,
    pub quote_token_mint: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub woopool: &'me AccountInfo<'info>,
    pub token_vault: &'me AccountInfo<'info>,
    pub wooracle: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CreatePoolKeys {
    pub wooconfig: Pubkey,
    pub token_mint: Pubkey,
    pub quote_token_mint: Pubkey,
    pub authority: Pubkey,
    pub woopool: Pubkey,
    pub token_vault: Pubkey,
    pub wooracle: Pubkey,
    pub token_program: Pubkey,
    pub system_program: Pubkey,
}
impl From<CreatePoolAccounts<'_, '_>> for CreatePoolKeys {
    fn from(accounts: CreatePoolAccounts) -> Self {
        Self {
            wooconfig: *accounts.wooconfig.key,
            token_mint: *accounts.token_mint.key,
            quote_token_mint: *accounts.quote_token_mint.key,
            authority: *accounts.authority.key,
            woopool: *accounts.woopool.key,
            token_vault: *accounts.token_vault.key,
            wooracle: *accounts.wooracle.key,
            token_program: *accounts.token_program.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<CreatePoolKeys> for [AccountMeta; CREATE_POOL_IX_ACCOUNTS_LEN] {
    fn from(keys: CreatePoolKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.wooconfig,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.quote_token_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.authority,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.woopool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_vault,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.wooracle,
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
        ]
    }
}
impl From<[Pubkey; CREATE_POOL_IX_ACCOUNTS_LEN]> for CreatePoolKeys {
    fn from(pubkeys: [Pubkey; CREATE_POOL_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            wooconfig: pubkeys[0],
            token_mint: pubkeys[1],
            quote_token_mint: pubkeys[2],
            authority: pubkeys[3],
            woopool: pubkeys[4],
            token_vault: pubkeys[5],
            wooracle: pubkeys[6],
            token_program: pubkeys[7],
            system_program: pubkeys[8],
        }
    }
}
impl<'info> From<CreatePoolAccounts<'_, 'info>>
for [AccountInfo<'info>; CREATE_POOL_IX_ACCOUNTS_LEN] {
    fn from(accounts: CreatePoolAccounts<'_, 'info>) -> Self {
        [
            accounts.wooconfig.clone(),
            accounts.token_mint.clone(),
            accounts.quote_token_mint.clone(),
            accounts.authority.clone(),
            accounts.woopool.clone(),
            accounts.token_vault.clone(),
            accounts.wooracle.clone(),
            accounts.token_program.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CREATE_POOL_IX_ACCOUNTS_LEN]>
for CreatePoolAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; CREATE_POOL_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            wooconfig: &arr[0],
            token_mint: &arr[1],
            quote_token_mint: &arr[2],
            authority: &arr[3],
            woopool: &arr[4],
            token_vault: &arr[5],
            wooracle: &arr[6],
            token_program: &arr[7],
            system_program: &arr[8],
        }
    }
}
pub const CREATE_POOL_IX_DISCM: [u8; 8usize] = [233, 146, 209, 142, 207, 104, 64, 188];
#[derive(Clone, Debug, PartialEq)]
pub struct CreatePoolIxData;
impl CreatePoolIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CREATE_POOL_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CREATE_POOL_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn create_pool_ix_with_program_id(
    program_id: Pubkey,
    keys: CreatePoolKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CREATE_POOL_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: CreatePoolIxData.try_to_vec()?,
    })
}
pub fn create_pool_ix(keys: CreatePoolKeys) -> std::io::Result<Instruction> {
    create_pool_ix_with_program_id(WOOFI_PROGRAM_ID, keys)
}
pub fn create_pool_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CreatePoolAccounts<'_, '_>,
) -> ProgramResult {
    let keys: CreatePoolKeys = accounts.into();
    let ix = create_pool_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn create_pool_invoke(accounts: CreatePoolAccounts<'_, '_>) -> ProgramResult {
    create_pool_invoke_with_program_id(WOOFI_PROGRAM_ID, accounts)
}
pub fn create_pool_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CreatePoolAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CreatePoolKeys = accounts.into();
    let ix = create_pool_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn create_pool_invoke_signed(
    accounts: CreatePoolAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    create_pool_invoke_signed_with_program_id(WOOFI_PROGRAM_ID, accounts, seeds)
}
pub fn create_pool_verify_account_keys(
    accounts: CreatePoolAccounts<'_, '_>,
    keys: CreatePoolKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.wooconfig.key, keys.wooconfig),
        (*accounts.token_mint.key, keys.token_mint),
        (*accounts.quote_token_mint.key, keys.quote_token_mint),
        (*accounts.authority.key, keys.authority),
        (*accounts.woopool.key, keys.woopool),
        (*accounts.token_vault.key, keys.token_vault),
        (*accounts.wooracle.key, keys.wooracle),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn create_pool_verify_writable_privileges<'me, 'info>(
    accounts: CreatePoolAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.authority,
        accounts.woopool,
        accounts.token_vault,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn create_pool_verify_signer_privileges<'me, 'info>(
    accounts: CreatePoolAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority, accounts.token_vault] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn create_pool_verify_account_privileges<'me, 'info>(
    accounts: CreatePoolAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    create_pool_verify_writable_privileges(accounts)?;
    create_pool_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CREATE_WOO_AMM_POOL_IX_ACCOUNTS_LEN: usize = 9;
#[derive(Copy, Clone, Debug)]
pub struct CreateWooAmmPoolAccounts<'me, 'info> {
    pub wooconfig: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub wooammpool: &'me AccountInfo<'info>,
    pub wooracle_a: &'me AccountInfo<'info>,
    pub woopool_a: &'me AccountInfo<'info>,
    pub wooracle_b: &'me AccountInfo<'info>,
    pub woopool_b: &'me AccountInfo<'info>,
    pub woopool_quote: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CreateWooAmmPoolKeys {
    pub wooconfig: Pubkey,
    pub authority: Pubkey,
    pub wooammpool: Pubkey,
    pub wooracle_a: Pubkey,
    pub woopool_a: Pubkey,
    pub wooracle_b: Pubkey,
    pub woopool_b: Pubkey,
    pub woopool_quote: Pubkey,
    pub system_program: Pubkey,
}
impl From<CreateWooAmmPoolAccounts<'_, '_>> for CreateWooAmmPoolKeys {
    fn from(accounts: CreateWooAmmPoolAccounts) -> Self {
        Self {
            wooconfig: *accounts.wooconfig.key,
            authority: *accounts.authority.key,
            wooammpool: *accounts.wooammpool.key,
            wooracle_a: *accounts.wooracle_a.key,
            woopool_a: *accounts.woopool_a.key,
            wooracle_b: *accounts.wooracle_b.key,
            woopool_b: *accounts.woopool_b.key,
            woopool_quote: *accounts.woopool_quote.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<CreateWooAmmPoolKeys> for [AccountMeta; CREATE_WOO_AMM_POOL_IX_ACCOUNTS_LEN] {
    fn from(keys: CreateWooAmmPoolKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.wooconfig,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.authority,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.wooammpool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.wooracle_a,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.woopool_a,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.wooracle_b,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.woopool_b,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.woopool_quote,
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
impl From<[Pubkey; CREATE_WOO_AMM_POOL_IX_ACCOUNTS_LEN]> for CreateWooAmmPoolKeys {
    fn from(pubkeys: [Pubkey; CREATE_WOO_AMM_POOL_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            wooconfig: pubkeys[0],
            authority: pubkeys[1],
            wooammpool: pubkeys[2],
            wooracle_a: pubkeys[3],
            woopool_a: pubkeys[4],
            wooracle_b: pubkeys[5],
            woopool_b: pubkeys[6],
            woopool_quote: pubkeys[7],
            system_program: pubkeys[8],
        }
    }
}
impl<'info> From<CreateWooAmmPoolAccounts<'_, 'info>>
for [AccountInfo<'info>; CREATE_WOO_AMM_POOL_IX_ACCOUNTS_LEN] {
    fn from(accounts: CreateWooAmmPoolAccounts<'_, 'info>) -> Self {
        [
            accounts.wooconfig.clone(),
            accounts.authority.clone(),
            accounts.wooammpool.clone(),
            accounts.wooracle_a.clone(),
            accounts.woopool_a.clone(),
            accounts.wooracle_b.clone(),
            accounts.woopool_b.clone(),
            accounts.woopool_quote.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CREATE_WOO_AMM_POOL_IX_ACCOUNTS_LEN]>
for CreateWooAmmPoolAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; CREATE_WOO_AMM_POOL_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            wooconfig: &arr[0],
            authority: &arr[1],
            wooammpool: &arr[2],
            wooracle_a: &arr[3],
            woopool_a: &arr[4],
            wooracle_b: &arr[5],
            woopool_b: &arr[6],
            woopool_quote: &arr[7],
            system_program: &arr[8],
        }
    }
}
pub const CREATE_WOO_AMM_POOL_IX_DISCM: [u8; 8usize] = [
    10, 26, 29, 176, 163, 101, 80, 158,
];
#[derive(Clone, Debug, PartialEq)]
pub struct CreateWooAmmPoolIxData;
impl CreateWooAmmPoolIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CREATE_WOO_AMM_POOL_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CREATE_WOO_AMM_POOL_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn create_woo_amm_pool_ix_with_program_id(
    program_id: Pubkey,
    keys: CreateWooAmmPoolKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CREATE_WOO_AMM_POOL_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: CreateWooAmmPoolIxData.try_to_vec()?,
    })
}
pub fn create_woo_amm_pool_ix(
    keys: CreateWooAmmPoolKeys,
) -> std::io::Result<Instruction> {
    create_woo_amm_pool_ix_with_program_id(WOOFI_PROGRAM_ID, keys)
}
pub fn create_woo_amm_pool_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CreateWooAmmPoolAccounts<'_, '_>,
) -> ProgramResult {
    let keys: CreateWooAmmPoolKeys = accounts.into();
    let ix = create_woo_amm_pool_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn create_woo_amm_pool_invoke(
    accounts: CreateWooAmmPoolAccounts<'_, '_>,
) -> ProgramResult {
    create_woo_amm_pool_invoke_with_program_id(WOOFI_PROGRAM_ID, accounts)
}
pub fn create_woo_amm_pool_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CreateWooAmmPoolAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CreateWooAmmPoolKeys = accounts.into();
    let ix = create_woo_amm_pool_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn create_woo_amm_pool_invoke_signed(
    accounts: CreateWooAmmPoolAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    create_woo_amm_pool_invoke_signed_with_program_id(WOOFI_PROGRAM_ID, accounts, seeds)
}
pub fn create_woo_amm_pool_verify_account_keys(
    accounts: CreateWooAmmPoolAccounts<'_, '_>,
    keys: CreateWooAmmPoolKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.wooconfig.key, keys.wooconfig),
        (*accounts.authority.key, keys.authority),
        (*accounts.wooammpool.key, keys.wooammpool),
        (*accounts.wooracle_a.key, keys.wooracle_a),
        (*accounts.woopool_a.key, keys.woopool_a),
        (*accounts.wooracle_b.key, keys.wooracle_b),
        (*accounts.woopool_b.key, keys.woopool_b),
        (*accounts.woopool_quote.key, keys.woopool_quote),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn create_woo_amm_pool_verify_writable_privileges<'me, 'info>(
    accounts: CreateWooAmmPoolAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.authority,
        accounts.wooammpool,
        accounts.woopool_quote,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn create_woo_amm_pool_verify_signer_privileges<'me, 'info>(
    accounts: CreateWooAmmPoolAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn create_woo_amm_pool_verify_account_privileges<'me, 'info>(
    accounts: CreateWooAmmPoolAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    create_woo_amm_pool_verify_writable_privileges(accounts)?;
    create_woo_amm_pool_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SET_POOL_ADMIN_IX_ACCOUNTS_LEN: usize = 2;
#[derive(Copy, Clone, Debug)]
pub struct SetPoolAdminAccounts<'me, 'info> {
    pub wooconfig: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SetPoolAdminKeys {
    pub wooconfig: Pubkey,
    pub authority: Pubkey,
}
impl From<SetPoolAdminAccounts<'_, '_>> for SetPoolAdminKeys {
    fn from(accounts: SetPoolAdminAccounts) -> Self {
        Self {
            wooconfig: *accounts.wooconfig.key,
            authority: *accounts.authority.key,
        }
    }
}
impl From<SetPoolAdminKeys> for [AccountMeta; SET_POOL_ADMIN_IX_ACCOUNTS_LEN] {
    fn from(keys: SetPoolAdminKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.wooconfig,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.authority,
                is_signer: true,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; SET_POOL_ADMIN_IX_ACCOUNTS_LEN]> for SetPoolAdminKeys {
    fn from(pubkeys: [Pubkey; SET_POOL_ADMIN_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            wooconfig: pubkeys[0],
            authority: pubkeys[1],
        }
    }
}
impl<'info> From<SetPoolAdminAccounts<'_, 'info>>
for [AccountInfo<'info>; SET_POOL_ADMIN_IX_ACCOUNTS_LEN] {
    fn from(accounts: SetPoolAdminAccounts<'_, 'info>) -> Self {
        [accounts.wooconfig.clone(), accounts.authority.clone()]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SET_POOL_ADMIN_IX_ACCOUNTS_LEN]>
for SetPoolAdminAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; SET_POOL_ADMIN_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            wooconfig: &arr[0],
            authority: &arr[1],
        }
    }
}
pub const SET_POOL_ADMIN_IX_DISCM: [u8; 8usize] = [37, 9, 3, 197, 132, 224, 165, 21];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SetPoolAdminIxArgs {
    pub admin_authority: Vec<Pubkey>,
}
#[derive(Clone, Debug, PartialEq)]
pub struct SetPoolAdminIxData(pub SetPoolAdminIxArgs);
impl From<SetPoolAdminIxArgs> for SetPoolAdminIxData {
    fn from(args: SetPoolAdminIxArgs) -> Self {
        Self(args)
    }
}
impl SetPoolAdminIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SET_POOL_ADMIN_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let admin_authority: Vec<Pubkey> = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(SetPoolAdminIxArgs {
                admin_authority,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SET_POOL_ADMIN_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.admin_authority, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn set_pool_admin_ix_with_program_id(
    program_id: Pubkey,
    keys: SetPoolAdminKeys,
    args: SetPoolAdminIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SET_POOL_ADMIN_IX_ACCOUNTS_LEN] = keys.into();
    let data: SetPoolAdminIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn set_pool_admin_ix(
    keys: SetPoolAdminKeys,
    args: SetPoolAdminIxArgs,
) -> std::io::Result<Instruction> {
    set_pool_admin_ix_with_program_id(WOOFI_PROGRAM_ID, keys, args)
}
pub fn set_pool_admin_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SetPoolAdminAccounts<'_, '_>,
    args: SetPoolAdminIxArgs,
) -> ProgramResult {
    let keys: SetPoolAdminKeys = accounts.into();
    let ix = set_pool_admin_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn set_pool_admin_invoke(
    accounts: SetPoolAdminAccounts<'_, '_>,
    args: SetPoolAdminIxArgs,
) -> ProgramResult {
    set_pool_admin_invoke_with_program_id(WOOFI_PROGRAM_ID, accounts, args)
}
pub fn set_pool_admin_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SetPoolAdminAccounts<'_, '_>,
    args: SetPoolAdminIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SetPoolAdminKeys = accounts.into();
    let ix = set_pool_admin_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn set_pool_admin_invoke_signed(
    accounts: SetPoolAdminAccounts<'_, '_>,
    args: SetPoolAdminIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    set_pool_admin_invoke_signed_with_program_id(WOOFI_PROGRAM_ID, accounts, args, seeds)
}
pub fn set_pool_admin_verify_account_keys(
    accounts: SetPoolAdminAccounts<'_, '_>,
    keys: SetPoolAdminKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.wooconfig.key, keys.wooconfig),
        (*accounts.authority.key, keys.authority),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn set_pool_admin_verify_writable_privileges<'me, 'info>(
    accounts: SetPoolAdminAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.wooconfig] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn set_pool_admin_verify_signer_privileges<'me, 'info>(
    accounts: SetPoolAdminAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn set_pool_admin_verify_account_privileges<'me, 'info>(
    accounts: SetPoolAdminAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    set_pool_admin_verify_writable_privileges(accounts)?;
    set_pool_admin_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SET_FEE_ADMIN_IX_ACCOUNTS_LEN: usize = 2;
#[derive(Copy, Clone, Debug)]
pub struct SetFeeAdminAccounts<'me, 'info> {
    pub wooconfig: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SetFeeAdminKeys {
    pub wooconfig: Pubkey,
    pub authority: Pubkey,
}
impl From<SetFeeAdminAccounts<'_, '_>> for SetFeeAdminKeys {
    fn from(accounts: SetFeeAdminAccounts) -> Self {
        Self {
            wooconfig: *accounts.wooconfig.key,
            authority: *accounts.authority.key,
        }
    }
}
impl From<SetFeeAdminKeys> for [AccountMeta; SET_FEE_ADMIN_IX_ACCOUNTS_LEN] {
    fn from(keys: SetFeeAdminKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.wooconfig,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.authority,
                is_signer: true,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; SET_FEE_ADMIN_IX_ACCOUNTS_LEN]> for SetFeeAdminKeys {
    fn from(pubkeys: [Pubkey; SET_FEE_ADMIN_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            wooconfig: pubkeys[0],
            authority: pubkeys[1],
        }
    }
}
impl<'info> From<SetFeeAdminAccounts<'_, 'info>>
for [AccountInfo<'info>; SET_FEE_ADMIN_IX_ACCOUNTS_LEN] {
    fn from(accounts: SetFeeAdminAccounts<'_, 'info>) -> Self {
        [accounts.wooconfig.clone(), accounts.authority.clone()]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SET_FEE_ADMIN_IX_ACCOUNTS_LEN]>
for SetFeeAdminAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; SET_FEE_ADMIN_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            wooconfig: &arr[0],
            authority: &arr[1],
        }
    }
}
pub const SET_FEE_ADMIN_IX_DISCM: [u8; 8usize] = [45, 227, 180, 113, 69, 212, 7, 7];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SetFeeAdminIxArgs {
    pub fee_authority: Vec<Pubkey>,
}
#[derive(Clone, Debug, PartialEq)]
pub struct SetFeeAdminIxData(pub SetFeeAdminIxArgs);
impl From<SetFeeAdminIxArgs> for SetFeeAdminIxData {
    fn from(args: SetFeeAdminIxArgs) -> Self {
        Self(args)
    }
}
impl SetFeeAdminIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SET_FEE_ADMIN_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let fee_authority: Vec<Pubkey> = crate::borsh_de_or_default(&mut reader)?;
        Ok(Self(SetFeeAdminIxArgs { fee_authority }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SET_FEE_ADMIN_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.fee_authority, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn set_fee_admin_ix_with_program_id(
    program_id: Pubkey,
    keys: SetFeeAdminKeys,
    args: SetFeeAdminIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SET_FEE_ADMIN_IX_ACCOUNTS_LEN] = keys.into();
    let data: SetFeeAdminIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn set_fee_admin_ix(
    keys: SetFeeAdminKeys,
    args: SetFeeAdminIxArgs,
) -> std::io::Result<Instruction> {
    set_fee_admin_ix_with_program_id(WOOFI_PROGRAM_ID, keys, args)
}
pub fn set_fee_admin_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SetFeeAdminAccounts<'_, '_>,
    args: SetFeeAdminIxArgs,
) -> ProgramResult {
    let keys: SetFeeAdminKeys = accounts.into();
    let ix = set_fee_admin_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn set_fee_admin_invoke(
    accounts: SetFeeAdminAccounts<'_, '_>,
    args: SetFeeAdminIxArgs,
) -> ProgramResult {
    set_fee_admin_invoke_with_program_id(WOOFI_PROGRAM_ID, accounts, args)
}
pub fn set_fee_admin_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SetFeeAdminAccounts<'_, '_>,
    args: SetFeeAdminIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SetFeeAdminKeys = accounts.into();
    let ix = set_fee_admin_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn set_fee_admin_invoke_signed(
    accounts: SetFeeAdminAccounts<'_, '_>,
    args: SetFeeAdminIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    set_fee_admin_invoke_signed_with_program_id(WOOFI_PROGRAM_ID, accounts, args, seeds)
}
pub fn set_fee_admin_verify_account_keys(
    accounts: SetFeeAdminAccounts<'_, '_>,
    keys: SetFeeAdminKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.wooconfig.key, keys.wooconfig),
        (*accounts.authority.key, keys.authority),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn set_fee_admin_verify_writable_privileges<'me, 'info>(
    accounts: SetFeeAdminAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.wooconfig] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn set_fee_admin_verify_signer_privileges<'me, 'info>(
    accounts: SetFeeAdminAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn set_fee_admin_verify_account_privileges<'me, 'info>(
    accounts: SetFeeAdminAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    set_fee_admin_verify_writable_privileges(accounts)?;
    set_fee_admin_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SET_PAUSE_ROLE_IX_ACCOUNTS_LEN: usize = 2;
#[derive(Copy, Clone, Debug)]
pub struct SetPauseRoleAccounts<'me, 'info> {
    pub wooconfig: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SetPauseRoleKeys {
    pub wooconfig: Pubkey,
    pub authority: Pubkey,
}
impl From<SetPauseRoleAccounts<'_, '_>> for SetPauseRoleKeys {
    fn from(accounts: SetPauseRoleAccounts) -> Self {
        Self {
            wooconfig: *accounts.wooconfig.key,
            authority: *accounts.authority.key,
        }
    }
}
impl From<SetPauseRoleKeys> for [AccountMeta; SET_PAUSE_ROLE_IX_ACCOUNTS_LEN] {
    fn from(keys: SetPauseRoleKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.wooconfig,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.authority,
                is_signer: true,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; SET_PAUSE_ROLE_IX_ACCOUNTS_LEN]> for SetPauseRoleKeys {
    fn from(pubkeys: [Pubkey; SET_PAUSE_ROLE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            wooconfig: pubkeys[0],
            authority: pubkeys[1],
        }
    }
}
impl<'info> From<SetPauseRoleAccounts<'_, 'info>>
for [AccountInfo<'info>; SET_PAUSE_ROLE_IX_ACCOUNTS_LEN] {
    fn from(accounts: SetPauseRoleAccounts<'_, 'info>) -> Self {
        [accounts.wooconfig.clone(), accounts.authority.clone()]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SET_PAUSE_ROLE_IX_ACCOUNTS_LEN]>
for SetPauseRoleAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; SET_PAUSE_ROLE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            wooconfig: &arr[0],
            authority: &arr[1],
        }
    }
}
pub const SET_PAUSE_ROLE_IX_DISCM: [u8; 8usize] = [251, 5, 169, 217, 244, 144, 160, 211];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SetPauseRoleIxArgs {
    pub pause_authority: Vec<Pubkey>,
}
#[derive(Clone, Debug, PartialEq)]
pub struct SetPauseRoleIxData(pub SetPauseRoleIxArgs);
impl From<SetPauseRoleIxArgs> for SetPauseRoleIxData {
    fn from(args: SetPauseRoleIxArgs) -> Self {
        Self(args)
    }
}
impl SetPauseRoleIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SET_PAUSE_ROLE_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let pause_authority: Vec<Pubkey> = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(SetPauseRoleIxArgs {
                pause_authority,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SET_PAUSE_ROLE_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.pause_authority, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn set_pause_role_ix_with_program_id(
    program_id: Pubkey,
    keys: SetPauseRoleKeys,
    args: SetPauseRoleIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SET_PAUSE_ROLE_IX_ACCOUNTS_LEN] = keys.into();
    let data: SetPauseRoleIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn set_pause_role_ix(
    keys: SetPauseRoleKeys,
    args: SetPauseRoleIxArgs,
) -> std::io::Result<Instruction> {
    set_pause_role_ix_with_program_id(WOOFI_PROGRAM_ID, keys, args)
}
pub fn set_pause_role_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SetPauseRoleAccounts<'_, '_>,
    args: SetPauseRoleIxArgs,
) -> ProgramResult {
    let keys: SetPauseRoleKeys = accounts.into();
    let ix = set_pause_role_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn set_pause_role_invoke(
    accounts: SetPauseRoleAccounts<'_, '_>,
    args: SetPauseRoleIxArgs,
) -> ProgramResult {
    set_pause_role_invoke_with_program_id(WOOFI_PROGRAM_ID, accounts, args)
}
pub fn set_pause_role_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SetPauseRoleAccounts<'_, '_>,
    args: SetPauseRoleIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SetPauseRoleKeys = accounts.into();
    let ix = set_pause_role_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn set_pause_role_invoke_signed(
    accounts: SetPauseRoleAccounts<'_, '_>,
    args: SetPauseRoleIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    set_pause_role_invoke_signed_with_program_id(WOOFI_PROGRAM_ID, accounts, args, seeds)
}
pub fn set_pause_role_verify_account_keys(
    accounts: SetPauseRoleAccounts<'_, '_>,
    keys: SetPauseRoleKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.wooconfig.key, keys.wooconfig),
        (*accounts.authority.key, keys.authority),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn set_pause_role_verify_writable_privileges<'me, 'info>(
    accounts: SetPauseRoleAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.wooconfig] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn set_pause_role_verify_signer_privileges<'me, 'info>(
    accounts: SetPauseRoleAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn set_pause_role_verify_account_privileges<'me, 'info>(
    accounts: SetPauseRoleAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    set_pause_role_verify_writable_privileges(accounts)?;
    set_pause_role_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const PAUSE_IX_ACCOUNTS_LEN: usize = 2;
#[derive(Copy, Clone, Debug)]
pub struct PauseAccounts<'me, 'info> {
    pub wooconfig: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct PauseKeys {
    pub wooconfig: Pubkey,
    pub authority: Pubkey,
}
impl From<PauseAccounts<'_, '_>> for PauseKeys {
    fn from(accounts: PauseAccounts) -> Self {
        Self {
            wooconfig: *accounts.wooconfig.key,
            authority: *accounts.authority.key,
        }
    }
}
impl From<PauseKeys> for [AccountMeta; PAUSE_IX_ACCOUNTS_LEN] {
    fn from(keys: PauseKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.wooconfig,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.authority,
                is_signer: true,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; PAUSE_IX_ACCOUNTS_LEN]> for PauseKeys {
    fn from(pubkeys: [Pubkey; PAUSE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            wooconfig: pubkeys[0],
            authority: pubkeys[1],
        }
    }
}
impl<'info> From<PauseAccounts<'_, 'info>>
for [AccountInfo<'info>; PAUSE_IX_ACCOUNTS_LEN] {
    fn from(accounts: PauseAccounts<'_, 'info>) -> Self {
        [accounts.wooconfig.clone(), accounts.authority.clone()]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; PAUSE_IX_ACCOUNTS_LEN]>
for PauseAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; PAUSE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            wooconfig: &arr[0],
            authority: &arr[1],
        }
    }
}
pub const PAUSE_IX_DISCM: [u8; 8usize] = [211, 22, 221, 251, 74, 121, 193, 47];
#[derive(Clone, Debug, PartialEq)]
pub struct PauseIxData;
impl PauseIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != PAUSE_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&PAUSE_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn pause_ix_with_program_id(
    program_id: Pubkey,
    keys: PauseKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; PAUSE_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: PauseIxData.try_to_vec()?,
    })
}
pub fn pause_ix(keys: PauseKeys) -> std::io::Result<Instruction> {
    pause_ix_with_program_id(WOOFI_PROGRAM_ID, keys)
}
pub fn pause_invoke_with_program_id(
    program_id: Pubkey,
    accounts: PauseAccounts<'_, '_>,
) -> ProgramResult {
    let keys: PauseKeys = accounts.into();
    let ix = pause_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn pause_invoke(accounts: PauseAccounts<'_, '_>) -> ProgramResult {
    pause_invoke_with_program_id(WOOFI_PROGRAM_ID, accounts)
}
pub fn pause_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: PauseAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: PauseKeys = accounts.into();
    let ix = pause_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn pause_invoke_signed(
    accounts: PauseAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    pause_invoke_signed_with_program_id(WOOFI_PROGRAM_ID, accounts, seeds)
}
pub fn pause_verify_account_keys(
    accounts: PauseAccounts<'_, '_>,
    keys: PauseKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.wooconfig.key, keys.wooconfig),
        (*accounts.authority.key, keys.authority),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn pause_verify_writable_privileges<'me, 'info>(
    accounts: PauseAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.wooconfig] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn pause_verify_signer_privileges<'me, 'info>(
    accounts: PauseAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn pause_verify_account_privileges<'me, 'info>(
    accounts: PauseAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    pause_verify_writable_privileges(accounts)?;
    pause_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const UNPAUSE_IX_ACCOUNTS_LEN: usize = 2;
#[derive(Copy, Clone, Debug)]
pub struct UnpauseAccounts<'me, 'info> {
    pub wooconfig: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UnpauseKeys {
    pub wooconfig: Pubkey,
    pub authority: Pubkey,
}
impl From<UnpauseAccounts<'_, '_>> for UnpauseKeys {
    fn from(accounts: UnpauseAccounts) -> Self {
        Self {
            wooconfig: *accounts.wooconfig.key,
            authority: *accounts.authority.key,
        }
    }
}
impl From<UnpauseKeys> for [AccountMeta; UNPAUSE_IX_ACCOUNTS_LEN] {
    fn from(keys: UnpauseKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.wooconfig,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.authority,
                is_signer: true,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; UNPAUSE_IX_ACCOUNTS_LEN]> for UnpauseKeys {
    fn from(pubkeys: [Pubkey; UNPAUSE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            wooconfig: pubkeys[0],
            authority: pubkeys[1],
        }
    }
}
impl<'info> From<UnpauseAccounts<'_, 'info>>
for [AccountInfo<'info>; UNPAUSE_IX_ACCOUNTS_LEN] {
    fn from(accounts: UnpauseAccounts<'_, 'info>) -> Self {
        [accounts.wooconfig.clone(), accounts.authority.clone()]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; UNPAUSE_IX_ACCOUNTS_LEN]>
for UnpauseAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; UNPAUSE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            wooconfig: &arr[0],
            authority: &arr[1],
        }
    }
}
pub const UNPAUSE_IX_DISCM: [u8; 8usize] = [169, 144, 4, 38, 10, 141, 188, 255];
#[derive(Clone, Debug, PartialEq)]
pub struct UnpauseIxData;
impl UnpauseIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UNPAUSE_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UNPAUSE_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn unpause_ix_with_program_id(
    program_id: Pubkey,
    keys: UnpauseKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; UNPAUSE_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: UnpauseIxData.try_to_vec()?,
    })
}
pub fn unpause_ix(keys: UnpauseKeys) -> std::io::Result<Instruction> {
    unpause_ix_with_program_id(WOOFI_PROGRAM_ID, keys)
}
pub fn unpause_invoke_with_program_id(
    program_id: Pubkey,
    accounts: UnpauseAccounts<'_, '_>,
) -> ProgramResult {
    let keys: UnpauseKeys = accounts.into();
    let ix = unpause_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn unpause_invoke(accounts: UnpauseAccounts<'_, '_>) -> ProgramResult {
    unpause_invoke_with_program_id(WOOFI_PROGRAM_ID, accounts)
}
pub fn unpause_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: UnpauseAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: UnpauseKeys = accounts.into();
    let ix = unpause_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn unpause_invoke_signed(
    accounts: UnpauseAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    unpause_invoke_signed_with_program_id(WOOFI_PROGRAM_ID, accounts, seeds)
}
pub fn unpause_verify_account_keys(
    accounts: UnpauseAccounts<'_, '_>,
    keys: UnpauseKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.wooconfig.key, keys.wooconfig),
        (*accounts.authority.key, keys.authority),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn unpause_verify_writable_privileges<'me, 'info>(
    accounts: UnpauseAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.wooconfig] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn unpause_verify_signer_privileges<'me, 'info>(
    accounts: UnpauseAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn unpause_verify_account_privileges<'me, 'info>(
    accounts: UnpauseAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    unpause_verify_writable_privileges(accounts)?;
    unpause_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SET_POOL_FEE_RATE_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct SetPoolFeeRateAccounts<'me, 'info> {
    pub wooconfig: &'me AccountInfo<'info>,
    pub woopool: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SetPoolFeeRateKeys {
    pub wooconfig: Pubkey,
    pub woopool: Pubkey,
    pub authority: Pubkey,
}
impl From<SetPoolFeeRateAccounts<'_, '_>> for SetPoolFeeRateKeys {
    fn from(accounts: SetPoolFeeRateAccounts) -> Self {
        Self {
            wooconfig: *accounts.wooconfig.key,
            woopool: *accounts.woopool.key,
            authority: *accounts.authority.key,
        }
    }
}
impl From<SetPoolFeeRateKeys> for [AccountMeta; SET_POOL_FEE_RATE_IX_ACCOUNTS_LEN] {
    fn from(keys: SetPoolFeeRateKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.wooconfig,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.woopool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.authority,
                is_signer: true,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; SET_POOL_FEE_RATE_IX_ACCOUNTS_LEN]> for SetPoolFeeRateKeys {
    fn from(pubkeys: [Pubkey; SET_POOL_FEE_RATE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            wooconfig: pubkeys[0],
            woopool: pubkeys[1],
            authority: pubkeys[2],
        }
    }
}
impl<'info> From<SetPoolFeeRateAccounts<'_, 'info>>
for [AccountInfo<'info>; SET_POOL_FEE_RATE_IX_ACCOUNTS_LEN] {
    fn from(accounts: SetPoolFeeRateAccounts<'_, 'info>) -> Self {
        [
            accounts.wooconfig.clone(),
            accounts.woopool.clone(),
            accounts.authority.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SET_POOL_FEE_RATE_IX_ACCOUNTS_LEN]>
for SetPoolFeeRateAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; SET_POOL_FEE_RATE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            wooconfig: &arr[0],
            woopool: &arr[1],
            authority: &arr[2],
        }
    }
}
pub const SET_POOL_FEE_RATE_IX_DISCM: [u8; 8usize] = [
    163, 215, 73, 116, 199, 173, 122, 165,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SetPoolFeeRateIxArgs {
    pub fee_rate: u16,
}
#[derive(Clone, Debug, PartialEq)]
pub struct SetPoolFeeRateIxData(pub SetPoolFeeRateIxArgs);
impl From<SetPoolFeeRateIxArgs> for SetPoolFeeRateIxData {
    fn from(args: SetPoolFeeRateIxArgs) -> Self {
        Self(args)
    }
}
impl SetPoolFeeRateIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SET_POOL_FEE_RATE_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let fee_rate: u16 = crate::borsh_de_or_default(&mut reader)?;
        Ok(Self(SetPoolFeeRateIxArgs { fee_rate }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SET_POOL_FEE_RATE_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.fee_rate, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn set_pool_fee_rate_ix_with_program_id(
    program_id: Pubkey,
    keys: SetPoolFeeRateKeys,
    args: SetPoolFeeRateIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SET_POOL_FEE_RATE_IX_ACCOUNTS_LEN] = keys.into();
    let data: SetPoolFeeRateIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn set_pool_fee_rate_ix(
    keys: SetPoolFeeRateKeys,
    args: SetPoolFeeRateIxArgs,
) -> std::io::Result<Instruction> {
    set_pool_fee_rate_ix_with_program_id(WOOFI_PROGRAM_ID, keys, args)
}
pub fn set_pool_fee_rate_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SetPoolFeeRateAccounts<'_, '_>,
    args: SetPoolFeeRateIxArgs,
) -> ProgramResult {
    let keys: SetPoolFeeRateKeys = accounts.into();
    let ix = set_pool_fee_rate_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn set_pool_fee_rate_invoke(
    accounts: SetPoolFeeRateAccounts<'_, '_>,
    args: SetPoolFeeRateIxArgs,
) -> ProgramResult {
    set_pool_fee_rate_invoke_with_program_id(WOOFI_PROGRAM_ID, accounts, args)
}
pub fn set_pool_fee_rate_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SetPoolFeeRateAccounts<'_, '_>,
    args: SetPoolFeeRateIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SetPoolFeeRateKeys = accounts.into();
    let ix = set_pool_fee_rate_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn set_pool_fee_rate_invoke_signed(
    accounts: SetPoolFeeRateAccounts<'_, '_>,
    args: SetPoolFeeRateIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    set_pool_fee_rate_invoke_signed_with_program_id(
        WOOFI_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn set_pool_fee_rate_verify_account_keys(
    accounts: SetPoolFeeRateAccounts<'_, '_>,
    keys: SetPoolFeeRateKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.wooconfig.key, keys.wooconfig),
        (*accounts.woopool.key, keys.woopool),
        (*accounts.authority.key, keys.authority),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn set_pool_fee_rate_verify_writable_privileges<'me, 'info>(
    accounts: SetPoolFeeRateAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.woopool] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn set_pool_fee_rate_verify_signer_privileges<'me, 'info>(
    accounts: SetPoolFeeRateAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn set_pool_fee_rate_verify_account_privileges<'me, 'info>(
    accounts: SetPoolFeeRateAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    set_pool_fee_rate_verify_writable_privileges(accounts)?;
    set_pool_fee_rate_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SET_POOL_MAX_GAMMA_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct SetPoolMaxGammaAccounts<'me, 'info> {
    pub wooconfig: &'me AccountInfo<'info>,
    pub woopool: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SetPoolMaxGammaKeys {
    pub wooconfig: Pubkey,
    pub woopool: Pubkey,
    pub authority: Pubkey,
}
impl From<SetPoolMaxGammaAccounts<'_, '_>> for SetPoolMaxGammaKeys {
    fn from(accounts: SetPoolMaxGammaAccounts) -> Self {
        Self {
            wooconfig: *accounts.wooconfig.key,
            woopool: *accounts.woopool.key,
            authority: *accounts.authority.key,
        }
    }
}
impl From<SetPoolMaxGammaKeys> for [AccountMeta; SET_POOL_MAX_GAMMA_IX_ACCOUNTS_LEN] {
    fn from(keys: SetPoolMaxGammaKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.wooconfig,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.woopool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.authority,
                is_signer: true,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; SET_POOL_MAX_GAMMA_IX_ACCOUNTS_LEN]> for SetPoolMaxGammaKeys {
    fn from(pubkeys: [Pubkey; SET_POOL_MAX_GAMMA_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            wooconfig: pubkeys[0],
            woopool: pubkeys[1],
            authority: pubkeys[2],
        }
    }
}
impl<'info> From<SetPoolMaxGammaAccounts<'_, 'info>>
for [AccountInfo<'info>; SET_POOL_MAX_GAMMA_IX_ACCOUNTS_LEN] {
    fn from(accounts: SetPoolMaxGammaAccounts<'_, 'info>) -> Self {
        [
            accounts.wooconfig.clone(),
            accounts.woopool.clone(),
            accounts.authority.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SET_POOL_MAX_GAMMA_IX_ACCOUNTS_LEN]>
for SetPoolMaxGammaAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; SET_POOL_MAX_GAMMA_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            wooconfig: &arr[0],
            woopool: &arr[1],
            authority: &arr[2],
        }
    }
}
pub const SET_POOL_MAX_GAMMA_IX_DISCM: [u8; 8usize] = [
    17, 128, 160, 226, 144, 180, 74, 207,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SetPoolMaxGammaIxArgs {
    pub max_gamma: u128,
}
#[derive(Clone, Debug, PartialEq)]
pub struct SetPoolMaxGammaIxData(pub SetPoolMaxGammaIxArgs);
impl From<SetPoolMaxGammaIxArgs> for SetPoolMaxGammaIxData {
    fn from(args: SetPoolMaxGammaIxArgs) -> Self {
        Self(args)
    }
}
impl SetPoolMaxGammaIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SET_POOL_MAX_GAMMA_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let max_gamma: u128 = crate::borsh_de_or_default(&mut reader)?;
        Ok(Self(SetPoolMaxGammaIxArgs { max_gamma }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SET_POOL_MAX_GAMMA_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.max_gamma, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn set_pool_max_gamma_ix_with_program_id(
    program_id: Pubkey,
    keys: SetPoolMaxGammaKeys,
    args: SetPoolMaxGammaIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SET_POOL_MAX_GAMMA_IX_ACCOUNTS_LEN] = keys.into();
    let data: SetPoolMaxGammaIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn set_pool_max_gamma_ix(
    keys: SetPoolMaxGammaKeys,
    args: SetPoolMaxGammaIxArgs,
) -> std::io::Result<Instruction> {
    set_pool_max_gamma_ix_with_program_id(WOOFI_PROGRAM_ID, keys, args)
}
pub fn set_pool_max_gamma_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SetPoolMaxGammaAccounts<'_, '_>,
    args: SetPoolMaxGammaIxArgs,
) -> ProgramResult {
    let keys: SetPoolMaxGammaKeys = accounts.into();
    let ix = set_pool_max_gamma_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn set_pool_max_gamma_invoke(
    accounts: SetPoolMaxGammaAccounts<'_, '_>,
    args: SetPoolMaxGammaIxArgs,
) -> ProgramResult {
    set_pool_max_gamma_invoke_with_program_id(WOOFI_PROGRAM_ID, accounts, args)
}
pub fn set_pool_max_gamma_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SetPoolMaxGammaAccounts<'_, '_>,
    args: SetPoolMaxGammaIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SetPoolMaxGammaKeys = accounts.into();
    let ix = set_pool_max_gamma_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn set_pool_max_gamma_invoke_signed(
    accounts: SetPoolMaxGammaAccounts<'_, '_>,
    args: SetPoolMaxGammaIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    set_pool_max_gamma_invoke_signed_with_program_id(
        WOOFI_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn set_pool_max_gamma_verify_account_keys(
    accounts: SetPoolMaxGammaAccounts<'_, '_>,
    keys: SetPoolMaxGammaKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.wooconfig.key, keys.wooconfig),
        (*accounts.woopool.key, keys.woopool),
        (*accounts.authority.key, keys.authority),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn set_pool_max_gamma_verify_writable_privileges<'me, 'info>(
    accounts: SetPoolMaxGammaAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.woopool] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn set_pool_max_gamma_verify_signer_privileges<'me, 'info>(
    accounts: SetPoolMaxGammaAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn set_pool_max_gamma_verify_account_privileges<'me, 'info>(
    accounts: SetPoolMaxGammaAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    set_pool_max_gamma_verify_writable_privileges(accounts)?;
    set_pool_max_gamma_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SET_POOL_MAX_NOTIONAL_SWAP_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct SetPoolMaxNotionalSwapAccounts<'me, 'info> {
    pub wooconfig: &'me AccountInfo<'info>,
    pub woopool: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SetPoolMaxNotionalSwapKeys {
    pub wooconfig: Pubkey,
    pub woopool: Pubkey,
    pub authority: Pubkey,
}
impl From<SetPoolMaxNotionalSwapAccounts<'_, '_>> for SetPoolMaxNotionalSwapKeys {
    fn from(accounts: SetPoolMaxNotionalSwapAccounts) -> Self {
        Self {
            wooconfig: *accounts.wooconfig.key,
            woopool: *accounts.woopool.key,
            authority: *accounts.authority.key,
        }
    }
}
impl From<SetPoolMaxNotionalSwapKeys>
for [AccountMeta; SET_POOL_MAX_NOTIONAL_SWAP_IX_ACCOUNTS_LEN] {
    fn from(keys: SetPoolMaxNotionalSwapKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.wooconfig,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.woopool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.authority,
                is_signer: true,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; SET_POOL_MAX_NOTIONAL_SWAP_IX_ACCOUNTS_LEN]>
for SetPoolMaxNotionalSwapKeys {
    fn from(pubkeys: [Pubkey; SET_POOL_MAX_NOTIONAL_SWAP_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            wooconfig: pubkeys[0],
            woopool: pubkeys[1],
            authority: pubkeys[2],
        }
    }
}
impl<'info> From<SetPoolMaxNotionalSwapAccounts<'_, 'info>>
for [AccountInfo<'info>; SET_POOL_MAX_NOTIONAL_SWAP_IX_ACCOUNTS_LEN] {
    fn from(accounts: SetPoolMaxNotionalSwapAccounts<'_, 'info>) -> Self {
        [
            accounts.wooconfig.clone(),
            accounts.woopool.clone(),
            accounts.authority.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; SET_POOL_MAX_NOTIONAL_SWAP_IX_ACCOUNTS_LEN]>
for SetPoolMaxNotionalSwapAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; SET_POOL_MAX_NOTIONAL_SWAP_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            wooconfig: &arr[0],
            woopool: &arr[1],
            authority: &arr[2],
        }
    }
}
pub const SET_POOL_MAX_NOTIONAL_SWAP_IX_DISCM: [u8; 8usize] = [
    130, 110, 160, 30, 77, 13, 141, 228,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SetPoolMaxNotionalSwapIxArgs {
    pub max_notional_swap: u128,
}
#[derive(Clone, Debug, PartialEq)]
pub struct SetPoolMaxNotionalSwapIxData(pub SetPoolMaxNotionalSwapIxArgs);
impl From<SetPoolMaxNotionalSwapIxArgs> for SetPoolMaxNotionalSwapIxData {
    fn from(args: SetPoolMaxNotionalSwapIxArgs) -> Self {
        Self(args)
    }
}
impl SetPoolMaxNotionalSwapIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SET_POOL_MAX_NOTIONAL_SWAP_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let max_notional_swap: u128 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(SetPoolMaxNotionalSwapIxArgs {
                max_notional_swap,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SET_POOL_MAX_NOTIONAL_SWAP_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.max_notional_swap, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn set_pool_max_notional_swap_ix_with_program_id(
    program_id: Pubkey,
    keys: SetPoolMaxNotionalSwapKeys,
    args: SetPoolMaxNotionalSwapIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SET_POOL_MAX_NOTIONAL_SWAP_IX_ACCOUNTS_LEN] = keys.into();
    let data: SetPoolMaxNotionalSwapIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn set_pool_max_notional_swap_ix(
    keys: SetPoolMaxNotionalSwapKeys,
    args: SetPoolMaxNotionalSwapIxArgs,
) -> std::io::Result<Instruction> {
    set_pool_max_notional_swap_ix_with_program_id(WOOFI_PROGRAM_ID, keys, args)
}
pub fn set_pool_max_notional_swap_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SetPoolMaxNotionalSwapAccounts<'_, '_>,
    args: SetPoolMaxNotionalSwapIxArgs,
) -> ProgramResult {
    let keys: SetPoolMaxNotionalSwapKeys = accounts.into();
    let ix = set_pool_max_notional_swap_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn set_pool_max_notional_swap_invoke(
    accounts: SetPoolMaxNotionalSwapAccounts<'_, '_>,
    args: SetPoolMaxNotionalSwapIxArgs,
) -> ProgramResult {
    set_pool_max_notional_swap_invoke_with_program_id(WOOFI_PROGRAM_ID, accounts, args)
}
pub fn set_pool_max_notional_swap_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SetPoolMaxNotionalSwapAccounts<'_, '_>,
    args: SetPoolMaxNotionalSwapIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SetPoolMaxNotionalSwapKeys = accounts.into();
    let ix = set_pool_max_notional_swap_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn set_pool_max_notional_swap_invoke_signed(
    accounts: SetPoolMaxNotionalSwapAccounts<'_, '_>,
    args: SetPoolMaxNotionalSwapIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    set_pool_max_notional_swap_invoke_signed_with_program_id(
        WOOFI_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn set_pool_max_notional_swap_verify_account_keys(
    accounts: SetPoolMaxNotionalSwapAccounts<'_, '_>,
    keys: SetPoolMaxNotionalSwapKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.wooconfig.key, keys.wooconfig),
        (*accounts.woopool.key, keys.woopool),
        (*accounts.authority.key, keys.authority),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn set_pool_max_notional_swap_verify_writable_privileges<'me, 'info>(
    accounts: SetPoolMaxNotionalSwapAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.woopool] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn set_pool_max_notional_swap_verify_signer_privileges<'me, 'info>(
    accounts: SetPoolMaxNotionalSwapAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn set_pool_max_notional_swap_verify_account_privileges<'me, 'info>(
    accounts: SetPoolMaxNotionalSwapAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    set_pool_max_notional_swap_verify_writable_privileges(accounts)?;
    set_pool_max_notional_swap_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SET_POOL_CAP_BAL_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct SetPoolCapBalAccounts<'me, 'info> {
    pub wooconfig: &'me AccountInfo<'info>,
    pub woopool: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SetPoolCapBalKeys {
    pub wooconfig: Pubkey,
    pub woopool: Pubkey,
    pub authority: Pubkey,
}
impl From<SetPoolCapBalAccounts<'_, '_>> for SetPoolCapBalKeys {
    fn from(accounts: SetPoolCapBalAccounts) -> Self {
        Self {
            wooconfig: *accounts.wooconfig.key,
            woopool: *accounts.woopool.key,
            authority: *accounts.authority.key,
        }
    }
}
impl From<SetPoolCapBalKeys> for [AccountMeta; SET_POOL_CAP_BAL_IX_ACCOUNTS_LEN] {
    fn from(keys: SetPoolCapBalKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.wooconfig,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.woopool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.authority,
                is_signer: true,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; SET_POOL_CAP_BAL_IX_ACCOUNTS_LEN]> for SetPoolCapBalKeys {
    fn from(pubkeys: [Pubkey; SET_POOL_CAP_BAL_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            wooconfig: pubkeys[0],
            woopool: pubkeys[1],
            authority: pubkeys[2],
        }
    }
}
impl<'info> From<SetPoolCapBalAccounts<'_, 'info>>
for [AccountInfo<'info>; SET_POOL_CAP_BAL_IX_ACCOUNTS_LEN] {
    fn from(accounts: SetPoolCapBalAccounts<'_, 'info>) -> Self {
        [
            accounts.wooconfig.clone(),
            accounts.woopool.clone(),
            accounts.authority.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SET_POOL_CAP_BAL_IX_ACCOUNTS_LEN]>
for SetPoolCapBalAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; SET_POOL_CAP_BAL_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            wooconfig: &arr[0],
            woopool: &arr[1],
            authority: &arr[2],
        }
    }
}
pub const SET_POOL_CAP_BAL_IX_DISCM: [u8; 8usize] = [
    57, 148, 133, 68, 253, 224, 251, 93,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SetPoolCapBalIxArgs {
    pub cap_bal: u128,
}
#[derive(Clone, Debug, PartialEq)]
pub struct SetPoolCapBalIxData(pub SetPoolCapBalIxArgs);
impl From<SetPoolCapBalIxArgs> for SetPoolCapBalIxData {
    fn from(args: SetPoolCapBalIxArgs) -> Self {
        Self(args)
    }
}
impl SetPoolCapBalIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SET_POOL_CAP_BAL_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let cap_bal: u128 = crate::borsh_de_or_default(&mut reader)?;
        Ok(Self(SetPoolCapBalIxArgs { cap_bal }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SET_POOL_CAP_BAL_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.cap_bal, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn set_pool_cap_bal_ix_with_program_id(
    program_id: Pubkey,
    keys: SetPoolCapBalKeys,
    args: SetPoolCapBalIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SET_POOL_CAP_BAL_IX_ACCOUNTS_LEN] = keys.into();
    let data: SetPoolCapBalIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn set_pool_cap_bal_ix(
    keys: SetPoolCapBalKeys,
    args: SetPoolCapBalIxArgs,
) -> std::io::Result<Instruction> {
    set_pool_cap_bal_ix_with_program_id(WOOFI_PROGRAM_ID, keys, args)
}
pub fn set_pool_cap_bal_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SetPoolCapBalAccounts<'_, '_>,
    args: SetPoolCapBalIxArgs,
) -> ProgramResult {
    let keys: SetPoolCapBalKeys = accounts.into();
    let ix = set_pool_cap_bal_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn set_pool_cap_bal_invoke(
    accounts: SetPoolCapBalAccounts<'_, '_>,
    args: SetPoolCapBalIxArgs,
) -> ProgramResult {
    set_pool_cap_bal_invoke_with_program_id(WOOFI_PROGRAM_ID, accounts, args)
}
pub fn set_pool_cap_bal_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SetPoolCapBalAccounts<'_, '_>,
    args: SetPoolCapBalIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SetPoolCapBalKeys = accounts.into();
    let ix = set_pool_cap_bal_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn set_pool_cap_bal_invoke_signed(
    accounts: SetPoolCapBalAccounts<'_, '_>,
    args: SetPoolCapBalIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    set_pool_cap_bal_invoke_signed_with_program_id(
        WOOFI_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn set_pool_cap_bal_verify_account_keys(
    accounts: SetPoolCapBalAccounts<'_, '_>,
    keys: SetPoolCapBalKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.wooconfig.key, keys.wooconfig),
        (*accounts.woopool.key, keys.woopool),
        (*accounts.authority.key, keys.authority),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn set_pool_cap_bal_verify_writable_privileges<'me, 'info>(
    accounts: SetPoolCapBalAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.woopool] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn set_pool_cap_bal_verify_signer_privileges<'me, 'info>(
    accounts: SetPoolCapBalAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn set_pool_cap_bal_verify_account_privileges<'me, 'info>(
    accounts: SetPoolCapBalAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    set_pool_cap_bal_verify_writable_privileges(accounts)?;
    set_pool_cap_bal_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SET_POOL_MIN_SWAP_AMOUNT_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct SetPoolMinSwapAmountAccounts<'me, 'info> {
    pub wooconfig: &'me AccountInfo<'info>,
    pub woopool: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SetPoolMinSwapAmountKeys {
    pub wooconfig: Pubkey,
    pub woopool: Pubkey,
    pub authority: Pubkey,
}
impl From<SetPoolMinSwapAmountAccounts<'_, '_>> for SetPoolMinSwapAmountKeys {
    fn from(accounts: SetPoolMinSwapAmountAccounts) -> Self {
        Self {
            wooconfig: *accounts.wooconfig.key,
            woopool: *accounts.woopool.key,
            authority: *accounts.authority.key,
        }
    }
}
impl From<SetPoolMinSwapAmountKeys>
for [AccountMeta; SET_POOL_MIN_SWAP_AMOUNT_IX_ACCOUNTS_LEN] {
    fn from(keys: SetPoolMinSwapAmountKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.wooconfig,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.woopool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.authority,
                is_signer: true,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; SET_POOL_MIN_SWAP_AMOUNT_IX_ACCOUNTS_LEN]>
for SetPoolMinSwapAmountKeys {
    fn from(pubkeys: [Pubkey; SET_POOL_MIN_SWAP_AMOUNT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            wooconfig: pubkeys[0],
            woopool: pubkeys[1],
            authority: pubkeys[2],
        }
    }
}
impl<'info> From<SetPoolMinSwapAmountAccounts<'_, 'info>>
for [AccountInfo<'info>; SET_POOL_MIN_SWAP_AMOUNT_IX_ACCOUNTS_LEN] {
    fn from(accounts: SetPoolMinSwapAmountAccounts<'_, 'info>) -> Self {
        [
            accounts.wooconfig.clone(),
            accounts.woopool.clone(),
            accounts.authority.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; SET_POOL_MIN_SWAP_AMOUNT_IX_ACCOUNTS_LEN]>
for SetPoolMinSwapAmountAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; SET_POOL_MIN_SWAP_AMOUNT_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            wooconfig: &arr[0],
            woopool: &arr[1],
            authority: &arr[2],
        }
    }
}
pub const SET_POOL_MIN_SWAP_AMOUNT_IX_DISCM: [u8; 8usize] = [
    137, 75, 238, 195, 163, 17, 73, 23,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SetPoolMinSwapAmountIxArgs {
    pub min_swap_amount: u128,
}
#[derive(Clone, Debug, PartialEq)]
pub struct SetPoolMinSwapAmountIxData(pub SetPoolMinSwapAmountIxArgs);
impl From<SetPoolMinSwapAmountIxArgs> for SetPoolMinSwapAmountIxData {
    fn from(args: SetPoolMinSwapAmountIxArgs) -> Self {
        Self(args)
    }
}
impl SetPoolMinSwapAmountIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SET_POOL_MIN_SWAP_AMOUNT_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let min_swap_amount: u128 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(SetPoolMinSwapAmountIxArgs {
                min_swap_amount,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SET_POOL_MIN_SWAP_AMOUNT_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.min_swap_amount, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn set_pool_min_swap_amount_ix_with_program_id(
    program_id: Pubkey,
    keys: SetPoolMinSwapAmountKeys,
    args: SetPoolMinSwapAmountIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SET_POOL_MIN_SWAP_AMOUNT_IX_ACCOUNTS_LEN] = keys.into();
    let data: SetPoolMinSwapAmountIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn set_pool_min_swap_amount_ix(
    keys: SetPoolMinSwapAmountKeys,
    args: SetPoolMinSwapAmountIxArgs,
) -> std::io::Result<Instruction> {
    set_pool_min_swap_amount_ix_with_program_id(WOOFI_PROGRAM_ID, keys, args)
}
pub fn set_pool_min_swap_amount_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SetPoolMinSwapAmountAccounts<'_, '_>,
    args: SetPoolMinSwapAmountIxArgs,
) -> ProgramResult {
    let keys: SetPoolMinSwapAmountKeys = accounts.into();
    let ix = set_pool_min_swap_amount_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn set_pool_min_swap_amount_invoke(
    accounts: SetPoolMinSwapAmountAccounts<'_, '_>,
    args: SetPoolMinSwapAmountIxArgs,
) -> ProgramResult {
    set_pool_min_swap_amount_invoke_with_program_id(WOOFI_PROGRAM_ID, accounts, args)
}
pub fn set_pool_min_swap_amount_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SetPoolMinSwapAmountAccounts<'_, '_>,
    args: SetPoolMinSwapAmountIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SetPoolMinSwapAmountKeys = accounts.into();
    let ix = set_pool_min_swap_amount_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn set_pool_min_swap_amount_invoke_signed(
    accounts: SetPoolMinSwapAmountAccounts<'_, '_>,
    args: SetPoolMinSwapAmountIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    set_pool_min_swap_amount_invoke_signed_with_program_id(
        WOOFI_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn set_pool_min_swap_amount_verify_account_keys(
    accounts: SetPoolMinSwapAmountAccounts<'_, '_>,
    keys: SetPoolMinSwapAmountKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.wooconfig.key, keys.wooconfig),
        (*accounts.woopool.key, keys.woopool),
        (*accounts.authority.key, keys.authority),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn set_pool_min_swap_amount_verify_writable_privileges<'me, 'info>(
    accounts: SetPoolMinSwapAmountAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.woopool] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn set_pool_min_swap_amount_verify_signer_privileges<'me, 'info>(
    accounts: SetPoolMinSwapAmountAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn set_pool_min_swap_amount_verify_account_privileges<'me, 'info>(
    accounts: SetPoolMinSwapAmountAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    set_pool_min_swap_amount_verify_writable_privileges(accounts)?;
    set_pool_min_swap_amount_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const TRY_QUERY_IX_ACCOUNTS_LEN: usize = 8;
#[derive(Copy, Clone, Debug)]
pub struct TryQueryAccounts<'me, 'info> {
    pub wooconfig: &'me AccountInfo<'info>,
    pub wooracle_from: &'me AccountInfo<'info>,
    pub woopool_from: &'me AccountInfo<'info>,
    pub price_update_from: &'me AccountInfo<'info>,
    pub wooracle_to: &'me AccountInfo<'info>,
    pub woopool_to: &'me AccountInfo<'info>,
    pub price_update_to: &'me AccountInfo<'info>,
    pub quote_price_update: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct TryQueryKeys {
    pub wooconfig: Pubkey,
    pub wooracle_from: Pubkey,
    pub woopool_from: Pubkey,
    pub price_update_from: Pubkey,
    pub wooracle_to: Pubkey,
    pub woopool_to: Pubkey,
    pub price_update_to: Pubkey,
    pub quote_price_update: Pubkey,
}
impl From<TryQueryAccounts<'_, '_>> for TryQueryKeys {
    fn from(accounts: TryQueryAccounts) -> Self {
        Self {
            wooconfig: *accounts.wooconfig.key,
            wooracle_from: *accounts.wooracle_from.key,
            woopool_from: *accounts.woopool_from.key,
            price_update_from: *accounts.price_update_from.key,
            wooracle_to: *accounts.wooracle_to.key,
            woopool_to: *accounts.woopool_to.key,
            price_update_to: *accounts.price_update_to.key,
            quote_price_update: *accounts.quote_price_update.key,
        }
    }
}
impl From<TryQueryKeys> for [AccountMeta; TRY_QUERY_IX_ACCOUNTS_LEN] {
    fn from(keys: TryQueryKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.wooconfig,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.wooracle_from,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.woopool_from,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.price_update_from,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.wooracle_to,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.woopool_to,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.price_update_to,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.quote_price_update,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; TRY_QUERY_IX_ACCOUNTS_LEN]> for TryQueryKeys {
    fn from(pubkeys: [Pubkey; TRY_QUERY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            wooconfig: pubkeys[0],
            wooracle_from: pubkeys[1],
            woopool_from: pubkeys[2],
            price_update_from: pubkeys[3],
            wooracle_to: pubkeys[4],
            woopool_to: pubkeys[5],
            price_update_to: pubkeys[6],
            quote_price_update: pubkeys[7],
        }
    }
}
impl<'info> From<TryQueryAccounts<'_, 'info>>
for [AccountInfo<'info>; TRY_QUERY_IX_ACCOUNTS_LEN] {
    fn from(accounts: TryQueryAccounts<'_, 'info>) -> Self {
        [
            accounts.wooconfig.clone(),
            accounts.wooracle_from.clone(),
            accounts.woopool_from.clone(),
            accounts.price_update_from.clone(),
            accounts.wooracle_to.clone(),
            accounts.woopool_to.clone(),
            accounts.price_update_to.clone(),
            accounts.quote_price_update.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; TRY_QUERY_IX_ACCOUNTS_LEN]>
for TryQueryAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; TRY_QUERY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            wooconfig: &arr[0],
            wooracle_from: &arr[1],
            woopool_from: &arr[2],
            price_update_from: &arr[3],
            wooracle_to: &arr[4],
            woopool_to: &arr[5],
            price_update_to: &arr[6],
            quote_price_update: &arr[7],
        }
    }
}
pub const TRY_QUERY_IX_DISCM: [u8; 8usize] = [132, 54, 250, 71, 244, 207, 193, 163];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TryQueryIxArgs {
    pub from_amount: u128,
}
#[derive(Clone, Debug, PartialEq)]
pub struct TryQueryIxData(pub TryQueryIxArgs);
impl From<TryQueryIxArgs> for TryQueryIxData {
    fn from(args: TryQueryIxArgs) -> Self {
        Self(args)
    }
}
impl TryQueryIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != TRY_QUERY_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let from_amount: u128 = crate::borsh_de_or_default(&mut reader)?;
        Ok(Self(TryQueryIxArgs { from_amount }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&TRY_QUERY_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.from_amount, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn try_query_ix_with_program_id(
    program_id: Pubkey,
    keys: TryQueryKeys,
    args: TryQueryIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; TRY_QUERY_IX_ACCOUNTS_LEN] = keys.into();
    let data: TryQueryIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn try_query_ix(
    keys: TryQueryKeys,
    args: TryQueryIxArgs,
) -> std::io::Result<Instruction> {
    try_query_ix_with_program_id(WOOFI_PROGRAM_ID, keys, args)
}
pub fn try_query_invoke_with_program_id(
    program_id: Pubkey,
    accounts: TryQueryAccounts<'_, '_>,
    args: TryQueryIxArgs,
) -> ProgramResult {
    let keys: TryQueryKeys = accounts.into();
    let ix = try_query_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn try_query_invoke(
    accounts: TryQueryAccounts<'_, '_>,
    args: TryQueryIxArgs,
) -> ProgramResult {
    try_query_invoke_with_program_id(WOOFI_PROGRAM_ID, accounts, args)
}
pub fn try_query_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: TryQueryAccounts<'_, '_>,
    args: TryQueryIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: TryQueryKeys = accounts.into();
    let ix = try_query_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn try_query_invoke_signed(
    accounts: TryQueryAccounts<'_, '_>,
    args: TryQueryIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    try_query_invoke_signed_with_program_id(WOOFI_PROGRAM_ID, accounts, args, seeds)
}
pub fn try_query_verify_account_keys(
    accounts: TryQueryAccounts<'_, '_>,
    keys: TryQueryKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.wooconfig.key, keys.wooconfig),
        (*accounts.wooracle_from.key, keys.wooracle_from),
        (*accounts.woopool_from.key, keys.woopool_from),
        (*accounts.price_update_from.key, keys.price_update_from),
        (*accounts.wooracle_to.key, keys.wooracle_to),
        (*accounts.woopool_to.key, keys.woopool_to),
        (*accounts.price_update_to.key, keys.price_update_to),
        (*accounts.quote_price_update.key, keys.quote_price_update),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn try_query_verify_writable_privileges<'me, 'info>(
    accounts: TryQueryAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.price_update_from, accounts.price_update_to] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn try_query_verify_account_privileges<'me, 'info>(
    accounts: TryQueryAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    try_query_verify_writable_privileges(accounts)?;
    Ok(())
}
pub const QUERY_IX_ACCOUNTS_LEN: usize = 12;
#[derive(Copy, Clone, Debug)]
pub struct QueryAccounts<'me, 'info> {
    pub wooconfig: &'me AccountInfo<'info>,
    pub wooracle_from: &'me AccountInfo<'info>,
    pub woopool_from: &'me AccountInfo<'info>,
    pub token_vault_from: &'me AccountInfo<'info>,
    pub price_update_from: &'me AccountInfo<'info>,
    pub wooracle_to: &'me AccountInfo<'info>,
    pub woopool_to: &'me AccountInfo<'info>,
    pub token_vault_to: &'me AccountInfo<'info>,
    pub price_update_to: &'me AccountInfo<'info>,
    pub woopool_quote: &'me AccountInfo<'info>,
    pub quote_price_update: &'me AccountInfo<'info>,
    pub quote_token_vault: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct QueryKeys {
    pub wooconfig: Pubkey,
    pub wooracle_from: Pubkey,
    pub woopool_from: Pubkey,
    pub token_vault_from: Pubkey,
    pub price_update_from: Pubkey,
    pub wooracle_to: Pubkey,
    pub woopool_to: Pubkey,
    pub token_vault_to: Pubkey,
    pub price_update_to: Pubkey,
    pub woopool_quote: Pubkey,
    pub quote_price_update: Pubkey,
    pub quote_token_vault: Pubkey,
}
impl From<QueryAccounts<'_, '_>> for QueryKeys {
    fn from(accounts: QueryAccounts) -> Self {
        Self {
            wooconfig: *accounts.wooconfig.key,
            wooracle_from: *accounts.wooracle_from.key,
            woopool_from: *accounts.woopool_from.key,
            token_vault_from: *accounts.token_vault_from.key,
            price_update_from: *accounts.price_update_from.key,
            wooracle_to: *accounts.wooracle_to.key,
            woopool_to: *accounts.woopool_to.key,
            token_vault_to: *accounts.token_vault_to.key,
            price_update_to: *accounts.price_update_to.key,
            woopool_quote: *accounts.woopool_quote.key,
            quote_price_update: *accounts.quote_price_update.key,
            quote_token_vault: *accounts.quote_token_vault.key,
        }
    }
}
impl From<QueryKeys> for [AccountMeta; QUERY_IX_ACCOUNTS_LEN] {
    fn from(keys: QueryKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.wooconfig,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.wooracle_from,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.woopool_from,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_vault_from,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.price_update_from,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.wooracle_to,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.woopool_to,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_vault_to,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.price_update_to,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.woopool_quote,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.quote_price_update,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.quote_token_vault,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; QUERY_IX_ACCOUNTS_LEN]> for QueryKeys {
    fn from(pubkeys: [Pubkey; QUERY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            wooconfig: pubkeys[0],
            wooracle_from: pubkeys[1],
            woopool_from: pubkeys[2],
            token_vault_from: pubkeys[3],
            price_update_from: pubkeys[4],
            wooracle_to: pubkeys[5],
            woopool_to: pubkeys[6],
            token_vault_to: pubkeys[7],
            price_update_to: pubkeys[8],
            woopool_quote: pubkeys[9],
            quote_price_update: pubkeys[10],
            quote_token_vault: pubkeys[11],
        }
    }
}
impl<'info> From<QueryAccounts<'_, 'info>>
for [AccountInfo<'info>; QUERY_IX_ACCOUNTS_LEN] {
    fn from(accounts: QueryAccounts<'_, 'info>) -> Self {
        [
            accounts.wooconfig.clone(),
            accounts.wooracle_from.clone(),
            accounts.woopool_from.clone(),
            accounts.token_vault_from.clone(),
            accounts.price_update_from.clone(),
            accounts.wooracle_to.clone(),
            accounts.woopool_to.clone(),
            accounts.token_vault_to.clone(),
            accounts.price_update_to.clone(),
            accounts.woopool_quote.clone(),
            accounts.quote_price_update.clone(),
            accounts.quote_token_vault.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; QUERY_IX_ACCOUNTS_LEN]>
for QueryAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; QUERY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            wooconfig: &arr[0],
            wooracle_from: &arr[1],
            woopool_from: &arr[2],
            token_vault_from: &arr[3],
            price_update_from: &arr[4],
            wooracle_to: &arr[5],
            woopool_to: &arr[6],
            token_vault_to: &arr[7],
            price_update_to: &arr[8],
            woopool_quote: &arr[9],
            quote_price_update: &arr[10],
            quote_token_vault: &arr[11],
        }
    }
}
pub const QUERY_IX_DISCM: [u8; 8usize] = [39, 251, 130, 159, 46, 136, 164, 169];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct QueryIxArgs {
    pub from_amount: u128,
    pub min_to_amount: u128,
}
#[derive(Clone, Debug, PartialEq)]
pub struct QueryIxData(pub QueryIxArgs);
impl From<QueryIxArgs> for QueryIxData {
    fn from(args: QueryIxArgs) -> Self {
        Self(args)
    }
}
impl QueryIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != QUERY_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let from_amount: u128 = crate::borsh_de_or_default(&mut reader)?;
        let min_to_amount: u128 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(QueryIxArgs {
                from_amount,
                min_to_amount,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&QUERY_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.from_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.min_to_amount, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn query_ix_with_program_id(
    program_id: Pubkey,
    keys: QueryKeys,
    args: QueryIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; QUERY_IX_ACCOUNTS_LEN] = keys.into();
    let data: QueryIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn query_ix(keys: QueryKeys, args: QueryIxArgs) -> std::io::Result<Instruction> {
    query_ix_with_program_id(WOOFI_PROGRAM_ID, keys, args)
}
pub fn query_invoke_with_program_id(
    program_id: Pubkey,
    accounts: QueryAccounts<'_, '_>,
    args: QueryIxArgs,
) -> ProgramResult {
    let keys: QueryKeys = accounts.into();
    let ix = query_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn query_invoke(
    accounts: QueryAccounts<'_, '_>,
    args: QueryIxArgs,
) -> ProgramResult {
    query_invoke_with_program_id(WOOFI_PROGRAM_ID, accounts, args)
}
pub fn query_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: QueryAccounts<'_, '_>,
    args: QueryIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: QueryKeys = accounts.into();
    let ix = query_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn query_invoke_signed(
    accounts: QueryAccounts<'_, '_>,
    args: QueryIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    query_invoke_signed_with_program_id(WOOFI_PROGRAM_ID, accounts, args, seeds)
}
pub fn query_verify_account_keys(
    accounts: QueryAccounts<'_, '_>,
    keys: QueryKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.wooconfig.key, keys.wooconfig),
        (*accounts.wooracle_from.key, keys.wooracle_from),
        (*accounts.woopool_from.key, keys.woopool_from),
        (*accounts.token_vault_from.key, keys.token_vault_from),
        (*accounts.price_update_from.key, keys.price_update_from),
        (*accounts.wooracle_to.key, keys.wooracle_to),
        (*accounts.woopool_to.key, keys.woopool_to),
        (*accounts.token_vault_to.key, keys.token_vault_to),
        (*accounts.price_update_to.key, keys.price_update_to),
        (*accounts.woopool_quote.key, keys.woopool_quote),
        (*accounts.quote_price_update.key, keys.quote_price_update),
        (*accounts.quote_token_vault.key, keys.quote_token_vault),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn query_verify_writable_privileges<'me, 'info>(
    accounts: QueryAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.token_vault_from,
        accounts.price_update_from,
        accounts.price_update_to,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn query_verify_account_privileges<'me, 'info>(
    accounts: QueryAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    query_verify_writable_privileges(accounts)?;
    Ok(())
}
pub const SWAP_IX_ACCOUNTS_LEN: usize = 17;
#[derive(Copy, Clone, Debug)]
pub struct SwapAccounts<'me, 'info> {
    pub wooconfig: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub payer: &'me AccountInfo<'info>,
    pub wooracle_from: &'me AccountInfo<'info>,
    pub woopool_from: &'me AccountInfo<'info>,
    pub token_owner_account_from: &'me AccountInfo<'info>,
    pub token_vault_from: &'me AccountInfo<'info>,
    pub price_update_from: &'me AccountInfo<'info>,
    pub wooracle_to: &'me AccountInfo<'info>,
    pub woopool_to: &'me AccountInfo<'info>,
    pub token_owner_account_to: &'me AccountInfo<'info>,
    pub token_vault_to: &'me AccountInfo<'info>,
    pub price_update_to: &'me AccountInfo<'info>,
    pub woopool_quote: &'me AccountInfo<'info>,
    pub quote_price_update: &'me AccountInfo<'info>,
    pub quote_token_vault: &'me AccountInfo<'info>,
    pub rebate_to: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SwapKeys {
    pub wooconfig: Pubkey,
    pub token_program: Pubkey,
    pub payer: Pubkey,
    pub wooracle_from: Pubkey,
    pub woopool_from: Pubkey,
    pub token_owner_account_from: Pubkey,
    pub token_vault_from: Pubkey,
    pub price_update_from: Pubkey,
    pub wooracle_to: Pubkey,
    pub woopool_to: Pubkey,
    pub token_owner_account_to: Pubkey,
    pub token_vault_to: Pubkey,
    pub price_update_to: Pubkey,
    pub woopool_quote: Pubkey,
    pub quote_price_update: Pubkey,
    pub quote_token_vault: Pubkey,
    pub rebate_to: Pubkey,
}
impl From<SwapAccounts<'_, '_>> for SwapKeys {
    fn from(accounts: SwapAccounts) -> Self {
        Self {
            wooconfig: *accounts.wooconfig.key,
            token_program: *accounts.token_program.key,
            payer: *accounts.payer.key,
            wooracle_from: *accounts.wooracle_from.key,
            woopool_from: *accounts.woopool_from.key,
            token_owner_account_from: *accounts.token_owner_account_from.key,
            token_vault_from: *accounts.token_vault_from.key,
            price_update_from: *accounts.price_update_from.key,
            wooracle_to: *accounts.wooracle_to.key,
            woopool_to: *accounts.woopool_to.key,
            token_owner_account_to: *accounts.token_owner_account_to.key,
            token_vault_to: *accounts.token_vault_to.key,
            price_update_to: *accounts.price_update_to.key,
            woopool_quote: *accounts.woopool_quote.key,
            quote_price_update: *accounts.quote_price_update.key,
            quote_token_vault: *accounts.quote_token_vault.key,
            rebate_to: *accounts.rebate_to.key,
        }
    }
}
impl From<SwapKeys> for [AccountMeta; SWAP_IX_ACCOUNTS_LEN] {
    fn from(keys: SwapKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.wooconfig,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.payer,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.wooracle_from,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.woopool_from,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_owner_account_from,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_vault_from,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.price_update_from,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.wooracle_to,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.woopool_to,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_owner_account_to,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_vault_to,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.price_update_to,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.woopool_quote,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.quote_price_update,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.quote_token_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.rebate_to,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; SWAP_IX_ACCOUNTS_LEN]> for SwapKeys {
    fn from(pubkeys: [Pubkey; SWAP_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            wooconfig: pubkeys[0],
            token_program: pubkeys[1],
            payer: pubkeys[2],
            wooracle_from: pubkeys[3],
            woopool_from: pubkeys[4],
            token_owner_account_from: pubkeys[5],
            token_vault_from: pubkeys[6],
            price_update_from: pubkeys[7],
            wooracle_to: pubkeys[8],
            woopool_to: pubkeys[9],
            token_owner_account_to: pubkeys[10],
            token_vault_to: pubkeys[11],
            price_update_to: pubkeys[12],
            woopool_quote: pubkeys[13],
            quote_price_update: pubkeys[14],
            quote_token_vault: pubkeys[15],
            rebate_to: pubkeys[16],
        }
    }
}
impl<'info> From<SwapAccounts<'_, 'info>>
for [AccountInfo<'info>; SWAP_IX_ACCOUNTS_LEN] {
    fn from(accounts: SwapAccounts<'_, 'info>) -> Self {
        [
            accounts.wooconfig.clone(),
            accounts.token_program.clone(),
            accounts.payer.clone(),
            accounts.wooracle_from.clone(),
            accounts.woopool_from.clone(),
            accounts.token_owner_account_from.clone(),
            accounts.token_vault_from.clone(),
            accounts.price_update_from.clone(),
            accounts.wooracle_to.clone(),
            accounts.woopool_to.clone(),
            accounts.token_owner_account_to.clone(),
            accounts.token_vault_to.clone(),
            accounts.price_update_to.clone(),
            accounts.woopool_quote.clone(),
            accounts.quote_price_update.clone(),
            accounts.quote_token_vault.clone(),
            accounts.rebate_to.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SWAP_IX_ACCOUNTS_LEN]>
for SwapAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; SWAP_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            wooconfig: &arr[0],
            token_program: &arr[1],
            payer: &arr[2],
            wooracle_from: &arr[3],
            woopool_from: &arr[4],
            token_owner_account_from: &arr[5],
            token_vault_from: &arr[6],
            price_update_from: &arr[7],
            wooracle_to: &arr[8],
            woopool_to: &arr[9],
            token_owner_account_to: &arr[10],
            token_vault_to: &arr[11],
            price_update_to: &arr[12],
            woopool_quote: &arr[13],
            quote_price_update: &arr[14],
            quote_token_vault: &arr[15],
            rebate_to: &arr[16],
        }
    }
}
pub const SWAP_IX_DISCM: [u8; 8usize] = [248, 198, 158, 145, 225, 117, 135, 200];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SwapIxArgs {
    pub from_amount: u128,
    pub min_to_amount: u128,
}
#[derive(Clone, Debug, PartialEq)]
pub struct SwapIxData(pub SwapIxArgs);
impl From<SwapIxArgs> for SwapIxData {
    fn from(args: SwapIxArgs) -> Self {
        Self(args)
    }
}
impl SwapIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SWAP_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let from_amount: u128 = crate::borsh_de_or_default(&mut reader)?;
        let min_to_amount: u128 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(SwapIxArgs {
                from_amount,
                min_to_amount,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SWAP_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.from_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.min_to_amount, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn swap_ix_with_program_id(
    program_id: Pubkey,
    keys: SwapKeys,
    args: SwapIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SWAP_IX_ACCOUNTS_LEN] = keys.into();
    let data: SwapIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn swap_ix(keys: SwapKeys, args: SwapIxArgs) -> std::io::Result<Instruction> {
    swap_ix_with_program_id(WOOFI_PROGRAM_ID, keys, args)
}
pub fn swap_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SwapAccounts<'_, '_>,
    args: SwapIxArgs,
) -> ProgramResult {
    let keys: SwapKeys = accounts.into();
    let ix = swap_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn swap_invoke(accounts: SwapAccounts<'_, '_>, args: SwapIxArgs) -> ProgramResult {
    swap_invoke_with_program_id(WOOFI_PROGRAM_ID, accounts, args)
}
pub fn swap_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SwapAccounts<'_, '_>,
    args: SwapIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SwapKeys = accounts.into();
    let ix = swap_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn swap_invoke_signed(
    accounts: SwapAccounts<'_, '_>,
    args: SwapIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    swap_invoke_signed_with_program_id(WOOFI_PROGRAM_ID, accounts, args, seeds)
}
pub fn swap_verify_account_keys(
    accounts: SwapAccounts<'_, '_>,
    keys: SwapKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.wooconfig.key, keys.wooconfig),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.payer.key, keys.payer),
        (*accounts.wooracle_from.key, keys.wooracle_from),
        (*accounts.woopool_from.key, keys.woopool_from),
        (*accounts.token_owner_account_from.key, keys.token_owner_account_from),
        (*accounts.token_vault_from.key, keys.token_vault_from),
        (*accounts.price_update_from.key, keys.price_update_from),
        (*accounts.wooracle_to.key, keys.wooracle_to),
        (*accounts.woopool_to.key, keys.woopool_to),
        (*accounts.token_owner_account_to.key, keys.token_owner_account_to),
        (*accounts.token_vault_to.key, keys.token_vault_to),
        (*accounts.price_update_to.key, keys.price_update_to),
        (*accounts.woopool_quote.key, keys.woopool_quote),
        (*accounts.quote_price_update.key, keys.quote_price_update),
        (*accounts.quote_token_vault.key, keys.quote_token_vault),
        (*accounts.rebate_to.key, keys.rebate_to),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn swap_verify_writable_privileges<'me, 'info>(
    accounts: SwapAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.wooracle_from,
        accounts.woopool_from,
        accounts.token_owner_account_from,
        accounts.token_vault_from,
        accounts.price_update_from,
        accounts.wooracle_to,
        accounts.woopool_to,
        accounts.token_owner_account_to,
        accounts.token_vault_to,
        accounts.price_update_to,
        accounts.woopool_quote,
        accounts.quote_token_vault,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn swap_verify_signer_privileges<'me, 'info>(
    accounts: SwapAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.payer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn swap_verify_account_privileges<'me, 'info>(
    accounts: SwapAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    swap_verify_writable_privileges(accounts)?;
    swap_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const DEPOSIT_IX_ACCOUNTS_LEN: usize = 7;
#[derive(Copy, Clone, Debug)]
pub struct DepositAccounts<'me, 'info> {
    pub wooconfig: &'me AccountInfo<'info>,
    pub token_mint: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub token_owner_account: &'me AccountInfo<'info>,
    pub woopool: &'me AccountInfo<'info>,
    pub token_vault: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct DepositKeys {
    pub wooconfig: Pubkey,
    pub token_mint: Pubkey,
    pub authority: Pubkey,
    pub token_owner_account: Pubkey,
    pub woopool: Pubkey,
    pub token_vault: Pubkey,
    pub token_program: Pubkey,
}
impl From<DepositAccounts<'_, '_>> for DepositKeys {
    fn from(accounts: DepositAccounts) -> Self {
        Self {
            wooconfig: *accounts.wooconfig.key,
            token_mint: *accounts.token_mint.key,
            authority: *accounts.authority.key,
            token_owner_account: *accounts.token_owner_account.key,
            woopool: *accounts.woopool.key,
            token_vault: *accounts.token_vault.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<DepositKeys> for [AccountMeta; DEPOSIT_IX_ACCOUNTS_LEN] {
    fn from(keys: DepositKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.wooconfig,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.authority,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_owner_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.woopool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_vault,
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
impl From<[Pubkey; DEPOSIT_IX_ACCOUNTS_LEN]> for DepositKeys {
    fn from(pubkeys: [Pubkey; DEPOSIT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            wooconfig: pubkeys[0],
            token_mint: pubkeys[1],
            authority: pubkeys[2],
            token_owner_account: pubkeys[3],
            woopool: pubkeys[4],
            token_vault: pubkeys[5],
            token_program: pubkeys[6],
        }
    }
}
impl<'info> From<DepositAccounts<'_, 'info>>
for [AccountInfo<'info>; DEPOSIT_IX_ACCOUNTS_LEN] {
    fn from(accounts: DepositAccounts<'_, 'info>) -> Self {
        [
            accounts.wooconfig.clone(),
            accounts.token_mint.clone(),
            accounts.authority.clone(),
            accounts.token_owner_account.clone(),
            accounts.woopool.clone(),
            accounts.token_vault.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; DEPOSIT_IX_ACCOUNTS_LEN]>
for DepositAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; DEPOSIT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            wooconfig: &arr[0],
            token_mint: &arr[1],
            authority: &arr[2],
            token_owner_account: &arr[3],
            woopool: &arr[4],
            token_vault: &arr[5],
            token_program: &arr[6],
        }
    }
}
pub const DEPOSIT_IX_DISCM: [u8; 8usize] = [242, 35, 198, 137, 82, 225, 242, 182];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DepositIxArgs {
    pub amount: u128,
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
        let amount: u128 = crate::borsh_de_or_default(&mut reader)?;
        Ok(Self(DepositIxArgs { amount }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&DEPOSIT_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.amount, &mut writer)?;
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
    deposit_ix_with_program_id(WOOFI_PROGRAM_ID, keys, args)
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
    deposit_invoke_with_program_id(WOOFI_PROGRAM_ID, accounts, args)
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
    deposit_invoke_signed_with_program_id(WOOFI_PROGRAM_ID, accounts, args, seeds)
}
pub fn deposit_verify_account_keys(
    accounts: DepositAccounts<'_, '_>,
    keys: DepositKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.wooconfig.key, keys.wooconfig),
        (*accounts.token_mint.key, keys.token_mint),
        (*accounts.authority.key, keys.authority),
        (*accounts.token_owner_account.key, keys.token_owner_account),
        (*accounts.woopool.key, keys.woopool),
        (*accounts.token_vault.key, keys.token_vault),
        (*accounts.token_program.key, keys.token_program),
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
        accounts.token_owner_account,
        accounts.woopool,
        accounts.token_vault,
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
    for should_be_signer in [accounts.authority] {
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
pub const WITHDRAW_IX_ACCOUNTS_LEN: usize = 7;
#[derive(Copy, Clone, Debug)]
pub struct WithdrawAccounts<'me, 'info> {
    pub wooconfig: &'me AccountInfo<'info>,
    pub token_mint: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub to_token_account: &'me AccountInfo<'info>,
    pub woopool: &'me AccountInfo<'info>,
    pub token_vault: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct WithdrawKeys {
    pub wooconfig: Pubkey,
    pub token_mint: Pubkey,
    pub authority: Pubkey,
    pub to_token_account: Pubkey,
    pub woopool: Pubkey,
    pub token_vault: Pubkey,
    pub token_program: Pubkey,
}
impl From<WithdrawAccounts<'_, '_>> for WithdrawKeys {
    fn from(accounts: WithdrawAccounts) -> Self {
        Self {
            wooconfig: *accounts.wooconfig.key,
            token_mint: *accounts.token_mint.key,
            authority: *accounts.authority.key,
            to_token_account: *accounts.to_token_account.key,
            woopool: *accounts.woopool.key,
            token_vault: *accounts.token_vault.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<WithdrawKeys> for [AccountMeta; WITHDRAW_IX_ACCOUNTS_LEN] {
    fn from(keys: WithdrawKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.wooconfig,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.authority,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.to_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.woopool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_vault,
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
impl From<[Pubkey; WITHDRAW_IX_ACCOUNTS_LEN]> for WithdrawKeys {
    fn from(pubkeys: [Pubkey; WITHDRAW_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            wooconfig: pubkeys[0],
            token_mint: pubkeys[1],
            authority: pubkeys[2],
            to_token_account: pubkeys[3],
            woopool: pubkeys[4],
            token_vault: pubkeys[5],
            token_program: pubkeys[6],
        }
    }
}
impl<'info> From<WithdrawAccounts<'_, 'info>>
for [AccountInfo<'info>; WITHDRAW_IX_ACCOUNTS_LEN] {
    fn from(accounts: WithdrawAccounts<'_, 'info>) -> Self {
        [
            accounts.wooconfig.clone(),
            accounts.token_mint.clone(),
            accounts.authority.clone(),
            accounts.to_token_account.clone(),
            accounts.woopool.clone(),
            accounts.token_vault.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; WITHDRAW_IX_ACCOUNTS_LEN]>
for WithdrawAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; WITHDRAW_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            wooconfig: &arr[0],
            token_mint: &arr[1],
            authority: &arr[2],
            to_token_account: &arr[3],
            woopool: &arr[4],
            token_vault: &arr[5],
            token_program: &arr[6],
        }
    }
}
pub const WITHDRAW_IX_DISCM: [u8; 8usize] = [183, 18, 70, 156, 148, 109, 161, 34];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WithdrawIxArgs {
    pub amount: u128,
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
        let amount: u128 = crate::borsh_de_or_default(&mut reader)?;
        Ok(Self(WithdrawIxArgs { amount }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&WITHDRAW_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.amount, &mut writer)?;
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
    withdraw_ix_with_program_id(WOOFI_PROGRAM_ID, keys, args)
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
    withdraw_invoke_with_program_id(WOOFI_PROGRAM_ID, accounts, args)
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
    withdraw_invoke_signed_with_program_id(WOOFI_PROGRAM_ID, accounts, args, seeds)
}
pub fn withdraw_verify_account_keys(
    accounts: WithdrawAccounts<'_, '_>,
    keys: WithdrawKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.wooconfig.key, keys.wooconfig),
        (*accounts.token_mint.key, keys.token_mint),
        (*accounts.authority.key, keys.authority),
        (*accounts.to_token_account.key, keys.to_token_account),
        (*accounts.woopool.key, keys.woopool),
        (*accounts.token_vault.key, keys.token_vault),
        (*accounts.token_program.key, keys.token_program),
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
        accounts.to_token_account,
        accounts.woopool,
        accounts.token_vault,
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
    for should_be_signer in [accounts.authority] {
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
pub const CLAIM_FEE_IX_ACCOUNTS_LEN: usize = 7;
#[derive(Copy, Clone, Debug)]
pub struct ClaimFeeAccounts<'me, 'info> {
    pub wooconfig: &'me AccountInfo<'info>,
    pub token_mint: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub woopool: &'me AccountInfo<'info>,
    pub token_vault: &'me AccountInfo<'info>,
    pub claim_fee_to_account: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ClaimFeeKeys {
    pub wooconfig: Pubkey,
    pub token_mint: Pubkey,
    pub authority: Pubkey,
    pub woopool: Pubkey,
    pub token_vault: Pubkey,
    pub claim_fee_to_account: Pubkey,
    pub token_program: Pubkey,
}
impl From<ClaimFeeAccounts<'_, '_>> for ClaimFeeKeys {
    fn from(accounts: ClaimFeeAccounts) -> Self {
        Self {
            wooconfig: *accounts.wooconfig.key,
            token_mint: *accounts.token_mint.key,
            authority: *accounts.authority.key,
            woopool: *accounts.woopool.key,
            token_vault: *accounts.token_vault.key,
            claim_fee_to_account: *accounts.claim_fee_to_account.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<ClaimFeeKeys> for [AccountMeta; CLAIM_FEE_IX_ACCOUNTS_LEN] {
    fn from(keys: ClaimFeeKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.wooconfig,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.authority,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.woopool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.claim_fee_to_account,
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
impl From<[Pubkey; CLAIM_FEE_IX_ACCOUNTS_LEN]> for ClaimFeeKeys {
    fn from(pubkeys: [Pubkey; CLAIM_FEE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            wooconfig: pubkeys[0],
            token_mint: pubkeys[1],
            authority: pubkeys[2],
            woopool: pubkeys[3],
            token_vault: pubkeys[4],
            claim_fee_to_account: pubkeys[5],
            token_program: pubkeys[6],
        }
    }
}
impl<'info> From<ClaimFeeAccounts<'_, 'info>>
for [AccountInfo<'info>; CLAIM_FEE_IX_ACCOUNTS_LEN] {
    fn from(accounts: ClaimFeeAccounts<'_, 'info>) -> Self {
        [
            accounts.wooconfig.clone(),
            accounts.token_mint.clone(),
            accounts.authority.clone(),
            accounts.woopool.clone(),
            accounts.token_vault.clone(),
            accounts.claim_fee_to_account.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CLAIM_FEE_IX_ACCOUNTS_LEN]>
for ClaimFeeAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; CLAIM_FEE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            wooconfig: &arr[0],
            token_mint: &arr[1],
            authority: &arr[2],
            woopool: &arr[3],
            token_vault: &arr[4],
            claim_fee_to_account: &arr[5],
            token_program: &arr[6],
        }
    }
}
pub const CLAIM_FEE_IX_DISCM: [u8; 8usize] = [169, 32, 79, 137, 136, 232, 70, 137];
#[derive(Clone, Debug, PartialEq)]
pub struct ClaimFeeIxData;
impl ClaimFeeIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CLAIM_FEE_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CLAIM_FEE_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn claim_fee_ix_with_program_id(
    program_id: Pubkey,
    keys: ClaimFeeKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CLAIM_FEE_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: ClaimFeeIxData.try_to_vec()?,
    })
}
pub fn claim_fee_ix(keys: ClaimFeeKeys) -> std::io::Result<Instruction> {
    claim_fee_ix_with_program_id(WOOFI_PROGRAM_ID, keys)
}
pub fn claim_fee_invoke_with_program_id(
    program_id: Pubkey,
    accounts: ClaimFeeAccounts<'_, '_>,
) -> ProgramResult {
    let keys: ClaimFeeKeys = accounts.into();
    let ix = claim_fee_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn claim_fee_invoke(accounts: ClaimFeeAccounts<'_, '_>) -> ProgramResult {
    claim_fee_invoke_with_program_id(WOOFI_PROGRAM_ID, accounts)
}
pub fn claim_fee_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: ClaimFeeAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: ClaimFeeKeys = accounts.into();
    let ix = claim_fee_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn claim_fee_invoke_signed(
    accounts: ClaimFeeAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    claim_fee_invoke_signed_with_program_id(WOOFI_PROGRAM_ID, accounts, seeds)
}
pub fn claim_fee_verify_account_keys(
    accounts: ClaimFeeAccounts<'_, '_>,
    keys: ClaimFeeKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.wooconfig.key, keys.wooconfig),
        (*accounts.token_mint.key, keys.token_mint),
        (*accounts.authority.key, keys.authority),
        (*accounts.woopool.key, keys.woopool),
        (*accounts.token_vault.key, keys.token_vault),
        (*accounts.claim_fee_to_account.key, keys.claim_fee_to_account),
        (*accounts.token_program.key, keys.token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn claim_fee_verify_writable_privileges<'me, 'info>(
    accounts: ClaimFeeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.woopool,
        accounts.token_vault,
        accounts.claim_fee_to_account,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn claim_fee_verify_signer_privileges<'me, 'info>(
    accounts: ClaimFeeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn claim_fee_verify_account_privileges<'me, 'info>(
    accounts: ClaimFeeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    claim_fee_verify_writable_privileges(accounts)?;
    claim_fee_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CLAIM_FEE_AMOUNT_IX_ACCOUNTS_LEN: usize = 7;
#[derive(Copy, Clone, Debug)]
pub struct ClaimFeeAmountAccounts<'me, 'info> {
    pub wooconfig: &'me AccountInfo<'info>,
    pub token_mint: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub woopool: &'me AccountInfo<'info>,
    pub token_vault: &'me AccountInfo<'info>,
    pub claim_fee_to_account: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ClaimFeeAmountKeys {
    pub wooconfig: Pubkey,
    pub token_mint: Pubkey,
    pub authority: Pubkey,
    pub woopool: Pubkey,
    pub token_vault: Pubkey,
    pub claim_fee_to_account: Pubkey,
    pub token_program: Pubkey,
}
impl From<ClaimFeeAmountAccounts<'_, '_>> for ClaimFeeAmountKeys {
    fn from(accounts: ClaimFeeAmountAccounts) -> Self {
        Self {
            wooconfig: *accounts.wooconfig.key,
            token_mint: *accounts.token_mint.key,
            authority: *accounts.authority.key,
            woopool: *accounts.woopool.key,
            token_vault: *accounts.token_vault.key,
            claim_fee_to_account: *accounts.claim_fee_to_account.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<ClaimFeeAmountKeys> for [AccountMeta; CLAIM_FEE_AMOUNT_IX_ACCOUNTS_LEN] {
    fn from(keys: ClaimFeeAmountKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.wooconfig,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.authority,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.woopool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.claim_fee_to_account,
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
impl From<[Pubkey; CLAIM_FEE_AMOUNT_IX_ACCOUNTS_LEN]> for ClaimFeeAmountKeys {
    fn from(pubkeys: [Pubkey; CLAIM_FEE_AMOUNT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            wooconfig: pubkeys[0],
            token_mint: pubkeys[1],
            authority: pubkeys[2],
            woopool: pubkeys[3],
            token_vault: pubkeys[4],
            claim_fee_to_account: pubkeys[5],
            token_program: pubkeys[6],
        }
    }
}
impl<'info> From<ClaimFeeAmountAccounts<'_, 'info>>
for [AccountInfo<'info>; CLAIM_FEE_AMOUNT_IX_ACCOUNTS_LEN] {
    fn from(accounts: ClaimFeeAmountAccounts<'_, 'info>) -> Self {
        [
            accounts.wooconfig.clone(),
            accounts.token_mint.clone(),
            accounts.authority.clone(),
            accounts.woopool.clone(),
            accounts.token_vault.clone(),
            accounts.claim_fee_to_account.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CLAIM_FEE_AMOUNT_IX_ACCOUNTS_LEN]>
for ClaimFeeAmountAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; CLAIM_FEE_AMOUNT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            wooconfig: &arr[0],
            token_mint: &arr[1],
            authority: &arr[2],
            woopool: &arr[3],
            token_vault: &arr[4],
            claim_fee_to_account: &arr[5],
            token_program: &arr[6],
        }
    }
}
pub const CLAIM_FEE_AMOUNT_IX_DISCM: [u8; 8usize] = [50, 43, 157, 70, 143, 92, 217, 90];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ClaimFeeAmountIxArgs {
    pub claim_amount: u128,
}
#[derive(Clone, Debug, PartialEq)]
pub struct ClaimFeeAmountIxData(pub ClaimFeeAmountIxArgs);
impl From<ClaimFeeAmountIxArgs> for ClaimFeeAmountIxData {
    fn from(args: ClaimFeeAmountIxArgs) -> Self {
        Self(args)
    }
}
impl ClaimFeeAmountIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CLAIM_FEE_AMOUNT_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let claim_amount: u128 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(ClaimFeeAmountIxArgs {
                claim_amount,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CLAIM_FEE_AMOUNT_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.claim_amount, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn claim_fee_amount_ix_with_program_id(
    program_id: Pubkey,
    keys: ClaimFeeAmountKeys,
    args: ClaimFeeAmountIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CLAIM_FEE_AMOUNT_IX_ACCOUNTS_LEN] = keys.into();
    let data: ClaimFeeAmountIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn claim_fee_amount_ix(
    keys: ClaimFeeAmountKeys,
    args: ClaimFeeAmountIxArgs,
) -> std::io::Result<Instruction> {
    claim_fee_amount_ix_with_program_id(WOOFI_PROGRAM_ID, keys, args)
}
pub fn claim_fee_amount_invoke_with_program_id(
    program_id: Pubkey,
    accounts: ClaimFeeAmountAccounts<'_, '_>,
    args: ClaimFeeAmountIxArgs,
) -> ProgramResult {
    let keys: ClaimFeeAmountKeys = accounts.into();
    let ix = claim_fee_amount_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn claim_fee_amount_invoke(
    accounts: ClaimFeeAmountAccounts<'_, '_>,
    args: ClaimFeeAmountIxArgs,
) -> ProgramResult {
    claim_fee_amount_invoke_with_program_id(WOOFI_PROGRAM_ID, accounts, args)
}
pub fn claim_fee_amount_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: ClaimFeeAmountAccounts<'_, '_>,
    args: ClaimFeeAmountIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: ClaimFeeAmountKeys = accounts.into();
    let ix = claim_fee_amount_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn claim_fee_amount_invoke_signed(
    accounts: ClaimFeeAmountAccounts<'_, '_>,
    args: ClaimFeeAmountIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    claim_fee_amount_invoke_signed_with_program_id(
        WOOFI_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn claim_fee_amount_verify_account_keys(
    accounts: ClaimFeeAmountAccounts<'_, '_>,
    keys: ClaimFeeAmountKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.wooconfig.key, keys.wooconfig),
        (*accounts.token_mint.key, keys.token_mint),
        (*accounts.authority.key, keys.authority),
        (*accounts.woopool.key, keys.woopool),
        (*accounts.token_vault.key, keys.token_vault),
        (*accounts.claim_fee_to_account.key, keys.claim_fee_to_account),
        (*accounts.token_program.key, keys.token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn claim_fee_amount_verify_writable_privileges<'me, 'info>(
    accounts: ClaimFeeAmountAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.woopool,
        accounts.token_vault,
        accounts.claim_fee_to_account,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn claim_fee_amount_verify_signer_privileges<'me, 'info>(
    accounts: ClaimFeeAmountAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn claim_fee_amount_verify_account_privileges<'me, 'info>(
    accounts: ClaimFeeAmountAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    claim_fee_amount_verify_writable_privileges(accounts)?;
    claim_fee_amount_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const INCASE_TOKEN_GOT_STUCK_IX_ACCOUNTS_LEN: usize = 6;
#[derive(Copy, Clone, Debug)]
pub struct IncaseTokenGotStuckAccounts<'me, 'info> {
    pub token_mint: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub to_token_account: &'me AccountInfo<'info>,
    pub woopool: &'me AccountInfo<'info>,
    pub token_vault: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct IncaseTokenGotStuckKeys {
    pub token_mint: Pubkey,
    pub authority: Pubkey,
    pub to_token_account: Pubkey,
    pub woopool: Pubkey,
    pub token_vault: Pubkey,
    pub token_program: Pubkey,
}
impl From<IncaseTokenGotStuckAccounts<'_, '_>> for IncaseTokenGotStuckKeys {
    fn from(accounts: IncaseTokenGotStuckAccounts) -> Self {
        Self {
            token_mint: *accounts.token_mint.key,
            authority: *accounts.authority.key,
            to_token_account: *accounts.to_token_account.key,
            woopool: *accounts.woopool.key,
            token_vault: *accounts.token_vault.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<IncaseTokenGotStuckKeys>
for [AccountMeta; INCASE_TOKEN_GOT_STUCK_IX_ACCOUNTS_LEN] {
    fn from(keys: IncaseTokenGotStuckKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.token_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.authority,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.to_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.woopool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_vault,
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
impl From<[Pubkey; INCASE_TOKEN_GOT_STUCK_IX_ACCOUNTS_LEN]> for IncaseTokenGotStuckKeys {
    fn from(pubkeys: [Pubkey; INCASE_TOKEN_GOT_STUCK_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            token_mint: pubkeys[0],
            authority: pubkeys[1],
            to_token_account: pubkeys[2],
            woopool: pubkeys[3],
            token_vault: pubkeys[4],
            token_program: pubkeys[5],
        }
    }
}
impl<'info> From<IncaseTokenGotStuckAccounts<'_, 'info>>
for [AccountInfo<'info>; INCASE_TOKEN_GOT_STUCK_IX_ACCOUNTS_LEN] {
    fn from(accounts: IncaseTokenGotStuckAccounts<'_, 'info>) -> Self {
        [
            accounts.token_mint.clone(),
            accounts.authority.clone(),
            accounts.to_token_account.clone(),
            accounts.woopool.clone(),
            accounts.token_vault.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; INCASE_TOKEN_GOT_STUCK_IX_ACCOUNTS_LEN]>
for IncaseTokenGotStuckAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; INCASE_TOKEN_GOT_STUCK_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            token_mint: &arr[0],
            authority: &arr[1],
            to_token_account: &arr[2],
            woopool: &arr[3],
            token_vault: &arr[4],
            token_program: &arr[5],
        }
    }
}
pub const INCASE_TOKEN_GOT_STUCK_IX_DISCM: [u8; 8usize] = [
    117, 232, 146, 8, 250, 186, 60, 2,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct IncaseTokenGotStuckIxArgs {
    pub amount: u128,
}
#[derive(Clone, Debug, PartialEq)]
pub struct IncaseTokenGotStuckIxData(pub IncaseTokenGotStuckIxArgs);
impl From<IncaseTokenGotStuckIxArgs> for IncaseTokenGotStuckIxData {
    fn from(args: IncaseTokenGotStuckIxArgs) -> Self {
        Self(args)
    }
}
impl IncaseTokenGotStuckIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INCASE_TOKEN_GOT_STUCK_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let amount: u128 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(IncaseTokenGotStuckIxArgs {
                amount,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INCASE_TOKEN_GOT_STUCK_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.amount, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn incase_token_got_stuck_ix_with_program_id(
    program_id: Pubkey,
    keys: IncaseTokenGotStuckKeys,
    args: IncaseTokenGotStuckIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; INCASE_TOKEN_GOT_STUCK_IX_ACCOUNTS_LEN] = keys.into();
    let data: IncaseTokenGotStuckIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn incase_token_got_stuck_ix(
    keys: IncaseTokenGotStuckKeys,
    args: IncaseTokenGotStuckIxArgs,
) -> std::io::Result<Instruction> {
    incase_token_got_stuck_ix_with_program_id(WOOFI_PROGRAM_ID, keys, args)
}
pub fn incase_token_got_stuck_invoke_with_program_id(
    program_id: Pubkey,
    accounts: IncaseTokenGotStuckAccounts<'_, '_>,
    args: IncaseTokenGotStuckIxArgs,
) -> ProgramResult {
    let keys: IncaseTokenGotStuckKeys = accounts.into();
    let ix = incase_token_got_stuck_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn incase_token_got_stuck_invoke(
    accounts: IncaseTokenGotStuckAccounts<'_, '_>,
    args: IncaseTokenGotStuckIxArgs,
) -> ProgramResult {
    incase_token_got_stuck_invoke_with_program_id(WOOFI_PROGRAM_ID, accounts, args)
}
pub fn incase_token_got_stuck_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: IncaseTokenGotStuckAccounts<'_, '_>,
    args: IncaseTokenGotStuckIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: IncaseTokenGotStuckKeys = accounts.into();
    let ix = incase_token_got_stuck_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn incase_token_got_stuck_invoke_signed(
    accounts: IncaseTokenGotStuckAccounts<'_, '_>,
    args: IncaseTokenGotStuckIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    incase_token_got_stuck_invoke_signed_with_program_id(
        WOOFI_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn incase_token_got_stuck_verify_account_keys(
    accounts: IncaseTokenGotStuckAccounts<'_, '_>,
    keys: IncaseTokenGotStuckKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.token_mint.key, keys.token_mint),
        (*accounts.authority.key, keys.authority),
        (*accounts.to_token_account.key, keys.to_token_account),
        (*accounts.woopool.key, keys.woopool),
        (*accounts.token_vault.key, keys.token_vault),
        (*accounts.token_program.key, keys.token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn incase_token_got_stuck_verify_writable_privileges<'me, 'info>(
    accounts: IncaseTokenGotStuckAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.to_token_account,
        accounts.woopool,
        accounts.token_vault,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn incase_token_got_stuck_verify_signer_privileges<'me, 'info>(
    accounts: IncaseTokenGotStuckAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn incase_token_got_stuck_verify_account_privileges<'me, 'info>(
    accounts: IncaseTokenGotStuckAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    incase_token_got_stuck_verify_writable_privileges(accounts)?;
    incase_token_got_stuck_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SET_WOOCONFIG_NEW_AUTHORITY_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct SetWooconfigNewAuthorityAccounts<'me, 'info> {
    pub authority: &'me AccountInfo<'info>,
    pub wooconfig: &'me AccountInfo<'info>,
    pub new_authority: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SetWooconfigNewAuthorityKeys {
    pub authority: Pubkey,
    pub wooconfig: Pubkey,
    pub new_authority: Pubkey,
}
impl From<SetWooconfigNewAuthorityAccounts<'_, '_>> for SetWooconfigNewAuthorityKeys {
    fn from(accounts: SetWooconfigNewAuthorityAccounts) -> Self {
        Self {
            authority: *accounts.authority.key,
            wooconfig: *accounts.wooconfig.key,
            new_authority: *accounts.new_authority.key,
        }
    }
}
impl From<SetWooconfigNewAuthorityKeys>
for [AccountMeta; SET_WOOCONFIG_NEW_AUTHORITY_IX_ACCOUNTS_LEN] {
    fn from(keys: SetWooconfigNewAuthorityKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.authority,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.wooconfig,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.new_authority,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; SET_WOOCONFIG_NEW_AUTHORITY_IX_ACCOUNTS_LEN]>
for SetWooconfigNewAuthorityKeys {
    fn from(pubkeys: [Pubkey; SET_WOOCONFIG_NEW_AUTHORITY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            authority: pubkeys[0],
            wooconfig: pubkeys[1],
            new_authority: pubkeys[2],
        }
    }
}
impl<'info> From<SetWooconfigNewAuthorityAccounts<'_, 'info>>
for [AccountInfo<'info>; SET_WOOCONFIG_NEW_AUTHORITY_IX_ACCOUNTS_LEN] {
    fn from(accounts: SetWooconfigNewAuthorityAccounts<'_, 'info>) -> Self {
        [
            accounts.authority.clone(),
            accounts.wooconfig.clone(),
            accounts.new_authority.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; SET_WOOCONFIG_NEW_AUTHORITY_IX_ACCOUNTS_LEN]>
for SetWooconfigNewAuthorityAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; SET_WOOCONFIG_NEW_AUTHORITY_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            authority: &arr[0],
            wooconfig: &arr[1],
            new_authority: &arr[2],
        }
    }
}
pub const SET_WOOCONFIG_NEW_AUTHORITY_IX_DISCM: [u8; 8usize] = [
    114, 190, 175, 59, 207, 93, 25, 90,
];
#[derive(Clone, Debug, PartialEq)]
pub struct SetWooconfigNewAuthorityIxData;
impl SetWooconfigNewAuthorityIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SET_WOOCONFIG_NEW_AUTHORITY_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SET_WOOCONFIG_NEW_AUTHORITY_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn set_wooconfig_new_authority_ix_with_program_id(
    program_id: Pubkey,
    keys: SetWooconfigNewAuthorityKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SET_WOOCONFIG_NEW_AUTHORITY_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: SetWooconfigNewAuthorityIxData.try_to_vec()?,
    })
}
pub fn set_wooconfig_new_authority_ix(
    keys: SetWooconfigNewAuthorityKeys,
) -> std::io::Result<Instruction> {
    set_wooconfig_new_authority_ix_with_program_id(WOOFI_PROGRAM_ID, keys)
}
pub fn set_wooconfig_new_authority_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SetWooconfigNewAuthorityAccounts<'_, '_>,
) -> ProgramResult {
    let keys: SetWooconfigNewAuthorityKeys = accounts.into();
    let ix = set_wooconfig_new_authority_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn set_wooconfig_new_authority_invoke(
    accounts: SetWooconfigNewAuthorityAccounts<'_, '_>,
) -> ProgramResult {
    set_wooconfig_new_authority_invoke_with_program_id(WOOFI_PROGRAM_ID, accounts)
}
pub fn set_wooconfig_new_authority_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SetWooconfigNewAuthorityAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SetWooconfigNewAuthorityKeys = accounts.into();
    let ix = set_wooconfig_new_authority_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn set_wooconfig_new_authority_invoke_signed(
    accounts: SetWooconfigNewAuthorityAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    set_wooconfig_new_authority_invoke_signed_with_program_id(
        WOOFI_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn set_wooconfig_new_authority_verify_account_keys(
    accounts: SetWooconfigNewAuthorityAccounts<'_, '_>,
    keys: SetWooconfigNewAuthorityKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.authority.key, keys.authority),
        (*accounts.wooconfig.key, keys.wooconfig),
        (*accounts.new_authority.key, keys.new_authority),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn set_wooconfig_new_authority_verify_writable_privileges<'me, 'info>(
    accounts: SetWooconfigNewAuthorityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.wooconfig] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn set_wooconfig_new_authority_verify_signer_privileges<'me, 'info>(
    accounts: SetWooconfigNewAuthorityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn set_wooconfig_new_authority_verify_account_privileges<'me, 'info>(
    accounts: SetWooconfigNewAuthorityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    set_wooconfig_new_authority_verify_writable_privileges(accounts)?;
    set_wooconfig_new_authority_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CLAIM_WOOCONFIG_AUTHORITY_IX_ACCOUNTS_LEN: usize = 2;
#[derive(Copy, Clone, Debug)]
pub struct ClaimWooconfigAuthorityAccounts<'me, 'info> {
    pub new_authority: &'me AccountInfo<'info>,
    pub wooconfig: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ClaimWooconfigAuthorityKeys {
    pub new_authority: Pubkey,
    pub wooconfig: Pubkey,
}
impl From<ClaimWooconfigAuthorityAccounts<'_, '_>> for ClaimWooconfigAuthorityKeys {
    fn from(accounts: ClaimWooconfigAuthorityAccounts) -> Self {
        Self {
            new_authority: *accounts.new_authority.key,
            wooconfig: *accounts.wooconfig.key,
        }
    }
}
impl From<ClaimWooconfigAuthorityKeys>
for [AccountMeta; CLAIM_WOOCONFIG_AUTHORITY_IX_ACCOUNTS_LEN] {
    fn from(keys: ClaimWooconfigAuthorityKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.new_authority,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.wooconfig,
                is_signer: false,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; CLAIM_WOOCONFIG_AUTHORITY_IX_ACCOUNTS_LEN]>
for ClaimWooconfigAuthorityKeys {
    fn from(pubkeys: [Pubkey; CLAIM_WOOCONFIG_AUTHORITY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            new_authority: pubkeys[0],
            wooconfig: pubkeys[1],
        }
    }
}
impl<'info> From<ClaimWooconfigAuthorityAccounts<'_, 'info>>
for [AccountInfo<'info>; CLAIM_WOOCONFIG_AUTHORITY_IX_ACCOUNTS_LEN] {
    fn from(accounts: ClaimWooconfigAuthorityAccounts<'_, 'info>) -> Self {
        [accounts.new_authority.clone(), accounts.wooconfig.clone()]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; CLAIM_WOOCONFIG_AUTHORITY_IX_ACCOUNTS_LEN]>
for ClaimWooconfigAuthorityAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; CLAIM_WOOCONFIG_AUTHORITY_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            new_authority: &arr[0],
            wooconfig: &arr[1],
        }
    }
}
pub const CLAIM_WOOCONFIG_AUTHORITY_IX_DISCM: [u8; 8usize] = [
    139, 138, 173, 105, 222, 232, 215, 115,
];
#[derive(Clone, Debug, PartialEq)]
pub struct ClaimWooconfigAuthorityIxData;
impl ClaimWooconfigAuthorityIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CLAIM_WOOCONFIG_AUTHORITY_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CLAIM_WOOCONFIG_AUTHORITY_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn claim_wooconfig_authority_ix_with_program_id(
    program_id: Pubkey,
    keys: ClaimWooconfigAuthorityKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CLAIM_WOOCONFIG_AUTHORITY_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: ClaimWooconfigAuthorityIxData.try_to_vec()?,
    })
}
pub fn claim_wooconfig_authority_ix(
    keys: ClaimWooconfigAuthorityKeys,
) -> std::io::Result<Instruction> {
    claim_wooconfig_authority_ix_with_program_id(WOOFI_PROGRAM_ID, keys)
}
pub fn claim_wooconfig_authority_invoke_with_program_id(
    program_id: Pubkey,
    accounts: ClaimWooconfigAuthorityAccounts<'_, '_>,
) -> ProgramResult {
    let keys: ClaimWooconfigAuthorityKeys = accounts.into();
    let ix = claim_wooconfig_authority_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn claim_wooconfig_authority_invoke(
    accounts: ClaimWooconfigAuthorityAccounts<'_, '_>,
) -> ProgramResult {
    claim_wooconfig_authority_invoke_with_program_id(WOOFI_PROGRAM_ID, accounts)
}
pub fn claim_wooconfig_authority_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: ClaimWooconfigAuthorityAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: ClaimWooconfigAuthorityKeys = accounts.into();
    let ix = claim_wooconfig_authority_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn claim_wooconfig_authority_invoke_signed(
    accounts: ClaimWooconfigAuthorityAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    claim_wooconfig_authority_invoke_signed_with_program_id(
        WOOFI_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn claim_wooconfig_authority_verify_account_keys(
    accounts: ClaimWooconfigAuthorityAccounts<'_, '_>,
    keys: ClaimWooconfigAuthorityKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.new_authority.key, keys.new_authority),
        (*accounts.wooconfig.key, keys.wooconfig),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn claim_wooconfig_authority_verify_writable_privileges<'me, 'info>(
    accounts: ClaimWooconfigAuthorityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.wooconfig] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn claim_wooconfig_authority_verify_signer_privileges<'me, 'info>(
    accounts: ClaimWooconfigAuthorityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.new_authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn claim_wooconfig_authority_verify_account_privileges<'me, 'info>(
    accounts: ClaimWooconfigAuthorityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    claim_wooconfig_authority_verify_writable_privileges(accounts)?;
    claim_wooconfig_authority_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CLAIM_WOORACLE_AUTHORITY_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct ClaimWooracleAuthorityAccounts<'me, 'info> {
    pub new_authority: &'me AccountInfo<'info>,
    pub wooconfig: &'me AccountInfo<'info>,
    pub wooracle: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ClaimWooracleAuthorityKeys {
    pub new_authority: Pubkey,
    pub wooconfig: Pubkey,
    pub wooracle: Pubkey,
}
impl From<ClaimWooracleAuthorityAccounts<'_, '_>> for ClaimWooracleAuthorityKeys {
    fn from(accounts: ClaimWooracleAuthorityAccounts) -> Self {
        Self {
            new_authority: *accounts.new_authority.key,
            wooconfig: *accounts.wooconfig.key,
            wooracle: *accounts.wooracle.key,
        }
    }
}
impl From<ClaimWooracleAuthorityKeys>
for [AccountMeta; CLAIM_WOORACLE_AUTHORITY_IX_ACCOUNTS_LEN] {
    fn from(keys: ClaimWooracleAuthorityKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.new_authority,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.wooconfig,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.wooracle,
                is_signer: false,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; CLAIM_WOORACLE_AUTHORITY_IX_ACCOUNTS_LEN]>
for ClaimWooracleAuthorityKeys {
    fn from(pubkeys: [Pubkey; CLAIM_WOORACLE_AUTHORITY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            new_authority: pubkeys[0],
            wooconfig: pubkeys[1],
            wooracle: pubkeys[2],
        }
    }
}
impl<'info> From<ClaimWooracleAuthorityAccounts<'_, 'info>>
for [AccountInfo<'info>; CLAIM_WOORACLE_AUTHORITY_IX_ACCOUNTS_LEN] {
    fn from(accounts: ClaimWooracleAuthorityAccounts<'_, 'info>) -> Self {
        [
            accounts.new_authority.clone(),
            accounts.wooconfig.clone(),
            accounts.wooracle.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; CLAIM_WOORACLE_AUTHORITY_IX_ACCOUNTS_LEN]>
for ClaimWooracleAuthorityAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; CLAIM_WOORACLE_AUTHORITY_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            new_authority: &arr[0],
            wooconfig: &arr[1],
            wooracle: &arr[2],
        }
    }
}
pub const CLAIM_WOORACLE_AUTHORITY_IX_DISCM: [u8; 8usize] = [
    225, 110, 65, 225, 138, 101, 142, 72,
];
#[derive(Clone, Debug, PartialEq)]
pub struct ClaimWooracleAuthorityIxData;
impl ClaimWooracleAuthorityIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CLAIM_WOORACLE_AUTHORITY_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CLAIM_WOORACLE_AUTHORITY_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn claim_wooracle_authority_ix_with_program_id(
    program_id: Pubkey,
    keys: ClaimWooracleAuthorityKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CLAIM_WOORACLE_AUTHORITY_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: ClaimWooracleAuthorityIxData.try_to_vec()?,
    })
}
pub fn claim_wooracle_authority_ix(
    keys: ClaimWooracleAuthorityKeys,
) -> std::io::Result<Instruction> {
    claim_wooracle_authority_ix_with_program_id(WOOFI_PROGRAM_ID, keys)
}
pub fn claim_wooracle_authority_invoke_with_program_id(
    program_id: Pubkey,
    accounts: ClaimWooracleAuthorityAccounts<'_, '_>,
) -> ProgramResult {
    let keys: ClaimWooracleAuthorityKeys = accounts.into();
    let ix = claim_wooracle_authority_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn claim_wooracle_authority_invoke(
    accounts: ClaimWooracleAuthorityAccounts<'_, '_>,
) -> ProgramResult {
    claim_wooracle_authority_invoke_with_program_id(WOOFI_PROGRAM_ID, accounts)
}
pub fn claim_wooracle_authority_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: ClaimWooracleAuthorityAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: ClaimWooracleAuthorityKeys = accounts.into();
    let ix = claim_wooracle_authority_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn claim_wooracle_authority_invoke_signed(
    accounts: ClaimWooracleAuthorityAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    claim_wooracle_authority_invoke_signed_with_program_id(
        WOOFI_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn claim_wooracle_authority_verify_account_keys(
    accounts: ClaimWooracleAuthorityAccounts<'_, '_>,
    keys: ClaimWooracleAuthorityKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.new_authority.key, keys.new_authority),
        (*accounts.wooconfig.key, keys.wooconfig),
        (*accounts.wooracle.key, keys.wooracle),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn claim_wooracle_authority_verify_writable_privileges<'me, 'info>(
    accounts: ClaimWooracleAuthorityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.wooracle] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn claim_wooracle_authority_verify_signer_privileges<'me, 'info>(
    accounts: ClaimWooracleAuthorityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.new_authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn claim_wooracle_authority_verify_account_privileges<'me, 'info>(
    accounts: ClaimWooracleAuthorityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    claim_wooracle_authority_verify_writable_privileges(accounts)?;
    claim_wooracle_authority_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CLAIM_WOOPOOL_AUTHORITY_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct ClaimWoopoolAuthorityAccounts<'me, 'info> {
    pub new_authority: &'me AccountInfo<'info>,
    pub wooconfig: &'me AccountInfo<'info>,
    pub woopool: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ClaimWoopoolAuthorityKeys {
    pub new_authority: Pubkey,
    pub wooconfig: Pubkey,
    pub woopool: Pubkey,
}
impl From<ClaimWoopoolAuthorityAccounts<'_, '_>> for ClaimWoopoolAuthorityKeys {
    fn from(accounts: ClaimWoopoolAuthorityAccounts) -> Self {
        Self {
            new_authority: *accounts.new_authority.key,
            wooconfig: *accounts.wooconfig.key,
            woopool: *accounts.woopool.key,
        }
    }
}
impl From<ClaimWoopoolAuthorityKeys>
for [AccountMeta; CLAIM_WOOPOOL_AUTHORITY_IX_ACCOUNTS_LEN] {
    fn from(keys: ClaimWoopoolAuthorityKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.new_authority,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.wooconfig,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.woopool,
                is_signer: false,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; CLAIM_WOOPOOL_AUTHORITY_IX_ACCOUNTS_LEN]>
for ClaimWoopoolAuthorityKeys {
    fn from(pubkeys: [Pubkey; CLAIM_WOOPOOL_AUTHORITY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            new_authority: pubkeys[0],
            wooconfig: pubkeys[1],
            woopool: pubkeys[2],
        }
    }
}
impl<'info> From<ClaimWoopoolAuthorityAccounts<'_, 'info>>
for [AccountInfo<'info>; CLAIM_WOOPOOL_AUTHORITY_IX_ACCOUNTS_LEN] {
    fn from(accounts: ClaimWoopoolAuthorityAccounts<'_, 'info>) -> Self {
        [
            accounts.new_authority.clone(),
            accounts.wooconfig.clone(),
            accounts.woopool.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CLAIM_WOOPOOL_AUTHORITY_IX_ACCOUNTS_LEN]>
for ClaimWoopoolAuthorityAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; CLAIM_WOOPOOL_AUTHORITY_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            new_authority: &arr[0],
            wooconfig: &arr[1],
            woopool: &arr[2],
        }
    }
}
pub const CLAIM_WOOPOOL_AUTHORITY_IX_DISCM: [u8; 8usize] = [
    173, 55, 188, 135, 230, 201, 96, 151,
];
#[derive(Clone, Debug, PartialEq)]
pub struct ClaimWoopoolAuthorityIxData;
impl ClaimWoopoolAuthorityIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CLAIM_WOOPOOL_AUTHORITY_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CLAIM_WOOPOOL_AUTHORITY_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn claim_woopool_authority_ix_with_program_id(
    program_id: Pubkey,
    keys: ClaimWoopoolAuthorityKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CLAIM_WOOPOOL_AUTHORITY_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: ClaimWoopoolAuthorityIxData.try_to_vec()?,
    })
}
pub fn claim_woopool_authority_ix(
    keys: ClaimWoopoolAuthorityKeys,
) -> std::io::Result<Instruction> {
    claim_woopool_authority_ix_with_program_id(WOOFI_PROGRAM_ID, keys)
}
pub fn claim_woopool_authority_invoke_with_program_id(
    program_id: Pubkey,
    accounts: ClaimWoopoolAuthorityAccounts<'_, '_>,
) -> ProgramResult {
    let keys: ClaimWoopoolAuthorityKeys = accounts.into();
    let ix = claim_woopool_authority_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn claim_woopool_authority_invoke(
    accounts: ClaimWoopoolAuthorityAccounts<'_, '_>,
) -> ProgramResult {
    claim_woopool_authority_invoke_with_program_id(WOOFI_PROGRAM_ID, accounts)
}
pub fn claim_woopool_authority_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: ClaimWoopoolAuthorityAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: ClaimWoopoolAuthorityKeys = accounts.into();
    let ix = claim_woopool_authority_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn claim_woopool_authority_invoke_signed(
    accounts: ClaimWoopoolAuthorityAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    claim_woopool_authority_invoke_signed_with_program_id(
        WOOFI_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn claim_woopool_authority_verify_account_keys(
    accounts: ClaimWoopoolAuthorityAccounts<'_, '_>,
    keys: ClaimWoopoolAuthorityKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.new_authority.key, keys.new_authority),
        (*accounts.wooconfig.key, keys.wooconfig),
        (*accounts.woopool.key, keys.woopool),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn claim_woopool_authority_verify_writable_privileges<'me, 'info>(
    accounts: ClaimWoopoolAuthorityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.woopool] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn claim_woopool_authority_verify_signer_privileges<'me, 'info>(
    accounts: ClaimWoopoolAuthorityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.new_authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn claim_woopool_authority_verify_account_privileges<'me, 'info>(
    accounts: ClaimWoopoolAuthorityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    claim_woopool_authority_verify_writable_privileges(accounts)?;
    claim_woopool_authority_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CLAIM_WOOAMMPOOL_AUTHORITY_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct ClaimWooammpoolAuthorityAccounts<'me, 'info> {
    pub new_authority: &'me AccountInfo<'info>,
    pub wooconfig: &'me AccountInfo<'info>,
    pub wooammpool: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ClaimWooammpoolAuthorityKeys {
    pub new_authority: Pubkey,
    pub wooconfig: Pubkey,
    pub wooammpool: Pubkey,
}
impl From<ClaimWooammpoolAuthorityAccounts<'_, '_>> for ClaimWooammpoolAuthorityKeys {
    fn from(accounts: ClaimWooammpoolAuthorityAccounts) -> Self {
        Self {
            new_authority: *accounts.new_authority.key,
            wooconfig: *accounts.wooconfig.key,
            wooammpool: *accounts.wooammpool.key,
        }
    }
}
impl From<ClaimWooammpoolAuthorityKeys>
for [AccountMeta; CLAIM_WOOAMMPOOL_AUTHORITY_IX_ACCOUNTS_LEN] {
    fn from(keys: ClaimWooammpoolAuthorityKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.new_authority,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.wooconfig,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.wooammpool,
                is_signer: false,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; CLAIM_WOOAMMPOOL_AUTHORITY_IX_ACCOUNTS_LEN]>
for ClaimWooammpoolAuthorityKeys {
    fn from(pubkeys: [Pubkey; CLAIM_WOOAMMPOOL_AUTHORITY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            new_authority: pubkeys[0],
            wooconfig: pubkeys[1],
            wooammpool: pubkeys[2],
        }
    }
}
impl<'info> From<ClaimWooammpoolAuthorityAccounts<'_, 'info>>
for [AccountInfo<'info>; CLAIM_WOOAMMPOOL_AUTHORITY_IX_ACCOUNTS_LEN] {
    fn from(accounts: ClaimWooammpoolAuthorityAccounts<'_, 'info>) -> Self {
        [
            accounts.new_authority.clone(),
            accounts.wooconfig.clone(),
            accounts.wooammpool.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; CLAIM_WOOAMMPOOL_AUTHORITY_IX_ACCOUNTS_LEN]>
for ClaimWooammpoolAuthorityAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; CLAIM_WOOAMMPOOL_AUTHORITY_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            new_authority: &arr[0],
            wooconfig: &arr[1],
            wooammpool: &arr[2],
        }
    }
}
pub const CLAIM_WOOAMMPOOL_AUTHORITY_IX_DISCM: [u8; 8usize] = [
    65, 224, 51, 150, 107, 227, 48, 59,
];
#[derive(Clone, Debug, PartialEq)]
pub struct ClaimWooammpoolAuthorityIxData;
impl ClaimWooammpoolAuthorityIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CLAIM_WOOAMMPOOL_AUTHORITY_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CLAIM_WOOAMMPOOL_AUTHORITY_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn claim_wooammpool_authority_ix_with_program_id(
    program_id: Pubkey,
    keys: ClaimWooammpoolAuthorityKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CLAIM_WOOAMMPOOL_AUTHORITY_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: ClaimWooammpoolAuthorityIxData.try_to_vec()?,
    })
}
pub fn claim_wooammpool_authority_ix(
    keys: ClaimWooammpoolAuthorityKeys,
) -> std::io::Result<Instruction> {
    claim_wooammpool_authority_ix_with_program_id(WOOFI_PROGRAM_ID, keys)
}
pub fn claim_wooammpool_authority_invoke_with_program_id(
    program_id: Pubkey,
    accounts: ClaimWooammpoolAuthorityAccounts<'_, '_>,
) -> ProgramResult {
    let keys: ClaimWooammpoolAuthorityKeys = accounts.into();
    let ix = claim_wooammpool_authority_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn claim_wooammpool_authority_invoke(
    accounts: ClaimWooammpoolAuthorityAccounts<'_, '_>,
) -> ProgramResult {
    claim_wooammpool_authority_invoke_with_program_id(WOOFI_PROGRAM_ID, accounts)
}
pub fn claim_wooammpool_authority_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: ClaimWooammpoolAuthorityAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: ClaimWooammpoolAuthorityKeys = accounts.into();
    let ix = claim_wooammpool_authority_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn claim_wooammpool_authority_invoke_signed(
    accounts: ClaimWooammpoolAuthorityAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    claim_wooammpool_authority_invoke_signed_with_program_id(
        WOOFI_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn claim_wooammpool_authority_verify_account_keys(
    accounts: ClaimWooammpoolAuthorityAccounts<'_, '_>,
    keys: ClaimWooammpoolAuthorityKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.new_authority.key, keys.new_authority),
        (*accounts.wooconfig.key, keys.wooconfig),
        (*accounts.wooammpool.key, keys.wooammpool),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn claim_wooammpool_authority_verify_writable_privileges<'me, 'info>(
    accounts: ClaimWooammpoolAuthorityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.wooammpool] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn claim_wooammpool_authority_verify_signer_privileges<'me, 'info>(
    accounts: ClaimWooammpoolAuthorityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.new_authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn claim_wooammpool_authority_verify_account_privileges<'me, 'info>(
    accounts: ClaimWooammpoolAuthorityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    claim_wooammpool_authority_verify_writable_privileges(accounts)?;
    claim_wooammpool_authority_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const REPAY_BY_LENDING_MANAGER_IX_ACCOUNTS_LEN: usize = 6;
#[derive(Copy, Clone, Debug)]
pub struct RepayByLendingManagerAccounts<'me, 'info> {
    pub wooconfig: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub woopool: &'me AccountInfo<'info>,
    pub token_vault: &'me AccountInfo<'info>,
    pub super_charger_vault: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct RepayByLendingManagerKeys {
    pub wooconfig: Pubkey,
    pub authority: Pubkey,
    pub woopool: Pubkey,
    pub token_vault: Pubkey,
    pub super_charger_vault: Pubkey,
    pub token_program: Pubkey,
}
impl From<RepayByLendingManagerAccounts<'_, '_>> for RepayByLendingManagerKeys {
    fn from(accounts: RepayByLendingManagerAccounts) -> Self {
        Self {
            wooconfig: *accounts.wooconfig.key,
            authority: *accounts.authority.key,
            woopool: *accounts.woopool.key,
            token_vault: *accounts.token_vault.key,
            super_charger_vault: *accounts.super_charger_vault.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<RepayByLendingManagerKeys>
for [AccountMeta; REPAY_BY_LENDING_MANAGER_IX_ACCOUNTS_LEN] {
    fn from(keys: RepayByLendingManagerKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.wooconfig,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.authority,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.woopool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.super_charger_vault,
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
impl From<[Pubkey; REPAY_BY_LENDING_MANAGER_IX_ACCOUNTS_LEN]>
for RepayByLendingManagerKeys {
    fn from(pubkeys: [Pubkey; REPAY_BY_LENDING_MANAGER_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            wooconfig: pubkeys[0],
            authority: pubkeys[1],
            woopool: pubkeys[2],
            token_vault: pubkeys[3],
            super_charger_vault: pubkeys[4],
            token_program: pubkeys[5],
        }
    }
}
impl<'info> From<RepayByLendingManagerAccounts<'_, 'info>>
for [AccountInfo<'info>; REPAY_BY_LENDING_MANAGER_IX_ACCOUNTS_LEN] {
    fn from(accounts: RepayByLendingManagerAccounts<'_, 'info>) -> Self {
        [
            accounts.wooconfig.clone(),
            accounts.authority.clone(),
            accounts.woopool.clone(),
            accounts.token_vault.clone(),
            accounts.super_charger_vault.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; REPAY_BY_LENDING_MANAGER_IX_ACCOUNTS_LEN]>
for RepayByLendingManagerAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; REPAY_BY_LENDING_MANAGER_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            wooconfig: &arr[0],
            authority: &arr[1],
            woopool: &arr[2],
            token_vault: &arr[3],
            super_charger_vault: &arr[4],
            token_program: &arr[5],
        }
    }
}
pub const REPAY_BY_LENDING_MANAGER_IX_DISCM: [u8; 8usize] = [
    158, 154, 161, 163, 96, 3, 0, 99,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct RepayByLendingManagerIxArgs {
    pub repay_amount: u128,
}
#[derive(Clone, Debug, PartialEq)]
pub struct RepayByLendingManagerIxData(pub RepayByLendingManagerIxArgs);
impl From<RepayByLendingManagerIxArgs> for RepayByLendingManagerIxData {
    fn from(args: RepayByLendingManagerIxArgs) -> Self {
        Self(args)
    }
}
impl RepayByLendingManagerIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != REPAY_BY_LENDING_MANAGER_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let repay_amount: u128 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(RepayByLendingManagerIxArgs {
                repay_amount,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&REPAY_BY_LENDING_MANAGER_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.repay_amount, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn repay_by_lending_manager_ix_with_program_id(
    program_id: Pubkey,
    keys: RepayByLendingManagerKeys,
    args: RepayByLendingManagerIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; REPAY_BY_LENDING_MANAGER_IX_ACCOUNTS_LEN] = keys.into();
    let data: RepayByLendingManagerIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn repay_by_lending_manager_ix(
    keys: RepayByLendingManagerKeys,
    args: RepayByLendingManagerIxArgs,
) -> std::io::Result<Instruction> {
    repay_by_lending_manager_ix_with_program_id(WOOFI_PROGRAM_ID, keys, args)
}
pub fn repay_by_lending_manager_invoke_with_program_id(
    program_id: Pubkey,
    accounts: RepayByLendingManagerAccounts<'_, '_>,
    args: RepayByLendingManagerIxArgs,
) -> ProgramResult {
    let keys: RepayByLendingManagerKeys = accounts.into();
    let ix = repay_by_lending_manager_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn repay_by_lending_manager_invoke(
    accounts: RepayByLendingManagerAccounts<'_, '_>,
    args: RepayByLendingManagerIxArgs,
) -> ProgramResult {
    repay_by_lending_manager_invoke_with_program_id(WOOFI_PROGRAM_ID, accounts, args)
}
pub fn repay_by_lending_manager_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: RepayByLendingManagerAccounts<'_, '_>,
    args: RepayByLendingManagerIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: RepayByLendingManagerKeys = accounts.into();
    let ix = repay_by_lending_manager_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn repay_by_lending_manager_invoke_signed(
    accounts: RepayByLendingManagerAccounts<'_, '_>,
    args: RepayByLendingManagerIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    repay_by_lending_manager_invoke_signed_with_program_id(
        WOOFI_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn repay_by_lending_manager_verify_account_keys(
    accounts: RepayByLendingManagerAccounts<'_, '_>,
    keys: RepayByLendingManagerKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.wooconfig.key, keys.wooconfig),
        (*accounts.authority.key, keys.authority),
        (*accounts.woopool.key, keys.woopool),
        (*accounts.token_vault.key, keys.token_vault),
        (*accounts.super_charger_vault.key, keys.super_charger_vault),
        (*accounts.token_program.key, keys.token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn repay_by_lending_manager_verify_writable_privileges<'me, 'info>(
    accounts: RepayByLendingManagerAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.woopool,
        accounts.token_vault,
        accounts.super_charger_vault,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn repay_by_lending_manager_verify_signer_privileges<'me, 'info>(
    accounts: RepayByLendingManagerAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn repay_by_lending_manager_verify_account_privileges<'me, 'info>(
    accounts: RepayByLendingManagerAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    repay_by_lending_manager_verify_writable_privileges(accounts)?;
    repay_by_lending_manager_verify_signer_privileges(accounts)?;
    Ok(())
}
