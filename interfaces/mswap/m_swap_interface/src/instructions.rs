use solana_pubkey::Pubkey;
use solana_cpi::{invoke, invoke_signed};
use solana_instruction::{AccountMeta, Instruction};
use solana_account_info::AccountInfo;
use solana_program_error::ProgramError;
use std::io::Read;
#[allow(unused_imports)]
use crate::*;
#[derive(Clone, Debug, PartialEq)]
pub enum MSwapProgramIx {
    InitializeGlobal,
    RemoveWhitelistedExtension(RemoveWhitelistedExtensionIxArgs),
    RemoveWhitelistedUnwrapper(RemoveWhitelistedUnwrapperIxArgs),
    Swap(SwapIxArgs),
    Unwrap(UnwrapIxArgs),
    WhitelistExtension,
    WhitelistUnwrapper(WhitelistUnwrapperIxArgs),
    Wrap(WrapIxArgs),
}
impl MSwapProgramIx {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        if buf.starts_with(&INITIALIZE_GLOBAL_IX_DISCM) {
            return Ok(Self::InitializeGlobal);
        }
        if buf.starts_with(&REMOVE_WHITELISTED_EXTENSION_IX_DISCM) {
            let mut reader = &buf[REMOVE_WHITELISTED_EXTENSION_IX_DISCM.len()..];
            let ext_program: Pubkey = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::RemoveWhitelistedExtension(RemoveWhitelistedExtensionIxArgs {
                    ext_program,
                }),
            );
        }
        if buf.starts_with(&REMOVE_WHITELISTED_UNWRAPPER_IX_DISCM) {
            let mut reader = &buf[REMOVE_WHITELISTED_UNWRAPPER_IX_DISCM.len()..];
            let authority: Pubkey = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::RemoveWhitelistedUnwrapper(RemoveWhitelistedUnwrapperIxArgs {
                    authority,
                }),
            );
        }
        if buf.starts_with(&SWAP_IX_DISCM) {
            let mut reader = &buf[SWAP_IX_DISCM.len()..];
            let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            let remaining_accounts_split_idx: u8 = crate::borsh_de_or_default(
                &mut reader,
            )?;
            return Ok(
                Self::Swap(SwapIxArgs {
                    amount,
                    remaining_accounts_split_idx,
                }),
            );
        }
        if buf.starts_with(&UNWRAP_IX_DISCM) {
            let mut reader = &buf[UNWRAP_IX_DISCM.len()..];
            let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(Self::Unwrap(UnwrapIxArgs { amount }));
        }
        if buf.starts_with(&WHITELIST_EXTENSION_IX_DISCM) {
            return Ok(Self::WhitelistExtension);
        }
        if buf.starts_with(&WHITELIST_UNWRAPPER_IX_DISCM) {
            let mut reader = &buf[WHITELIST_UNWRAPPER_IX_DISCM.len()..];
            let authority: Pubkey = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::WhitelistUnwrapper(WhitelistUnwrapperIxArgs {
                    authority,
                }),
            );
        }
        if buf.starts_with(&WRAP_IX_DISCM) {
            let mut reader = &buf[WRAP_IX_DISCM.len()..];
            let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(Self::Wrap(WrapIxArgs { amount }));
        }
        Err(std::io::Error::from(std::io::ErrorKind::InvalidData))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        match self {
            Self::InitializeGlobal => writer.write_all(&INITIALIZE_GLOBAL_IX_DISCM),
            Self::RemoveWhitelistedExtension(args) => {
                writer.write_all(&REMOVE_WHITELISTED_EXTENSION_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.ext_program, &mut writer)?;
                Ok(())
            }
            Self::RemoveWhitelistedUnwrapper(args) => {
                writer.write_all(&REMOVE_WHITELISTED_UNWRAPPER_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.authority, &mut writer)?;
                Ok(())
            }
            Self::Swap(args) => {
                writer.write_all(&SWAP_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.amount, &mut writer)?;
                borsh::BorshSerialize::serialize(
                    &args.remaining_accounts_split_idx,
                    &mut writer,
                )?;
                Ok(())
            }
            Self::Unwrap(args) => {
                writer.write_all(&UNWRAP_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.amount, &mut writer)?;
                Ok(())
            }
            Self::WhitelistExtension => writer.write_all(&WHITELIST_EXTENSION_IX_DISCM),
            Self::WhitelistUnwrapper(args) => {
                writer.write_all(&WHITELIST_UNWRAPPER_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.authority, &mut writer)?;
                Ok(())
            }
            Self::Wrap(args) => {
                writer.write_all(&WRAP_IX_DISCM)?;
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
pub const INITIALIZE_GLOBAL_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct InitializeGlobalAccounts<'me, 'info> {
    pub admin: &'me AccountInfo<'info>,
    pub swap_global: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct InitializeGlobalKeys {
    pub admin: Pubkey,
    pub swap_global: Pubkey,
    pub system_program: Pubkey,
}
impl From<InitializeGlobalAccounts<'_, '_>> for InitializeGlobalKeys {
    fn from(accounts: InitializeGlobalAccounts) -> Self {
        Self {
            admin: *accounts.admin.key,
            swap_global: *accounts.swap_global.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<InitializeGlobalKeys> for [AccountMeta; INITIALIZE_GLOBAL_IX_ACCOUNTS_LEN] {
    fn from(keys: InitializeGlobalKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.admin,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.swap_global,
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
impl From<[Pubkey; INITIALIZE_GLOBAL_IX_ACCOUNTS_LEN]> for InitializeGlobalKeys {
    fn from(pubkeys: [Pubkey; INITIALIZE_GLOBAL_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin: pubkeys[0],
            swap_global: pubkeys[1],
            system_program: pubkeys[2],
        }
    }
}
impl<'info> From<InitializeGlobalAccounts<'_, 'info>>
for [AccountInfo<'info>; INITIALIZE_GLOBAL_IX_ACCOUNTS_LEN] {
    fn from(accounts: InitializeGlobalAccounts<'_, 'info>) -> Self {
        [
            accounts.admin.clone(),
            accounts.swap_global.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; INITIALIZE_GLOBAL_IX_ACCOUNTS_LEN]>
for InitializeGlobalAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; INITIALIZE_GLOBAL_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin: &arr[0],
            swap_global: &arr[1],
            system_program: &arr[2],
        }
    }
}
pub const INITIALIZE_GLOBAL_IX_DISCM: [u8; 8usize] = [
    47, 225, 15, 112, 86, 51, 190, 231,
];
#[derive(Clone, Debug, PartialEq)]
pub struct InitializeGlobalIxData;
impl InitializeGlobalIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INITIALIZE_GLOBAL_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INITIALIZE_GLOBAL_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn initialize_global_ix_with_program_id(
    program_id: Pubkey,
    keys: InitializeGlobalKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; INITIALIZE_GLOBAL_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: InitializeGlobalIxData.try_to_vec()?,
    })
}
pub fn initialize_global_ix(keys: InitializeGlobalKeys) -> std::io::Result<Instruction> {
    initialize_global_ix_with_program_id(M_SWAP_PROGRAM_ID, keys)
}
pub fn initialize_global_invoke_with_program_id(
    program_id: Pubkey,
    accounts: InitializeGlobalAccounts<'_, '_>,
) -> ProgramResult {
    let keys: InitializeGlobalKeys = accounts.into();
    let ix = initialize_global_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn initialize_global_invoke(
    accounts: InitializeGlobalAccounts<'_, '_>,
) -> ProgramResult {
    initialize_global_invoke_with_program_id(M_SWAP_PROGRAM_ID, accounts)
}
pub fn initialize_global_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: InitializeGlobalAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: InitializeGlobalKeys = accounts.into();
    let ix = initialize_global_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn initialize_global_invoke_signed(
    accounts: InitializeGlobalAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    initialize_global_invoke_signed_with_program_id(M_SWAP_PROGRAM_ID, accounts, seeds)
}
pub fn initialize_global_verify_account_keys(
    accounts: InitializeGlobalAccounts<'_, '_>,
    keys: InitializeGlobalKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.admin.key, keys.admin),
        (*accounts.swap_global.key, keys.swap_global),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn initialize_global_verify_writable_privileges<'me, 'info>(
    accounts: InitializeGlobalAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.admin, accounts.swap_global] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn initialize_global_verify_signer_privileges<'me, 'info>(
    accounts: InitializeGlobalAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn initialize_global_verify_account_privileges<'me, 'info>(
    accounts: InitializeGlobalAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    initialize_global_verify_writable_privileges(accounts)?;
    initialize_global_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const REMOVE_WHITELISTED_EXTENSION_IX_ACCOUNTS_LEN: usize = 2;
#[derive(Copy, Clone, Debug)]
pub struct RemoveWhitelistedExtensionAccounts<'me, 'info> {
    pub admin: &'me AccountInfo<'info>,
    pub swap_global: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct RemoveWhitelistedExtensionKeys {
    pub admin: Pubkey,
    pub swap_global: Pubkey,
}
impl From<RemoveWhitelistedExtensionAccounts<'_, '_>>
for RemoveWhitelistedExtensionKeys {
    fn from(accounts: RemoveWhitelistedExtensionAccounts) -> Self {
        Self {
            admin: *accounts.admin.key,
            swap_global: *accounts.swap_global.key,
        }
    }
}
impl From<RemoveWhitelistedExtensionKeys>
for [AccountMeta; REMOVE_WHITELISTED_EXTENSION_IX_ACCOUNTS_LEN] {
    fn from(keys: RemoveWhitelistedExtensionKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.admin,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.swap_global,
                is_signer: false,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; REMOVE_WHITELISTED_EXTENSION_IX_ACCOUNTS_LEN]>
for RemoveWhitelistedExtensionKeys {
    fn from(pubkeys: [Pubkey; REMOVE_WHITELISTED_EXTENSION_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin: pubkeys[0],
            swap_global: pubkeys[1],
        }
    }
}
impl<'info> From<RemoveWhitelistedExtensionAccounts<'_, 'info>>
for [AccountInfo<'info>; REMOVE_WHITELISTED_EXTENSION_IX_ACCOUNTS_LEN] {
    fn from(accounts: RemoveWhitelistedExtensionAccounts<'_, 'info>) -> Self {
        [accounts.admin.clone(), accounts.swap_global.clone()]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; REMOVE_WHITELISTED_EXTENSION_IX_ACCOUNTS_LEN]>
for RemoveWhitelistedExtensionAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; REMOVE_WHITELISTED_EXTENSION_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            admin: &arr[0],
            swap_global: &arr[1],
        }
    }
}
pub const REMOVE_WHITELISTED_EXTENSION_IX_DISCM: [u8; 8usize] = [
    248, 52, 115, 71, 67, 42, 71, 252,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct RemoveWhitelistedExtensionIxArgs {
    pub ext_program: Pubkey,
}
#[derive(Clone, Debug, PartialEq)]
pub struct RemoveWhitelistedExtensionIxData(pub RemoveWhitelistedExtensionIxArgs);
impl From<RemoveWhitelistedExtensionIxArgs> for RemoveWhitelistedExtensionIxData {
    fn from(args: RemoveWhitelistedExtensionIxArgs) -> Self {
        Self(args)
    }
}
impl RemoveWhitelistedExtensionIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != REMOVE_WHITELISTED_EXTENSION_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let ext_program: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(RemoveWhitelistedExtensionIxArgs {
                ext_program,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&REMOVE_WHITELISTED_EXTENSION_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.ext_program, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn remove_whitelisted_extension_ix_with_program_id(
    program_id: Pubkey,
    keys: RemoveWhitelistedExtensionKeys,
    args: RemoveWhitelistedExtensionIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; REMOVE_WHITELISTED_EXTENSION_IX_ACCOUNTS_LEN] = keys.into();
    let data: RemoveWhitelistedExtensionIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn remove_whitelisted_extension_ix(
    keys: RemoveWhitelistedExtensionKeys,
    args: RemoveWhitelistedExtensionIxArgs,
) -> std::io::Result<Instruction> {
    remove_whitelisted_extension_ix_with_program_id(M_SWAP_PROGRAM_ID, keys, args)
}
pub fn remove_whitelisted_extension_invoke_with_program_id(
    program_id: Pubkey,
    accounts: RemoveWhitelistedExtensionAccounts<'_, '_>,
    args: RemoveWhitelistedExtensionIxArgs,
) -> ProgramResult {
    let keys: RemoveWhitelistedExtensionKeys = accounts.into();
    let ix = remove_whitelisted_extension_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn remove_whitelisted_extension_invoke(
    accounts: RemoveWhitelistedExtensionAccounts<'_, '_>,
    args: RemoveWhitelistedExtensionIxArgs,
) -> ProgramResult {
    remove_whitelisted_extension_invoke_with_program_id(
        M_SWAP_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn remove_whitelisted_extension_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: RemoveWhitelistedExtensionAccounts<'_, '_>,
    args: RemoveWhitelistedExtensionIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: RemoveWhitelistedExtensionKeys = accounts.into();
    let ix = remove_whitelisted_extension_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn remove_whitelisted_extension_invoke_signed(
    accounts: RemoveWhitelistedExtensionAccounts<'_, '_>,
    args: RemoveWhitelistedExtensionIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    remove_whitelisted_extension_invoke_signed_with_program_id(
        M_SWAP_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn remove_whitelisted_extension_verify_account_keys(
    accounts: RemoveWhitelistedExtensionAccounts<'_, '_>,
    keys: RemoveWhitelistedExtensionKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.admin.key, keys.admin),
        (*accounts.swap_global.key, keys.swap_global),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn remove_whitelisted_extension_verify_writable_privileges<'me, 'info>(
    accounts: RemoveWhitelistedExtensionAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.admin, accounts.swap_global] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn remove_whitelisted_extension_verify_signer_privileges<'me, 'info>(
    accounts: RemoveWhitelistedExtensionAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn remove_whitelisted_extension_verify_account_privileges<'me, 'info>(
    accounts: RemoveWhitelistedExtensionAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    remove_whitelisted_extension_verify_writable_privileges(accounts)?;
    remove_whitelisted_extension_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const REMOVE_WHITELISTED_UNWRAPPER_IX_ACCOUNTS_LEN: usize = 2;
#[derive(Copy, Clone, Debug)]
pub struct RemoveWhitelistedUnwrapperAccounts<'me, 'info> {
    pub admin: &'me AccountInfo<'info>,
    pub swap_global: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct RemoveWhitelistedUnwrapperKeys {
    pub admin: Pubkey,
    pub swap_global: Pubkey,
}
impl From<RemoveWhitelistedUnwrapperAccounts<'_, '_>>
for RemoveWhitelistedUnwrapperKeys {
    fn from(accounts: RemoveWhitelistedUnwrapperAccounts) -> Self {
        Self {
            admin: *accounts.admin.key,
            swap_global: *accounts.swap_global.key,
        }
    }
}
impl From<RemoveWhitelistedUnwrapperKeys>
for [AccountMeta; REMOVE_WHITELISTED_UNWRAPPER_IX_ACCOUNTS_LEN] {
    fn from(keys: RemoveWhitelistedUnwrapperKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.admin,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.swap_global,
                is_signer: false,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; REMOVE_WHITELISTED_UNWRAPPER_IX_ACCOUNTS_LEN]>
for RemoveWhitelistedUnwrapperKeys {
    fn from(pubkeys: [Pubkey; REMOVE_WHITELISTED_UNWRAPPER_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin: pubkeys[0],
            swap_global: pubkeys[1],
        }
    }
}
impl<'info> From<RemoveWhitelistedUnwrapperAccounts<'_, 'info>>
for [AccountInfo<'info>; REMOVE_WHITELISTED_UNWRAPPER_IX_ACCOUNTS_LEN] {
    fn from(accounts: RemoveWhitelistedUnwrapperAccounts<'_, 'info>) -> Self {
        [accounts.admin.clone(), accounts.swap_global.clone()]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; REMOVE_WHITELISTED_UNWRAPPER_IX_ACCOUNTS_LEN]>
for RemoveWhitelistedUnwrapperAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; REMOVE_WHITELISTED_UNWRAPPER_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            admin: &arr[0],
            swap_global: &arr[1],
        }
    }
}
pub const REMOVE_WHITELISTED_UNWRAPPER_IX_DISCM: [u8; 8usize] = [
    166, 23, 120, 95, 66, 168, 192, 163,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct RemoveWhitelistedUnwrapperIxArgs {
    pub authority: Pubkey,
}
#[derive(Clone, Debug, PartialEq)]
pub struct RemoveWhitelistedUnwrapperIxData(pub RemoveWhitelistedUnwrapperIxArgs);
impl From<RemoveWhitelistedUnwrapperIxArgs> for RemoveWhitelistedUnwrapperIxData {
    fn from(args: RemoveWhitelistedUnwrapperIxArgs) -> Self {
        Self(args)
    }
}
impl RemoveWhitelistedUnwrapperIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != REMOVE_WHITELISTED_UNWRAPPER_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let authority: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(RemoveWhitelistedUnwrapperIxArgs {
                authority,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&REMOVE_WHITELISTED_UNWRAPPER_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.authority, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn remove_whitelisted_unwrapper_ix_with_program_id(
    program_id: Pubkey,
    keys: RemoveWhitelistedUnwrapperKeys,
    args: RemoveWhitelistedUnwrapperIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; REMOVE_WHITELISTED_UNWRAPPER_IX_ACCOUNTS_LEN] = keys.into();
    let data: RemoveWhitelistedUnwrapperIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn remove_whitelisted_unwrapper_ix(
    keys: RemoveWhitelistedUnwrapperKeys,
    args: RemoveWhitelistedUnwrapperIxArgs,
) -> std::io::Result<Instruction> {
    remove_whitelisted_unwrapper_ix_with_program_id(M_SWAP_PROGRAM_ID, keys, args)
}
pub fn remove_whitelisted_unwrapper_invoke_with_program_id(
    program_id: Pubkey,
    accounts: RemoveWhitelistedUnwrapperAccounts<'_, '_>,
    args: RemoveWhitelistedUnwrapperIxArgs,
) -> ProgramResult {
    let keys: RemoveWhitelistedUnwrapperKeys = accounts.into();
    let ix = remove_whitelisted_unwrapper_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn remove_whitelisted_unwrapper_invoke(
    accounts: RemoveWhitelistedUnwrapperAccounts<'_, '_>,
    args: RemoveWhitelistedUnwrapperIxArgs,
) -> ProgramResult {
    remove_whitelisted_unwrapper_invoke_with_program_id(
        M_SWAP_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn remove_whitelisted_unwrapper_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: RemoveWhitelistedUnwrapperAccounts<'_, '_>,
    args: RemoveWhitelistedUnwrapperIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: RemoveWhitelistedUnwrapperKeys = accounts.into();
    let ix = remove_whitelisted_unwrapper_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn remove_whitelisted_unwrapper_invoke_signed(
    accounts: RemoveWhitelistedUnwrapperAccounts<'_, '_>,
    args: RemoveWhitelistedUnwrapperIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    remove_whitelisted_unwrapper_invoke_signed_with_program_id(
        M_SWAP_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn remove_whitelisted_unwrapper_verify_account_keys(
    accounts: RemoveWhitelistedUnwrapperAccounts<'_, '_>,
    keys: RemoveWhitelistedUnwrapperKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.admin.key, keys.admin),
        (*accounts.swap_global.key, keys.swap_global),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn remove_whitelisted_unwrapper_verify_writable_privileges<'me, 'info>(
    accounts: RemoveWhitelistedUnwrapperAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.admin, accounts.swap_global] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn remove_whitelisted_unwrapper_verify_signer_privileges<'me, 'info>(
    accounts: RemoveWhitelistedUnwrapperAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn remove_whitelisted_unwrapper_verify_account_privileges<'me, 'info>(
    accounts: RemoveWhitelistedUnwrapperAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    remove_whitelisted_unwrapper_verify_writable_privileges(accounts)?;
    remove_whitelisted_unwrapper_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SWAP_IX_ACCOUNTS_LEN: usize = 24;
#[derive(Copy, Clone, Debug)]
pub struct SwapAccounts<'me, 'info> {
    pub signer: &'me AccountInfo<'info>,
    pub wrap_authority: &'me AccountInfo<'info>,
    pub unwrap_authority: &'me AccountInfo<'info>,
    pub swap_global: &'me AccountInfo<'info>,
    pub from_global: &'me AccountInfo<'info>,
    pub to_global: &'me AccountInfo<'info>,
    pub from_mint: &'me AccountInfo<'info>,
    pub to_mint: &'me AccountInfo<'info>,
    pub m_mint: &'me AccountInfo<'info>,
    pub from_token_account: &'me AccountInfo<'info>,
    pub to_token_account: &'me AccountInfo<'info>,
    pub swap_m_account: &'me AccountInfo<'info>,
    pub from_m_vault_auth: &'me AccountInfo<'info>,
    pub to_m_vault_auth: &'me AccountInfo<'info>,
    pub from_mint_authority: &'me AccountInfo<'info>,
    pub to_mint_authority: &'me AccountInfo<'info>,
    pub from_m_vault: &'me AccountInfo<'info>,
    pub to_m_vault: &'me AccountInfo<'info>,
    pub from_token_program: &'me AccountInfo<'info>,
    pub to_token_program: &'me AccountInfo<'info>,
    pub m_token_program: &'me AccountInfo<'info>,
    pub from_ext_program: &'me AccountInfo<'info>,
    pub to_ext_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SwapKeys {
    pub signer: Pubkey,
    pub wrap_authority: Pubkey,
    pub unwrap_authority: Pubkey,
    pub swap_global: Pubkey,
    pub from_global: Pubkey,
    pub to_global: Pubkey,
    pub from_mint: Pubkey,
    pub to_mint: Pubkey,
    pub m_mint: Pubkey,
    pub from_token_account: Pubkey,
    pub to_token_account: Pubkey,
    pub swap_m_account: Pubkey,
    pub from_m_vault_auth: Pubkey,
    pub to_m_vault_auth: Pubkey,
    pub from_mint_authority: Pubkey,
    pub to_mint_authority: Pubkey,
    pub from_m_vault: Pubkey,
    pub to_m_vault: Pubkey,
    pub from_token_program: Pubkey,
    pub to_token_program: Pubkey,
    pub m_token_program: Pubkey,
    pub from_ext_program: Pubkey,
    pub to_ext_program: Pubkey,
    pub system_program: Pubkey,
}
impl From<SwapAccounts<'_, '_>> for SwapKeys {
    fn from(accounts: SwapAccounts) -> Self {
        Self {
            signer: *accounts.signer.key,
            wrap_authority: *accounts.wrap_authority.key,
            unwrap_authority: *accounts.unwrap_authority.key,
            swap_global: *accounts.swap_global.key,
            from_global: *accounts.from_global.key,
            to_global: *accounts.to_global.key,
            from_mint: *accounts.from_mint.key,
            to_mint: *accounts.to_mint.key,
            m_mint: *accounts.m_mint.key,
            from_token_account: *accounts.from_token_account.key,
            to_token_account: *accounts.to_token_account.key,
            swap_m_account: *accounts.swap_m_account.key,
            from_m_vault_auth: *accounts.from_m_vault_auth.key,
            to_m_vault_auth: *accounts.to_m_vault_auth.key,
            from_mint_authority: *accounts.from_mint_authority.key,
            to_mint_authority: *accounts.to_mint_authority.key,
            from_m_vault: *accounts.from_m_vault.key,
            to_m_vault: *accounts.to_m_vault.key,
            from_token_program: *accounts.from_token_program.key,
            to_token_program: *accounts.to_token_program.key,
            m_token_program: *accounts.m_token_program.key,
            from_ext_program: *accounts.from_ext_program.key,
            to_ext_program: *accounts.to_ext_program.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<SwapKeys> for [AccountMeta; SWAP_IX_ACCOUNTS_LEN] {
    fn from(keys: SwapKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.signer,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.wrap_authority,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.unwrap_authority,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.swap_global,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.from_global,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.to_global,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.from_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.to_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.m_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.from_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.to_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.swap_m_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.from_m_vault_auth,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.to_m_vault_auth,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.from_mint_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.to_mint_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.from_m_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.to_m_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.from_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.to_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.m_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.from_ext_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.to_ext_program,
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
impl From<[Pubkey; SWAP_IX_ACCOUNTS_LEN]> for SwapKeys {
    fn from(pubkeys: [Pubkey; SWAP_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            signer: pubkeys[0],
            wrap_authority: pubkeys[1],
            unwrap_authority: pubkeys[2],
            swap_global: pubkeys[3],
            from_global: pubkeys[4],
            to_global: pubkeys[5],
            from_mint: pubkeys[6],
            to_mint: pubkeys[7],
            m_mint: pubkeys[8],
            from_token_account: pubkeys[9],
            to_token_account: pubkeys[10],
            swap_m_account: pubkeys[11],
            from_m_vault_auth: pubkeys[12],
            to_m_vault_auth: pubkeys[13],
            from_mint_authority: pubkeys[14],
            to_mint_authority: pubkeys[15],
            from_m_vault: pubkeys[16],
            to_m_vault: pubkeys[17],
            from_token_program: pubkeys[18],
            to_token_program: pubkeys[19],
            m_token_program: pubkeys[20],
            from_ext_program: pubkeys[21],
            to_ext_program: pubkeys[22],
            system_program: pubkeys[23],
        }
    }
}
impl<'info> From<SwapAccounts<'_, 'info>>
for [AccountInfo<'info>; SWAP_IX_ACCOUNTS_LEN] {
    fn from(accounts: SwapAccounts<'_, 'info>) -> Self {
        [
            accounts.signer.clone(),
            accounts.wrap_authority.clone(),
            accounts.unwrap_authority.clone(),
            accounts.swap_global.clone(),
            accounts.from_global.clone(),
            accounts.to_global.clone(),
            accounts.from_mint.clone(),
            accounts.to_mint.clone(),
            accounts.m_mint.clone(),
            accounts.from_token_account.clone(),
            accounts.to_token_account.clone(),
            accounts.swap_m_account.clone(),
            accounts.from_m_vault_auth.clone(),
            accounts.to_m_vault_auth.clone(),
            accounts.from_mint_authority.clone(),
            accounts.to_mint_authority.clone(),
            accounts.from_m_vault.clone(),
            accounts.to_m_vault.clone(),
            accounts.from_token_program.clone(),
            accounts.to_token_program.clone(),
            accounts.m_token_program.clone(),
            accounts.from_ext_program.clone(),
            accounts.to_ext_program.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SWAP_IX_ACCOUNTS_LEN]>
for SwapAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; SWAP_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            signer: &arr[0],
            wrap_authority: &arr[1],
            unwrap_authority: &arr[2],
            swap_global: &arr[3],
            from_global: &arr[4],
            to_global: &arr[5],
            from_mint: &arr[6],
            to_mint: &arr[7],
            m_mint: &arr[8],
            from_token_account: &arr[9],
            to_token_account: &arr[10],
            swap_m_account: &arr[11],
            from_m_vault_auth: &arr[12],
            to_m_vault_auth: &arr[13],
            from_mint_authority: &arr[14],
            to_mint_authority: &arr[15],
            from_m_vault: &arr[16],
            to_m_vault: &arr[17],
            from_token_program: &arr[18],
            to_token_program: &arr[19],
            m_token_program: &arr[20],
            from_ext_program: &arr[21],
            to_ext_program: &arr[22],
            system_program: &arr[23],
        }
    }
}
pub const SWAP_IX_DISCM: [u8; 8usize] = [248, 198, 158, 145, 225, 117, 135, 200];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SwapIxArgs {
    pub amount: u64,
    pub remaining_accounts_split_idx: u8,
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
        let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let remaining_accounts_split_idx: u8 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(SwapIxArgs {
                amount,
                remaining_accounts_split_idx,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SWAP_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.amount, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.0.remaining_accounts_split_idx,
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
    swap_ix_with_program_id(M_SWAP_PROGRAM_ID, keys, args)
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
    swap_invoke_with_program_id(M_SWAP_PROGRAM_ID, accounts, args)
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
    swap_invoke_signed_with_program_id(M_SWAP_PROGRAM_ID, accounts, args, seeds)
}
pub fn swap_verify_account_keys(
    accounts: SwapAccounts<'_, '_>,
    keys: SwapKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.signer.key, keys.signer),
        (*accounts.wrap_authority.key, keys.wrap_authority),
        (*accounts.unwrap_authority.key, keys.unwrap_authority),
        (*accounts.swap_global.key, keys.swap_global),
        (*accounts.from_global.key, keys.from_global),
        (*accounts.to_global.key, keys.to_global),
        (*accounts.from_mint.key, keys.from_mint),
        (*accounts.to_mint.key, keys.to_mint),
        (*accounts.m_mint.key, keys.m_mint),
        (*accounts.from_token_account.key, keys.from_token_account),
        (*accounts.to_token_account.key, keys.to_token_account),
        (*accounts.swap_m_account.key, keys.swap_m_account),
        (*accounts.from_m_vault_auth.key, keys.from_m_vault_auth),
        (*accounts.to_m_vault_auth.key, keys.to_m_vault_auth),
        (*accounts.from_mint_authority.key, keys.from_mint_authority),
        (*accounts.to_mint_authority.key, keys.to_mint_authority),
        (*accounts.from_m_vault.key, keys.from_m_vault),
        (*accounts.to_m_vault.key, keys.to_m_vault),
        (*accounts.from_token_program.key, keys.from_token_program),
        (*accounts.to_token_program.key, keys.to_token_program),
        (*accounts.m_token_program.key, keys.m_token_program),
        (*accounts.from_ext_program.key, keys.from_ext_program),
        (*accounts.to_ext_program.key, keys.to_ext_program),
        (*accounts.system_program.key, keys.system_program),
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
        accounts.from_global,
        accounts.to_global,
        accounts.from_mint,
        accounts.to_mint,
        accounts.from_token_account,
        accounts.to_token_account,
        accounts.swap_m_account,
        accounts.from_m_vault,
        accounts.to_m_vault,
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
    for should_be_signer in [
        accounts.signer,
        accounts.wrap_authority,
        accounts.unwrap_authority,
    ] {
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
pub const UNWRAP_IX_ACCOUNTS_LEN: usize = 15;
#[derive(Copy, Clone, Debug)]
pub struct UnwrapAccounts<'me, 'info> {
    pub signer: &'me AccountInfo<'info>,
    pub unwrap_authority: &'me AccountInfo<'info>,
    pub swap_global: &'me AccountInfo<'info>,
    pub from_global: &'me AccountInfo<'info>,
    pub from_mint: &'me AccountInfo<'info>,
    pub m_mint: &'me AccountInfo<'info>,
    pub m_token_account: &'me AccountInfo<'info>,
    pub from_token_account: &'me AccountInfo<'info>,
    pub from_m_vault_auth: &'me AccountInfo<'info>,
    pub from_mint_authority: &'me AccountInfo<'info>,
    pub from_m_vault: &'me AccountInfo<'info>,
    pub from_token_program: &'me AccountInfo<'info>,
    pub m_token_program: &'me AccountInfo<'info>,
    pub from_ext_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UnwrapKeys {
    pub signer: Pubkey,
    pub unwrap_authority: Pubkey,
    pub swap_global: Pubkey,
    pub from_global: Pubkey,
    pub from_mint: Pubkey,
    pub m_mint: Pubkey,
    pub m_token_account: Pubkey,
    pub from_token_account: Pubkey,
    pub from_m_vault_auth: Pubkey,
    pub from_mint_authority: Pubkey,
    pub from_m_vault: Pubkey,
    pub from_token_program: Pubkey,
    pub m_token_program: Pubkey,
    pub from_ext_program: Pubkey,
    pub system_program: Pubkey,
}
impl From<UnwrapAccounts<'_, '_>> for UnwrapKeys {
    fn from(accounts: UnwrapAccounts) -> Self {
        Self {
            signer: *accounts.signer.key,
            unwrap_authority: *accounts.unwrap_authority.key,
            swap_global: *accounts.swap_global.key,
            from_global: *accounts.from_global.key,
            from_mint: *accounts.from_mint.key,
            m_mint: *accounts.m_mint.key,
            m_token_account: *accounts.m_token_account.key,
            from_token_account: *accounts.from_token_account.key,
            from_m_vault_auth: *accounts.from_m_vault_auth.key,
            from_mint_authority: *accounts.from_mint_authority.key,
            from_m_vault: *accounts.from_m_vault.key,
            from_token_program: *accounts.from_token_program.key,
            m_token_program: *accounts.m_token_program.key,
            from_ext_program: *accounts.from_ext_program.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<UnwrapKeys> for [AccountMeta; UNWRAP_IX_ACCOUNTS_LEN] {
    fn from(keys: UnwrapKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.signer,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.unwrap_authority,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.swap_global,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.from_global,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.from_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.m_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.m_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.from_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.from_m_vault_auth,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.from_mint_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.from_m_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.from_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.m_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.from_ext_program,
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
impl From<[Pubkey; UNWRAP_IX_ACCOUNTS_LEN]> for UnwrapKeys {
    fn from(pubkeys: [Pubkey; UNWRAP_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            signer: pubkeys[0],
            unwrap_authority: pubkeys[1],
            swap_global: pubkeys[2],
            from_global: pubkeys[3],
            from_mint: pubkeys[4],
            m_mint: pubkeys[5],
            m_token_account: pubkeys[6],
            from_token_account: pubkeys[7],
            from_m_vault_auth: pubkeys[8],
            from_mint_authority: pubkeys[9],
            from_m_vault: pubkeys[10],
            from_token_program: pubkeys[11],
            m_token_program: pubkeys[12],
            from_ext_program: pubkeys[13],
            system_program: pubkeys[14],
        }
    }
}
impl<'info> From<UnwrapAccounts<'_, 'info>>
for [AccountInfo<'info>; UNWRAP_IX_ACCOUNTS_LEN] {
    fn from(accounts: UnwrapAccounts<'_, 'info>) -> Self {
        [
            accounts.signer.clone(),
            accounts.unwrap_authority.clone(),
            accounts.swap_global.clone(),
            accounts.from_global.clone(),
            accounts.from_mint.clone(),
            accounts.m_mint.clone(),
            accounts.m_token_account.clone(),
            accounts.from_token_account.clone(),
            accounts.from_m_vault_auth.clone(),
            accounts.from_mint_authority.clone(),
            accounts.from_m_vault.clone(),
            accounts.from_token_program.clone(),
            accounts.m_token_program.clone(),
            accounts.from_ext_program.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; UNWRAP_IX_ACCOUNTS_LEN]>
for UnwrapAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; UNWRAP_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            signer: &arr[0],
            unwrap_authority: &arr[1],
            swap_global: &arr[2],
            from_global: &arr[3],
            from_mint: &arr[4],
            m_mint: &arr[5],
            m_token_account: &arr[6],
            from_token_account: &arr[7],
            from_m_vault_auth: &arr[8],
            from_mint_authority: &arr[9],
            from_m_vault: &arr[10],
            from_token_program: &arr[11],
            m_token_program: &arr[12],
            from_ext_program: &arr[13],
            system_program: &arr[14],
        }
    }
}
pub const UNWRAP_IX_DISCM: [u8; 8usize] = [126, 175, 198, 14, 212, 69, 50, 44];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UnwrapIxArgs {
    pub amount: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct UnwrapIxData(pub UnwrapIxArgs);
impl From<UnwrapIxArgs> for UnwrapIxData {
    fn from(args: UnwrapIxArgs) -> Self {
        Self(args)
    }
}
impl UnwrapIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UNWRAP_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(Self(UnwrapIxArgs { amount }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UNWRAP_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.amount, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn unwrap_ix_with_program_id(
    program_id: Pubkey,
    keys: UnwrapKeys,
    args: UnwrapIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; UNWRAP_IX_ACCOUNTS_LEN] = keys.into();
    let data: UnwrapIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn unwrap_ix(keys: UnwrapKeys, args: UnwrapIxArgs) -> std::io::Result<Instruction> {
    unwrap_ix_with_program_id(M_SWAP_PROGRAM_ID, keys, args)
}
pub fn unwrap_invoke_with_program_id(
    program_id: Pubkey,
    accounts: UnwrapAccounts<'_, '_>,
    args: UnwrapIxArgs,
) -> ProgramResult {
    let keys: UnwrapKeys = accounts.into();
    let ix = unwrap_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn unwrap_invoke(
    accounts: UnwrapAccounts<'_, '_>,
    args: UnwrapIxArgs,
) -> ProgramResult {
    unwrap_invoke_with_program_id(M_SWAP_PROGRAM_ID, accounts, args)
}
pub fn unwrap_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: UnwrapAccounts<'_, '_>,
    args: UnwrapIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: UnwrapKeys = accounts.into();
    let ix = unwrap_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn unwrap_invoke_signed(
    accounts: UnwrapAccounts<'_, '_>,
    args: UnwrapIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    unwrap_invoke_signed_with_program_id(M_SWAP_PROGRAM_ID, accounts, args, seeds)
}
pub fn unwrap_verify_account_keys(
    accounts: UnwrapAccounts<'_, '_>,
    keys: UnwrapKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.signer.key, keys.signer),
        (*accounts.unwrap_authority.key, keys.unwrap_authority),
        (*accounts.swap_global.key, keys.swap_global),
        (*accounts.from_global.key, keys.from_global),
        (*accounts.from_mint.key, keys.from_mint),
        (*accounts.m_mint.key, keys.m_mint),
        (*accounts.m_token_account.key, keys.m_token_account),
        (*accounts.from_token_account.key, keys.from_token_account),
        (*accounts.from_m_vault_auth.key, keys.from_m_vault_auth),
        (*accounts.from_mint_authority.key, keys.from_mint_authority),
        (*accounts.from_m_vault.key, keys.from_m_vault),
        (*accounts.from_token_program.key, keys.from_token_program),
        (*accounts.m_token_program.key, keys.m_token_program),
        (*accounts.from_ext_program.key, keys.from_ext_program),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn unwrap_verify_writable_privileges<'me, 'info>(
    accounts: UnwrapAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.from_global,
        accounts.from_mint,
        accounts.m_token_account,
        accounts.from_token_account,
        accounts.from_m_vault,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn unwrap_verify_signer_privileges<'me, 'info>(
    accounts: UnwrapAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.signer, accounts.unwrap_authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn unwrap_verify_account_privileges<'me, 'info>(
    accounts: UnwrapAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    unwrap_verify_writable_privileges(accounts)?;
    unwrap_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const WHITELIST_EXTENSION_IX_ACCOUNTS_LEN: usize = 5;
#[derive(Copy, Clone, Debug)]
pub struct WhitelistExtensionAccounts<'me, 'info> {
    pub admin: &'me AccountInfo<'info>,
    pub swap_global: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub ext_program: &'me AccountInfo<'info>,
    pub ext_mint: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct WhitelistExtensionKeys {
    pub admin: Pubkey,
    pub swap_global: Pubkey,
    pub system_program: Pubkey,
    pub ext_program: Pubkey,
    pub ext_mint: Pubkey,
}
impl From<WhitelistExtensionAccounts<'_, '_>> for WhitelistExtensionKeys {
    fn from(accounts: WhitelistExtensionAccounts) -> Self {
        Self {
            admin: *accounts.admin.key,
            swap_global: *accounts.swap_global.key,
            system_program: *accounts.system_program.key,
            ext_program: *accounts.ext_program.key,
            ext_mint: *accounts.ext_mint.key,
        }
    }
}
impl From<WhitelistExtensionKeys>
for [AccountMeta; WHITELIST_EXTENSION_IX_ACCOUNTS_LEN] {
    fn from(keys: WhitelistExtensionKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.admin,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.swap_global,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.system_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.ext_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.ext_mint,
                is_signer: false,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; WHITELIST_EXTENSION_IX_ACCOUNTS_LEN]> for WhitelistExtensionKeys {
    fn from(pubkeys: [Pubkey; WHITELIST_EXTENSION_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin: pubkeys[0],
            swap_global: pubkeys[1],
            system_program: pubkeys[2],
            ext_program: pubkeys[3],
            ext_mint: pubkeys[4],
        }
    }
}
impl<'info> From<WhitelistExtensionAccounts<'_, 'info>>
for [AccountInfo<'info>; WHITELIST_EXTENSION_IX_ACCOUNTS_LEN] {
    fn from(accounts: WhitelistExtensionAccounts<'_, 'info>) -> Self {
        [
            accounts.admin.clone(),
            accounts.swap_global.clone(),
            accounts.system_program.clone(),
            accounts.ext_program.clone(),
            accounts.ext_mint.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; WHITELIST_EXTENSION_IX_ACCOUNTS_LEN]>
for WhitelistExtensionAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; WHITELIST_EXTENSION_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            admin: &arr[0],
            swap_global: &arr[1],
            system_program: &arr[2],
            ext_program: &arr[3],
            ext_mint: &arr[4],
        }
    }
}
pub const WHITELIST_EXTENSION_IX_DISCM: [u8; 8usize] = [
    186, 175, 23, 231, 77, 201, 205, 165,
];
#[derive(Clone, Debug, PartialEq)]
pub struct WhitelistExtensionIxData;
impl WhitelistExtensionIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != WHITELIST_EXTENSION_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&WHITELIST_EXTENSION_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn whitelist_extension_ix_with_program_id(
    program_id: Pubkey,
    keys: WhitelistExtensionKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; WHITELIST_EXTENSION_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: WhitelistExtensionIxData.try_to_vec()?,
    })
}
pub fn whitelist_extension_ix(
    keys: WhitelistExtensionKeys,
) -> std::io::Result<Instruction> {
    whitelist_extension_ix_with_program_id(M_SWAP_PROGRAM_ID, keys)
}
pub fn whitelist_extension_invoke_with_program_id(
    program_id: Pubkey,
    accounts: WhitelistExtensionAccounts<'_, '_>,
) -> ProgramResult {
    let keys: WhitelistExtensionKeys = accounts.into();
    let ix = whitelist_extension_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn whitelist_extension_invoke(
    accounts: WhitelistExtensionAccounts<'_, '_>,
) -> ProgramResult {
    whitelist_extension_invoke_with_program_id(M_SWAP_PROGRAM_ID, accounts)
}
pub fn whitelist_extension_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: WhitelistExtensionAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: WhitelistExtensionKeys = accounts.into();
    let ix = whitelist_extension_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn whitelist_extension_invoke_signed(
    accounts: WhitelistExtensionAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    whitelist_extension_invoke_signed_with_program_id(M_SWAP_PROGRAM_ID, accounts, seeds)
}
pub fn whitelist_extension_verify_account_keys(
    accounts: WhitelistExtensionAccounts<'_, '_>,
    keys: WhitelistExtensionKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.admin.key, keys.admin),
        (*accounts.swap_global.key, keys.swap_global),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.ext_program.key, keys.ext_program),
        (*accounts.ext_mint.key, keys.ext_mint),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn whitelist_extension_verify_writable_privileges<'me, 'info>(
    accounts: WhitelistExtensionAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.admin, accounts.swap_global, accounts.ext_mint] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn whitelist_extension_verify_signer_privileges<'me, 'info>(
    accounts: WhitelistExtensionAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn whitelist_extension_verify_account_privileges<'me, 'info>(
    accounts: WhitelistExtensionAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    whitelist_extension_verify_writable_privileges(accounts)?;
    whitelist_extension_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const WHITELIST_UNWRAPPER_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct WhitelistUnwrapperAccounts<'me, 'info> {
    pub admin: &'me AccountInfo<'info>,
    pub swap_global: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct WhitelistUnwrapperKeys {
    pub admin: Pubkey,
    pub swap_global: Pubkey,
    pub system_program: Pubkey,
}
impl From<WhitelistUnwrapperAccounts<'_, '_>> for WhitelistUnwrapperKeys {
    fn from(accounts: WhitelistUnwrapperAccounts) -> Self {
        Self {
            admin: *accounts.admin.key,
            swap_global: *accounts.swap_global.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<WhitelistUnwrapperKeys>
for [AccountMeta; WHITELIST_UNWRAPPER_IX_ACCOUNTS_LEN] {
    fn from(keys: WhitelistUnwrapperKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.admin,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.swap_global,
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
impl From<[Pubkey; WHITELIST_UNWRAPPER_IX_ACCOUNTS_LEN]> for WhitelistUnwrapperKeys {
    fn from(pubkeys: [Pubkey; WHITELIST_UNWRAPPER_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin: pubkeys[0],
            swap_global: pubkeys[1],
            system_program: pubkeys[2],
        }
    }
}
impl<'info> From<WhitelistUnwrapperAccounts<'_, 'info>>
for [AccountInfo<'info>; WHITELIST_UNWRAPPER_IX_ACCOUNTS_LEN] {
    fn from(accounts: WhitelistUnwrapperAccounts<'_, 'info>) -> Self {
        [
            accounts.admin.clone(),
            accounts.swap_global.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; WHITELIST_UNWRAPPER_IX_ACCOUNTS_LEN]>
for WhitelistUnwrapperAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; WHITELIST_UNWRAPPER_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            admin: &arr[0],
            swap_global: &arr[1],
            system_program: &arr[2],
        }
    }
}
pub const WHITELIST_UNWRAPPER_IX_DISCM: [u8; 8usize] = [
    219, 87, 23, 47, 189, 191, 123, 235,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WhitelistUnwrapperIxArgs {
    pub authority: Pubkey,
}
#[derive(Clone, Debug, PartialEq)]
pub struct WhitelistUnwrapperIxData(pub WhitelistUnwrapperIxArgs);
impl From<WhitelistUnwrapperIxArgs> for WhitelistUnwrapperIxData {
    fn from(args: WhitelistUnwrapperIxArgs) -> Self {
        Self(args)
    }
}
impl WhitelistUnwrapperIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != WHITELIST_UNWRAPPER_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let authority: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(WhitelistUnwrapperIxArgs {
                authority,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&WHITELIST_UNWRAPPER_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.authority, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn whitelist_unwrapper_ix_with_program_id(
    program_id: Pubkey,
    keys: WhitelistUnwrapperKeys,
    args: WhitelistUnwrapperIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; WHITELIST_UNWRAPPER_IX_ACCOUNTS_LEN] = keys.into();
    let data: WhitelistUnwrapperIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn whitelist_unwrapper_ix(
    keys: WhitelistUnwrapperKeys,
    args: WhitelistUnwrapperIxArgs,
) -> std::io::Result<Instruction> {
    whitelist_unwrapper_ix_with_program_id(M_SWAP_PROGRAM_ID, keys, args)
}
pub fn whitelist_unwrapper_invoke_with_program_id(
    program_id: Pubkey,
    accounts: WhitelistUnwrapperAccounts<'_, '_>,
    args: WhitelistUnwrapperIxArgs,
) -> ProgramResult {
    let keys: WhitelistUnwrapperKeys = accounts.into();
    let ix = whitelist_unwrapper_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn whitelist_unwrapper_invoke(
    accounts: WhitelistUnwrapperAccounts<'_, '_>,
    args: WhitelistUnwrapperIxArgs,
) -> ProgramResult {
    whitelist_unwrapper_invoke_with_program_id(M_SWAP_PROGRAM_ID, accounts, args)
}
pub fn whitelist_unwrapper_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: WhitelistUnwrapperAccounts<'_, '_>,
    args: WhitelistUnwrapperIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: WhitelistUnwrapperKeys = accounts.into();
    let ix = whitelist_unwrapper_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn whitelist_unwrapper_invoke_signed(
    accounts: WhitelistUnwrapperAccounts<'_, '_>,
    args: WhitelistUnwrapperIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    whitelist_unwrapper_invoke_signed_with_program_id(
        M_SWAP_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn whitelist_unwrapper_verify_account_keys(
    accounts: WhitelistUnwrapperAccounts<'_, '_>,
    keys: WhitelistUnwrapperKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.admin.key, keys.admin),
        (*accounts.swap_global.key, keys.swap_global),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn whitelist_unwrapper_verify_writable_privileges<'me, 'info>(
    accounts: WhitelistUnwrapperAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.admin, accounts.swap_global] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn whitelist_unwrapper_verify_signer_privileges<'me, 'info>(
    accounts: WhitelistUnwrapperAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn whitelist_unwrapper_verify_account_privileges<'me, 'info>(
    accounts: WhitelistUnwrapperAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    whitelist_unwrapper_verify_writable_privileges(accounts)?;
    whitelist_unwrapper_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const WRAP_IX_ACCOUNTS_LEN: usize = 15;
#[derive(Copy, Clone, Debug)]
pub struct WrapAccounts<'me, 'info> {
    pub signer: &'me AccountInfo<'info>,
    pub wrap_authority: &'me AccountInfo<'info>,
    pub swap_global: &'me AccountInfo<'info>,
    pub to_global: &'me AccountInfo<'info>,
    pub to_mint: &'me AccountInfo<'info>,
    pub m_mint: &'me AccountInfo<'info>,
    pub m_token_account: &'me AccountInfo<'info>,
    pub to_token_account: &'me AccountInfo<'info>,
    pub to_m_vault_auth: &'me AccountInfo<'info>,
    pub to_mint_authority: &'me AccountInfo<'info>,
    pub to_m_vault: &'me AccountInfo<'info>,
    pub to_token_program: &'me AccountInfo<'info>,
    pub m_token_program: &'me AccountInfo<'info>,
    pub to_ext_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct WrapKeys {
    pub signer: Pubkey,
    pub wrap_authority: Pubkey,
    pub swap_global: Pubkey,
    pub to_global: Pubkey,
    pub to_mint: Pubkey,
    pub m_mint: Pubkey,
    pub m_token_account: Pubkey,
    pub to_token_account: Pubkey,
    pub to_m_vault_auth: Pubkey,
    pub to_mint_authority: Pubkey,
    pub to_m_vault: Pubkey,
    pub to_token_program: Pubkey,
    pub m_token_program: Pubkey,
    pub to_ext_program: Pubkey,
    pub system_program: Pubkey,
}
impl From<WrapAccounts<'_, '_>> for WrapKeys {
    fn from(accounts: WrapAccounts) -> Self {
        Self {
            signer: *accounts.signer.key,
            wrap_authority: *accounts.wrap_authority.key,
            swap_global: *accounts.swap_global.key,
            to_global: *accounts.to_global.key,
            to_mint: *accounts.to_mint.key,
            m_mint: *accounts.m_mint.key,
            m_token_account: *accounts.m_token_account.key,
            to_token_account: *accounts.to_token_account.key,
            to_m_vault_auth: *accounts.to_m_vault_auth.key,
            to_mint_authority: *accounts.to_mint_authority.key,
            to_m_vault: *accounts.to_m_vault.key,
            to_token_program: *accounts.to_token_program.key,
            m_token_program: *accounts.m_token_program.key,
            to_ext_program: *accounts.to_ext_program.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<WrapKeys> for [AccountMeta; WRAP_IX_ACCOUNTS_LEN] {
    fn from(keys: WrapKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.signer,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.wrap_authority,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.swap_global,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.to_global,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.to_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.m_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.m_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.to_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.to_m_vault_auth,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.to_mint_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.to_m_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.to_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.m_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.to_ext_program,
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
impl From<[Pubkey; WRAP_IX_ACCOUNTS_LEN]> for WrapKeys {
    fn from(pubkeys: [Pubkey; WRAP_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            signer: pubkeys[0],
            wrap_authority: pubkeys[1],
            swap_global: pubkeys[2],
            to_global: pubkeys[3],
            to_mint: pubkeys[4],
            m_mint: pubkeys[5],
            m_token_account: pubkeys[6],
            to_token_account: pubkeys[7],
            to_m_vault_auth: pubkeys[8],
            to_mint_authority: pubkeys[9],
            to_m_vault: pubkeys[10],
            to_token_program: pubkeys[11],
            m_token_program: pubkeys[12],
            to_ext_program: pubkeys[13],
            system_program: pubkeys[14],
        }
    }
}
impl<'info> From<WrapAccounts<'_, 'info>>
for [AccountInfo<'info>; WRAP_IX_ACCOUNTS_LEN] {
    fn from(accounts: WrapAccounts<'_, 'info>) -> Self {
        [
            accounts.signer.clone(),
            accounts.wrap_authority.clone(),
            accounts.swap_global.clone(),
            accounts.to_global.clone(),
            accounts.to_mint.clone(),
            accounts.m_mint.clone(),
            accounts.m_token_account.clone(),
            accounts.to_token_account.clone(),
            accounts.to_m_vault_auth.clone(),
            accounts.to_mint_authority.clone(),
            accounts.to_m_vault.clone(),
            accounts.to_token_program.clone(),
            accounts.m_token_program.clone(),
            accounts.to_ext_program.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; WRAP_IX_ACCOUNTS_LEN]>
for WrapAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; WRAP_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            signer: &arr[0],
            wrap_authority: &arr[1],
            swap_global: &arr[2],
            to_global: &arr[3],
            to_mint: &arr[4],
            m_mint: &arr[5],
            m_token_account: &arr[6],
            to_token_account: &arr[7],
            to_m_vault_auth: &arr[8],
            to_mint_authority: &arr[9],
            to_m_vault: &arr[10],
            to_token_program: &arr[11],
            m_token_program: &arr[12],
            to_ext_program: &arr[13],
            system_program: &arr[14],
        }
    }
}
pub const WRAP_IX_DISCM: [u8; 8usize] = [178, 40, 10, 189, 228, 129, 186, 140];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WrapIxArgs {
    pub amount: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct WrapIxData(pub WrapIxArgs);
impl From<WrapIxArgs> for WrapIxData {
    fn from(args: WrapIxArgs) -> Self {
        Self(args)
    }
}
impl WrapIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != WRAP_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(Self(WrapIxArgs { amount }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&WRAP_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.amount, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn wrap_ix_with_program_id(
    program_id: Pubkey,
    keys: WrapKeys,
    args: WrapIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; WRAP_IX_ACCOUNTS_LEN] = keys.into();
    let data: WrapIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn wrap_ix(keys: WrapKeys, args: WrapIxArgs) -> std::io::Result<Instruction> {
    wrap_ix_with_program_id(M_SWAP_PROGRAM_ID, keys, args)
}
pub fn wrap_invoke_with_program_id(
    program_id: Pubkey,
    accounts: WrapAccounts<'_, '_>,
    args: WrapIxArgs,
) -> ProgramResult {
    let keys: WrapKeys = accounts.into();
    let ix = wrap_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn wrap_invoke(accounts: WrapAccounts<'_, '_>, args: WrapIxArgs) -> ProgramResult {
    wrap_invoke_with_program_id(M_SWAP_PROGRAM_ID, accounts, args)
}
pub fn wrap_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: WrapAccounts<'_, '_>,
    args: WrapIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: WrapKeys = accounts.into();
    let ix = wrap_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn wrap_invoke_signed(
    accounts: WrapAccounts<'_, '_>,
    args: WrapIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    wrap_invoke_signed_with_program_id(M_SWAP_PROGRAM_ID, accounts, args, seeds)
}
pub fn wrap_verify_account_keys(
    accounts: WrapAccounts<'_, '_>,
    keys: WrapKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.signer.key, keys.signer),
        (*accounts.wrap_authority.key, keys.wrap_authority),
        (*accounts.swap_global.key, keys.swap_global),
        (*accounts.to_global.key, keys.to_global),
        (*accounts.to_mint.key, keys.to_mint),
        (*accounts.m_mint.key, keys.m_mint),
        (*accounts.m_token_account.key, keys.m_token_account),
        (*accounts.to_token_account.key, keys.to_token_account),
        (*accounts.to_m_vault_auth.key, keys.to_m_vault_auth),
        (*accounts.to_mint_authority.key, keys.to_mint_authority),
        (*accounts.to_m_vault.key, keys.to_m_vault),
        (*accounts.to_token_program.key, keys.to_token_program),
        (*accounts.m_token_program.key, keys.m_token_program),
        (*accounts.to_ext_program.key, keys.to_ext_program),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn wrap_verify_writable_privileges<'me, 'info>(
    accounts: WrapAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.to_global,
        accounts.to_mint,
        accounts.m_token_account,
        accounts.to_token_account,
        accounts.to_m_vault,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn wrap_verify_signer_privileges<'me, 'info>(
    accounts: WrapAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.signer, accounts.wrap_authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn wrap_verify_account_privileges<'me, 'info>(
    accounts: WrapAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    wrap_verify_writable_privileges(accounts)?;
    wrap_verify_signer_privileges(accounts)?;
    Ok(())
}
