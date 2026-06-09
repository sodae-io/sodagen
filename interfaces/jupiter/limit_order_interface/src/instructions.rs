use solana_pubkey::Pubkey;
use solana_cpi::{invoke, invoke_signed};
use solana_instruction::{AccountMeta, Instruction};
use solana_account_info::AccountInfo;
use solana_program_error::ProgramError;
use std::io::Read;
#[allow(unused_imports)]
use crate::*;
#[derive(Clone, Debug, PartialEq)]
pub enum LimitOrderProgramIx {
    InitializeOrder(InitializeOrderIxArgs),
    FillOrder(FillOrderIxArgs),
    PreFlashFillOrder(PreFlashFillOrderIxArgs),
    FlashFillOrder(FlashFillOrderIxArgs),
    CancelOrder,
    CancelExpiredOrder,
    WithdrawFee(WithdrawFeeIxArgs),
    InitFee(InitFeeIxArgs),
    UpdateFee(UpdateFeeIxArgs),
}
impl LimitOrderProgramIx {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        if buf.starts_with(&INITIALIZE_ORDER_IX_DISCM) {
            let mut reader = &buf[INITIALIZE_ORDER_IX_DISCM.len()..];
            let making_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            let taking_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            let expired_at: Option<i64> = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::InitializeOrder(InitializeOrderIxArgs {
                    making_amount,
                    taking_amount,
                    expired_at,
                }),
            );
        }
        if buf.starts_with(&FILL_ORDER_IX_DISCM) {
            let mut reader = &buf[FILL_ORDER_IX_DISCM.len()..];
            let making_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            let max_taking_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::FillOrder(FillOrderIxArgs {
                    making_amount,
                    max_taking_amount,
                }),
            );
        }
        if buf.starts_with(&PRE_FLASH_FILL_ORDER_IX_DISCM) {
            let mut reader = &buf[PRE_FLASH_FILL_ORDER_IX_DISCM.len()..];
            let making_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::PreFlashFillOrder(PreFlashFillOrderIxArgs {
                    making_amount,
                }),
            );
        }
        if buf.starts_with(&FLASH_FILL_ORDER_IX_DISCM) {
            let mut reader = &buf[FLASH_FILL_ORDER_IX_DISCM.len()..];
            let max_taking_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::FlashFillOrder(FlashFillOrderIxArgs {
                    max_taking_amount,
                }),
            );
        }
        if buf.starts_with(&CANCEL_ORDER_IX_DISCM) {
            return Ok(Self::CancelOrder);
        }
        if buf.starts_with(&CANCEL_EXPIRED_ORDER_IX_DISCM) {
            return Ok(Self::CancelExpiredOrder);
        }
        if buf.starts_with(&WITHDRAW_FEE_IX_DISCM) {
            let mut reader = &buf[WITHDRAW_FEE_IX_DISCM.len()..];
            let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(Self::WithdrawFee(WithdrawFeeIxArgs { amount }));
        }
        if buf.starts_with(&INIT_FEE_IX_DISCM) {
            let mut reader = &buf[INIT_FEE_IX_DISCM.len()..];
            let maker_fee: u64 = crate::borsh_de_or_default(&mut reader)?;
            let maker_stable_fee: u64 = crate::borsh_de_or_default(&mut reader)?;
            let taker_fee: u64 = crate::borsh_de_or_default(&mut reader)?;
            let taker_stable_fee: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::InitFee(InitFeeIxArgs {
                    maker_fee,
                    maker_stable_fee,
                    taker_fee,
                    taker_stable_fee,
                }),
            );
        }
        if buf.starts_with(&UPDATE_FEE_IX_DISCM) {
            let mut reader = &buf[UPDATE_FEE_IX_DISCM.len()..];
            let maker_fee: u64 = crate::borsh_de_or_default(&mut reader)?;
            let maker_stable_fee: u64 = crate::borsh_de_or_default(&mut reader)?;
            let taker_fee: u64 = crate::borsh_de_or_default(&mut reader)?;
            let taker_stable_fee: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::UpdateFee(UpdateFeeIxArgs {
                    maker_fee,
                    maker_stable_fee,
                    taker_fee,
                    taker_stable_fee,
                }),
            );
        }
        Err(std::io::Error::from(std::io::ErrorKind::InvalidData))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        match self {
            Self::InitializeOrder(args) => {
                writer.write_all(&INITIALIZE_ORDER_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.making_amount, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.taking_amount, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.expired_at, &mut writer)?;
                Ok(())
            }
            Self::FillOrder(args) => {
                writer.write_all(&FILL_ORDER_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.making_amount, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.max_taking_amount, &mut writer)?;
                Ok(())
            }
            Self::PreFlashFillOrder(args) => {
                writer.write_all(&PRE_FLASH_FILL_ORDER_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.making_amount, &mut writer)?;
                Ok(())
            }
            Self::FlashFillOrder(args) => {
                writer.write_all(&FLASH_FILL_ORDER_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.max_taking_amount, &mut writer)?;
                Ok(())
            }
            Self::CancelOrder => writer.write_all(&CANCEL_ORDER_IX_DISCM),
            Self::CancelExpiredOrder => writer.write_all(&CANCEL_EXPIRED_ORDER_IX_DISCM),
            Self::WithdrawFee(args) => {
                writer.write_all(&WITHDRAW_FEE_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.amount, &mut writer)?;
                Ok(())
            }
            Self::InitFee(args) => {
                writer.write_all(&INIT_FEE_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.maker_fee, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.maker_stable_fee, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.taker_fee, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.taker_stable_fee, &mut writer)?;
                Ok(())
            }
            Self::UpdateFee(args) => {
                writer.write_all(&UPDATE_FEE_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.maker_fee, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.maker_stable_fee, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.taker_fee, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.taker_stable_fee, &mut writer)?;
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
pub const INITIALIZE_ORDER_IX_ACCOUNTS_LEN: usize = 12;
#[derive(Copy, Clone, Debug)]
pub struct InitializeOrderAccounts<'me, 'info> {
    pub base: &'me AccountInfo<'info>,
    pub maker: &'me AccountInfo<'info>,
    pub order: &'me AccountInfo<'info>,
    pub reserve: &'me AccountInfo<'info>,
    pub maker_input_account: &'me AccountInfo<'info>,
    pub input_mint: &'me AccountInfo<'info>,
    pub maker_output_account: &'me AccountInfo<'info>,
    pub referral: &'me AccountInfo<'info>,
    pub output_mint: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub rent: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct InitializeOrderKeys {
    pub base: Pubkey,
    pub maker: Pubkey,
    pub order: Pubkey,
    pub reserve: Pubkey,
    pub maker_input_account: Pubkey,
    pub input_mint: Pubkey,
    pub maker_output_account: Pubkey,
    pub referral: Pubkey,
    pub output_mint: Pubkey,
    pub system_program: Pubkey,
    pub token_program: Pubkey,
    pub rent: Pubkey,
}
impl From<InitializeOrderAccounts<'_, '_>> for InitializeOrderKeys {
    fn from(accounts: InitializeOrderAccounts) -> Self {
        Self {
            base: *accounts.base.key,
            maker: *accounts.maker.key,
            order: *accounts.order.key,
            reserve: *accounts.reserve.key,
            maker_input_account: *accounts.maker_input_account.key,
            input_mint: *accounts.input_mint.key,
            maker_output_account: *accounts.maker_output_account.key,
            referral: *accounts.referral.key,
            output_mint: *accounts.output_mint.key,
            system_program: *accounts.system_program.key,
            token_program: *accounts.token_program.key,
            rent: *accounts.rent.key,
        }
    }
}
impl From<InitializeOrderKeys> for [AccountMeta; INITIALIZE_ORDER_IX_ACCOUNTS_LEN] {
    fn from(keys: InitializeOrderKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.base,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.maker,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.order,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.reserve,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.maker_input_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.input_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.maker_output_account,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.referral,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.output_mint,
                is_signer: false,
                is_writable: false,
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
                pubkey: keys.rent,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; INITIALIZE_ORDER_IX_ACCOUNTS_LEN]> for InitializeOrderKeys {
    fn from(pubkeys: [Pubkey; INITIALIZE_ORDER_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            base: pubkeys[0],
            maker: pubkeys[1],
            order: pubkeys[2],
            reserve: pubkeys[3],
            maker_input_account: pubkeys[4],
            input_mint: pubkeys[5],
            maker_output_account: pubkeys[6],
            referral: pubkeys[7],
            output_mint: pubkeys[8],
            system_program: pubkeys[9],
            token_program: pubkeys[10],
            rent: pubkeys[11],
        }
    }
}
impl<'info> From<InitializeOrderAccounts<'_, 'info>>
for [AccountInfo<'info>; INITIALIZE_ORDER_IX_ACCOUNTS_LEN] {
    fn from(accounts: InitializeOrderAccounts<'_, 'info>) -> Self {
        [
            accounts.base.clone(),
            accounts.maker.clone(),
            accounts.order.clone(),
            accounts.reserve.clone(),
            accounts.maker_input_account.clone(),
            accounts.input_mint.clone(),
            accounts.maker_output_account.clone(),
            accounts.referral.clone(),
            accounts.output_mint.clone(),
            accounts.system_program.clone(),
            accounts.token_program.clone(),
            accounts.rent.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; INITIALIZE_ORDER_IX_ACCOUNTS_LEN]>
for InitializeOrderAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; INITIALIZE_ORDER_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            base: &arr[0],
            maker: &arr[1],
            order: &arr[2],
            reserve: &arr[3],
            maker_input_account: &arr[4],
            input_mint: &arr[5],
            maker_output_account: &arr[6],
            referral: &arr[7],
            output_mint: &arr[8],
            system_program: &arr[9],
            token_program: &arr[10],
            rent: &arr[11],
        }
    }
}
pub const INITIALIZE_ORDER_IX_DISCM: [u8; 8usize] = [
    133, 110, 74, 175, 112, 159, 245, 159,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct InitializeOrderIxArgs {
    pub making_amount: u64,
    pub taking_amount: u64,
    pub expired_at: Option<i64>,
}
#[derive(Clone, Debug, PartialEq)]
pub struct InitializeOrderIxData(pub InitializeOrderIxArgs);
impl From<InitializeOrderIxArgs> for InitializeOrderIxData {
    fn from(args: InitializeOrderIxArgs) -> Self {
        Self(args)
    }
}
impl InitializeOrderIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INITIALIZE_ORDER_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let making_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let taking_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let expired_at: Option<i64> = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(InitializeOrderIxArgs {
                making_amount,
                taking_amount,
                expired_at,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INITIALIZE_ORDER_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.making_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.taking_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.expired_at, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn initialize_order_ix_with_program_id(
    program_id: Pubkey,
    keys: InitializeOrderKeys,
    args: InitializeOrderIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; INITIALIZE_ORDER_IX_ACCOUNTS_LEN] = keys.into();
    let data: InitializeOrderIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn initialize_order_ix(
    keys: InitializeOrderKeys,
    args: InitializeOrderIxArgs,
) -> std::io::Result<Instruction> {
    initialize_order_ix_with_program_id(LIMIT_ORDER_PROGRAM_ID, keys, args)
}
pub fn initialize_order_invoke_with_program_id(
    program_id: Pubkey,
    accounts: InitializeOrderAccounts<'_, '_>,
    args: InitializeOrderIxArgs,
) -> ProgramResult {
    let keys: InitializeOrderKeys = accounts.into();
    let ix = initialize_order_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn initialize_order_invoke(
    accounts: InitializeOrderAccounts<'_, '_>,
    args: InitializeOrderIxArgs,
) -> ProgramResult {
    initialize_order_invoke_with_program_id(LIMIT_ORDER_PROGRAM_ID, accounts, args)
}
pub fn initialize_order_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: InitializeOrderAccounts<'_, '_>,
    args: InitializeOrderIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: InitializeOrderKeys = accounts.into();
    let ix = initialize_order_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn initialize_order_invoke_signed(
    accounts: InitializeOrderAccounts<'_, '_>,
    args: InitializeOrderIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    initialize_order_invoke_signed_with_program_id(
        LIMIT_ORDER_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn initialize_order_verify_account_keys(
    accounts: InitializeOrderAccounts<'_, '_>,
    keys: InitializeOrderKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.base.key, keys.base),
        (*accounts.maker.key, keys.maker),
        (*accounts.order.key, keys.order),
        (*accounts.reserve.key, keys.reserve),
        (*accounts.maker_input_account.key, keys.maker_input_account),
        (*accounts.input_mint.key, keys.input_mint),
        (*accounts.maker_output_account.key, keys.maker_output_account),
        (*accounts.referral.key, keys.referral),
        (*accounts.output_mint.key, keys.output_mint),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.rent.key, keys.rent),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn initialize_order_verify_writable_privileges<'me, 'info>(
    accounts: InitializeOrderAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.maker,
        accounts.order,
        accounts.reserve,
        accounts.maker_input_account,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn initialize_order_verify_signer_privileges<'me, 'info>(
    accounts: InitializeOrderAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.base, accounts.maker] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn initialize_order_verify_account_privileges<'me, 'info>(
    accounts: InitializeOrderAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    initialize_order_verify_writable_privileges(accounts)?;
    initialize_order_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const FILL_ORDER_IX_ACCOUNTS_LEN: usize = 12;
#[derive(Copy, Clone, Debug)]
pub struct FillOrderAccounts<'me, 'info> {
    pub order: &'me AccountInfo<'info>,
    pub reserve: &'me AccountInfo<'info>,
    pub maker: &'me AccountInfo<'info>,
    pub taker: &'me AccountInfo<'info>,
    pub taker_output_account: &'me AccountInfo<'info>,
    pub maker_output_account: &'me AccountInfo<'info>,
    pub taker_input_account: &'me AccountInfo<'info>,
    pub fee_authority: &'me AccountInfo<'info>,
    pub program_fee_account: &'me AccountInfo<'info>,
    pub referral: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FillOrderKeys {
    pub order: Pubkey,
    pub reserve: Pubkey,
    pub maker: Pubkey,
    pub taker: Pubkey,
    pub taker_output_account: Pubkey,
    pub maker_output_account: Pubkey,
    pub taker_input_account: Pubkey,
    pub fee_authority: Pubkey,
    pub program_fee_account: Pubkey,
    pub referral: Pubkey,
    pub token_program: Pubkey,
    pub system_program: Pubkey,
}
impl From<FillOrderAccounts<'_, '_>> for FillOrderKeys {
    fn from(accounts: FillOrderAccounts) -> Self {
        Self {
            order: *accounts.order.key,
            reserve: *accounts.reserve.key,
            maker: *accounts.maker.key,
            taker: *accounts.taker.key,
            taker_output_account: *accounts.taker_output_account.key,
            maker_output_account: *accounts.maker_output_account.key,
            taker_input_account: *accounts.taker_input_account.key,
            fee_authority: *accounts.fee_authority.key,
            program_fee_account: *accounts.program_fee_account.key,
            referral: *accounts.referral.key,
            token_program: *accounts.token_program.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<FillOrderKeys> for [AccountMeta; FILL_ORDER_IX_ACCOUNTS_LEN] {
    fn from(keys: FillOrderKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.order,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.reserve,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.maker,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.taker,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.taker_output_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.maker_output_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.taker_input_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.fee_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.program_fee_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.referral,
                is_signer: false,
                is_writable: true,
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
impl From<[Pubkey; FILL_ORDER_IX_ACCOUNTS_LEN]> for FillOrderKeys {
    fn from(pubkeys: [Pubkey; FILL_ORDER_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            order: pubkeys[0],
            reserve: pubkeys[1],
            maker: pubkeys[2],
            taker: pubkeys[3],
            taker_output_account: pubkeys[4],
            maker_output_account: pubkeys[5],
            taker_input_account: pubkeys[6],
            fee_authority: pubkeys[7],
            program_fee_account: pubkeys[8],
            referral: pubkeys[9],
            token_program: pubkeys[10],
            system_program: pubkeys[11],
        }
    }
}
impl<'info> From<FillOrderAccounts<'_, 'info>>
for [AccountInfo<'info>; FILL_ORDER_IX_ACCOUNTS_LEN] {
    fn from(accounts: FillOrderAccounts<'_, 'info>) -> Self {
        [
            accounts.order.clone(),
            accounts.reserve.clone(),
            accounts.maker.clone(),
            accounts.taker.clone(),
            accounts.taker_output_account.clone(),
            accounts.maker_output_account.clone(),
            accounts.taker_input_account.clone(),
            accounts.fee_authority.clone(),
            accounts.program_fee_account.clone(),
            accounts.referral.clone(),
            accounts.token_program.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; FILL_ORDER_IX_ACCOUNTS_LEN]>
for FillOrderAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; FILL_ORDER_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            order: &arr[0],
            reserve: &arr[1],
            maker: &arr[2],
            taker: &arr[3],
            taker_output_account: &arr[4],
            maker_output_account: &arr[5],
            taker_input_account: &arr[6],
            fee_authority: &arr[7],
            program_fee_account: &arr[8],
            referral: &arr[9],
            token_program: &arr[10],
            system_program: &arr[11],
        }
    }
}
pub const FILL_ORDER_IX_DISCM: [u8; 8usize] = [232, 122, 115, 25, 199, 143, 136, 162];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct FillOrderIxArgs {
    pub making_amount: u64,
    pub max_taking_amount: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct FillOrderIxData(pub FillOrderIxArgs);
impl From<FillOrderIxArgs> for FillOrderIxData {
    fn from(args: FillOrderIxArgs) -> Self {
        Self(args)
    }
}
impl FillOrderIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != FILL_ORDER_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let making_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let max_taking_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(FillOrderIxArgs {
                making_amount,
                max_taking_amount,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&FILL_ORDER_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.making_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.max_taking_amount, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn fill_order_ix_with_program_id(
    program_id: Pubkey,
    keys: FillOrderKeys,
    args: FillOrderIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; FILL_ORDER_IX_ACCOUNTS_LEN] = keys.into();
    let data: FillOrderIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn fill_order_ix(
    keys: FillOrderKeys,
    args: FillOrderIxArgs,
) -> std::io::Result<Instruction> {
    fill_order_ix_with_program_id(LIMIT_ORDER_PROGRAM_ID, keys, args)
}
pub fn fill_order_invoke_with_program_id(
    program_id: Pubkey,
    accounts: FillOrderAccounts<'_, '_>,
    args: FillOrderIxArgs,
) -> ProgramResult {
    let keys: FillOrderKeys = accounts.into();
    let ix = fill_order_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn fill_order_invoke(
    accounts: FillOrderAccounts<'_, '_>,
    args: FillOrderIxArgs,
) -> ProgramResult {
    fill_order_invoke_with_program_id(LIMIT_ORDER_PROGRAM_ID, accounts, args)
}
pub fn fill_order_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: FillOrderAccounts<'_, '_>,
    args: FillOrderIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: FillOrderKeys = accounts.into();
    let ix = fill_order_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn fill_order_invoke_signed(
    accounts: FillOrderAccounts<'_, '_>,
    args: FillOrderIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    fill_order_invoke_signed_with_program_id(
        LIMIT_ORDER_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn fill_order_verify_account_keys(
    accounts: FillOrderAccounts<'_, '_>,
    keys: FillOrderKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.order.key, keys.order),
        (*accounts.reserve.key, keys.reserve),
        (*accounts.maker.key, keys.maker),
        (*accounts.taker.key, keys.taker),
        (*accounts.taker_output_account.key, keys.taker_output_account),
        (*accounts.maker_output_account.key, keys.maker_output_account),
        (*accounts.taker_input_account.key, keys.taker_input_account),
        (*accounts.fee_authority.key, keys.fee_authority),
        (*accounts.program_fee_account.key, keys.program_fee_account),
        (*accounts.referral.key, keys.referral),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn fill_order_verify_writable_privileges<'me, 'info>(
    accounts: FillOrderAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.order,
        accounts.reserve,
        accounts.maker,
        accounts.taker_output_account,
        accounts.maker_output_account,
        accounts.taker_input_account,
        accounts.program_fee_account,
        accounts.referral,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn fill_order_verify_signer_privileges<'me, 'info>(
    accounts: FillOrderAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.taker] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn fill_order_verify_account_privileges<'me, 'info>(
    accounts: FillOrderAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    fill_order_verify_writable_privileges(accounts)?;
    fill_order_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const PRE_FLASH_FILL_ORDER_IX_ACCOUNTS_LEN: usize = 8;
#[derive(Copy, Clone, Debug)]
pub struct PreFlashFillOrderAccounts<'me, 'info> {
    pub order: &'me AccountInfo<'info>,
    pub reserve: &'me AccountInfo<'info>,
    pub taker: &'me AccountInfo<'info>,
    pub taker_output_account: &'me AccountInfo<'info>,
    pub input_mint: &'me AccountInfo<'info>,
    pub input_mint_token_program: &'me AccountInfo<'info>,
    pub instruction: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct PreFlashFillOrderKeys {
    pub order: Pubkey,
    pub reserve: Pubkey,
    pub taker: Pubkey,
    pub taker_output_account: Pubkey,
    pub input_mint: Pubkey,
    pub input_mint_token_program: Pubkey,
    pub instruction: Pubkey,
    pub system_program: Pubkey,
}
impl From<PreFlashFillOrderAccounts<'_, '_>> for PreFlashFillOrderKeys {
    fn from(accounts: PreFlashFillOrderAccounts) -> Self {
        Self {
            order: *accounts.order.key,
            reserve: *accounts.reserve.key,
            taker: *accounts.taker.key,
            taker_output_account: *accounts.taker_output_account.key,
            input_mint: *accounts.input_mint.key,
            input_mint_token_program: *accounts.input_mint_token_program.key,
            instruction: *accounts.instruction.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<PreFlashFillOrderKeys>
for [AccountMeta; PRE_FLASH_FILL_ORDER_IX_ACCOUNTS_LEN] {
    fn from(keys: PreFlashFillOrderKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.order,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.reserve,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.taker,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.taker_output_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.input_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.input_mint_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.instruction,
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
impl From<[Pubkey; PRE_FLASH_FILL_ORDER_IX_ACCOUNTS_LEN]> for PreFlashFillOrderKeys {
    fn from(pubkeys: [Pubkey; PRE_FLASH_FILL_ORDER_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            order: pubkeys[0],
            reserve: pubkeys[1],
            taker: pubkeys[2],
            taker_output_account: pubkeys[3],
            input_mint: pubkeys[4],
            input_mint_token_program: pubkeys[5],
            instruction: pubkeys[6],
            system_program: pubkeys[7],
        }
    }
}
impl<'info> From<PreFlashFillOrderAccounts<'_, 'info>>
for [AccountInfo<'info>; PRE_FLASH_FILL_ORDER_IX_ACCOUNTS_LEN] {
    fn from(accounts: PreFlashFillOrderAccounts<'_, 'info>) -> Self {
        [
            accounts.order.clone(),
            accounts.reserve.clone(),
            accounts.taker.clone(),
            accounts.taker_output_account.clone(),
            accounts.input_mint.clone(),
            accounts.input_mint_token_program.clone(),
            accounts.instruction.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; PRE_FLASH_FILL_ORDER_IX_ACCOUNTS_LEN]>
for PreFlashFillOrderAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; PRE_FLASH_FILL_ORDER_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            order: &arr[0],
            reserve: &arr[1],
            taker: &arr[2],
            taker_output_account: &arr[3],
            input_mint: &arr[4],
            input_mint_token_program: &arr[5],
            instruction: &arr[6],
            system_program: &arr[7],
        }
    }
}
pub const PRE_FLASH_FILL_ORDER_IX_DISCM: [u8; 8usize] = [
    240, 47, 153, 68, 13, 190, 225, 42,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PreFlashFillOrderIxArgs {
    pub making_amount: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct PreFlashFillOrderIxData(pub PreFlashFillOrderIxArgs);
impl From<PreFlashFillOrderIxArgs> for PreFlashFillOrderIxData {
    fn from(args: PreFlashFillOrderIxArgs) -> Self {
        Self(args)
    }
}
impl PreFlashFillOrderIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != PRE_FLASH_FILL_ORDER_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let making_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(PreFlashFillOrderIxArgs {
                making_amount,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&PRE_FLASH_FILL_ORDER_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.making_amount, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn pre_flash_fill_order_ix_with_program_id(
    program_id: Pubkey,
    keys: PreFlashFillOrderKeys,
    args: PreFlashFillOrderIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; PRE_FLASH_FILL_ORDER_IX_ACCOUNTS_LEN] = keys.into();
    let data: PreFlashFillOrderIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn pre_flash_fill_order_ix(
    keys: PreFlashFillOrderKeys,
    args: PreFlashFillOrderIxArgs,
) -> std::io::Result<Instruction> {
    pre_flash_fill_order_ix_with_program_id(LIMIT_ORDER_PROGRAM_ID, keys, args)
}
pub fn pre_flash_fill_order_invoke_with_program_id(
    program_id: Pubkey,
    accounts: PreFlashFillOrderAccounts<'_, '_>,
    args: PreFlashFillOrderIxArgs,
) -> ProgramResult {
    let keys: PreFlashFillOrderKeys = accounts.into();
    let ix = pre_flash_fill_order_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn pre_flash_fill_order_invoke(
    accounts: PreFlashFillOrderAccounts<'_, '_>,
    args: PreFlashFillOrderIxArgs,
) -> ProgramResult {
    pre_flash_fill_order_invoke_with_program_id(LIMIT_ORDER_PROGRAM_ID, accounts, args)
}
pub fn pre_flash_fill_order_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: PreFlashFillOrderAccounts<'_, '_>,
    args: PreFlashFillOrderIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: PreFlashFillOrderKeys = accounts.into();
    let ix = pre_flash_fill_order_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn pre_flash_fill_order_invoke_signed(
    accounts: PreFlashFillOrderAccounts<'_, '_>,
    args: PreFlashFillOrderIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    pre_flash_fill_order_invoke_signed_with_program_id(
        LIMIT_ORDER_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn pre_flash_fill_order_verify_account_keys(
    accounts: PreFlashFillOrderAccounts<'_, '_>,
    keys: PreFlashFillOrderKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.order.key, keys.order),
        (*accounts.reserve.key, keys.reserve),
        (*accounts.taker.key, keys.taker),
        (*accounts.taker_output_account.key, keys.taker_output_account),
        (*accounts.input_mint.key, keys.input_mint),
        (*accounts.input_mint_token_program.key, keys.input_mint_token_program),
        (*accounts.instruction.key, keys.instruction),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn pre_flash_fill_order_verify_writable_privileges<'me, 'info>(
    accounts: PreFlashFillOrderAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.order,
        accounts.reserve,
        accounts.taker_output_account,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn pre_flash_fill_order_verify_signer_privileges<'me, 'info>(
    accounts: PreFlashFillOrderAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.taker] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn pre_flash_fill_order_verify_account_privileges<'me, 'info>(
    accounts: PreFlashFillOrderAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    pre_flash_fill_order_verify_writable_privileges(accounts)?;
    pre_flash_fill_order_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const FLASH_FILL_ORDER_IX_ACCOUNTS_LEN: usize = 14;
#[derive(Copy, Clone, Debug)]
pub struct FlashFillOrderAccounts<'me, 'info> {
    pub order: &'me AccountInfo<'info>,
    pub reserve: &'me AccountInfo<'info>,
    pub maker: &'me AccountInfo<'info>,
    pub taker: &'me AccountInfo<'info>,
    pub maker_output_account: &'me AccountInfo<'info>,
    pub taker_input_account: &'me AccountInfo<'info>,
    pub fee_authority: &'me AccountInfo<'info>,
    pub program_fee_account: &'me AccountInfo<'info>,
    pub referral: &'me AccountInfo<'info>,
    pub input_mint: &'me AccountInfo<'info>,
    pub input_mint_token_program: &'me AccountInfo<'info>,
    pub output_mint: &'me AccountInfo<'info>,
    pub output_mint_token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FlashFillOrderKeys {
    pub order: Pubkey,
    pub reserve: Pubkey,
    pub maker: Pubkey,
    pub taker: Pubkey,
    pub maker_output_account: Pubkey,
    pub taker_input_account: Pubkey,
    pub fee_authority: Pubkey,
    pub program_fee_account: Pubkey,
    pub referral: Pubkey,
    pub input_mint: Pubkey,
    pub input_mint_token_program: Pubkey,
    pub output_mint: Pubkey,
    pub output_mint_token_program: Pubkey,
    pub system_program: Pubkey,
}
impl From<FlashFillOrderAccounts<'_, '_>> for FlashFillOrderKeys {
    fn from(accounts: FlashFillOrderAccounts) -> Self {
        Self {
            order: *accounts.order.key,
            reserve: *accounts.reserve.key,
            maker: *accounts.maker.key,
            taker: *accounts.taker.key,
            maker_output_account: *accounts.maker_output_account.key,
            taker_input_account: *accounts.taker_input_account.key,
            fee_authority: *accounts.fee_authority.key,
            program_fee_account: *accounts.program_fee_account.key,
            referral: *accounts.referral.key,
            input_mint: *accounts.input_mint.key,
            input_mint_token_program: *accounts.input_mint_token_program.key,
            output_mint: *accounts.output_mint.key,
            output_mint_token_program: *accounts.output_mint_token_program.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<FlashFillOrderKeys> for [AccountMeta; FLASH_FILL_ORDER_IX_ACCOUNTS_LEN] {
    fn from(keys: FlashFillOrderKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.order,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.reserve,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.maker,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.taker,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.maker_output_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.taker_input_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.fee_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.program_fee_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.referral,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.input_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.input_mint_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.output_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.output_mint_token_program,
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
impl From<[Pubkey; FLASH_FILL_ORDER_IX_ACCOUNTS_LEN]> for FlashFillOrderKeys {
    fn from(pubkeys: [Pubkey; FLASH_FILL_ORDER_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            order: pubkeys[0],
            reserve: pubkeys[1],
            maker: pubkeys[2],
            taker: pubkeys[3],
            maker_output_account: pubkeys[4],
            taker_input_account: pubkeys[5],
            fee_authority: pubkeys[6],
            program_fee_account: pubkeys[7],
            referral: pubkeys[8],
            input_mint: pubkeys[9],
            input_mint_token_program: pubkeys[10],
            output_mint: pubkeys[11],
            output_mint_token_program: pubkeys[12],
            system_program: pubkeys[13],
        }
    }
}
impl<'info> From<FlashFillOrderAccounts<'_, 'info>>
for [AccountInfo<'info>; FLASH_FILL_ORDER_IX_ACCOUNTS_LEN] {
    fn from(accounts: FlashFillOrderAccounts<'_, 'info>) -> Self {
        [
            accounts.order.clone(),
            accounts.reserve.clone(),
            accounts.maker.clone(),
            accounts.taker.clone(),
            accounts.maker_output_account.clone(),
            accounts.taker_input_account.clone(),
            accounts.fee_authority.clone(),
            accounts.program_fee_account.clone(),
            accounts.referral.clone(),
            accounts.input_mint.clone(),
            accounts.input_mint_token_program.clone(),
            accounts.output_mint.clone(),
            accounts.output_mint_token_program.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; FLASH_FILL_ORDER_IX_ACCOUNTS_LEN]>
for FlashFillOrderAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; FLASH_FILL_ORDER_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            order: &arr[0],
            reserve: &arr[1],
            maker: &arr[2],
            taker: &arr[3],
            maker_output_account: &arr[4],
            taker_input_account: &arr[5],
            fee_authority: &arr[6],
            program_fee_account: &arr[7],
            referral: &arr[8],
            input_mint: &arr[9],
            input_mint_token_program: &arr[10],
            output_mint: &arr[11],
            output_mint_token_program: &arr[12],
            system_program: &arr[13],
        }
    }
}
pub const FLASH_FILL_ORDER_IX_DISCM: [u8; 8usize] = [
    252, 104, 18, 134, 164, 78, 18, 140,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct FlashFillOrderIxArgs {
    pub max_taking_amount: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct FlashFillOrderIxData(pub FlashFillOrderIxArgs);
impl From<FlashFillOrderIxArgs> for FlashFillOrderIxData {
    fn from(args: FlashFillOrderIxArgs) -> Self {
        Self(args)
    }
}
impl FlashFillOrderIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != FLASH_FILL_ORDER_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let max_taking_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(FlashFillOrderIxArgs {
                max_taking_amount,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&FLASH_FILL_ORDER_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.max_taking_amount, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn flash_fill_order_ix_with_program_id(
    program_id: Pubkey,
    keys: FlashFillOrderKeys,
    args: FlashFillOrderIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; FLASH_FILL_ORDER_IX_ACCOUNTS_LEN] = keys.into();
    let data: FlashFillOrderIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn flash_fill_order_ix(
    keys: FlashFillOrderKeys,
    args: FlashFillOrderIxArgs,
) -> std::io::Result<Instruction> {
    flash_fill_order_ix_with_program_id(LIMIT_ORDER_PROGRAM_ID, keys, args)
}
pub fn flash_fill_order_invoke_with_program_id(
    program_id: Pubkey,
    accounts: FlashFillOrderAccounts<'_, '_>,
    args: FlashFillOrderIxArgs,
) -> ProgramResult {
    let keys: FlashFillOrderKeys = accounts.into();
    let ix = flash_fill_order_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn flash_fill_order_invoke(
    accounts: FlashFillOrderAccounts<'_, '_>,
    args: FlashFillOrderIxArgs,
) -> ProgramResult {
    flash_fill_order_invoke_with_program_id(LIMIT_ORDER_PROGRAM_ID, accounts, args)
}
pub fn flash_fill_order_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: FlashFillOrderAccounts<'_, '_>,
    args: FlashFillOrderIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: FlashFillOrderKeys = accounts.into();
    let ix = flash_fill_order_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn flash_fill_order_invoke_signed(
    accounts: FlashFillOrderAccounts<'_, '_>,
    args: FlashFillOrderIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    flash_fill_order_invoke_signed_with_program_id(
        LIMIT_ORDER_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn flash_fill_order_verify_account_keys(
    accounts: FlashFillOrderAccounts<'_, '_>,
    keys: FlashFillOrderKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.order.key, keys.order),
        (*accounts.reserve.key, keys.reserve),
        (*accounts.maker.key, keys.maker),
        (*accounts.taker.key, keys.taker),
        (*accounts.maker_output_account.key, keys.maker_output_account),
        (*accounts.taker_input_account.key, keys.taker_input_account),
        (*accounts.fee_authority.key, keys.fee_authority),
        (*accounts.program_fee_account.key, keys.program_fee_account),
        (*accounts.referral.key, keys.referral),
        (*accounts.input_mint.key, keys.input_mint),
        (*accounts.input_mint_token_program.key, keys.input_mint_token_program),
        (*accounts.output_mint.key, keys.output_mint),
        (*accounts.output_mint_token_program.key, keys.output_mint_token_program),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn flash_fill_order_verify_writable_privileges<'me, 'info>(
    accounts: FlashFillOrderAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.order,
        accounts.reserve,
        accounts.maker,
        accounts.maker_output_account,
        accounts.taker_input_account,
        accounts.program_fee_account,
        accounts.referral,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn flash_fill_order_verify_signer_privileges<'me, 'info>(
    accounts: FlashFillOrderAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.taker] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn flash_fill_order_verify_account_privileges<'me, 'info>(
    accounts: FlashFillOrderAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    flash_fill_order_verify_writable_privileges(accounts)?;
    flash_fill_order_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CANCEL_ORDER_IX_ACCOUNTS_LEN: usize = 7;
#[derive(Copy, Clone, Debug)]
pub struct CancelOrderAccounts<'me, 'info> {
    pub order: &'me AccountInfo<'info>,
    pub reserve: &'me AccountInfo<'info>,
    pub maker: &'me AccountInfo<'info>,
    pub maker_input_account: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub input_mint: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CancelOrderKeys {
    pub order: Pubkey,
    pub reserve: Pubkey,
    pub maker: Pubkey,
    pub maker_input_account: Pubkey,
    pub system_program: Pubkey,
    pub token_program: Pubkey,
    pub input_mint: Pubkey,
}
impl From<CancelOrderAccounts<'_, '_>> for CancelOrderKeys {
    fn from(accounts: CancelOrderAccounts) -> Self {
        Self {
            order: *accounts.order.key,
            reserve: *accounts.reserve.key,
            maker: *accounts.maker.key,
            maker_input_account: *accounts.maker_input_account.key,
            system_program: *accounts.system_program.key,
            token_program: *accounts.token_program.key,
            input_mint: *accounts.input_mint.key,
        }
    }
}
impl From<CancelOrderKeys> for [AccountMeta; CANCEL_ORDER_IX_ACCOUNTS_LEN] {
    fn from(keys: CancelOrderKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.order,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.reserve,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.maker,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.maker_input_account,
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
                pubkey: keys.input_mint,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; CANCEL_ORDER_IX_ACCOUNTS_LEN]> for CancelOrderKeys {
    fn from(pubkeys: [Pubkey; CANCEL_ORDER_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            order: pubkeys[0],
            reserve: pubkeys[1],
            maker: pubkeys[2],
            maker_input_account: pubkeys[3],
            system_program: pubkeys[4],
            token_program: pubkeys[5],
            input_mint: pubkeys[6],
        }
    }
}
impl<'info> From<CancelOrderAccounts<'_, 'info>>
for [AccountInfo<'info>; CANCEL_ORDER_IX_ACCOUNTS_LEN] {
    fn from(accounts: CancelOrderAccounts<'_, 'info>) -> Self {
        [
            accounts.order.clone(),
            accounts.reserve.clone(),
            accounts.maker.clone(),
            accounts.maker_input_account.clone(),
            accounts.system_program.clone(),
            accounts.token_program.clone(),
            accounts.input_mint.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CANCEL_ORDER_IX_ACCOUNTS_LEN]>
for CancelOrderAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; CANCEL_ORDER_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            order: &arr[0],
            reserve: &arr[1],
            maker: &arr[2],
            maker_input_account: &arr[3],
            system_program: &arr[4],
            token_program: &arr[5],
            input_mint: &arr[6],
        }
    }
}
pub const CANCEL_ORDER_IX_DISCM: [u8; 8usize] = [95, 129, 237, 240, 8, 49, 223, 132];
#[derive(Clone, Debug, PartialEq)]
pub struct CancelOrderIxData;
impl CancelOrderIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CANCEL_ORDER_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CANCEL_ORDER_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn cancel_order_ix_with_program_id(
    program_id: Pubkey,
    keys: CancelOrderKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CANCEL_ORDER_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: CancelOrderIxData.try_to_vec()?,
    })
}
pub fn cancel_order_ix(keys: CancelOrderKeys) -> std::io::Result<Instruction> {
    cancel_order_ix_with_program_id(LIMIT_ORDER_PROGRAM_ID, keys)
}
pub fn cancel_order_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CancelOrderAccounts<'_, '_>,
) -> ProgramResult {
    let keys: CancelOrderKeys = accounts.into();
    let ix = cancel_order_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn cancel_order_invoke(accounts: CancelOrderAccounts<'_, '_>) -> ProgramResult {
    cancel_order_invoke_with_program_id(LIMIT_ORDER_PROGRAM_ID, accounts)
}
pub fn cancel_order_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CancelOrderAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CancelOrderKeys = accounts.into();
    let ix = cancel_order_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn cancel_order_invoke_signed(
    accounts: CancelOrderAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    cancel_order_invoke_signed_with_program_id(LIMIT_ORDER_PROGRAM_ID, accounts, seeds)
}
pub fn cancel_order_verify_account_keys(
    accounts: CancelOrderAccounts<'_, '_>,
    keys: CancelOrderKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.order.key, keys.order),
        (*accounts.reserve.key, keys.reserve),
        (*accounts.maker.key, keys.maker),
        (*accounts.maker_input_account.key, keys.maker_input_account),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.input_mint.key, keys.input_mint),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn cancel_order_verify_writable_privileges<'me, 'info>(
    accounts: CancelOrderAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.order,
        accounts.reserve,
        accounts.maker,
        accounts.maker_input_account,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn cancel_order_verify_signer_privileges<'me, 'info>(
    accounts: CancelOrderAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.maker] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn cancel_order_verify_account_privileges<'me, 'info>(
    accounts: CancelOrderAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    cancel_order_verify_writable_privileges(accounts)?;
    cancel_order_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CANCEL_EXPIRED_ORDER_IX_ACCOUNTS_LEN: usize = 7;
#[derive(Copy, Clone, Debug)]
pub struct CancelExpiredOrderAccounts<'me, 'info> {
    pub order: &'me AccountInfo<'info>,
    pub reserve: &'me AccountInfo<'info>,
    pub maker: &'me AccountInfo<'info>,
    pub maker_input_account: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub input_mint: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CancelExpiredOrderKeys {
    pub order: Pubkey,
    pub reserve: Pubkey,
    pub maker: Pubkey,
    pub maker_input_account: Pubkey,
    pub system_program: Pubkey,
    pub token_program: Pubkey,
    pub input_mint: Pubkey,
}
impl From<CancelExpiredOrderAccounts<'_, '_>> for CancelExpiredOrderKeys {
    fn from(accounts: CancelExpiredOrderAccounts) -> Self {
        Self {
            order: *accounts.order.key,
            reserve: *accounts.reserve.key,
            maker: *accounts.maker.key,
            maker_input_account: *accounts.maker_input_account.key,
            system_program: *accounts.system_program.key,
            token_program: *accounts.token_program.key,
            input_mint: *accounts.input_mint.key,
        }
    }
}
impl From<CancelExpiredOrderKeys>
for [AccountMeta; CANCEL_EXPIRED_ORDER_IX_ACCOUNTS_LEN] {
    fn from(keys: CancelExpiredOrderKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.order,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.reserve,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.maker,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.maker_input_account,
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
                pubkey: keys.input_mint,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; CANCEL_EXPIRED_ORDER_IX_ACCOUNTS_LEN]> for CancelExpiredOrderKeys {
    fn from(pubkeys: [Pubkey; CANCEL_EXPIRED_ORDER_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            order: pubkeys[0],
            reserve: pubkeys[1],
            maker: pubkeys[2],
            maker_input_account: pubkeys[3],
            system_program: pubkeys[4],
            token_program: pubkeys[5],
            input_mint: pubkeys[6],
        }
    }
}
impl<'info> From<CancelExpiredOrderAccounts<'_, 'info>>
for [AccountInfo<'info>; CANCEL_EXPIRED_ORDER_IX_ACCOUNTS_LEN] {
    fn from(accounts: CancelExpiredOrderAccounts<'_, 'info>) -> Self {
        [
            accounts.order.clone(),
            accounts.reserve.clone(),
            accounts.maker.clone(),
            accounts.maker_input_account.clone(),
            accounts.system_program.clone(),
            accounts.token_program.clone(),
            accounts.input_mint.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CANCEL_EXPIRED_ORDER_IX_ACCOUNTS_LEN]>
for CancelExpiredOrderAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; CANCEL_EXPIRED_ORDER_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            order: &arr[0],
            reserve: &arr[1],
            maker: &arr[2],
            maker_input_account: &arr[3],
            system_program: &arr[4],
            token_program: &arr[5],
            input_mint: &arr[6],
        }
    }
}
pub const CANCEL_EXPIRED_ORDER_IX_DISCM: [u8; 8usize] = [
    216, 120, 64, 235, 155, 19, 229, 99,
];
#[derive(Clone, Debug, PartialEq)]
pub struct CancelExpiredOrderIxData;
impl CancelExpiredOrderIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CANCEL_EXPIRED_ORDER_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CANCEL_EXPIRED_ORDER_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn cancel_expired_order_ix_with_program_id(
    program_id: Pubkey,
    keys: CancelExpiredOrderKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CANCEL_EXPIRED_ORDER_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: CancelExpiredOrderIxData.try_to_vec()?,
    })
}
pub fn cancel_expired_order_ix(
    keys: CancelExpiredOrderKeys,
) -> std::io::Result<Instruction> {
    cancel_expired_order_ix_with_program_id(LIMIT_ORDER_PROGRAM_ID, keys)
}
pub fn cancel_expired_order_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CancelExpiredOrderAccounts<'_, '_>,
) -> ProgramResult {
    let keys: CancelExpiredOrderKeys = accounts.into();
    let ix = cancel_expired_order_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn cancel_expired_order_invoke(
    accounts: CancelExpiredOrderAccounts<'_, '_>,
) -> ProgramResult {
    cancel_expired_order_invoke_with_program_id(LIMIT_ORDER_PROGRAM_ID, accounts)
}
pub fn cancel_expired_order_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CancelExpiredOrderAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CancelExpiredOrderKeys = accounts.into();
    let ix = cancel_expired_order_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn cancel_expired_order_invoke_signed(
    accounts: CancelExpiredOrderAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    cancel_expired_order_invoke_signed_with_program_id(
        LIMIT_ORDER_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn cancel_expired_order_verify_account_keys(
    accounts: CancelExpiredOrderAccounts<'_, '_>,
    keys: CancelExpiredOrderKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.order.key, keys.order),
        (*accounts.reserve.key, keys.reserve),
        (*accounts.maker.key, keys.maker),
        (*accounts.maker_input_account.key, keys.maker_input_account),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.input_mint.key, keys.input_mint),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn cancel_expired_order_verify_writable_privileges<'me, 'info>(
    accounts: CancelExpiredOrderAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.order,
        accounts.reserve,
        accounts.maker,
        accounts.maker_input_account,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn cancel_expired_order_verify_account_privileges<'me, 'info>(
    accounts: CancelExpiredOrderAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    cancel_expired_order_verify_writable_privileges(accounts)?;
    Ok(())
}
pub const WITHDRAW_FEE_IX_ACCOUNTS_LEN: usize = 6;
#[derive(Copy, Clone, Debug)]
pub struct WithdrawFeeAccounts<'me, 'info> {
    pub admin: &'me AccountInfo<'info>,
    pub fee_authority: &'me AccountInfo<'info>,
    pub program_fee_account: &'me AccountInfo<'info>,
    pub admin_token_acocunt: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub mint: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct WithdrawFeeKeys {
    pub admin: Pubkey,
    pub fee_authority: Pubkey,
    pub program_fee_account: Pubkey,
    pub admin_token_acocunt: Pubkey,
    pub token_program: Pubkey,
    pub mint: Pubkey,
}
impl From<WithdrawFeeAccounts<'_, '_>> for WithdrawFeeKeys {
    fn from(accounts: WithdrawFeeAccounts) -> Self {
        Self {
            admin: *accounts.admin.key,
            fee_authority: *accounts.fee_authority.key,
            program_fee_account: *accounts.program_fee_account.key,
            admin_token_acocunt: *accounts.admin_token_acocunt.key,
            token_program: *accounts.token_program.key,
            mint: *accounts.mint.key,
        }
    }
}
impl From<WithdrawFeeKeys> for [AccountMeta; WITHDRAW_FEE_IX_ACCOUNTS_LEN] {
    fn from(keys: WithdrawFeeKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.admin,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.fee_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.program_fee_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.admin_token_acocunt,
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
impl From<[Pubkey; WITHDRAW_FEE_IX_ACCOUNTS_LEN]> for WithdrawFeeKeys {
    fn from(pubkeys: [Pubkey; WITHDRAW_FEE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin: pubkeys[0],
            fee_authority: pubkeys[1],
            program_fee_account: pubkeys[2],
            admin_token_acocunt: pubkeys[3],
            token_program: pubkeys[4],
            mint: pubkeys[5],
        }
    }
}
impl<'info> From<WithdrawFeeAccounts<'_, 'info>>
for [AccountInfo<'info>; WITHDRAW_FEE_IX_ACCOUNTS_LEN] {
    fn from(accounts: WithdrawFeeAccounts<'_, 'info>) -> Self {
        [
            accounts.admin.clone(),
            accounts.fee_authority.clone(),
            accounts.program_fee_account.clone(),
            accounts.admin_token_acocunt.clone(),
            accounts.token_program.clone(),
            accounts.mint.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; WITHDRAW_FEE_IX_ACCOUNTS_LEN]>
for WithdrawFeeAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; WITHDRAW_FEE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin: &arr[0],
            fee_authority: &arr[1],
            program_fee_account: &arr[2],
            admin_token_acocunt: &arr[3],
            token_program: &arr[4],
            mint: &arr[5],
        }
    }
}
pub const WITHDRAW_FEE_IX_DISCM: [u8; 8usize] = [14, 122, 231, 218, 31, 238, 223, 150];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WithdrawFeeIxArgs {
    pub amount: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct WithdrawFeeIxData(pub WithdrawFeeIxArgs);
impl From<WithdrawFeeIxArgs> for WithdrawFeeIxData {
    fn from(args: WithdrawFeeIxArgs) -> Self {
        Self(args)
    }
}
impl WithdrawFeeIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != WITHDRAW_FEE_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(Self(WithdrawFeeIxArgs { amount }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&WITHDRAW_FEE_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.amount, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn withdraw_fee_ix_with_program_id(
    program_id: Pubkey,
    keys: WithdrawFeeKeys,
    args: WithdrawFeeIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; WITHDRAW_FEE_IX_ACCOUNTS_LEN] = keys.into();
    let data: WithdrawFeeIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn withdraw_fee_ix(
    keys: WithdrawFeeKeys,
    args: WithdrawFeeIxArgs,
) -> std::io::Result<Instruction> {
    withdraw_fee_ix_with_program_id(LIMIT_ORDER_PROGRAM_ID, keys, args)
}
pub fn withdraw_fee_invoke_with_program_id(
    program_id: Pubkey,
    accounts: WithdrawFeeAccounts<'_, '_>,
    args: WithdrawFeeIxArgs,
) -> ProgramResult {
    let keys: WithdrawFeeKeys = accounts.into();
    let ix = withdraw_fee_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn withdraw_fee_invoke(
    accounts: WithdrawFeeAccounts<'_, '_>,
    args: WithdrawFeeIxArgs,
) -> ProgramResult {
    withdraw_fee_invoke_with_program_id(LIMIT_ORDER_PROGRAM_ID, accounts, args)
}
pub fn withdraw_fee_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: WithdrawFeeAccounts<'_, '_>,
    args: WithdrawFeeIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: WithdrawFeeKeys = accounts.into();
    let ix = withdraw_fee_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn withdraw_fee_invoke_signed(
    accounts: WithdrawFeeAccounts<'_, '_>,
    args: WithdrawFeeIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    withdraw_fee_invoke_signed_with_program_id(
        LIMIT_ORDER_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn withdraw_fee_verify_account_keys(
    accounts: WithdrawFeeAccounts<'_, '_>,
    keys: WithdrawFeeKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.admin.key, keys.admin),
        (*accounts.fee_authority.key, keys.fee_authority),
        (*accounts.program_fee_account.key, keys.program_fee_account),
        (*accounts.admin_token_acocunt.key, keys.admin_token_acocunt),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.mint.key, keys.mint),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn withdraw_fee_verify_writable_privileges<'me, 'info>(
    accounts: WithdrawFeeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.admin,
        accounts.program_fee_account,
        accounts.admin_token_acocunt,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn withdraw_fee_verify_signer_privileges<'me, 'info>(
    accounts: WithdrawFeeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn withdraw_fee_verify_account_privileges<'me, 'info>(
    accounts: WithdrawFeeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    withdraw_fee_verify_writable_privileges(accounts)?;
    withdraw_fee_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const INIT_FEE_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct InitFeeAccounts<'me, 'info> {
    pub keeper: &'me AccountInfo<'info>,
    pub fee_authority: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct InitFeeKeys {
    pub keeper: Pubkey,
    pub fee_authority: Pubkey,
    pub system_program: Pubkey,
}
impl From<InitFeeAccounts<'_, '_>> for InitFeeKeys {
    fn from(accounts: InitFeeAccounts) -> Self {
        Self {
            keeper: *accounts.keeper.key,
            fee_authority: *accounts.fee_authority.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<InitFeeKeys> for [AccountMeta; INIT_FEE_IX_ACCOUNTS_LEN] {
    fn from(keys: InitFeeKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.keeper,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.fee_authority,
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
impl From<[Pubkey; INIT_FEE_IX_ACCOUNTS_LEN]> for InitFeeKeys {
    fn from(pubkeys: [Pubkey; INIT_FEE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            keeper: pubkeys[0],
            fee_authority: pubkeys[1],
            system_program: pubkeys[2],
        }
    }
}
impl<'info> From<InitFeeAccounts<'_, 'info>>
for [AccountInfo<'info>; INIT_FEE_IX_ACCOUNTS_LEN] {
    fn from(accounts: InitFeeAccounts<'_, 'info>) -> Self {
        [
            accounts.keeper.clone(),
            accounts.fee_authority.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; INIT_FEE_IX_ACCOUNTS_LEN]>
for InitFeeAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; INIT_FEE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            keeper: &arr[0],
            fee_authority: &arr[1],
            system_program: &arr[2],
        }
    }
}
pub const INIT_FEE_IX_DISCM: [u8; 8usize] = [13, 9, 211, 107, 62, 172, 224, 67];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct InitFeeIxArgs {
    pub maker_fee: u64,
    pub maker_stable_fee: u64,
    pub taker_fee: u64,
    pub taker_stable_fee: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct InitFeeIxData(pub InitFeeIxArgs);
impl From<InitFeeIxArgs> for InitFeeIxData {
    fn from(args: InitFeeIxArgs) -> Self {
        Self(args)
    }
}
impl InitFeeIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INIT_FEE_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let maker_fee: u64 = crate::borsh_de_or_default(&mut reader)?;
        let maker_stable_fee: u64 = crate::borsh_de_or_default(&mut reader)?;
        let taker_fee: u64 = crate::borsh_de_or_default(&mut reader)?;
        let taker_stable_fee: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(InitFeeIxArgs {
                maker_fee,
                maker_stable_fee,
                taker_fee,
                taker_stable_fee,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INIT_FEE_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.maker_fee, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.maker_stable_fee, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.taker_fee, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.taker_stable_fee, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn init_fee_ix_with_program_id(
    program_id: Pubkey,
    keys: InitFeeKeys,
    args: InitFeeIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; INIT_FEE_IX_ACCOUNTS_LEN] = keys.into();
    let data: InitFeeIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn init_fee_ix(
    keys: InitFeeKeys,
    args: InitFeeIxArgs,
) -> std::io::Result<Instruction> {
    init_fee_ix_with_program_id(LIMIT_ORDER_PROGRAM_ID, keys, args)
}
pub fn init_fee_invoke_with_program_id(
    program_id: Pubkey,
    accounts: InitFeeAccounts<'_, '_>,
    args: InitFeeIxArgs,
) -> ProgramResult {
    let keys: InitFeeKeys = accounts.into();
    let ix = init_fee_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn init_fee_invoke(
    accounts: InitFeeAccounts<'_, '_>,
    args: InitFeeIxArgs,
) -> ProgramResult {
    init_fee_invoke_with_program_id(LIMIT_ORDER_PROGRAM_ID, accounts, args)
}
pub fn init_fee_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: InitFeeAccounts<'_, '_>,
    args: InitFeeIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: InitFeeKeys = accounts.into();
    let ix = init_fee_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn init_fee_invoke_signed(
    accounts: InitFeeAccounts<'_, '_>,
    args: InitFeeIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    init_fee_invoke_signed_with_program_id(LIMIT_ORDER_PROGRAM_ID, accounts, args, seeds)
}
pub fn init_fee_verify_account_keys(
    accounts: InitFeeAccounts<'_, '_>,
    keys: InitFeeKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.keeper.key, keys.keeper),
        (*accounts.fee_authority.key, keys.fee_authority),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn init_fee_verify_writable_privileges<'me, 'info>(
    accounts: InitFeeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.keeper, accounts.fee_authority] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn init_fee_verify_signer_privileges<'me, 'info>(
    accounts: InitFeeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.keeper] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn init_fee_verify_account_privileges<'me, 'info>(
    accounts: InitFeeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    init_fee_verify_writable_privileges(accounts)?;
    init_fee_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const UPDATE_FEE_IX_ACCOUNTS_LEN: usize = 2;
#[derive(Copy, Clone, Debug)]
pub struct UpdateFeeAccounts<'me, 'info> {
    pub keeper: &'me AccountInfo<'info>,
    pub fee_authority: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UpdateFeeKeys {
    pub keeper: Pubkey,
    pub fee_authority: Pubkey,
}
impl From<UpdateFeeAccounts<'_, '_>> for UpdateFeeKeys {
    fn from(accounts: UpdateFeeAccounts) -> Self {
        Self {
            keeper: *accounts.keeper.key,
            fee_authority: *accounts.fee_authority.key,
        }
    }
}
impl From<UpdateFeeKeys> for [AccountMeta; UPDATE_FEE_IX_ACCOUNTS_LEN] {
    fn from(keys: UpdateFeeKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.keeper,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.fee_authority,
                is_signer: false,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; UPDATE_FEE_IX_ACCOUNTS_LEN]> for UpdateFeeKeys {
    fn from(pubkeys: [Pubkey; UPDATE_FEE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            keeper: pubkeys[0],
            fee_authority: pubkeys[1],
        }
    }
}
impl<'info> From<UpdateFeeAccounts<'_, 'info>>
for [AccountInfo<'info>; UPDATE_FEE_IX_ACCOUNTS_LEN] {
    fn from(accounts: UpdateFeeAccounts<'_, 'info>) -> Self {
        [accounts.keeper.clone(), accounts.fee_authority.clone()]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; UPDATE_FEE_IX_ACCOUNTS_LEN]>
for UpdateFeeAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; UPDATE_FEE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            keeper: &arr[0],
            fee_authority: &arr[1],
        }
    }
}
pub const UPDATE_FEE_IX_DISCM: [u8; 8usize] = [232, 253, 195, 247, 148, 212, 73, 222];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UpdateFeeIxArgs {
    pub maker_fee: u64,
    pub maker_stable_fee: u64,
    pub taker_fee: u64,
    pub taker_stable_fee: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateFeeIxData(pub UpdateFeeIxArgs);
impl From<UpdateFeeIxArgs> for UpdateFeeIxData {
    fn from(args: UpdateFeeIxArgs) -> Self {
        Self(args)
    }
}
impl UpdateFeeIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_FEE_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let maker_fee: u64 = crate::borsh_de_or_default(&mut reader)?;
        let maker_stable_fee: u64 = crate::borsh_de_or_default(&mut reader)?;
        let taker_fee: u64 = crate::borsh_de_or_default(&mut reader)?;
        let taker_stable_fee: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(UpdateFeeIxArgs {
                maker_fee,
                maker_stable_fee,
                taker_fee,
                taker_stable_fee,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_FEE_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.maker_fee, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.maker_stable_fee, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.taker_fee, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.taker_stable_fee, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn update_fee_ix_with_program_id(
    program_id: Pubkey,
    keys: UpdateFeeKeys,
    args: UpdateFeeIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; UPDATE_FEE_IX_ACCOUNTS_LEN] = keys.into();
    let data: UpdateFeeIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_fee_ix(
    keys: UpdateFeeKeys,
    args: UpdateFeeIxArgs,
) -> std::io::Result<Instruction> {
    update_fee_ix_with_program_id(LIMIT_ORDER_PROGRAM_ID, keys, args)
}
pub fn update_fee_invoke_with_program_id(
    program_id: Pubkey,
    accounts: UpdateFeeAccounts<'_, '_>,
    args: UpdateFeeIxArgs,
) -> ProgramResult {
    let keys: UpdateFeeKeys = accounts.into();
    let ix = update_fee_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn update_fee_invoke(
    accounts: UpdateFeeAccounts<'_, '_>,
    args: UpdateFeeIxArgs,
) -> ProgramResult {
    update_fee_invoke_with_program_id(LIMIT_ORDER_PROGRAM_ID, accounts, args)
}
pub fn update_fee_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: UpdateFeeAccounts<'_, '_>,
    args: UpdateFeeIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: UpdateFeeKeys = accounts.into();
    let ix = update_fee_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn update_fee_invoke_signed(
    accounts: UpdateFeeAccounts<'_, '_>,
    args: UpdateFeeIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    update_fee_invoke_signed_with_program_id(
        LIMIT_ORDER_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn update_fee_verify_account_keys(
    accounts: UpdateFeeAccounts<'_, '_>,
    keys: UpdateFeeKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.keeper.key, keys.keeper),
        (*accounts.fee_authority.key, keys.fee_authority),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn update_fee_verify_writable_privileges<'me, 'info>(
    accounts: UpdateFeeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.keeper, accounts.fee_authority] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn update_fee_verify_signer_privileges<'me, 'info>(
    accounts: UpdateFeeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.keeper] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn update_fee_verify_account_privileges<'me, 'info>(
    accounts: UpdateFeeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    update_fee_verify_writable_privileges(accounts)?;
    update_fee_verify_signer_privileges(accounts)?;
    Ok(())
}
