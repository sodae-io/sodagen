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
pub enum PlasmaProgramIx {
    Swap(SwapIxArgs),
    AddLiquidity(AddLiquidityIxArgs),
    RemoveLiquidity(RemoveLiquidityIxArgs),
    RenounceLiquidity(RenounceLiquidityIxArgs),
    WithdrawLpFees,
    InitializeLpPosition,
    InitializePool(InitializePoolIxArgs),
    WithdrawProtocolFees,
    Log,
    TransferLiquidity,
}
impl PlasmaProgramIx {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm_buf = [0u8; 1];
        reader.read_exact(&mut maybe_discm_buf)?;
        let maybe_discm = maybe_discm_buf[0];
        match maybe_discm {
            SWAP_IX_DISCM => Ok(Self::Swap(SwapIxArgs::deserialize(&mut reader)?)),
            ADD_LIQUIDITY_IX_DISCM => {
                Ok(Self::AddLiquidity(AddLiquidityIxArgs::deserialize(&mut reader)?))
            }
            REMOVE_LIQUIDITY_IX_DISCM => {
                Ok(
                    Self::RemoveLiquidity(
                        RemoveLiquidityIxArgs::deserialize(&mut reader)?,
                    ),
                )
            }
            RENOUNCE_LIQUIDITY_IX_DISCM => {
                Ok(
                    Self::RenounceLiquidity(
                        RenounceLiquidityIxArgs::deserialize(&mut reader)?,
                    ),
                )
            }
            WITHDRAW_LP_FEES_IX_DISCM => Ok(Self::WithdrawLpFees),
            INITIALIZE_LP_POSITION_IX_DISCM => Ok(Self::InitializeLpPosition),
            INITIALIZE_POOL_IX_DISCM => {
                Ok(Self::InitializePool(InitializePoolIxArgs::deserialize(&mut reader)?))
            }
            WITHDRAW_PROTOCOL_FEES_IX_DISCM => Ok(Self::WithdrawProtocolFees),
            LOG_IX_DISCM => Ok(Self::Log),
            TRANSFER_LIQUIDITY_IX_DISCM => Ok(Self::TransferLiquidity),
            _ => Err(std::io::Error::from(std::io::ErrorKind::InvalidData)),
        }
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        match self {
            Self::Swap(args) => {
                writer.write_all(&[SWAP_IX_DISCM])?;
                args.serialize(&mut writer)
            }
            Self::AddLiquidity(args) => {
                writer.write_all(&[ADD_LIQUIDITY_IX_DISCM])?;
                args.serialize(&mut writer)
            }
            Self::RemoveLiquidity(args) => {
                writer.write_all(&[REMOVE_LIQUIDITY_IX_DISCM])?;
                args.serialize(&mut writer)
            }
            Self::RenounceLiquidity(args) => {
                writer.write_all(&[RENOUNCE_LIQUIDITY_IX_DISCM])?;
                args.serialize(&mut writer)
            }
            Self::WithdrawLpFees => writer.write_all(&[WITHDRAW_LP_FEES_IX_DISCM]),
            Self::InitializeLpPosition => {
                writer.write_all(&[INITIALIZE_LP_POSITION_IX_DISCM])
            }
            Self::InitializePool(args) => {
                writer.write_all(&[INITIALIZE_POOL_IX_DISCM])?;
                args.serialize(&mut writer)
            }
            Self::WithdrawProtocolFees => {
                writer.write_all(&[WITHDRAW_PROTOCOL_FEES_IX_DISCM])
            }
            Self::Log => writer.write_all(&[LOG_IX_DISCM]),
            Self::TransferLiquidity => writer.write_all(&[TRANSFER_LIQUIDITY_IX_DISCM]),
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
pub const SWAP_IX_ACCOUNTS_LEN: usize = 9;
#[derive(Copy, Clone, Debug)]
pub struct SwapAccounts<'me, 'info> {
    pub plasma_program: &'me AccountInfo<'info>,
    pub log_authority: &'me AccountInfo<'info>,
    pub pool: &'me AccountInfo<'info>,
    pub trader: &'me AccountInfo<'info>,
    pub base_account: &'me AccountInfo<'info>,
    pub quote_account: &'me AccountInfo<'info>,
    pub base_vault: &'me AccountInfo<'info>,
    pub quote_vault: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct SwapKeys {
    pub plasma_program: Pubkey,
    pub log_authority: Pubkey,
    pub pool: Pubkey,
    pub trader: Pubkey,
    pub base_account: Pubkey,
    pub quote_account: Pubkey,
    pub base_vault: Pubkey,
    pub quote_vault: Pubkey,
    pub token_program: Pubkey,
}
impl From<SwapAccounts<'_, '_>> for SwapKeys {
    fn from(accounts: SwapAccounts) -> Self {
        Self {
            plasma_program: *accounts.plasma_program.key,
            log_authority: *accounts.log_authority.key,
            pool: *accounts.pool.key,
            trader: *accounts.trader.key,
            base_account: *accounts.base_account.key,
            quote_account: *accounts.quote_account.key,
            base_vault: *accounts.base_vault.key,
            quote_vault: *accounts.quote_vault.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<SwapKeys> for [AccountMeta; SWAP_IX_ACCOUNTS_LEN] {
    fn from(keys: SwapKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.plasma_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.log_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.trader,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.base_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.quote_account,
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
            plasma_program: pubkeys[0],
            log_authority: pubkeys[1],
            pool: pubkeys[2],
            trader: pubkeys[3],
            base_account: pubkeys[4],
            quote_account: pubkeys[5],
            base_vault: pubkeys[6],
            quote_vault: pubkeys[7],
            token_program: pubkeys[8],
        }
    }
}
impl<'info> From<SwapAccounts<'_, 'info>>
for [AccountInfo<'info>; SWAP_IX_ACCOUNTS_LEN] {
    fn from(accounts: SwapAccounts<'_, 'info>) -> Self {
        [
            accounts.plasma_program.clone(),
            accounts.log_authority.clone(),
            accounts.pool.clone(),
            accounts.trader.clone(),
            accounts.base_account.clone(),
            accounts.quote_account.clone(),
            accounts.base_vault.clone(),
            accounts.quote_vault.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SWAP_IX_ACCOUNTS_LEN]>
for SwapAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; SWAP_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            plasma_program: &arr[0],
            log_authority: &arr[1],
            pool: &arr[2],
            trader: &arr[3],
            base_account: &arr[4],
            quote_account: &arr[5],
            base_vault: &arr[6],
            quote_vault: &arr[7],
            token_program: &arr[8],
        }
    }
}
pub const SWAP_IX_DISCM: u8 = 0u8;
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
    pub params: SwapIxParams,
}
impl SwapIxArgs {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let params = <SwapIxParams>::deserialize(&mut reader)?;
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
    swap_ix_with_program_id(PLASMA_PROGRAM_ID, keys, args)
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
    swap_invoke_with_program_id(PLASMA_PROGRAM_ID, accounts, args)
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
    swap_invoke_signed_with_program_id(PLASMA_PROGRAM_ID, accounts, args, seeds)
}
pub fn swap_verify_account_keys(
    accounts: SwapAccounts<'_, '_>,
    keys: SwapKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.plasma_program.key, &keys.plasma_program),
        (accounts.log_authority.key, &keys.log_authority),
        (accounts.pool.key, &keys.pool),
        (accounts.trader.key, &keys.trader),
        (accounts.base_account.key, &keys.base_account),
        (accounts.quote_account.key, &keys.quote_account),
        (accounts.base_vault.key, &keys.base_vault),
        (accounts.quote_vault.key, &keys.quote_vault),
        (accounts.token_program.key, &keys.token_program),
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
        accounts.pool,
        accounts.base_account,
        accounts.quote_account,
        accounts.base_vault,
        accounts.quote_vault,
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
    for should_be_signer in [accounts.trader] {
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
pub const ADD_LIQUIDITY_IX_ACCOUNTS_LEN: usize = 10;
#[derive(Copy, Clone, Debug)]
pub struct AddLiquidityAccounts<'me, 'info> {
    pub plasma_program: &'me AccountInfo<'info>,
    pub log_authority: &'me AccountInfo<'info>,
    pub pool: &'me AccountInfo<'info>,
    pub trader: &'me AccountInfo<'info>,
    pub lp_position: &'me AccountInfo<'info>,
    pub base_account: &'me AccountInfo<'info>,
    pub quote_account: &'me AccountInfo<'info>,
    pub base_vault: &'me AccountInfo<'info>,
    pub quote_vault: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct AddLiquidityKeys {
    pub plasma_program: Pubkey,
    pub log_authority: Pubkey,
    pub pool: Pubkey,
    pub trader: Pubkey,
    pub lp_position: Pubkey,
    pub base_account: Pubkey,
    pub quote_account: Pubkey,
    pub base_vault: Pubkey,
    pub quote_vault: Pubkey,
    pub token_program: Pubkey,
}
impl From<AddLiquidityAccounts<'_, '_>> for AddLiquidityKeys {
    fn from(accounts: AddLiquidityAccounts) -> Self {
        Self {
            plasma_program: *accounts.plasma_program.key,
            log_authority: *accounts.log_authority.key,
            pool: *accounts.pool.key,
            trader: *accounts.trader.key,
            lp_position: *accounts.lp_position.key,
            base_account: *accounts.base_account.key,
            quote_account: *accounts.quote_account.key,
            base_vault: *accounts.base_vault.key,
            quote_vault: *accounts.quote_vault.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<AddLiquidityKeys> for [AccountMeta; ADD_LIQUIDITY_IX_ACCOUNTS_LEN] {
    fn from(keys: AddLiquidityKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.plasma_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.log_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.trader,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.lp_position,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.base_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.quote_account,
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
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; ADD_LIQUIDITY_IX_ACCOUNTS_LEN]> for AddLiquidityKeys {
    fn from(pubkeys: [Pubkey; ADD_LIQUIDITY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            plasma_program: pubkeys[0],
            log_authority: pubkeys[1],
            pool: pubkeys[2],
            trader: pubkeys[3],
            lp_position: pubkeys[4],
            base_account: pubkeys[5],
            quote_account: pubkeys[6],
            base_vault: pubkeys[7],
            quote_vault: pubkeys[8],
            token_program: pubkeys[9],
        }
    }
}
impl<'info> From<AddLiquidityAccounts<'_, 'info>>
for [AccountInfo<'info>; ADD_LIQUIDITY_IX_ACCOUNTS_LEN] {
    fn from(accounts: AddLiquidityAccounts<'_, 'info>) -> Self {
        [
            accounts.plasma_program.clone(),
            accounts.log_authority.clone(),
            accounts.pool.clone(),
            accounts.trader.clone(),
            accounts.lp_position.clone(),
            accounts.base_account.clone(),
            accounts.quote_account.clone(),
            accounts.base_vault.clone(),
            accounts.quote_vault.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; ADD_LIQUIDITY_IX_ACCOUNTS_LEN]>
for AddLiquidityAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; ADD_LIQUIDITY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            plasma_program: &arr[0],
            log_authority: &arr[1],
            pool: &arr[2],
            trader: &arr[3],
            lp_position: &arr[4],
            base_account: &arr[5],
            quote_account: &arr[6],
            base_vault: &arr[7],
            quote_vault: &arr[8],
            token_program: &arr[9],
        }
    }
}
pub const ADD_LIQUIDITY_IX_DISCM: u8 = 1u8;
#[derive(
    BorshDeserialize,
    BorshSerialize,
    Clone,
    Debug,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
pub struct AddLiquidityIxArgs {
    pub params: AddLiquidityIxParams,
}
impl AddLiquidityIxArgs {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let params = if reader.is_empty() {
            Default::default()
        } else {
            <AddLiquidityIxParams>::deserialize(&mut reader)?
        };
        *__buf = reader;
        Ok(Self { params })
    }
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
        let mut maybe_discm_buf = [0u8; 1];
        reader.read_exact(&mut maybe_discm_buf)?;
        let maybe_discm = maybe_discm_buf[0];
        if maybe_discm != ADD_LIQUIDITY_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(AddLiquidityIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&[ADD_LIQUIDITY_IX_DISCM])?;
        self.0.serialize(&mut writer)
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
    add_liquidity_ix_with_program_id(PLASMA_PROGRAM_ID, keys, args)
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
    add_liquidity_invoke_with_program_id(PLASMA_PROGRAM_ID, accounts, args)
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
    add_liquidity_invoke_signed_with_program_id(PLASMA_PROGRAM_ID, accounts, args, seeds)
}
pub fn add_liquidity_verify_account_keys(
    accounts: AddLiquidityAccounts<'_, '_>,
    keys: AddLiquidityKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.plasma_program.key, &keys.plasma_program),
        (accounts.log_authority.key, &keys.log_authority),
        (accounts.pool.key, &keys.pool),
        (accounts.trader.key, &keys.trader),
        (accounts.lp_position.key, &keys.lp_position),
        (accounts.base_account.key, &keys.base_account),
        (accounts.quote_account.key, &keys.quote_account),
        (accounts.base_vault.key, &keys.base_vault),
        (accounts.quote_vault.key, &keys.quote_vault),
        (accounts.token_program.key, &keys.token_program),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn add_liquidity_verify_writable_privileges<'me, 'info>(
    accounts: AddLiquidityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.pool,
        accounts.lp_position,
        accounts.base_account,
        accounts.quote_account,
        accounts.base_vault,
        accounts.quote_vault,
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
    for should_be_signer in [accounts.trader] {
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
pub const REMOVE_LIQUIDITY_IX_ACCOUNTS_LEN: usize = 10;
#[derive(Copy, Clone, Debug)]
pub struct RemoveLiquidityAccounts<'me, 'info> {
    pub plasma_program: &'me AccountInfo<'info>,
    pub log_authority: &'me AccountInfo<'info>,
    pub pool: &'me AccountInfo<'info>,
    pub trader: &'me AccountInfo<'info>,
    pub lp_position: &'me AccountInfo<'info>,
    pub base_account: &'me AccountInfo<'info>,
    pub quote_account: &'me AccountInfo<'info>,
    pub base_vault: &'me AccountInfo<'info>,
    pub quote_vault: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct RemoveLiquidityKeys {
    pub plasma_program: Pubkey,
    pub log_authority: Pubkey,
    pub pool: Pubkey,
    pub trader: Pubkey,
    pub lp_position: Pubkey,
    pub base_account: Pubkey,
    pub quote_account: Pubkey,
    pub base_vault: Pubkey,
    pub quote_vault: Pubkey,
    pub token_program: Pubkey,
}
impl From<RemoveLiquidityAccounts<'_, '_>> for RemoveLiquidityKeys {
    fn from(accounts: RemoveLiquidityAccounts) -> Self {
        Self {
            plasma_program: *accounts.plasma_program.key,
            log_authority: *accounts.log_authority.key,
            pool: *accounts.pool.key,
            trader: *accounts.trader.key,
            lp_position: *accounts.lp_position.key,
            base_account: *accounts.base_account.key,
            quote_account: *accounts.quote_account.key,
            base_vault: *accounts.base_vault.key,
            quote_vault: *accounts.quote_vault.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<RemoveLiquidityKeys> for [AccountMeta; REMOVE_LIQUIDITY_IX_ACCOUNTS_LEN] {
    fn from(keys: RemoveLiquidityKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.plasma_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.log_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.trader,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.lp_position,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.base_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.quote_account,
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
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; REMOVE_LIQUIDITY_IX_ACCOUNTS_LEN]> for RemoveLiquidityKeys {
    fn from(pubkeys: [Pubkey; REMOVE_LIQUIDITY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            plasma_program: pubkeys[0],
            log_authority: pubkeys[1],
            pool: pubkeys[2],
            trader: pubkeys[3],
            lp_position: pubkeys[4],
            base_account: pubkeys[5],
            quote_account: pubkeys[6],
            base_vault: pubkeys[7],
            quote_vault: pubkeys[8],
            token_program: pubkeys[9],
        }
    }
}
impl<'info> From<RemoveLiquidityAccounts<'_, 'info>>
for [AccountInfo<'info>; REMOVE_LIQUIDITY_IX_ACCOUNTS_LEN] {
    fn from(accounts: RemoveLiquidityAccounts<'_, 'info>) -> Self {
        [
            accounts.plasma_program.clone(),
            accounts.log_authority.clone(),
            accounts.pool.clone(),
            accounts.trader.clone(),
            accounts.lp_position.clone(),
            accounts.base_account.clone(),
            accounts.quote_account.clone(),
            accounts.base_vault.clone(),
            accounts.quote_vault.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; REMOVE_LIQUIDITY_IX_ACCOUNTS_LEN]>
for RemoveLiquidityAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; REMOVE_LIQUIDITY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            plasma_program: &arr[0],
            log_authority: &arr[1],
            pool: &arr[2],
            trader: &arr[3],
            lp_position: &arr[4],
            base_account: &arr[5],
            quote_account: &arr[6],
            base_vault: &arr[7],
            quote_vault: &arr[8],
            token_program: &arr[9],
        }
    }
}
pub const REMOVE_LIQUIDITY_IX_DISCM: u8 = 2u8;
#[derive(
    BorshDeserialize,
    BorshSerialize,
    Clone,
    Debug,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
pub struct RemoveLiquidityIxArgs {
    pub params: RemoveLiquidityIxParams,
}
impl RemoveLiquidityIxArgs {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let params = if reader.is_empty() {
            Default::default()
        } else {
            <RemoveLiquidityIxParams>::deserialize(&mut reader)?
        };
        *__buf = reader;
        Ok(Self { params })
    }
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
        let mut maybe_discm_buf = [0u8; 1];
        reader.read_exact(&mut maybe_discm_buf)?;
        let maybe_discm = maybe_discm_buf[0];
        if maybe_discm != REMOVE_LIQUIDITY_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(RemoveLiquidityIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&[REMOVE_LIQUIDITY_IX_DISCM])?;
        self.0.serialize(&mut writer)
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
    remove_liquidity_ix_with_program_id(PLASMA_PROGRAM_ID, keys, args)
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
    remove_liquidity_invoke_with_program_id(PLASMA_PROGRAM_ID, accounts, args)
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
        PLASMA_PROGRAM_ID,
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
        (accounts.plasma_program.key, &keys.plasma_program),
        (accounts.log_authority.key, &keys.log_authority),
        (accounts.pool.key, &keys.pool),
        (accounts.trader.key, &keys.trader),
        (accounts.lp_position.key, &keys.lp_position),
        (accounts.base_account.key, &keys.base_account),
        (accounts.quote_account.key, &keys.quote_account),
        (accounts.base_vault.key, &keys.base_vault),
        (accounts.quote_vault.key, &keys.quote_vault),
        (accounts.token_program.key, &keys.token_program),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn remove_liquidity_verify_writable_privileges<'me, 'info>(
    accounts: RemoveLiquidityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.pool,
        accounts.lp_position,
        accounts.base_account,
        accounts.quote_account,
        accounts.base_vault,
        accounts.quote_vault,
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
    for should_be_signer in [accounts.trader] {
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
pub const RENOUNCE_LIQUIDITY_IX_ACCOUNTS_LEN: usize = 5;
#[derive(Copy, Clone, Debug)]
pub struct RenounceLiquidityAccounts<'me, 'info> {
    pub plasma_program: &'me AccountInfo<'info>,
    pub log_authority: &'me AccountInfo<'info>,
    pub pool: &'me AccountInfo<'info>,
    pub trader: &'me AccountInfo<'info>,
    pub lp_position: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct RenounceLiquidityKeys {
    pub plasma_program: Pubkey,
    pub log_authority: Pubkey,
    pub pool: Pubkey,
    pub trader: Pubkey,
    pub lp_position: Pubkey,
}
impl From<RenounceLiquidityAccounts<'_, '_>> for RenounceLiquidityKeys {
    fn from(accounts: RenounceLiquidityAccounts) -> Self {
        Self {
            plasma_program: *accounts.plasma_program.key,
            log_authority: *accounts.log_authority.key,
            pool: *accounts.pool.key,
            trader: *accounts.trader.key,
            lp_position: *accounts.lp_position.key,
        }
    }
}
impl From<RenounceLiquidityKeys> for [AccountMeta; RENOUNCE_LIQUIDITY_IX_ACCOUNTS_LEN] {
    fn from(keys: RenounceLiquidityKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.plasma_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.log_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.trader,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.lp_position,
                is_signer: false,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; RENOUNCE_LIQUIDITY_IX_ACCOUNTS_LEN]> for RenounceLiquidityKeys {
    fn from(pubkeys: [Pubkey; RENOUNCE_LIQUIDITY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            plasma_program: pubkeys[0],
            log_authority: pubkeys[1],
            pool: pubkeys[2],
            trader: pubkeys[3],
            lp_position: pubkeys[4],
        }
    }
}
impl<'info> From<RenounceLiquidityAccounts<'_, 'info>>
for [AccountInfo<'info>; RENOUNCE_LIQUIDITY_IX_ACCOUNTS_LEN] {
    fn from(accounts: RenounceLiquidityAccounts<'_, 'info>) -> Self {
        [
            accounts.plasma_program.clone(),
            accounts.log_authority.clone(),
            accounts.pool.clone(),
            accounts.trader.clone(),
            accounts.lp_position.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; RENOUNCE_LIQUIDITY_IX_ACCOUNTS_LEN]>
for RenounceLiquidityAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; RENOUNCE_LIQUIDITY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            plasma_program: &arr[0],
            log_authority: &arr[1],
            pool: &arr[2],
            trader: &arr[3],
            lp_position: &arr[4],
        }
    }
}
pub const RENOUNCE_LIQUIDITY_IX_DISCM: u8 = 3u8;
#[derive(
    BorshDeserialize,
    BorshSerialize,
    Clone,
    Debug,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
pub struct RenounceLiquidityIxArgs {
    pub params: RenounceLiquidityIxParams,
}
impl RenounceLiquidityIxArgs {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let params = if reader.is_empty() {
            Default::default()
        } else {
            <RenounceLiquidityIxParams>::deserialize(&mut reader)?
        };
        *__buf = reader;
        Ok(Self { params })
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct RenounceLiquidityIxData(pub RenounceLiquidityIxArgs);
impl From<RenounceLiquidityIxArgs> for RenounceLiquidityIxData {
    fn from(args: RenounceLiquidityIxArgs) -> Self {
        Self(args)
    }
}
impl RenounceLiquidityIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm_buf = [0u8; 1];
        reader.read_exact(&mut maybe_discm_buf)?;
        let maybe_discm = maybe_discm_buf[0];
        if maybe_discm != RENOUNCE_LIQUIDITY_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(RenounceLiquidityIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&[RENOUNCE_LIQUIDITY_IX_DISCM])?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn renounce_liquidity_ix_with_program_id(
    program_id: Pubkey,
    keys: RenounceLiquidityKeys,
    args: RenounceLiquidityIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; RENOUNCE_LIQUIDITY_IX_ACCOUNTS_LEN] = keys.into();
    let data: RenounceLiquidityIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn renounce_liquidity_ix(
    keys: RenounceLiquidityKeys,
    args: RenounceLiquidityIxArgs,
) -> std::io::Result<Instruction> {
    renounce_liquidity_ix_with_program_id(PLASMA_PROGRAM_ID, keys, args)
}
pub fn renounce_liquidity_invoke_with_program_id(
    program_id: Pubkey,
    accounts: RenounceLiquidityAccounts<'_, '_>,
    args: RenounceLiquidityIxArgs,
) -> ProgramResult {
    let keys: RenounceLiquidityKeys = accounts.into();
    let ix = renounce_liquidity_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn renounce_liquidity_invoke(
    accounts: RenounceLiquidityAccounts<'_, '_>,
    args: RenounceLiquidityIxArgs,
) -> ProgramResult {
    renounce_liquidity_invoke_with_program_id(PLASMA_PROGRAM_ID, accounts, args)
}
pub fn renounce_liquidity_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: RenounceLiquidityAccounts<'_, '_>,
    args: RenounceLiquidityIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: RenounceLiquidityKeys = accounts.into();
    let ix = renounce_liquidity_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn renounce_liquidity_invoke_signed(
    accounts: RenounceLiquidityAccounts<'_, '_>,
    args: RenounceLiquidityIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    renounce_liquidity_invoke_signed_with_program_id(
        PLASMA_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn renounce_liquidity_verify_account_keys(
    accounts: RenounceLiquidityAccounts<'_, '_>,
    keys: RenounceLiquidityKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.plasma_program.key, &keys.plasma_program),
        (accounts.log_authority.key, &keys.log_authority),
        (accounts.pool.key, &keys.pool),
        (accounts.trader.key, &keys.trader),
        (accounts.lp_position.key, &keys.lp_position),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn renounce_liquidity_verify_writable_privileges<'me, 'info>(
    accounts: RenounceLiquidityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.pool, accounts.lp_position] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn renounce_liquidity_verify_signer_privileges<'me, 'info>(
    accounts: RenounceLiquidityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.trader] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn renounce_liquidity_verify_account_privileges<'me, 'info>(
    accounts: RenounceLiquidityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    renounce_liquidity_verify_writable_privileges(accounts)?;
    renounce_liquidity_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const WITHDRAW_LP_FEES_IX_ACCOUNTS_LEN: usize = 9;
#[derive(Copy, Clone, Debug)]
pub struct WithdrawLpFeesAccounts<'me, 'info> {
    pub plasma_program: &'me AccountInfo<'info>,
    pub log_authority: &'me AccountInfo<'info>,
    pub pool: &'me AccountInfo<'info>,
    pub trader: &'me AccountInfo<'info>,
    pub lp_position_owner: &'me AccountInfo<'info>,
    pub lp_position: &'me AccountInfo<'info>,
    pub quote_account: &'me AccountInfo<'info>,
    pub quote_vault: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct WithdrawLpFeesKeys {
    pub plasma_program: Pubkey,
    pub log_authority: Pubkey,
    pub pool: Pubkey,
    pub trader: Pubkey,
    pub lp_position_owner: Pubkey,
    pub lp_position: Pubkey,
    pub quote_account: Pubkey,
    pub quote_vault: Pubkey,
    pub token_program: Pubkey,
}
impl From<WithdrawLpFeesAccounts<'_, '_>> for WithdrawLpFeesKeys {
    fn from(accounts: WithdrawLpFeesAccounts) -> Self {
        Self {
            plasma_program: *accounts.plasma_program.key,
            log_authority: *accounts.log_authority.key,
            pool: *accounts.pool.key,
            trader: *accounts.trader.key,
            lp_position_owner: *accounts.lp_position_owner.key,
            lp_position: *accounts.lp_position.key,
            quote_account: *accounts.quote_account.key,
            quote_vault: *accounts.quote_vault.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<WithdrawLpFeesKeys> for [AccountMeta; WITHDRAW_LP_FEES_IX_ACCOUNTS_LEN] {
    fn from(keys: WithdrawLpFeesKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.plasma_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.log_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.trader,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.lp_position_owner,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.lp_position,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.quote_account,
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
        ]
    }
}
impl From<[Pubkey; WITHDRAW_LP_FEES_IX_ACCOUNTS_LEN]> for WithdrawLpFeesKeys {
    fn from(pubkeys: [Pubkey; WITHDRAW_LP_FEES_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            plasma_program: pubkeys[0],
            log_authority: pubkeys[1],
            pool: pubkeys[2],
            trader: pubkeys[3],
            lp_position_owner: pubkeys[4],
            lp_position: pubkeys[5],
            quote_account: pubkeys[6],
            quote_vault: pubkeys[7],
            token_program: pubkeys[8],
        }
    }
}
impl<'info> From<WithdrawLpFeesAccounts<'_, 'info>>
for [AccountInfo<'info>; WITHDRAW_LP_FEES_IX_ACCOUNTS_LEN] {
    fn from(accounts: WithdrawLpFeesAccounts<'_, 'info>) -> Self {
        [
            accounts.plasma_program.clone(),
            accounts.log_authority.clone(),
            accounts.pool.clone(),
            accounts.trader.clone(),
            accounts.lp_position_owner.clone(),
            accounts.lp_position.clone(),
            accounts.quote_account.clone(),
            accounts.quote_vault.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; WITHDRAW_LP_FEES_IX_ACCOUNTS_LEN]>
for WithdrawLpFeesAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; WITHDRAW_LP_FEES_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            plasma_program: &arr[0],
            log_authority: &arr[1],
            pool: &arr[2],
            trader: &arr[3],
            lp_position_owner: &arr[4],
            lp_position: &arr[5],
            quote_account: &arr[6],
            quote_vault: &arr[7],
            token_program: &arr[8],
        }
    }
}
pub const WITHDRAW_LP_FEES_IX_DISCM: u8 = 4u8;
#[derive(Clone, Debug, PartialEq)]
pub struct WithdrawLpFeesIxData;
impl WithdrawLpFeesIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm_buf = [0u8; 1];
        reader.read_exact(&mut maybe_discm_buf)?;
        let maybe_discm = maybe_discm_buf[0];
        if maybe_discm != WITHDRAW_LP_FEES_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&[WITHDRAW_LP_FEES_IX_DISCM])
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn withdraw_lp_fees_ix_with_program_id(
    program_id: Pubkey,
    keys: WithdrawLpFeesKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; WITHDRAW_LP_FEES_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: WithdrawLpFeesIxData.try_to_vec()?,
    })
}
pub fn withdraw_lp_fees_ix(keys: WithdrawLpFeesKeys) -> std::io::Result<Instruction> {
    withdraw_lp_fees_ix_with_program_id(PLASMA_PROGRAM_ID, keys)
}
pub fn withdraw_lp_fees_invoke_with_program_id(
    program_id: Pubkey,
    accounts: WithdrawLpFeesAccounts<'_, '_>,
) -> ProgramResult {
    let keys: WithdrawLpFeesKeys = accounts.into();
    let ix = withdraw_lp_fees_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn withdraw_lp_fees_invoke(
    accounts: WithdrawLpFeesAccounts<'_, '_>,
) -> ProgramResult {
    withdraw_lp_fees_invoke_with_program_id(PLASMA_PROGRAM_ID, accounts)
}
pub fn withdraw_lp_fees_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: WithdrawLpFeesAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: WithdrawLpFeesKeys = accounts.into();
    let ix = withdraw_lp_fees_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn withdraw_lp_fees_invoke_signed(
    accounts: WithdrawLpFeesAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    withdraw_lp_fees_invoke_signed_with_program_id(PLASMA_PROGRAM_ID, accounts, seeds)
}
pub fn withdraw_lp_fees_verify_account_keys(
    accounts: WithdrawLpFeesAccounts<'_, '_>,
    keys: WithdrawLpFeesKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.plasma_program.key, &keys.plasma_program),
        (accounts.log_authority.key, &keys.log_authority),
        (accounts.pool.key, &keys.pool),
        (accounts.trader.key, &keys.trader),
        (accounts.lp_position_owner.key, &keys.lp_position_owner),
        (accounts.lp_position.key, &keys.lp_position),
        (accounts.quote_account.key, &keys.quote_account),
        (accounts.quote_vault.key, &keys.quote_vault),
        (accounts.token_program.key, &keys.token_program),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn withdraw_lp_fees_verify_writable_privileges<'me, 'info>(
    accounts: WithdrawLpFeesAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.pool,
        accounts.lp_position,
        accounts.quote_account,
        accounts.quote_vault,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn withdraw_lp_fees_verify_signer_privileges<'me, 'info>(
    accounts: WithdrawLpFeesAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.trader] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn withdraw_lp_fees_verify_account_privileges<'me, 'info>(
    accounts: WithdrawLpFeesAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    withdraw_lp_fees_verify_writable_privileges(accounts)?;
    withdraw_lp_fees_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const INITIALIZE_LP_POSITION_IX_ACCOUNTS_LEN: usize = 7;
#[derive(Copy, Clone, Debug)]
pub struct InitializeLpPositionAccounts<'me, 'info> {
    pub plasma_program: &'me AccountInfo<'info>,
    pub log_authority: &'me AccountInfo<'info>,
    pub pool: &'me AccountInfo<'info>,
    pub payer: &'me AccountInfo<'info>,
    pub lp_position_owner: &'me AccountInfo<'info>,
    pub lp_position: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct InitializeLpPositionKeys {
    pub plasma_program: Pubkey,
    pub log_authority: Pubkey,
    pub pool: Pubkey,
    pub payer: Pubkey,
    pub lp_position_owner: Pubkey,
    pub lp_position: Pubkey,
    pub system_program: Pubkey,
}
impl From<InitializeLpPositionAccounts<'_, '_>> for InitializeLpPositionKeys {
    fn from(accounts: InitializeLpPositionAccounts) -> Self {
        Self {
            plasma_program: *accounts.plasma_program.key,
            log_authority: *accounts.log_authority.key,
            pool: *accounts.pool.key,
            payer: *accounts.payer.key,
            lp_position_owner: *accounts.lp_position_owner.key,
            lp_position: *accounts.lp_position.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<InitializeLpPositionKeys>
for [AccountMeta; INITIALIZE_LP_POSITION_IX_ACCOUNTS_LEN] {
    fn from(keys: InitializeLpPositionKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.plasma_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.log_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.payer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.lp_position_owner,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.lp_position,
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
impl From<[Pubkey; INITIALIZE_LP_POSITION_IX_ACCOUNTS_LEN]>
for InitializeLpPositionKeys {
    fn from(pubkeys: [Pubkey; INITIALIZE_LP_POSITION_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            plasma_program: pubkeys[0],
            log_authority: pubkeys[1],
            pool: pubkeys[2],
            payer: pubkeys[3],
            lp_position_owner: pubkeys[4],
            lp_position: pubkeys[5],
            system_program: pubkeys[6],
        }
    }
}
impl<'info> From<InitializeLpPositionAccounts<'_, 'info>>
for [AccountInfo<'info>; INITIALIZE_LP_POSITION_IX_ACCOUNTS_LEN] {
    fn from(accounts: InitializeLpPositionAccounts<'_, 'info>) -> Self {
        [
            accounts.plasma_program.clone(),
            accounts.log_authority.clone(),
            accounts.pool.clone(),
            accounts.payer.clone(),
            accounts.lp_position_owner.clone(),
            accounts.lp_position.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; INITIALIZE_LP_POSITION_IX_ACCOUNTS_LEN]>
for InitializeLpPositionAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; INITIALIZE_LP_POSITION_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            plasma_program: &arr[0],
            log_authority: &arr[1],
            pool: &arr[2],
            payer: &arr[3],
            lp_position_owner: &arr[4],
            lp_position: &arr[5],
            system_program: &arr[6],
        }
    }
}
pub const INITIALIZE_LP_POSITION_IX_DISCM: u8 = 5u8;
#[derive(Clone, Debug, PartialEq)]
pub struct InitializeLpPositionIxData;
impl InitializeLpPositionIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm_buf = [0u8; 1];
        reader.read_exact(&mut maybe_discm_buf)?;
        let maybe_discm = maybe_discm_buf[0];
        if maybe_discm != INITIALIZE_LP_POSITION_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&[INITIALIZE_LP_POSITION_IX_DISCM])
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn initialize_lp_position_ix_with_program_id(
    program_id: Pubkey,
    keys: InitializeLpPositionKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; INITIALIZE_LP_POSITION_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: InitializeLpPositionIxData.try_to_vec()?,
    })
}
pub fn initialize_lp_position_ix(
    keys: InitializeLpPositionKeys,
) -> std::io::Result<Instruction> {
    initialize_lp_position_ix_with_program_id(PLASMA_PROGRAM_ID, keys)
}
pub fn initialize_lp_position_invoke_with_program_id(
    program_id: Pubkey,
    accounts: InitializeLpPositionAccounts<'_, '_>,
) -> ProgramResult {
    let keys: InitializeLpPositionKeys = accounts.into();
    let ix = initialize_lp_position_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn initialize_lp_position_invoke(
    accounts: InitializeLpPositionAccounts<'_, '_>,
) -> ProgramResult {
    initialize_lp_position_invoke_with_program_id(PLASMA_PROGRAM_ID, accounts)
}
pub fn initialize_lp_position_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: InitializeLpPositionAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: InitializeLpPositionKeys = accounts.into();
    let ix = initialize_lp_position_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn initialize_lp_position_invoke_signed(
    accounts: InitializeLpPositionAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    initialize_lp_position_invoke_signed_with_program_id(
        PLASMA_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn initialize_lp_position_verify_account_keys(
    accounts: InitializeLpPositionAccounts<'_, '_>,
    keys: InitializeLpPositionKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.plasma_program.key, &keys.plasma_program),
        (accounts.log_authority.key, &keys.log_authority),
        (accounts.pool.key, &keys.pool),
        (accounts.payer.key, &keys.payer),
        (accounts.lp_position_owner.key, &keys.lp_position_owner),
        (accounts.lp_position.key, &keys.lp_position),
        (accounts.system_program.key, &keys.system_program),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn initialize_lp_position_verify_writable_privileges<'me, 'info>(
    accounts: InitializeLpPositionAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.pool, accounts.payer, accounts.lp_position] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn initialize_lp_position_verify_signer_privileges<'me, 'info>(
    accounts: InitializeLpPositionAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.payer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn initialize_lp_position_verify_account_privileges<'me, 'info>(
    accounts: InitializeLpPositionAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    initialize_lp_position_verify_writable_privileges(accounts)?;
    initialize_lp_position_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const INITIALIZE_POOL_IX_ACCOUNTS_LEN: usize = 10;
#[derive(Copy, Clone, Debug)]
pub struct InitializePoolAccounts<'me, 'info> {
    pub plasma_program: &'me AccountInfo<'info>,
    pub log_authority: &'me AccountInfo<'info>,
    pub pool: &'me AccountInfo<'info>,
    pub pool_creator: &'me AccountInfo<'info>,
    pub base_mint: &'me AccountInfo<'info>,
    pub quote_mint: &'me AccountInfo<'info>,
    pub base_vault: &'me AccountInfo<'info>,
    pub quote_vault: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct InitializePoolKeys {
    pub plasma_program: Pubkey,
    pub log_authority: Pubkey,
    pub pool: Pubkey,
    pub pool_creator: Pubkey,
    pub base_mint: Pubkey,
    pub quote_mint: Pubkey,
    pub base_vault: Pubkey,
    pub quote_vault: Pubkey,
    pub system_program: Pubkey,
    pub token_program: Pubkey,
}
impl From<InitializePoolAccounts<'_, '_>> for InitializePoolKeys {
    fn from(accounts: InitializePoolAccounts) -> Self {
        Self {
            plasma_program: *accounts.plasma_program.key,
            log_authority: *accounts.log_authority.key,
            pool: *accounts.pool.key,
            pool_creator: *accounts.pool_creator.key,
            base_mint: *accounts.base_mint.key,
            quote_mint: *accounts.quote_mint.key,
            base_vault: *accounts.base_vault.key,
            quote_vault: *accounts.quote_vault.key,
            system_program: *accounts.system_program.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<InitializePoolKeys> for [AccountMeta; INITIALIZE_POOL_IX_ACCOUNTS_LEN] {
    fn from(keys: InitializePoolKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.plasma_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.log_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pool_creator,
                is_signer: true,
                is_writable: true,
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
impl From<[Pubkey; INITIALIZE_POOL_IX_ACCOUNTS_LEN]> for InitializePoolKeys {
    fn from(pubkeys: [Pubkey; INITIALIZE_POOL_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            plasma_program: pubkeys[0],
            log_authority: pubkeys[1],
            pool: pubkeys[2],
            pool_creator: pubkeys[3],
            base_mint: pubkeys[4],
            quote_mint: pubkeys[5],
            base_vault: pubkeys[6],
            quote_vault: pubkeys[7],
            system_program: pubkeys[8],
            token_program: pubkeys[9],
        }
    }
}
impl<'info> From<InitializePoolAccounts<'_, 'info>>
for [AccountInfo<'info>; INITIALIZE_POOL_IX_ACCOUNTS_LEN] {
    fn from(accounts: InitializePoolAccounts<'_, 'info>) -> Self {
        [
            accounts.plasma_program.clone(),
            accounts.log_authority.clone(),
            accounts.pool.clone(),
            accounts.pool_creator.clone(),
            accounts.base_mint.clone(),
            accounts.quote_mint.clone(),
            accounts.base_vault.clone(),
            accounts.quote_vault.clone(),
            accounts.system_program.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; INITIALIZE_POOL_IX_ACCOUNTS_LEN]>
for InitializePoolAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; INITIALIZE_POOL_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            plasma_program: &arr[0],
            log_authority: &arr[1],
            pool: &arr[2],
            pool_creator: &arr[3],
            base_mint: &arr[4],
            quote_mint: &arr[5],
            base_vault: &arr[6],
            quote_vault: &arr[7],
            system_program: &arr[8],
            token_program: &arr[9],
        }
    }
}
pub const INITIALIZE_POOL_IX_DISCM: u8 = 6u8;
#[derive(
    BorshDeserialize,
    BorshSerialize,
    Clone,
    Debug,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
pub struct InitializePoolIxArgs {
    pub params: InitializePoolIxParams,
}
impl InitializePoolIxArgs {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let params = if reader.is_empty() {
            Default::default()
        } else {
            <InitializePoolIxParams>::deserialize(&mut reader)?
        };
        *__buf = reader;
        Ok(Self { params })
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct InitializePoolIxData(pub InitializePoolIxArgs);
impl From<InitializePoolIxArgs> for InitializePoolIxData {
    fn from(args: InitializePoolIxArgs) -> Self {
        Self(args)
    }
}
impl InitializePoolIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm_buf = [0u8; 1];
        reader.read_exact(&mut maybe_discm_buf)?;
        let maybe_discm = maybe_discm_buf[0];
        if maybe_discm != INITIALIZE_POOL_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(InitializePoolIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&[INITIALIZE_POOL_IX_DISCM])?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn initialize_pool_ix_with_program_id(
    program_id: Pubkey,
    keys: InitializePoolKeys,
    args: InitializePoolIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; INITIALIZE_POOL_IX_ACCOUNTS_LEN] = keys.into();
    let data: InitializePoolIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn initialize_pool_ix(
    keys: InitializePoolKeys,
    args: InitializePoolIxArgs,
) -> std::io::Result<Instruction> {
    initialize_pool_ix_with_program_id(PLASMA_PROGRAM_ID, keys, args)
}
pub fn initialize_pool_invoke_with_program_id(
    program_id: Pubkey,
    accounts: InitializePoolAccounts<'_, '_>,
    args: InitializePoolIxArgs,
) -> ProgramResult {
    let keys: InitializePoolKeys = accounts.into();
    let ix = initialize_pool_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn initialize_pool_invoke(
    accounts: InitializePoolAccounts<'_, '_>,
    args: InitializePoolIxArgs,
) -> ProgramResult {
    initialize_pool_invoke_with_program_id(PLASMA_PROGRAM_ID, accounts, args)
}
pub fn initialize_pool_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: InitializePoolAccounts<'_, '_>,
    args: InitializePoolIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: InitializePoolKeys = accounts.into();
    let ix = initialize_pool_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn initialize_pool_invoke_signed(
    accounts: InitializePoolAccounts<'_, '_>,
    args: InitializePoolIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    initialize_pool_invoke_signed_with_program_id(
        PLASMA_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn initialize_pool_verify_account_keys(
    accounts: InitializePoolAccounts<'_, '_>,
    keys: InitializePoolKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.plasma_program.key, &keys.plasma_program),
        (accounts.log_authority.key, &keys.log_authority),
        (accounts.pool.key, &keys.pool),
        (accounts.pool_creator.key, &keys.pool_creator),
        (accounts.base_mint.key, &keys.base_mint),
        (accounts.quote_mint.key, &keys.quote_mint),
        (accounts.base_vault.key, &keys.base_vault),
        (accounts.quote_vault.key, &keys.quote_vault),
        (accounts.system_program.key, &keys.system_program),
        (accounts.token_program.key, &keys.token_program),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn initialize_pool_verify_writable_privileges<'me, 'info>(
    accounts: InitializePoolAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.pool,
        accounts.pool_creator,
        accounts.base_vault,
        accounts.quote_vault,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn initialize_pool_verify_signer_privileges<'me, 'info>(
    accounts: InitializePoolAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.pool_creator] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn initialize_pool_verify_account_privileges<'me, 'info>(
    accounts: InitializePoolAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    initialize_pool_verify_writable_privileges(accounts)?;
    initialize_pool_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const WITHDRAW_PROTOCOL_FEES_IX_ACCOUNTS_LEN: usize = 7;
#[derive(Copy, Clone, Debug)]
pub struct WithdrawProtocolFeesAccounts<'me, 'info> {
    pub plasma_program: &'me AccountInfo<'info>,
    pub log_authority: &'me AccountInfo<'info>,
    pub pool: &'me AccountInfo<'info>,
    pub protocol_fee_recipient: &'me AccountInfo<'info>,
    pub quote_account: &'me AccountInfo<'info>,
    pub quote_vault: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct WithdrawProtocolFeesKeys {
    pub plasma_program: Pubkey,
    pub log_authority: Pubkey,
    pub pool: Pubkey,
    pub protocol_fee_recipient: Pubkey,
    pub quote_account: Pubkey,
    pub quote_vault: Pubkey,
    pub token_program: Pubkey,
}
impl From<WithdrawProtocolFeesAccounts<'_, '_>> for WithdrawProtocolFeesKeys {
    fn from(accounts: WithdrawProtocolFeesAccounts) -> Self {
        Self {
            plasma_program: *accounts.plasma_program.key,
            log_authority: *accounts.log_authority.key,
            pool: *accounts.pool.key,
            protocol_fee_recipient: *accounts.protocol_fee_recipient.key,
            quote_account: *accounts.quote_account.key,
            quote_vault: *accounts.quote_vault.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<WithdrawProtocolFeesKeys>
for [AccountMeta; WITHDRAW_PROTOCOL_FEES_IX_ACCOUNTS_LEN] {
    fn from(keys: WithdrawProtocolFeesKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.plasma_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.log_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.protocol_fee_recipient,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.quote_account,
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
        ]
    }
}
impl From<[Pubkey; WITHDRAW_PROTOCOL_FEES_IX_ACCOUNTS_LEN]>
for WithdrawProtocolFeesKeys {
    fn from(pubkeys: [Pubkey; WITHDRAW_PROTOCOL_FEES_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            plasma_program: pubkeys[0],
            log_authority: pubkeys[1],
            pool: pubkeys[2],
            protocol_fee_recipient: pubkeys[3],
            quote_account: pubkeys[4],
            quote_vault: pubkeys[5],
            token_program: pubkeys[6],
        }
    }
}
impl<'info> From<WithdrawProtocolFeesAccounts<'_, 'info>>
for [AccountInfo<'info>; WITHDRAW_PROTOCOL_FEES_IX_ACCOUNTS_LEN] {
    fn from(accounts: WithdrawProtocolFeesAccounts<'_, 'info>) -> Self {
        [
            accounts.plasma_program.clone(),
            accounts.log_authority.clone(),
            accounts.pool.clone(),
            accounts.protocol_fee_recipient.clone(),
            accounts.quote_account.clone(),
            accounts.quote_vault.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; WITHDRAW_PROTOCOL_FEES_IX_ACCOUNTS_LEN]>
for WithdrawProtocolFeesAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; WITHDRAW_PROTOCOL_FEES_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            plasma_program: &arr[0],
            log_authority: &arr[1],
            pool: &arr[2],
            protocol_fee_recipient: &arr[3],
            quote_account: &arr[4],
            quote_vault: &arr[5],
            token_program: &arr[6],
        }
    }
}
pub const WITHDRAW_PROTOCOL_FEES_IX_DISCM: u8 = 7u8;
#[derive(Clone, Debug, PartialEq)]
pub struct WithdrawProtocolFeesIxData;
impl WithdrawProtocolFeesIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm_buf = [0u8; 1];
        reader.read_exact(&mut maybe_discm_buf)?;
        let maybe_discm = maybe_discm_buf[0];
        if maybe_discm != WITHDRAW_PROTOCOL_FEES_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&[WITHDRAW_PROTOCOL_FEES_IX_DISCM])
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn withdraw_protocol_fees_ix_with_program_id(
    program_id: Pubkey,
    keys: WithdrawProtocolFeesKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; WITHDRAW_PROTOCOL_FEES_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: WithdrawProtocolFeesIxData.try_to_vec()?,
    })
}
pub fn withdraw_protocol_fees_ix(
    keys: WithdrawProtocolFeesKeys,
) -> std::io::Result<Instruction> {
    withdraw_protocol_fees_ix_with_program_id(PLASMA_PROGRAM_ID, keys)
}
pub fn withdraw_protocol_fees_invoke_with_program_id(
    program_id: Pubkey,
    accounts: WithdrawProtocolFeesAccounts<'_, '_>,
) -> ProgramResult {
    let keys: WithdrawProtocolFeesKeys = accounts.into();
    let ix = withdraw_protocol_fees_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn withdraw_protocol_fees_invoke(
    accounts: WithdrawProtocolFeesAccounts<'_, '_>,
) -> ProgramResult {
    withdraw_protocol_fees_invoke_with_program_id(PLASMA_PROGRAM_ID, accounts)
}
pub fn withdraw_protocol_fees_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: WithdrawProtocolFeesAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: WithdrawProtocolFeesKeys = accounts.into();
    let ix = withdraw_protocol_fees_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn withdraw_protocol_fees_invoke_signed(
    accounts: WithdrawProtocolFeesAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    withdraw_protocol_fees_invoke_signed_with_program_id(
        PLASMA_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn withdraw_protocol_fees_verify_account_keys(
    accounts: WithdrawProtocolFeesAccounts<'_, '_>,
    keys: WithdrawProtocolFeesKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.plasma_program.key, &keys.plasma_program),
        (accounts.log_authority.key, &keys.log_authority),
        (accounts.pool.key, &keys.pool),
        (accounts.protocol_fee_recipient.key, &keys.protocol_fee_recipient),
        (accounts.quote_account.key, &keys.quote_account),
        (accounts.quote_vault.key, &keys.quote_vault),
        (accounts.token_program.key, &keys.token_program),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn withdraw_protocol_fees_verify_writable_privileges<'me, 'info>(
    accounts: WithdrawProtocolFeesAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.pool,
        accounts.quote_account,
        accounts.quote_vault,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn withdraw_protocol_fees_verify_signer_privileges<'me, 'info>(
    accounts: WithdrawProtocolFeesAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.protocol_fee_recipient] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn withdraw_protocol_fees_verify_account_privileges<'me, 'info>(
    accounts: WithdrawProtocolFeesAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    withdraw_protocol_fees_verify_writable_privileges(accounts)?;
    withdraw_protocol_fees_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const LOG_IX_ACCOUNTS_LEN: usize = 1;
#[derive(Copy, Clone, Debug)]
pub struct LogAccounts<'me, 'info> {
    pub log_authority: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct LogKeys {
    pub log_authority: Pubkey,
}
impl From<LogAccounts<'_, '_>> for LogKeys {
    fn from(accounts: LogAccounts) -> Self {
        Self {
            log_authority: *accounts.log_authority.key,
        }
    }
}
impl From<LogKeys> for [AccountMeta; LOG_IX_ACCOUNTS_LEN] {
    fn from(keys: LogKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.log_authority,
                is_signer: true,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; LOG_IX_ACCOUNTS_LEN]> for LogKeys {
    fn from(pubkeys: [Pubkey; LOG_IX_ACCOUNTS_LEN]) -> Self {
        Self { log_authority: pubkeys[0] }
    }
}
impl<'info> From<LogAccounts<'_, 'info>> for [AccountInfo<'info>; LOG_IX_ACCOUNTS_LEN] {
    fn from(accounts: LogAccounts<'_, 'info>) -> Self {
        [accounts.log_authority.clone()]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; LOG_IX_ACCOUNTS_LEN]>
for LogAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; LOG_IX_ACCOUNTS_LEN]) -> Self {
        Self { log_authority: &arr[0] }
    }
}
pub const LOG_IX_DISCM: u8 = 8u8;
#[derive(Clone, Debug, PartialEq)]
pub struct LogIxData;
impl LogIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm_buf = [0u8; 1];
        reader.read_exact(&mut maybe_discm_buf)?;
        let maybe_discm = maybe_discm_buf[0];
        if maybe_discm != LOG_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&[LOG_IX_DISCM])
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn log_ix_with_program_id(
    program_id: Pubkey,
    keys: LogKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; LOG_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: LogIxData.try_to_vec()?,
    })
}
pub fn log_ix(keys: LogKeys) -> std::io::Result<Instruction> {
    log_ix_with_program_id(PLASMA_PROGRAM_ID, keys)
}
pub fn log_invoke_with_program_id(
    program_id: Pubkey,
    accounts: LogAccounts<'_, '_>,
) -> ProgramResult {
    let keys: LogKeys = accounts.into();
    let ix = log_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn log_invoke(accounts: LogAccounts<'_, '_>) -> ProgramResult {
    log_invoke_with_program_id(PLASMA_PROGRAM_ID, accounts)
}
pub fn log_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: LogAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: LogKeys = accounts.into();
    let ix = log_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn log_invoke_signed(
    accounts: LogAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    log_invoke_signed_with_program_id(PLASMA_PROGRAM_ID, accounts, seeds)
}
pub fn log_verify_account_keys(
    accounts: LogAccounts<'_, '_>,
    keys: LogKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [(accounts.log_authority.key, &keys.log_authority)] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn log_verify_signer_privileges<'me, 'info>(
    accounts: LogAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.log_authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn log_verify_account_privileges<'me, 'info>(
    accounts: LogAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    log_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const TRANSFER_LIQUIDITY_IX_ACCOUNTS_LEN: usize = 6;
#[derive(Copy, Clone, Debug)]
pub struct TransferLiquidityAccounts<'me, 'info> {
    pub plasma_program: &'me AccountInfo<'info>,
    pub log_authority: &'me AccountInfo<'info>,
    pub pool: &'me AccountInfo<'info>,
    pub trader: &'me AccountInfo<'info>,
    pub src_lp_position: &'me AccountInfo<'info>,
    pub dst_lp_position: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct TransferLiquidityKeys {
    pub plasma_program: Pubkey,
    pub log_authority: Pubkey,
    pub pool: Pubkey,
    pub trader: Pubkey,
    pub src_lp_position: Pubkey,
    pub dst_lp_position: Pubkey,
}
impl From<TransferLiquidityAccounts<'_, '_>> for TransferLiquidityKeys {
    fn from(accounts: TransferLiquidityAccounts) -> Self {
        Self {
            plasma_program: *accounts.plasma_program.key,
            log_authority: *accounts.log_authority.key,
            pool: *accounts.pool.key,
            trader: *accounts.trader.key,
            src_lp_position: *accounts.src_lp_position.key,
            dst_lp_position: *accounts.dst_lp_position.key,
        }
    }
}
impl From<TransferLiquidityKeys> for [AccountMeta; TRANSFER_LIQUIDITY_IX_ACCOUNTS_LEN] {
    fn from(keys: TransferLiquidityKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.plasma_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.log_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.trader,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.src_lp_position,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.dst_lp_position,
                is_signer: false,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; TRANSFER_LIQUIDITY_IX_ACCOUNTS_LEN]> for TransferLiquidityKeys {
    fn from(pubkeys: [Pubkey; TRANSFER_LIQUIDITY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            plasma_program: pubkeys[0],
            log_authority: pubkeys[1],
            pool: pubkeys[2],
            trader: pubkeys[3],
            src_lp_position: pubkeys[4],
            dst_lp_position: pubkeys[5],
        }
    }
}
impl<'info> From<TransferLiquidityAccounts<'_, 'info>>
for [AccountInfo<'info>; TRANSFER_LIQUIDITY_IX_ACCOUNTS_LEN] {
    fn from(accounts: TransferLiquidityAccounts<'_, 'info>) -> Self {
        [
            accounts.plasma_program.clone(),
            accounts.log_authority.clone(),
            accounts.pool.clone(),
            accounts.trader.clone(),
            accounts.src_lp_position.clone(),
            accounts.dst_lp_position.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; TRANSFER_LIQUIDITY_IX_ACCOUNTS_LEN]>
for TransferLiquidityAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; TRANSFER_LIQUIDITY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            plasma_program: &arr[0],
            log_authority: &arr[1],
            pool: &arr[2],
            trader: &arr[3],
            src_lp_position: &arr[4],
            dst_lp_position: &arr[5],
        }
    }
}
pub const TRANSFER_LIQUIDITY_IX_DISCM: u8 = 9u8;
#[derive(Clone, Debug, PartialEq)]
pub struct TransferLiquidityIxData;
impl TransferLiquidityIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm_buf = [0u8; 1];
        reader.read_exact(&mut maybe_discm_buf)?;
        let maybe_discm = maybe_discm_buf[0];
        if maybe_discm != TRANSFER_LIQUIDITY_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&[TRANSFER_LIQUIDITY_IX_DISCM])
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn transfer_liquidity_ix_with_program_id(
    program_id: Pubkey,
    keys: TransferLiquidityKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; TRANSFER_LIQUIDITY_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: TransferLiquidityIxData.try_to_vec()?,
    })
}
pub fn transfer_liquidity_ix(
    keys: TransferLiquidityKeys,
) -> std::io::Result<Instruction> {
    transfer_liquidity_ix_with_program_id(PLASMA_PROGRAM_ID, keys)
}
pub fn transfer_liquidity_invoke_with_program_id(
    program_id: Pubkey,
    accounts: TransferLiquidityAccounts<'_, '_>,
) -> ProgramResult {
    let keys: TransferLiquidityKeys = accounts.into();
    let ix = transfer_liquidity_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn transfer_liquidity_invoke(
    accounts: TransferLiquidityAccounts<'_, '_>,
) -> ProgramResult {
    transfer_liquidity_invoke_with_program_id(PLASMA_PROGRAM_ID, accounts)
}
pub fn transfer_liquidity_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: TransferLiquidityAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: TransferLiquidityKeys = accounts.into();
    let ix = transfer_liquidity_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn transfer_liquidity_invoke_signed(
    accounts: TransferLiquidityAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    transfer_liquidity_invoke_signed_with_program_id(PLASMA_PROGRAM_ID, accounts, seeds)
}
pub fn transfer_liquidity_verify_account_keys(
    accounts: TransferLiquidityAccounts<'_, '_>,
    keys: TransferLiquidityKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.plasma_program.key, &keys.plasma_program),
        (accounts.log_authority.key, &keys.log_authority),
        (accounts.pool.key, &keys.pool),
        (accounts.trader.key, &keys.trader),
        (accounts.src_lp_position.key, &keys.src_lp_position),
        (accounts.dst_lp_position.key, &keys.dst_lp_position),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn transfer_liquidity_verify_writable_privileges<'me, 'info>(
    accounts: TransferLiquidityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.pool,
        accounts.src_lp_position,
        accounts.dst_lp_position,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn transfer_liquidity_verify_signer_privileges<'me, 'info>(
    accounts: TransferLiquidityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.trader] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn transfer_liquidity_verify_account_privileges<'me, 'info>(
    accounts: TransferLiquidityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    transfer_liquidity_verify_writable_privileges(accounts)?;
    transfer_liquidity_verify_signer_privileges(accounts)?;
    Ok(())
}
