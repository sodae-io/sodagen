use solana_pubkey::Pubkey;
use solana_cpi::{invoke, invoke_signed};
use solana_instruction::{AccountMeta, Instruction};
use solana_account_info::AccountInfo;
use solana_program_error::ProgramError;
use std::io::Read;
#[allow(unused_imports)]
use crate::*;
#[derive(Clone, Debug, PartialEq)]
pub enum AddDecimalsProgramIx {
    InitializeWrapper(InitializeWrapperIxArgs),
    Deposit(DepositIxArgs),
    Withdraw(WithdrawIxArgs),
}
impl AddDecimalsProgramIx {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        if buf.starts_with(&INITIALIZE_WRAPPER_IX_DISCM) {
            let mut reader = &buf[INITIALIZE_WRAPPER_IX_DISCM.len()..];
            let nonce: u8 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(Self::InitializeWrapper(InitializeWrapperIxArgs { nonce }));
        }
        if buf.starts_with(&DEPOSIT_IX_DISCM) {
            let mut reader = &buf[DEPOSIT_IX_DISCM.len()..];
            let deposit_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(Self::Deposit(DepositIxArgs { deposit_amount }));
        }
        if buf.starts_with(&WITHDRAW_IX_DISCM) {
            let mut reader = &buf[WITHDRAW_IX_DISCM.len()..];
            let max_burn_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(Self::Withdraw(WithdrawIxArgs { max_burn_amount }));
        }
        Err(std::io::Error::from(std::io::ErrorKind::InvalidData))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        match self {
            Self::InitializeWrapper(args) => {
                writer.write_all(&INITIALIZE_WRAPPER_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.nonce, &mut writer)?;
                Ok(())
            }
            Self::Deposit(args) => {
                writer.write_all(&DEPOSIT_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.deposit_amount, &mut writer)?;
                Ok(())
            }
            Self::Withdraw(args) => {
                writer.write_all(&WITHDRAW_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.max_burn_amount, &mut writer)?;
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
pub const INITIALIZE_WRAPPER_IX_ACCOUNTS_LEN: usize = 7;
#[derive(Copy, Clone, Debug)]
pub struct InitializeWrapperAccounts<'me, 'info> {
    pub wrapper: &'me AccountInfo<'info>,
    pub wrapper_underlying_tokens: &'me AccountInfo<'info>,
    pub underlying_mint: &'me AccountInfo<'info>,
    pub wrapper_mint: &'me AccountInfo<'info>,
    pub payer: &'me AccountInfo<'info>,
    pub rent: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct InitializeWrapperKeys {
    pub wrapper: Pubkey,
    pub wrapper_underlying_tokens: Pubkey,
    pub underlying_mint: Pubkey,
    pub wrapper_mint: Pubkey,
    pub payer: Pubkey,
    pub rent: Pubkey,
    pub system_program: Pubkey,
}
impl From<InitializeWrapperAccounts<'_, '_>> for InitializeWrapperKeys {
    fn from(accounts: InitializeWrapperAccounts) -> Self {
        Self {
            wrapper: *accounts.wrapper.key,
            wrapper_underlying_tokens: *accounts.wrapper_underlying_tokens.key,
            underlying_mint: *accounts.underlying_mint.key,
            wrapper_mint: *accounts.wrapper_mint.key,
            payer: *accounts.payer.key,
            rent: *accounts.rent.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<InitializeWrapperKeys> for [AccountMeta; INITIALIZE_WRAPPER_IX_ACCOUNTS_LEN] {
    fn from(keys: InitializeWrapperKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.wrapper,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.wrapper_underlying_tokens,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.underlying_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.wrapper_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.payer,
                is_signer: true,
                is_writable: false,
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
        ]
    }
}
impl From<[Pubkey; INITIALIZE_WRAPPER_IX_ACCOUNTS_LEN]> for InitializeWrapperKeys {
    fn from(pubkeys: [Pubkey; INITIALIZE_WRAPPER_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            wrapper: pubkeys[0],
            wrapper_underlying_tokens: pubkeys[1],
            underlying_mint: pubkeys[2],
            wrapper_mint: pubkeys[3],
            payer: pubkeys[4],
            rent: pubkeys[5],
            system_program: pubkeys[6],
        }
    }
}
impl<'info> From<InitializeWrapperAccounts<'_, 'info>>
for [AccountInfo<'info>; INITIALIZE_WRAPPER_IX_ACCOUNTS_LEN] {
    fn from(accounts: InitializeWrapperAccounts<'_, 'info>) -> Self {
        [
            accounts.wrapper.clone(),
            accounts.wrapper_underlying_tokens.clone(),
            accounts.underlying_mint.clone(),
            accounts.wrapper_mint.clone(),
            accounts.payer.clone(),
            accounts.rent.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; INITIALIZE_WRAPPER_IX_ACCOUNTS_LEN]>
for InitializeWrapperAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; INITIALIZE_WRAPPER_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            wrapper: &arr[0],
            wrapper_underlying_tokens: &arr[1],
            underlying_mint: &arr[2],
            wrapper_mint: &arr[3],
            payer: &arr[4],
            rent: &arr[5],
            system_program: &arr[6],
        }
    }
}
pub const INITIALIZE_WRAPPER_IX_DISCM: [u8; 8usize] = [
    143, 211, 228, 247, 131, 67, 40, 30,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct InitializeWrapperIxArgs {
    pub nonce: u8,
}
#[derive(Clone, Debug, PartialEq)]
pub struct InitializeWrapperIxData(pub InitializeWrapperIxArgs);
impl From<InitializeWrapperIxArgs> for InitializeWrapperIxData {
    fn from(args: InitializeWrapperIxArgs) -> Self {
        Self(args)
    }
}
impl InitializeWrapperIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INITIALIZE_WRAPPER_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let nonce: u8 = crate::borsh_de_or_default(&mut reader)?;
        Ok(Self(InitializeWrapperIxArgs { nonce }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INITIALIZE_WRAPPER_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.nonce, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn initialize_wrapper_ix_with_program_id(
    program_id: Pubkey,
    keys: InitializeWrapperKeys,
    args: InitializeWrapperIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; INITIALIZE_WRAPPER_IX_ACCOUNTS_LEN] = keys.into();
    let data: InitializeWrapperIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn initialize_wrapper_ix(
    keys: InitializeWrapperKeys,
    args: InitializeWrapperIxArgs,
) -> std::io::Result<Instruction> {
    initialize_wrapper_ix_with_program_id(ADD_DECIMALS_PROGRAM_ID, keys, args)
}
pub fn initialize_wrapper_invoke_with_program_id(
    program_id: Pubkey,
    accounts: InitializeWrapperAccounts<'_, '_>,
    args: InitializeWrapperIxArgs,
) -> ProgramResult {
    let keys: InitializeWrapperKeys = accounts.into();
    let ix = initialize_wrapper_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn initialize_wrapper_invoke(
    accounts: InitializeWrapperAccounts<'_, '_>,
    args: InitializeWrapperIxArgs,
) -> ProgramResult {
    initialize_wrapper_invoke_with_program_id(ADD_DECIMALS_PROGRAM_ID, accounts, args)
}
pub fn initialize_wrapper_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: InitializeWrapperAccounts<'_, '_>,
    args: InitializeWrapperIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: InitializeWrapperKeys = accounts.into();
    let ix = initialize_wrapper_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn initialize_wrapper_invoke_signed(
    accounts: InitializeWrapperAccounts<'_, '_>,
    args: InitializeWrapperIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    initialize_wrapper_invoke_signed_with_program_id(
        ADD_DECIMALS_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn initialize_wrapper_verify_account_keys(
    accounts: InitializeWrapperAccounts<'_, '_>,
    keys: InitializeWrapperKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.wrapper.key, keys.wrapper),
        (*accounts.wrapper_underlying_tokens.key, keys.wrapper_underlying_tokens),
        (*accounts.underlying_mint.key, keys.underlying_mint),
        (*accounts.wrapper_mint.key, keys.wrapper_mint),
        (*accounts.payer.key, keys.payer),
        (*accounts.rent.key, keys.rent),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn initialize_wrapper_verify_writable_privileges<'me, 'info>(
    accounts: InitializeWrapperAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.wrapper] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn initialize_wrapper_verify_signer_privileges<'me, 'info>(
    accounts: InitializeWrapperAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.payer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn initialize_wrapper_verify_account_privileges<'me, 'info>(
    accounts: InitializeWrapperAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    initialize_wrapper_verify_writable_privileges(accounts)?;
    initialize_wrapper_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const DEPOSIT_IX_ACCOUNTS_LEN: usize = 7;
#[derive(Copy, Clone, Debug)]
pub struct DepositAccounts<'me, 'info> {
    pub wrapper: &'me AccountInfo<'info>,
    pub wrapper_mint: &'me AccountInfo<'info>,
    pub wrapper_underlying_tokens: &'me AccountInfo<'info>,
    pub owner: &'me AccountInfo<'info>,
    pub user_underlying_tokens: &'me AccountInfo<'info>,
    pub user_wrapped_tokens: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct DepositKeys {
    pub wrapper: Pubkey,
    pub wrapper_mint: Pubkey,
    pub wrapper_underlying_tokens: Pubkey,
    pub owner: Pubkey,
    pub user_underlying_tokens: Pubkey,
    pub user_wrapped_tokens: Pubkey,
    pub token_program: Pubkey,
}
impl From<DepositAccounts<'_, '_>> for DepositKeys {
    fn from(accounts: DepositAccounts) -> Self {
        Self {
            wrapper: *accounts.wrapper.key,
            wrapper_mint: *accounts.wrapper_mint.key,
            wrapper_underlying_tokens: *accounts.wrapper_underlying_tokens.key,
            owner: *accounts.owner.key,
            user_underlying_tokens: *accounts.user_underlying_tokens.key,
            user_wrapped_tokens: *accounts.user_wrapped_tokens.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<DepositKeys> for [AccountMeta; DEPOSIT_IX_ACCOUNTS_LEN] {
    fn from(keys: DepositKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.wrapper,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.wrapper_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.wrapper_underlying_tokens,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.owner,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.user_underlying_tokens,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_wrapped_tokens,
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
            wrapper: pubkeys[0],
            wrapper_mint: pubkeys[1],
            wrapper_underlying_tokens: pubkeys[2],
            owner: pubkeys[3],
            user_underlying_tokens: pubkeys[4],
            user_wrapped_tokens: pubkeys[5],
            token_program: pubkeys[6],
        }
    }
}
impl<'info> From<DepositAccounts<'_, 'info>>
for [AccountInfo<'info>; DEPOSIT_IX_ACCOUNTS_LEN] {
    fn from(accounts: DepositAccounts<'_, 'info>) -> Self {
        [
            accounts.wrapper.clone(),
            accounts.wrapper_mint.clone(),
            accounts.wrapper_underlying_tokens.clone(),
            accounts.owner.clone(),
            accounts.user_underlying_tokens.clone(),
            accounts.user_wrapped_tokens.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; DEPOSIT_IX_ACCOUNTS_LEN]>
for DepositAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; DEPOSIT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            wrapper: &arr[0],
            wrapper_mint: &arr[1],
            wrapper_underlying_tokens: &arr[2],
            owner: &arr[3],
            user_underlying_tokens: &arr[4],
            user_wrapped_tokens: &arr[5],
            token_program: &arr[6],
        }
    }
}
pub const DEPOSIT_IX_DISCM: [u8; 8usize] = [242, 35, 198, 137, 82, 225, 242, 182];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DepositIxArgs {
    pub deposit_amount: u64,
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
        let deposit_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(Self(DepositIxArgs { deposit_amount }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&DEPOSIT_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.deposit_amount, &mut writer)?;
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
    deposit_ix_with_program_id(ADD_DECIMALS_PROGRAM_ID, keys, args)
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
    deposit_invoke_with_program_id(ADD_DECIMALS_PROGRAM_ID, accounts, args)
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
    deposit_invoke_signed_with_program_id(ADD_DECIMALS_PROGRAM_ID, accounts, args, seeds)
}
pub fn deposit_verify_account_keys(
    accounts: DepositAccounts<'_, '_>,
    keys: DepositKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.wrapper.key, keys.wrapper),
        (*accounts.wrapper_mint.key, keys.wrapper_mint),
        (*accounts.wrapper_underlying_tokens.key, keys.wrapper_underlying_tokens),
        (*accounts.owner.key, keys.owner),
        (*accounts.user_underlying_tokens.key, keys.user_underlying_tokens),
        (*accounts.user_wrapped_tokens.key, keys.user_wrapped_tokens),
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
        accounts.wrapper_mint,
        accounts.wrapper_underlying_tokens,
        accounts.user_underlying_tokens,
        accounts.user_wrapped_tokens,
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
    for should_be_signer in [accounts.owner] {
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
    pub wrapper: &'me AccountInfo<'info>,
    pub wrapper_mint: &'me AccountInfo<'info>,
    pub wrapper_underlying_tokens: &'me AccountInfo<'info>,
    pub owner: &'me AccountInfo<'info>,
    pub user_underlying_tokens: &'me AccountInfo<'info>,
    pub user_wrapped_tokens: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct WithdrawKeys {
    pub wrapper: Pubkey,
    pub wrapper_mint: Pubkey,
    pub wrapper_underlying_tokens: Pubkey,
    pub owner: Pubkey,
    pub user_underlying_tokens: Pubkey,
    pub user_wrapped_tokens: Pubkey,
    pub token_program: Pubkey,
}
impl From<WithdrawAccounts<'_, '_>> for WithdrawKeys {
    fn from(accounts: WithdrawAccounts) -> Self {
        Self {
            wrapper: *accounts.wrapper.key,
            wrapper_mint: *accounts.wrapper_mint.key,
            wrapper_underlying_tokens: *accounts.wrapper_underlying_tokens.key,
            owner: *accounts.owner.key,
            user_underlying_tokens: *accounts.user_underlying_tokens.key,
            user_wrapped_tokens: *accounts.user_wrapped_tokens.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<WithdrawKeys> for [AccountMeta; WITHDRAW_IX_ACCOUNTS_LEN] {
    fn from(keys: WithdrawKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.wrapper,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.wrapper_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.wrapper_underlying_tokens,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.owner,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.user_underlying_tokens,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_wrapped_tokens,
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
            wrapper: pubkeys[0],
            wrapper_mint: pubkeys[1],
            wrapper_underlying_tokens: pubkeys[2],
            owner: pubkeys[3],
            user_underlying_tokens: pubkeys[4],
            user_wrapped_tokens: pubkeys[5],
            token_program: pubkeys[6],
        }
    }
}
impl<'info> From<WithdrawAccounts<'_, 'info>>
for [AccountInfo<'info>; WITHDRAW_IX_ACCOUNTS_LEN] {
    fn from(accounts: WithdrawAccounts<'_, 'info>) -> Self {
        [
            accounts.wrapper.clone(),
            accounts.wrapper_mint.clone(),
            accounts.wrapper_underlying_tokens.clone(),
            accounts.owner.clone(),
            accounts.user_underlying_tokens.clone(),
            accounts.user_wrapped_tokens.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; WITHDRAW_IX_ACCOUNTS_LEN]>
for WithdrawAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; WITHDRAW_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            wrapper: &arr[0],
            wrapper_mint: &arr[1],
            wrapper_underlying_tokens: &arr[2],
            owner: &arr[3],
            user_underlying_tokens: &arr[4],
            user_wrapped_tokens: &arr[5],
            token_program: &arr[6],
        }
    }
}
pub const WITHDRAW_IX_DISCM: [u8; 8usize] = [183, 18, 70, 156, 148, 109, 161, 34];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WithdrawIxArgs {
    pub max_burn_amount: u64,
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
        let max_burn_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(Self(WithdrawIxArgs { max_burn_amount }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&WITHDRAW_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.max_burn_amount, &mut writer)?;
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
    withdraw_ix_with_program_id(ADD_DECIMALS_PROGRAM_ID, keys, args)
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
    withdraw_invoke_with_program_id(ADD_DECIMALS_PROGRAM_ID, accounts, args)
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
        ADD_DECIMALS_PROGRAM_ID,
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
        (*accounts.wrapper.key, keys.wrapper),
        (*accounts.wrapper_mint.key, keys.wrapper_mint),
        (*accounts.wrapper_underlying_tokens.key, keys.wrapper_underlying_tokens),
        (*accounts.owner.key, keys.owner),
        (*accounts.user_underlying_tokens.key, keys.user_underlying_tokens),
        (*accounts.user_wrapped_tokens.key, keys.user_wrapped_tokens),
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
        accounts.wrapper_mint,
        accounts.wrapper_underlying_tokens,
        accounts.user_underlying_tokens,
        accounts.user_wrapped_tokens,
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
    for should_be_signer in [accounts.owner] {
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
