use solana_pubkey::Pubkey;
use solana_cpi::{invoke, invoke_signed};
use solana_instruction::{AccountMeta, Instruction};
use solana_account_info::AccountInfo;
use solana_program_error::ProgramError;
use std::io::Read;
#[allow(unused_imports)]
use crate::*;
#[derive(Clone, Debug, PartialEq)]
pub enum WeightedSwapProgramIx {
    AcceptOwner,
    ChangeMaxSupply(ChangeMaxSupplyIxArgs),
    ChangeSwapFee(ChangeSwapFeeIxArgs),
    Deposit(DepositIxArgs),
    Initialize(InitializeIxArgs),
    Pause,
    RejectOwner,
    Shutdown,
    Swap(SwapIxArgs),
    SwapV2(SwapV2IxArgs),
    TransferOwner(TransferOwnerIxArgs),
    Unpause,
    Withdraw(WithdrawIxArgs),
}
impl WeightedSwapProgramIx {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        if buf.starts_with(&ACCEPT_OWNER_IX_DISCM) {
            return Ok(Self::AcceptOwner);
        }
        if buf.starts_with(&CHANGE_MAX_SUPPLY_IX_DISCM) {
            let mut reader = &buf[CHANGE_MAX_SUPPLY_IX_DISCM.len()..];
            let new_max_supply: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::ChangeMaxSupply(ChangeMaxSupplyIxArgs {
                    new_max_supply,
                }),
            );
        }
        if buf.starts_with(&CHANGE_SWAP_FEE_IX_DISCM) {
            let mut reader = &buf[CHANGE_SWAP_FEE_IX_DISCM.len()..];
            let new_swap_fee: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::ChangeSwapFee(ChangeSwapFeeIxArgs {
                    new_swap_fee,
                }),
            );
        }
        if buf.starts_with(&DEPOSIT_IX_DISCM) {
            let mut reader = &buf[DEPOSIT_IX_DISCM.len()..];
            let amounts: Vec<u64> = crate::borsh_de_or_default(&mut reader)?;
            let minimum_amount_out: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::Deposit(DepositIxArgs {
                    amounts,
                    minimum_amount_out,
                }),
            );
        }
        if buf.starts_with(&INITIALIZE_IX_DISCM) {
            let mut reader = &buf[INITIALIZE_IX_DISCM.len()..];
            let swap_fee: u64 = crate::borsh_de_or_default(&mut reader)?;
            let weights: Vec<u64> = crate::borsh_de_or_default(&mut reader)?;
            let max_caps: Vec<u64> = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::Initialize(InitializeIxArgs {
                    swap_fee,
                    weights,
                    max_caps,
                }),
            );
        }
        if buf.starts_with(&PAUSE_IX_DISCM) {
            return Ok(Self::Pause);
        }
        if buf.starts_with(&REJECT_OWNER_IX_DISCM) {
            return Ok(Self::RejectOwner);
        }
        if buf.starts_with(&SHUTDOWN_IX_DISCM) {
            return Ok(Self::Shutdown);
        }
        if buf.starts_with(&SWAP_IX_DISCM) {
            let mut reader = &buf[SWAP_IX_DISCM.len()..];
            let amount_in: Option<u64> = crate::borsh_de_or_default(&mut reader)?;
            let minimum_amount_out: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::Swap(SwapIxArgs {
                    amount_in,
                    minimum_amount_out,
                }),
            );
        }
        if buf.starts_with(&SWAP_V2_IX_DISCM) {
            let mut reader = &buf[SWAP_V2_IX_DISCM.len()..];
            let amount_in: Option<u64> = crate::borsh_de_or_default(&mut reader)?;
            let minimum_amount_out: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::SwapV2(SwapV2IxArgs {
                    amount_in,
                    minimum_amount_out,
                }),
            );
        }
        if buf.starts_with(&TRANSFER_OWNER_IX_DISCM) {
            let mut reader = &buf[TRANSFER_OWNER_IX_DISCM.len()..];
            let new_owner: Pubkey = crate::borsh_de_or_default(&mut reader)?;
            return Ok(Self::TransferOwner(TransferOwnerIxArgs { new_owner }));
        }
        if buf.starts_with(&UNPAUSE_IX_DISCM) {
            return Ok(Self::Unpause);
        }
        if buf.starts_with(&WITHDRAW_IX_DISCM) {
            let mut reader = &buf[WITHDRAW_IX_DISCM.len()..];
            let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            let minimum_amounts_out: Vec<u64> = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::Withdraw(WithdrawIxArgs {
                    amount,
                    minimum_amounts_out,
                }),
            );
        }
        Err(std::io::Error::from(std::io::ErrorKind::InvalidData))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        match self {
            Self::AcceptOwner => writer.write_all(&ACCEPT_OWNER_IX_DISCM),
            Self::ChangeMaxSupply(args) => {
                writer.write_all(&CHANGE_MAX_SUPPLY_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.new_max_supply, &mut writer)?;
                Ok(())
            }
            Self::ChangeSwapFee(args) => {
                writer.write_all(&CHANGE_SWAP_FEE_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.new_swap_fee, &mut writer)?;
                Ok(())
            }
            Self::Deposit(args) => {
                writer.write_all(&DEPOSIT_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.amounts, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.minimum_amount_out, &mut writer)?;
                Ok(())
            }
            Self::Initialize(args) => {
                writer.write_all(&INITIALIZE_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.swap_fee, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.weights, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.max_caps, &mut writer)?;
                Ok(())
            }
            Self::Pause => writer.write_all(&PAUSE_IX_DISCM),
            Self::RejectOwner => writer.write_all(&REJECT_OWNER_IX_DISCM),
            Self::Shutdown => writer.write_all(&SHUTDOWN_IX_DISCM),
            Self::Swap(args) => {
                writer.write_all(&SWAP_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.amount_in, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.minimum_amount_out, &mut writer)?;
                Ok(())
            }
            Self::SwapV2(args) => {
                writer.write_all(&SWAP_V2_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.amount_in, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.minimum_amount_out, &mut writer)?;
                Ok(())
            }
            Self::TransferOwner(args) => {
                writer.write_all(&TRANSFER_OWNER_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.new_owner, &mut writer)?;
                Ok(())
            }
            Self::Unpause => writer.write_all(&UNPAUSE_IX_DISCM),
            Self::Withdraw(args) => {
                writer.write_all(&WITHDRAW_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.amount, &mut writer)?;
                borsh::BorshSerialize::serialize(
                    &args.minimum_amounts_out,
                    &mut writer,
                )?;
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
pub const ACCEPT_OWNER_IX_ACCOUNTS_LEN: usize = 2;
#[derive(Copy, Clone, Debug)]
pub struct AcceptOwnerAccounts<'me, 'info> {
    pub pending_owner: &'me AccountInfo<'info>,
    pub pool: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct AcceptOwnerKeys {
    pub pending_owner: Pubkey,
    pub pool: Pubkey,
}
impl From<AcceptOwnerAccounts<'_, '_>> for AcceptOwnerKeys {
    fn from(accounts: AcceptOwnerAccounts) -> Self {
        Self {
            pending_owner: *accounts.pending_owner.key,
            pool: *accounts.pool.key,
        }
    }
}
impl From<AcceptOwnerKeys> for [AccountMeta; ACCEPT_OWNER_IX_ACCOUNTS_LEN] {
    fn from(keys: AcceptOwnerKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.pending_owner,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool,
                is_signer: false,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; ACCEPT_OWNER_IX_ACCOUNTS_LEN]> for AcceptOwnerKeys {
    fn from(pubkeys: [Pubkey; ACCEPT_OWNER_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            pending_owner: pubkeys[0],
            pool: pubkeys[1],
        }
    }
}
impl<'info> From<AcceptOwnerAccounts<'_, 'info>>
for [AccountInfo<'info>; ACCEPT_OWNER_IX_ACCOUNTS_LEN] {
    fn from(accounts: AcceptOwnerAccounts<'_, 'info>) -> Self {
        [accounts.pending_owner.clone(), accounts.pool.clone()]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; ACCEPT_OWNER_IX_ACCOUNTS_LEN]>
for AcceptOwnerAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; ACCEPT_OWNER_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            pending_owner: &arr[0],
            pool: &arr[1],
        }
    }
}
pub const ACCEPT_OWNER_IX_DISCM: [u8; 8usize] = [176, 23, 41, 28, 23, 111, 8, 4];
#[derive(Clone, Debug, PartialEq)]
pub struct AcceptOwnerIxData;
impl AcceptOwnerIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != ACCEPT_OWNER_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&ACCEPT_OWNER_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn accept_owner_ix_with_program_id(
    program_id: Pubkey,
    keys: AcceptOwnerKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; ACCEPT_OWNER_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: AcceptOwnerIxData.try_to_vec()?,
    })
}
pub fn accept_owner_ix(keys: AcceptOwnerKeys) -> std::io::Result<Instruction> {
    accept_owner_ix_with_program_id(WEIGHTED_SWAP_PROGRAM_ID, keys)
}
pub fn accept_owner_invoke_with_program_id(
    program_id: Pubkey,
    accounts: AcceptOwnerAccounts<'_, '_>,
) -> ProgramResult {
    let keys: AcceptOwnerKeys = accounts.into();
    let ix = accept_owner_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn accept_owner_invoke(accounts: AcceptOwnerAccounts<'_, '_>) -> ProgramResult {
    accept_owner_invoke_with_program_id(WEIGHTED_SWAP_PROGRAM_ID, accounts)
}
pub fn accept_owner_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: AcceptOwnerAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: AcceptOwnerKeys = accounts.into();
    let ix = accept_owner_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn accept_owner_invoke_signed(
    accounts: AcceptOwnerAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    accept_owner_invoke_signed_with_program_id(WEIGHTED_SWAP_PROGRAM_ID, accounts, seeds)
}
pub fn accept_owner_verify_account_keys(
    accounts: AcceptOwnerAccounts<'_, '_>,
    keys: AcceptOwnerKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.pending_owner.key, keys.pending_owner),
        (*accounts.pool.key, keys.pool),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn accept_owner_verify_writable_privileges<'me, 'info>(
    accounts: AcceptOwnerAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.pool] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn accept_owner_verify_signer_privileges<'me, 'info>(
    accounts: AcceptOwnerAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.pending_owner] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn accept_owner_verify_account_privileges<'me, 'info>(
    accounts: AcceptOwnerAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    accept_owner_verify_writable_privileges(accounts)?;
    accept_owner_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CHANGE_MAX_SUPPLY_IX_ACCOUNTS_LEN: usize = 2;
#[derive(Copy, Clone, Debug)]
pub struct ChangeMaxSupplyAccounts<'me, 'info> {
    pub owner: &'me AccountInfo<'info>,
    pub pool: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ChangeMaxSupplyKeys {
    pub owner: Pubkey,
    pub pool: Pubkey,
}
impl From<ChangeMaxSupplyAccounts<'_, '_>> for ChangeMaxSupplyKeys {
    fn from(accounts: ChangeMaxSupplyAccounts) -> Self {
        Self {
            owner: *accounts.owner.key,
            pool: *accounts.pool.key,
        }
    }
}
impl From<ChangeMaxSupplyKeys> for [AccountMeta; CHANGE_MAX_SUPPLY_IX_ACCOUNTS_LEN] {
    fn from(keys: ChangeMaxSupplyKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.owner,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool,
                is_signer: false,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; CHANGE_MAX_SUPPLY_IX_ACCOUNTS_LEN]> for ChangeMaxSupplyKeys {
    fn from(pubkeys: [Pubkey; CHANGE_MAX_SUPPLY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            owner: pubkeys[0],
            pool: pubkeys[1],
        }
    }
}
impl<'info> From<ChangeMaxSupplyAccounts<'_, 'info>>
for [AccountInfo<'info>; CHANGE_MAX_SUPPLY_IX_ACCOUNTS_LEN] {
    fn from(accounts: ChangeMaxSupplyAccounts<'_, 'info>) -> Self {
        [accounts.owner.clone(), accounts.pool.clone()]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CHANGE_MAX_SUPPLY_IX_ACCOUNTS_LEN]>
for ChangeMaxSupplyAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; CHANGE_MAX_SUPPLY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            owner: &arr[0],
            pool: &arr[1],
        }
    }
}
pub const CHANGE_MAX_SUPPLY_IX_DISCM: [u8; 8usize] = [93, 176, 0, 205, 69, 63, 87, 80];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ChangeMaxSupplyIxArgs {
    pub new_max_supply: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct ChangeMaxSupplyIxData(pub ChangeMaxSupplyIxArgs);
impl From<ChangeMaxSupplyIxArgs> for ChangeMaxSupplyIxData {
    fn from(args: ChangeMaxSupplyIxArgs) -> Self {
        Self(args)
    }
}
impl ChangeMaxSupplyIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CHANGE_MAX_SUPPLY_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let new_max_supply: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(ChangeMaxSupplyIxArgs {
                new_max_supply,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CHANGE_MAX_SUPPLY_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.new_max_supply, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn change_max_supply_ix_with_program_id(
    program_id: Pubkey,
    keys: ChangeMaxSupplyKeys,
    args: ChangeMaxSupplyIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CHANGE_MAX_SUPPLY_IX_ACCOUNTS_LEN] = keys.into();
    let data: ChangeMaxSupplyIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn change_max_supply_ix(
    keys: ChangeMaxSupplyKeys,
    args: ChangeMaxSupplyIxArgs,
) -> std::io::Result<Instruction> {
    change_max_supply_ix_with_program_id(WEIGHTED_SWAP_PROGRAM_ID, keys, args)
}
pub fn change_max_supply_invoke_with_program_id(
    program_id: Pubkey,
    accounts: ChangeMaxSupplyAccounts<'_, '_>,
    args: ChangeMaxSupplyIxArgs,
) -> ProgramResult {
    let keys: ChangeMaxSupplyKeys = accounts.into();
    let ix = change_max_supply_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn change_max_supply_invoke(
    accounts: ChangeMaxSupplyAccounts<'_, '_>,
    args: ChangeMaxSupplyIxArgs,
) -> ProgramResult {
    change_max_supply_invoke_with_program_id(WEIGHTED_SWAP_PROGRAM_ID, accounts, args)
}
pub fn change_max_supply_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: ChangeMaxSupplyAccounts<'_, '_>,
    args: ChangeMaxSupplyIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: ChangeMaxSupplyKeys = accounts.into();
    let ix = change_max_supply_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn change_max_supply_invoke_signed(
    accounts: ChangeMaxSupplyAccounts<'_, '_>,
    args: ChangeMaxSupplyIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    change_max_supply_invoke_signed_with_program_id(
        WEIGHTED_SWAP_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn change_max_supply_verify_account_keys(
    accounts: ChangeMaxSupplyAccounts<'_, '_>,
    keys: ChangeMaxSupplyKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.owner.key, keys.owner),
        (*accounts.pool.key, keys.pool),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn change_max_supply_verify_writable_privileges<'me, 'info>(
    accounts: ChangeMaxSupplyAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.pool] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn change_max_supply_verify_signer_privileges<'me, 'info>(
    accounts: ChangeMaxSupplyAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.owner] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn change_max_supply_verify_account_privileges<'me, 'info>(
    accounts: ChangeMaxSupplyAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    change_max_supply_verify_writable_privileges(accounts)?;
    change_max_supply_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CHANGE_SWAP_FEE_IX_ACCOUNTS_LEN: usize = 2;
#[derive(Copy, Clone, Debug)]
pub struct ChangeSwapFeeAccounts<'me, 'info> {
    pub owner: &'me AccountInfo<'info>,
    pub pool: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ChangeSwapFeeKeys {
    pub owner: Pubkey,
    pub pool: Pubkey,
}
impl From<ChangeSwapFeeAccounts<'_, '_>> for ChangeSwapFeeKeys {
    fn from(accounts: ChangeSwapFeeAccounts) -> Self {
        Self {
            owner: *accounts.owner.key,
            pool: *accounts.pool.key,
        }
    }
}
impl From<ChangeSwapFeeKeys> for [AccountMeta; CHANGE_SWAP_FEE_IX_ACCOUNTS_LEN] {
    fn from(keys: ChangeSwapFeeKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.owner,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool,
                is_signer: false,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; CHANGE_SWAP_FEE_IX_ACCOUNTS_LEN]> for ChangeSwapFeeKeys {
    fn from(pubkeys: [Pubkey; CHANGE_SWAP_FEE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            owner: pubkeys[0],
            pool: pubkeys[1],
        }
    }
}
impl<'info> From<ChangeSwapFeeAccounts<'_, 'info>>
for [AccountInfo<'info>; CHANGE_SWAP_FEE_IX_ACCOUNTS_LEN] {
    fn from(accounts: ChangeSwapFeeAccounts<'_, 'info>) -> Self {
        [accounts.owner.clone(), accounts.pool.clone()]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CHANGE_SWAP_FEE_IX_ACCOUNTS_LEN]>
for ChangeSwapFeeAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; CHANGE_SWAP_FEE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            owner: &arr[0],
            pool: &arr[1],
        }
    }
}
pub const CHANGE_SWAP_FEE_IX_DISCM: [u8; 8usize] = [231, 15, 132, 51, 132, 165, 64, 170];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ChangeSwapFeeIxArgs {
    pub new_swap_fee: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct ChangeSwapFeeIxData(pub ChangeSwapFeeIxArgs);
impl From<ChangeSwapFeeIxArgs> for ChangeSwapFeeIxData {
    fn from(args: ChangeSwapFeeIxArgs) -> Self {
        Self(args)
    }
}
impl ChangeSwapFeeIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CHANGE_SWAP_FEE_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let new_swap_fee: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(ChangeSwapFeeIxArgs {
                new_swap_fee,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CHANGE_SWAP_FEE_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.new_swap_fee, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn change_swap_fee_ix_with_program_id(
    program_id: Pubkey,
    keys: ChangeSwapFeeKeys,
    args: ChangeSwapFeeIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CHANGE_SWAP_FEE_IX_ACCOUNTS_LEN] = keys.into();
    let data: ChangeSwapFeeIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn change_swap_fee_ix(
    keys: ChangeSwapFeeKeys,
    args: ChangeSwapFeeIxArgs,
) -> std::io::Result<Instruction> {
    change_swap_fee_ix_with_program_id(WEIGHTED_SWAP_PROGRAM_ID, keys, args)
}
pub fn change_swap_fee_invoke_with_program_id(
    program_id: Pubkey,
    accounts: ChangeSwapFeeAccounts<'_, '_>,
    args: ChangeSwapFeeIxArgs,
) -> ProgramResult {
    let keys: ChangeSwapFeeKeys = accounts.into();
    let ix = change_swap_fee_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn change_swap_fee_invoke(
    accounts: ChangeSwapFeeAccounts<'_, '_>,
    args: ChangeSwapFeeIxArgs,
) -> ProgramResult {
    change_swap_fee_invoke_with_program_id(WEIGHTED_SWAP_PROGRAM_ID, accounts, args)
}
pub fn change_swap_fee_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: ChangeSwapFeeAccounts<'_, '_>,
    args: ChangeSwapFeeIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: ChangeSwapFeeKeys = accounts.into();
    let ix = change_swap_fee_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn change_swap_fee_invoke_signed(
    accounts: ChangeSwapFeeAccounts<'_, '_>,
    args: ChangeSwapFeeIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    change_swap_fee_invoke_signed_with_program_id(
        WEIGHTED_SWAP_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn change_swap_fee_verify_account_keys(
    accounts: ChangeSwapFeeAccounts<'_, '_>,
    keys: ChangeSwapFeeKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.owner.key, keys.owner),
        (*accounts.pool.key, keys.pool),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn change_swap_fee_verify_writable_privileges<'me, 'info>(
    accounts: ChangeSwapFeeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.pool] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn change_swap_fee_verify_signer_privileges<'me, 'info>(
    accounts: ChangeSwapFeeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.owner] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn change_swap_fee_verify_account_privileges<'me, 'info>(
    accounts: ChangeSwapFeeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    change_swap_fee_verify_writable_privileges(accounts)?;
    change_swap_fee_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const DEPOSIT_IX_ACCOUNTS_LEN: usize = 9;
#[derive(Copy, Clone, Debug)]
pub struct DepositAccounts<'me, 'info> {
    pub user: &'me AccountInfo<'info>,
    pub user_pool_token: &'me AccountInfo<'info>,
    pub mint: &'me AccountInfo<'info>,
    pub pool: &'me AccountInfo<'info>,
    pub pool_authority: &'me AccountInfo<'info>,
    pub vault: &'me AccountInfo<'info>,
    pub vault_authority: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub token_program_2022: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct DepositKeys {
    pub user: Pubkey,
    pub user_pool_token: Pubkey,
    pub mint: Pubkey,
    pub pool: Pubkey,
    pub pool_authority: Pubkey,
    pub vault: Pubkey,
    pub vault_authority: Pubkey,
    pub token_program: Pubkey,
    pub token_program_2022: Pubkey,
}
impl From<DepositAccounts<'_, '_>> for DepositKeys {
    fn from(accounts: DepositAccounts) -> Self {
        Self {
            user: *accounts.user.key,
            user_pool_token: *accounts.user_pool_token.key,
            mint: *accounts.mint.key,
            pool: *accounts.pool.key,
            pool_authority: *accounts.pool_authority.key,
            vault: *accounts.vault.key,
            vault_authority: *accounts.vault_authority.key,
            token_program: *accounts.token_program.key,
            token_program_2022: *accounts.token_program_2022.key,
        }
    }
}
impl From<DepositKeys> for [AccountMeta; DEPOSIT_IX_ACCOUNTS_LEN] {
    fn from(keys: DepositKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.user,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.user_pool_token,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pool_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_program_2022,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; DEPOSIT_IX_ACCOUNTS_LEN]> for DepositKeys {
    fn from(pubkeys: [Pubkey; DEPOSIT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            user: pubkeys[0],
            user_pool_token: pubkeys[1],
            mint: pubkeys[2],
            pool: pubkeys[3],
            pool_authority: pubkeys[4],
            vault: pubkeys[5],
            vault_authority: pubkeys[6],
            token_program: pubkeys[7],
            token_program_2022: pubkeys[8],
        }
    }
}
impl<'info> From<DepositAccounts<'_, 'info>>
for [AccountInfo<'info>; DEPOSIT_IX_ACCOUNTS_LEN] {
    fn from(accounts: DepositAccounts<'_, 'info>) -> Self {
        [
            accounts.user.clone(),
            accounts.user_pool_token.clone(),
            accounts.mint.clone(),
            accounts.pool.clone(),
            accounts.pool_authority.clone(),
            accounts.vault.clone(),
            accounts.vault_authority.clone(),
            accounts.token_program.clone(),
            accounts.token_program_2022.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; DEPOSIT_IX_ACCOUNTS_LEN]>
for DepositAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; DEPOSIT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            user: &arr[0],
            user_pool_token: &arr[1],
            mint: &arr[2],
            pool: &arr[3],
            pool_authority: &arr[4],
            vault: &arr[5],
            vault_authority: &arr[6],
            token_program: &arr[7],
            token_program_2022: &arr[8],
        }
    }
}
pub const DEPOSIT_IX_DISCM: [u8; 8usize] = [242, 35, 198, 137, 82, 225, 242, 182];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DepositIxArgs {
    pub amounts: Vec<u64>,
    pub minimum_amount_out: u64,
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
        let amounts: Vec<u64> = crate::borsh_de_or_default(&mut reader)?;
        let minimum_amount_out: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(DepositIxArgs {
                amounts,
                minimum_amount_out,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&DEPOSIT_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.amounts, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.minimum_amount_out, &mut writer)?;
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
    deposit_ix_with_program_id(WEIGHTED_SWAP_PROGRAM_ID, keys, args)
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
    deposit_invoke_with_program_id(WEIGHTED_SWAP_PROGRAM_ID, accounts, args)
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
    deposit_invoke_signed_with_program_id(
        WEIGHTED_SWAP_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn deposit_verify_account_keys(
    accounts: DepositAccounts<'_, '_>,
    keys: DepositKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.user.key, keys.user),
        (*accounts.user_pool_token.key, keys.user_pool_token),
        (*accounts.mint.key, keys.mint),
        (*accounts.pool.key, keys.pool),
        (*accounts.pool_authority.key, keys.pool_authority),
        (*accounts.vault.key, keys.vault),
        (*accounts.vault_authority.key, keys.vault_authority),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.token_program_2022.key, keys.token_program_2022),
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
    for should_be_writable in [accounts.user_pool_token, accounts.mint, accounts.pool] {
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
pub const INITIALIZE_IX_ACCOUNTS_LEN: usize = 6;
#[derive(Copy, Clone, Debug)]
pub struct InitializeAccounts<'me, 'info> {
    pub owner: &'me AccountInfo<'info>,
    pub mint: &'me AccountInfo<'info>,
    pub pool: &'me AccountInfo<'info>,
    pub pool_authority: &'me AccountInfo<'info>,
    pub withdraw_authority: &'me AccountInfo<'info>,
    pub vault: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct InitializeKeys {
    pub owner: Pubkey,
    pub mint: Pubkey,
    pub pool: Pubkey,
    pub pool_authority: Pubkey,
    pub withdraw_authority: Pubkey,
    pub vault: Pubkey,
}
impl From<InitializeAccounts<'_, '_>> for InitializeKeys {
    fn from(accounts: InitializeAccounts) -> Self {
        Self {
            owner: *accounts.owner.key,
            mint: *accounts.mint.key,
            pool: *accounts.pool.key,
            pool_authority: *accounts.pool_authority.key,
            withdraw_authority: *accounts.withdraw_authority.key,
            vault: *accounts.vault.key,
        }
    }
}
impl From<InitializeKeys> for [AccountMeta; INITIALIZE_IX_ACCOUNTS_LEN] {
    fn from(keys: InitializeKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.owner,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pool_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.withdraw_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; INITIALIZE_IX_ACCOUNTS_LEN]> for InitializeKeys {
    fn from(pubkeys: [Pubkey; INITIALIZE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            owner: pubkeys[0],
            mint: pubkeys[1],
            pool: pubkeys[2],
            pool_authority: pubkeys[3],
            withdraw_authority: pubkeys[4],
            vault: pubkeys[5],
        }
    }
}
impl<'info> From<InitializeAccounts<'_, 'info>>
for [AccountInfo<'info>; INITIALIZE_IX_ACCOUNTS_LEN] {
    fn from(accounts: InitializeAccounts<'_, 'info>) -> Self {
        [
            accounts.owner.clone(),
            accounts.mint.clone(),
            accounts.pool.clone(),
            accounts.pool_authority.clone(),
            accounts.withdraw_authority.clone(),
            accounts.vault.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; INITIALIZE_IX_ACCOUNTS_LEN]>
for InitializeAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; INITIALIZE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            owner: &arr[0],
            mint: &arr[1],
            pool: &arr[2],
            pool_authority: &arr[3],
            withdraw_authority: &arr[4],
            vault: &arr[5],
        }
    }
}
pub const INITIALIZE_IX_DISCM: [u8; 8usize] = [175, 175, 109, 31, 13, 152, 155, 237];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct InitializeIxArgs {
    pub swap_fee: u64,
    pub weights: Vec<u64>,
    pub max_caps: Vec<u64>,
}
#[derive(Clone, Debug, PartialEq)]
pub struct InitializeIxData(pub InitializeIxArgs);
impl From<InitializeIxArgs> for InitializeIxData {
    fn from(args: InitializeIxArgs) -> Self {
        Self(args)
    }
}
impl InitializeIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INITIALIZE_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let swap_fee: u64 = crate::borsh_de_or_default(&mut reader)?;
        let weights: Vec<u64> = crate::borsh_de_or_default(&mut reader)?;
        let max_caps: Vec<u64> = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(InitializeIxArgs {
                swap_fee,
                weights,
                max_caps,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INITIALIZE_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.swap_fee, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.weights, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.max_caps, &mut writer)?;
        Ok(())
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
    args: InitializeIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; INITIALIZE_IX_ACCOUNTS_LEN] = keys.into();
    let data: InitializeIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn initialize_ix(
    keys: InitializeKeys,
    args: InitializeIxArgs,
) -> std::io::Result<Instruction> {
    initialize_ix_with_program_id(WEIGHTED_SWAP_PROGRAM_ID, keys, args)
}
pub fn initialize_invoke_with_program_id(
    program_id: Pubkey,
    accounts: InitializeAccounts<'_, '_>,
    args: InitializeIxArgs,
) -> ProgramResult {
    let keys: InitializeKeys = accounts.into();
    let ix = initialize_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn initialize_invoke(
    accounts: InitializeAccounts<'_, '_>,
    args: InitializeIxArgs,
) -> ProgramResult {
    initialize_invoke_with_program_id(WEIGHTED_SWAP_PROGRAM_ID, accounts, args)
}
pub fn initialize_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: InitializeAccounts<'_, '_>,
    args: InitializeIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: InitializeKeys = accounts.into();
    let ix = initialize_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn initialize_invoke_signed(
    accounts: InitializeAccounts<'_, '_>,
    args: InitializeIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    initialize_invoke_signed_with_program_id(
        WEIGHTED_SWAP_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn initialize_verify_account_keys(
    accounts: InitializeAccounts<'_, '_>,
    keys: InitializeKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.owner.key, keys.owner),
        (*accounts.mint.key, keys.mint),
        (*accounts.pool.key, keys.pool),
        (*accounts.pool_authority.key, keys.pool_authority),
        (*accounts.withdraw_authority.key, keys.withdraw_authority),
        (*accounts.vault.key, keys.vault),
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
    for should_be_writable in [accounts.pool] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn initialize_verify_signer_privileges<'me, 'info>(
    accounts: InitializeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.owner] {
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
pub const PAUSE_IX_ACCOUNTS_LEN: usize = 2;
#[derive(Copy, Clone, Debug)]
pub struct PauseAccounts<'me, 'info> {
    pub owner: &'me AccountInfo<'info>,
    pub pool: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct PauseKeys {
    pub owner: Pubkey,
    pub pool: Pubkey,
}
impl From<PauseAccounts<'_, '_>> for PauseKeys {
    fn from(accounts: PauseAccounts) -> Self {
        Self {
            owner: *accounts.owner.key,
            pool: *accounts.pool.key,
        }
    }
}
impl From<PauseKeys> for [AccountMeta; PAUSE_IX_ACCOUNTS_LEN] {
    fn from(keys: PauseKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.owner,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool,
                is_signer: false,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; PAUSE_IX_ACCOUNTS_LEN]> for PauseKeys {
    fn from(pubkeys: [Pubkey; PAUSE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            owner: pubkeys[0],
            pool: pubkeys[1],
        }
    }
}
impl<'info> From<PauseAccounts<'_, 'info>>
for [AccountInfo<'info>; PAUSE_IX_ACCOUNTS_LEN] {
    fn from(accounts: PauseAccounts<'_, 'info>) -> Self {
        [accounts.owner.clone(), accounts.pool.clone()]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; PAUSE_IX_ACCOUNTS_LEN]>
for PauseAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; PAUSE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            owner: &arr[0],
            pool: &arr[1],
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
    pause_ix_with_program_id(WEIGHTED_SWAP_PROGRAM_ID, keys)
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
    pause_invoke_with_program_id(WEIGHTED_SWAP_PROGRAM_ID, accounts)
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
    pause_invoke_signed_with_program_id(WEIGHTED_SWAP_PROGRAM_ID, accounts, seeds)
}
pub fn pause_verify_account_keys(
    accounts: PauseAccounts<'_, '_>,
    keys: PauseKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.owner.key, keys.owner),
        (*accounts.pool.key, keys.pool),
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
    for should_be_writable in [accounts.pool] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn pause_verify_signer_privileges<'me, 'info>(
    accounts: PauseAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.owner] {
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
pub const REJECT_OWNER_IX_ACCOUNTS_LEN: usize = 2;
#[derive(Copy, Clone, Debug)]
pub struct RejectOwnerAccounts<'me, 'info> {
    pub pending_owner: &'me AccountInfo<'info>,
    pub pool: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct RejectOwnerKeys {
    pub pending_owner: Pubkey,
    pub pool: Pubkey,
}
impl From<RejectOwnerAccounts<'_, '_>> for RejectOwnerKeys {
    fn from(accounts: RejectOwnerAccounts) -> Self {
        Self {
            pending_owner: *accounts.pending_owner.key,
            pool: *accounts.pool.key,
        }
    }
}
impl From<RejectOwnerKeys> for [AccountMeta; REJECT_OWNER_IX_ACCOUNTS_LEN] {
    fn from(keys: RejectOwnerKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.pending_owner,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool,
                is_signer: false,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; REJECT_OWNER_IX_ACCOUNTS_LEN]> for RejectOwnerKeys {
    fn from(pubkeys: [Pubkey; REJECT_OWNER_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            pending_owner: pubkeys[0],
            pool: pubkeys[1],
        }
    }
}
impl<'info> From<RejectOwnerAccounts<'_, 'info>>
for [AccountInfo<'info>; REJECT_OWNER_IX_ACCOUNTS_LEN] {
    fn from(accounts: RejectOwnerAccounts<'_, 'info>) -> Self {
        [accounts.pending_owner.clone(), accounts.pool.clone()]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; REJECT_OWNER_IX_ACCOUNTS_LEN]>
for RejectOwnerAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; REJECT_OWNER_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            pending_owner: &arr[0],
            pool: &arr[1],
        }
    }
}
pub const REJECT_OWNER_IX_DISCM: [u8; 8usize] = [238, 206, 198, 215, 51, 178, 133, 228];
#[derive(Clone, Debug, PartialEq)]
pub struct RejectOwnerIxData;
impl RejectOwnerIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != REJECT_OWNER_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&REJECT_OWNER_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn reject_owner_ix_with_program_id(
    program_id: Pubkey,
    keys: RejectOwnerKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; REJECT_OWNER_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: RejectOwnerIxData.try_to_vec()?,
    })
}
pub fn reject_owner_ix(keys: RejectOwnerKeys) -> std::io::Result<Instruction> {
    reject_owner_ix_with_program_id(WEIGHTED_SWAP_PROGRAM_ID, keys)
}
pub fn reject_owner_invoke_with_program_id(
    program_id: Pubkey,
    accounts: RejectOwnerAccounts<'_, '_>,
) -> ProgramResult {
    let keys: RejectOwnerKeys = accounts.into();
    let ix = reject_owner_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn reject_owner_invoke(accounts: RejectOwnerAccounts<'_, '_>) -> ProgramResult {
    reject_owner_invoke_with_program_id(WEIGHTED_SWAP_PROGRAM_ID, accounts)
}
pub fn reject_owner_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: RejectOwnerAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: RejectOwnerKeys = accounts.into();
    let ix = reject_owner_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn reject_owner_invoke_signed(
    accounts: RejectOwnerAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    reject_owner_invoke_signed_with_program_id(WEIGHTED_SWAP_PROGRAM_ID, accounts, seeds)
}
pub fn reject_owner_verify_account_keys(
    accounts: RejectOwnerAccounts<'_, '_>,
    keys: RejectOwnerKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.pending_owner.key, keys.pending_owner),
        (*accounts.pool.key, keys.pool),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn reject_owner_verify_writable_privileges<'me, 'info>(
    accounts: RejectOwnerAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.pool] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn reject_owner_verify_signer_privileges<'me, 'info>(
    accounts: RejectOwnerAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.pending_owner] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn reject_owner_verify_account_privileges<'me, 'info>(
    accounts: RejectOwnerAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    reject_owner_verify_writable_privileges(accounts)?;
    reject_owner_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SHUTDOWN_IX_ACCOUNTS_LEN: usize = 2;
#[derive(Copy, Clone, Debug)]
pub struct ShutdownAccounts<'me, 'info> {
    pub owner: &'me AccountInfo<'info>,
    pub pool: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ShutdownKeys {
    pub owner: Pubkey,
    pub pool: Pubkey,
}
impl From<ShutdownAccounts<'_, '_>> for ShutdownKeys {
    fn from(accounts: ShutdownAccounts) -> Self {
        Self {
            owner: *accounts.owner.key,
            pool: *accounts.pool.key,
        }
    }
}
impl From<ShutdownKeys> for [AccountMeta; SHUTDOWN_IX_ACCOUNTS_LEN] {
    fn from(keys: ShutdownKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.owner,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pool,
                is_signer: false,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; SHUTDOWN_IX_ACCOUNTS_LEN]> for ShutdownKeys {
    fn from(pubkeys: [Pubkey; SHUTDOWN_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            owner: pubkeys[0],
            pool: pubkeys[1],
        }
    }
}
impl<'info> From<ShutdownAccounts<'_, 'info>>
for [AccountInfo<'info>; SHUTDOWN_IX_ACCOUNTS_LEN] {
    fn from(accounts: ShutdownAccounts<'_, 'info>) -> Self {
        [accounts.owner.clone(), accounts.pool.clone()]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SHUTDOWN_IX_ACCOUNTS_LEN]>
for ShutdownAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; SHUTDOWN_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            owner: &arr[0],
            pool: &arr[1],
        }
    }
}
pub const SHUTDOWN_IX_DISCM: [u8; 8usize] = [146, 204, 241, 213, 86, 21, 253, 211];
#[derive(Clone, Debug, PartialEq)]
pub struct ShutdownIxData;
impl ShutdownIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SHUTDOWN_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SHUTDOWN_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn shutdown_ix_with_program_id(
    program_id: Pubkey,
    keys: ShutdownKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SHUTDOWN_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: ShutdownIxData.try_to_vec()?,
    })
}
pub fn shutdown_ix(keys: ShutdownKeys) -> std::io::Result<Instruction> {
    shutdown_ix_with_program_id(WEIGHTED_SWAP_PROGRAM_ID, keys)
}
pub fn shutdown_invoke_with_program_id(
    program_id: Pubkey,
    accounts: ShutdownAccounts<'_, '_>,
) -> ProgramResult {
    let keys: ShutdownKeys = accounts.into();
    let ix = shutdown_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn shutdown_invoke(accounts: ShutdownAccounts<'_, '_>) -> ProgramResult {
    shutdown_invoke_with_program_id(WEIGHTED_SWAP_PROGRAM_ID, accounts)
}
pub fn shutdown_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: ShutdownAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: ShutdownKeys = accounts.into();
    let ix = shutdown_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn shutdown_invoke_signed(
    accounts: ShutdownAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    shutdown_invoke_signed_with_program_id(WEIGHTED_SWAP_PROGRAM_ID, accounts, seeds)
}
pub fn shutdown_verify_account_keys(
    accounts: ShutdownAccounts<'_, '_>,
    keys: ShutdownKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.owner.key, keys.owner),
        (*accounts.pool.key, keys.pool),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn shutdown_verify_writable_privileges<'me, 'info>(
    accounts: ShutdownAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.owner, accounts.pool] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn shutdown_verify_account_privileges<'me, 'info>(
    accounts: ShutdownAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    shutdown_verify_writable_privileges(accounts)?;
    Ok(())
}
pub const SWAP_IX_ACCOUNTS_LEN: usize = 12;
#[derive(Copy, Clone, Debug)]
pub struct SwapAccounts<'me, 'info> {
    pub user: &'me AccountInfo<'info>,
    pub user_token_in: &'me AccountInfo<'info>,
    pub user_token_out: &'me AccountInfo<'info>,
    pub vault_token_in: &'me AccountInfo<'info>,
    pub vault_token_out: &'me AccountInfo<'info>,
    pub beneficiary_token_out: &'me AccountInfo<'info>,
    pub pool: &'me AccountInfo<'info>,
    pub withdraw_authority: &'me AccountInfo<'info>,
    pub vault: &'me AccountInfo<'info>,
    pub vault_authority: &'me AccountInfo<'info>,
    pub vault_program: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SwapKeys {
    pub user: Pubkey,
    pub user_token_in: Pubkey,
    pub user_token_out: Pubkey,
    pub vault_token_in: Pubkey,
    pub vault_token_out: Pubkey,
    pub beneficiary_token_out: Pubkey,
    pub pool: Pubkey,
    pub withdraw_authority: Pubkey,
    pub vault: Pubkey,
    pub vault_authority: Pubkey,
    pub vault_program: Pubkey,
    pub token_program: Pubkey,
}
impl From<SwapAccounts<'_, '_>> for SwapKeys {
    fn from(accounts: SwapAccounts) -> Self {
        Self {
            user: *accounts.user.key,
            user_token_in: *accounts.user_token_in.key,
            user_token_out: *accounts.user_token_out.key,
            vault_token_in: *accounts.vault_token_in.key,
            vault_token_out: *accounts.vault_token_out.key,
            beneficiary_token_out: *accounts.beneficiary_token_out.key,
            pool: *accounts.pool.key,
            withdraw_authority: *accounts.withdraw_authority.key,
            vault: *accounts.vault.key,
            vault_authority: *accounts.vault_authority.key,
            vault_program: *accounts.vault_program.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<SwapKeys> for [AccountMeta; SWAP_IX_ACCOUNTS_LEN] {
    fn from(keys: SwapKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.user,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.user_token_in,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_token_out,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vault_token_in,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vault_token_out,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.beneficiary_token_out,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.withdraw_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault_program,
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
            user: pubkeys[0],
            user_token_in: pubkeys[1],
            user_token_out: pubkeys[2],
            vault_token_in: pubkeys[3],
            vault_token_out: pubkeys[4],
            beneficiary_token_out: pubkeys[5],
            pool: pubkeys[6],
            withdraw_authority: pubkeys[7],
            vault: pubkeys[8],
            vault_authority: pubkeys[9],
            vault_program: pubkeys[10],
            token_program: pubkeys[11],
        }
    }
}
impl<'info> From<SwapAccounts<'_, 'info>>
for [AccountInfo<'info>; SWAP_IX_ACCOUNTS_LEN] {
    fn from(accounts: SwapAccounts<'_, 'info>) -> Self {
        [
            accounts.user.clone(),
            accounts.user_token_in.clone(),
            accounts.user_token_out.clone(),
            accounts.vault_token_in.clone(),
            accounts.vault_token_out.clone(),
            accounts.beneficiary_token_out.clone(),
            accounts.pool.clone(),
            accounts.withdraw_authority.clone(),
            accounts.vault.clone(),
            accounts.vault_authority.clone(),
            accounts.vault_program.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SWAP_IX_ACCOUNTS_LEN]>
for SwapAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; SWAP_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            user: &arr[0],
            user_token_in: &arr[1],
            user_token_out: &arr[2],
            vault_token_in: &arr[3],
            vault_token_out: &arr[4],
            beneficiary_token_out: &arr[5],
            pool: &arr[6],
            withdraw_authority: &arr[7],
            vault: &arr[8],
            vault_authority: &arr[9],
            vault_program: &arr[10],
            token_program: &arr[11],
        }
    }
}
pub const SWAP_IX_DISCM: [u8; 8usize] = [248, 198, 158, 145, 225, 117, 135, 200];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SwapIxArgs {
    pub amount_in: Option<u64>,
    pub minimum_amount_out: u64,
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
        let amount_in: Option<u64> = crate::borsh_de_or_default(&mut reader)?;
        let minimum_amount_out: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(SwapIxArgs {
                amount_in,
                minimum_amount_out,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SWAP_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.amount_in, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.minimum_amount_out, &mut writer)?;
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
    swap_ix_with_program_id(WEIGHTED_SWAP_PROGRAM_ID, keys, args)
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
    swap_invoke_with_program_id(WEIGHTED_SWAP_PROGRAM_ID, accounts, args)
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
    swap_invoke_signed_with_program_id(WEIGHTED_SWAP_PROGRAM_ID, accounts, args, seeds)
}
pub fn swap_verify_account_keys(
    accounts: SwapAccounts<'_, '_>,
    keys: SwapKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.user.key, keys.user),
        (*accounts.user_token_in.key, keys.user_token_in),
        (*accounts.user_token_out.key, keys.user_token_out),
        (*accounts.vault_token_in.key, keys.vault_token_in),
        (*accounts.vault_token_out.key, keys.vault_token_out),
        (*accounts.beneficiary_token_out.key, keys.beneficiary_token_out),
        (*accounts.pool.key, keys.pool),
        (*accounts.withdraw_authority.key, keys.withdraw_authority),
        (*accounts.vault.key, keys.vault),
        (*accounts.vault_authority.key, keys.vault_authority),
        (*accounts.vault_program.key, keys.vault_program),
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
        accounts.user_token_in,
        accounts.user_token_out,
        accounts.vault_token_in,
        accounts.vault_token_out,
        accounts.beneficiary_token_out,
        accounts.pool,
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
    for should_be_signer in [accounts.user] {
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
pub const SWAP_V2_IX_ACCOUNTS_LEN: usize = 15;
#[derive(Copy, Clone, Debug)]
pub struct SwapV2Accounts<'me, 'info> {
    pub user: &'me AccountInfo<'info>,
    pub mint_in: &'me AccountInfo<'info>,
    pub mint_out: &'me AccountInfo<'info>,
    pub user_token_in: &'me AccountInfo<'info>,
    pub user_token_out: &'me AccountInfo<'info>,
    pub vault_token_in: &'me AccountInfo<'info>,
    pub vault_token_out: &'me AccountInfo<'info>,
    pub beneficiary_token_out: &'me AccountInfo<'info>,
    pub pool: &'me AccountInfo<'info>,
    pub withdraw_authority: &'me AccountInfo<'info>,
    pub vault: &'me AccountInfo<'info>,
    pub vault_authority: &'me AccountInfo<'info>,
    pub vault_program: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub token_2022_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SwapV2Keys {
    pub user: Pubkey,
    pub mint_in: Pubkey,
    pub mint_out: Pubkey,
    pub user_token_in: Pubkey,
    pub user_token_out: Pubkey,
    pub vault_token_in: Pubkey,
    pub vault_token_out: Pubkey,
    pub beneficiary_token_out: Pubkey,
    pub pool: Pubkey,
    pub withdraw_authority: Pubkey,
    pub vault: Pubkey,
    pub vault_authority: Pubkey,
    pub vault_program: Pubkey,
    pub token_program: Pubkey,
    pub token_2022_program: Pubkey,
}
impl From<SwapV2Accounts<'_, '_>> for SwapV2Keys {
    fn from(accounts: SwapV2Accounts) -> Self {
        Self {
            user: *accounts.user.key,
            mint_in: *accounts.mint_in.key,
            mint_out: *accounts.mint_out.key,
            user_token_in: *accounts.user_token_in.key,
            user_token_out: *accounts.user_token_out.key,
            vault_token_in: *accounts.vault_token_in.key,
            vault_token_out: *accounts.vault_token_out.key,
            beneficiary_token_out: *accounts.beneficiary_token_out.key,
            pool: *accounts.pool.key,
            withdraw_authority: *accounts.withdraw_authority.key,
            vault: *accounts.vault.key,
            vault_authority: *accounts.vault_authority.key,
            vault_program: *accounts.vault_program.key,
            token_program: *accounts.token_program.key,
            token_2022_program: *accounts.token_2022_program.key,
        }
    }
}
impl From<SwapV2Keys> for [AccountMeta; SWAP_V2_IX_ACCOUNTS_LEN] {
    fn from(keys: SwapV2Keys) -> Self {
        [
            AccountMeta {
                pubkey: keys.user,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.mint_in,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.mint_out,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.user_token_in,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_token_out,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vault_token_in,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vault_token_out,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.beneficiary_token_out,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.withdraw_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_2022_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; SWAP_V2_IX_ACCOUNTS_LEN]> for SwapV2Keys {
    fn from(pubkeys: [Pubkey; SWAP_V2_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            user: pubkeys[0],
            mint_in: pubkeys[1],
            mint_out: pubkeys[2],
            user_token_in: pubkeys[3],
            user_token_out: pubkeys[4],
            vault_token_in: pubkeys[5],
            vault_token_out: pubkeys[6],
            beneficiary_token_out: pubkeys[7],
            pool: pubkeys[8],
            withdraw_authority: pubkeys[9],
            vault: pubkeys[10],
            vault_authority: pubkeys[11],
            vault_program: pubkeys[12],
            token_program: pubkeys[13],
            token_2022_program: pubkeys[14],
        }
    }
}
impl<'info> From<SwapV2Accounts<'_, 'info>>
for [AccountInfo<'info>; SWAP_V2_IX_ACCOUNTS_LEN] {
    fn from(accounts: SwapV2Accounts<'_, 'info>) -> Self {
        [
            accounts.user.clone(),
            accounts.mint_in.clone(),
            accounts.mint_out.clone(),
            accounts.user_token_in.clone(),
            accounts.user_token_out.clone(),
            accounts.vault_token_in.clone(),
            accounts.vault_token_out.clone(),
            accounts.beneficiary_token_out.clone(),
            accounts.pool.clone(),
            accounts.withdraw_authority.clone(),
            accounts.vault.clone(),
            accounts.vault_authority.clone(),
            accounts.vault_program.clone(),
            accounts.token_program.clone(),
            accounts.token_2022_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SWAP_V2_IX_ACCOUNTS_LEN]>
for SwapV2Accounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; SWAP_V2_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            user: &arr[0],
            mint_in: &arr[1],
            mint_out: &arr[2],
            user_token_in: &arr[3],
            user_token_out: &arr[4],
            vault_token_in: &arr[5],
            vault_token_out: &arr[6],
            beneficiary_token_out: &arr[7],
            pool: &arr[8],
            withdraw_authority: &arr[9],
            vault: &arr[10],
            vault_authority: &arr[11],
            vault_program: &arr[12],
            token_program: &arr[13],
            token_2022_program: &arr[14],
        }
    }
}
pub const SWAP_V2_IX_DISCM: [u8; 8usize] = [43, 4, 237, 11, 26, 201, 30, 98];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SwapV2IxArgs {
    pub amount_in: Option<u64>,
    pub minimum_amount_out: u64,
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
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SWAP_V2_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let amount_in: Option<u64> = crate::borsh_de_or_default(&mut reader)?;
        let minimum_amount_out: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(SwapV2IxArgs {
                amount_in,
                minimum_amount_out,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SWAP_V2_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.amount_in, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.minimum_amount_out, &mut writer)?;
        Ok(())
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
    swap_v2_ix_with_program_id(WEIGHTED_SWAP_PROGRAM_ID, keys, args)
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
    swap_v2_invoke_with_program_id(WEIGHTED_SWAP_PROGRAM_ID, accounts, args)
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
    swap_v2_invoke_signed_with_program_id(
        WEIGHTED_SWAP_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn swap_v2_verify_account_keys(
    accounts: SwapV2Accounts<'_, '_>,
    keys: SwapV2Keys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.user.key, keys.user),
        (*accounts.mint_in.key, keys.mint_in),
        (*accounts.mint_out.key, keys.mint_out),
        (*accounts.user_token_in.key, keys.user_token_in),
        (*accounts.user_token_out.key, keys.user_token_out),
        (*accounts.vault_token_in.key, keys.vault_token_in),
        (*accounts.vault_token_out.key, keys.vault_token_out),
        (*accounts.beneficiary_token_out.key, keys.beneficiary_token_out),
        (*accounts.pool.key, keys.pool),
        (*accounts.withdraw_authority.key, keys.withdraw_authority),
        (*accounts.vault.key, keys.vault),
        (*accounts.vault_authority.key, keys.vault_authority),
        (*accounts.vault_program.key, keys.vault_program),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.token_2022_program.key, keys.token_2022_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn swap_v2_verify_writable_privileges<'me, 'info>(
    accounts: SwapV2Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.user_token_in,
        accounts.user_token_out,
        accounts.vault_token_in,
        accounts.vault_token_out,
        accounts.beneficiary_token_out,
        accounts.pool,
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
    for should_be_signer in [accounts.user] {
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
pub const TRANSFER_OWNER_IX_ACCOUNTS_LEN: usize = 2;
#[derive(Copy, Clone, Debug)]
pub struct TransferOwnerAccounts<'me, 'info> {
    pub owner: &'me AccountInfo<'info>,
    pub pool: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct TransferOwnerKeys {
    pub owner: Pubkey,
    pub pool: Pubkey,
}
impl From<TransferOwnerAccounts<'_, '_>> for TransferOwnerKeys {
    fn from(accounts: TransferOwnerAccounts) -> Self {
        Self {
            owner: *accounts.owner.key,
            pool: *accounts.pool.key,
        }
    }
}
impl From<TransferOwnerKeys> for [AccountMeta; TRANSFER_OWNER_IX_ACCOUNTS_LEN] {
    fn from(keys: TransferOwnerKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.owner,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool,
                is_signer: false,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; TRANSFER_OWNER_IX_ACCOUNTS_LEN]> for TransferOwnerKeys {
    fn from(pubkeys: [Pubkey; TRANSFER_OWNER_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            owner: pubkeys[0],
            pool: pubkeys[1],
        }
    }
}
impl<'info> From<TransferOwnerAccounts<'_, 'info>>
for [AccountInfo<'info>; TRANSFER_OWNER_IX_ACCOUNTS_LEN] {
    fn from(accounts: TransferOwnerAccounts<'_, 'info>) -> Self {
        [accounts.owner.clone(), accounts.pool.clone()]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; TRANSFER_OWNER_IX_ACCOUNTS_LEN]>
for TransferOwnerAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; TRANSFER_OWNER_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            owner: &arr[0],
            pool: &arr[1],
        }
    }
}
pub const TRANSFER_OWNER_IX_DISCM: [u8; 8usize] = [245, 25, 221, 175, 106, 229, 225, 45];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TransferOwnerIxArgs {
    pub new_owner: Pubkey,
}
#[derive(Clone, Debug, PartialEq)]
pub struct TransferOwnerIxData(pub TransferOwnerIxArgs);
impl From<TransferOwnerIxArgs> for TransferOwnerIxData {
    fn from(args: TransferOwnerIxArgs) -> Self {
        Self(args)
    }
}
impl TransferOwnerIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != TRANSFER_OWNER_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let new_owner: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        Ok(Self(TransferOwnerIxArgs { new_owner }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&TRANSFER_OWNER_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.new_owner, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn transfer_owner_ix_with_program_id(
    program_id: Pubkey,
    keys: TransferOwnerKeys,
    args: TransferOwnerIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; TRANSFER_OWNER_IX_ACCOUNTS_LEN] = keys.into();
    let data: TransferOwnerIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn transfer_owner_ix(
    keys: TransferOwnerKeys,
    args: TransferOwnerIxArgs,
) -> std::io::Result<Instruction> {
    transfer_owner_ix_with_program_id(WEIGHTED_SWAP_PROGRAM_ID, keys, args)
}
pub fn transfer_owner_invoke_with_program_id(
    program_id: Pubkey,
    accounts: TransferOwnerAccounts<'_, '_>,
    args: TransferOwnerIxArgs,
) -> ProgramResult {
    let keys: TransferOwnerKeys = accounts.into();
    let ix = transfer_owner_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn transfer_owner_invoke(
    accounts: TransferOwnerAccounts<'_, '_>,
    args: TransferOwnerIxArgs,
) -> ProgramResult {
    transfer_owner_invoke_with_program_id(WEIGHTED_SWAP_PROGRAM_ID, accounts, args)
}
pub fn transfer_owner_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: TransferOwnerAccounts<'_, '_>,
    args: TransferOwnerIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: TransferOwnerKeys = accounts.into();
    let ix = transfer_owner_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn transfer_owner_invoke_signed(
    accounts: TransferOwnerAccounts<'_, '_>,
    args: TransferOwnerIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    transfer_owner_invoke_signed_with_program_id(
        WEIGHTED_SWAP_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn transfer_owner_verify_account_keys(
    accounts: TransferOwnerAccounts<'_, '_>,
    keys: TransferOwnerKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.owner.key, keys.owner),
        (*accounts.pool.key, keys.pool),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn transfer_owner_verify_writable_privileges<'me, 'info>(
    accounts: TransferOwnerAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.pool] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn transfer_owner_verify_signer_privileges<'me, 'info>(
    accounts: TransferOwnerAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.owner] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn transfer_owner_verify_account_privileges<'me, 'info>(
    accounts: TransferOwnerAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    transfer_owner_verify_writable_privileges(accounts)?;
    transfer_owner_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const UNPAUSE_IX_ACCOUNTS_LEN: usize = 2;
#[derive(Copy, Clone, Debug)]
pub struct UnpauseAccounts<'me, 'info> {
    pub owner: &'me AccountInfo<'info>,
    pub pool: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UnpauseKeys {
    pub owner: Pubkey,
    pub pool: Pubkey,
}
impl From<UnpauseAccounts<'_, '_>> for UnpauseKeys {
    fn from(accounts: UnpauseAccounts) -> Self {
        Self {
            owner: *accounts.owner.key,
            pool: *accounts.pool.key,
        }
    }
}
impl From<UnpauseKeys> for [AccountMeta; UNPAUSE_IX_ACCOUNTS_LEN] {
    fn from(keys: UnpauseKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.owner,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool,
                is_signer: false,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; UNPAUSE_IX_ACCOUNTS_LEN]> for UnpauseKeys {
    fn from(pubkeys: [Pubkey; UNPAUSE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            owner: pubkeys[0],
            pool: pubkeys[1],
        }
    }
}
impl<'info> From<UnpauseAccounts<'_, 'info>>
for [AccountInfo<'info>; UNPAUSE_IX_ACCOUNTS_LEN] {
    fn from(accounts: UnpauseAccounts<'_, 'info>) -> Self {
        [accounts.owner.clone(), accounts.pool.clone()]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; UNPAUSE_IX_ACCOUNTS_LEN]>
for UnpauseAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; UNPAUSE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            owner: &arr[0],
            pool: &arr[1],
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
    unpause_ix_with_program_id(WEIGHTED_SWAP_PROGRAM_ID, keys)
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
    unpause_invoke_with_program_id(WEIGHTED_SWAP_PROGRAM_ID, accounts)
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
    unpause_invoke_signed_with_program_id(WEIGHTED_SWAP_PROGRAM_ID, accounts, seeds)
}
pub fn unpause_verify_account_keys(
    accounts: UnpauseAccounts<'_, '_>,
    keys: UnpauseKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.owner.key, keys.owner),
        (*accounts.pool.key, keys.pool),
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
    for should_be_writable in [accounts.pool] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn unpause_verify_signer_privileges<'me, 'info>(
    accounts: UnpauseAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.owner] {
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
pub const WITHDRAW_IX_ACCOUNTS_LEN: usize = 10;
#[derive(Copy, Clone, Debug)]
pub struct WithdrawAccounts<'me, 'info> {
    pub user: &'me AccountInfo<'info>,
    pub user_pool_token: &'me AccountInfo<'info>,
    pub mint: &'me AccountInfo<'info>,
    pub pool: &'me AccountInfo<'info>,
    pub withdraw_authority: &'me AccountInfo<'info>,
    pub vault: &'me AccountInfo<'info>,
    pub vault_authority: &'me AccountInfo<'info>,
    pub vault_program: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub token_program_2022: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct WithdrawKeys {
    pub user: Pubkey,
    pub user_pool_token: Pubkey,
    pub mint: Pubkey,
    pub pool: Pubkey,
    pub withdraw_authority: Pubkey,
    pub vault: Pubkey,
    pub vault_authority: Pubkey,
    pub vault_program: Pubkey,
    pub token_program: Pubkey,
    pub token_program_2022: Pubkey,
}
impl From<WithdrawAccounts<'_, '_>> for WithdrawKeys {
    fn from(accounts: WithdrawAccounts) -> Self {
        Self {
            user: *accounts.user.key,
            user_pool_token: *accounts.user_pool_token.key,
            mint: *accounts.mint.key,
            pool: *accounts.pool.key,
            withdraw_authority: *accounts.withdraw_authority.key,
            vault: *accounts.vault.key,
            vault_authority: *accounts.vault_authority.key,
            vault_program: *accounts.vault_program.key,
            token_program: *accounts.token_program.key,
            token_program_2022: *accounts.token_program_2022.key,
        }
    }
}
impl From<WithdrawKeys> for [AccountMeta; WITHDRAW_IX_ACCOUNTS_LEN] {
    fn from(keys: WithdrawKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.user,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.user_pool_token,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.withdraw_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_program_2022,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; WITHDRAW_IX_ACCOUNTS_LEN]> for WithdrawKeys {
    fn from(pubkeys: [Pubkey; WITHDRAW_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            user: pubkeys[0],
            user_pool_token: pubkeys[1],
            mint: pubkeys[2],
            pool: pubkeys[3],
            withdraw_authority: pubkeys[4],
            vault: pubkeys[5],
            vault_authority: pubkeys[6],
            vault_program: pubkeys[7],
            token_program: pubkeys[8],
            token_program_2022: pubkeys[9],
        }
    }
}
impl<'info> From<WithdrawAccounts<'_, 'info>>
for [AccountInfo<'info>; WITHDRAW_IX_ACCOUNTS_LEN] {
    fn from(accounts: WithdrawAccounts<'_, 'info>) -> Self {
        [
            accounts.user.clone(),
            accounts.user_pool_token.clone(),
            accounts.mint.clone(),
            accounts.pool.clone(),
            accounts.withdraw_authority.clone(),
            accounts.vault.clone(),
            accounts.vault_authority.clone(),
            accounts.vault_program.clone(),
            accounts.token_program.clone(),
            accounts.token_program_2022.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; WITHDRAW_IX_ACCOUNTS_LEN]>
for WithdrawAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; WITHDRAW_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            user: &arr[0],
            user_pool_token: &arr[1],
            mint: &arr[2],
            pool: &arr[3],
            withdraw_authority: &arr[4],
            vault: &arr[5],
            vault_authority: &arr[6],
            vault_program: &arr[7],
            token_program: &arr[8],
            token_program_2022: &arr[9],
        }
    }
}
pub const WITHDRAW_IX_DISCM: [u8; 8usize] = [183, 18, 70, 156, 148, 109, 161, 34];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WithdrawIxArgs {
    pub amount: u64,
    pub minimum_amounts_out: Vec<u64>,
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
        let minimum_amounts_out: Vec<u64> = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(WithdrawIxArgs {
                amount,
                minimum_amounts_out,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&WITHDRAW_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.minimum_amounts_out, &mut writer)?;
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
    withdraw_ix_with_program_id(WEIGHTED_SWAP_PROGRAM_ID, keys, args)
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
    withdraw_invoke_with_program_id(WEIGHTED_SWAP_PROGRAM_ID, accounts, args)
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
        WEIGHTED_SWAP_PROGRAM_ID,
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
        (*accounts.user.key, keys.user),
        (*accounts.user_pool_token.key, keys.user_pool_token),
        (*accounts.mint.key, keys.mint),
        (*accounts.pool.key, keys.pool),
        (*accounts.withdraw_authority.key, keys.withdraw_authority),
        (*accounts.vault.key, keys.vault),
        (*accounts.vault_authority.key, keys.vault_authority),
        (*accounts.vault_program.key, keys.vault_program),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.token_program_2022.key, keys.token_program_2022),
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
    for should_be_writable in [accounts.user_pool_token, accounts.mint, accounts.pool] {
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
