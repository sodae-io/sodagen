use solana_pubkey::Pubkey;
use solana_cpi::{invoke, invoke_signed};
use solana_instruction::{AccountMeta, Instruction};
use solana_account_info::AccountInfo;
use solana_program_error::ProgramError;
use std::io::Read;
#[allow(unused_imports)]
use crate::*;
#[derive(Clone, Debug, PartialEq)]
pub enum JupiterLendProgramIx {
    Deposit(DepositIxArgs),
    DepositWithMinAmountOut(DepositWithMinAmountOutIxArgs),
    InitLending(InitLendingIxArgs),
    InitLendingAdmin(InitLendingAdminIxArgs),
    Mint(MintIxArgs),
    MintWithMaxAssets(MintWithMaxAssetsIxArgs),
    Rebalance,
    Redeem(RedeemIxArgs),
    RedeemWithMinAmountOut(RedeemWithMinAmountOutIxArgs),
    SetRewardsRateModel(SetRewardsRateModelIxArgs),
    UpdateAuthority(UpdateAuthorityIxArgs),
    UpdateAuths(UpdateAuthsIxArgs),
    UpdateRate,
    UpdateRebalancer(UpdateRebalancerIxArgs),
    Withdraw(WithdrawIxArgs),
    WithdrawWithMaxSharesBurn(WithdrawWithMaxSharesBurnIxArgs),
}
impl JupiterLendProgramIx {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        if buf.starts_with(&DEPOSIT_IX_DISCM) {
            let mut reader = &buf[DEPOSIT_IX_DISCM.len()..];
            let assets: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(Self::Deposit(DepositIxArgs { assets }));
        }
        if buf.starts_with(&DEPOSIT_WITH_MIN_AMOUNT_OUT_IX_DISCM) {
            let mut reader = &buf[DEPOSIT_WITH_MIN_AMOUNT_OUT_IX_DISCM.len()..];
            let assets: u64 = crate::borsh_de_or_default(&mut reader)?;
            let min_amount_out: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::DepositWithMinAmountOut(DepositWithMinAmountOutIxArgs {
                    assets,
                    min_amount_out,
                }),
            );
        }
        if buf.starts_with(&INIT_LENDING_IX_DISCM) {
            let mut reader = &buf[INIT_LENDING_IX_DISCM.len()..];
            let symbol: String = crate::borsh_de_or_default(&mut reader)?;
            let liquidity_program: Pubkey = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::InitLending(InitLendingIxArgs {
                    symbol,
                    liquidity_program,
                }),
            );
        }
        if buf.starts_with(&INIT_LENDING_ADMIN_IX_DISCM) {
            let mut reader = &buf[INIT_LENDING_ADMIN_IX_DISCM.len()..];
            let liquidity_program: Pubkey = crate::borsh_de_or_default(&mut reader)?;
            let rebalancer: Pubkey = crate::borsh_de_or_default(&mut reader)?;
            let authority: Pubkey = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::InitLendingAdmin(InitLendingAdminIxArgs {
                    liquidity_program,
                    rebalancer,
                    authority,
                }),
            );
        }
        if buf.starts_with(&MINT_IX_DISCM) {
            let mut reader = &buf[MINT_IX_DISCM.len()..];
            let shares: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(Self::Mint(MintIxArgs { shares }));
        }
        if buf.starts_with(&MINT_WITH_MAX_ASSETS_IX_DISCM) {
            let mut reader = &buf[MINT_WITH_MAX_ASSETS_IX_DISCM.len()..];
            let shares: u64 = crate::borsh_de_or_default(&mut reader)?;
            let max_assets: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::MintWithMaxAssets(MintWithMaxAssetsIxArgs {
                    shares,
                    max_assets,
                }),
            );
        }
        if buf.starts_with(&REBALANCE_IX_DISCM) {
            return Ok(Self::Rebalance);
        }
        if buf.starts_with(&REDEEM_IX_DISCM) {
            let mut reader = &buf[REDEEM_IX_DISCM.len()..];
            let shares: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(Self::Redeem(RedeemIxArgs { shares }));
        }
        if buf.starts_with(&REDEEM_WITH_MIN_AMOUNT_OUT_IX_DISCM) {
            let mut reader = &buf[REDEEM_WITH_MIN_AMOUNT_OUT_IX_DISCM.len()..];
            let shares: u64 = crate::borsh_de_or_default(&mut reader)?;
            let min_amount_out: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::RedeemWithMinAmountOut(RedeemWithMinAmountOutIxArgs {
                    shares,
                    min_amount_out,
                }),
            );
        }
        if buf.starts_with(&SET_REWARDS_RATE_MODEL_IX_DISCM) {
            let mut reader = &buf[SET_REWARDS_RATE_MODEL_IX_DISCM.len()..];
            let mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
            return Ok(Self::SetRewardsRateModel(SetRewardsRateModelIxArgs { mint }));
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
        if buf.starts_with(&UPDATE_RATE_IX_DISCM) {
            return Ok(Self::UpdateRate);
        }
        if buf.starts_with(&UPDATE_REBALANCER_IX_DISCM) {
            let mut reader = &buf[UPDATE_REBALANCER_IX_DISCM.len()..];
            let new_rebalancer: Pubkey = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::UpdateRebalancer(UpdateRebalancerIxArgs {
                    new_rebalancer,
                }),
            );
        }
        if buf.starts_with(&WITHDRAW_IX_DISCM) {
            let mut reader = &buf[WITHDRAW_IX_DISCM.len()..];
            let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(Self::Withdraw(WithdrawIxArgs { amount }));
        }
        if buf.starts_with(&WITHDRAW_WITH_MAX_SHARES_BURN_IX_DISCM) {
            let mut reader = &buf[WITHDRAW_WITH_MAX_SHARES_BURN_IX_DISCM.len()..];
            let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            let max_shares_burn: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::WithdrawWithMaxSharesBurn(WithdrawWithMaxSharesBurnIxArgs {
                    amount,
                    max_shares_burn,
                }),
            );
        }
        Err(std::io::Error::from(std::io::ErrorKind::InvalidData))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        match self {
            Self::Deposit(args) => {
                writer.write_all(&DEPOSIT_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.assets, &mut writer)?;
                Ok(())
            }
            Self::DepositWithMinAmountOut(args) => {
                writer.write_all(&DEPOSIT_WITH_MIN_AMOUNT_OUT_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.assets, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.min_amount_out, &mut writer)?;
                Ok(())
            }
            Self::InitLending(args) => {
                writer.write_all(&INIT_LENDING_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.symbol, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.liquidity_program, &mut writer)?;
                Ok(())
            }
            Self::InitLendingAdmin(args) => {
                writer.write_all(&INIT_LENDING_ADMIN_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.liquidity_program, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.rebalancer, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.authority, &mut writer)?;
                Ok(())
            }
            Self::Mint(args) => {
                writer.write_all(&MINT_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.shares, &mut writer)?;
                Ok(())
            }
            Self::MintWithMaxAssets(args) => {
                writer.write_all(&MINT_WITH_MAX_ASSETS_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.shares, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.max_assets, &mut writer)?;
                Ok(())
            }
            Self::Rebalance => writer.write_all(&REBALANCE_IX_DISCM),
            Self::Redeem(args) => {
                writer.write_all(&REDEEM_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.shares, &mut writer)?;
                Ok(())
            }
            Self::RedeemWithMinAmountOut(args) => {
                writer.write_all(&REDEEM_WITH_MIN_AMOUNT_OUT_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.shares, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.min_amount_out, &mut writer)?;
                Ok(())
            }
            Self::SetRewardsRateModel(args) => {
                writer.write_all(&SET_REWARDS_RATE_MODEL_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.mint, &mut writer)?;
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
            Self::UpdateRate => writer.write_all(&UPDATE_RATE_IX_DISCM),
            Self::UpdateRebalancer(args) => {
                writer.write_all(&UPDATE_REBALANCER_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.new_rebalancer, &mut writer)?;
                Ok(())
            }
            Self::Withdraw(args) => {
                writer.write_all(&WITHDRAW_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.amount, &mut writer)?;
                Ok(())
            }
            Self::WithdrawWithMaxSharesBurn(args) => {
                writer.write_all(&WITHDRAW_WITH_MAX_SHARES_BURN_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.amount, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.max_shares_burn, &mut writer)?;
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
pub const DEPOSIT_IX_ACCOUNTS_LEN: usize = 17;
#[derive(Copy, Clone, Debug)]
pub struct DepositAccounts<'me, 'info> {
    pub signer: &'me AccountInfo<'info>,
    pub depositor_token_account: &'me AccountInfo<'info>,
    pub recipient_token_account: &'me AccountInfo<'info>,
    pub mint: &'me AccountInfo<'info>,
    pub lending_admin: &'me AccountInfo<'info>,
    pub lending: &'me AccountInfo<'info>,
    pub f_token_mint: &'me AccountInfo<'info>,
    pub supply_token_reserves_liquidity: &'me AccountInfo<'info>,
    pub lending_supply_position_on_liquidity: &'me AccountInfo<'info>,
    pub rate_model: &'me AccountInfo<'info>,
    pub vault: &'me AccountInfo<'info>,
    pub liquidity: &'me AccountInfo<'info>,
    pub liquidity_program: &'me AccountInfo<'info>,
    pub rewards_rate_model: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct DepositKeys {
    pub signer: Pubkey,
    pub depositor_token_account: Pubkey,
    pub recipient_token_account: Pubkey,
    pub mint: Pubkey,
    pub lending_admin: Pubkey,
    pub lending: Pubkey,
    pub f_token_mint: Pubkey,
    pub supply_token_reserves_liquidity: Pubkey,
    pub lending_supply_position_on_liquidity: Pubkey,
    pub rate_model: Pubkey,
    pub vault: Pubkey,
    pub liquidity: Pubkey,
    pub liquidity_program: Pubkey,
    pub rewards_rate_model: Pubkey,
    pub token_program: Pubkey,
    pub associated_token_program: Pubkey,
    pub system_program: Pubkey,
}
impl From<DepositAccounts<'_, '_>> for DepositKeys {
    fn from(accounts: DepositAccounts) -> Self {
        Self {
            signer: *accounts.signer.key,
            depositor_token_account: *accounts.depositor_token_account.key,
            recipient_token_account: *accounts.recipient_token_account.key,
            mint: *accounts.mint.key,
            lending_admin: *accounts.lending_admin.key,
            lending: *accounts.lending.key,
            f_token_mint: *accounts.f_token_mint.key,
            supply_token_reserves_liquidity: *accounts
                .supply_token_reserves_liquidity
                .key,
            lending_supply_position_on_liquidity: *accounts
                .lending_supply_position_on_liquidity
                .key,
            rate_model: *accounts.rate_model.key,
            vault: *accounts.vault.key,
            liquidity: *accounts.liquidity.key,
            liquidity_program: *accounts.liquidity_program.key,
            rewards_rate_model: *accounts.rewards_rate_model.key,
            token_program: *accounts.token_program.key,
            associated_token_program: *accounts.associated_token_program.key,
            system_program: *accounts.system_program.key,
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
                pubkey: keys.depositor_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.recipient_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.lending_admin,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.lending,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.f_token_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.supply_token_reserves_liquidity,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.lending_supply_position_on_liquidity,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.rate_model,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.liquidity,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.liquidity_program,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.rewards_rate_model,
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
        ]
    }
}
impl From<[Pubkey; DEPOSIT_IX_ACCOUNTS_LEN]> for DepositKeys {
    fn from(pubkeys: [Pubkey; DEPOSIT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            signer: pubkeys[0],
            depositor_token_account: pubkeys[1],
            recipient_token_account: pubkeys[2],
            mint: pubkeys[3],
            lending_admin: pubkeys[4],
            lending: pubkeys[5],
            f_token_mint: pubkeys[6],
            supply_token_reserves_liquidity: pubkeys[7],
            lending_supply_position_on_liquidity: pubkeys[8],
            rate_model: pubkeys[9],
            vault: pubkeys[10],
            liquidity: pubkeys[11],
            liquidity_program: pubkeys[12],
            rewards_rate_model: pubkeys[13],
            token_program: pubkeys[14],
            associated_token_program: pubkeys[15],
            system_program: pubkeys[16],
        }
    }
}
impl<'info> From<DepositAccounts<'_, 'info>>
for [AccountInfo<'info>; DEPOSIT_IX_ACCOUNTS_LEN] {
    fn from(accounts: DepositAccounts<'_, 'info>) -> Self {
        [
            accounts.signer.clone(),
            accounts.depositor_token_account.clone(),
            accounts.recipient_token_account.clone(),
            accounts.mint.clone(),
            accounts.lending_admin.clone(),
            accounts.lending.clone(),
            accounts.f_token_mint.clone(),
            accounts.supply_token_reserves_liquidity.clone(),
            accounts.lending_supply_position_on_liquidity.clone(),
            accounts.rate_model.clone(),
            accounts.vault.clone(),
            accounts.liquidity.clone(),
            accounts.liquidity_program.clone(),
            accounts.rewards_rate_model.clone(),
            accounts.token_program.clone(),
            accounts.associated_token_program.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; DEPOSIT_IX_ACCOUNTS_LEN]>
for DepositAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; DEPOSIT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            signer: &arr[0],
            depositor_token_account: &arr[1],
            recipient_token_account: &arr[2],
            mint: &arr[3],
            lending_admin: &arr[4],
            lending: &arr[5],
            f_token_mint: &arr[6],
            supply_token_reserves_liquidity: &arr[7],
            lending_supply_position_on_liquidity: &arr[8],
            rate_model: &arr[9],
            vault: &arr[10],
            liquidity: &arr[11],
            liquidity_program: &arr[12],
            rewards_rate_model: &arr[13],
            token_program: &arr[14],
            associated_token_program: &arr[15],
            system_program: &arr[16],
        }
    }
}
pub const DEPOSIT_IX_DISCM: [u8; 8usize] = [242, 35, 198, 137, 82, 225, 242, 182];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DepositIxArgs {
    pub assets: u64,
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
        let assets: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(Self(DepositIxArgs { assets }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&DEPOSIT_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.assets, &mut writer)?;
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
    deposit_ix_with_program_id(JUPITER_LEND_PROGRAM_ID, keys, args)
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
    deposit_invoke_with_program_id(JUPITER_LEND_PROGRAM_ID, accounts, args)
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
    deposit_invoke_signed_with_program_id(JUPITER_LEND_PROGRAM_ID, accounts, args, seeds)
}
pub fn deposit_verify_account_keys(
    accounts: DepositAccounts<'_, '_>,
    keys: DepositKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.signer.key, keys.signer),
        (*accounts.depositor_token_account.key, keys.depositor_token_account),
        (*accounts.recipient_token_account.key, keys.recipient_token_account),
        (*accounts.mint.key, keys.mint),
        (*accounts.lending_admin.key, keys.lending_admin),
        (*accounts.lending.key, keys.lending),
        (*accounts.f_token_mint.key, keys.f_token_mint),
        (
            *accounts.supply_token_reserves_liquidity.key,
            keys.supply_token_reserves_liquidity,
        ),
        (
            *accounts.lending_supply_position_on_liquidity.key,
            keys.lending_supply_position_on_liquidity,
        ),
        (*accounts.rate_model.key, keys.rate_model),
        (*accounts.vault.key, keys.vault),
        (*accounts.liquidity.key, keys.liquidity),
        (*accounts.liquidity_program.key, keys.liquidity_program),
        (*accounts.rewards_rate_model.key, keys.rewards_rate_model),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.associated_token_program.key, keys.associated_token_program),
        (*accounts.system_program.key, keys.system_program),
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
        accounts.depositor_token_account,
        accounts.recipient_token_account,
        accounts.lending,
        accounts.f_token_mint,
        accounts.supply_token_reserves_liquidity,
        accounts.lending_supply_position_on_liquidity,
        accounts.vault,
        accounts.liquidity,
        accounts.liquidity_program,
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
    for should_be_signer in [accounts.signer] {
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
pub const DEPOSIT_WITH_MIN_AMOUNT_OUT_IX_ACCOUNTS_LEN: usize = 17;
#[derive(Copy, Clone, Debug)]
pub struct DepositWithMinAmountOutAccounts<'me, 'info> {
    pub signer: &'me AccountInfo<'info>,
    pub depositor_token_account: &'me AccountInfo<'info>,
    pub recipient_token_account: &'me AccountInfo<'info>,
    pub mint: &'me AccountInfo<'info>,
    pub lending_admin: &'me AccountInfo<'info>,
    pub lending: &'me AccountInfo<'info>,
    pub f_token_mint: &'me AccountInfo<'info>,
    pub supply_token_reserves_liquidity: &'me AccountInfo<'info>,
    pub lending_supply_position_on_liquidity: &'me AccountInfo<'info>,
    pub rate_model: &'me AccountInfo<'info>,
    pub vault: &'me AccountInfo<'info>,
    pub liquidity: &'me AccountInfo<'info>,
    pub liquidity_program: &'me AccountInfo<'info>,
    pub rewards_rate_model: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct DepositWithMinAmountOutKeys {
    pub signer: Pubkey,
    pub depositor_token_account: Pubkey,
    pub recipient_token_account: Pubkey,
    pub mint: Pubkey,
    pub lending_admin: Pubkey,
    pub lending: Pubkey,
    pub f_token_mint: Pubkey,
    pub supply_token_reserves_liquidity: Pubkey,
    pub lending_supply_position_on_liquidity: Pubkey,
    pub rate_model: Pubkey,
    pub vault: Pubkey,
    pub liquidity: Pubkey,
    pub liquidity_program: Pubkey,
    pub rewards_rate_model: Pubkey,
    pub token_program: Pubkey,
    pub associated_token_program: Pubkey,
    pub system_program: Pubkey,
}
impl From<DepositWithMinAmountOutAccounts<'_, '_>> for DepositWithMinAmountOutKeys {
    fn from(accounts: DepositWithMinAmountOutAccounts) -> Self {
        Self {
            signer: *accounts.signer.key,
            depositor_token_account: *accounts.depositor_token_account.key,
            recipient_token_account: *accounts.recipient_token_account.key,
            mint: *accounts.mint.key,
            lending_admin: *accounts.lending_admin.key,
            lending: *accounts.lending.key,
            f_token_mint: *accounts.f_token_mint.key,
            supply_token_reserves_liquidity: *accounts
                .supply_token_reserves_liquidity
                .key,
            lending_supply_position_on_liquidity: *accounts
                .lending_supply_position_on_liquidity
                .key,
            rate_model: *accounts.rate_model.key,
            vault: *accounts.vault.key,
            liquidity: *accounts.liquidity.key,
            liquidity_program: *accounts.liquidity_program.key,
            rewards_rate_model: *accounts.rewards_rate_model.key,
            token_program: *accounts.token_program.key,
            associated_token_program: *accounts.associated_token_program.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<DepositWithMinAmountOutKeys>
for [AccountMeta; DEPOSIT_WITH_MIN_AMOUNT_OUT_IX_ACCOUNTS_LEN] {
    fn from(keys: DepositWithMinAmountOutKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.signer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.depositor_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.recipient_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.lending_admin,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.lending,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.f_token_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.supply_token_reserves_liquidity,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.lending_supply_position_on_liquidity,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.rate_model,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.liquidity,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.liquidity_program,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.rewards_rate_model,
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
        ]
    }
}
impl From<[Pubkey; DEPOSIT_WITH_MIN_AMOUNT_OUT_IX_ACCOUNTS_LEN]>
for DepositWithMinAmountOutKeys {
    fn from(pubkeys: [Pubkey; DEPOSIT_WITH_MIN_AMOUNT_OUT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            signer: pubkeys[0],
            depositor_token_account: pubkeys[1],
            recipient_token_account: pubkeys[2],
            mint: pubkeys[3],
            lending_admin: pubkeys[4],
            lending: pubkeys[5],
            f_token_mint: pubkeys[6],
            supply_token_reserves_liquidity: pubkeys[7],
            lending_supply_position_on_liquidity: pubkeys[8],
            rate_model: pubkeys[9],
            vault: pubkeys[10],
            liquidity: pubkeys[11],
            liquidity_program: pubkeys[12],
            rewards_rate_model: pubkeys[13],
            token_program: pubkeys[14],
            associated_token_program: pubkeys[15],
            system_program: pubkeys[16],
        }
    }
}
impl<'info> From<DepositWithMinAmountOutAccounts<'_, 'info>>
for [AccountInfo<'info>; DEPOSIT_WITH_MIN_AMOUNT_OUT_IX_ACCOUNTS_LEN] {
    fn from(accounts: DepositWithMinAmountOutAccounts<'_, 'info>) -> Self {
        [
            accounts.signer.clone(),
            accounts.depositor_token_account.clone(),
            accounts.recipient_token_account.clone(),
            accounts.mint.clone(),
            accounts.lending_admin.clone(),
            accounts.lending.clone(),
            accounts.f_token_mint.clone(),
            accounts.supply_token_reserves_liquidity.clone(),
            accounts.lending_supply_position_on_liquidity.clone(),
            accounts.rate_model.clone(),
            accounts.vault.clone(),
            accounts.liquidity.clone(),
            accounts.liquidity_program.clone(),
            accounts.rewards_rate_model.clone(),
            accounts.token_program.clone(),
            accounts.associated_token_program.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; DEPOSIT_WITH_MIN_AMOUNT_OUT_IX_ACCOUNTS_LEN]>
for DepositWithMinAmountOutAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; DEPOSIT_WITH_MIN_AMOUNT_OUT_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            signer: &arr[0],
            depositor_token_account: &arr[1],
            recipient_token_account: &arr[2],
            mint: &arr[3],
            lending_admin: &arr[4],
            lending: &arr[5],
            f_token_mint: &arr[6],
            supply_token_reserves_liquidity: &arr[7],
            lending_supply_position_on_liquidity: &arr[8],
            rate_model: &arr[9],
            vault: &arr[10],
            liquidity: &arr[11],
            liquidity_program: &arr[12],
            rewards_rate_model: &arr[13],
            token_program: &arr[14],
            associated_token_program: &arr[15],
            system_program: &arr[16],
        }
    }
}
pub const DEPOSIT_WITH_MIN_AMOUNT_OUT_IX_DISCM: [u8; 8usize] = [
    116, 144, 16, 97, 118, 109, 40, 119,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DepositWithMinAmountOutIxArgs {
    pub assets: u64,
    pub min_amount_out: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct DepositWithMinAmountOutIxData(pub DepositWithMinAmountOutIxArgs);
impl From<DepositWithMinAmountOutIxArgs> for DepositWithMinAmountOutIxData {
    fn from(args: DepositWithMinAmountOutIxArgs) -> Self {
        Self(args)
    }
}
impl DepositWithMinAmountOutIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != DEPOSIT_WITH_MIN_AMOUNT_OUT_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let assets: u64 = crate::borsh_de_or_default(&mut reader)?;
        let min_amount_out: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(DepositWithMinAmountOutIxArgs {
                assets,
                min_amount_out,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&DEPOSIT_WITH_MIN_AMOUNT_OUT_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.assets, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.min_amount_out, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn deposit_with_min_amount_out_ix_with_program_id(
    program_id: Pubkey,
    keys: DepositWithMinAmountOutKeys,
    args: DepositWithMinAmountOutIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; DEPOSIT_WITH_MIN_AMOUNT_OUT_IX_ACCOUNTS_LEN] = keys.into();
    let data: DepositWithMinAmountOutIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn deposit_with_min_amount_out_ix(
    keys: DepositWithMinAmountOutKeys,
    args: DepositWithMinAmountOutIxArgs,
) -> std::io::Result<Instruction> {
    deposit_with_min_amount_out_ix_with_program_id(JUPITER_LEND_PROGRAM_ID, keys, args)
}
pub fn deposit_with_min_amount_out_invoke_with_program_id(
    program_id: Pubkey,
    accounts: DepositWithMinAmountOutAccounts<'_, '_>,
    args: DepositWithMinAmountOutIxArgs,
) -> ProgramResult {
    let keys: DepositWithMinAmountOutKeys = accounts.into();
    let ix = deposit_with_min_amount_out_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn deposit_with_min_amount_out_invoke(
    accounts: DepositWithMinAmountOutAccounts<'_, '_>,
    args: DepositWithMinAmountOutIxArgs,
) -> ProgramResult {
    deposit_with_min_amount_out_invoke_with_program_id(
        JUPITER_LEND_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn deposit_with_min_amount_out_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: DepositWithMinAmountOutAccounts<'_, '_>,
    args: DepositWithMinAmountOutIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: DepositWithMinAmountOutKeys = accounts.into();
    let ix = deposit_with_min_amount_out_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn deposit_with_min_amount_out_invoke_signed(
    accounts: DepositWithMinAmountOutAccounts<'_, '_>,
    args: DepositWithMinAmountOutIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    deposit_with_min_amount_out_invoke_signed_with_program_id(
        JUPITER_LEND_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn deposit_with_min_amount_out_verify_account_keys(
    accounts: DepositWithMinAmountOutAccounts<'_, '_>,
    keys: DepositWithMinAmountOutKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.signer.key, keys.signer),
        (*accounts.depositor_token_account.key, keys.depositor_token_account),
        (*accounts.recipient_token_account.key, keys.recipient_token_account),
        (*accounts.mint.key, keys.mint),
        (*accounts.lending_admin.key, keys.lending_admin),
        (*accounts.lending.key, keys.lending),
        (*accounts.f_token_mint.key, keys.f_token_mint),
        (
            *accounts.supply_token_reserves_liquidity.key,
            keys.supply_token_reserves_liquidity,
        ),
        (
            *accounts.lending_supply_position_on_liquidity.key,
            keys.lending_supply_position_on_liquidity,
        ),
        (*accounts.rate_model.key, keys.rate_model),
        (*accounts.vault.key, keys.vault),
        (*accounts.liquidity.key, keys.liquidity),
        (*accounts.liquidity_program.key, keys.liquidity_program),
        (*accounts.rewards_rate_model.key, keys.rewards_rate_model),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.associated_token_program.key, keys.associated_token_program),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn deposit_with_min_amount_out_verify_writable_privileges<'me, 'info>(
    accounts: DepositWithMinAmountOutAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.signer,
        accounts.depositor_token_account,
        accounts.recipient_token_account,
        accounts.lending,
        accounts.f_token_mint,
        accounts.supply_token_reserves_liquidity,
        accounts.lending_supply_position_on_liquidity,
        accounts.vault,
        accounts.liquidity,
        accounts.liquidity_program,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn deposit_with_min_amount_out_verify_signer_privileges<'me, 'info>(
    accounts: DepositWithMinAmountOutAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.signer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn deposit_with_min_amount_out_verify_account_privileges<'me, 'info>(
    accounts: DepositWithMinAmountOutAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    deposit_with_min_amount_out_verify_writable_privileges(accounts)?;
    deposit_with_min_amount_out_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const INIT_LENDING_IX_ACCOUNTS_LEN: usize = 12;
#[derive(Copy, Clone, Debug)]
pub struct InitLendingAccounts<'me, 'info> {
    pub signer: &'me AccountInfo<'info>,
    pub lending_admin: &'me AccountInfo<'info>,
    pub mint: &'me AccountInfo<'info>,
    pub f_token_mint: &'me AccountInfo<'info>,
    pub metadata_account: &'me AccountInfo<'info>,
    pub lending: &'me AccountInfo<'info>,
    pub token_reserves_liquidity: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub sysvar_instruction: &'me AccountInfo<'info>,
    pub metadata_program: &'me AccountInfo<'info>,
    pub rent: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct InitLendingKeys {
    pub signer: Pubkey,
    pub lending_admin: Pubkey,
    pub mint: Pubkey,
    pub f_token_mint: Pubkey,
    pub metadata_account: Pubkey,
    pub lending: Pubkey,
    pub token_reserves_liquidity: Pubkey,
    pub token_program: Pubkey,
    pub system_program: Pubkey,
    pub sysvar_instruction: Pubkey,
    pub metadata_program: Pubkey,
    pub rent: Pubkey,
}
impl From<InitLendingAccounts<'_, '_>> for InitLendingKeys {
    fn from(accounts: InitLendingAccounts) -> Self {
        Self {
            signer: *accounts.signer.key,
            lending_admin: *accounts.lending_admin.key,
            mint: *accounts.mint.key,
            f_token_mint: *accounts.f_token_mint.key,
            metadata_account: *accounts.metadata_account.key,
            lending: *accounts.lending.key,
            token_reserves_liquidity: *accounts.token_reserves_liquidity.key,
            token_program: *accounts.token_program.key,
            system_program: *accounts.system_program.key,
            sysvar_instruction: *accounts.sysvar_instruction.key,
            metadata_program: *accounts.metadata_program.key,
            rent: *accounts.rent.key,
        }
    }
}
impl From<InitLendingKeys> for [AccountMeta; INIT_LENDING_IX_ACCOUNTS_LEN] {
    fn from(keys: InitLendingKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.signer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.lending_admin,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.f_token_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.metadata_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.lending,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_reserves_liquidity,
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
                pubkey: keys.sysvar_instruction,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.metadata_program,
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
impl From<[Pubkey; INIT_LENDING_IX_ACCOUNTS_LEN]> for InitLendingKeys {
    fn from(pubkeys: [Pubkey; INIT_LENDING_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            signer: pubkeys[0],
            lending_admin: pubkeys[1],
            mint: pubkeys[2],
            f_token_mint: pubkeys[3],
            metadata_account: pubkeys[4],
            lending: pubkeys[5],
            token_reserves_liquidity: pubkeys[6],
            token_program: pubkeys[7],
            system_program: pubkeys[8],
            sysvar_instruction: pubkeys[9],
            metadata_program: pubkeys[10],
            rent: pubkeys[11],
        }
    }
}
impl<'info> From<InitLendingAccounts<'_, 'info>>
for [AccountInfo<'info>; INIT_LENDING_IX_ACCOUNTS_LEN] {
    fn from(accounts: InitLendingAccounts<'_, 'info>) -> Self {
        [
            accounts.signer.clone(),
            accounts.lending_admin.clone(),
            accounts.mint.clone(),
            accounts.f_token_mint.clone(),
            accounts.metadata_account.clone(),
            accounts.lending.clone(),
            accounts.token_reserves_liquidity.clone(),
            accounts.token_program.clone(),
            accounts.system_program.clone(),
            accounts.sysvar_instruction.clone(),
            accounts.metadata_program.clone(),
            accounts.rent.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; INIT_LENDING_IX_ACCOUNTS_LEN]>
for InitLendingAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; INIT_LENDING_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            signer: &arr[0],
            lending_admin: &arr[1],
            mint: &arr[2],
            f_token_mint: &arr[3],
            metadata_account: &arr[4],
            lending: &arr[5],
            token_reserves_liquidity: &arr[6],
            token_program: &arr[7],
            system_program: &arr[8],
            sysvar_instruction: &arr[9],
            metadata_program: &arr[10],
            rent: &arr[11],
        }
    }
}
pub const INIT_LENDING_IX_DISCM: [u8; 8usize] = [156, 224, 67, 46, 89, 189, 157, 209];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct InitLendingIxArgs {
    pub symbol: String,
    pub liquidity_program: Pubkey,
}
#[derive(Clone, Debug, PartialEq)]
pub struct InitLendingIxData(pub InitLendingIxArgs);
impl From<InitLendingIxArgs> for InitLendingIxData {
    fn from(args: InitLendingIxArgs) -> Self {
        Self(args)
    }
}
impl InitLendingIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INIT_LENDING_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let symbol: String = crate::borsh_de_or_default(&mut reader)?;
        let liquidity_program: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(InitLendingIxArgs {
                symbol,
                liquidity_program,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INIT_LENDING_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.symbol, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.liquidity_program, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn init_lending_ix_with_program_id(
    program_id: Pubkey,
    keys: InitLendingKeys,
    args: InitLendingIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; INIT_LENDING_IX_ACCOUNTS_LEN] = keys.into();
    let data: InitLendingIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn init_lending_ix(
    keys: InitLendingKeys,
    args: InitLendingIxArgs,
) -> std::io::Result<Instruction> {
    init_lending_ix_with_program_id(JUPITER_LEND_PROGRAM_ID, keys, args)
}
pub fn init_lending_invoke_with_program_id(
    program_id: Pubkey,
    accounts: InitLendingAccounts<'_, '_>,
    args: InitLendingIxArgs,
) -> ProgramResult {
    let keys: InitLendingKeys = accounts.into();
    let ix = init_lending_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn init_lending_invoke(
    accounts: InitLendingAccounts<'_, '_>,
    args: InitLendingIxArgs,
) -> ProgramResult {
    init_lending_invoke_with_program_id(JUPITER_LEND_PROGRAM_ID, accounts, args)
}
pub fn init_lending_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: InitLendingAccounts<'_, '_>,
    args: InitLendingIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: InitLendingKeys = accounts.into();
    let ix = init_lending_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn init_lending_invoke_signed(
    accounts: InitLendingAccounts<'_, '_>,
    args: InitLendingIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    init_lending_invoke_signed_with_program_id(
        JUPITER_LEND_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn init_lending_verify_account_keys(
    accounts: InitLendingAccounts<'_, '_>,
    keys: InitLendingKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.signer.key, keys.signer),
        (*accounts.lending_admin.key, keys.lending_admin),
        (*accounts.mint.key, keys.mint),
        (*accounts.f_token_mint.key, keys.f_token_mint),
        (*accounts.metadata_account.key, keys.metadata_account),
        (*accounts.lending.key, keys.lending),
        (*accounts.token_reserves_liquidity.key, keys.token_reserves_liquidity),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.sysvar_instruction.key, keys.sysvar_instruction),
        (*accounts.metadata_program.key, keys.metadata_program),
        (*accounts.rent.key, keys.rent),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn init_lending_verify_writable_privileges<'me, 'info>(
    accounts: InitLendingAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.signer,
        accounts.lending_admin,
        accounts.f_token_mint,
        accounts.metadata_account,
        accounts.lending,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn init_lending_verify_signer_privileges<'me, 'info>(
    accounts: InitLendingAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.signer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn init_lending_verify_account_privileges<'me, 'info>(
    accounts: InitLendingAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    init_lending_verify_writable_privileges(accounts)?;
    init_lending_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const INIT_LENDING_ADMIN_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct InitLendingAdminAccounts<'me, 'info> {
    pub authority: &'me AccountInfo<'info>,
    pub lending_admin: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct InitLendingAdminKeys {
    pub authority: Pubkey,
    pub lending_admin: Pubkey,
    pub system_program: Pubkey,
}
impl From<InitLendingAdminAccounts<'_, '_>> for InitLendingAdminKeys {
    fn from(accounts: InitLendingAdminAccounts) -> Self {
        Self {
            authority: *accounts.authority.key,
            lending_admin: *accounts.lending_admin.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<InitLendingAdminKeys> for [AccountMeta; INIT_LENDING_ADMIN_IX_ACCOUNTS_LEN] {
    fn from(keys: InitLendingAdminKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.authority,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.lending_admin,
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
impl From<[Pubkey; INIT_LENDING_ADMIN_IX_ACCOUNTS_LEN]> for InitLendingAdminKeys {
    fn from(pubkeys: [Pubkey; INIT_LENDING_ADMIN_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            authority: pubkeys[0],
            lending_admin: pubkeys[1],
            system_program: pubkeys[2],
        }
    }
}
impl<'info> From<InitLendingAdminAccounts<'_, 'info>>
for [AccountInfo<'info>; INIT_LENDING_ADMIN_IX_ACCOUNTS_LEN] {
    fn from(accounts: InitLendingAdminAccounts<'_, 'info>) -> Self {
        [
            accounts.authority.clone(),
            accounts.lending_admin.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; INIT_LENDING_ADMIN_IX_ACCOUNTS_LEN]>
for InitLendingAdminAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; INIT_LENDING_ADMIN_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            authority: &arr[0],
            lending_admin: &arr[1],
            system_program: &arr[2],
        }
    }
}
pub const INIT_LENDING_ADMIN_IX_DISCM: [u8; 8usize] = [
    203, 185, 241, 165, 56, 254, 33, 9,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct InitLendingAdminIxArgs {
    pub liquidity_program: Pubkey,
    pub rebalancer: Pubkey,
    pub authority: Pubkey,
}
#[derive(Clone, Debug, PartialEq)]
pub struct InitLendingAdminIxData(pub InitLendingAdminIxArgs);
impl From<InitLendingAdminIxArgs> for InitLendingAdminIxData {
    fn from(args: InitLendingAdminIxArgs) -> Self {
        Self(args)
    }
}
impl InitLendingAdminIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INIT_LENDING_ADMIN_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let liquidity_program: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let rebalancer: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let authority: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(InitLendingAdminIxArgs {
                liquidity_program,
                rebalancer,
                authority,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INIT_LENDING_ADMIN_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.liquidity_program, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.rebalancer, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.authority, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn init_lending_admin_ix_with_program_id(
    program_id: Pubkey,
    keys: InitLendingAdminKeys,
    args: InitLendingAdminIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; INIT_LENDING_ADMIN_IX_ACCOUNTS_LEN] = keys.into();
    let data: InitLendingAdminIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn init_lending_admin_ix(
    keys: InitLendingAdminKeys,
    args: InitLendingAdminIxArgs,
) -> std::io::Result<Instruction> {
    init_lending_admin_ix_with_program_id(JUPITER_LEND_PROGRAM_ID, keys, args)
}
pub fn init_lending_admin_invoke_with_program_id(
    program_id: Pubkey,
    accounts: InitLendingAdminAccounts<'_, '_>,
    args: InitLendingAdminIxArgs,
) -> ProgramResult {
    let keys: InitLendingAdminKeys = accounts.into();
    let ix = init_lending_admin_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn init_lending_admin_invoke(
    accounts: InitLendingAdminAccounts<'_, '_>,
    args: InitLendingAdminIxArgs,
) -> ProgramResult {
    init_lending_admin_invoke_with_program_id(JUPITER_LEND_PROGRAM_ID, accounts, args)
}
pub fn init_lending_admin_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: InitLendingAdminAccounts<'_, '_>,
    args: InitLendingAdminIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: InitLendingAdminKeys = accounts.into();
    let ix = init_lending_admin_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn init_lending_admin_invoke_signed(
    accounts: InitLendingAdminAccounts<'_, '_>,
    args: InitLendingAdminIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    init_lending_admin_invoke_signed_with_program_id(
        JUPITER_LEND_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn init_lending_admin_verify_account_keys(
    accounts: InitLendingAdminAccounts<'_, '_>,
    keys: InitLendingAdminKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.authority.key, keys.authority),
        (*accounts.lending_admin.key, keys.lending_admin),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn init_lending_admin_verify_writable_privileges<'me, 'info>(
    accounts: InitLendingAdminAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.authority, accounts.lending_admin] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn init_lending_admin_verify_signer_privileges<'me, 'info>(
    accounts: InitLendingAdminAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn init_lending_admin_verify_account_privileges<'me, 'info>(
    accounts: InitLendingAdminAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    init_lending_admin_verify_writable_privileges(accounts)?;
    init_lending_admin_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const MINT_IX_ACCOUNTS_LEN: usize = 17;
#[derive(Copy, Clone, Debug)]
pub struct MintAccounts<'me, 'info> {
    pub signer: &'me AccountInfo<'info>,
    pub depositor_token_account: &'me AccountInfo<'info>,
    pub recipient_token_account: &'me AccountInfo<'info>,
    pub mint: &'me AccountInfo<'info>,
    pub lending_admin: &'me AccountInfo<'info>,
    pub lending: &'me AccountInfo<'info>,
    pub f_token_mint: &'me AccountInfo<'info>,
    pub supply_token_reserves_liquidity: &'me AccountInfo<'info>,
    pub lending_supply_position_on_liquidity: &'me AccountInfo<'info>,
    pub rate_model: &'me AccountInfo<'info>,
    pub vault: &'me AccountInfo<'info>,
    pub liquidity: &'me AccountInfo<'info>,
    pub liquidity_program: &'me AccountInfo<'info>,
    pub rewards_rate_model: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MintKeys {
    pub signer: Pubkey,
    pub depositor_token_account: Pubkey,
    pub recipient_token_account: Pubkey,
    pub mint: Pubkey,
    pub lending_admin: Pubkey,
    pub lending: Pubkey,
    pub f_token_mint: Pubkey,
    pub supply_token_reserves_liquidity: Pubkey,
    pub lending_supply_position_on_liquidity: Pubkey,
    pub rate_model: Pubkey,
    pub vault: Pubkey,
    pub liquidity: Pubkey,
    pub liquidity_program: Pubkey,
    pub rewards_rate_model: Pubkey,
    pub token_program: Pubkey,
    pub associated_token_program: Pubkey,
    pub system_program: Pubkey,
}
impl From<MintAccounts<'_, '_>> for MintKeys {
    fn from(accounts: MintAccounts) -> Self {
        Self {
            signer: *accounts.signer.key,
            depositor_token_account: *accounts.depositor_token_account.key,
            recipient_token_account: *accounts.recipient_token_account.key,
            mint: *accounts.mint.key,
            lending_admin: *accounts.lending_admin.key,
            lending: *accounts.lending.key,
            f_token_mint: *accounts.f_token_mint.key,
            supply_token_reserves_liquidity: *accounts
                .supply_token_reserves_liquidity
                .key,
            lending_supply_position_on_liquidity: *accounts
                .lending_supply_position_on_liquidity
                .key,
            rate_model: *accounts.rate_model.key,
            vault: *accounts.vault.key,
            liquidity: *accounts.liquidity.key,
            liquidity_program: *accounts.liquidity_program.key,
            rewards_rate_model: *accounts.rewards_rate_model.key,
            token_program: *accounts.token_program.key,
            associated_token_program: *accounts.associated_token_program.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<MintKeys> for [AccountMeta; MINT_IX_ACCOUNTS_LEN] {
    fn from(keys: MintKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.signer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.depositor_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.recipient_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.lending_admin,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.lending,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.f_token_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.supply_token_reserves_liquidity,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.lending_supply_position_on_liquidity,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.rate_model,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.liquidity,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.liquidity_program,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.rewards_rate_model,
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
        ]
    }
}
impl From<[Pubkey; MINT_IX_ACCOUNTS_LEN]> for MintKeys {
    fn from(pubkeys: [Pubkey; MINT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            signer: pubkeys[0],
            depositor_token_account: pubkeys[1],
            recipient_token_account: pubkeys[2],
            mint: pubkeys[3],
            lending_admin: pubkeys[4],
            lending: pubkeys[5],
            f_token_mint: pubkeys[6],
            supply_token_reserves_liquidity: pubkeys[7],
            lending_supply_position_on_liquidity: pubkeys[8],
            rate_model: pubkeys[9],
            vault: pubkeys[10],
            liquidity: pubkeys[11],
            liquidity_program: pubkeys[12],
            rewards_rate_model: pubkeys[13],
            token_program: pubkeys[14],
            associated_token_program: pubkeys[15],
            system_program: pubkeys[16],
        }
    }
}
impl<'info> From<MintAccounts<'_, 'info>>
for [AccountInfo<'info>; MINT_IX_ACCOUNTS_LEN] {
    fn from(accounts: MintAccounts<'_, 'info>) -> Self {
        [
            accounts.signer.clone(),
            accounts.depositor_token_account.clone(),
            accounts.recipient_token_account.clone(),
            accounts.mint.clone(),
            accounts.lending_admin.clone(),
            accounts.lending.clone(),
            accounts.f_token_mint.clone(),
            accounts.supply_token_reserves_liquidity.clone(),
            accounts.lending_supply_position_on_liquidity.clone(),
            accounts.rate_model.clone(),
            accounts.vault.clone(),
            accounts.liquidity.clone(),
            accounts.liquidity_program.clone(),
            accounts.rewards_rate_model.clone(),
            accounts.token_program.clone(),
            accounts.associated_token_program.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; MINT_IX_ACCOUNTS_LEN]>
for MintAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; MINT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            signer: &arr[0],
            depositor_token_account: &arr[1],
            recipient_token_account: &arr[2],
            mint: &arr[3],
            lending_admin: &arr[4],
            lending: &arr[5],
            f_token_mint: &arr[6],
            supply_token_reserves_liquidity: &arr[7],
            lending_supply_position_on_liquidity: &arr[8],
            rate_model: &arr[9],
            vault: &arr[10],
            liquidity: &arr[11],
            liquidity_program: &arr[12],
            rewards_rate_model: &arr[13],
            token_program: &arr[14],
            associated_token_program: &arr[15],
            system_program: &arr[16],
        }
    }
}
pub const MINT_IX_DISCM: [u8; 8usize] = [51, 57, 225, 47, 182, 146, 137, 166];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MintIxArgs {
    pub shares: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct MintIxData(pub MintIxArgs);
impl From<MintIxArgs> for MintIxData {
    fn from(args: MintIxArgs) -> Self {
        Self(args)
    }
}
impl MintIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != MINT_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let shares: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(Self(MintIxArgs { shares }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&MINT_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.shares, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn mint_ix_with_program_id(
    program_id: Pubkey,
    keys: MintKeys,
    args: MintIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; MINT_IX_ACCOUNTS_LEN] = keys.into();
    let data: MintIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn mint_ix(keys: MintKeys, args: MintIxArgs) -> std::io::Result<Instruction> {
    mint_ix_with_program_id(JUPITER_LEND_PROGRAM_ID, keys, args)
}
pub fn mint_invoke_with_program_id(
    program_id: Pubkey,
    accounts: MintAccounts<'_, '_>,
    args: MintIxArgs,
) -> ProgramResult {
    let keys: MintKeys = accounts.into();
    let ix = mint_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn mint_invoke(accounts: MintAccounts<'_, '_>, args: MintIxArgs) -> ProgramResult {
    mint_invoke_with_program_id(JUPITER_LEND_PROGRAM_ID, accounts, args)
}
pub fn mint_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: MintAccounts<'_, '_>,
    args: MintIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: MintKeys = accounts.into();
    let ix = mint_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn mint_invoke_signed(
    accounts: MintAccounts<'_, '_>,
    args: MintIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    mint_invoke_signed_with_program_id(JUPITER_LEND_PROGRAM_ID, accounts, args, seeds)
}
pub fn mint_verify_account_keys(
    accounts: MintAccounts<'_, '_>,
    keys: MintKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.signer.key, keys.signer),
        (*accounts.depositor_token_account.key, keys.depositor_token_account),
        (*accounts.recipient_token_account.key, keys.recipient_token_account),
        (*accounts.mint.key, keys.mint),
        (*accounts.lending_admin.key, keys.lending_admin),
        (*accounts.lending.key, keys.lending),
        (*accounts.f_token_mint.key, keys.f_token_mint),
        (
            *accounts.supply_token_reserves_liquidity.key,
            keys.supply_token_reserves_liquidity,
        ),
        (
            *accounts.lending_supply_position_on_liquidity.key,
            keys.lending_supply_position_on_liquidity,
        ),
        (*accounts.rate_model.key, keys.rate_model),
        (*accounts.vault.key, keys.vault),
        (*accounts.liquidity.key, keys.liquidity),
        (*accounts.liquidity_program.key, keys.liquidity_program),
        (*accounts.rewards_rate_model.key, keys.rewards_rate_model),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.associated_token_program.key, keys.associated_token_program),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn mint_verify_writable_privileges<'me, 'info>(
    accounts: MintAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.signer,
        accounts.depositor_token_account,
        accounts.recipient_token_account,
        accounts.lending,
        accounts.f_token_mint,
        accounts.supply_token_reserves_liquidity,
        accounts.lending_supply_position_on_liquidity,
        accounts.vault,
        accounts.liquidity,
        accounts.liquidity_program,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn mint_verify_signer_privileges<'me, 'info>(
    accounts: MintAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.signer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn mint_verify_account_privileges<'me, 'info>(
    accounts: MintAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    mint_verify_writable_privileges(accounts)?;
    mint_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const MINT_WITH_MAX_ASSETS_IX_ACCOUNTS_LEN: usize = 17;
#[derive(Copy, Clone, Debug)]
pub struct MintWithMaxAssetsAccounts<'me, 'info> {
    pub signer: &'me AccountInfo<'info>,
    pub depositor_token_account: &'me AccountInfo<'info>,
    pub recipient_token_account: &'me AccountInfo<'info>,
    pub mint: &'me AccountInfo<'info>,
    pub lending_admin: &'me AccountInfo<'info>,
    pub lending: &'me AccountInfo<'info>,
    pub f_token_mint: &'me AccountInfo<'info>,
    pub supply_token_reserves_liquidity: &'me AccountInfo<'info>,
    pub lending_supply_position_on_liquidity: &'me AccountInfo<'info>,
    pub rate_model: &'me AccountInfo<'info>,
    pub vault: &'me AccountInfo<'info>,
    pub liquidity: &'me AccountInfo<'info>,
    pub liquidity_program: &'me AccountInfo<'info>,
    pub rewards_rate_model: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MintWithMaxAssetsKeys {
    pub signer: Pubkey,
    pub depositor_token_account: Pubkey,
    pub recipient_token_account: Pubkey,
    pub mint: Pubkey,
    pub lending_admin: Pubkey,
    pub lending: Pubkey,
    pub f_token_mint: Pubkey,
    pub supply_token_reserves_liquidity: Pubkey,
    pub lending_supply_position_on_liquidity: Pubkey,
    pub rate_model: Pubkey,
    pub vault: Pubkey,
    pub liquidity: Pubkey,
    pub liquidity_program: Pubkey,
    pub rewards_rate_model: Pubkey,
    pub token_program: Pubkey,
    pub associated_token_program: Pubkey,
    pub system_program: Pubkey,
}
impl From<MintWithMaxAssetsAccounts<'_, '_>> for MintWithMaxAssetsKeys {
    fn from(accounts: MintWithMaxAssetsAccounts) -> Self {
        Self {
            signer: *accounts.signer.key,
            depositor_token_account: *accounts.depositor_token_account.key,
            recipient_token_account: *accounts.recipient_token_account.key,
            mint: *accounts.mint.key,
            lending_admin: *accounts.lending_admin.key,
            lending: *accounts.lending.key,
            f_token_mint: *accounts.f_token_mint.key,
            supply_token_reserves_liquidity: *accounts
                .supply_token_reserves_liquidity
                .key,
            lending_supply_position_on_liquidity: *accounts
                .lending_supply_position_on_liquidity
                .key,
            rate_model: *accounts.rate_model.key,
            vault: *accounts.vault.key,
            liquidity: *accounts.liquidity.key,
            liquidity_program: *accounts.liquidity_program.key,
            rewards_rate_model: *accounts.rewards_rate_model.key,
            token_program: *accounts.token_program.key,
            associated_token_program: *accounts.associated_token_program.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<MintWithMaxAssetsKeys>
for [AccountMeta; MINT_WITH_MAX_ASSETS_IX_ACCOUNTS_LEN] {
    fn from(keys: MintWithMaxAssetsKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.signer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.depositor_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.recipient_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.lending_admin,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.lending,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.f_token_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.supply_token_reserves_liquidity,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.lending_supply_position_on_liquidity,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.rate_model,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.liquidity,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.liquidity_program,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.rewards_rate_model,
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
        ]
    }
}
impl From<[Pubkey; MINT_WITH_MAX_ASSETS_IX_ACCOUNTS_LEN]> for MintWithMaxAssetsKeys {
    fn from(pubkeys: [Pubkey; MINT_WITH_MAX_ASSETS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            signer: pubkeys[0],
            depositor_token_account: pubkeys[1],
            recipient_token_account: pubkeys[2],
            mint: pubkeys[3],
            lending_admin: pubkeys[4],
            lending: pubkeys[5],
            f_token_mint: pubkeys[6],
            supply_token_reserves_liquidity: pubkeys[7],
            lending_supply_position_on_liquidity: pubkeys[8],
            rate_model: pubkeys[9],
            vault: pubkeys[10],
            liquidity: pubkeys[11],
            liquidity_program: pubkeys[12],
            rewards_rate_model: pubkeys[13],
            token_program: pubkeys[14],
            associated_token_program: pubkeys[15],
            system_program: pubkeys[16],
        }
    }
}
impl<'info> From<MintWithMaxAssetsAccounts<'_, 'info>>
for [AccountInfo<'info>; MINT_WITH_MAX_ASSETS_IX_ACCOUNTS_LEN] {
    fn from(accounts: MintWithMaxAssetsAccounts<'_, 'info>) -> Self {
        [
            accounts.signer.clone(),
            accounts.depositor_token_account.clone(),
            accounts.recipient_token_account.clone(),
            accounts.mint.clone(),
            accounts.lending_admin.clone(),
            accounts.lending.clone(),
            accounts.f_token_mint.clone(),
            accounts.supply_token_reserves_liquidity.clone(),
            accounts.lending_supply_position_on_liquidity.clone(),
            accounts.rate_model.clone(),
            accounts.vault.clone(),
            accounts.liquidity.clone(),
            accounts.liquidity_program.clone(),
            accounts.rewards_rate_model.clone(),
            accounts.token_program.clone(),
            accounts.associated_token_program.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; MINT_WITH_MAX_ASSETS_IX_ACCOUNTS_LEN]>
for MintWithMaxAssetsAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; MINT_WITH_MAX_ASSETS_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            signer: &arr[0],
            depositor_token_account: &arr[1],
            recipient_token_account: &arr[2],
            mint: &arr[3],
            lending_admin: &arr[4],
            lending: &arr[5],
            f_token_mint: &arr[6],
            supply_token_reserves_liquidity: &arr[7],
            lending_supply_position_on_liquidity: &arr[8],
            rate_model: &arr[9],
            vault: &arr[10],
            liquidity: &arr[11],
            liquidity_program: &arr[12],
            rewards_rate_model: &arr[13],
            token_program: &arr[14],
            associated_token_program: &arr[15],
            system_program: &arr[16],
        }
    }
}
pub const MINT_WITH_MAX_ASSETS_IX_DISCM: [u8; 8usize] = [
    6, 94, 69, 122, 30, 179, 146, 171,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MintWithMaxAssetsIxArgs {
    pub shares: u64,
    pub max_assets: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct MintWithMaxAssetsIxData(pub MintWithMaxAssetsIxArgs);
impl From<MintWithMaxAssetsIxArgs> for MintWithMaxAssetsIxData {
    fn from(args: MintWithMaxAssetsIxArgs) -> Self {
        Self(args)
    }
}
impl MintWithMaxAssetsIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != MINT_WITH_MAX_ASSETS_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let shares: u64 = crate::borsh_de_or_default(&mut reader)?;
        let max_assets: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(MintWithMaxAssetsIxArgs {
                shares,
                max_assets,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&MINT_WITH_MAX_ASSETS_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.shares, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.max_assets, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn mint_with_max_assets_ix_with_program_id(
    program_id: Pubkey,
    keys: MintWithMaxAssetsKeys,
    args: MintWithMaxAssetsIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; MINT_WITH_MAX_ASSETS_IX_ACCOUNTS_LEN] = keys.into();
    let data: MintWithMaxAssetsIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn mint_with_max_assets_ix(
    keys: MintWithMaxAssetsKeys,
    args: MintWithMaxAssetsIxArgs,
) -> std::io::Result<Instruction> {
    mint_with_max_assets_ix_with_program_id(JUPITER_LEND_PROGRAM_ID, keys, args)
}
pub fn mint_with_max_assets_invoke_with_program_id(
    program_id: Pubkey,
    accounts: MintWithMaxAssetsAccounts<'_, '_>,
    args: MintWithMaxAssetsIxArgs,
) -> ProgramResult {
    let keys: MintWithMaxAssetsKeys = accounts.into();
    let ix = mint_with_max_assets_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn mint_with_max_assets_invoke(
    accounts: MintWithMaxAssetsAccounts<'_, '_>,
    args: MintWithMaxAssetsIxArgs,
) -> ProgramResult {
    mint_with_max_assets_invoke_with_program_id(JUPITER_LEND_PROGRAM_ID, accounts, args)
}
pub fn mint_with_max_assets_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: MintWithMaxAssetsAccounts<'_, '_>,
    args: MintWithMaxAssetsIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: MintWithMaxAssetsKeys = accounts.into();
    let ix = mint_with_max_assets_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn mint_with_max_assets_invoke_signed(
    accounts: MintWithMaxAssetsAccounts<'_, '_>,
    args: MintWithMaxAssetsIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    mint_with_max_assets_invoke_signed_with_program_id(
        JUPITER_LEND_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn mint_with_max_assets_verify_account_keys(
    accounts: MintWithMaxAssetsAccounts<'_, '_>,
    keys: MintWithMaxAssetsKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.signer.key, keys.signer),
        (*accounts.depositor_token_account.key, keys.depositor_token_account),
        (*accounts.recipient_token_account.key, keys.recipient_token_account),
        (*accounts.mint.key, keys.mint),
        (*accounts.lending_admin.key, keys.lending_admin),
        (*accounts.lending.key, keys.lending),
        (*accounts.f_token_mint.key, keys.f_token_mint),
        (
            *accounts.supply_token_reserves_liquidity.key,
            keys.supply_token_reserves_liquidity,
        ),
        (
            *accounts.lending_supply_position_on_liquidity.key,
            keys.lending_supply_position_on_liquidity,
        ),
        (*accounts.rate_model.key, keys.rate_model),
        (*accounts.vault.key, keys.vault),
        (*accounts.liquidity.key, keys.liquidity),
        (*accounts.liquidity_program.key, keys.liquidity_program),
        (*accounts.rewards_rate_model.key, keys.rewards_rate_model),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.associated_token_program.key, keys.associated_token_program),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn mint_with_max_assets_verify_writable_privileges<'me, 'info>(
    accounts: MintWithMaxAssetsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.signer,
        accounts.depositor_token_account,
        accounts.recipient_token_account,
        accounts.lending,
        accounts.f_token_mint,
        accounts.supply_token_reserves_liquidity,
        accounts.lending_supply_position_on_liquidity,
        accounts.vault,
        accounts.liquidity,
        accounts.liquidity_program,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn mint_with_max_assets_verify_signer_privileges<'me, 'info>(
    accounts: MintWithMaxAssetsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.signer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn mint_with_max_assets_verify_account_privileges<'me, 'info>(
    accounts: MintWithMaxAssetsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    mint_with_max_assets_verify_writable_privileges(accounts)?;
    mint_with_max_assets_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const REBALANCE_IX_ACCOUNTS_LEN: usize = 16;
#[derive(Copy, Clone, Debug)]
pub struct RebalanceAccounts<'me, 'info> {
    pub signer: &'me AccountInfo<'info>,
    pub depositor_token_account: &'me AccountInfo<'info>,
    pub lending_admin: &'me AccountInfo<'info>,
    pub lending: &'me AccountInfo<'info>,
    pub mint: &'me AccountInfo<'info>,
    pub f_token_mint: &'me AccountInfo<'info>,
    pub supply_token_reserves_liquidity: &'me AccountInfo<'info>,
    pub lending_supply_position_on_liquidity: &'me AccountInfo<'info>,
    pub rate_model: &'me AccountInfo<'info>,
    pub vault: &'me AccountInfo<'info>,
    pub liquidity: &'me AccountInfo<'info>,
    pub liquidity_program: &'me AccountInfo<'info>,
    pub rewards_rate_model: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct RebalanceKeys {
    pub signer: Pubkey,
    pub depositor_token_account: Pubkey,
    pub lending_admin: Pubkey,
    pub lending: Pubkey,
    pub mint: Pubkey,
    pub f_token_mint: Pubkey,
    pub supply_token_reserves_liquidity: Pubkey,
    pub lending_supply_position_on_liquidity: Pubkey,
    pub rate_model: Pubkey,
    pub vault: Pubkey,
    pub liquidity: Pubkey,
    pub liquidity_program: Pubkey,
    pub rewards_rate_model: Pubkey,
    pub token_program: Pubkey,
    pub associated_token_program: Pubkey,
    pub system_program: Pubkey,
}
impl From<RebalanceAccounts<'_, '_>> for RebalanceKeys {
    fn from(accounts: RebalanceAccounts) -> Self {
        Self {
            signer: *accounts.signer.key,
            depositor_token_account: *accounts.depositor_token_account.key,
            lending_admin: *accounts.lending_admin.key,
            lending: *accounts.lending.key,
            mint: *accounts.mint.key,
            f_token_mint: *accounts.f_token_mint.key,
            supply_token_reserves_liquidity: *accounts
                .supply_token_reserves_liquidity
                .key,
            lending_supply_position_on_liquidity: *accounts
                .lending_supply_position_on_liquidity
                .key,
            rate_model: *accounts.rate_model.key,
            vault: *accounts.vault.key,
            liquidity: *accounts.liquidity.key,
            liquidity_program: *accounts.liquidity_program.key,
            rewards_rate_model: *accounts.rewards_rate_model.key,
            token_program: *accounts.token_program.key,
            associated_token_program: *accounts.associated_token_program.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<RebalanceKeys> for [AccountMeta; REBALANCE_IX_ACCOUNTS_LEN] {
    fn from(keys: RebalanceKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.signer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.depositor_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.lending_admin,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.lending,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.f_token_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.supply_token_reserves_liquidity,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.lending_supply_position_on_liquidity,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.rate_model,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.liquidity,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.liquidity_program,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.rewards_rate_model,
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
        ]
    }
}
impl From<[Pubkey; REBALANCE_IX_ACCOUNTS_LEN]> for RebalanceKeys {
    fn from(pubkeys: [Pubkey; REBALANCE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            signer: pubkeys[0],
            depositor_token_account: pubkeys[1],
            lending_admin: pubkeys[2],
            lending: pubkeys[3],
            mint: pubkeys[4],
            f_token_mint: pubkeys[5],
            supply_token_reserves_liquidity: pubkeys[6],
            lending_supply_position_on_liquidity: pubkeys[7],
            rate_model: pubkeys[8],
            vault: pubkeys[9],
            liquidity: pubkeys[10],
            liquidity_program: pubkeys[11],
            rewards_rate_model: pubkeys[12],
            token_program: pubkeys[13],
            associated_token_program: pubkeys[14],
            system_program: pubkeys[15],
        }
    }
}
impl<'info> From<RebalanceAccounts<'_, 'info>>
for [AccountInfo<'info>; REBALANCE_IX_ACCOUNTS_LEN] {
    fn from(accounts: RebalanceAccounts<'_, 'info>) -> Self {
        [
            accounts.signer.clone(),
            accounts.depositor_token_account.clone(),
            accounts.lending_admin.clone(),
            accounts.lending.clone(),
            accounts.mint.clone(),
            accounts.f_token_mint.clone(),
            accounts.supply_token_reserves_liquidity.clone(),
            accounts.lending_supply_position_on_liquidity.clone(),
            accounts.rate_model.clone(),
            accounts.vault.clone(),
            accounts.liquidity.clone(),
            accounts.liquidity_program.clone(),
            accounts.rewards_rate_model.clone(),
            accounts.token_program.clone(),
            accounts.associated_token_program.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; REBALANCE_IX_ACCOUNTS_LEN]>
for RebalanceAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; REBALANCE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            signer: &arr[0],
            depositor_token_account: &arr[1],
            lending_admin: &arr[2],
            lending: &arr[3],
            mint: &arr[4],
            f_token_mint: &arr[5],
            supply_token_reserves_liquidity: &arr[6],
            lending_supply_position_on_liquidity: &arr[7],
            rate_model: &arr[8],
            vault: &arr[9],
            liquidity: &arr[10],
            liquidity_program: &arr[11],
            rewards_rate_model: &arr[12],
            token_program: &arr[13],
            associated_token_program: &arr[14],
            system_program: &arr[15],
        }
    }
}
pub const REBALANCE_IX_DISCM: [u8; 8usize] = [108, 158, 77, 9, 210, 52, 88, 62];
#[derive(Clone, Debug, PartialEq)]
pub struct RebalanceIxData;
impl RebalanceIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != REBALANCE_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&REBALANCE_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn rebalance_ix_with_program_id(
    program_id: Pubkey,
    keys: RebalanceKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; REBALANCE_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: RebalanceIxData.try_to_vec()?,
    })
}
pub fn rebalance_ix(keys: RebalanceKeys) -> std::io::Result<Instruction> {
    rebalance_ix_with_program_id(JUPITER_LEND_PROGRAM_ID, keys)
}
pub fn rebalance_invoke_with_program_id(
    program_id: Pubkey,
    accounts: RebalanceAccounts<'_, '_>,
) -> ProgramResult {
    let keys: RebalanceKeys = accounts.into();
    let ix = rebalance_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn rebalance_invoke(accounts: RebalanceAccounts<'_, '_>) -> ProgramResult {
    rebalance_invoke_with_program_id(JUPITER_LEND_PROGRAM_ID, accounts)
}
pub fn rebalance_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: RebalanceAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: RebalanceKeys = accounts.into();
    let ix = rebalance_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn rebalance_invoke_signed(
    accounts: RebalanceAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    rebalance_invoke_signed_with_program_id(JUPITER_LEND_PROGRAM_ID, accounts, seeds)
}
pub fn rebalance_verify_account_keys(
    accounts: RebalanceAccounts<'_, '_>,
    keys: RebalanceKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.signer.key, keys.signer),
        (*accounts.depositor_token_account.key, keys.depositor_token_account),
        (*accounts.lending_admin.key, keys.lending_admin),
        (*accounts.lending.key, keys.lending),
        (*accounts.mint.key, keys.mint),
        (*accounts.f_token_mint.key, keys.f_token_mint),
        (
            *accounts.supply_token_reserves_liquidity.key,
            keys.supply_token_reserves_liquidity,
        ),
        (
            *accounts.lending_supply_position_on_liquidity.key,
            keys.lending_supply_position_on_liquidity,
        ),
        (*accounts.rate_model.key, keys.rate_model),
        (*accounts.vault.key, keys.vault),
        (*accounts.liquidity.key, keys.liquidity),
        (*accounts.liquidity_program.key, keys.liquidity_program),
        (*accounts.rewards_rate_model.key, keys.rewards_rate_model),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.associated_token_program.key, keys.associated_token_program),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn rebalance_verify_writable_privileges<'me, 'info>(
    accounts: RebalanceAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.signer,
        accounts.depositor_token_account,
        accounts.lending,
        accounts.f_token_mint,
        accounts.supply_token_reserves_liquidity,
        accounts.lending_supply_position_on_liquidity,
        accounts.rate_model,
        accounts.vault,
        accounts.liquidity,
        accounts.liquidity_program,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn rebalance_verify_signer_privileges<'me, 'info>(
    accounts: RebalanceAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.signer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn rebalance_verify_account_privileges<'me, 'info>(
    accounts: RebalanceAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    rebalance_verify_writable_privileges(accounts)?;
    rebalance_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const REDEEM_IX_ACCOUNTS_LEN: usize = 18;
#[derive(Copy, Clone, Debug)]
pub struct RedeemAccounts<'me, 'info> {
    pub signer: &'me AccountInfo<'info>,
    pub owner_token_account: &'me AccountInfo<'info>,
    pub recipient_token_account: &'me AccountInfo<'info>,
    pub lending_admin: &'me AccountInfo<'info>,
    pub lending: &'me AccountInfo<'info>,
    pub mint: &'me AccountInfo<'info>,
    pub f_token_mint: &'me AccountInfo<'info>,
    pub supply_token_reserves_liquidity: &'me AccountInfo<'info>,
    pub lending_supply_position_on_liquidity: &'me AccountInfo<'info>,
    pub rate_model: &'me AccountInfo<'info>,
    pub vault: &'me AccountInfo<'info>,
    pub claim_account: &'me AccountInfo<'info>,
    pub liquidity: &'me AccountInfo<'info>,
    pub liquidity_program: &'me AccountInfo<'info>,
    pub rewards_rate_model: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct RedeemKeys {
    pub signer: Pubkey,
    pub owner_token_account: Pubkey,
    pub recipient_token_account: Pubkey,
    pub lending_admin: Pubkey,
    pub lending: Pubkey,
    pub mint: Pubkey,
    pub f_token_mint: Pubkey,
    pub supply_token_reserves_liquidity: Pubkey,
    pub lending_supply_position_on_liquidity: Pubkey,
    pub rate_model: Pubkey,
    pub vault: Pubkey,
    pub claim_account: Pubkey,
    pub liquidity: Pubkey,
    pub liquidity_program: Pubkey,
    pub rewards_rate_model: Pubkey,
    pub token_program: Pubkey,
    pub associated_token_program: Pubkey,
    pub system_program: Pubkey,
}
impl From<RedeemAccounts<'_, '_>> for RedeemKeys {
    fn from(accounts: RedeemAccounts) -> Self {
        Self {
            signer: *accounts.signer.key,
            owner_token_account: *accounts.owner_token_account.key,
            recipient_token_account: *accounts.recipient_token_account.key,
            lending_admin: *accounts.lending_admin.key,
            lending: *accounts.lending.key,
            mint: *accounts.mint.key,
            f_token_mint: *accounts.f_token_mint.key,
            supply_token_reserves_liquidity: *accounts
                .supply_token_reserves_liquidity
                .key,
            lending_supply_position_on_liquidity: *accounts
                .lending_supply_position_on_liquidity
                .key,
            rate_model: *accounts.rate_model.key,
            vault: *accounts.vault.key,
            claim_account: *accounts.claim_account.key,
            liquidity: *accounts.liquidity.key,
            liquidity_program: *accounts.liquidity_program.key,
            rewards_rate_model: *accounts.rewards_rate_model.key,
            token_program: *accounts.token_program.key,
            associated_token_program: *accounts.associated_token_program.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<RedeemKeys> for [AccountMeta; REDEEM_IX_ACCOUNTS_LEN] {
    fn from(keys: RedeemKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.signer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.owner_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.recipient_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.lending_admin,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.lending,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.f_token_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.supply_token_reserves_liquidity,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.lending_supply_position_on_liquidity,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.rate_model,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.claim_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.liquidity,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.liquidity_program,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.rewards_rate_model,
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
        ]
    }
}
impl From<[Pubkey; REDEEM_IX_ACCOUNTS_LEN]> for RedeemKeys {
    fn from(pubkeys: [Pubkey; REDEEM_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            signer: pubkeys[0],
            owner_token_account: pubkeys[1],
            recipient_token_account: pubkeys[2],
            lending_admin: pubkeys[3],
            lending: pubkeys[4],
            mint: pubkeys[5],
            f_token_mint: pubkeys[6],
            supply_token_reserves_liquidity: pubkeys[7],
            lending_supply_position_on_liquidity: pubkeys[8],
            rate_model: pubkeys[9],
            vault: pubkeys[10],
            claim_account: pubkeys[11],
            liquidity: pubkeys[12],
            liquidity_program: pubkeys[13],
            rewards_rate_model: pubkeys[14],
            token_program: pubkeys[15],
            associated_token_program: pubkeys[16],
            system_program: pubkeys[17],
        }
    }
}
impl<'info> From<RedeemAccounts<'_, 'info>>
for [AccountInfo<'info>; REDEEM_IX_ACCOUNTS_LEN] {
    fn from(accounts: RedeemAccounts<'_, 'info>) -> Self {
        [
            accounts.signer.clone(),
            accounts.owner_token_account.clone(),
            accounts.recipient_token_account.clone(),
            accounts.lending_admin.clone(),
            accounts.lending.clone(),
            accounts.mint.clone(),
            accounts.f_token_mint.clone(),
            accounts.supply_token_reserves_liquidity.clone(),
            accounts.lending_supply_position_on_liquidity.clone(),
            accounts.rate_model.clone(),
            accounts.vault.clone(),
            accounts.claim_account.clone(),
            accounts.liquidity.clone(),
            accounts.liquidity_program.clone(),
            accounts.rewards_rate_model.clone(),
            accounts.token_program.clone(),
            accounts.associated_token_program.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; REDEEM_IX_ACCOUNTS_LEN]>
for RedeemAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; REDEEM_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            signer: &arr[0],
            owner_token_account: &arr[1],
            recipient_token_account: &arr[2],
            lending_admin: &arr[3],
            lending: &arr[4],
            mint: &arr[5],
            f_token_mint: &arr[6],
            supply_token_reserves_liquidity: &arr[7],
            lending_supply_position_on_liquidity: &arr[8],
            rate_model: &arr[9],
            vault: &arr[10],
            claim_account: &arr[11],
            liquidity: &arr[12],
            liquidity_program: &arr[13],
            rewards_rate_model: &arr[14],
            token_program: &arr[15],
            associated_token_program: &arr[16],
            system_program: &arr[17],
        }
    }
}
pub const REDEEM_IX_DISCM: [u8; 8usize] = [184, 12, 86, 149, 70, 196, 97, 225];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct RedeemIxArgs {
    pub shares: u64,
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
        let shares: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(Self(RedeemIxArgs { shares }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&REDEEM_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.shares, &mut writer)?;
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
    redeem_ix_with_program_id(JUPITER_LEND_PROGRAM_ID, keys, args)
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
    redeem_invoke_with_program_id(JUPITER_LEND_PROGRAM_ID, accounts, args)
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
    redeem_invoke_signed_with_program_id(JUPITER_LEND_PROGRAM_ID, accounts, args, seeds)
}
pub fn redeem_verify_account_keys(
    accounts: RedeemAccounts<'_, '_>,
    keys: RedeemKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.signer.key, keys.signer),
        (*accounts.owner_token_account.key, keys.owner_token_account),
        (*accounts.recipient_token_account.key, keys.recipient_token_account),
        (*accounts.lending_admin.key, keys.lending_admin),
        (*accounts.lending.key, keys.lending),
        (*accounts.mint.key, keys.mint),
        (*accounts.f_token_mint.key, keys.f_token_mint),
        (
            *accounts.supply_token_reserves_liquidity.key,
            keys.supply_token_reserves_liquidity,
        ),
        (
            *accounts.lending_supply_position_on_liquidity.key,
            keys.lending_supply_position_on_liquidity,
        ),
        (*accounts.rate_model.key, keys.rate_model),
        (*accounts.vault.key, keys.vault),
        (*accounts.claim_account.key, keys.claim_account),
        (*accounts.liquidity.key, keys.liquidity),
        (*accounts.liquidity_program.key, keys.liquidity_program),
        (*accounts.rewards_rate_model.key, keys.rewards_rate_model),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.associated_token_program.key, keys.associated_token_program),
        (*accounts.system_program.key, keys.system_program),
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
        accounts.signer,
        accounts.owner_token_account,
        accounts.recipient_token_account,
        accounts.lending,
        accounts.f_token_mint,
        accounts.supply_token_reserves_liquidity,
        accounts.lending_supply_position_on_liquidity,
        accounts.vault,
        accounts.claim_account,
        accounts.liquidity,
        accounts.liquidity_program,
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
    for should_be_signer in [accounts.signer] {
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
pub const REDEEM_WITH_MIN_AMOUNT_OUT_IX_ACCOUNTS_LEN: usize = 18;
#[derive(Copy, Clone, Debug)]
pub struct RedeemWithMinAmountOutAccounts<'me, 'info> {
    pub signer: &'me AccountInfo<'info>,
    pub owner_token_account: &'me AccountInfo<'info>,
    pub recipient_token_account: &'me AccountInfo<'info>,
    pub lending_admin: &'me AccountInfo<'info>,
    pub lending: &'me AccountInfo<'info>,
    pub mint: &'me AccountInfo<'info>,
    pub f_token_mint: &'me AccountInfo<'info>,
    pub supply_token_reserves_liquidity: &'me AccountInfo<'info>,
    pub lending_supply_position_on_liquidity: &'me AccountInfo<'info>,
    pub rate_model: &'me AccountInfo<'info>,
    pub vault: &'me AccountInfo<'info>,
    pub claim_account: &'me AccountInfo<'info>,
    pub liquidity: &'me AccountInfo<'info>,
    pub liquidity_program: &'me AccountInfo<'info>,
    pub rewards_rate_model: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct RedeemWithMinAmountOutKeys {
    pub signer: Pubkey,
    pub owner_token_account: Pubkey,
    pub recipient_token_account: Pubkey,
    pub lending_admin: Pubkey,
    pub lending: Pubkey,
    pub mint: Pubkey,
    pub f_token_mint: Pubkey,
    pub supply_token_reserves_liquidity: Pubkey,
    pub lending_supply_position_on_liquidity: Pubkey,
    pub rate_model: Pubkey,
    pub vault: Pubkey,
    pub claim_account: Pubkey,
    pub liquidity: Pubkey,
    pub liquidity_program: Pubkey,
    pub rewards_rate_model: Pubkey,
    pub token_program: Pubkey,
    pub associated_token_program: Pubkey,
    pub system_program: Pubkey,
}
impl From<RedeemWithMinAmountOutAccounts<'_, '_>> for RedeemWithMinAmountOutKeys {
    fn from(accounts: RedeemWithMinAmountOutAccounts) -> Self {
        Self {
            signer: *accounts.signer.key,
            owner_token_account: *accounts.owner_token_account.key,
            recipient_token_account: *accounts.recipient_token_account.key,
            lending_admin: *accounts.lending_admin.key,
            lending: *accounts.lending.key,
            mint: *accounts.mint.key,
            f_token_mint: *accounts.f_token_mint.key,
            supply_token_reserves_liquidity: *accounts
                .supply_token_reserves_liquidity
                .key,
            lending_supply_position_on_liquidity: *accounts
                .lending_supply_position_on_liquidity
                .key,
            rate_model: *accounts.rate_model.key,
            vault: *accounts.vault.key,
            claim_account: *accounts.claim_account.key,
            liquidity: *accounts.liquidity.key,
            liquidity_program: *accounts.liquidity_program.key,
            rewards_rate_model: *accounts.rewards_rate_model.key,
            token_program: *accounts.token_program.key,
            associated_token_program: *accounts.associated_token_program.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<RedeemWithMinAmountOutKeys>
for [AccountMeta; REDEEM_WITH_MIN_AMOUNT_OUT_IX_ACCOUNTS_LEN] {
    fn from(keys: RedeemWithMinAmountOutKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.signer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.owner_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.recipient_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.lending_admin,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.lending,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.f_token_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.supply_token_reserves_liquidity,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.lending_supply_position_on_liquidity,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.rate_model,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.claim_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.liquidity,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.liquidity_program,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.rewards_rate_model,
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
        ]
    }
}
impl From<[Pubkey; REDEEM_WITH_MIN_AMOUNT_OUT_IX_ACCOUNTS_LEN]>
for RedeemWithMinAmountOutKeys {
    fn from(pubkeys: [Pubkey; REDEEM_WITH_MIN_AMOUNT_OUT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            signer: pubkeys[0],
            owner_token_account: pubkeys[1],
            recipient_token_account: pubkeys[2],
            lending_admin: pubkeys[3],
            lending: pubkeys[4],
            mint: pubkeys[5],
            f_token_mint: pubkeys[6],
            supply_token_reserves_liquidity: pubkeys[7],
            lending_supply_position_on_liquidity: pubkeys[8],
            rate_model: pubkeys[9],
            vault: pubkeys[10],
            claim_account: pubkeys[11],
            liquidity: pubkeys[12],
            liquidity_program: pubkeys[13],
            rewards_rate_model: pubkeys[14],
            token_program: pubkeys[15],
            associated_token_program: pubkeys[16],
            system_program: pubkeys[17],
        }
    }
}
impl<'info> From<RedeemWithMinAmountOutAccounts<'_, 'info>>
for [AccountInfo<'info>; REDEEM_WITH_MIN_AMOUNT_OUT_IX_ACCOUNTS_LEN] {
    fn from(accounts: RedeemWithMinAmountOutAccounts<'_, 'info>) -> Self {
        [
            accounts.signer.clone(),
            accounts.owner_token_account.clone(),
            accounts.recipient_token_account.clone(),
            accounts.lending_admin.clone(),
            accounts.lending.clone(),
            accounts.mint.clone(),
            accounts.f_token_mint.clone(),
            accounts.supply_token_reserves_liquidity.clone(),
            accounts.lending_supply_position_on_liquidity.clone(),
            accounts.rate_model.clone(),
            accounts.vault.clone(),
            accounts.claim_account.clone(),
            accounts.liquidity.clone(),
            accounts.liquidity_program.clone(),
            accounts.rewards_rate_model.clone(),
            accounts.token_program.clone(),
            accounts.associated_token_program.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; REDEEM_WITH_MIN_AMOUNT_OUT_IX_ACCOUNTS_LEN]>
for RedeemWithMinAmountOutAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; REDEEM_WITH_MIN_AMOUNT_OUT_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            signer: &arr[0],
            owner_token_account: &arr[1],
            recipient_token_account: &arr[2],
            lending_admin: &arr[3],
            lending: &arr[4],
            mint: &arr[5],
            f_token_mint: &arr[6],
            supply_token_reserves_liquidity: &arr[7],
            lending_supply_position_on_liquidity: &arr[8],
            rate_model: &arr[9],
            vault: &arr[10],
            claim_account: &arr[11],
            liquidity: &arr[12],
            liquidity_program: &arr[13],
            rewards_rate_model: &arr[14],
            token_program: &arr[15],
            associated_token_program: &arr[16],
            system_program: &arr[17],
        }
    }
}
pub const REDEEM_WITH_MIN_AMOUNT_OUT_IX_DISCM: [u8; 8usize] = [
    235, 189, 237, 56, 166, 180, 184, 149,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct RedeemWithMinAmountOutIxArgs {
    pub shares: u64,
    pub min_amount_out: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct RedeemWithMinAmountOutIxData(pub RedeemWithMinAmountOutIxArgs);
impl From<RedeemWithMinAmountOutIxArgs> for RedeemWithMinAmountOutIxData {
    fn from(args: RedeemWithMinAmountOutIxArgs) -> Self {
        Self(args)
    }
}
impl RedeemWithMinAmountOutIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != REDEEM_WITH_MIN_AMOUNT_OUT_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let shares: u64 = crate::borsh_de_or_default(&mut reader)?;
        let min_amount_out: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(RedeemWithMinAmountOutIxArgs {
                shares,
                min_amount_out,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&REDEEM_WITH_MIN_AMOUNT_OUT_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.shares, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.min_amount_out, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn redeem_with_min_amount_out_ix_with_program_id(
    program_id: Pubkey,
    keys: RedeemWithMinAmountOutKeys,
    args: RedeemWithMinAmountOutIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; REDEEM_WITH_MIN_AMOUNT_OUT_IX_ACCOUNTS_LEN] = keys.into();
    let data: RedeemWithMinAmountOutIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn redeem_with_min_amount_out_ix(
    keys: RedeemWithMinAmountOutKeys,
    args: RedeemWithMinAmountOutIxArgs,
) -> std::io::Result<Instruction> {
    redeem_with_min_amount_out_ix_with_program_id(JUPITER_LEND_PROGRAM_ID, keys, args)
}
pub fn redeem_with_min_amount_out_invoke_with_program_id(
    program_id: Pubkey,
    accounts: RedeemWithMinAmountOutAccounts<'_, '_>,
    args: RedeemWithMinAmountOutIxArgs,
) -> ProgramResult {
    let keys: RedeemWithMinAmountOutKeys = accounts.into();
    let ix = redeem_with_min_amount_out_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn redeem_with_min_amount_out_invoke(
    accounts: RedeemWithMinAmountOutAccounts<'_, '_>,
    args: RedeemWithMinAmountOutIxArgs,
) -> ProgramResult {
    redeem_with_min_amount_out_invoke_with_program_id(
        JUPITER_LEND_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn redeem_with_min_amount_out_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: RedeemWithMinAmountOutAccounts<'_, '_>,
    args: RedeemWithMinAmountOutIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: RedeemWithMinAmountOutKeys = accounts.into();
    let ix = redeem_with_min_amount_out_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn redeem_with_min_amount_out_invoke_signed(
    accounts: RedeemWithMinAmountOutAccounts<'_, '_>,
    args: RedeemWithMinAmountOutIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    redeem_with_min_amount_out_invoke_signed_with_program_id(
        JUPITER_LEND_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn redeem_with_min_amount_out_verify_account_keys(
    accounts: RedeemWithMinAmountOutAccounts<'_, '_>,
    keys: RedeemWithMinAmountOutKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.signer.key, keys.signer),
        (*accounts.owner_token_account.key, keys.owner_token_account),
        (*accounts.recipient_token_account.key, keys.recipient_token_account),
        (*accounts.lending_admin.key, keys.lending_admin),
        (*accounts.lending.key, keys.lending),
        (*accounts.mint.key, keys.mint),
        (*accounts.f_token_mint.key, keys.f_token_mint),
        (
            *accounts.supply_token_reserves_liquidity.key,
            keys.supply_token_reserves_liquidity,
        ),
        (
            *accounts.lending_supply_position_on_liquidity.key,
            keys.lending_supply_position_on_liquidity,
        ),
        (*accounts.rate_model.key, keys.rate_model),
        (*accounts.vault.key, keys.vault),
        (*accounts.claim_account.key, keys.claim_account),
        (*accounts.liquidity.key, keys.liquidity),
        (*accounts.liquidity_program.key, keys.liquidity_program),
        (*accounts.rewards_rate_model.key, keys.rewards_rate_model),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.associated_token_program.key, keys.associated_token_program),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn redeem_with_min_amount_out_verify_writable_privileges<'me, 'info>(
    accounts: RedeemWithMinAmountOutAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.signer,
        accounts.owner_token_account,
        accounts.recipient_token_account,
        accounts.lending,
        accounts.f_token_mint,
        accounts.supply_token_reserves_liquidity,
        accounts.lending_supply_position_on_liquidity,
        accounts.vault,
        accounts.claim_account,
        accounts.liquidity,
        accounts.liquidity_program,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn redeem_with_min_amount_out_verify_signer_privileges<'me, 'info>(
    accounts: RedeemWithMinAmountOutAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.signer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn redeem_with_min_amount_out_verify_account_privileges<'me, 'info>(
    accounts: RedeemWithMinAmountOutAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    redeem_with_min_amount_out_verify_writable_privileges(accounts)?;
    redeem_with_min_amount_out_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SET_REWARDS_RATE_MODEL_IX_ACCOUNTS_LEN: usize = 6;
#[derive(Copy, Clone, Debug)]
pub struct SetRewardsRateModelAccounts<'me, 'info> {
    pub signer: &'me AccountInfo<'info>,
    pub lending_admin: &'me AccountInfo<'info>,
    pub lending: &'me AccountInfo<'info>,
    pub f_token_mint: &'me AccountInfo<'info>,
    pub new_rewards_rate_model: &'me AccountInfo<'info>,
    pub supply_token_reserves_liquidity: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SetRewardsRateModelKeys {
    pub signer: Pubkey,
    pub lending_admin: Pubkey,
    pub lending: Pubkey,
    pub f_token_mint: Pubkey,
    pub new_rewards_rate_model: Pubkey,
    pub supply_token_reserves_liquidity: Pubkey,
}
impl From<SetRewardsRateModelAccounts<'_, '_>> for SetRewardsRateModelKeys {
    fn from(accounts: SetRewardsRateModelAccounts) -> Self {
        Self {
            signer: *accounts.signer.key,
            lending_admin: *accounts.lending_admin.key,
            lending: *accounts.lending.key,
            f_token_mint: *accounts.f_token_mint.key,
            new_rewards_rate_model: *accounts.new_rewards_rate_model.key,
            supply_token_reserves_liquidity: *accounts
                .supply_token_reserves_liquidity
                .key,
        }
    }
}
impl From<SetRewardsRateModelKeys>
for [AccountMeta; SET_REWARDS_RATE_MODEL_IX_ACCOUNTS_LEN] {
    fn from(keys: SetRewardsRateModelKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.signer,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.lending_admin,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.lending,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.f_token_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.new_rewards_rate_model,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.supply_token_reserves_liquidity,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; SET_REWARDS_RATE_MODEL_IX_ACCOUNTS_LEN]> for SetRewardsRateModelKeys {
    fn from(pubkeys: [Pubkey; SET_REWARDS_RATE_MODEL_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            signer: pubkeys[0],
            lending_admin: pubkeys[1],
            lending: pubkeys[2],
            f_token_mint: pubkeys[3],
            new_rewards_rate_model: pubkeys[4],
            supply_token_reserves_liquidity: pubkeys[5],
        }
    }
}
impl<'info> From<SetRewardsRateModelAccounts<'_, 'info>>
for [AccountInfo<'info>; SET_REWARDS_RATE_MODEL_IX_ACCOUNTS_LEN] {
    fn from(accounts: SetRewardsRateModelAccounts<'_, 'info>) -> Self {
        [
            accounts.signer.clone(),
            accounts.lending_admin.clone(),
            accounts.lending.clone(),
            accounts.f_token_mint.clone(),
            accounts.new_rewards_rate_model.clone(),
            accounts.supply_token_reserves_liquidity.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SET_REWARDS_RATE_MODEL_IX_ACCOUNTS_LEN]>
for SetRewardsRateModelAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; SET_REWARDS_RATE_MODEL_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            signer: &arr[0],
            lending_admin: &arr[1],
            lending: &arr[2],
            f_token_mint: &arr[3],
            new_rewards_rate_model: &arr[4],
            supply_token_reserves_liquidity: &arr[5],
        }
    }
}
pub const SET_REWARDS_RATE_MODEL_IX_DISCM: [u8; 8usize] = [
    174, 231, 116, 203, 8, 58, 143, 203,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SetRewardsRateModelIxArgs {
    pub mint: Pubkey,
}
#[derive(Clone, Debug, PartialEq)]
pub struct SetRewardsRateModelIxData(pub SetRewardsRateModelIxArgs);
impl From<SetRewardsRateModelIxArgs> for SetRewardsRateModelIxData {
    fn from(args: SetRewardsRateModelIxArgs) -> Self {
        Self(args)
    }
}
impl SetRewardsRateModelIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SET_REWARDS_RATE_MODEL_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        Ok(Self(SetRewardsRateModelIxArgs { mint }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SET_REWARDS_RATE_MODEL_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.mint, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn set_rewards_rate_model_ix_with_program_id(
    program_id: Pubkey,
    keys: SetRewardsRateModelKeys,
    args: SetRewardsRateModelIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SET_REWARDS_RATE_MODEL_IX_ACCOUNTS_LEN] = keys.into();
    let data: SetRewardsRateModelIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn set_rewards_rate_model_ix(
    keys: SetRewardsRateModelKeys,
    args: SetRewardsRateModelIxArgs,
) -> std::io::Result<Instruction> {
    set_rewards_rate_model_ix_with_program_id(JUPITER_LEND_PROGRAM_ID, keys, args)
}
pub fn set_rewards_rate_model_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SetRewardsRateModelAccounts<'_, '_>,
    args: SetRewardsRateModelIxArgs,
) -> ProgramResult {
    let keys: SetRewardsRateModelKeys = accounts.into();
    let ix = set_rewards_rate_model_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn set_rewards_rate_model_invoke(
    accounts: SetRewardsRateModelAccounts<'_, '_>,
    args: SetRewardsRateModelIxArgs,
) -> ProgramResult {
    set_rewards_rate_model_invoke_with_program_id(
        JUPITER_LEND_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn set_rewards_rate_model_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SetRewardsRateModelAccounts<'_, '_>,
    args: SetRewardsRateModelIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SetRewardsRateModelKeys = accounts.into();
    let ix = set_rewards_rate_model_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn set_rewards_rate_model_invoke_signed(
    accounts: SetRewardsRateModelAccounts<'_, '_>,
    args: SetRewardsRateModelIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    set_rewards_rate_model_invoke_signed_with_program_id(
        JUPITER_LEND_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn set_rewards_rate_model_verify_account_keys(
    accounts: SetRewardsRateModelAccounts<'_, '_>,
    keys: SetRewardsRateModelKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.signer.key, keys.signer),
        (*accounts.lending_admin.key, keys.lending_admin),
        (*accounts.lending.key, keys.lending),
        (*accounts.f_token_mint.key, keys.f_token_mint),
        (*accounts.new_rewards_rate_model.key, keys.new_rewards_rate_model),
        (
            *accounts.supply_token_reserves_liquidity.key,
            keys.supply_token_reserves_liquidity,
        ),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn set_rewards_rate_model_verify_writable_privileges<'me, 'info>(
    accounts: SetRewardsRateModelAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.lending] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn set_rewards_rate_model_verify_signer_privileges<'me, 'info>(
    accounts: SetRewardsRateModelAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.signer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn set_rewards_rate_model_verify_account_privileges<'me, 'info>(
    accounts: SetRewardsRateModelAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    set_rewards_rate_model_verify_writable_privileges(accounts)?;
    set_rewards_rate_model_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const UPDATE_AUTHORITY_IX_ACCOUNTS_LEN: usize = 2;
#[derive(Copy, Clone, Debug)]
pub struct UpdateAuthorityAccounts<'me, 'info> {
    pub signer: &'me AccountInfo<'info>,
    pub lending_admin: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UpdateAuthorityKeys {
    pub signer: Pubkey,
    pub lending_admin: Pubkey,
}
impl From<UpdateAuthorityAccounts<'_, '_>> for UpdateAuthorityKeys {
    fn from(accounts: UpdateAuthorityAccounts) -> Self {
        Self {
            signer: *accounts.signer.key,
            lending_admin: *accounts.lending_admin.key,
        }
    }
}
impl From<UpdateAuthorityKeys> for [AccountMeta; UPDATE_AUTHORITY_IX_ACCOUNTS_LEN] {
    fn from(keys: UpdateAuthorityKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.signer,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.lending_admin,
                is_signer: false,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; UPDATE_AUTHORITY_IX_ACCOUNTS_LEN]> for UpdateAuthorityKeys {
    fn from(pubkeys: [Pubkey; UPDATE_AUTHORITY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            signer: pubkeys[0],
            lending_admin: pubkeys[1],
        }
    }
}
impl<'info> From<UpdateAuthorityAccounts<'_, 'info>>
for [AccountInfo<'info>; UPDATE_AUTHORITY_IX_ACCOUNTS_LEN] {
    fn from(accounts: UpdateAuthorityAccounts<'_, 'info>) -> Self {
        [accounts.signer.clone(), accounts.lending_admin.clone()]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; UPDATE_AUTHORITY_IX_ACCOUNTS_LEN]>
for UpdateAuthorityAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; UPDATE_AUTHORITY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            signer: &arr[0],
            lending_admin: &arr[1],
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
    update_authority_ix_with_program_id(JUPITER_LEND_PROGRAM_ID, keys, args)
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
    update_authority_invoke_with_program_id(JUPITER_LEND_PROGRAM_ID, accounts, args)
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
        JUPITER_LEND_PROGRAM_ID,
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
        (*accounts.signer.key, keys.signer),
        (*accounts.lending_admin.key, keys.lending_admin),
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
    for should_be_writable in [accounts.lending_admin] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn update_authority_verify_signer_privileges<'me, 'info>(
    accounts: UpdateAuthorityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.signer] {
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
    pub signer: &'me AccountInfo<'info>,
    pub lending_admin: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UpdateAuthsKeys {
    pub signer: Pubkey,
    pub lending_admin: Pubkey,
}
impl From<UpdateAuthsAccounts<'_, '_>> for UpdateAuthsKeys {
    fn from(accounts: UpdateAuthsAccounts) -> Self {
        Self {
            signer: *accounts.signer.key,
            lending_admin: *accounts.lending_admin.key,
        }
    }
}
impl From<UpdateAuthsKeys> for [AccountMeta; UPDATE_AUTHS_IX_ACCOUNTS_LEN] {
    fn from(keys: UpdateAuthsKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.signer,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.lending_admin,
                is_signer: false,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; UPDATE_AUTHS_IX_ACCOUNTS_LEN]> for UpdateAuthsKeys {
    fn from(pubkeys: [Pubkey; UPDATE_AUTHS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            signer: pubkeys[0],
            lending_admin: pubkeys[1],
        }
    }
}
impl<'info> From<UpdateAuthsAccounts<'_, 'info>>
for [AccountInfo<'info>; UPDATE_AUTHS_IX_ACCOUNTS_LEN] {
    fn from(accounts: UpdateAuthsAccounts<'_, 'info>) -> Self {
        [accounts.signer.clone(), accounts.lending_admin.clone()]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; UPDATE_AUTHS_IX_ACCOUNTS_LEN]>
for UpdateAuthsAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; UPDATE_AUTHS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            signer: &arr[0],
            lending_admin: &arr[1],
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
    update_auths_ix_with_program_id(JUPITER_LEND_PROGRAM_ID, keys, args)
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
    update_auths_invoke_with_program_id(JUPITER_LEND_PROGRAM_ID, accounts, args)
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
        JUPITER_LEND_PROGRAM_ID,
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
        (*accounts.signer.key, keys.signer),
        (*accounts.lending_admin.key, keys.lending_admin),
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
    for should_be_writable in [accounts.lending_admin] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn update_auths_verify_signer_privileges<'me, 'info>(
    accounts: UpdateAuthsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.signer] {
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
pub const UPDATE_RATE_IX_ACCOUNTS_LEN: usize = 5;
#[derive(Copy, Clone, Debug)]
pub struct UpdateRateAccounts<'me, 'info> {
    pub lending: &'me AccountInfo<'info>,
    pub mint: &'me AccountInfo<'info>,
    pub f_token_mint: &'me AccountInfo<'info>,
    pub supply_token_reserves_liquidity: &'me AccountInfo<'info>,
    pub rewards_rate_model: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UpdateRateKeys {
    pub lending: Pubkey,
    pub mint: Pubkey,
    pub f_token_mint: Pubkey,
    pub supply_token_reserves_liquidity: Pubkey,
    pub rewards_rate_model: Pubkey,
}
impl From<UpdateRateAccounts<'_, '_>> for UpdateRateKeys {
    fn from(accounts: UpdateRateAccounts) -> Self {
        Self {
            lending: *accounts.lending.key,
            mint: *accounts.mint.key,
            f_token_mint: *accounts.f_token_mint.key,
            supply_token_reserves_liquidity: *accounts
                .supply_token_reserves_liquidity
                .key,
            rewards_rate_model: *accounts.rewards_rate_model.key,
        }
    }
}
impl From<UpdateRateKeys> for [AccountMeta; UPDATE_RATE_IX_ACCOUNTS_LEN] {
    fn from(keys: UpdateRateKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.lending,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.f_token_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.supply_token_reserves_liquidity,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.rewards_rate_model,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; UPDATE_RATE_IX_ACCOUNTS_LEN]> for UpdateRateKeys {
    fn from(pubkeys: [Pubkey; UPDATE_RATE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            lending: pubkeys[0],
            mint: pubkeys[1],
            f_token_mint: pubkeys[2],
            supply_token_reserves_liquidity: pubkeys[3],
            rewards_rate_model: pubkeys[4],
        }
    }
}
impl<'info> From<UpdateRateAccounts<'_, 'info>>
for [AccountInfo<'info>; UPDATE_RATE_IX_ACCOUNTS_LEN] {
    fn from(accounts: UpdateRateAccounts<'_, 'info>) -> Self {
        [
            accounts.lending.clone(),
            accounts.mint.clone(),
            accounts.f_token_mint.clone(),
            accounts.supply_token_reserves_liquidity.clone(),
            accounts.rewards_rate_model.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; UPDATE_RATE_IX_ACCOUNTS_LEN]>
for UpdateRateAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; UPDATE_RATE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            lending: &arr[0],
            mint: &arr[1],
            f_token_mint: &arr[2],
            supply_token_reserves_liquidity: &arr[3],
            rewards_rate_model: &arr[4],
        }
    }
}
pub const UPDATE_RATE_IX_DISCM: [u8; 8usize] = [24, 225, 53, 189, 72, 212, 225, 178];
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateRateIxData;
impl UpdateRateIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_RATE_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_RATE_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn update_rate_ix_with_program_id(
    program_id: Pubkey,
    keys: UpdateRateKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; UPDATE_RATE_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: UpdateRateIxData.try_to_vec()?,
    })
}
pub fn update_rate_ix(keys: UpdateRateKeys) -> std::io::Result<Instruction> {
    update_rate_ix_with_program_id(JUPITER_LEND_PROGRAM_ID, keys)
}
pub fn update_rate_invoke_with_program_id(
    program_id: Pubkey,
    accounts: UpdateRateAccounts<'_, '_>,
) -> ProgramResult {
    let keys: UpdateRateKeys = accounts.into();
    let ix = update_rate_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn update_rate_invoke(accounts: UpdateRateAccounts<'_, '_>) -> ProgramResult {
    update_rate_invoke_with_program_id(JUPITER_LEND_PROGRAM_ID, accounts)
}
pub fn update_rate_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: UpdateRateAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: UpdateRateKeys = accounts.into();
    let ix = update_rate_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn update_rate_invoke_signed(
    accounts: UpdateRateAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    update_rate_invoke_signed_with_program_id(JUPITER_LEND_PROGRAM_ID, accounts, seeds)
}
pub fn update_rate_verify_account_keys(
    accounts: UpdateRateAccounts<'_, '_>,
    keys: UpdateRateKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.lending.key, keys.lending),
        (*accounts.mint.key, keys.mint),
        (*accounts.f_token_mint.key, keys.f_token_mint),
        (
            *accounts.supply_token_reserves_liquidity.key,
            keys.supply_token_reserves_liquidity,
        ),
        (*accounts.rewards_rate_model.key, keys.rewards_rate_model),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn update_rate_verify_writable_privileges<'me, 'info>(
    accounts: UpdateRateAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.lending] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn update_rate_verify_account_privileges<'me, 'info>(
    accounts: UpdateRateAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    update_rate_verify_writable_privileges(accounts)?;
    Ok(())
}
pub const UPDATE_REBALANCER_IX_ACCOUNTS_LEN: usize = 2;
#[derive(Copy, Clone, Debug)]
pub struct UpdateRebalancerAccounts<'me, 'info> {
    pub signer: &'me AccountInfo<'info>,
    pub lending_admin: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UpdateRebalancerKeys {
    pub signer: Pubkey,
    pub lending_admin: Pubkey,
}
impl From<UpdateRebalancerAccounts<'_, '_>> for UpdateRebalancerKeys {
    fn from(accounts: UpdateRebalancerAccounts) -> Self {
        Self {
            signer: *accounts.signer.key,
            lending_admin: *accounts.lending_admin.key,
        }
    }
}
impl From<UpdateRebalancerKeys> for [AccountMeta; UPDATE_REBALANCER_IX_ACCOUNTS_LEN] {
    fn from(keys: UpdateRebalancerKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.signer,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.lending_admin,
                is_signer: false,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; UPDATE_REBALANCER_IX_ACCOUNTS_LEN]> for UpdateRebalancerKeys {
    fn from(pubkeys: [Pubkey; UPDATE_REBALANCER_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            signer: pubkeys[0],
            lending_admin: pubkeys[1],
        }
    }
}
impl<'info> From<UpdateRebalancerAccounts<'_, 'info>>
for [AccountInfo<'info>; UPDATE_REBALANCER_IX_ACCOUNTS_LEN] {
    fn from(accounts: UpdateRebalancerAccounts<'_, 'info>) -> Self {
        [accounts.signer.clone(), accounts.lending_admin.clone()]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; UPDATE_REBALANCER_IX_ACCOUNTS_LEN]>
for UpdateRebalancerAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; UPDATE_REBALANCER_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            signer: &arr[0],
            lending_admin: &arr[1],
        }
    }
}
pub const UPDATE_REBALANCER_IX_DISCM: [u8; 8usize] = [
    206, 187, 54, 228, 145, 8, 203, 111,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UpdateRebalancerIxArgs {
    pub new_rebalancer: Pubkey,
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateRebalancerIxData(pub UpdateRebalancerIxArgs);
impl From<UpdateRebalancerIxArgs> for UpdateRebalancerIxData {
    fn from(args: UpdateRebalancerIxArgs) -> Self {
        Self(args)
    }
}
impl UpdateRebalancerIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_REBALANCER_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let new_rebalancer: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(UpdateRebalancerIxArgs {
                new_rebalancer,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_REBALANCER_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.new_rebalancer, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn update_rebalancer_ix_with_program_id(
    program_id: Pubkey,
    keys: UpdateRebalancerKeys,
    args: UpdateRebalancerIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; UPDATE_REBALANCER_IX_ACCOUNTS_LEN] = keys.into();
    let data: UpdateRebalancerIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_rebalancer_ix(
    keys: UpdateRebalancerKeys,
    args: UpdateRebalancerIxArgs,
) -> std::io::Result<Instruction> {
    update_rebalancer_ix_with_program_id(JUPITER_LEND_PROGRAM_ID, keys, args)
}
pub fn update_rebalancer_invoke_with_program_id(
    program_id: Pubkey,
    accounts: UpdateRebalancerAccounts<'_, '_>,
    args: UpdateRebalancerIxArgs,
) -> ProgramResult {
    let keys: UpdateRebalancerKeys = accounts.into();
    let ix = update_rebalancer_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn update_rebalancer_invoke(
    accounts: UpdateRebalancerAccounts<'_, '_>,
    args: UpdateRebalancerIxArgs,
) -> ProgramResult {
    update_rebalancer_invoke_with_program_id(JUPITER_LEND_PROGRAM_ID, accounts, args)
}
pub fn update_rebalancer_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: UpdateRebalancerAccounts<'_, '_>,
    args: UpdateRebalancerIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: UpdateRebalancerKeys = accounts.into();
    let ix = update_rebalancer_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn update_rebalancer_invoke_signed(
    accounts: UpdateRebalancerAccounts<'_, '_>,
    args: UpdateRebalancerIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    update_rebalancer_invoke_signed_with_program_id(
        JUPITER_LEND_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn update_rebalancer_verify_account_keys(
    accounts: UpdateRebalancerAccounts<'_, '_>,
    keys: UpdateRebalancerKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.signer.key, keys.signer),
        (*accounts.lending_admin.key, keys.lending_admin),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn update_rebalancer_verify_writable_privileges<'me, 'info>(
    accounts: UpdateRebalancerAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.lending_admin] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn update_rebalancer_verify_signer_privileges<'me, 'info>(
    accounts: UpdateRebalancerAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.signer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn update_rebalancer_verify_account_privileges<'me, 'info>(
    accounts: UpdateRebalancerAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    update_rebalancer_verify_writable_privileges(accounts)?;
    update_rebalancer_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const WITHDRAW_IX_ACCOUNTS_LEN: usize = 18;
#[derive(Copy, Clone, Debug)]
pub struct WithdrawAccounts<'me, 'info> {
    pub signer: &'me AccountInfo<'info>,
    pub owner_token_account: &'me AccountInfo<'info>,
    pub recipient_token_account: &'me AccountInfo<'info>,
    pub lending_admin: &'me AccountInfo<'info>,
    pub lending: &'me AccountInfo<'info>,
    pub mint: &'me AccountInfo<'info>,
    pub f_token_mint: &'me AccountInfo<'info>,
    pub supply_token_reserves_liquidity: &'me AccountInfo<'info>,
    pub lending_supply_position_on_liquidity: &'me AccountInfo<'info>,
    pub rate_model: &'me AccountInfo<'info>,
    pub vault: &'me AccountInfo<'info>,
    pub claim_account: &'me AccountInfo<'info>,
    pub liquidity: &'me AccountInfo<'info>,
    pub liquidity_program: &'me AccountInfo<'info>,
    pub rewards_rate_model: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct WithdrawKeys {
    pub signer: Pubkey,
    pub owner_token_account: Pubkey,
    pub recipient_token_account: Pubkey,
    pub lending_admin: Pubkey,
    pub lending: Pubkey,
    pub mint: Pubkey,
    pub f_token_mint: Pubkey,
    pub supply_token_reserves_liquidity: Pubkey,
    pub lending_supply_position_on_liquidity: Pubkey,
    pub rate_model: Pubkey,
    pub vault: Pubkey,
    pub claim_account: Pubkey,
    pub liquidity: Pubkey,
    pub liquidity_program: Pubkey,
    pub rewards_rate_model: Pubkey,
    pub token_program: Pubkey,
    pub associated_token_program: Pubkey,
    pub system_program: Pubkey,
}
impl From<WithdrawAccounts<'_, '_>> for WithdrawKeys {
    fn from(accounts: WithdrawAccounts) -> Self {
        Self {
            signer: *accounts.signer.key,
            owner_token_account: *accounts.owner_token_account.key,
            recipient_token_account: *accounts.recipient_token_account.key,
            lending_admin: *accounts.lending_admin.key,
            lending: *accounts.lending.key,
            mint: *accounts.mint.key,
            f_token_mint: *accounts.f_token_mint.key,
            supply_token_reserves_liquidity: *accounts
                .supply_token_reserves_liquidity
                .key,
            lending_supply_position_on_liquidity: *accounts
                .lending_supply_position_on_liquidity
                .key,
            rate_model: *accounts.rate_model.key,
            vault: *accounts.vault.key,
            claim_account: *accounts.claim_account.key,
            liquidity: *accounts.liquidity.key,
            liquidity_program: *accounts.liquidity_program.key,
            rewards_rate_model: *accounts.rewards_rate_model.key,
            token_program: *accounts.token_program.key,
            associated_token_program: *accounts.associated_token_program.key,
            system_program: *accounts.system_program.key,
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
                pubkey: keys.owner_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.recipient_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.lending_admin,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.lending,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.f_token_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.supply_token_reserves_liquidity,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.lending_supply_position_on_liquidity,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.rate_model,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.claim_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.liquidity,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.liquidity_program,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.rewards_rate_model,
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
        ]
    }
}
impl From<[Pubkey; WITHDRAW_IX_ACCOUNTS_LEN]> for WithdrawKeys {
    fn from(pubkeys: [Pubkey; WITHDRAW_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            signer: pubkeys[0],
            owner_token_account: pubkeys[1],
            recipient_token_account: pubkeys[2],
            lending_admin: pubkeys[3],
            lending: pubkeys[4],
            mint: pubkeys[5],
            f_token_mint: pubkeys[6],
            supply_token_reserves_liquidity: pubkeys[7],
            lending_supply_position_on_liquidity: pubkeys[8],
            rate_model: pubkeys[9],
            vault: pubkeys[10],
            claim_account: pubkeys[11],
            liquidity: pubkeys[12],
            liquidity_program: pubkeys[13],
            rewards_rate_model: pubkeys[14],
            token_program: pubkeys[15],
            associated_token_program: pubkeys[16],
            system_program: pubkeys[17],
        }
    }
}
impl<'info> From<WithdrawAccounts<'_, 'info>>
for [AccountInfo<'info>; WITHDRAW_IX_ACCOUNTS_LEN] {
    fn from(accounts: WithdrawAccounts<'_, 'info>) -> Self {
        [
            accounts.signer.clone(),
            accounts.owner_token_account.clone(),
            accounts.recipient_token_account.clone(),
            accounts.lending_admin.clone(),
            accounts.lending.clone(),
            accounts.mint.clone(),
            accounts.f_token_mint.clone(),
            accounts.supply_token_reserves_liquidity.clone(),
            accounts.lending_supply_position_on_liquidity.clone(),
            accounts.rate_model.clone(),
            accounts.vault.clone(),
            accounts.claim_account.clone(),
            accounts.liquidity.clone(),
            accounts.liquidity_program.clone(),
            accounts.rewards_rate_model.clone(),
            accounts.token_program.clone(),
            accounts.associated_token_program.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; WITHDRAW_IX_ACCOUNTS_LEN]>
for WithdrawAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; WITHDRAW_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            signer: &arr[0],
            owner_token_account: &arr[1],
            recipient_token_account: &arr[2],
            lending_admin: &arr[3],
            lending: &arr[4],
            mint: &arr[5],
            f_token_mint: &arr[6],
            supply_token_reserves_liquidity: &arr[7],
            lending_supply_position_on_liquidity: &arr[8],
            rate_model: &arr[9],
            vault: &arr[10],
            claim_account: &arr[11],
            liquidity: &arr[12],
            liquidity_program: &arr[13],
            rewards_rate_model: &arr[14],
            token_program: &arr[15],
            associated_token_program: &arr[16],
            system_program: &arr[17],
        }
    }
}
pub const WITHDRAW_IX_DISCM: [u8; 8usize] = [183, 18, 70, 156, 148, 109, 161, 34];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WithdrawIxArgs {
    pub amount: u64,
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
        let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
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
    withdraw_ix_with_program_id(JUPITER_LEND_PROGRAM_ID, keys, args)
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
    withdraw_invoke_with_program_id(JUPITER_LEND_PROGRAM_ID, accounts, args)
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
        JUPITER_LEND_PROGRAM_ID,
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
        (*accounts.owner_token_account.key, keys.owner_token_account),
        (*accounts.recipient_token_account.key, keys.recipient_token_account),
        (*accounts.lending_admin.key, keys.lending_admin),
        (*accounts.lending.key, keys.lending),
        (*accounts.mint.key, keys.mint),
        (*accounts.f_token_mint.key, keys.f_token_mint),
        (
            *accounts.supply_token_reserves_liquidity.key,
            keys.supply_token_reserves_liquidity,
        ),
        (
            *accounts.lending_supply_position_on_liquidity.key,
            keys.lending_supply_position_on_liquidity,
        ),
        (*accounts.rate_model.key, keys.rate_model),
        (*accounts.vault.key, keys.vault),
        (*accounts.claim_account.key, keys.claim_account),
        (*accounts.liquidity.key, keys.liquidity),
        (*accounts.liquidity_program.key, keys.liquidity_program),
        (*accounts.rewards_rate_model.key, keys.rewards_rate_model),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.associated_token_program.key, keys.associated_token_program),
        (*accounts.system_program.key, keys.system_program),
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
        accounts.owner_token_account,
        accounts.recipient_token_account,
        accounts.lending,
        accounts.f_token_mint,
        accounts.supply_token_reserves_liquidity,
        accounts.lending_supply_position_on_liquidity,
        accounts.vault,
        accounts.claim_account,
        accounts.liquidity,
        accounts.liquidity_program,
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
    for should_be_signer in [accounts.signer] {
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
pub const WITHDRAW_WITH_MAX_SHARES_BURN_IX_ACCOUNTS_LEN: usize = 18;
#[derive(Copy, Clone, Debug)]
pub struct WithdrawWithMaxSharesBurnAccounts<'me, 'info> {
    pub signer: &'me AccountInfo<'info>,
    pub owner_token_account: &'me AccountInfo<'info>,
    pub recipient_token_account: &'me AccountInfo<'info>,
    pub lending_admin: &'me AccountInfo<'info>,
    pub lending: &'me AccountInfo<'info>,
    pub mint: &'me AccountInfo<'info>,
    pub f_token_mint: &'me AccountInfo<'info>,
    pub supply_token_reserves_liquidity: &'me AccountInfo<'info>,
    pub lending_supply_position_on_liquidity: &'me AccountInfo<'info>,
    pub rate_model: &'me AccountInfo<'info>,
    pub vault: &'me AccountInfo<'info>,
    pub claim_account: &'me AccountInfo<'info>,
    pub liquidity: &'me AccountInfo<'info>,
    pub liquidity_program: &'me AccountInfo<'info>,
    pub rewards_rate_model: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct WithdrawWithMaxSharesBurnKeys {
    pub signer: Pubkey,
    pub owner_token_account: Pubkey,
    pub recipient_token_account: Pubkey,
    pub lending_admin: Pubkey,
    pub lending: Pubkey,
    pub mint: Pubkey,
    pub f_token_mint: Pubkey,
    pub supply_token_reserves_liquidity: Pubkey,
    pub lending_supply_position_on_liquidity: Pubkey,
    pub rate_model: Pubkey,
    pub vault: Pubkey,
    pub claim_account: Pubkey,
    pub liquidity: Pubkey,
    pub liquidity_program: Pubkey,
    pub rewards_rate_model: Pubkey,
    pub token_program: Pubkey,
    pub associated_token_program: Pubkey,
    pub system_program: Pubkey,
}
impl From<WithdrawWithMaxSharesBurnAccounts<'_, '_>> for WithdrawWithMaxSharesBurnKeys {
    fn from(accounts: WithdrawWithMaxSharesBurnAccounts) -> Self {
        Self {
            signer: *accounts.signer.key,
            owner_token_account: *accounts.owner_token_account.key,
            recipient_token_account: *accounts.recipient_token_account.key,
            lending_admin: *accounts.lending_admin.key,
            lending: *accounts.lending.key,
            mint: *accounts.mint.key,
            f_token_mint: *accounts.f_token_mint.key,
            supply_token_reserves_liquidity: *accounts
                .supply_token_reserves_liquidity
                .key,
            lending_supply_position_on_liquidity: *accounts
                .lending_supply_position_on_liquidity
                .key,
            rate_model: *accounts.rate_model.key,
            vault: *accounts.vault.key,
            claim_account: *accounts.claim_account.key,
            liquidity: *accounts.liquidity.key,
            liquidity_program: *accounts.liquidity_program.key,
            rewards_rate_model: *accounts.rewards_rate_model.key,
            token_program: *accounts.token_program.key,
            associated_token_program: *accounts.associated_token_program.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<WithdrawWithMaxSharesBurnKeys>
for [AccountMeta; WITHDRAW_WITH_MAX_SHARES_BURN_IX_ACCOUNTS_LEN] {
    fn from(keys: WithdrawWithMaxSharesBurnKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.signer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.owner_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.recipient_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.lending_admin,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.lending,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.f_token_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.supply_token_reserves_liquidity,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.lending_supply_position_on_liquidity,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.rate_model,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.claim_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.liquidity,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.liquidity_program,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.rewards_rate_model,
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
        ]
    }
}
impl From<[Pubkey; WITHDRAW_WITH_MAX_SHARES_BURN_IX_ACCOUNTS_LEN]>
for WithdrawWithMaxSharesBurnKeys {
    fn from(pubkeys: [Pubkey; WITHDRAW_WITH_MAX_SHARES_BURN_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            signer: pubkeys[0],
            owner_token_account: pubkeys[1],
            recipient_token_account: pubkeys[2],
            lending_admin: pubkeys[3],
            lending: pubkeys[4],
            mint: pubkeys[5],
            f_token_mint: pubkeys[6],
            supply_token_reserves_liquidity: pubkeys[7],
            lending_supply_position_on_liquidity: pubkeys[8],
            rate_model: pubkeys[9],
            vault: pubkeys[10],
            claim_account: pubkeys[11],
            liquidity: pubkeys[12],
            liquidity_program: pubkeys[13],
            rewards_rate_model: pubkeys[14],
            token_program: pubkeys[15],
            associated_token_program: pubkeys[16],
            system_program: pubkeys[17],
        }
    }
}
impl<'info> From<WithdrawWithMaxSharesBurnAccounts<'_, 'info>>
for [AccountInfo<'info>; WITHDRAW_WITH_MAX_SHARES_BURN_IX_ACCOUNTS_LEN] {
    fn from(accounts: WithdrawWithMaxSharesBurnAccounts<'_, 'info>) -> Self {
        [
            accounts.signer.clone(),
            accounts.owner_token_account.clone(),
            accounts.recipient_token_account.clone(),
            accounts.lending_admin.clone(),
            accounts.lending.clone(),
            accounts.mint.clone(),
            accounts.f_token_mint.clone(),
            accounts.supply_token_reserves_liquidity.clone(),
            accounts.lending_supply_position_on_liquidity.clone(),
            accounts.rate_model.clone(),
            accounts.vault.clone(),
            accounts.claim_account.clone(),
            accounts.liquidity.clone(),
            accounts.liquidity_program.clone(),
            accounts.rewards_rate_model.clone(),
            accounts.token_program.clone(),
            accounts.associated_token_program.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; WITHDRAW_WITH_MAX_SHARES_BURN_IX_ACCOUNTS_LEN]>
for WithdrawWithMaxSharesBurnAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; WITHDRAW_WITH_MAX_SHARES_BURN_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            signer: &arr[0],
            owner_token_account: &arr[1],
            recipient_token_account: &arr[2],
            lending_admin: &arr[3],
            lending: &arr[4],
            mint: &arr[5],
            f_token_mint: &arr[6],
            supply_token_reserves_liquidity: &arr[7],
            lending_supply_position_on_liquidity: &arr[8],
            rate_model: &arr[9],
            vault: &arr[10],
            claim_account: &arr[11],
            liquidity: &arr[12],
            liquidity_program: &arr[13],
            rewards_rate_model: &arr[14],
            token_program: &arr[15],
            associated_token_program: &arr[16],
            system_program: &arr[17],
        }
    }
}
pub const WITHDRAW_WITH_MAX_SHARES_BURN_IX_DISCM: [u8; 8usize] = [
    47, 197, 183, 171, 239, 18, 245, 171,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WithdrawWithMaxSharesBurnIxArgs {
    pub amount: u64,
    pub max_shares_burn: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct WithdrawWithMaxSharesBurnIxData(pub WithdrawWithMaxSharesBurnIxArgs);
impl From<WithdrawWithMaxSharesBurnIxArgs> for WithdrawWithMaxSharesBurnIxData {
    fn from(args: WithdrawWithMaxSharesBurnIxArgs) -> Self {
        Self(args)
    }
}
impl WithdrawWithMaxSharesBurnIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != WITHDRAW_WITH_MAX_SHARES_BURN_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let max_shares_burn: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(WithdrawWithMaxSharesBurnIxArgs {
                amount,
                max_shares_burn,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&WITHDRAW_WITH_MAX_SHARES_BURN_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.max_shares_burn, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn withdraw_with_max_shares_burn_ix_with_program_id(
    program_id: Pubkey,
    keys: WithdrawWithMaxSharesBurnKeys,
    args: WithdrawWithMaxSharesBurnIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; WITHDRAW_WITH_MAX_SHARES_BURN_IX_ACCOUNTS_LEN] = keys
        .into();
    let data: WithdrawWithMaxSharesBurnIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn withdraw_with_max_shares_burn_ix(
    keys: WithdrawWithMaxSharesBurnKeys,
    args: WithdrawWithMaxSharesBurnIxArgs,
) -> std::io::Result<Instruction> {
    withdraw_with_max_shares_burn_ix_with_program_id(JUPITER_LEND_PROGRAM_ID, keys, args)
}
pub fn withdraw_with_max_shares_burn_invoke_with_program_id(
    program_id: Pubkey,
    accounts: WithdrawWithMaxSharesBurnAccounts<'_, '_>,
    args: WithdrawWithMaxSharesBurnIxArgs,
) -> ProgramResult {
    let keys: WithdrawWithMaxSharesBurnKeys = accounts.into();
    let ix = withdraw_with_max_shares_burn_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn withdraw_with_max_shares_burn_invoke(
    accounts: WithdrawWithMaxSharesBurnAccounts<'_, '_>,
    args: WithdrawWithMaxSharesBurnIxArgs,
) -> ProgramResult {
    withdraw_with_max_shares_burn_invoke_with_program_id(
        JUPITER_LEND_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn withdraw_with_max_shares_burn_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: WithdrawWithMaxSharesBurnAccounts<'_, '_>,
    args: WithdrawWithMaxSharesBurnIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: WithdrawWithMaxSharesBurnKeys = accounts.into();
    let ix = withdraw_with_max_shares_burn_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn withdraw_with_max_shares_burn_invoke_signed(
    accounts: WithdrawWithMaxSharesBurnAccounts<'_, '_>,
    args: WithdrawWithMaxSharesBurnIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    withdraw_with_max_shares_burn_invoke_signed_with_program_id(
        JUPITER_LEND_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn withdraw_with_max_shares_burn_verify_account_keys(
    accounts: WithdrawWithMaxSharesBurnAccounts<'_, '_>,
    keys: WithdrawWithMaxSharesBurnKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.signer.key, keys.signer),
        (*accounts.owner_token_account.key, keys.owner_token_account),
        (*accounts.recipient_token_account.key, keys.recipient_token_account),
        (*accounts.lending_admin.key, keys.lending_admin),
        (*accounts.lending.key, keys.lending),
        (*accounts.mint.key, keys.mint),
        (*accounts.f_token_mint.key, keys.f_token_mint),
        (
            *accounts.supply_token_reserves_liquidity.key,
            keys.supply_token_reserves_liquidity,
        ),
        (
            *accounts.lending_supply_position_on_liquidity.key,
            keys.lending_supply_position_on_liquidity,
        ),
        (*accounts.rate_model.key, keys.rate_model),
        (*accounts.vault.key, keys.vault),
        (*accounts.claim_account.key, keys.claim_account),
        (*accounts.liquidity.key, keys.liquidity),
        (*accounts.liquidity_program.key, keys.liquidity_program),
        (*accounts.rewards_rate_model.key, keys.rewards_rate_model),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.associated_token_program.key, keys.associated_token_program),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn withdraw_with_max_shares_burn_verify_writable_privileges<'me, 'info>(
    accounts: WithdrawWithMaxSharesBurnAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.signer,
        accounts.owner_token_account,
        accounts.recipient_token_account,
        accounts.lending,
        accounts.f_token_mint,
        accounts.supply_token_reserves_liquidity,
        accounts.lending_supply_position_on_liquidity,
        accounts.vault,
        accounts.claim_account,
        accounts.liquidity,
        accounts.liquidity_program,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn withdraw_with_max_shares_burn_verify_signer_privileges<'me, 'info>(
    accounts: WithdrawWithMaxSharesBurnAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.signer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn withdraw_with_max_shares_burn_verify_account_privileges<'me, 'info>(
    accounts: WithdrawWithMaxSharesBurnAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    withdraw_with_max_shares_burn_verify_writable_privileges(accounts)?;
    withdraw_with_max_shares_burn_verify_signer_privileges(accounts)?;
    Ok(())
}
