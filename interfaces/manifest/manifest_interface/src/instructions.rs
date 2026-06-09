use borsh::{BorshDeserialize, BorshSerialize};
use solana_pubkey::Pubkey;
use solana_cpi::{invoke, invoke_signed};
use solana_instruction::{AccountMeta, Instruction};
use solana_account_info::AccountInfo;
use solana_program_error::ProgramError;
use std::io::Read;
#[allow(unused_imports)]
use crate::*;
#[derive(Clone, Debug, PartialEq)]
pub enum ManifestProgramIx {
    CreateMarket,
    ClaimSeat,
    Deposit(DepositIxArgs),
    Withdraw(WithdrawIxArgs),
    Swap(SwapIxArgs),
    SwapV2(SwapV2IxArgs),
    Expand,
    BatchUpdate(BatchUpdateIxArgs),
    GlobalCreate,
    GlobalAddTrader,
    GlobalDeposit(GlobalDepositIxArgs),
    GlobalWithdraw(GlobalWithdrawIxArgs),
    GlobalEvict(GlobalEvictIxArgs),
    GlobalClean(GlobalCleanIxArgs),
}
impl ManifestProgramIx {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm_buf = [0u8; 1];
        reader.read_exact(&mut maybe_discm_buf)?;
        let maybe_discm = maybe_discm_buf[0];
        match maybe_discm {
            CREATE_MARKET_IX_DISCM => Ok(Self::CreateMarket),
            CLAIM_SEAT_IX_DISCM => Ok(Self::ClaimSeat),
            DEPOSIT_IX_DISCM => {
                Ok(Self::Deposit(DepositIxArgs::deserialize(&mut reader)?))
            }
            WITHDRAW_IX_DISCM => {
                Ok(Self::Withdraw(WithdrawIxArgs::deserialize(&mut reader)?))
            }
            SWAP_IX_DISCM => Ok(Self::Swap(SwapIxArgs::deserialize(&mut reader)?)),
            SWAP_V2_IX_DISCM => Ok(Self::SwapV2(SwapV2IxArgs::deserialize(&mut reader)?)),
            EXPAND_IX_DISCM => Ok(Self::Expand),
            BATCH_UPDATE_IX_DISCM => {
                Ok(Self::BatchUpdate(BatchUpdateIxArgs::deserialize(&mut reader)?))
            }
            GLOBAL_CREATE_IX_DISCM => Ok(Self::GlobalCreate),
            GLOBAL_ADD_TRADER_IX_DISCM => Ok(Self::GlobalAddTrader),
            GLOBAL_DEPOSIT_IX_DISCM => {
                Ok(Self::GlobalDeposit(GlobalDepositIxArgs::deserialize(&mut reader)?))
            }
            GLOBAL_WITHDRAW_IX_DISCM => {
                Ok(Self::GlobalWithdraw(GlobalWithdrawIxArgs::deserialize(&mut reader)?))
            }
            GLOBAL_EVICT_IX_DISCM => {
                Ok(Self::GlobalEvict(GlobalEvictIxArgs::deserialize(&mut reader)?))
            }
            GLOBAL_CLEAN_IX_DISCM => {
                Ok(Self::GlobalClean(GlobalCleanIxArgs::deserialize(&mut reader)?))
            }
            _ => Err(std::io::Error::from(std::io::ErrorKind::InvalidData)),
        }
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        match self {
            Self::CreateMarket => writer.write_all(&[CREATE_MARKET_IX_DISCM]),
            Self::ClaimSeat => writer.write_all(&[CLAIM_SEAT_IX_DISCM]),
            Self::Deposit(args) => {
                writer.write_all(&[DEPOSIT_IX_DISCM])?;
                args.serialize(&mut writer)
            }
            Self::Withdraw(args) => {
                writer.write_all(&[WITHDRAW_IX_DISCM])?;
                args.serialize(&mut writer)
            }
            Self::Swap(args) => {
                writer.write_all(&[SWAP_IX_DISCM])?;
                args.serialize(&mut writer)
            }
            Self::SwapV2(args) => {
                writer.write_all(&[SWAP_V2_IX_DISCM])?;
                args.serialize(&mut writer)
            }
            Self::Expand => writer.write_all(&[EXPAND_IX_DISCM]),
            Self::BatchUpdate(args) => {
                writer.write_all(&[BATCH_UPDATE_IX_DISCM])?;
                args.serialize(&mut writer)
            }
            Self::GlobalCreate => writer.write_all(&[GLOBAL_CREATE_IX_DISCM]),
            Self::GlobalAddTrader => writer.write_all(&[GLOBAL_ADD_TRADER_IX_DISCM]),
            Self::GlobalDeposit(args) => {
                writer.write_all(&[GLOBAL_DEPOSIT_IX_DISCM])?;
                args.serialize(&mut writer)
            }
            Self::GlobalWithdraw(args) => {
                writer.write_all(&[GLOBAL_WITHDRAW_IX_DISCM])?;
                args.serialize(&mut writer)
            }
            Self::GlobalEvict(args) => {
                writer.write_all(&[GLOBAL_EVICT_IX_DISCM])?;
                args.serialize(&mut writer)
            }
            Self::GlobalClean(args) => {
                writer.write_all(&[GLOBAL_CLEAN_IX_DISCM])?;
                args.serialize(&mut writer)
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
pub const CREATE_MARKET_IX_ACCOUNTS_LEN: usize = 9;
#[derive(Copy, Clone, Debug)]
pub struct CreateMarketAccounts<'me, 'info> {
    pub payer: &'me AccountInfo<'info>,
    pub market: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub base_mint: &'me AccountInfo<'info>,
    pub quote_mint: &'me AccountInfo<'info>,
    pub base_vault: &'me AccountInfo<'info>,
    pub quote_vault: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub token_program22: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct CreateMarketKeys {
    pub payer: Pubkey,
    pub market: Pubkey,
    pub system_program: Pubkey,
    pub base_mint: Pubkey,
    pub quote_mint: Pubkey,
    pub base_vault: Pubkey,
    pub quote_vault: Pubkey,
    pub token_program: Pubkey,
    pub token_program22: Pubkey,
}
impl From<CreateMarketAccounts<'_, '_>> for CreateMarketKeys {
    fn from(accounts: CreateMarketAccounts) -> Self {
        Self {
            payer: *accounts.payer.key,
            market: *accounts.market.key,
            system_program: *accounts.system_program.key,
            base_mint: *accounts.base_mint.key,
            quote_mint: *accounts.quote_mint.key,
            base_vault: *accounts.base_vault.key,
            quote_vault: *accounts.quote_vault.key,
            token_program: *accounts.token_program.key,
            token_program22: *accounts.token_program22.key,
        }
    }
}
impl From<CreateMarketKeys> for [AccountMeta; CREATE_MARKET_IX_ACCOUNTS_LEN] {
    fn from(keys: CreateMarketKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.payer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.market,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.system_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.base_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.quote_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.base_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.quote_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_program22,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; CREATE_MARKET_IX_ACCOUNTS_LEN]> for CreateMarketKeys {
    fn from(pubkeys: [Pubkey; CREATE_MARKET_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            payer: pubkeys[0],
            market: pubkeys[1],
            system_program: pubkeys[2],
            base_mint: pubkeys[3],
            quote_mint: pubkeys[4],
            base_vault: pubkeys[5],
            quote_vault: pubkeys[6],
            token_program: pubkeys[7],
            token_program22: pubkeys[8],
        }
    }
}
impl<'info> From<CreateMarketAccounts<'_, 'info>>
for [AccountInfo<'info>; CREATE_MARKET_IX_ACCOUNTS_LEN] {
    fn from(accounts: CreateMarketAccounts<'_, 'info>) -> Self {
        [
            accounts.payer.clone(),
            accounts.market.clone(),
            accounts.system_program.clone(),
            accounts.base_mint.clone(),
            accounts.quote_mint.clone(),
            accounts.base_vault.clone(),
            accounts.quote_vault.clone(),
            accounts.token_program.clone(),
            accounts.token_program22.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CREATE_MARKET_IX_ACCOUNTS_LEN]>
for CreateMarketAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; CREATE_MARKET_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            payer: &arr[0],
            market: &arr[1],
            system_program: &arr[2],
            base_mint: &arr[3],
            quote_mint: &arr[4],
            base_vault: &arr[5],
            quote_vault: &arr[6],
            token_program: &arr[7],
            token_program22: &arr[8],
        }
    }
}
pub const CREATE_MARKET_IX_DISCM: u8 = 0u8;
#[derive(Clone, Debug, PartialEq)]
pub struct CreateMarketIxData;
impl CreateMarketIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm_buf = [0u8; 1];
        reader.read_exact(&mut maybe_discm_buf)?;
        let maybe_discm = maybe_discm_buf[0];
        if maybe_discm != CREATE_MARKET_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&[CREATE_MARKET_IX_DISCM])
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn create_market_ix_with_program_id(
    program_id: Pubkey,
    keys: CreateMarketKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CREATE_MARKET_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: CreateMarketIxData.try_to_vec()?,
    })
}
pub fn create_market_ix(keys: CreateMarketKeys) -> std::io::Result<Instruction> {
    create_market_ix_with_program_id(MANIFEST_PROGRAM_ID, keys)
}
pub fn create_market_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CreateMarketAccounts<'_, '_>,
) -> ProgramResult {
    let keys: CreateMarketKeys = accounts.into();
    let ix = create_market_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn create_market_invoke(accounts: CreateMarketAccounts<'_, '_>) -> ProgramResult {
    create_market_invoke_with_program_id(MANIFEST_PROGRAM_ID, accounts)
}
pub fn create_market_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CreateMarketAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CreateMarketKeys = accounts.into();
    let ix = create_market_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn create_market_invoke_signed(
    accounts: CreateMarketAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    create_market_invoke_signed_with_program_id(MANIFEST_PROGRAM_ID, accounts, seeds)
}
pub fn create_market_verify_account_keys(
    accounts: CreateMarketAccounts<'_, '_>,
    keys: CreateMarketKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.payer.key, &keys.payer),
        (accounts.market.key, &keys.market),
        (accounts.system_program.key, &keys.system_program),
        (accounts.base_mint.key, &keys.base_mint),
        (accounts.quote_mint.key, &keys.quote_mint),
        (accounts.base_vault.key, &keys.base_vault),
        (accounts.quote_vault.key, &keys.quote_vault),
        (accounts.token_program.key, &keys.token_program),
        (accounts.token_program22.key, &keys.token_program22),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn create_market_verify_writable_privileges<'me, 'info>(
    accounts: CreateMarketAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.payer,
        accounts.market,
        accounts.base_vault,
        accounts.quote_vault,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn create_market_verify_signer_privileges<'me, 'info>(
    accounts: CreateMarketAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.payer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn create_market_verify_account_privileges<'me, 'info>(
    accounts: CreateMarketAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    create_market_verify_writable_privileges(accounts)?;
    create_market_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CLAIM_SEAT_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct ClaimSeatAccounts<'me, 'info> {
    pub payer: &'me AccountInfo<'info>,
    pub market: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct ClaimSeatKeys {
    pub payer: Pubkey,
    pub market: Pubkey,
    pub system_program: Pubkey,
}
impl From<ClaimSeatAccounts<'_, '_>> for ClaimSeatKeys {
    fn from(accounts: ClaimSeatAccounts) -> Self {
        Self {
            payer: *accounts.payer.key,
            market: *accounts.market.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<ClaimSeatKeys> for [AccountMeta; CLAIM_SEAT_IX_ACCOUNTS_LEN] {
    fn from(keys: ClaimSeatKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.payer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.market,
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
impl From<[Pubkey; CLAIM_SEAT_IX_ACCOUNTS_LEN]> for ClaimSeatKeys {
    fn from(pubkeys: [Pubkey; CLAIM_SEAT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            payer: pubkeys[0],
            market: pubkeys[1],
            system_program: pubkeys[2],
        }
    }
}
impl<'info> From<ClaimSeatAccounts<'_, 'info>>
for [AccountInfo<'info>; CLAIM_SEAT_IX_ACCOUNTS_LEN] {
    fn from(accounts: ClaimSeatAccounts<'_, 'info>) -> Self {
        [
            accounts.payer.clone(),
            accounts.market.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CLAIM_SEAT_IX_ACCOUNTS_LEN]>
for ClaimSeatAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; CLAIM_SEAT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            payer: &arr[0],
            market: &arr[1],
            system_program: &arr[2],
        }
    }
}
pub const CLAIM_SEAT_IX_DISCM: u8 = 1u8;
#[derive(Clone, Debug, PartialEq)]
pub struct ClaimSeatIxData;
impl ClaimSeatIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm_buf = [0u8; 1];
        reader.read_exact(&mut maybe_discm_buf)?;
        let maybe_discm = maybe_discm_buf[0];
        if maybe_discm != CLAIM_SEAT_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&[CLAIM_SEAT_IX_DISCM])
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn claim_seat_ix_with_program_id(
    program_id: Pubkey,
    keys: ClaimSeatKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CLAIM_SEAT_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: ClaimSeatIxData.try_to_vec()?,
    })
}
pub fn claim_seat_ix(keys: ClaimSeatKeys) -> std::io::Result<Instruction> {
    claim_seat_ix_with_program_id(MANIFEST_PROGRAM_ID, keys)
}
pub fn claim_seat_invoke_with_program_id(
    program_id: Pubkey,
    accounts: ClaimSeatAccounts<'_, '_>,
) -> ProgramResult {
    let keys: ClaimSeatKeys = accounts.into();
    let ix = claim_seat_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn claim_seat_invoke(accounts: ClaimSeatAccounts<'_, '_>) -> ProgramResult {
    claim_seat_invoke_with_program_id(MANIFEST_PROGRAM_ID, accounts)
}
pub fn claim_seat_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: ClaimSeatAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: ClaimSeatKeys = accounts.into();
    let ix = claim_seat_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn claim_seat_invoke_signed(
    accounts: ClaimSeatAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    claim_seat_invoke_signed_with_program_id(MANIFEST_PROGRAM_ID, accounts, seeds)
}
pub fn claim_seat_verify_account_keys(
    accounts: ClaimSeatAccounts<'_, '_>,
    keys: ClaimSeatKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.payer.key, &keys.payer),
        (accounts.market.key, &keys.market),
        (accounts.system_program.key, &keys.system_program),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn claim_seat_verify_writable_privileges<'me, 'info>(
    accounts: ClaimSeatAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.payer, accounts.market] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn claim_seat_verify_signer_privileges<'me, 'info>(
    accounts: ClaimSeatAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.payer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn claim_seat_verify_account_privileges<'me, 'info>(
    accounts: ClaimSeatAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    claim_seat_verify_writable_privileges(accounts)?;
    claim_seat_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const DEPOSIT_IX_ACCOUNTS_LEN: usize = 6;
#[derive(Copy, Clone, Debug)]
pub struct DepositAccounts<'me, 'info> {
    pub payer: &'me AccountInfo<'info>,
    pub market: &'me AccountInfo<'info>,
    pub trader_token: &'me AccountInfo<'info>,
    pub vault: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub mint: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct DepositKeys {
    pub payer: Pubkey,
    pub market: Pubkey,
    pub trader_token: Pubkey,
    pub vault: Pubkey,
    pub token_program: Pubkey,
    pub mint: Pubkey,
}
impl From<DepositAccounts<'_, '_>> for DepositKeys {
    fn from(accounts: DepositAccounts) -> Self {
        Self {
            payer: *accounts.payer.key,
            market: *accounts.market.key,
            trader_token: *accounts.trader_token.key,
            vault: *accounts.vault.key,
            token_program: *accounts.token_program.key,
            mint: *accounts.mint.key,
        }
    }
}
impl From<DepositKeys> for [AccountMeta; DEPOSIT_IX_ACCOUNTS_LEN] {
    fn from(keys: DepositKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.payer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.market,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.trader_token,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.mint,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; DEPOSIT_IX_ACCOUNTS_LEN]> for DepositKeys {
    fn from(pubkeys: [Pubkey; DEPOSIT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            payer: pubkeys[0],
            market: pubkeys[1],
            trader_token: pubkeys[2],
            vault: pubkeys[3],
            token_program: pubkeys[4],
            mint: pubkeys[5],
        }
    }
}
impl<'info> From<DepositAccounts<'_, 'info>>
for [AccountInfo<'info>; DEPOSIT_IX_ACCOUNTS_LEN] {
    fn from(accounts: DepositAccounts<'_, 'info>) -> Self {
        [
            accounts.payer.clone(),
            accounts.market.clone(),
            accounts.trader_token.clone(),
            accounts.vault.clone(),
            accounts.token_program.clone(),
            accounts.mint.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; DEPOSIT_IX_ACCOUNTS_LEN]>
for DepositAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; DEPOSIT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            payer: &arr[0],
            market: &arr[1],
            trader_token: &arr[2],
            vault: &arr[3],
            token_program: &arr[4],
            mint: &arr[5],
        }
    }
}
pub const DEPOSIT_IX_DISCM: u8 = 2u8;
#[derive(
    BorshDeserialize,
    BorshSerialize,
    Clone,
    Debug,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
pub struct DepositIxArgs {
    pub params: DepositParams,
    pub trader_index_hint: Option<u32>,
}
impl DepositIxArgs {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let params = if reader.is_empty() {
            Default::default()
        } else {
            <DepositParams>::deserialize(&mut reader)?
        };
        let trader_index_hint: Option<u32> = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { params, trader_index_hint })
    }
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
        let mut maybe_discm_buf = [0u8; 1];
        reader.read_exact(&mut maybe_discm_buf)?;
        let maybe_discm = maybe_discm_buf[0];
        if maybe_discm != DEPOSIT_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(DepositIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&[DEPOSIT_IX_DISCM])?;
        self.0.serialize(&mut writer)
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
    deposit_ix_with_program_id(MANIFEST_PROGRAM_ID, keys, args)
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
    deposit_invoke_with_program_id(MANIFEST_PROGRAM_ID, accounts, args)
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
    deposit_invoke_signed_with_program_id(MANIFEST_PROGRAM_ID, accounts, args, seeds)
}
pub fn deposit_verify_account_keys(
    accounts: DepositAccounts<'_, '_>,
    keys: DepositKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.payer.key, &keys.payer),
        (accounts.market.key, &keys.market),
        (accounts.trader_token.key, &keys.trader_token),
        (accounts.vault.key, &keys.vault),
        (accounts.token_program.key, &keys.token_program),
        (accounts.mint.key, &keys.mint),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn deposit_verify_writable_privileges<'me, 'info>(
    accounts: DepositAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.payer,
        accounts.market,
        accounts.trader_token,
        accounts.vault,
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
    for should_be_signer in [accounts.payer] {
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
pub const WITHDRAW_IX_ACCOUNTS_LEN: usize = 6;
#[derive(Copy, Clone, Debug)]
pub struct WithdrawAccounts<'me, 'info> {
    pub payer: &'me AccountInfo<'info>,
    pub market: &'me AccountInfo<'info>,
    pub trader_token: &'me AccountInfo<'info>,
    pub vault: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub mint: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct WithdrawKeys {
    pub payer: Pubkey,
    pub market: Pubkey,
    pub trader_token: Pubkey,
    pub vault: Pubkey,
    pub token_program: Pubkey,
    pub mint: Pubkey,
}
impl From<WithdrawAccounts<'_, '_>> for WithdrawKeys {
    fn from(accounts: WithdrawAccounts) -> Self {
        Self {
            payer: *accounts.payer.key,
            market: *accounts.market.key,
            trader_token: *accounts.trader_token.key,
            vault: *accounts.vault.key,
            token_program: *accounts.token_program.key,
            mint: *accounts.mint.key,
        }
    }
}
impl From<WithdrawKeys> for [AccountMeta; WITHDRAW_IX_ACCOUNTS_LEN] {
    fn from(keys: WithdrawKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.payer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.market,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.trader_token,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.mint,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; WITHDRAW_IX_ACCOUNTS_LEN]> for WithdrawKeys {
    fn from(pubkeys: [Pubkey; WITHDRAW_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            payer: pubkeys[0],
            market: pubkeys[1],
            trader_token: pubkeys[2],
            vault: pubkeys[3],
            token_program: pubkeys[4],
            mint: pubkeys[5],
        }
    }
}
impl<'info> From<WithdrawAccounts<'_, 'info>>
for [AccountInfo<'info>; WITHDRAW_IX_ACCOUNTS_LEN] {
    fn from(accounts: WithdrawAccounts<'_, 'info>) -> Self {
        [
            accounts.payer.clone(),
            accounts.market.clone(),
            accounts.trader_token.clone(),
            accounts.vault.clone(),
            accounts.token_program.clone(),
            accounts.mint.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; WITHDRAW_IX_ACCOUNTS_LEN]>
for WithdrawAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; WITHDRAW_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            payer: &arr[0],
            market: &arr[1],
            trader_token: &arr[2],
            vault: &arr[3],
            token_program: &arr[4],
            mint: &arr[5],
        }
    }
}
pub const WITHDRAW_IX_DISCM: u8 = 3u8;
#[derive(
    BorshDeserialize,
    BorshSerialize,
    Clone,
    Debug,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
pub struct WithdrawIxArgs {
    pub params: WithdrawParams,
}
impl WithdrawIxArgs {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let params = if reader.is_empty() {
            Default::default()
        } else {
            <WithdrawParams>::deserialize(&mut reader)?
        };
        *__buf = reader;
        Ok(Self { params })
    }
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
        let mut maybe_discm_buf = [0u8; 1];
        reader.read_exact(&mut maybe_discm_buf)?;
        let maybe_discm = maybe_discm_buf[0];
        if maybe_discm != WITHDRAW_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(WithdrawIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&[WITHDRAW_IX_DISCM])?;
        self.0.serialize(&mut writer)
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
    withdraw_ix_with_program_id(MANIFEST_PROGRAM_ID, keys, args)
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
    withdraw_invoke_with_program_id(MANIFEST_PROGRAM_ID, accounts, args)
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
    withdraw_invoke_signed_with_program_id(MANIFEST_PROGRAM_ID, accounts, args, seeds)
}
pub fn withdraw_verify_account_keys(
    accounts: WithdrawAccounts<'_, '_>,
    keys: WithdrawKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.payer.key, &keys.payer),
        (accounts.market.key, &keys.market),
        (accounts.trader_token.key, &keys.trader_token),
        (accounts.vault.key, &keys.vault),
        (accounts.token_program.key, &keys.token_program),
        (accounts.mint.key, &keys.mint),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn withdraw_verify_writable_privileges<'me, 'info>(
    accounts: WithdrawAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.payer,
        accounts.market,
        accounts.trader_token,
        accounts.vault,
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
    for should_be_signer in [accounts.payer] {
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
pub const SWAP_IX_ACCOUNTS_LEN: usize = 13;
#[derive(Copy, Clone, Debug)]
pub struct SwapAccounts<'me, 'info> {
    pub payer: &'me AccountInfo<'info>,
    pub market: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub trader_base: &'me AccountInfo<'info>,
    pub trader_quote: &'me AccountInfo<'info>,
    pub base_vault: &'me AccountInfo<'info>,
    pub quote_vault: &'me AccountInfo<'info>,
    pub token_program_base: &'me AccountInfo<'info>,
    pub base_mint: &'me AccountInfo<'info>,
    pub token_program_quote: &'me AccountInfo<'info>,
    pub quote_mint: &'me AccountInfo<'info>,
    pub global: &'me AccountInfo<'info>,
    pub global_vault: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct SwapKeys {
    pub payer: Pubkey,
    pub market: Pubkey,
    pub system_program: Pubkey,
    pub trader_base: Pubkey,
    pub trader_quote: Pubkey,
    pub base_vault: Pubkey,
    pub quote_vault: Pubkey,
    pub token_program_base: Pubkey,
    pub base_mint: Pubkey,
    pub token_program_quote: Pubkey,
    pub quote_mint: Pubkey,
    pub global: Pubkey,
    pub global_vault: Pubkey,
}
impl From<SwapAccounts<'_, '_>> for SwapKeys {
    fn from(accounts: SwapAccounts) -> Self {
        Self {
            payer: *accounts.payer.key,
            market: *accounts.market.key,
            system_program: *accounts.system_program.key,
            trader_base: *accounts.trader_base.key,
            trader_quote: *accounts.trader_quote.key,
            base_vault: *accounts.base_vault.key,
            quote_vault: *accounts.quote_vault.key,
            token_program_base: *accounts.token_program_base.key,
            base_mint: *accounts.base_mint.key,
            token_program_quote: *accounts.token_program_quote.key,
            quote_mint: *accounts.quote_mint.key,
            global: *accounts.global.key,
            global_vault: *accounts.global_vault.key,
        }
    }
}
impl From<SwapKeys> for [AccountMeta; SWAP_IX_ACCOUNTS_LEN] {
    fn from(keys: SwapKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.payer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.market,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.system_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.trader_base,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.trader_quote,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.base_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.quote_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_program_base,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.base_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_program_quote,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.quote_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.global,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.global_vault,
                is_signer: false,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; SWAP_IX_ACCOUNTS_LEN]> for SwapKeys {
    fn from(pubkeys: [Pubkey; SWAP_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            payer: pubkeys[0],
            market: pubkeys[1],
            system_program: pubkeys[2],
            trader_base: pubkeys[3],
            trader_quote: pubkeys[4],
            base_vault: pubkeys[5],
            quote_vault: pubkeys[6],
            token_program_base: pubkeys[7],
            base_mint: pubkeys[8],
            token_program_quote: pubkeys[9],
            quote_mint: pubkeys[10],
            global: pubkeys[11],
            global_vault: pubkeys[12],
        }
    }
}
impl<'info> From<SwapAccounts<'_, 'info>>
for [AccountInfo<'info>; SWAP_IX_ACCOUNTS_LEN] {
    fn from(accounts: SwapAccounts<'_, 'info>) -> Self {
        [
            accounts.payer.clone(),
            accounts.market.clone(),
            accounts.system_program.clone(),
            accounts.trader_base.clone(),
            accounts.trader_quote.clone(),
            accounts.base_vault.clone(),
            accounts.quote_vault.clone(),
            accounts.token_program_base.clone(),
            accounts.base_mint.clone(),
            accounts.token_program_quote.clone(),
            accounts.quote_mint.clone(),
            accounts.global.clone(),
            accounts.global_vault.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SWAP_IX_ACCOUNTS_LEN]>
for SwapAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; SWAP_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            payer: &arr[0],
            market: &arr[1],
            system_program: &arr[2],
            trader_base: &arr[3],
            trader_quote: &arr[4],
            base_vault: &arr[5],
            quote_vault: &arr[6],
            token_program_base: &arr[7],
            base_mint: &arr[8],
            token_program_quote: &arr[9],
            quote_mint: &arr[10],
            global: &arr[11],
            global_vault: &arr[12],
        }
    }
}
pub const SWAP_IX_DISCM: u8 = 4u8;
#[derive(
    BorshDeserialize,
    BorshSerialize,
    Clone,
    Debug,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
pub struct SwapIxArgs {
    pub params: SwapParams,
}
impl SwapIxArgs {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let params = if reader.is_empty() {
            Default::default()
        } else {
            <SwapParams>::deserialize(&mut reader)?
        };
        *__buf = reader;
        Ok(Self { params })
    }
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
        let mut maybe_discm_buf = [0u8; 1];
        reader.read_exact(&mut maybe_discm_buf)?;
        let maybe_discm = maybe_discm_buf[0];
        if maybe_discm != SWAP_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(SwapIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&[SWAP_IX_DISCM])?;
        self.0.serialize(&mut writer)
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
    swap_ix_with_program_id(MANIFEST_PROGRAM_ID, keys, args)
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
    swap_invoke_with_program_id(MANIFEST_PROGRAM_ID, accounts, args)
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
    swap_invoke_signed_with_program_id(MANIFEST_PROGRAM_ID, accounts, args, seeds)
}
pub fn swap_verify_account_keys(
    accounts: SwapAccounts<'_, '_>,
    keys: SwapKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.payer.key, &keys.payer),
        (accounts.market.key, &keys.market),
        (accounts.system_program.key, &keys.system_program),
        (accounts.trader_base.key, &keys.trader_base),
        (accounts.trader_quote.key, &keys.trader_quote),
        (accounts.base_vault.key, &keys.base_vault),
        (accounts.quote_vault.key, &keys.quote_vault),
        (accounts.token_program_base.key, &keys.token_program_base),
        (accounts.base_mint.key, &keys.base_mint),
        (accounts.token_program_quote.key, &keys.token_program_quote),
        (accounts.quote_mint.key, &keys.quote_mint),
        (accounts.global.key, &keys.global),
        (accounts.global_vault.key, &keys.global_vault),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn swap_verify_writable_privileges<'me, 'info>(
    accounts: SwapAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.payer,
        accounts.market,
        accounts.trader_base,
        accounts.trader_quote,
        accounts.base_vault,
        accounts.quote_vault,
        accounts.global,
        accounts.global_vault,
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
pub const SWAP_V2_IX_ACCOUNTS_LEN: usize = 14;
#[derive(Copy, Clone, Debug)]
pub struct SwapV2Accounts<'me, 'info> {
    pub payer: &'me AccountInfo<'info>,
    pub owner: &'me AccountInfo<'info>,
    pub market: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub trader_base: &'me AccountInfo<'info>,
    pub trader_quote: &'me AccountInfo<'info>,
    pub base_vault: &'me AccountInfo<'info>,
    pub quote_vault: &'me AccountInfo<'info>,
    pub token_program_base: &'me AccountInfo<'info>,
    pub base_mint: &'me AccountInfo<'info>,
    pub token_program_quote: &'me AccountInfo<'info>,
    pub quote_mint: &'me AccountInfo<'info>,
    pub global: &'me AccountInfo<'info>,
    pub global_vault: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct SwapV2Keys {
    pub payer: Pubkey,
    pub owner: Pubkey,
    pub market: Pubkey,
    pub system_program: Pubkey,
    pub trader_base: Pubkey,
    pub trader_quote: Pubkey,
    pub base_vault: Pubkey,
    pub quote_vault: Pubkey,
    pub token_program_base: Pubkey,
    pub base_mint: Pubkey,
    pub token_program_quote: Pubkey,
    pub quote_mint: Pubkey,
    pub global: Pubkey,
    pub global_vault: Pubkey,
}
impl From<SwapV2Accounts<'_, '_>> for SwapV2Keys {
    fn from(accounts: SwapV2Accounts) -> Self {
        Self {
            payer: *accounts.payer.key,
            owner: *accounts.owner.key,
            market: *accounts.market.key,
            system_program: *accounts.system_program.key,
            trader_base: *accounts.trader_base.key,
            trader_quote: *accounts.trader_quote.key,
            base_vault: *accounts.base_vault.key,
            quote_vault: *accounts.quote_vault.key,
            token_program_base: *accounts.token_program_base.key,
            base_mint: *accounts.base_mint.key,
            token_program_quote: *accounts.token_program_quote.key,
            quote_mint: *accounts.quote_mint.key,
            global: *accounts.global.key,
            global_vault: *accounts.global_vault.key,
        }
    }
}
impl From<SwapV2Keys> for [AccountMeta; SWAP_V2_IX_ACCOUNTS_LEN] {
    fn from(keys: SwapV2Keys) -> Self {
        [
            AccountMeta {
                pubkey: keys.payer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.owner,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.market,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.system_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.trader_base,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.trader_quote,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.base_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.quote_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_program_base,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.base_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_program_quote,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.quote_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.global,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.global_vault,
                is_signer: false,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; SWAP_V2_IX_ACCOUNTS_LEN]> for SwapV2Keys {
    fn from(pubkeys: [Pubkey; SWAP_V2_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            payer: pubkeys[0],
            owner: pubkeys[1],
            market: pubkeys[2],
            system_program: pubkeys[3],
            trader_base: pubkeys[4],
            trader_quote: pubkeys[5],
            base_vault: pubkeys[6],
            quote_vault: pubkeys[7],
            token_program_base: pubkeys[8],
            base_mint: pubkeys[9],
            token_program_quote: pubkeys[10],
            quote_mint: pubkeys[11],
            global: pubkeys[12],
            global_vault: pubkeys[13],
        }
    }
}
impl<'info> From<SwapV2Accounts<'_, 'info>>
for [AccountInfo<'info>; SWAP_V2_IX_ACCOUNTS_LEN] {
    fn from(accounts: SwapV2Accounts<'_, 'info>) -> Self {
        [
            accounts.payer.clone(),
            accounts.owner.clone(),
            accounts.market.clone(),
            accounts.system_program.clone(),
            accounts.trader_base.clone(),
            accounts.trader_quote.clone(),
            accounts.base_vault.clone(),
            accounts.quote_vault.clone(),
            accounts.token_program_base.clone(),
            accounts.base_mint.clone(),
            accounts.token_program_quote.clone(),
            accounts.quote_mint.clone(),
            accounts.global.clone(),
            accounts.global_vault.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SWAP_V2_IX_ACCOUNTS_LEN]>
for SwapV2Accounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; SWAP_V2_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            payer: &arr[0],
            owner: &arr[1],
            market: &arr[2],
            system_program: &arr[3],
            trader_base: &arr[4],
            trader_quote: &arr[5],
            base_vault: &arr[6],
            quote_vault: &arr[7],
            token_program_base: &arr[8],
            base_mint: &arr[9],
            token_program_quote: &arr[10],
            quote_mint: &arr[11],
            global: &arr[12],
            global_vault: &arr[13],
        }
    }
}
pub const SWAP_V2_IX_DISCM: u8 = 4u8;
#[derive(
    BorshDeserialize,
    BorshSerialize,
    Clone,
    Debug,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
pub struct SwapV2IxArgs {
    pub params: SwapParams,
}
impl SwapV2IxArgs {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let params = if reader.is_empty() {
            Default::default()
        } else {
            <SwapParams>::deserialize(&mut reader)?
        };
        *__buf = reader;
        Ok(Self { params })
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct SwapV2IxData(pub SwapV2IxArgs);
impl From<SwapV2IxArgs> for SwapV2IxData {
    fn from(args: SwapV2IxArgs) -> Self {
        Self(args)
    }
}
impl SwapV2IxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm_buf = [0u8; 1];
        reader.read_exact(&mut maybe_discm_buf)?;
        let maybe_discm = maybe_discm_buf[0];
        if maybe_discm != SWAP_V2_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(SwapV2IxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&[SWAP_V2_IX_DISCM])?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn swap_v2_ix_with_program_id(
    program_id: Pubkey,
    keys: SwapV2Keys,
    args: SwapV2IxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SWAP_V2_IX_ACCOUNTS_LEN] = keys.into();
    let data: SwapV2IxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn swap_v2_ix(keys: SwapV2Keys, args: SwapV2IxArgs) -> std::io::Result<Instruction> {
    swap_v2_ix_with_program_id(MANIFEST_PROGRAM_ID, keys, args)
}
pub fn swap_v2_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SwapV2Accounts<'_, '_>,
    args: SwapV2IxArgs,
) -> ProgramResult {
    let keys: SwapV2Keys = accounts.into();
    let ix = swap_v2_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn swap_v2_invoke(
    accounts: SwapV2Accounts<'_, '_>,
    args: SwapV2IxArgs,
) -> ProgramResult {
    swap_v2_invoke_with_program_id(MANIFEST_PROGRAM_ID, accounts, args)
}
pub fn swap_v2_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SwapV2Accounts<'_, '_>,
    args: SwapV2IxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SwapV2Keys = accounts.into();
    let ix = swap_v2_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn swap_v2_invoke_signed(
    accounts: SwapV2Accounts<'_, '_>,
    args: SwapV2IxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    swap_v2_invoke_signed_with_program_id(MANIFEST_PROGRAM_ID, accounts, args, seeds)
}
pub fn swap_v2_verify_account_keys(
    accounts: SwapV2Accounts<'_, '_>,
    keys: SwapV2Keys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.payer.key, &keys.payer),
        (accounts.owner.key, &keys.owner),
        (accounts.market.key, &keys.market),
        (accounts.system_program.key, &keys.system_program),
        (accounts.trader_base.key, &keys.trader_base),
        (accounts.trader_quote.key, &keys.trader_quote),
        (accounts.base_vault.key, &keys.base_vault),
        (accounts.quote_vault.key, &keys.quote_vault),
        (accounts.token_program_base.key, &keys.token_program_base),
        (accounts.base_mint.key, &keys.base_mint),
        (accounts.token_program_quote.key, &keys.token_program_quote),
        (accounts.quote_mint.key, &keys.quote_mint),
        (accounts.global.key, &keys.global),
        (accounts.global_vault.key, &keys.global_vault),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn swap_v2_verify_writable_privileges<'me, 'info>(
    accounts: SwapV2Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.payer,
        accounts.owner,
        accounts.market,
        accounts.trader_base,
        accounts.trader_quote,
        accounts.base_vault,
        accounts.quote_vault,
        accounts.global,
        accounts.global_vault,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn swap_v2_verify_signer_privileges<'me, 'info>(
    accounts: SwapV2Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.payer, accounts.owner] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn swap_v2_verify_account_privileges<'me, 'info>(
    accounts: SwapV2Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    swap_v2_verify_writable_privileges(accounts)?;
    swap_v2_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const EXPAND_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct ExpandAccounts<'me, 'info> {
    pub payer: &'me AccountInfo<'info>,
    pub market: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct ExpandKeys {
    pub payer: Pubkey,
    pub market: Pubkey,
    pub system_program: Pubkey,
}
impl From<ExpandAccounts<'_, '_>> for ExpandKeys {
    fn from(accounts: ExpandAccounts) -> Self {
        Self {
            payer: *accounts.payer.key,
            market: *accounts.market.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<ExpandKeys> for [AccountMeta; EXPAND_IX_ACCOUNTS_LEN] {
    fn from(keys: ExpandKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.payer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.market,
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
impl From<[Pubkey; EXPAND_IX_ACCOUNTS_LEN]> for ExpandKeys {
    fn from(pubkeys: [Pubkey; EXPAND_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            payer: pubkeys[0],
            market: pubkeys[1],
            system_program: pubkeys[2],
        }
    }
}
impl<'info> From<ExpandAccounts<'_, 'info>>
for [AccountInfo<'info>; EXPAND_IX_ACCOUNTS_LEN] {
    fn from(accounts: ExpandAccounts<'_, 'info>) -> Self {
        [
            accounts.payer.clone(),
            accounts.market.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; EXPAND_IX_ACCOUNTS_LEN]>
for ExpandAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; EXPAND_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            payer: &arr[0],
            market: &arr[1],
            system_program: &arr[2],
        }
    }
}
pub const EXPAND_IX_DISCM: u8 = 5u8;
#[derive(Clone, Debug, PartialEq)]
pub struct ExpandIxData;
impl ExpandIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm_buf = [0u8; 1];
        reader.read_exact(&mut maybe_discm_buf)?;
        let maybe_discm = maybe_discm_buf[0];
        if maybe_discm != EXPAND_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&[EXPAND_IX_DISCM])
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn expand_ix_with_program_id(
    program_id: Pubkey,
    keys: ExpandKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; EXPAND_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: ExpandIxData.try_to_vec()?,
    })
}
pub fn expand_ix(keys: ExpandKeys) -> std::io::Result<Instruction> {
    expand_ix_with_program_id(MANIFEST_PROGRAM_ID, keys)
}
pub fn expand_invoke_with_program_id(
    program_id: Pubkey,
    accounts: ExpandAccounts<'_, '_>,
) -> ProgramResult {
    let keys: ExpandKeys = accounts.into();
    let ix = expand_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn expand_invoke(accounts: ExpandAccounts<'_, '_>) -> ProgramResult {
    expand_invoke_with_program_id(MANIFEST_PROGRAM_ID, accounts)
}
pub fn expand_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: ExpandAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: ExpandKeys = accounts.into();
    let ix = expand_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn expand_invoke_signed(
    accounts: ExpandAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    expand_invoke_signed_with_program_id(MANIFEST_PROGRAM_ID, accounts, seeds)
}
pub fn expand_verify_account_keys(
    accounts: ExpandAccounts<'_, '_>,
    keys: ExpandKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.payer.key, &keys.payer),
        (accounts.market.key, &keys.market),
        (accounts.system_program.key, &keys.system_program),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn expand_verify_writable_privileges<'me, 'info>(
    accounts: ExpandAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.payer, accounts.market] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn expand_verify_signer_privileges<'me, 'info>(
    accounts: ExpandAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.payer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn expand_verify_account_privileges<'me, 'info>(
    accounts: ExpandAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    expand_verify_writable_privileges(accounts)?;
    expand_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const BATCH_UPDATE_IX_ACCOUNTS_LEN: usize = 13;
#[derive(Copy, Clone, Debug)]
pub struct BatchUpdateAccounts<'me, 'info> {
    pub payer: &'me AccountInfo<'info>,
    pub market: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub base_mint: &'me AccountInfo<'info>,
    pub base_global: &'me AccountInfo<'info>,
    pub base_global_vault: &'me AccountInfo<'info>,
    pub base_market_vault: &'me AccountInfo<'info>,
    pub base_token_program: &'me AccountInfo<'info>,
    pub quote_mint: &'me AccountInfo<'info>,
    pub quote_global: &'me AccountInfo<'info>,
    pub quote_global_vault: &'me AccountInfo<'info>,
    pub quote_market_vault: &'me AccountInfo<'info>,
    pub quote_token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct BatchUpdateKeys {
    pub payer: Pubkey,
    pub market: Pubkey,
    pub system_program: Pubkey,
    pub base_mint: Pubkey,
    pub base_global: Pubkey,
    pub base_global_vault: Pubkey,
    pub base_market_vault: Pubkey,
    pub base_token_program: Pubkey,
    pub quote_mint: Pubkey,
    pub quote_global: Pubkey,
    pub quote_global_vault: Pubkey,
    pub quote_market_vault: Pubkey,
    pub quote_token_program: Pubkey,
}
impl From<BatchUpdateAccounts<'_, '_>> for BatchUpdateKeys {
    fn from(accounts: BatchUpdateAccounts) -> Self {
        Self {
            payer: *accounts.payer.key,
            market: *accounts.market.key,
            system_program: *accounts.system_program.key,
            base_mint: *accounts.base_mint.key,
            base_global: *accounts.base_global.key,
            base_global_vault: *accounts.base_global_vault.key,
            base_market_vault: *accounts.base_market_vault.key,
            base_token_program: *accounts.base_token_program.key,
            quote_mint: *accounts.quote_mint.key,
            quote_global: *accounts.quote_global.key,
            quote_global_vault: *accounts.quote_global_vault.key,
            quote_market_vault: *accounts.quote_market_vault.key,
            quote_token_program: *accounts.quote_token_program.key,
        }
    }
}
impl From<BatchUpdateKeys> for [AccountMeta; BATCH_UPDATE_IX_ACCOUNTS_LEN] {
    fn from(keys: BatchUpdateKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.payer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.market,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.system_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.base_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.base_global,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.base_global_vault,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.base_market_vault,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.base_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.quote_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.quote_global,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.quote_global_vault,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.quote_market_vault,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.quote_token_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; BATCH_UPDATE_IX_ACCOUNTS_LEN]> for BatchUpdateKeys {
    fn from(pubkeys: [Pubkey; BATCH_UPDATE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            payer: pubkeys[0],
            market: pubkeys[1],
            system_program: pubkeys[2],
            base_mint: pubkeys[3],
            base_global: pubkeys[4],
            base_global_vault: pubkeys[5],
            base_market_vault: pubkeys[6],
            base_token_program: pubkeys[7],
            quote_mint: pubkeys[8],
            quote_global: pubkeys[9],
            quote_global_vault: pubkeys[10],
            quote_market_vault: pubkeys[11],
            quote_token_program: pubkeys[12],
        }
    }
}
impl<'info> From<BatchUpdateAccounts<'_, 'info>>
for [AccountInfo<'info>; BATCH_UPDATE_IX_ACCOUNTS_LEN] {
    fn from(accounts: BatchUpdateAccounts<'_, 'info>) -> Self {
        [
            accounts.payer.clone(),
            accounts.market.clone(),
            accounts.system_program.clone(),
            accounts.base_mint.clone(),
            accounts.base_global.clone(),
            accounts.base_global_vault.clone(),
            accounts.base_market_vault.clone(),
            accounts.base_token_program.clone(),
            accounts.quote_mint.clone(),
            accounts.quote_global.clone(),
            accounts.quote_global_vault.clone(),
            accounts.quote_market_vault.clone(),
            accounts.quote_token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; BATCH_UPDATE_IX_ACCOUNTS_LEN]>
for BatchUpdateAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; BATCH_UPDATE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            payer: &arr[0],
            market: &arr[1],
            system_program: &arr[2],
            base_mint: &arr[3],
            base_global: &arr[4],
            base_global_vault: &arr[5],
            base_market_vault: &arr[6],
            base_token_program: &arr[7],
            quote_mint: &arr[8],
            quote_global: &arr[9],
            quote_global_vault: &arr[10],
            quote_market_vault: &arr[11],
            quote_token_program: &arr[12],
        }
    }
}
pub const BATCH_UPDATE_IX_DISCM: u8 = 6u8;
#[derive(
    BorshDeserialize,
    BorshSerialize,
    Clone,
    Debug,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
pub struct BatchUpdateIxArgs {
    pub params: BatchUpdateParams,
}
impl BatchUpdateIxArgs {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let params = if reader.is_empty() {
            Default::default()
        } else {
            <BatchUpdateParams>::deserialize(&mut reader)?
        };
        *__buf = reader;
        Ok(Self { params })
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct BatchUpdateIxData(pub BatchUpdateIxArgs);
impl From<BatchUpdateIxArgs> for BatchUpdateIxData {
    fn from(args: BatchUpdateIxArgs) -> Self {
        Self(args)
    }
}
impl BatchUpdateIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm_buf = [0u8; 1];
        reader.read_exact(&mut maybe_discm_buf)?;
        let maybe_discm = maybe_discm_buf[0];
        if maybe_discm != BATCH_UPDATE_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(BatchUpdateIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&[BATCH_UPDATE_IX_DISCM])?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn batch_update_ix_with_program_id(
    program_id: Pubkey,
    keys: BatchUpdateKeys,
    args: BatchUpdateIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; BATCH_UPDATE_IX_ACCOUNTS_LEN] = keys.into();
    let data: BatchUpdateIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn batch_update_ix(
    keys: BatchUpdateKeys,
    args: BatchUpdateIxArgs,
) -> std::io::Result<Instruction> {
    batch_update_ix_with_program_id(MANIFEST_PROGRAM_ID, keys, args)
}
pub fn batch_update_invoke_with_program_id(
    program_id: Pubkey,
    accounts: BatchUpdateAccounts<'_, '_>,
    args: BatchUpdateIxArgs,
) -> ProgramResult {
    let keys: BatchUpdateKeys = accounts.into();
    let ix = batch_update_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn batch_update_invoke(
    accounts: BatchUpdateAccounts<'_, '_>,
    args: BatchUpdateIxArgs,
) -> ProgramResult {
    batch_update_invoke_with_program_id(MANIFEST_PROGRAM_ID, accounts, args)
}
pub fn batch_update_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: BatchUpdateAccounts<'_, '_>,
    args: BatchUpdateIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: BatchUpdateKeys = accounts.into();
    let ix = batch_update_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn batch_update_invoke_signed(
    accounts: BatchUpdateAccounts<'_, '_>,
    args: BatchUpdateIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    batch_update_invoke_signed_with_program_id(
        MANIFEST_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn batch_update_verify_account_keys(
    accounts: BatchUpdateAccounts<'_, '_>,
    keys: BatchUpdateKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.payer.key, &keys.payer),
        (accounts.market.key, &keys.market),
        (accounts.system_program.key, &keys.system_program),
        (accounts.base_mint.key, &keys.base_mint),
        (accounts.base_global.key, &keys.base_global),
        (accounts.base_global_vault.key, &keys.base_global_vault),
        (accounts.base_market_vault.key, &keys.base_market_vault),
        (accounts.base_token_program.key, &keys.base_token_program),
        (accounts.quote_mint.key, &keys.quote_mint),
        (accounts.quote_global.key, &keys.quote_global),
        (accounts.quote_global_vault.key, &keys.quote_global_vault),
        (accounts.quote_market_vault.key, &keys.quote_market_vault),
        (accounts.quote_token_program.key, &keys.quote_token_program),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn batch_update_verify_writable_privileges<'me, 'info>(
    accounts: BatchUpdateAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.payer,
        accounts.market,
        accounts.base_global,
        accounts.quote_global,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn batch_update_verify_signer_privileges<'me, 'info>(
    accounts: BatchUpdateAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.payer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn batch_update_verify_account_privileges<'me, 'info>(
    accounts: BatchUpdateAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    batch_update_verify_writable_privileges(accounts)?;
    batch_update_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const GLOBAL_CREATE_IX_ACCOUNTS_LEN: usize = 6;
#[derive(Copy, Clone, Debug)]
pub struct GlobalCreateAccounts<'me, 'info> {
    pub payer: &'me AccountInfo<'info>,
    pub global: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub mint: &'me AccountInfo<'info>,
    pub global_vault: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct GlobalCreateKeys {
    pub payer: Pubkey,
    pub global: Pubkey,
    pub system_program: Pubkey,
    pub mint: Pubkey,
    pub global_vault: Pubkey,
    pub token_program: Pubkey,
}
impl From<GlobalCreateAccounts<'_, '_>> for GlobalCreateKeys {
    fn from(accounts: GlobalCreateAccounts) -> Self {
        Self {
            payer: *accounts.payer.key,
            global: *accounts.global.key,
            system_program: *accounts.system_program.key,
            mint: *accounts.mint.key,
            global_vault: *accounts.global_vault.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<GlobalCreateKeys> for [AccountMeta; GLOBAL_CREATE_IX_ACCOUNTS_LEN] {
    fn from(keys: GlobalCreateKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.payer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.global,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.system_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.global_vault,
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
impl From<[Pubkey; GLOBAL_CREATE_IX_ACCOUNTS_LEN]> for GlobalCreateKeys {
    fn from(pubkeys: [Pubkey; GLOBAL_CREATE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            payer: pubkeys[0],
            global: pubkeys[1],
            system_program: pubkeys[2],
            mint: pubkeys[3],
            global_vault: pubkeys[4],
            token_program: pubkeys[5],
        }
    }
}
impl<'info> From<GlobalCreateAccounts<'_, 'info>>
for [AccountInfo<'info>; GLOBAL_CREATE_IX_ACCOUNTS_LEN] {
    fn from(accounts: GlobalCreateAccounts<'_, 'info>) -> Self {
        [
            accounts.payer.clone(),
            accounts.global.clone(),
            accounts.system_program.clone(),
            accounts.mint.clone(),
            accounts.global_vault.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; GLOBAL_CREATE_IX_ACCOUNTS_LEN]>
for GlobalCreateAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; GLOBAL_CREATE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            payer: &arr[0],
            global: &arr[1],
            system_program: &arr[2],
            mint: &arr[3],
            global_vault: &arr[4],
            token_program: &arr[5],
        }
    }
}
pub const GLOBAL_CREATE_IX_DISCM: u8 = 7u8;
#[derive(Clone, Debug, PartialEq)]
pub struct GlobalCreateIxData;
impl GlobalCreateIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm_buf = [0u8; 1];
        reader.read_exact(&mut maybe_discm_buf)?;
        let maybe_discm = maybe_discm_buf[0];
        if maybe_discm != GLOBAL_CREATE_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&[GLOBAL_CREATE_IX_DISCM])
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn global_create_ix_with_program_id(
    program_id: Pubkey,
    keys: GlobalCreateKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; GLOBAL_CREATE_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: GlobalCreateIxData.try_to_vec()?,
    })
}
pub fn global_create_ix(keys: GlobalCreateKeys) -> std::io::Result<Instruction> {
    global_create_ix_with_program_id(MANIFEST_PROGRAM_ID, keys)
}
pub fn global_create_invoke_with_program_id(
    program_id: Pubkey,
    accounts: GlobalCreateAccounts<'_, '_>,
) -> ProgramResult {
    let keys: GlobalCreateKeys = accounts.into();
    let ix = global_create_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn global_create_invoke(accounts: GlobalCreateAccounts<'_, '_>) -> ProgramResult {
    global_create_invoke_with_program_id(MANIFEST_PROGRAM_ID, accounts)
}
pub fn global_create_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: GlobalCreateAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: GlobalCreateKeys = accounts.into();
    let ix = global_create_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn global_create_invoke_signed(
    accounts: GlobalCreateAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    global_create_invoke_signed_with_program_id(MANIFEST_PROGRAM_ID, accounts, seeds)
}
pub fn global_create_verify_account_keys(
    accounts: GlobalCreateAccounts<'_, '_>,
    keys: GlobalCreateKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.payer.key, &keys.payer),
        (accounts.global.key, &keys.global),
        (accounts.system_program.key, &keys.system_program),
        (accounts.mint.key, &keys.mint),
        (accounts.global_vault.key, &keys.global_vault),
        (accounts.token_program.key, &keys.token_program),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn global_create_verify_writable_privileges<'me, 'info>(
    accounts: GlobalCreateAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.payer, accounts.global, accounts.global_vault] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn global_create_verify_signer_privileges<'me, 'info>(
    accounts: GlobalCreateAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.payer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn global_create_verify_account_privileges<'me, 'info>(
    accounts: GlobalCreateAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    global_create_verify_writable_privileges(accounts)?;
    global_create_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const GLOBAL_ADD_TRADER_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct GlobalAddTraderAccounts<'me, 'info> {
    pub payer: &'me AccountInfo<'info>,
    pub global: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct GlobalAddTraderKeys {
    pub payer: Pubkey,
    pub global: Pubkey,
    pub system_program: Pubkey,
}
impl From<GlobalAddTraderAccounts<'_, '_>> for GlobalAddTraderKeys {
    fn from(accounts: GlobalAddTraderAccounts) -> Self {
        Self {
            payer: *accounts.payer.key,
            global: *accounts.global.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<GlobalAddTraderKeys> for [AccountMeta; GLOBAL_ADD_TRADER_IX_ACCOUNTS_LEN] {
    fn from(keys: GlobalAddTraderKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.payer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.global,
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
impl From<[Pubkey; GLOBAL_ADD_TRADER_IX_ACCOUNTS_LEN]> for GlobalAddTraderKeys {
    fn from(pubkeys: [Pubkey; GLOBAL_ADD_TRADER_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            payer: pubkeys[0],
            global: pubkeys[1],
            system_program: pubkeys[2],
        }
    }
}
impl<'info> From<GlobalAddTraderAccounts<'_, 'info>>
for [AccountInfo<'info>; GLOBAL_ADD_TRADER_IX_ACCOUNTS_LEN] {
    fn from(accounts: GlobalAddTraderAccounts<'_, 'info>) -> Self {
        [
            accounts.payer.clone(),
            accounts.global.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; GLOBAL_ADD_TRADER_IX_ACCOUNTS_LEN]>
for GlobalAddTraderAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; GLOBAL_ADD_TRADER_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            payer: &arr[0],
            global: &arr[1],
            system_program: &arr[2],
        }
    }
}
pub const GLOBAL_ADD_TRADER_IX_DISCM: u8 = 8u8;
#[derive(Clone, Debug, PartialEq)]
pub struct GlobalAddTraderIxData;
impl GlobalAddTraderIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm_buf = [0u8; 1];
        reader.read_exact(&mut maybe_discm_buf)?;
        let maybe_discm = maybe_discm_buf[0];
        if maybe_discm != GLOBAL_ADD_TRADER_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&[GLOBAL_ADD_TRADER_IX_DISCM])
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn global_add_trader_ix_with_program_id(
    program_id: Pubkey,
    keys: GlobalAddTraderKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; GLOBAL_ADD_TRADER_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: GlobalAddTraderIxData.try_to_vec()?,
    })
}
pub fn global_add_trader_ix(keys: GlobalAddTraderKeys) -> std::io::Result<Instruction> {
    global_add_trader_ix_with_program_id(MANIFEST_PROGRAM_ID, keys)
}
pub fn global_add_trader_invoke_with_program_id(
    program_id: Pubkey,
    accounts: GlobalAddTraderAccounts<'_, '_>,
) -> ProgramResult {
    let keys: GlobalAddTraderKeys = accounts.into();
    let ix = global_add_trader_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn global_add_trader_invoke(
    accounts: GlobalAddTraderAccounts<'_, '_>,
) -> ProgramResult {
    global_add_trader_invoke_with_program_id(MANIFEST_PROGRAM_ID, accounts)
}
pub fn global_add_trader_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: GlobalAddTraderAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: GlobalAddTraderKeys = accounts.into();
    let ix = global_add_trader_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn global_add_trader_invoke_signed(
    accounts: GlobalAddTraderAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    global_add_trader_invoke_signed_with_program_id(MANIFEST_PROGRAM_ID, accounts, seeds)
}
pub fn global_add_trader_verify_account_keys(
    accounts: GlobalAddTraderAccounts<'_, '_>,
    keys: GlobalAddTraderKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.payer.key, &keys.payer),
        (accounts.global.key, &keys.global),
        (accounts.system_program.key, &keys.system_program),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn global_add_trader_verify_writable_privileges<'me, 'info>(
    accounts: GlobalAddTraderAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.payer, accounts.global] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn global_add_trader_verify_signer_privileges<'me, 'info>(
    accounts: GlobalAddTraderAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.payer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn global_add_trader_verify_account_privileges<'me, 'info>(
    accounts: GlobalAddTraderAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    global_add_trader_verify_writable_privileges(accounts)?;
    global_add_trader_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const GLOBAL_DEPOSIT_IX_ACCOUNTS_LEN: usize = 6;
#[derive(Copy, Clone, Debug)]
pub struct GlobalDepositAccounts<'me, 'info> {
    pub payer: &'me AccountInfo<'info>,
    pub global: &'me AccountInfo<'info>,
    pub mint: &'me AccountInfo<'info>,
    pub global_vault: &'me AccountInfo<'info>,
    pub trader_token: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct GlobalDepositKeys {
    pub payer: Pubkey,
    pub global: Pubkey,
    pub mint: Pubkey,
    pub global_vault: Pubkey,
    pub trader_token: Pubkey,
    pub token_program: Pubkey,
}
impl From<GlobalDepositAccounts<'_, '_>> for GlobalDepositKeys {
    fn from(accounts: GlobalDepositAccounts) -> Self {
        Self {
            payer: *accounts.payer.key,
            global: *accounts.global.key,
            mint: *accounts.mint.key,
            global_vault: *accounts.global_vault.key,
            trader_token: *accounts.trader_token.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<GlobalDepositKeys> for [AccountMeta; GLOBAL_DEPOSIT_IX_ACCOUNTS_LEN] {
    fn from(keys: GlobalDepositKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.payer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.global,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.global_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.trader_token,
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
impl From<[Pubkey; GLOBAL_DEPOSIT_IX_ACCOUNTS_LEN]> for GlobalDepositKeys {
    fn from(pubkeys: [Pubkey; GLOBAL_DEPOSIT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            payer: pubkeys[0],
            global: pubkeys[1],
            mint: pubkeys[2],
            global_vault: pubkeys[3],
            trader_token: pubkeys[4],
            token_program: pubkeys[5],
        }
    }
}
impl<'info> From<GlobalDepositAccounts<'_, 'info>>
for [AccountInfo<'info>; GLOBAL_DEPOSIT_IX_ACCOUNTS_LEN] {
    fn from(accounts: GlobalDepositAccounts<'_, 'info>) -> Self {
        [
            accounts.payer.clone(),
            accounts.global.clone(),
            accounts.mint.clone(),
            accounts.global_vault.clone(),
            accounts.trader_token.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; GLOBAL_DEPOSIT_IX_ACCOUNTS_LEN]>
for GlobalDepositAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; GLOBAL_DEPOSIT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            payer: &arr[0],
            global: &arr[1],
            mint: &arr[2],
            global_vault: &arr[3],
            trader_token: &arr[4],
            token_program: &arr[5],
        }
    }
}
pub const GLOBAL_DEPOSIT_IX_DISCM: u8 = 9u8;
#[derive(
    BorshDeserialize,
    BorshSerialize,
    Clone,
    Debug,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
pub struct GlobalDepositIxArgs {
    pub params: GlobalDepositParams,
}
impl GlobalDepositIxArgs {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let params = if reader.is_empty() {
            Default::default()
        } else {
            <GlobalDepositParams>::deserialize(&mut reader)?
        };
        *__buf = reader;
        Ok(Self { params })
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct GlobalDepositIxData(pub GlobalDepositIxArgs);
impl From<GlobalDepositIxArgs> for GlobalDepositIxData {
    fn from(args: GlobalDepositIxArgs) -> Self {
        Self(args)
    }
}
impl GlobalDepositIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm_buf = [0u8; 1];
        reader.read_exact(&mut maybe_discm_buf)?;
        let maybe_discm = maybe_discm_buf[0];
        if maybe_discm != GLOBAL_DEPOSIT_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(GlobalDepositIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&[GLOBAL_DEPOSIT_IX_DISCM])?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn global_deposit_ix_with_program_id(
    program_id: Pubkey,
    keys: GlobalDepositKeys,
    args: GlobalDepositIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; GLOBAL_DEPOSIT_IX_ACCOUNTS_LEN] = keys.into();
    let data: GlobalDepositIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn global_deposit_ix(
    keys: GlobalDepositKeys,
    args: GlobalDepositIxArgs,
) -> std::io::Result<Instruction> {
    global_deposit_ix_with_program_id(MANIFEST_PROGRAM_ID, keys, args)
}
pub fn global_deposit_invoke_with_program_id(
    program_id: Pubkey,
    accounts: GlobalDepositAccounts<'_, '_>,
    args: GlobalDepositIxArgs,
) -> ProgramResult {
    let keys: GlobalDepositKeys = accounts.into();
    let ix = global_deposit_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn global_deposit_invoke(
    accounts: GlobalDepositAccounts<'_, '_>,
    args: GlobalDepositIxArgs,
) -> ProgramResult {
    global_deposit_invoke_with_program_id(MANIFEST_PROGRAM_ID, accounts, args)
}
pub fn global_deposit_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: GlobalDepositAccounts<'_, '_>,
    args: GlobalDepositIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: GlobalDepositKeys = accounts.into();
    let ix = global_deposit_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn global_deposit_invoke_signed(
    accounts: GlobalDepositAccounts<'_, '_>,
    args: GlobalDepositIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    global_deposit_invoke_signed_with_program_id(
        MANIFEST_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn global_deposit_verify_account_keys(
    accounts: GlobalDepositAccounts<'_, '_>,
    keys: GlobalDepositKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.payer.key, &keys.payer),
        (accounts.global.key, &keys.global),
        (accounts.mint.key, &keys.mint),
        (accounts.global_vault.key, &keys.global_vault),
        (accounts.trader_token.key, &keys.trader_token),
        (accounts.token_program.key, &keys.token_program),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn global_deposit_verify_writable_privileges<'me, 'info>(
    accounts: GlobalDepositAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.payer,
        accounts.global,
        accounts.global_vault,
        accounts.trader_token,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn global_deposit_verify_signer_privileges<'me, 'info>(
    accounts: GlobalDepositAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.payer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn global_deposit_verify_account_privileges<'me, 'info>(
    accounts: GlobalDepositAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    global_deposit_verify_writable_privileges(accounts)?;
    global_deposit_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const GLOBAL_WITHDRAW_IX_ACCOUNTS_LEN: usize = 6;
#[derive(Copy, Clone, Debug)]
pub struct GlobalWithdrawAccounts<'me, 'info> {
    pub payer: &'me AccountInfo<'info>,
    pub global: &'me AccountInfo<'info>,
    pub mint: &'me AccountInfo<'info>,
    pub global_vault: &'me AccountInfo<'info>,
    pub trader_token: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct GlobalWithdrawKeys {
    pub payer: Pubkey,
    pub global: Pubkey,
    pub mint: Pubkey,
    pub global_vault: Pubkey,
    pub trader_token: Pubkey,
    pub token_program: Pubkey,
}
impl From<GlobalWithdrawAccounts<'_, '_>> for GlobalWithdrawKeys {
    fn from(accounts: GlobalWithdrawAccounts) -> Self {
        Self {
            payer: *accounts.payer.key,
            global: *accounts.global.key,
            mint: *accounts.mint.key,
            global_vault: *accounts.global_vault.key,
            trader_token: *accounts.trader_token.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<GlobalWithdrawKeys> for [AccountMeta; GLOBAL_WITHDRAW_IX_ACCOUNTS_LEN] {
    fn from(keys: GlobalWithdrawKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.payer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.global,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.global_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.trader_token,
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
impl From<[Pubkey; GLOBAL_WITHDRAW_IX_ACCOUNTS_LEN]> for GlobalWithdrawKeys {
    fn from(pubkeys: [Pubkey; GLOBAL_WITHDRAW_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            payer: pubkeys[0],
            global: pubkeys[1],
            mint: pubkeys[2],
            global_vault: pubkeys[3],
            trader_token: pubkeys[4],
            token_program: pubkeys[5],
        }
    }
}
impl<'info> From<GlobalWithdrawAccounts<'_, 'info>>
for [AccountInfo<'info>; GLOBAL_WITHDRAW_IX_ACCOUNTS_LEN] {
    fn from(accounts: GlobalWithdrawAccounts<'_, 'info>) -> Self {
        [
            accounts.payer.clone(),
            accounts.global.clone(),
            accounts.mint.clone(),
            accounts.global_vault.clone(),
            accounts.trader_token.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; GLOBAL_WITHDRAW_IX_ACCOUNTS_LEN]>
for GlobalWithdrawAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; GLOBAL_WITHDRAW_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            payer: &arr[0],
            global: &arr[1],
            mint: &arr[2],
            global_vault: &arr[3],
            trader_token: &arr[4],
            token_program: &arr[5],
        }
    }
}
pub const GLOBAL_WITHDRAW_IX_DISCM: u8 = 10u8;
#[derive(
    BorshDeserialize,
    BorshSerialize,
    Clone,
    Debug,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
pub struct GlobalWithdrawIxArgs {
    pub params: GlobalWithdrawParams,
}
impl GlobalWithdrawIxArgs {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let params = if reader.is_empty() {
            Default::default()
        } else {
            <GlobalWithdrawParams>::deserialize(&mut reader)?
        };
        *__buf = reader;
        Ok(Self { params })
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct GlobalWithdrawIxData(pub GlobalWithdrawIxArgs);
impl From<GlobalWithdrawIxArgs> for GlobalWithdrawIxData {
    fn from(args: GlobalWithdrawIxArgs) -> Self {
        Self(args)
    }
}
impl GlobalWithdrawIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm_buf = [0u8; 1];
        reader.read_exact(&mut maybe_discm_buf)?;
        let maybe_discm = maybe_discm_buf[0];
        if maybe_discm != GLOBAL_WITHDRAW_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(GlobalWithdrawIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&[GLOBAL_WITHDRAW_IX_DISCM])?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn global_withdraw_ix_with_program_id(
    program_id: Pubkey,
    keys: GlobalWithdrawKeys,
    args: GlobalWithdrawIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; GLOBAL_WITHDRAW_IX_ACCOUNTS_LEN] = keys.into();
    let data: GlobalWithdrawIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn global_withdraw_ix(
    keys: GlobalWithdrawKeys,
    args: GlobalWithdrawIxArgs,
) -> std::io::Result<Instruction> {
    global_withdraw_ix_with_program_id(MANIFEST_PROGRAM_ID, keys, args)
}
pub fn global_withdraw_invoke_with_program_id(
    program_id: Pubkey,
    accounts: GlobalWithdrawAccounts<'_, '_>,
    args: GlobalWithdrawIxArgs,
) -> ProgramResult {
    let keys: GlobalWithdrawKeys = accounts.into();
    let ix = global_withdraw_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn global_withdraw_invoke(
    accounts: GlobalWithdrawAccounts<'_, '_>,
    args: GlobalWithdrawIxArgs,
) -> ProgramResult {
    global_withdraw_invoke_with_program_id(MANIFEST_PROGRAM_ID, accounts, args)
}
pub fn global_withdraw_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: GlobalWithdrawAccounts<'_, '_>,
    args: GlobalWithdrawIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: GlobalWithdrawKeys = accounts.into();
    let ix = global_withdraw_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn global_withdraw_invoke_signed(
    accounts: GlobalWithdrawAccounts<'_, '_>,
    args: GlobalWithdrawIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    global_withdraw_invoke_signed_with_program_id(
        MANIFEST_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn global_withdraw_verify_account_keys(
    accounts: GlobalWithdrawAccounts<'_, '_>,
    keys: GlobalWithdrawKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.payer.key, &keys.payer),
        (accounts.global.key, &keys.global),
        (accounts.mint.key, &keys.mint),
        (accounts.global_vault.key, &keys.global_vault),
        (accounts.trader_token.key, &keys.trader_token),
        (accounts.token_program.key, &keys.token_program),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn global_withdraw_verify_writable_privileges<'me, 'info>(
    accounts: GlobalWithdrawAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.payer,
        accounts.global,
        accounts.global_vault,
        accounts.trader_token,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn global_withdraw_verify_signer_privileges<'me, 'info>(
    accounts: GlobalWithdrawAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.payer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn global_withdraw_verify_account_privileges<'me, 'info>(
    accounts: GlobalWithdrawAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    global_withdraw_verify_writable_privileges(accounts)?;
    global_withdraw_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const GLOBAL_EVICT_IX_ACCOUNTS_LEN: usize = 7;
#[derive(Copy, Clone, Debug)]
pub struct GlobalEvictAccounts<'me, 'info> {
    pub payer: &'me AccountInfo<'info>,
    pub global: &'me AccountInfo<'info>,
    pub mint: &'me AccountInfo<'info>,
    pub global_vault: &'me AccountInfo<'info>,
    pub trader_token: &'me AccountInfo<'info>,
    pub evictee_token: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct GlobalEvictKeys {
    pub payer: Pubkey,
    pub global: Pubkey,
    pub mint: Pubkey,
    pub global_vault: Pubkey,
    pub trader_token: Pubkey,
    pub evictee_token: Pubkey,
    pub token_program: Pubkey,
}
impl From<GlobalEvictAccounts<'_, '_>> for GlobalEvictKeys {
    fn from(accounts: GlobalEvictAccounts) -> Self {
        Self {
            payer: *accounts.payer.key,
            global: *accounts.global.key,
            mint: *accounts.mint.key,
            global_vault: *accounts.global_vault.key,
            trader_token: *accounts.trader_token.key,
            evictee_token: *accounts.evictee_token.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<GlobalEvictKeys> for [AccountMeta; GLOBAL_EVICT_IX_ACCOUNTS_LEN] {
    fn from(keys: GlobalEvictKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.payer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.global,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.global_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.trader_token,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.evictee_token,
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
impl From<[Pubkey; GLOBAL_EVICT_IX_ACCOUNTS_LEN]> for GlobalEvictKeys {
    fn from(pubkeys: [Pubkey; GLOBAL_EVICT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            payer: pubkeys[0],
            global: pubkeys[1],
            mint: pubkeys[2],
            global_vault: pubkeys[3],
            trader_token: pubkeys[4],
            evictee_token: pubkeys[5],
            token_program: pubkeys[6],
        }
    }
}
impl<'info> From<GlobalEvictAccounts<'_, 'info>>
for [AccountInfo<'info>; GLOBAL_EVICT_IX_ACCOUNTS_LEN] {
    fn from(accounts: GlobalEvictAccounts<'_, 'info>) -> Self {
        [
            accounts.payer.clone(),
            accounts.global.clone(),
            accounts.mint.clone(),
            accounts.global_vault.clone(),
            accounts.trader_token.clone(),
            accounts.evictee_token.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; GLOBAL_EVICT_IX_ACCOUNTS_LEN]>
for GlobalEvictAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; GLOBAL_EVICT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            payer: &arr[0],
            global: &arr[1],
            mint: &arr[2],
            global_vault: &arr[3],
            trader_token: &arr[4],
            evictee_token: &arr[5],
            token_program: &arr[6],
        }
    }
}
pub const GLOBAL_EVICT_IX_DISCM: u8 = 11u8;
#[derive(
    BorshDeserialize,
    BorshSerialize,
    Clone,
    Debug,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
pub struct GlobalEvictIxArgs {
    pub params: GlobalEvictParams,
}
impl GlobalEvictIxArgs {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let params = if reader.is_empty() {
            Default::default()
        } else {
            <GlobalEvictParams>::deserialize(&mut reader)?
        };
        *__buf = reader;
        Ok(Self { params })
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct GlobalEvictIxData(pub GlobalEvictIxArgs);
impl From<GlobalEvictIxArgs> for GlobalEvictIxData {
    fn from(args: GlobalEvictIxArgs) -> Self {
        Self(args)
    }
}
impl GlobalEvictIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm_buf = [0u8; 1];
        reader.read_exact(&mut maybe_discm_buf)?;
        let maybe_discm = maybe_discm_buf[0];
        if maybe_discm != GLOBAL_EVICT_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(GlobalEvictIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&[GLOBAL_EVICT_IX_DISCM])?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn global_evict_ix_with_program_id(
    program_id: Pubkey,
    keys: GlobalEvictKeys,
    args: GlobalEvictIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; GLOBAL_EVICT_IX_ACCOUNTS_LEN] = keys.into();
    let data: GlobalEvictIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn global_evict_ix(
    keys: GlobalEvictKeys,
    args: GlobalEvictIxArgs,
) -> std::io::Result<Instruction> {
    global_evict_ix_with_program_id(MANIFEST_PROGRAM_ID, keys, args)
}
pub fn global_evict_invoke_with_program_id(
    program_id: Pubkey,
    accounts: GlobalEvictAccounts<'_, '_>,
    args: GlobalEvictIxArgs,
) -> ProgramResult {
    let keys: GlobalEvictKeys = accounts.into();
    let ix = global_evict_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn global_evict_invoke(
    accounts: GlobalEvictAccounts<'_, '_>,
    args: GlobalEvictIxArgs,
) -> ProgramResult {
    global_evict_invoke_with_program_id(MANIFEST_PROGRAM_ID, accounts, args)
}
pub fn global_evict_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: GlobalEvictAccounts<'_, '_>,
    args: GlobalEvictIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: GlobalEvictKeys = accounts.into();
    let ix = global_evict_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn global_evict_invoke_signed(
    accounts: GlobalEvictAccounts<'_, '_>,
    args: GlobalEvictIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    global_evict_invoke_signed_with_program_id(
        MANIFEST_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn global_evict_verify_account_keys(
    accounts: GlobalEvictAccounts<'_, '_>,
    keys: GlobalEvictKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.payer.key, &keys.payer),
        (accounts.global.key, &keys.global),
        (accounts.mint.key, &keys.mint),
        (accounts.global_vault.key, &keys.global_vault),
        (accounts.trader_token.key, &keys.trader_token),
        (accounts.evictee_token.key, &keys.evictee_token),
        (accounts.token_program.key, &keys.token_program),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn global_evict_verify_writable_privileges<'me, 'info>(
    accounts: GlobalEvictAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.payer, accounts.global, accounts.global_vault] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn global_evict_verify_signer_privileges<'me, 'info>(
    accounts: GlobalEvictAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.payer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn global_evict_verify_account_privileges<'me, 'info>(
    accounts: GlobalEvictAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    global_evict_verify_writable_privileges(accounts)?;
    global_evict_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const GLOBAL_CLEAN_IX_ACCOUNTS_LEN: usize = 4;
#[derive(Copy, Clone, Debug)]
pub struct GlobalCleanAccounts<'me, 'info> {
    pub payer: &'me AccountInfo<'info>,
    pub market: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub global: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct GlobalCleanKeys {
    pub payer: Pubkey,
    pub market: Pubkey,
    pub system_program: Pubkey,
    pub global: Pubkey,
}
impl From<GlobalCleanAccounts<'_, '_>> for GlobalCleanKeys {
    fn from(accounts: GlobalCleanAccounts) -> Self {
        Self {
            payer: *accounts.payer.key,
            market: *accounts.market.key,
            system_program: *accounts.system_program.key,
            global: *accounts.global.key,
        }
    }
}
impl From<GlobalCleanKeys> for [AccountMeta; GLOBAL_CLEAN_IX_ACCOUNTS_LEN] {
    fn from(keys: GlobalCleanKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.payer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.market,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.system_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.global,
                is_signer: false,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; GLOBAL_CLEAN_IX_ACCOUNTS_LEN]> for GlobalCleanKeys {
    fn from(pubkeys: [Pubkey; GLOBAL_CLEAN_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            payer: pubkeys[0],
            market: pubkeys[1],
            system_program: pubkeys[2],
            global: pubkeys[3],
        }
    }
}
impl<'info> From<GlobalCleanAccounts<'_, 'info>>
for [AccountInfo<'info>; GLOBAL_CLEAN_IX_ACCOUNTS_LEN] {
    fn from(accounts: GlobalCleanAccounts<'_, 'info>) -> Self {
        [
            accounts.payer.clone(),
            accounts.market.clone(),
            accounts.system_program.clone(),
            accounts.global.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; GLOBAL_CLEAN_IX_ACCOUNTS_LEN]>
for GlobalCleanAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; GLOBAL_CLEAN_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            payer: &arr[0],
            market: &arr[1],
            system_program: &arr[2],
            global: &arr[3],
        }
    }
}
pub const GLOBAL_CLEAN_IX_DISCM: u8 = 12u8;
#[derive(
    BorshDeserialize,
    BorshSerialize,
    Clone,
    Debug,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
pub struct GlobalCleanIxArgs {
    pub params: GlobalCleanParams,
}
impl GlobalCleanIxArgs {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let params = if reader.is_empty() {
            Default::default()
        } else {
            <GlobalCleanParams>::deserialize(&mut reader)?
        };
        *__buf = reader;
        Ok(Self { params })
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct GlobalCleanIxData(pub GlobalCleanIxArgs);
impl From<GlobalCleanIxArgs> for GlobalCleanIxData {
    fn from(args: GlobalCleanIxArgs) -> Self {
        Self(args)
    }
}
impl GlobalCleanIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm_buf = [0u8; 1];
        reader.read_exact(&mut maybe_discm_buf)?;
        let maybe_discm = maybe_discm_buf[0];
        if maybe_discm != GLOBAL_CLEAN_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(GlobalCleanIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&[GLOBAL_CLEAN_IX_DISCM])?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn global_clean_ix_with_program_id(
    program_id: Pubkey,
    keys: GlobalCleanKeys,
    args: GlobalCleanIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; GLOBAL_CLEAN_IX_ACCOUNTS_LEN] = keys.into();
    let data: GlobalCleanIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn global_clean_ix(
    keys: GlobalCleanKeys,
    args: GlobalCleanIxArgs,
) -> std::io::Result<Instruction> {
    global_clean_ix_with_program_id(MANIFEST_PROGRAM_ID, keys, args)
}
pub fn global_clean_invoke_with_program_id(
    program_id: Pubkey,
    accounts: GlobalCleanAccounts<'_, '_>,
    args: GlobalCleanIxArgs,
) -> ProgramResult {
    let keys: GlobalCleanKeys = accounts.into();
    let ix = global_clean_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn global_clean_invoke(
    accounts: GlobalCleanAccounts<'_, '_>,
    args: GlobalCleanIxArgs,
) -> ProgramResult {
    global_clean_invoke_with_program_id(MANIFEST_PROGRAM_ID, accounts, args)
}
pub fn global_clean_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: GlobalCleanAccounts<'_, '_>,
    args: GlobalCleanIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: GlobalCleanKeys = accounts.into();
    let ix = global_clean_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn global_clean_invoke_signed(
    accounts: GlobalCleanAccounts<'_, '_>,
    args: GlobalCleanIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    global_clean_invoke_signed_with_program_id(
        MANIFEST_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn global_clean_verify_account_keys(
    accounts: GlobalCleanAccounts<'_, '_>,
    keys: GlobalCleanKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.payer.key, &keys.payer),
        (accounts.market.key, &keys.market),
        (accounts.system_program.key, &keys.system_program),
        (accounts.global.key, &keys.global),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn global_clean_verify_writable_privileges<'me, 'info>(
    accounts: GlobalCleanAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.payer, accounts.market, accounts.global] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn global_clean_verify_signer_privileges<'me, 'info>(
    accounts: GlobalCleanAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.payer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn global_clean_verify_account_privileges<'me, 'info>(
    accounts: GlobalCleanAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    global_clean_verify_writable_privileges(accounts)?;
    global_clean_verify_signer_privileges(accounts)?;
    Ok(())
}
