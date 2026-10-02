use solana_pubkey::Pubkey;
use solana_cpi::{invoke, invoke_signed};
use solana_instruction::{AccountMeta, Instruction};
use solana_account_info::AccountInfo;
use solana_program_error::ProgramError;
use std::io::Read;
#[allow(unused_imports)]
use crate::*;
#[derive(Clone, Debug, PartialEq)]
pub enum StableswapProgramIx {
    AddPair(AddPairIxArgs),
    AddToWhitelist,
    AddToken,
    ChangeAdmin(ChangeAdminIxArgs),
    Deposit(DepositIxArgs),
    Freeze,
    Initialize(InitializeIxArgs),
    RemoveFromWhitelist,
    RemovePair,
    RemoveToken,
    Swap(SwapIxArgs),
    SweepFees(SweepFeesIxArgs),
    Unfreeze,
    UpdatePair(UpdatePairIxArgs),
    UpdateRouterPubkey(UpdateRouterPubkeyIxArgs),
    Withdraw(WithdrawIxArgs),
}
impl StableswapProgramIx {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        if buf.starts_with(&ADD_PAIR_IX_DISCM) {
            let mut reader = &buf[ADD_PAIR_IX_DISCM.len()..];
            let x_to_y_fee_bps: i16 = crate::borsh_de_or_default(&mut reader)?;
            let y_to_x_fee_bps: i16 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::AddPair(AddPairIxArgs {
                    x_to_y_fee_bps,
                    y_to_x_fee_bps,
                }),
            );
        }
        if buf.starts_with(&ADD_TO_WHITELIST_IX_DISCM) {
            return Ok(Self::AddToWhitelist);
        }
        if buf.starts_with(&ADD_TOKEN_IX_DISCM) {
            return Ok(Self::AddToken);
        }
        if buf.starts_with(&CHANGE_ADMIN_IX_DISCM) {
            let mut reader = &buf[CHANGE_ADMIN_IX_DISCM.len()..];
            let new_admin: Pubkey = crate::borsh_de_or_default(&mut reader)?;
            return Ok(Self::ChangeAdmin(ChangeAdminIxArgs { new_admin }));
        }
        if buf.starts_with(&DEPOSIT_IX_DISCM) {
            let mut reader = &buf[DEPOSIT_IX_DISCM.len()..];
            let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(Self::Deposit(DepositIxArgs { amount }));
        }
        if buf.starts_with(&FREEZE_IX_DISCM) {
            return Ok(Self::Freeze);
        }
        if buf.starts_with(&INITIALIZE_IX_DISCM) {
            let mut reader = &buf[INITIALIZE_IX_DISCM.len()..];
            let router_pubkey: Pubkey = crate::borsh_de_or_default(&mut reader)?;
            return Ok(Self::Initialize(InitializeIxArgs { router_pubkey }));
        }
        if buf.starts_with(&REMOVE_FROM_WHITELIST_IX_DISCM) {
            return Ok(Self::RemoveFromWhitelist);
        }
        if buf.starts_with(&REMOVE_PAIR_IX_DISCM) {
            return Ok(Self::RemovePair);
        }
        if buf.starts_with(&REMOVE_TOKEN_IX_DISCM) {
            return Ok(Self::RemoveToken);
        }
        if buf.starts_with(&SWAP_IX_DISCM) {
            let mut reader = &buf[SWAP_IX_DISCM.len()..];
            let amount_in: u64 = crate::borsh_de_or_default(&mut reader)?;
            let minimum_amount_out: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::Swap(SwapIxArgs {
                    amount_in,
                    minimum_amount_out,
                }),
            );
        }
        if buf.starts_with(&SWEEP_FEES_IX_DISCM) {
            let mut reader = &buf[SWEEP_FEES_IX_DISCM.len()..];
            let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(Self::SweepFees(SweepFeesIxArgs { amount }));
        }
        if buf.starts_with(&UNFREEZE_IX_DISCM) {
            return Ok(Self::Unfreeze);
        }
        if buf.starts_with(&UPDATE_PAIR_IX_DISCM) {
            let mut reader = &buf[UPDATE_PAIR_IX_DISCM.len()..];
            let x_to_y_fee_bps: i16 = crate::borsh_de_or_default(&mut reader)?;
            let y_to_x_fee_bps: i16 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::UpdatePair(UpdatePairIxArgs {
                    x_to_y_fee_bps,
                    y_to_x_fee_bps,
                }),
            );
        }
        if buf.starts_with(&UPDATE_ROUTER_PUBKEY_IX_DISCM) {
            let mut reader = &buf[UPDATE_ROUTER_PUBKEY_IX_DISCM.len()..];
            let router_pubkey: Pubkey = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::UpdateRouterPubkey(UpdateRouterPubkeyIxArgs {
                    router_pubkey,
                }),
            );
        }
        if buf.starts_with(&WITHDRAW_IX_DISCM) {
            let mut reader = &buf[WITHDRAW_IX_DISCM.len()..];
            let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(Self::Withdraw(WithdrawIxArgs { amount }));
        }
        Err(std::io::Error::from(std::io::ErrorKind::InvalidData))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        match self {
            Self::AddPair(args) => {
                writer.write_all(&ADD_PAIR_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.x_to_y_fee_bps, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.y_to_x_fee_bps, &mut writer)?;
                Ok(())
            }
            Self::AddToWhitelist => writer.write_all(&ADD_TO_WHITELIST_IX_DISCM),
            Self::AddToken => writer.write_all(&ADD_TOKEN_IX_DISCM),
            Self::ChangeAdmin(args) => {
                writer.write_all(&CHANGE_ADMIN_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.new_admin, &mut writer)?;
                Ok(())
            }
            Self::Deposit(args) => {
                writer.write_all(&DEPOSIT_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.amount, &mut writer)?;
                Ok(())
            }
            Self::Freeze => writer.write_all(&FREEZE_IX_DISCM),
            Self::Initialize(args) => {
                writer.write_all(&INITIALIZE_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.router_pubkey, &mut writer)?;
                Ok(())
            }
            Self::RemoveFromWhitelist => {
                writer.write_all(&REMOVE_FROM_WHITELIST_IX_DISCM)
            }
            Self::RemovePair => writer.write_all(&REMOVE_PAIR_IX_DISCM),
            Self::RemoveToken => writer.write_all(&REMOVE_TOKEN_IX_DISCM),
            Self::Swap(args) => {
                writer.write_all(&SWAP_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.amount_in, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.minimum_amount_out, &mut writer)?;
                Ok(())
            }
            Self::SweepFees(args) => {
                writer.write_all(&SWEEP_FEES_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.amount, &mut writer)?;
                Ok(())
            }
            Self::Unfreeze => writer.write_all(&UNFREEZE_IX_DISCM),
            Self::UpdatePair(args) => {
                writer.write_all(&UPDATE_PAIR_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.x_to_y_fee_bps, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.y_to_x_fee_bps, &mut writer)?;
                Ok(())
            }
            Self::UpdateRouterPubkey(args) => {
                writer.write_all(&UPDATE_ROUTER_PUBKEY_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.router_pubkey, &mut writer)?;
                Ok(())
            }
            Self::Withdraw(args) => {
                writer.write_all(&WITHDRAW_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.amount, &mut writer)?;
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
pub const ADD_PAIR_IX_ACCOUNTS_LEN: usize = 8;
#[derive(Copy, Clone, Debug)]
pub struct AddPairAccounts<'me, 'info> {
    pub admin: &'me AccountInfo<'info>,
    pub exchange_state: &'me AccountInfo<'info>,
    pub mint_x: &'me AccountInfo<'info>,
    pub mint_y: &'me AccountInfo<'info>,
    pub token_state_x: &'me AccountInfo<'info>,
    pub token_state_y: &'me AccountInfo<'info>,
    pub pair_state: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct AddPairKeys {
    pub admin: Pubkey,
    pub exchange_state: Pubkey,
    pub mint_x: Pubkey,
    pub mint_y: Pubkey,
    pub token_state_x: Pubkey,
    pub token_state_y: Pubkey,
    pub pair_state: Pubkey,
    pub system_program: Pubkey,
}
impl From<AddPairAccounts<'_, '_>> for AddPairKeys {
    fn from(accounts: AddPairAccounts) -> Self {
        Self {
            admin: *accounts.admin.key,
            exchange_state: *accounts.exchange_state.key,
            mint_x: *accounts.mint_x.key,
            mint_y: *accounts.mint_y.key,
            token_state_x: *accounts.token_state_x.key,
            token_state_y: *accounts.token_state_y.key,
            pair_state: *accounts.pair_state.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<AddPairKeys> for [AccountMeta; ADD_PAIR_IX_ACCOUNTS_LEN] {
    fn from(keys: AddPairKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.admin,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.exchange_state,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.mint_x,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.mint_y,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_state_x,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_state_y,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pair_state,
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
impl From<[Pubkey; ADD_PAIR_IX_ACCOUNTS_LEN]> for AddPairKeys {
    fn from(pubkeys: [Pubkey; ADD_PAIR_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin: pubkeys[0],
            exchange_state: pubkeys[1],
            mint_x: pubkeys[2],
            mint_y: pubkeys[3],
            token_state_x: pubkeys[4],
            token_state_y: pubkeys[5],
            pair_state: pubkeys[6],
            system_program: pubkeys[7],
        }
    }
}
impl<'info> From<AddPairAccounts<'_, 'info>>
for [AccountInfo<'info>; ADD_PAIR_IX_ACCOUNTS_LEN] {
    fn from(accounts: AddPairAccounts<'_, 'info>) -> Self {
        [
            accounts.admin.clone(),
            accounts.exchange_state.clone(),
            accounts.mint_x.clone(),
            accounts.mint_y.clone(),
            accounts.token_state_x.clone(),
            accounts.token_state_y.clone(),
            accounts.pair_state.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; ADD_PAIR_IX_ACCOUNTS_LEN]>
for AddPairAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; ADD_PAIR_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin: &arr[0],
            exchange_state: &arr[1],
            mint_x: &arr[2],
            mint_y: &arr[3],
            token_state_x: &arr[4],
            token_state_y: &arr[5],
            pair_state: &arr[6],
            system_program: &arr[7],
        }
    }
}
pub const ADD_PAIR_IX_DISCM: [u8; 8usize] = [209, 230, 17, 236, 218, 162, 86, 118];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AddPairIxArgs {
    pub x_to_y_fee_bps: i16,
    pub y_to_x_fee_bps: i16,
}
#[derive(Clone, Debug, PartialEq)]
pub struct AddPairIxData(pub AddPairIxArgs);
impl From<AddPairIxArgs> for AddPairIxData {
    fn from(args: AddPairIxArgs) -> Self {
        Self(args)
    }
}
impl AddPairIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != ADD_PAIR_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let x_to_y_fee_bps: i16 = crate::borsh_de_or_default(&mut reader)?;
        let y_to_x_fee_bps: i16 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(AddPairIxArgs {
                x_to_y_fee_bps,
                y_to_x_fee_bps,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&ADD_PAIR_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.x_to_y_fee_bps, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.y_to_x_fee_bps, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn add_pair_ix_with_program_id(
    program_id: Pubkey,
    keys: AddPairKeys,
    args: AddPairIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; ADD_PAIR_IX_ACCOUNTS_LEN] = keys.into();
    let data: AddPairIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn add_pair_ix(
    keys: AddPairKeys,
    args: AddPairIxArgs,
) -> std::io::Result<Instruction> {
    add_pair_ix_with_program_id(STABLESWAP_PROGRAM_ID, keys, args)
}
pub fn add_pair_invoke_with_program_id(
    program_id: Pubkey,
    accounts: AddPairAccounts<'_, '_>,
    args: AddPairIxArgs,
) -> ProgramResult {
    let keys: AddPairKeys = accounts.into();
    let ix = add_pair_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn add_pair_invoke(
    accounts: AddPairAccounts<'_, '_>,
    args: AddPairIxArgs,
) -> ProgramResult {
    add_pair_invoke_with_program_id(STABLESWAP_PROGRAM_ID, accounts, args)
}
pub fn add_pair_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: AddPairAccounts<'_, '_>,
    args: AddPairIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: AddPairKeys = accounts.into();
    let ix = add_pair_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn add_pair_invoke_signed(
    accounts: AddPairAccounts<'_, '_>,
    args: AddPairIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    add_pair_invoke_signed_with_program_id(STABLESWAP_PROGRAM_ID, accounts, args, seeds)
}
pub fn add_pair_verify_account_keys(
    accounts: AddPairAccounts<'_, '_>,
    keys: AddPairKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.admin.key, keys.admin),
        (*accounts.exchange_state.key, keys.exchange_state),
        (*accounts.mint_x.key, keys.mint_x),
        (*accounts.mint_y.key, keys.mint_y),
        (*accounts.token_state_x.key, keys.token_state_x),
        (*accounts.token_state_y.key, keys.token_state_y),
        (*accounts.pair_state.key, keys.pair_state),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn add_pair_verify_writable_privileges<'me, 'info>(
    accounts: AddPairAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.admin, accounts.pair_state] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn add_pair_verify_signer_privileges<'me, 'info>(
    accounts: AddPairAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn add_pair_verify_account_privileges<'me, 'info>(
    accounts: AddPairAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    add_pair_verify_writable_privileges(accounts)?;
    add_pair_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const ADD_TO_WHITELIST_IX_ACCOUNTS_LEN: usize = 5;
#[derive(Copy, Clone, Debug)]
pub struct AddToWhitelistAccounts<'me, 'info> {
    pub admin: &'me AccountInfo<'info>,
    pub exchange_state: &'me AccountInfo<'info>,
    pub wallet: &'me AccountInfo<'info>,
    pub whitelist_entry: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct AddToWhitelistKeys {
    pub admin: Pubkey,
    pub exchange_state: Pubkey,
    pub wallet: Pubkey,
    pub whitelist_entry: Pubkey,
    pub system_program: Pubkey,
}
impl From<AddToWhitelistAccounts<'_, '_>> for AddToWhitelistKeys {
    fn from(accounts: AddToWhitelistAccounts) -> Self {
        Self {
            admin: *accounts.admin.key,
            exchange_state: *accounts.exchange_state.key,
            wallet: *accounts.wallet.key,
            whitelist_entry: *accounts.whitelist_entry.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<AddToWhitelistKeys> for [AccountMeta; ADD_TO_WHITELIST_IX_ACCOUNTS_LEN] {
    fn from(keys: AddToWhitelistKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.admin,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.exchange_state,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.wallet,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.whitelist_entry,
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
impl From<[Pubkey; ADD_TO_WHITELIST_IX_ACCOUNTS_LEN]> for AddToWhitelistKeys {
    fn from(pubkeys: [Pubkey; ADD_TO_WHITELIST_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin: pubkeys[0],
            exchange_state: pubkeys[1],
            wallet: pubkeys[2],
            whitelist_entry: pubkeys[3],
            system_program: pubkeys[4],
        }
    }
}
impl<'info> From<AddToWhitelistAccounts<'_, 'info>>
for [AccountInfo<'info>; ADD_TO_WHITELIST_IX_ACCOUNTS_LEN] {
    fn from(accounts: AddToWhitelistAccounts<'_, 'info>) -> Self {
        [
            accounts.admin.clone(),
            accounts.exchange_state.clone(),
            accounts.wallet.clone(),
            accounts.whitelist_entry.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; ADD_TO_WHITELIST_IX_ACCOUNTS_LEN]>
for AddToWhitelistAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; ADD_TO_WHITELIST_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin: &arr[0],
            exchange_state: &arr[1],
            wallet: &arr[2],
            whitelist_entry: &arr[3],
            system_program: &arr[4],
        }
    }
}
pub const ADD_TO_WHITELIST_IX_DISCM: [u8; 8usize] = [157, 211, 52, 54, 144, 81, 5, 55];
#[derive(Clone, Debug, PartialEq)]
pub struct AddToWhitelistIxData;
impl AddToWhitelistIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != ADD_TO_WHITELIST_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&ADD_TO_WHITELIST_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn add_to_whitelist_ix_with_program_id(
    program_id: Pubkey,
    keys: AddToWhitelistKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; ADD_TO_WHITELIST_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: AddToWhitelistIxData.try_to_vec()?,
    })
}
pub fn add_to_whitelist_ix(keys: AddToWhitelistKeys) -> std::io::Result<Instruction> {
    add_to_whitelist_ix_with_program_id(STABLESWAP_PROGRAM_ID, keys)
}
pub fn add_to_whitelist_invoke_with_program_id(
    program_id: Pubkey,
    accounts: AddToWhitelistAccounts<'_, '_>,
) -> ProgramResult {
    let keys: AddToWhitelistKeys = accounts.into();
    let ix = add_to_whitelist_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn add_to_whitelist_invoke(
    accounts: AddToWhitelistAccounts<'_, '_>,
) -> ProgramResult {
    add_to_whitelist_invoke_with_program_id(STABLESWAP_PROGRAM_ID, accounts)
}
pub fn add_to_whitelist_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: AddToWhitelistAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: AddToWhitelistKeys = accounts.into();
    let ix = add_to_whitelist_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn add_to_whitelist_invoke_signed(
    accounts: AddToWhitelistAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    add_to_whitelist_invoke_signed_with_program_id(
        STABLESWAP_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn add_to_whitelist_verify_account_keys(
    accounts: AddToWhitelistAccounts<'_, '_>,
    keys: AddToWhitelistKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.admin.key, keys.admin),
        (*accounts.exchange_state.key, keys.exchange_state),
        (*accounts.wallet.key, keys.wallet),
        (*accounts.whitelist_entry.key, keys.whitelist_entry),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn add_to_whitelist_verify_writable_privileges<'me, 'info>(
    accounts: AddToWhitelistAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.admin, accounts.whitelist_entry] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn add_to_whitelist_verify_signer_privileges<'me, 'info>(
    accounts: AddToWhitelistAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn add_to_whitelist_verify_account_privileges<'me, 'info>(
    accounts: AddToWhitelistAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    add_to_whitelist_verify_writable_privileges(accounts)?;
    add_to_whitelist_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const ADD_TOKEN_IX_ACCOUNTS_LEN: usize = 7;
#[derive(Copy, Clone, Debug)]
pub struct AddTokenAccounts<'me, 'info> {
    pub admin: &'me AccountInfo<'info>,
    pub exchange_state: &'me AccountInfo<'info>,
    pub mint: &'me AccountInfo<'info>,
    pub token_state: &'me AccountInfo<'info>,
    pub token_vault: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct AddTokenKeys {
    pub admin: Pubkey,
    pub exchange_state: Pubkey,
    pub mint: Pubkey,
    pub token_state: Pubkey,
    pub token_vault: Pubkey,
    pub system_program: Pubkey,
    pub token_program: Pubkey,
}
impl From<AddTokenAccounts<'_, '_>> for AddTokenKeys {
    fn from(accounts: AddTokenAccounts) -> Self {
        Self {
            admin: *accounts.admin.key,
            exchange_state: *accounts.exchange_state.key,
            mint: *accounts.mint.key,
            token_state: *accounts.token_state.key,
            token_vault: *accounts.token_vault.key,
            system_program: *accounts.system_program.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<AddTokenKeys> for [AccountMeta; ADD_TOKEN_IX_ACCOUNTS_LEN] {
    fn from(keys: AddTokenKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.admin,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.exchange_state,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_vault,
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
impl From<[Pubkey; ADD_TOKEN_IX_ACCOUNTS_LEN]> for AddTokenKeys {
    fn from(pubkeys: [Pubkey; ADD_TOKEN_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin: pubkeys[0],
            exchange_state: pubkeys[1],
            mint: pubkeys[2],
            token_state: pubkeys[3],
            token_vault: pubkeys[4],
            system_program: pubkeys[5],
            token_program: pubkeys[6],
        }
    }
}
impl<'info> From<AddTokenAccounts<'_, 'info>>
for [AccountInfo<'info>; ADD_TOKEN_IX_ACCOUNTS_LEN] {
    fn from(accounts: AddTokenAccounts<'_, 'info>) -> Self {
        [
            accounts.admin.clone(),
            accounts.exchange_state.clone(),
            accounts.mint.clone(),
            accounts.token_state.clone(),
            accounts.token_vault.clone(),
            accounts.system_program.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; ADD_TOKEN_IX_ACCOUNTS_LEN]>
for AddTokenAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; ADD_TOKEN_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin: &arr[0],
            exchange_state: &arr[1],
            mint: &arr[2],
            token_state: &arr[3],
            token_vault: &arr[4],
            system_program: &arr[5],
            token_program: &arr[6],
        }
    }
}
pub const ADD_TOKEN_IX_DISCM: [u8; 8usize] = [237, 255, 26, 54, 56, 48, 68, 52];
#[derive(Clone, Debug, PartialEq)]
pub struct AddTokenIxData;
impl AddTokenIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != ADD_TOKEN_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&ADD_TOKEN_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn add_token_ix_with_program_id(
    program_id: Pubkey,
    keys: AddTokenKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; ADD_TOKEN_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: AddTokenIxData.try_to_vec()?,
    })
}
pub fn add_token_ix(keys: AddTokenKeys) -> std::io::Result<Instruction> {
    add_token_ix_with_program_id(STABLESWAP_PROGRAM_ID, keys)
}
pub fn add_token_invoke_with_program_id(
    program_id: Pubkey,
    accounts: AddTokenAccounts<'_, '_>,
) -> ProgramResult {
    let keys: AddTokenKeys = accounts.into();
    let ix = add_token_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn add_token_invoke(accounts: AddTokenAccounts<'_, '_>) -> ProgramResult {
    add_token_invoke_with_program_id(STABLESWAP_PROGRAM_ID, accounts)
}
pub fn add_token_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: AddTokenAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: AddTokenKeys = accounts.into();
    let ix = add_token_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn add_token_invoke_signed(
    accounts: AddTokenAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    add_token_invoke_signed_with_program_id(STABLESWAP_PROGRAM_ID, accounts, seeds)
}
pub fn add_token_verify_account_keys(
    accounts: AddTokenAccounts<'_, '_>,
    keys: AddTokenKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.admin.key, keys.admin),
        (*accounts.exchange_state.key, keys.exchange_state),
        (*accounts.mint.key, keys.mint),
        (*accounts.token_state.key, keys.token_state),
        (*accounts.token_vault.key, keys.token_vault),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.token_program.key, keys.token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn add_token_verify_writable_privileges<'me, 'info>(
    accounts: AddTokenAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.admin,
        accounts.token_state,
        accounts.token_vault,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn add_token_verify_signer_privileges<'me, 'info>(
    accounts: AddTokenAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn add_token_verify_account_privileges<'me, 'info>(
    accounts: AddTokenAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    add_token_verify_writable_privileges(accounts)?;
    add_token_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CHANGE_ADMIN_IX_ACCOUNTS_LEN: usize = 2;
#[derive(Copy, Clone, Debug)]
pub struct ChangeAdminAccounts<'me, 'info> {
    pub admin: &'me AccountInfo<'info>,
    pub exchange_state: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ChangeAdminKeys {
    pub admin: Pubkey,
    pub exchange_state: Pubkey,
}
impl From<ChangeAdminAccounts<'_, '_>> for ChangeAdminKeys {
    fn from(accounts: ChangeAdminAccounts) -> Self {
        Self {
            admin: *accounts.admin.key,
            exchange_state: *accounts.exchange_state.key,
        }
    }
}
impl From<ChangeAdminKeys> for [AccountMeta; CHANGE_ADMIN_IX_ACCOUNTS_LEN] {
    fn from(keys: ChangeAdminKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.admin,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.exchange_state,
                is_signer: false,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; CHANGE_ADMIN_IX_ACCOUNTS_LEN]> for ChangeAdminKeys {
    fn from(pubkeys: [Pubkey; CHANGE_ADMIN_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin: pubkeys[0],
            exchange_state: pubkeys[1],
        }
    }
}
impl<'info> From<ChangeAdminAccounts<'_, 'info>>
for [AccountInfo<'info>; CHANGE_ADMIN_IX_ACCOUNTS_LEN] {
    fn from(accounts: ChangeAdminAccounts<'_, 'info>) -> Self {
        [accounts.admin.clone(), accounts.exchange_state.clone()]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CHANGE_ADMIN_IX_ACCOUNTS_LEN]>
for ChangeAdminAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; CHANGE_ADMIN_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin: &arr[0],
            exchange_state: &arr[1],
        }
    }
}
pub const CHANGE_ADMIN_IX_DISCM: [u8; 8usize] = [193, 151, 203, 161, 200, 202, 32, 146];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ChangeAdminIxArgs {
    pub new_admin: Pubkey,
}
#[derive(Clone, Debug, PartialEq)]
pub struct ChangeAdminIxData(pub ChangeAdminIxArgs);
impl From<ChangeAdminIxArgs> for ChangeAdminIxData {
    fn from(args: ChangeAdminIxArgs) -> Self {
        Self(args)
    }
}
impl ChangeAdminIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CHANGE_ADMIN_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let new_admin: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        Ok(Self(ChangeAdminIxArgs { new_admin }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CHANGE_ADMIN_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.new_admin, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn change_admin_ix_with_program_id(
    program_id: Pubkey,
    keys: ChangeAdminKeys,
    args: ChangeAdminIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CHANGE_ADMIN_IX_ACCOUNTS_LEN] = keys.into();
    let data: ChangeAdminIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn change_admin_ix(
    keys: ChangeAdminKeys,
    args: ChangeAdminIxArgs,
) -> std::io::Result<Instruction> {
    change_admin_ix_with_program_id(STABLESWAP_PROGRAM_ID, keys, args)
}
pub fn change_admin_invoke_with_program_id(
    program_id: Pubkey,
    accounts: ChangeAdminAccounts<'_, '_>,
    args: ChangeAdminIxArgs,
) -> ProgramResult {
    let keys: ChangeAdminKeys = accounts.into();
    let ix = change_admin_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn change_admin_invoke(
    accounts: ChangeAdminAccounts<'_, '_>,
    args: ChangeAdminIxArgs,
) -> ProgramResult {
    change_admin_invoke_with_program_id(STABLESWAP_PROGRAM_ID, accounts, args)
}
pub fn change_admin_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: ChangeAdminAccounts<'_, '_>,
    args: ChangeAdminIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: ChangeAdminKeys = accounts.into();
    let ix = change_admin_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn change_admin_invoke_signed(
    accounts: ChangeAdminAccounts<'_, '_>,
    args: ChangeAdminIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    change_admin_invoke_signed_with_program_id(
        STABLESWAP_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn change_admin_verify_account_keys(
    accounts: ChangeAdminAccounts<'_, '_>,
    keys: ChangeAdminKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.admin.key, keys.admin),
        (*accounts.exchange_state.key, keys.exchange_state),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn change_admin_verify_writable_privileges<'me, 'info>(
    accounts: ChangeAdminAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.admin, accounts.exchange_state] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn change_admin_verify_signer_privileges<'me, 'info>(
    accounts: ChangeAdminAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn change_admin_verify_account_privileges<'me, 'info>(
    accounts: ChangeAdminAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    change_admin_verify_writable_privileges(accounts)?;
    change_admin_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const DEPOSIT_IX_ACCOUNTS_LEN: usize = 10;
#[derive(Copy, Clone, Debug)]
pub struct DepositAccounts<'me, 'info> {
    pub depositor: &'me AccountInfo<'info>,
    pub exchange_state: &'me AccountInfo<'info>,
    pub mint: &'me AccountInfo<'info>,
    pub token_state: &'me AccountInfo<'info>,
    pub token_vault: &'me AccountInfo<'info>,
    pub whitelist_entry: &'me AccountInfo<'info>,
    pub lp_position: &'me AccountInfo<'info>,
    pub depositor_token_account: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct DepositKeys {
    pub depositor: Pubkey,
    pub exchange_state: Pubkey,
    pub mint: Pubkey,
    pub token_state: Pubkey,
    pub token_vault: Pubkey,
    pub whitelist_entry: Pubkey,
    pub lp_position: Pubkey,
    pub depositor_token_account: Pubkey,
    pub system_program: Pubkey,
    pub token_program: Pubkey,
}
impl From<DepositAccounts<'_, '_>> for DepositKeys {
    fn from(accounts: DepositAccounts) -> Self {
        Self {
            depositor: *accounts.depositor.key,
            exchange_state: *accounts.exchange_state.key,
            mint: *accounts.mint.key,
            token_state: *accounts.token_state.key,
            token_vault: *accounts.token_vault.key,
            whitelist_entry: *accounts.whitelist_entry.key,
            lp_position: *accounts.lp_position.key,
            depositor_token_account: *accounts.depositor_token_account.key,
            system_program: *accounts.system_program.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<DepositKeys> for [AccountMeta; DEPOSIT_IX_ACCOUNTS_LEN] {
    fn from(keys: DepositKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.depositor,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.exchange_state,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_state,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.whitelist_entry,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.lp_position,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.depositor_token_account,
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
impl From<[Pubkey; DEPOSIT_IX_ACCOUNTS_LEN]> for DepositKeys {
    fn from(pubkeys: [Pubkey; DEPOSIT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            depositor: pubkeys[0],
            exchange_state: pubkeys[1],
            mint: pubkeys[2],
            token_state: pubkeys[3],
            token_vault: pubkeys[4],
            whitelist_entry: pubkeys[5],
            lp_position: pubkeys[6],
            depositor_token_account: pubkeys[7],
            system_program: pubkeys[8],
            token_program: pubkeys[9],
        }
    }
}
impl<'info> From<DepositAccounts<'_, 'info>>
for [AccountInfo<'info>; DEPOSIT_IX_ACCOUNTS_LEN] {
    fn from(accounts: DepositAccounts<'_, 'info>) -> Self {
        [
            accounts.depositor.clone(),
            accounts.exchange_state.clone(),
            accounts.mint.clone(),
            accounts.token_state.clone(),
            accounts.token_vault.clone(),
            accounts.whitelist_entry.clone(),
            accounts.lp_position.clone(),
            accounts.depositor_token_account.clone(),
            accounts.system_program.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; DEPOSIT_IX_ACCOUNTS_LEN]>
for DepositAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; DEPOSIT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            depositor: &arr[0],
            exchange_state: &arr[1],
            mint: &arr[2],
            token_state: &arr[3],
            token_vault: &arr[4],
            whitelist_entry: &arr[5],
            lp_position: &arr[6],
            depositor_token_account: &arr[7],
            system_program: &arr[8],
            token_program: &arr[9],
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
    deposit_ix_with_program_id(STABLESWAP_PROGRAM_ID, keys, args)
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
    deposit_invoke_with_program_id(STABLESWAP_PROGRAM_ID, accounts, args)
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
    deposit_invoke_signed_with_program_id(STABLESWAP_PROGRAM_ID, accounts, args, seeds)
}
pub fn deposit_verify_account_keys(
    accounts: DepositAccounts<'_, '_>,
    keys: DepositKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.depositor.key, keys.depositor),
        (*accounts.exchange_state.key, keys.exchange_state),
        (*accounts.mint.key, keys.mint),
        (*accounts.token_state.key, keys.token_state),
        (*accounts.token_vault.key, keys.token_vault),
        (*accounts.whitelist_entry.key, keys.whitelist_entry),
        (*accounts.lp_position.key, keys.lp_position),
        (*accounts.depositor_token_account.key, keys.depositor_token_account),
        (*accounts.system_program.key, keys.system_program),
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
        accounts.depositor,
        accounts.token_vault,
        accounts.lp_position,
        accounts.depositor_token_account,
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
    for should_be_signer in [accounts.depositor] {
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
pub const FREEZE_IX_ACCOUNTS_LEN: usize = 2;
#[derive(Copy, Clone, Debug)]
pub struct FreezeAccounts<'me, 'info> {
    pub admin: &'me AccountInfo<'info>,
    pub exchange_state: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FreezeKeys {
    pub admin: Pubkey,
    pub exchange_state: Pubkey,
}
impl From<FreezeAccounts<'_, '_>> for FreezeKeys {
    fn from(accounts: FreezeAccounts) -> Self {
        Self {
            admin: *accounts.admin.key,
            exchange_state: *accounts.exchange_state.key,
        }
    }
}
impl From<FreezeKeys> for [AccountMeta; FREEZE_IX_ACCOUNTS_LEN] {
    fn from(keys: FreezeKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.admin,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.exchange_state,
                is_signer: false,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; FREEZE_IX_ACCOUNTS_LEN]> for FreezeKeys {
    fn from(pubkeys: [Pubkey; FREEZE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin: pubkeys[0],
            exchange_state: pubkeys[1],
        }
    }
}
impl<'info> From<FreezeAccounts<'_, 'info>>
for [AccountInfo<'info>; FREEZE_IX_ACCOUNTS_LEN] {
    fn from(accounts: FreezeAccounts<'_, 'info>) -> Self {
        [accounts.admin.clone(), accounts.exchange_state.clone()]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; FREEZE_IX_ACCOUNTS_LEN]>
for FreezeAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; FREEZE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin: &arr[0],
            exchange_state: &arr[1],
        }
    }
}
pub const FREEZE_IX_DISCM: [u8; 8usize] = [255, 91, 207, 84, 251, 194, 254, 63];
#[derive(Clone, Debug, PartialEq)]
pub struct FreezeIxData;
impl FreezeIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != FREEZE_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&FREEZE_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn freeze_ix_with_program_id(
    program_id: Pubkey,
    keys: FreezeKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; FREEZE_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: FreezeIxData.try_to_vec()?,
    })
}
pub fn freeze_ix(keys: FreezeKeys) -> std::io::Result<Instruction> {
    freeze_ix_with_program_id(STABLESWAP_PROGRAM_ID, keys)
}
pub fn freeze_invoke_with_program_id(
    program_id: Pubkey,
    accounts: FreezeAccounts<'_, '_>,
) -> ProgramResult {
    let keys: FreezeKeys = accounts.into();
    let ix = freeze_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn freeze_invoke(accounts: FreezeAccounts<'_, '_>) -> ProgramResult {
    freeze_invoke_with_program_id(STABLESWAP_PROGRAM_ID, accounts)
}
pub fn freeze_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: FreezeAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: FreezeKeys = accounts.into();
    let ix = freeze_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn freeze_invoke_signed(
    accounts: FreezeAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    freeze_invoke_signed_with_program_id(STABLESWAP_PROGRAM_ID, accounts, seeds)
}
pub fn freeze_verify_account_keys(
    accounts: FreezeAccounts<'_, '_>,
    keys: FreezeKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.admin.key, keys.admin),
        (*accounts.exchange_state.key, keys.exchange_state),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn freeze_verify_writable_privileges<'me, 'info>(
    accounts: FreezeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.admin, accounts.exchange_state] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn freeze_verify_signer_privileges<'me, 'info>(
    accounts: FreezeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn freeze_verify_account_privileges<'me, 'info>(
    accounts: FreezeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    freeze_verify_writable_privileges(accounts)?;
    freeze_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const INITIALIZE_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct InitializeAccounts<'me, 'info> {
    pub admin: &'me AccountInfo<'info>,
    pub exchange_state: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct InitializeKeys {
    pub admin: Pubkey,
    pub exchange_state: Pubkey,
    pub system_program: Pubkey,
}
impl From<InitializeAccounts<'_, '_>> for InitializeKeys {
    fn from(accounts: InitializeAccounts) -> Self {
        Self {
            admin: *accounts.admin.key,
            exchange_state: *accounts.exchange_state.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<InitializeKeys> for [AccountMeta; INITIALIZE_IX_ACCOUNTS_LEN] {
    fn from(keys: InitializeKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.admin,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.exchange_state,
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
impl From<[Pubkey; INITIALIZE_IX_ACCOUNTS_LEN]> for InitializeKeys {
    fn from(pubkeys: [Pubkey; INITIALIZE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin: pubkeys[0],
            exchange_state: pubkeys[1],
            system_program: pubkeys[2],
        }
    }
}
impl<'info> From<InitializeAccounts<'_, 'info>>
for [AccountInfo<'info>; INITIALIZE_IX_ACCOUNTS_LEN] {
    fn from(accounts: InitializeAccounts<'_, 'info>) -> Self {
        [
            accounts.admin.clone(),
            accounts.exchange_state.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; INITIALIZE_IX_ACCOUNTS_LEN]>
for InitializeAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; INITIALIZE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin: &arr[0],
            exchange_state: &arr[1],
            system_program: &arr[2],
        }
    }
}
pub const INITIALIZE_IX_DISCM: [u8; 8usize] = [175, 175, 109, 31, 13, 152, 155, 237];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct InitializeIxArgs {
    pub router_pubkey: Pubkey,
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
        let router_pubkey: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        Ok(Self(InitializeIxArgs { router_pubkey }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INITIALIZE_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.router_pubkey, &mut writer)?;
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
    initialize_ix_with_program_id(STABLESWAP_PROGRAM_ID, keys, args)
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
    initialize_invoke_with_program_id(STABLESWAP_PROGRAM_ID, accounts, args)
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
        STABLESWAP_PROGRAM_ID,
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
        (*accounts.admin.key, keys.admin),
        (*accounts.exchange_state.key, keys.exchange_state),
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
    for should_be_writable in [accounts.admin, accounts.exchange_state] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn initialize_verify_signer_privileges<'me, 'info>(
    accounts: InitializeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin] {
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
pub const REMOVE_FROM_WHITELIST_IX_ACCOUNTS_LEN: usize = 5;
#[derive(Copy, Clone, Debug)]
pub struct RemoveFromWhitelistAccounts<'me, 'info> {
    pub admin: &'me AccountInfo<'info>,
    pub exchange_state: &'me AccountInfo<'info>,
    pub wallet: &'me AccountInfo<'info>,
    pub whitelist_entry: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct RemoveFromWhitelistKeys {
    pub admin: Pubkey,
    pub exchange_state: Pubkey,
    pub wallet: Pubkey,
    pub whitelist_entry: Pubkey,
    pub system_program: Pubkey,
}
impl From<RemoveFromWhitelistAccounts<'_, '_>> for RemoveFromWhitelistKeys {
    fn from(accounts: RemoveFromWhitelistAccounts) -> Self {
        Self {
            admin: *accounts.admin.key,
            exchange_state: *accounts.exchange_state.key,
            wallet: *accounts.wallet.key,
            whitelist_entry: *accounts.whitelist_entry.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<RemoveFromWhitelistKeys>
for [AccountMeta; REMOVE_FROM_WHITELIST_IX_ACCOUNTS_LEN] {
    fn from(keys: RemoveFromWhitelistKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.admin,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.exchange_state,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.wallet,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.whitelist_entry,
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
impl From<[Pubkey; REMOVE_FROM_WHITELIST_IX_ACCOUNTS_LEN]> for RemoveFromWhitelistKeys {
    fn from(pubkeys: [Pubkey; REMOVE_FROM_WHITELIST_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin: pubkeys[0],
            exchange_state: pubkeys[1],
            wallet: pubkeys[2],
            whitelist_entry: pubkeys[3],
            system_program: pubkeys[4],
        }
    }
}
impl<'info> From<RemoveFromWhitelistAccounts<'_, 'info>>
for [AccountInfo<'info>; REMOVE_FROM_WHITELIST_IX_ACCOUNTS_LEN] {
    fn from(accounts: RemoveFromWhitelistAccounts<'_, 'info>) -> Self {
        [
            accounts.admin.clone(),
            accounts.exchange_state.clone(),
            accounts.wallet.clone(),
            accounts.whitelist_entry.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; REMOVE_FROM_WHITELIST_IX_ACCOUNTS_LEN]>
for RemoveFromWhitelistAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; REMOVE_FROM_WHITELIST_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            admin: &arr[0],
            exchange_state: &arr[1],
            wallet: &arr[2],
            whitelist_entry: &arr[3],
            system_program: &arr[4],
        }
    }
}
pub const REMOVE_FROM_WHITELIST_IX_DISCM: [u8; 8usize] = [
    7, 144, 216, 239, 243, 236, 193, 235,
];
#[derive(Clone, Debug, PartialEq)]
pub struct RemoveFromWhitelistIxData;
impl RemoveFromWhitelistIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != REMOVE_FROM_WHITELIST_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&REMOVE_FROM_WHITELIST_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn remove_from_whitelist_ix_with_program_id(
    program_id: Pubkey,
    keys: RemoveFromWhitelistKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; REMOVE_FROM_WHITELIST_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: RemoveFromWhitelistIxData.try_to_vec()?,
    })
}
pub fn remove_from_whitelist_ix(
    keys: RemoveFromWhitelistKeys,
) -> std::io::Result<Instruction> {
    remove_from_whitelist_ix_with_program_id(STABLESWAP_PROGRAM_ID, keys)
}
pub fn remove_from_whitelist_invoke_with_program_id(
    program_id: Pubkey,
    accounts: RemoveFromWhitelistAccounts<'_, '_>,
) -> ProgramResult {
    let keys: RemoveFromWhitelistKeys = accounts.into();
    let ix = remove_from_whitelist_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn remove_from_whitelist_invoke(
    accounts: RemoveFromWhitelistAccounts<'_, '_>,
) -> ProgramResult {
    remove_from_whitelist_invoke_with_program_id(STABLESWAP_PROGRAM_ID, accounts)
}
pub fn remove_from_whitelist_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: RemoveFromWhitelistAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: RemoveFromWhitelistKeys = accounts.into();
    let ix = remove_from_whitelist_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn remove_from_whitelist_invoke_signed(
    accounts: RemoveFromWhitelistAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    remove_from_whitelist_invoke_signed_with_program_id(
        STABLESWAP_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn remove_from_whitelist_verify_account_keys(
    accounts: RemoveFromWhitelistAccounts<'_, '_>,
    keys: RemoveFromWhitelistKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.admin.key, keys.admin),
        (*accounts.exchange_state.key, keys.exchange_state),
        (*accounts.wallet.key, keys.wallet),
        (*accounts.whitelist_entry.key, keys.whitelist_entry),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn remove_from_whitelist_verify_writable_privileges<'me, 'info>(
    accounts: RemoveFromWhitelistAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.admin, accounts.whitelist_entry] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn remove_from_whitelist_verify_signer_privileges<'me, 'info>(
    accounts: RemoveFromWhitelistAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn remove_from_whitelist_verify_account_privileges<'me, 'info>(
    accounts: RemoveFromWhitelistAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    remove_from_whitelist_verify_writable_privileges(accounts)?;
    remove_from_whitelist_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const REMOVE_PAIR_IX_ACCOUNTS_LEN: usize = 5;
#[derive(Copy, Clone, Debug)]
pub struct RemovePairAccounts<'me, 'info> {
    pub admin: &'me AccountInfo<'info>,
    pub exchange_state: &'me AccountInfo<'info>,
    pub mint_x: &'me AccountInfo<'info>,
    pub mint_y: &'me AccountInfo<'info>,
    pub pair_state: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct RemovePairKeys {
    pub admin: Pubkey,
    pub exchange_state: Pubkey,
    pub mint_x: Pubkey,
    pub mint_y: Pubkey,
    pub pair_state: Pubkey,
}
impl From<RemovePairAccounts<'_, '_>> for RemovePairKeys {
    fn from(accounts: RemovePairAccounts) -> Self {
        Self {
            admin: *accounts.admin.key,
            exchange_state: *accounts.exchange_state.key,
            mint_x: *accounts.mint_x.key,
            mint_y: *accounts.mint_y.key,
            pair_state: *accounts.pair_state.key,
        }
    }
}
impl From<RemovePairKeys> for [AccountMeta; REMOVE_PAIR_IX_ACCOUNTS_LEN] {
    fn from(keys: RemovePairKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.admin,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.exchange_state,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.mint_x,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.mint_y,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pair_state,
                is_signer: false,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; REMOVE_PAIR_IX_ACCOUNTS_LEN]> for RemovePairKeys {
    fn from(pubkeys: [Pubkey; REMOVE_PAIR_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin: pubkeys[0],
            exchange_state: pubkeys[1],
            mint_x: pubkeys[2],
            mint_y: pubkeys[3],
            pair_state: pubkeys[4],
        }
    }
}
impl<'info> From<RemovePairAccounts<'_, 'info>>
for [AccountInfo<'info>; REMOVE_PAIR_IX_ACCOUNTS_LEN] {
    fn from(accounts: RemovePairAccounts<'_, 'info>) -> Self {
        [
            accounts.admin.clone(),
            accounts.exchange_state.clone(),
            accounts.mint_x.clone(),
            accounts.mint_y.clone(),
            accounts.pair_state.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; REMOVE_PAIR_IX_ACCOUNTS_LEN]>
for RemovePairAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; REMOVE_PAIR_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin: &arr[0],
            exchange_state: &arr[1],
            mint_x: &arr[2],
            mint_y: &arr[3],
            pair_state: &arr[4],
        }
    }
}
pub const REMOVE_PAIR_IX_DISCM: [u8; 8usize] = [181, 42, 154, 249, 167, 123, 20, 81];
#[derive(Clone, Debug, PartialEq)]
pub struct RemovePairIxData;
impl RemovePairIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != REMOVE_PAIR_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&REMOVE_PAIR_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn remove_pair_ix_with_program_id(
    program_id: Pubkey,
    keys: RemovePairKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; REMOVE_PAIR_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: RemovePairIxData.try_to_vec()?,
    })
}
pub fn remove_pair_ix(keys: RemovePairKeys) -> std::io::Result<Instruction> {
    remove_pair_ix_with_program_id(STABLESWAP_PROGRAM_ID, keys)
}
pub fn remove_pair_invoke_with_program_id(
    program_id: Pubkey,
    accounts: RemovePairAccounts<'_, '_>,
) -> ProgramResult {
    let keys: RemovePairKeys = accounts.into();
    let ix = remove_pair_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn remove_pair_invoke(accounts: RemovePairAccounts<'_, '_>) -> ProgramResult {
    remove_pair_invoke_with_program_id(STABLESWAP_PROGRAM_ID, accounts)
}
pub fn remove_pair_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: RemovePairAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: RemovePairKeys = accounts.into();
    let ix = remove_pair_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn remove_pair_invoke_signed(
    accounts: RemovePairAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    remove_pair_invoke_signed_with_program_id(STABLESWAP_PROGRAM_ID, accounts, seeds)
}
pub fn remove_pair_verify_account_keys(
    accounts: RemovePairAccounts<'_, '_>,
    keys: RemovePairKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.admin.key, keys.admin),
        (*accounts.exchange_state.key, keys.exchange_state),
        (*accounts.mint_x.key, keys.mint_x),
        (*accounts.mint_y.key, keys.mint_y),
        (*accounts.pair_state.key, keys.pair_state),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn remove_pair_verify_writable_privileges<'me, 'info>(
    accounts: RemovePairAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.admin, accounts.pair_state] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn remove_pair_verify_signer_privileges<'me, 'info>(
    accounts: RemovePairAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn remove_pair_verify_account_privileges<'me, 'info>(
    accounts: RemovePairAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    remove_pair_verify_writable_privileges(accounts)?;
    remove_pair_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const REMOVE_TOKEN_IX_ACCOUNTS_LEN: usize = 6;
#[derive(Copy, Clone, Debug)]
pub struct RemoveTokenAccounts<'me, 'info> {
    pub admin: &'me AccountInfo<'info>,
    pub exchange_state: &'me AccountInfo<'info>,
    pub mint: &'me AccountInfo<'info>,
    pub token_state: &'me AccountInfo<'info>,
    pub vault: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct RemoveTokenKeys {
    pub admin: Pubkey,
    pub exchange_state: Pubkey,
    pub mint: Pubkey,
    pub token_state: Pubkey,
    pub vault: Pubkey,
    pub token_program: Pubkey,
}
impl From<RemoveTokenAccounts<'_, '_>> for RemoveTokenKeys {
    fn from(accounts: RemoveTokenAccounts) -> Self {
        Self {
            admin: *accounts.admin.key,
            exchange_state: *accounts.exchange_state.key,
            mint: *accounts.mint.key,
            token_state: *accounts.token_state.key,
            vault: *accounts.vault.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<RemoveTokenKeys> for [AccountMeta; REMOVE_TOKEN_IX_ACCOUNTS_LEN] {
    fn from(keys: RemoveTokenKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.admin,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.exchange_state,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_state,
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
        ]
    }
}
impl From<[Pubkey; REMOVE_TOKEN_IX_ACCOUNTS_LEN]> for RemoveTokenKeys {
    fn from(pubkeys: [Pubkey; REMOVE_TOKEN_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin: pubkeys[0],
            exchange_state: pubkeys[1],
            mint: pubkeys[2],
            token_state: pubkeys[3],
            vault: pubkeys[4],
            token_program: pubkeys[5],
        }
    }
}
impl<'info> From<RemoveTokenAccounts<'_, 'info>>
for [AccountInfo<'info>; REMOVE_TOKEN_IX_ACCOUNTS_LEN] {
    fn from(accounts: RemoveTokenAccounts<'_, 'info>) -> Self {
        [
            accounts.admin.clone(),
            accounts.exchange_state.clone(),
            accounts.mint.clone(),
            accounts.token_state.clone(),
            accounts.vault.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; REMOVE_TOKEN_IX_ACCOUNTS_LEN]>
for RemoveTokenAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; REMOVE_TOKEN_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin: &arr[0],
            exchange_state: &arr[1],
            mint: &arr[2],
            token_state: &arr[3],
            vault: &arr[4],
            token_program: &arr[5],
        }
    }
}
pub const REMOVE_TOKEN_IX_DISCM: [u8; 8usize] = [149, 134, 57, 61, 136, 2, 144, 145];
#[derive(Clone, Debug, PartialEq)]
pub struct RemoveTokenIxData;
impl RemoveTokenIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != REMOVE_TOKEN_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&REMOVE_TOKEN_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn remove_token_ix_with_program_id(
    program_id: Pubkey,
    keys: RemoveTokenKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; REMOVE_TOKEN_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: RemoveTokenIxData.try_to_vec()?,
    })
}
pub fn remove_token_ix(keys: RemoveTokenKeys) -> std::io::Result<Instruction> {
    remove_token_ix_with_program_id(STABLESWAP_PROGRAM_ID, keys)
}
pub fn remove_token_invoke_with_program_id(
    program_id: Pubkey,
    accounts: RemoveTokenAccounts<'_, '_>,
) -> ProgramResult {
    let keys: RemoveTokenKeys = accounts.into();
    let ix = remove_token_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn remove_token_invoke(accounts: RemoveTokenAccounts<'_, '_>) -> ProgramResult {
    remove_token_invoke_with_program_id(STABLESWAP_PROGRAM_ID, accounts)
}
pub fn remove_token_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: RemoveTokenAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: RemoveTokenKeys = accounts.into();
    let ix = remove_token_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn remove_token_invoke_signed(
    accounts: RemoveTokenAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    remove_token_invoke_signed_with_program_id(STABLESWAP_PROGRAM_ID, accounts, seeds)
}
pub fn remove_token_verify_account_keys(
    accounts: RemoveTokenAccounts<'_, '_>,
    keys: RemoveTokenKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.admin.key, keys.admin),
        (*accounts.exchange_state.key, keys.exchange_state),
        (*accounts.mint.key, keys.mint),
        (*accounts.token_state.key, keys.token_state),
        (*accounts.vault.key, keys.vault),
        (*accounts.token_program.key, keys.token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn remove_token_verify_writable_privileges<'me, 'info>(
    accounts: RemoveTokenAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.admin, accounts.token_state, accounts.vault] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn remove_token_verify_signer_privileges<'me, 'info>(
    accounts: RemoveTokenAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn remove_token_verify_account_privileges<'me, 'info>(
    accounts: RemoveTokenAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    remove_token_verify_writable_privileges(accounts)?;
    remove_token_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SWAP_IX_ACCOUNTS_LEN: usize = 16;
#[derive(Copy, Clone, Debug)]
pub struct SwapAccounts<'me, 'info> {
    pub user: &'me AccountInfo<'info>,
    pub exchange_state: &'me AccountInfo<'info>,
    pub mint_in: &'me AccountInfo<'info>,
    pub mint_out: &'me AccountInfo<'info>,
    pub token_state_in: &'me AccountInfo<'info>,
    pub token_state_out: &'me AccountInfo<'info>,
    pub token_vault_in: &'me AccountInfo<'info>,
    pub token_vault_out: &'me AccountInfo<'info>,
    pub user_token_account_in: &'me AccountInfo<'info>,
    pub recipient_token_account_out: &'me AccountInfo<'info>,
    pub pair_state: &'me AccountInfo<'info>,
    pub instructions: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
    pub token_program_in: &'me AccountInfo<'info>,
    pub token_program_out: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SwapKeys {
    pub user: Pubkey,
    pub exchange_state: Pubkey,
    pub mint_in: Pubkey,
    pub mint_out: Pubkey,
    pub token_state_in: Pubkey,
    pub token_state_out: Pubkey,
    pub token_vault_in: Pubkey,
    pub token_vault_out: Pubkey,
    pub user_token_account_in: Pubkey,
    pub recipient_token_account_out: Pubkey,
    pub pair_state: Pubkey,
    pub instructions: Pubkey,
    pub system_program: Pubkey,
    pub associated_token_program: Pubkey,
    pub token_program_in: Pubkey,
    pub token_program_out: Pubkey,
}
impl From<SwapAccounts<'_, '_>> for SwapKeys {
    fn from(accounts: SwapAccounts) -> Self {
        Self {
            user: *accounts.user.key,
            exchange_state: *accounts.exchange_state.key,
            mint_in: *accounts.mint_in.key,
            mint_out: *accounts.mint_out.key,
            token_state_in: *accounts.token_state_in.key,
            token_state_out: *accounts.token_state_out.key,
            token_vault_in: *accounts.token_vault_in.key,
            token_vault_out: *accounts.token_vault_out.key,
            user_token_account_in: *accounts.user_token_account_in.key,
            recipient_token_account_out: *accounts.recipient_token_account_out.key,
            pair_state: *accounts.pair_state.key,
            instructions: *accounts.instructions.key,
            system_program: *accounts.system_program.key,
            associated_token_program: *accounts.associated_token_program.key,
            token_program_in: *accounts.token_program_in.key,
            token_program_out: *accounts.token_program_out.key,
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
                pubkey: keys.exchange_state,
                is_signer: false,
                is_writable: true,
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
                pubkey: keys.token_state_in,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_state_out,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_vault_in,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_vault_out,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_token_account_in,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.recipient_token_account_out,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pair_state,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.instructions,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.system_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.associated_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_program_in,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_program_out,
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
            exchange_state: pubkeys[1],
            mint_in: pubkeys[2],
            mint_out: pubkeys[3],
            token_state_in: pubkeys[4],
            token_state_out: pubkeys[5],
            token_vault_in: pubkeys[6],
            token_vault_out: pubkeys[7],
            user_token_account_in: pubkeys[8],
            recipient_token_account_out: pubkeys[9],
            pair_state: pubkeys[10],
            instructions: pubkeys[11],
            system_program: pubkeys[12],
            associated_token_program: pubkeys[13],
            token_program_in: pubkeys[14],
            token_program_out: pubkeys[15],
        }
    }
}
impl<'info> From<SwapAccounts<'_, 'info>>
for [AccountInfo<'info>; SWAP_IX_ACCOUNTS_LEN] {
    fn from(accounts: SwapAccounts<'_, 'info>) -> Self {
        [
            accounts.user.clone(),
            accounts.exchange_state.clone(),
            accounts.mint_in.clone(),
            accounts.mint_out.clone(),
            accounts.token_state_in.clone(),
            accounts.token_state_out.clone(),
            accounts.token_vault_in.clone(),
            accounts.token_vault_out.clone(),
            accounts.user_token_account_in.clone(),
            accounts.recipient_token_account_out.clone(),
            accounts.pair_state.clone(),
            accounts.instructions.clone(),
            accounts.system_program.clone(),
            accounts.associated_token_program.clone(),
            accounts.token_program_in.clone(),
            accounts.token_program_out.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SWAP_IX_ACCOUNTS_LEN]>
for SwapAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; SWAP_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            user: &arr[0],
            exchange_state: &arr[1],
            mint_in: &arr[2],
            mint_out: &arr[3],
            token_state_in: &arr[4],
            token_state_out: &arr[5],
            token_vault_in: &arr[6],
            token_vault_out: &arr[7],
            user_token_account_in: &arr[8],
            recipient_token_account_out: &arr[9],
            pair_state: &arr[10],
            instructions: &arr[11],
            system_program: &arr[12],
            associated_token_program: &arr[13],
            token_program_in: &arr[14],
            token_program_out: &arr[15],
        }
    }
}
pub const SWAP_IX_DISCM: [u8; 8usize] = [248, 198, 158, 145, 225, 117, 135, 200];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SwapIxArgs {
    pub amount_in: u64,
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
        let amount_in: u64 = crate::borsh_de_or_default(&mut reader)?;
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
    swap_ix_with_program_id(STABLESWAP_PROGRAM_ID, keys, args)
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
    swap_invoke_with_program_id(STABLESWAP_PROGRAM_ID, accounts, args)
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
    swap_invoke_signed_with_program_id(STABLESWAP_PROGRAM_ID, accounts, args, seeds)
}
pub fn swap_verify_account_keys(
    accounts: SwapAccounts<'_, '_>,
    keys: SwapKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.user.key, keys.user),
        (*accounts.exchange_state.key, keys.exchange_state),
        (*accounts.mint_in.key, keys.mint_in),
        (*accounts.mint_out.key, keys.mint_out),
        (*accounts.token_state_in.key, keys.token_state_in),
        (*accounts.token_state_out.key, keys.token_state_out),
        (*accounts.token_vault_in.key, keys.token_vault_in),
        (*accounts.token_vault_out.key, keys.token_vault_out),
        (*accounts.user_token_account_in.key, keys.user_token_account_in),
        (*accounts.recipient_token_account_out.key, keys.recipient_token_account_out),
        (*accounts.pair_state.key, keys.pair_state),
        (*accounts.instructions.key, keys.instructions),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.associated_token_program.key, keys.associated_token_program),
        (*accounts.token_program_in.key, keys.token_program_in),
        (*accounts.token_program_out.key, keys.token_program_out),
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
        accounts.exchange_state,
        accounts.token_vault_in,
        accounts.token_vault_out,
        accounts.user_token_account_in,
        accounts.recipient_token_account_out,
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
pub const SWEEP_FEES_IX_ACCOUNTS_LEN: usize = 7;
#[derive(Copy, Clone, Debug)]
pub struct SweepFeesAccounts<'me, 'info> {
    pub admin: &'me AccountInfo<'info>,
    pub exchange_state: &'me AccountInfo<'info>,
    pub mint: &'me AccountInfo<'info>,
    pub token_state: &'me AccountInfo<'info>,
    pub token_vault: &'me AccountInfo<'info>,
    pub recipient_token_account: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SweepFeesKeys {
    pub admin: Pubkey,
    pub exchange_state: Pubkey,
    pub mint: Pubkey,
    pub token_state: Pubkey,
    pub token_vault: Pubkey,
    pub recipient_token_account: Pubkey,
    pub token_program: Pubkey,
}
impl From<SweepFeesAccounts<'_, '_>> for SweepFeesKeys {
    fn from(accounts: SweepFeesAccounts) -> Self {
        Self {
            admin: *accounts.admin.key,
            exchange_state: *accounts.exchange_state.key,
            mint: *accounts.mint.key,
            token_state: *accounts.token_state.key,
            token_vault: *accounts.token_vault.key,
            recipient_token_account: *accounts.recipient_token_account.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<SweepFeesKeys> for [AccountMeta; SWEEP_FEES_IX_ACCOUNTS_LEN] {
    fn from(keys: SweepFeesKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.admin,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.exchange_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_state,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.recipient_token_account,
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
impl From<[Pubkey; SWEEP_FEES_IX_ACCOUNTS_LEN]> for SweepFeesKeys {
    fn from(pubkeys: [Pubkey; SWEEP_FEES_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin: pubkeys[0],
            exchange_state: pubkeys[1],
            mint: pubkeys[2],
            token_state: pubkeys[3],
            token_vault: pubkeys[4],
            recipient_token_account: pubkeys[5],
            token_program: pubkeys[6],
        }
    }
}
impl<'info> From<SweepFeesAccounts<'_, 'info>>
for [AccountInfo<'info>; SWEEP_FEES_IX_ACCOUNTS_LEN] {
    fn from(accounts: SweepFeesAccounts<'_, 'info>) -> Self {
        [
            accounts.admin.clone(),
            accounts.exchange_state.clone(),
            accounts.mint.clone(),
            accounts.token_state.clone(),
            accounts.token_vault.clone(),
            accounts.recipient_token_account.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SWEEP_FEES_IX_ACCOUNTS_LEN]>
for SweepFeesAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; SWEEP_FEES_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin: &arr[0],
            exchange_state: &arr[1],
            mint: &arr[2],
            token_state: &arr[3],
            token_vault: &arr[4],
            recipient_token_account: &arr[5],
            token_program: &arr[6],
        }
    }
}
pub const SWEEP_FEES_IX_DISCM: [u8; 8usize] = [175, 225, 98, 71, 118, 66, 34, 148];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SweepFeesIxArgs {
    pub amount: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct SweepFeesIxData(pub SweepFeesIxArgs);
impl From<SweepFeesIxArgs> for SweepFeesIxData {
    fn from(args: SweepFeesIxArgs) -> Self {
        Self(args)
    }
}
impl SweepFeesIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SWEEP_FEES_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(Self(SweepFeesIxArgs { amount }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SWEEP_FEES_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.amount, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn sweep_fees_ix_with_program_id(
    program_id: Pubkey,
    keys: SweepFeesKeys,
    args: SweepFeesIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SWEEP_FEES_IX_ACCOUNTS_LEN] = keys.into();
    let data: SweepFeesIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn sweep_fees_ix(
    keys: SweepFeesKeys,
    args: SweepFeesIxArgs,
) -> std::io::Result<Instruction> {
    sweep_fees_ix_with_program_id(STABLESWAP_PROGRAM_ID, keys, args)
}
pub fn sweep_fees_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SweepFeesAccounts<'_, '_>,
    args: SweepFeesIxArgs,
) -> ProgramResult {
    let keys: SweepFeesKeys = accounts.into();
    let ix = sweep_fees_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn sweep_fees_invoke(
    accounts: SweepFeesAccounts<'_, '_>,
    args: SweepFeesIxArgs,
) -> ProgramResult {
    sweep_fees_invoke_with_program_id(STABLESWAP_PROGRAM_ID, accounts, args)
}
pub fn sweep_fees_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SweepFeesAccounts<'_, '_>,
    args: SweepFeesIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SweepFeesKeys = accounts.into();
    let ix = sweep_fees_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn sweep_fees_invoke_signed(
    accounts: SweepFeesAccounts<'_, '_>,
    args: SweepFeesIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    sweep_fees_invoke_signed_with_program_id(
        STABLESWAP_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn sweep_fees_verify_account_keys(
    accounts: SweepFeesAccounts<'_, '_>,
    keys: SweepFeesKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.admin.key, keys.admin),
        (*accounts.exchange_state.key, keys.exchange_state),
        (*accounts.mint.key, keys.mint),
        (*accounts.token_state.key, keys.token_state),
        (*accounts.token_vault.key, keys.token_vault),
        (*accounts.recipient_token_account.key, keys.recipient_token_account),
        (*accounts.token_program.key, keys.token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn sweep_fees_verify_writable_privileges<'me, 'info>(
    accounts: SweepFeesAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.admin,
        accounts.exchange_state,
        accounts.token_vault,
        accounts.recipient_token_account,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn sweep_fees_verify_signer_privileges<'me, 'info>(
    accounts: SweepFeesAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn sweep_fees_verify_account_privileges<'me, 'info>(
    accounts: SweepFeesAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    sweep_fees_verify_writable_privileges(accounts)?;
    sweep_fees_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const UNFREEZE_IX_ACCOUNTS_LEN: usize = 2;
#[derive(Copy, Clone, Debug)]
pub struct UnfreezeAccounts<'me, 'info> {
    pub admin: &'me AccountInfo<'info>,
    pub exchange_state: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UnfreezeKeys {
    pub admin: Pubkey,
    pub exchange_state: Pubkey,
}
impl From<UnfreezeAccounts<'_, '_>> for UnfreezeKeys {
    fn from(accounts: UnfreezeAccounts) -> Self {
        Self {
            admin: *accounts.admin.key,
            exchange_state: *accounts.exchange_state.key,
        }
    }
}
impl From<UnfreezeKeys> for [AccountMeta; UNFREEZE_IX_ACCOUNTS_LEN] {
    fn from(keys: UnfreezeKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.admin,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.exchange_state,
                is_signer: false,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; UNFREEZE_IX_ACCOUNTS_LEN]> for UnfreezeKeys {
    fn from(pubkeys: [Pubkey; UNFREEZE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin: pubkeys[0],
            exchange_state: pubkeys[1],
        }
    }
}
impl<'info> From<UnfreezeAccounts<'_, 'info>>
for [AccountInfo<'info>; UNFREEZE_IX_ACCOUNTS_LEN] {
    fn from(accounts: UnfreezeAccounts<'_, 'info>) -> Self {
        [accounts.admin.clone(), accounts.exchange_state.clone()]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; UNFREEZE_IX_ACCOUNTS_LEN]>
for UnfreezeAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; UNFREEZE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin: &arr[0],
            exchange_state: &arr[1],
        }
    }
}
pub const UNFREEZE_IX_DISCM: [u8; 8usize] = [133, 160, 68, 253, 80, 232, 218, 247];
#[derive(Clone, Debug, PartialEq)]
pub struct UnfreezeIxData;
impl UnfreezeIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UNFREEZE_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UNFREEZE_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn unfreeze_ix_with_program_id(
    program_id: Pubkey,
    keys: UnfreezeKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; UNFREEZE_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: UnfreezeIxData.try_to_vec()?,
    })
}
pub fn unfreeze_ix(keys: UnfreezeKeys) -> std::io::Result<Instruction> {
    unfreeze_ix_with_program_id(STABLESWAP_PROGRAM_ID, keys)
}
pub fn unfreeze_invoke_with_program_id(
    program_id: Pubkey,
    accounts: UnfreezeAccounts<'_, '_>,
) -> ProgramResult {
    let keys: UnfreezeKeys = accounts.into();
    let ix = unfreeze_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn unfreeze_invoke(accounts: UnfreezeAccounts<'_, '_>) -> ProgramResult {
    unfreeze_invoke_with_program_id(STABLESWAP_PROGRAM_ID, accounts)
}
pub fn unfreeze_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: UnfreezeAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: UnfreezeKeys = accounts.into();
    let ix = unfreeze_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn unfreeze_invoke_signed(
    accounts: UnfreezeAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    unfreeze_invoke_signed_with_program_id(STABLESWAP_PROGRAM_ID, accounts, seeds)
}
pub fn unfreeze_verify_account_keys(
    accounts: UnfreezeAccounts<'_, '_>,
    keys: UnfreezeKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.admin.key, keys.admin),
        (*accounts.exchange_state.key, keys.exchange_state),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn unfreeze_verify_writable_privileges<'me, 'info>(
    accounts: UnfreezeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.admin, accounts.exchange_state] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn unfreeze_verify_signer_privileges<'me, 'info>(
    accounts: UnfreezeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn unfreeze_verify_account_privileges<'me, 'info>(
    accounts: UnfreezeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    unfreeze_verify_writable_privileges(accounts)?;
    unfreeze_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const UPDATE_PAIR_IX_ACCOUNTS_LEN: usize = 5;
#[derive(Copy, Clone, Debug)]
pub struct UpdatePairAccounts<'me, 'info> {
    pub admin: &'me AccountInfo<'info>,
    pub exchange_state: &'me AccountInfo<'info>,
    pub mint_x: &'me AccountInfo<'info>,
    pub mint_y: &'me AccountInfo<'info>,
    pub pair_state: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UpdatePairKeys {
    pub admin: Pubkey,
    pub exchange_state: Pubkey,
    pub mint_x: Pubkey,
    pub mint_y: Pubkey,
    pub pair_state: Pubkey,
}
impl From<UpdatePairAccounts<'_, '_>> for UpdatePairKeys {
    fn from(accounts: UpdatePairAccounts) -> Self {
        Self {
            admin: *accounts.admin.key,
            exchange_state: *accounts.exchange_state.key,
            mint_x: *accounts.mint_x.key,
            mint_y: *accounts.mint_y.key,
            pair_state: *accounts.pair_state.key,
        }
    }
}
impl From<UpdatePairKeys> for [AccountMeta; UPDATE_PAIR_IX_ACCOUNTS_LEN] {
    fn from(keys: UpdatePairKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.admin,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.exchange_state,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.mint_x,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.mint_y,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pair_state,
                is_signer: false,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; UPDATE_PAIR_IX_ACCOUNTS_LEN]> for UpdatePairKeys {
    fn from(pubkeys: [Pubkey; UPDATE_PAIR_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin: pubkeys[0],
            exchange_state: pubkeys[1],
            mint_x: pubkeys[2],
            mint_y: pubkeys[3],
            pair_state: pubkeys[4],
        }
    }
}
impl<'info> From<UpdatePairAccounts<'_, 'info>>
for [AccountInfo<'info>; UPDATE_PAIR_IX_ACCOUNTS_LEN] {
    fn from(accounts: UpdatePairAccounts<'_, 'info>) -> Self {
        [
            accounts.admin.clone(),
            accounts.exchange_state.clone(),
            accounts.mint_x.clone(),
            accounts.mint_y.clone(),
            accounts.pair_state.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; UPDATE_PAIR_IX_ACCOUNTS_LEN]>
for UpdatePairAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; UPDATE_PAIR_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin: &arr[0],
            exchange_state: &arr[1],
            mint_x: &arr[2],
            mint_y: &arr[3],
            pair_state: &arr[4],
        }
    }
}
pub const UPDATE_PAIR_IX_DISCM: [u8; 8usize] = [176, 62, 36, 215, 255, 206, 35, 12];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UpdatePairIxArgs {
    pub x_to_y_fee_bps: i16,
    pub y_to_x_fee_bps: i16,
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdatePairIxData(pub UpdatePairIxArgs);
impl From<UpdatePairIxArgs> for UpdatePairIxData {
    fn from(args: UpdatePairIxArgs) -> Self {
        Self(args)
    }
}
impl UpdatePairIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_PAIR_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let x_to_y_fee_bps: i16 = crate::borsh_de_or_default(&mut reader)?;
        let y_to_x_fee_bps: i16 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(UpdatePairIxArgs {
                x_to_y_fee_bps,
                y_to_x_fee_bps,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_PAIR_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.x_to_y_fee_bps, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.y_to_x_fee_bps, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn update_pair_ix_with_program_id(
    program_id: Pubkey,
    keys: UpdatePairKeys,
    args: UpdatePairIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; UPDATE_PAIR_IX_ACCOUNTS_LEN] = keys.into();
    let data: UpdatePairIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_pair_ix(
    keys: UpdatePairKeys,
    args: UpdatePairIxArgs,
) -> std::io::Result<Instruction> {
    update_pair_ix_with_program_id(STABLESWAP_PROGRAM_ID, keys, args)
}
pub fn update_pair_invoke_with_program_id(
    program_id: Pubkey,
    accounts: UpdatePairAccounts<'_, '_>,
    args: UpdatePairIxArgs,
) -> ProgramResult {
    let keys: UpdatePairKeys = accounts.into();
    let ix = update_pair_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn update_pair_invoke(
    accounts: UpdatePairAccounts<'_, '_>,
    args: UpdatePairIxArgs,
) -> ProgramResult {
    update_pair_invoke_with_program_id(STABLESWAP_PROGRAM_ID, accounts, args)
}
pub fn update_pair_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: UpdatePairAccounts<'_, '_>,
    args: UpdatePairIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: UpdatePairKeys = accounts.into();
    let ix = update_pair_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn update_pair_invoke_signed(
    accounts: UpdatePairAccounts<'_, '_>,
    args: UpdatePairIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    update_pair_invoke_signed_with_program_id(
        STABLESWAP_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn update_pair_verify_account_keys(
    accounts: UpdatePairAccounts<'_, '_>,
    keys: UpdatePairKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.admin.key, keys.admin),
        (*accounts.exchange_state.key, keys.exchange_state),
        (*accounts.mint_x.key, keys.mint_x),
        (*accounts.mint_y.key, keys.mint_y),
        (*accounts.pair_state.key, keys.pair_state),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn update_pair_verify_writable_privileges<'me, 'info>(
    accounts: UpdatePairAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.admin, accounts.pair_state] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn update_pair_verify_signer_privileges<'me, 'info>(
    accounts: UpdatePairAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn update_pair_verify_account_privileges<'me, 'info>(
    accounts: UpdatePairAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    update_pair_verify_writable_privileges(accounts)?;
    update_pair_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const UPDATE_ROUTER_PUBKEY_IX_ACCOUNTS_LEN: usize = 2;
#[derive(Copy, Clone, Debug)]
pub struct UpdateRouterPubkeyAccounts<'me, 'info> {
    pub admin: &'me AccountInfo<'info>,
    pub exchange_state: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UpdateRouterPubkeyKeys {
    pub admin: Pubkey,
    pub exchange_state: Pubkey,
}
impl From<UpdateRouterPubkeyAccounts<'_, '_>> for UpdateRouterPubkeyKeys {
    fn from(accounts: UpdateRouterPubkeyAccounts) -> Self {
        Self {
            admin: *accounts.admin.key,
            exchange_state: *accounts.exchange_state.key,
        }
    }
}
impl From<UpdateRouterPubkeyKeys>
for [AccountMeta; UPDATE_ROUTER_PUBKEY_IX_ACCOUNTS_LEN] {
    fn from(keys: UpdateRouterPubkeyKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.admin,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.exchange_state,
                is_signer: false,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; UPDATE_ROUTER_PUBKEY_IX_ACCOUNTS_LEN]> for UpdateRouterPubkeyKeys {
    fn from(pubkeys: [Pubkey; UPDATE_ROUTER_PUBKEY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin: pubkeys[0],
            exchange_state: pubkeys[1],
        }
    }
}
impl<'info> From<UpdateRouterPubkeyAccounts<'_, 'info>>
for [AccountInfo<'info>; UPDATE_ROUTER_PUBKEY_IX_ACCOUNTS_LEN] {
    fn from(accounts: UpdateRouterPubkeyAccounts<'_, 'info>) -> Self {
        [accounts.admin.clone(), accounts.exchange_state.clone()]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; UPDATE_ROUTER_PUBKEY_IX_ACCOUNTS_LEN]>
for UpdateRouterPubkeyAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; UPDATE_ROUTER_PUBKEY_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            admin: &arr[0],
            exchange_state: &arr[1],
        }
    }
}
pub const UPDATE_ROUTER_PUBKEY_IX_DISCM: [u8; 8usize] = [
    192, 247, 142, 136, 199, 124, 202, 30,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UpdateRouterPubkeyIxArgs {
    pub router_pubkey: Pubkey,
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateRouterPubkeyIxData(pub UpdateRouterPubkeyIxArgs);
impl From<UpdateRouterPubkeyIxArgs> for UpdateRouterPubkeyIxData {
    fn from(args: UpdateRouterPubkeyIxArgs) -> Self {
        Self(args)
    }
}
impl UpdateRouterPubkeyIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_ROUTER_PUBKEY_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let router_pubkey: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(UpdateRouterPubkeyIxArgs {
                router_pubkey,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_ROUTER_PUBKEY_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.router_pubkey, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn update_router_pubkey_ix_with_program_id(
    program_id: Pubkey,
    keys: UpdateRouterPubkeyKeys,
    args: UpdateRouterPubkeyIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; UPDATE_ROUTER_PUBKEY_IX_ACCOUNTS_LEN] = keys.into();
    let data: UpdateRouterPubkeyIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_router_pubkey_ix(
    keys: UpdateRouterPubkeyKeys,
    args: UpdateRouterPubkeyIxArgs,
) -> std::io::Result<Instruction> {
    update_router_pubkey_ix_with_program_id(STABLESWAP_PROGRAM_ID, keys, args)
}
pub fn update_router_pubkey_invoke_with_program_id(
    program_id: Pubkey,
    accounts: UpdateRouterPubkeyAccounts<'_, '_>,
    args: UpdateRouterPubkeyIxArgs,
) -> ProgramResult {
    let keys: UpdateRouterPubkeyKeys = accounts.into();
    let ix = update_router_pubkey_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn update_router_pubkey_invoke(
    accounts: UpdateRouterPubkeyAccounts<'_, '_>,
    args: UpdateRouterPubkeyIxArgs,
) -> ProgramResult {
    update_router_pubkey_invoke_with_program_id(STABLESWAP_PROGRAM_ID, accounts, args)
}
pub fn update_router_pubkey_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: UpdateRouterPubkeyAccounts<'_, '_>,
    args: UpdateRouterPubkeyIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: UpdateRouterPubkeyKeys = accounts.into();
    let ix = update_router_pubkey_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn update_router_pubkey_invoke_signed(
    accounts: UpdateRouterPubkeyAccounts<'_, '_>,
    args: UpdateRouterPubkeyIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    update_router_pubkey_invoke_signed_with_program_id(
        STABLESWAP_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn update_router_pubkey_verify_account_keys(
    accounts: UpdateRouterPubkeyAccounts<'_, '_>,
    keys: UpdateRouterPubkeyKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.admin.key, keys.admin),
        (*accounts.exchange_state.key, keys.exchange_state),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn update_router_pubkey_verify_writable_privileges<'me, 'info>(
    accounts: UpdateRouterPubkeyAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.admin, accounts.exchange_state] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn update_router_pubkey_verify_signer_privileges<'me, 'info>(
    accounts: UpdateRouterPubkeyAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn update_router_pubkey_verify_account_privileges<'me, 'info>(
    accounts: UpdateRouterPubkeyAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    update_router_pubkey_verify_writable_privileges(accounts)?;
    update_router_pubkey_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const WITHDRAW_IX_ACCOUNTS_LEN: usize = 10;
#[derive(Copy, Clone, Debug)]
pub struct WithdrawAccounts<'me, 'info> {
    pub withdrawer: &'me AccountInfo<'info>,
    pub lp_position: &'me AccountInfo<'info>,
    pub exchange_state: &'me AccountInfo<'info>,
    pub mint: &'me AccountInfo<'info>,
    pub token_state: &'me AccountInfo<'info>,
    pub token_vault: &'me AccountInfo<'info>,
    pub withdrawer_token_account: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct WithdrawKeys {
    pub withdrawer: Pubkey,
    pub lp_position: Pubkey,
    pub exchange_state: Pubkey,
    pub mint: Pubkey,
    pub token_state: Pubkey,
    pub token_vault: Pubkey,
    pub withdrawer_token_account: Pubkey,
    pub system_program: Pubkey,
    pub token_program: Pubkey,
    pub associated_token_program: Pubkey,
}
impl From<WithdrawAccounts<'_, '_>> for WithdrawKeys {
    fn from(accounts: WithdrawAccounts) -> Self {
        Self {
            withdrawer: *accounts.withdrawer.key,
            lp_position: *accounts.lp_position.key,
            exchange_state: *accounts.exchange_state.key,
            mint: *accounts.mint.key,
            token_state: *accounts.token_state.key,
            token_vault: *accounts.token_vault.key,
            withdrawer_token_account: *accounts.withdrawer_token_account.key,
            system_program: *accounts.system_program.key,
            token_program: *accounts.token_program.key,
            associated_token_program: *accounts.associated_token_program.key,
        }
    }
}
impl From<WithdrawKeys> for [AccountMeta; WITHDRAW_IX_ACCOUNTS_LEN] {
    fn from(keys: WithdrawKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.withdrawer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.lp_position,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.exchange_state,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_state,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.withdrawer_token_account,
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
            AccountMeta {
                pubkey: keys.associated_token_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; WITHDRAW_IX_ACCOUNTS_LEN]> for WithdrawKeys {
    fn from(pubkeys: [Pubkey; WITHDRAW_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            withdrawer: pubkeys[0],
            lp_position: pubkeys[1],
            exchange_state: pubkeys[2],
            mint: pubkeys[3],
            token_state: pubkeys[4],
            token_vault: pubkeys[5],
            withdrawer_token_account: pubkeys[6],
            system_program: pubkeys[7],
            token_program: pubkeys[8],
            associated_token_program: pubkeys[9],
        }
    }
}
impl<'info> From<WithdrawAccounts<'_, 'info>>
for [AccountInfo<'info>; WITHDRAW_IX_ACCOUNTS_LEN] {
    fn from(accounts: WithdrawAccounts<'_, 'info>) -> Self {
        [
            accounts.withdrawer.clone(),
            accounts.lp_position.clone(),
            accounts.exchange_state.clone(),
            accounts.mint.clone(),
            accounts.token_state.clone(),
            accounts.token_vault.clone(),
            accounts.withdrawer_token_account.clone(),
            accounts.system_program.clone(),
            accounts.token_program.clone(),
            accounts.associated_token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; WITHDRAW_IX_ACCOUNTS_LEN]>
for WithdrawAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; WITHDRAW_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            withdrawer: &arr[0],
            lp_position: &arr[1],
            exchange_state: &arr[2],
            mint: &arr[3],
            token_state: &arr[4],
            token_vault: &arr[5],
            withdrawer_token_account: &arr[6],
            system_program: &arr[7],
            token_program: &arr[8],
            associated_token_program: &arr[9],
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
    withdraw_ix_with_program_id(STABLESWAP_PROGRAM_ID, keys, args)
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
    withdraw_invoke_with_program_id(STABLESWAP_PROGRAM_ID, accounts, args)
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
    withdraw_invoke_signed_with_program_id(STABLESWAP_PROGRAM_ID, accounts, args, seeds)
}
pub fn withdraw_verify_account_keys(
    accounts: WithdrawAccounts<'_, '_>,
    keys: WithdrawKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.withdrawer.key, keys.withdrawer),
        (*accounts.lp_position.key, keys.lp_position),
        (*accounts.exchange_state.key, keys.exchange_state),
        (*accounts.mint.key, keys.mint),
        (*accounts.token_state.key, keys.token_state),
        (*accounts.token_vault.key, keys.token_vault),
        (*accounts.withdrawer_token_account.key, keys.withdrawer_token_account),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.associated_token_program.key, keys.associated_token_program),
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
        accounts.withdrawer,
        accounts.lp_position,
        accounts.token_vault,
        accounts.withdrawer_token_account,
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
    for should_be_signer in [accounts.withdrawer] {
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
