use solana_pubkey::Pubkey;
use solana_cpi::{invoke, invoke_signed};
use solana_instruction::{AccountMeta, Instruction};
use solana_account_info::AccountInfo;
use solana_program_error::ProgramError;
use std::io::Read;
#[allow(unused_imports)]
use crate::*;
#[derive(Clone, Debug, PartialEq)]
pub enum GfxSslV2ProgramIx {
    CrankPriceHistories,
    InternalSwap,
    ClaimFees,
    CreateLiquidityAccount,
    CloseLiquidityAccount,
    Deposit(DepositIxArgs),
    Withdraw(WithdrawIxArgs),
    Swap(SwapIxArgs),
}
impl GfxSslV2ProgramIx {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        if buf.starts_with(&CRANK_PRICE_HISTORIES_IX_DISCM) {
            return Ok(Self::CrankPriceHistories);
        }
        if buf.starts_with(&INTERNAL_SWAP_IX_DISCM) {
            return Ok(Self::InternalSwap);
        }
        if buf.starts_with(&CLAIM_FEES_IX_DISCM) {
            return Ok(Self::ClaimFees);
        }
        if buf.starts_with(&CREATE_LIQUIDITY_ACCOUNT_IX_DISCM) {
            return Ok(Self::CreateLiquidityAccount);
        }
        if buf.starts_with(&CLOSE_LIQUIDITY_ACCOUNT_IX_DISCM) {
            return Ok(Self::CloseLiquidityAccount);
        }
        if buf.starts_with(&DEPOSIT_IX_DISCM) {
            let mut reader = &buf[DEPOSIT_IX_DISCM.len()..];
            let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(Self::Deposit(DepositIxArgs { amount }));
        }
        if buf.starts_with(&WITHDRAW_IX_DISCM) {
            let mut reader = &buf[WITHDRAW_IX_DISCM.len()..];
            let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(Self::Withdraw(WithdrawIxArgs { amount }));
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
            Self::CrankPriceHistories => {
                writer.write_all(&CRANK_PRICE_HISTORIES_IX_DISCM)
            }
            Self::InternalSwap => writer.write_all(&INTERNAL_SWAP_IX_DISCM),
            Self::ClaimFees => writer.write_all(&CLAIM_FEES_IX_DISCM),
            Self::CreateLiquidityAccount => {
                writer.write_all(&CREATE_LIQUIDITY_ACCOUNT_IX_DISCM)
            }
            Self::CloseLiquidityAccount => {
                writer.write_all(&CLOSE_LIQUIDITY_ACCOUNT_IX_DISCM)
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
pub const CRANK_PRICE_HISTORIES_IX_ACCOUNTS_LEN: usize = 1;
#[derive(Copy, Clone, Debug)]
pub struct CrankPriceHistoriesAccounts<'me, 'info> {
    pub pool_registry: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CrankPriceHistoriesKeys {
    pub pool_registry: Pubkey,
}
impl From<CrankPriceHistoriesAccounts<'_, '_>> for CrankPriceHistoriesKeys {
    fn from(accounts: CrankPriceHistoriesAccounts) -> Self {
        Self {
            pool_registry: *accounts.pool_registry.key,
        }
    }
}
impl From<CrankPriceHistoriesKeys>
for [AccountMeta; CRANK_PRICE_HISTORIES_IX_ACCOUNTS_LEN] {
    fn from(keys: CrankPriceHistoriesKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.pool_registry,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; CRANK_PRICE_HISTORIES_IX_ACCOUNTS_LEN]> for CrankPriceHistoriesKeys {
    fn from(pubkeys: [Pubkey; CRANK_PRICE_HISTORIES_IX_ACCOUNTS_LEN]) -> Self {
        Self { pool_registry: pubkeys[0] }
    }
}
impl<'info> From<CrankPriceHistoriesAccounts<'_, 'info>>
for [AccountInfo<'info>; CRANK_PRICE_HISTORIES_IX_ACCOUNTS_LEN] {
    fn from(accounts: CrankPriceHistoriesAccounts<'_, 'info>) -> Self {
        [accounts.pool_registry.clone()]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CRANK_PRICE_HISTORIES_IX_ACCOUNTS_LEN]>
for CrankPriceHistoriesAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; CRANK_PRICE_HISTORIES_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self { pool_registry: &arr[0] }
    }
}
pub const CRANK_PRICE_HISTORIES_IX_DISCM: [u8; 8usize] = [
    38, 86, 89, 61, 14, 160, 191, 78,
];
#[derive(Clone, Debug, PartialEq)]
pub struct CrankPriceHistoriesIxData;
impl CrankPriceHistoriesIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CRANK_PRICE_HISTORIES_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CRANK_PRICE_HISTORIES_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn crank_price_histories_ix_with_program_id(
    program_id: Pubkey,
    keys: CrankPriceHistoriesKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CRANK_PRICE_HISTORIES_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: CrankPriceHistoriesIxData.try_to_vec()?,
    })
}
pub fn crank_price_histories_ix(
    keys: CrankPriceHistoriesKeys,
) -> std::io::Result<Instruction> {
    crank_price_histories_ix_with_program_id(GFX_SSL_V2_PROGRAM_ID, keys)
}
pub fn crank_price_histories_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CrankPriceHistoriesAccounts<'_, '_>,
) -> ProgramResult {
    let keys: CrankPriceHistoriesKeys = accounts.into();
    let ix = crank_price_histories_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn crank_price_histories_invoke(
    accounts: CrankPriceHistoriesAccounts<'_, '_>,
) -> ProgramResult {
    crank_price_histories_invoke_with_program_id(GFX_SSL_V2_PROGRAM_ID, accounts)
}
pub fn crank_price_histories_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CrankPriceHistoriesAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CrankPriceHistoriesKeys = accounts.into();
    let ix = crank_price_histories_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn crank_price_histories_invoke_signed(
    accounts: CrankPriceHistoriesAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    crank_price_histories_invoke_signed_with_program_id(
        GFX_SSL_V2_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn crank_price_histories_verify_account_keys(
    accounts: CrankPriceHistoriesAccounts<'_, '_>,
    keys: CrankPriceHistoriesKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [(*accounts.pool_registry.key, keys.pool_registry)] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub const INTERNAL_SWAP_IX_ACCOUNTS_LEN: usize = 14;
#[derive(Copy, Clone, Debug)]
pub struct InternalSwapAccounts<'me, 'info> {
    pub pool_registry: &'me AccountInfo<'info>,
    pub pair: &'me AccountInfo<'info>,
    pub ssl_a_main_token: &'me AccountInfo<'info>,
    pub ssl_b_main_token: &'me AccountInfo<'info>,
    pub ssl_a_secondary_token: &'me AccountInfo<'info>,
    pub ssl_b_secondary_token: &'me AccountInfo<'info>,
    pub token_a_price_history: &'me AccountInfo<'info>,
    pub token_a_oracle: &'me AccountInfo<'info>,
    pub token_b_price_history: &'me AccountInfo<'info>,
    pub token_b_oracle: &'me AccountInfo<'info>,
    pub ssl_pool_a_signer: &'me AccountInfo<'info>,
    pub ssl_pool_b_signer: &'me AccountInfo<'info>,
    pub event_emitter: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct InternalSwapKeys {
    pub pool_registry: Pubkey,
    pub pair: Pubkey,
    pub ssl_a_main_token: Pubkey,
    pub ssl_b_main_token: Pubkey,
    pub ssl_a_secondary_token: Pubkey,
    pub ssl_b_secondary_token: Pubkey,
    pub token_a_price_history: Pubkey,
    pub token_a_oracle: Pubkey,
    pub token_b_price_history: Pubkey,
    pub token_b_oracle: Pubkey,
    pub ssl_pool_a_signer: Pubkey,
    pub ssl_pool_b_signer: Pubkey,
    pub event_emitter: Pubkey,
    pub token_program: Pubkey,
}
impl From<InternalSwapAccounts<'_, '_>> for InternalSwapKeys {
    fn from(accounts: InternalSwapAccounts) -> Self {
        Self {
            pool_registry: *accounts.pool_registry.key,
            pair: *accounts.pair.key,
            ssl_a_main_token: *accounts.ssl_a_main_token.key,
            ssl_b_main_token: *accounts.ssl_b_main_token.key,
            ssl_a_secondary_token: *accounts.ssl_a_secondary_token.key,
            ssl_b_secondary_token: *accounts.ssl_b_secondary_token.key,
            token_a_price_history: *accounts.token_a_price_history.key,
            token_a_oracle: *accounts.token_a_oracle.key,
            token_b_price_history: *accounts.token_b_price_history.key,
            token_b_oracle: *accounts.token_b_oracle.key,
            ssl_pool_a_signer: *accounts.ssl_pool_a_signer.key,
            ssl_pool_b_signer: *accounts.ssl_pool_b_signer.key,
            event_emitter: *accounts.event_emitter.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<InternalSwapKeys> for [AccountMeta; INTERNAL_SWAP_IX_ACCOUNTS_LEN] {
    fn from(keys: InternalSwapKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.pool_registry,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pair,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.ssl_a_main_token,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.ssl_b_main_token,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.ssl_a_secondary_token,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.ssl_b_secondary_token,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_a_price_history,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_a_oracle,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_b_price_history,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_b_oracle,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.ssl_pool_a_signer,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.ssl_pool_b_signer,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.event_emitter,
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
impl From<[Pubkey; INTERNAL_SWAP_IX_ACCOUNTS_LEN]> for InternalSwapKeys {
    fn from(pubkeys: [Pubkey; INTERNAL_SWAP_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            pool_registry: pubkeys[0],
            pair: pubkeys[1],
            ssl_a_main_token: pubkeys[2],
            ssl_b_main_token: pubkeys[3],
            ssl_a_secondary_token: pubkeys[4],
            ssl_b_secondary_token: pubkeys[5],
            token_a_price_history: pubkeys[6],
            token_a_oracle: pubkeys[7],
            token_b_price_history: pubkeys[8],
            token_b_oracle: pubkeys[9],
            ssl_pool_a_signer: pubkeys[10],
            ssl_pool_b_signer: pubkeys[11],
            event_emitter: pubkeys[12],
            token_program: pubkeys[13],
        }
    }
}
impl<'info> From<InternalSwapAccounts<'_, 'info>>
for [AccountInfo<'info>; INTERNAL_SWAP_IX_ACCOUNTS_LEN] {
    fn from(accounts: InternalSwapAccounts<'_, 'info>) -> Self {
        [
            accounts.pool_registry.clone(),
            accounts.pair.clone(),
            accounts.ssl_a_main_token.clone(),
            accounts.ssl_b_main_token.clone(),
            accounts.ssl_a_secondary_token.clone(),
            accounts.ssl_b_secondary_token.clone(),
            accounts.token_a_price_history.clone(),
            accounts.token_a_oracle.clone(),
            accounts.token_b_price_history.clone(),
            accounts.token_b_oracle.clone(),
            accounts.ssl_pool_a_signer.clone(),
            accounts.ssl_pool_b_signer.clone(),
            accounts.event_emitter.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; INTERNAL_SWAP_IX_ACCOUNTS_LEN]>
for InternalSwapAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; INTERNAL_SWAP_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            pool_registry: &arr[0],
            pair: &arr[1],
            ssl_a_main_token: &arr[2],
            ssl_b_main_token: &arr[3],
            ssl_a_secondary_token: &arr[4],
            ssl_b_secondary_token: &arr[5],
            token_a_price_history: &arr[6],
            token_a_oracle: &arr[7],
            token_b_price_history: &arr[8],
            token_b_oracle: &arr[9],
            ssl_pool_a_signer: &arr[10],
            ssl_pool_b_signer: &arr[11],
            event_emitter: &arr[12],
            token_program: &arr[13],
        }
    }
}
pub const INTERNAL_SWAP_IX_DISCM: [u8; 8usize] = [232, 212, 71, 49, 58, 172, 85, 234];
#[derive(Clone, Debug, PartialEq)]
pub struct InternalSwapIxData;
impl InternalSwapIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INTERNAL_SWAP_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INTERNAL_SWAP_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn internal_swap_ix_with_program_id(
    program_id: Pubkey,
    keys: InternalSwapKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; INTERNAL_SWAP_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: InternalSwapIxData.try_to_vec()?,
    })
}
pub fn internal_swap_ix(keys: InternalSwapKeys) -> std::io::Result<Instruction> {
    internal_swap_ix_with_program_id(GFX_SSL_V2_PROGRAM_ID, keys)
}
pub fn internal_swap_invoke_with_program_id(
    program_id: Pubkey,
    accounts: InternalSwapAccounts<'_, '_>,
) -> ProgramResult {
    let keys: InternalSwapKeys = accounts.into();
    let ix = internal_swap_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn internal_swap_invoke(accounts: InternalSwapAccounts<'_, '_>) -> ProgramResult {
    internal_swap_invoke_with_program_id(GFX_SSL_V2_PROGRAM_ID, accounts)
}
pub fn internal_swap_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: InternalSwapAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: InternalSwapKeys = accounts.into();
    let ix = internal_swap_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn internal_swap_invoke_signed(
    accounts: InternalSwapAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    internal_swap_invoke_signed_with_program_id(GFX_SSL_V2_PROGRAM_ID, accounts, seeds)
}
pub fn internal_swap_verify_account_keys(
    accounts: InternalSwapAccounts<'_, '_>,
    keys: InternalSwapKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.pool_registry.key, keys.pool_registry),
        (*accounts.pair.key, keys.pair),
        (*accounts.ssl_a_main_token.key, keys.ssl_a_main_token),
        (*accounts.ssl_b_main_token.key, keys.ssl_b_main_token),
        (*accounts.ssl_a_secondary_token.key, keys.ssl_a_secondary_token),
        (*accounts.ssl_b_secondary_token.key, keys.ssl_b_secondary_token),
        (*accounts.token_a_price_history.key, keys.token_a_price_history),
        (*accounts.token_a_oracle.key, keys.token_a_oracle),
        (*accounts.token_b_price_history.key, keys.token_b_price_history),
        (*accounts.token_b_oracle.key, keys.token_b_oracle),
        (*accounts.ssl_pool_a_signer.key, keys.ssl_pool_a_signer),
        (*accounts.ssl_pool_b_signer.key, keys.ssl_pool_b_signer),
        (*accounts.event_emitter.key, keys.event_emitter),
        (*accounts.token_program.key, keys.token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn internal_swap_verify_writable_privileges<'me, 'info>(
    accounts: InternalSwapAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.pair,
        accounts.ssl_a_main_token,
        accounts.ssl_b_main_token,
        accounts.ssl_a_secondary_token,
        accounts.ssl_b_secondary_token,
        accounts.token_a_price_history,
        accounts.token_b_price_history,
        accounts.event_emitter,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn internal_swap_verify_account_privileges<'me, 'info>(
    accounts: InternalSwapAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    internal_swap_verify_writable_privileges(accounts)?;
    Ok(())
}
pub const CLAIM_FEES_IX_ACCOUNTS_LEN: usize = 7;
#[derive(Copy, Clone, Debug)]
pub struct ClaimFeesAccounts<'me, 'info> {
    pub pool_registry: &'me AccountInfo<'info>,
    pub owner: &'me AccountInfo<'info>,
    pub ssl_fee_vault: &'me AccountInfo<'info>,
    pub owner_ata: &'me AccountInfo<'info>,
    pub liquidity_account: &'me AccountInfo<'info>,
    pub event_emitter: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ClaimFeesKeys {
    pub pool_registry: Pubkey,
    pub owner: Pubkey,
    pub ssl_fee_vault: Pubkey,
    pub owner_ata: Pubkey,
    pub liquidity_account: Pubkey,
    pub event_emitter: Pubkey,
    pub token_program: Pubkey,
}
impl From<ClaimFeesAccounts<'_, '_>> for ClaimFeesKeys {
    fn from(accounts: ClaimFeesAccounts) -> Self {
        Self {
            pool_registry: *accounts.pool_registry.key,
            owner: *accounts.owner.key,
            ssl_fee_vault: *accounts.ssl_fee_vault.key,
            owner_ata: *accounts.owner_ata.key,
            liquidity_account: *accounts.liquidity_account.key,
            event_emitter: *accounts.event_emitter.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<ClaimFeesKeys> for [AccountMeta; CLAIM_FEES_IX_ACCOUNTS_LEN] {
    fn from(keys: ClaimFeesKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.pool_registry,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.owner,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.ssl_fee_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.owner_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.liquidity_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.event_emitter,
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
impl From<[Pubkey; CLAIM_FEES_IX_ACCOUNTS_LEN]> for ClaimFeesKeys {
    fn from(pubkeys: [Pubkey; CLAIM_FEES_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            pool_registry: pubkeys[0],
            owner: pubkeys[1],
            ssl_fee_vault: pubkeys[2],
            owner_ata: pubkeys[3],
            liquidity_account: pubkeys[4],
            event_emitter: pubkeys[5],
            token_program: pubkeys[6],
        }
    }
}
impl<'info> From<ClaimFeesAccounts<'_, 'info>>
for [AccountInfo<'info>; CLAIM_FEES_IX_ACCOUNTS_LEN] {
    fn from(accounts: ClaimFeesAccounts<'_, 'info>) -> Self {
        [
            accounts.pool_registry.clone(),
            accounts.owner.clone(),
            accounts.ssl_fee_vault.clone(),
            accounts.owner_ata.clone(),
            accounts.liquidity_account.clone(),
            accounts.event_emitter.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CLAIM_FEES_IX_ACCOUNTS_LEN]>
for ClaimFeesAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; CLAIM_FEES_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            pool_registry: &arr[0],
            owner: &arr[1],
            ssl_fee_vault: &arr[2],
            owner_ata: &arr[3],
            liquidity_account: &arr[4],
            event_emitter: &arr[5],
            token_program: &arr[6],
        }
    }
}
pub const CLAIM_FEES_IX_DISCM: [u8; 8usize] = [82, 251, 233, 156, 12, 52, 184, 202];
#[derive(Clone, Debug, PartialEq)]
pub struct ClaimFeesIxData;
impl ClaimFeesIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CLAIM_FEES_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CLAIM_FEES_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn claim_fees_ix_with_program_id(
    program_id: Pubkey,
    keys: ClaimFeesKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CLAIM_FEES_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: ClaimFeesIxData.try_to_vec()?,
    })
}
pub fn claim_fees_ix(keys: ClaimFeesKeys) -> std::io::Result<Instruction> {
    claim_fees_ix_with_program_id(GFX_SSL_V2_PROGRAM_ID, keys)
}
pub fn claim_fees_invoke_with_program_id(
    program_id: Pubkey,
    accounts: ClaimFeesAccounts<'_, '_>,
) -> ProgramResult {
    let keys: ClaimFeesKeys = accounts.into();
    let ix = claim_fees_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn claim_fees_invoke(accounts: ClaimFeesAccounts<'_, '_>) -> ProgramResult {
    claim_fees_invoke_with_program_id(GFX_SSL_V2_PROGRAM_ID, accounts)
}
pub fn claim_fees_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: ClaimFeesAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: ClaimFeesKeys = accounts.into();
    let ix = claim_fees_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn claim_fees_invoke_signed(
    accounts: ClaimFeesAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    claim_fees_invoke_signed_with_program_id(GFX_SSL_V2_PROGRAM_ID, accounts, seeds)
}
pub fn claim_fees_verify_account_keys(
    accounts: ClaimFeesAccounts<'_, '_>,
    keys: ClaimFeesKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.pool_registry.key, keys.pool_registry),
        (*accounts.owner.key, keys.owner),
        (*accounts.ssl_fee_vault.key, keys.ssl_fee_vault),
        (*accounts.owner_ata.key, keys.owner_ata),
        (*accounts.liquidity_account.key, keys.liquidity_account),
        (*accounts.event_emitter.key, keys.event_emitter),
        (*accounts.token_program.key, keys.token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn claim_fees_verify_writable_privileges<'me, 'info>(
    accounts: ClaimFeesAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.owner,
        accounts.ssl_fee_vault,
        accounts.owner_ata,
        accounts.liquidity_account,
        accounts.event_emitter,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn claim_fees_verify_signer_privileges<'me, 'info>(
    accounts: ClaimFeesAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.owner] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn claim_fees_verify_account_privileges<'me, 'info>(
    accounts: ClaimFeesAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    claim_fees_verify_writable_privileges(accounts)?;
    claim_fees_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CREATE_LIQUIDITY_ACCOUNT_IX_ACCOUNTS_LEN: usize = 6;
#[derive(Copy, Clone, Debug)]
pub struct CreateLiquidityAccountAccounts<'me, 'info> {
    pub pool_registry: &'me AccountInfo<'info>,
    pub mint: &'me AccountInfo<'info>,
    pub liquidity_account: &'me AccountInfo<'info>,
    pub owner: &'me AccountInfo<'info>,
    pub event_emitter: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CreateLiquidityAccountKeys {
    pub pool_registry: Pubkey,
    pub mint: Pubkey,
    pub liquidity_account: Pubkey,
    pub owner: Pubkey,
    pub event_emitter: Pubkey,
    pub system_program: Pubkey,
}
impl From<CreateLiquidityAccountAccounts<'_, '_>> for CreateLiquidityAccountKeys {
    fn from(accounts: CreateLiquidityAccountAccounts) -> Self {
        Self {
            pool_registry: *accounts.pool_registry.key,
            mint: *accounts.mint.key,
            liquidity_account: *accounts.liquidity_account.key,
            owner: *accounts.owner.key,
            event_emitter: *accounts.event_emitter.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<CreateLiquidityAccountKeys>
for [AccountMeta; CREATE_LIQUIDITY_ACCOUNT_IX_ACCOUNTS_LEN] {
    fn from(keys: CreateLiquidityAccountKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.pool_registry,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.liquidity_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.owner,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.event_emitter,
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
impl From<[Pubkey; CREATE_LIQUIDITY_ACCOUNT_IX_ACCOUNTS_LEN]>
for CreateLiquidityAccountKeys {
    fn from(pubkeys: [Pubkey; CREATE_LIQUIDITY_ACCOUNT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            pool_registry: pubkeys[0],
            mint: pubkeys[1],
            liquidity_account: pubkeys[2],
            owner: pubkeys[3],
            event_emitter: pubkeys[4],
            system_program: pubkeys[5],
        }
    }
}
impl<'info> From<CreateLiquidityAccountAccounts<'_, 'info>>
for [AccountInfo<'info>; CREATE_LIQUIDITY_ACCOUNT_IX_ACCOUNTS_LEN] {
    fn from(accounts: CreateLiquidityAccountAccounts<'_, 'info>) -> Self {
        [
            accounts.pool_registry.clone(),
            accounts.mint.clone(),
            accounts.liquidity_account.clone(),
            accounts.owner.clone(),
            accounts.event_emitter.clone(),
            accounts.system_program.clone(),
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
            pool_registry: &arr[0],
            mint: &arr[1],
            liquidity_account: &arr[2],
            owner: &arr[3],
            event_emitter: &arr[4],
            system_program: &arr[5],
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
    create_liquidity_account_ix_with_program_id(GFX_SSL_V2_PROGRAM_ID, keys)
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
    create_liquidity_account_invoke_with_program_id(GFX_SSL_V2_PROGRAM_ID, accounts)
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
        GFX_SSL_V2_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn create_liquidity_account_verify_account_keys(
    accounts: CreateLiquidityAccountAccounts<'_, '_>,
    keys: CreateLiquidityAccountKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.pool_registry.key, keys.pool_registry),
        (*accounts.mint.key, keys.mint),
        (*accounts.liquidity_account.key, keys.liquidity_account),
        (*accounts.owner.key, keys.owner),
        (*accounts.event_emitter.key, keys.event_emitter),
        (*accounts.system_program.key, keys.system_program),
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
        accounts.liquidity_account,
        accounts.owner,
        accounts.event_emitter,
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
    for should_be_signer in [accounts.owner] {
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
pub const CLOSE_LIQUIDITY_ACCOUNT_IX_ACCOUNTS_LEN: usize = 5;
#[derive(Copy, Clone, Debug)]
pub struct CloseLiquidityAccountAccounts<'me, 'info> {
    pub owner: &'me AccountInfo<'info>,
    pub rent_recipient: &'me AccountInfo<'info>,
    pub liquidity_account: &'me AccountInfo<'info>,
    pub event_emitter: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CloseLiquidityAccountKeys {
    pub owner: Pubkey,
    pub rent_recipient: Pubkey,
    pub liquidity_account: Pubkey,
    pub event_emitter: Pubkey,
    pub system_program: Pubkey,
}
impl From<CloseLiquidityAccountAccounts<'_, '_>> for CloseLiquidityAccountKeys {
    fn from(accounts: CloseLiquidityAccountAccounts) -> Self {
        Self {
            owner: *accounts.owner.key,
            rent_recipient: *accounts.rent_recipient.key,
            liquidity_account: *accounts.liquidity_account.key,
            event_emitter: *accounts.event_emitter.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<CloseLiquidityAccountKeys>
for [AccountMeta; CLOSE_LIQUIDITY_ACCOUNT_IX_ACCOUNTS_LEN] {
    fn from(keys: CloseLiquidityAccountKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.owner,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.rent_recipient,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.liquidity_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.event_emitter,
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
impl From<[Pubkey; CLOSE_LIQUIDITY_ACCOUNT_IX_ACCOUNTS_LEN]>
for CloseLiquidityAccountKeys {
    fn from(pubkeys: [Pubkey; CLOSE_LIQUIDITY_ACCOUNT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            owner: pubkeys[0],
            rent_recipient: pubkeys[1],
            liquidity_account: pubkeys[2],
            event_emitter: pubkeys[3],
            system_program: pubkeys[4],
        }
    }
}
impl<'info> From<CloseLiquidityAccountAccounts<'_, 'info>>
for [AccountInfo<'info>; CLOSE_LIQUIDITY_ACCOUNT_IX_ACCOUNTS_LEN] {
    fn from(accounts: CloseLiquidityAccountAccounts<'_, 'info>) -> Self {
        [
            accounts.owner.clone(),
            accounts.rent_recipient.clone(),
            accounts.liquidity_account.clone(),
            accounts.event_emitter.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CLOSE_LIQUIDITY_ACCOUNT_IX_ACCOUNTS_LEN]>
for CloseLiquidityAccountAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; CLOSE_LIQUIDITY_ACCOUNT_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            owner: &arr[0],
            rent_recipient: &arr[1],
            liquidity_account: &arr[2],
            event_emitter: &arr[3],
            system_program: &arr[4],
        }
    }
}
pub const CLOSE_LIQUIDITY_ACCOUNT_IX_DISCM: [u8; 8usize] = [
    175, 243, 103, 208, 220, 169, 43, 193,
];
#[derive(Clone, Debug, PartialEq)]
pub struct CloseLiquidityAccountIxData;
impl CloseLiquidityAccountIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CLOSE_LIQUIDITY_ACCOUNT_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CLOSE_LIQUIDITY_ACCOUNT_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn close_liquidity_account_ix_with_program_id(
    program_id: Pubkey,
    keys: CloseLiquidityAccountKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CLOSE_LIQUIDITY_ACCOUNT_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: CloseLiquidityAccountIxData.try_to_vec()?,
    })
}
pub fn close_liquidity_account_ix(
    keys: CloseLiquidityAccountKeys,
) -> std::io::Result<Instruction> {
    close_liquidity_account_ix_with_program_id(GFX_SSL_V2_PROGRAM_ID, keys)
}
pub fn close_liquidity_account_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CloseLiquidityAccountAccounts<'_, '_>,
) -> ProgramResult {
    let keys: CloseLiquidityAccountKeys = accounts.into();
    let ix = close_liquidity_account_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn close_liquidity_account_invoke(
    accounts: CloseLiquidityAccountAccounts<'_, '_>,
) -> ProgramResult {
    close_liquidity_account_invoke_with_program_id(GFX_SSL_V2_PROGRAM_ID, accounts)
}
pub fn close_liquidity_account_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CloseLiquidityAccountAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CloseLiquidityAccountKeys = accounts.into();
    let ix = close_liquidity_account_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn close_liquidity_account_invoke_signed(
    accounts: CloseLiquidityAccountAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    close_liquidity_account_invoke_signed_with_program_id(
        GFX_SSL_V2_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn close_liquidity_account_verify_account_keys(
    accounts: CloseLiquidityAccountAccounts<'_, '_>,
    keys: CloseLiquidityAccountKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.owner.key, keys.owner),
        (*accounts.rent_recipient.key, keys.rent_recipient),
        (*accounts.liquidity_account.key, keys.liquidity_account),
        (*accounts.event_emitter.key, keys.event_emitter),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn close_liquidity_account_verify_writable_privileges<'me, 'info>(
    accounts: CloseLiquidityAccountAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.owner,
        accounts.rent_recipient,
        accounts.liquidity_account,
        accounts.event_emitter,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn close_liquidity_account_verify_signer_privileges<'me, 'info>(
    accounts: CloseLiquidityAccountAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.owner] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn close_liquidity_account_verify_account_privileges<'me, 'info>(
    accounts: CloseLiquidityAccountAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    close_liquidity_account_verify_writable_privileges(accounts)?;
    close_liquidity_account_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const DEPOSIT_IX_ACCOUNTS_LEN: usize = 9;
#[derive(Copy, Clone, Debug)]
pub struct DepositAccounts<'me, 'info> {
    pub liquidity_account: &'me AccountInfo<'info>,
    pub owner: &'me AccountInfo<'info>,
    pub user_ata: &'me AccountInfo<'info>,
    pub ssl_pool_signer: &'me AccountInfo<'info>,
    pub pool_vault: &'me AccountInfo<'info>,
    pub ssl_fee_vault: &'me AccountInfo<'info>,
    pub pool_registry: &'me AccountInfo<'info>,
    pub event_emitter: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct DepositKeys {
    pub liquidity_account: Pubkey,
    pub owner: Pubkey,
    pub user_ata: Pubkey,
    pub ssl_pool_signer: Pubkey,
    pub pool_vault: Pubkey,
    pub ssl_fee_vault: Pubkey,
    pub pool_registry: Pubkey,
    pub event_emitter: Pubkey,
    pub token_program: Pubkey,
}
impl From<DepositAccounts<'_, '_>> for DepositKeys {
    fn from(accounts: DepositAccounts) -> Self {
        Self {
            liquidity_account: *accounts.liquidity_account.key,
            owner: *accounts.owner.key,
            user_ata: *accounts.user_ata.key,
            ssl_pool_signer: *accounts.ssl_pool_signer.key,
            pool_vault: *accounts.pool_vault.key,
            ssl_fee_vault: *accounts.ssl_fee_vault.key,
            pool_registry: *accounts.pool_registry.key,
            event_emitter: *accounts.event_emitter.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<DepositKeys> for [AccountMeta; DEPOSIT_IX_ACCOUNTS_LEN] {
    fn from(keys: DepositKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.liquidity_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.owner,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.user_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.ssl_pool_signer,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.ssl_fee_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pool_registry,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.event_emitter,
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
            liquidity_account: pubkeys[0],
            owner: pubkeys[1],
            user_ata: pubkeys[2],
            ssl_pool_signer: pubkeys[3],
            pool_vault: pubkeys[4],
            ssl_fee_vault: pubkeys[5],
            pool_registry: pubkeys[6],
            event_emitter: pubkeys[7],
            token_program: pubkeys[8],
        }
    }
}
impl<'info> From<DepositAccounts<'_, 'info>>
for [AccountInfo<'info>; DEPOSIT_IX_ACCOUNTS_LEN] {
    fn from(accounts: DepositAccounts<'_, 'info>) -> Self {
        [
            accounts.liquidity_account.clone(),
            accounts.owner.clone(),
            accounts.user_ata.clone(),
            accounts.ssl_pool_signer.clone(),
            accounts.pool_vault.clone(),
            accounts.ssl_fee_vault.clone(),
            accounts.pool_registry.clone(),
            accounts.event_emitter.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; DEPOSIT_IX_ACCOUNTS_LEN]>
for DepositAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; DEPOSIT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            liquidity_account: &arr[0],
            owner: &arr[1],
            user_ata: &arr[2],
            ssl_pool_signer: &arr[3],
            pool_vault: &arr[4],
            ssl_fee_vault: &arr[5],
            pool_registry: &arr[6],
            event_emitter: &arr[7],
            token_program: &arr[8],
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
    deposit_ix_with_program_id(GFX_SSL_V2_PROGRAM_ID, keys, args)
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
    deposit_invoke_with_program_id(GFX_SSL_V2_PROGRAM_ID, accounts, args)
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
    deposit_invoke_signed_with_program_id(GFX_SSL_V2_PROGRAM_ID, accounts, args, seeds)
}
pub fn deposit_verify_account_keys(
    accounts: DepositAccounts<'_, '_>,
    keys: DepositKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.liquidity_account.key, keys.liquidity_account),
        (*accounts.owner.key, keys.owner),
        (*accounts.user_ata.key, keys.user_ata),
        (*accounts.ssl_pool_signer.key, keys.ssl_pool_signer),
        (*accounts.pool_vault.key, keys.pool_vault),
        (*accounts.ssl_fee_vault.key, keys.ssl_fee_vault),
        (*accounts.pool_registry.key, keys.pool_registry),
        (*accounts.event_emitter.key, keys.event_emitter),
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
        accounts.liquidity_account,
        accounts.user_ata,
        accounts.pool_vault,
        accounts.ssl_fee_vault,
        accounts.pool_registry,
        accounts.event_emitter,
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
pub const WITHDRAW_IX_ACCOUNTS_LEN: usize = 9;
#[derive(Copy, Clone, Debug)]
pub struct WithdrawAccounts<'me, 'info> {
    pub liquidity_account: &'me AccountInfo<'info>,
    pub owner: &'me AccountInfo<'info>,
    pub user_ata: &'me AccountInfo<'info>,
    pub ssl_pool_signer: &'me AccountInfo<'info>,
    pub pool_vault: &'me AccountInfo<'info>,
    pub ssl_fee_vault: &'me AccountInfo<'info>,
    pub pool_registry: &'me AccountInfo<'info>,
    pub event_emitter: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct WithdrawKeys {
    pub liquidity_account: Pubkey,
    pub owner: Pubkey,
    pub user_ata: Pubkey,
    pub ssl_pool_signer: Pubkey,
    pub pool_vault: Pubkey,
    pub ssl_fee_vault: Pubkey,
    pub pool_registry: Pubkey,
    pub event_emitter: Pubkey,
    pub token_program: Pubkey,
}
impl From<WithdrawAccounts<'_, '_>> for WithdrawKeys {
    fn from(accounts: WithdrawAccounts) -> Self {
        Self {
            liquidity_account: *accounts.liquidity_account.key,
            owner: *accounts.owner.key,
            user_ata: *accounts.user_ata.key,
            ssl_pool_signer: *accounts.ssl_pool_signer.key,
            pool_vault: *accounts.pool_vault.key,
            ssl_fee_vault: *accounts.ssl_fee_vault.key,
            pool_registry: *accounts.pool_registry.key,
            event_emitter: *accounts.event_emitter.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<WithdrawKeys> for [AccountMeta; WITHDRAW_IX_ACCOUNTS_LEN] {
    fn from(keys: WithdrawKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.liquidity_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.owner,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.user_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.ssl_pool_signer,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.ssl_fee_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pool_registry,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.event_emitter,
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
            liquidity_account: pubkeys[0],
            owner: pubkeys[1],
            user_ata: pubkeys[2],
            ssl_pool_signer: pubkeys[3],
            pool_vault: pubkeys[4],
            ssl_fee_vault: pubkeys[5],
            pool_registry: pubkeys[6],
            event_emitter: pubkeys[7],
            token_program: pubkeys[8],
        }
    }
}
impl<'info> From<WithdrawAccounts<'_, 'info>>
for [AccountInfo<'info>; WITHDRAW_IX_ACCOUNTS_LEN] {
    fn from(accounts: WithdrawAccounts<'_, 'info>) -> Self {
        [
            accounts.liquidity_account.clone(),
            accounts.owner.clone(),
            accounts.user_ata.clone(),
            accounts.ssl_pool_signer.clone(),
            accounts.pool_vault.clone(),
            accounts.ssl_fee_vault.clone(),
            accounts.pool_registry.clone(),
            accounts.event_emitter.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; WITHDRAW_IX_ACCOUNTS_LEN]>
for WithdrawAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; WITHDRAW_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            liquidity_account: &arr[0],
            owner: &arr[1],
            user_ata: &arr[2],
            ssl_pool_signer: &arr[3],
            pool_vault: &arr[4],
            ssl_fee_vault: &arr[5],
            pool_registry: &arr[6],
            event_emitter: &arr[7],
            token_program: &arr[8],
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
    withdraw_ix_with_program_id(GFX_SSL_V2_PROGRAM_ID, keys, args)
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
    withdraw_invoke_with_program_id(GFX_SSL_V2_PROGRAM_ID, accounts, args)
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
    withdraw_invoke_signed_with_program_id(GFX_SSL_V2_PROGRAM_ID, accounts, args, seeds)
}
pub fn withdraw_verify_account_keys(
    accounts: WithdrawAccounts<'_, '_>,
    keys: WithdrawKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.liquidity_account.key, keys.liquidity_account),
        (*accounts.owner.key, keys.owner),
        (*accounts.user_ata.key, keys.user_ata),
        (*accounts.ssl_pool_signer.key, keys.ssl_pool_signer),
        (*accounts.pool_vault.key, keys.pool_vault),
        (*accounts.ssl_fee_vault.key, keys.ssl_fee_vault),
        (*accounts.pool_registry.key, keys.pool_registry),
        (*accounts.event_emitter.key, keys.event_emitter),
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
        accounts.liquidity_account,
        accounts.user_ata,
        accounts.pool_vault,
        accounts.ssl_fee_vault,
        accounts.pool_registry,
        accounts.event_emitter,
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
pub const SWAP_IX_ACCOUNTS_LEN: usize = 20;
#[derive(Copy, Clone, Debug)]
pub struct SwapAccounts<'me, 'info> {
    pub pair: &'me AccountInfo<'info>,
    pub pool_registry: &'me AccountInfo<'info>,
    pub user_wallet_0: &'me AccountInfo<'info>,
    pub user_wallet_1: &'me AccountInfo<'info>,
    pub ssl_pool_in_signer: &'me AccountInfo<'info>,
    pub ssl_pool_out_signer: &'me AccountInfo<'info>,
    pub user_ata_in: &'me AccountInfo<'info>,
    pub user_ata_out: &'me AccountInfo<'info>,
    pub ssl_out_main_vault: &'me AccountInfo<'info>,
    pub ssl_out_secondary_vault: &'me AccountInfo<'info>,
    pub ssl_in_main_vault: &'me AccountInfo<'info>,
    pub ssl_in_secondary_vault: &'me AccountInfo<'info>,
    pub ssl_out_fee_vault: &'me AccountInfo<'info>,
    pub ssl_out_fee_destination: &'me AccountInfo<'info>,
    pub output_token_price_history: &'me AccountInfo<'info>,
    pub output_token_oracle: &'me AccountInfo<'info>,
    pub input_token_price_history: &'me AccountInfo<'info>,
    pub input_token_oracle: &'me AccountInfo<'info>,
    pub event_emitter: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SwapKeys {
    pub pair: Pubkey,
    pub pool_registry: Pubkey,
    pub user_wallet_0: Pubkey,
    pub user_wallet_1: Pubkey,
    pub ssl_pool_in_signer: Pubkey,
    pub ssl_pool_out_signer: Pubkey,
    pub user_ata_in: Pubkey,
    pub user_ata_out: Pubkey,
    pub ssl_out_main_vault: Pubkey,
    pub ssl_out_secondary_vault: Pubkey,
    pub ssl_in_main_vault: Pubkey,
    pub ssl_in_secondary_vault: Pubkey,
    pub ssl_out_fee_vault: Pubkey,
    pub ssl_out_fee_destination: Pubkey,
    pub output_token_price_history: Pubkey,
    pub output_token_oracle: Pubkey,
    pub input_token_price_history: Pubkey,
    pub input_token_oracle: Pubkey,
    pub event_emitter: Pubkey,
    pub token_program: Pubkey,
}
impl From<SwapAccounts<'_, '_>> for SwapKeys {
    fn from(accounts: SwapAccounts) -> Self {
        Self {
            pair: *accounts.pair.key,
            pool_registry: *accounts.pool_registry.key,
            user_wallet_0: *accounts.user_wallet_0.key,
            user_wallet_1: *accounts.user_wallet_1.key,
            ssl_pool_in_signer: *accounts.ssl_pool_in_signer.key,
            ssl_pool_out_signer: *accounts.ssl_pool_out_signer.key,
            user_ata_in: *accounts.user_ata_in.key,
            user_ata_out: *accounts.user_ata_out.key,
            ssl_out_main_vault: *accounts.ssl_out_main_vault.key,
            ssl_out_secondary_vault: *accounts.ssl_out_secondary_vault.key,
            ssl_in_main_vault: *accounts.ssl_in_main_vault.key,
            ssl_in_secondary_vault: *accounts.ssl_in_secondary_vault.key,
            ssl_out_fee_vault: *accounts.ssl_out_fee_vault.key,
            ssl_out_fee_destination: *accounts.ssl_out_fee_destination.key,
            output_token_price_history: *accounts.output_token_price_history.key,
            output_token_oracle: *accounts.output_token_oracle.key,
            input_token_price_history: *accounts.input_token_price_history.key,
            input_token_oracle: *accounts.input_token_oracle.key,
            event_emitter: *accounts.event_emitter.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<SwapKeys> for [AccountMeta; SWAP_IX_ACCOUNTS_LEN] {
    fn from(keys: SwapKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.pair,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pool_registry,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_wallet_0,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.user_wallet_1,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.ssl_pool_in_signer,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.ssl_pool_out_signer,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.user_ata_in,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_ata_out,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.ssl_out_main_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.ssl_out_secondary_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.ssl_in_main_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.ssl_in_secondary_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.ssl_out_fee_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.ssl_out_fee_destination,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.output_token_price_history,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.output_token_oracle,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.input_token_price_history,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.input_token_oracle,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.event_emitter,
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
impl From<[Pubkey; SWAP_IX_ACCOUNTS_LEN]> for SwapKeys {
    fn from(pubkeys: [Pubkey; SWAP_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            pair: pubkeys[0],
            pool_registry: pubkeys[1],
            user_wallet_0: pubkeys[2],
            user_wallet_1: pubkeys[3],
            ssl_pool_in_signer: pubkeys[4],
            ssl_pool_out_signer: pubkeys[5],
            user_ata_in: pubkeys[6],
            user_ata_out: pubkeys[7],
            ssl_out_main_vault: pubkeys[8],
            ssl_out_secondary_vault: pubkeys[9],
            ssl_in_main_vault: pubkeys[10],
            ssl_in_secondary_vault: pubkeys[11],
            ssl_out_fee_vault: pubkeys[12],
            ssl_out_fee_destination: pubkeys[13],
            output_token_price_history: pubkeys[14],
            output_token_oracle: pubkeys[15],
            input_token_price_history: pubkeys[16],
            input_token_oracle: pubkeys[17],
            event_emitter: pubkeys[18],
            token_program: pubkeys[19],
        }
    }
}
impl<'info> From<SwapAccounts<'_, 'info>>
for [AccountInfo<'info>; SWAP_IX_ACCOUNTS_LEN] {
    fn from(accounts: SwapAccounts<'_, 'info>) -> Self {
        [
            accounts.pair.clone(),
            accounts.pool_registry.clone(),
            accounts.user_wallet_0.clone(),
            accounts.user_wallet_1.clone(),
            accounts.ssl_pool_in_signer.clone(),
            accounts.ssl_pool_out_signer.clone(),
            accounts.user_ata_in.clone(),
            accounts.user_ata_out.clone(),
            accounts.ssl_out_main_vault.clone(),
            accounts.ssl_out_secondary_vault.clone(),
            accounts.ssl_in_main_vault.clone(),
            accounts.ssl_in_secondary_vault.clone(),
            accounts.ssl_out_fee_vault.clone(),
            accounts.ssl_out_fee_destination.clone(),
            accounts.output_token_price_history.clone(),
            accounts.output_token_oracle.clone(),
            accounts.input_token_price_history.clone(),
            accounts.input_token_oracle.clone(),
            accounts.event_emitter.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SWAP_IX_ACCOUNTS_LEN]>
for SwapAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; SWAP_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            pair: &arr[0],
            pool_registry: &arr[1],
            user_wallet_0: &arr[2],
            user_wallet_1: &arr[3],
            ssl_pool_in_signer: &arr[4],
            ssl_pool_out_signer: &arr[5],
            user_ata_in: &arr[6],
            user_ata_out: &arr[7],
            ssl_out_main_vault: &arr[8],
            ssl_out_secondary_vault: &arr[9],
            ssl_in_main_vault: &arr[10],
            ssl_in_secondary_vault: &arr[11],
            ssl_out_fee_vault: &arr[12],
            ssl_out_fee_destination: &arr[13],
            output_token_price_history: &arr[14],
            output_token_oracle: &arr[15],
            input_token_price_history: &arr[16],
            input_token_oracle: &arr[17],
            event_emitter: &arr[18],
            token_program: &arr[19],
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
    swap_ix_with_program_id(GFX_SSL_V2_PROGRAM_ID, keys, args)
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
    swap_invoke_with_program_id(GFX_SSL_V2_PROGRAM_ID, accounts, args)
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
    swap_invoke_signed_with_program_id(GFX_SSL_V2_PROGRAM_ID, accounts, args, seeds)
}
pub fn swap_verify_account_keys(
    accounts: SwapAccounts<'_, '_>,
    keys: SwapKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.pair.key, keys.pair),
        (*accounts.pool_registry.key, keys.pool_registry),
        (*accounts.user_wallet_0.key, keys.user_wallet_0),
        (*accounts.user_wallet_1.key, keys.user_wallet_1),
        (*accounts.ssl_pool_in_signer.key, keys.ssl_pool_in_signer),
        (*accounts.ssl_pool_out_signer.key, keys.ssl_pool_out_signer),
        (*accounts.user_ata_in.key, keys.user_ata_in),
        (*accounts.user_ata_out.key, keys.user_ata_out),
        (*accounts.ssl_out_main_vault.key, keys.ssl_out_main_vault),
        (*accounts.ssl_out_secondary_vault.key, keys.ssl_out_secondary_vault),
        (*accounts.ssl_in_main_vault.key, keys.ssl_in_main_vault),
        (*accounts.ssl_in_secondary_vault.key, keys.ssl_in_secondary_vault),
        (*accounts.ssl_out_fee_vault.key, keys.ssl_out_fee_vault),
        (*accounts.ssl_out_fee_destination.key, keys.ssl_out_fee_destination),
        (*accounts.output_token_price_history.key, keys.output_token_price_history),
        (*accounts.output_token_oracle.key, keys.output_token_oracle),
        (*accounts.input_token_price_history.key, keys.input_token_price_history),
        (*accounts.input_token_oracle.key, keys.input_token_oracle),
        (*accounts.event_emitter.key, keys.event_emitter),
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
        accounts.pool_registry,
        accounts.user_ata_in,
        accounts.user_ata_out,
        accounts.ssl_out_main_vault,
        accounts.ssl_out_secondary_vault,
        accounts.ssl_in_main_vault,
        accounts.ssl_in_secondary_vault,
        accounts.ssl_out_fee_vault,
        accounts.ssl_out_fee_destination,
        accounts.output_token_price_history,
        accounts.input_token_price_history,
        accounts.event_emitter,
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
    for should_be_signer in [accounts.user_wallet_1] {
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
