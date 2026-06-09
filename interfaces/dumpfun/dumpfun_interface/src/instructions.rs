use solana_pubkey::Pubkey;
use solana_cpi::{invoke, invoke_signed};
use solana_instruction::{AccountMeta, Instruction};
use solana_account_info::AccountInfo;
use solana_program_error::ProgramError;
use std::io::Read;
#[allow(unused_imports)]
use crate::*;
#[derive(Clone, Debug, PartialEq)]
pub enum DumpfunProgramIx {
    BuyExactTokens(BuyExactTokensIxArgs),
    BuyTokensWithExactSol(BuyTokensWithExactSolIxArgs),
    DrainPoolSurplus,
    Mint(MintIxArgs),
    SellExactTokens(SellExactTokensIxArgs),
}
impl DumpfunProgramIx {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        if buf.starts_with(&BUY_EXACT_TOKENS_IX_DISCM) {
            let mut reader = &buf[BUY_EXACT_TOKENS_IX_DISCM.len()..];
            let token_out: u64 = crate::borsh_de_or_default(&mut reader)?;
            let max_sol_in: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::BuyExactTokens(BuyExactTokensIxArgs {
                    token_out,
                    max_sol_in,
                }),
            );
        }
        if buf.starts_with(&BUY_TOKENS_WITH_EXACT_SOL_IX_DISCM) {
            let mut reader = &buf[BUY_TOKENS_WITH_EXACT_SOL_IX_DISCM.len()..];
            let sol_in: u64 = crate::borsh_de_or_default(&mut reader)?;
            let min_token_out: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::BuyTokensWithExactSol(BuyTokensWithExactSolIxArgs {
                    sol_in,
                    min_token_out,
                }),
            );
        }
        if buf.starts_with(&DRAIN_POOL_SURPLUS_IX_DISCM) {
            return Ok(Self::DrainPoolSurplus);
        }
        if buf.starts_with(&MINT_IX_DISCM) {
            let mut reader = &buf[MINT_IX_DISCM.len()..];
            let token_name: String = crate::borsh_de_or_default(&mut reader)?;
            let token_symbol: String = crate::borsh_de_or_default(&mut reader)?;
            let token_uri: String = crate::borsh_de_or_default(&mut reader)?;
            let sell_lock_period: i64 = crate::borsh_de_or_default(&mut reader)?;
            let virtual_sol_reserve: u64 = crate::borsh_de_or_default(&mut reader)?;
            let ramping_limits: Vec<RampingLimit> = crate::borsh_de_or_default(
                &mut reader,
            )?;
            return Ok(
                Self::Mint(MintIxArgs {
                    token_name,
                    token_symbol,
                    token_uri,
                    sell_lock_period,
                    virtual_sol_reserve,
                    ramping_limits,
                }),
            );
        }
        if buf.starts_with(&SELL_EXACT_TOKENS_IX_DISCM) {
            let mut reader = &buf[SELL_EXACT_TOKENS_IX_DISCM.len()..];
            let token_in: u64 = crate::borsh_de_or_default(&mut reader)?;
            let min_sol_out: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::SellExactTokens(SellExactTokensIxArgs {
                    token_in,
                    min_sol_out,
                }),
            );
        }
        Err(std::io::Error::from(std::io::ErrorKind::InvalidData))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        match self {
            Self::BuyExactTokens(args) => {
                writer.write_all(&BUY_EXACT_TOKENS_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.token_out, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.max_sol_in, &mut writer)?;
                Ok(())
            }
            Self::BuyTokensWithExactSol(args) => {
                writer.write_all(&BUY_TOKENS_WITH_EXACT_SOL_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.sol_in, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.min_token_out, &mut writer)?;
                Ok(())
            }
            Self::DrainPoolSurplus => writer.write_all(&DRAIN_POOL_SURPLUS_IX_DISCM),
            Self::Mint(args) => {
                writer.write_all(&MINT_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.token_name, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.token_symbol, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.token_uri, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.sell_lock_period, &mut writer)?;
                borsh::BorshSerialize::serialize(
                    &args.virtual_sol_reserve,
                    &mut writer,
                )?;
                borsh::BorshSerialize::serialize(&args.ramping_limits, &mut writer)?;
                Ok(())
            }
            Self::SellExactTokens(args) => {
                writer.write_all(&SELL_EXACT_TOKENS_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.token_in, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.min_sol_out, &mut writer)?;
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
pub const BUY_EXACT_TOKENS_IX_ACCOUNTS_LEN: usize = 11;
#[derive(Copy, Clone, Debug)]
pub struct BuyExactTokensAccounts<'me, 'info> {
    pub user: &'me AccountInfo<'info>,
    pub platform_fee_wallet: &'me AccountInfo<'info>,
    pub mint_account: &'me AccountInfo<'info>,
    pub liquidity_pool: &'me AccountInfo<'info>,
    pub liquidity_pool_authority: &'me AccountInfo<'info>,
    pub liquidity_pool_token_account: &'me AccountInfo<'info>,
    pub user_token_account: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub rent: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct BuyExactTokensKeys {
    pub user: Pubkey,
    pub platform_fee_wallet: Pubkey,
    pub mint_account: Pubkey,
    pub liquidity_pool: Pubkey,
    pub liquidity_pool_authority: Pubkey,
    pub liquidity_pool_token_account: Pubkey,
    pub user_token_account: Pubkey,
    pub token_program: Pubkey,
    pub associated_token_program: Pubkey,
    pub system_program: Pubkey,
    pub rent: Pubkey,
}
impl From<BuyExactTokensAccounts<'_, '_>> for BuyExactTokensKeys {
    fn from(accounts: BuyExactTokensAccounts) -> Self {
        Self {
            user: *accounts.user.key,
            platform_fee_wallet: *accounts.platform_fee_wallet.key,
            mint_account: *accounts.mint_account.key,
            liquidity_pool: *accounts.liquidity_pool.key,
            liquidity_pool_authority: *accounts.liquidity_pool_authority.key,
            liquidity_pool_token_account: *accounts.liquidity_pool_token_account.key,
            user_token_account: *accounts.user_token_account.key,
            token_program: *accounts.token_program.key,
            associated_token_program: *accounts.associated_token_program.key,
            system_program: *accounts.system_program.key,
            rent: *accounts.rent.key,
        }
    }
}
impl From<BuyExactTokensKeys> for [AccountMeta; BUY_EXACT_TOKENS_IX_ACCOUNTS_LEN] {
    fn from(keys: BuyExactTokensKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.user,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.platform_fee_wallet,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.mint_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.liquidity_pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.liquidity_pool_authority,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.liquidity_pool_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_token_account,
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
                pubkey: keys.rent,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; BUY_EXACT_TOKENS_IX_ACCOUNTS_LEN]> for BuyExactTokensKeys {
    fn from(pubkeys: [Pubkey; BUY_EXACT_TOKENS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            user: pubkeys[0],
            platform_fee_wallet: pubkeys[1],
            mint_account: pubkeys[2],
            liquidity_pool: pubkeys[3],
            liquidity_pool_authority: pubkeys[4],
            liquidity_pool_token_account: pubkeys[5],
            user_token_account: pubkeys[6],
            token_program: pubkeys[7],
            associated_token_program: pubkeys[8],
            system_program: pubkeys[9],
            rent: pubkeys[10],
        }
    }
}
impl<'info> From<BuyExactTokensAccounts<'_, 'info>>
for [AccountInfo<'info>; BUY_EXACT_TOKENS_IX_ACCOUNTS_LEN] {
    fn from(accounts: BuyExactTokensAccounts<'_, 'info>) -> Self {
        [
            accounts.user.clone(),
            accounts.platform_fee_wallet.clone(),
            accounts.mint_account.clone(),
            accounts.liquidity_pool.clone(),
            accounts.liquidity_pool_authority.clone(),
            accounts.liquidity_pool_token_account.clone(),
            accounts.user_token_account.clone(),
            accounts.token_program.clone(),
            accounts.associated_token_program.clone(),
            accounts.system_program.clone(),
            accounts.rent.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; BUY_EXACT_TOKENS_IX_ACCOUNTS_LEN]>
for BuyExactTokensAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; BUY_EXACT_TOKENS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            user: &arr[0],
            platform_fee_wallet: &arr[1],
            mint_account: &arr[2],
            liquidity_pool: &arr[3],
            liquidity_pool_authority: &arr[4],
            liquidity_pool_token_account: &arr[5],
            user_token_account: &arr[6],
            token_program: &arr[7],
            associated_token_program: &arr[8],
            system_program: &arr[9],
            rent: &arr[10],
        }
    }
}
pub const BUY_EXACT_TOKENS_IX_DISCM: [u8; 8usize] = [129, 145, 209, 75, 88, 169, 142, 8];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct BuyExactTokensIxArgs {
    pub token_out: u64,
    pub max_sol_in: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct BuyExactTokensIxData(pub BuyExactTokensIxArgs);
impl From<BuyExactTokensIxArgs> for BuyExactTokensIxData {
    fn from(args: BuyExactTokensIxArgs) -> Self {
        Self(args)
    }
}
impl BuyExactTokensIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != BUY_EXACT_TOKENS_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let token_out: u64 = crate::borsh_de_or_default(&mut reader)?;
        let max_sol_in: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(BuyExactTokensIxArgs {
                token_out,
                max_sol_in,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&BUY_EXACT_TOKENS_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.token_out, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.max_sol_in, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn buy_exact_tokens_ix_with_program_id(
    program_id: Pubkey,
    keys: BuyExactTokensKeys,
    args: BuyExactTokensIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; BUY_EXACT_TOKENS_IX_ACCOUNTS_LEN] = keys.into();
    let data: BuyExactTokensIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn buy_exact_tokens_ix(
    keys: BuyExactTokensKeys,
    args: BuyExactTokensIxArgs,
) -> std::io::Result<Instruction> {
    buy_exact_tokens_ix_with_program_id(DUMPFUN_PROGRAM_ID, keys, args)
}
pub fn buy_exact_tokens_invoke_with_program_id(
    program_id: Pubkey,
    accounts: BuyExactTokensAccounts<'_, '_>,
    args: BuyExactTokensIxArgs,
) -> ProgramResult {
    let keys: BuyExactTokensKeys = accounts.into();
    let ix = buy_exact_tokens_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn buy_exact_tokens_invoke(
    accounts: BuyExactTokensAccounts<'_, '_>,
    args: BuyExactTokensIxArgs,
) -> ProgramResult {
    buy_exact_tokens_invoke_with_program_id(DUMPFUN_PROGRAM_ID, accounts, args)
}
pub fn buy_exact_tokens_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: BuyExactTokensAccounts<'_, '_>,
    args: BuyExactTokensIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: BuyExactTokensKeys = accounts.into();
    let ix = buy_exact_tokens_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn buy_exact_tokens_invoke_signed(
    accounts: BuyExactTokensAccounts<'_, '_>,
    args: BuyExactTokensIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    buy_exact_tokens_invoke_signed_with_program_id(
        DUMPFUN_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn buy_exact_tokens_verify_account_keys(
    accounts: BuyExactTokensAccounts<'_, '_>,
    keys: BuyExactTokensKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.user.key, keys.user),
        (*accounts.platform_fee_wallet.key, keys.platform_fee_wallet),
        (*accounts.mint_account.key, keys.mint_account),
        (*accounts.liquidity_pool.key, keys.liquidity_pool),
        (*accounts.liquidity_pool_authority.key, keys.liquidity_pool_authority),
        (*accounts.liquidity_pool_token_account.key, keys.liquidity_pool_token_account),
        (*accounts.user_token_account.key, keys.user_token_account),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.associated_token_program.key, keys.associated_token_program),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.rent.key, keys.rent),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn buy_exact_tokens_verify_writable_privileges<'me, 'info>(
    accounts: BuyExactTokensAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.user,
        accounts.platform_fee_wallet,
        accounts.mint_account,
        accounts.liquidity_pool,
        accounts.liquidity_pool_authority,
        accounts.liquidity_pool_token_account,
        accounts.user_token_account,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn buy_exact_tokens_verify_signer_privileges<'me, 'info>(
    accounts: BuyExactTokensAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.user] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn buy_exact_tokens_verify_account_privileges<'me, 'info>(
    accounts: BuyExactTokensAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    buy_exact_tokens_verify_writable_privileges(accounts)?;
    buy_exact_tokens_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const BUY_TOKENS_WITH_EXACT_SOL_IX_ACCOUNTS_LEN: usize = 11;
#[derive(Copy, Clone, Debug)]
pub struct BuyTokensWithExactSolAccounts<'me, 'info> {
    pub user: &'me AccountInfo<'info>,
    pub platform_fee_wallet: &'me AccountInfo<'info>,
    pub mint_account: &'me AccountInfo<'info>,
    pub liquidity_pool: &'me AccountInfo<'info>,
    pub liquidity_pool_authority: &'me AccountInfo<'info>,
    pub liquidity_pool_token_account: &'me AccountInfo<'info>,
    pub user_token_account: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub rent: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct BuyTokensWithExactSolKeys {
    pub user: Pubkey,
    pub platform_fee_wallet: Pubkey,
    pub mint_account: Pubkey,
    pub liquidity_pool: Pubkey,
    pub liquidity_pool_authority: Pubkey,
    pub liquidity_pool_token_account: Pubkey,
    pub user_token_account: Pubkey,
    pub token_program: Pubkey,
    pub associated_token_program: Pubkey,
    pub system_program: Pubkey,
    pub rent: Pubkey,
}
impl From<BuyTokensWithExactSolAccounts<'_, '_>> for BuyTokensWithExactSolKeys {
    fn from(accounts: BuyTokensWithExactSolAccounts) -> Self {
        Self {
            user: *accounts.user.key,
            platform_fee_wallet: *accounts.platform_fee_wallet.key,
            mint_account: *accounts.mint_account.key,
            liquidity_pool: *accounts.liquidity_pool.key,
            liquidity_pool_authority: *accounts.liquidity_pool_authority.key,
            liquidity_pool_token_account: *accounts.liquidity_pool_token_account.key,
            user_token_account: *accounts.user_token_account.key,
            token_program: *accounts.token_program.key,
            associated_token_program: *accounts.associated_token_program.key,
            system_program: *accounts.system_program.key,
            rent: *accounts.rent.key,
        }
    }
}
impl From<BuyTokensWithExactSolKeys>
for [AccountMeta; BUY_TOKENS_WITH_EXACT_SOL_IX_ACCOUNTS_LEN] {
    fn from(keys: BuyTokensWithExactSolKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.user,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.platform_fee_wallet,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.mint_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.liquidity_pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.liquidity_pool_authority,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.liquidity_pool_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_token_account,
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
                pubkey: keys.rent,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; BUY_TOKENS_WITH_EXACT_SOL_IX_ACCOUNTS_LEN]>
for BuyTokensWithExactSolKeys {
    fn from(pubkeys: [Pubkey; BUY_TOKENS_WITH_EXACT_SOL_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            user: pubkeys[0],
            platform_fee_wallet: pubkeys[1],
            mint_account: pubkeys[2],
            liquidity_pool: pubkeys[3],
            liquidity_pool_authority: pubkeys[4],
            liquidity_pool_token_account: pubkeys[5],
            user_token_account: pubkeys[6],
            token_program: pubkeys[7],
            associated_token_program: pubkeys[8],
            system_program: pubkeys[9],
            rent: pubkeys[10],
        }
    }
}
impl<'info> From<BuyTokensWithExactSolAccounts<'_, 'info>>
for [AccountInfo<'info>; BUY_TOKENS_WITH_EXACT_SOL_IX_ACCOUNTS_LEN] {
    fn from(accounts: BuyTokensWithExactSolAccounts<'_, 'info>) -> Self {
        [
            accounts.user.clone(),
            accounts.platform_fee_wallet.clone(),
            accounts.mint_account.clone(),
            accounts.liquidity_pool.clone(),
            accounts.liquidity_pool_authority.clone(),
            accounts.liquidity_pool_token_account.clone(),
            accounts.user_token_account.clone(),
            accounts.token_program.clone(),
            accounts.associated_token_program.clone(),
            accounts.system_program.clone(),
            accounts.rent.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; BUY_TOKENS_WITH_EXACT_SOL_IX_ACCOUNTS_LEN]>
for BuyTokensWithExactSolAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; BUY_TOKENS_WITH_EXACT_SOL_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            user: &arr[0],
            platform_fee_wallet: &arr[1],
            mint_account: &arr[2],
            liquidity_pool: &arr[3],
            liquidity_pool_authority: &arr[4],
            liquidity_pool_token_account: &arr[5],
            user_token_account: &arr[6],
            token_program: &arr[7],
            associated_token_program: &arr[8],
            system_program: &arr[9],
            rent: &arr[10],
        }
    }
}
pub const BUY_TOKENS_WITH_EXACT_SOL_IX_DISCM: [u8; 8usize] = [
    107, 198, 250, 226, 105, 48, 89, 135,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct BuyTokensWithExactSolIxArgs {
    pub sol_in: u64,
    pub min_token_out: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct BuyTokensWithExactSolIxData(pub BuyTokensWithExactSolIxArgs);
impl From<BuyTokensWithExactSolIxArgs> for BuyTokensWithExactSolIxData {
    fn from(args: BuyTokensWithExactSolIxArgs) -> Self {
        Self(args)
    }
}
impl BuyTokensWithExactSolIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != BUY_TOKENS_WITH_EXACT_SOL_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let sol_in: u64 = crate::borsh_de_or_default(&mut reader)?;
        let min_token_out: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(BuyTokensWithExactSolIxArgs {
                sol_in,
                min_token_out,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&BUY_TOKENS_WITH_EXACT_SOL_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.sol_in, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.min_token_out, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn buy_tokens_with_exact_sol_ix_with_program_id(
    program_id: Pubkey,
    keys: BuyTokensWithExactSolKeys,
    args: BuyTokensWithExactSolIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; BUY_TOKENS_WITH_EXACT_SOL_IX_ACCOUNTS_LEN] = keys.into();
    let data: BuyTokensWithExactSolIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn buy_tokens_with_exact_sol_ix(
    keys: BuyTokensWithExactSolKeys,
    args: BuyTokensWithExactSolIxArgs,
) -> std::io::Result<Instruction> {
    buy_tokens_with_exact_sol_ix_with_program_id(DUMPFUN_PROGRAM_ID, keys, args)
}
pub fn buy_tokens_with_exact_sol_invoke_with_program_id(
    program_id: Pubkey,
    accounts: BuyTokensWithExactSolAccounts<'_, '_>,
    args: BuyTokensWithExactSolIxArgs,
) -> ProgramResult {
    let keys: BuyTokensWithExactSolKeys = accounts.into();
    let ix = buy_tokens_with_exact_sol_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn buy_tokens_with_exact_sol_invoke(
    accounts: BuyTokensWithExactSolAccounts<'_, '_>,
    args: BuyTokensWithExactSolIxArgs,
) -> ProgramResult {
    buy_tokens_with_exact_sol_invoke_with_program_id(DUMPFUN_PROGRAM_ID, accounts, args)
}
pub fn buy_tokens_with_exact_sol_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: BuyTokensWithExactSolAccounts<'_, '_>,
    args: BuyTokensWithExactSolIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: BuyTokensWithExactSolKeys = accounts.into();
    let ix = buy_tokens_with_exact_sol_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn buy_tokens_with_exact_sol_invoke_signed(
    accounts: BuyTokensWithExactSolAccounts<'_, '_>,
    args: BuyTokensWithExactSolIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    buy_tokens_with_exact_sol_invoke_signed_with_program_id(
        DUMPFUN_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn buy_tokens_with_exact_sol_verify_account_keys(
    accounts: BuyTokensWithExactSolAccounts<'_, '_>,
    keys: BuyTokensWithExactSolKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.user.key, keys.user),
        (*accounts.platform_fee_wallet.key, keys.platform_fee_wallet),
        (*accounts.mint_account.key, keys.mint_account),
        (*accounts.liquidity_pool.key, keys.liquidity_pool),
        (*accounts.liquidity_pool_authority.key, keys.liquidity_pool_authority),
        (*accounts.liquidity_pool_token_account.key, keys.liquidity_pool_token_account),
        (*accounts.user_token_account.key, keys.user_token_account),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.associated_token_program.key, keys.associated_token_program),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.rent.key, keys.rent),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn buy_tokens_with_exact_sol_verify_writable_privileges<'me, 'info>(
    accounts: BuyTokensWithExactSolAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.user,
        accounts.platform_fee_wallet,
        accounts.mint_account,
        accounts.liquidity_pool,
        accounts.liquidity_pool_authority,
        accounts.liquidity_pool_token_account,
        accounts.user_token_account,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn buy_tokens_with_exact_sol_verify_signer_privileges<'me, 'info>(
    accounts: BuyTokensWithExactSolAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.user] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn buy_tokens_with_exact_sol_verify_account_privileges<'me, 'info>(
    accounts: BuyTokensWithExactSolAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    buy_tokens_with_exact_sol_verify_writable_privileges(accounts)?;
    buy_tokens_with_exact_sol_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const DRAIN_POOL_SURPLUS_IX_ACCOUNTS_LEN: usize = 6;
#[derive(Copy, Clone, Debug)]
pub struct DrainPoolSurplusAccounts<'me, 'info> {
    pub creator_wallet: &'me AccountInfo<'info>,
    pub company_wallet: &'me AccountInfo<'info>,
    pub mint_account: &'me AccountInfo<'info>,
    pub liquidity_pool: &'me AccountInfo<'info>,
    pub liquidity_pool_authority: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct DrainPoolSurplusKeys {
    pub creator_wallet: Pubkey,
    pub company_wallet: Pubkey,
    pub mint_account: Pubkey,
    pub liquidity_pool: Pubkey,
    pub liquidity_pool_authority: Pubkey,
    pub system_program: Pubkey,
}
impl From<DrainPoolSurplusAccounts<'_, '_>> for DrainPoolSurplusKeys {
    fn from(accounts: DrainPoolSurplusAccounts) -> Self {
        Self {
            creator_wallet: *accounts.creator_wallet.key,
            company_wallet: *accounts.company_wallet.key,
            mint_account: *accounts.mint_account.key,
            liquidity_pool: *accounts.liquidity_pool.key,
            liquidity_pool_authority: *accounts.liquidity_pool_authority.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<DrainPoolSurplusKeys> for [AccountMeta; DRAIN_POOL_SURPLUS_IX_ACCOUNTS_LEN] {
    fn from(keys: DrainPoolSurplusKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.creator_wallet,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.company_wallet,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.mint_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.liquidity_pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.liquidity_pool_authority,
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
impl From<[Pubkey; DRAIN_POOL_SURPLUS_IX_ACCOUNTS_LEN]> for DrainPoolSurplusKeys {
    fn from(pubkeys: [Pubkey; DRAIN_POOL_SURPLUS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            creator_wallet: pubkeys[0],
            company_wallet: pubkeys[1],
            mint_account: pubkeys[2],
            liquidity_pool: pubkeys[3],
            liquidity_pool_authority: pubkeys[4],
            system_program: pubkeys[5],
        }
    }
}
impl<'info> From<DrainPoolSurplusAccounts<'_, 'info>>
for [AccountInfo<'info>; DRAIN_POOL_SURPLUS_IX_ACCOUNTS_LEN] {
    fn from(accounts: DrainPoolSurplusAccounts<'_, 'info>) -> Self {
        [
            accounts.creator_wallet.clone(),
            accounts.company_wallet.clone(),
            accounts.mint_account.clone(),
            accounts.liquidity_pool.clone(),
            accounts.liquidity_pool_authority.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; DRAIN_POOL_SURPLUS_IX_ACCOUNTS_LEN]>
for DrainPoolSurplusAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; DRAIN_POOL_SURPLUS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            creator_wallet: &arr[0],
            company_wallet: &arr[1],
            mint_account: &arr[2],
            liquidity_pool: &arr[3],
            liquidity_pool_authority: &arr[4],
            system_program: &arr[5],
        }
    }
}
pub const DRAIN_POOL_SURPLUS_IX_DISCM: [u8; 8usize] = [
    172, 123, 233, 0, 98, 159, 144, 191,
];
#[derive(Clone, Debug, PartialEq)]
pub struct DrainPoolSurplusIxData;
impl DrainPoolSurplusIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != DRAIN_POOL_SURPLUS_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&DRAIN_POOL_SURPLUS_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn drain_pool_surplus_ix_with_program_id(
    program_id: Pubkey,
    keys: DrainPoolSurplusKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; DRAIN_POOL_SURPLUS_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: DrainPoolSurplusIxData.try_to_vec()?,
    })
}
pub fn drain_pool_surplus_ix(
    keys: DrainPoolSurplusKeys,
) -> std::io::Result<Instruction> {
    drain_pool_surplus_ix_with_program_id(DUMPFUN_PROGRAM_ID, keys)
}
pub fn drain_pool_surplus_invoke_with_program_id(
    program_id: Pubkey,
    accounts: DrainPoolSurplusAccounts<'_, '_>,
) -> ProgramResult {
    let keys: DrainPoolSurplusKeys = accounts.into();
    let ix = drain_pool_surplus_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn drain_pool_surplus_invoke(
    accounts: DrainPoolSurplusAccounts<'_, '_>,
) -> ProgramResult {
    drain_pool_surplus_invoke_with_program_id(DUMPFUN_PROGRAM_ID, accounts)
}
pub fn drain_pool_surplus_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: DrainPoolSurplusAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: DrainPoolSurplusKeys = accounts.into();
    let ix = drain_pool_surplus_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn drain_pool_surplus_invoke_signed(
    accounts: DrainPoolSurplusAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    drain_pool_surplus_invoke_signed_with_program_id(DUMPFUN_PROGRAM_ID, accounts, seeds)
}
pub fn drain_pool_surplus_verify_account_keys(
    accounts: DrainPoolSurplusAccounts<'_, '_>,
    keys: DrainPoolSurplusKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.creator_wallet.key, keys.creator_wallet),
        (*accounts.company_wallet.key, keys.company_wallet),
        (*accounts.mint_account.key, keys.mint_account),
        (*accounts.liquidity_pool.key, keys.liquidity_pool),
        (*accounts.liquidity_pool_authority.key, keys.liquidity_pool_authority),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn drain_pool_surplus_verify_writable_privileges<'me, 'info>(
    accounts: DrainPoolSurplusAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.creator_wallet,
        accounts.company_wallet,
        accounts.mint_account,
        accounts.liquidity_pool,
        accounts.liquidity_pool_authority,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn drain_pool_surplus_verify_signer_privileges<'me, 'info>(
    accounts: DrainPoolSurplusAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.creator_wallet] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn drain_pool_surplus_verify_account_privileges<'me, 'info>(
    accounts: DrainPoolSurplusAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    drain_pool_surplus_verify_writable_privileges(accounts)?;
    drain_pool_surplus_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const MINT_IX_ACCOUNTS_LEN: usize = 11;
#[derive(Copy, Clone, Debug)]
pub struct MintAccounts<'me, 'info> {
    pub payer: &'me AccountInfo<'info>,
    pub metadata_account: &'me AccountInfo<'info>,
    pub mint_account: &'me AccountInfo<'info>,
    pub liquidity_pool: &'me AccountInfo<'info>,
    pub liquidity_pool_authority: &'me AccountInfo<'info>,
    pub liquidity_pool_token_account: &'me AccountInfo<'info>,
    pub token_metadata_program: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub rent: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MintKeys {
    pub payer: Pubkey,
    pub metadata_account: Pubkey,
    pub mint_account: Pubkey,
    pub liquidity_pool: Pubkey,
    pub liquidity_pool_authority: Pubkey,
    pub liquidity_pool_token_account: Pubkey,
    pub token_metadata_program: Pubkey,
    pub token_program: Pubkey,
    pub associated_token_program: Pubkey,
    pub system_program: Pubkey,
    pub rent: Pubkey,
}
impl From<MintAccounts<'_, '_>> for MintKeys {
    fn from(accounts: MintAccounts) -> Self {
        Self {
            payer: *accounts.payer.key,
            metadata_account: *accounts.metadata_account.key,
            mint_account: *accounts.mint_account.key,
            liquidity_pool: *accounts.liquidity_pool.key,
            liquidity_pool_authority: *accounts.liquidity_pool_authority.key,
            liquidity_pool_token_account: *accounts.liquidity_pool_token_account.key,
            token_metadata_program: *accounts.token_metadata_program.key,
            token_program: *accounts.token_program.key,
            associated_token_program: *accounts.associated_token_program.key,
            system_program: *accounts.system_program.key,
            rent: *accounts.rent.key,
        }
    }
}
impl From<MintKeys> for [AccountMeta; MINT_IX_ACCOUNTS_LEN] {
    fn from(keys: MintKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.payer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.metadata_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.mint_account,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.liquidity_pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.liquidity_pool_authority,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.liquidity_pool_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_metadata_program,
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
            AccountMeta {
                pubkey: keys.rent,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; MINT_IX_ACCOUNTS_LEN]> for MintKeys {
    fn from(pubkeys: [Pubkey; MINT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            payer: pubkeys[0],
            metadata_account: pubkeys[1],
            mint_account: pubkeys[2],
            liquidity_pool: pubkeys[3],
            liquidity_pool_authority: pubkeys[4],
            liquidity_pool_token_account: pubkeys[5],
            token_metadata_program: pubkeys[6],
            token_program: pubkeys[7],
            associated_token_program: pubkeys[8],
            system_program: pubkeys[9],
            rent: pubkeys[10],
        }
    }
}
impl<'info> From<MintAccounts<'_, 'info>>
for [AccountInfo<'info>; MINT_IX_ACCOUNTS_LEN] {
    fn from(accounts: MintAccounts<'_, 'info>) -> Self {
        [
            accounts.payer.clone(),
            accounts.metadata_account.clone(),
            accounts.mint_account.clone(),
            accounts.liquidity_pool.clone(),
            accounts.liquidity_pool_authority.clone(),
            accounts.liquidity_pool_token_account.clone(),
            accounts.token_metadata_program.clone(),
            accounts.token_program.clone(),
            accounts.associated_token_program.clone(),
            accounts.system_program.clone(),
            accounts.rent.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; MINT_IX_ACCOUNTS_LEN]>
for MintAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; MINT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            payer: &arr[0],
            metadata_account: &arr[1],
            mint_account: &arr[2],
            liquidity_pool: &arr[3],
            liquidity_pool_authority: &arr[4],
            liquidity_pool_token_account: &arr[5],
            token_metadata_program: &arr[6],
            token_program: &arr[7],
            associated_token_program: &arr[8],
            system_program: &arr[9],
            rent: &arr[10],
        }
    }
}
pub const MINT_IX_DISCM: [u8; 8usize] = [51, 57, 225, 47, 182, 146, 137, 166];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MintIxArgs {
    pub token_name: String,
    pub token_symbol: String,
    pub token_uri: String,
    pub sell_lock_period: i64,
    pub virtual_sol_reserve: u64,
    pub ramping_limits: Vec<RampingLimit>,
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
        let token_name: String = crate::borsh_de_or_default(&mut reader)?;
        let token_symbol: String = crate::borsh_de_or_default(&mut reader)?;
        let token_uri: String = crate::borsh_de_or_default(&mut reader)?;
        let sell_lock_period: i64 = crate::borsh_de_or_default(&mut reader)?;
        let virtual_sol_reserve: u64 = crate::borsh_de_or_default(&mut reader)?;
        let ramping_limits: Vec<RampingLimit> = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(MintIxArgs {
                token_name,
                token_symbol,
                token_uri,
                sell_lock_period,
                virtual_sol_reserve,
                ramping_limits,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&MINT_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.token_name, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.token_symbol, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.token_uri, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.sell_lock_period, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.virtual_sol_reserve, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.ramping_limits, &mut writer)?;
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
    mint_ix_with_program_id(DUMPFUN_PROGRAM_ID, keys, args)
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
    mint_invoke_with_program_id(DUMPFUN_PROGRAM_ID, accounts, args)
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
    mint_invoke_signed_with_program_id(DUMPFUN_PROGRAM_ID, accounts, args, seeds)
}
pub fn mint_verify_account_keys(
    accounts: MintAccounts<'_, '_>,
    keys: MintKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.payer.key, keys.payer),
        (*accounts.metadata_account.key, keys.metadata_account),
        (*accounts.mint_account.key, keys.mint_account),
        (*accounts.liquidity_pool.key, keys.liquidity_pool),
        (*accounts.liquidity_pool_authority.key, keys.liquidity_pool_authority),
        (*accounts.liquidity_pool_token_account.key, keys.liquidity_pool_token_account),
        (*accounts.token_metadata_program.key, keys.token_metadata_program),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.associated_token_program.key, keys.associated_token_program),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.rent.key, keys.rent),
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
        accounts.payer,
        accounts.metadata_account,
        accounts.mint_account,
        accounts.liquidity_pool,
        accounts.liquidity_pool_authority,
        accounts.liquidity_pool_token_account,
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
    for should_be_signer in [accounts.payer, accounts.mint_account] {
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
pub const SELL_EXACT_TOKENS_IX_ACCOUNTS_LEN: usize = 13;
#[derive(Copy, Clone, Debug)]
pub struct SellExactTokensAccounts<'me, 'info> {
    pub user: &'me AccountInfo<'info>,
    pub platform_fee_wallet: &'me AccountInfo<'info>,
    pub company_tax_wallet: &'me AccountInfo<'info>,
    pub creator_wallet: &'me AccountInfo<'info>,
    pub mint_account: &'me AccountInfo<'info>,
    pub liquidity_pool: &'me AccountInfo<'info>,
    pub liquidity_pool_authority: &'me AccountInfo<'info>,
    pub liquidity_pool_token_account: &'me AccountInfo<'info>,
    pub user_token_account: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub rent: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SellExactTokensKeys {
    pub user: Pubkey,
    pub platform_fee_wallet: Pubkey,
    pub company_tax_wallet: Pubkey,
    pub creator_wallet: Pubkey,
    pub mint_account: Pubkey,
    pub liquidity_pool: Pubkey,
    pub liquidity_pool_authority: Pubkey,
    pub liquidity_pool_token_account: Pubkey,
    pub user_token_account: Pubkey,
    pub token_program: Pubkey,
    pub associated_token_program: Pubkey,
    pub system_program: Pubkey,
    pub rent: Pubkey,
}
impl From<SellExactTokensAccounts<'_, '_>> for SellExactTokensKeys {
    fn from(accounts: SellExactTokensAccounts) -> Self {
        Self {
            user: *accounts.user.key,
            platform_fee_wallet: *accounts.platform_fee_wallet.key,
            company_tax_wallet: *accounts.company_tax_wallet.key,
            creator_wallet: *accounts.creator_wallet.key,
            mint_account: *accounts.mint_account.key,
            liquidity_pool: *accounts.liquidity_pool.key,
            liquidity_pool_authority: *accounts.liquidity_pool_authority.key,
            liquidity_pool_token_account: *accounts.liquidity_pool_token_account.key,
            user_token_account: *accounts.user_token_account.key,
            token_program: *accounts.token_program.key,
            associated_token_program: *accounts.associated_token_program.key,
            system_program: *accounts.system_program.key,
            rent: *accounts.rent.key,
        }
    }
}
impl From<SellExactTokensKeys> for [AccountMeta; SELL_EXACT_TOKENS_IX_ACCOUNTS_LEN] {
    fn from(keys: SellExactTokensKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.user,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.platform_fee_wallet,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.company_tax_wallet,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.creator_wallet,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.mint_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.liquidity_pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.liquidity_pool_authority,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.liquidity_pool_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_token_account,
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
                pubkey: keys.rent,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; SELL_EXACT_TOKENS_IX_ACCOUNTS_LEN]> for SellExactTokensKeys {
    fn from(pubkeys: [Pubkey; SELL_EXACT_TOKENS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            user: pubkeys[0],
            platform_fee_wallet: pubkeys[1],
            company_tax_wallet: pubkeys[2],
            creator_wallet: pubkeys[3],
            mint_account: pubkeys[4],
            liquidity_pool: pubkeys[5],
            liquidity_pool_authority: pubkeys[6],
            liquidity_pool_token_account: pubkeys[7],
            user_token_account: pubkeys[8],
            token_program: pubkeys[9],
            associated_token_program: pubkeys[10],
            system_program: pubkeys[11],
            rent: pubkeys[12],
        }
    }
}
impl<'info> From<SellExactTokensAccounts<'_, 'info>>
for [AccountInfo<'info>; SELL_EXACT_TOKENS_IX_ACCOUNTS_LEN] {
    fn from(accounts: SellExactTokensAccounts<'_, 'info>) -> Self {
        [
            accounts.user.clone(),
            accounts.platform_fee_wallet.clone(),
            accounts.company_tax_wallet.clone(),
            accounts.creator_wallet.clone(),
            accounts.mint_account.clone(),
            accounts.liquidity_pool.clone(),
            accounts.liquidity_pool_authority.clone(),
            accounts.liquidity_pool_token_account.clone(),
            accounts.user_token_account.clone(),
            accounts.token_program.clone(),
            accounts.associated_token_program.clone(),
            accounts.system_program.clone(),
            accounts.rent.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SELL_EXACT_TOKENS_IX_ACCOUNTS_LEN]>
for SellExactTokensAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; SELL_EXACT_TOKENS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            user: &arr[0],
            platform_fee_wallet: &arr[1],
            company_tax_wallet: &arr[2],
            creator_wallet: &arr[3],
            mint_account: &arr[4],
            liquidity_pool: &arr[5],
            liquidity_pool_authority: &arr[6],
            liquidity_pool_token_account: &arr[7],
            user_token_account: &arr[8],
            token_program: &arr[9],
            associated_token_program: &arr[10],
            system_program: &arr[11],
            rent: &arr[12],
        }
    }
}
pub const SELL_EXACT_TOKENS_IX_DISCM: [u8; 8usize] = [34, 198, 103, 105, 28, 0, 138, 92];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SellExactTokensIxArgs {
    pub token_in: u64,
    pub min_sol_out: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct SellExactTokensIxData(pub SellExactTokensIxArgs);
impl From<SellExactTokensIxArgs> for SellExactTokensIxData {
    fn from(args: SellExactTokensIxArgs) -> Self {
        Self(args)
    }
}
impl SellExactTokensIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SELL_EXACT_TOKENS_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let token_in: u64 = crate::borsh_de_or_default(&mut reader)?;
        let min_sol_out: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(SellExactTokensIxArgs {
                token_in,
                min_sol_out,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SELL_EXACT_TOKENS_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.token_in, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.min_sol_out, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn sell_exact_tokens_ix_with_program_id(
    program_id: Pubkey,
    keys: SellExactTokensKeys,
    args: SellExactTokensIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SELL_EXACT_TOKENS_IX_ACCOUNTS_LEN] = keys.into();
    let data: SellExactTokensIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn sell_exact_tokens_ix(
    keys: SellExactTokensKeys,
    args: SellExactTokensIxArgs,
) -> std::io::Result<Instruction> {
    sell_exact_tokens_ix_with_program_id(DUMPFUN_PROGRAM_ID, keys, args)
}
pub fn sell_exact_tokens_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SellExactTokensAccounts<'_, '_>,
    args: SellExactTokensIxArgs,
) -> ProgramResult {
    let keys: SellExactTokensKeys = accounts.into();
    let ix = sell_exact_tokens_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn sell_exact_tokens_invoke(
    accounts: SellExactTokensAccounts<'_, '_>,
    args: SellExactTokensIxArgs,
) -> ProgramResult {
    sell_exact_tokens_invoke_with_program_id(DUMPFUN_PROGRAM_ID, accounts, args)
}
pub fn sell_exact_tokens_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SellExactTokensAccounts<'_, '_>,
    args: SellExactTokensIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SellExactTokensKeys = accounts.into();
    let ix = sell_exact_tokens_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn sell_exact_tokens_invoke_signed(
    accounts: SellExactTokensAccounts<'_, '_>,
    args: SellExactTokensIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    sell_exact_tokens_invoke_signed_with_program_id(
        DUMPFUN_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn sell_exact_tokens_verify_account_keys(
    accounts: SellExactTokensAccounts<'_, '_>,
    keys: SellExactTokensKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.user.key, keys.user),
        (*accounts.platform_fee_wallet.key, keys.platform_fee_wallet),
        (*accounts.company_tax_wallet.key, keys.company_tax_wallet),
        (*accounts.creator_wallet.key, keys.creator_wallet),
        (*accounts.mint_account.key, keys.mint_account),
        (*accounts.liquidity_pool.key, keys.liquidity_pool),
        (*accounts.liquidity_pool_authority.key, keys.liquidity_pool_authority),
        (*accounts.liquidity_pool_token_account.key, keys.liquidity_pool_token_account),
        (*accounts.user_token_account.key, keys.user_token_account),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.associated_token_program.key, keys.associated_token_program),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.rent.key, keys.rent),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn sell_exact_tokens_verify_writable_privileges<'me, 'info>(
    accounts: SellExactTokensAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.user,
        accounts.platform_fee_wallet,
        accounts.company_tax_wallet,
        accounts.creator_wallet,
        accounts.mint_account,
        accounts.liquidity_pool,
        accounts.liquidity_pool_authority,
        accounts.liquidity_pool_token_account,
        accounts.user_token_account,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn sell_exact_tokens_verify_signer_privileges<'me, 'info>(
    accounts: SellExactTokensAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.user] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn sell_exact_tokens_verify_account_privileges<'me, 'info>(
    accounts: SellExactTokensAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    sell_exact_tokens_verify_writable_privileges(accounts)?;
    sell_exact_tokens_verify_signer_privileges(accounts)?;
    Ok(())
}
