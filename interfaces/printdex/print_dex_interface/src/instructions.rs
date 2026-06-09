use solana_pubkey::Pubkey;
use solana_cpi::{invoke, invoke_signed};
use solana_instruction::{AccountMeta, Instruction};
use solana_account_info::AccountInfo;
use solana_program_error::ProgramError;
use std::io::Read;
#[allow(unused_imports)]
use crate::*;
#[derive(Clone, Debug, PartialEq)]
pub enum PrintDexProgramIx {
    InitPlatform,
    CreatePoolAccounts,
    CreatePool(CreatePoolIxArgs),
    AddLiquidity(AddLiquidityIxArgs),
    RemoveLiquidity(RemoveLiquidityIxArgs),
    Swap(SwapIxArgs),
    CollectPlatformFees,
}
impl PrintDexProgramIx {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        if buf.starts_with(&INIT_PLATFORM_IX_DISCM) {
            return Ok(Self::InitPlatform);
        }
        if buf.starts_with(&CREATE_POOL_ACCOUNTS_IX_DISCM) {
            return Ok(Self::CreatePoolAccounts);
        }
        if buf.starts_with(&CREATE_POOL_IX_DISCM) {
            let mut reader = &buf[CREATE_POOL_IX_DISCM.len()..];
            let amount_a: u64 = crate::borsh_de_or_default(&mut reader)?;
            let amount_b: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::CreatePool(CreatePoolIxArgs {
                    amount_a,
                    amount_b,
                }),
            );
        }
        if buf.starts_with(&ADD_LIQUIDITY_IX_DISCM) {
            let mut reader = &buf[ADD_LIQUIDITY_IX_DISCM.len()..];
            let amount_a: u64 = crate::borsh_de_or_default(&mut reader)?;
            let amount_b: u64 = crate::borsh_de_or_default(&mut reader)?;
            let slippage: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::AddLiquidity(AddLiquidityIxArgs {
                    amount_a,
                    amount_b,
                    slippage,
                }),
            );
        }
        if buf.starts_with(&REMOVE_LIQUIDITY_IX_DISCM) {
            let mut reader = &buf[REMOVE_LIQUIDITY_IX_DISCM.len()..];
            let liquidity: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(Self::RemoveLiquidity(RemoveLiquidityIxArgs { liquidity }));
        }
        if buf.starts_with(&SWAP_IX_DISCM) {
            let mut reader = &buf[SWAP_IX_DISCM.len()..];
            let amount_a_in: u64 = crate::borsh_de_or_default(&mut reader)?;
            let expected_amount_b_out: u64 = crate::borsh_de_or_default(&mut reader)?;
            let slippage: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::Swap(SwapIxArgs {
                    amount_a_in,
                    expected_amount_b_out,
                    slippage,
                }),
            );
        }
        if buf.starts_with(&COLLECT_PLATFORM_FEES_IX_DISCM) {
            return Ok(Self::CollectPlatformFees);
        }
        Err(std::io::Error::from(std::io::ErrorKind::InvalidData))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        match self {
            Self::InitPlatform => writer.write_all(&INIT_PLATFORM_IX_DISCM),
            Self::CreatePoolAccounts => writer.write_all(&CREATE_POOL_ACCOUNTS_IX_DISCM),
            Self::CreatePool(args) => {
                writer.write_all(&CREATE_POOL_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.amount_a, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.amount_b, &mut writer)?;
                Ok(())
            }
            Self::AddLiquidity(args) => {
                writer.write_all(&ADD_LIQUIDITY_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.amount_a, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.amount_b, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.slippage, &mut writer)?;
                Ok(())
            }
            Self::RemoveLiquidity(args) => {
                writer.write_all(&REMOVE_LIQUIDITY_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.liquidity, &mut writer)?;
                Ok(())
            }
            Self::Swap(args) => {
                writer.write_all(&SWAP_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.amount_a_in, &mut writer)?;
                borsh::BorshSerialize::serialize(
                    &args.expected_amount_b_out,
                    &mut writer,
                )?;
                borsh::BorshSerialize::serialize(&args.slippage, &mut writer)?;
                Ok(())
            }
            Self::CollectPlatformFees => {
                writer.write_all(&COLLECT_PLATFORM_FEES_IX_DISCM)
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
pub const INIT_PLATFORM_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct InitPlatformAccounts<'me, 'info> {
    pub config: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct InitPlatformKeys {
    pub config: Pubkey,
    pub authority: Pubkey,
    pub system_program: Pubkey,
}
impl From<InitPlatformAccounts<'_, '_>> for InitPlatformKeys {
    fn from(accounts: InitPlatformAccounts) -> Self {
        Self {
            config: *accounts.config.key,
            authority: *accounts.authority.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<InitPlatformKeys> for [AccountMeta; INIT_PLATFORM_IX_ACCOUNTS_LEN] {
    fn from(keys: InitPlatformKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.config,
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
impl From<[Pubkey; INIT_PLATFORM_IX_ACCOUNTS_LEN]> for InitPlatformKeys {
    fn from(pubkeys: [Pubkey; INIT_PLATFORM_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            config: pubkeys[0],
            authority: pubkeys[1],
            system_program: pubkeys[2],
        }
    }
}
impl<'info> From<InitPlatformAccounts<'_, 'info>>
for [AccountInfo<'info>; INIT_PLATFORM_IX_ACCOUNTS_LEN] {
    fn from(accounts: InitPlatformAccounts<'_, 'info>) -> Self {
        [
            accounts.config.clone(),
            accounts.authority.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; INIT_PLATFORM_IX_ACCOUNTS_LEN]>
for InitPlatformAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; INIT_PLATFORM_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            config: &arr[0],
            authority: &arr[1],
            system_program: &arr[2],
        }
    }
}
pub const INIT_PLATFORM_IX_DISCM: [u8; 8usize] = [29, 22, 210, 225, 219, 114, 193, 169];
#[derive(Clone, Debug, PartialEq)]
pub struct InitPlatformIxData;
impl InitPlatformIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INIT_PLATFORM_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INIT_PLATFORM_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn init_platform_ix_with_program_id(
    program_id: Pubkey,
    keys: InitPlatformKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; INIT_PLATFORM_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: InitPlatformIxData.try_to_vec()?,
    })
}
pub fn init_platform_ix(keys: InitPlatformKeys) -> std::io::Result<Instruction> {
    init_platform_ix_with_program_id(PRINT_DEX_PROGRAM_ID, keys)
}
pub fn init_platform_invoke_with_program_id(
    program_id: Pubkey,
    accounts: InitPlatformAccounts<'_, '_>,
) -> ProgramResult {
    let keys: InitPlatformKeys = accounts.into();
    let ix = init_platform_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn init_platform_invoke(accounts: InitPlatformAccounts<'_, '_>) -> ProgramResult {
    init_platform_invoke_with_program_id(PRINT_DEX_PROGRAM_ID, accounts)
}
pub fn init_platform_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: InitPlatformAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: InitPlatformKeys = accounts.into();
    let ix = init_platform_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn init_platform_invoke_signed(
    accounts: InitPlatformAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    init_platform_invoke_signed_with_program_id(PRINT_DEX_PROGRAM_ID, accounts, seeds)
}
pub fn init_platform_verify_account_keys(
    accounts: InitPlatformAccounts<'_, '_>,
    keys: InitPlatformKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.config.key, keys.config),
        (*accounts.authority.key, keys.authority),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn init_platform_verify_writable_privileges<'me, 'info>(
    accounts: InitPlatformAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.config, accounts.authority] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn init_platform_verify_signer_privileges<'me, 'info>(
    accounts: InitPlatformAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn init_platform_verify_account_privileges<'me, 'info>(
    accounts: InitPlatformAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    init_platform_verify_writable_privileges(accounts)?;
    init_platform_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CREATE_POOL_ACCOUNTS_IX_ACCOUNTS_LEN: usize = 11;
#[derive(Copy, Clone, Debug)]
pub struct CreatePoolAccountsAccounts<'me, 'info> {
    pub config: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub pool: &'me AccountInfo<'info>,
    pub vault: &'me AccountInfo<'info>,
    pub mint_a: &'me AccountInfo<'info>,
    pub mint_b: &'me AccountInfo<'info>,
    pub liquidity_token: &'me AccountInfo<'info>,
    pub authority_liquidity_token_account: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CreatePoolAccountsKeys {
    pub config: Pubkey,
    pub authority: Pubkey,
    pub pool: Pubkey,
    pub vault: Pubkey,
    pub mint_a: Pubkey,
    pub mint_b: Pubkey,
    pub liquidity_token: Pubkey,
    pub authority_liquidity_token_account: Pubkey,
    pub token_program: Pubkey,
    pub associated_token_program: Pubkey,
    pub system_program: Pubkey,
}
impl From<CreatePoolAccountsAccounts<'_, '_>> for CreatePoolAccountsKeys {
    fn from(accounts: CreatePoolAccountsAccounts) -> Self {
        Self {
            config: *accounts.config.key,
            authority: *accounts.authority.key,
            pool: *accounts.pool.key,
            vault: *accounts.vault.key,
            mint_a: *accounts.mint_a.key,
            mint_b: *accounts.mint_b.key,
            liquidity_token: *accounts.liquidity_token.key,
            authority_liquidity_token_account: *accounts
                .authority_liquidity_token_account
                .key,
            token_program: *accounts.token_program.key,
            associated_token_program: *accounts.associated_token_program.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<CreatePoolAccountsKeys>
for [AccountMeta; CREATE_POOL_ACCOUNTS_IX_ACCOUNTS_LEN] {
    fn from(keys: CreatePoolAccountsKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.config,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.authority,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.mint_a,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.mint_b,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.liquidity_token,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.authority_liquidity_token_account,
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
        ]
    }
}
impl From<[Pubkey; CREATE_POOL_ACCOUNTS_IX_ACCOUNTS_LEN]> for CreatePoolAccountsKeys {
    fn from(pubkeys: [Pubkey; CREATE_POOL_ACCOUNTS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            config: pubkeys[0],
            authority: pubkeys[1],
            pool: pubkeys[2],
            vault: pubkeys[3],
            mint_a: pubkeys[4],
            mint_b: pubkeys[5],
            liquidity_token: pubkeys[6],
            authority_liquidity_token_account: pubkeys[7],
            token_program: pubkeys[8],
            associated_token_program: pubkeys[9],
            system_program: pubkeys[10],
        }
    }
}
impl<'info> From<CreatePoolAccountsAccounts<'_, 'info>>
for [AccountInfo<'info>; CREATE_POOL_ACCOUNTS_IX_ACCOUNTS_LEN] {
    fn from(accounts: CreatePoolAccountsAccounts<'_, 'info>) -> Self {
        [
            accounts.config.clone(),
            accounts.authority.clone(),
            accounts.pool.clone(),
            accounts.vault.clone(),
            accounts.mint_a.clone(),
            accounts.mint_b.clone(),
            accounts.liquidity_token.clone(),
            accounts.authority_liquidity_token_account.clone(),
            accounts.token_program.clone(),
            accounts.associated_token_program.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CREATE_POOL_ACCOUNTS_IX_ACCOUNTS_LEN]>
for CreatePoolAccountsAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; CREATE_POOL_ACCOUNTS_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            config: &arr[0],
            authority: &arr[1],
            pool: &arr[2],
            vault: &arr[3],
            mint_a: &arr[4],
            mint_b: &arr[5],
            liquidity_token: &arr[6],
            authority_liquidity_token_account: &arr[7],
            token_program: &arr[8],
            associated_token_program: &arr[9],
            system_program: &arr[10],
        }
    }
}
pub const CREATE_POOL_ACCOUNTS_IX_DISCM: [u8; 8usize] = [
    173, 80, 72, 98, 140, 177, 251, 8,
];
#[derive(Clone, Debug, PartialEq)]
pub struct CreatePoolAccountsIxData;
impl CreatePoolAccountsIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CREATE_POOL_ACCOUNTS_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CREATE_POOL_ACCOUNTS_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn create_pool_accounts_ix_with_program_id(
    program_id: Pubkey,
    keys: CreatePoolAccountsKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CREATE_POOL_ACCOUNTS_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: CreatePoolAccountsIxData.try_to_vec()?,
    })
}
pub fn create_pool_accounts_ix(
    keys: CreatePoolAccountsKeys,
) -> std::io::Result<Instruction> {
    create_pool_accounts_ix_with_program_id(PRINT_DEX_PROGRAM_ID, keys)
}
pub fn create_pool_accounts_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CreatePoolAccountsAccounts<'_, '_>,
) -> ProgramResult {
    let keys: CreatePoolAccountsKeys = accounts.into();
    let ix = create_pool_accounts_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn create_pool_accounts_invoke(
    accounts: CreatePoolAccountsAccounts<'_, '_>,
) -> ProgramResult {
    create_pool_accounts_invoke_with_program_id(PRINT_DEX_PROGRAM_ID, accounts)
}
pub fn create_pool_accounts_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CreatePoolAccountsAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CreatePoolAccountsKeys = accounts.into();
    let ix = create_pool_accounts_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn create_pool_accounts_invoke_signed(
    accounts: CreatePoolAccountsAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    create_pool_accounts_invoke_signed_with_program_id(
        PRINT_DEX_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn create_pool_accounts_verify_account_keys(
    accounts: CreatePoolAccountsAccounts<'_, '_>,
    keys: CreatePoolAccountsKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.config.key, keys.config),
        (*accounts.authority.key, keys.authority),
        (*accounts.pool.key, keys.pool),
        (*accounts.vault.key, keys.vault),
        (*accounts.mint_a.key, keys.mint_a),
        (*accounts.mint_b.key, keys.mint_b),
        (*accounts.liquidity_token.key, keys.liquidity_token),
        (
            *accounts.authority_liquidity_token_account.key,
            keys.authority_liquidity_token_account,
        ),
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
pub fn create_pool_accounts_verify_writable_privileges<'me, 'info>(
    accounts: CreatePoolAccountsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.config,
        accounts.authority,
        accounts.pool,
        accounts.vault,
        accounts.liquidity_token,
        accounts.authority_liquidity_token_account,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn create_pool_accounts_verify_signer_privileges<'me, 'info>(
    accounts: CreatePoolAccountsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn create_pool_accounts_verify_account_privileges<'me, 'info>(
    accounts: CreatePoolAccountsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    create_pool_accounts_verify_writable_privileges(accounts)?;
    create_pool_accounts_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CREATE_POOL_IX_ACCOUNTS_LEN: usize = 19;
#[derive(Copy, Clone, Debug)]
pub struct CreatePoolAccounts<'me, 'info> {
    pub config: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub pool: &'me AccountInfo<'info>,
    pub vault: &'me AccountInfo<'info>,
    pub mint_a: &'me AccountInfo<'info>,
    pub mint_b: &'me AccountInfo<'info>,
    pub authority_token_account_a: &'me AccountInfo<'info>,
    pub authority_token_account_b: &'me AccountInfo<'info>,
    pub vault_token_account_a: &'me AccountInfo<'info>,
    pub vault_token_account_b: &'me AccountInfo<'info>,
    pub liquidity_token: &'me AccountInfo<'info>,
    pub authority_liquidity_token_account: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub token_program_a: &'me AccountInfo<'info>,
    pub token_program_b: &'me AccountInfo<'info>,
    pub hook_program_a: &'me AccountInfo<'info>,
    pub hook_program_b: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CreatePoolKeys {
    pub config: Pubkey,
    pub authority: Pubkey,
    pub pool: Pubkey,
    pub vault: Pubkey,
    pub mint_a: Pubkey,
    pub mint_b: Pubkey,
    pub authority_token_account_a: Pubkey,
    pub authority_token_account_b: Pubkey,
    pub vault_token_account_a: Pubkey,
    pub vault_token_account_b: Pubkey,
    pub liquidity_token: Pubkey,
    pub authority_liquidity_token_account: Pubkey,
    pub token_program: Pubkey,
    pub token_program_a: Pubkey,
    pub token_program_b: Pubkey,
    pub hook_program_a: Pubkey,
    pub hook_program_b: Pubkey,
    pub associated_token_program: Pubkey,
    pub system_program: Pubkey,
}
impl From<CreatePoolAccounts<'_, '_>> for CreatePoolKeys {
    fn from(accounts: CreatePoolAccounts) -> Self {
        Self {
            config: *accounts.config.key,
            authority: *accounts.authority.key,
            pool: *accounts.pool.key,
            vault: *accounts.vault.key,
            mint_a: *accounts.mint_a.key,
            mint_b: *accounts.mint_b.key,
            authority_token_account_a: *accounts.authority_token_account_a.key,
            authority_token_account_b: *accounts.authority_token_account_b.key,
            vault_token_account_a: *accounts.vault_token_account_a.key,
            vault_token_account_b: *accounts.vault_token_account_b.key,
            liquidity_token: *accounts.liquidity_token.key,
            authority_liquidity_token_account: *accounts
                .authority_liquidity_token_account
                .key,
            token_program: *accounts.token_program.key,
            token_program_a: *accounts.token_program_a.key,
            token_program_b: *accounts.token_program_b.key,
            hook_program_a: *accounts.hook_program_a.key,
            hook_program_b: *accounts.hook_program_b.key,
            associated_token_program: *accounts.associated_token_program.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<CreatePoolKeys> for [AccountMeta; CREATE_POOL_IX_ACCOUNTS_LEN] {
    fn from(keys: CreatePoolKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.config,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.authority,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.mint_a,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.mint_b,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.authority_token_account_a,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.authority_token_account_b,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vault_token_account_a,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vault_token_account_b,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.liquidity_token,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.authority_liquidity_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_program_a,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_program_b,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.hook_program_a,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.hook_program_b,
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
impl From<[Pubkey; CREATE_POOL_IX_ACCOUNTS_LEN]> for CreatePoolKeys {
    fn from(pubkeys: [Pubkey; CREATE_POOL_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            config: pubkeys[0],
            authority: pubkeys[1],
            pool: pubkeys[2],
            vault: pubkeys[3],
            mint_a: pubkeys[4],
            mint_b: pubkeys[5],
            authority_token_account_a: pubkeys[6],
            authority_token_account_b: pubkeys[7],
            vault_token_account_a: pubkeys[8],
            vault_token_account_b: pubkeys[9],
            liquidity_token: pubkeys[10],
            authority_liquidity_token_account: pubkeys[11],
            token_program: pubkeys[12],
            token_program_a: pubkeys[13],
            token_program_b: pubkeys[14],
            hook_program_a: pubkeys[15],
            hook_program_b: pubkeys[16],
            associated_token_program: pubkeys[17],
            system_program: pubkeys[18],
        }
    }
}
impl<'info> From<CreatePoolAccounts<'_, 'info>>
for [AccountInfo<'info>; CREATE_POOL_IX_ACCOUNTS_LEN] {
    fn from(accounts: CreatePoolAccounts<'_, 'info>) -> Self {
        [
            accounts.config.clone(),
            accounts.authority.clone(),
            accounts.pool.clone(),
            accounts.vault.clone(),
            accounts.mint_a.clone(),
            accounts.mint_b.clone(),
            accounts.authority_token_account_a.clone(),
            accounts.authority_token_account_b.clone(),
            accounts.vault_token_account_a.clone(),
            accounts.vault_token_account_b.clone(),
            accounts.liquidity_token.clone(),
            accounts.authority_liquidity_token_account.clone(),
            accounts.token_program.clone(),
            accounts.token_program_a.clone(),
            accounts.token_program_b.clone(),
            accounts.hook_program_a.clone(),
            accounts.hook_program_b.clone(),
            accounts.associated_token_program.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CREATE_POOL_IX_ACCOUNTS_LEN]>
for CreatePoolAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; CREATE_POOL_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            config: &arr[0],
            authority: &arr[1],
            pool: &arr[2],
            vault: &arr[3],
            mint_a: &arr[4],
            mint_b: &arr[5],
            authority_token_account_a: &arr[6],
            authority_token_account_b: &arr[7],
            vault_token_account_a: &arr[8],
            vault_token_account_b: &arr[9],
            liquidity_token: &arr[10],
            authority_liquidity_token_account: &arr[11],
            token_program: &arr[12],
            token_program_a: &arr[13],
            token_program_b: &arr[14],
            hook_program_a: &arr[15],
            hook_program_b: &arr[16],
            associated_token_program: &arr[17],
            system_program: &arr[18],
        }
    }
}
pub const CREATE_POOL_IX_DISCM: [u8; 8usize] = [233, 146, 209, 142, 207, 104, 64, 188];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CreatePoolIxArgs {
    pub amount_a: u64,
    pub amount_b: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct CreatePoolIxData(pub CreatePoolIxArgs);
impl From<CreatePoolIxArgs> for CreatePoolIxData {
    fn from(args: CreatePoolIxArgs) -> Self {
        Self(args)
    }
}
impl CreatePoolIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CREATE_POOL_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let amount_a: u64 = crate::borsh_de_or_default(&mut reader)?;
        let amount_b: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(CreatePoolIxArgs {
                amount_a,
                amount_b,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CREATE_POOL_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.amount_a, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.amount_b, &mut writer)?;
        Ok(())
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
    args: CreatePoolIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CREATE_POOL_IX_ACCOUNTS_LEN] = keys.into();
    let data: CreatePoolIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn create_pool_ix(
    keys: CreatePoolKeys,
    args: CreatePoolIxArgs,
) -> std::io::Result<Instruction> {
    create_pool_ix_with_program_id(PRINT_DEX_PROGRAM_ID, keys, args)
}
pub fn create_pool_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CreatePoolAccounts<'_, '_>,
    args: CreatePoolIxArgs,
) -> ProgramResult {
    let keys: CreatePoolKeys = accounts.into();
    let ix = create_pool_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn create_pool_invoke(
    accounts: CreatePoolAccounts<'_, '_>,
    args: CreatePoolIxArgs,
) -> ProgramResult {
    create_pool_invoke_with_program_id(PRINT_DEX_PROGRAM_ID, accounts, args)
}
pub fn create_pool_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CreatePoolAccounts<'_, '_>,
    args: CreatePoolIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CreatePoolKeys = accounts.into();
    let ix = create_pool_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn create_pool_invoke_signed(
    accounts: CreatePoolAccounts<'_, '_>,
    args: CreatePoolIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    create_pool_invoke_signed_with_program_id(
        PRINT_DEX_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn create_pool_verify_account_keys(
    accounts: CreatePoolAccounts<'_, '_>,
    keys: CreatePoolKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.config.key, keys.config),
        (*accounts.authority.key, keys.authority),
        (*accounts.pool.key, keys.pool),
        (*accounts.vault.key, keys.vault),
        (*accounts.mint_a.key, keys.mint_a),
        (*accounts.mint_b.key, keys.mint_b),
        (*accounts.authority_token_account_a.key, keys.authority_token_account_a),
        (*accounts.authority_token_account_b.key, keys.authority_token_account_b),
        (*accounts.vault_token_account_a.key, keys.vault_token_account_a),
        (*accounts.vault_token_account_b.key, keys.vault_token_account_b),
        (*accounts.liquidity_token.key, keys.liquidity_token),
        (
            *accounts.authority_liquidity_token_account.key,
            keys.authority_liquidity_token_account,
        ),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.token_program_a.key, keys.token_program_a),
        (*accounts.token_program_b.key, keys.token_program_b),
        (*accounts.hook_program_a.key, keys.hook_program_a),
        (*accounts.hook_program_b.key, keys.hook_program_b),
        (*accounts.associated_token_program.key, keys.associated_token_program),
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
        accounts.config,
        accounts.authority,
        accounts.pool,
        accounts.vault,
        accounts.authority_token_account_a,
        accounts.authority_token_account_b,
        accounts.vault_token_account_a,
        accounts.vault_token_account_b,
        accounts.liquidity_token,
        accounts.authority_liquidity_token_account,
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
    for should_be_signer in [accounts.authority] {
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
pub const ADD_LIQUIDITY_IX_ACCOUNTS_LEN: usize = 19;
#[derive(Copy, Clone, Debug)]
pub struct AddLiquidityAccounts<'me, 'info> {
    pub config: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub pool: &'me AccountInfo<'info>,
    pub vault: &'me AccountInfo<'info>,
    pub mint_a: &'me AccountInfo<'info>,
    pub mint_b: &'me AccountInfo<'info>,
    pub authority_token_account_a: &'me AccountInfo<'info>,
    pub authority_token_account_b: &'me AccountInfo<'info>,
    pub vault_token_account_a: &'me AccountInfo<'info>,
    pub vault_token_account_b: &'me AccountInfo<'info>,
    pub liquidity_token: &'me AccountInfo<'info>,
    pub authority_liquidity_token_account: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub token_program_a: &'me AccountInfo<'info>,
    pub token_program_b: &'me AccountInfo<'info>,
    pub hook_program_a: &'me AccountInfo<'info>,
    pub hook_program_b: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct AddLiquidityKeys {
    pub config: Pubkey,
    pub authority: Pubkey,
    pub pool: Pubkey,
    pub vault: Pubkey,
    pub mint_a: Pubkey,
    pub mint_b: Pubkey,
    pub authority_token_account_a: Pubkey,
    pub authority_token_account_b: Pubkey,
    pub vault_token_account_a: Pubkey,
    pub vault_token_account_b: Pubkey,
    pub liquidity_token: Pubkey,
    pub authority_liquidity_token_account: Pubkey,
    pub token_program: Pubkey,
    pub token_program_a: Pubkey,
    pub token_program_b: Pubkey,
    pub hook_program_a: Pubkey,
    pub hook_program_b: Pubkey,
    pub associated_token_program: Pubkey,
    pub system_program: Pubkey,
}
impl From<AddLiquidityAccounts<'_, '_>> for AddLiquidityKeys {
    fn from(accounts: AddLiquidityAccounts) -> Self {
        Self {
            config: *accounts.config.key,
            authority: *accounts.authority.key,
            pool: *accounts.pool.key,
            vault: *accounts.vault.key,
            mint_a: *accounts.mint_a.key,
            mint_b: *accounts.mint_b.key,
            authority_token_account_a: *accounts.authority_token_account_a.key,
            authority_token_account_b: *accounts.authority_token_account_b.key,
            vault_token_account_a: *accounts.vault_token_account_a.key,
            vault_token_account_b: *accounts.vault_token_account_b.key,
            liquidity_token: *accounts.liquidity_token.key,
            authority_liquidity_token_account: *accounts
                .authority_liquidity_token_account
                .key,
            token_program: *accounts.token_program.key,
            token_program_a: *accounts.token_program_a.key,
            token_program_b: *accounts.token_program_b.key,
            hook_program_a: *accounts.hook_program_a.key,
            hook_program_b: *accounts.hook_program_b.key,
            associated_token_program: *accounts.associated_token_program.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<AddLiquidityKeys> for [AccountMeta; ADD_LIQUIDITY_IX_ACCOUNTS_LEN] {
    fn from(keys: AddLiquidityKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.config,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.authority,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.mint_a,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.mint_b,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.authority_token_account_a,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.authority_token_account_b,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vault_token_account_a,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vault_token_account_b,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.liquidity_token,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.authority_liquidity_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_program_a,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_program_b,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.hook_program_a,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.hook_program_b,
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
impl From<[Pubkey; ADD_LIQUIDITY_IX_ACCOUNTS_LEN]> for AddLiquidityKeys {
    fn from(pubkeys: [Pubkey; ADD_LIQUIDITY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            config: pubkeys[0],
            authority: pubkeys[1],
            pool: pubkeys[2],
            vault: pubkeys[3],
            mint_a: pubkeys[4],
            mint_b: pubkeys[5],
            authority_token_account_a: pubkeys[6],
            authority_token_account_b: pubkeys[7],
            vault_token_account_a: pubkeys[8],
            vault_token_account_b: pubkeys[9],
            liquidity_token: pubkeys[10],
            authority_liquidity_token_account: pubkeys[11],
            token_program: pubkeys[12],
            token_program_a: pubkeys[13],
            token_program_b: pubkeys[14],
            hook_program_a: pubkeys[15],
            hook_program_b: pubkeys[16],
            associated_token_program: pubkeys[17],
            system_program: pubkeys[18],
        }
    }
}
impl<'info> From<AddLiquidityAccounts<'_, 'info>>
for [AccountInfo<'info>; ADD_LIQUIDITY_IX_ACCOUNTS_LEN] {
    fn from(accounts: AddLiquidityAccounts<'_, 'info>) -> Self {
        [
            accounts.config.clone(),
            accounts.authority.clone(),
            accounts.pool.clone(),
            accounts.vault.clone(),
            accounts.mint_a.clone(),
            accounts.mint_b.clone(),
            accounts.authority_token_account_a.clone(),
            accounts.authority_token_account_b.clone(),
            accounts.vault_token_account_a.clone(),
            accounts.vault_token_account_b.clone(),
            accounts.liquidity_token.clone(),
            accounts.authority_liquidity_token_account.clone(),
            accounts.token_program.clone(),
            accounts.token_program_a.clone(),
            accounts.token_program_b.clone(),
            accounts.hook_program_a.clone(),
            accounts.hook_program_b.clone(),
            accounts.associated_token_program.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; ADD_LIQUIDITY_IX_ACCOUNTS_LEN]>
for AddLiquidityAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; ADD_LIQUIDITY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            config: &arr[0],
            authority: &arr[1],
            pool: &arr[2],
            vault: &arr[3],
            mint_a: &arr[4],
            mint_b: &arr[5],
            authority_token_account_a: &arr[6],
            authority_token_account_b: &arr[7],
            vault_token_account_a: &arr[8],
            vault_token_account_b: &arr[9],
            liquidity_token: &arr[10],
            authority_liquidity_token_account: &arr[11],
            token_program: &arr[12],
            token_program_a: &arr[13],
            token_program_b: &arr[14],
            hook_program_a: &arr[15],
            hook_program_b: &arr[16],
            associated_token_program: &arr[17],
            system_program: &arr[18],
        }
    }
}
pub const ADD_LIQUIDITY_IX_DISCM: [u8; 8usize] = [181, 157, 89, 67, 143, 182, 52, 72];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AddLiquidityIxArgs {
    pub amount_a: u64,
    pub amount_b: u64,
    pub slippage: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct AddLiquidityIxData(pub AddLiquidityIxArgs);
impl From<AddLiquidityIxArgs> for AddLiquidityIxData {
    fn from(args: AddLiquidityIxArgs) -> Self {
        Self(args)
    }
}
impl AddLiquidityIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != ADD_LIQUIDITY_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let amount_a: u64 = crate::borsh_de_or_default(&mut reader)?;
        let amount_b: u64 = crate::borsh_de_or_default(&mut reader)?;
        let slippage: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(AddLiquidityIxArgs {
                amount_a,
                amount_b,
                slippage,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&ADD_LIQUIDITY_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.amount_a, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.amount_b, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.slippage, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn add_liquidity_ix_with_program_id(
    program_id: Pubkey,
    keys: AddLiquidityKeys,
    args: AddLiquidityIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; ADD_LIQUIDITY_IX_ACCOUNTS_LEN] = keys.into();
    let data: AddLiquidityIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn add_liquidity_ix(
    keys: AddLiquidityKeys,
    args: AddLiquidityIxArgs,
) -> std::io::Result<Instruction> {
    add_liquidity_ix_with_program_id(PRINT_DEX_PROGRAM_ID, keys, args)
}
pub fn add_liquidity_invoke_with_program_id(
    program_id: Pubkey,
    accounts: AddLiquidityAccounts<'_, '_>,
    args: AddLiquidityIxArgs,
) -> ProgramResult {
    let keys: AddLiquidityKeys = accounts.into();
    let ix = add_liquidity_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn add_liquidity_invoke(
    accounts: AddLiquidityAccounts<'_, '_>,
    args: AddLiquidityIxArgs,
) -> ProgramResult {
    add_liquidity_invoke_with_program_id(PRINT_DEX_PROGRAM_ID, accounts, args)
}
pub fn add_liquidity_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: AddLiquidityAccounts<'_, '_>,
    args: AddLiquidityIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: AddLiquidityKeys = accounts.into();
    let ix = add_liquidity_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn add_liquidity_invoke_signed(
    accounts: AddLiquidityAccounts<'_, '_>,
    args: AddLiquidityIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    add_liquidity_invoke_signed_with_program_id(
        PRINT_DEX_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn add_liquidity_verify_account_keys(
    accounts: AddLiquidityAccounts<'_, '_>,
    keys: AddLiquidityKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.config.key, keys.config),
        (*accounts.authority.key, keys.authority),
        (*accounts.pool.key, keys.pool),
        (*accounts.vault.key, keys.vault),
        (*accounts.mint_a.key, keys.mint_a),
        (*accounts.mint_b.key, keys.mint_b),
        (*accounts.authority_token_account_a.key, keys.authority_token_account_a),
        (*accounts.authority_token_account_b.key, keys.authority_token_account_b),
        (*accounts.vault_token_account_a.key, keys.vault_token_account_a),
        (*accounts.vault_token_account_b.key, keys.vault_token_account_b),
        (*accounts.liquidity_token.key, keys.liquidity_token),
        (
            *accounts.authority_liquidity_token_account.key,
            keys.authority_liquidity_token_account,
        ),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.token_program_a.key, keys.token_program_a),
        (*accounts.token_program_b.key, keys.token_program_b),
        (*accounts.hook_program_a.key, keys.hook_program_a),
        (*accounts.hook_program_b.key, keys.hook_program_b),
        (*accounts.associated_token_program.key, keys.associated_token_program),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn add_liquidity_verify_writable_privileges<'me, 'info>(
    accounts: AddLiquidityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.config,
        accounts.authority,
        accounts.pool,
        accounts.vault,
        accounts.authority_token_account_a,
        accounts.authority_token_account_b,
        accounts.vault_token_account_a,
        accounts.vault_token_account_b,
        accounts.liquidity_token,
        accounts.authority_liquidity_token_account,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn add_liquidity_verify_signer_privileges<'me, 'info>(
    accounts: AddLiquidityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn add_liquidity_verify_account_privileges<'me, 'info>(
    accounts: AddLiquidityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    add_liquidity_verify_writable_privileges(accounts)?;
    add_liquidity_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const REMOVE_LIQUIDITY_IX_ACCOUNTS_LEN: usize = 19;
#[derive(Copy, Clone, Debug)]
pub struct RemoveLiquidityAccounts<'me, 'info> {
    pub config: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub pool: &'me AccountInfo<'info>,
    pub vault: &'me AccountInfo<'info>,
    pub mint_a: &'me AccountInfo<'info>,
    pub mint_b: &'me AccountInfo<'info>,
    pub authority_token_account_a: &'me AccountInfo<'info>,
    pub authority_token_account_b: &'me AccountInfo<'info>,
    pub vault_token_account_a: &'me AccountInfo<'info>,
    pub vault_token_account_b: &'me AccountInfo<'info>,
    pub liquidity_token: &'me AccountInfo<'info>,
    pub authority_liquidity_token_account: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub token_program_a: &'me AccountInfo<'info>,
    pub token_program_b: &'me AccountInfo<'info>,
    pub hook_program_a: &'me AccountInfo<'info>,
    pub hook_program_b: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct RemoveLiquidityKeys {
    pub config: Pubkey,
    pub authority: Pubkey,
    pub pool: Pubkey,
    pub vault: Pubkey,
    pub mint_a: Pubkey,
    pub mint_b: Pubkey,
    pub authority_token_account_a: Pubkey,
    pub authority_token_account_b: Pubkey,
    pub vault_token_account_a: Pubkey,
    pub vault_token_account_b: Pubkey,
    pub liquidity_token: Pubkey,
    pub authority_liquidity_token_account: Pubkey,
    pub token_program: Pubkey,
    pub token_program_a: Pubkey,
    pub token_program_b: Pubkey,
    pub hook_program_a: Pubkey,
    pub hook_program_b: Pubkey,
    pub associated_token_program: Pubkey,
    pub system_program: Pubkey,
}
impl From<RemoveLiquidityAccounts<'_, '_>> for RemoveLiquidityKeys {
    fn from(accounts: RemoveLiquidityAccounts) -> Self {
        Self {
            config: *accounts.config.key,
            authority: *accounts.authority.key,
            pool: *accounts.pool.key,
            vault: *accounts.vault.key,
            mint_a: *accounts.mint_a.key,
            mint_b: *accounts.mint_b.key,
            authority_token_account_a: *accounts.authority_token_account_a.key,
            authority_token_account_b: *accounts.authority_token_account_b.key,
            vault_token_account_a: *accounts.vault_token_account_a.key,
            vault_token_account_b: *accounts.vault_token_account_b.key,
            liquidity_token: *accounts.liquidity_token.key,
            authority_liquidity_token_account: *accounts
                .authority_liquidity_token_account
                .key,
            token_program: *accounts.token_program.key,
            token_program_a: *accounts.token_program_a.key,
            token_program_b: *accounts.token_program_b.key,
            hook_program_a: *accounts.hook_program_a.key,
            hook_program_b: *accounts.hook_program_b.key,
            associated_token_program: *accounts.associated_token_program.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<RemoveLiquidityKeys> for [AccountMeta; REMOVE_LIQUIDITY_IX_ACCOUNTS_LEN] {
    fn from(keys: RemoveLiquidityKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.config,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.authority,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.mint_a,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.mint_b,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.authority_token_account_a,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.authority_token_account_b,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vault_token_account_a,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vault_token_account_b,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.liquidity_token,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.authority_liquidity_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_program_a,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_program_b,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.hook_program_a,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.hook_program_b,
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
impl From<[Pubkey; REMOVE_LIQUIDITY_IX_ACCOUNTS_LEN]> for RemoveLiquidityKeys {
    fn from(pubkeys: [Pubkey; REMOVE_LIQUIDITY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            config: pubkeys[0],
            authority: pubkeys[1],
            pool: pubkeys[2],
            vault: pubkeys[3],
            mint_a: pubkeys[4],
            mint_b: pubkeys[5],
            authority_token_account_a: pubkeys[6],
            authority_token_account_b: pubkeys[7],
            vault_token_account_a: pubkeys[8],
            vault_token_account_b: pubkeys[9],
            liquidity_token: pubkeys[10],
            authority_liquidity_token_account: pubkeys[11],
            token_program: pubkeys[12],
            token_program_a: pubkeys[13],
            token_program_b: pubkeys[14],
            hook_program_a: pubkeys[15],
            hook_program_b: pubkeys[16],
            associated_token_program: pubkeys[17],
            system_program: pubkeys[18],
        }
    }
}
impl<'info> From<RemoveLiquidityAccounts<'_, 'info>>
for [AccountInfo<'info>; REMOVE_LIQUIDITY_IX_ACCOUNTS_LEN] {
    fn from(accounts: RemoveLiquidityAccounts<'_, 'info>) -> Self {
        [
            accounts.config.clone(),
            accounts.authority.clone(),
            accounts.pool.clone(),
            accounts.vault.clone(),
            accounts.mint_a.clone(),
            accounts.mint_b.clone(),
            accounts.authority_token_account_a.clone(),
            accounts.authority_token_account_b.clone(),
            accounts.vault_token_account_a.clone(),
            accounts.vault_token_account_b.clone(),
            accounts.liquidity_token.clone(),
            accounts.authority_liquidity_token_account.clone(),
            accounts.token_program.clone(),
            accounts.token_program_a.clone(),
            accounts.token_program_b.clone(),
            accounts.hook_program_a.clone(),
            accounts.hook_program_b.clone(),
            accounts.associated_token_program.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; REMOVE_LIQUIDITY_IX_ACCOUNTS_LEN]>
for RemoveLiquidityAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; REMOVE_LIQUIDITY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            config: &arr[0],
            authority: &arr[1],
            pool: &arr[2],
            vault: &arr[3],
            mint_a: &arr[4],
            mint_b: &arr[5],
            authority_token_account_a: &arr[6],
            authority_token_account_b: &arr[7],
            vault_token_account_a: &arr[8],
            vault_token_account_b: &arr[9],
            liquidity_token: &arr[10],
            authority_liquidity_token_account: &arr[11],
            token_program: &arr[12],
            token_program_a: &arr[13],
            token_program_b: &arr[14],
            hook_program_a: &arr[15],
            hook_program_b: &arr[16],
            associated_token_program: &arr[17],
            system_program: &arr[18],
        }
    }
}
pub const REMOVE_LIQUIDITY_IX_DISCM: [u8; 8usize] = [80, 85, 209, 72, 24, 206, 177, 108];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct RemoveLiquidityIxArgs {
    pub liquidity: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct RemoveLiquidityIxData(pub RemoveLiquidityIxArgs);
impl From<RemoveLiquidityIxArgs> for RemoveLiquidityIxData {
    fn from(args: RemoveLiquidityIxArgs) -> Self {
        Self(args)
    }
}
impl RemoveLiquidityIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != REMOVE_LIQUIDITY_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let liquidity: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(Self(RemoveLiquidityIxArgs { liquidity }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&REMOVE_LIQUIDITY_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.liquidity, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn remove_liquidity_ix_with_program_id(
    program_id: Pubkey,
    keys: RemoveLiquidityKeys,
    args: RemoveLiquidityIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; REMOVE_LIQUIDITY_IX_ACCOUNTS_LEN] = keys.into();
    let data: RemoveLiquidityIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn remove_liquidity_ix(
    keys: RemoveLiquidityKeys,
    args: RemoveLiquidityIxArgs,
) -> std::io::Result<Instruction> {
    remove_liquidity_ix_with_program_id(PRINT_DEX_PROGRAM_ID, keys, args)
}
pub fn remove_liquidity_invoke_with_program_id(
    program_id: Pubkey,
    accounts: RemoveLiquidityAccounts<'_, '_>,
    args: RemoveLiquidityIxArgs,
) -> ProgramResult {
    let keys: RemoveLiquidityKeys = accounts.into();
    let ix = remove_liquidity_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn remove_liquidity_invoke(
    accounts: RemoveLiquidityAccounts<'_, '_>,
    args: RemoveLiquidityIxArgs,
) -> ProgramResult {
    remove_liquidity_invoke_with_program_id(PRINT_DEX_PROGRAM_ID, accounts, args)
}
pub fn remove_liquidity_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: RemoveLiquidityAccounts<'_, '_>,
    args: RemoveLiquidityIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: RemoveLiquidityKeys = accounts.into();
    let ix = remove_liquidity_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn remove_liquidity_invoke_signed(
    accounts: RemoveLiquidityAccounts<'_, '_>,
    args: RemoveLiquidityIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    remove_liquidity_invoke_signed_with_program_id(
        PRINT_DEX_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn remove_liquidity_verify_account_keys(
    accounts: RemoveLiquidityAccounts<'_, '_>,
    keys: RemoveLiquidityKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.config.key, keys.config),
        (*accounts.authority.key, keys.authority),
        (*accounts.pool.key, keys.pool),
        (*accounts.vault.key, keys.vault),
        (*accounts.mint_a.key, keys.mint_a),
        (*accounts.mint_b.key, keys.mint_b),
        (*accounts.authority_token_account_a.key, keys.authority_token_account_a),
        (*accounts.authority_token_account_b.key, keys.authority_token_account_b),
        (*accounts.vault_token_account_a.key, keys.vault_token_account_a),
        (*accounts.vault_token_account_b.key, keys.vault_token_account_b),
        (*accounts.liquidity_token.key, keys.liquidity_token),
        (
            *accounts.authority_liquidity_token_account.key,
            keys.authority_liquidity_token_account,
        ),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.token_program_a.key, keys.token_program_a),
        (*accounts.token_program_b.key, keys.token_program_b),
        (*accounts.hook_program_a.key, keys.hook_program_a),
        (*accounts.hook_program_b.key, keys.hook_program_b),
        (*accounts.associated_token_program.key, keys.associated_token_program),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn remove_liquidity_verify_writable_privileges<'me, 'info>(
    accounts: RemoveLiquidityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.config,
        accounts.authority,
        accounts.pool,
        accounts.vault,
        accounts.authority_token_account_a,
        accounts.authority_token_account_b,
        accounts.vault_token_account_a,
        accounts.vault_token_account_b,
        accounts.liquidity_token,
        accounts.authority_liquidity_token_account,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn remove_liquidity_verify_signer_privileges<'me, 'info>(
    accounts: RemoveLiquidityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn remove_liquidity_verify_account_privileges<'me, 'info>(
    accounts: RemoveLiquidityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    remove_liquidity_verify_writable_privileges(accounts)?;
    remove_liquidity_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SWAP_IX_ACCOUNTS_LEN: usize = 17;
#[derive(Copy, Clone, Debug)]
pub struct SwapAccounts<'me, 'info> {
    pub config: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub pool: &'me AccountInfo<'info>,
    pub vault: &'me AccountInfo<'info>,
    pub mint_a: &'me AccountInfo<'info>,
    pub mint_b: &'me AccountInfo<'info>,
    pub authority_token_account_a: &'me AccountInfo<'info>,
    pub authority_token_account_b: &'me AccountInfo<'info>,
    pub vault_token_account_a: &'me AccountInfo<'info>,
    pub vault_token_account_b: &'me AccountInfo<'info>,
    pub token_program_a: &'me AccountInfo<'info>,
    pub token_program_b: &'me AccountInfo<'info>,
    pub hook_program_a: &'me AccountInfo<'info>,
    pub hook_program_b: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SwapKeys {
    pub config: Pubkey,
    pub authority: Pubkey,
    pub pool: Pubkey,
    pub vault: Pubkey,
    pub mint_a: Pubkey,
    pub mint_b: Pubkey,
    pub authority_token_account_a: Pubkey,
    pub authority_token_account_b: Pubkey,
    pub vault_token_account_a: Pubkey,
    pub vault_token_account_b: Pubkey,
    pub token_program_a: Pubkey,
    pub token_program_b: Pubkey,
    pub hook_program_a: Pubkey,
    pub hook_program_b: Pubkey,
    pub associated_token_program: Pubkey,
    pub token_program: Pubkey,
    pub system_program: Pubkey,
}
impl From<SwapAccounts<'_, '_>> for SwapKeys {
    fn from(accounts: SwapAccounts) -> Self {
        Self {
            config: *accounts.config.key,
            authority: *accounts.authority.key,
            pool: *accounts.pool.key,
            vault: *accounts.vault.key,
            mint_a: *accounts.mint_a.key,
            mint_b: *accounts.mint_b.key,
            authority_token_account_a: *accounts.authority_token_account_a.key,
            authority_token_account_b: *accounts.authority_token_account_b.key,
            vault_token_account_a: *accounts.vault_token_account_a.key,
            vault_token_account_b: *accounts.vault_token_account_b.key,
            token_program_a: *accounts.token_program_a.key,
            token_program_b: *accounts.token_program_b.key,
            hook_program_a: *accounts.hook_program_a.key,
            hook_program_b: *accounts.hook_program_b.key,
            associated_token_program: *accounts.associated_token_program.key,
            token_program: *accounts.token_program.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<SwapKeys> for [AccountMeta; SWAP_IX_ACCOUNTS_LEN] {
    fn from(keys: SwapKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.config,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.authority,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pool,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.mint_a,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.mint_b,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.authority_token_account_a,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.authority_token_account_b,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vault_token_account_a,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vault_token_account_b,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_program_a,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_program_b,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.hook_program_a,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.hook_program_b,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.associated_token_program,
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
impl From<[Pubkey; SWAP_IX_ACCOUNTS_LEN]> for SwapKeys {
    fn from(pubkeys: [Pubkey; SWAP_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            config: pubkeys[0],
            authority: pubkeys[1],
            pool: pubkeys[2],
            vault: pubkeys[3],
            mint_a: pubkeys[4],
            mint_b: pubkeys[5],
            authority_token_account_a: pubkeys[6],
            authority_token_account_b: pubkeys[7],
            vault_token_account_a: pubkeys[8],
            vault_token_account_b: pubkeys[9],
            token_program_a: pubkeys[10],
            token_program_b: pubkeys[11],
            hook_program_a: pubkeys[12],
            hook_program_b: pubkeys[13],
            associated_token_program: pubkeys[14],
            token_program: pubkeys[15],
            system_program: pubkeys[16],
        }
    }
}
impl<'info> From<SwapAccounts<'_, 'info>>
for [AccountInfo<'info>; SWAP_IX_ACCOUNTS_LEN] {
    fn from(accounts: SwapAccounts<'_, 'info>) -> Self {
        [
            accounts.config.clone(),
            accounts.authority.clone(),
            accounts.pool.clone(),
            accounts.vault.clone(),
            accounts.mint_a.clone(),
            accounts.mint_b.clone(),
            accounts.authority_token_account_a.clone(),
            accounts.authority_token_account_b.clone(),
            accounts.vault_token_account_a.clone(),
            accounts.vault_token_account_b.clone(),
            accounts.token_program_a.clone(),
            accounts.token_program_b.clone(),
            accounts.hook_program_a.clone(),
            accounts.hook_program_b.clone(),
            accounts.associated_token_program.clone(),
            accounts.token_program.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SWAP_IX_ACCOUNTS_LEN]>
for SwapAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; SWAP_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            config: &arr[0],
            authority: &arr[1],
            pool: &arr[2],
            vault: &arr[3],
            mint_a: &arr[4],
            mint_b: &arr[5],
            authority_token_account_a: &arr[6],
            authority_token_account_b: &arr[7],
            vault_token_account_a: &arr[8],
            vault_token_account_b: &arr[9],
            token_program_a: &arr[10],
            token_program_b: &arr[11],
            hook_program_a: &arr[12],
            hook_program_b: &arr[13],
            associated_token_program: &arr[14],
            token_program: &arr[15],
            system_program: &arr[16],
        }
    }
}
pub const SWAP_IX_DISCM: [u8; 8usize] = [248, 198, 158, 145, 225, 117, 135, 200];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SwapIxArgs {
    pub amount_a_in: u64,
    pub expected_amount_b_out: u64,
    pub slippage: u64,
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
        let amount_a_in: u64 = crate::borsh_de_or_default(&mut reader)?;
        let expected_amount_b_out: u64 = crate::borsh_de_or_default(&mut reader)?;
        let slippage: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(SwapIxArgs {
                amount_a_in,
                expected_amount_b_out,
                slippage,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SWAP_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.amount_a_in, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.expected_amount_b_out, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.slippage, &mut writer)?;
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
    swap_ix_with_program_id(PRINT_DEX_PROGRAM_ID, keys, args)
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
    swap_invoke_with_program_id(PRINT_DEX_PROGRAM_ID, accounts, args)
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
    swap_invoke_signed_with_program_id(PRINT_DEX_PROGRAM_ID, accounts, args, seeds)
}
pub fn swap_verify_account_keys(
    accounts: SwapAccounts<'_, '_>,
    keys: SwapKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.config.key, keys.config),
        (*accounts.authority.key, keys.authority),
        (*accounts.pool.key, keys.pool),
        (*accounts.vault.key, keys.vault),
        (*accounts.mint_a.key, keys.mint_a),
        (*accounts.mint_b.key, keys.mint_b),
        (*accounts.authority_token_account_a.key, keys.authority_token_account_a),
        (*accounts.authority_token_account_b.key, keys.authority_token_account_b),
        (*accounts.vault_token_account_a.key, keys.vault_token_account_a),
        (*accounts.vault_token_account_b.key, keys.vault_token_account_b),
        (*accounts.token_program_a.key, keys.token_program_a),
        (*accounts.token_program_b.key, keys.token_program_b),
        (*accounts.hook_program_a.key, keys.hook_program_a),
        (*accounts.hook_program_b.key, keys.hook_program_b),
        (*accounts.associated_token_program.key, keys.associated_token_program),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.system_program.key, keys.system_program),
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
        accounts.config,
        accounts.authority,
        accounts.vault,
        accounts.authority_token_account_a,
        accounts.authority_token_account_b,
        accounts.vault_token_account_a,
        accounts.vault_token_account_b,
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
    for should_be_signer in [accounts.authority] {
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
pub const COLLECT_PLATFORM_FEES_IX_ACCOUNTS_LEN: usize = 6;
#[derive(Copy, Clone, Debug)]
pub struct CollectPlatformFeesAccounts<'me, 'info> {
    pub config: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub pool: &'me AccountInfo<'info>,
    pub vault: &'me AccountInfo<'info>,
    pub fee_wallet: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CollectPlatformFeesKeys {
    pub config: Pubkey,
    pub authority: Pubkey,
    pub pool: Pubkey,
    pub vault: Pubkey,
    pub fee_wallet: Pubkey,
    pub system_program: Pubkey,
}
impl From<CollectPlatformFeesAccounts<'_, '_>> for CollectPlatformFeesKeys {
    fn from(accounts: CollectPlatformFeesAccounts) -> Self {
        Self {
            config: *accounts.config.key,
            authority: *accounts.authority.key,
            pool: *accounts.pool.key,
            vault: *accounts.vault.key,
            fee_wallet: *accounts.fee_wallet.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<CollectPlatformFeesKeys>
for [AccountMeta; COLLECT_PLATFORM_FEES_IX_ACCOUNTS_LEN] {
    fn from(keys: CollectPlatformFeesKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.authority,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pool,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.fee_wallet,
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
impl From<[Pubkey; COLLECT_PLATFORM_FEES_IX_ACCOUNTS_LEN]> for CollectPlatformFeesKeys {
    fn from(pubkeys: [Pubkey; COLLECT_PLATFORM_FEES_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            config: pubkeys[0],
            authority: pubkeys[1],
            pool: pubkeys[2],
            vault: pubkeys[3],
            fee_wallet: pubkeys[4],
            system_program: pubkeys[5],
        }
    }
}
impl<'info> From<CollectPlatformFeesAccounts<'_, 'info>>
for [AccountInfo<'info>; COLLECT_PLATFORM_FEES_IX_ACCOUNTS_LEN] {
    fn from(accounts: CollectPlatformFeesAccounts<'_, 'info>) -> Self {
        [
            accounts.config.clone(),
            accounts.authority.clone(),
            accounts.pool.clone(),
            accounts.vault.clone(),
            accounts.fee_wallet.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; COLLECT_PLATFORM_FEES_IX_ACCOUNTS_LEN]>
for CollectPlatformFeesAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; COLLECT_PLATFORM_FEES_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            config: &arr[0],
            authority: &arr[1],
            pool: &arr[2],
            vault: &arr[3],
            fee_wallet: &arr[4],
            system_program: &arr[5],
        }
    }
}
pub const COLLECT_PLATFORM_FEES_IX_DISCM: [u8; 8usize] = [
    191, 153, 219, 164, 5, 65, 153, 48,
];
#[derive(Clone, Debug, PartialEq)]
pub struct CollectPlatformFeesIxData;
impl CollectPlatformFeesIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != COLLECT_PLATFORM_FEES_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&COLLECT_PLATFORM_FEES_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn collect_platform_fees_ix_with_program_id(
    program_id: Pubkey,
    keys: CollectPlatformFeesKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; COLLECT_PLATFORM_FEES_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: CollectPlatformFeesIxData.try_to_vec()?,
    })
}
pub fn collect_platform_fees_ix(
    keys: CollectPlatformFeesKeys,
) -> std::io::Result<Instruction> {
    collect_platform_fees_ix_with_program_id(PRINT_DEX_PROGRAM_ID, keys)
}
pub fn collect_platform_fees_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CollectPlatformFeesAccounts<'_, '_>,
) -> ProgramResult {
    let keys: CollectPlatformFeesKeys = accounts.into();
    let ix = collect_platform_fees_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn collect_platform_fees_invoke(
    accounts: CollectPlatformFeesAccounts<'_, '_>,
) -> ProgramResult {
    collect_platform_fees_invoke_with_program_id(PRINT_DEX_PROGRAM_ID, accounts)
}
pub fn collect_platform_fees_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CollectPlatformFeesAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CollectPlatformFeesKeys = accounts.into();
    let ix = collect_platform_fees_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn collect_platform_fees_invoke_signed(
    accounts: CollectPlatformFeesAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    collect_platform_fees_invoke_signed_with_program_id(
        PRINT_DEX_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn collect_platform_fees_verify_account_keys(
    accounts: CollectPlatformFeesAccounts<'_, '_>,
    keys: CollectPlatformFeesKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.config.key, keys.config),
        (*accounts.authority.key, keys.authority),
        (*accounts.pool.key, keys.pool),
        (*accounts.vault.key, keys.vault),
        (*accounts.fee_wallet.key, keys.fee_wallet),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn collect_platform_fees_verify_writable_privileges<'me, 'info>(
    accounts: CollectPlatformFeesAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.authority, accounts.vault, accounts.fee_wallet] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn collect_platform_fees_verify_signer_privileges<'me, 'info>(
    accounts: CollectPlatformFeesAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn collect_platform_fees_verify_account_privileges<'me, 'info>(
    accounts: CollectPlatformFeesAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    collect_platform_fees_verify_writable_privileges(accounts)?;
    collect_platform_fees_verify_signer_privileges(accounts)?;
    Ok(())
}
