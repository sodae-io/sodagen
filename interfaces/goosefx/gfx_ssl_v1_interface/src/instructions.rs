use solana_pubkey::Pubkey;
use solana_cpi::{invoke, invoke_signed};
use solana_instruction::{AccountMeta, Instruction};
use solana_account_info::AccountInfo;
use solana_program_error::ProgramError;
use std::io::Read;
#[allow(unused_imports)]
use crate::*;
#[derive(Clone, Debug, PartialEq)]
pub enum GfxSslV1ProgramIx {
    CreateLiquidityAccount,
    Deposit(DepositIxArgs),
    Withdraw(WithdrawIxArgs),
    MintPt(MintPtIxArgs),
    BurnPt(BurnPtIxArgs),
    Swap(SwapIxArgs),
}
impl GfxSslV1ProgramIx {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        if buf.starts_with(&CREATE_LIQUIDITY_ACCOUNT_IX_DISCM) {
            return Ok(Self::CreateLiquidityAccount);
        }
        if buf.starts_with(&DEPOSIT_IX_DISCM) {
            let mut reader = &buf[DEPOSIT_IX_DISCM.len()..];
            let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(Self::Deposit(DepositIxArgs { amount }));
        }
        if buf.starts_with(&WITHDRAW_IX_DISCM) {
            let mut reader = &buf[WITHDRAW_IX_DISCM.len()..];
            let withdraw_percent: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(Self::Withdraw(WithdrawIxArgs { withdraw_percent }));
        }
        if buf.starts_with(&MINT_PT_IX_DISCM) {
            let mut reader = &buf[MINT_PT_IX_DISCM.len()..];
            let amount_to_mint: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(Self::MintPt(MintPtIxArgs { amount_to_mint }));
        }
        if buf.starts_with(&BURN_PT_IX_DISCM) {
            let mut reader = &buf[BURN_PT_IX_DISCM.len()..];
            let amount_to_burn: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(Self::BurnPt(BurnPtIxArgs { amount_to_burn }));
        }
        if buf.starts_with(&SWAP_IX_DISCM) {
            let mut reader = &buf[SWAP_IX_DISCM.len()..];
            let amount_in: u64 = crate::borsh_de_or_default(&mut reader)?;
            let min_out: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(Self::Swap(SwapIxArgs { amount_in, min_out }));
        }
        Err(std::io::Error::from(std::io::ErrorKind::InvalidData))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        match self {
            Self::CreateLiquidityAccount => {
                writer.write_all(&CREATE_LIQUIDITY_ACCOUNT_IX_DISCM)
            }
            Self::Deposit(args) => {
                writer.write_all(&DEPOSIT_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.amount, &mut writer)?;
                Ok(())
            }
            Self::Withdraw(args) => {
                writer.write_all(&WITHDRAW_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.withdraw_percent, &mut writer)?;
                Ok(())
            }
            Self::MintPt(args) => {
                writer.write_all(&MINT_PT_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.amount_to_mint, &mut writer)?;
                Ok(())
            }
            Self::BurnPt(args) => {
                writer.write_all(&BURN_PT_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.amount_to_burn, &mut writer)?;
                Ok(())
            }
            Self::Swap(args) => {
                writer.write_all(&SWAP_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.amount_in, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.min_out, &mut writer)?;
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
pub const CREATE_LIQUIDITY_ACCOUNT_IX_ACCOUNTS_LEN: usize = 6;
#[derive(Copy, Clone, Debug)]
pub struct CreateLiquidityAccountAccounts<'me, 'info> {
    pub controller: &'me AccountInfo<'info>,
    pub ssl: &'me AccountInfo<'info>,
    pub liquidity_account: &'me AccountInfo<'info>,
    pub user_wallet: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub rent: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CreateLiquidityAccountKeys {
    pub controller: Pubkey,
    pub ssl: Pubkey,
    pub liquidity_account: Pubkey,
    pub user_wallet: Pubkey,
    pub system_program: Pubkey,
    pub rent: Pubkey,
}
impl From<CreateLiquidityAccountAccounts<'_, '_>> for CreateLiquidityAccountKeys {
    fn from(accounts: CreateLiquidityAccountAccounts) -> Self {
        Self {
            controller: *accounts.controller.key,
            ssl: *accounts.ssl.key,
            liquidity_account: *accounts.liquidity_account.key,
            user_wallet: *accounts.user_wallet.key,
            system_program: *accounts.system_program.key,
            rent: *accounts.rent.key,
        }
    }
}
impl From<CreateLiquidityAccountKeys>
for [AccountMeta; CREATE_LIQUIDITY_ACCOUNT_IX_ACCOUNTS_LEN] {
    fn from(keys: CreateLiquidityAccountKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.controller,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.ssl,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.liquidity_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_wallet,
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
impl From<[Pubkey; CREATE_LIQUIDITY_ACCOUNT_IX_ACCOUNTS_LEN]>
for CreateLiquidityAccountKeys {
    fn from(pubkeys: [Pubkey; CREATE_LIQUIDITY_ACCOUNT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            controller: pubkeys[0],
            ssl: pubkeys[1],
            liquidity_account: pubkeys[2],
            user_wallet: pubkeys[3],
            system_program: pubkeys[4],
            rent: pubkeys[5],
        }
    }
}
impl<'info> From<CreateLiquidityAccountAccounts<'_, 'info>>
for [AccountInfo<'info>; CREATE_LIQUIDITY_ACCOUNT_IX_ACCOUNTS_LEN] {
    fn from(accounts: CreateLiquidityAccountAccounts<'_, 'info>) -> Self {
        [
            accounts.controller.clone(),
            accounts.ssl.clone(),
            accounts.liquidity_account.clone(),
            accounts.user_wallet.clone(),
            accounts.system_program.clone(),
            accounts.rent.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; CREATE_LIQUIDITY_ACCOUNT_IX_ACCOUNTS_LEN]>
for CreateLiquidityAccountAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; CREATE_LIQUIDITY_ACCOUNT_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            controller: &arr[0],
            ssl: &arr[1],
            liquidity_account: &arr[2],
            user_wallet: &arr[3],
            system_program: &arr[4],
            rent: &arr[5],
        }
    }
}
pub const CREATE_LIQUIDITY_ACCOUNT_IX_DISCM: [u8; 8usize] = [
    112, 19, 213, 238, 68, 113, 146, 38,
];
#[derive(Clone, Debug, PartialEq)]
pub struct CreateLiquidityAccountIxData;
impl CreateLiquidityAccountIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CREATE_LIQUIDITY_ACCOUNT_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CREATE_LIQUIDITY_ACCOUNT_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn create_liquidity_account_ix_with_program_id(
    program_id: Pubkey,
    keys: CreateLiquidityAccountKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CREATE_LIQUIDITY_ACCOUNT_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: CreateLiquidityAccountIxData.try_to_vec()?,
    })
}
pub fn create_liquidity_account_ix(
    keys: CreateLiquidityAccountKeys,
) -> std::io::Result<Instruction> {
    create_liquidity_account_ix_with_program_id(GFX_SSL_V1_PROGRAM_ID, keys)
}
pub fn create_liquidity_account_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CreateLiquidityAccountAccounts<'_, '_>,
) -> ProgramResult {
    let keys: CreateLiquidityAccountKeys = accounts.into();
    let ix = create_liquidity_account_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn create_liquidity_account_invoke(
    accounts: CreateLiquidityAccountAccounts<'_, '_>,
) -> ProgramResult {
    create_liquidity_account_invoke_with_program_id(GFX_SSL_V1_PROGRAM_ID, accounts)
}
pub fn create_liquidity_account_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CreateLiquidityAccountAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CreateLiquidityAccountKeys = accounts.into();
    let ix = create_liquidity_account_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn create_liquidity_account_invoke_signed(
    accounts: CreateLiquidityAccountAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    create_liquidity_account_invoke_signed_with_program_id(
        GFX_SSL_V1_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn create_liquidity_account_verify_account_keys(
    accounts: CreateLiquidityAccountAccounts<'_, '_>,
    keys: CreateLiquidityAccountKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.controller.key, keys.controller),
        (*accounts.ssl.key, keys.ssl),
        (*accounts.liquidity_account.key, keys.liquidity_account),
        (*accounts.user_wallet.key, keys.user_wallet),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.rent.key, keys.rent),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn create_liquidity_account_verify_writable_privileges<'me, 'info>(
    accounts: CreateLiquidityAccountAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.ssl,
        accounts.liquidity_account,
        accounts.user_wallet,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn create_liquidity_account_verify_signer_privileges<'me, 'info>(
    accounts: CreateLiquidityAccountAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.user_wallet] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn create_liquidity_account_verify_account_privileges<'me, 'info>(
    accounts: CreateLiquidityAccountAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    create_liquidity_account_verify_writable_privileges(accounts)?;
    create_liquidity_account_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const DEPOSIT_IX_ACCOUNTS_LEN: usize = 7;
#[derive(Copy, Clone, Debug)]
pub struct DepositAccounts<'me, 'info> {
    pub controller: &'me AccountInfo<'info>,
    pub ssl: &'me AccountInfo<'info>,
    pub liquidity_account: &'me AccountInfo<'info>,
    pub rt_vault: &'me AccountInfo<'info>,
    pub user_rt_ata: &'me AccountInfo<'info>,
    pub user_wallet: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct DepositKeys {
    pub controller: Pubkey,
    pub ssl: Pubkey,
    pub liquidity_account: Pubkey,
    pub rt_vault: Pubkey,
    pub user_rt_ata: Pubkey,
    pub user_wallet: Pubkey,
    pub token_program: Pubkey,
}
impl From<DepositAccounts<'_, '_>> for DepositKeys {
    fn from(accounts: DepositAccounts) -> Self {
        Self {
            controller: *accounts.controller.key,
            ssl: *accounts.ssl.key,
            liquidity_account: *accounts.liquidity_account.key,
            rt_vault: *accounts.rt_vault.key,
            user_rt_ata: *accounts.user_rt_ata.key,
            user_wallet: *accounts.user_wallet.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<DepositKeys> for [AccountMeta; DEPOSIT_IX_ACCOUNTS_LEN] {
    fn from(keys: DepositKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.controller,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.ssl,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.liquidity_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.rt_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_rt_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_wallet,
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
            controller: pubkeys[0],
            ssl: pubkeys[1],
            liquidity_account: pubkeys[2],
            rt_vault: pubkeys[3],
            user_rt_ata: pubkeys[4],
            user_wallet: pubkeys[5],
            token_program: pubkeys[6],
        }
    }
}
impl<'info> From<DepositAccounts<'_, 'info>>
for [AccountInfo<'info>; DEPOSIT_IX_ACCOUNTS_LEN] {
    fn from(accounts: DepositAccounts<'_, 'info>) -> Self {
        [
            accounts.controller.clone(),
            accounts.ssl.clone(),
            accounts.liquidity_account.clone(),
            accounts.rt_vault.clone(),
            accounts.user_rt_ata.clone(),
            accounts.user_wallet.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; DEPOSIT_IX_ACCOUNTS_LEN]>
for DepositAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; DEPOSIT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            controller: &arr[0],
            ssl: &arr[1],
            liquidity_account: &arr[2],
            rt_vault: &arr[3],
            user_rt_ata: &arr[4],
            user_wallet: &arr[5],
            token_program: &arr[6],
        }
    }
}
pub const DEPOSIT_IX_DISCM: [u8; 8usize] = [242, 35, 198, 137, 82, 225, 242, 182];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DepositIxArgs {
    pub amount: u64,
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
        let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
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
    deposit_ix_with_program_id(GFX_SSL_V1_PROGRAM_ID, keys, args)
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
    deposit_invoke_with_program_id(GFX_SSL_V1_PROGRAM_ID, accounts, args)
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
    deposit_invoke_signed_with_program_id(GFX_SSL_V1_PROGRAM_ID, accounts, args, seeds)
}
pub fn deposit_verify_account_keys(
    accounts: DepositAccounts<'_, '_>,
    keys: DepositKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.controller.key, keys.controller),
        (*accounts.ssl.key, keys.ssl),
        (*accounts.liquidity_account.key, keys.liquidity_account),
        (*accounts.rt_vault.key, keys.rt_vault),
        (*accounts.user_rt_ata.key, keys.user_rt_ata),
        (*accounts.user_wallet.key, keys.user_wallet),
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
        accounts.ssl,
        accounts.liquidity_account,
        accounts.rt_vault,
        accounts.user_rt_ata,
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
    for should_be_signer in [accounts.user_wallet] {
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
    pub controller: &'me AccountInfo<'info>,
    pub ssl: &'me AccountInfo<'info>,
    pub liquidity_account: &'me AccountInfo<'info>,
    pub rt_vault: &'me AccountInfo<'info>,
    pub user_rt_ata: &'me AccountInfo<'info>,
    pub user_wallet: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct WithdrawKeys {
    pub controller: Pubkey,
    pub ssl: Pubkey,
    pub liquidity_account: Pubkey,
    pub rt_vault: Pubkey,
    pub user_rt_ata: Pubkey,
    pub user_wallet: Pubkey,
    pub token_program: Pubkey,
}
impl From<WithdrawAccounts<'_, '_>> for WithdrawKeys {
    fn from(accounts: WithdrawAccounts) -> Self {
        Self {
            controller: *accounts.controller.key,
            ssl: *accounts.ssl.key,
            liquidity_account: *accounts.liquidity_account.key,
            rt_vault: *accounts.rt_vault.key,
            user_rt_ata: *accounts.user_rt_ata.key,
            user_wallet: *accounts.user_wallet.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<WithdrawKeys> for [AccountMeta; WITHDRAW_IX_ACCOUNTS_LEN] {
    fn from(keys: WithdrawKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.controller,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.ssl,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.liquidity_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.rt_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_rt_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_wallet,
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
            controller: pubkeys[0],
            ssl: pubkeys[1],
            liquidity_account: pubkeys[2],
            rt_vault: pubkeys[3],
            user_rt_ata: pubkeys[4],
            user_wallet: pubkeys[5],
            token_program: pubkeys[6],
        }
    }
}
impl<'info> From<WithdrawAccounts<'_, 'info>>
for [AccountInfo<'info>; WITHDRAW_IX_ACCOUNTS_LEN] {
    fn from(accounts: WithdrawAccounts<'_, 'info>) -> Self {
        [
            accounts.controller.clone(),
            accounts.ssl.clone(),
            accounts.liquidity_account.clone(),
            accounts.rt_vault.clone(),
            accounts.user_rt_ata.clone(),
            accounts.user_wallet.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; WITHDRAW_IX_ACCOUNTS_LEN]>
for WithdrawAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; WITHDRAW_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            controller: &arr[0],
            ssl: &arr[1],
            liquidity_account: &arr[2],
            rt_vault: &arr[3],
            user_rt_ata: &arr[4],
            user_wallet: &arr[5],
            token_program: &arr[6],
        }
    }
}
pub const WITHDRAW_IX_DISCM: [u8; 8usize] = [183, 18, 70, 156, 148, 109, 161, 34];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WithdrawIxArgs {
    pub withdraw_percent: u64,
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
        let withdraw_percent: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(Self(WithdrawIxArgs { withdraw_percent }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&WITHDRAW_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.withdraw_percent, &mut writer)?;
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
    withdraw_ix_with_program_id(GFX_SSL_V1_PROGRAM_ID, keys, args)
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
    withdraw_invoke_with_program_id(GFX_SSL_V1_PROGRAM_ID, accounts, args)
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
    withdraw_invoke_signed_with_program_id(GFX_SSL_V1_PROGRAM_ID, accounts, args, seeds)
}
pub fn withdraw_verify_account_keys(
    accounts: WithdrawAccounts<'_, '_>,
    keys: WithdrawKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.controller.key, keys.controller),
        (*accounts.ssl.key, keys.ssl),
        (*accounts.liquidity_account.key, keys.liquidity_account),
        (*accounts.rt_vault.key, keys.rt_vault),
        (*accounts.user_rt_ata.key, keys.user_rt_ata),
        (*accounts.user_wallet.key, keys.user_wallet),
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
        accounts.ssl,
        accounts.liquidity_account,
        accounts.rt_vault,
        accounts.user_rt_ata,
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
    for should_be_signer in [accounts.user_wallet] {
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
pub const MINT_PT_IX_ACCOUNTS_LEN: usize = 8;
#[derive(Copy, Clone, Debug)]
pub struct MintPtAccounts<'me, 'info> {
    pub controller: &'me AccountInfo<'info>,
    pub ssl: &'me AccountInfo<'info>,
    pub rt_vault: &'me AccountInfo<'info>,
    pub liquidity_account: &'me AccountInfo<'info>,
    pub pt_mint: &'me AccountInfo<'info>,
    pub user_pt_ata: &'me AccountInfo<'info>,
    pub user_wallet: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MintPtKeys {
    pub controller: Pubkey,
    pub ssl: Pubkey,
    pub rt_vault: Pubkey,
    pub liquidity_account: Pubkey,
    pub pt_mint: Pubkey,
    pub user_pt_ata: Pubkey,
    pub user_wallet: Pubkey,
    pub token_program: Pubkey,
}
impl From<MintPtAccounts<'_, '_>> for MintPtKeys {
    fn from(accounts: MintPtAccounts) -> Self {
        Self {
            controller: *accounts.controller.key,
            ssl: *accounts.ssl.key,
            rt_vault: *accounts.rt_vault.key,
            liquidity_account: *accounts.liquidity_account.key,
            pt_mint: *accounts.pt_mint.key,
            user_pt_ata: *accounts.user_pt_ata.key,
            user_wallet: *accounts.user_wallet.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<MintPtKeys> for [AccountMeta; MINT_PT_IX_ACCOUNTS_LEN] {
    fn from(keys: MintPtKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.controller,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.ssl,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.rt_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.liquidity_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pt_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_pt_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_wallet,
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
impl From<[Pubkey; MINT_PT_IX_ACCOUNTS_LEN]> for MintPtKeys {
    fn from(pubkeys: [Pubkey; MINT_PT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            controller: pubkeys[0],
            ssl: pubkeys[1],
            rt_vault: pubkeys[2],
            liquidity_account: pubkeys[3],
            pt_mint: pubkeys[4],
            user_pt_ata: pubkeys[5],
            user_wallet: pubkeys[6],
            token_program: pubkeys[7],
        }
    }
}
impl<'info> From<MintPtAccounts<'_, 'info>>
for [AccountInfo<'info>; MINT_PT_IX_ACCOUNTS_LEN] {
    fn from(accounts: MintPtAccounts<'_, 'info>) -> Self {
        [
            accounts.controller.clone(),
            accounts.ssl.clone(),
            accounts.rt_vault.clone(),
            accounts.liquidity_account.clone(),
            accounts.pt_mint.clone(),
            accounts.user_pt_ata.clone(),
            accounts.user_wallet.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; MINT_PT_IX_ACCOUNTS_LEN]>
for MintPtAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; MINT_PT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            controller: &arr[0],
            ssl: &arr[1],
            rt_vault: &arr[2],
            liquidity_account: &arr[3],
            pt_mint: &arr[4],
            user_pt_ata: &arr[5],
            user_wallet: &arr[6],
            token_program: &arr[7],
        }
    }
}
pub const MINT_PT_IX_DISCM: [u8; 8usize] = [115, 247, 213, 190, 222, 163, 95, 229];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MintPtIxArgs {
    pub amount_to_mint: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct MintPtIxData(pub MintPtIxArgs);
impl From<MintPtIxArgs> for MintPtIxData {
    fn from(args: MintPtIxArgs) -> Self {
        Self(args)
    }
}
impl MintPtIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != MINT_PT_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let amount_to_mint: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(Self(MintPtIxArgs { amount_to_mint }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&MINT_PT_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.amount_to_mint, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn mint_pt_ix_with_program_id(
    program_id: Pubkey,
    keys: MintPtKeys,
    args: MintPtIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; MINT_PT_IX_ACCOUNTS_LEN] = keys.into();
    let data: MintPtIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn mint_pt_ix(keys: MintPtKeys, args: MintPtIxArgs) -> std::io::Result<Instruction> {
    mint_pt_ix_with_program_id(GFX_SSL_V1_PROGRAM_ID, keys, args)
}
pub fn mint_pt_invoke_with_program_id(
    program_id: Pubkey,
    accounts: MintPtAccounts<'_, '_>,
    args: MintPtIxArgs,
) -> ProgramResult {
    let keys: MintPtKeys = accounts.into();
    let ix = mint_pt_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn mint_pt_invoke(
    accounts: MintPtAccounts<'_, '_>,
    args: MintPtIxArgs,
) -> ProgramResult {
    mint_pt_invoke_with_program_id(GFX_SSL_V1_PROGRAM_ID, accounts, args)
}
pub fn mint_pt_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: MintPtAccounts<'_, '_>,
    args: MintPtIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: MintPtKeys = accounts.into();
    let ix = mint_pt_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn mint_pt_invoke_signed(
    accounts: MintPtAccounts<'_, '_>,
    args: MintPtIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    mint_pt_invoke_signed_with_program_id(GFX_SSL_V1_PROGRAM_ID, accounts, args, seeds)
}
pub fn mint_pt_verify_account_keys(
    accounts: MintPtAccounts<'_, '_>,
    keys: MintPtKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.controller.key, keys.controller),
        (*accounts.ssl.key, keys.ssl),
        (*accounts.rt_vault.key, keys.rt_vault),
        (*accounts.liquidity_account.key, keys.liquidity_account),
        (*accounts.pt_mint.key, keys.pt_mint),
        (*accounts.user_pt_ata.key, keys.user_pt_ata),
        (*accounts.user_wallet.key, keys.user_wallet),
        (*accounts.token_program.key, keys.token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn mint_pt_verify_writable_privileges<'me, 'info>(
    accounts: MintPtAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.ssl,
        accounts.rt_vault,
        accounts.liquidity_account,
        accounts.pt_mint,
        accounts.user_pt_ata,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn mint_pt_verify_signer_privileges<'me, 'info>(
    accounts: MintPtAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.user_wallet] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn mint_pt_verify_account_privileges<'me, 'info>(
    accounts: MintPtAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    mint_pt_verify_writable_privileges(accounts)?;
    mint_pt_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const BURN_PT_IX_ACCOUNTS_LEN: usize = 7;
#[derive(Copy, Clone, Debug)]
pub struct BurnPtAccounts<'me, 'info> {
    pub controller: &'me AccountInfo<'info>,
    pub ssl: &'me AccountInfo<'info>,
    pub liquidity_account: &'me AccountInfo<'info>,
    pub pt_mint: &'me AccountInfo<'info>,
    pub user_pt_ata: &'me AccountInfo<'info>,
    pub user_wallet: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct BurnPtKeys {
    pub controller: Pubkey,
    pub ssl: Pubkey,
    pub liquidity_account: Pubkey,
    pub pt_mint: Pubkey,
    pub user_pt_ata: Pubkey,
    pub user_wallet: Pubkey,
    pub token_program: Pubkey,
}
impl From<BurnPtAccounts<'_, '_>> for BurnPtKeys {
    fn from(accounts: BurnPtAccounts) -> Self {
        Self {
            controller: *accounts.controller.key,
            ssl: *accounts.ssl.key,
            liquidity_account: *accounts.liquidity_account.key,
            pt_mint: *accounts.pt_mint.key,
            user_pt_ata: *accounts.user_pt_ata.key,
            user_wallet: *accounts.user_wallet.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<BurnPtKeys> for [AccountMeta; BURN_PT_IX_ACCOUNTS_LEN] {
    fn from(keys: BurnPtKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.controller,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.ssl,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.liquidity_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pt_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_pt_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_wallet,
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
impl From<[Pubkey; BURN_PT_IX_ACCOUNTS_LEN]> for BurnPtKeys {
    fn from(pubkeys: [Pubkey; BURN_PT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            controller: pubkeys[0],
            ssl: pubkeys[1],
            liquidity_account: pubkeys[2],
            pt_mint: pubkeys[3],
            user_pt_ata: pubkeys[4],
            user_wallet: pubkeys[5],
            token_program: pubkeys[6],
        }
    }
}
impl<'info> From<BurnPtAccounts<'_, 'info>>
for [AccountInfo<'info>; BURN_PT_IX_ACCOUNTS_LEN] {
    fn from(accounts: BurnPtAccounts<'_, 'info>) -> Self {
        [
            accounts.controller.clone(),
            accounts.ssl.clone(),
            accounts.liquidity_account.clone(),
            accounts.pt_mint.clone(),
            accounts.user_pt_ata.clone(),
            accounts.user_wallet.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; BURN_PT_IX_ACCOUNTS_LEN]>
for BurnPtAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; BURN_PT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            controller: &arr[0],
            ssl: &arr[1],
            liquidity_account: &arr[2],
            pt_mint: &arr[3],
            user_pt_ata: &arr[4],
            user_wallet: &arr[5],
            token_program: &arr[6],
        }
    }
}
pub const BURN_PT_IX_DISCM: [u8; 8usize] = [72, 94, 156, 201, 183, 224, 52, 100];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct BurnPtIxArgs {
    pub amount_to_burn: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct BurnPtIxData(pub BurnPtIxArgs);
impl From<BurnPtIxArgs> for BurnPtIxData {
    fn from(args: BurnPtIxArgs) -> Self {
        Self(args)
    }
}
impl BurnPtIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != BURN_PT_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let amount_to_burn: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(Self(BurnPtIxArgs { amount_to_burn }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&BURN_PT_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.amount_to_burn, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn burn_pt_ix_with_program_id(
    program_id: Pubkey,
    keys: BurnPtKeys,
    args: BurnPtIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; BURN_PT_IX_ACCOUNTS_LEN] = keys.into();
    let data: BurnPtIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn burn_pt_ix(keys: BurnPtKeys, args: BurnPtIxArgs) -> std::io::Result<Instruction> {
    burn_pt_ix_with_program_id(GFX_SSL_V1_PROGRAM_ID, keys, args)
}
pub fn burn_pt_invoke_with_program_id(
    program_id: Pubkey,
    accounts: BurnPtAccounts<'_, '_>,
    args: BurnPtIxArgs,
) -> ProgramResult {
    let keys: BurnPtKeys = accounts.into();
    let ix = burn_pt_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn burn_pt_invoke(
    accounts: BurnPtAccounts<'_, '_>,
    args: BurnPtIxArgs,
) -> ProgramResult {
    burn_pt_invoke_with_program_id(GFX_SSL_V1_PROGRAM_ID, accounts, args)
}
pub fn burn_pt_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: BurnPtAccounts<'_, '_>,
    args: BurnPtIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: BurnPtKeys = accounts.into();
    let ix = burn_pt_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn burn_pt_invoke_signed(
    accounts: BurnPtAccounts<'_, '_>,
    args: BurnPtIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    burn_pt_invoke_signed_with_program_id(GFX_SSL_V1_PROGRAM_ID, accounts, args, seeds)
}
pub fn burn_pt_verify_account_keys(
    accounts: BurnPtAccounts<'_, '_>,
    keys: BurnPtKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.controller.key, keys.controller),
        (*accounts.ssl.key, keys.ssl),
        (*accounts.liquidity_account.key, keys.liquidity_account),
        (*accounts.pt_mint.key, keys.pt_mint),
        (*accounts.user_pt_ata.key, keys.user_pt_ata),
        (*accounts.user_wallet.key, keys.user_wallet),
        (*accounts.token_program.key, keys.token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn burn_pt_verify_writable_privileges<'me, 'info>(
    accounts: BurnPtAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.ssl,
        accounts.liquidity_account,
        accounts.pt_mint,
        accounts.user_pt_ata,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn burn_pt_verify_signer_privileges<'me, 'info>(
    accounts: BurnPtAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.user_wallet] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn burn_pt_verify_account_privileges<'me, 'info>(
    accounts: BurnPtAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    burn_pt_verify_writable_privileges(accounts)?;
    burn_pt_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SWAP_IX_ACCOUNTS_LEN: usize = 14;
#[derive(Copy, Clone, Debug)]
pub struct SwapAccounts<'me, 'info> {
    pub controller: &'me AccountInfo<'info>,
    pub pair: &'me AccountInfo<'info>,
    pub ssl_in: &'me AccountInfo<'info>,
    pub ssl_out: &'me AccountInfo<'info>,
    pub liability_vault_in: &'me AccountInfo<'info>,
    pub swapped_liability_vault_in: &'me AccountInfo<'info>,
    pub liability_vault_out: &'me AccountInfo<'info>,
    pub swapped_liability_vault_out: &'me AccountInfo<'info>,
    pub user_in_ata: &'me AccountInfo<'info>,
    pub user_out_ata: &'me AccountInfo<'info>,
    pub fee_collector_ata: &'me AccountInfo<'info>,
    pub user_wallet: &'me AccountInfo<'info>,
    pub fee_collector: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SwapKeys {
    pub controller: Pubkey,
    pub pair: Pubkey,
    pub ssl_in: Pubkey,
    pub ssl_out: Pubkey,
    pub liability_vault_in: Pubkey,
    pub swapped_liability_vault_in: Pubkey,
    pub liability_vault_out: Pubkey,
    pub swapped_liability_vault_out: Pubkey,
    pub user_in_ata: Pubkey,
    pub user_out_ata: Pubkey,
    pub fee_collector_ata: Pubkey,
    pub user_wallet: Pubkey,
    pub fee_collector: Pubkey,
    pub token_program: Pubkey,
}
impl From<SwapAccounts<'_, '_>> for SwapKeys {
    fn from(accounts: SwapAccounts) -> Self {
        Self {
            controller: *accounts.controller.key,
            pair: *accounts.pair.key,
            ssl_in: *accounts.ssl_in.key,
            ssl_out: *accounts.ssl_out.key,
            liability_vault_in: *accounts.liability_vault_in.key,
            swapped_liability_vault_in: *accounts.swapped_liability_vault_in.key,
            liability_vault_out: *accounts.liability_vault_out.key,
            swapped_liability_vault_out: *accounts.swapped_liability_vault_out.key,
            user_in_ata: *accounts.user_in_ata.key,
            user_out_ata: *accounts.user_out_ata.key,
            fee_collector_ata: *accounts.fee_collector_ata.key,
            user_wallet: *accounts.user_wallet.key,
            fee_collector: *accounts.fee_collector.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<SwapKeys> for [AccountMeta; SWAP_IX_ACCOUNTS_LEN] {
    fn from(keys: SwapKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.controller,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pair,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.ssl_in,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.ssl_out,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.liability_vault_in,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.swapped_liability_vault_in,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.liability_vault_out,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.swapped_liability_vault_out,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_in_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_out_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.fee_collector_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_wallet,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.fee_collector,
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
impl From<[Pubkey; SWAP_IX_ACCOUNTS_LEN]> for SwapKeys {
    fn from(pubkeys: [Pubkey; SWAP_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            controller: pubkeys[0],
            pair: pubkeys[1],
            ssl_in: pubkeys[2],
            ssl_out: pubkeys[3],
            liability_vault_in: pubkeys[4],
            swapped_liability_vault_in: pubkeys[5],
            liability_vault_out: pubkeys[6],
            swapped_liability_vault_out: pubkeys[7],
            user_in_ata: pubkeys[8],
            user_out_ata: pubkeys[9],
            fee_collector_ata: pubkeys[10],
            user_wallet: pubkeys[11],
            fee_collector: pubkeys[12],
            token_program: pubkeys[13],
        }
    }
}
impl<'info> From<SwapAccounts<'_, 'info>>
for [AccountInfo<'info>; SWAP_IX_ACCOUNTS_LEN] {
    fn from(accounts: SwapAccounts<'_, 'info>) -> Self {
        [
            accounts.controller.clone(),
            accounts.pair.clone(),
            accounts.ssl_in.clone(),
            accounts.ssl_out.clone(),
            accounts.liability_vault_in.clone(),
            accounts.swapped_liability_vault_in.clone(),
            accounts.liability_vault_out.clone(),
            accounts.swapped_liability_vault_out.clone(),
            accounts.user_in_ata.clone(),
            accounts.user_out_ata.clone(),
            accounts.fee_collector_ata.clone(),
            accounts.user_wallet.clone(),
            accounts.fee_collector.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SWAP_IX_ACCOUNTS_LEN]>
for SwapAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; SWAP_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            controller: &arr[0],
            pair: &arr[1],
            ssl_in: &arr[2],
            ssl_out: &arr[3],
            liability_vault_in: &arr[4],
            swapped_liability_vault_in: &arr[5],
            liability_vault_out: &arr[6],
            swapped_liability_vault_out: &arr[7],
            user_in_ata: &arr[8],
            user_out_ata: &arr[9],
            fee_collector_ata: &arr[10],
            user_wallet: &arr[11],
            fee_collector: &arr[12],
            token_program: &arr[13],
        }
    }
}
pub const SWAP_IX_DISCM: [u8; 8usize] = [248, 198, 158, 145, 225, 117, 135, 200];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SwapIxArgs {
    pub amount_in: u64,
    pub min_out: u64,
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
        let amount_in: u64 = crate::borsh_de_or_default(&mut reader)?;
        let min_out: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(Self(SwapIxArgs { amount_in, min_out }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SWAP_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.amount_in, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.min_out, &mut writer)?;
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
    swap_ix_with_program_id(GFX_SSL_V1_PROGRAM_ID, keys, args)
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
    swap_invoke_with_program_id(GFX_SSL_V1_PROGRAM_ID, accounts, args)
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
    swap_invoke_signed_with_program_id(GFX_SSL_V1_PROGRAM_ID, accounts, args, seeds)
}
pub fn swap_verify_account_keys(
    accounts: SwapAccounts<'_, '_>,
    keys: SwapKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.controller.key, keys.controller),
        (*accounts.pair.key, keys.pair),
        (*accounts.ssl_in.key, keys.ssl_in),
        (*accounts.ssl_out.key, keys.ssl_out),
        (*accounts.liability_vault_in.key, keys.liability_vault_in),
        (*accounts.swapped_liability_vault_in.key, keys.swapped_liability_vault_in),
        (*accounts.liability_vault_out.key, keys.liability_vault_out),
        (*accounts.swapped_liability_vault_out.key, keys.swapped_liability_vault_out),
        (*accounts.user_in_ata.key, keys.user_in_ata),
        (*accounts.user_out_ata.key, keys.user_out_ata),
        (*accounts.fee_collector_ata.key, keys.fee_collector_ata),
        (*accounts.user_wallet.key, keys.user_wallet),
        (*accounts.fee_collector.key, keys.fee_collector),
        (*accounts.token_program.key, keys.token_program),
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
        accounts.pair,
        accounts.ssl_in,
        accounts.ssl_out,
        accounts.liability_vault_in,
        accounts.swapped_liability_vault_in,
        accounts.liability_vault_out,
        accounts.swapped_liability_vault_out,
        accounts.user_in_ata,
        accounts.user_out_ata,
        accounts.fee_collector_ata,
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
    for should_be_signer in [accounts.user_wallet] {
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
