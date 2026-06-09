use solana_pubkey::Pubkey;
use solana_cpi::{invoke, invoke_signed};
use solana_instruction::{AccountMeta, Instruction};
use solana_account_info::AccountInfo;
use solana_program_error::ProgramError;
use std::io::Read;
#[allow(unused_imports)]
use crate::*;
#[derive(Clone, Debug, PartialEq)]
pub enum DynamicFeeSharingProgramIx {
    ClaimFee(ClaimFeeIxArgs),
    FundByClaimingFee(FundByClaimingFeeIxArgs),
    FundFee(FundFeeIxArgs),
    InitializeFeeVault(InitializeFeeVaultIxArgs),
    InitializeFeeVaultPda(InitializeFeeVaultPdaIxArgs),
}
impl DynamicFeeSharingProgramIx {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        if buf.starts_with(&CLAIM_FEE_IX_DISCM) {
            let mut reader = &buf[CLAIM_FEE_IX_DISCM.len()..];
            let index: u8 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(Self::ClaimFee(ClaimFeeIxArgs { index }));
        }
        if buf.starts_with(&FUND_BY_CLAIMING_FEE_IX_DISCM) {
            let mut reader = &buf[FUND_BY_CLAIMING_FEE_IX_DISCM.len()..];
            let payload: Vec<u8> = crate::borsh_de_or_default(&mut reader)?;
            return Ok(Self::FundByClaimingFee(FundByClaimingFeeIxArgs { payload }));
        }
        if buf.starts_with(&FUND_FEE_IX_DISCM) {
            let mut reader = &buf[FUND_FEE_IX_DISCM.len()..];
            let max_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(Self::FundFee(FundFeeIxArgs { max_amount }));
        }
        if buf.starts_with(&INITIALIZE_FEE_VAULT_IX_DISCM) {
            let mut reader = &buf[INITIALIZE_FEE_VAULT_IX_DISCM.len()..];
            let params = if reader.is_empty() {
                Default::default()
            } else {
                <InitializeFeeVaultParameters>::deserialize(&mut reader)?
            };
            return Ok(Self::InitializeFeeVault(InitializeFeeVaultIxArgs { params }));
        }
        if buf.starts_with(&INITIALIZE_FEE_VAULT_PDA_IX_DISCM) {
            let mut reader = &buf[INITIALIZE_FEE_VAULT_PDA_IX_DISCM.len()..];
            let params = if reader.is_empty() {
                Default::default()
            } else {
                <InitializeFeeVaultParameters>::deserialize(&mut reader)?
            };
            return Ok(
                Self::InitializeFeeVaultPda(InitializeFeeVaultPdaIxArgs {
                    params,
                }),
            );
        }
        Err(std::io::Error::from(std::io::ErrorKind::InvalidData))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        match self {
            Self::ClaimFee(args) => {
                writer.write_all(&CLAIM_FEE_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.index, &mut writer)?;
                Ok(())
            }
            Self::FundByClaimingFee(args) => {
                writer.write_all(&FUND_BY_CLAIMING_FEE_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.payload, &mut writer)?;
                Ok(())
            }
            Self::FundFee(args) => {
                writer.write_all(&FUND_FEE_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.max_amount, &mut writer)?;
                Ok(())
            }
            Self::InitializeFeeVault(args) => {
                writer.write_all(&INITIALIZE_FEE_VAULT_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.params, &mut writer)?;
                Ok(())
            }
            Self::InitializeFeeVaultPda(args) => {
                writer.write_all(&INITIALIZE_FEE_VAULT_PDA_IX_DISCM)?;
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
pub const CLAIM_FEE_IX_ACCOUNTS_LEN: usize = 9;
#[derive(Copy, Clone, Debug)]
pub struct ClaimFeeAccounts<'me, 'info> {
    pub fee_vault: &'me AccountInfo<'info>,
    pub fee_vault_authority: &'me AccountInfo<'info>,
    pub token_vault: &'me AccountInfo<'info>,
    pub token_mint: &'me AccountInfo<'info>,
    pub user_token_vault: &'me AccountInfo<'info>,
    pub user: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ClaimFeeKeys {
    pub fee_vault: Pubkey,
    pub fee_vault_authority: Pubkey,
    pub token_vault: Pubkey,
    pub token_mint: Pubkey,
    pub user_token_vault: Pubkey,
    pub user: Pubkey,
    pub token_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<ClaimFeeAccounts<'_, '_>> for ClaimFeeKeys {
    fn from(accounts: ClaimFeeAccounts) -> Self {
        Self {
            fee_vault: *accounts.fee_vault.key,
            fee_vault_authority: *accounts.fee_vault_authority.key,
            token_vault: *accounts.token_vault.key,
            token_mint: *accounts.token_mint.key,
            user_token_vault: *accounts.user_token_vault.key,
            user: *accounts.user.key,
            token_program: *accounts.token_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<ClaimFeeKeys> for [AccountMeta; CLAIM_FEE_IX_ACCOUNTS_LEN] {
    fn from(keys: ClaimFeeKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.fee_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.fee_vault_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.user_token_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user,
                is_signer: true,
                is_writable: false,
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
impl From<[Pubkey; CLAIM_FEE_IX_ACCOUNTS_LEN]> for ClaimFeeKeys {
    fn from(pubkeys: [Pubkey; CLAIM_FEE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            fee_vault: pubkeys[0],
            fee_vault_authority: pubkeys[1],
            token_vault: pubkeys[2],
            token_mint: pubkeys[3],
            user_token_vault: pubkeys[4],
            user: pubkeys[5],
            token_program: pubkeys[6],
            event_authority: pubkeys[7],
            program: pubkeys[8],
        }
    }
}
impl<'info> From<ClaimFeeAccounts<'_, 'info>>
for [AccountInfo<'info>; CLAIM_FEE_IX_ACCOUNTS_LEN] {
    fn from(accounts: ClaimFeeAccounts<'_, 'info>) -> Self {
        [
            accounts.fee_vault.clone(),
            accounts.fee_vault_authority.clone(),
            accounts.token_vault.clone(),
            accounts.token_mint.clone(),
            accounts.user_token_vault.clone(),
            accounts.user.clone(),
            accounts.token_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CLAIM_FEE_IX_ACCOUNTS_LEN]>
for ClaimFeeAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; CLAIM_FEE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            fee_vault: &arr[0],
            fee_vault_authority: &arr[1],
            token_vault: &arr[2],
            token_mint: &arr[3],
            user_token_vault: &arr[4],
            user: &arr[5],
            token_program: &arr[6],
            event_authority: &arr[7],
            program: &arr[8],
        }
    }
}
pub const CLAIM_FEE_IX_DISCM: [u8; 8usize] = [169, 32, 79, 137, 136, 232, 70, 137];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ClaimFeeIxArgs {
    pub index: u8,
}
#[derive(Clone, Debug, PartialEq)]
pub struct ClaimFeeIxData(pub ClaimFeeIxArgs);
impl From<ClaimFeeIxArgs> for ClaimFeeIxData {
    fn from(args: ClaimFeeIxArgs) -> Self {
        Self(args)
    }
}
impl ClaimFeeIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CLAIM_FEE_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let index: u8 = crate::borsh_de_or_default(&mut reader)?;
        Ok(Self(ClaimFeeIxArgs { index }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CLAIM_FEE_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.index, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn claim_fee_ix_with_program_id(
    program_id: Pubkey,
    keys: ClaimFeeKeys,
    args: ClaimFeeIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CLAIM_FEE_IX_ACCOUNTS_LEN] = keys.into();
    let data: ClaimFeeIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn claim_fee_ix(
    keys: ClaimFeeKeys,
    args: ClaimFeeIxArgs,
) -> std::io::Result<Instruction> {
    claim_fee_ix_with_program_id(DYNAMIC_FEE_SHARING_PROGRAM_ID, keys, args)
}
pub fn claim_fee_invoke_with_program_id(
    program_id: Pubkey,
    accounts: ClaimFeeAccounts<'_, '_>,
    args: ClaimFeeIxArgs,
) -> ProgramResult {
    let keys: ClaimFeeKeys = accounts.into();
    let ix = claim_fee_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn claim_fee_invoke(
    accounts: ClaimFeeAccounts<'_, '_>,
    args: ClaimFeeIxArgs,
) -> ProgramResult {
    claim_fee_invoke_with_program_id(DYNAMIC_FEE_SHARING_PROGRAM_ID, accounts, args)
}
pub fn claim_fee_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: ClaimFeeAccounts<'_, '_>,
    args: ClaimFeeIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: ClaimFeeKeys = accounts.into();
    let ix = claim_fee_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn claim_fee_invoke_signed(
    accounts: ClaimFeeAccounts<'_, '_>,
    args: ClaimFeeIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    claim_fee_invoke_signed_with_program_id(
        DYNAMIC_FEE_SHARING_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn claim_fee_verify_account_keys(
    accounts: ClaimFeeAccounts<'_, '_>,
    keys: ClaimFeeKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.fee_vault.key, keys.fee_vault),
        (*accounts.fee_vault_authority.key, keys.fee_vault_authority),
        (*accounts.token_vault.key, keys.token_vault),
        (*accounts.token_mint.key, keys.token_mint),
        (*accounts.user_token_vault.key, keys.user_token_vault),
        (*accounts.user.key, keys.user),
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
pub fn claim_fee_verify_writable_privileges<'me, 'info>(
    accounts: ClaimFeeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.fee_vault,
        accounts.token_vault,
        accounts.user_token_vault,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn claim_fee_verify_signer_privileges<'me, 'info>(
    accounts: ClaimFeeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.user] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn claim_fee_verify_account_privileges<'me, 'info>(
    accounts: ClaimFeeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    claim_fee_verify_writable_privileges(accounts)?;
    claim_fee_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const FUND_BY_CLAIMING_FEE_IX_ACCOUNTS_LEN: usize = 6;
#[derive(Copy, Clone, Debug)]
pub struct FundByClaimingFeeAccounts<'me, 'info> {
    pub fee_vault: &'me AccountInfo<'info>,
    pub token_vault: &'me AccountInfo<'info>,
    pub signer: &'me AccountInfo<'info>,
    pub source_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FundByClaimingFeeKeys {
    pub fee_vault: Pubkey,
    pub token_vault: Pubkey,
    pub signer: Pubkey,
    pub source_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<FundByClaimingFeeAccounts<'_, '_>> for FundByClaimingFeeKeys {
    fn from(accounts: FundByClaimingFeeAccounts) -> Self {
        Self {
            fee_vault: *accounts.fee_vault.key,
            token_vault: *accounts.token_vault.key,
            signer: *accounts.signer.key,
            source_program: *accounts.source_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<FundByClaimingFeeKeys>
for [AccountMeta; FUND_BY_CLAIMING_FEE_IX_ACCOUNTS_LEN] {
    fn from(keys: FundByClaimingFeeKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.fee_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.signer,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.source_program,
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
impl From<[Pubkey; FUND_BY_CLAIMING_FEE_IX_ACCOUNTS_LEN]> for FundByClaimingFeeKeys {
    fn from(pubkeys: [Pubkey; FUND_BY_CLAIMING_FEE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            fee_vault: pubkeys[0],
            token_vault: pubkeys[1],
            signer: pubkeys[2],
            source_program: pubkeys[3],
            event_authority: pubkeys[4],
            program: pubkeys[5],
        }
    }
}
impl<'info> From<FundByClaimingFeeAccounts<'_, 'info>>
for [AccountInfo<'info>; FUND_BY_CLAIMING_FEE_IX_ACCOUNTS_LEN] {
    fn from(accounts: FundByClaimingFeeAccounts<'_, 'info>) -> Self {
        [
            accounts.fee_vault.clone(),
            accounts.token_vault.clone(),
            accounts.signer.clone(),
            accounts.source_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; FUND_BY_CLAIMING_FEE_IX_ACCOUNTS_LEN]>
for FundByClaimingFeeAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; FUND_BY_CLAIMING_FEE_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            fee_vault: &arr[0],
            token_vault: &arr[1],
            signer: &arr[2],
            source_program: &arr[3],
            event_authority: &arr[4],
            program: &arr[5],
        }
    }
}
pub const FUND_BY_CLAIMING_FEE_IX_DISCM: [u8; 8usize] = [
    48, 226, 100, 60, 217, 101, 248, 182,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct FundByClaimingFeeIxArgs {
    pub payload: Vec<u8>,
}
#[derive(Clone, Debug, PartialEq)]
pub struct FundByClaimingFeeIxData(pub FundByClaimingFeeIxArgs);
impl From<FundByClaimingFeeIxArgs> for FundByClaimingFeeIxData {
    fn from(args: FundByClaimingFeeIxArgs) -> Self {
        Self(args)
    }
}
impl FundByClaimingFeeIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != FUND_BY_CLAIMING_FEE_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let payload: Vec<u8> = crate::borsh_de_or_default(&mut reader)?;
        Ok(Self(FundByClaimingFeeIxArgs { payload }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&FUND_BY_CLAIMING_FEE_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.payload, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn fund_by_claiming_fee_ix_with_program_id(
    program_id: Pubkey,
    keys: FundByClaimingFeeKeys,
    args: FundByClaimingFeeIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; FUND_BY_CLAIMING_FEE_IX_ACCOUNTS_LEN] = keys.into();
    let data: FundByClaimingFeeIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn fund_by_claiming_fee_ix(
    keys: FundByClaimingFeeKeys,
    args: FundByClaimingFeeIxArgs,
) -> std::io::Result<Instruction> {
    fund_by_claiming_fee_ix_with_program_id(DYNAMIC_FEE_SHARING_PROGRAM_ID, keys, args)
}
pub fn fund_by_claiming_fee_invoke_with_program_id(
    program_id: Pubkey,
    accounts: FundByClaimingFeeAccounts<'_, '_>,
    args: FundByClaimingFeeIxArgs,
) -> ProgramResult {
    let keys: FundByClaimingFeeKeys = accounts.into();
    let ix = fund_by_claiming_fee_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn fund_by_claiming_fee_invoke(
    accounts: FundByClaimingFeeAccounts<'_, '_>,
    args: FundByClaimingFeeIxArgs,
) -> ProgramResult {
    fund_by_claiming_fee_invoke_with_program_id(
        DYNAMIC_FEE_SHARING_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn fund_by_claiming_fee_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: FundByClaimingFeeAccounts<'_, '_>,
    args: FundByClaimingFeeIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: FundByClaimingFeeKeys = accounts.into();
    let ix = fund_by_claiming_fee_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn fund_by_claiming_fee_invoke_signed(
    accounts: FundByClaimingFeeAccounts<'_, '_>,
    args: FundByClaimingFeeIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    fund_by_claiming_fee_invoke_signed_with_program_id(
        DYNAMIC_FEE_SHARING_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn fund_by_claiming_fee_verify_account_keys(
    accounts: FundByClaimingFeeAccounts<'_, '_>,
    keys: FundByClaimingFeeKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.fee_vault.key, keys.fee_vault),
        (*accounts.token_vault.key, keys.token_vault),
        (*accounts.signer.key, keys.signer),
        (*accounts.source_program.key, keys.source_program),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn fund_by_claiming_fee_verify_writable_privileges<'me, 'info>(
    accounts: FundByClaimingFeeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.fee_vault, accounts.token_vault] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn fund_by_claiming_fee_verify_signer_privileges<'me, 'info>(
    accounts: FundByClaimingFeeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.signer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn fund_by_claiming_fee_verify_account_privileges<'me, 'info>(
    accounts: FundByClaimingFeeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    fund_by_claiming_fee_verify_writable_privileges(accounts)?;
    fund_by_claiming_fee_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const FUND_FEE_IX_ACCOUNTS_LEN: usize = 8;
#[derive(Copy, Clone, Debug)]
pub struct FundFeeAccounts<'me, 'info> {
    pub fee_vault: &'me AccountInfo<'info>,
    pub token_vault: &'me AccountInfo<'info>,
    pub token_mint: &'me AccountInfo<'info>,
    pub fund_token_vault: &'me AccountInfo<'info>,
    pub funder: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FundFeeKeys {
    pub fee_vault: Pubkey,
    pub token_vault: Pubkey,
    pub token_mint: Pubkey,
    pub fund_token_vault: Pubkey,
    pub funder: Pubkey,
    pub token_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<FundFeeAccounts<'_, '_>> for FundFeeKeys {
    fn from(accounts: FundFeeAccounts) -> Self {
        Self {
            fee_vault: *accounts.fee_vault.key,
            token_vault: *accounts.token_vault.key,
            token_mint: *accounts.token_mint.key,
            fund_token_vault: *accounts.fund_token_vault.key,
            funder: *accounts.funder.key,
            token_program: *accounts.token_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<FundFeeKeys> for [AccountMeta; FUND_FEE_IX_ACCOUNTS_LEN] {
    fn from(keys: FundFeeKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.fee_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.fund_token_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.funder,
                is_signer: true,
                is_writable: false,
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
impl From<[Pubkey; FUND_FEE_IX_ACCOUNTS_LEN]> for FundFeeKeys {
    fn from(pubkeys: [Pubkey; FUND_FEE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            fee_vault: pubkeys[0],
            token_vault: pubkeys[1],
            token_mint: pubkeys[2],
            fund_token_vault: pubkeys[3],
            funder: pubkeys[4],
            token_program: pubkeys[5],
            event_authority: pubkeys[6],
            program: pubkeys[7],
        }
    }
}
impl<'info> From<FundFeeAccounts<'_, 'info>>
for [AccountInfo<'info>; FUND_FEE_IX_ACCOUNTS_LEN] {
    fn from(accounts: FundFeeAccounts<'_, 'info>) -> Self {
        [
            accounts.fee_vault.clone(),
            accounts.token_vault.clone(),
            accounts.token_mint.clone(),
            accounts.fund_token_vault.clone(),
            accounts.funder.clone(),
            accounts.token_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; FUND_FEE_IX_ACCOUNTS_LEN]>
for FundFeeAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; FUND_FEE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            fee_vault: &arr[0],
            token_vault: &arr[1],
            token_mint: &arr[2],
            fund_token_vault: &arr[3],
            funder: &arr[4],
            token_program: &arr[5],
            event_authority: &arr[6],
            program: &arr[7],
        }
    }
}
pub const FUND_FEE_IX_DISCM: [u8; 8usize] = [243, 236, 235, 235, 101, 24, 186, 178];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct FundFeeIxArgs {
    pub max_amount: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct FundFeeIxData(pub FundFeeIxArgs);
impl From<FundFeeIxArgs> for FundFeeIxData {
    fn from(args: FundFeeIxArgs) -> Self {
        Self(args)
    }
}
impl FundFeeIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != FUND_FEE_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let max_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(Self(FundFeeIxArgs { max_amount }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&FUND_FEE_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.max_amount, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn fund_fee_ix_with_program_id(
    program_id: Pubkey,
    keys: FundFeeKeys,
    args: FundFeeIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; FUND_FEE_IX_ACCOUNTS_LEN] = keys.into();
    let data: FundFeeIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn fund_fee_ix(
    keys: FundFeeKeys,
    args: FundFeeIxArgs,
) -> std::io::Result<Instruction> {
    fund_fee_ix_with_program_id(DYNAMIC_FEE_SHARING_PROGRAM_ID, keys, args)
}
pub fn fund_fee_invoke_with_program_id(
    program_id: Pubkey,
    accounts: FundFeeAccounts<'_, '_>,
    args: FundFeeIxArgs,
) -> ProgramResult {
    let keys: FundFeeKeys = accounts.into();
    let ix = fund_fee_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn fund_fee_invoke(
    accounts: FundFeeAccounts<'_, '_>,
    args: FundFeeIxArgs,
) -> ProgramResult {
    fund_fee_invoke_with_program_id(DYNAMIC_FEE_SHARING_PROGRAM_ID, accounts, args)
}
pub fn fund_fee_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: FundFeeAccounts<'_, '_>,
    args: FundFeeIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: FundFeeKeys = accounts.into();
    let ix = fund_fee_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn fund_fee_invoke_signed(
    accounts: FundFeeAccounts<'_, '_>,
    args: FundFeeIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    fund_fee_invoke_signed_with_program_id(
        DYNAMIC_FEE_SHARING_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn fund_fee_verify_account_keys(
    accounts: FundFeeAccounts<'_, '_>,
    keys: FundFeeKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.fee_vault.key, keys.fee_vault),
        (*accounts.token_vault.key, keys.token_vault),
        (*accounts.token_mint.key, keys.token_mint),
        (*accounts.fund_token_vault.key, keys.fund_token_vault),
        (*accounts.funder.key, keys.funder),
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
pub fn fund_fee_verify_writable_privileges<'me, 'info>(
    accounts: FundFeeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.fee_vault,
        accounts.token_vault,
        accounts.fund_token_vault,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn fund_fee_verify_signer_privileges<'me, 'info>(
    accounts: FundFeeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.funder] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn fund_fee_verify_account_privileges<'me, 'info>(
    accounts: FundFeeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    fund_fee_verify_writable_privileges(accounts)?;
    fund_fee_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const INITIALIZE_FEE_VAULT_IX_ACCOUNTS_LEN: usize = 10;
#[derive(Copy, Clone, Debug)]
pub struct InitializeFeeVaultAccounts<'me, 'info> {
    pub fee_vault: &'me AccountInfo<'info>,
    pub fee_vault_authority: &'me AccountInfo<'info>,
    pub token_vault: &'me AccountInfo<'info>,
    pub token_mint: &'me AccountInfo<'info>,
    pub owner: &'me AccountInfo<'info>,
    pub payer: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct InitializeFeeVaultKeys {
    pub fee_vault: Pubkey,
    pub fee_vault_authority: Pubkey,
    pub token_vault: Pubkey,
    pub token_mint: Pubkey,
    pub owner: Pubkey,
    pub payer: Pubkey,
    pub token_program: Pubkey,
    pub system_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<InitializeFeeVaultAccounts<'_, '_>> for InitializeFeeVaultKeys {
    fn from(accounts: InitializeFeeVaultAccounts) -> Self {
        Self {
            fee_vault: *accounts.fee_vault.key,
            fee_vault_authority: *accounts.fee_vault_authority.key,
            token_vault: *accounts.token_vault.key,
            token_mint: *accounts.token_mint.key,
            owner: *accounts.owner.key,
            payer: *accounts.payer.key,
            token_program: *accounts.token_program.key,
            system_program: *accounts.system_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<InitializeFeeVaultKeys>
for [AccountMeta; INITIALIZE_FEE_VAULT_IX_ACCOUNTS_LEN] {
    fn from(keys: InitializeFeeVaultKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.fee_vault,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.fee_vault_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.owner,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.payer,
                is_signer: true,
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
impl From<[Pubkey; INITIALIZE_FEE_VAULT_IX_ACCOUNTS_LEN]> for InitializeFeeVaultKeys {
    fn from(pubkeys: [Pubkey; INITIALIZE_FEE_VAULT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            fee_vault: pubkeys[0],
            fee_vault_authority: pubkeys[1],
            token_vault: pubkeys[2],
            token_mint: pubkeys[3],
            owner: pubkeys[4],
            payer: pubkeys[5],
            token_program: pubkeys[6],
            system_program: pubkeys[7],
            event_authority: pubkeys[8],
            program: pubkeys[9],
        }
    }
}
impl<'info> From<InitializeFeeVaultAccounts<'_, 'info>>
for [AccountInfo<'info>; INITIALIZE_FEE_VAULT_IX_ACCOUNTS_LEN] {
    fn from(accounts: InitializeFeeVaultAccounts<'_, 'info>) -> Self {
        [
            accounts.fee_vault.clone(),
            accounts.fee_vault_authority.clone(),
            accounts.token_vault.clone(),
            accounts.token_mint.clone(),
            accounts.owner.clone(),
            accounts.payer.clone(),
            accounts.token_program.clone(),
            accounts.system_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; INITIALIZE_FEE_VAULT_IX_ACCOUNTS_LEN]>
for InitializeFeeVaultAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; INITIALIZE_FEE_VAULT_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            fee_vault: &arr[0],
            fee_vault_authority: &arr[1],
            token_vault: &arr[2],
            token_mint: &arr[3],
            owner: &arr[4],
            payer: &arr[5],
            token_program: &arr[6],
            system_program: &arr[7],
            event_authority: &arr[8],
            program: &arr[9],
        }
    }
}
pub const INITIALIZE_FEE_VAULT_IX_DISCM: [u8; 8usize] = [
    185, 140, 228, 234, 79, 203, 252, 50,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct InitializeFeeVaultIxArgs {
    pub params: InitializeFeeVaultParameters,
}
#[derive(Clone, Debug, PartialEq)]
pub struct InitializeFeeVaultIxData(pub InitializeFeeVaultIxArgs);
impl From<InitializeFeeVaultIxArgs> for InitializeFeeVaultIxData {
    fn from(args: InitializeFeeVaultIxArgs) -> Self {
        Self(args)
    }
}
impl InitializeFeeVaultIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INITIALIZE_FEE_VAULT_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let params = if reader.is_empty() {
            Default::default()
        } else {
            <InitializeFeeVaultParameters>::deserialize(&mut reader)?
        };
        Ok(Self(InitializeFeeVaultIxArgs { params }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INITIALIZE_FEE_VAULT_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.params, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn initialize_fee_vault_ix_with_program_id(
    program_id: Pubkey,
    keys: InitializeFeeVaultKeys,
    args: InitializeFeeVaultIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; INITIALIZE_FEE_VAULT_IX_ACCOUNTS_LEN] = keys.into();
    let data: InitializeFeeVaultIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn initialize_fee_vault_ix(
    keys: InitializeFeeVaultKeys,
    args: InitializeFeeVaultIxArgs,
) -> std::io::Result<Instruction> {
    initialize_fee_vault_ix_with_program_id(DYNAMIC_FEE_SHARING_PROGRAM_ID, keys, args)
}
pub fn initialize_fee_vault_invoke_with_program_id(
    program_id: Pubkey,
    accounts: InitializeFeeVaultAccounts<'_, '_>,
    args: InitializeFeeVaultIxArgs,
) -> ProgramResult {
    let keys: InitializeFeeVaultKeys = accounts.into();
    let ix = initialize_fee_vault_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn initialize_fee_vault_invoke(
    accounts: InitializeFeeVaultAccounts<'_, '_>,
    args: InitializeFeeVaultIxArgs,
) -> ProgramResult {
    initialize_fee_vault_invoke_with_program_id(
        DYNAMIC_FEE_SHARING_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn initialize_fee_vault_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: InitializeFeeVaultAccounts<'_, '_>,
    args: InitializeFeeVaultIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: InitializeFeeVaultKeys = accounts.into();
    let ix = initialize_fee_vault_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn initialize_fee_vault_invoke_signed(
    accounts: InitializeFeeVaultAccounts<'_, '_>,
    args: InitializeFeeVaultIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    initialize_fee_vault_invoke_signed_with_program_id(
        DYNAMIC_FEE_SHARING_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn initialize_fee_vault_verify_account_keys(
    accounts: InitializeFeeVaultAccounts<'_, '_>,
    keys: InitializeFeeVaultKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.fee_vault.key, keys.fee_vault),
        (*accounts.fee_vault_authority.key, keys.fee_vault_authority),
        (*accounts.token_vault.key, keys.token_vault),
        (*accounts.token_mint.key, keys.token_mint),
        (*accounts.owner.key, keys.owner),
        (*accounts.payer.key, keys.payer),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn initialize_fee_vault_verify_writable_privileges<'me, 'info>(
    accounts: InitializeFeeVaultAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.fee_vault,
        accounts.token_vault,
        accounts.payer,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn initialize_fee_vault_verify_signer_privileges<'me, 'info>(
    accounts: InitializeFeeVaultAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.fee_vault, accounts.payer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn initialize_fee_vault_verify_account_privileges<'me, 'info>(
    accounts: InitializeFeeVaultAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    initialize_fee_vault_verify_writable_privileges(accounts)?;
    initialize_fee_vault_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const INITIALIZE_FEE_VAULT_PDA_IX_ACCOUNTS_LEN: usize = 11;
#[derive(Copy, Clone, Debug)]
pub struct InitializeFeeVaultPdaAccounts<'me, 'info> {
    pub fee_vault: &'me AccountInfo<'info>,
    pub fee_vault_authority: &'me AccountInfo<'info>,
    pub token_vault: &'me AccountInfo<'info>,
    pub token_mint: &'me AccountInfo<'info>,
    pub owner: &'me AccountInfo<'info>,
    pub base: &'me AccountInfo<'info>,
    pub payer: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct InitializeFeeVaultPdaKeys {
    pub fee_vault: Pubkey,
    pub fee_vault_authority: Pubkey,
    pub token_vault: Pubkey,
    pub token_mint: Pubkey,
    pub owner: Pubkey,
    pub base: Pubkey,
    pub payer: Pubkey,
    pub token_program: Pubkey,
    pub system_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<InitializeFeeVaultPdaAccounts<'_, '_>> for InitializeFeeVaultPdaKeys {
    fn from(accounts: InitializeFeeVaultPdaAccounts) -> Self {
        Self {
            fee_vault: *accounts.fee_vault.key,
            fee_vault_authority: *accounts.fee_vault_authority.key,
            token_vault: *accounts.token_vault.key,
            token_mint: *accounts.token_mint.key,
            owner: *accounts.owner.key,
            base: *accounts.base.key,
            payer: *accounts.payer.key,
            token_program: *accounts.token_program.key,
            system_program: *accounts.system_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<InitializeFeeVaultPdaKeys>
for [AccountMeta; INITIALIZE_FEE_VAULT_PDA_IX_ACCOUNTS_LEN] {
    fn from(keys: InitializeFeeVaultPdaKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.fee_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.fee_vault_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.owner,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.base,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.payer,
                is_signer: true,
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
impl From<[Pubkey; INITIALIZE_FEE_VAULT_PDA_IX_ACCOUNTS_LEN]>
for InitializeFeeVaultPdaKeys {
    fn from(pubkeys: [Pubkey; INITIALIZE_FEE_VAULT_PDA_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            fee_vault: pubkeys[0],
            fee_vault_authority: pubkeys[1],
            token_vault: pubkeys[2],
            token_mint: pubkeys[3],
            owner: pubkeys[4],
            base: pubkeys[5],
            payer: pubkeys[6],
            token_program: pubkeys[7],
            system_program: pubkeys[8],
            event_authority: pubkeys[9],
            program: pubkeys[10],
        }
    }
}
impl<'info> From<InitializeFeeVaultPdaAccounts<'_, 'info>>
for [AccountInfo<'info>; INITIALIZE_FEE_VAULT_PDA_IX_ACCOUNTS_LEN] {
    fn from(accounts: InitializeFeeVaultPdaAccounts<'_, 'info>) -> Self {
        [
            accounts.fee_vault.clone(),
            accounts.fee_vault_authority.clone(),
            accounts.token_vault.clone(),
            accounts.token_mint.clone(),
            accounts.owner.clone(),
            accounts.base.clone(),
            accounts.payer.clone(),
            accounts.token_program.clone(),
            accounts.system_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; INITIALIZE_FEE_VAULT_PDA_IX_ACCOUNTS_LEN]>
for InitializeFeeVaultPdaAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; INITIALIZE_FEE_VAULT_PDA_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            fee_vault: &arr[0],
            fee_vault_authority: &arr[1],
            token_vault: &arr[2],
            token_mint: &arr[3],
            owner: &arr[4],
            base: &arr[5],
            payer: &arr[6],
            token_program: &arr[7],
            system_program: &arr[8],
            event_authority: &arr[9],
            program: &arr[10],
        }
    }
}
pub const INITIALIZE_FEE_VAULT_PDA_IX_DISCM: [u8; 8usize] = [
    250, 250, 156, 113, 88, 143, 60, 233,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct InitializeFeeVaultPdaIxArgs {
    pub params: InitializeFeeVaultParameters,
}
#[derive(Clone, Debug, PartialEq)]
pub struct InitializeFeeVaultPdaIxData(pub InitializeFeeVaultPdaIxArgs);
impl From<InitializeFeeVaultPdaIxArgs> for InitializeFeeVaultPdaIxData {
    fn from(args: InitializeFeeVaultPdaIxArgs) -> Self {
        Self(args)
    }
}
impl InitializeFeeVaultPdaIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INITIALIZE_FEE_VAULT_PDA_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let params = if reader.is_empty() {
            Default::default()
        } else {
            <InitializeFeeVaultParameters>::deserialize(&mut reader)?
        };
        Ok(
            Self(InitializeFeeVaultPdaIxArgs {
                params,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INITIALIZE_FEE_VAULT_PDA_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.params, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn initialize_fee_vault_pda_ix_with_program_id(
    program_id: Pubkey,
    keys: InitializeFeeVaultPdaKeys,
    args: InitializeFeeVaultPdaIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; INITIALIZE_FEE_VAULT_PDA_IX_ACCOUNTS_LEN] = keys.into();
    let data: InitializeFeeVaultPdaIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn initialize_fee_vault_pda_ix(
    keys: InitializeFeeVaultPdaKeys,
    args: InitializeFeeVaultPdaIxArgs,
) -> std::io::Result<Instruction> {
    initialize_fee_vault_pda_ix_with_program_id(
        DYNAMIC_FEE_SHARING_PROGRAM_ID,
        keys,
        args,
    )
}
pub fn initialize_fee_vault_pda_invoke_with_program_id(
    program_id: Pubkey,
    accounts: InitializeFeeVaultPdaAccounts<'_, '_>,
    args: InitializeFeeVaultPdaIxArgs,
) -> ProgramResult {
    let keys: InitializeFeeVaultPdaKeys = accounts.into();
    let ix = initialize_fee_vault_pda_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn initialize_fee_vault_pda_invoke(
    accounts: InitializeFeeVaultPdaAccounts<'_, '_>,
    args: InitializeFeeVaultPdaIxArgs,
) -> ProgramResult {
    initialize_fee_vault_pda_invoke_with_program_id(
        DYNAMIC_FEE_SHARING_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn initialize_fee_vault_pda_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: InitializeFeeVaultPdaAccounts<'_, '_>,
    args: InitializeFeeVaultPdaIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: InitializeFeeVaultPdaKeys = accounts.into();
    let ix = initialize_fee_vault_pda_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn initialize_fee_vault_pda_invoke_signed(
    accounts: InitializeFeeVaultPdaAccounts<'_, '_>,
    args: InitializeFeeVaultPdaIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    initialize_fee_vault_pda_invoke_signed_with_program_id(
        DYNAMIC_FEE_SHARING_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn initialize_fee_vault_pda_verify_account_keys(
    accounts: InitializeFeeVaultPdaAccounts<'_, '_>,
    keys: InitializeFeeVaultPdaKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.fee_vault.key, keys.fee_vault),
        (*accounts.fee_vault_authority.key, keys.fee_vault_authority),
        (*accounts.token_vault.key, keys.token_vault),
        (*accounts.token_mint.key, keys.token_mint),
        (*accounts.owner.key, keys.owner),
        (*accounts.base.key, keys.base),
        (*accounts.payer.key, keys.payer),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn initialize_fee_vault_pda_verify_writable_privileges<'me, 'info>(
    accounts: InitializeFeeVaultPdaAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.fee_vault,
        accounts.token_vault,
        accounts.payer,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn initialize_fee_vault_pda_verify_signer_privileges<'me, 'info>(
    accounts: InitializeFeeVaultPdaAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.base, accounts.payer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn initialize_fee_vault_pda_verify_account_privileges<'me, 'info>(
    accounts: InitializeFeeVaultPdaAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    initialize_fee_vault_pda_verify_writable_privileges(accounts)?;
    initialize_fee_vault_pda_verify_signer_privileges(accounts)?;
    Ok(())
}
