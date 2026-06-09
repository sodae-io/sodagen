use solana_pubkey::Pubkey;
use solana_cpi::{invoke, invoke_signed};
use solana_instruction::{AccountMeta, Instruction};
use solana_account_info::AccountInfo;
use solana_program_error::ProgramError;
use std::io::Read;
#[allow(unused_imports)]
use crate::*;
#[derive(Clone, Debug, PartialEq)]
pub enum DcaProgramIx {
    OpenDca(OpenDcaIxArgs),
    OpenDcaV2(OpenDcaV2IxArgs),
    CloseDca,
    Withdraw(WithdrawIxArgs),
    Deposit(DepositIxArgs),
    WithdrawFees(WithdrawFeesIxArgs),
    InitiateFlashFill,
    FulfillFlashFill(FulfillFlashFillIxArgs),
    InitiateDlmmFill,
    FulfillDlmmFill(FulfillDlmmFillIxArgs),
    Transfer,
    EndAndClose,
}
impl DcaProgramIx {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        if buf.starts_with(&OPEN_DCA_IX_DISCM) {
            let mut reader = &buf[OPEN_DCA_IX_DISCM.len()..];
            let application_idx: u64 = crate::borsh_de_or_default(&mut reader)?;
            let in_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            let in_amount_per_cycle: u64 = crate::borsh_de_or_default(&mut reader)?;
            let cycle_frequency: i64 = crate::borsh_de_or_default(&mut reader)?;
            let min_out_amount: Option<u64> = crate::borsh_de_or_default(&mut reader)?;
            let max_out_amount: Option<u64> = crate::borsh_de_or_default(&mut reader)?;
            let start_at: Option<i64> = crate::borsh_de_or_default(&mut reader)?;
            let close_wsol_in_ata: Option<bool> = crate::borsh_de_or_default(
                &mut reader,
            )?;
            return Ok(
                Self::OpenDca(OpenDcaIxArgs {
                    application_idx,
                    in_amount,
                    in_amount_per_cycle,
                    cycle_frequency,
                    min_out_amount,
                    max_out_amount,
                    start_at,
                    close_wsol_in_ata,
                }),
            );
        }
        if buf.starts_with(&OPEN_DCA_V2_IX_DISCM) {
            let mut reader = &buf[OPEN_DCA_V2_IX_DISCM.len()..];
            let application_idx: u64 = crate::borsh_de_or_default(&mut reader)?;
            let in_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            let in_amount_per_cycle: u64 = crate::borsh_de_or_default(&mut reader)?;
            let cycle_frequency: i64 = crate::borsh_de_or_default(&mut reader)?;
            let min_out_amount: Option<u64> = crate::borsh_de_or_default(&mut reader)?;
            let max_out_amount: Option<u64> = crate::borsh_de_or_default(&mut reader)?;
            let start_at: Option<i64> = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::OpenDcaV2(OpenDcaV2IxArgs {
                    application_idx,
                    in_amount,
                    in_amount_per_cycle,
                    cycle_frequency,
                    min_out_amount,
                    max_out_amount,
                    start_at,
                }),
            );
        }
        if buf.starts_with(&CLOSE_DCA_IX_DISCM) {
            return Ok(Self::CloseDca);
        }
        if buf.starts_with(&WITHDRAW_IX_DISCM) {
            let mut reader = &buf[WITHDRAW_IX_DISCM.len()..];
            let withdraw_params = if reader.is_empty() {
                Default::default()
            } else {
                <WithdrawParams>::deserialize(&mut reader)?
            };
            return Ok(Self::Withdraw(WithdrawIxArgs { withdraw_params }));
        }
        if buf.starts_with(&DEPOSIT_IX_DISCM) {
            let mut reader = &buf[DEPOSIT_IX_DISCM.len()..];
            let deposit_in: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(Self::Deposit(DepositIxArgs { deposit_in }));
        }
        if buf.starts_with(&WITHDRAW_FEES_IX_DISCM) {
            let mut reader = &buf[WITHDRAW_FEES_IX_DISCM.len()..];
            let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(Self::WithdrawFees(WithdrawFeesIxArgs { amount }));
        }
        if buf.starts_with(&INITIATE_FLASH_FILL_IX_DISCM) {
            return Ok(Self::InitiateFlashFill);
        }
        if buf.starts_with(&FULFILL_FLASH_FILL_IX_DISCM) {
            let mut reader = &buf[FULFILL_FLASH_FILL_IX_DISCM.len()..];
            let repay_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::FulfillFlashFill(FulfillFlashFillIxArgs {
                    repay_amount,
                }),
            );
        }
        if buf.starts_with(&INITIATE_DLMM_FILL_IX_DISCM) {
            return Ok(Self::InitiateDlmmFill);
        }
        if buf.starts_with(&FULFILL_DLMM_FILL_IX_DISCM) {
            let mut reader = &buf[FULFILL_DLMM_FILL_IX_DISCM.len()..];
            let repay_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::FulfillDlmmFill(FulfillDlmmFillIxArgs {
                    repay_amount,
                }),
            );
        }
        if buf.starts_with(&TRANSFER_IX_DISCM) {
            return Ok(Self::Transfer);
        }
        if buf.starts_with(&END_AND_CLOSE_IX_DISCM) {
            return Ok(Self::EndAndClose);
        }
        Err(std::io::Error::from(std::io::ErrorKind::InvalidData))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        match self {
            Self::OpenDca(args) => {
                writer.write_all(&OPEN_DCA_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.application_idx, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.in_amount, &mut writer)?;
                borsh::BorshSerialize::serialize(
                    &args.in_amount_per_cycle,
                    &mut writer,
                )?;
                borsh::BorshSerialize::serialize(&args.cycle_frequency, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.min_out_amount, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.max_out_amount, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.start_at, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.close_wsol_in_ata, &mut writer)?;
                Ok(())
            }
            Self::OpenDcaV2(args) => {
                writer.write_all(&OPEN_DCA_V2_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.application_idx, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.in_amount, &mut writer)?;
                borsh::BorshSerialize::serialize(
                    &args.in_amount_per_cycle,
                    &mut writer,
                )?;
                borsh::BorshSerialize::serialize(&args.cycle_frequency, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.min_out_amount, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.max_out_amount, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.start_at, &mut writer)?;
                Ok(())
            }
            Self::CloseDca => writer.write_all(&CLOSE_DCA_IX_DISCM),
            Self::Withdraw(args) => {
                writer.write_all(&WITHDRAW_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.withdraw_params, &mut writer)?;
                Ok(())
            }
            Self::Deposit(args) => {
                writer.write_all(&DEPOSIT_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.deposit_in, &mut writer)?;
                Ok(())
            }
            Self::WithdrawFees(args) => {
                writer.write_all(&WITHDRAW_FEES_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.amount, &mut writer)?;
                Ok(())
            }
            Self::InitiateFlashFill => writer.write_all(&INITIATE_FLASH_FILL_IX_DISCM),
            Self::FulfillFlashFill(args) => {
                writer.write_all(&FULFILL_FLASH_FILL_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.repay_amount, &mut writer)?;
                Ok(())
            }
            Self::InitiateDlmmFill => writer.write_all(&INITIATE_DLMM_FILL_IX_DISCM),
            Self::FulfillDlmmFill(args) => {
                writer.write_all(&FULFILL_DLMM_FILL_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.repay_amount, &mut writer)?;
                Ok(())
            }
            Self::Transfer => writer.write_all(&TRANSFER_IX_DISCM),
            Self::EndAndClose => writer.write_all(&END_AND_CLOSE_IX_DISCM),
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
pub const OPEN_DCA_IX_ACCOUNTS_LEN: usize = 12;
#[derive(Copy, Clone, Debug)]
pub struct OpenDcaAccounts<'me, 'info> {
    pub dca: &'me AccountInfo<'info>,
    pub user: &'me AccountInfo<'info>,
    pub input_mint: &'me AccountInfo<'info>,
    pub output_mint: &'me AccountInfo<'info>,
    pub user_ata: &'me AccountInfo<'info>,
    pub in_ata: &'me AccountInfo<'info>,
    pub out_ata: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct OpenDcaKeys {
    pub dca: Pubkey,
    pub user: Pubkey,
    pub input_mint: Pubkey,
    pub output_mint: Pubkey,
    pub user_ata: Pubkey,
    pub in_ata: Pubkey,
    pub out_ata: Pubkey,
    pub system_program: Pubkey,
    pub token_program: Pubkey,
    pub associated_token_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<OpenDcaAccounts<'_, '_>> for OpenDcaKeys {
    fn from(accounts: OpenDcaAccounts) -> Self {
        Self {
            dca: *accounts.dca.key,
            user: *accounts.user.key,
            input_mint: *accounts.input_mint.key,
            output_mint: *accounts.output_mint.key,
            user_ata: *accounts.user_ata.key,
            in_ata: *accounts.in_ata.key,
            out_ata: *accounts.out_ata.key,
            system_program: *accounts.system_program.key,
            token_program: *accounts.token_program.key,
            associated_token_program: *accounts.associated_token_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<OpenDcaKeys> for [AccountMeta; OPEN_DCA_IX_ACCOUNTS_LEN] {
    fn from(keys: OpenDcaKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.dca,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.input_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.output_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.user_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.in_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.out_ata,
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
            AccountMeta {
                pubkey: keys.event_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; OPEN_DCA_IX_ACCOUNTS_LEN]> for OpenDcaKeys {
    fn from(pubkeys: [Pubkey; OPEN_DCA_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            dca: pubkeys[0],
            user: pubkeys[1],
            input_mint: pubkeys[2],
            output_mint: pubkeys[3],
            user_ata: pubkeys[4],
            in_ata: pubkeys[5],
            out_ata: pubkeys[6],
            system_program: pubkeys[7],
            token_program: pubkeys[8],
            associated_token_program: pubkeys[9],
            event_authority: pubkeys[10],
            program: pubkeys[11],
        }
    }
}
impl<'info> From<OpenDcaAccounts<'_, 'info>>
for [AccountInfo<'info>; OPEN_DCA_IX_ACCOUNTS_LEN] {
    fn from(accounts: OpenDcaAccounts<'_, 'info>) -> Self {
        [
            accounts.dca.clone(),
            accounts.user.clone(),
            accounts.input_mint.clone(),
            accounts.output_mint.clone(),
            accounts.user_ata.clone(),
            accounts.in_ata.clone(),
            accounts.out_ata.clone(),
            accounts.system_program.clone(),
            accounts.token_program.clone(),
            accounts.associated_token_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; OPEN_DCA_IX_ACCOUNTS_LEN]>
for OpenDcaAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; OPEN_DCA_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            dca: &arr[0],
            user: &arr[1],
            input_mint: &arr[2],
            output_mint: &arr[3],
            user_ata: &arr[4],
            in_ata: &arr[5],
            out_ata: &arr[6],
            system_program: &arr[7],
            token_program: &arr[8],
            associated_token_program: &arr[9],
            event_authority: &arr[10],
            program: &arr[11],
        }
    }
}
pub const OPEN_DCA_IX_DISCM: [u8; 8usize] = [36, 65, 185, 54, 1, 210, 100, 163];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct OpenDcaIxArgs {
    pub application_idx: u64,
    pub in_amount: u64,
    pub in_amount_per_cycle: u64,
    pub cycle_frequency: i64,
    pub min_out_amount: Option<u64>,
    pub max_out_amount: Option<u64>,
    pub start_at: Option<i64>,
    pub close_wsol_in_ata: Option<bool>,
}
#[derive(Clone, Debug, PartialEq)]
pub struct OpenDcaIxData(pub OpenDcaIxArgs);
impl From<OpenDcaIxArgs> for OpenDcaIxData {
    fn from(args: OpenDcaIxArgs) -> Self {
        Self(args)
    }
}
impl OpenDcaIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != OPEN_DCA_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let application_idx: u64 = crate::borsh_de_or_default(&mut reader)?;
        let in_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let in_amount_per_cycle: u64 = crate::borsh_de_or_default(&mut reader)?;
        let cycle_frequency: i64 = crate::borsh_de_or_default(&mut reader)?;
        let min_out_amount: Option<u64> = crate::borsh_de_or_default(&mut reader)?;
        let max_out_amount: Option<u64> = crate::borsh_de_or_default(&mut reader)?;
        let start_at: Option<i64> = crate::borsh_de_or_default(&mut reader)?;
        let close_wsol_in_ata: Option<bool> = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(OpenDcaIxArgs {
                application_idx,
                in_amount,
                in_amount_per_cycle,
                cycle_frequency,
                min_out_amount,
                max_out_amount,
                start_at,
                close_wsol_in_ata,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&OPEN_DCA_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.application_idx, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.in_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.in_amount_per_cycle, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.cycle_frequency, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.min_out_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.max_out_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.start_at, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.close_wsol_in_ata, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn open_dca_ix_with_program_id(
    program_id: Pubkey,
    keys: OpenDcaKeys,
    args: OpenDcaIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; OPEN_DCA_IX_ACCOUNTS_LEN] = keys.into();
    let data: OpenDcaIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn open_dca_ix(
    keys: OpenDcaKeys,
    args: OpenDcaIxArgs,
) -> std::io::Result<Instruction> {
    open_dca_ix_with_program_id(DCA_PROGRAM_ID, keys, args)
}
pub fn open_dca_invoke_with_program_id(
    program_id: Pubkey,
    accounts: OpenDcaAccounts<'_, '_>,
    args: OpenDcaIxArgs,
) -> ProgramResult {
    let keys: OpenDcaKeys = accounts.into();
    let ix = open_dca_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn open_dca_invoke(
    accounts: OpenDcaAccounts<'_, '_>,
    args: OpenDcaIxArgs,
) -> ProgramResult {
    open_dca_invoke_with_program_id(DCA_PROGRAM_ID, accounts, args)
}
pub fn open_dca_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: OpenDcaAccounts<'_, '_>,
    args: OpenDcaIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: OpenDcaKeys = accounts.into();
    let ix = open_dca_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn open_dca_invoke_signed(
    accounts: OpenDcaAccounts<'_, '_>,
    args: OpenDcaIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    open_dca_invoke_signed_with_program_id(DCA_PROGRAM_ID, accounts, args, seeds)
}
pub fn open_dca_verify_account_keys(
    accounts: OpenDcaAccounts<'_, '_>,
    keys: OpenDcaKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.dca.key, keys.dca),
        (*accounts.user.key, keys.user),
        (*accounts.input_mint.key, keys.input_mint),
        (*accounts.output_mint.key, keys.output_mint),
        (*accounts.user_ata.key, keys.user_ata),
        (*accounts.in_ata.key, keys.in_ata),
        (*accounts.out_ata.key, keys.out_ata),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.associated_token_program.key, keys.associated_token_program),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn open_dca_verify_writable_privileges<'me, 'info>(
    accounts: OpenDcaAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.dca,
        accounts.user,
        accounts.user_ata,
        accounts.in_ata,
        accounts.out_ata,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn open_dca_verify_signer_privileges<'me, 'info>(
    accounts: OpenDcaAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.user] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn open_dca_verify_account_privileges<'me, 'info>(
    accounts: OpenDcaAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    open_dca_verify_writable_privileges(accounts)?;
    open_dca_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const OPEN_DCA_V2_IX_ACCOUNTS_LEN: usize = 13;
#[derive(Copy, Clone, Debug)]
pub struct OpenDcaV2Accounts<'me, 'info> {
    pub dca: &'me AccountInfo<'info>,
    pub user: &'me AccountInfo<'info>,
    pub payer: &'me AccountInfo<'info>,
    pub input_mint: &'me AccountInfo<'info>,
    pub output_mint: &'me AccountInfo<'info>,
    pub user_ata: &'me AccountInfo<'info>,
    pub in_ata: &'me AccountInfo<'info>,
    pub out_ata: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct OpenDcaV2Keys {
    pub dca: Pubkey,
    pub user: Pubkey,
    pub payer: Pubkey,
    pub input_mint: Pubkey,
    pub output_mint: Pubkey,
    pub user_ata: Pubkey,
    pub in_ata: Pubkey,
    pub out_ata: Pubkey,
    pub system_program: Pubkey,
    pub token_program: Pubkey,
    pub associated_token_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<OpenDcaV2Accounts<'_, '_>> for OpenDcaV2Keys {
    fn from(accounts: OpenDcaV2Accounts) -> Self {
        Self {
            dca: *accounts.dca.key,
            user: *accounts.user.key,
            payer: *accounts.payer.key,
            input_mint: *accounts.input_mint.key,
            output_mint: *accounts.output_mint.key,
            user_ata: *accounts.user_ata.key,
            in_ata: *accounts.in_ata.key,
            out_ata: *accounts.out_ata.key,
            system_program: *accounts.system_program.key,
            token_program: *accounts.token_program.key,
            associated_token_program: *accounts.associated_token_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<OpenDcaV2Keys> for [AccountMeta; OPEN_DCA_V2_IX_ACCOUNTS_LEN] {
    fn from(keys: OpenDcaV2Keys) -> Self {
        [
            AccountMeta {
                pubkey: keys.dca,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.payer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.input_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.output_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.user_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.in_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.out_ata,
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
            AccountMeta {
                pubkey: keys.event_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; OPEN_DCA_V2_IX_ACCOUNTS_LEN]> for OpenDcaV2Keys {
    fn from(pubkeys: [Pubkey; OPEN_DCA_V2_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            dca: pubkeys[0],
            user: pubkeys[1],
            payer: pubkeys[2],
            input_mint: pubkeys[3],
            output_mint: pubkeys[4],
            user_ata: pubkeys[5],
            in_ata: pubkeys[6],
            out_ata: pubkeys[7],
            system_program: pubkeys[8],
            token_program: pubkeys[9],
            associated_token_program: pubkeys[10],
            event_authority: pubkeys[11],
            program: pubkeys[12],
        }
    }
}
impl<'info> From<OpenDcaV2Accounts<'_, 'info>>
for [AccountInfo<'info>; OPEN_DCA_V2_IX_ACCOUNTS_LEN] {
    fn from(accounts: OpenDcaV2Accounts<'_, 'info>) -> Self {
        [
            accounts.dca.clone(),
            accounts.user.clone(),
            accounts.payer.clone(),
            accounts.input_mint.clone(),
            accounts.output_mint.clone(),
            accounts.user_ata.clone(),
            accounts.in_ata.clone(),
            accounts.out_ata.clone(),
            accounts.system_program.clone(),
            accounts.token_program.clone(),
            accounts.associated_token_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; OPEN_DCA_V2_IX_ACCOUNTS_LEN]>
for OpenDcaV2Accounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; OPEN_DCA_V2_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            dca: &arr[0],
            user: &arr[1],
            payer: &arr[2],
            input_mint: &arr[3],
            output_mint: &arr[4],
            user_ata: &arr[5],
            in_ata: &arr[6],
            out_ata: &arr[7],
            system_program: &arr[8],
            token_program: &arr[9],
            associated_token_program: &arr[10],
            event_authority: &arr[11],
            program: &arr[12],
        }
    }
}
pub const OPEN_DCA_V2_IX_DISCM: [u8; 8usize] = [142, 119, 43, 109, 162, 52, 11, 177];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct OpenDcaV2IxArgs {
    pub application_idx: u64,
    pub in_amount: u64,
    pub in_amount_per_cycle: u64,
    pub cycle_frequency: i64,
    pub min_out_amount: Option<u64>,
    pub max_out_amount: Option<u64>,
    pub start_at: Option<i64>,
}
#[derive(Clone, Debug, PartialEq)]
pub struct OpenDcaV2IxData(pub OpenDcaV2IxArgs);
impl From<OpenDcaV2IxArgs> for OpenDcaV2IxData {
    fn from(args: OpenDcaV2IxArgs) -> Self {
        Self(args)
    }
}
impl OpenDcaV2IxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != OPEN_DCA_V2_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let application_idx: u64 = crate::borsh_de_or_default(&mut reader)?;
        let in_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let in_amount_per_cycle: u64 = crate::borsh_de_or_default(&mut reader)?;
        let cycle_frequency: i64 = crate::borsh_de_or_default(&mut reader)?;
        let min_out_amount: Option<u64> = crate::borsh_de_or_default(&mut reader)?;
        let max_out_amount: Option<u64> = crate::borsh_de_or_default(&mut reader)?;
        let start_at: Option<i64> = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(OpenDcaV2IxArgs {
                application_idx,
                in_amount,
                in_amount_per_cycle,
                cycle_frequency,
                min_out_amount,
                max_out_amount,
                start_at,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&OPEN_DCA_V2_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.application_idx, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.in_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.in_amount_per_cycle, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.cycle_frequency, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.min_out_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.max_out_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.start_at, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn open_dca_v2_ix_with_program_id(
    program_id: Pubkey,
    keys: OpenDcaV2Keys,
    args: OpenDcaV2IxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; OPEN_DCA_V2_IX_ACCOUNTS_LEN] = keys.into();
    let data: OpenDcaV2IxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn open_dca_v2_ix(
    keys: OpenDcaV2Keys,
    args: OpenDcaV2IxArgs,
) -> std::io::Result<Instruction> {
    open_dca_v2_ix_with_program_id(DCA_PROGRAM_ID, keys, args)
}
pub fn open_dca_v2_invoke_with_program_id(
    program_id: Pubkey,
    accounts: OpenDcaV2Accounts<'_, '_>,
    args: OpenDcaV2IxArgs,
) -> ProgramResult {
    let keys: OpenDcaV2Keys = accounts.into();
    let ix = open_dca_v2_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn open_dca_v2_invoke(
    accounts: OpenDcaV2Accounts<'_, '_>,
    args: OpenDcaV2IxArgs,
) -> ProgramResult {
    open_dca_v2_invoke_with_program_id(DCA_PROGRAM_ID, accounts, args)
}
pub fn open_dca_v2_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: OpenDcaV2Accounts<'_, '_>,
    args: OpenDcaV2IxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: OpenDcaV2Keys = accounts.into();
    let ix = open_dca_v2_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn open_dca_v2_invoke_signed(
    accounts: OpenDcaV2Accounts<'_, '_>,
    args: OpenDcaV2IxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    open_dca_v2_invoke_signed_with_program_id(DCA_PROGRAM_ID, accounts, args, seeds)
}
pub fn open_dca_v2_verify_account_keys(
    accounts: OpenDcaV2Accounts<'_, '_>,
    keys: OpenDcaV2Keys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.dca.key, keys.dca),
        (*accounts.user.key, keys.user),
        (*accounts.payer.key, keys.payer),
        (*accounts.input_mint.key, keys.input_mint),
        (*accounts.output_mint.key, keys.output_mint),
        (*accounts.user_ata.key, keys.user_ata),
        (*accounts.in_ata.key, keys.in_ata),
        (*accounts.out_ata.key, keys.out_ata),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.associated_token_program.key, keys.associated_token_program),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn open_dca_v2_verify_writable_privileges<'me, 'info>(
    accounts: OpenDcaV2Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.dca,
        accounts.payer,
        accounts.user_ata,
        accounts.in_ata,
        accounts.out_ata,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn open_dca_v2_verify_signer_privileges<'me, 'info>(
    accounts: OpenDcaV2Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.user, accounts.payer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn open_dca_v2_verify_account_privileges<'me, 'info>(
    accounts: OpenDcaV2Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    open_dca_v2_verify_writable_privileges(accounts)?;
    open_dca_v2_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CLOSE_DCA_IX_ACCOUNTS_LEN: usize = 13;
#[derive(Copy, Clone, Debug)]
pub struct CloseDcaAccounts<'me, 'info> {
    pub user: &'me AccountInfo<'info>,
    pub dca: &'me AccountInfo<'info>,
    pub input_mint: &'me AccountInfo<'info>,
    pub output_mint: &'me AccountInfo<'info>,
    pub in_ata: &'me AccountInfo<'info>,
    pub out_ata: &'me AccountInfo<'info>,
    pub user_in_ata: &'me AccountInfo<'info>,
    pub user_out_ata: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CloseDcaKeys {
    pub user: Pubkey,
    pub dca: Pubkey,
    pub input_mint: Pubkey,
    pub output_mint: Pubkey,
    pub in_ata: Pubkey,
    pub out_ata: Pubkey,
    pub user_in_ata: Pubkey,
    pub user_out_ata: Pubkey,
    pub system_program: Pubkey,
    pub token_program: Pubkey,
    pub associated_token_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<CloseDcaAccounts<'_, '_>> for CloseDcaKeys {
    fn from(accounts: CloseDcaAccounts) -> Self {
        Self {
            user: *accounts.user.key,
            dca: *accounts.dca.key,
            input_mint: *accounts.input_mint.key,
            output_mint: *accounts.output_mint.key,
            in_ata: *accounts.in_ata.key,
            out_ata: *accounts.out_ata.key,
            user_in_ata: *accounts.user_in_ata.key,
            user_out_ata: *accounts.user_out_ata.key,
            system_program: *accounts.system_program.key,
            token_program: *accounts.token_program.key,
            associated_token_program: *accounts.associated_token_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<CloseDcaKeys> for [AccountMeta; CLOSE_DCA_IX_ACCOUNTS_LEN] {
    fn from(keys: CloseDcaKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.user,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.dca,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.input_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.output_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.in_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.out_ata,
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
            AccountMeta {
                pubkey: keys.event_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; CLOSE_DCA_IX_ACCOUNTS_LEN]> for CloseDcaKeys {
    fn from(pubkeys: [Pubkey; CLOSE_DCA_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            user: pubkeys[0],
            dca: pubkeys[1],
            input_mint: pubkeys[2],
            output_mint: pubkeys[3],
            in_ata: pubkeys[4],
            out_ata: pubkeys[5],
            user_in_ata: pubkeys[6],
            user_out_ata: pubkeys[7],
            system_program: pubkeys[8],
            token_program: pubkeys[9],
            associated_token_program: pubkeys[10],
            event_authority: pubkeys[11],
            program: pubkeys[12],
        }
    }
}
impl<'info> From<CloseDcaAccounts<'_, 'info>>
for [AccountInfo<'info>; CLOSE_DCA_IX_ACCOUNTS_LEN] {
    fn from(accounts: CloseDcaAccounts<'_, 'info>) -> Self {
        [
            accounts.user.clone(),
            accounts.dca.clone(),
            accounts.input_mint.clone(),
            accounts.output_mint.clone(),
            accounts.in_ata.clone(),
            accounts.out_ata.clone(),
            accounts.user_in_ata.clone(),
            accounts.user_out_ata.clone(),
            accounts.system_program.clone(),
            accounts.token_program.clone(),
            accounts.associated_token_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CLOSE_DCA_IX_ACCOUNTS_LEN]>
for CloseDcaAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; CLOSE_DCA_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            user: &arr[0],
            dca: &arr[1],
            input_mint: &arr[2],
            output_mint: &arr[3],
            in_ata: &arr[4],
            out_ata: &arr[5],
            user_in_ata: &arr[6],
            user_out_ata: &arr[7],
            system_program: &arr[8],
            token_program: &arr[9],
            associated_token_program: &arr[10],
            event_authority: &arr[11],
            program: &arr[12],
        }
    }
}
pub const CLOSE_DCA_IX_DISCM: [u8; 8usize] = [22, 7, 33, 98, 168, 183, 34, 243];
#[derive(Clone, Debug, PartialEq)]
pub struct CloseDcaIxData;
impl CloseDcaIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CLOSE_DCA_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CLOSE_DCA_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn close_dca_ix_with_program_id(
    program_id: Pubkey,
    keys: CloseDcaKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CLOSE_DCA_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: CloseDcaIxData.try_to_vec()?,
    })
}
pub fn close_dca_ix(keys: CloseDcaKeys) -> std::io::Result<Instruction> {
    close_dca_ix_with_program_id(DCA_PROGRAM_ID, keys)
}
pub fn close_dca_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CloseDcaAccounts<'_, '_>,
) -> ProgramResult {
    let keys: CloseDcaKeys = accounts.into();
    let ix = close_dca_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn close_dca_invoke(accounts: CloseDcaAccounts<'_, '_>) -> ProgramResult {
    close_dca_invoke_with_program_id(DCA_PROGRAM_ID, accounts)
}
pub fn close_dca_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CloseDcaAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CloseDcaKeys = accounts.into();
    let ix = close_dca_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn close_dca_invoke_signed(
    accounts: CloseDcaAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    close_dca_invoke_signed_with_program_id(DCA_PROGRAM_ID, accounts, seeds)
}
pub fn close_dca_verify_account_keys(
    accounts: CloseDcaAccounts<'_, '_>,
    keys: CloseDcaKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.user.key, keys.user),
        (*accounts.dca.key, keys.dca),
        (*accounts.input_mint.key, keys.input_mint),
        (*accounts.output_mint.key, keys.output_mint),
        (*accounts.in_ata.key, keys.in_ata),
        (*accounts.out_ata.key, keys.out_ata),
        (*accounts.user_in_ata.key, keys.user_in_ata),
        (*accounts.user_out_ata.key, keys.user_out_ata),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.associated_token_program.key, keys.associated_token_program),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn close_dca_verify_writable_privileges<'me, 'info>(
    accounts: CloseDcaAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.user,
        accounts.dca,
        accounts.in_ata,
        accounts.out_ata,
        accounts.user_in_ata,
        accounts.user_out_ata,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn close_dca_verify_signer_privileges<'me, 'info>(
    accounts: CloseDcaAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.user] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn close_dca_verify_account_privileges<'me, 'info>(
    accounts: CloseDcaAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    close_dca_verify_writable_privileges(accounts)?;
    close_dca_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const WITHDRAW_IX_ACCOUNTS_LEN: usize = 12;
#[derive(Copy, Clone, Debug)]
pub struct WithdrawAccounts<'me, 'info> {
    pub user: &'me AccountInfo<'info>,
    pub dca: &'me AccountInfo<'info>,
    pub input_mint: &'me AccountInfo<'info>,
    pub output_mint: &'me AccountInfo<'info>,
    pub dca_ata: &'me AccountInfo<'info>,
    pub user_in_ata: &'me AccountInfo<'info>,
    pub user_out_ata: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct WithdrawKeys {
    pub user: Pubkey,
    pub dca: Pubkey,
    pub input_mint: Pubkey,
    pub output_mint: Pubkey,
    pub dca_ata: Pubkey,
    pub user_in_ata: Pubkey,
    pub user_out_ata: Pubkey,
    pub system_program: Pubkey,
    pub token_program: Pubkey,
    pub associated_token_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<WithdrawAccounts<'_, '_>> for WithdrawKeys {
    fn from(accounts: WithdrawAccounts) -> Self {
        Self {
            user: *accounts.user.key,
            dca: *accounts.dca.key,
            input_mint: *accounts.input_mint.key,
            output_mint: *accounts.output_mint.key,
            dca_ata: *accounts.dca_ata.key,
            user_in_ata: *accounts.user_in_ata.key,
            user_out_ata: *accounts.user_out_ata.key,
            system_program: *accounts.system_program.key,
            token_program: *accounts.token_program.key,
            associated_token_program: *accounts.associated_token_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<WithdrawKeys> for [AccountMeta; WITHDRAW_IX_ACCOUNTS_LEN] {
    fn from(keys: WithdrawKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.user,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.dca,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.input_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.output_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.dca_ata,
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
            AccountMeta {
                pubkey: keys.event_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.program,
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
            dca: pubkeys[1],
            input_mint: pubkeys[2],
            output_mint: pubkeys[3],
            dca_ata: pubkeys[4],
            user_in_ata: pubkeys[5],
            user_out_ata: pubkeys[6],
            system_program: pubkeys[7],
            token_program: pubkeys[8],
            associated_token_program: pubkeys[9],
            event_authority: pubkeys[10],
            program: pubkeys[11],
        }
    }
}
impl<'info> From<WithdrawAccounts<'_, 'info>>
for [AccountInfo<'info>; WITHDRAW_IX_ACCOUNTS_LEN] {
    fn from(accounts: WithdrawAccounts<'_, 'info>) -> Self {
        [
            accounts.user.clone(),
            accounts.dca.clone(),
            accounts.input_mint.clone(),
            accounts.output_mint.clone(),
            accounts.dca_ata.clone(),
            accounts.user_in_ata.clone(),
            accounts.user_out_ata.clone(),
            accounts.system_program.clone(),
            accounts.token_program.clone(),
            accounts.associated_token_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; WITHDRAW_IX_ACCOUNTS_LEN]>
for WithdrawAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; WITHDRAW_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            user: &arr[0],
            dca: &arr[1],
            input_mint: &arr[2],
            output_mint: &arr[3],
            dca_ata: &arr[4],
            user_in_ata: &arr[5],
            user_out_ata: &arr[6],
            system_program: &arr[7],
            token_program: &arr[8],
            associated_token_program: &arr[9],
            event_authority: &arr[10],
            program: &arr[11],
        }
    }
}
pub const WITHDRAW_IX_DISCM: [u8; 8usize] = [183, 18, 70, 156, 148, 109, 161, 34];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WithdrawIxArgs {
    pub withdraw_params: WithdrawParams,
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
        let withdraw_params = if reader.is_empty() {
            Default::default()
        } else {
            <WithdrawParams>::deserialize(&mut reader)?
        };
        Ok(Self(WithdrawIxArgs { withdraw_params }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&WITHDRAW_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.withdraw_params, &mut writer)?;
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
    withdraw_ix_with_program_id(DCA_PROGRAM_ID, keys, args)
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
    withdraw_invoke_with_program_id(DCA_PROGRAM_ID, accounts, args)
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
    withdraw_invoke_signed_with_program_id(DCA_PROGRAM_ID, accounts, args, seeds)
}
pub fn withdraw_verify_account_keys(
    accounts: WithdrawAccounts<'_, '_>,
    keys: WithdrawKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.user.key, keys.user),
        (*accounts.dca.key, keys.dca),
        (*accounts.input_mint.key, keys.input_mint),
        (*accounts.output_mint.key, keys.output_mint),
        (*accounts.dca_ata.key, keys.dca_ata),
        (*accounts.user_in_ata.key, keys.user_in_ata),
        (*accounts.user_out_ata.key, keys.user_out_ata),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.associated_token_program.key, keys.associated_token_program),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
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
        accounts.user,
        accounts.dca,
        accounts.dca_ata,
        accounts.user_in_ata,
        accounts.user_out_ata,
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
pub const DEPOSIT_IX_ACCOUNTS_LEN: usize = 7;
#[derive(Copy, Clone, Debug)]
pub struct DepositAccounts<'me, 'info> {
    pub user: &'me AccountInfo<'info>,
    pub dca: &'me AccountInfo<'info>,
    pub in_ata: &'me AccountInfo<'info>,
    pub user_in_ata: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct DepositKeys {
    pub user: Pubkey,
    pub dca: Pubkey,
    pub in_ata: Pubkey,
    pub user_in_ata: Pubkey,
    pub token_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<DepositAccounts<'_, '_>> for DepositKeys {
    fn from(accounts: DepositAccounts) -> Self {
        Self {
            user: *accounts.user.key,
            dca: *accounts.dca.key,
            in_ata: *accounts.in_ata.key,
            user_in_ata: *accounts.user_in_ata.key,
            token_program: *accounts.token_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<DepositKeys> for [AccountMeta; DEPOSIT_IX_ACCOUNTS_LEN] {
    fn from(keys: DepositKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.user,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.dca,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.in_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_in_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.event_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.program,
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
            dca: pubkeys[1],
            in_ata: pubkeys[2],
            user_in_ata: pubkeys[3],
            token_program: pubkeys[4],
            event_authority: pubkeys[5],
            program: pubkeys[6],
        }
    }
}
impl<'info> From<DepositAccounts<'_, 'info>>
for [AccountInfo<'info>; DEPOSIT_IX_ACCOUNTS_LEN] {
    fn from(accounts: DepositAccounts<'_, 'info>) -> Self {
        [
            accounts.user.clone(),
            accounts.dca.clone(),
            accounts.in_ata.clone(),
            accounts.user_in_ata.clone(),
            accounts.token_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; DEPOSIT_IX_ACCOUNTS_LEN]>
for DepositAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; DEPOSIT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            user: &arr[0],
            dca: &arr[1],
            in_ata: &arr[2],
            user_in_ata: &arr[3],
            token_program: &arr[4],
            event_authority: &arr[5],
            program: &arr[6],
        }
    }
}
pub const DEPOSIT_IX_DISCM: [u8; 8usize] = [242, 35, 198, 137, 82, 225, 242, 182];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DepositIxArgs {
    pub deposit_in: u64,
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
        let deposit_in: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(Self(DepositIxArgs { deposit_in }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&DEPOSIT_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.deposit_in, &mut writer)?;
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
    deposit_ix_with_program_id(DCA_PROGRAM_ID, keys, args)
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
    deposit_invoke_with_program_id(DCA_PROGRAM_ID, accounts, args)
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
    deposit_invoke_signed_with_program_id(DCA_PROGRAM_ID, accounts, args, seeds)
}
pub fn deposit_verify_account_keys(
    accounts: DepositAccounts<'_, '_>,
    keys: DepositKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.user.key, keys.user),
        (*accounts.dca.key, keys.dca),
        (*accounts.in_ata.key, keys.in_ata),
        (*accounts.user_in_ata.key, keys.user_in_ata),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
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
        accounts.user,
        accounts.dca,
        accounts.in_ata,
        accounts.user_in_ata,
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
pub const WITHDRAW_FEES_IX_ACCOUNTS_LEN: usize = 8;
#[derive(Copy, Clone, Debug)]
pub struct WithdrawFeesAccounts<'me, 'info> {
    pub admin: &'me AccountInfo<'info>,
    pub mint: &'me AccountInfo<'info>,
    pub fee_authority: &'me AccountInfo<'info>,
    pub program_fee_ata: &'me AccountInfo<'info>,
    pub admin_fee_ata: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct WithdrawFeesKeys {
    pub admin: Pubkey,
    pub mint: Pubkey,
    pub fee_authority: Pubkey,
    pub program_fee_ata: Pubkey,
    pub admin_fee_ata: Pubkey,
    pub system_program: Pubkey,
    pub token_program: Pubkey,
    pub associated_token_program: Pubkey,
}
impl From<WithdrawFeesAccounts<'_, '_>> for WithdrawFeesKeys {
    fn from(accounts: WithdrawFeesAccounts) -> Self {
        Self {
            admin: *accounts.admin.key,
            mint: *accounts.mint.key,
            fee_authority: *accounts.fee_authority.key,
            program_fee_ata: *accounts.program_fee_ata.key,
            admin_fee_ata: *accounts.admin_fee_ata.key,
            system_program: *accounts.system_program.key,
            token_program: *accounts.token_program.key,
            associated_token_program: *accounts.associated_token_program.key,
        }
    }
}
impl From<WithdrawFeesKeys> for [AccountMeta; WITHDRAW_FEES_IX_ACCOUNTS_LEN] {
    fn from(keys: WithdrawFeesKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.admin,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.fee_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.program_fee_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.admin_fee_ata,
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
impl From<[Pubkey; WITHDRAW_FEES_IX_ACCOUNTS_LEN]> for WithdrawFeesKeys {
    fn from(pubkeys: [Pubkey; WITHDRAW_FEES_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin: pubkeys[0],
            mint: pubkeys[1],
            fee_authority: pubkeys[2],
            program_fee_ata: pubkeys[3],
            admin_fee_ata: pubkeys[4],
            system_program: pubkeys[5],
            token_program: pubkeys[6],
            associated_token_program: pubkeys[7],
        }
    }
}
impl<'info> From<WithdrawFeesAccounts<'_, 'info>>
for [AccountInfo<'info>; WITHDRAW_FEES_IX_ACCOUNTS_LEN] {
    fn from(accounts: WithdrawFeesAccounts<'_, 'info>) -> Self {
        [
            accounts.admin.clone(),
            accounts.mint.clone(),
            accounts.fee_authority.clone(),
            accounts.program_fee_ata.clone(),
            accounts.admin_fee_ata.clone(),
            accounts.system_program.clone(),
            accounts.token_program.clone(),
            accounts.associated_token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; WITHDRAW_FEES_IX_ACCOUNTS_LEN]>
for WithdrawFeesAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; WITHDRAW_FEES_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin: &arr[0],
            mint: &arr[1],
            fee_authority: &arr[2],
            program_fee_ata: &arr[3],
            admin_fee_ata: &arr[4],
            system_program: &arr[5],
            token_program: &arr[6],
            associated_token_program: &arr[7],
        }
    }
}
pub const WITHDRAW_FEES_IX_DISCM: [u8; 8usize] = [198, 212, 171, 109, 144, 215, 174, 89];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WithdrawFeesIxArgs {
    pub amount: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct WithdrawFeesIxData(pub WithdrawFeesIxArgs);
impl From<WithdrawFeesIxArgs> for WithdrawFeesIxData {
    fn from(args: WithdrawFeesIxArgs) -> Self {
        Self(args)
    }
}
impl WithdrawFeesIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != WITHDRAW_FEES_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(Self(WithdrawFeesIxArgs { amount }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&WITHDRAW_FEES_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.amount, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn withdraw_fees_ix_with_program_id(
    program_id: Pubkey,
    keys: WithdrawFeesKeys,
    args: WithdrawFeesIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; WITHDRAW_FEES_IX_ACCOUNTS_LEN] = keys.into();
    let data: WithdrawFeesIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn withdraw_fees_ix(
    keys: WithdrawFeesKeys,
    args: WithdrawFeesIxArgs,
) -> std::io::Result<Instruction> {
    withdraw_fees_ix_with_program_id(DCA_PROGRAM_ID, keys, args)
}
pub fn withdraw_fees_invoke_with_program_id(
    program_id: Pubkey,
    accounts: WithdrawFeesAccounts<'_, '_>,
    args: WithdrawFeesIxArgs,
) -> ProgramResult {
    let keys: WithdrawFeesKeys = accounts.into();
    let ix = withdraw_fees_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn withdraw_fees_invoke(
    accounts: WithdrawFeesAccounts<'_, '_>,
    args: WithdrawFeesIxArgs,
) -> ProgramResult {
    withdraw_fees_invoke_with_program_id(DCA_PROGRAM_ID, accounts, args)
}
pub fn withdraw_fees_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: WithdrawFeesAccounts<'_, '_>,
    args: WithdrawFeesIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: WithdrawFeesKeys = accounts.into();
    let ix = withdraw_fees_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn withdraw_fees_invoke_signed(
    accounts: WithdrawFeesAccounts<'_, '_>,
    args: WithdrawFeesIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    withdraw_fees_invoke_signed_with_program_id(DCA_PROGRAM_ID, accounts, args, seeds)
}
pub fn withdraw_fees_verify_account_keys(
    accounts: WithdrawFeesAccounts<'_, '_>,
    keys: WithdrawFeesKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.admin.key, keys.admin),
        (*accounts.mint.key, keys.mint),
        (*accounts.fee_authority.key, keys.fee_authority),
        (*accounts.program_fee_ata.key, keys.program_fee_ata),
        (*accounts.admin_fee_ata.key, keys.admin_fee_ata),
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
pub fn withdraw_fees_verify_writable_privileges<'me, 'info>(
    accounts: WithdrawFeesAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.admin,
        accounts.program_fee_ata,
        accounts.admin_fee_ata,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn withdraw_fees_verify_signer_privileges<'me, 'info>(
    accounts: WithdrawFeesAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn withdraw_fees_verify_account_privileges<'me, 'info>(
    accounts: WithdrawFeesAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    withdraw_fees_verify_writable_privileges(accounts)?;
    withdraw_fees_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const INITIATE_FLASH_FILL_IX_ACCOUNTS_LEN: usize = 10;
#[derive(Copy, Clone, Debug)]
pub struct InitiateFlashFillAccounts<'me, 'info> {
    pub keeper: &'me AccountInfo<'info>,
    pub dca: &'me AccountInfo<'info>,
    pub input_mint: &'me AccountInfo<'info>,
    pub keeper_in_ata: &'me AccountInfo<'info>,
    pub in_ata: &'me AccountInfo<'info>,
    pub out_ata: &'me AccountInfo<'info>,
    pub instructions_sysvar: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct InitiateFlashFillKeys {
    pub keeper: Pubkey,
    pub dca: Pubkey,
    pub input_mint: Pubkey,
    pub keeper_in_ata: Pubkey,
    pub in_ata: Pubkey,
    pub out_ata: Pubkey,
    pub instructions_sysvar: Pubkey,
    pub system_program: Pubkey,
    pub token_program: Pubkey,
    pub associated_token_program: Pubkey,
}
impl From<InitiateFlashFillAccounts<'_, '_>> for InitiateFlashFillKeys {
    fn from(accounts: InitiateFlashFillAccounts) -> Self {
        Self {
            keeper: *accounts.keeper.key,
            dca: *accounts.dca.key,
            input_mint: *accounts.input_mint.key,
            keeper_in_ata: *accounts.keeper_in_ata.key,
            in_ata: *accounts.in_ata.key,
            out_ata: *accounts.out_ata.key,
            instructions_sysvar: *accounts.instructions_sysvar.key,
            system_program: *accounts.system_program.key,
            token_program: *accounts.token_program.key,
            associated_token_program: *accounts.associated_token_program.key,
        }
    }
}
impl From<InitiateFlashFillKeys> for [AccountMeta; INITIATE_FLASH_FILL_IX_ACCOUNTS_LEN] {
    fn from(keys: InitiateFlashFillKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.keeper,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.dca,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.input_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.keeper_in_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.in_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.out_ata,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.instructions_sysvar,
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
                pubkey: keys.associated_token_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; INITIATE_FLASH_FILL_IX_ACCOUNTS_LEN]> for InitiateFlashFillKeys {
    fn from(pubkeys: [Pubkey; INITIATE_FLASH_FILL_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            keeper: pubkeys[0],
            dca: pubkeys[1],
            input_mint: pubkeys[2],
            keeper_in_ata: pubkeys[3],
            in_ata: pubkeys[4],
            out_ata: pubkeys[5],
            instructions_sysvar: pubkeys[6],
            system_program: pubkeys[7],
            token_program: pubkeys[8],
            associated_token_program: pubkeys[9],
        }
    }
}
impl<'info> From<InitiateFlashFillAccounts<'_, 'info>>
for [AccountInfo<'info>; INITIATE_FLASH_FILL_IX_ACCOUNTS_LEN] {
    fn from(accounts: InitiateFlashFillAccounts<'_, 'info>) -> Self {
        [
            accounts.keeper.clone(),
            accounts.dca.clone(),
            accounts.input_mint.clone(),
            accounts.keeper_in_ata.clone(),
            accounts.in_ata.clone(),
            accounts.out_ata.clone(),
            accounts.instructions_sysvar.clone(),
            accounts.system_program.clone(),
            accounts.token_program.clone(),
            accounts.associated_token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; INITIATE_FLASH_FILL_IX_ACCOUNTS_LEN]>
for InitiateFlashFillAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; INITIATE_FLASH_FILL_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            keeper: &arr[0],
            dca: &arr[1],
            input_mint: &arr[2],
            keeper_in_ata: &arr[3],
            in_ata: &arr[4],
            out_ata: &arr[5],
            instructions_sysvar: &arr[6],
            system_program: &arr[7],
            token_program: &arr[8],
            associated_token_program: &arr[9],
        }
    }
}
pub const INITIATE_FLASH_FILL_IX_DISCM: [u8; 8usize] = [
    143, 205, 3, 191, 162, 215, 245, 49,
];
#[derive(Clone, Debug, PartialEq)]
pub struct InitiateFlashFillIxData;
impl InitiateFlashFillIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INITIATE_FLASH_FILL_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INITIATE_FLASH_FILL_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn initiate_flash_fill_ix_with_program_id(
    program_id: Pubkey,
    keys: InitiateFlashFillKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; INITIATE_FLASH_FILL_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: InitiateFlashFillIxData.try_to_vec()?,
    })
}
pub fn initiate_flash_fill_ix(
    keys: InitiateFlashFillKeys,
) -> std::io::Result<Instruction> {
    initiate_flash_fill_ix_with_program_id(DCA_PROGRAM_ID, keys)
}
pub fn initiate_flash_fill_invoke_with_program_id(
    program_id: Pubkey,
    accounts: InitiateFlashFillAccounts<'_, '_>,
) -> ProgramResult {
    let keys: InitiateFlashFillKeys = accounts.into();
    let ix = initiate_flash_fill_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn initiate_flash_fill_invoke(
    accounts: InitiateFlashFillAccounts<'_, '_>,
) -> ProgramResult {
    initiate_flash_fill_invoke_with_program_id(DCA_PROGRAM_ID, accounts)
}
pub fn initiate_flash_fill_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: InitiateFlashFillAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: InitiateFlashFillKeys = accounts.into();
    let ix = initiate_flash_fill_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn initiate_flash_fill_invoke_signed(
    accounts: InitiateFlashFillAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    initiate_flash_fill_invoke_signed_with_program_id(DCA_PROGRAM_ID, accounts, seeds)
}
pub fn initiate_flash_fill_verify_account_keys(
    accounts: InitiateFlashFillAccounts<'_, '_>,
    keys: InitiateFlashFillKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.keeper.key, keys.keeper),
        (*accounts.dca.key, keys.dca),
        (*accounts.input_mint.key, keys.input_mint),
        (*accounts.keeper_in_ata.key, keys.keeper_in_ata),
        (*accounts.in_ata.key, keys.in_ata),
        (*accounts.out_ata.key, keys.out_ata),
        (*accounts.instructions_sysvar.key, keys.instructions_sysvar),
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
pub fn initiate_flash_fill_verify_writable_privileges<'me, 'info>(
    accounts: InitiateFlashFillAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.keeper,
        accounts.dca,
        accounts.keeper_in_ata,
        accounts.in_ata,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn initiate_flash_fill_verify_signer_privileges<'me, 'info>(
    accounts: InitiateFlashFillAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.keeper] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn initiate_flash_fill_verify_account_privileges<'me, 'info>(
    accounts: InitiateFlashFillAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    initiate_flash_fill_verify_writable_privileges(accounts)?;
    initiate_flash_fill_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const FULFILL_FLASH_FILL_IX_ACCOUNTS_LEN: usize = 15;
#[derive(Copy, Clone, Debug)]
pub struct FulfillFlashFillAccounts<'me, 'info> {
    pub keeper: &'me AccountInfo<'info>,
    pub dca: &'me AccountInfo<'info>,
    pub input_mint: &'me AccountInfo<'info>,
    pub output_mint: &'me AccountInfo<'info>,
    pub keeper_in_ata: &'me AccountInfo<'info>,
    pub in_ata: &'me AccountInfo<'info>,
    pub out_ata: &'me AccountInfo<'info>,
    pub fee_authority: &'me AccountInfo<'info>,
    pub fee_ata: &'me AccountInfo<'info>,
    pub instructions_sysvar: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FulfillFlashFillKeys {
    pub keeper: Pubkey,
    pub dca: Pubkey,
    pub input_mint: Pubkey,
    pub output_mint: Pubkey,
    pub keeper_in_ata: Pubkey,
    pub in_ata: Pubkey,
    pub out_ata: Pubkey,
    pub fee_authority: Pubkey,
    pub fee_ata: Pubkey,
    pub instructions_sysvar: Pubkey,
    pub system_program: Pubkey,
    pub token_program: Pubkey,
    pub associated_token_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<FulfillFlashFillAccounts<'_, '_>> for FulfillFlashFillKeys {
    fn from(accounts: FulfillFlashFillAccounts) -> Self {
        Self {
            keeper: *accounts.keeper.key,
            dca: *accounts.dca.key,
            input_mint: *accounts.input_mint.key,
            output_mint: *accounts.output_mint.key,
            keeper_in_ata: *accounts.keeper_in_ata.key,
            in_ata: *accounts.in_ata.key,
            out_ata: *accounts.out_ata.key,
            fee_authority: *accounts.fee_authority.key,
            fee_ata: *accounts.fee_ata.key,
            instructions_sysvar: *accounts.instructions_sysvar.key,
            system_program: *accounts.system_program.key,
            token_program: *accounts.token_program.key,
            associated_token_program: *accounts.associated_token_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<FulfillFlashFillKeys> for [AccountMeta; FULFILL_FLASH_FILL_IX_ACCOUNTS_LEN] {
    fn from(keys: FulfillFlashFillKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.keeper,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.dca,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.input_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.output_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.keeper_in_ata,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.in_ata,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.out_ata,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.fee_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.fee_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.instructions_sysvar,
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
                pubkey: keys.associated_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.event_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; FULFILL_FLASH_FILL_IX_ACCOUNTS_LEN]> for FulfillFlashFillKeys {
    fn from(pubkeys: [Pubkey; FULFILL_FLASH_FILL_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            keeper: pubkeys[0],
            dca: pubkeys[1],
            input_mint: pubkeys[2],
            output_mint: pubkeys[3],
            keeper_in_ata: pubkeys[4],
            in_ata: pubkeys[5],
            out_ata: pubkeys[6],
            fee_authority: pubkeys[7],
            fee_ata: pubkeys[8],
            instructions_sysvar: pubkeys[9],
            system_program: pubkeys[10],
            token_program: pubkeys[11],
            associated_token_program: pubkeys[12],
            event_authority: pubkeys[13],
            program: pubkeys[14],
        }
    }
}
impl<'info> From<FulfillFlashFillAccounts<'_, 'info>>
for [AccountInfo<'info>; FULFILL_FLASH_FILL_IX_ACCOUNTS_LEN] {
    fn from(accounts: FulfillFlashFillAccounts<'_, 'info>) -> Self {
        [
            accounts.keeper.clone(),
            accounts.dca.clone(),
            accounts.input_mint.clone(),
            accounts.output_mint.clone(),
            accounts.keeper_in_ata.clone(),
            accounts.in_ata.clone(),
            accounts.out_ata.clone(),
            accounts.fee_authority.clone(),
            accounts.fee_ata.clone(),
            accounts.instructions_sysvar.clone(),
            accounts.system_program.clone(),
            accounts.token_program.clone(),
            accounts.associated_token_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; FULFILL_FLASH_FILL_IX_ACCOUNTS_LEN]>
for FulfillFlashFillAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; FULFILL_FLASH_FILL_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            keeper: &arr[0],
            dca: &arr[1],
            input_mint: &arr[2],
            output_mint: &arr[3],
            keeper_in_ata: &arr[4],
            in_ata: &arr[5],
            out_ata: &arr[6],
            fee_authority: &arr[7],
            fee_ata: &arr[8],
            instructions_sysvar: &arr[9],
            system_program: &arr[10],
            token_program: &arr[11],
            associated_token_program: &arr[12],
            event_authority: &arr[13],
            program: &arr[14],
        }
    }
}
pub const FULFILL_FLASH_FILL_IX_DISCM: [u8; 8usize] = [
    115, 64, 226, 78, 33, 211, 105, 162,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct FulfillFlashFillIxArgs {
    pub repay_amount: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct FulfillFlashFillIxData(pub FulfillFlashFillIxArgs);
impl From<FulfillFlashFillIxArgs> for FulfillFlashFillIxData {
    fn from(args: FulfillFlashFillIxArgs) -> Self {
        Self(args)
    }
}
impl FulfillFlashFillIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != FULFILL_FLASH_FILL_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let repay_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(FulfillFlashFillIxArgs {
                repay_amount,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&FULFILL_FLASH_FILL_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.repay_amount, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn fulfill_flash_fill_ix_with_program_id(
    program_id: Pubkey,
    keys: FulfillFlashFillKeys,
    args: FulfillFlashFillIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; FULFILL_FLASH_FILL_IX_ACCOUNTS_LEN] = keys.into();
    let data: FulfillFlashFillIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn fulfill_flash_fill_ix(
    keys: FulfillFlashFillKeys,
    args: FulfillFlashFillIxArgs,
) -> std::io::Result<Instruction> {
    fulfill_flash_fill_ix_with_program_id(DCA_PROGRAM_ID, keys, args)
}
pub fn fulfill_flash_fill_invoke_with_program_id(
    program_id: Pubkey,
    accounts: FulfillFlashFillAccounts<'_, '_>,
    args: FulfillFlashFillIxArgs,
) -> ProgramResult {
    let keys: FulfillFlashFillKeys = accounts.into();
    let ix = fulfill_flash_fill_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn fulfill_flash_fill_invoke(
    accounts: FulfillFlashFillAccounts<'_, '_>,
    args: FulfillFlashFillIxArgs,
) -> ProgramResult {
    fulfill_flash_fill_invoke_with_program_id(DCA_PROGRAM_ID, accounts, args)
}
pub fn fulfill_flash_fill_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: FulfillFlashFillAccounts<'_, '_>,
    args: FulfillFlashFillIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: FulfillFlashFillKeys = accounts.into();
    let ix = fulfill_flash_fill_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn fulfill_flash_fill_invoke_signed(
    accounts: FulfillFlashFillAccounts<'_, '_>,
    args: FulfillFlashFillIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    fulfill_flash_fill_invoke_signed_with_program_id(
        DCA_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn fulfill_flash_fill_verify_account_keys(
    accounts: FulfillFlashFillAccounts<'_, '_>,
    keys: FulfillFlashFillKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.keeper.key, keys.keeper),
        (*accounts.dca.key, keys.dca),
        (*accounts.input_mint.key, keys.input_mint),
        (*accounts.output_mint.key, keys.output_mint),
        (*accounts.keeper_in_ata.key, keys.keeper_in_ata),
        (*accounts.in_ata.key, keys.in_ata),
        (*accounts.out_ata.key, keys.out_ata),
        (*accounts.fee_authority.key, keys.fee_authority),
        (*accounts.fee_ata.key, keys.fee_ata),
        (*accounts.instructions_sysvar.key, keys.instructions_sysvar),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.associated_token_program.key, keys.associated_token_program),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn fulfill_flash_fill_verify_writable_privileges<'me, 'info>(
    accounts: FulfillFlashFillAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.keeper, accounts.dca, accounts.fee_ata] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn fulfill_flash_fill_verify_signer_privileges<'me, 'info>(
    accounts: FulfillFlashFillAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.keeper] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn fulfill_flash_fill_verify_account_privileges<'me, 'info>(
    accounts: FulfillFlashFillAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    fulfill_flash_fill_verify_writable_privileges(accounts)?;
    fulfill_flash_fill_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const INITIATE_DLMM_FILL_IX_ACCOUNTS_LEN: usize = 10;
#[derive(Copy, Clone, Debug)]
pub struct InitiateDlmmFillAccounts<'me, 'info> {
    pub keeper: &'me AccountInfo<'info>,
    pub dca: &'me AccountInfo<'info>,
    pub input_mint: &'me AccountInfo<'info>,
    pub keeper_in_ata: &'me AccountInfo<'info>,
    pub in_ata: &'me AccountInfo<'info>,
    pub out_ata: &'me AccountInfo<'info>,
    pub instructions_sysvar: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct InitiateDlmmFillKeys {
    pub keeper: Pubkey,
    pub dca: Pubkey,
    pub input_mint: Pubkey,
    pub keeper_in_ata: Pubkey,
    pub in_ata: Pubkey,
    pub out_ata: Pubkey,
    pub instructions_sysvar: Pubkey,
    pub system_program: Pubkey,
    pub token_program: Pubkey,
    pub associated_token_program: Pubkey,
}
impl From<InitiateDlmmFillAccounts<'_, '_>> for InitiateDlmmFillKeys {
    fn from(accounts: InitiateDlmmFillAccounts) -> Self {
        Self {
            keeper: *accounts.keeper.key,
            dca: *accounts.dca.key,
            input_mint: *accounts.input_mint.key,
            keeper_in_ata: *accounts.keeper_in_ata.key,
            in_ata: *accounts.in_ata.key,
            out_ata: *accounts.out_ata.key,
            instructions_sysvar: *accounts.instructions_sysvar.key,
            system_program: *accounts.system_program.key,
            token_program: *accounts.token_program.key,
            associated_token_program: *accounts.associated_token_program.key,
        }
    }
}
impl From<InitiateDlmmFillKeys> for [AccountMeta; INITIATE_DLMM_FILL_IX_ACCOUNTS_LEN] {
    fn from(keys: InitiateDlmmFillKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.keeper,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.dca,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.input_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.keeper_in_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.in_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.out_ata,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.instructions_sysvar,
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
                pubkey: keys.associated_token_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; INITIATE_DLMM_FILL_IX_ACCOUNTS_LEN]> for InitiateDlmmFillKeys {
    fn from(pubkeys: [Pubkey; INITIATE_DLMM_FILL_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            keeper: pubkeys[0],
            dca: pubkeys[1],
            input_mint: pubkeys[2],
            keeper_in_ata: pubkeys[3],
            in_ata: pubkeys[4],
            out_ata: pubkeys[5],
            instructions_sysvar: pubkeys[6],
            system_program: pubkeys[7],
            token_program: pubkeys[8],
            associated_token_program: pubkeys[9],
        }
    }
}
impl<'info> From<InitiateDlmmFillAccounts<'_, 'info>>
for [AccountInfo<'info>; INITIATE_DLMM_FILL_IX_ACCOUNTS_LEN] {
    fn from(accounts: InitiateDlmmFillAccounts<'_, 'info>) -> Self {
        [
            accounts.keeper.clone(),
            accounts.dca.clone(),
            accounts.input_mint.clone(),
            accounts.keeper_in_ata.clone(),
            accounts.in_ata.clone(),
            accounts.out_ata.clone(),
            accounts.instructions_sysvar.clone(),
            accounts.system_program.clone(),
            accounts.token_program.clone(),
            accounts.associated_token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; INITIATE_DLMM_FILL_IX_ACCOUNTS_LEN]>
for InitiateDlmmFillAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; INITIATE_DLMM_FILL_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            keeper: &arr[0],
            dca: &arr[1],
            input_mint: &arr[2],
            keeper_in_ata: &arr[3],
            in_ata: &arr[4],
            out_ata: &arr[5],
            instructions_sysvar: &arr[6],
            system_program: &arr[7],
            token_program: &arr[8],
            associated_token_program: &arr[9],
        }
    }
}
pub const INITIATE_DLMM_FILL_IX_DISCM: [u8; 8usize] = [
    155, 193, 80, 121, 91, 147, 254, 187,
];
#[derive(Clone, Debug, PartialEq)]
pub struct InitiateDlmmFillIxData;
impl InitiateDlmmFillIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INITIATE_DLMM_FILL_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INITIATE_DLMM_FILL_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn initiate_dlmm_fill_ix_with_program_id(
    program_id: Pubkey,
    keys: InitiateDlmmFillKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; INITIATE_DLMM_FILL_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: InitiateDlmmFillIxData.try_to_vec()?,
    })
}
pub fn initiate_dlmm_fill_ix(
    keys: InitiateDlmmFillKeys,
) -> std::io::Result<Instruction> {
    initiate_dlmm_fill_ix_with_program_id(DCA_PROGRAM_ID, keys)
}
pub fn initiate_dlmm_fill_invoke_with_program_id(
    program_id: Pubkey,
    accounts: InitiateDlmmFillAccounts<'_, '_>,
) -> ProgramResult {
    let keys: InitiateDlmmFillKeys = accounts.into();
    let ix = initiate_dlmm_fill_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn initiate_dlmm_fill_invoke(
    accounts: InitiateDlmmFillAccounts<'_, '_>,
) -> ProgramResult {
    initiate_dlmm_fill_invoke_with_program_id(DCA_PROGRAM_ID, accounts)
}
pub fn initiate_dlmm_fill_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: InitiateDlmmFillAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: InitiateDlmmFillKeys = accounts.into();
    let ix = initiate_dlmm_fill_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn initiate_dlmm_fill_invoke_signed(
    accounts: InitiateDlmmFillAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    initiate_dlmm_fill_invoke_signed_with_program_id(DCA_PROGRAM_ID, accounts, seeds)
}
pub fn initiate_dlmm_fill_verify_account_keys(
    accounts: InitiateDlmmFillAccounts<'_, '_>,
    keys: InitiateDlmmFillKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.keeper.key, keys.keeper),
        (*accounts.dca.key, keys.dca),
        (*accounts.input_mint.key, keys.input_mint),
        (*accounts.keeper_in_ata.key, keys.keeper_in_ata),
        (*accounts.in_ata.key, keys.in_ata),
        (*accounts.out_ata.key, keys.out_ata),
        (*accounts.instructions_sysvar.key, keys.instructions_sysvar),
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
pub fn initiate_dlmm_fill_verify_writable_privileges<'me, 'info>(
    accounts: InitiateDlmmFillAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.keeper,
        accounts.dca,
        accounts.keeper_in_ata,
        accounts.in_ata,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn initiate_dlmm_fill_verify_signer_privileges<'me, 'info>(
    accounts: InitiateDlmmFillAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.keeper] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn initiate_dlmm_fill_verify_account_privileges<'me, 'info>(
    accounts: InitiateDlmmFillAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    initiate_dlmm_fill_verify_writable_privileges(accounts)?;
    initiate_dlmm_fill_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const FULFILL_DLMM_FILL_IX_ACCOUNTS_LEN: usize = 15;
#[derive(Copy, Clone, Debug)]
pub struct FulfillDlmmFillAccounts<'me, 'info> {
    pub keeper: &'me AccountInfo<'info>,
    pub dca: &'me AccountInfo<'info>,
    pub input_mint: &'me AccountInfo<'info>,
    pub output_mint: &'me AccountInfo<'info>,
    pub keeper_in_ata: &'me AccountInfo<'info>,
    pub in_ata: &'me AccountInfo<'info>,
    pub out_ata: &'me AccountInfo<'info>,
    pub fee_authority: &'me AccountInfo<'info>,
    pub fee_ata: &'me AccountInfo<'info>,
    pub instructions_sysvar: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FulfillDlmmFillKeys {
    pub keeper: Pubkey,
    pub dca: Pubkey,
    pub input_mint: Pubkey,
    pub output_mint: Pubkey,
    pub keeper_in_ata: Pubkey,
    pub in_ata: Pubkey,
    pub out_ata: Pubkey,
    pub fee_authority: Pubkey,
    pub fee_ata: Pubkey,
    pub instructions_sysvar: Pubkey,
    pub system_program: Pubkey,
    pub token_program: Pubkey,
    pub associated_token_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<FulfillDlmmFillAccounts<'_, '_>> for FulfillDlmmFillKeys {
    fn from(accounts: FulfillDlmmFillAccounts) -> Self {
        Self {
            keeper: *accounts.keeper.key,
            dca: *accounts.dca.key,
            input_mint: *accounts.input_mint.key,
            output_mint: *accounts.output_mint.key,
            keeper_in_ata: *accounts.keeper_in_ata.key,
            in_ata: *accounts.in_ata.key,
            out_ata: *accounts.out_ata.key,
            fee_authority: *accounts.fee_authority.key,
            fee_ata: *accounts.fee_ata.key,
            instructions_sysvar: *accounts.instructions_sysvar.key,
            system_program: *accounts.system_program.key,
            token_program: *accounts.token_program.key,
            associated_token_program: *accounts.associated_token_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<FulfillDlmmFillKeys> for [AccountMeta; FULFILL_DLMM_FILL_IX_ACCOUNTS_LEN] {
    fn from(keys: FulfillDlmmFillKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.keeper,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.dca,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.input_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.output_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.keeper_in_ata,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.in_ata,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.out_ata,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.fee_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.fee_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.instructions_sysvar,
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
                pubkey: keys.associated_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.event_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; FULFILL_DLMM_FILL_IX_ACCOUNTS_LEN]> for FulfillDlmmFillKeys {
    fn from(pubkeys: [Pubkey; FULFILL_DLMM_FILL_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            keeper: pubkeys[0],
            dca: pubkeys[1],
            input_mint: pubkeys[2],
            output_mint: pubkeys[3],
            keeper_in_ata: pubkeys[4],
            in_ata: pubkeys[5],
            out_ata: pubkeys[6],
            fee_authority: pubkeys[7],
            fee_ata: pubkeys[8],
            instructions_sysvar: pubkeys[9],
            system_program: pubkeys[10],
            token_program: pubkeys[11],
            associated_token_program: pubkeys[12],
            event_authority: pubkeys[13],
            program: pubkeys[14],
        }
    }
}
impl<'info> From<FulfillDlmmFillAccounts<'_, 'info>>
for [AccountInfo<'info>; FULFILL_DLMM_FILL_IX_ACCOUNTS_LEN] {
    fn from(accounts: FulfillDlmmFillAccounts<'_, 'info>) -> Self {
        [
            accounts.keeper.clone(),
            accounts.dca.clone(),
            accounts.input_mint.clone(),
            accounts.output_mint.clone(),
            accounts.keeper_in_ata.clone(),
            accounts.in_ata.clone(),
            accounts.out_ata.clone(),
            accounts.fee_authority.clone(),
            accounts.fee_ata.clone(),
            accounts.instructions_sysvar.clone(),
            accounts.system_program.clone(),
            accounts.token_program.clone(),
            accounts.associated_token_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; FULFILL_DLMM_FILL_IX_ACCOUNTS_LEN]>
for FulfillDlmmFillAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; FULFILL_DLMM_FILL_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            keeper: &arr[0],
            dca: &arr[1],
            input_mint: &arr[2],
            output_mint: &arr[3],
            keeper_in_ata: &arr[4],
            in_ata: &arr[5],
            out_ata: &arr[6],
            fee_authority: &arr[7],
            fee_ata: &arr[8],
            instructions_sysvar: &arr[9],
            system_program: &arr[10],
            token_program: &arr[11],
            associated_token_program: &arr[12],
            event_authority: &arr[13],
            program: &arr[14],
        }
    }
}
pub const FULFILL_DLMM_FILL_IX_DISCM: [u8; 8usize] = [
    1, 230, 118, 251, 45, 177, 101, 187,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct FulfillDlmmFillIxArgs {
    pub repay_amount: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct FulfillDlmmFillIxData(pub FulfillDlmmFillIxArgs);
impl From<FulfillDlmmFillIxArgs> for FulfillDlmmFillIxData {
    fn from(args: FulfillDlmmFillIxArgs) -> Self {
        Self(args)
    }
}
impl FulfillDlmmFillIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != FULFILL_DLMM_FILL_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let repay_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(FulfillDlmmFillIxArgs {
                repay_amount,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&FULFILL_DLMM_FILL_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.repay_amount, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn fulfill_dlmm_fill_ix_with_program_id(
    program_id: Pubkey,
    keys: FulfillDlmmFillKeys,
    args: FulfillDlmmFillIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; FULFILL_DLMM_FILL_IX_ACCOUNTS_LEN] = keys.into();
    let data: FulfillDlmmFillIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn fulfill_dlmm_fill_ix(
    keys: FulfillDlmmFillKeys,
    args: FulfillDlmmFillIxArgs,
) -> std::io::Result<Instruction> {
    fulfill_dlmm_fill_ix_with_program_id(DCA_PROGRAM_ID, keys, args)
}
pub fn fulfill_dlmm_fill_invoke_with_program_id(
    program_id: Pubkey,
    accounts: FulfillDlmmFillAccounts<'_, '_>,
    args: FulfillDlmmFillIxArgs,
) -> ProgramResult {
    let keys: FulfillDlmmFillKeys = accounts.into();
    let ix = fulfill_dlmm_fill_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn fulfill_dlmm_fill_invoke(
    accounts: FulfillDlmmFillAccounts<'_, '_>,
    args: FulfillDlmmFillIxArgs,
) -> ProgramResult {
    fulfill_dlmm_fill_invoke_with_program_id(DCA_PROGRAM_ID, accounts, args)
}
pub fn fulfill_dlmm_fill_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: FulfillDlmmFillAccounts<'_, '_>,
    args: FulfillDlmmFillIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: FulfillDlmmFillKeys = accounts.into();
    let ix = fulfill_dlmm_fill_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn fulfill_dlmm_fill_invoke_signed(
    accounts: FulfillDlmmFillAccounts<'_, '_>,
    args: FulfillDlmmFillIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    fulfill_dlmm_fill_invoke_signed_with_program_id(
        DCA_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn fulfill_dlmm_fill_verify_account_keys(
    accounts: FulfillDlmmFillAccounts<'_, '_>,
    keys: FulfillDlmmFillKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.keeper.key, keys.keeper),
        (*accounts.dca.key, keys.dca),
        (*accounts.input_mint.key, keys.input_mint),
        (*accounts.output_mint.key, keys.output_mint),
        (*accounts.keeper_in_ata.key, keys.keeper_in_ata),
        (*accounts.in_ata.key, keys.in_ata),
        (*accounts.out_ata.key, keys.out_ata),
        (*accounts.fee_authority.key, keys.fee_authority),
        (*accounts.fee_ata.key, keys.fee_ata),
        (*accounts.instructions_sysvar.key, keys.instructions_sysvar),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.associated_token_program.key, keys.associated_token_program),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn fulfill_dlmm_fill_verify_writable_privileges<'me, 'info>(
    accounts: FulfillDlmmFillAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.keeper, accounts.dca, accounts.fee_ata] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn fulfill_dlmm_fill_verify_signer_privileges<'me, 'info>(
    accounts: FulfillDlmmFillAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.keeper] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn fulfill_dlmm_fill_verify_account_privileges<'me, 'info>(
    accounts: FulfillDlmmFillAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    fulfill_dlmm_fill_verify_writable_privileges(accounts)?;
    fulfill_dlmm_fill_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const TRANSFER_IX_ACCOUNTS_LEN: usize = 12;
#[derive(Copy, Clone, Debug)]
pub struct TransferAccounts<'me, 'info> {
    pub keeper: &'me AccountInfo<'info>,
    pub dca: &'me AccountInfo<'info>,
    pub user: &'me AccountInfo<'info>,
    pub output_mint: &'me AccountInfo<'info>,
    pub dca_out_ata: &'me AccountInfo<'info>,
    pub user_out_ata: &'me AccountInfo<'info>,
    pub intermediate_account: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct TransferKeys {
    pub keeper: Pubkey,
    pub dca: Pubkey,
    pub user: Pubkey,
    pub output_mint: Pubkey,
    pub dca_out_ata: Pubkey,
    pub user_out_ata: Pubkey,
    pub intermediate_account: Pubkey,
    pub system_program: Pubkey,
    pub token_program: Pubkey,
    pub associated_token_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<TransferAccounts<'_, '_>> for TransferKeys {
    fn from(accounts: TransferAccounts) -> Self {
        Self {
            keeper: *accounts.keeper.key,
            dca: *accounts.dca.key,
            user: *accounts.user.key,
            output_mint: *accounts.output_mint.key,
            dca_out_ata: *accounts.dca_out_ata.key,
            user_out_ata: *accounts.user_out_ata.key,
            intermediate_account: *accounts.intermediate_account.key,
            system_program: *accounts.system_program.key,
            token_program: *accounts.token_program.key,
            associated_token_program: *accounts.associated_token_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<TransferKeys> for [AccountMeta; TRANSFER_IX_ACCOUNTS_LEN] {
    fn from(keys: TransferKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.keeper,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.dca,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.output_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.dca_out_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_out_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.intermediate_account,
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
            AccountMeta {
                pubkey: keys.event_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; TRANSFER_IX_ACCOUNTS_LEN]> for TransferKeys {
    fn from(pubkeys: [Pubkey; TRANSFER_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            keeper: pubkeys[0],
            dca: pubkeys[1],
            user: pubkeys[2],
            output_mint: pubkeys[3],
            dca_out_ata: pubkeys[4],
            user_out_ata: pubkeys[5],
            intermediate_account: pubkeys[6],
            system_program: pubkeys[7],
            token_program: pubkeys[8],
            associated_token_program: pubkeys[9],
            event_authority: pubkeys[10],
            program: pubkeys[11],
        }
    }
}
impl<'info> From<TransferAccounts<'_, 'info>>
for [AccountInfo<'info>; TRANSFER_IX_ACCOUNTS_LEN] {
    fn from(accounts: TransferAccounts<'_, 'info>) -> Self {
        [
            accounts.keeper.clone(),
            accounts.dca.clone(),
            accounts.user.clone(),
            accounts.output_mint.clone(),
            accounts.dca_out_ata.clone(),
            accounts.user_out_ata.clone(),
            accounts.intermediate_account.clone(),
            accounts.system_program.clone(),
            accounts.token_program.clone(),
            accounts.associated_token_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; TRANSFER_IX_ACCOUNTS_LEN]>
for TransferAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; TRANSFER_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            keeper: &arr[0],
            dca: &arr[1],
            user: &arr[2],
            output_mint: &arr[3],
            dca_out_ata: &arr[4],
            user_out_ata: &arr[5],
            intermediate_account: &arr[6],
            system_program: &arr[7],
            token_program: &arr[8],
            associated_token_program: &arr[9],
            event_authority: &arr[10],
            program: &arr[11],
        }
    }
}
pub const TRANSFER_IX_DISCM: [u8; 8usize] = [163, 52, 200, 231, 140, 3, 69, 186];
#[derive(Clone, Debug, PartialEq)]
pub struct TransferIxData;
impl TransferIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != TRANSFER_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&TRANSFER_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn transfer_ix_with_program_id(
    program_id: Pubkey,
    keys: TransferKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; TRANSFER_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: TransferIxData.try_to_vec()?,
    })
}
pub fn transfer_ix(keys: TransferKeys) -> std::io::Result<Instruction> {
    transfer_ix_with_program_id(DCA_PROGRAM_ID, keys)
}
pub fn transfer_invoke_with_program_id(
    program_id: Pubkey,
    accounts: TransferAccounts<'_, '_>,
) -> ProgramResult {
    let keys: TransferKeys = accounts.into();
    let ix = transfer_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn transfer_invoke(accounts: TransferAccounts<'_, '_>) -> ProgramResult {
    transfer_invoke_with_program_id(DCA_PROGRAM_ID, accounts)
}
pub fn transfer_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: TransferAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: TransferKeys = accounts.into();
    let ix = transfer_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn transfer_invoke_signed(
    accounts: TransferAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    transfer_invoke_signed_with_program_id(DCA_PROGRAM_ID, accounts, seeds)
}
pub fn transfer_verify_account_keys(
    accounts: TransferAccounts<'_, '_>,
    keys: TransferKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.keeper.key, keys.keeper),
        (*accounts.dca.key, keys.dca),
        (*accounts.user.key, keys.user),
        (*accounts.output_mint.key, keys.output_mint),
        (*accounts.dca_out_ata.key, keys.dca_out_ata),
        (*accounts.user_out_ata.key, keys.user_out_ata),
        (*accounts.intermediate_account.key, keys.intermediate_account),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.associated_token_program.key, keys.associated_token_program),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn transfer_verify_writable_privileges<'me, 'info>(
    accounts: TransferAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.keeper,
        accounts.dca,
        accounts.user,
        accounts.dca_out_ata,
        accounts.user_out_ata,
        accounts.intermediate_account,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn transfer_verify_signer_privileges<'me, 'info>(
    accounts: TransferAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.keeper] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn transfer_verify_account_privileges<'me, 'info>(
    accounts: TransferAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    transfer_verify_writable_privileges(accounts)?;
    transfer_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const END_AND_CLOSE_IX_ACCOUNTS_LEN: usize = 15;
#[derive(Copy, Clone, Debug)]
pub struct EndAndCloseAccounts<'me, 'info> {
    pub keeper: &'me AccountInfo<'info>,
    pub dca: &'me AccountInfo<'info>,
    pub input_mint: &'me AccountInfo<'info>,
    pub output_mint: &'me AccountInfo<'info>,
    pub in_ata: &'me AccountInfo<'info>,
    pub out_ata: &'me AccountInfo<'info>,
    pub user: &'me AccountInfo<'info>,
    pub user_out_ata: &'me AccountInfo<'info>,
    pub init_user_out_ata: &'me AccountInfo<'info>,
    pub intermediate_account: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct EndAndCloseKeys {
    pub keeper: Pubkey,
    pub dca: Pubkey,
    pub input_mint: Pubkey,
    pub output_mint: Pubkey,
    pub in_ata: Pubkey,
    pub out_ata: Pubkey,
    pub user: Pubkey,
    pub user_out_ata: Pubkey,
    pub init_user_out_ata: Pubkey,
    pub intermediate_account: Pubkey,
    pub system_program: Pubkey,
    pub token_program: Pubkey,
    pub associated_token_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<EndAndCloseAccounts<'_, '_>> for EndAndCloseKeys {
    fn from(accounts: EndAndCloseAccounts) -> Self {
        Self {
            keeper: *accounts.keeper.key,
            dca: *accounts.dca.key,
            input_mint: *accounts.input_mint.key,
            output_mint: *accounts.output_mint.key,
            in_ata: *accounts.in_ata.key,
            out_ata: *accounts.out_ata.key,
            user: *accounts.user.key,
            user_out_ata: *accounts.user_out_ata.key,
            init_user_out_ata: *accounts.init_user_out_ata.key,
            intermediate_account: *accounts.intermediate_account.key,
            system_program: *accounts.system_program.key,
            token_program: *accounts.token_program.key,
            associated_token_program: *accounts.associated_token_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<EndAndCloseKeys> for [AccountMeta; END_AND_CLOSE_IX_ACCOUNTS_LEN] {
    fn from(keys: EndAndCloseKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.keeper,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.dca,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.input_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.output_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.in_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.out_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_out_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.init_user_out_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.intermediate_account,
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
            AccountMeta {
                pubkey: keys.event_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; END_AND_CLOSE_IX_ACCOUNTS_LEN]> for EndAndCloseKeys {
    fn from(pubkeys: [Pubkey; END_AND_CLOSE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            keeper: pubkeys[0],
            dca: pubkeys[1],
            input_mint: pubkeys[2],
            output_mint: pubkeys[3],
            in_ata: pubkeys[4],
            out_ata: pubkeys[5],
            user: pubkeys[6],
            user_out_ata: pubkeys[7],
            init_user_out_ata: pubkeys[8],
            intermediate_account: pubkeys[9],
            system_program: pubkeys[10],
            token_program: pubkeys[11],
            associated_token_program: pubkeys[12],
            event_authority: pubkeys[13],
            program: pubkeys[14],
        }
    }
}
impl<'info> From<EndAndCloseAccounts<'_, 'info>>
for [AccountInfo<'info>; END_AND_CLOSE_IX_ACCOUNTS_LEN] {
    fn from(accounts: EndAndCloseAccounts<'_, 'info>) -> Self {
        [
            accounts.keeper.clone(),
            accounts.dca.clone(),
            accounts.input_mint.clone(),
            accounts.output_mint.clone(),
            accounts.in_ata.clone(),
            accounts.out_ata.clone(),
            accounts.user.clone(),
            accounts.user_out_ata.clone(),
            accounts.init_user_out_ata.clone(),
            accounts.intermediate_account.clone(),
            accounts.system_program.clone(),
            accounts.token_program.clone(),
            accounts.associated_token_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; END_AND_CLOSE_IX_ACCOUNTS_LEN]>
for EndAndCloseAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; END_AND_CLOSE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            keeper: &arr[0],
            dca: &arr[1],
            input_mint: &arr[2],
            output_mint: &arr[3],
            in_ata: &arr[4],
            out_ata: &arr[5],
            user: &arr[6],
            user_out_ata: &arr[7],
            init_user_out_ata: &arr[8],
            intermediate_account: &arr[9],
            system_program: &arr[10],
            token_program: &arr[11],
            associated_token_program: &arr[12],
            event_authority: &arr[13],
            program: &arr[14],
        }
    }
}
pub const END_AND_CLOSE_IX_DISCM: [u8; 8usize] = [83, 125, 166, 69, 247, 252, 103, 133];
#[derive(Clone, Debug, PartialEq)]
pub struct EndAndCloseIxData;
impl EndAndCloseIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != END_AND_CLOSE_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&END_AND_CLOSE_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn end_and_close_ix_with_program_id(
    program_id: Pubkey,
    keys: EndAndCloseKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; END_AND_CLOSE_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: EndAndCloseIxData.try_to_vec()?,
    })
}
pub fn end_and_close_ix(keys: EndAndCloseKeys) -> std::io::Result<Instruction> {
    end_and_close_ix_with_program_id(DCA_PROGRAM_ID, keys)
}
pub fn end_and_close_invoke_with_program_id(
    program_id: Pubkey,
    accounts: EndAndCloseAccounts<'_, '_>,
) -> ProgramResult {
    let keys: EndAndCloseKeys = accounts.into();
    let ix = end_and_close_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn end_and_close_invoke(accounts: EndAndCloseAccounts<'_, '_>) -> ProgramResult {
    end_and_close_invoke_with_program_id(DCA_PROGRAM_ID, accounts)
}
pub fn end_and_close_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: EndAndCloseAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: EndAndCloseKeys = accounts.into();
    let ix = end_and_close_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn end_and_close_invoke_signed(
    accounts: EndAndCloseAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    end_and_close_invoke_signed_with_program_id(DCA_PROGRAM_ID, accounts, seeds)
}
pub fn end_and_close_verify_account_keys(
    accounts: EndAndCloseAccounts<'_, '_>,
    keys: EndAndCloseKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.keeper.key, keys.keeper),
        (*accounts.dca.key, keys.dca),
        (*accounts.input_mint.key, keys.input_mint),
        (*accounts.output_mint.key, keys.output_mint),
        (*accounts.in_ata.key, keys.in_ata),
        (*accounts.out_ata.key, keys.out_ata),
        (*accounts.user.key, keys.user),
        (*accounts.user_out_ata.key, keys.user_out_ata),
        (*accounts.init_user_out_ata.key, keys.init_user_out_ata),
        (*accounts.intermediate_account.key, keys.intermediate_account),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.associated_token_program.key, keys.associated_token_program),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn end_and_close_verify_writable_privileges<'me, 'info>(
    accounts: EndAndCloseAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.keeper,
        accounts.dca,
        accounts.in_ata,
        accounts.out_ata,
        accounts.user,
        accounts.user_out_ata,
        accounts.init_user_out_ata,
        accounts.intermediate_account,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn end_and_close_verify_signer_privileges<'me, 'info>(
    accounts: EndAndCloseAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.keeper] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn end_and_close_verify_account_privileges<'me, 'info>(
    accounts: EndAndCloseAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    end_and_close_verify_writable_privileges(accounts)?;
    end_and_close_verify_signer_privileges(accounts)?;
    Ok(())
}
