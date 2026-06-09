use solana_pubkey::Pubkey;
use solana_cpi::{invoke, invoke_signed};
use solana_instruction::{AccountMeta, Instruction};
use solana_account_info::AccountInfo;
use solana_program_error::ProgramError;
use std::io::Read;
#[allow(unused_imports)]
use crate::*;
#[derive(Clone, Debug, PartialEq)]
pub enum VaultProgramIx {
    Initialize,
    EnableVault(EnableVaultIxArgs),
    SetOperator,
    InitializeStrategy(InitializeStrategyIxArgs),
    RemoveStrategy,
    RemoveStrategy2(RemoveStrategy2IxArgs),
    CollectDust,
    AddStrategy,
    DepositStrategy(DepositStrategyIxArgs),
    WithdrawStrategy(WithdrawStrategyIxArgs),
    Withdraw2(Withdraw2IxArgs),
    Deposit(DepositIxArgs),
    Withdraw(WithdrawIxArgs),
    WithdrawDirectlyFromStrategy(WithdrawDirectlyFromStrategyIxArgs),
}
impl VaultProgramIx {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        if buf.starts_with(&INITIALIZE_IX_DISCM) {
            return Ok(Self::Initialize);
        }
        if buf.starts_with(&ENABLE_VAULT_IX_DISCM) {
            let mut reader = &buf[ENABLE_VAULT_IX_DISCM.len()..];
            let enabled: u8 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(Self::EnableVault(EnableVaultIxArgs { enabled }));
        }
        if buf.starts_with(&SET_OPERATOR_IX_DISCM) {
            return Ok(Self::SetOperator);
        }
        if buf.starts_with(&INITIALIZE_STRATEGY_IX_DISCM) {
            let mut reader = &buf[INITIALIZE_STRATEGY_IX_DISCM.len()..];
            let bumps = if reader.is_empty() {
                Default::default()
            } else {
                <StrategyBumps>::deserialize(&mut reader)?
            };
            let strategy_type: StrategyType = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::InitializeStrategy(InitializeStrategyIxArgs {
                    bumps,
                    strategy_type,
                }),
            );
        }
        if buf.starts_with(&REMOVE_STRATEGY_IX_DISCM) {
            return Ok(Self::RemoveStrategy);
        }
        if buf.starts_with(&REMOVE_STRATEGY2_IX_DISCM) {
            let mut reader = &buf[REMOVE_STRATEGY2_IX_DISCM.len()..];
            let max_admin_pay_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::RemoveStrategy2(RemoveStrategy2IxArgs {
                    max_admin_pay_amount,
                }),
            );
        }
        if buf.starts_with(&COLLECT_DUST_IX_DISCM) {
            return Ok(Self::CollectDust);
        }
        if buf.starts_with(&ADD_STRATEGY_IX_DISCM) {
            return Ok(Self::AddStrategy);
        }
        if buf.starts_with(&DEPOSIT_STRATEGY_IX_DISCM) {
            let mut reader = &buf[DEPOSIT_STRATEGY_IX_DISCM.len()..];
            let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(Self::DepositStrategy(DepositStrategyIxArgs { amount }));
        }
        if buf.starts_with(&WITHDRAW_STRATEGY_IX_DISCM) {
            let mut reader = &buf[WITHDRAW_STRATEGY_IX_DISCM.len()..];
            let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(Self::WithdrawStrategy(WithdrawStrategyIxArgs { amount }));
        }
        if buf.starts_with(&WITHDRAW2_IX_DISCM) {
            let mut reader = &buf[WITHDRAW2_IX_DISCM.len()..];
            let unmint_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            let min_out_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::Withdraw2(Withdraw2IxArgs {
                    unmint_amount,
                    min_out_amount,
                }),
            );
        }
        if buf.starts_with(&DEPOSIT_IX_DISCM) {
            let mut reader = &buf[DEPOSIT_IX_DISCM.len()..];
            let token_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            let minimum_lp_token_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::Deposit(DepositIxArgs {
                    token_amount,
                    minimum_lp_token_amount,
                }),
            );
        }
        if buf.starts_with(&WITHDRAW_IX_DISCM) {
            let mut reader = &buf[WITHDRAW_IX_DISCM.len()..];
            let unmint_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            let min_out_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::Withdraw(WithdrawIxArgs {
                    unmint_amount,
                    min_out_amount,
                }),
            );
        }
        if buf.starts_with(&WITHDRAW_DIRECTLY_FROM_STRATEGY_IX_DISCM) {
            let mut reader = &buf[WITHDRAW_DIRECTLY_FROM_STRATEGY_IX_DISCM.len()..];
            let unmint_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            let min_out_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::WithdrawDirectlyFromStrategy(WithdrawDirectlyFromStrategyIxArgs {
                    unmint_amount,
                    min_out_amount,
                }),
            );
        }
        Err(std::io::Error::from(std::io::ErrorKind::InvalidData))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        match self {
            Self::Initialize => writer.write_all(&INITIALIZE_IX_DISCM),
            Self::EnableVault(args) => {
                writer.write_all(&ENABLE_VAULT_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.enabled, &mut writer)?;
                Ok(())
            }
            Self::SetOperator => writer.write_all(&SET_OPERATOR_IX_DISCM),
            Self::InitializeStrategy(args) => {
                writer.write_all(&INITIALIZE_STRATEGY_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.bumps, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.strategy_type, &mut writer)?;
                Ok(())
            }
            Self::RemoveStrategy => writer.write_all(&REMOVE_STRATEGY_IX_DISCM),
            Self::RemoveStrategy2(args) => {
                writer.write_all(&REMOVE_STRATEGY2_IX_DISCM)?;
                borsh::BorshSerialize::serialize(
                    &args.max_admin_pay_amount,
                    &mut writer,
                )?;
                Ok(())
            }
            Self::CollectDust => writer.write_all(&COLLECT_DUST_IX_DISCM),
            Self::AddStrategy => writer.write_all(&ADD_STRATEGY_IX_DISCM),
            Self::DepositStrategy(args) => {
                writer.write_all(&DEPOSIT_STRATEGY_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.amount, &mut writer)?;
                Ok(())
            }
            Self::WithdrawStrategy(args) => {
                writer.write_all(&WITHDRAW_STRATEGY_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.amount, &mut writer)?;
                Ok(())
            }
            Self::Withdraw2(args) => {
                writer.write_all(&WITHDRAW2_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.unmint_amount, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.min_out_amount, &mut writer)?;
                Ok(())
            }
            Self::Deposit(args) => {
                writer.write_all(&DEPOSIT_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.token_amount, &mut writer)?;
                borsh::BorshSerialize::serialize(
                    &args.minimum_lp_token_amount,
                    &mut writer,
                )?;
                Ok(())
            }
            Self::Withdraw(args) => {
                writer.write_all(&WITHDRAW_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.unmint_amount, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.min_out_amount, &mut writer)?;
                Ok(())
            }
            Self::WithdrawDirectlyFromStrategy(args) => {
                writer.write_all(&WITHDRAW_DIRECTLY_FROM_STRATEGY_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.unmint_amount, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.min_out_amount, &mut writer)?;
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
pub const INITIALIZE_IX_ACCOUNTS_LEN: usize = 8;
#[derive(Copy, Clone, Debug)]
pub struct InitializeAccounts<'me, 'info> {
    pub vault: &'me AccountInfo<'info>,
    pub payer: &'me AccountInfo<'info>,
    pub token_vault: &'me AccountInfo<'info>,
    pub token_mint: &'me AccountInfo<'info>,
    pub lp_mint: &'me AccountInfo<'info>,
    pub rent: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct InitializeKeys {
    pub vault: Pubkey,
    pub payer: Pubkey,
    pub token_vault: Pubkey,
    pub token_mint: Pubkey,
    pub lp_mint: Pubkey,
    pub rent: Pubkey,
    pub token_program: Pubkey,
    pub system_program: Pubkey,
}
impl From<InitializeAccounts<'_, '_>> for InitializeKeys {
    fn from(accounts: InitializeAccounts) -> Self {
        Self {
            vault: *accounts.vault.key,
            payer: *accounts.payer.key,
            token_vault: *accounts.token_vault.key,
            token_mint: *accounts.token_mint.key,
            lp_mint: *accounts.lp_mint.key,
            rent: *accounts.rent.key,
            token_program: *accounts.token_program.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<InitializeKeys> for [AccountMeta; INITIALIZE_IX_ACCOUNTS_LEN] {
    fn from(keys: InitializeKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.payer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.lp_mint,
                is_signer: false,
                is_writable: true,
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
            vault: pubkeys[0],
            payer: pubkeys[1],
            token_vault: pubkeys[2],
            token_mint: pubkeys[3],
            lp_mint: pubkeys[4],
            rent: pubkeys[5],
            token_program: pubkeys[6],
            system_program: pubkeys[7],
        }
    }
}
impl<'info> From<InitializeAccounts<'_, 'info>>
for [AccountInfo<'info>; INITIALIZE_IX_ACCOUNTS_LEN] {
    fn from(accounts: InitializeAccounts<'_, 'info>) -> Self {
        [
            accounts.vault.clone(),
            accounts.payer.clone(),
            accounts.token_vault.clone(),
            accounts.token_mint.clone(),
            accounts.lp_mint.clone(),
            accounts.rent.clone(),
            accounts.token_program.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; INITIALIZE_IX_ACCOUNTS_LEN]>
for InitializeAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; INITIALIZE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            vault: &arr[0],
            payer: &arr[1],
            token_vault: &arr[2],
            token_mint: &arr[3],
            lp_mint: &arr[4],
            rent: &arr[5],
            token_program: &arr[6],
            system_program: &arr[7],
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
    initialize_ix_with_program_id(VAULT_PROGRAM_ID, keys)
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
    initialize_invoke_with_program_id(VAULT_PROGRAM_ID, accounts)
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
    initialize_invoke_signed_with_program_id(VAULT_PROGRAM_ID, accounts, seeds)
}
pub fn initialize_verify_account_keys(
    accounts: InitializeAccounts<'_, '_>,
    keys: InitializeKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.vault.key, keys.vault),
        (*accounts.payer.key, keys.payer),
        (*accounts.token_vault.key, keys.token_vault),
        (*accounts.token_mint.key, keys.token_mint),
        (*accounts.lp_mint.key, keys.lp_mint),
        (*accounts.rent.key, keys.rent),
        (*accounts.token_program.key, keys.token_program),
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
    for should_be_writable in [
        accounts.vault,
        accounts.payer,
        accounts.token_vault,
        accounts.lp_mint,
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
    for should_be_signer in [accounts.payer] {
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
pub const ENABLE_VAULT_IX_ACCOUNTS_LEN: usize = 2;
#[derive(Copy, Clone, Debug)]
pub struct EnableVaultAccounts<'me, 'info> {
    pub vault: &'me AccountInfo<'info>,
    pub admin: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct EnableVaultKeys {
    pub vault: Pubkey,
    pub admin: Pubkey,
}
impl From<EnableVaultAccounts<'_, '_>> for EnableVaultKeys {
    fn from(accounts: EnableVaultAccounts) -> Self {
        Self {
            vault: *accounts.vault.key,
            admin: *accounts.admin.key,
        }
    }
}
impl From<EnableVaultKeys> for [AccountMeta; ENABLE_VAULT_IX_ACCOUNTS_LEN] {
    fn from(keys: EnableVaultKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.vault,
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
impl From<[Pubkey; ENABLE_VAULT_IX_ACCOUNTS_LEN]> for EnableVaultKeys {
    fn from(pubkeys: [Pubkey; ENABLE_VAULT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            vault: pubkeys[0],
            admin: pubkeys[1],
        }
    }
}
impl<'info> From<EnableVaultAccounts<'_, 'info>>
for [AccountInfo<'info>; ENABLE_VAULT_IX_ACCOUNTS_LEN] {
    fn from(accounts: EnableVaultAccounts<'_, 'info>) -> Self {
        [accounts.vault.clone(), accounts.admin.clone()]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; ENABLE_VAULT_IX_ACCOUNTS_LEN]>
for EnableVaultAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; ENABLE_VAULT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            vault: &arr[0],
            admin: &arr[1],
        }
    }
}
pub const ENABLE_VAULT_IX_DISCM: [u8; 8usize] = [145, 82, 241, 156, 26, 154, 233, 211];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct EnableVaultIxArgs {
    pub enabled: u8,
}
#[derive(Clone, Debug, PartialEq)]
pub struct EnableVaultIxData(pub EnableVaultIxArgs);
impl From<EnableVaultIxArgs> for EnableVaultIxData {
    fn from(args: EnableVaultIxArgs) -> Self {
        Self(args)
    }
}
impl EnableVaultIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != ENABLE_VAULT_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let enabled: u8 = crate::borsh_de_or_default(&mut reader)?;
        Ok(Self(EnableVaultIxArgs { enabled }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&ENABLE_VAULT_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.enabled, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn enable_vault_ix_with_program_id(
    program_id: Pubkey,
    keys: EnableVaultKeys,
    args: EnableVaultIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; ENABLE_VAULT_IX_ACCOUNTS_LEN] = keys.into();
    let data: EnableVaultIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn enable_vault_ix(
    keys: EnableVaultKeys,
    args: EnableVaultIxArgs,
) -> std::io::Result<Instruction> {
    enable_vault_ix_with_program_id(VAULT_PROGRAM_ID, keys, args)
}
pub fn enable_vault_invoke_with_program_id(
    program_id: Pubkey,
    accounts: EnableVaultAccounts<'_, '_>,
    args: EnableVaultIxArgs,
) -> ProgramResult {
    let keys: EnableVaultKeys = accounts.into();
    let ix = enable_vault_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn enable_vault_invoke(
    accounts: EnableVaultAccounts<'_, '_>,
    args: EnableVaultIxArgs,
) -> ProgramResult {
    enable_vault_invoke_with_program_id(VAULT_PROGRAM_ID, accounts, args)
}
pub fn enable_vault_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: EnableVaultAccounts<'_, '_>,
    args: EnableVaultIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: EnableVaultKeys = accounts.into();
    let ix = enable_vault_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn enable_vault_invoke_signed(
    accounts: EnableVaultAccounts<'_, '_>,
    args: EnableVaultIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    enable_vault_invoke_signed_with_program_id(VAULT_PROGRAM_ID, accounts, args, seeds)
}
pub fn enable_vault_verify_account_keys(
    accounts: EnableVaultAccounts<'_, '_>,
    keys: EnableVaultKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.vault.key, keys.vault),
        (*accounts.admin.key, keys.admin),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn enable_vault_verify_writable_privileges<'me, 'info>(
    accounts: EnableVaultAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.vault] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn enable_vault_verify_signer_privileges<'me, 'info>(
    accounts: EnableVaultAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn enable_vault_verify_account_privileges<'me, 'info>(
    accounts: EnableVaultAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    enable_vault_verify_writable_privileges(accounts)?;
    enable_vault_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SET_OPERATOR_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct SetOperatorAccounts<'me, 'info> {
    pub vault: &'me AccountInfo<'info>,
    pub operator: &'me AccountInfo<'info>,
    pub admin: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SetOperatorKeys {
    pub vault: Pubkey,
    pub operator: Pubkey,
    pub admin: Pubkey,
}
impl From<SetOperatorAccounts<'_, '_>> for SetOperatorKeys {
    fn from(accounts: SetOperatorAccounts) -> Self {
        Self {
            vault: *accounts.vault.key,
            operator: *accounts.operator.key,
            admin: *accounts.admin.key,
        }
    }
}
impl From<SetOperatorKeys> for [AccountMeta; SET_OPERATOR_IX_ACCOUNTS_LEN] {
    fn from(keys: SetOperatorKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.operator,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.admin,
                is_signer: true,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; SET_OPERATOR_IX_ACCOUNTS_LEN]> for SetOperatorKeys {
    fn from(pubkeys: [Pubkey; SET_OPERATOR_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            vault: pubkeys[0],
            operator: pubkeys[1],
            admin: pubkeys[2],
        }
    }
}
impl<'info> From<SetOperatorAccounts<'_, 'info>>
for [AccountInfo<'info>; SET_OPERATOR_IX_ACCOUNTS_LEN] {
    fn from(accounts: SetOperatorAccounts<'_, 'info>) -> Self {
        [accounts.vault.clone(), accounts.operator.clone(), accounts.admin.clone()]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SET_OPERATOR_IX_ACCOUNTS_LEN]>
for SetOperatorAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; SET_OPERATOR_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            vault: &arr[0],
            operator: &arr[1],
            admin: &arr[2],
        }
    }
}
pub const SET_OPERATOR_IX_DISCM: [u8; 8usize] = [238, 153, 101, 169, 243, 131, 36, 1];
#[derive(Clone, Debug, PartialEq)]
pub struct SetOperatorIxData;
impl SetOperatorIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SET_OPERATOR_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SET_OPERATOR_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn set_operator_ix_with_program_id(
    program_id: Pubkey,
    keys: SetOperatorKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SET_OPERATOR_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: SetOperatorIxData.try_to_vec()?,
    })
}
pub fn set_operator_ix(keys: SetOperatorKeys) -> std::io::Result<Instruction> {
    set_operator_ix_with_program_id(VAULT_PROGRAM_ID, keys)
}
pub fn set_operator_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SetOperatorAccounts<'_, '_>,
) -> ProgramResult {
    let keys: SetOperatorKeys = accounts.into();
    let ix = set_operator_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn set_operator_invoke(accounts: SetOperatorAccounts<'_, '_>) -> ProgramResult {
    set_operator_invoke_with_program_id(VAULT_PROGRAM_ID, accounts)
}
pub fn set_operator_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SetOperatorAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SetOperatorKeys = accounts.into();
    let ix = set_operator_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn set_operator_invoke_signed(
    accounts: SetOperatorAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    set_operator_invoke_signed_with_program_id(VAULT_PROGRAM_ID, accounts, seeds)
}
pub fn set_operator_verify_account_keys(
    accounts: SetOperatorAccounts<'_, '_>,
    keys: SetOperatorKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.vault.key, keys.vault),
        (*accounts.operator.key, keys.operator),
        (*accounts.admin.key, keys.admin),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn set_operator_verify_writable_privileges<'me, 'info>(
    accounts: SetOperatorAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.vault] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn set_operator_verify_signer_privileges<'me, 'info>(
    accounts: SetOperatorAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn set_operator_verify_account_privileges<'me, 'info>(
    accounts: SetOperatorAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    set_operator_verify_writable_privileges(accounts)?;
    set_operator_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const INITIALIZE_STRATEGY_IX_ACCOUNTS_LEN: usize = 10;
#[derive(Copy, Clone, Debug)]
pub struct InitializeStrategyAccounts<'me, 'info> {
    pub vault: &'me AccountInfo<'info>,
    pub strategy_program: &'me AccountInfo<'info>,
    pub strategy: &'me AccountInfo<'info>,
    pub reserve: &'me AccountInfo<'info>,
    pub collateral_vault: &'me AccountInfo<'info>,
    pub collateral_mint: &'me AccountInfo<'info>,
    pub admin: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub rent: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct InitializeStrategyKeys {
    pub vault: Pubkey,
    pub strategy_program: Pubkey,
    pub strategy: Pubkey,
    pub reserve: Pubkey,
    pub collateral_vault: Pubkey,
    pub collateral_mint: Pubkey,
    pub admin: Pubkey,
    pub system_program: Pubkey,
    pub rent: Pubkey,
    pub token_program: Pubkey,
}
impl From<InitializeStrategyAccounts<'_, '_>> for InitializeStrategyKeys {
    fn from(accounts: InitializeStrategyAccounts) -> Self {
        Self {
            vault: *accounts.vault.key,
            strategy_program: *accounts.strategy_program.key,
            strategy: *accounts.strategy.key,
            reserve: *accounts.reserve.key,
            collateral_vault: *accounts.collateral_vault.key,
            collateral_mint: *accounts.collateral_mint.key,
            admin: *accounts.admin.key,
            system_program: *accounts.system_program.key,
            rent: *accounts.rent.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<InitializeStrategyKeys>
for [AccountMeta; INITIALIZE_STRATEGY_IX_ACCOUNTS_LEN] {
    fn from(keys: InitializeStrategyKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.strategy_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.strategy,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.reserve,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.collateral_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.collateral_mint,
                is_signer: false,
                is_writable: false,
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
impl From<[Pubkey; INITIALIZE_STRATEGY_IX_ACCOUNTS_LEN]> for InitializeStrategyKeys {
    fn from(pubkeys: [Pubkey; INITIALIZE_STRATEGY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            vault: pubkeys[0],
            strategy_program: pubkeys[1],
            strategy: pubkeys[2],
            reserve: pubkeys[3],
            collateral_vault: pubkeys[4],
            collateral_mint: pubkeys[5],
            admin: pubkeys[6],
            system_program: pubkeys[7],
            rent: pubkeys[8],
            token_program: pubkeys[9],
        }
    }
}
impl<'info> From<InitializeStrategyAccounts<'_, 'info>>
for [AccountInfo<'info>; INITIALIZE_STRATEGY_IX_ACCOUNTS_LEN] {
    fn from(accounts: InitializeStrategyAccounts<'_, 'info>) -> Self {
        [
            accounts.vault.clone(),
            accounts.strategy_program.clone(),
            accounts.strategy.clone(),
            accounts.reserve.clone(),
            accounts.collateral_vault.clone(),
            accounts.collateral_mint.clone(),
            accounts.admin.clone(),
            accounts.system_program.clone(),
            accounts.rent.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; INITIALIZE_STRATEGY_IX_ACCOUNTS_LEN]>
for InitializeStrategyAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; INITIALIZE_STRATEGY_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            vault: &arr[0],
            strategy_program: &arr[1],
            strategy: &arr[2],
            reserve: &arr[3],
            collateral_vault: &arr[4],
            collateral_mint: &arr[5],
            admin: &arr[6],
            system_program: &arr[7],
            rent: &arr[8],
            token_program: &arr[9],
        }
    }
}
pub const INITIALIZE_STRATEGY_IX_DISCM: [u8; 8usize] = [
    208, 119, 144, 145, 178, 57, 105, 252,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct InitializeStrategyIxArgs {
    pub bumps: StrategyBumps,
    pub strategy_type: StrategyType,
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
        let bumps = if reader.is_empty() {
            Default::default()
        } else {
            <StrategyBumps>::deserialize(&mut reader)?
        };
        let strategy_type: StrategyType = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(InitializeStrategyIxArgs {
                bumps,
                strategy_type,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INITIALIZE_STRATEGY_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.bumps, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.strategy_type, &mut writer)?;
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
    initialize_strategy_ix_with_program_id(VAULT_PROGRAM_ID, keys, args)
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
    initialize_strategy_invoke_with_program_id(VAULT_PROGRAM_ID, accounts, args)
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
        VAULT_PROGRAM_ID,
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
        (*accounts.vault.key, keys.vault),
        (*accounts.strategy_program.key, keys.strategy_program),
        (*accounts.strategy.key, keys.strategy),
        (*accounts.reserve.key, keys.reserve),
        (*accounts.collateral_vault.key, keys.collateral_vault),
        (*accounts.collateral_mint.key, keys.collateral_mint),
        (*accounts.admin.key, keys.admin),
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
pub fn initialize_strategy_verify_writable_privileges<'me, 'info>(
    accounts: InitializeStrategyAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.vault,
        accounts.strategy,
        accounts.reserve,
        accounts.collateral_vault,
        accounts.admin,
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
    for should_be_signer in [accounts.admin] {
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
pub const REMOVE_STRATEGY_IX_ACCOUNTS_LEN: usize = 10;
#[derive(Copy, Clone, Debug)]
pub struct RemoveStrategyAccounts<'me, 'info> {
    pub vault: &'me AccountInfo<'info>,
    pub strategy: &'me AccountInfo<'info>,
    pub strategy_program: &'me AccountInfo<'info>,
    pub collateral_vault: &'me AccountInfo<'info>,
    pub reserve: &'me AccountInfo<'info>,
    pub token_vault: &'me AccountInfo<'info>,
    pub fee_vault: &'me AccountInfo<'info>,
    pub lp_mint: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub admin: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct RemoveStrategyKeys {
    pub vault: Pubkey,
    pub strategy: Pubkey,
    pub strategy_program: Pubkey,
    pub collateral_vault: Pubkey,
    pub reserve: Pubkey,
    pub token_vault: Pubkey,
    pub fee_vault: Pubkey,
    pub lp_mint: Pubkey,
    pub token_program: Pubkey,
    pub admin: Pubkey,
}
impl From<RemoveStrategyAccounts<'_, '_>> for RemoveStrategyKeys {
    fn from(accounts: RemoveStrategyAccounts) -> Self {
        Self {
            vault: *accounts.vault.key,
            strategy: *accounts.strategy.key,
            strategy_program: *accounts.strategy_program.key,
            collateral_vault: *accounts.collateral_vault.key,
            reserve: *accounts.reserve.key,
            token_vault: *accounts.token_vault.key,
            fee_vault: *accounts.fee_vault.key,
            lp_mint: *accounts.lp_mint.key,
            token_program: *accounts.token_program.key,
            admin: *accounts.admin.key,
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
                pubkey: keys.strategy_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.collateral_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.reserve,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.fee_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.lp_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.admin,
                is_signer: true,
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
            strategy_program: pubkeys[2],
            collateral_vault: pubkeys[3],
            reserve: pubkeys[4],
            token_vault: pubkeys[5],
            fee_vault: pubkeys[6],
            lp_mint: pubkeys[7],
            token_program: pubkeys[8],
            admin: pubkeys[9],
        }
    }
}
impl<'info> From<RemoveStrategyAccounts<'_, 'info>>
for [AccountInfo<'info>; REMOVE_STRATEGY_IX_ACCOUNTS_LEN] {
    fn from(accounts: RemoveStrategyAccounts<'_, 'info>) -> Self {
        [
            accounts.vault.clone(),
            accounts.strategy.clone(),
            accounts.strategy_program.clone(),
            accounts.collateral_vault.clone(),
            accounts.reserve.clone(),
            accounts.token_vault.clone(),
            accounts.fee_vault.clone(),
            accounts.lp_mint.clone(),
            accounts.token_program.clone(),
            accounts.admin.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; REMOVE_STRATEGY_IX_ACCOUNTS_LEN]>
for RemoveStrategyAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; REMOVE_STRATEGY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            vault: &arr[0],
            strategy: &arr[1],
            strategy_program: &arr[2],
            collateral_vault: &arr[3],
            reserve: &arr[4],
            token_vault: &arr[5],
            fee_vault: &arr[6],
            lp_mint: &arr[7],
            token_program: &arr[8],
            admin: &arr[9],
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
    remove_strategy_ix_with_program_id(VAULT_PROGRAM_ID, keys)
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
    remove_strategy_invoke_with_program_id(VAULT_PROGRAM_ID, accounts)
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
    remove_strategy_invoke_signed_with_program_id(VAULT_PROGRAM_ID, accounts, seeds)
}
pub fn remove_strategy_verify_account_keys(
    accounts: RemoveStrategyAccounts<'_, '_>,
    keys: RemoveStrategyKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.vault.key, keys.vault),
        (*accounts.strategy.key, keys.strategy),
        (*accounts.strategy_program.key, keys.strategy_program),
        (*accounts.collateral_vault.key, keys.collateral_vault),
        (*accounts.reserve.key, keys.reserve),
        (*accounts.token_vault.key, keys.token_vault),
        (*accounts.fee_vault.key, keys.fee_vault),
        (*accounts.lp_mint.key, keys.lp_mint),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.admin.key, keys.admin),
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
    for should_be_writable in [
        accounts.vault,
        accounts.strategy,
        accounts.collateral_vault,
        accounts.reserve,
        accounts.token_vault,
        accounts.fee_vault,
        accounts.lp_mint,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn remove_strategy_verify_signer_privileges<'me, 'info>(
    accounts: RemoveStrategyAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin] {
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
pub const REMOVE_STRATEGY2_IX_ACCOUNTS_LEN: usize = 12;
#[derive(Copy, Clone, Debug)]
pub struct RemoveStrategy2Accounts<'me, 'info> {
    pub vault: &'me AccountInfo<'info>,
    pub strategy: &'me AccountInfo<'info>,
    pub strategy_program: &'me AccountInfo<'info>,
    pub collateral_vault: &'me AccountInfo<'info>,
    pub reserve: &'me AccountInfo<'info>,
    pub token_vault: &'me AccountInfo<'info>,
    pub token_admin_advance_payment: &'me AccountInfo<'info>,
    pub token_vault_advance_payment: &'me AccountInfo<'info>,
    pub fee_vault: &'me AccountInfo<'info>,
    pub lp_mint: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub admin: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct RemoveStrategy2Keys {
    pub vault: Pubkey,
    pub strategy: Pubkey,
    pub strategy_program: Pubkey,
    pub collateral_vault: Pubkey,
    pub reserve: Pubkey,
    pub token_vault: Pubkey,
    pub token_admin_advance_payment: Pubkey,
    pub token_vault_advance_payment: Pubkey,
    pub fee_vault: Pubkey,
    pub lp_mint: Pubkey,
    pub token_program: Pubkey,
    pub admin: Pubkey,
}
impl From<RemoveStrategy2Accounts<'_, '_>> for RemoveStrategy2Keys {
    fn from(accounts: RemoveStrategy2Accounts) -> Self {
        Self {
            vault: *accounts.vault.key,
            strategy: *accounts.strategy.key,
            strategy_program: *accounts.strategy_program.key,
            collateral_vault: *accounts.collateral_vault.key,
            reserve: *accounts.reserve.key,
            token_vault: *accounts.token_vault.key,
            token_admin_advance_payment: *accounts.token_admin_advance_payment.key,
            token_vault_advance_payment: *accounts.token_vault_advance_payment.key,
            fee_vault: *accounts.fee_vault.key,
            lp_mint: *accounts.lp_mint.key,
            token_program: *accounts.token_program.key,
            admin: *accounts.admin.key,
        }
    }
}
impl From<RemoveStrategy2Keys> for [AccountMeta; REMOVE_STRATEGY2_IX_ACCOUNTS_LEN] {
    fn from(keys: RemoveStrategy2Keys) -> Self {
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
                pubkey: keys.strategy_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.collateral_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.reserve,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_admin_advance_payment,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_vault_advance_payment,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.fee_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.lp_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.admin,
                is_signer: true,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; REMOVE_STRATEGY2_IX_ACCOUNTS_LEN]> for RemoveStrategy2Keys {
    fn from(pubkeys: [Pubkey; REMOVE_STRATEGY2_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            vault: pubkeys[0],
            strategy: pubkeys[1],
            strategy_program: pubkeys[2],
            collateral_vault: pubkeys[3],
            reserve: pubkeys[4],
            token_vault: pubkeys[5],
            token_admin_advance_payment: pubkeys[6],
            token_vault_advance_payment: pubkeys[7],
            fee_vault: pubkeys[8],
            lp_mint: pubkeys[9],
            token_program: pubkeys[10],
            admin: pubkeys[11],
        }
    }
}
impl<'info> From<RemoveStrategy2Accounts<'_, 'info>>
for [AccountInfo<'info>; REMOVE_STRATEGY2_IX_ACCOUNTS_LEN] {
    fn from(accounts: RemoveStrategy2Accounts<'_, 'info>) -> Self {
        [
            accounts.vault.clone(),
            accounts.strategy.clone(),
            accounts.strategy_program.clone(),
            accounts.collateral_vault.clone(),
            accounts.reserve.clone(),
            accounts.token_vault.clone(),
            accounts.token_admin_advance_payment.clone(),
            accounts.token_vault_advance_payment.clone(),
            accounts.fee_vault.clone(),
            accounts.lp_mint.clone(),
            accounts.token_program.clone(),
            accounts.admin.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; REMOVE_STRATEGY2_IX_ACCOUNTS_LEN]>
for RemoveStrategy2Accounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; REMOVE_STRATEGY2_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            vault: &arr[0],
            strategy: &arr[1],
            strategy_program: &arr[2],
            collateral_vault: &arr[3],
            reserve: &arr[4],
            token_vault: &arr[5],
            token_admin_advance_payment: &arr[6],
            token_vault_advance_payment: &arr[7],
            fee_vault: &arr[8],
            lp_mint: &arr[9],
            token_program: &arr[10],
            admin: &arr[11],
        }
    }
}
pub const REMOVE_STRATEGY2_IX_DISCM: [u8; 8usize] = [
    138, 104, 208, 148, 126, 35, 195, 14,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct RemoveStrategy2IxArgs {
    pub max_admin_pay_amount: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct RemoveStrategy2IxData(pub RemoveStrategy2IxArgs);
impl From<RemoveStrategy2IxArgs> for RemoveStrategy2IxData {
    fn from(args: RemoveStrategy2IxArgs) -> Self {
        Self(args)
    }
}
impl RemoveStrategy2IxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != REMOVE_STRATEGY2_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let max_admin_pay_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(RemoveStrategy2IxArgs {
                max_admin_pay_amount,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&REMOVE_STRATEGY2_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.max_admin_pay_amount, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn remove_strategy2_ix_with_program_id(
    program_id: Pubkey,
    keys: RemoveStrategy2Keys,
    args: RemoveStrategy2IxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; REMOVE_STRATEGY2_IX_ACCOUNTS_LEN] = keys.into();
    let data: RemoveStrategy2IxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn remove_strategy2_ix(
    keys: RemoveStrategy2Keys,
    args: RemoveStrategy2IxArgs,
) -> std::io::Result<Instruction> {
    remove_strategy2_ix_with_program_id(VAULT_PROGRAM_ID, keys, args)
}
pub fn remove_strategy2_invoke_with_program_id(
    program_id: Pubkey,
    accounts: RemoveStrategy2Accounts<'_, '_>,
    args: RemoveStrategy2IxArgs,
) -> ProgramResult {
    let keys: RemoveStrategy2Keys = accounts.into();
    let ix = remove_strategy2_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn remove_strategy2_invoke(
    accounts: RemoveStrategy2Accounts<'_, '_>,
    args: RemoveStrategy2IxArgs,
) -> ProgramResult {
    remove_strategy2_invoke_with_program_id(VAULT_PROGRAM_ID, accounts, args)
}
pub fn remove_strategy2_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: RemoveStrategy2Accounts<'_, '_>,
    args: RemoveStrategy2IxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: RemoveStrategy2Keys = accounts.into();
    let ix = remove_strategy2_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn remove_strategy2_invoke_signed(
    accounts: RemoveStrategy2Accounts<'_, '_>,
    args: RemoveStrategy2IxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    remove_strategy2_invoke_signed_with_program_id(
        VAULT_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn remove_strategy2_verify_account_keys(
    accounts: RemoveStrategy2Accounts<'_, '_>,
    keys: RemoveStrategy2Keys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.vault.key, keys.vault),
        (*accounts.strategy.key, keys.strategy),
        (*accounts.strategy_program.key, keys.strategy_program),
        (*accounts.collateral_vault.key, keys.collateral_vault),
        (*accounts.reserve.key, keys.reserve),
        (*accounts.token_vault.key, keys.token_vault),
        (*accounts.token_admin_advance_payment.key, keys.token_admin_advance_payment),
        (*accounts.token_vault_advance_payment.key, keys.token_vault_advance_payment),
        (*accounts.fee_vault.key, keys.fee_vault),
        (*accounts.lp_mint.key, keys.lp_mint),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.admin.key, keys.admin),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn remove_strategy2_verify_writable_privileges<'me, 'info>(
    accounts: RemoveStrategy2Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.vault,
        accounts.strategy,
        accounts.collateral_vault,
        accounts.reserve,
        accounts.token_vault,
        accounts.token_admin_advance_payment,
        accounts.token_vault_advance_payment,
        accounts.fee_vault,
        accounts.lp_mint,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn remove_strategy2_verify_signer_privileges<'me, 'info>(
    accounts: RemoveStrategy2Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn remove_strategy2_verify_account_privileges<'me, 'info>(
    accounts: RemoveStrategy2Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    remove_strategy2_verify_writable_privileges(accounts)?;
    remove_strategy2_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const COLLECT_DUST_IX_ACCOUNTS_LEN: usize = 5;
#[derive(Copy, Clone, Debug)]
pub struct CollectDustAccounts<'me, 'info> {
    pub vault: &'me AccountInfo<'info>,
    pub token_vault: &'me AccountInfo<'info>,
    pub token_admin: &'me AccountInfo<'info>,
    pub admin: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CollectDustKeys {
    pub vault: Pubkey,
    pub token_vault: Pubkey,
    pub token_admin: Pubkey,
    pub admin: Pubkey,
    pub token_program: Pubkey,
}
impl From<CollectDustAccounts<'_, '_>> for CollectDustKeys {
    fn from(accounts: CollectDustAccounts) -> Self {
        Self {
            vault: *accounts.vault.key,
            token_vault: *accounts.token_vault.key,
            token_admin: *accounts.token_admin.key,
            admin: *accounts.admin.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<CollectDustKeys> for [AccountMeta; COLLECT_DUST_IX_ACCOUNTS_LEN] {
    fn from(keys: CollectDustKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.vault,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_admin,
                is_signer: false,
                is_writable: true,
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
impl From<[Pubkey; COLLECT_DUST_IX_ACCOUNTS_LEN]> for CollectDustKeys {
    fn from(pubkeys: [Pubkey; COLLECT_DUST_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            vault: pubkeys[0],
            token_vault: pubkeys[1],
            token_admin: pubkeys[2],
            admin: pubkeys[3],
            token_program: pubkeys[4],
        }
    }
}
impl<'info> From<CollectDustAccounts<'_, 'info>>
for [AccountInfo<'info>; COLLECT_DUST_IX_ACCOUNTS_LEN] {
    fn from(accounts: CollectDustAccounts<'_, 'info>) -> Self {
        [
            accounts.vault.clone(),
            accounts.token_vault.clone(),
            accounts.token_admin.clone(),
            accounts.admin.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; COLLECT_DUST_IX_ACCOUNTS_LEN]>
for CollectDustAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; COLLECT_DUST_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            vault: &arr[0],
            token_vault: &arr[1],
            token_admin: &arr[2],
            admin: &arr[3],
            token_program: &arr[4],
        }
    }
}
pub const COLLECT_DUST_IX_DISCM: [u8; 8usize] = [246, 149, 21, 82, 160, 74, 254, 240];
#[derive(Clone, Debug, PartialEq)]
pub struct CollectDustIxData;
impl CollectDustIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != COLLECT_DUST_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&COLLECT_DUST_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn collect_dust_ix_with_program_id(
    program_id: Pubkey,
    keys: CollectDustKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; COLLECT_DUST_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: CollectDustIxData.try_to_vec()?,
    })
}
pub fn collect_dust_ix(keys: CollectDustKeys) -> std::io::Result<Instruction> {
    collect_dust_ix_with_program_id(VAULT_PROGRAM_ID, keys)
}
pub fn collect_dust_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CollectDustAccounts<'_, '_>,
) -> ProgramResult {
    let keys: CollectDustKeys = accounts.into();
    let ix = collect_dust_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn collect_dust_invoke(accounts: CollectDustAccounts<'_, '_>) -> ProgramResult {
    collect_dust_invoke_with_program_id(VAULT_PROGRAM_ID, accounts)
}
pub fn collect_dust_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CollectDustAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CollectDustKeys = accounts.into();
    let ix = collect_dust_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn collect_dust_invoke_signed(
    accounts: CollectDustAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    collect_dust_invoke_signed_with_program_id(VAULT_PROGRAM_ID, accounts, seeds)
}
pub fn collect_dust_verify_account_keys(
    accounts: CollectDustAccounts<'_, '_>,
    keys: CollectDustKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.vault.key, keys.vault),
        (*accounts.token_vault.key, keys.token_vault),
        (*accounts.token_admin.key, keys.token_admin),
        (*accounts.admin.key, keys.admin),
        (*accounts.token_program.key, keys.token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn collect_dust_verify_writable_privileges<'me, 'info>(
    accounts: CollectDustAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.token_vault, accounts.token_admin] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn collect_dust_verify_signer_privileges<'me, 'info>(
    accounts: CollectDustAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn collect_dust_verify_account_privileges<'me, 'info>(
    accounts: CollectDustAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    collect_dust_verify_writable_privileges(accounts)?;
    collect_dust_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const ADD_STRATEGY_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct AddStrategyAccounts<'me, 'info> {
    pub vault: &'me AccountInfo<'info>,
    pub strategy: &'me AccountInfo<'info>,
    pub admin: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct AddStrategyKeys {
    pub vault: Pubkey,
    pub strategy: Pubkey,
    pub admin: Pubkey,
}
impl From<AddStrategyAccounts<'_, '_>> for AddStrategyKeys {
    fn from(accounts: AddStrategyAccounts) -> Self {
        Self {
            vault: *accounts.vault.key,
            strategy: *accounts.strategy.key,
            admin: *accounts.admin.key,
        }
    }
}
impl From<AddStrategyKeys> for [AccountMeta; ADD_STRATEGY_IX_ACCOUNTS_LEN] {
    fn from(keys: AddStrategyKeys) -> Self {
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
                pubkey: keys.admin,
                is_signer: true,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; ADD_STRATEGY_IX_ACCOUNTS_LEN]> for AddStrategyKeys {
    fn from(pubkeys: [Pubkey; ADD_STRATEGY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            vault: pubkeys[0],
            strategy: pubkeys[1],
            admin: pubkeys[2],
        }
    }
}
impl<'info> From<AddStrategyAccounts<'_, 'info>>
for [AccountInfo<'info>; ADD_STRATEGY_IX_ACCOUNTS_LEN] {
    fn from(accounts: AddStrategyAccounts<'_, 'info>) -> Self {
        [accounts.vault.clone(), accounts.strategy.clone(), accounts.admin.clone()]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; ADD_STRATEGY_IX_ACCOUNTS_LEN]>
for AddStrategyAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; ADD_STRATEGY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            vault: &arr[0],
            strategy: &arr[1],
            admin: &arr[2],
        }
    }
}
pub const ADD_STRATEGY_IX_DISCM: [u8; 8usize] = [64, 123, 127, 227, 192, 234, 198, 20];
#[derive(Clone, Debug, PartialEq)]
pub struct AddStrategyIxData;
impl AddStrategyIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != ADD_STRATEGY_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&ADD_STRATEGY_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn add_strategy_ix_with_program_id(
    program_id: Pubkey,
    keys: AddStrategyKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; ADD_STRATEGY_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: AddStrategyIxData.try_to_vec()?,
    })
}
pub fn add_strategy_ix(keys: AddStrategyKeys) -> std::io::Result<Instruction> {
    add_strategy_ix_with_program_id(VAULT_PROGRAM_ID, keys)
}
pub fn add_strategy_invoke_with_program_id(
    program_id: Pubkey,
    accounts: AddStrategyAccounts<'_, '_>,
) -> ProgramResult {
    let keys: AddStrategyKeys = accounts.into();
    let ix = add_strategy_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn add_strategy_invoke(accounts: AddStrategyAccounts<'_, '_>) -> ProgramResult {
    add_strategy_invoke_with_program_id(VAULT_PROGRAM_ID, accounts)
}
pub fn add_strategy_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: AddStrategyAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: AddStrategyKeys = accounts.into();
    let ix = add_strategy_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn add_strategy_invoke_signed(
    accounts: AddStrategyAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    add_strategy_invoke_signed_with_program_id(VAULT_PROGRAM_ID, accounts, seeds)
}
pub fn add_strategy_verify_account_keys(
    accounts: AddStrategyAccounts<'_, '_>,
    keys: AddStrategyKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.vault.key, keys.vault),
        (*accounts.strategy.key, keys.strategy),
        (*accounts.admin.key, keys.admin),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn add_strategy_verify_writable_privileges<'me, 'info>(
    accounts: AddStrategyAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.vault] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn add_strategy_verify_signer_privileges<'me, 'info>(
    accounts: AddStrategyAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn add_strategy_verify_account_privileges<'me, 'info>(
    accounts: AddStrategyAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    add_strategy_verify_writable_privileges(accounts)?;
    add_strategy_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const DEPOSIT_STRATEGY_IX_ACCOUNTS_LEN: usize = 10;
#[derive(Copy, Clone, Debug)]
pub struct DepositStrategyAccounts<'me, 'info> {
    pub vault: &'me AccountInfo<'info>,
    pub strategy: &'me AccountInfo<'info>,
    pub token_vault: &'me AccountInfo<'info>,
    pub fee_vault: &'me AccountInfo<'info>,
    pub lp_mint: &'me AccountInfo<'info>,
    pub strategy_program: &'me AccountInfo<'info>,
    pub collateral_vault: &'me AccountInfo<'info>,
    pub reserve: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub operator: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct DepositStrategyKeys {
    pub vault: Pubkey,
    pub strategy: Pubkey,
    pub token_vault: Pubkey,
    pub fee_vault: Pubkey,
    pub lp_mint: Pubkey,
    pub strategy_program: Pubkey,
    pub collateral_vault: Pubkey,
    pub reserve: Pubkey,
    pub token_program: Pubkey,
    pub operator: Pubkey,
}
impl From<DepositStrategyAccounts<'_, '_>> for DepositStrategyKeys {
    fn from(accounts: DepositStrategyAccounts) -> Self {
        Self {
            vault: *accounts.vault.key,
            strategy: *accounts.strategy.key,
            token_vault: *accounts.token_vault.key,
            fee_vault: *accounts.fee_vault.key,
            lp_mint: *accounts.lp_mint.key,
            strategy_program: *accounts.strategy_program.key,
            collateral_vault: *accounts.collateral_vault.key,
            reserve: *accounts.reserve.key,
            token_program: *accounts.token_program.key,
            operator: *accounts.operator.key,
        }
    }
}
impl From<DepositStrategyKeys> for [AccountMeta; DEPOSIT_STRATEGY_IX_ACCOUNTS_LEN] {
    fn from(keys: DepositStrategyKeys) -> Self {
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
                pubkey: keys.token_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.fee_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.lp_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.strategy_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.collateral_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.reserve,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.operator,
                is_signer: true,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; DEPOSIT_STRATEGY_IX_ACCOUNTS_LEN]> for DepositStrategyKeys {
    fn from(pubkeys: [Pubkey; DEPOSIT_STRATEGY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            vault: pubkeys[0],
            strategy: pubkeys[1],
            token_vault: pubkeys[2],
            fee_vault: pubkeys[3],
            lp_mint: pubkeys[4],
            strategy_program: pubkeys[5],
            collateral_vault: pubkeys[6],
            reserve: pubkeys[7],
            token_program: pubkeys[8],
            operator: pubkeys[9],
        }
    }
}
impl<'info> From<DepositStrategyAccounts<'_, 'info>>
for [AccountInfo<'info>; DEPOSIT_STRATEGY_IX_ACCOUNTS_LEN] {
    fn from(accounts: DepositStrategyAccounts<'_, 'info>) -> Self {
        [
            accounts.vault.clone(),
            accounts.strategy.clone(),
            accounts.token_vault.clone(),
            accounts.fee_vault.clone(),
            accounts.lp_mint.clone(),
            accounts.strategy_program.clone(),
            accounts.collateral_vault.clone(),
            accounts.reserve.clone(),
            accounts.token_program.clone(),
            accounts.operator.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; DEPOSIT_STRATEGY_IX_ACCOUNTS_LEN]>
for DepositStrategyAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; DEPOSIT_STRATEGY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            vault: &arr[0],
            strategy: &arr[1],
            token_vault: &arr[2],
            fee_vault: &arr[3],
            lp_mint: &arr[4],
            strategy_program: &arr[5],
            collateral_vault: &arr[6],
            reserve: &arr[7],
            token_program: &arr[8],
            operator: &arr[9],
        }
    }
}
pub const DEPOSIT_STRATEGY_IX_DISCM: [u8; 8usize] = [
    246, 82, 57, 226, 131, 222, 253, 249,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DepositStrategyIxArgs {
    pub amount: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct DepositStrategyIxData(pub DepositStrategyIxArgs);
impl From<DepositStrategyIxArgs> for DepositStrategyIxData {
    fn from(args: DepositStrategyIxArgs) -> Self {
        Self(args)
    }
}
impl DepositStrategyIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != DEPOSIT_STRATEGY_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(Self(DepositStrategyIxArgs { amount }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&DEPOSIT_STRATEGY_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.amount, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn deposit_strategy_ix_with_program_id(
    program_id: Pubkey,
    keys: DepositStrategyKeys,
    args: DepositStrategyIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; DEPOSIT_STRATEGY_IX_ACCOUNTS_LEN] = keys.into();
    let data: DepositStrategyIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn deposit_strategy_ix(
    keys: DepositStrategyKeys,
    args: DepositStrategyIxArgs,
) -> std::io::Result<Instruction> {
    deposit_strategy_ix_with_program_id(VAULT_PROGRAM_ID, keys, args)
}
pub fn deposit_strategy_invoke_with_program_id(
    program_id: Pubkey,
    accounts: DepositStrategyAccounts<'_, '_>,
    args: DepositStrategyIxArgs,
) -> ProgramResult {
    let keys: DepositStrategyKeys = accounts.into();
    let ix = deposit_strategy_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn deposit_strategy_invoke(
    accounts: DepositStrategyAccounts<'_, '_>,
    args: DepositStrategyIxArgs,
) -> ProgramResult {
    deposit_strategy_invoke_with_program_id(VAULT_PROGRAM_ID, accounts, args)
}
pub fn deposit_strategy_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: DepositStrategyAccounts<'_, '_>,
    args: DepositStrategyIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: DepositStrategyKeys = accounts.into();
    let ix = deposit_strategy_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn deposit_strategy_invoke_signed(
    accounts: DepositStrategyAccounts<'_, '_>,
    args: DepositStrategyIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    deposit_strategy_invoke_signed_with_program_id(
        VAULT_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn deposit_strategy_verify_account_keys(
    accounts: DepositStrategyAccounts<'_, '_>,
    keys: DepositStrategyKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.vault.key, keys.vault),
        (*accounts.strategy.key, keys.strategy),
        (*accounts.token_vault.key, keys.token_vault),
        (*accounts.fee_vault.key, keys.fee_vault),
        (*accounts.lp_mint.key, keys.lp_mint),
        (*accounts.strategy_program.key, keys.strategy_program),
        (*accounts.collateral_vault.key, keys.collateral_vault),
        (*accounts.reserve.key, keys.reserve),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.operator.key, keys.operator),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn deposit_strategy_verify_writable_privileges<'me, 'info>(
    accounts: DepositStrategyAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.vault,
        accounts.strategy,
        accounts.token_vault,
        accounts.fee_vault,
        accounts.lp_mint,
        accounts.collateral_vault,
        accounts.reserve,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn deposit_strategy_verify_signer_privileges<'me, 'info>(
    accounts: DepositStrategyAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.operator] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn deposit_strategy_verify_account_privileges<'me, 'info>(
    accounts: DepositStrategyAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    deposit_strategy_verify_writable_privileges(accounts)?;
    deposit_strategy_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const WITHDRAW_STRATEGY_IX_ACCOUNTS_LEN: usize = 10;
#[derive(Copy, Clone, Debug)]
pub struct WithdrawStrategyAccounts<'me, 'info> {
    pub vault: &'me AccountInfo<'info>,
    pub strategy: &'me AccountInfo<'info>,
    pub token_vault: &'me AccountInfo<'info>,
    pub fee_vault: &'me AccountInfo<'info>,
    pub lp_mint: &'me AccountInfo<'info>,
    pub strategy_program: &'me AccountInfo<'info>,
    pub collateral_vault: &'me AccountInfo<'info>,
    pub reserve: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub operator: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct WithdrawStrategyKeys {
    pub vault: Pubkey,
    pub strategy: Pubkey,
    pub token_vault: Pubkey,
    pub fee_vault: Pubkey,
    pub lp_mint: Pubkey,
    pub strategy_program: Pubkey,
    pub collateral_vault: Pubkey,
    pub reserve: Pubkey,
    pub token_program: Pubkey,
    pub operator: Pubkey,
}
impl From<WithdrawStrategyAccounts<'_, '_>> for WithdrawStrategyKeys {
    fn from(accounts: WithdrawStrategyAccounts) -> Self {
        Self {
            vault: *accounts.vault.key,
            strategy: *accounts.strategy.key,
            token_vault: *accounts.token_vault.key,
            fee_vault: *accounts.fee_vault.key,
            lp_mint: *accounts.lp_mint.key,
            strategy_program: *accounts.strategy_program.key,
            collateral_vault: *accounts.collateral_vault.key,
            reserve: *accounts.reserve.key,
            token_program: *accounts.token_program.key,
            operator: *accounts.operator.key,
        }
    }
}
impl From<WithdrawStrategyKeys> for [AccountMeta; WITHDRAW_STRATEGY_IX_ACCOUNTS_LEN] {
    fn from(keys: WithdrawStrategyKeys) -> Self {
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
                pubkey: keys.token_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.fee_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.lp_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.strategy_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.collateral_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.reserve,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.operator,
                is_signer: true,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; WITHDRAW_STRATEGY_IX_ACCOUNTS_LEN]> for WithdrawStrategyKeys {
    fn from(pubkeys: [Pubkey; WITHDRAW_STRATEGY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            vault: pubkeys[0],
            strategy: pubkeys[1],
            token_vault: pubkeys[2],
            fee_vault: pubkeys[3],
            lp_mint: pubkeys[4],
            strategy_program: pubkeys[5],
            collateral_vault: pubkeys[6],
            reserve: pubkeys[7],
            token_program: pubkeys[8],
            operator: pubkeys[9],
        }
    }
}
impl<'info> From<WithdrawStrategyAccounts<'_, 'info>>
for [AccountInfo<'info>; WITHDRAW_STRATEGY_IX_ACCOUNTS_LEN] {
    fn from(accounts: WithdrawStrategyAccounts<'_, 'info>) -> Self {
        [
            accounts.vault.clone(),
            accounts.strategy.clone(),
            accounts.token_vault.clone(),
            accounts.fee_vault.clone(),
            accounts.lp_mint.clone(),
            accounts.strategy_program.clone(),
            accounts.collateral_vault.clone(),
            accounts.reserve.clone(),
            accounts.token_program.clone(),
            accounts.operator.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; WITHDRAW_STRATEGY_IX_ACCOUNTS_LEN]>
for WithdrawStrategyAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; WITHDRAW_STRATEGY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            vault: &arr[0],
            strategy: &arr[1],
            token_vault: &arr[2],
            fee_vault: &arr[3],
            lp_mint: &arr[4],
            strategy_program: &arr[5],
            collateral_vault: &arr[6],
            reserve: &arr[7],
            token_program: &arr[8],
            operator: &arr[9],
        }
    }
}
pub const WITHDRAW_STRATEGY_IX_DISCM: [u8; 8usize] = [
    31, 45, 162, 5, 193, 217, 134, 188,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WithdrawStrategyIxArgs {
    pub amount: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct WithdrawStrategyIxData(pub WithdrawStrategyIxArgs);
impl From<WithdrawStrategyIxArgs> for WithdrawStrategyIxData {
    fn from(args: WithdrawStrategyIxArgs) -> Self {
        Self(args)
    }
}
impl WithdrawStrategyIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != WITHDRAW_STRATEGY_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(Self(WithdrawStrategyIxArgs { amount }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&WITHDRAW_STRATEGY_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.amount, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn withdraw_strategy_ix_with_program_id(
    program_id: Pubkey,
    keys: WithdrawStrategyKeys,
    args: WithdrawStrategyIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; WITHDRAW_STRATEGY_IX_ACCOUNTS_LEN] = keys.into();
    let data: WithdrawStrategyIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn withdraw_strategy_ix(
    keys: WithdrawStrategyKeys,
    args: WithdrawStrategyIxArgs,
) -> std::io::Result<Instruction> {
    withdraw_strategy_ix_with_program_id(VAULT_PROGRAM_ID, keys, args)
}
pub fn withdraw_strategy_invoke_with_program_id(
    program_id: Pubkey,
    accounts: WithdrawStrategyAccounts<'_, '_>,
    args: WithdrawStrategyIxArgs,
) -> ProgramResult {
    let keys: WithdrawStrategyKeys = accounts.into();
    let ix = withdraw_strategy_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn withdraw_strategy_invoke(
    accounts: WithdrawStrategyAccounts<'_, '_>,
    args: WithdrawStrategyIxArgs,
) -> ProgramResult {
    withdraw_strategy_invoke_with_program_id(VAULT_PROGRAM_ID, accounts, args)
}
pub fn withdraw_strategy_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: WithdrawStrategyAccounts<'_, '_>,
    args: WithdrawStrategyIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: WithdrawStrategyKeys = accounts.into();
    let ix = withdraw_strategy_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn withdraw_strategy_invoke_signed(
    accounts: WithdrawStrategyAccounts<'_, '_>,
    args: WithdrawStrategyIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    withdraw_strategy_invoke_signed_with_program_id(
        VAULT_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn withdraw_strategy_verify_account_keys(
    accounts: WithdrawStrategyAccounts<'_, '_>,
    keys: WithdrawStrategyKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.vault.key, keys.vault),
        (*accounts.strategy.key, keys.strategy),
        (*accounts.token_vault.key, keys.token_vault),
        (*accounts.fee_vault.key, keys.fee_vault),
        (*accounts.lp_mint.key, keys.lp_mint),
        (*accounts.strategy_program.key, keys.strategy_program),
        (*accounts.collateral_vault.key, keys.collateral_vault),
        (*accounts.reserve.key, keys.reserve),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.operator.key, keys.operator),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn withdraw_strategy_verify_writable_privileges<'me, 'info>(
    accounts: WithdrawStrategyAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.vault,
        accounts.strategy,
        accounts.token_vault,
        accounts.fee_vault,
        accounts.lp_mint,
        accounts.collateral_vault,
        accounts.reserve,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn withdraw_strategy_verify_signer_privileges<'me, 'info>(
    accounts: WithdrawStrategyAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.operator] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn withdraw_strategy_verify_account_privileges<'me, 'info>(
    accounts: WithdrawStrategyAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    withdraw_strategy_verify_writable_privileges(accounts)?;
    withdraw_strategy_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const WITHDRAW2_IX_ACCOUNTS_LEN: usize = 7;
#[derive(Copy, Clone, Debug)]
pub struct Withdraw2Accounts<'me, 'info> {
    pub vault: &'me AccountInfo<'info>,
    pub token_vault: &'me AccountInfo<'info>,
    pub lp_mint: &'me AccountInfo<'info>,
    pub user_token: &'me AccountInfo<'info>,
    pub user_lp: &'me AccountInfo<'info>,
    pub user: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Withdraw2Keys {
    pub vault: Pubkey,
    pub token_vault: Pubkey,
    pub lp_mint: Pubkey,
    pub user_token: Pubkey,
    pub user_lp: Pubkey,
    pub user: Pubkey,
    pub token_program: Pubkey,
}
impl From<Withdraw2Accounts<'_, '_>> for Withdraw2Keys {
    fn from(accounts: Withdraw2Accounts) -> Self {
        Self {
            vault: *accounts.vault.key,
            token_vault: *accounts.token_vault.key,
            lp_mint: *accounts.lp_mint.key,
            user_token: *accounts.user_token.key,
            user_lp: *accounts.user_lp.key,
            user: *accounts.user.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<Withdraw2Keys> for [AccountMeta; WITHDRAW2_IX_ACCOUNTS_LEN] {
    fn from(keys: Withdraw2Keys) -> Self {
        [
            AccountMeta {
                pubkey: keys.vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.lp_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_token,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_lp,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user,
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
impl From<[Pubkey; WITHDRAW2_IX_ACCOUNTS_LEN]> for Withdraw2Keys {
    fn from(pubkeys: [Pubkey; WITHDRAW2_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            vault: pubkeys[0],
            token_vault: pubkeys[1],
            lp_mint: pubkeys[2],
            user_token: pubkeys[3],
            user_lp: pubkeys[4],
            user: pubkeys[5],
            token_program: pubkeys[6],
        }
    }
}
impl<'info> From<Withdraw2Accounts<'_, 'info>>
for [AccountInfo<'info>; WITHDRAW2_IX_ACCOUNTS_LEN] {
    fn from(accounts: Withdraw2Accounts<'_, 'info>) -> Self {
        [
            accounts.vault.clone(),
            accounts.token_vault.clone(),
            accounts.lp_mint.clone(),
            accounts.user_token.clone(),
            accounts.user_lp.clone(),
            accounts.user.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; WITHDRAW2_IX_ACCOUNTS_LEN]>
for Withdraw2Accounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; WITHDRAW2_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            vault: &arr[0],
            token_vault: &arr[1],
            lp_mint: &arr[2],
            user_token: &arr[3],
            user_lp: &arr[4],
            user: &arr[5],
            token_program: &arr[6],
        }
    }
}
pub const WITHDRAW2_IX_DISCM: [u8; 8usize] = [80, 6, 111, 73, 174, 211, 66, 132];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Withdraw2IxArgs {
    pub unmint_amount: u64,
    pub min_out_amount: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct Withdraw2IxData(pub Withdraw2IxArgs);
impl From<Withdraw2IxArgs> for Withdraw2IxData {
    fn from(args: Withdraw2IxArgs) -> Self {
        Self(args)
    }
}
impl Withdraw2IxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != WITHDRAW2_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let unmint_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let min_out_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(Withdraw2IxArgs {
                unmint_amount,
                min_out_amount,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&WITHDRAW2_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.unmint_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.min_out_amount, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn withdraw2_ix_with_program_id(
    program_id: Pubkey,
    keys: Withdraw2Keys,
    args: Withdraw2IxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; WITHDRAW2_IX_ACCOUNTS_LEN] = keys.into();
    let data: Withdraw2IxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn withdraw2_ix(
    keys: Withdraw2Keys,
    args: Withdraw2IxArgs,
) -> std::io::Result<Instruction> {
    withdraw2_ix_with_program_id(VAULT_PROGRAM_ID, keys, args)
}
pub fn withdraw2_invoke_with_program_id(
    program_id: Pubkey,
    accounts: Withdraw2Accounts<'_, '_>,
    args: Withdraw2IxArgs,
) -> ProgramResult {
    let keys: Withdraw2Keys = accounts.into();
    let ix = withdraw2_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn withdraw2_invoke(
    accounts: Withdraw2Accounts<'_, '_>,
    args: Withdraw2IxArgs,
) -> ProgramResult {
    withdraw2_invoke_with_program_id(VAULT_PROGRAM_ID, accounts, args)
}
pub fn withdraw2_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: Withdraw2Accounts<'_, '_>,
    args: Withdraw2IxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: Withdraw2Keys = accounts.into();
    let ix = withdraw2_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn withdraw2_invoke_signed(
    accounts: Withdraw2Accounts<'_, '_>,
    args: Withdraw2IxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    withdraw2_invoke_signed_with_program_id(VAULT_PROGRAM_ID, accounts, args, seeds)
}
pub fn withdraw2_verify_account_keys(
    accounts: Withdraw2Accounts<'_, '_>,
    keys: Withdraw2Keys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.vault.key, keys.vault),
        (*accounts.token_vault.key, keys.token_vault),
        (*accounts.lp_mint.key, keys.lp_mint),
        (*accounts.user_token.key, keys.user_token),
        (*accounts.user_lp.key, keys.user_lp),
        (*accounts.user.key, keys.user),
        (*accounts.token_program.key, keys.token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn withdraw2_verify_writable_privileges<'me, 'info>(
    accounts: Withdraw2Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.vault,
        accounts.token_vault,
        accounts.lp_mint,
        accounts.user_token,
        accounts.user_lp,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn withdraw2_verify_signer_privileges<'me, 'info>(
    accounts: Withdraw2Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.user] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn withdraw2_verify_account_privileges<'me, 'info>(
    accounts: Withdraw2Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    withdraw2_verify_writable_privileges(accounts)?;
    withdraw2_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const DEPOSIT_IX_ACCOUNTS_LEN: usize = 7;
#[derive(Copy, Clone, Debug)]
pub struct DepositAccounts<'me, 'info> {
    pub vault: &'me AccountInfo<'info>,
    pub token_vault: &'me AccountInfo<'info>,
    pub lp_mint: &'me AccountInfo<'info>,
    pub user_token: &'me AccountInfo<'info>,
    pub user_lp: &'me AccountInfo<'info>,
    pub user: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct DepositKeys {
    pub vault: Pubkey,
    pub token_vault: Pubkey,
    pub lp_mint: Pubkey,
    pub user_token: Pubkey,
    pub user_lp: Pubkey,
    pub user: Pubkey,
    pub token_program: Pubkey,
}
impl From<DepositAccounts<'_, '_>> for DepositKeys {
    fn from(accounts: DepositAccounts) -> Self {
        Self {
            vault: *accounts.vault.key,
            token_vault: *accounts.token_vault.key,
            lp_mint: *accounts.lp_mint.key,
            user_token: *accounts.user_token.key,
            user_lp: *accounts.user_lp.key,
            user: *accounts.user.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<DepositKeys> for [AccountMeta; DEPOSIT_IX_ACCOUNTS_LEN] {
    fn from(keys: DepositKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.lp_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_token,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_lp,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user,
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
impl From<[Pubkey; DEPOSIT_IX_ACCOUNTS_LEN]> for DepositKeys {
    fn from(pubkeys: [Pubkey; DEPOSIT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            vault: pubkeys[0],
            token_vault: pubkeys[1],
            lp_mint: pubkeys[2],
            user_token: pubkeys[3],
            user_lp: pubkeys[4],
            user: pubkeys[5],
            token_program: pubkeys[6],
        }
    }
}
impl<'info> From<DepositAccounts<'_, 'info>>
for [AccountInfo<'info>; DEPOSIT_IX_ACCOUNTS_LEN] {
    fn from(accounts: DepositAccounts<'_, 'info>) -> Self {
        [
            accounts.vault.clone(),
            accounts.token_vault.clone(),
            accounts.lp_mint.clone(),
            accounts.user_token.clone(),
            accounts.user_lp.clone(),
            accounts.user.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; DEPOSIT_IX_ACCOUNTS_LEN]>
for DepositAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; DEPOSIT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            vault: &arr[0],
            token_vault: &arr[1],
            lp_mint: &arr[2],
            user_token: &arr[3],
            user_lp: &arr[4],
            user: &arr[5],
            token_program: &arr[6],
        }
    }
}
pub const DEPOSIT_IX_DISCM: [u8; 8usize] = [242, 35, 198, 137, 82, 225, 242, 182];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DepositIxArgs {
    pub token_amount: u64,
    pub minimum_lp_token_amount: u64,
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
        let token_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let minimum_lp_token_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(DepositIxArgs {
                token_amount,
                minimum_lp_token_amount,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&DEPOSIT_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.token_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.minimum_lp_token_amount, &mut writer)?;
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
    deposit_ix_with_program_id(VAULT_PROGRAM_ID, keys, args)
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
    deposit_invoke_with_program_id(VAULT_PROGRAM_ID, accounts, args)
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
    deposit_invoke_signed_with_program_id(VAULT_PROGRAM_ID, accounts, args, seeds)
}
pub fn deposit_verify_account_keys(
    accounts: DepositAccounts<'_, '_>,
    keys: DepositKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.vault.key, keys.vault),
        (*accounts.token_vault.key, keys.token_vault),
        (*accounts.lp_mint.key, keys.lp_mint),
        (*accounts.user_token.key, keys.user_token),
        (*accounts.user_lp.key, keys.user_lp),
        (*accounts.user.key, keys.user),
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
        accounts.vault,
        accounts.token_vault,
        accounts.lp_mint,
        accounts.user_token,
        accounts.user_lp,
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
pub const WITHDRAW_IX_ACCOUNTS_LEN: usize = 7;
#[derive(Copy, Clone, Debug)]
pub struct WithdrawAccounts<'me, 'info> {
    pub vault: &'me AccountInfo<'info>,
    pub token_vault: &'me AccountInfo<'info>,
    pub lp_mint: &'me AccountInfo<'info>,
    pub user_token: &'me AccountInfo<'info>,
    pub user_lp: &'me AccountInfo<'info>,
    pub user: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct WithdrawKeys {
    pub vault: Pubkey,
    pub token_vault: Pubkey,
    pub lp_mint: Pubkey,
    pub user_token: Pubkey,
    pub user_lp: Pubkey,
    pub user: Pubkey,
    pub token_program: Pubkey,
}
impl From<WithdrawAccounts<'_, '_>> for WithdrawKeys {
    fn from(accounts: WithdrawAccounts) -> Self {
        Self {
            vault: *accounts.vault.key,
            token_vault: *accounts.token_vault.key,
            lp_mint: *accounts.lp_mint.key,
            user_token: *accounts.user_token.key,
            user_lp: *accounts.user_lp.key,
            user: *accounts.user.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<WithdrawKeys> for [AccountMeta; WITHDRAW_IX_ACCOUNTS_LEN] {
    fn from(keys: WithdrawKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.lp_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_token,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_lp,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user,
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
impl From<[Pubkey; WITHDRAW_IX_ACCOUNTS_LEN]> for WithdrawKeys {
    fn from(pubkeys: [Pubkey; WITHDRAW_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            vault: pubkeys[0],
            token_vault: pubkeys[1],
            lp_mint: pubkeys[2],
            user_token: pubkeys[3],
            user_lp: pubkeys[4],
            user: pubkeys[5],
            token_program: pubkeys[6],
        }
    }
}
impl<'info> From<WithdrawAccounts<'_, 'info>>
for [AccountInfo<'info>; WITHDRAW_IX_ACCOUNTS_LEN] {
    fn from(accounts: WithdrawAccounts<'_, 'info>) -> Self {
        [
            accounts.vault.clone(),
            accounts.token_vault.clone(),
            accounts.lp_mint.clone(),
            accounts.user_token.clone(),
            accounts.user_lp.clone(),
            accounts.user.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; WITHDRAW_IX_ACCOUNTS_LEN]>
for WithdrawAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; WITHDRAW_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            vault: &arr[0],
            token_vault: &arr[1],
            lp_mint: &arr[2],
            user_token: &arr[3],
            user_lp: &arr[4],
            user: &arr[5],
            token_program: &arr[6],
        }
    }
}
pub const WITHDRAW_IX_DISCM: [u8; 8usize] = [183, 18, 70, 156, 148, 109, 161, 34];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WithdrawIxArgs {
    pub unmint_amount: u64,
    pub min_out_amount: u64,
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
        let unmint_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let min_out_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(WithdrawIxArgs {
                unmint_amount,
                min_out_amount,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&WITHDRAW_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.unmint_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.min_out_amount, &mut writer)?;
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
    withdraw_ix_with_program_id(VAULT_PROGRAM_ID, keys, args)
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
    withdraw_invoke_with_program_id(VAULT_PROGRAM_ID, accounts, args)
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
    withdraw_invoke_signed_with_program_id(VAULT_PROGRAM_ID, accounts, args, seeds)
}
pub fn withdraw_verify_account_keys(
    accounts: WithdrawAccounts<'_, '_>,
    keys: WithdrawKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.vault.key, keys.vault),
        (*accounts.token_vault.key, keys.token_vault),
        (*accounts.lp_mint.key, keys.lp_mint),
        (*accounts.user_token.key, keys.user_token),
        (*accounts.user_lp.key, keys.user_lp),
        (*accounts.user.key, keys.user),
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
        accounts.vault,
        accounts.token_vault,
        accounts.lp_mint,
        accounts.user_token,
        accounts.user_lp,
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
pub const WITHDRAW_DIRECTLY_FROM_STRATEGY_IX_ACCOUNTS_LEN: usize = 12;
#[derive(Copy, Clone, Debug)]
pub struct WithdrawDirectlyFromStrategyAccounts<'me, 'info> {
    pub vault: &'me AccountInfo<'info>,
    pub strategy: &'me AccountInfo<'info>,
    pub reserve: &'me AccountInfo<'info>,
    pub strategy_program: &'me AccountInfo<'info>,
    pub collateral_vault: &'me AccountInfo<'info>,
    pub token_vault: &'me AccountInfo<'info>,
    pub lp_mint: &'me AccountInfo<'info>,
    pub fee_vault: &'me AccountInfo<'info>,
    pub user_token: &'me AccountInfo<'info>,
    pub user_lp: &'me AccountInfo<'info>,
    pub user: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct WithdrawDirectlyFromStrategyKeys {
    pub vault: Pubkey,
    pub strategy: Pubkey,
    pub reserve: Pubkey,
    pub strategy_program: Pubkey,
    pub collateral_vault: Pubkey,
    pub token_vault: Pubkey,
    pub lp_mint: Pubkey,
    pub fee_vault: Pubkey,
    pub user_token: Pubkey,
    pub user_lp: Pubkey,
    pub user: Pubkey,
    pub token_program: Pubkey,
}
impl From<WithdrawDirectlyFromStrategyAccounts<'_, '_>>
for WithdrawDirectlyFromStrategyKeys {
    fn from(accounts: WithdrawDirectlyFromStrategyAccounts) -> Self {
        Self {
            vault: *accounts.vault.key,
            strategy: *accounts.strategy.key,
            reserve: *accounts.reserve.key,
            strategy_program: *accounts.strategy_program.key,
            collateral_vault: *accounts.collateral_vault.key,
            token_vault: *accounts.token_vault.key,
            lp_mint: *accounts.lp_mint.key,
            fee_vault: *accounts.fee_vault.key,
            user_token: *accounts.user_token.key,
            user_lp: *accounts.user_lp.key,
            user: *accounts.user.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<WithdrawDirectlyFromStrategyKeys>
for [AccountMeta; WITHDRAW_DIRECTLY_FROM_STRATEGY_IX_ACCOUNTS_LEN] {
    fn from(keys: WithdrawDirectlyFromStrategyKeys) -> Self {
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
                pubkey: keys.reserve,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.strategy_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.collateral_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.lp_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.fee_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_token,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_lp,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user,
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
impl From<[Pubkey; WITHDRAW_DIRECTLY_FROM_STRATEGY_IX_ACCOUNTS_LEN]>
for WithdrawDirectlyFromStrategyKeys {
    fn from(pubkeys: [Pubkey; WITHDRAW_DIRECTLY_FROM_STRATEGY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            vault: pubkeys[0],
            strategy: pubkeys[1],
            reserve: pubkeys[2],
            strategy_program: pubkeys[3],
            collateral_vault: pubkeys[4],
            token_vault: pubkeys[5],
            lp_mint: pubkeys[6],
            fee_vault: pubkeys[7],
            user_token: pubkeys[8],
            user_lp: pubkeys[9],
            user: pubkeys[10],
            token_program: pubkeys[11],
        }
    }
}
impl<'info> From<WithdrawDirectlyFromStrategyAccounts<'_, 'info>>
for [AccountInfo<'info>; WITHDRAW_DIRECTLY_FROM_STRATEGY_IX_ACCOUNTS_LEN] {
    fn from(accounts: WithdrawDirectlyFromStrategyAccounts<'_, 'info>) -> Self {
        [
            accounts.vault.clone(),
            accounts.strategy.clone(),
            accounts.reserve.clone(),
            accounts.strategy_program.clone(),
            accounts.collateral_vault.clone(),
            accounts.token_vault.clone(),
            accounts.lp_mint.clone(),
            accounts.fee_vault.clone(),
            accounts.user_token.clone(),
            accounts.user_lp.clone(),
            accounts.user.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; WITHDRAW_DIRECTLY_FROM_STRATEGY_IX_ACCOUNTS_LEN]>
for WithdrawDirectlyFromStrategyAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; WITHDRAW_DIRECTLY_FROM_STRATEGY_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            vault: &arr[0],
            strategy: &arr[1],
            reserve: &arr[2],
            strategy_program: &arr[3],
            collateral_vault: &arr[4],
            token_vault: &arr[5],
            lp_mint: &arr[6],
            fee_vault: &arr[7],
            user_token: &arr[8],
            user_lp: &arr[9],
            user: &arr[10],
            token_program: &arr[11],
        }
    }
}
pub const WITHDRAW_DIRECTLY_FROM_STRATEGY_IX_DISCM: [u8; 8usize] = [
    201, 141, 146, 46, 173, 116, 198, 22,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WithdrawDirectlyFromStrategyIxArgs {
    pub unmint_amount: u64,
    pub min_out_amount: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct WithdrawDirectlyFromStrategyIxData(pub WithdrawDirectlyFromStrategyIxArgs);
impl From<WithdrawDirectlyFromStrategyIxArgs> for WithdrawDirectlyFromStrategyIxData {
    fn from(args: WithdrawDirectlyFromStrategyIxArgs) -> Self {
        Self(args)
    }
}
impl WithdrawDirectlyFromStrategyIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != WITHDRAW_DIRECTLY_FROM_STRATEGY_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let unmint_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let min_out_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(WithdrawDirectlyFromStrategyIxArgs {
                unmint_amount,
                min_out_amount,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&WITHDRAW_DIRECTLY_FROM_STRATEGY_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.unmint_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.min_out_amount, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn withdraw_directly_from_strategy_ix_with_program_id(
    program_id: Pubkey,
    keys: WithdrawDirectlyFromStrategyKeys,
    args: WithdrawDirectlyFromStrategyIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; WITHDRAW_DIRECTLY_FROM_STRATEGY_IX_ACCOUNTS_LEN] = keys
        .into();
    let data: WithdrawDirectlyFromStrategyIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn withdraw_directly_from_strategy_ix(
    keys: WithdrawDirectlyFromStrategyKeys,
    args: WithdrawDirectlyFromStrategyIxArgs,
) -> std::io::Result<Instruction> {
    withdraw_directly_from_strategy_ix_with_program_id(VAULT_PROGRAM_ID, keys, args)
}
pub fn withdraw_directly_from_strategy_invoke_with_program_id(
    program_id: Pubkey,
    accounts: WithdrawDirectlyFromStrategyAccounts<'_, '_>,
    args: WithdrawDirectlyFromStrategyIxArgs,
) -> ProgramResult {
    let keys: WithdrawDirectlyFromStrategyKeys = accounts.into();
    let ix = withdraw_directly_from_strategy_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn withdraw_directly_from_strategy_invoke(
    accounts: WithdrawDirectlyFromStrategyAccounts<'_, '_>,
    args: WithdrawDirectlyFromStrategyIxArgs,
) -> ProgramResult {
    withdraw_directly_from_strategy_invoke_with_program_id(
        VAULT_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn withdraw_directly_from_strategy_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: WithdrawDirectlyFromStrategyAccounts<'_, '_>,
    args: WithdrawDirectlyFromStrategyIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: WithdrawDirectlyFromStrategyKeys = accounts.into();
    let ix = withdraw_directly_from_strategy_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn withdraw_directly_from_strategy_invoke_signed(
    accounts: WithdrawDirectlyFromStrategyAccounts<'_, '_>,
    args: WithdrawDirectlyFromStrategyIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    withdraw_directly_from_strategy_invoke_signed_with_program_id(
        VAULT_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn withdraw_directly_from_strategy_verify_account_keys(
    accounts: WithdrawDirectlyFromStrategyAccounts<'_, '_>,
    keys: WithdrawDirectlyFromStrategyKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.vault.key, keys.vault),
        (*accounts.strategy.key, keys.strategy),
        (*accounts.reserve.key, keys.reserve),
        (*accounts.strategy_program.key, keys.strategy_program),
        (*accounts.collateral_vault.key, keys.collateral_vault),
        (*accounts.token_vault.key, keys.token_vault),
        (*accounts.lp_mint.key, keys.lp_mint),
        (*accounts.fee_vault.key, keys.fee_vault),
        (*accounts.user_token.key, keys.user_token),
        (*accounts.user_lp.key, keys.user_lp),
        (*accounts.user.key, keys.user),
        (*accounts.token_program.key, keys.token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn withdraw_directly_from_strategy_verify_writable_privileges<'me, 'info>(
    accounts: WithdrawDirectlyFromStrategyAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.vault,
        accounts.strategy,
        accounts.reserve,
        accounts.collateral_vault,
        accounts.token_vault,
        accounts.lp_mint,
        accounts.fee_vault,
        accounts.user_token,
        accounts.user_lp,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn withdraw_directly_from_strategy_verify_signer_privileges<'me, 'info>(
    accounts: WithdrawDirectlyFromStrategyAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.user] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn withdraw_directly_from_strategy_verify_account_privileges<'me, 'info>(
    accounts: WithdrawDirectlyFromStrategyAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    withdraw_directly_from_strategy_verify_writable_privileges(accounts)?;
    withdraw_directly_from_strategy_verify_signer_privileges(accounts)?;
    Ok(())
}
