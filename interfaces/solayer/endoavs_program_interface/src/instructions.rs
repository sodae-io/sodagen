use solana_pubkey::Pubkey;
use solana_cpi::{invoke, invoke_signed};
use solana_instruction::{AccountMeta, Instruction};
use solana_account_info::AccountInfo;
use solana_program_error::ProgramError;
use std::io::Read;
#[allow(unused_imports)]
use crate::*;
#[derive(Clone, Debug, PartialEq)]
pub enum EndoavsProgramProgramIx {
    Create(CreateIxArgs),
    Delegate(DelegateIxArgs),
    Undelegate(UndelegateIxArgs),
    TransferAuthority,
    UpdateTokenMetadata(UpdateTokenMetadataIxArgs),
    UpdateEndoavs(UpdateEndoavsIxArgs),
}
impl EndoavsProgramProgramIx {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        if buf.starts_with(&CREATE_IX_DISCM) {
            let mut reader = &buf[CREATE_IX_DISCM.len()..];
            let name: String = crate::borsh_de_or_default(&mut reader)?;
            return Ok(Self::Create(CreateIxArgs { name }));
        }
        if buf.starts_with(&DELEGATE_IX_DISCM) {
            let mut reader = &buf[DELEGATE_IX_DISCM.len()..];
            let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(Self::Delegate(DelegateIxArgs { amount }));
        }
        if buf.starts_with(&UNDELEGATE_IX_DISCM) {
            let mut reader = &buf[UNDELEGATE_IX_DISCM.len()..];
            let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(Self::Undelegate(UndelegateIxArgs { amount }));
        }
        if buf.starts_with(&TRANSFER_AUTHORITY_IX_DISCM) {
            return Ok(Self::TransferAuthority);
        }
        if buf.starts_with(&UPDATE_TOKEN_METADATA_IX_DISCM) {
            let mut reader = &buf[UPDATE_TOKEN_METADATA_IX_DISCM.len()..];
            let name: String = crate::borsh_de_or_default(&mut reader)?;
            let symbol: String = crate::borsh_de_or_default(&mut reader)?;
            let uri: String = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::UpdateTokenMetadata(UpdateTokenMetadataIxArgs {
                    name,
                    symbol,
                    uri,
                }),
            );
        }
        if buf.starts_with(&UPDATE_ENDOAVS_IX_DISCM) {
            let mut reader = &buf[UPDATE_ENDOAVS_IX_DISCM.len()..];
            let name: Option<String> = crate::borsh_de_or_default(&mut reader)?;
            let url: Option<String> = crate::borsh_de_or_default(&mut reader)?;
            return Ok(Self::UpdateEndoavs(UpdateEndoavsIxArgs { name, url }));
        }
        Err(std::io::Error::from(std::io::ErrorKind::InvalidData))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        match self {
            Self::Create(args) => {
                writer.write_all(&CREATE_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.name, &mut writer)?;
                Ok(())
            }
            Self::Delegate(args) => {
                writer.write_all(&DELEGATE_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.amount, &mut writer)?;
                Ok(())
            }
            Self::Undelegate(args) => {
                writer.write_all(&UNDELEGATE_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.amount, &mut writer)?;
                Ok(())
            }
            Self::TransferAuthority => writer.write_all(&TRANSFER_AUTHORITY_IX_DISCM),
            Self::UpdateTokenMetadata(args) => {
                writer.write_all(&UPDATE_TOKEN_METADATA_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.name, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.symbol, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.uri, &mut writer)?;
                Ok(())
            }
            Self::UpdateEndoavs(args) => {
                writer.write_all(&UPDATE_ENDOAVS_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.name, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.url, &mut writer)?;
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
pub const CREATE_IX_ACCOUNTS_LEN: usize = 12;
#[derive(Copy, Clone, Debug)]
pub struct CreateAccounts<'me, 'info> {
    pub endo_avs: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub create_fee_recipient: &'me AccountInfo<'info>,
    pub avs_token_mint: &'me AccountInfo<'info>,
    pub avs_token_metadata: &'me AccountInfo<'info>,
    pub delegated_token_vault: &'me AccountInfo<'info>,
    pub delegated_token_mint: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
    pub token_metadata_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub rent: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CreateKeys {
    pub endo_avs: Pubkey,
    pub authority: Pubkey,
    pub create_fee_recipient: Pubkey,
    pub avs_token_mint: Pubkey,
    pub avs_token_metadata: Pubkey,
    pub delegated_token_vault: Pubkey,
    pub delegated_token_mint: Pubkey,
    pub token_program: Pubkey,
    pub associated_token_program: Pubkey,
    pub token_metadata_program: Pubkey,
    pub system_program: Pubkey,
    pub rent: Pubkey,
}
impl From<CreateAccounts<'_, '_>> for CreateKeys {
    fn from(accounts: CreateAccounts) -> Self {
        Self {
            endo_avs: *accounts.endo_avs.key,
            authority: *accounts.authority.key,
            create_fee_recipient: *accounts.create_fee_recipient.key,
            avs_token_mint: *accounts.avs_token_mint.key,
            avs_token_metadata: *accounts.avs_token_metadata.key,
            delegated_token_vault: *accounts.delegated_token_vault.key,
            delegated_token_mint: *accounts.delegated_token_mint.key,
            token_program: *accounts.token_program.key,
            associated_token_program: *accounts.associated_token_program.key,
            token_metadata_program: *accounts.token_metadata_program.key,
            system_program: *accounts.system_program.key,
            rent: *accounts.rent.key,
        }
    }
}
impl From<CreateKeys> for [AccountMeta; CREATE_IX_ACCOUNTS_LEN] {
    fn from(keys: CreateKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.endo_avs,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.authority,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.create_fee_recipient,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.avs_token_mint,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.avs_token_metadata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.delegated_token_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.delegated_token_mint,
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
                pubkey: keys.token_metadata_program,
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
impl From<[Pubkey; CREATE_IX_ACCOUNTS_LEN]> for CreateKeys {
    fn from(pubkeys: [Pubkey; CREATE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            endo_avs: pubkeys[0],
            authority: pubkeys[1],
            create_fee_recipient: pubkeys[2],
            avs_token_mint: pubkeys[3],
            avs_token_metadata: pubkeys[4],
            delegated_token_vault: pubkeys[5],
            delegated_token_mint: pubkeys[6],
            token_program: pubkeys[7],
            associated_token_program: pubkeys[8],
            token_metadata_program: pubkeys[9],
            system_program: pubkeys[10],
            rent: pubkeys[11],
        }
    }
}
impl<'info> From<CreateAccounts<'_, 'info>>
for [AccountInfo<'info>; CREATE_IX_ACCOUNTS_LEN] {
    fn from(accounts: CreateAccounts<'_, 'info>) -> Self {
        [
            accounts.endo_avs.clone(),
            accounts.authority.clone(),
            accounts.create_fee_recipient.clone(),
            accounts.avs_token_mint.clone(),
            accounts.avs_token_metadata.clone(),
            accounts.delegated_token_vault.clone(),
            accounts.delegated_token_mint.clone(),
            accounts.token_program.clone(),
            accounts.associated_token_program.clone(),
            accounts.token_metadata_program.clone(),
            accounts.system_program.clone(),
            accounts.rent.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CREATE_IX_ACCOUNTS_LEN]>
for CreateAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; CREATE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            endo_avs: &arr[0],
            authority: &arr[1],
            create_fee_recipient: &arr[2],
            avs_token_mint: &arr[3],
            avs_token_metadata: &arr[4],
            delegated_token_vault: &arr[5],
            delegated_token_mint: &arr[6],
            token_program: &arr[7],
            associated_token_program: &arr[8],
            token_metadata_program: &arr[9],
            system_program: &arr[10],
            rent: &arr[11],
        }
    }
}
pub const CREATE_IX_DISCM: [u8; 8usize] = [24, 30, 200, 40, 5, 28, 7, 119];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CreateIxArgs {
    pub name: String,
}
#[derive(Clone, Debug, PartialEq)]
pub struct CreateIxData(pub CreateIxArgs);
impl From<CreateIxArgs> for CreateIxData {
    fn from(args: CreateIxArgs) -> Self {
        Self(args)
    }
}
impl CreateIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CREATE_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let name: String = crate::borsh_de_or_default(&mut reader)?;
        Ok(Self(CreateIxArgs { name }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CREATE_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.name, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn create_ix_with_program_id(
    program_id: Pubkey,
    keys: CreateKeys,
    args: CreateIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CREATE_IX_ACCOUNTS_LEN] = keys.into();
    let data: CreateIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn create_ix(keys: CreateKeys, args: CreateIxArgs) -> std::io::Result<Instruction> {
    create_ix_with_program_id(ENDOAVS_PROGRAM_PROGRAM_ID, keys, args)
}
pub fn create_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CreateAccounts<'_, '_>,
    args: CreateIxArgs,
) -> ProgramResult {
    let keys: CreateKeys = accounts.into();
    let ix = create_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn create_invoke(
    accounts: CreateAccounts<'_, '_>,
    args: CreateIxArgs,
) -> ProgramResult {
    create_invoke_with_program_id(ENDOAVS_PROGRAM_PROGRAM_ID, accounts, args)
}
pub fn create_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CreateAccounts<'_, '_>,
    args: CreateIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CreateKeys = accounts.into();
    let ix = create_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn create_invoke_signed(
    accounts: CreateAccounts<'_, '_>,
    args: CreateIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    create_invoke_signed_with_program_id(
        ENDOAVS_PROGRAM_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn create_verify_account_keys(
    accounts: CreateAccounts<'_, '_>,
    keys: CreateKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.endo_avs.key, keys.endo_avs),
        (*accounts.authority.key, keys.authority),
        (*accounts.create_fee_recipient.key, keys.create_fee_recipient),
        (*accounts.avs_token_mint.key, keys.avs_token_mint),
        (*accounts.avs_token_metadata.key, keys.avs_token_metadata),
        (*accounts.delegated_token_vault.key, keys.delegated_token_vault),
        (*accounts.delegated_token_mint.key, keys.delegated_token_mint),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.associated_token_program.key, keys.associated_token_program),
        (*accounts.token_metadata_program.key, keys.token_metadata_program),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.rent.key, keys.rent),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn create_verify_writable_privileges<'me, 'info>(
    accounts: CreateAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.endo_avs,
        accounts.authority,
        accounts.create_fee_recipient,
        accounts.avs_token_mint,
        accounts.avs_token_metadata,
        accounts.delegated_token_vault,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn create_verify_signer_privileges<'me, 'info>(
    accounts: CreateAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority, accounts.avs_token_mint] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn create_verify_account_privileges<'me, 'info>(
    accounts: CreateAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    create_verify_writable_privileges(accounts)?;
    create_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const DELEGATE_IX_ACCOUNTS_LEN: usize = 10;
#[derive(Copy, Clone, Debug)]
pub struct DelegateAccounts<'me, 'info> {
    pub staker: &'me AccountInfo<'info>,
    pub endo_avs: &'me AccountInfo<'info>,
    pub avs_token_mint: &'me AccountInfo<'info>,
    pub delegated_token_vault: &'me AccountInfo<'info>,
    pub delegated_token_mint: &'me AccountInfo<'info>,
    pub staker_delegated_token_account: &'me AccountInfo<'info>,
    pub staker_avs_token_account: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct DelegateKeys {
    pub staker: Pubkey,
    pub endo_avs: Pubkey,
    pub avs_token_mint: Pubkey,
    pub delegated_token_vault: Pubkey,
    pub delegated_token_mint: Pubkey,
    pub staker_delegated_token_account: Pubkey,
    pub staker_avs_token_account: Pubkey,
    pub token_program: Pubkey,
    pub associated_token_program: Pubkey,
    pub system_program: Pubkey,
}
impl From<DelegateAccounts<'_, '_>> for DelegateKeys {
    fn from(accounts: DelegateAccounts) -> Self {
        Self {
            staker: *accounts.staker.key,
            endo_avs: *accounts.endo_avs.key,
            avs_token_mint: *accounts.avs_token_mint.key,
            delegated_token_vault: *accounts.delegated_token_vault.key,
            delegated_token_mint: *accounts.delegated_token_mint.key,
            staker_delegated_token_account: *accounts.staker_delegated_token_account.key,
            staker_avs_token_account: *accounts.staker_avs_token_account.key,
            token_program: *accounts.token_program.key,
            associated_token_program: *accounts.associated_token_program.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<DelegateKeys> for [AccountMeta; DELEGATE_IX_ACCOUNTS_LEN] {
    fn from(keys: DelegateKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.staker,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.endo_avs,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.avs_token_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.delegated_token_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.delegated_token_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.staker_delegated_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.staker_avs_token_account,
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
        ]
    }
}
impl From<[Pubkey; DELEGATE_IX_ACCOUNTS_LEN]> for DelegateKeys {
    fn from(pubkeys: [Pubkey; DELEGATE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            staker: pubkeys[0],
            endo_avs: pubkeys[1],
            avs_token_mint: pubkeys[2],
            delegated_token_vault: pubkeys[3],
            delegated_token_mint: pubkeys[4],
            staker_delegated_token_account: pubkeys[5],
            staker_avs_token_account: pubkeys[6],
            token_program: pubkeys[7],
            associated_token_program: pubkeys[8],
            system_program: pubkeys[9],
        }
    }
}
impl<'info> From<DelegateAccounts<'_, 'info>>
for [AccountInfo<'info>; DELEGATE_IX_ACCOUNTS_LEN] {
    fn from(accounts: DelegateAccounts<'_, 'info>) -> Self {
        [
            accounts.staker.clone(),
            accounts.endo_avs.clone(),
            accounts.avs_token_mint.clone(),
            accounts.delegated_token_vault.clone(),
            accounts.delegated_token_mint.clone(),
            accounts.staker_delegated_token_account.clone(),
            accounts.staker_avs_token_account.clone(),
            accounts.token_program.clone(),
            accounts.associated_token_program.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; DELEGATE_IX_ACCOUNTS_LEN]>
for DelegateAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; DELEGATE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            staker: &arr[0],
            endo_avs: &arr[1],
            avs_token_mint: &arr[2],
            delegated_token_vault: &arr[3],
            delegated_token_mint: &arr[4],
            staker_delegated_token_account: &arr[5],
            staker_avs_token_account: &arr[6],
            token_program: &arr[7],
            associated_token_program: &arr[8],
            system_program: &arr[9],
        }
    }
}
pub const DELEGATE_IX_DISCM: [u8; 8usize] = [90, 147, 75, 178, 85, 88, 4, 137];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DelegateIxArgs {
    pub amount: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct DelegateIxData(pub DelegateIxArgs);
impl From<DelegateIxArgs> for DelegateIxData {
    fn from(args: DelegateIxArgs) -> Self {
        Self(args)
    }
}
impl DelegateIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != DELEGATE_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(Self(DelegateIxArgs { amount }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&DELEGATE_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.amount, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn delegate_ix_with_program_id(
    program_id: Pubkey,
    keys: DelegateKeys,
    args: DelegateIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; DELEGATE_IX_ACCOUNTS_LEN] = keys.into();
    let data: DelegateIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn delegate_ix(
    keys: DelegateKeys,
    args: DelegateIxArgs,
) -> std::io::Result<Instruction> {
    delegate_ix_with_program_id(ENDOAVS_PROGRAM_PROGRAM_ID, keys, args)
}
pub fn delegate_invoke_with_program_id(
    program_id: Pubkey,
    accounts: DelegateAccounts<'_, '_>,
    args: DelegateIxArgs,
) -> ProgramResult {
    let keys: DelegateKeys = accounts.into();
    let ix = delegate_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn delegate_invoke(
    accounts: DelegateAccounts<'_, '_>,
    args: DelegateIxArgs,
) -> ProgramResult {
    delegate_invoke_with_program_id(ENDOAVS_PROGRAM_PROGRAM_ID, accounts, args)
}
pub fn delegate_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: DelegateAccounts<'_, '_>,
    args: DelegateIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: DelegateKeys = accounts.into();
    let ix = delegate_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn delegate_invoke_signed(
    accounts: DelegateAccounts<'_, '_>,
    args: DelegateIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    delegate_invoke_signed_with_program_id(
        ENDOAVS_PROGRAM_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn delegate_verify_account_keys(
    accounts: DelegateAccounts<'_, '_>,
    keys: DelegateKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.staker.key, keys.staker),
        (*accounts.endo_avs.key, keys.endo_avs),
        (*accounts.avs_token_mint.key, keys.avs_token_mint),
        (*accounts.delegated_token_vault.key, keys.delegated_token_vault),
        (*accounts.delegated_token_mint.key, keys.delegated_token_mint),
        (
            *accounts.staker_delegated_token_account.key,
            keys.staker_delegated_token_account,
        ),
        (*accounts.staker_avs_token_account.key, keys.staker_avs_token_account),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.associated_token_program.key, keys.associated_token_program),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn delegate_verify_writable_privileges<'me, 'info>(
    accounts: DelegateAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.staker,
        accounts.avs_token_mint,
        accounts.delegated_token_vault,
        accounts.staker_delegated_token_account,
        accounts.staker_avs_token_account,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn delegate_verify_signer_privileges<'me, 'info>(
    accounts: DelegateAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.staker] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn delegate_verify_account_privileges<'me, 'info>(
    accounts: DelegateAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    delegate_verify_writable_privileges(accounts)?;
    delegate_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const UNDELEGATE_IX_ACCOUNTS_LEN: usize = 10;
#[derive(Copy, Clone, Debug)]
pub struct UndelegateAccounts<'me, 'info> {
    pub staker: &'me AccountInfo<'info>,
    pub endo_avs: &'me AccountInfo<'info>,
    pub avs_token_mint: &'me AccountInfo<'info>,
    pub delegated_token_vault: &'me AccountInfo<'info>,
    pub delegated_token_mint: &'me AccountInfo<'info>,
    pub staker_delegated_token_account: &'me AccountInfo<'info>,
    pub staker_avs_token_account: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UndelegateKeys {
    pub staker: Pubkey,
    pub endo_avs: Pubkey,
    pub avs_token_mint: Pubkey,
    pub delegated_token_vault: Pubkey,
    pub delegated_token_mint: Pubkey,
    pub staker_delegated_token_account: Pubkey,
    pub staker_avs_token_account: Pubkey,
    pub token_program: Pubkey,
    pub associated_token_program: Pubkey,
    pub system_program: Pubkey,
}
impl From<UndelegateAccounts<'_, '_>> for UndelegateKeys {
    fn from(accounts: UndelegateAccounts) -> Self {
        Self {
            staker: *accounts.staker.key,
            endo_avs: *accounts.endo_avs.key,
            avs_token_mint: *accounts.avs_token_mint.key,
            delegated_token_vault: *accounts.delegated_token_vault.key,
            delegated_token_mint: *accounts.delegated_token_mint.key,
            staker_delegated_token_account: *accounts.staker_delegated_token_account.key,
            staker_avs_token_account: *accounts.staker_avs_token_account.key,
            token_program: *accounts.token_program.key,
            associated_token_program: *accounts.associated_token_program.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<UndelegateKeys> for [AccountMeta; UNDELEGATE_IX_ACCOUNTS_LEN] {
    fn from(keys: UndelegateKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.staker,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.endo_avs,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.avs_token_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.delegated_token_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.delegated_token_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.staker_delegated_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.staker_avs_token_account,
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
        ]
    }
}
impl From<[Pubkey; UNDELEGATE_IX_ACCOUNTS_LEN]> for UndelegateKeys {
    fn from(pubkeys: [Pubkey; UNDELEGATE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            staker: pubkeys[0],
            endo_avs: pubkeys[1],
            avs_token_mint: pubkeys[2],
            delegated_token_vault: pubkeys[3],
            delegated_token_mint: pubkeys[4],
            staker_delegated_token_account: pubkeys[5],
            staker_avs_token_account: pubkeys[6],
            token_program: pubkeys[7],
            associated_token_program: pubkeys[8],
            system_program: pubkeys[9],
        }
    }
}
impl<'info> From<UndelegateAccounts<'_, 'info>>
for [AccountInfo<'info>; UNDELEGATE_IX_ACCOUNTS_LEN] {
    fn from(accounts: UndelegateAccounts<'_, 'info>) -> Self {
        [
            accounts.staker.clone(),
            accounts.endo_avs.clone(),
            accounts.avs_token_mint.clone(),
            accounts.delegated_token_vault.clone(),
            accounts.delegated_token_mint.clone(),
            accounts.staker_delegated_token_account.clone(),
            accounts.staker_avs_token_account.clone(),
            accounts.token_program.clone(),
            accounts.associated_token_program.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; UNDELEGATE_IX_ACCOUNTS_LEN]>
for UndelegateAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; UNDELEGATE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            staker: &arr[0],
            endo_avs: &arr[1],
            avs_token_mint: &arr[2],
            delegated_token_vault: &arr[3],
            delegated_token_mint: &arr[4],
            staker_delegated_token_account: &arr[5],
            staker_avs_token_account: &arr[6],
            token_program: &arr[7],
            associated_token_program: &arr[8],
            system_program: &arr[9],
        }
    }
}
pub const UNDELEGATE_IX_DISCM: [u8; 8usize] = [131, 148, 180, 198, 91, 104, 42, 238];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UndelegateIxArgs {
    pub amount: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct UndelegateIxData(pub UndelegateIxArgs);
impl From<UndelegateIxArgs> for UndelegateIxData {
    fn from(args: UndelegateIxArgs) -> Self {
        Self(args)
    }
}
impl UndelegateIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UNDELEGATE_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(Self(UndelegateIxArgs { amount }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UNDELEGATE_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.amount, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn undelegate_ix_with_program_id(
    program_id: Pubkey,
    keys: UndelegateKeys,
    args: UndelegateIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; UNDELEGATE_IX_ACCOUNTS_LEN] = keys.into();
    let data: UndelegateIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn undelegate_ix(
    keys: UndelegateKeys,
    args: UndelegateIxArgs,
) -> std::io::Result<Instruction> {
    undelegate_ix_with_program_id(ENDOAVS_PROGRAM_PROGRAM_ID, keys, args)
}
pub fn undelegate_invoke_with_program_id(
    program_id: Pubkey,
    accounts: UndelegateAccounts<'_, '_>,
    args: UndelegateIxArgs,
) -> ProgramResult {
    let keys: UndelegateKeys = accounts.into();
    let ix = undelegate_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn undelegate_invoke(
    accounts: UndelegateAccounts<'_, '_>,
    args: UndelegateIxArgs,
) -> ProgramResult {
    undelegate_invoke_with_program_id(ENDOAVS_PROGRAM_PROGRAM_ID, accounts, args)
}
pub fn undelegate_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: UndelegateAccounts<'_, '_>,
    args: UndelegateIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: UndelegateKeys = accounts.into();
    let ix = undelegate_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn undelegate_invoke_signed(
    accounts: UndelegateAccounts<'_, '_>,
    args: UndelegateIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    undelegate_invoke_signed_with_program_id(
        ENDOAVS_PROGRAM_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn undelegate_verify_account_keys(
    accounts: UndelegateAccounts<'_, '_>,
    keys: UndelegateKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.staker.key, keys.staker),
        (*accounts.endo_avs.key, keys.endo_avs),
        (*accounts.avs_token_mint.key, keys.avs_token_mint),
        (*accounts.delegated_token_vault.key, keys.delegated_token_vault),
        (*accounts.delegated_token_mint.key, keys.delegated_token_mint),
        (
            *accounts.staker_delegated_token_account.key,
            keys.staker_delegated_token_account,
        ),
        (*accounts.staker_avs_token_account.key, keys.staker_avs_token_account),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.associated_token_program.key, keys.associated_token_program),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn undelegate_verify_writable_privileges<'me, 'info>(
    accounts: UndelegateAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.staker,
        accounts.avs_token_mint,
        accounts.delegated_token_vault,
        accounts.staker_delegated_token_account,
        accounts.staker_avs_token_account,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn undelegate_verify_signer_privileges<'me, 'info>(
    accounts: UndelegateAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.staker] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn undelegate_verify_account_privileges<'me, 'info>(
    accounts: UndelegateAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    undelegate_verify_writable_privileges(accounts)?;
    undelegate_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const TRANSFER_AUTHORITY_IX_ACCOUNTS_LEN: usize = 5;
#[derive(Copy, Clone, Debug)]
pub struct TransferAuthorityAccounts<'me, 'info> {
    pub authority: &'me AccountInfo<'info>,
    pub endo_avs: &'me AccountInfo<'info>,
    pub new_authority: &'me AccountInfo<'info>,
    pub avs_token_mint: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct TransferAuthorityKeys {
    pub authority: Pubkey,
    pub endo_avs: Pubkey,
    pub new_authority: Pubkey,
    pub avs_token_mint: Pubkey,
    pub system_program: Pubkey,
}
impl From<TransferAuthorityAccounts<'_, '_>> for TransferAuthorityKeys {
    fn from(accounts: TransferAuthorityAccounts) -> Self {
        Self {
            authority: *accounts.authority.key,
            endo_avs: *accounts.endo_avs.key,
            new_authority: *accounts.new_authority.key,
            avs_token_mint: *accounts.avs_token_mint.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<TransferAuthorityKeys> for [AccountMeta; TRANSFER_AUTHORITY_IX_ACCOUNTS_LEN] {
    fn from(keys: TransferAuthorityKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.authority,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.endo_avs,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.new_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.avs_token_mint,
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
impl From<[Pubkey; TRANSFER_AUTHORITY_IX_ACCOUNTS_LEN]> for TransferAuthorityKeys {
    fn from(pubkeys: [Pubkey; TRANSFER_AUTHORITY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            authority: pubkeys[0],
            endo_avs: pubkeys[1],
            new_authority: pubkeys[2],
            avs_token_mint: pubkeys[3],
            system_program: pubkeys[4],
        }
    }
}
impl<'info> From<TransferAuthorityAccounts<'_, 'info>>
for [AccountInfo<'info>; TRANSFER_AUTHORITY_IX_ACCOUNTS_LEN] {
    fn from(accounts: TransferAuthorityAccounts<'_, 'info>) -> Self {
        [
            accounts.authority.clone(),
            accounts.endo_avs.clone(),
            accounts.new_authority.clone(),
            accounts.avs_token_mint.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; TRANSFER_AUTHORITY_IX_ACCOUNTS_LEN]>
for TransferAuthorityAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; TRANSFER_AUTHORITY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            authority: &arr[0],
            endo_avs: &arr[1],
            new_authority: &arr[2],
            avs_token_mint: &arr[3],
            system_program: &arr[4],
        }
    }
}
pub const TRANSFER_AUTHORITY_IX_DISCM: [u8; 8usize] = [
    48, 169, 76, 72, 229, 180, 55, 161,
];
#[derive(Clone, Debug, PartialEq)]
pub struct TransferAuthorityIxData;
impl TransferAuthorityIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != TRANSFER_AUTHORITY_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&TRANSFER_AUTHORITY_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn transfer_authority_ix_with_program_id(
    program_id: Pubkey,
    keys: TransferAuthorityKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; TRANSFER_AUTHORITY_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: TransferAuthorityIxData.try_to_vec()?,
    })
}
pub fn transfer_authority_ix(
    keys: TransferAuthorityKeys,
) -> std::io::Result<Instruction> {
    transfer_authority_ix_with_program_id(ENDOAVS_PROGRAM_PROGRAM_ID, keys)
}
pub fn transfer_authority_invoke_with_program_id(
    program_id: Pubkey,
    accounts: TransferAuthorityAccounts<'_, '_>,
) -> ProgramResult {
    let keys: TransferAuthorityKeys = accounts.into();
    let ix = transfer_authority_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn transfer_authority_invoke(
    accounts: TransferAuthorityAccounts<'_, '_>,
) -> ProgramResult {
    transfer_authority_invoke_with_program_id(ENDOAVS_PROGRAM_PROGRAM_ID, accounts)
}
pub fn transfer_authority_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: TransferAuthorityAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: TransferAuthorityKeys = accounts.into();
    let ix = transfer_authority_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn transfer_authority_invoke_signed(
    accounts: TransferAuthorityAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    transfer_authority_invoke_signed_with_program_id(
        ENDOAVS_PROGRAM_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn transfer_authority_verify_account_keys(
    accounts: TransferAuthorityAccounts<'_, '_>,
    keys: TransferAuthorityKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.authority.key, keys.authority),
        (*accounts.endo_avs.key, keys.endo_avs),
        (*accounts.new_authority.key, keys.new_authority),
        (*accounts.avs_token_mint.key, keys.avs_token_mint),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn transfer_authority_verify_writable_privileges<'me, 'info>(
    accounts: TransferAuthorityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.authority, accounts.endo_avs] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn transfer_authority_verify_signer_privileges<'me, 'info>(
    accounts: TransferAuthorityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn transfer_authority_verify_account_privileges<'me, 'info>(
    accounts: TransferAuthorityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    transfer_authority_verify_writable_privileges(accounts)?;
    transfer_authority_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const UPDATE_TOKEN_METADATA_IX_ACCOUNTS_LEN: usize = 7;
#[derive(Copy, Clone, Debug)]
pub struct UpdateTokenMetadataAccounts<'me, 'info> {
    pub endo_avs: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub avs_token_mint: &'me AccountInfo<'info>,
    pub avs_token_metadata: &'me AccountInfo<'info>,
    pub token_metadata_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub rent: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UpdateTokenMetadataKeys {
    pub endo_avs: Pubkey,
    pub authority: Pubkey,
    pub avs_token_mint: Pubkey,
    pub avs_token_metadata: Pubkey,
    pub token_metadata_program: Pubkey,
    pub system_program: Pubkey,
    pub rent: Pubkey,
}
impl From<UpdateTokenMetadataAccounts<'_, '_>> for UpdateTokenMetadataKeys {
    fn from(accounts: UpdateTokenMetadataAccounts) -> Self {
        Self {
            endo_avs: *accounts.endo_avs.key,
            authority: *accounts.authority.key,
            avs_token_mint: *accounts.avs_token_mint.key,
            avs_token_metadata: *accounts.avs_token_metadata.key,
            token_metadata_program: *accounts.token_metadata_program.key,
            system_program: *accounts.system_program.key,
            rent: *accounts.rent.key,
        }
    }
}
impl From<UpdateTokenMetadataKeys>
for [AccountMeta; UPDATE_TOKEN_METADATA_IX_ACCOUNTS_LEN] {
    fn from(keys: UpdateTokenMetadataKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.endo_avs,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.authority,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.avs_token_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.avs_token_metadata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_metadata_program,
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
impl From<[Pubkey; UPDATE_TOKEN_METADATA_IX_ACCOUNTS_LEN]> for UpdateTokenMetadataKeys {
    fn from(pubkeys: [Pubkey; UPDATE_TOKEN_METADATA_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            endo_avs: pubkeys[0],
            authority: pubkeys[1],
            avs_token_mint: pubkeys[2],
            avs_token_metadata: pubkeys[3],
            token_metadata_program: pubkeys[4],
            system_program: pubkeys[5],
            rent: pubkeys[6],
        }
    }
}
impl<'info> From<UpdateTokenMetadataAccounts<'_, 'info>>
for [AccountInfo<'info>; UPDATE_TOKEN_METADATA_IX_ACCOUNTS_LEN] {
    fn from(accounts: UpdateTokenMetadataAccounts<'_, 'info>) -> Self {
        [
            accounts.endo_avs.clone(),
            accounts.authority.clone(),
            accounts.avs_token_mint.clone(),
            accounts.avs_token_metadata.clone(),
            accounts.token_metadata_program.clone(),
            accounts.system_program.clone(),
            accounts.rent.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; UPDATE_TOKEN_METADATA_IX_ACCOUNTS_LEN]>
for UpdateTokenMetadataAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; UPDATE_TOKEN_METADATA_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            endo_avs: &arr[0],
            authority: &arr[1],
            avs_token_mint: &arr[2],
            avs_token_metadata: &arr[3],
            token_metadata_program: &arr[4],
            system_program: &arr[5],
            rent: &arr[6],
        }
    }
}
pub const UPDATE_TOKEN_METADATA_IX_DISCM: [u8; 8usize] = [
    243, 6, 8, 23, 126, 181, 251, 158,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UpdateTokenMetadataIxArgs {
    pub name: String,
    pub symbol: String,
    pub uri: String,
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateTokenMetadataIxData(pub UpdateTokenMetadataIxArgs);
impl From<UpdateTokenMetadataIxArgs> for UpdateTokenMetadataIxData {
    fn from(args: UpdateTokenMetadataIxArgs) -> Self {
        Self(args)
    }
}
impl UpdateTokenMetadataIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_TOKEN_METADATA_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let name: String = crate::borsh_de_or_default(&mut reader)?;
        let symbol: String = crate::borsh_de_or_default(&mut reader)?;
        let uri: String = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(UpdateTokenMetadataIxArgs {
                name,
                symbol,
                uri,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_TOKEN_METADATA_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.name, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.symbol, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.uri, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn update_token_metadata_ix_with_program_id(
    program_id: Pubkey,
    keys: UpdateTokenMetadataKeys,
    args: UpdateTokenMetadataIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; UPDATE_TOKEN_METADATA_IX_ACCOUNTS_LEN] = keys.into();
    let data: UpdateTokenMetadataIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_token_metadata_ix(
    keys: UpdateTokenMetadataKeys,
    args: UpdateTokenMetadataIxArgs,
) -> std::io::Result<Instruction> {
    update_token_metadata_ix_with_program_id(ENDOAVS_PROGRAM_PROGRAM_ID, keys, args)
}
pub fn update_token_metadata_invoke_with_program_id(
    program_id: Pubkey,
    accounts: UpdateTokenMetadataAccounts<'_, '_>,
    args: UpdateTokenMetadataIxArgs,
) -> ProgramResult {
    let keys: UpdateTokenMetadataKeys = accounts.into();
    let ix = update_token_metadata_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn update_token_metadata_invoke(
    accounts: UpdateTokenMetadataAccounts<'_, '_>,
    args: UpdateTokenMetadataIxArgs,
) -> ProgramResult {
    update_token_metadata_invoke_with_program_id(
        ENDOAVS_PROGRAM_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn update_token_metadata_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: UpdateTokenMetadataAccounts<'_, '_>,
    args: UpdateTokenMetadataIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: UpdateTokenMetadataKeys = accounts.into();
    let ix = update_token_metadata_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn update_token_metadata_invoke_signed(
    accounts: UpdateTokenMetadataAccounts<'_, '_>,
    args: UpdateTokenMetadataIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    update_token_metadata_invoke_signed_with_program_id(
        ENDOAVS_PROGRAM_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn update_token_metadata_verify_account_keys(
    accounts: UpdateTokenMetadataAccounts<'_, '_>,
    keys: UpdateTokenMetadataKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.endo_avs.key, keys.endo_avs),
        (*accounts.authority.key, keys.authority),
        (*accounts.avs_token_mint.key, keys.avs_token_mint),
        (*accounts.avs_token_metadata.key, keys.avs_token_metadata),
        (*accounts.token_metadata_program.key, keys.token_metadata_program),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.rent.key, keys.rent),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn update_token_metadata_verify_writable_privileges<'me, 'info>(
    accounts: UpdateTokenMetadataAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.authority, accounts.avs_token_metadata] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn update_token_metadata_verify_signer_privileges<'me, 'info>(
    accounts: UpdateTokenMetadataAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn update_token_metadata_verify_account_privileges<'me, 'info>(
    accounts: UpdateTokenMetadataAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    update_token_metadata_verify_writable_privileges(accounts)?;
    update_token_metadata_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const UPDATE_ENDOAVS_IX_ACCOUNTS_LEN: usize = 4;
#[derive(Copy, Clone, Debug)]
pub struct UpdateEndoavsAccounts<'me, 'info> {
    pub authority: &'me AccountInfo<'info>,
    pub endo_avs: &'me AccountInfo<'info>,
    pub avs_token_mint: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UpdateEndoavsKeys {
    pub authority: Pubkey,
    pub endo_avs: Pubkey,
    pub avs_token_mint: Pubkey,
    pub system_program: Pubkey,
}
impl From<UpdateEndoavsAccounts<'_, '_>> for UpdateEndoavsKeys {
    fn from(accounts: UpdateEndoavsAccounts) -> Self {
        Self {
            authority: *accounts.authority.key,
            endo_avs: *accounts.endo_avs.key,
            avs_token_mint: *accounts.avs_token_mint.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<UpdateEndoavsKeys> for [AccountMeta; UPDATE_ENDOAVS_IX_ACCOUNTS_LEN] {
    fn from(keys: UpdateEndoavsKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.authority,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.endo_avs,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.avs_token_mint,
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
impl From<[Pubkey; UPDATE_ENDOAVS_IX_ACCOUNTS_LEN]> for UpdateEndoavsKeys {
    fn from(pubkeys: [Pubkey; UPDATE_ENDOAVS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            authority: pubkeys[0],
            endo_avs: pubkeys[1],
            avs_token_mint: pubkeys[2],
            system_program: pubkeys[3],
        }
    }
}
impl<'info> From<UpdateEndoavsAccounts<'_, 'info>>
for [AccountInfo<'info>; UPDATE_ENDOAVS_IX_ACCOUNTS_LEN] {
    fn from(accounts: UpdateEndoavsAccounts<'_, 'info>) -> Self {
        [
            accounts.authority.clone(),
            accounts.endo_avs.clone(),
            accounts.avs_token_mint.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; UPDATE_ENDOAVS_IX_ACCOUNTS_LEN]>
for UpdateEndoavsAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; UPDATE_ENDOAVS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            authority: &arr[0],
            endo_avs: &arr[1],
            avs_token_mint: &arr[2],
            system_program: &arr[3],
        }
    }
}
pub const UPDATE_ENDOAVS_IX_DISCM: [u8; 8usize] = [14, 245, 59, 169, 39, 126, 250, 78];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UpdateEndoavsIxArgs {
    pub name: Option<String>,
    pub url: Option<String>,
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateEndoavsIxData(pub UpdateEndoavsIxArgs);
impl From<UpdateEndoavsIxArgs> for UpdateEndoavsIxData {
    fn from(args: UpdateEndoavsIxArgs) -> Self {
        Self(args)
    }
}
impl UpdateEndoavsIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_ENDOAVS_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let name: Option<String> = crate::borsh_de_or_default(&mut reader)?;
        let url: Option<String> = crate::borsh_de_or_default(&mut reader)?;
        Ok(Self(UpdateEndoavsIxArgs { name, url }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_ENDOAVS_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.name, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.url, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn update_endoavs_ix_with_program_id(
    program_id: Pubkey,
    keys: UpdateEndoavsKeys,
    args: UpdateEndoavsIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; UPDATE_ENDOAVS_IX_ACCOUNTS_LEN] = keys.into();
    let data: UpdateEndoavsIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_endoavs_ix(
    keys: UpdateEndoavsKeys,
    args: UpdateEndoavsIxArgs,
) -> std::io::Result<Instruction> {
    update_endoavs_ix_with_program_id(ENDOAVS_PROGRAM_PROGRAM_ID, keys, args)
}
pub fn update_endoavs_invoke_with_program_id(
    program_id: Pubkey,
    accounts: UpdateEndoavsAccounts<'_, '_>,
    args: UpdateEndoavsIxArgs,
) -> ProgramResult {
    let keys: UpdateEndoavsKeys = accounts.into();
    let ix = update_endoavs_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn update_endoavs_invoke(
    accounts: UpdateEndoavsAccounts<'_, '_>,
    args: UpdateEndoavsIxArgs,
) -> ProgramResult {
    update_endoavs_invoke_with_program_id(ENDOAVS_PROGRAM_PROGRAM_ID, accounts, args)
}
pub fn update_endoavs_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: UpdateEndoavsAccounts<'_, '_>,
    args: UpdateEndoavsIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: UpdateEndoavsKeys = accounts.into();
    let ix = update_endoavs_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn update_endoavs_invoke_signed(
    accounts: UpdateEndoavsAccounts<'_, '_>,
    args: UpdateEndoavsIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    update_endoavs_invoke_signed_with_program_id(
        ENDOAVS_PROGRAM_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn update_endoavs_verify_account_keys(
    accounts: UpdateEndoavsAccounts<'_, '_>,
    keys: UpdateEndoavsKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.authority.key, keys.authority),
        (*accounts.endo_avs.key, keys.endo_avs),
        (*accounts.avs_token_mint.key, keys.avs_token_mint),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn update_endoavs_verify_writable_privileges<'me, 'info>(
    accounts: UpdateEndoavsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.endo_avs] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn update_endoavs_verify_signer_privileges<'me, 'info>(
    accounts: UpdateEndoavsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn update_endoavs_verify_account_privileges<'me, 'info>(
    accounts: UpdateEndoavsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    update_endoavs_verify_writable_privileges(accounts)?;
    update_endoavs_verify_signer_privileges(accounts)?;
    Ok(())
}
