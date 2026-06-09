use solana_pubkey::Pubkey;
use solana_cpi::{invoke, invoke_signed};
use solana_instruction::{AccountMeta, Instruction};
use solana_account_info::AccountInfo;
use solana_program_error::ProgramError;
use std::io::Read;
#[allow(unused_imports)]
use crate::*;
#[derive(Clone, Debug, PartialEq)]
pub enum ZapProgramIx {
    CloseLedgerAccount,
    InitializeLedgerAccount,
    SetLedgerBalance(SetLedgerBalanceIxArgs),
    UpdateLedgerBalanceAfterSwap(UpdateLedgerBalanceAfterSwapIxArgs),
    ZapInDammV2(ZapInDammV2IxArgs),
    ZapInDlmmForInitializedPosition(ZapInDlmmForInitializedPositionIxArgs),
    ZapInDlmmForUninitializedPosition(ZapInDlmmForUninitializedPositionIxArgs),
    ZapOut(ZapOutIxArgs),
}
impl ZapProgramIx {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        if buf.starts_with(&CLOSE_LEDGER_ACCOUNT_IX_DISCM) {
            return Ok(Self::CloseLedgerAccount);
        }
        if buf.starts_with(&INITIALIZE_LEDGER_ACCOUNT_IX_DISCM) {
            return Ok(Self::InitializeLedgerAccount);
        }
        if buf.starts_with(&SET_LEDGER_BALANCE_IX_DISCM) {
            let mut reader = &buf[SET_LEDGER_BALANCE_IX_DISCM.len()..];
            let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            let is_token_a: bool = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::SetLedgerBalance(SetLedgerBalanceIxArgs {
                    amount,
                    is_token_a,
                }),
            );
        }
        if buf.starts_with(&UPDATE_LEDGER_BALANCE_AFTER_SWAP_IX_DISCM) {
            let mut reader = &buf[UPDATE_LEDGER_BALANCE_AFTER_SWAP_IX_DISCM.len()..];
            let pre_source_token_balance: u64 = crate::borsh_de_or_default(&mut reader)?;
            let max_transfer_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            let is_token_a: bool = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::UpdateLedgerBalanceAfterSwap(UpdateLedgerBalanceAfterSwapIxArgs {
                    pre_source_token_balance,
                    max_transfer_amount,
                    is_token_a,
                }),
            );
        }
        if buf.starts_with(&ZAP_IN_DAMM_V2_IX_DISCM) {
            let mut reader = &buf[ZAP_IN_DAMM_V2_IX_DISCM.len()..];
            let pre_sqrt_price: u128 = crate::borsh_de_or_default(&mut reader)?;
            let max_sqrt_price_change_bps: u32 = crate::borsh_de_or_default(
                &mut reader,
            )?;
            return Ok(
                Self::ZapInDammV2(ZapInDammV2IxArgs {
                    pre_sqrt_price,
                    max_sqrt_price_change_bps,
                }),
            );
        }
        if buf.starts_with(&ZAP_IN_DLMM_FOR_INITIALIZED_POSITION_IX_DISCM) {
            let mut reader = &buf[ZAP_IN_DLMM_FOR_INITIALIZED_POSITION_IX_DISCM.len()..];
            let active_id: i32 = crate::borsh_de_or_default(&mut reader)?;
            let min_delta_id: i32 = crate::borsh_de_or_default(&mut reader)?;
            let max_delta_id: i32 = crate::borsh_de_or_default(&mut reader)?;
            let max_active_bin_slippage: u16 = crate::borsh_de_or_default(&mut reader)?;
            let favor_x_in_active_id: bool = crate::borsh_de_or_default(&mut reader)?;
            let strategy: StrategyType = crate::borsh_de_or_default(&mut reader)?;
            let remaining_accounts_info = if reader.is_empty() {
                Default::default()
            } else {
                <RemainingAccountsInfo>::deserialize(&mut reader)?
            };
            return Ok(
                Self::ZapInDlmmForInitializedPosition(ZapInDlmmForInitializedPositionIxArgs {
                    active_id,
                    min_delta_id,
                    max_delta_id,
                    max_active_bin_slippage,
                    favor_x_in_active_id,
                    strategy,
                    remaining_accounts_info,
                }),
            );
        }
        if buf.starts_with(&ZAP_IN_DLMM_FOR_UNINITIALIZED_POSITION_IX_DISCM) {
            let mut reader = &buf[ZAP_IN_DLMM_FOR_UNINITIALIZED_POSITION_IX_DISCM
                .len()..];
            let min_delta_id: i32 = crate::borsh_de_or_default(&mut reader)?;
            let max_delta_id: i32 = crate::borsh_de_or_default(&mut reader)?;
            let active_id: i32 = crate::borsh_de_or_default(&mut reader)?;
            let max_active_bin_slippage: u16 = crate::borsh_de_or_default(&mut reader)?;
            let favor_x_in_active_id: bool = crate::borsh_de_or_default(&mut reader)?;
            let strategy: StrategyType = crate::borsh_de_or_default(&mut reader)?;
            let remaining_accounts_info = if reader.is_empty() {
                Default::default()
            } else {
                <RemainingAccountsInfo>::deserialize(&mut reader)?
            };
            return Ok(
                Self::ZapInDlmmForUninitializedPosition(ZapInDlmmForUninitializedPositionIxArgs {
                    min_delta_id,
                    max_delta_id,
                    active_id,
                    max_active_bin_slippage,
                    favor_x_in_active_id,
                    strategy,
                    remaining_accounts_info,
                }),
            );
        }
        if buf.starts_with(&ZAP_OUT_IX_DISCM) {
            let mut reader = &buf[ZAP_OUT_IX_DISCM.len()..];
            let params = if reader.is_empty() {
                Default::default()
            } else {
                <ZapOutParameters>::deserialize(&mut reader)?
            };
            return Ok(Self::ZapOut(ZapOutIxArgs { params }));
        }
        Err(std::io::Error::from(std::io::ErrorKind::InvalidData))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        match self {
            Self::CloseLedgerAccount => writer.write_all(&CLOSE_LEDGER_ACCOUNT_IX_DISCM),
            Self::InitializeLedgerAccount => {
                writer.write_all(&INITIALIZE_LEDGER_ACCOUNT_IX_DISCM)
            }
            Self::SetLedgerBalance(args) => {
                writer.write_all(&SET_LEDGER_BALANCE_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.amount, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.is_token_a, &mut writer)?;
                Ok(())
            }
            Self::UpdateLedgerBalanceAfterSwap(args) => {
                writer.write_all(&UPDATE_LEDGER_BALANCE_AFTER_SWAP_IX_DISCM)?;
                borsh::BorshSerialize::serialize(
                    &args.pre_source_token_balance,
                    &mut writer,
                )?;
                borsh::BorshSerialize::serialize(
                    &args.max_transfer_amount,
                    &mut writer,
                )?;
                borsh::BorshSerialize::serialize(&args.is_token_a, &mut writer)?;
                Ok(())
            }
            Self::ZapInDammV2(args) => {
                writer.write_all(&ZAP_IN_DAMM_V2_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.pre_sqrt_price, &mut writer)?;
                borsh::BorshSerialize::serialize(
                    &args.max_sqrt_price_change_bps,
                    &mut writer,
                )?;
                Ok(())
            }
            Self::ZapInDlmmForInitializedPosition(args) => {
                writer.write_all(&ZAP_IN_DLMM_FOR_INITIALIZED_POSITION_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.active_id, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.min_delta_id, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.max_delta_id, &mut writer)?;
                borsh::BorshSerialize::serialize(
                    &args.max_active_bin_slippage,
                    &mut writer,
                )?;
                borsh::BorshSerialize::serialize(
                    &args.favor_x_in_active_id,
                    &mut writer,
                )?;
                borsh::BorshSerialize::serialize(&args.strategy, &mut writer)?;
                borsh::BorshSerialize::serialize(
                    &args.remaining_accounts_info,
                    &mut writer,
                )?;
                Ok(())
            }
            Self::ZapInDlmmForUninitializedPosition(args) => {
                writer.write_all(&ZAP_IN_DLMM_FOR_UNINITIALIZED_POSITION_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.min_delta_id, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.max_delta_id, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.active_id, &mut writer)?;
                borsh::BorshSerialize::serialize(
                    &args.max_active_bin_slippage,
                    &mut writer,
                )?;
                borsh::BorshSerialize::serialize(
                    &args.favor_x_in_active_id,
                    &mut writer,
                )?;
                borsh::BorshSerialize::serialize(&args.strategy, &mut writer)?;
                borsh::BorshSerialize::serialize(
                    &args.remaining_accounts_info,
                    &mut writer,
                )?;
                Ok(())
            }
            Self::ZapOut(args) => {
                writer.write_all(&ZAP_OUT_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.params, &mut writer)?;
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
pub const CLOSE_LEDGER_ACCOUNT_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct CloseLedgerAccountAccounts<'me, 'info> {
    pub ledger: &'me AccountInfo<'info>,
    pub owner: &'me AccountInfo<'info>,
    pub rent_receiver: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CloseLedgerAccountKeys {
    pub ledger: Pubkey,
    pub owner: Pubkey,
    pub rent_receiver: Pubkey,
}
impl From<CloseLedgerAccountAccounts<'_, '_>> for CloseLedgerAccountKeys {
    fn from(accounts: CloseLedgerAccountAccounts) -> Self {
        Self {
            ledger: *accounts.ledger.key,
            owner: *accounts.owner.key,
            rent_receiver: *accounts.rent_receiver.key,
        }
    }
}
impl From<CloseLedgerAccountKeys>
for [AccountMeta; CLOSE_LEDGER_ACCOUNT_IX_ACCOUNTS_LEN] {
    fn from(keys: CloseLedgerAccountKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.ledger,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.owner,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.rent_receiver,
                is_signer: true,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; CLOSE_LEDGER_ACCOUNT_IX_ACCOUNTS_LEN]> for CloseLedgerAccountKeys {
    fn from(pubkeys: [Pubkey; CLOSE_LEDGER_ACCOUNT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            ledger: pubkeys[0],
            owner: pubkeys[1],
            rent_receiver: pubkeys[2],
        }
    }
}
impl<'info> From<CloseLedgerAccountAccounts<'_, 'info>>
for [AccountInfo<'info>; CLOSE_LEDGER_ACCOUNT_IX_ACCOUNTS_LEN] {
    fn from(accounts: CloseLedgerAccountAccounts<'_, 'info>) -> Self {
        [accounts.ledger.clone(), accounts.owner.clone(), accounts.rent_receiver.clone()]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CLOSE_LEDGER_ACCOUNT_IX_ACCOUNTS_LEN]>
for CloseLedgerAccountAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; CLOSE_LEDGER_ACCOUNT_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            ledger: &arr[0],
            owner: &arr[1],
            rent_receiver: &arr[2],
        }
    }
}
pub const CLOSE_LEDGER_ACCOUNT_IX_DISCM: [u8; 8usize] = [
    189, 122, 172, 13, 122, 54, 54, 51,
];
#[derive(Clone, Debug, PartialEq)]
pub struct CloseLedgerAccountIxData;
impl CloseLedgerAccountIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CLOSE_LEDGER_ACCOUNT_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CLOSE_LEDGER_ACCOUNT_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn close_ledger_account_ix_with_program_id(
    program_id: Pubkey,
    keys: CloseLedgerAccountKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CLOSE_LEDGER_ACCOUNT_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: CloseLedgerAccountIxData.try_to_vec()?,
    })
}
pub fn close_ledger_account_ix(
    keys: CloseLedgerAccountKeys,
) -> std::io::Result<Instruction> {
    close_ledger_account_ix_with_program_id(ZAP_PROGRAM_ID, keys)
}
pub fn close_ledger_account_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CloseLedgerAccountAccounts<'_, '_>,
) -> ProgramResult {
    let keys: CloseLedgerAccountKeys = accounts.into();
    let ix = close_ledger_account_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn close_ledger_account_invoke(
    accounts: CloseLedgerAccountAccounts<'_, '_>,
) -> ProgramResult {
    close_ledger_account_invoke_with_program_id(ZAP_PROGRAM_ID, accounts)
}
pub fn close_ledger_account_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CloseLedgerAccountAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CloseLedgerAccountKeys = accounts.into();
    let ix = close_ledger_account_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn close_ledger_account_invoke_signed(
    accounts: CloseLedgerAccountAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    close_ledger_account_invoke_signed_with_program_id(ZAP_PROGRAM_ID, accounts, seeds)
}
pub fn close_ledger_account_verify_account_keys(
    accounts: CloseLedgerAccountAccounts<'_, '_>,
    keys: CloseLedgerAccountKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.ledger.key, keys.ledger),
        (*accounts.owner.key, keys.owner),
        (*accounts.rent_receiver.key, keys.rent_receiver),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn close_ledger_account_verify_writable_privileges<'me, 'info>(
    accounts: CloseLedgerAccountAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.ledger, accounts.rent_receiver] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn close_ledger_account_verify_signer_privileges<'me, 'info>(
    accounts: CloseLedgerAccountAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.owner, accounts.rent_receiver] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn close_ledger_account_verify_account_privileges<'me, 'info>(
    accounts: CloseLedgerAccountAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    close_ledger_account_verify_writable_privileges(accounts)?;
    close_ledger_account_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const INITIALIZE_LEDGER_ACCOUNT_IX_ACCOUNTS_LEN: usize = 4;
#[derive(Copy, Clone, Debug)]
pub struct InitializeLedgerAccountAccounts<'me, 'info> {
    pub ledger: &'me AccountInfo<'info>,
    pub owner: &'me AccountInfo<'info>,
    pub payer: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct InitializeLedgerAccountKeys {
    pub ledger: Pubkey,
    pub owner: Pubkey,
    pub payer: Pubkey,
    pub system_program: Pubkey,
}
impl From<InitializeLedgerAccountAccounts<'_, '_>> for InitializeLedgerAccountKeys {
    fn from(accounts: InitializeLedgerAccountAccounts) -> Self {
        Self {
            ledger: *accounts.ledger.key,
            owner: *accounts.owner.key,
            payer: *accounts.payer.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<InitializeLedgerAccountKeys>
for [AccountMeta; INITIALIZE_LEDGER_ACCOUNT_IX_ACCOUNTS_LEN] {
    fn from(keys: InitializeLedgerAccountKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.ledger,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.owner,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.payer,
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
impl From<[Pubkey; INITIALIZE_LEDGER_ACCOUNT_IX_ACCOUNTS_LEN]>
for InitializeLedgerAccountKeys {
    fn from(pubkeys: [Pubkey; INITIALIZE_LEDGER_ACCOUNT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            ledger: pubkeys[0],
            owner: pubkeys[1],
            payer: pubkeys[2],
            system_program: pubkeys[3],
        }
    }
}
impl<'info> From<InitializeLedgerAccountAccounts<'_, 'info>>
for [AccountInfo<'info>; INITIALIZE_LEDGER_ACCOUNT_IX_ACCOUNTS_LEN] {
    fn from(accounts: InitializeLedgerAccountAccounts<'_, 'info>) -> Self {
        [
            accounts.ledger.clone(),
            accounts.owner.clone(),
            accounts.payer.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; INITIALIZE_LEDGER_ACCOUNT_IX_ACCOUNTS_LEN]>
for InitializeLedgerAccountAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; INITIALIZE_LEDGER_ACCOUNT_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            ledger: &arr[0],
            owner: &arr[1],
            payer: &arr[2],
            system_program: &arr[3],
        }
    }
}
pub const INITIALIZE_LEDGER_ACCOUNT_IX_DISCM: [u8; 8usize] = [
    120, 69, 30, 74, 76, 242, 153, 162,
];
#[derive(Clone, Debug, PartialEq)]
pub struct InitializeLedgerAccountIxData;
impl InitializeLedgerAccountIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INITIALIZE_LEDGER_ACCOUNT_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INITIALIZE_LEDGER_ACCOUNT_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn initialize_ledger_account_ix_with_program_id(
    program_id: Pubkey,
    keys: InitializeLedgerAccountKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; INITIALIZE_LEDGER_ACCOUNT_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: InitializeLedgerAccountIxData.try_to_vec()?,
    })
}
pub fn initialize_ledger_account_ix(
    keys: InitializeLedgerAccountKeys,
) -> std::io::Result<Instruction> {
    initialize_ledger_account_ix_with_program_id(ZAP_PROGRAM_ID, keys)
}
pub fn initialize_ledger_account_invoke_with_program_id(
    program_id: Pubkey,
    accounts: InitializeLedgerAccountAccounts<'_, '_>,
) -> ProgramResult {
    let keys: InitializeLedgerAccountKeys = accounts.into();
    let ix = initialize_ledger_account_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn initialize_ledger_account_invoke(
    accounts: InitializeLedgerAccountAccounts<'_, '_>,
) -> ProgramResult {
    initialize_ledger_account_invoke_with_program_id(ZAP_PROGRAM_ID, accounts)
}
pub fn initialize_ledger_account_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: InitializeLedgerAccountAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: InitializeLedgerAccountKeys = accounts.into();
    let ix = initialize_ledger_account_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn initialize_ledger_account_invoke_signed(
    accounts: InitializeLedgerAccountAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    initialize_ledger_account_invoke_signed_with_program_id(
        ZAP_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn initialize_ledger_account_verify_account_keys(
    accounts: InitializeLedgerAccountAccounts<'_, '_>,
    keys: InitializeLedgerAccountKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.ledger.key, keys.ledger),
        (*accounts.owner.key, keys.owner),
        (*accounts.payer.key, keys.payer),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn initialize_ledger_account_verify_writable_privileges<'me, 'info>(
    accounts: InitializeLedgerAccountAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.ledger, accounts.payer] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn initialize_ledger_account_verify_signer_privileges<'me, 'info>(
    accounts: InitializeLedgerAccountAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.owner, accounts.payer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn initialize_ledger_account_verify_account_privileges<'me, 'info>(
    accounts: InitializeLedgerAccountAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    initialize_ledger_account_verify_writable_privileges(accounts)?;
    initialize_ledger_account_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SET_LEDGER_BALANCE_IX_ACCOUNTS_LEN: usize = 2;
#[derive(Copy, Clone, Debug)]
pub struct SetLedgerBalanceAccounts<'me, 'info> {
    pub ledger: &'me AccountInfo<'info>,
    pub owner: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SetLedgerBalanceKeys {
    pub ledger: Pubkey,
    pub owner: Pubkey,
}
impl From<SetLedgerBalanceAccounts<'_, '_>> for SetLedgerBalanceKeys {
    fn from(accounts: SetLedgerBalanceAccounts) -> Self {
        Self {
            ledger: *accounts.ledger.key,
            owner: *accounts.owner.key,
        }
    }
}
impl From<SetLedgerBalanceKeys> for [AccountMeta; SET_LEDGER_BALANCE_IX_ACCOUNTS_LEN] {
    fn from(keys: SetLedgerBalanceKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.ledger,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.owner,
                is_signer: true,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; SET_LEDGER_BALANCE_IX_ACCOUNTS_LEN]> for SetLedgerBalanceKeys {
    fn from(pubkeys: [Pubkey; SET_LEDGER_BALANCE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            ledger: pubkeys[0],
            owner: pubkeys[1],
        }
    }
}
impl<'info> From<SetLedgerBalanceAccounts<'_, 'info>>
for [AccountInfo<'info>; SET_LEDGER_BALANCE_IX_ACCOUNTS_LEN] {
    fn from(accounts: SetLedgerBalanceAccounts<'_, 'info>) -> Self {
        [accounts.ledger.clone(), accounts.owner.clone()]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SET_LEDGER_BALANCE_IX_ACCOUNTS_LEN]>
for SetLedgerBalanceAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; SET_LEDGER_BALANCE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            ledger: &arr[0],
            owner: &arr[1],
        }
    }
}
pub const SET_LEDGER_BALANCE_IX_DISCM: [u8; 8usize] = [
    131, 49, 240, 17, 228, 248, 156, 54,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SetLedgerBalanceIxArgs {
    pub amount: u64,
    pub is_token_a: bool,
}
#[derive(Clone, Debug, PartialEq)]
pub struct SetLedgerBalanceIxData(pub SetLedgerBalanceIxArgs);
impl From<SetLedgerBalanceIxArgs> for SetLedgerBalanceIxData {
    fn from(args: SetLedgerBalanceIxArgs) -> Self {
        Self(args)
    }
}
impl SetLedgerBalanceIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SET_LEDGER_BALANCE_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let is_token_a: bool = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(SetLedgerBalanceIxArgs {
                amount,
                is_token_a,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SET_LEDGER_BALANCE_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.is_token_a, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn set_ledger_balance_ix_with_program_id(
    program_id: Pubkey,
    keys: SetLedgerBalanceKeys,
    args: SetLedgerBalanceIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SET_LEDGER_BALANCE_IX_ACCOUNTS_LEN] = keys.into();
    let data: SetLedgerBalanceIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn set_ledger_balance_ix(
    keys: SetLedgerBalanceKeys,
    args: SetLedgerBalanceIxArgs,
) -> std::io::Result<Instruction> {
    set_ledger_balance_ix_with_program_id(ZAP_PROGRAM_ID, keys, args)
}
pub fn set_ledger_balance_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SetLedgerBalanceAccounts<'_, '_>,
    args: SetLedgerBalanceIxArgs,
) -> ProgramResult {
    let keys: SetLedgerBalanceKeys = accounts.into();
    let ix = set_ledger_balance_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn set_ledger_balance_invoke(
    accounts: SetLedgerBalanceAccounts<'_, '_>,
    args: SetLedgerBalanceIxArgs,
) -> ProgramResult {
    set_ledger_balance_invoke_with_program_id(ZAP_PROGRAM_ID, accounts, args)
}
pub fn set_ledger_balance_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SetLedgerBalanceAccounts<'_, '_>,
    args: SetLedgerBalanceIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SetLedgerBalanceKeys = accounts.into();
    let ix = set_ledger_balance_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn set_ledger_balance_invoke_signed(
    accounts: SetLedgerBalanceAccounts<'_, '_>,
    args: SetLedgerBalanceIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    set_ledger_balance_invoke_signed_with_program_id(
        ZAP_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn set_ledger_balance_verify_account_keys(
    accounts: SetLedgerBalanceAccounts<'_, '_>,
    keys: SetLedgerBalanceKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.ledger.key, keys.ledger),
        (*accounts.owner.key, keys.owner),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn set_ledger_balance_verify_writable_privileges<'me, 'info>(
    accounts: SetLedgerBalanceAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.ledger] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn set_ledger_balance_verify_signer_privileges<'me, 'info>(
    accounts: SetLedgerBalanceAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.owner] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn set_ledger_balance_verify_account_privileges<'me, 'info>(
    accounts: SetLedgerBalanceAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    set_ledger_balance_verify_writable_privileges(accounts)?;
    set_ledger_balance_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const UPDATE_LEDGER_BALANCE_AFTER_SWAP_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct UpdateLedgerBalanceAfterSwapAccounts<'me, 'info> {
    pub ledger: &'me AccountInfo<'info>,
    pub token_account: &'me AccountInfo<'info>,
    pub owner: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UpdateLedgerBalanceAfterSwapKeys {
    pub ledger: Pubkey,
    pub token_account: Pubkey,
    pub owner: Pubkey,
}
impl From<UpdateLedgerBalanceAfterSwapAccounts<'_, '_>>
for UpdateLedgerBalanceAfterSwapKeys {
    fn from(accounts: UpdateLedgerBalanceAfterSwapAccounts) -> Self {
        Self {
            ledger: *accounts.ledger.key,
            token_account: *accounts.token_account.key,
            owner: *accounts.owner.key,
        }
    }
}
impl From<UpdateLedgerBalanceAfterSwapKeys>
for [AccountMeta; UPDATE_LEDGER_BALANCE_AFTER_SWAP_IX_ACCOUNTS_LEN] {
    fn from(keys: UpdateLedgerBalanceAfterSwapKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.ledger,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_account,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.owner,
                is_signer: true,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; UPDATE_LEDGER_BALANCE_AFTER_SWAP_IX_ACCOUNTS_LEN]>
for UpdateLedgerBalanceAfterSwapKeys {
    fn from(
        pubkeys: [Pubkey; UPDATE_LEDGER_BALANCE_AFTER_SWAP_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            ledger: pubkeys[0],
            token_account: pubkeys[1],
            owner: pubkeys[2],
        }
    }
}
impl<'info> From<UpdateLedgerBalanceAfterSwapAccounts<'_, 'info>>
for [AccountInfo<'info>; UPDATE_LEDGER_BALANCE_AFTER_SWAP_IX_ACCOUNTS_LEN] {
    fn from(accounts: UpdateLedgerBalanceAfterSwapAccounts<'_, 'info>) -> Self {
        [accounts.ledger.clone(), accounts.token_account.clone(), accounts.owner.clone()]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; UPDATE_LEDGER_BALANCE_AFTER_SWAP_IX_ACCOUNTS_LEN]>
for UpdateLedgerBalanceAfterSwapAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; UPDATE_LEDGER_BALANCE_AFTER_SWAP_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            ledger: &arr[0],
            token_account: &arr[1],
            owner: &arr[2],
        }
    }
}
pub const UPDATE_LEDGER_BALANCE_AFTER_SWAP_IX_DISCM: [u8; 8usize] = [
    59, 206, 173, 232, 94, 57, 174, 202,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UpdateLedgerBalanceAfterSwapIxArgs {
    pub pre_source_token_balance: u64,
    pub max_transfer_amount: u64,
    pub is_token_a: bool,
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateLedgerBalanceAfterSwapIxData(pub UpdateLedgerBalanceAfterSwapIxArgs);
impl From<UpdateLedgerBalanceAfterSwapIxArgs> for UpdateLedgerBalanceAfterSwapIxData {
    fn from(args: UpdateLedgerBalanceAfterSwapIxArgs) -> Self {
        Self(args)
    }
}
impl UpdateLedgerBalanceAfterSwapIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_LEDGER_BALANCE_AFTER_SWAP_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let pre_source_token_balance: u64 = crate::borsh_de_or_default(&mut reader)?;
        let max_transfer_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let is_token_a: bool = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(UpdateLedgerBalanceAfterSwapIxArgs {
                pre_source_token_balance,
                max_transfer_amount,
                is_token_a,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_LEDGER_BALANCE_AFTER_SWAP_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.pre_source_token_balance, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.max_transfer_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.is_token_a, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn update_ledger_balance_after_swap_ix_with_program_id(
    program_id: Pubkey,
    keys: UpdateLedgerBalanceAfterSwapKeys,
    args: UpdateLedgerBalanceAfterSwapIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; UPDATE_LEDGER_BALANCE_AFTER_SWAP_IX_ACCOUNTS_LEN] = keys
        .into();
    let data: UpdateLedgerBalanceAfterSwapIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_ledger_balance_after_swap_ix(
    keys: UpdateLedgerBalanceAfterSwapKeys,
    args: UpdateLedgerBalanceAfterSwapIxArgs,
) -> std::io::Result<Instruction> {
    update_ledger_balance_after_swap_ix_with_program_id(ZAP_PROGRAM_ID, keys, args)
}
pub fn update_ledger_balance_after_swap_invoke_with_program_id(
    program_id: Pubkey,
    accounts: UpdateLedgerBalanceAfterSwapAccounts<'_, '_>,
    args: UpdateLedgerBalanceAfterSwapIxArgs,
) -> ProgramResult {
    let keys: UpdateLedgerBalanceAfterSwapKeys = accounts.into();
    let ix = update_ledger_balance_after_swap_ix_with_program_id(
        program_id,
        keys,
        args,
    )?;
    invoke_instruction(&ix, accounts)
}
pub fn update_ledger_balance_after_swap_invoke(
    accounts: UpdateLedgerBalanceAfterSwapAccounts<'_, '_>,
    args: UpdateLedgerBalanceAfterSwapIxArgs,
) -> ProgramResult {
    update_ledger_balance_after_swap_invoke_with_program_id(
        ZAP_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn update_ledger_balance_after_swap_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: UpdateLedgerBalanceAfterSwapAccounts<'_, '_>,
    args: UpdateLedgerBalanceAfterSwapIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: UpdateLedgerBalanceAfterSwapKeys = accounts.into();
    let ix = update_ledger_balance_after_swap_ix_with_program_id(
        program_id,
        keys,
        args,
    )?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn update_ledger_balance_after_swap_invoke_signed(
    accounts: UpdateLedgerBalanceAfterSwapAccounts<'_, '_>,
    args: UpdateLedgerBalanceAfterSwapIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    update_ledger_balance_after_swap_invoke_signed_with_program_id(
        ZAP_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn update_ledger_balance_after_swap_verify_account_keys(
    accounts: UpdateLedgerBalanceAfterSwapAccounts<'_, '_>,
    keys: UpdateLedgerBalanceAfterSwapKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.ledger.key, keys.ledger),
        (*accounts.token_account.key, keys.token_account),
        (*accounts.owner.key, keys.owner),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn update_ledger_balance_after_swap_verify_writable_privileges<'me, 'info>(
    accounts: UpdateLedgerBalanceAfterSwapAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.ledger] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn update_ledger_balance_after_swap_verify_signer_privileges<'me, 'info>(
    accounts: UpdateLedgerBalanceAfterSwapAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.owner] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn update_ledger_balance_after_swap_verify_account_privileges<'me, 'info>(
    accounts: UpdateLedgerBalanceAfterSwapAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    update_ledger_balance_after_swap_verify_writable_privileges(accounts)?;
    update_ledger_balance_after_swap_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const ZAP_IN_DAMM_V2_IX_ACCOUNTS_LEN: usize = 16;
#[derive(Copy, Clone, Debug)]
pub struct ZapInDammV2Accounts<'me, 'info> {
    pub ledger: &'me AccountInfo<'info>,
    pub pool: &'me AccountInfo<'info>,
    pub pool_authority: &'me AccountInfo<'info>,
    pub position: &'me AccountInfo<'info>,
    pub token_a_account: &'me AccountInfo<'info>,
    pub token_b_account: &'me AccountInfo<'info>,
    pub token_a_vault: &'me AccountInfo<'info>,
    pub token_b_vault: &'me AccountInfo<'info>,
    pub token_a_mint: &'me AccountInfo<'info>,
    pub token_b_mint: &'me AccountInfo<'info>,
    pub position_nft_account: &'me AccountInfo<'info>,
    pub owner: &'me AccountInfo<'info>,
    pub token_a_program: &'me AccountInfo<'info>,
    pub token_b_program: &'me AccountInfo<'info>,
    pub damm_program: &'me AccountInfo<'info>,
    pub damm_event_authority: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ZapInDammV2Keys {
    pub ledger: Pubkey,
    pub pool: Pubkey,
    pub pool_authority: Pubkey,
    pub position: Pubkey,
    pub token_a_account: Pubkey,
    pub token_b_account: Pubkey,
    pub token_a_vault: Pubkey,
    pub token_b_vault: Pubkey,
    pub token_a_mint: Pubkey,
    pub token_b_mint: Pubkey,
    pub position_nft_account: Pubkey,
    pub owner: Pubkey,
    pub token_a_program: Pubkey,
    pub token_b_program: Pubkey,
    pub damm_program: Pubkey,
    pub damm_event_authority: Pubkey,
}
impl From<ZapInDammV2Accounts<'_, '_>> for ZapInDammV2Keys {
    fn from(accounts: ZapInDammV2Accounts) -> Self {
        Self {
            ledger: *accounts.ledger.key,
            pool: *accounts.pool.key,
            pool_authority: *accounts.pool_authority.key,
            position: *accounts.position.key,
            token_a_account: *accounts.token_a_account.key,
            token_b_account: *accounts.token_b_account.key,
            token_a_vault: *accounts.token_a_vault.key,
            token_b_vault: *accounts.token_b_vault.key,
            token_a_mint: *accounts.token_a_mint.key,
            token_b_mint: *accounts.token_b_mint.key,
            position_nft_account: *accounts.position_nft_account.key,
            owner: *accounts.owner.key,
            token_a_program: *accounts.token_a_program.key,
            token_b_program: *accounts.token_b_program.key,
            damm_program: *accounts.damm_program.key,
            damm_event_authority: *accounts.damm_event_authority.key,
        }
    }
}
impl From<ZapInDammV2Keys> for [AccountMeta; ZAP_IN_DAMM_V2_IX_ACCOUNTS_LEN] {
    fn from(keys: ZapInDammV2Keys) -> Self {
        [
            AccountMeta {
                pubkey: keys.ledger,
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
                pubkey: keys.position,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_a_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_b_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_a_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_b_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_a_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_b_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.position_nft_account,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.owner,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_a_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_b_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.damm_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.damm_event_authority,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; ZAP_IN_DAMM_V2_IX_ACCOUNTS_LEN]> for ZapInDammV2Keys {
    fn from(pubkeys: [Pubkey; ZAP_IN_DAMM_V2_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            ledger: pubkeys[0],
            pool: pubkeys[1],
            pool_authority: pubkeys[2],
            position: pubkeys[3],
            token_a_account: pubkeys[4],
            token_b_account: pubkeys[5],
            token_a_vault: pubkeys[6],
            token_b_vault: pubkeys[7],
            token_a_mint: pubkeys[8],
            token_b_mint: pubkeys[9],
            position_nft_account: pubkeys[10],
            owner: pubkeys[11],
            token_a_program: pubkeys[12],
            token_b_program: pubkeys[13],
            damm_program: pubkeys[14],
            damm_event_authority: pubkeys[15],
        }
    }
}
impl<'info> From<ZapInDammV2Accounts<'_, 'info>>
for [AccountInfo<'info>; ZAP_IN_DAMM_V2_IX_ACCOUNTS_LEN] {
    fn from(accounts: ZapInDammV2Accounts<'_, 'info>) -> Self {
        [
            accounts.ledger.clone(),
            accounts.pool.clone(),
            accounts.pool_authority.clone(),
            accounts.position.clone(),
            accounts.token_a_account.clone(),
            accounts.token_b_account.clone(),
            accounts.token_a_vault.clone(),
            accounts.token_b_vault.clone(),
            accounts.token_a_mint.clone(),
            accounts.token_b_mint.clone(),
            accounts.position_nft_account.clone(),
            accounts.owner.clone(),
            accounts.token_a_program.clone(),
            accounts.token_b_program.clone(),
            accounts.damm_program.clone(),
            accounts.damm_event_authority.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; ZAP_IN_DAMM_V2_IX_ACCOUNTS_LEN]>
for ZapInDammV2Accounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; ZAP_IN_DAMM_V2_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            ledger: &arr[0],
            pool: &arr[1],
            pool_authority: &arr[2],
            position: &arr[3],
            token_a_account: &arr[4],
            token_b_account: &arr[5],
            token_a_vault: &arr[6],
            token_b_vault: &arr[7],
            token_a_mint: &arr[8],
            token_b_mint: &arr[9],
            position_nft_account: &arr[10],
            owner: &arr[11],
            token_a_program: &arr[12],
            token_b_program: &arr[13],
            damm_program: &arr[14],
            damm_event_authority: &arr[15],
        }
    }
}
pub const ZAP_IN_DAMM_V2_IX_DISCM: [u8; 8usize] = [243, 243, 119, 52, 199, 44, 154, 186];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ZapInDammV2IxArgs {
    pub pre_sqrt_price: u128,
    pub max_sqrt_price_change_bps: u32,
}
#[derive(Clone, Debug, PartialEq)]
pub struct ZapInDammV2IxData(pub ZapInDammV2IxArgs);
impl From<ZapInDammV2IxArgs> for ZapInDammV2IxData {
    fn from(args: ZapInDammV2IxArgs) -> Self {
        Self(args)
    }
}
impl ZapInDammV2IxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != ZAP_IN_DAMM_V2_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let pre_sqrt_price: u128 = crate::borsh_de_or_default(&mut reader)?;
        let max_sqrt_price_change_bps: u32 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(ZapInDammV2IxArgs {
                pre_sqrt_price,
                max_sqrt_price_change_bps,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&ZAP_IN_DAMM_V2_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.pre_sqrt_price, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.0.max_sqrt_price_change_bps,
            &mut writer,
        )?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn zap_in_damm_v2_ix_with_program_id(
    program_id: Pubkey,
    keys: ZapInDammV2Keys,
    args: ZapInDammV2IxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; ZAP_IN_DAMM_V2_IX_ACCOUNTS_LEN] = keys.into();
    let data: ZapInDammV2IxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn zap_in_damm_v2_ix(
    keys: ZapInDammV2Keys,
    args: ZapInDammV2IxArgs,
) -> std::io::Result<Instruction> {
    zap_in_damm_v2_ix_with_program_id(ZAP_PROGRAM_ID, keys, args)
}
pub fn zap_in_damm_v2_invoke_with_program_id(
    program_id: Pubkey,
    accounts: ZapInDammV2Accounts<'_, '_>,
    args: ZapInDammV2IxArgs,
) -> ProgramResult {
    let keys: ZapInDammV2Keys = accounts.into();
    let ix = zap_in_damm_v2_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn zap_in_damm_v2_invoke(
    accounts: ZapInDammV2Accounts<'_, '_>,
    args: ZapInDammV2IxArgs,
) -> ProgramResult {
    zap_in_damm_v2_invoke_with_program_id(ZAP_PROGRAM_ID, accounts, args)
}
pub fn zap_in_damm_v2_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: ZapInDammV2Accounts<'_, '_>,
    args: ZapInDammV2IxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: ZapInDammV2Keys = accounts.into();
    let ix = zap_in_damm_v2_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn zap_in_damm_v2_invoke_signed(
    accounts: ZapInDammV2Accounts<'_, '_>,
    args: ZapInDammV2IxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    zap_in_damm_v2_invoke_signed_with_program_id(ZAP_PROGRAM_ID, accounts, args, seeds)
}
pub fn zap_in_damm_v2_verify_account_keys(
    accounts: ZapInDammV2Accounts<'_, '_>,
    keys: ZapInDammV2Keys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.ledger.key, keys.ledger),
        (*accounts.pool.key, keys.pool),
        (*accounts.pool_authority.key, keys.pool_authority),
        (*accounts.position.key, keys.position),
        (*accounts.token_a_account.key, keys.token_a_account),
        (*accounts.token_b_account.key, keys.token_b_account),
        (*accounts.token_a_vault.key, keys.token_a_vault),
        (*accounts.token_b_vault.key, keys.token_b_vault),
        (*accounts.token_a_mint.key, keys.token_a_mint),
        (*accounts.token_b_mint.key, keys.token_b_mint),
        (*accounts.position_nft_account.key, keys.position_nft_account),
        (*accounts.owner.key, keys.owner),
        (*accounts.token_a_program.key, keys.token_a_program),
        (*accounts.token_b_program.key, keys.token_b_program),
        (*accounts.damm_program.key, keys.damm_program),
        (*accounts.damm_event_authority.key, keys.damm_event_authority),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn zap_in_damm_v2_verify_writable_privileges<'me, 'info>(
    accounts: ZapInDammV2Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.ledger,
        accounts.pool,
        accounts.position,
        accounts.token_a_account,
        accounts.token_b_account,
        accounts.token_a_vault,
        accounts.token_b_vault,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn zap_in_damm_v2_verify_signer_privileges<'me, 'info>(
    accounts: ZapInDammV2Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.owner] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn zap_in_damm_v2_verify_account_privileges<'me, 'info>(
    accounts: ZapInDammV2Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    zap_in_damm_v2_verify_writable_privileges(accounts)?;
    zap_in_damm_v2_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const ZAP_IN_DLMM_FOR_INITIALIZED_POSITION_IX_ACCOUNTS_LEN: usize = 18;
#[derive(Copy, Clone, Debug)]
pub struct ZapInDlmmForInitializedPositionAccounts<'me, 'info> {
    pub ledger: &'me AccountInfo<'info>,
    pub lb_pair: &'me AccountInfo<'info>,
    pub position: &'me AccountInfo<'info>,
    pub bin_array_bitmap_extension: &'me AccountInfo<'info>,
    pub user_token_x: &'me AccountInfo<'info>,
    pub user_token_y: &'me AccountInfo<'info>,
    pub reserve_x: &'me AccountInfo<'info>,
    pub reserve_y: &'me AccountInfo<'info>,
    pub token_x_mint: &'me AccountInfo<'info>,
    pub token_y_mint: &'me AccountInfo<'info>,
    pub dlmm_program: &'me AccountInfo<'info>,
    pub owner: &'me AccountInfo<'info>,
    pub rent_payer: &'me AccountInfo<'info>,
    pub token_x_program: &'me AccountInfo<'info>,
    pub token_y_program: &'me AccountInfo<'info>,
    pub memo_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub dlmm_event_authority: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ZapInDlmmForInitializedPositionKeys {
    pub ledger: Pubkey,
    pub lb_pair: Pubkey,
    pub position: Pubkey,
    pub bin_array_bitmap_extension: Pubkey,
    pub user_token_x: Pubkey,
    pub user_token_y: Pubkey,
    pub reserve_x: Pubkey,
    pub reserve_y: Pubkey,
    pub token_x_mint: Pubkey,
    pub token_y_mint: Pubkey,
    pub dlmm_program: Pubkey,
    pub owner: Pubkey,
    pub rent_payer: Pubkey,
    pub token_x_program: Pubkey,
    pub token_y_program: Pubkey,
    pub memo_program: Pubkey,
    pub system_program: Pubkey,
    pub dlmm_event_authority: Pubkey,
}
impl From<ZapInDlmmForInitializedPositionAccounts<'_, '_>>
for ZapInDlmmForInitializedPositionKeys {
    fn from(accounts: ZapInDlmmForInitializedPositionAccounts) -> Self {
        Self {
            ledger: *accounts.ledger.key,
            lb_pair: *accounts.lb_pair.key,
            position: *accounts.position.key,
            bin_array_bitmap_extension: *accounts.bin_array_bitmap_extension.key,
            user_token_x: *accounts.user_token_x.key,
            user_token_y: *accounts.user_token_y.key,
            reserve_x: *accounts.reserve_x.key,
            reserve_y: *accounts.reserve_y.key,
            token_x_mint: *accounts.token_x_mint.key,
            token_y_mint: *accounts.token_y_mint.key,
            dlmm_program: *accounts.dlmm_program.key,
            owner: *accounts.owner.key,
            rent_payer: *accounts.rent_payer.key,
            token_x_program: *accounts.token_x_program.key,
            token_y_program: *accounts.token_y_program.key,
            memo_program: *accounts.memo_program.key,
            system_program: *accounts.system_program.key,
            dlmm_event_authority: *accounts.dlmm_event_authority.key,
        }
    }
}
impl From<ZapInDlmmForInitializedPositionKeys>
for [AccountMeta; ZAP_IN_DLMM_FOR_INITIALIZED_POSITION_IX_ACCOUNTS_LEN] {
    fn from(keys: ZapInDlmmForInitializedPositionKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.ledger,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.lb_pair,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.position,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.bin_array_bitmap_extension,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_token_x,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_token_y,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.reserve_x,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.reserve_y,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_x_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_y_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.dlmm_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.owner,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.rent_payer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_x_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_y_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.memo_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.system_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.dlmm_event_authority,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; ZAP_IN_DLMM_FOR_INITIALIZED_POSITION_IX_ACCOUNTS_LEN]>
for ZapInDlmmForInitializedPositionKeys {
    fn from(
        pubkeys: [Pubkey; ZAP_IN_DLMM_FOR_INITIALIZED_POSITION_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            ledger: pubkeys[0],
            lb_pair: pubkeys[1],
            position: pubkeys[2],
            bin_array_bitmap_extension: pubkeys[3],
            user_token_x: pubkeys[4],
            user_token_y: pubkeys[5],
            reserve_x: pubkeys[6],
            reserve_y: pubkeys[7],
            token_x_mint: pubkeys[8],
            token_y_mint: pubkeys[9],
            dlmm_program: pubkeys[10],
            owner: pubkeys[11],
            rent_payer: pubkeys[12],
            token_x_program: pubkeys[13],
            token_y_program: pubkeys[14],
            memo_program: pubkeys[15],
            system_program: pubkeys[16],
            dlmm_event_authority: pubkeys[17],
        }
    }
}
impl<'info> From<ZapInDlmmForInitializedPositionAccounts<'_, 'info>>
for [AccountInfo<'info>; ZAP_IN_DLMM_FOR_INITIALIZED_POSITION_IX_ACCOUNTS_LEN] {
    fn from(accounts: ZapInDlmmForInitializedPositionAccounts<'_, 'info>) -> Self {
        [
            accounts.ledger.clone(),
            accounts.lb_pair.clone(),
            accounts.position.clone(),
            accounts.bin_array_bitmap_extension.clone(),
            accounts.user_token_x.clone(),
            accounts.user_token_y.clone(),
            accounts.reserve_x.clone(),
            accounts.reserve_y.clone(),
            accounts.token_x_mint.clone(),
            accounts.token_y_mint.clone(),
            accounts.dlmm_program.clone(),
            accounts.owner.clone(),
            accounts.rent_payer.clone(),
            accounts.token_x_program.clone(),
            accounts.token_y_program.clone(),
            accounts.memo_program.clone(),
            accounts.system_program.clone(),
            accounts.dlmm_event_authority.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; ZAP_IN_DLMM_FOR_INITIALIZED_POSITION_IX_ACCOUNTS_LEN]>
for ZapInDlmmForInitializedPositionAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<
            'info,
        >; ZAP_IN_DLMM_FOR_INITIALIZED_POSITION_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            ledger: &arr[0],
            lb_pair: &arr[1],
            position: &arr[2],
            bin_array_bitmap_extension: &arr[3],
            user_token_x: &arr[4],
            user_token_y: &arr[5],
            reserve_x: &arr[6],
            reserve_y: &arr[7],
            token_x_mint: &arr[8],
            token_y_mint: &arr[9],
            dlmm_program: &arr[10],
            owner: &arr[11],
            rent_payer: &arr[12],
            token_x_program: &arr[13],
            token_y_program: &arr[14],
            memo_program: &arr[15],
            system_program: &arr[16],
            dlmm_event_authority: &arr[17],
        }
    }
}
pub const ZAP_IN_DLMM_FOR_INITIALIZED_POSITION_IX_DISCM: [u8; 8usize] = [
    184, 71, 198, 231, 129, 110, 193, 67,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ZapInDlmmForInitializedPositionIxArgs {
    pub active_id: i32,
    pub min_delta_id: i32,
    pub max_delta_id: i32,
    pub max_active_bin_slippage: u16,
    pub favor_x_in_active_id: bool,
    pub strategy: StrategyType,
    pub remaining_accounts_info: RemainingAccountsInfo,
}
#[derive(Clone, Debug, PartialEq)]
pub struct ZapInDlmmForInitializedPositionIxData(
    pub ZapInDlmmForInitializedPositionIxArgs,
);
impl From<ZapInDlmmForInitializedPositionIxArgs>
for ZapInDlmmForInitializedPositionIxData {
    fn from(args: ZapInDlmmForInitializedPositionIxArgs) -> Self {
        Self(args)
    }
}
impl ZapInDlmmForInitializedPositionIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != ZAP_IN_DLMM_FOR_INITIALIZED_POSITION_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let active_id: i32 = crate::borsh_de_or_default(&mut reader)?;
        let min_delta_id: i32 = crate::borsh_de_or_default(&mut reader)?;
        let max_delta_id: i32 = crate::borsh_de_or_default(&mut reader)?;
        let max_active_bin_slippage: u16 = crate::borsh_de_or_default(&mut reader)?;
        let favor_x_in_active_id: bool = crate::borsh_de_or_default(&mut reader)?;
        let strategy: StrategyType = crate::borsh_de_or_default(&mut reader)?;
        let remaining_accounts_info = if reader.is_empty() {
            Default::default()
        } else {
            <RemainingAccountsInfo>::deserialize(&mut reader)?
        };
        Ok(
            Self(ZapInDlmmForInitializedPositionIxArgs {
                active_id,
                min_delta_id,
                max_delta_id,
                max_active_bin_slippage,
                favor_x_in_active_id,
                strategy,
                remaining_accounts_info,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&ZAP_IN_DLMM_FOR_INITIALIZED_POSITION_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.active_id, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.min_delta_id, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.max_delta_id, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.max_active_bin_slippage, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.favor_x_in_active_id, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.strategy, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.remaining_accounts_info, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn zap_in_dlmm_for_initialized_position_ix_with_program_id(
    program_id: Pubkey,
    keys: ZapInDlmmForInitializedPositionKeys,
    args: ZapInDlmmForInitializedPositionIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; ZAP_IN_DLMM_FOR_INITIALIZED_POSITION_IX_ACCOUNTS_LEN] = keys
        .into();
    let data: ZapInDlmmForInitializedPositionIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn zap_in_dlmm_for_initialized_position_ix(
    keys: ZapInDlmmForInitializedPositionKeys,
    args: ZapInDlmmForInitializedPositionIxArgs,
) -> std::io::Result<Instruction> {
    zap_in_dlmm_for_initialized_position_ix_with_program_id(ZAP_PROGRAM_ID, keys, args)
}
pub fn zap_in_dlmm_for_initialized_position_invoke_with_program_id(
    program_id: Pubkey,
    accounts: ZapInDlmmForInitializedPositionAccounts<'_, '_>,
    args: ZapInDlmmForInitializedPositionIxArgs,
) -> ProgramResult {
    let keys: ZapInDlmmForInitializedPositionKeys = accounts.into();
    let ix = zap_in_dlmm_for_initialized_position_ix_with_program_id(
        program_id,
        keys,
        args,
    )?;
    invoke_instruction(&ix, accounts)
}
pub fn zap_in_dlmm_for_initialized_position_invoke(
    accounts: ZapInDlmmForInitializedPositionAccounts<'_, '_>,
    args: ZapInDlmmForInitializedPositionIxArgs,
) -> ProgramResult {
    zap_in_dlmm_for_initialized_position_invoke_with_program_id(
        ZAP_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn zap_in_dlmm_for_initialized_position_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: ZapInDlmmForInitializedPositionAccounts<'_, '_>,
    args: ZapInDlmmForInitializedPositionIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: ZapInDlmmForInitializedPositionKeys = accounts.into();
    let ix = zap_in_dlmm_for_initialized_position_ix_with_program_id(
        program_id,
        keys,
        args,
    )?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn zap_in_dlmm_for_initialized_position_invoke_signed(
    accounts: ZapInDlmmForInitializedPositionAccounts<'_, '_>,
    args: ZapInDlmmForInitializedPositionIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    zap_in_dlmm_for_initialized_position_invoke_signed_with_program_id(
        ZAP_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn zap_in_dlmm_for_initialized_position_verify_account_keys(
    accounts: ZapInDlmmForInitializedPositionAccounts<'_, '_>,
    keys: ZapInDlmmForInitializedPositionKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.ledger.key, keys.ledger),
        (*accounts.lb_pair.key, keys.lb_pair),
        (*accounts.position.key, keys.position),
        (*accounts.bin_array_bitmap_extension.key, keys.bin_array_bitmap_extension),
        (*accounts.user_token_x.key, keys.user_token_x),
        (*accounts.user_token_y.key, keys.user_token_y),
        (*accounts.reserve_x.key, keys.reserve_x),
        (*accounts.reserve_y.key, keys.reserve_y),
        (*accounts.token_x_mint.key, keys.token_x_mint),
        (*accounts.token_y_mint.key, keys.token_y_mint),
        (*accounts.dlmm_program.key, keys.dlmm_program),
        (*accounts.owner.key, keys.owner),
        (*accounts.rent_payer.key, keys.rent_payer),
        (*accounts.token_x_program.key, keys.token_x_program),
        (*accounts.token_y_program.key, keys.token_y_program),
        (*accounts.memo_program.key, keys.memo_program),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.dlmm_event_authority.key, keys.dlmm_event_authority),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn zap_in_dlmm_for_initialized_position_verify_writable_privileges<'me, 'info>(
    accounts: ZapInDlmmForInitializedPositionAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.ledger,
        accounts.lb_pair,
        accounts.position,
        accounts.bin_array_bitmap_extension,
        accounts.user_token_x,
        accounts.user_token_y,
        accounts.reserve_x,
        accounts.reserve_y,
        accounts.rent_payer,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn zap_in_dlmm_for_initialized_position_verify_signer_privileges<'me, 'info>(
    accounts: ZapInDlmmForInitializedPositionAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.owner, accounts.rent_payer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn zap_in_dlmm_for_initialized_position_verify_account_privileges<'me, 'info>(
    accounts: ZapInDlmmForInitializedPositionAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    zap_in_dlmm_for_initialized_position_verify_writable_privileges(accounts)?;
    zap_in_dlmm_for_initialized_position_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const ZAP_IN_DLMM_FOR_UNINITIALIZED_POSITION_IX_ACCOUNTS_LEN: usize = 18;
#[derive(Copy, Clone, Debug)]
pub struct ZapInDlmmForUninitializedPositionAccounts<'me, 'info> {
    pub ledger: &'me AccountInfo<'info>,
    pub lb_pair: &'me AccountInfo<'info>,
    pub position: &'me AccountInfo<'info>,
    pub bin_array_bitmap_extension: &'me AccountInfo<'info>,
    pub user_token_x: &'me AccountInfo<'info>,
    pub user_token_y: &'me AccountInfo<'info>,
    pub reserve_x: &'me AccountInfo<'info>,
    pub reserve_y: &'me AccountInfo<'info>,
    pub token_x_mint: &'me AccountInfo<'info>,
    pub token_y_mint: &'me AccountInfo<'info>,
    pub dlmm_program: &'me AccountInfo<'info>,
    pub owner: &'me AccountInfo<'info>,
    pub rent_payer: &'me AccountInfo<'info>,
    pub token_x_program: &'me AccountInfo<'info>,
    pub token_y_program: &'me AccountInfo<'info>,
    pub memo_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub dlmm_event_authority: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ZapInDlmmForUninitializedPositionKeys {
    pub ledger: Pubkey,
    pub lb_pair: Pubkey,
    pub position: Pubkey,
    pub bin_array_bitmap_extension: Pubkey,
    pub user_token_x: Pubkey,
    pub user_token_y: Pubkey,
    pub reserve_x: Pubkey,
    pub reserve_y: Pubkey,
    pub token_x_mint: Pubkey,
    pub token_y_mint: Pubkey,
    pub dlmm_program: Pubkey,
    pub owner: Pubkey,
    pub rent_payer: Pubkey,
    pub token_x_program: Pubkey,
    pub token_y_program: Pubkey,
    pub memo_program: Pubkey,
    pub system_program: Pubkey,
    pub dlmm_event_authority: Pubkey,
}
impl From<ZapInDlmmForUninitializedPositionAccounts<'_, '_>>
for ZapInDlmmForUninitializedPositionKeys {
    fn from(accounts: ZapInDlmmForUninitializedPositionAccounts) -> Self {
        Self {
            ledger: *accounts.ledger.key,
            lb_pair: *accounts.lb_pair.key,
            position: *accounts.position.key,
            bin_array_bitmap_extension: *accounts.bin_array_bitmap_extension.key,
            user_token_x: *accounts.user_token_x.key,
            user_token_y: *accounts.user_token_y.key,
            reserve_x: *accounts.reserve_x.key,
            reserve_y: *accounts.reserve_y.key,
            token_x_mint: *accounts.token_x_mint.key,
            token_y_mint: *accounts.token_y_mint.key,
            dlmm_program: *accounts.dlmm_program.key,
            owner: *accounts.owner.key,
            rent_payer: *accounts.rent_payer.key,
            token_x_program: *accounts.token_x_program.key,
            token_y_program: *accounts.token_y_program.key,
            memo_program: *accounts.memo_program.key,
            system_program: *accounts.system_program.key,
            dlmm_event_authority: *accounts.dlmm_event_authority.key,
        }
    }
}
impl From<ZapInDlmmForUninitializedPositionKeys>
for [AccountMeta; ZAP_IN_DLMM_FOR_UNINITIALIZED_POSITION_IX_ACCOUNTS_LEN] {
    fn from(keys: ZapInDlmmForUninitializedPositionKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.ledger,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.lb_pair,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.position,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.bin_array_bitmap_extension,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_token_x,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_token_y,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.reserve_x,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.reserve_y,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_x_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_y_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.dlmm_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.owner,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.rent_payer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_x_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_y_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.memo_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.system_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.dlmm_event_authority,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; ZAP_IN_DLMM_FOR_UNINITIALIZED_POSITION_IX_ACCOUNTS_LEN]>
for ZapInDlmmForUninitializedPositionKeys {
    fn from(
        pubkeys: [Pubkey; ZAP_IN_DLMM_FOR_UNINITIALIZED_POSITION_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            ledger: pubkeys[0],
            lb_pair: pubkeys[1],
            position: pubkeys[2],
            bin_array_bitmap_extension: pubkeys[3],
            user_token_x: pubkeys[4],
            user_token_y: pubkeys[5],
            reserve_x: pubkeys[6],
            reserve_y: pubkeys[7],
            token_x_mint: pubkeys[8],
            token_y_mint: pubkeys[9],
            dlmm_program: pubkeys[10],
            owner: pubkeys[11],
            rent_payer: pubkeys[12],
            token_x_program: pubkeys[13],
            token_y_program: pubkeys[14],
            memo_program: pubkeys[15],
            system_program: pubkeys[16],
            dlmm_event_authority: pubkeys[17],
        }
    }
}
impl<'info> From<ZapInDlmmForUninitializedPositionAccounts<'_, 'info>>
for [AccountInfo<'info>; ZAP_IN_DLMM_FOR_UNINITIALIZED_POSITION_IX_ACCOUNTS_LEN] {
    fn from(accounts: ZapInDlmmForUninitializedPositionAccounts<'_, 'info>) -> Self {
        [
            accounts.ledger.clone(),
            accounts.lb_pair.clone(),
            accounts.position.clone(),
            accounts.bin_array_bitmap_extension.clone(),
            accounts.user_token_x.clone(),
            accounts.user_token_y.clone(),
            accounts.reserve_x.clone(),
            accounts.reserve_y.clone(),
            accounts.token_x_mint.clone(),
            accounts.token_y_mint.clone(),
            accounts.dlmm_program.clone(),
            accounts.owner.clone(),
            accounts.rent_payer.clone(),
            accounts.token_x_program.clone(),
            accounts.token_y_program.clone(),
            accounts.memo_program.clone(),
            accounts.system_program.clone(),
            accounts.dlmm_event_authority.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; ZAP_IN_DLMM_FOR_UNINITIALIZED_POSITION_IX_ACCOUNTS_LEN]>
for ZapInDlmmForUninitializedPositionAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<
            'info,
        >; ZAP_IN_DLMM_FOR_UNINITIALIZED_POSITION_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            ledger: &arr[0],
            lb_pair: &arr[1],
            position: &arr[2],
            bin_array_bitmap_extension: &arr[3],
            user_token_x: &arr[4],
            user_token_y: &arr[5],
            reserve_x: &arr[6],
            reserve_y: &arr[7],
            token_x_mint: &arr[8],
            token_y_mint: &arr[9],
            dlmm_program: &arr[10],
            owner: &arr[11],
            rent_payer: &arr[12],
            token_x_program: &arr[13],
            token_y_program: &arr[14],
            memo_program: &arr[15],
            system_program: &arr[16],
            dlmm_event_authority: &arr[17],
        }
    }
}
pub const ZAP_IN_DLMM_FOR_UNINITIALIZED_POSITION_IX_DISCM: [u8; 8usize] = [
    59, 220, 182, 27, 254, 253, 2, 232,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ZapInDlmmForUninitializedPositionIxArgs {
    pub min_delta_id: i32,
    pub max_delta_id: i32,
    pub active_id: i32,
    pub max_active_bin_slippage: u16,
    pub favor_x_in_active_id: bool,
    pub strategy: StrategyType,
    pub remaining_accounts_info: RemainingAccountsInfo,
}
#[derive(Clone, Debug, PartialEq)]
pub struct ZapInDlmmForUninitializedPositionIxData(
    pub ZapInDlmmForUninitializedPositionIxArgs,
);
impl From<ZapInDlmmForUninitializedPositionIxArgs>
for ZapInDlmmForUninitializedPositionIxData {
    fn from(args: ZapInDlmmForUninitializedPositionIxArgs) -> Self {
        Self(args)
    }
}
impl ZapInDlmmForUninitializedPositionIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != ZAP_IN_DLMM_FOR_UNINITIALIZED_POSITION_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let min_delta_id: i32 = crate::borsh_de_or_default(&mut reader)?;
        let max_delta_id: i32 = crate::borsh_de_or_default(&mut reader)?;
        let active_id: i32 = crate::borsh_de_or_default(&mut reader)?;
        let max_active_bin_slippage: u16 = crate::borsh_de_or_default(&mut reader)?;
        let favor_x_in_active_id: bool = crate::borsh_de_or_default(&mut reader)?;
        let strategy: StrategyType = crate::borsh_de_or_default(&mut reader)?;
        let remaining_accounts_info = if reader.is_empty() {
            Default::default()
        } else {
            <RemainingAccountsInfo>::deserialize(&mut reader)?
        };
        Ok(
            Self(ZapInDlmmForUninitializedPositionIxArgs {
                min_delta_id,
                max_delta_id,
                active_id,
                max_active_bin_slippage,
                favor_x_in_active_id,
                strategy,
                remaining_accounts_info,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&ZAP_IN_DLMM_FOR_UNINITIALIZED_POSITION_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.min_delta_id, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.max_delta_id, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.active_id, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.max_active_bin_slippage, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.favor_x_in_active_id, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.strategy, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.remaining_accounts_info, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn zap_in_dlmm_for_uninitialized_position_ix_with_program_id(
    program_id: Pubkey,
    keys: ZapInDlmmForUninitializedPositionKeys,
    args: ZapInDlmmForUninitializedPositionIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; ZAP_IN_DLMM_FOR_UNINITIALIZED_POSITION_IX_ACCOUNTS_LEN] = keys
        .into();
    let data: ZapInDlmmForUninitializedPositionIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn zap_in_dlmm_for_uninitialized_position_ix(
    keys: ZapInDlmmForUninitializedPositionKeys,
    args: ZapInDlmmForUninitializedPositionIxArgs,
) -> std::io::Result<Instruction> {
    zap_in_dlmm_for_uninitialized_position_ix_with_program_id(ZAP_PROGRAM_ID, keys, args)
}
pub fn zap_in_dlmm_for_uninitialized_position_invoke_with_program_id(
    program_id: Pubkey,
    accounts: ZapInDlmmForUninitializedPositionAccounts<'_, '_>,
    args: ZapInDlmmForUninitializedPositionIxArgs,
) -> ProgramResult {
    let keys: ZapInDlmmForUninitializedPositionKeys = accounts.into();
    let ix = zap_in_dlmm_for_uninitialized_position_ix_with_program_id(
        program_id,
        keys,
        args,
    )?;
    invoke_instruction(&ix, accounts)
}
pub fn zap_in_dlmm_for_uninitialized_position_invoke(
    accounts: ZapInDlmmForUninitializedPositionAccounts<'_, '_>,
    args: ZapInDlmmForUninitializedPositionIxArgs,
) -> ProgramResult {
    zap_in_dlmm_for_uninitialized_position_invoke_with_program_id(
        ZAP_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn zap_in_dlmm_for_uninitialized_position_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: ZapInDlmmForUninitializedPositionAccounts<'_, '_>,
    args: ZapInDlmmForUninitializedPositionIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: ZapInDlmmForUninitializedPositionKeys = accounts.into();
    let ix = zap_in_dlmm_for_uninitialized_position_ix_with_program_id(
        program_id,
        keys,
        args,
    )?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn zap_in_dlmm_for_uninitialized_position_invoke_signed(
    accounts: ZapInDlmmForUninitializedPositionAccounts<'_, '_>,
    args: ZapInDlmmForUninitializedPositionIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    zap_in_dlmm_for_uninitialized_position_invoke_signed_with_program_id(
        ZAP_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn zap_in_dlmm_for_uninitialized_position_verify_account_keys(
    accounts: ZapInDlmmForUninitializedPositionAccounts<'_, '_>,
    keys: ZapInDlmmForUninitializedPositionKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.ledger.key, keys.ledger),
        (*accounts.lb_pair.key, keys.lb_pair),
        (*accounts.position.key, keys.position),
        (*accounts.bin_array_bitmap_extension.key, keys.bin_array_bitmap_extension),
        (*accounts.user_token_x.key, keys.user_token_x),
        (*accounts.user_token_y.key, keys.user_token_y),
        (*accounts.reserve_x.key, keys.reserve_x),
        (*accounts.reserve_y.key, keys.reserve_y),
        (*accounts.token_x_mint.key, keys.token_x_mint),
        (*accounts.token_y_mint.key, keys.token_y_mint),
        (*accounts.dlmm_program.key, keys.dlmm_program),
        (*accounts.owner.key, keys.owner),
        (*accounts.rent_payer.key, keys.rent_payer),
        (*accounts.token_x_program.key, keys.token_x_program),
        (*accounts.token_y_program.key, keys.token_y_program),
        (*accounts.memo_program.key, keys.memo_program),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.dlmm_event_authority.key, keys.dlmm_event_authority),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn zap_in_dlmm_for_uninitialized_position_verify_writable_privileges<'me, 'info>(
    accounts: ZapInDlmmForUninitializedPositionAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.ledger,
        accounts.lb_pair,
        accounts.position,
        accounts.bin_array_bitmap_extension,
        accounts.user_token_x,
        accounts.user_token_y,
        accounts.reserve_x,
        accounts.reserve_y,
        accounts.rent_payer,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn zap_in_dlmm_for_uninitialized_position_verify_signer_privileges<'me, 'info>(
    accounts: ZapInDlmmForUninitializedPositionAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.position, accounts.owner, accounts.rent_payer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn zap_in_dlmm_for_uninitialized_position_verify_account_privileges<'me, 'info>(
    accounts: ZapInDlmmForUninitializedPositionAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    zap_in_dlmm_for_uninitialized_position_verify_writable_privileges(accounts)?;
    zap_in_dlmm_for_uninitialized_position_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const ZAP_OUT_IX_ACCOUNTS_LEN: usize = 2;
#[derive(Copy, Clone, Debug)]
pub struct ZapOutAccounts<'me, 'info> {
    pub user_token_in_account: &'me AccountInfo<'info>,
    pub amm_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ZapOutKeys {
    pub user_token_in_account: Pubkey,
    pub amm_program: Pubkey,
}
impl From<ZapOutAccounts<'_, '_>> for ZapOutKeys {
    fn from(accounts: ZapOutAccounts) -> Self {
        Self {
            user_token_in_account: *accounts.user_token_in_account.key,
            amm_program: *accounts.amm_program.key,
        }
    }
}
impl From<ZapOutKeys> for [AccountMeta; ZAP_OUT_IX_ACCOUNTS_LEN] {
    fn from(keys: ZapOutKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.user_token_in_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.amm_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; ZAP_OUT_IX_ACCOUNTS_LEN]> for ZapOutKeys {
    fn from(pubkeys: [Pubkey; ZAP_OUT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            user_token_in_account: pubkeys[0],
            amm_program: pubkeys[1],
        }
    }
}
impl<'info> From<ZapOutAccounts<'_, 'info>>
for [AccountInfo<'info>; ZAP_OUT_IX_ACCOUNTS_LEN] {
    fn from(accounts: ZapOutAccounts<'_, 'info>) -> Self {
        [accounts.user_token_in_account.clone(), accounts.amm_program.clone()]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; ZAP_OUT_IX_ACCOUNTS_LEN]>
for ZapOutAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; ZAP_OUT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            user_token_in_account: &arr[0],
            amm_program: &arr[1],
        }
    }
}
pub const ZAP_OUT_IX_DISCM: [u8; 8usize] = [155, 108, 185, 112, 104, 210, 161, 64];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ZapOutIxArgs {
    pub params: ZapOutParameters,
}
#[derive(Clone, Debug, PartialEq)]
pub struct ZapOutIxData(pub ZapOutIxArgs);
impl From<ZapOutIxArgs> for ZapOutIxData {
    fn from(args: ZapOutIxArgs) -> Self {
        Self(args)
    }
}
impl ZapOutIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != ZAP_OUT_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let params = if reader.is_empty() {
            Default::default()
        } else {
            <ZapOutParameters>::deserialize(&mut reader)?
        };
        Ok(Self(ZapOutIxArgs { params }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&ZAP_OUT_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.params, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn zap_out_ix_with_program_id(
    program_id: Pubkey,
    keys: ZapOutKeys,
    args: ZapOutIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; ZAP_OUT_IX_ACCOUNTS_LEN] = keys.into();
    let data: ZapOutIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn zap_out_ix(keys: ZapOutKeys, args: ZapOutIxArgs) -> std::io::Result<Instruction> {
    zap_out_ix_with_program_id(ZAP_PROGRAM_ID, keys, args)
}
pub fn zap_out_invoke_with_program_id(
    program_id: Pubkey,
    accounts: ZapOutAccounts<'_, '_>,
    args: ZapOutIxArgs,
) -> ProgramResult {
    let keys: ZapOutKeys = accounts.into();
    let ix = zap_out_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn zap_out_invoke(
    accounts: ZapOutAccounts<'_, '_>,
    args: ZapOutIxArgs,
) -> ProgramResult {
    zap_out_invoke_with_program_id(ZAP_PROGRAM_ID, accounts, args)
}
pub fn zap_out_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: ZapOutAccounts<'_, '_>,
    args: ZapOutIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: ZapOutKeys = accounts.into();
    let ix = zap_out_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn zap_out_invoke_signed(
    accounts: ZapOutAccounts<'_, '_>,
    args: ZapOutIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    zap_out_invoke_signed_with_program_id(ZAP_PROGRAM_ID, accounts, args, seeds)
}
pub fn zap_out_verify_account_keys(
    accounts: ZapOutAccounts<'_, '_>,
    keys: ZapOutKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.user_token_in_account.key, keys.user_token_in_account),
        (*accounts.amm_program.key, keys.amm_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn zap_out_verify_writable_privileges<'me, 'info>(
    accounts: ZapOutAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.user_token_in_account] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn zap_out_verify_account_privileges<'me, 'info>(
    accounts: ZapOutAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    zap_out_verify_writable_privileges(accounts)?;
    Ok(())
}
