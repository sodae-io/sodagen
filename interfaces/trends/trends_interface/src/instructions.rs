use solana_pubkey::Pubkey;
use solana_cpi::{invoke, invoke_signed};
use solana_instruction::{AccountMeta, Instruction};
use solana_account_info::AccountInfo;
use solana_program_error::ProgramError;
use std::io::Read;
#[allow(unused_imports)]
use crate::*;
#[derive(Clone, Debug, PartialEq)]
pub enum TrendsProgramIx {
    ClaimCreatorFee,
    ClaimProtocolFee,
    CollectRaydiumCreatorFee,
    Initialize,
    InitializePool(InitializePoolIxArgs),
    Migrate,
    Swap(SwapIxArgs),
}
impl TrendsProgramIx {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        if buf.starts_with(&CLAIM_CREATOR_FEE_IX_DISCM) {
            return Ok(Self::ClaimCreatorFee);
        }
        if buf.starts_with(&CLAIM_PROTOCOL_FEE_IX_DISCM) {
            return Ok(Self::ClaimProtocolFee);
        }
        if buf.starts_with(&COLLECT_RAYDIUM_CREATOR_FEE_IX_DISCM) {
            return Ok(Self::CollectRaydiumCreatorFee);
        }
        if buf.starts_with(&INITIALIZE_IX_DISCM) {
            return Ok(Self::Initialize);
        }
        if buf.starts_with(&INITIALIZE_POOL_IX_DISCM) {
            let mut reader = &buf[INITIALIZE_POOL_IX_DISCM.len()..];
            let params = if reader.is_empty() {
                Default::default()
            } else {
                <InitializePoolParams>::deserialize(&mut reader)?
            };
            return Ok(Self::InitializePool(InitializePoolIxArgs { params }));
        }
        if buf.starts_with(&MIGRATE_IX_DISCM) {
            return Ok(Self::Migrate);
        }
        if buf.starts_with(&SWAP_IX_DISCM) {
            let mut reader = &buf[SWAP_IX_DISCM.len()..];
            let params = if reader.is_empty() {
                Default::default()
            } else {
                <SwapParams>::deserialize(&mut reader)?
            };
            return Ok(Self::Swap(SwapIxArgs { params }));
        }
        Err(std::io::Error::from(std::io::ErrorKind::InvalidData))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        match self {
            Self::ClaimCreatorFee => writer.write_all(&CLAIM_CREATOR_FEE_IX_DISCM),
            Self::ClaimProtocolFee => writer.write_all(&CLAIM_PROTOCOL_FEE_IX_DISCM),
            Self::CollectRaydiumCreatorFee => {
                writer.write_all(&COLLECT_RAYDIUM_CREATOR_FEE_IX_DISCM)
            }
            Self::Initialize => writer.write_all(&INITIALIZE_IX_DISCM),
            Self::InitializePool(args) => {
                writer.write_all(&INITIALIZE_POOL_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.params, &mut writer)?;
                Ok(())
            }
            Self::Migrate => writer.write_all(&MIGRATE_IX_DISCM),
            Self::Swap(args) => {
                writer.write_all(&SWAP_IX_DISCM)?;
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
pub const CLAIM_CREATOR_FEE_IX_ACCOUNTS_LEN: usize = 10;
#[derive(Copy, Clone, Debug)]
pub struct ClaimCreatorFeeAccounts<'me, 'info> {
    pub config: &'me AccountInfo<'info>,
    pub pool: &'me AccountInfo<'info>,
    pub pool_authority: &'me AccountInfo<'info>,
    pub quote_mint: &'me AccountInfo<'info>,
    pub quote_token_account: &'me AccountInfo<'info>,
    pub quote_vault: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub quote_token_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ClaimCreatorFeeKeys {
    pub config: Pubkey,
    pub pool: Pubkey,
    pub pool_authority: Pubkey,
    pub quote_mint: Pubkey,
    pub quote_token_account: Pubkey,
    pub quote_vault: Pubkey,
    pub authority: Pubkey,
    pub quote_token_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<ClaimCreatorFeeAccounts<'_, '_>> for ClaimCreatorFeeKeys {
    fn from(accounts: ClaimCreatorFeeAccounts) -> Self {
        Self {
            config: *accounts.config.key,
            pool: *accounts.pool.key,
            pool_authority: *accounts.pool_authority.key,
            quote_mint: *accounts.quote_mint.key,
            quote_token_account: *accounts.quote_token_account.key,
            quote_vault: *accounts.quote_vault.key,
            authority: *accounts.authority.key,
            quote_token_program: *accounts.quote_token_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<ClaimCreatorFeeKeys> for [AccountMeta; CLAIM_CREATOR_FEE_IX_ACCOUNTS_LEN] {
    fn from(keys: ClaimCreatorFeeKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.config,
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
                pubkey: keys.quote_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.quote_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.quote_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.authority,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.quote_token_program,
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
impl From<[Pubkey; CLAIM_CREATOR_FEE_IX_ACCOUNTS_LEN]> for ClaimCreatorFeeKeys {
    fn from(pubkeys: [Pubkey; CLAIM_CREATOR_FEE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            config: pubkeys[0],
            pool: pubkeys[1],
            pool_authority: pubkeys[2],
            quote_mint: pubkeys[3],
            quote_token_account: pubkeys[4],
            quote_vault: pubkeys[5],
            authority: pubkeys[6],
            quote_token_program: pubkeys[7],
            event_authority: pubkeys[8],
            program: pubkeys[9],
        }
    }
}
impl<'info> From<ClaimCreatorFeeAccounts<'_, 'info>>
for [AccountInfo<'info>; CLAIM_CREATOR_FEE_IX_ACCOUNTS_LEN] {
    fn from(accounts: ClaimCreatorFeeAccounts<'_, 'info>) -> Self {
        [
            accounts.config.clone(),
            accounts.pool.clone(),
            accounts.pool_authority.clone(),
            accounts.quote_mint.clone(),
            accounts.quote_token_account.clone(),
            accounts.quote_vault.clone(),
            accounts.authority.clone(),
            accounts.quote_token_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CLAIM_CREATOR_FEE_IX_ACCOUNTS_LEN]>
for ClaimCreatorFeeAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; CLAIM_CREATOR_FEE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            config: &arr[0],
            pool: &arr[1],
            pool_authority: &arr[2],
            quote_mint: &arr[3],
            quote_token_account: &arr[4],
            quote_vault: &arr[5],
            authority: &arr[6],
            quote_token_program: &arr[7],
            event_authority: &arr[8],
            program: &arr[9],
        }
    }
}
pub const CLAIM_CREATOR_FEE_IX_DISCM: [u8; 8usize] = [
    26, 97, 138, 203, 132, 171, 141, 252,
];
#[derive(Clone, Debug, PartialEq)]
pub struct ClaimCreatorFeeIxData;
impl ClaimCreatorFeeIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CLAIM_CREATOR_FEE_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CLAIM_CREATOR_FEE_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn claim_creator_fee_ix_with_program_id(
    program_id: Pubkey,
    keys: ClaimCreatorFeeKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CLAIM_CREATOR_FEE_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: ClaimCreatorFeeIxData.try_to_vec()?,
    })
}
pub fn claim_creator_fee_ix(keys: ClaimCreatorFeeKeys) -> std::io::Result<Instruction> {
    claim_creator_fee_ix_with_program_id(TRENDS_PROGRAM_ID, keys)
}
pub fn claim_creator_fee_invoke_with_program_id(
    program_id: Pubkey,
    accounts: ClaimCreatorFeeAccounts<'_, '_>,
) -> ProgramResult {
    let keys: ClaimCreatorFeeKeys = accounts.into();
    let ix = claim_creator_fee_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn claim_creator_fee_invoke(
    accounts: ClaimCreatorFeeAccounts<'_, '_>,
) -> ProgramResult {
    claim_creator_fee_invoke_with_program_id(TRENDS_PROGRAM_ID, accounts)
}
pub fn claim_creator_fee_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: ClaimCreatorFeeAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: ClaimCreatorFeeKeys = accounts.into();
    let ix = claim_creator_fee_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn claim_creator_fee_invoke_signed(
    accounts: ClaimCreatorFeeAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    claim_creator_fee_invoke_signed_with_program_id(TRENDS_PROGRAM_ID, accounts, seeds)
}
pub fn claim_creator_fee_verify_account_keys(
    accounts: ClaimCreatorFeeAccounts<'_, '_>,
    keys: ClaimCreatorFeeKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.config.key, keys.config),
        (*accounts.pool.key, keys.pool),
        (*accounts.pool_authority.key, keys.pool_authority),
        (*accounts.quote_mint.key, keys.quote_mint),
        (*accounts.quote_token_account.key, keys.quote_token_account),
        (*accounts.quote_vault.key, keys.quote_vault),
        (*accounts.authority.key, keys.authority),
        (*accounts.quote_token_program.key, keys.quote_token_program),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn claim_creator_fee_verify_writable_privileges<'me, 'info>(
    accounts: ClaimCreatorFeeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.pool,
        accounts.quote_token_account,
        accounts.quote_vault,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn claim_creator_fee_verify_signer_privileges<'me, 'info>(
    accounts: ClaimCreatorFeeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn claim_creator_fee_verify_account_privileges<'me, 'info>(
    accounts: ClaimCreatorFeeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    claim_creator_fee_verify_writable_privileges(accounts)?;
    claim_creator_fee_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CLAIM_PROTOCOL_FEE_IX_ACCOUNTS_LEN: usize = 10;
#[derive(Copy, Clone, Debug)]
pub struct ClaimProtocolFeeAccounts<'me, 'info> {
    pub config: &'me AccountInfo<'info>,
    pub pool: &'me AccountInfo<'info>,
    pub pool_authority: &'me AccountInfo<'info>,
    pub quote_mint: &'me AccountInfo<'info>,
    pub quote_token_account: &'me AccountInfo<'info>,
    pub quote_vault: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub quote_token_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ClaimProtocolFeeKeys {
    pub config: Pubkey,
    pub pool: Pubkey,
    pub pool_authority: Pubkey,
    pub quote_mint: Pubkey,
    pub quote_token_account: Pubkey,
    pub quote_vault: Pubkey,
    pub authority: Pubkey,
    pub quote_token_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<ClaimProtocolFeeAccounts<'_, '_>> for ClaimProtocolFeeKeys {
    fn from(accounts: ClaimProtocolFeeAccounts) -> Self {
        Self {
            config: *accounts.config.key,
            pool: *accounts.pool.key,
            pool_authority: *accounts.pool_authority.key,
            quote_mint: *accounts.quote_mint.key,
            quote_token_account: *accounts.quote_token_account.key,
            quote_vault: *accounts.quote_vault.key,
            authority: *accounts.authority.key,
            quote_token_program: *accounts.quote_token_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<ClaimProtocolFeeKeys> for [AccountMeta; CLAIM_PROTOCOL_FEE_IX_ACCOUNTS_LEN] {
    fn from(keys: ClaimProtocolFeeKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.config,
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
                pubkey: keys.quote_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.quote_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.quote_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.authority,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.quote_token_program,
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
impl From<[Pubkey; CLAIM_PROTOCOL_FEE_IX_ACCOUNTS_LEN]> for ClaimProtocolFeeKeys {
    fn from(pubkeys: [Pubkey; CLAIM_PROTOCOL_FEE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            config: pubkeys[0],
            pool: pubkeys[1],
            pool_authority: pubkeys[2],
            quote_mint: pubkeys[3],
            quote_token_account: pubkeys[4],
            quote_vault: pubkeys[5],
            authority: pubkeys[6],
            quote_token_program: pubkeys[7],
            event_authority: pubkeys[8],
            program: pubkeys[9],
        }
    }
}
impl<'info> From<ClaimProtocolFeeAccounts<'_, 'info>>
for [AccountInfo<'info>; CLAIM_PROTOCOL_FEE_IX_ACCOUNTS_LEN] {
    fn from(accounts: ClaimProtocolFeeAccounts<'_, 'info>) -> Self {
        [
            accounts.config.clone(),
            accounts.pool.clone(),
            accounts.pool_authority.clone(),
            accounts.quote_mint.clone(),
            accounts.quote_token_account.clone(),
            accounts.quote_vault.clone(),
            accounts.authority.clone(),
            accounts.quote_token_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CLAIM_PROTOCOL_FEE_IX_ACCOUNTS_LEN]>
for ClaimProtocolFeeAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; CLAIM_PROTOCOL_FEE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            config: &arr[0],
            pool: &arr[1],
            pool_authority: &arr[2],
            quote_mint: &arr[3],
            quote_token_account: &arr[4],
            quote_vault: &arr[5],
            authority: &arr[6],
            quote_token_program: &arr[7],
            event_authority: &arr[8],
            program: &arr[9],
        }
    }
}
pub const CLAIM_PROTOCOL_FEE_IX_DISCM: [u8; 8usize] = [
    165, 228, 133, 48, 99, 249, 255, 33,
];
#[derive(Clone, Debug, PartialEq)]
pub struct ClaimProtocolFeeIxData;
impl ClaimProtocolFeeIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CLAIM_PROTOCOL_FEE_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CLAIM_PROTOCOL_FEE_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn claim_protocol_fee_ix_with_program_id(
    program_id: Pubkey,
    keys: ClaimProtocolFeeKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CLAIM_PROTOCOL_FEE_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: ClaimProtocolFeeIxData.try_to_vec()?,
    })
}
pub fn claim_protocol_fee_ix(
    keys: ClaimProtocolFeeKeys,
) -> std::io::Result<Instruction> {
    claim_protocol_fee_ix_with_program_id(TRENDS_PROGRAM_ID, keys)
}
pub fn claim_protocol_fee_invoke_with_program_id(
    program_id: Pubkey,
    accounts: ClaimProtocolFeeAccounts<'_, '_>,
) -> ProgramResult {
    let keys: ClaimProtocolFeeKeys = accounts.into();
    let ix = claim_protocol_fee_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn claim_protocol_fee_invoke(
    accounts: ClaimProtocolFeeAccounts<'_, '_>,
) -> ProgramResult {
    claim_protocol_fee_invoke_with_program_id(TRENDS_PROGRAM_ID, accounts)
}
pub fn claim_protocol_fee_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: ClaimProtocolFeeAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: ClaimProtocolFeeKeys = accounts.into();
    let ix = claim_protocol_fee_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn claim_protocol_fee_invoke_signed(
    accounts: ClaimProtocolFeeAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    claim_protocol_fee_invoke_signed_with_program_id(TRENDS_PROGRAM_ID, accounts, seeds)
}
pub fn claim_protocol_fee_verify_account_keys(
    accounts: ClaimProtocolFeeAccounts<'_, '_>,
    keys: ClaimProtocolFeeKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.config.key, keys.config),
        (*accounts.pool.key, keys.pool),
        (*accounts.pool_authority.key, keys.pool_authority),
        (*accounts.quote_mint.key, keys.quote_mint),
        (*accounts.quote_token_account.key, keys.quote_token_account),
        (*accounts.quote_vault.key, keys.quote_vault),
        (*accounts.authority.key, keys.authority),
        (*accounts.quote_token_program.key, keys.quote_token_program),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn claim_protocol_fee_verify_writable_privileges<'me, 'info>(
    accounts: ClaimProtocolFeeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.pool,
        accounts.quote_token_account,
        accounts.quote_vault,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn claim_protocol_fee_verify_signer_privileges<'me, 'info>(
    accounts: ClaimProtocolFeeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn claim_protocol_fee_verify_account_privileges<'me, 'info>(
    accounts: ClaimProtocolFeeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    claim_protocol_fee_verify_writable_privileges(accounts)?;
    claim_protocol_fee_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const COLLECT_RAYDIUM_CREATOR_FEE_IX_ACCOUNTS_LEN: usize = 23;
#[derive(Copy, Clone, Debug)]
pub struct CollectRaydiumCreatorFeeAccounts<'me, 'info> {
    pub config: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub pool_authority: &'me AccountInfo<'info>,
    pub raydium_program: &'me AccountInfo<'info>,
    pub raydium_authority: &'me AccountInfo<'info>,
    pub raydium_pool_state: &'me AccountInfo<'info>,
    pub raydium_amm_config: &'me AccountInfo<'info>,
    pub raydium_token_0_vault: &'me AccountInfo<'info>,
    pub raydium_token_1_vault: &'me AccountInfo<'info>,
    pub vault_0_mint: &'me AccountInfo<'info>,
    pub vault_1_mint: &'me AccountInfo<'info>,
    pub creator_token_0: &'me AccountInfo<'info>,
    pub creator_token_1: &'me AccountInfo<'info>,
    pub destination_token_0: &'me AccountInfo<'info>,
    pub destination_token_1: &'me AccountInfo<'info>,
    pub protocol_destination_token_0: &'me AccountInfo<'info>,
    pub protocol_destination_token_1: &'me AccountInfo<'info>,
    pub token_0_program: &'me AccountInfo<'info>,
    pub token_1_program: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CollectRaydiumCreatorFeeKeys {
    pub config: Pubkey,
    pub authority: Pubkey,
    pub pool_authority: Pubkey,
    pub raydium_program: Pubkey,
    pub raydium_authority: Pubkey,
    pub raydium_pool_state: Pubkey,
    pub raydium_amm_config: Pubkey,
    pub raydium_token_0_vault: Pubkey,
    pub raydium_token_1_vault: Pubkey,
    pub vault_0_mint: Pubkey,
    pub vault_1_mint: Pubkey,
    pub creator_token_0: Pubkey,
    pub creator_token_1: Pubkey,
    pub destination_token_0: Pubkey,
    pub destination_token_1: Pubkey,
    pub protocol_destination_token_0: Pubkey,
    pub protocol_destination_token_1: Pubkey,
    pub token_0_program: Pubkey,
    pub token_1_program: Pubkey,
    pub associated_token_program: Pubkey,
    pub system_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<CollectRaydiumCreatorFeeAccounts<'_, '_>> for CollectRaydiumCreatorFeeKeys {
    fn from(accounts: CollectRaydiumCreatorFeeAccounts) -> Self {
        Self {
            config: *accounts.config.key,
            authority: *accounts.authority.key,
            pool_authority: *accounts.pool_authority.key,
            raydium_program: *accounts.raydium_program.key,
            raydium_authority: *accounts.raydium_authority.key,
            raydium_pool_state: *accounts.raydium_pool_state.key,
            raydium_amm_config: *accounts.raydium_amm_config.key,
            raydium_token_0_vault: *accounts.raydium_token_0_vault.key,
            raydium_token_1_vault: *accounts.raydium_token_1_vault.key,
            vault_0_mint: *accounts.vault_0_mint.key,
            vault_1_mint: *accounts.vault_1_mint.key,
            creator_token_0: *accounts.creator_token_0.key,
            creator_token_1: *accounts.creator_token_1.key,
            destination_token_0: *accounts.destination_token_0.key,
            destination_token_1: *accounts.destination_token_1.key,
            protocol_destination_token_0: *accounts.protocol_destination_token_0.key,
            protocol_destination_token_1: *accounts.protocol_destination_token_1.key,
            token_0_program: *accounts.token_0_program.key,
            token_1_program: *accounts.token_1_program.key,
            associated_token_program: *accounts.associated_token_program.key,
            system_program: *accounts.system_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<CollectRaydiumCreatorFeeKeys>
for [AccountMeta; COLLECT_RAYDIUM_CREATOR_FEE_IX_ACCOUNTS_LEN] {
    fn from(keys: CollectRaydiumCreatorFeeKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.authority,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pool_authority,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.raydium_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.raydium_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.raydium_pool_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.raydium_amm_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.raydium_token_0_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.raydium_token_1_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vault_0_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault_1_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.creator_token_0,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.creator_token_1,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.destination_token_0,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.destination_token_1,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.protocol_destination_token_0,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.protocol_destination_token_1,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_0_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_1_program,
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
impl From<[Pubkey; COLLECT_RAYDIUM_CREATOR_FEE_IX_ACCOUNTS_LEN]>
for CollectRaydiumCreatorFeeKeys {
    fn from(pubkeys: [Pubkey; COLLECT_RAYDIUM_CREATOR_FEE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            config: pubkeys[0],
            authority: pubkeys[1],
            pool_authority: pubkeys[2],
            raydium_program: pubkeys[3],
            raydium_authority: pubkeys[4],
            raydium_pool_state: pubkeys[5],
            raydium_amm_config: pubkeys[6],
            raydium_token_0_vault: pubkeys[7],
            raydium_token_1_vault: pubkeys[8],
            vault_0_mint: pubkeys[9],
            vault_1_mint: pubkeys[10],
            creator_token_0: pubkeys[11],
            creator_token_1: pubkeys[12],
            destination_token_0: pubkeys[13],
            destination_token_1: pubkeys[14],
            protocol_destination_token_0: pubkeys[15],
            protocol_destination_token_1: pubkeys[16],
            token_0_program: pubkeys[17],
            token_1_program: pubkeys[18],
            associated_token_program: pubkeys[19],
            system_program: pubkeys[20],
            event_authority: pubkeys[21],
            program: pubkeys[22],
        }
    }
}
impl<'info> From<CollectRaydiumCreatorFeeAccounts<'_, 'info>>
for [AccountInfo<'info>; COLLECT_RAYDIUM_CREATOR_FEE_IX_ACCOUNTS_LEN] {
    fn from(accounts: CollectRaydiumCreatorFeeAccounts<'_, 'info>) -> Self {
        [
            accounts.config.clone(),
            accounts.authority.clone(),
            accounts.pool_authority.clone(),
            accounts.raydium_program.clone(),
            accounts.raydium_authority.clone(),
            accounts.raydium_pool_state.clone(),
            accounts.raydium_amm_config.clone(),
            accounts.raydium_token_0_vault.clone(),
            accounts.raydium_token_1_vault.clone(),
            accounts.vault_0_mint.clone(),
            accounts.vault_1_mint.clone(),
            accounts.creator_token_0.clone(),
            accounts.creator_token_1.clone(),
            accounts.destination_token_0.clone(),
            accounts.destination_token_1.clone(),
            accounts.protocol_destination_token_0.clone(),
            accounts.protocol_destination_token_1.clone(),
            accounts.token_0_program.clone(),
            accounts.token_1_program.clone(),
            accounts.associated_token_program.clone(),
            accounts.system_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; COLLECT_RAYDIUM_CREATOR_FEE_IX_ACCOUNTS_LEN]>
for CollectRaydiumCreatorFeeAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; COLLECT_RAYDIUM_CREATOR_FEE_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            config: &arr[0],
            authority: &arr[1],
            pool_authority: &arr[2],
            raydium_program: &arr[3],
            raydium_authority: &arr[4],
            raydium_pool_state: &arr[5],
            raydium_amm_config: &arr[6],
            raydium_token_0_vault: &arr[7],
            raydium_token_1_vault: &arr[8],
            vault_0_mint: &arr[9],
            vault_1_mint: &arr[10],
            creator_token_0: &arr[11],
            creator_token_1: &arr[12],
            destination_token_0: &arr[13],
            destination_token_1: &arr[14],
            protocol_destination_token_0: &arr[15],
            protocol_destination_token_1: &arr[16],
            token_0_program: &arr[17],
            token_1_program: &arr[18],
            associated_token_program: &arr[19],
            system_program: &arr[20],
            event_authority: &arr[21],
            program: &arr[22],
        }
    }
}
pub const COLLECT_RAYDIUM_CREATOR_FEE_IX_DISCM: [u8; 8usize] = [
    172, 243, 148, 144, 247, 140, 97, 46,
];
#[derive(Clone, Debug, PartialEq)]
pub struct CollectRaydiumCreatorFeeIxData;
impl CollectRaydiumCreatorFeeIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != COLLECT_RAYDIUM_CREATOR_FEE_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&COLLECT_RAYDIUM_CREATOR_FEE_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn collect_raydium_creator_fee_ix_with_program_id(
    program_id: Pubkey,
    keys: CollectRaydiumCreatorFeeKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; COLLECT_RAYDIUM_CREATOR_FEE_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: CollectRaydiumCreatorFeeIxData.try_to_vec()?,
    })
}
pub fn collect_raydium_creator_fee_ix(
    keys: CollectRaydiumCreatorFeeKeys,
) -> std::io::Result<Instruction> {
    collect_raydium_creator_fee_ix_with_program_id(TRENDS_PROGRAM_ID, keys)
}
pub fn collect_raydium_creator_fee_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CollectRaydiumCreatorFeeAccounts<'_, '_>,
) -> ProgramResult {
    let keys: CollectRaydiumCreatorFeeKeys = accounts.into();
    let ix = collect_raydium_creator_fee_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn collect_raydium_creator_fee_invoke(
    accounts: CollectRaydiumCreatorFeeAccounts<'_, '_>,
) -> ProgramResult {
    collect_raydium_creator_fee_invoke_with_program_id(TRENDS_PROGRAM_ID, accounts)
}
pub fn collect_raydium_creator_fee_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CollectRaydiumCreatorFeeAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CollectRaydiumCreatorFeeKeys = accounts.into();
    let ix = collect_raydium_creator_fee_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn collect_raydium_creator_fee_invoke_signed(
    accounts: CollectRaydiumCreatorFeeAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    collect_raydium_creator_fee_invoke_signed_with_program_id(
        TRENDS_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn collect_raydium_creator_fee_verify_account_keys(
    accounts: CollectRaydiumCreatorFeeAccounts<'_, '_>,
    keys: CollectRaydiumCreatorFeeKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.config.key, keys.config),
        (*accounts.authority.key, keys.authority),
        (*accounts.pool_authority.key, keys.pool_authority),
        (*accounts.raydium_program.key, keys.raydium_program),
        (*accounts.raydium_authority.key, keys.raydium_authority),
        (*accounts.raydium_pool_state.key, keys.raydium_pool_state),
        (*accounts.raydium_amm_config.key, keys.raydium_amm_config),
        (*accounts.raydium_token_0_vault.key, keys.raydium_token_0_vault),
        (*accounts.raydium_token_1_vault.key, keys.raydium_token_1_vault),
        (*accounts.vault_0_mint.key, keys.vault_0_mint),
        (*accounts.vault_1_mint.key, keys.vault_1_mint),
        (*accounts.creator_token_0.key, keys.creator_token_0),
        (*accounts.creator_token_1.key, keys.creator_token_1),
        (*accounts.destination_token_0.key, keys.destination_token_0),
        (*accounts.destination_token_1.key, keys.destination_token_1),
        (*accounts.protocol_destination_token_0.key, keys.protocol_destination_token_0),
        (*accounts.protocol_destination_token_1.key, keys.protocol_destination_token_1),
        (*accounts.token_0_program.key, keys.token_0_program),
        (*accounts.token_1_program.key, keys.token_1_program),
        (*accounts.associated_token_program.key, keys.associated_token_program),
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
pub fn collect_raydium_creator_fee_verify_writable_privileges<'me, 'info>(
    accounts: CollectRaydiumCreatorFeeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.authority,
        accounts.pool_authority,
        accounts.raydium_pool_state,
        accounts.raydium_token_0_vault,
        accounts.raydium_token_1_vault,
        accounts.creator_token_0,
        accounts.creator_token_1,
        accounts.destination_token_0,
        accounts.destination_token_1,
        accounts.protocol_destination_token_0,
        accounts.protocol_destination_token_1,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn collect_raydium_creator_fee_verify_signer_privileges<'me, 'info>(
    accounts: CollectRaydiumCreatorFeeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn collect_raydium_creator_fee_verify_account_privileges<'me, 'info>(
    accounts: CollectRaydiumCreatorFeeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    collect_raydium_creator_fee_verify_writable_privileges(accounts)?;
    collect_raydium_creator_fee_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const INITIALIZE_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct InitializeAccounts<'me, 'info> {
    pub config: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct InitializeKeys {
    pub config: Pubkey,
    pub authority: Pubkey,
    pub system_program: Pubkey,
}
impl From<InitializeAccounts<'_, '_>> for InitializeKeys {
    fn from(accounts: InitializeAccounts) -> Self {
        Self {
            config: *accounts.config.key,
            authority: *accounts.authority.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<InitializeKeys> for [AccountMeta; INITIALIZE_IX_ACCOUNTS_LEN] {
    fn from(keys: InitializeKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.config,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.authority,
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
impl From<[Pubkey; INITIALIZE_IX_ACCOUNTS_LEN]> for InitializeKeys {
    fn from(pubkeys: [Pubkey; INITIALIZE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            config: pubkeys[0],
            authority: pubkeys[1],
            system_program: pubkeys[2],
        }
    }
}
impl<'info> From<InitializeAccounts<'_, 'info>>
for [AccountInfo<'info>; INITIALIZE_IX_ACCOUNTS_LEN] {
    fn from(accounts: InitializeAccounts<'_, 'info>) -> Self {
        [
            accounts.config.clone(),
            accounts.authority.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; INITIALIZE_IX_ACCOUNTS_LEN]>
for InitializeAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; INITIALIZE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            config: &arr[0],
            authority: &arr[1],
            system_program: &arr[2],
        }
    }
}
pub const INITIALIZE_IX_DISCM: [u8; 8usize] = [175, 175, 109, 31, 13, 152, 155, 237];
#[derive(Clone, Debug, PartialEq)]
pub struct InitializeIxData;
impl InitializeIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INITIALIZE_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INITIALIZE_IX_DISCM)
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
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; INITIALIZE_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: InitializeIxData.try_to_vec()?,
    })
}
pub fn initialize_ix(keys: InitializeKeys) -> std::io::Result<Instruction> {
    initialize_ix_with_program_id(TRENDS_PROGRAM_ID, keys)
}
pub fn initialize_invoke_with_program_id(
    program_id: Pubkey,
    accounts: InitializeAccounts<'_, '_>,
) -> ProgramResult {
    let keys: InitializeKeys = accounts.into();
    let ix = initialize_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn initialize_invoke(accounts: InitializeAccounts<'_, '_>) -> ProgramResult {
    initialize_invoke_with_program_id(TRENDS_PROGRAM_ID, accounts)
}
pub fn initialize_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: InitializeAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: InitializeKeys = accounts.into();
    let ix = initialize_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn initialize_invoke_signed(
    accounts: InitializeAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    initialize_invoke_signed_with_program_id(TRENDS_PROGRAM_ID, accounts, seeds)
}
pub fn initialize_verify_account_keys(
    accounts: InitializeAccounts<'_, '_>,
    keys: InitializeKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.config.key, keys.config),
        (*accounts.authority.key, keys.authority),
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
    for should_be_writable in [accounts.config, accounts.authority] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn initialize_verify_signer_privileges<'me, 'info>(
    accounts: InitializeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority] {
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
pub const INITIALIZE_POOL_IX_ACCOUNTS_LEN: usize = 13;
#[derive(Copy, Clone, Debug)]
pub struct InitializePoolAccounts<'me, 'info> {
    pub config: &'me AccountInfo<'info>,
    pub pool: &'me AccountInfo<'info>,
    pub pool_authority: &'me AccountInfo<'info>,
    pub base_mint: &'me AccountInfo<'info>,
    pub quote_mint: &'me AccountInfo<'info>,
    pub base_vault: &'me AccountInfo<'info>,
    pub quote_vault: &'me AccountInfo<'info>,
    pub creator: &'me AccountInfo<'info>,
    pub base_token_program: &'me AccountInfo<'info>,
    pub quote_token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct InitializePoolKeys {
    pub config: Pubkey,
    pub pool: Pubkey,
    pub pool_authority: Pubkey,
    pub base_mint: Pubkey,
    pub quote_mint: Pubkey,
    pub base_vault: Pubkey,
    pub quote_vault: Pubkey,
    pub creator: Pubkey,
    pub base_token_program: Pubkey,
    pub quote_token_program: Pubkey,
    pub system_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<InitializePoolAccounts<'_, '_>> for InitializePoolKeys {
    fn from(accounts: InitializePoolAccounts) -> Self {
        Self {
            config: *accounts.config.key,
            pool: *accounts.pool.key,
            pool_authority: *accounts.pool_authority.key,
            base_mint: *accounts.base_mint.key,
            quote_mint: *accounts.quote_mint.key,
            base_vault: *accounts.base_vault.key,
            quote_vault: *accounts.quote_vault.key,
            creator: *accounts.creator.key,
            base_token_program: *accounts.base_token_program.key,
            quote_token_program: *accounts.quote_token_program.key,
            system_program: *accounts.system_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<InitializePoolKeys> for [AccountMeta; INITIALIZE_POOL_IX_ACCOUNTS_LEN] {
    fn from(keys: InitializePoolKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.config,
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
                pubkey: keys.base_mint,
                is_signer: true,
                is_writable: true,
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
                pubkey: keys.creator,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.base_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.quote_token_program,
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
impl From<[Pubkey; INITIALIZE_POOL_IX_ACCOUNTS_LEN]> for InitializePoolKeys {
    fn from(pubkeys: [Pubkey; INITIALIZE_POOL_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            config: pubkeys[0],
            pool: pubkeys[1],
            pool_authority: pubkeys[2],
            base_mint: pubkeys[3],
            quote_mint: pubkeys[4],
            base_vault: pubkeys[5],
            quote_vault: pubkeys[6],
            creator: pubkeys[7],
            base_token_program: pubkeys[8],
            quote_token_program: pubkeys[9],
            system_program: pubkeys[10],
            event_authority: pubkeys[11],
            program: pubkeys[12],
        }
    }
}
impl<'info> From<InitializePoolAccounts<'_, 'info>>
for [AccountInfo<'info>; INITIALIZE_POOL_IX_ACCOUNTS_LEN] {
    fn from(accounts: InitializePoolAccounts<'_, 'info>) -> Self {
        [
            accounts.config.clone(),
            accounts.pool.clone(),
            accounts.pool_authority.clone(),
            accounts.base_mint.clone(),
            accounts.quote_mint.clone(),
            accounts.base_vault.clone(),
            accounts.quote_vault.clone(),
            accounts.creator.clone(),
            accounts.base_token_program.clone(),
            accounts.quote_token_program.clone(),
            accounts.system_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; INITIALIZE_POOL_IX_ACCOUNTS_LEN]>
for InitializePoolAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; INITIALIZE_POOL_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            config: &arr[0],
            pool: &arr[1],
            pool_authority: &arr[2],
            base_mint: &arr[3],
            quote_mint: &arr[4],
            base_vault: &arr[5],
            quote_vault: &arr[6],
            creator: &arr[7],
            base_token_program: &arr[8],
            quote_token_program: &arr[9],
            system_program: &arr[10],
            event_authority: &arr[11],
            program: &arr[12],
        }
    }
}
pub const INITIALIZE_POOL_IX_DISCM: [u8; 8usize] = [95, 180, 10, 172, 84, 174, 232, 40];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct InitializePoolIxArgs {
    pub params: InitializePoolParams,
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
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INITIALIZE_POOL_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let params = if reader.is_empty() {
            Default::default()
        } else {
            <InitializePoolParams>::deserialize(&mut reader)?
        };
        Ok(Self(InitializePoolIxArgs { params }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INITIALIZE_POOL_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.params, &mut writer)?;
        Ok(())
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
    initialize_pool_ix_with_program_id(TRENDS_PROGRAM_ID, keys, args)
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
    initialize_pool_invoke_with_program_id(TRENDS_PROGRAM_ID, accounts, args)
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
        TRENDS_PROGRAM_ID,
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
        (*accounts.config.key, keys.config),
        (*accounts.pool.key, keys.pool),
        (*accounts.pool_authority.key, keys.pool_authority),
        (*accounts.base_mint.key, keys.base_mint),
        (*accounts.quote_mint.key, keys.quote_mint),
        (*accounts.base_vault.key, keys.base_vault),
        (*accounts.quote_vault.key, keys.quote_vault),
        (*accounts.creator.key, keys.creator),
        (*accounts.base_token_program.key, keys.base_token_program),
        (*accounts.quote_token_program.key, keys.quote_token_program),
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
pub fn initialize_pool_verify_writable_privileges<'me, 'info>(
    accounts: InitializePoolAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.pool,
        accounts.base_mint,
        accounts.base_vault,
        accounts.quote_vault,
        accounts.creator,
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
    for should_be_signer in [accounts.base_mint, accounts.creator] {
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
pub const MIGRATE_IX_ACCOUNTS_LEN: usize = 27;
#[derive(Copy, Clone, Debug)]
pub struct MigrateAccounts<'me, 'info> {
    pub pool: &'me AccountInfo<'info>,
    pub pool_authority: &'me AccountInfo<'info>,
    pub base_mint: &'me AccountInfo<'info>,
    pub quote_mint: &'me AccountInfo<'info>,
    pub base_vault: &'me AccountInfo<'info>,
    pub quote_vault: &'me AccountInfo<'info>,
    pub migration_payer: &'me AccountInfo<'info>,
    pub raydium_program: &'me AccountInfo<'info>,
    pub raydium_amm_config: &'me AccountInfo<'info>,
    pub raydium_authority: &'me AccountInfo<'info>,
    pub raydium_pool_state: &'me AccountInfo<'info>,
    pub raydium_lp_mint: &'me AccountInfo<'info>,
    pub payer_lp_token: &'me AccountInfo<'info>,
    pub raydium_token_0_vault: &'me AccountInfo<'info>,
    pub raydium_token_1_vault: &'me AccountInfo<'info>,
    pub raydium_create_pool_fee: &'me AccountInfo<'info>,
    pub raydium_observation_state: &'me AccountInfo<'info>,
    pub raydium_permission: &'me AccountInfo<'info>,
    pub lp_authority: &'me AccountInfo<'info>,
    pub lp_authority_lp_token: &'me AccountInfo<'info>,
    pub base_token_program: &'me AccountInfo<'info>,
    pub quote_token_program: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MigrateKeys {
    pub pool: Pubkey,
    pub pool_authority: Pubkey,
    pub base_mint: Pubkey,
    pub quote_mint: Pubkey,
    pub base_vault: Pubkey,
    pub quote_vault: Pubkey,
    pub migration_payer: Pubkey,
    pub raydium_program: Pubkey,
    pub raydium_amm_config: Pubkey,
    pub raydium_authority: Pubkey,
    pub raydium_pool_state: Pubkey,
    pub raydium_lp_mint: Pubkey,
    pub payer_lp_token: Pubkey,
    pub raydium_token_0_vault: Pubkey,
    pub raydium_token_1_vault: Pubkey,
    pub raydium_create_pool_fee: Pubkey,
    pub raydium_observation_state: Pubkey,
    pub raydium_permission: Pubkey,
    pub lp_authority: Pubkey,
    pub lp_authority_lp_token: Pubkey,
    pub base_token_program: Pubkey,
    pub quote_token_program: Pubkey,
    pub token_program: Pubkey,
    pub associated_token_program: Pubkey,
    pub system_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<MigrateAccounts<'_, '_>> for MigrateKeys {
    fn from(accounts: MigrateAccounts) -> Self {
        Self {
            pool: *accounts.pool.key,
            pool_authority: *accounts.pool_authority.key,
            base_mint: *accounts.base_mint.key,
            quote_mint: *accounts.quote_mint.key,
            base_vault: *accounts.base_vault.key,
            quote_vault: *accounts.quote_vault.key,
            migration_payer: *accounts.migration_payer.key,
            raydium_program: *accounts.raydium_program.key,
            raydium_amm_config: *accounts.raydium_amm_config.key,
            raydium_authority: *accounts.raydium_authority.key,
            raydium_pool_state: *accounts.raydium_pool_state.key,
            raydium_lp_mint: *accounts.raydium_lp_mint.key,
            payer_lp_token: *accounts.payer_lp_token.key,
            raydium_token_0_vault: *accounts.raydium_token_0_vault.key,
            raydium_token_1_vault: *accounts.raydium_token_1_vault.key,
            raydium_create_pool_fee: *accounts.raydium_create_pool_fee.key,
            raydium_observation_state: *accounts.raydium_observation_state.key,
            raydium_permission: *accounts.raydium_permission.key,
            lp_authority: *accounts.lp_authority.key,
            lp_authority_lp_token: *accounts.lp_authority_lp_token.key,
            base_token_program: *accounts.base_token_program.key,
            quote_token_program: *accounts.quote_token_program.key,
            token_program: *accounts.token_program.key,
            associated_token_program: *accounts.associated_token_program.key,
            system_program: *accounts.system_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<MigrateKeys> for [AccountMeta; MIGRATE_IX_ACCOUNTS_LEN] {
    fn from(keys: MigrateKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pool_authority,
                is_signer: false,
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
                pubkey: keys.migration_payer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.raydium_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.raydium_amm_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.raydium_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.raydium_pool_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.raydium_lp_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.payer_lp_token,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.raydium_token_0_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.raydium_token_1_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.raydium_create_pool_fee,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.raydium_observation_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.raydium_permission,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.lp_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.lp_authority_lp_token,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.base_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.quote_token_program,
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
impl From<[Pubkey; MIGRATE_IX_ACCOUNTS_LEN]> for MigrateKeys {
    fn from(pubkeys: [Pubkey; MIGRATE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            pool: pubkeys[0],
            pool_authority: pubkeys[1],
            base_mint: pubkeys[2],
            quote_mint: pubkeys[3],
            base_vault: pubkeys[4],
            quote_vault: pubkeys[5],
            migration_payer: pubkeys[6],
            raydium_program: pubkeys[7],
            raydium_amm_config: pubkeys[8],
            raydium_authority: pubkeys[9],
            raydium_pool_state: pubkeys[10],
            raydium_lp_mint: pubkeys[11],
            payer_lp_token: pubkeys[12],
            raydium_token_0_vault: pubkeys[13],
            raydium_token_1_vault: pubkeys[14],
            raydium_create_pool_fee: pubkeys[15],
            raydium_observation_state: pubkeys[16],
            raydium_permission: pubkeys[17],
            lp_authority: pubkeys[18],
            lp_authority_lp_token: pubkeys[19],
            base_token_program: pubkeys[20],
            quote_token_program: pubkeys[21],
            token_program: pubkeys[22],
            associated_token_program: pubkeys[23],
            system_program: pubkeys[24],
            event_authority: pubkeys[25],
            program: pubkeys[26],
        }
    }
}
impl<'info> From<MigrateAccounts<'_, 'info>>
for [AccountInfo<'info>; MIGRATE_IX_ACCOUNTS_LEN] {
    fn from(accounts: MigrateAccounts<'_, 'info>) -> Self {
        [
            accounts.pool.clone(),
            accounts.pool_authority.clone(),
            accounts.base_mint.clone(),
            accounts.quote_mint.clone(),
            accounts.base_vault.clone(),
            accounts.quote_vault.clone(),
            accounts.migration_payer.clone(),
            accounts.raydium_program.clone(),
            accounts.raydium_amm_config.clone(),
            accounts.raydium_authority.clone(),
            accounts.raydium_pool_state.clone(),
            accounts.raydium_lp_mint.clone(),
            accounts.payer_lp_token.clone(),
            accounts.raydium_token_0_vault.clone(),
            accounts.raydium_token_1_vault.clone(),
            accounts.raydium_create_pool_fee.clone(),
            accounts.raydium_observation_state.clone(),
            accounts.raydium_permission.clone(),
            accounts.lp_authority.clone(),
            accounts.lp_authority_lp_token.clone(),
            accounts.base_token_program.clone(),
            accounts.quote_token_program.clone(),
            accounts.token_program.clone(),
            accounts.associated_token_program.clone(),
            accounts.system_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; MIGRATE_IX_ACCOUNTS_LEN]>
for MigrateAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; MIGRATE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            pool: &arr[0],
            pool_authority: &arr[1],
            base_mint: &arr[2],
            quote_mint: &arr[3],
            base_vault: &arr[4],
            quote_vault: &arr[5],
            migration_payer: &arr[6],
            raydium_program: &arr[7],
            raydium_amm_config: &arr[8],
            raydium_authority: &arr[9],
            raydium_pool_state: &arr[10],
            raydium_lp_mint: &arr[11],
            payer_lp_token: &arr[12],
            raydium_token_0_vault: &arr[13],
            raydium_token_1_vault: &arr[14],
            raydium_create_pool_fee: &arr[15],
            raydium_observation_state: &arr[16],
            raydium_permission: &arr[17],
            lp_authority: &arr[18],
            lp_authority_lp_token: &arr[19],
            base_token_program: &arr[20],
            quote_token_program: &arr[21],
            token_program: &arr[22],
            associated_token_program: &arr[23],
            system_program: &arr[24],
            event_authority: &arr[25],
            program: &arr[26],
        }
    }
}
pub const MIGRATE_IX_DISCM: [u8; 8usize] = [155, 234, 231, 146, 236, 158, 162, 30];
#[derive(Clone, Debug, PartialEq)]
pub struct MigrateIxData;
impl MigrateIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != MIGRATE_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&MIGRATE_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn migrate_ix_with_program_id(
    program_id: Pubkey,
    keys: MigrateKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; MIGRATE_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: MigrateIxData.try_to_vec()?,
    })
}
pub fn migrate_ix(keys: MigrateKeys) -> std::io::Result<Instruction> {
    migrate_ix_with_program_id(TRENDS_PROGRAM_ID, keys)
}
pub fn migrate_invoke_with_program_id(
    program_id: Pubkey,
    accounts: MigrateAccounts<'_, '_>,
) -> ProgramResult {
    let keys: MigrateKeys = accounts.into();
    let ix = migrate_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn migrate_invoke(accounts: MigrateAccounts<'_, '_>) -> ProgramResult {
    migrate_invoke_with_program_id(TRENDS_PROGRAM_ID, accounts)
}
pub fn migrate_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: MigrateAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: MigrateKeys = accounts.into();
    let ix = migrate_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn migrate_invoke_signed(
    accounts: MigrateAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    migrate_invoke_signed_with_program_id(TRENDS_PROGRAM_ID, accounts, seeds)
}
pub fn migrate_verify_account_keys(
    accounts: MigrateAccounts<'_, '_>,
    keys: MigrateKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.pool.key, keys.pool),
        (*accounts.pool_authority.key, keys.pool_authority),
        (*accounts.base_mint.key, keys.base_mint),
        (*accounts.quote_mint.key, keys.quote_mint),
        (*accounts.base_vault.key, keys.base_vault),
        (*accounts.quote_vault.key, keys.quote_vault),
        (*accounts.migration_payer.key, keys.migration_payer),
        (*accounts.raydium_program.key, keys.raydium_program),
        (*accounts.raydium_amm_config.key, keys.raydium_amm_config),
        (*accounts.raydium_authority.key, keys.raydium_authority),
        (*accounts.raydium_pool_state.key, keys.raydium_pool_state),
        (*accounts.raydium_lp_mint.key, keys.raydium_lp_mint),
        (*accounts.payer_lp_token.key, keys.payer_lp_token),
        (*accounts.raydium_token_0_vault.key, keys.raydium_token_0_vault),
        (*accounts.raydium_token_1_vault.key, keys.raydium_token_1_vault),
        (*accounts.raydium_create_pool_fee.key, keys.raydium_create_pool_fee),
        (*accounts.raydium_observation_state.key, keys.raydium_observation_state),
        (*accounts.raydium_permission.key, keys.raydium_permission),
        (*accounts.lp_authority.key, keys.lp_authority),
        (*accounts.lp_authority_lp_token.key, keys.lp_authority_lp_token),
        (*accounts.base_token_program.key, keys.base_token_program),
        (*accounts.quote_token_program.key, keys.quote_token_program),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.associated_token_program.key, keys.associated_token_program),
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
pub fn migrate_verify_writable_privileges<'me, 'info>(
    accounts: MigrateAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.pool,
        accounts.pool_authority,
        accounts.base_vault,
        accounts.quote_vault,
        accounts.migration_payer,
        accounts.raydium_pool_state,
        accounts.raydium_lp_mint,
        accounts.payer_lp_token,
        accounts.raydium_token_0_vault,
        accounts.raydium_token_1_vault,
        accounts.raydium_create_pool_fee,
        accounts.raydium_observation_state,
        accounts.lp_authority_lp_token,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn migrate_verify_signer_privileges<'me, 'info>(
    accounts: MigrateAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.migration_payer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn migrate_verify_account_privileges<'me, 'info>(
    accounts: MigrateAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    migrate_verify_writable_privileges(accounts)?;
    migrate_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SWAP_IX_ACCOUNTS_LEN: usize = 15;
#[derive(Copy, Clone, Debug)]
pub struct SwapAccounts<'me, 'info> {
    pub config: &'me AccountInfo<'info>,
    pub pool: &'me AccountInfo<'info>,
    pub pool_authority: &'me AccountInfo<'info>,
    pub input_token_account: &'me AccountInfo<'info>,
    pub output_token_account: &'me AccountInfo<'info>,
    pub base_mint: &'me AccountInfo<'info>,
    pub quote_mint: &'me AccountInfo<'info>,
    pub base_vault: &'me AccountInfo<'info>,
    pub quote_vault: &'me AccountInfo<'info>,
    pub trader: &'me AccountInfo<'info>,
    pub base_token_program: &'me AccountInfo<'info>,
    pub quote_token_program: &'me AccountInfo<'info>,
    pub referral_token_account: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SwapKeys {
    pub config: Pubkey,
    pub pool: Pubkey,
    pub pool_authority: Pubkey,
    pub input_token_account: Pubkey,
    pub output_token_account: Pubkey,
    pub base_mint: Pubkey,
    pub quote_mint: Pubkey,
    pub base_vault: Pubkey,
    pub quote_vault: Pubkey,
    pub trader: Pubkey,
    pub base_token_program: Pubkey,
    pub quote_token_program: Pubkey,
    pub referral_token_account: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<SwapAccounts<'_, '_>> for SwapKeys {
    fn from(accounts: SwapAccounts) -> Self {
        Self {
            config: *accounts.config.key,
            pool: *accounts.pool.key,
            pool_authority: *accounts.pool_authority.key,
            input_token_account: *accounts.input_token_account.key,
            output_token_account: *accounts.output_token_account.key,
            base_mint: *accounts.base_mint.key,
            quote_mint: *accounts.quote_mint.key,
            base_vault: *accounts.base_vault.key,
            quote_vault: *accounts.quote_vault.key,
            trader: *accounts.trader.key,
            base_token_program: *accounts.base_token_program.key,
            quote_token_program: *accounts.quote_token_program.key,
            referral_token_account: *accounts.referral_token_account.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<SwapKeys> for [AccountMeta; SWAP_IX_ACCOUNTS_LEN] {
    fn from(keys: SwapKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.config,
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
                pubkey: keys.input_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.output_token_account,
                is_signer: false,
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
                pubkey: keys.trader,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.base_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.quote_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.referral_token_account,
                is_signer: false,
                is_writable: true,
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
impl From<[Pubkey; SWAP_IX_ACCOUNTS_LEN]> for SwapKeys {
    fn from(pubkeys: [Pubkey; SWAP_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            config: pubkeys[0],
            pool: pubkeys[1],
            pool_authority: pubkeys[2],
            input_token_account: pubkeys[3],
            output_token_account: pubkeys[4],
            base_mint: pubkeys[5],
            quote_mint: pubkeys[6],
            base_vault: pubkeys[7],
            quote_vault: pubkeys[8],
            trader: pubkeys[9],
            base_token_program: pubkeys[10],
            quote_token_program: pubkeys[11],
            referral_token_account: pubkeys[12],
            event_authority: pubkeys[13],
            program: pubkeys[14],
        }
    }
}
impl<'info> From<SwapAccounts<'_, 'info>>
for [AccountInfo<'info>; SWAP_IX_ACCOUNTS_LEN] {
    fn from(accounts: SwapAccounts<'_, 'info>) -> Self {
        [
            accounts.config.clone(),
            accounts.pool.clone(),
            accounts.pool_authority.clone(),
            accounts.input_token_account.clone(),
            accounts.output_token_account.clone(),
            accounts.base_mint.clone(),
            accounts.quote_mint.clone(),
            accounts.base_vault.clone(),
            accounts.quote_vault.clone(),
            accounts.trader.clone(),
            accounts.base_token_program.clone(),
            accounts.quote_token_program.clone(),
            accounts.referral_token_account.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SWAP_IX_ACCOUNTS_LEN]>
for SwapAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; SWAP_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            config: &arr[0],
            pool: &arr[1],
            pool_authority: &arr[2],
            input_token_account: &arr[3],
            output_token_account: &arr[4],
            base_mint: &arr[5],
            quote_mint: &arr[6],
            base_vault: &arr[7],
            quote_vault: &arr[8],
            trader: &arr[9],
            base_token_program: &arr[10],
            quote_token_program: &arr[11],
            referral_token_account: &arr[12],
            event_authority: &arr[13],
            program: &arr[14],
        }
    }
}
pub const SWAP_IX_DISCM: [u8; 8usize] = [248, 198, 158, 145, 225, 117, 135, 200];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SwapIxArgs {
    pub params: SwapParams,
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
        let params = if reader.is_empty() {
            Default::default()
        } else {
            <SwapParams>::deserialize(&mut reader)?
        };
        Ok(Self(SwapIxArgs { params }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SWAP_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.params, &mut writer)?;
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
    swap_ix_with_program_id(TRENDS_PROGRAM_ID, keys, args)
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
    swap_invoke_with_program_id(TRENDS_PROGRAM_ID, accounts, args)
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
    swap_invoke_signed_with_program_id(TRENDS_PROGRAM_ID, accounts, args, seeds)
}
pub fn swap_verify_account_keys(
    accounts: SwapAccounts<'_, '_>,
    keys: SwapKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.config.key, keys.config),
        (*accounts.pool.key, keys.pool),
        (*accounts.pool_authority.key, keys.pool_authority),
        (*accounts.input_token_account.key, keys.input_token_account),
        (*accounts.output_token_account.key, keys.output_token_account),
        (*accounts.base_mint.key, keys.base_mint),
        (*accounts.quote_mint.key, keys.quote_mint),
        (*accounts.base_vault.key, keys.base_vault),
        (*accounts.quote_vault.key, keys.quote_vault),
        (*accounts.trader.key, keys.trader),
        (*accounts.base_token_program.key, keys.base_token_program),
        (*accounts.quote_token_program.key, keys.quote_token_program),
        (*accounts.referral_token_account.key, keys.referral_token_account),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
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
        accounts.pool,
        accounts.input_token_account,
        accounts.output_token_account,
        accounts.base_vault,
        accounts.quote_vault,
        accounts.referral_token_account,
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
