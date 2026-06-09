use solana_pubkey::Pubkey;
use solana_cpi::{invoke, invoke_signed};
use solana_instruction::{AccountMeta, Instruction};
use solana_account_info::AccountInfo;
use solana_program_error::ProgramError;
use std::io::Read;
#[allow(unused_imports)]
use crate::*;
#[derive(Clone, Debug, PartialEq)]
pub enum AlphaVaultProgramIx {
    ClaimToken,
    CloseCrankFeeWhitelist,
    CloseEscrow,
    CloseFcfsConfig,
    CloseMerkleProofMetadata,
    CloseProrataConfig,
    CreateCrankFeeWhitelist,
    CreateFcfsConfig(CreateFcfsConfigIxArgs),
    CreateMerkleProofMetadata(CreateMerkleProofMetadataIxArgs),
    CreateMerkleRootConfig(CreateMerkleRootConfigIxArgs),
    CreateNewEscrow,
    CreatePermissionedEscrow(CreatePermissionedEscrowIxArgs),
    CreatePermissionedEscrowWithAuthority(CreatePermissionedEscrowWithAuthorityIxArgs),
    CreateProrataConfig(CreateProrataConfigIxArgs),
    Deposit(DepositIxArgs),
    FillDammV2(FillDammV2IxArgs),
    FillDlmm(FillDlmmIxArgs),
    FillDynamicAmm(FillDynamicAmmIxArgs),
    InitializeFcfsVault(InitializeFcfsVaultIxArgs),
    InitializeProrataVault(InitializeProrataVaultIxArgs),
    InitializeVaultWithFcfsConfig(InitializeVaultWithFcfsConfigIxArgs),
    InitializeVaultWithProrataConfig(InitializeVaultWithProrataConfigIxArgs),
    TransferVaultAuthority(TransferVaultAuthorityIxArgs),
    UpdateFcfsVaultParameters(UpdateFcfsVaultParametersIxArgs),
    UpdateProrataVaultParameters(UpdateProrataVaultParametersIxArgs),
    Withdraw(WithdrawIxArgs),
    WithdrawRemainingQuote,
}
impl AlphaVaultProgramIx {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        if buf.starts_with(&CLAIM_TOKEN_IX_DISCM) {
            return Ok(Self::ClaimToken);
        }
        if buf.starts_with(&CLOSE_CRANK_FEE_WHITELIST_IX_DISCM) {
            return Ok(Self::CloseCrankFeeWhitelist);
        }
        if buf.starts_with(&CLOSE_ESCROW_IX_DISCM) {
            return Ok(Self::CloseEscrow);
        }
        if buf.starts_with(&CLOSE_FCFS_CONFIG_IX_DISCM) {
            return Ok(Self::CloseFcfsConfig);
        }
        if buf.starts_with(&CLOSE_MERKLE_PROOF_METADATA_IX_DISCM) {
            return Ok(Self::CloseMerkleProofMetadata);
        }
        if buf.starts_with(&CLOSE_PRORATA_CONFIG_IX_DISCM) {
            return Ok(Self::CloseProrataConfig);
        }
        if buf.starts_with(&CREATE_CRANK_FEE_WHITELIST_IX_DISCM) {
            return Ok(Self::CreateCrankFeeWhitelist);
        }
        if buf.starts_with(&CREATE_FCFS_CONFIG_IX_DISCM) {
            let mut reader = &buf[CREATE_FCFS_CONFIG_IX_DISCM.len()..];
            let config_parameters = if reader.is_empty() {
                Default::default()
            } else {
                <FcfsConfigParameters>::deserialize(&mut reader)?
            };
            return Ok(
                Self::CreateFcfsConfig(CreateFcfsConfigIxArgs {
                    config_parameters,
                }),
            );
        }
        if buf.starts_with(&CREATE_MERKLE_PROOF_METADATA_IX_DISCM) {
            let mut reader = &buf[CREATE_MERKLE_PROOF_METADATA_IX_DISCM.len()..];
            let proof_url: String = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::CreateMerkleProofMetadata(CreateMerkleProofMetadataIxArgs {
                    proof_url,
                }),
            );
        }
        if buf.starts_with(&CREATE_MERKLE_ROOT_CONFIG_IX_DISCM) {
            let mut reader = &buf[CREATE_MERKLE_ROOT_CONFIG_IX_DISCM.len()..];
            let params = if reader.is_empty() {
                Default::default()
            } else {
                <CreateMerkleRootConfigParams>::deserialize(&mut reader)?
            };
            return Ok(
                Self::CreateMerkleRootConfig(CreateMerkleRootConfigIxArgs {
                    params,
                }),
            );
        }
        if buf.starts_with(&CREATE_NEW_ESCROW_IX_DISCM) {
            return Ok(Self::CreateNewEscrow);
        }
        if buf.starts_with(&CREATE_PERMISSIONED_ESCROW_IX_DISCM) {
            let mut reader = &buf[CREATE_PERMISSIONED_ESCROW_IX_DISCM.len()..];
            let max_cap: u64 = crate::borsh_de_or_default(&mut reader)?;
            let proof: Vec<[u8; 32]> = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::CreatePermissionedEscrow(CreatePermissionedEscrowIxArgs {
                    max_cap,
                    proof,
                }),
            );
        }
        if buf.starts_with(&CREATE_PERMISSIONED_ESCROW_WITH_AUTHORITY_IX_DISCM) {
            let mut reader = &buf[CREATE_PERMISSIONED_ESCROW_WITH_AUTHORITY_IX_DISCM
                .len()..];
            let max_cap: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::CreatePermissionedEscrowWithAuthority(CreatePermissionedEscrowWithAuthorityIxArgs {
                    max_cap,
                }),
            );
        }
        if buf.starts_with(&CREATE_PRORATA_CONFIG_IX_DISCM) {
            let mut reader = &buf[CREATE_PRORATA_CONFIG_IX_DISCM.len()..];
            let config_parameters = if reader.is_empty() {
                Default::default()
            } else {
                <ProrataConfigParameters>::deserialize(&mut reader)?
            };
            return Ok(
                Self::CreateProrataConfig(CreateProrataConfigIxArgs {
                    config_parameters,
                }),
            );
        }
        if buf.starts_with(&DEPOSIT_IX_DISCM) {
            let mut reader = &buf[DEPOSIT_IX_DISCM.len()..];
            let max_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(Self::Deposit(DepositIxArgs { max_amount }));
        }
        if buf.starts_with(&FILL_DAMM_V2_IX_DISCM) {
            let mut reader = &buf[FILL_DAMM_V2_IX_DISCM.len()..];
            let max_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(Self::FillDammV2(FillDammV2IxArgs { max_amount }));
        }
        if buf.starts_with(&FILL_DLMM_IX_DISCM) {
            let mut reader = &buf[FILL_DLMM_IX_DISCM.len()..];
            let max_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            let remaining_accounts_info = if reader.is_empty() {
                Default::default()
            } else {
                <RemainingAccountsInfo>::deserialize(&mut reader)?
            };
            return Ok(
                Self::FillDlmm(FillDlmmIxArgs {
                    max_amount,
                    remaining_accounts_info,
                }),
            );
        }
        if buf.starts_with(&FILL_DYNAMIC_AMM_IX_DISCM) {
            let mut reader = &buf[FILL_DYNAMIC_AMM_IX_DISCM.len()..];
            let max_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(Self::FillDynamicAmm(FillDynamicAmmIxArgs { max_amount }));
        }
        if buf.starts_with(&INITIALIZE_FCFS_VAULT_IX_DISCM) {
            let mut reader = &buf[INITIALIZE_FCFS_VAULT_IX_DISCM.len()..];
            let params = if reader.is_empty() {
                Default::default()
            } else {
                <InitializeFcfsVaultParams>::deserialize(&mut reader)?
            };
            return Ok(
                Self::InitializeFcfsVault(InitializeFcfsVaultIxArgs {
                    params,
                }),
            );
        }
        if buf.starts_with(&INITIALIZE_PRORATA_VAULT_IX_DISCM) {
            let mut reader = &buf[INITIALIZE_PRORATA_VAULT_IX_DISCM.len()..];
            let params = if reader.is_empty() {
                Default::default()
            } else {
                <InitializeProrataVaultParams>::deserialize(&mut reader)?
            };
            return Ok(
                Self::InitializeProrataVault(InitializeProrataVaultIxArgs {
                    params,
                }),
            );
        }
        if buf.starts_with(&INITIALIZE_VAULT_WITH_FCFS_CONFIG_IX_DISCM) {
            let mut reader = &buf[INITIALIZE_VAULT_WITH_FCFS_CONFIG_IX_DISCM.len()..];
            let params = if reader.is_empty() {
                Default::default()
            } else {
                <InitializeVaultWithConfigParams>::deserialize(&mut reader)?
            };
            return Ok(
                Self::InitializeVaultWithFcfsConfig(InitializeVaultWithFcfsConfigIxArgs {
                    params,
                }),
            );
        }
        if buf.starts_with(&INITIALIZE_VAULT_WITH_PRORATA_CONFIG_IX_DISCM) {
            let mut reader = &buf[INITIALIZE_VAULT_WITH_PRORATA_CONFIG_IX_DISCM.len()..];
            let params = if reader.is_empty() {
                Default::default()
            } else {
                <InitializeVaultWithConfigParams>::deserialize(&mut reader)?
            };
            return Ok(
                Self::InitializeVaultWithProrataConfig(InitializeVaultWithProrataConfigIxArgs {
                    params,
                }),
            );
        }
        if buf.starts_with(&TRANSFER_VAULT_AUTHORITY_IX_DISCM) {
            let mut reader = &buf[TRANSFER_VAULT_AUTHORITY_IX_DISCM.len()..];
            let new_authority: Pubkey = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::TransferVaultAuthority(TransferVaultAuthorityIxArgs {
                    new_authority,
                }),
            );
        }
        if buf.starts_with(&UPDATE_FCFS_VAULT_PARAMETERS_IX_DISCM) {
            let mut reader = &buf[UPDATE_FCFS_VAULT_PARAMETERS_IX_DISCM.len()..];
            let params = if reader.is_empty() {
                Default::default()
            } else {
                <UpdateFcfsVaultParams>::deserialize(&mut reader)?
            };
            return Ok(
                Self::UpdateFcfsVaultParameters(UpdateFcfsVaultParametersIxArgs {
                    params,
                }),
            );
        }
        if buf.starts_with(&UPDATE_PRORATA_VAULT_PARAMETERS_IX_DISCM) {
            let mut reader = &buf[UPDATE_PRORATA_VAULT_PARAMETERS_IX_DISCM.len()..];
            let params = if reader.is_empty() {
                Default::default()
            } else {
                <UpdateProrataVaultParams>::deserialize(&mut reader)?
            };
            return Ok(
                Self::UpdateProrataVaultParameters(UpdateProrataVaultParametersIxArgs {
                    params,
                }),
            );
        }
        if buf.starts_with(&WITHDRAW_IX_DISCM) {
            let mut reader = &buf[WITHDRAW_IX_DISCM.len()..];
            let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(Self::Withdraw(WithdrawIxArgs { amount }));
        }
        if buf.starts_with(&WITHDRAW_REMAINING_QUOTE_IX_DISCM) {
            return Ok(Self::WithdrawRemainingQuote);
        }
        Err(std::io::Error::from(std::io::ErrorKind::InvalidData))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        match self {
            Self::ClaimToken => writer.write_all(&CLAIM_TOKEN_IX_DISCM),
            Self::CloseCrankFeeWhitelist => {
                writer.write_all(&CLOSE_CRANK_FEE_WHITELIST_IX_DISCM)
            }
            Self::CloseEscrow => writer.write_all(&CLOSE_ESCROW_IX_DISCM),
            Self::CloseFcfsConfig => writer.write_all(&CLOSE_FCFS_CONFIG_IX_DISCM),
            Self::CloseMerkleProofMetadata => {
                writer.write_all(&CLOSE_MERKLE_PROOF_METADATA_IX_DISCM)
            }
            Self::CloseProrataConfig => writer.write_all(&CLOSE_PRORATA_CONFIG_IX_DISCM),
            Self::CreateCrankFeeWhitelist => {
                writer.write_all(&CREATE_CRANK_FEE_WHITELIST_IX_DISCM)
            }
            Self::CreateFcfsConfig(args) => {
                writer.write_all(&CREATE_FCFS_CONFIG_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.config_parameters, &mut writer)?;
                Ok(())
            }
            Self::CreateMerkleProofMetadata(args) => {
                writer.write_all(&CREATE_MERKLE_PROOF_METADATA_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.proof_url, &mut writer)?;
                Ok(())
            }
            Self::CreateMerkleRootConfig(args) => {
                writer.write_all(&CREATE_MERKLE_ROOT_CONFIG_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.params, &mut writer)?;
                Ok(())
            }
            Self::CreateNewEscrow => writer.write_all(&CREATE_NEW_ESCROW_IX_DISCM),
            Self::CreatePermissionedEscrow(args) => {
                writer.write_all(&CREATE_PERMISSIONED_ESCROW_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.max_cap, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.proof, &mut writer)?;
                Ok(())
            }
            Self::CreatePermissionedEscrowWithAuthority(args) => {
                writer.write_all(&CREATE_PERMISSIONED_ESCROW_WITH_AUTHORITY_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.max_cap, &mut writer)?;
                Ok(())
            }
            Self::CreateProrataConfig(args) => {
                writer.write_all(&CREATE_PRORATA_CONFIG_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.config_parameters, &mut writer)?;
                Ok(())
            }
            Self::Deposit(args) => {
                writer.write_all(&DEPOSIT_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.max_amount, &mut writer)?;
                Ok(())
            }
            Self::FillDammV2(args) => {
                writer.write_all(&FILL_DAMM_V2_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.max_amount, &mut writer)?;
                Ok(())
            }
            Self::FillDlmm(args) => {
                writer.write_all(&FILL_DLMM_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.max_amount, &mut writer)?;
                borsh::BorshSerialize::serialize(
                    &args.remaining_accounts_info,
                    &mut writer,
                )?;
                Ok(())
            }
            Self::FillDynamicAmm(args) => {
                writer.write_all(&FILL_DYNAMIC_AMM_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.max_amount, &mut writer)?;
                Ok(())
            }
            Self::InitializeFcfsVault(args) => {
                writer.write_all(&INITIALIZE_FCFS_VAULT_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.params, &mut writer)?;
                Ok(())
            }
            Self::InitializeProrataVault(args) => {
                writer.write_all(&INITIALIZE_PRORATA_VAULT_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.params, &mut writer)?;
                Ok(())
            }
            Self::InitializeVaultWithFcfsConfig(args) => {
                writer.write_all(&INITIALIZE_VAULT_WITH_FCFS_CONFIG_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.params, &mut writer)?;
                Ok(())
            }
            Self::InitializeVaultWithProrataConfig(args) => {
                writer.write_all(&INITIALIZE_VAULT_WITH_PRORATA_CONFIG_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.params, &mut writer)?;
                Ok(())
            }
            Self::TransferVaultAuthority(args) => {
                writer.write_all(&TRANSFER_VAULT_AUTHORITY_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.new_authority, &mut writer)?;
                Ok(())
            }
            Self::UpdateFcfsVaultParameters(args) => {
                writer.write_all(&UPDATE_FCFS_VAULT_PARAMETERS_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.params, &mut writer)?;
                Ok(())
            }
            Self::UpdateProrataVaultParameters(args) => {
                writer.write_all(&UPDATE_PRORATA_VAULT_PARAMETERS_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.params, &mut writer)?;
                Ok(())
            }
            Self::Withdraw(args) => {
                writer.write_all(&WITHDRAW_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.amount, &mut writer)?;
                Ok(())
            }
            Self::WithdrawRemainingQuote => {
                writer.write_all(&WITHDRAW_REMAINING_QUOTE_IX_DISCM)
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
pub const CLAIM_TOKEN_IX_ACCOUNTS_LEN: usize = 9;
#[derive(Copy, Clone, Debug)]
pub struct ClaimTokenAccounts<'me, 'info> {
    pub vault: &'me AccountInfo<'info>,
    pub escrow: &'me AccountInfo<'info>,
    pub token_out_vault: &'me AccountInfo<'info>,
    pub destination_token: &'me AccountInfo<'info>,
    pub token_mint: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub owner: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ClaimTokenKeys {
    pub vault: Pubkey,
    pub escrow: Pubkey,
    pub token_out_vault: Pubkey,
    pub destination_token: Pubkey,
    pub token_mint: Pubkey,
    pub token_program: Pubkey,
    pub owner: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<ClaimTokenAccounts<'_, '_>> for ClaimTokenKeys {
    fn from(accounts: ClaimTokenAccounts) -> Self {
        Self {
            vault: *accounts.vault.key,
            escrow: *accounts.escrow.key,
            token_out_vault: *accounts.token_out_vault.key,
            destination_token: *accounts.destination_token.key,
            token_mint: *accounts.token_mint.key,
            token_program: *accounts.token_program.key,
            owner: *accounts.owner.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<ClaimTokenKeys> for [AccountMeta; CLAIM_TOKEN_IX_ACCOUNTS_LEN] {
    fn from(keys: ClaimTokenKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.escrow,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_out_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.destination_token,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.owner,
                is_signer: true,
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
impl From<[Pubkey; CLAIM_TOKEN_IX_ACCOUNTS_LEN]> for ClaimTokenKeys {
    fn from(pubkeys: [Pubkey; CLAIM_TOKEN_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            vault: pubkeys[0],
            escrow: pubkeys[1],
            token_out_vault: pubkeys[2],
            destination_token: pubkeys[3],
            token_mint: pubkeys[4],
            token_program: pubkeys[5],
            owner: pubkeys[6],
            event_authority: pubkeys[7],
            program: pubkeys[8],
        }
    }
}
impl<'info> From<ClaimTokenAccounts<'_, 'info>>
for [AccountInfo<'info>; CLAIM_TOKEN_IX_ACCOUNTS_LEN] {
    fn from(accounts: ClaimTokenAccounts<'_, 'info>) -> Self {
        [
            accounts.vault.clone(),
            accounts.escrow.clone(),
            accounts.token_out_vault.clone(),
            accounts.destination_token.clone(),
            accounts.token_mint.clone(),
            accounts.token_program.clone(),
            accounts.owner.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CLAIM_TOKEN_IX_ACCOUNTS_LEN]>
for ClaimTokenAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; CLAIM_TOKEN_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            vault: &arr[0],
            escrow: &arr[1],
            token_out_vault: &arr[2],
            destination_token: &arr[3],
            token_mint: &arr[4],
            token_program: &arr[5],
            owner: &arr[6],
            event_authority: &arr[7],
            program: &arr[8],
        }
    }
}
pub const CLAIM_TOKEN_IX_DISCM: [u8; 8usize] = [116, 206, 27, 191, 166, 19, 0, 73];
#[derive(Clone, Debug, PartialEq)]
pub struct ClaimTokenIxData;
impl ClaimTokenIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CLAIM_TOKEN_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CLAIM_TOKEN_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn claim_token_ix_with_program_id(
    program_id: Pubkey,
    keys: ClaimTokenKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CLAIM_TOKEN_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: ClaimTokenIxData.try_to_vec()?,
    })
}
pub fn claim_token_ix(keys: ClaimTokenKeys) -> std::io::Result<Instruction> {
    claim_token_ix_with_program_id(ALPHA_VAULT_PROGRAM_ID, keys)
}
pub fn claim_token_invoke_with_program_id(
    program_id: Pubkey,
    accounts: ClaimTokenAccounts<'_, '_>,
) -> ProgramResult {
    let keys: ClaimTokenKeys = accounts.into();
    let ix = claim_token_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn claim_token_invoke(accounts: ClaimTokenAccounts<'_, '_>) -> ProgramResult {
    claim_token_invoke_with_program_id(ALPHA_VAULT_PROGRAM_ID, accounts)
}
pub fn claim_token_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: ClaimTokenAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: ClaimTokenKeys = accounts.into();
    let ix = claim_token_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn claim_token_invoke_signed(
    accounts: ClaimTokenAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    claim_token_invoke_signed_with_program_id(ALPHA_VAULT_PROGRAM_ID, accounts, seeds)
}
pub fn claim_token_verify_account_keys(
    accounts: ClaimTokenAccounts<'_, '_>,
    keys: ClaimTokenKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.vault.key, keys.vault),
        (*accounts.escrow.key, keys.escrow),
        (*accounts.token_out_vault.key, keys.token_out_vault),
        (*accounts.destination_token.key, keys.destination_token),
        (*accounts.token_mint.key, keys.token_mint),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.owner.key, keys.owner),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn claim_token_verify_writable_privileges<'me, 'info>(
    accounts: ClaimTokenAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.vault,
        accounts.escrow,
        accounts.token_out_vault,
        accounts.destination_token,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn claim_token_verify_signer_privileges<'me, 'info>(
    accounts: ClaimTokenAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.owner] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn claim_token_verify_account_privileges<'me, 'info>(
    accounts: ClaimTokenAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    claim_token_verify_writable_privileges(accounts)?;
    claim_token_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CLOSE_CRANK_FEE_WHITELIST_IX_ACCOUNTS_LEN: usize = 5;
#[derive(Copy, Clone, Debug)]
pub struct CloseCrankFeeWhitelistAccounts<'me, 'info> {
    pub crank_fee_whitelist: &'me AccountInfo<'info>,
    pub admin: &'me AccountInfo<'info>,
    pub rent_receiver: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CloseCrankFeeWhitelistKeys {
    pub crank_fee_whitelist: Pubkey,
    pub admin: Pubkey,
    pub rent_receiver: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<CloseCrankFeeWhitelistAccounts<'_, '_>> for CloseCrankFeeWhitelistKeys {
    fn from(accounts: CloseCrankFeeWhitelistAccounts) -> Self {
        Self {
            crank_fee_whitelist: *accounts.crank_fee_whitelist.key,
            admin: *accounts.admin.key,
            rent_receiver: *accounts.rent_receiver.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<CloseCrankFeeWhitelistKeys>
for [AccountMeta; CLOSE_CRANK_FEE_WHITELIST_IX_ACCOUNTS_LEN] {
    fn from(keys: CloseCrankFeeWhitelistKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.crank_fee_whitelist,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.admin,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.rent_receiver,
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
impl From<[Pubkey; CLOSE_CRANK_FEE_WHITELIST_IX_ACCOUNTS_LEN]>
for CloseCrankFeeWhitelistKeys {
    fn from(pubkeys: [Pubkey; CLOSE_CRANK_FEE_WHITELIST_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            crank_fee_whitelist: pubkeys[0],
            admin: pubkeys[1],
            rent_receiver: pubkeys[2],
            event_authority: pubkeys[3],
            program: pubkeys[4],
        }
    }
}
impl<'info> From<CloseCrankFeeWhitelistAccounts<'_, 'info>>
for [AccountInfo<'info>; CLOSE_CRANK_FEE_WHITELIST_IX_ACCOUNTS_LEN] {
    fn from(accounts: CloseCrankFeeWhitelistAccounts<'_, 'info>) -> Self {
        [
            accounts.crank_fee_whitelist.clone(),
            accounts.admin.clone(),
            accounts.rent_receiver.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; CLOSE_CRANK_FEE_WHITELIST_IX_ACCOUNTS_LEN]>
for CloseCrankFeeWhitelistAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; CLOSE_CRANK_FEE_WHITELIST_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            crank_fee_whitelist: &arr[0],
            admin: &arr[1],
            rent_receiver: &arr[2],
            event_authority: &arr[3],
            program: &arr[4],
        }
    }
}
pub const CLOSE_CRANK_FEE_WHITELIST_IX_DISCM: [u8; 8usize] = [
    189, 166, 73, 241, 81, 12, 246, 170,
];
#[derive(Clone, Debug, PartialEq)]
pub struct CloseCrankFeeWhitelistIxData;
impl CloseCrankFeeWhitelistIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CLOSE_CRANK_FEE_WHITELIST_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CLOSE_CRANK_FEE_WHITELIST_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn close_crank_fee_whitelist_ix_with_program_id(
    program_id: Pubkey,
    keys: CloseCrankFeeWhitelistKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CLOSE_CRANK_FEE_WHITELIST_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: CloseCrankFeeWhitelistIxData.try_to_vec()?,
    })
}
pub fn close_crank_fee_whitelist_ix(
    keys: CloseCrankFeeWhitelistKeys,
) -> std::io::Result<Instruction> {
    close_crank_fee_whitelist_ix_with_program_id(ALPHA_VAULT_PROGRAM_ID, keys)
}
pub fn close_crank_fee_whitelist_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CloseCrankFeeWhitelistAccounts<'_, '_>,
) -> ProgramResult {
    let keys: CloseCrankFeeWhitelistKeys = accounts.into();
    let ix = close_crank_fee_whitelist_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn close_crank_fee_whitelist_invoke(
    accounts: CloseCrankFeeWhitelistAccounts<'_, '_>,
) -> ProgramResult {
    close_crank_fee_whitelist_invoke_with_program_id(ALPHA_VAULT_PROGRAM_ID, accounts)
}
pub fn close_crank_fee_whitelist_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CloseCrankFeeWhitelistAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CloseCrankFeeWhitelistKeys = accounts.into();
    let ix = close_crank_fee_whitelist_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn close_crank_fee_whitelist_invoke_signed(
    accounts: CloseCrankFeeWhitelistAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    close_crank_fee_whitelist_invoke_signed_with_program_id(
        ALPHA_VAULT_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn close_crank_fee_whitelist_verify_account_keys(
    accounts: CloseCrankFeeWhitelistAccounts<'_, '_>,
    keys: CloseCrankFeeWhitelistKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.crank_fee_whitelist.key, keys.crank_fee_whitelist),
        (*accounts.admin.key, keys.admin),
        (*accounts.rent_receiver.key, keys.rent_receiver),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn close_crank_fee_whitelist_verify_writable_privileges<'me, 'info>(
    accounts: CloseCrankFeeWhitelistAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.crank_fee_whitelist,
        accounts.admin,
        accounts.rent_receiver,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn close_crank_fee_whitelist_verify_signer_privileges<'me, 'info>(
    accounts: CloseCrankFeeWhitelistAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn close_crank_fee_whitelist_verify_account_privileges<'me, 'info>(
    accounts: CloseCrankFeeWhitelistAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    close_crank_fee_whitelist_verify_writable_privileges(accounts)?;
    close_crank_fee_whitelist_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CLOSE_ESCROW_IX_ACCOUNTS_LEN: usize = 6;
#[derive(Copy, Clone, Debug)]
pub struct CloseEscrowAccounts<'me, 'info> {
    pub vault: &'me AccountInfo<'info>,
    pub escrow: &'me AccountInfo<'info>,
    pub owner: &'me AccountInfo<'info>,
    pub rent_receiver: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CloseEscrowKeys {
    pub vault: Pubkey,
    pub escrow: Pubkey,
    pub owner: Pubkey,
    pub rent_receiver: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<CloseEscrowAccounts<'_, '_>> for CloseEscrowKeys {
    fn from(accounts: CloseEscrowAccounts) -> Self {
        Self {
            vault: *accounts.vault.key,
            escrow: *accounts.escrow.key,
            owner: *accounts.owner.key,
            rent_receiver: *accounts.rent_receiver.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<CloseEscrowKeys> for [AccountMeta; CLOSE_ESCROW_IX_ACCOUNTS_LEN] {
    fn from(keys: CloseEscrowKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.escrow,
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
impl From<[Pubkey; CLOSE_ESCROW_IX_ACCOUNTS_LEN]> for CloseEscrowKeys {
    fn from(pubkeys: [Pubkey; CLOSE_ESCROW_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            vault: pubkeys[0],
            escrow: pubkeys[1],
            owner: pubkeys[2],
            rent_receiver: pubkeys[3],
            event_authority: pubkeys[4],
            program: pubkeys[5],
        }
    }
}
impl<'info> From<CloseEscrowAccounts<'_, 'info>>
for [AccountInfo<'info>; CLOSE_ESCROW_IX_ACCOUNTS_LEN] {
    fn from(accounts: CloseEscrowAccounts<'_, 'info>) -> Self {
        [
            accounts.vault.clone(),
            accounts.escrow.clone(),
            accounts.owner.clone(),
            accounts.rent_receiver.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CLOSE_ESCROW_IX_ACCOUNTS_LEN]>
for CloseEscrowAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; CLOSE_ESCROW_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            vault: &arr[0],
            escrow: &arr[1],
            owner: &arr[2],
            rent_receiver: &arr[3],
            event_authority: &arr[4],
            program: &arr[5],
        }
    }
}
pub const CLOSE_ESCROW_IX_DISCM: [u8; 8usize] = [139, 171, 94, 146, 191, 91, 144, 50];
#[derive(Clone, Debug, PartialEq)]
pub struct CloseEscrowIxData;
impl CloseEscrowIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CLOSE_ESCROW_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CLOSE_ESCROW_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn close_escrow_ix_with_program_id(
    program_id: Pubkey,
    keys: CloseEscrowKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CLOSE_ESCROW_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: CloseEscrowIxData.try_to_vec()?,
    })
}
pub fn close_escrow_ix(keys: CloseEscrowKeys) -> std::io::Result<Instruction> {
    close_escrow_ix_with_program_id(ALPHA_VAULT_PROGRAM_ID, keys)
}
pub fn close_escrow_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CloseEscrowAccounts<'_, '_>,
) -> ProgramResult {
    let keys: CloseEscrowKeys = accounts.into();
    let ix = close_escrow_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn close_escrow_invoke(accounts: CloseEscrowAccounts<'_, '_>) -> ProgramResult {
    close_escrow_invoke_with_program_id(ALPHA_VAULT_PROGRAM_ID, accounts)
}
pub fn close_escrow_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CloseEscrowAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CloseEscrowKeys = accounts.into();
    let ix = close_escrow_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn close_escrow_invoke_signed(
    accounts: CloseEscrowAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    close_escrow_invoke_signed_with_program_id(ALPHA_VAULT_PROGRAM_ID, accounts, seeds)
}
pub fn close_escrow_verify_account_keys(
    accounts: CloseEscrowAccounts<'_, '_>,
    keys: CloseEscrowKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.vault.key, keys.vault),
        (*accounts.escrow.key, keys.escrow),
        (*accounts.owner.key, keys.owner),
        (*accounts.rent_receiver.key, keys.rent_receiver),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn close_escrow_verify_writable_privileges<'me, 'info>(
    accounts: CloseEscrowAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.vault, accounts.escrow, accounts.rent_receiver] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn close_escrow_verify_signer_privileges<'me, 'info>(
    accounts: CloseEscrowAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.owner] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn close_escrow_verify_account_privileges<'me, 'info>(
    accounts: CloseEscrowAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    close_escrow_verify_writable_privileges(accounts)?;
    close_escrow_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CLOSE_FCFS_CONFIG_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct CloseFcfsConfigAccounts<'me, 'info> {
    pub config: &'me AccountInfo<'info>,
    pub admin: &'me AccountInfo<'info>,
    pub rent_receiver: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CloseFcfsConfigKeys {
    pub config: Pubkey,
    pub admin: Pubkey,
    pub rent_receiver: Pubkey,
}
impl From<CloseFcfsConfigAccounts<'_, '_>> for CloseFcfsConfigKeys {
    fn from(accounts: CloseFcfsConfigAccounts) -> Self {
        Self {
            config: *accounts.config.key,
            admin: *accounts.admin.key,
            rent_receiver: *accounts.rent_receiver.key,
        }
    }
}
impl From<CloseFcfsConfigKeys> for [AccountMeta; CLOSE_FCFS_CONFIG_IX_ACCOUNTS_LEN] {
    fn from(keys: CloseFcfsConfigKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.config,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.admin,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.rent_receiver,
                is_signer: false,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; CLOSE_FCFS_CONFIG_IX_ACCOUNTS_LEN]> for CloseFcfsConfigKeys {
    fn from(pubkeys: [Pubkey; CLOSE_FCFS_CONFIG_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            config: pubkeys[0],
            admin: pubkeys[1],
            rent_receiver: pubkeys[2],
        }
    }
}
impl<'info> From<CloseFcfsConfigAccounts<'_, 'info>>
for [AccountInfo<'info>; CLOSE_FCFS_CONFIG_IX_ACCOUNTS_LEN] {
    fn from(accounts: CloseFcfsConfigAccounts<'_, 'info>) -> Self {
        [accounts.config.clone(), accounts.admin.clone(), accounts.rent_receiver.clone()]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CLOSE_FCFS_CONFIG_IX_ACCOUNTS_LEN]>
for CloseFcfsConfigAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; CLOSE_FCFS_CONFIG_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            config: &arr[0],
            admin: &arr[1],
            rent_receiver: &arr[2],
        }
    }
}
pub const CLOSE_FCFS_CONFIG_IX_DISCM: [u8; 8usize] = [
    48, 178, 212, 101, 23, 138, 233, 90,
];
#[derive(Clone, Debug, PartialEq)]
pub struct CloseFcfsConfigIxData;
impl CloseFcfsConfigIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CLOSE_FCFS_CONFIG_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CLOSE_FCFS_CONFIG_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn close_fcfs_config_ix_with_program_id(
    program_id: Pubkey,
    keys: CloseFcfsConfigKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CLOSE_FCFS_CONFIG_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: CloseFcfsConfigIxData.try_to_vec()?,
    })
}
pub fn close_fcfs_config_ix(keys: CloseFcfsConfigKeys) -> std::io::Result<Instruction> {
    close_fcfs_config_ix_with_program_id(ALPHA_VAULT_PROGRAM_ID, keys)
}
pub fn close_fcfs_config_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CloseFcfsConfigAccounts<'_, '_>,
) -> ProgramResult {
    let keys: CloseFcfsConfigKeys = accounts.into();
    let ix = close_fcfs_config_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn close_fcfs_config_invoke(
    accounts: CloseFcfsConfigAccounts<'_, '_>,
) -> ProgramResult {
    close_fcfs_config_invoke_with_program_id(ALPHA_VAULT_PROGRAM_ID, accounts)
}
pub fn close_fcfs_config_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CloseFcfsConfigAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CloseFcfsConfigKeys = accounts.into();
    let ix = close_fcfs_config_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn close_fcfs_config_invoke_signed(
    accounts: CloseFcfsConfigAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    close_fcfs_config_invoke_signed_with_program_id(
        ALPHA_VAULT_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn close_fcfs_config_verify_account_keys(
    accounts: CloseFcfsConfigAccounts<'_, '_>,
    keys: CloseFcfsConfigKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.config.key, keys.config),
        (*accounts.admin.key, keys.admin),
        (*accounts.rent_receiver.key, keys.rent_receiver),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn close_fcfs_config_verify_writable_privileges<'me, 'info>(
    accounts: CloseFcfsConfigAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.config, accounts.admin, accounts.rent_receiver] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn close_fcfs_config_verify_signer_privileges<'me, 'info>(
    accounts: CloseFcfsConfigAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn close_fcfs_config_verify_account_privileges<'me, 'info>(
    accounts: CloseFcfsConfigAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    close_fcfs_config_verify_writable_privileges(accounts)?;
    close_fcfs_config_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CLOSE_MERKLE_PROOF_METADATA_IX_ACCOUNTS_LEN: usize = 6;
#[derive(Copy, Clone, Debug)]
pub struct CloseMerkleProofMetadataAccounts<'me, 'info> {
    pub vault: &'me AccountInfo<'info>,
    pub merkle_proof_metadata: &'me AccountInfo<'info>,
    pub admin: &'me AccountInfo<'info>,
    pub rent_receiver: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CloseMerkleProofMetadataKeys {
    pub vault: Pubkey,
    pub merkle_proof_metadata: Pubkey,
    pub admin: Pubkey,
    pub rent_receiver: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<CloseMerkleProofMetadataAccounts<'_, '_>> for CloseMerkleProofMetadataKeys {
    fn from(accounts: CloseMerkleProofMetadataAccounts) -> Self {
        Self {
            vault: *accounts.vault.key,
            merkle_proof_metadata: *accounts.merkle_proof_metadata.key,
            admin: *accounts.admin.key,
            rent_receiver: *accounts.rent_receiver.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<CloseMerkleProofMetadataKeys>
for [AccountMeta; CLOSE_MERKLE_PROOF_METADATA_IX_ACCOUNTS_LEN] {
    fn from(keys: CloseMerkleProofMetadataKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.vault,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.merkle_proof_metadata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.admin,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.rent_receiver,
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
impl From<[Pubkey; CLOSE_MERKLE_PROOF_METADATA_IX_ACCOUNTS_LEN]>
for CloseMerkleProofMetadataKeys {
    fn from(pubkeys: [Pubkey; CLOSE_MERKLE_PROOF_METADATA_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            vault: pubkeys[0],
            merkle_proof_metadata: pubkeys[1],
            admin: pubkeys[2],
            rent_receiver: pubkeys[3],
            event_authority: pubkeys[4],
            program: pubkeys[5],
        }
    }
}
impl<'info> From<CloseMerkleProofMetadataAccounts<'_, 'info>>
for [AccountInfo<'info>; CLOSE_MERKLE_PROOF_METADATA_IX_ACCOUNTS_LEN] {
    fn from(accounts: CloseMerkleProofMetadataAccounts<'_, 'info>) -> Self {
        [
            accounts.vault.clone(),
            accounts.merkle_proof_metadata.clone(),
            accounts.admin.clone(),
            accounts.rent_receiver.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; CLOSE_MERKLE_PROOF_METADATA_IX_ACCOUNTS_LEN]>
for CloseMerkleProofMetadataAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; CLOSE_MERKLE_PROOF_METADATA_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            vault: &arr[0],
            merkle_proof_metadata: &arr[1],
            admin: &arr[2],
            rent_receiver: &arr[3],
            event_authority: &arr[4],
            program: &arr[5],
        }
    }
}
pub const CLOSE_MERKLE_PROOF_METADATA_IX_DISCM: [u8; 8usize] = [
    23, 52, 170, 30, 252, 47, 100, 129,
];
#[derive(Clone, Debug, PartialEq)]
pub struct CloseMerkleProofMetadataIxData;
impl CloseMerkleProofMetadataIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CLOSE_MERKLE_PROOF_METADATA_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CLOSE_MERKLE_PROOF_METADATA_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn close_merkle_proof_metadata_ix_with_program_id(
    program_id: Pubkey,
    keys: CloseMerkleProofMetadataKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CLOSE_MERKLE_PROOF_METADATA_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: CloseMerkleProofMetadataIxData.try_to_vec()?,
    })
}
pub fn close_merkle_proof_metadata_ix(
    keys: CloseMerkleProofMetadataKeys,
) -> std::io::Result<Instruction> {
    close_merkle_proof_metadata_ix_with_program_id(ALPHA_VAULT_PROGRAM_ID, keys)
}
pub fn close_merkle_proof_metadata_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CloseMerkleProofMetadataAccounts<'_, '_>,
) -> ProgramResult {
    let keys: CloseMerkleProofMetadataKeys = accounts.into();
    let ix = close_merkle_proof_metadata_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn close_merkle_proof_metadata_invoke(
    accounts: CloseMerkleProofMetadataAccounts<'_, '_>,
) -> ProgramResult {
    close_merkle_proof_metadata_invoke_with_program_id(ALPHA_VAULT_PROGRAM_ID, accounts)
}
pub fn close_merkle_proof_metadata_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CloseMerkleProofMetadataAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CloseMerkleProofMetadataKeys = accounts.into();
    let ix = close_merkle_proof_metadata_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn close_merkle_proof_metadata_invoke_signed(
    accounts: CloseMerkleProofMetadataAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    close_merkle_proof_metadata_invoke_signed_with_program_id(
        ALPHA_VAULT_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn close_merkle_proof_metadata_verify_account_keys(
    accounts: CloseMerkleProofMetadataAccounts<'_, '_>,
    keys: CloseMerkleProofMetadataKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.vault.key, keys.vault),
        (*accounts.merkle_proof_metadata.key, keys.merkle_proof_metadata),
        (*accounts.admin.key, keys.admin),
        (*accounts.rent_receiver.key, keys.rent_receiver),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn close_merkle_proof_metadata_verify_writable_privileges<'me, 'info>(
    accounts: CloseMerkleProofMetadataAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.merkle_proof_metadata, accounts.rent_receiver] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn close_merkle_proof_metadata_verify_signer_privileges<'me, 'info>(
    accounts: CloseMerkleProofMetadataAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn close_merkle_proof_metadata_verify_account_privileges<'me, 'info>(
    accounts: CloseMerkleProofMetadataAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    close_merkle_proof_metadata_verify_writable_privileges(accounts)?;
    close_merkle_proof_metadata_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CLOSE_PRORATA_CONFIG_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct CloseProrataConfigAccounts<'me, 'info> {
    pub config: &'me AccountInfo<'info>,
    pub admin: &'me AccountInfo<'info>,
    pub rent_receiver: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CloseProrataConfigKeys {
    pub config: Pubkey,
    pub admin: Pubkey,
    pub rent_receiver: Pubkey,
}
impl From<CloseProrataConfigAccounts<'_, '_>> for CloseProrataConfigKeys {
    fn from(accounts: CloseProrataConfigAccounts) -> Self {
        Self {
            config: *accounts.config.key,
            admin: *accounts.admin.key,
            rent_receiver: *accounts.rent_receiver.key,
        }
    }
}
impl From<CloseProrataConfigKeys>
for [AccountMeta; CLOSE_PRORATA_CONFIG_IX_ACCOUNTS_LEN] {
    fn from(keys: CloseProrataConfigKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.config,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.admin,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.rent_receiver,
                is_signer: false,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; CLOSE_PRORATA_CONFIG_IX_ACCOUNTS_LEN]> for CloseProrataConfigKeys {
    fn from(pubkeys: [Pubkey; CLOSE_PRORATA_CONFIG_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            config: pubkeys[0],
            admin: pubkeys[1],
            rent_receiver: pubkeys[2],
        }
    }
}
impl<'info> From<CloseProrataConfigAccounts<'_, 'info>>
for [AccountInfo<'info>; CLOSE_PRORATA_CONFIG_IX_ACCOUNTS_LEN] {
    fn from(accounts: CloseProrataConfigAccounts<'_, 'info>) -> Self {
        [accounts.config.clone(), accounts.admin.clone(), accounts.rent_receiver.clone()]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CLOSE_PRORATA_CONFIG_IX_ACCOUNTS_LEN]>
for CloseProrataConfigAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; CLOSE_PRORATA_CONFIG_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            config: &arr[0],
            admin: &arr[1],
            rent_receiver: &arr[2],
        }
    }
}
pub const CLOSE_PRORATA_CONFIG_IX_DISCM: [u8; 8usize] = [
    84, 140, 103, 57, 178, 155, 57, 26,
];
#[derive(Clone, Debug, PartialEq)]
pub struct CloseProrataConfigIxData;
impl CloseProrataConfigIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CLOSE_PRORATA_CONFIG_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CLOSE_PRORATA_CONFIG_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn close_prorata_config_ix_with_program_id(
    program_id: Pubkey,
    keys: CloseProrataConfigKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CLOSE_PRORATA_CONFIG_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: CloseProrataConfigIxData.try_to_vec()?,
    })
}
pub fn close_prorata_config_ix(
    keys: CloseProrataConfigKeys,
) -> std::io::Result<Instruction> {
    close_prorata_config_ix_with_program_id(ALPHA_VAULT_PROGRAM_ID, keys)
}
pub fn close_prorata_config_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CloseProrataConfigAccounts<'_, '_>,
) -> ProgramResult {
    let keys: CloseProrataConfigKeys = accounts.into();
    let ix = close_prorata_config_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn close_prorata_config_invoke(
    accounts: CloseProrataConfigAccounts<'_, '_>,
) -> ProgramResult {
    close_prorata_config_invoke_with_program_id(ALPHA_VAULT_PROGRAM_ID, accounts)
}
pub fn close_prorata_config_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CloseProrataConfigAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CloseProrataConfigKeys = accounts.into();
    let ix = close_prorata_config_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn close_prorata_config_invoke_signed(
    accounts: CloseProrataConfigAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    close_prorata_config_invoke_signed_with_program_id(
        ALPHA_VAULT_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn close_prorata_config_verify_account_keys(
    accounts: CloseProrataConfigAccounts<'_, '_>,
    keys: CloseProrataConfigKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.config.key, keys.config),
        (*accounts.admin.key, keys.admin),
        (*accounts.rent_receiver.key, keys.rent_receiver),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn close_prorata_config_verify_writable_privileges<'me, 'info>(
    accounts: CloseProrataConfigAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.config, accounts.admin, accounts.rent_receiver] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn close_prorata_config_verify_signer_privileges<'me, 'info>(
    accounts: CloseProrataConfigAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn close_prorata_config_verify_account_privileges<'me, 'info>(
    accounts: CloseProrataConfigAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    close_prorata_config_verify_writable_privileges(accounts)?;
    close_prorata_config_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CREATE_CRANK_FEE_WHITELIST_IX_ACCOUNTS_LEN: usize = 6;
#[derive(Copy, Clone, Debug)]
pub struct CreateCrankFeeWhitelistAccounts<'me, 'info> {
    pub crank_fee_whitelist: &'me AccountInfo<'info>,
    pub cranker: &'me AccountInfo<'info>,
    pub admin: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CreateCrankFeeWhitelistKeys {
    pub crank_fee_whitelist: Pubkey,
    pub cranker: Pubkey,
    pub admin: Pubkey,
    pub system_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<CreateCrankFeeWhitelistAccounts<'_, '_>> for CreateCrankFeeWhitelistKeys {
    fn from(accounts: CreateCrankFeeWhitelistAccounts) -> Self {
        Self {
            crank_fee_whitelist: *accounts.crank_fee_whitelist.key,
            cranker: *accounts.cranker.key,
            admin: *accounts.admin.key,
            system_program: *accounts.system_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<CreateCrankFeeWhitelistKeys>
for [AccountMeta; CREATE_CRANK_FEE_WHITELIST_IX_ACCOUNTS_LEN] {
    fn from(keys: CreateCrankFeeWhitelistKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.crank_fee_whitelist,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.cranker,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.admin,
                is_signer: true,
                is_writable: true,
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
impl From<[Pubkey; CREATE_CRANK_FEE_WHITELIST_IX_ACCOUNTS_LEN]>
for CreateCrankFeeWhitelistKeys {
    fn from(pubkeys: [Pubkey; CREATE_CRANK_FEE_WHITELIST_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            crank_fee_whitelist: pubkeys[0],
            cranker: pubkeys[1],
            admin: pubkeys[2],
            system_program: pubkeys[3],
            event_authority: pubkeys[4],
            program: pubkeys[5],
        }
    }
}
impl<'info> From<CreateCrankFeeWhitelistAccounts<'_, 'info>>
for [AccountInfo<'info>; CREATE_CRANK_FEE_WHITELIST_IX_ACCOUNTS_LEN] {
    fn from(accounts: CreateCrankFeeWhitelistAccounts<'_, 'info>) -> Self {
        [
            accounts.crank_fee_whitelist.clone(),
            accounts.cranker.clone(),
            accounts.admin.clone(),
            accounts.system_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; CREATE_CRANK_FEE_WHITELIST_IX_ACCOUNTS_LEN]>
for CreateCrankFeeWhitelistAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; CREATE_CRANK_FEE_WHITELIST_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            crank_fee_whitelist: &arr[0],
            cranker: &arr[1],
            admin: &arr[2],
            system_program: &arr[3],
            event_authority: &arr[4],
            program: &arr[5],
        }
    }
}
pub const CREATE_CRANK_FEE_WHITELIST_IX_DISCM: [u8; 8usize] = [
    120, 91, 25, 162, 211, 27, 100, 199,
];
#[derive(Clone, Debug, PartialEq)]
pub struct CreateCrankFeeWhitelistIxData;
impl CreateCrankFeeWhitelistIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CREATE_CRANK_FEE_WHITELIST_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CREATE_CRANK_FEE_WHITELIST_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn create_crank_fee_whitelist_ix_with_program_id(
    program_id: Pubkey,
    keys: CreateCrankFeeWhitelistKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CREATE_CRANK_FEE_WHITELIST_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: CreateCrankFeeWhitelistIxData.try_to_vec()?,
    })
}
pub fn create_crank_fee_whitelist_ix(
    keys: CreateCrankFeeWhitelistKeys,
) -> std::io::Result<Instruction> {
    create_crank_fee_whitelist_ix_with_program_id(ALPHA_VAULT_PROGRAM_ID, keys)
}
pub fn create_crank_fee_whitelist_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CreateCrankFeeWhitelistAccounts<'_, '_>,
) -> ProgramResult {
    let keys: CreateCrankFeeWhitelistKeys = accounts.into();
    let ix = create_crank_fee_whitelist_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn create_crank_fee_whitelist_invoke(
    accounts: CreateCrankFeeWhitelistAccounts<'_, '_>,
) -> ProgramResult {
    create_crank_fee_whitelist_invoke_with_program_id(ALPHA_VAULT_PROGRAM_ID, accounts)
}
pub fn create_crank_fee_whitelist_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CreateCrankFeeWhitelistAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CreateCrankFeeWhitelistKeys = accounts.into();
    let ix = create_crank_fee_whitelist_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn create_crank_fee_whitelist_invoke_signed(
    accounts: CreateCrankFeeWhitelistAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    create_crank_fee_whitelist_invoke_signed_with_program_id(
        ALPHA_VAULT_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn create_crank_fee_whitelist_verify_account_keys(
    accounts: CreateCrankFeeWhitelistAccounts<'_, '_>,
    keys: CreateCrankFeeWhitelistKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.crank_fee_whitelist.key, keys.crank_fee_whitelist),
        (*accounts.cranker.key, keys.cranker),
        (*accounts.admin.key, keys.admin),
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
pub fn create_crank_fee_whitelist_verify_writable_privileges<'me, 'info>(
    accounts: CreateCrankFeeWhitelistAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.crank_fee_whitelist, accounts.admin] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn create_crank_fee_whitelist_verify_signer_privileges<'me, 'info>(
    accounts: CreateCrankFeeWhitelistAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn create_crank_fee_whitelist_verify_account_privileges<'me, 'info>(
    accounts: CreateCrankFeeWhitelistAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    create_crank_fee_whitelist_verify_writable_privileges(accounts)?;
    create_crank_fee_whitelist_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CREATE_FCFS_CONFIG_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct CreateFcfsConfigAccounts<'me, 'info> {
    pub config: &'me AccountInfo<'info>,
    pub admin: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CreateFcfsConfigKeys {
    pub config: Pubkey,
    pub admin: Pubkey,
    pub system_program: Pubkey,
}
impl From<CreateFcfsConfigAccounts<'_, '_>> for CreateFcfsConfigKeys {
    fn from(accounts: CreateFcfsConfigAccounts) -> Self {
        Self {
            config: *accounts.config.key,
            admin: *accounts.admin.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<CreateFcfsConfigKeys> for [AccountMeta; CREATE_FCFS_CONFIG_IX_ACCOUNTS_LEN] {
    fn from(keys: CreateFcfsConfigKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.config,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.admin,
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
impl From<[Pubkey; CREATE_FCFS_CONFIG_IX_ACCOUNTS_LEN]> for CreateFcfsConfigKeys {
    fn from(pubkeys: [Pubkey; CREATE_FCFS_CONFIG_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            config: pubkeys[0],
            admin: pubkeys[1],
            system_program: pubkeys[2],
        }
    }
}
impl<'info> From<CreateFcfsConfigAccounts<'_, 'info>>
for [AccountInfo<'info>; CREATE_FCFS_CONFIG_IX_ACCOUNTS_LEN] {
    fn from(accounts: CreateFcfsConfigAccounts<'_, 'info>) -> Self {
        [
            accounts.config.clone(),
            accounts.admin.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CREATE_FCFS_CONFIG_IX_ACCOUNTS_LEN]>
for CreateFcfsConfigAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; CREATE_FCFS_CONFIG_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            config: &arr[0],
            admin: &arr[1],
            system_program: &arr[2],
        }
    }
}
pub const CREATE_FCFS_CONFIG_IX_DISCM: [u8; 8usize] = [7, 255, 242, 242, 1, 99, 179, 12];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CreateFcfsConfigIxArgs {
    pub config_parameters: FcfsConfigParameters,
}
#[derive(Clone, Debug, PartialEq)]
pub struct CreateFcfsConfigIxData(pub CreateFcfsConfigIxArgs);
impl From<CreateFcfsConfigIxArgs> for CreateFcfsConfigIxData {
    fn from(args: CreateFcfsConfigIxArgs) -> Self {
        Self(args)
    }
}
impl CreateFcfsConfigIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CREATE_FCFS_CONFIG_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let config_parameters = if reader.is_empty() {
            Default::default()
        } else {
            <FcfsConfigParameters>::deserialize(&mut reader)?
        };
        Ok(
            Self(CreateFcfsConfigIxArgs {
                config_parameters,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CREATE_FCFS_CONFIG_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.config_parameters, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn create_fcfs_config_ix_with_program_id(
    program_id: Pubkey,
    keys: CreateFcfsConfigKeys,
    args: CreateFcfsConfigIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CREATE_FCFS_CONFIG_IX_ACCOUNTS_LEN] = keys.into();
    let data: CreateFcfsConfigIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn create_fcfs_config_ix(
    keys: CreateFcfsConfigKeys,
    args: CreateFcfsConfigIxArgs,
) -> std::io::Result<Instruction> {
    create_fcfs_config_ix_with_program_id(ALPHA_VAULT_PROGRAM_ID, keys, args)
}
pub fn create_fcfs_config_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CreateFcfsConfigAccounts<'_, '_>,
    args: CreateFcfsConfigIxArgs,
) -> ProgramResult {
    let keys: CreateFcfsConfigKeys = accounts.into();
    let ix = create_fcfs_config_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn create_fcfs_config_invoke(
    accounts: CreateFcfsConfigAccounts<'_, '_>,
    args: CreateFcfsConfigIxArgs,
) -> ProgramResult {
    create_fcfs_config_invoke_with_program_id(ALPHA_VAULT_PROGRAM_ID, accounts, args)
}
pub fn create_fcfs_config_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CreateFcfsConfigAccounts<'_, '_>,
    args: CreateFcfsConfigIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CreateFcfsConfigKeys = accounts.into();
    let ix = create_fcfs_config_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn create_fcfs_config_invoke_signed(
    accounts: CreateFcfsConfigAccounts<'_, '_>,
    args: CreateFcfsConfigIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    create_fcfs_config_invoke_signed_with_program_id(
        ALPHA_VAULT_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn create_fcfs_config_verify_account_keys(
    accounts: CreateFcfsConfigAccounts<'_, '_>,
    keys: CreateFcfsConfigKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.config.key, keys.config),
        (*accounts.admin.key, keys.admin),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn create_fcfs_config_verify_writable_privileges<'me, 'info>(
    accounts: CreateFcfsConfigAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.config, accounts.admin] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn create_fcfs_config_verify_signer_privileges<'me, 'info>(
    accounts: CreateFcfsConfigAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn create_fcfs_config_verify_account_privileges<'me, 'info>(
    accounts: CreateFcfsConfigAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    create_fcfs_config_verify_writable_privileges(accounts)?;
    create_fcfs_config_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CREATE_MERKLE_PROOF_METADATA_IX_ACCOUNTS_LEN: usize = 6;
#[derive(Copy, Clone, Debug)]
pub struct CreateMerkleProofMetadataAccounts<'me, 'info> {
    pub vault: &'me AccountInfo<'info>,
    pub merkle_proof_metadata: &'me AccountInfo<'info>,
    pub admin: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CreateMerkleProofMetadataKeys {
    pub vault: Pubkey,
    pub merkle_proof_metadata: Pubkey,
    pub admin: Pubkey,
    pub system_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<CreateMerkleProofMetadataAccounts<'_, '_>> for CreateMerkleProofMetadataKeys {
    fn from(accounts: CreateMerkleProofMetadataAccounts) -> Self {
        Self {
            vault: *accounts.vault.key,
            merkle_proof_metadata: *accounts.merkle_proof_metadata.key,
            admin: *accounts.admin.key,
            system_program: *accounts.system_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<CreateMerkleProofMetadataKeys>
for [AccountMeta; CREATE_MERKLE_PROOF_METADATA_IX_ACCOUNTS_LEN] {
    fn from(keys: CreateMerkleProofMetadataKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.vault,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.merkle_proof_metadata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.admin,
                is_signer: true,
                is_writable: true,
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
impl From<[Pubkey; CREATE_MERKLE_PROOF_METADATA_IX_ACCOUNTS_LEN]>
for CreateMerkleProofMetadataKeys {
    fn from(pubkeys: [Pubkey; CREATE_MERKLE_PROOF_METADATA_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            vault: pubkeys[0],
            merkle_proof_metadata: pubkeys[1],
            admin: pubkeys[2],
            system_program: pubkeys[3],
            event_authority: pubkeys[4],
            program: pubkeys[5],
        }
    }
}
impl<'info> From<CreateMerkleProofMetadataAccounts<'_, 'info>>
for [AccountInfo<'info>; CREATE_MERKLE_PROOF_METADATA_IX_ACCOUNTS_LEN] {
    fn from(accounts: CreateMerkleProofMetadataAccounts<'_, 'info>) -> Self {
        [
            accounts.vault.clone(),
            accounts.merkle_proof_metadata.clone(),
            accounts.admin.clone(),
            accounts.system_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; CREATE_MERKLE_PROOF_METADATA_IX_ACCOUNTS_LEN]>
for CreateMerkleProofMetadataAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; CREATE_MERKLE_PROOF_METADATA_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            vault: &arr[0],
            merkle_proof_metadata: &arr[1],
            admin: &arr[2],
            system_program: &arr[3],
            event_authority: &arr[4],
            program: &arr[5],
        }
    }
}
pub const CREATE_MERKLE_PROOF_METADATA_IX_DISCM: [u8; 8usize] = [
    151, 46, 163, 52, 181, 178, 47, 227,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CreateMerkleProofMetadataIxArgs {
    pub proof_url: String,
}
#[derive(Clone, Debug, PartialEq)]
pub struct CreateMerkleProofMetadataIxData(pub CreateMerkleProofMetadataIxArgs);
impl From<CreateMerkleProofMetadataIxArgs> for CreateMerkleProofMetadataIxData {
    fn from(args: CreateMerkleProofMetadataIxArgs) -> Self {
        Self(args)
    }
}
impl CreateMerkleProofMetadataIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CREATE_MERKLE_PROOF_METADATA_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let proof_url: String = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(CreateMerkleProofMetadataIxArgs {
                proof_url,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CREATE_MERKLE_PROOF_METADATA_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.proof_url, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn create_merkle_proof_metadata_ix_with_program_id(
    program_id: Pubkey,
    keys: CreateMerkleProofMetadataKeys,
    args: CreateMerkleProofMetadataIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CREATE_MERKLE_PROOF_METADATA_IX_ACCOUNTS_LEN] = keys.into();
    let data: CreateMerkleProofMetadataIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn create_merkle_proof_metadata_ix(
    keys: CreateMerkleProofMetadataKeys,
    args: CreateMerkleProofMetadataIxArgs,
) -> std::io::Result<Instruction> {
    create_merkle_proof_metadata_ix_with_program_id(ALPHA_VAULT_PROGRAM_ID, keys, args)
}
pub fn create_merkle_proof_metadata_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CreateMerkleProofMetadataAccounts<'_, '_>,
    args: CreateMerkleProofMetadataIxArgs,
) -> ProgramResult {
    let keys: CreateMerkleProofMetadataKeys = accounts.into();
    let ix = create_merkle_proof_metadata_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn create_merkle_proof_metadata_invoke(
    accounts: CreateMerkleProofMetadataAccounts<'_, '_>,
    args: CreateMerkleProofMetadataIxArgs,
) -> ProgramResult {
    create_merkle_proof_metadata_invoke_with_program_id(
        ALPHA_VAULT_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn create_merkle_proof_metadata_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CreateMerkleProofMetadataAccounts<'_, '_>,
    args: CreateMerkleProofMetadataIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CreateMerkleProofMetadataKeys = accounts.into();
    let ix = create_merkle_proof_metadata_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn create_merkle_proof_metadata_invoke_signed(
    accounts: CreateMerkleProofMetadataAccounts<'_, '_>,
    args: CreateMerkleProofMetadataIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    create_merkle_proof_metadata_invoke_signed_with_program_id(
        ALPHA_VAULT_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn create_merkle_proof_metadata_verify_account_keys(
    accounts: CreateMerkleProofMetadataAccounts<'_, '_>,
    keys: CreateMerkleProofMetadataKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.vault.key, keys.vault),
        (*accounts.merkle_proof_metadata.key, keys.merkle_proof_metadata),
        (*accounts.admin.key, keys.admin),
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
pub fn create_merkle_proof_metadata_verify_writable_privileges<'me, 'info>(
    accounts: CreateMerkleProofMetadataAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.merkle_proof_metadata, accounts.admin] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn create_merkle_proof_metadata_verify_signer_privileges<'me, 'info>(
    accounts: CreateMerkleProofMetadataAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn create_merkle_proof_metadata_verify_account_privileges<'me, 'info>(
    accounts: CreateMerkleProofMetadataAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    create_merkle_proof_metadata_verify_writable_privileges(accounts)?;
    create_merkle_proof_metadata_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CREATE_MERKLE_ROOT_CONFIG_IX_ACCOUNTS_LEN: usize = 6;
#[derive(Copy, Clone, Debug)]
pub struct CreateMerkleRootConfigAccounts<'me, 'info> {
    pub vault: &'me AccountInfo<'info>,
    pub merkle_root_config: &'me AccountInfo<'info>,
    pub admin: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CreateMerkleRootConfigKeys {
    pub vault: Pubkey,
    pub merkle_root_config: Pubkey,
    pub admin: Pubkey,
    pub system_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<CreateMerkleRootConfigAccounts<'_, '_>> for CreateMerkleRootConfigKeys {
    fn from(accounts: CreateMerkleRootConfigAccounts) -> Self {
        Self {
            vault: *accounts.vault.key,
            merkle_root_config: *accounts.merkle_root_config.key,
            admin: *accounts.admin.key,
            system_program: *accounts.system_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<CreateMerkleRootConfigKeys>
for [AccountMeta; CREATE_MERKLE_ROOT_CONFIG_IX_ACCOUNTS_LEN] {
    fn from(keys: CreateMerkleRootConfigKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.vault,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.merkle_root_config,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.admin,
                is_signer: true,
                is_writable: true,
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
impl From<[Pubkey; CREATE_MERKLE_ROOT_CONFIG_IX_ACCOUNTS_LEN]>
for CreateMerkleRootConfigKeys {
    fn from(pubkeys: [Pubkey; CREATE_MERKLE_ROOT_CONFIG_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            vault: pubkeys[0],
            merkle_root_config: pubkeys[1],
            admin: pubkeys[2],
            system_program: pubkeys[3],
            event_authority: pubkeys[4],
            program: pubkeys[5],
        }
    }
}
impl<'info> From<CreateMerkleRootConfigAccounts<'_, 'info>>
for [AccountInfo<'info>; CREATE_MERKLE_ROOT_CONFIG_IX_ACCOUNTS_LEN] {
    fn from(accounts: CreateMerkleRootConfigAccounts<'_, 'info>) -> Self {
        [
            accounts.vault.clone(),
            accounts.merkle_root_config.clone(),
            accounts.admin.clone(),
            accounts.system_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; CREATE_MERKLE_ROOT_CONFIG_IX_ACCOUNTS_LEN]>
for CreateMerkleRootConfigAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; CREATE_MERKLE_ROOT_CONFIG_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            vault: &arr[0],
            merkle_root_config: &arr[1],
            admin: &arr[2],
            system_program: &arr[3],
            event_authority: &arr[4],
            program: &arr[5],
        }
    }
}
pub const CREATE_MERKLE_ROOT_CONFIG_IX_DISCM: [u8; 8usize] = [
    55, 243, 253, 240, 78, 186, 232, 166,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CreateMerkleRootConfigIxArgs {
    pub params: CreateMerkleRootConfigParams,
}
#[derive(Clone, Debug, PartialEq)]
pub struct CreateMerkleRootConfigIxData(pub CreateMerkleRootConfigIxArgs);
impl From<CreateMerkleRootConfigIxArgs> for CreateMerkleRootConfigIxData {
    fn from(args: CreateMerkleRootConfigIxArgs) -> Self {
        Self(args)
    }
}
impl CreateMerkleRootConfigIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CREATE_MERKLE_ROOT_CONFIG_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let params = if reader.is_empty() {
            Default::default()
        } else {
            <CreateMerkleRootConfigParams>::deserialize(&mut reader)?
        };
        Ok(
            Self(CreateMerkleRootConfigIxArgs {
                params,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CREATE_MERKLE_ROOT_CONFIG_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.params, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn create_merkle_root_config_ix_with_program_id(
    program_id: Pubkey,
    keys: CreateMerkleRootConfigKeys,
    args: CreateMerkleRootConfigIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CREATE_MERKLE_ROOT_CONFIG_IX_ACCOUNTS_LEN] = keys.into();
    let data: CreateMerkleRootConfigIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn create_merkle_root_config_ix(
    keys: CreateMerkleRootConfigKeys,
    args: CreateMerkleRootConfigIxArgs,
) -> std::io::Result<Instruction> {
    create_merkle_root_config_ix_with_program_id(ALPHA_VAULT_PROGRAM_ID, keys, args)
}
pub fn create_merkle_root_config_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CreateMerkleRootConfigAccounts<'_, '_>,
    args: CreateMerkleRootConfigIxArgs,
) -> ProgramResult {
    let keys: CreateMerkleRootConfigKeys = accounts.into();
    let ix = create_merkle_root_config_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn create_merkle_root_config_invoke(
    accounts: CreateMerkleRootConfigAccounts<'_, '_>,
    args: CreateMerkleRootConfigIxArgs,
) -> ProgramResult {
    create_merkle_root_config_invoke_with_program_id(
        ALPHA_VAULT_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn create_merkle_root_config_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CreateMerkleRootConfigAccounts<'_, '_>,
    args: CreateMerkleRootConfigIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CreateMerkleRootConfigKeys = accounts.into();
    let ix = create_merkle_root_config_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn create_merkle_root_config_invoke_signed(
    accounts: CreateMerkleRootConfigAccounts<'_, '_>,
    args: CreateMerkleRootConfigIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    create_merkle_root_config_invoke_signed_with_program_id(
        ALPHA_VAULT_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn create_merkle_root_config_verify_account_keys(
    accounts: CreateMerkleRootConfigAccounts<'_, '_>,
    keys: CreateMerkleRootConfigKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.vault.key, keys.vault),
        (*accounts.merkle_root_config.key, keys.merkle_root_config),
        (*accounts.admin.key, keys.admin),
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
pub fn create_merkle_root_config_verify_writable_privileges<'me, 'info>(
    accounts: CreateMerkleRootConfigAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.merkle_root_config, accounts.admin] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn create_merkle_root_config_verify_signer_privileges<'me, 'info>(
    accounts: CreateMerkleRootConfigAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn create_merkle_root_config_verify_account_privileges<'me, 'info>(
    accounts: CreateMerkleRootConfigAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    create_merkle_root_config_verify_writable_privileges(accounts)?;
    create_merkle_root_config_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CREATE_NEW_ESCROW_IX_ACCOUNTS_LEN: usize = 9;
#[derive(Copy, Clone, Debug)]
pub struct CreateNewEscrowAccounts<'me, 'info> {
    pub vault: &'me AccountInfo<'info>,
    pub pool: &'me AccountInfo<'info>,
    pub escrow: &'me AccountInfo<'info>,
    pub owner: &'me AccountInfo<'info>,
    pub payer: &'me AccountInfo<'info>,
    pub escrow_fee_receiver: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CreateNewEscrowKeys {
    pub vault: Pubkey,
    pub pool: Pubkey,
    pub escrow: Pubkey,
    pub owner: Pubkey,
    pub payer: Pubkey,
    pub escrow_fee_receiver: Pubkey,
    pub system_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<CreateNewEscrowAccounts<'_, '_>> for CreateNewEscrowKeys {
    fn from(accounts: CreateNewEscrowAccounts) -> Self {
        Self {
            vault: *accounts.vault.key,
            pool: *accounts.pool.key,
            escrow: *accounts.escrow.key,
            owner: *accounts.owner.key,
            payer: *accounts.payer.key,
            escrow_fee_receiver: *accounts.escrow_fee_receiver.key,
            system_program: *accounts.system_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<CreateNewEscrowKeys> for [AccountMeta; CREATE_NEW_ESCROW_IX_ACCOUNTS_LEN] {
    fn from(keys: CreateNewEscrowKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pool,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.escrow,
                is_signer: false,
                is_writable: true,
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
                pubkey: keys.escrow_fee_receiver,
                is_signer: false,
                is_writable: true,
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
impl From<[Pubkey; CREATE_NEW_ESCROW_IX_ACCOUNTS_LEN]> for CreateNewEscrowKeys {
    fn from(pubkeys: [Pubkey; CREATE_NEW_ESCROW_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            vault: pubkeys[0],
            pool: pubkeys[1],
            escrow: pubkeys[2],
            owner: pubkeys[3],
            payer: pubkeys[4],
            escrow_fee_receiver: pubkeys[5],
            system_program: pubkeys[6],
            event_authority: pubkeys[7],
            program: pubkeys[8],
        }
    }
}
impl<'info> From<CreateNewEscrowAccounts<'_, 'info>>
for [AccountInfo<'info>; CREATE_NEW_ESCROW_IX_ACCOUNTS_LEN] {
    fn from(accounts: CreateNewEscrowAccounts<'_, 'info>) -> Self {
        [
            accounts.vault.clone(),
            accounts.pool.clone(),
            accounts.escrow.clone(),
            accounts.owner.clone(),
            accounts.payer.clone(),
            accounts.escrow_fee_receiver.clone(),
            accounts.system_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CREATE_NEW_ESCROW_IX_ACCOUNTS_LEN]>
for CreateNewEscrowAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; CREATE_NEW_ESCROW_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            vault: &arr[0],
            pool: &arr[1],
            escrow: &arr[2],
            owner: &arr[3],
            payer: &arr[4],
            escrow_fee_receiver: &arr[5],
            system_program: &arr[6],
            event_authority: &arr[7],
            program: &arr[8],
        }
    }
}
pub const CREATE_NEW_ESCROW_IX_DISCM: [u8; 8usize] = [
    60, 154, 170, 202, 252, 109, 83, 199,
];
#[derive(Clone, Debug, PartialEq)]
pub struct CreateNewEscrowIxData;
impl CreateNewEscrowIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CREATE_NEW_ESCROW_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CREATE_NEW_ESCROW_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn create_new_escrow_ix_with_program_id(
    program_id: Pubkey,
    keys: CreateNewEscrowKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CREATE_NEW_ESCROW_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: CreateNewEscrowIxData.try_to_vec()?,
    })
}
pub fn create_new_escrow_ix(keys: CreateNewEscrowKeys) -> std::io::Result<Instruction> {
    create_new_escrow_ix_with_program_id(ALPHA_VAULT_PROGRAM_ID, keys)
}
pub fn create_new_escrow_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CreateNewEscrowAccounts<'_, '_>,
) -> ProgramResult {
    let keys: CreateNewEscrowKeys = accounts.into();
    let ix = create_new_escrow_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn create_new_escrow_invoke(
    accounts: CreateNewEscrowAccounts<'_, '_>,
) -> ProgramResult {
    create_new_escrow_invoke_with_program_id(ALPHA_VAULT_PROGRAM_ID, accounts)
}
pub fn create_new_escrow_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CreateNewEscrowAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CreateNewEscrowKeys = accounts.into();
    let ix = create_new_escrow_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn create_new_escrow_invoke_signed(
    accounts: CreateNewEscrowAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    create_new_escrow_invoke_signed_with_program_id(
        ALPHA_VAULT_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn create_new_escrow_verify_account_keys(
    accounts: CreateNewEscrowAccounts<'_, '_>,
    keys: CreateNewEscrowKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.vault.key, keys.vault),
        (*accounts.pool.key, keys.pool),
        (*accounts.escrow.key, keys.escrow),
        (*accounts.owner.key, keys.owner),
        (*accounts.payer.key, keys.payer),
        (*accounts.escrow_fee_receiver.key, keys.escrow_fee_receiver),
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
pub fn create_new_escrow_verify_writable_privileges<'me, 'info>(
    accounts: CreateNewEscrowAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.vault,
        accounts.escrow,
        accounts.payer,
        accounts.escrow_fee_receiver,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn create_new_escrow_verify_signer_privileges<'me, 'info>(
    accounts: CreateNewEscrowAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.payer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn create_new_escrow_verify_account_privileges<'me, 'info>(
    accounts: CreateNewEscrowAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    create_new_escrow_verify_writable_privileges(accounts)?;
    create_new_escrow_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CREATE_PERMISSIONED_ESCROW_IX_ACCOUNTS_LEN: usize = 10;
#[derive(Copy, Clone, Debug)]
pub struct CreatePermissionedEscrowAccounts<'me, 'info> {
    pub vault: &'me AccountInfo<'info>,
    pub pool: &'me AccountInfo<'info>,
    pub escrow: &'me AccountInfo<'info>,
    pub owner: &'me AccountInfo<'info>,
    pub merkle_root_config: &'me AccountInfo<'info>,
    pub payer: &'me AccountInfo<'info>,
    pub escrow_fee_receiver: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CreatePermissionedEscrowKeys {
    pub vault: Pubkey,
    pub pool: Pubkey,
    pub escrow: Pubkey,
    pub owner: Pubkey,
    pub merkle_root_config: Pubkey,
    pub payer: Pubkey,
    pub escrow_fee_receiver: Pubkey,
    pub system_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<CreatePermissionedEscrowAccounts<'_, '_>> for CreatePermissionedEscrowKeys {
    fn from(accounts: CreatePermissionedEscrowAccounts) -> Self {
        Self {
            vault: *accounts.vault.key,
            pool: *accounts.pool.key,
            escrow: *accounts.escrow.key,
            owner: *accounts.owner.key,
            merkle_root_config: *accounts.merkle_root_config.key,
            payer: *accounts.payer.key,
            escrow_fee_receiver: *accounts.escrow_fee_receiver.key,
            system_program: *accounts.system_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<CreatePermissionedEscrowKeys>
for [AccountMeta; CREATE_PERMISSIONED_ESCROW_IX_ACCOUNTS_LEN] {
    fn from(keys: CreatePermissionedEscrowKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pool,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.escrow,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.owner,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.merkle_root_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.payer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.escrow_fee_receiver,
                is_signer: false,
                is_writable: true,
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
impl From<[Pubkey; CREATE_PERMISSIONED_ESCROW_IX_ACCOUNTS_LEN]>
for CreatePermissionedEscrowKeys {
    fn from(pubkeys: [Pubkey; CREATE_PERMISSIONED_ESCROW_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            vault: pubkeys[0],
            pool: pubkeys[1],
            escrow: pubkeys[2],
            owner: pubkeys[3],
            merkle_root_config: pubkeys[4],
            payer: pubkeys[5],
            escrow_fee_receiver: pubkeys[6],
            system_program: pubkeys[7],
            event_authority: pubkeys[8],
            program: pubkeys[9],
        }
    }
}
impl<'info> From<CreatePermissionedEscrowAccounts<'_, 'info>>
for [AccountInfo<'info>; CREATE_PERMISSIONED_ESCROW_IX_ACCOUNTS_LEN] {
    fn from(accounts: CreatePermissionedEscrowAccounts<'_, 'info>) -> Self {
        [
            accounts.vault.clone(),
            accounts.pool.clone(),
            accounts.escrow.clone(),
            accounts.owner.clone(),
            accounts.merkle_root_config.clone(),
            accounts.payer.clone(),
            accounts.escrow_fee_receiver.clone(),
            accounts.system_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; CREATE_PERMISSIONED_ESCROW_IX_ACCOUNTS_LEN]>
for CreatePermissionedEscrowAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; CREATE_PERMISSIONED_ESCROW_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            vault: &arr[0],
            pool: &arr[1],
            escrow: &arr[2],
            owner: &arr[3],
            merkle_root_config: &arr[4],
            payer: &arr[5],
            escrow_fee_receiver: &arr[6],
            system_program: &arr[7],
            event_authority: &arr[8],
            program: &arr[9],
        }
    }
}
pub const CREATE_PERMISSIONED_ESCROW_IX_DISCM: [u8; 8usize] = [
    60, 166, 36, 85, 96, 137, 132, 184,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CreatePermissionedEscrowIxArgs {
    pub max_cap: u64,
    pub proof: Vec<[u8; 32]>,
}
#[derive(Clone, Debug, PartialEq)]
pub struct CreatePermissionedEscrowIxData(pub CreatePermissionedEscrowIxArgs);
impl From<CreatePermissionedEscrowIxArgs> for CreatePermissionedEscrowIxData {
    fn from(args: CreatePermissionedEscrowIxArgs) -> Self {
        Self(args)
    }
}
impl CreatePermissionedEscrowIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CREATE_PERMISSIONED_ESCROW_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let max_cap: u64 = crate::borsh_de_or_default(&mut reader)?;
        let proof: Vec<[u8; 32]> = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(CreatePermissionedEscrowIxArgs {
                max_cap,
                proof,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CREATE_PERMISSIONED_ESCROW_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.max_cap, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.proof, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn create_permissioned_escrow_ix_with_program_id(
    program_id: Pubkey,
    keys: CreatePermissionedEscrowKeys,
    args: CreatePermissionedEscrowIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CREATE_PERMISSIONED_ESCROW_IX_ACCOUNTS_LEN] = keys.into();
    let data: CreatePermissionedEscrowIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn create_permissioned_escrow_ix(
    keys: CreatePermissionedEscrowKeys,
    args: CreatePermissionedEscrowIxArgs,
) -> std::io::Result<Instruction> {
    create_permissioned_escrow_ix_with_program_id(ALPHA_VAULT_PROGRAM_ID, keys, args)
}
pub fn create_permissioned_escrow_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CreatePermissionedEscrowAccounts<'_, '_>,
    args: CreatePermissionedEscrowIxArgs,
) -> ProgramResult {
    let keys: CreatePermissionedEscrowKeys = accounts.into();
    let ix = create_permissioned_escrow_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn create_permissioned_escrow_invoke(
    accounts: CreatePermissionedEscrowAccounts<'_, '_>,
    args: CreatePermissionedEscrowIxArgs,
) -> ProgramResult {
    create_permissioned_escrow_invoke_with_program_id(
        ALPHA_VAULT_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn create_permissioned_escrow_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CreatePermissionedEscrowAccounts<'_, '_>,
    args: CreatePermissionedEscrowIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CreatePermissionedEscrowKeys = accounts.into();
    let ix = create_permissioned_escrow_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn create_permissioned_escrow_invoke_signed(
    accounts: CreatePermissionedEscrowAccounts<'_, '_>,
    args: CreatePermissionedEscrowIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    create_permissioned_escrow_invoke_signed_with_program_id(
        ALPHA_VAULT_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn create_permissioned_escrow_verify_account_keys(
    accounts: CreatePermissionedEscrowAccounts<'_, '_>,
    keys: CreatePermissionedEscrowKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.vault.key, keys.vault),
        (*accounts.pool.key, keys.pool),
        (*accounts.escrow.key, keys.escrow),
        (*accounts.owner.key, keys.owner),
        (*accounts.merkle_root_config.key, keys.merkle_root_config),
        (*accounts.payer.key, keys.payer),
        (*accounts.escrow_fee_receiver.key, keys.escrow_fee_receiver),
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
pub fn create_permissioned_escrow_verify_writable_privileges<'me, 'info>(
    accounts: CreatePermissionedEscrowAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.vault,
        accounts.escrow,
        accounts.payer,
        accounts.escrow_fee_receiver,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn create_permissioned_escrow_verify_signer_privileges<'me, 'info>(
    accounts: CreatePermissionedEscrowAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.payer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn create_permissioned_escrow_verify_account_privileges<'me, 'info>(
    accounts: CreatePermissionedEscrowAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    create_permissioned_escrow_verify_writable_privileges(accounts)?;
    create_permissioned_escrow_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CREATE_PERMISSIONED_ESCROW_WITH_AUTHORITY_IX_ACCOUNTS_LEN: usize = 8;
#[derive(Copy, Clone, Debug)]
pub struct CreatePermissionedEscrowWithAuthorityAccounts<'me, 'info> {
    pub vault: &'me AccountInfo<'info>,
    pub pool: &'me AccountInfo<'info>,
    pub escrow: &'me AccountInfo<'info>,
    pub owner: &'me AccountInfo<'info>,
    pub payer: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CreatePermissionedEscrowWithAuthorityKeys {
    pub vault: Pubkey,
    pub pool: Pubkey,
    pub escrow: Pubkey,
    pub owner: Pubkey,
    pub payer: Pubkey,
    pub system_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<CreatePermissionedEscrowWithAuthorityAccounts<'_, '_>>
for CreatePermissionedEscrowWithAuthorityKeys {
    fn from(accounts: CreatePermissionedEscrowWithAuthorityAccounts) -> Self {
        Self {
            vault: *accounts.vault.key,
            pool: *accounts.pool.key,
            escrow: *accounts.escrow.key,
            owner: *accounts.owner.key,
            payer: *accounts.payer.key,
            system_program: *accounts.system_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<CreatePermissionedEscrowWithAuthorityKeys>
for [AccountMeta; CREATE_PERMISSIONED_ESCROW_WITH_AUTHORITY_IX_ACCOUNTS_LEN] {
    fn from(keys: CreatePermissionedEscrowWithAuthorityKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pool,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.escrow,
                is_signer: false,
                is_writable: true,
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
impl From<[Pubkey; CREATE_PERMISSIONED_ESCROW_WITH_AUTHORITY_IX_ACCOUNTS_LEN]>
for CreatePermissionedEscrowWithAuthorityKeys {
    fn from(
        pubkeys: [Pubkey; CREATE_PERMISSIONED_ESCROW_WITH_AUTHORITY_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            vault: pubkeys[0],
            pool: pubkeys[1],
            escrow: pubkeys[2],
            owner: pubkeys[3],
            payer: pubkeys[4],
            system_program: pubkeys[5],
            event_authority: pubkeys[6],
            program: pubkeys[7],
        }
    }
}
impl<'info> From<CreatePermissionedEscrowWithAuthorityAccounts<'_, 'info>>
for [AccountInfo<'info>; CREATE_PERMISSIONED_ESCROW_WITH_AUTHORITY_IX_ACCOUNTS_LEN] {
    fn from(accounts: CreatePermissionedEscrowWithAuthorityAccounts<'_, 'info>) -> Self {
        [
            accounts.vault.clone(),
            accounts.pool.clone(),
            accounts.escrow.clone(),
            accounts.owner.clone(),
            accounts.payer.clone(),
            accounts.system_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<
    &'me [AccountInfo<'info>; CREATE_PERMISSIONED_ESCROW_WITH_AUTHORITY_IX_ACCOUNTS_LEN],
> for CreatePermissionedEscrowWithAuthorityAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<
            'info,
        >; CREATE_PERMISSIONED_ESCROW_WITH_AUTHORITY_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            vault: &arr[0],
            pool: &arr[1],
            escrow: &arr[2],
            owner: &arr[3],
            payer: &arr[4],
            system_program: &arr[5],
            event_authority: &arr[6],
            program: &arr[7],
        }
    }
}
pub const CREATE_PERMISSIONED_ESCROW_WITH_AUTHORITY_IX_DISCM: [u8; 8usize] = [
    211, 231, 194, 69, 65, 11, 123, 93,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CreatePermissionedEscrowWithAuthorityIxArgs {
    pub max_cap: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct CreatePermissionedEscrowWithAuthorityIxData(
    pub CreatePermissionedEscrowWithAuthorityIxArgs,
);
impl From<CreatePermissionedEscrowWithAuthorityIxArgs>
for CreatePermissionedEscrowWithAuthorityIxData {
    fn from(args: CreatePermissionedEscrowWithAuthorityIxArgs) -> Self {
        Self(args)
    }
}
impl CreatePermissionedEscrowWithAuthorityIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CREATE_PERMISSIONED_ESCROW_WITH_AUTHORITY_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let max_cap: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(CreatePermissionedEscrowWithAuthorityIxArgs {
                max_cap,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CREATE_PERMISSIONED_ESCROW_WITH_AUTHORITY_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.max_cap, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn create_permissioned_escrow_with_authority_ix_with_program_id(
    program_id: Pubkey,
    keys: CreatePermissionedEscrowWithAuthorityKeys,
    args: CreatePermissionedEscrowWithAuthorityIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CREATE_PERMISSIONED_ESCROW_WITH_AUTHORITY_IX_ACCOUNTS_LEN] = keys
        .into();
    let data: CreatePermissionedEscrowWithAuthorityIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn create_permissioned_escrow_with_authority_ix(
    keys: CreatePermissionedEscrowWithAuthorityKeys,
    args: CreatePermissionedEscrowWithAuthorityIxArgs,
) -> std::io::Result<Instruction> {
    create_permissioned_escrow_with_authority_ix_with_program_id(
        ALPHA_VAULT_PROGRAM_ID,
        keys,
        args,
    )
}
pub fn create_permissioned_escrow_with_authority_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CreatePermissionedEscrowWithAuthorityAccounts<'_, '_>,
    args: CreatePermissionedEscrowWithAuthorityIxArgs,
) -> ProgramResult {
    let keys: CreatePermissionedEscrowWithAuthorityKeys = accounts.into();
    let ix = create_permissioned_escrow_with_authority_ix_with_program_id(
        program_id,
        keys,
        args,
    )?;
    invoke_instruction(&ix, accounts)
}
pub fn create_permissioned_escrow_with_authority_invoke(
    accounts: CreatePermissionedEscrowWithAuthorityAccounts<'_, '_>,
    args: CreatePermissionedEscrowWithAuthorityIxArgs,
) -> ProgramResult {
    create_permissioned_escrow_with_authority_invoke_with_program_id(
        ALPHA_VAULT_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn create_permissioned_escrow_with_authority_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CreatePermissionedEscrowWithAuthorityAccounts<'_, '_>,
    args: CreatePermissionedEscrowWithAuthorityIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CreatePermissionedEscrowWithAuthorityKeys = accounts.into();
    let ix = create_permissioned_escrow_with_authority_ix_with_program_id(
        program_id,
        keys,
        args,
    )?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn create_permissioned_escrow_with_authority_invoke_signed(
    accounts: CreatePermissionedEscrowWithAuthorityAccounts<'_, '_>,
    args: CreatePermissionedEscrowWithAuthorityIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    create_permissioned_escrow_with_authority_invoke_signed_with_program_id(
        ALPHA_VAULT_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn create_permissioned_escrow_with_authority_verify_account_keys(
    accounts: CreatePermissionedEscrowWithAuthorityAccounts<'_, '_>,
    keys: CreatePermissionedEscrowWithAuthorityKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.vault.key, keys.vault),
        (*accounts.pool.key, keys.pool),
        (*accounts.escrow.key, keys.escrow),
        (*accounts.owner.key, keys.owner),
        (*accounts.payer.key, keys.payer),
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
pub fn create_permissioned_escrow_with_authority_verify_writable_privileges<'me, 'info>(
    accounts: CreatePermissionedEscrowWithAuthorityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.vault, accounts.escrow, accounts.payer] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn create_permissioned_escrow_with_authority_verify_signer_privileges<'me, 'info>(
    accounts: CreatePermissionedEscrowWithAuthorityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.payer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn create_permissioned_escrow_with_authority_verify_account_privileges<'me, 'info>(
    accounts: CreatePermissionedEscrowWithAuthorityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    create_permissioned_escrow_with_authority_verify_writable_privileges(accounts)?;
    create_permissioned_escrow_with_authority_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CREATE_PRORATA_CONFIG_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct CreateProrataConfigAccounts<'me, 'info> {
    pub config: &'me AccountInfo<'info>,
    pub admin: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CreateProrataConfigKeys {
    pub config: Pubkey,
    pub admin: Pubkey,
    pub system_program: Pubkey,
}
impl From<CreateProrataConfigAccounts<'_, '_>> for CreateProrataConfigKeys {
    fn from(accounts: CreateProrataConfigAccounts) -> Self {
        Self {
            config: *accounts.config.key,
            admin: *accounts.admin.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<CreateProrataConfigKeys>
for [AccountMeta; CREATE_PRORATA_CONFIG_IX_ACCOUNTS_LEN] {
    fn from(keys: CreateProrataConfigKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.config,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.admin,
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
impl From<[Pubkey; CREATE_PRORATA_CONFIG_IX_ACCOUNTS_LEN]> for CreateProrataConfigKeys {
    fn from(pubkeys: [Pubkey; CREATE_PRORATA_CONFIG_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            config: pubkeys[0],
            admin: pubkeys[1],
            system_program: pubkeys[2],
        }
    }
}
impl<'info> From<CreateProrataConfigAccounts<'_, 'info>>
for [AccountInfo<'info>; CREATE_PRORATA_CONFIG_IX_ACCOUNTS_LEN] {
    fn from(accounts: CreateProrataConfigAccounts<'_, 'info>) -> Self {
        [
            accounts.config.clone(),
            accounts.admin.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CREATE_PRORATA_CONFIG_IX_ACCOUNTS_LEN]>
for CreateProrataConfigAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; CREATE_PRORATA_CONFIG_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            config: &arr[0],
            admin: &arr[1],
            system_program: &arr[2],
        }
    }
}
pub const CREATE_PRORATA_CONFIG_IX_DISCM: [u8; 8usize] = [
    38, 203, 72, 231, 103, 29, 195, 61,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CreateProrataConfigIxArgs {
    pub config_parameters: ProrataConfigParameters,
}
#[derive(Clone, Debug, PartialEq)]
pub struct CreateProrataConfigIxData(pub CreateProrataConfigIxArgs);
impl From<CreateProrataConfigIxArgs> for CreateProrataConfigIxData {
    fn from(args: CreateProrataConfigIxArgs) -> Self {
        Self(args)
    }
}
impl CreateProrataConfigIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CREATE_PRORATA_CONFIG_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let config_parameters = if reader.is_empty() {
            Default::default()
        } else {
            <ProrataConfigParameters>::deserialize(&mut reader)?
        };
        Ok(
            Self(CreateProrataConfigIxArgs {
                config_parameters,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CREATE_PRORATA_CONFIG_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.config_parameters, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn create_prorata_config_ix_with_program_id(
    program_id: Pubkey,
    keys: CreateProrataConfigKeys,
    args: CreateProrataConfigIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CREATE_PRORATA_CONFIG_IX_ACCOUNTS_LEN] = keys.into();
    let data: CreateProrataConfigIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn create_prorata_config_ix(
    keys: CreateProrataConfigKeys,
    args: CreateProrataConfigIxArgs,
) -> std::io::Result<Instruction> {
    create_prorata_config_ix_with_program_id(ALPHA_VAULT_PROGRAM_ID, keys, args)
}
pub fn create_prorata_config_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CreateProrataConfigAccounts<'_, '_>,
    args: CreateProrataConfigIxArgs,
) -> ProgramResult {
    let keys: CreateProrataConfigKeys = accounts.into();
    let ix = create_prorata_config_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn create_prorata_config_invoke(
    accounts: CreateProrataConfigAccounts<'_, '_>,
    args: CreateProrataConfigIxArgs,
) -> ProgramResult {
    create_prorata_config_invoke_with_program_id(ALPHA_VAULT_PROGRAM_ID, accounts, args)
}
pub fn create_prorata_config_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CreateProrataConfigAccounts<'_, '_>,
    args: CreateProrataConfigIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CreateProrataConfigKeys = accounts.into();
    let ix = create_prorata_config_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn create_prorata_config_invoke_signed(
    accounts: CreateProrataConfigAccounts<'_, '_>,
    args: CreateProrataConfigIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    create_prorata_config_invoke_signed_with_program_id(
        ALPHA_VAULT_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn create_prorata_config_verify_account_keys(
    accounts: CreateProrataConfigAccounts<'_, '_>,
    keys: CreateProrataConfigKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.config.key, keys.config),
        (*accounts.admin.key, keys.admin),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn create_prorata_config_verify_writable_privileges<'me, 'info>(
    accounts: CreateProrataConfigAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.config, accounts.admin] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn create_prorata_config_verify_signer_privileges<'me, 'info>(
    accounts: CreateProrataConfigAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn create_prorata_config_verify_account_privileges<'me, 'info>(
    accounts: CreateProrataConfigAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    create_prorata_config_verify_writable_privileges(accounts)?;
    create_prorata_config_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const DEPOSIT_IX_ACCOUNTS_LEN: usize = 10;
#[derive(Copy, Clone, Debug)]
pub struct DepositAccounts<'me, 'info> {
    pub vault: &'me AccountInfo<'info>,
    pub pool: &'me AccountInfo<'info>,
    pub escrow: &'me AccountInfo<'info>,
    pub source_token: &'me AccountInfo<'info>,
    pub token_vault: &'me AccountInfo<'info>,
    pub token_mint: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub owner: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct DepositKeys {
    pub vault: Pubkey,
    pub pool: Pubkey,
    pub escrow: Pubkey,
    pub source_token: Pubkey,
    pub token_vault: Pubkey,
    pub token_mint: Pubkey,
    pub token_program: Pubkey,
    pub owner: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<DepositAccounts<'_, '_>> for DepositKeys {
    fn from(accounts: DepositAccounts) -> Self {
        Self {
            vault: *accounts.vault.key,
            pool: *accounts.pool.key,
            escrow: *accounts.escrow.key,
            source_token: *accounts.source_token.key,
            token_vault: *accounts.token_vault.key,
            token_mint: *accounts.token_mint.key,
            token_program: *accounts.token_program.key,
            owner: *accounts.owner.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<DepositKeys> for [AccountMeta; DEPOSIT_IX_ACCOUNTS_LEN] {
    fn from(keys: DepositKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pool,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.escrow,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.source_token,
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
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.owner,
                is_signer: true,
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
            vault: pubkeys[0],
            pool: pubkeys[1],
            escrow: pubkeys[2],
            source_token: pubkeys[3],
            token_vault: pubkeys[4],
            token_mint: pubkeys[5],
            token_program: pubkeys[6],
            owner: pubkeys[7],
            event_authority: pubkeys[8],
            program: pubkeys[9],
        }
    }
}
impl<'info> From<DepositAccounts<'_, 'info>>
for [AccountInfo<'info>; DEPOSIT_IX_ACCOUNTS_LEN] {
    fn from(accounts: DepositAccounts<'_, 'info>) -> Self {
        [
            accounts.vault.clone(),
            accounts.pool.clone(),
            accounts.escrow.clone(),
            accounts.source_token.clone(),
            accounts.token_vault.clone(),
            accounts.token_mint.clone(),
            accounts.token_program.clone(),
            accounts.owner.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; DEPOSIT_IX_ACCOUNTS_LEN]>
for DepositAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; DEPOSIT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            vault: &arr[0],
            pool: &arr[1],
            escrow: &arr[2],
            source_token: &arr[3],
            token_vault: &arr[4],
            token_mint: &arr[5],
            token_program: &arr[6],
            owner: &arr[7],
            event_authority: &arr[8],
            program: &arr[9],
        }
    }
}
pub const DEPOSIT_IX_DISCM: [u8; 8usize] = [242, 35, 198, 137, 82, 225, 242, 182];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DepositIxArgs {
    pub max_amount: u64,
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
        let max_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(Self(DepositIxArgs { max_amount }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&DEPOSIT_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.max_amount, &mut writer)?;
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
    deposit_ix_with_program_id(ALPHA_VAULT_PROGRAM_ID, keys, args)
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
    deposit_invoke_with_program_id(ALPHA_VAULT_PROGRAM_ID, accounts, args)
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
    deposit_invoke_signed_with_program_id(ALPHA_VAULT_PROGRAM_ID, accounts, args, seeds)
}
pub fn deposit_verify_account_keys(
    accounts: DepositAccounts<'_, '_>,
    keys: DepositKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.vault.key, keys.vault),
        (*accounts.pool.key, keys.pool),
        (*accounts.escrow.key, keys.escrow),
        (*accounts.source_token.key, keys.source_token),
        (*accounts.token_vault.key, keys.token_vault),
        (*accounts.token_mint.key, keys.token_mint),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.owner.key, keys.owner),
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
        accounts.vault,
        accounts.escrow,
        accounts.source_token,
        accounts.token_vault,
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
    for should_be_signer in [accounts.owner] {
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
pub const FILL_DAMM_V2_IX_ACCOUNTS_LEN: usize = 19;
#[derive(Copy, Clone, Debug)]
pub struct FillDammV2Accounts<'me, 'info> {
    pub vault: &'me AccountInfo<'info>,
    pub token_vault: &'me AccountInfo<'info>,
    pub token_out_vault: &'me AccountInfo<'info>,
    pub amm_program: &'me AccountInfo<'info>,
    pub pool_authority: &'me AccountInfo<'info>,
    pub pool: &'me AccountInfo<'info>,
    pub token_a_vault: &'me AccountInfo<'info>,
    pub token_b_vault: &'me AccountInfo<'info>,
    pub token_a_mint: &'me AccountInfo<'info>,
    pub token_b_mint: &'me AccountInfo<'info>,
    pub token_a_program: &'me AccountInfo<'info>,
    pub token_b_program: &'me AccountInfo<'info>,
    pub damm_event_authority: &'me AccountInfo<'info>,
    pub crank_fee_whitelist: &'me AccountInfo<'info>,
    pub crank_fee_receiver: &'me AccountInfo<'info>,
    pub cranker: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FillDammV2Keys {
    pub vault: Pubkey,
    pub token_vault: Pubkey,
    pub token_out_vault: Pubkey,
    pub amm_program: Pubkey,
    pub pool_authority: Pubkey,
    pub pool: Pubkey,
    pub token_a_vault: Pubkey,
    pub token_b_vault: Pubkey,
    pub token_a_mint: Pubkey,
    pub token_b_mint: Pubkey,
    pub token_a_program: Pubkey,
    pub token_b_program: Pubkey,
    pub damm_event_authority: Pubkey,
    pub crank_fee_whitelist: Pubkey,
    pub crank_fee_receiver: Pubkey,
    pub cranker: Pubkey,
    pub system_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<FillDammV2Accounts<'_, '_>> for FillDammV2Keys {
    fn from(accounts: FillDammV2Accounts) -> Self {
        Self {
            vault: *accounts.vault.key,
            token_vault: *accounts.token_vault.key,
            token_out_vault: *accounts.token_out_vault.key,
            amm_program: *accounts.amm_program.key,
            pool_authority: *accounts.pool_authority.key,
            pool: *accounts.pool.key,
            token_a_vault: *accounts.token_a_vault.key,
            token_b_vault: *accounts.token_b_vault.key,
            token_a_mint: *accounts.token_a_mint.key,
            token_b_mint: *accounts.token_b_mint.key,
            token_a_program: *accounts.token_a_program.key,
            token_b_program: *accounts.token_b_program.key,
            damm_event_authority: *accounts.damm_event_authority.key,
            crank_fee_whitelist: *accounts.crank_fee_whitelist.key,
            crank_fee_receiver: *accounts.crank_fee_receiver.key,
            cranker: *accounts.cranker.key,
            system_program: *accounts.system_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<FillDammV2Keys> for [AccountMeta; FILL_DAMM_V2_IX_ACCOUNTS_LEN] {
    fn from(keys: FillDammV2Keys) -> Self {
        [
            AccountMeta {
                pubkey: keys.vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_out_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.amm_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool,
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
                pubkey: keys.damm_event_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.crank_fee_whitelist,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.crank_fee_receiver,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.cranker,
                is_signer: true,
                is_writable: true,
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
impl From<[Pubkey; FILL_DAMM_V2_IX_ACCOUNTS_LEN]> for FillDammV2Keys {
    fn from(pubkeys: [Pubkey; FILL_DAMM_V2_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            vault: pubkeys[0],
            token_vault: pubkeys[1],
            token_out_vault: pubkeys[2],
            amm_program: pubkeys[3],
            pool_authority: pubkeys[4],
            pool: pubkeys[5],
            token_a_vault: pubkeys[6],
            token_b_vault: pubkeys[7],
            token_a_mint: pubkeys[8],
            token_b_mint: pubkeys[9],
            token_a_program: pubkeys[10],
            token_b_program: pubkeys[11],
            damm_event_authority: pubkeys[12],
            crank_fee_whitelist: pubkeys[13],
            crank_fee_receiver: pubkeys[14],
            cranker: pubkeys[15],
            system_program: pubkeys[16],
            event_authority: pubkeys[17],
            program: pubkeys[18],
        }
    }
}
impl<'info> From<FillDammV2Accounts<'_, 'info>>
for [AccountInfo<'info>; FILL_DAMM_V2_IX_ACCOUNTS_LEN] {
    fn from(accounts: FillDammV2Accounts<'_, 'info>) -> Self {
        [
            accounts.vault.clone(),
            accounts.token_vault.clone(),
            accounts.token_out_vault.clone(),
            accounts.amm_program.clone(),
            accounts.pool_authority.clone(),
            accounts.pool.clone(),
            accounts.token_a_vault.clone(),
            accounts.token_b_vault.clone(),
            accounts.token_a_mint.clone(),
            accounts.token_b_mint.clone(),
            accounts.token_a_program.clone(),
            accounts.token_b_program.clone(),
            accounts.damm_event_authority.clone(),
            accounts.crank_fee_whitelist.clone(),
            accounts.crank_fee_receiver.clone(),
            accounts.cranker.clone(),
            accounts.system_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; FILL_DAMM_V2_IX_ACCOUNTS_LEN]>
for FillDammV2Accounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; FILL_DAMM_V2_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            vault: &arr[0],
            token_vault: &arr[1],
            token_out_vault: &arr[2],
            amm_program: &arr[3],
            pool_authority: &arr[4],
            pool: &arr[5],
            token_a_vault: &arr[6],
            token_b_vault: &arr[7],
            token_a_mint: &arr[8],
            token_b_mint: &arr[9],
            token_a_program: &arr[10],
            token_b_program: &arr[11],
            damm_event_authority: &arr[12],
            crank_fee_whitelist: &arr[13],
            crank_fee_receiver: &arr[14],
            cranker: &arr[15],
            system_program: &arr[16],
            event_authority: &arr[17],
            program: &arr[18],
        }
    }
}
pub const FILL_DAMM_V2_IX_DISCM: [u8; 8usize] = [221, 175, 108, 48, 19, 204, 125, 23];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct FillDammV2IxArgs {
    pub max_amount: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct FillDammV2IxData(pub FillDammV2IxArgs);
impl From<FillDammV2IxArgs> for FillDammV2IxData {
    fn from(args: FillDammV2IxArgs) -> Self {
        Self(args)
    }
}
impl FillDammV2IxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != FILL_DAMM_V2_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let max_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(Self(FillDammV2IxArgs { max_amount }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&FILL_DAMM_V2_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.max_amount, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn fill_damm_v2_ix_with_program_id(
    program_id: Pubkey,
    keys: FillDammV2Keys,
    args: FillDammV2IxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; FILL_DAMM_V2_IX_ACCOUNTS_LEN] = keys.into();
    let data: FillDammV2IxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn fill_damm_v2_ix(
    keys: FillDammV2Keys,
    args: FillDammV2IxArgs,
) -> std::io::Result<Instruction> {
    fill_damm_v2_ix_with_program_id(ALPHA_VAULT_PROGRAM_ID, keys, args)
}
pub fn fill_damm_v2_invoke_with_program_id(
    program_id: Pubkey,
    accounts: FillDammV2Accounts<'_, '_>,
    args: FillDammV2IxArgs,
) -> ProgramResult {
    let keys: FillDammV2Keys = accounts.into();
    let ix = fill_damm_v2_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn fill_damm_v2_invoke(
    accounts: FillDammV2Accounts<'_, '_>,
    args: FillDammV2IxArgs,
) -> ProgramResult {
    fill_damm_v2_invoke_with_program_id(ALPHA_VAULT_PROGRAM_ID, accounts, args)
}
pub fn fill_damm_v2_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: FillDammV2Accounts<'_, '_>,
    args: FillDammV2IxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: FillDammV2Keys = accounts.into();
    let ix = fill_damm_v2_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn fill_damm_v2_invoke_signed(
    accounts: FillDammV2Accounts<'_, '_>,
    args: FillDammV2IxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    fill_damm_v2_invoke_signed_with_program_id(
        ALPHA_VAULT_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn fill_damm_v2_verify_account_keys(
    accounts: FillDammV2Accounts<'_, '_>,
    keys: FillDammV2Keys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.vault.key, keys.vault),
        (*accounts.token_vault.key, keys.token_vault),
        (*accounts.token_out_vault.key, keys.token_out_vault),
        (*accounts.amm_program.key, keys.amm_program),
        (*accounts.pool_authority.key, keys.pool_authority),
        (*accounts.pool.key, keys.pool),
        (*accounts.token_a_vault.key, keys.token_a_vault),
        (*accounts.token_b_vault.key, keys.token_b_vault),
        (*accounts.token_a_mint.key, keys.token_a_mint),
        (*accounts.token_b_mint.key, keys.token_b_mint),
        (*accounts.token_a_program.key, keys.token_a_program),
        (*accounts.token_b_program.key, keys.token_b_program),
        (*accounts.damm_event_authority.key, keys.damm_event_authority),
        (*accounts.crank_fee_whitelist.key, keys.crank_fee_whitelist),
        (*accounts.crank_fee_receiver.key, keys.crank_fee_receiver),
        (*accounts.cranker.key, keys.cranker),
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
pub fn fill_damm_v2_verify_writable_privileges<'me, 'info>(
    accounts: FillDammV2Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.vault,
        accounts.token_vault,
        accounts.token_out_vault,
        accounts.pool,
        accounts.token_a_vault,
        accounts.token_b_vault,
        accounts.crank_fee_receiver,
        accounts.cranker,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn fill_damm_v2_verify_signer_privileges<'me, 'info>(
    accounts: FillDammV2Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.cranker] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn fill_damm_v2_verify_account_privileges<'me, 'info>(
    accounts: FillDammV2Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    fill_damm_v2_verify_writable_privileges(accounts)?;
    fill_damm_v2_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const FILL_DLMM_IX_ACCOUNTS_LEN: usize = 21;
#[derive(Copy, Clone, Debug)]
pub struct FillDlmmAccounts<'me, 'info> {
    pub vault: &'me AccountInfo<'info>,
    pub token_vault: &'me AccountInfo<'info>,
    pub token_out_vault: &'me AccountInfo<'info>,
    pub amm_program: &'me AccountInfo<'info>,
    pub pool: &'me AccountInfo<'info>,
    pub bin_array_bitmap_extension: &'me AccountInfo<'info>,
    pub reserve_x: &'me AccountInfo<'info>,
    pub reserve_y: &'me AccountInfo<'info>,
    pub token_x_mint: &'me AccountInfo<'info>,
    pub token_y_mint: &'me AccountInfo<'info>,
    pub oracle: &'me AccountInfo<'info>,
    pub token_x_program: &'me AccountInfo<'info>,
    pub token_y_program: &'me AccountInfo<'info>,
    pub dlmm_event_authority: &'me AccountInfo<'info>,
    pub crank_fee_whitelist: &'me AccountInfo<'info>,
    pub crank_fee_receiver: &'me AccountInfo<'info>,
    pub cranker: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub memo_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FillDlmmKeys {
    pub vault: Pubkey,
    pub token_vault: Pubkey,
    pub token_out_vault: Pubkey,
    pub amm_program: Pubkey,
    pub pool: Pubkey,
    pub bin_array_bitmap_extension: Pubkey,
    pub reserve_x: Pubkey,
    pub reserve_y: Pubkey,
    pub token_x_mint: Pubkey,
    pub token_y_mint: Pubkey,
    pub oracle: Pubkey,
    pub token_x_program: Pubkey,
    pub token_y_program: Pubkey,
    pub dlmm_event_authority: Pubkey,
    pub crank_fee_whitelist: Pubkey,
    pub crank_fee_receiver: Pubkey,
    pub cranker: Pubkey,
    pub system_program: Pubkey,
    pub memo_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<FillDlmmAccounts<'_, '_>> for FillDlmmKeys {
    fn from(accounts: FillDlmmAccounts) -> Self {
        Self {
            vault: *accounts.vault.key,
            token_vault: *accounts.token_vault.key,
            token_out_vault: *accounts.token_out_vault.key,
            amm_program: *accounts.amm_program.key,
            pool: *accounts.pool.key,
            bin_array_bitmap_extension: *accounts.bin_array_bitmap_extension.key,
            reserve_x: *accounts.reserve_x.key,
            reserve_y: *accounts.reserve_y.key,
            token_x_mint: *accounts.token_x_mint.key,
            token_y_mint: *accounts.token_y_mint.key,
            oracle: *accounts.oracle.key,
            token_x_program: *accounts.token_x_program.key,
            token_y_program: *accounts.token_y_program.key,
            dlmm_event_authority: *accounts.dlmm_event_authority.key,
            crank_fee_whitelist: *accounts.crank_fee_whitelist.key,
            crank_fee_receiver: *accounts.crank_fee_receiver.key,
            cranker: *accounts.cranker.key,
            system_program: *accounts.system_program.key,
            memo_program: *accounts.memo_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<FillDlmmKeys> for [AccountMeta; FILL_DLMM_IX_ACCOUNTS_LEN] {
    fn from(keys: FillDlmmKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_out_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.amm_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.bin_array_bitmap_extension,
                is_signer: false,
                is_writable: false,
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
                pubkey: keys.oracle,
                is_signer: false,
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
                pubkey: keys.dlmm_event_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.crank_fee_whitelist,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.crank_fee_receiver,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.cranker,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.system_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.memo_program,
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
impl From<[Pubkey; FILL_DLMM_IX_ACCOUNTS_LEN]> for FillDlmmKeys {
    fn from(pubkeys: [Pubkey; FILL_DLMM_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            vault: pubkeys[0],
            token_vault: pubkeys[1],
            token_out_vault: pubkeys[2],
            amm_program: pubkeys[3],
            pool: pubkeys[4],
            bin_array_bitmap_extension: pubkeys[5],
            reserve_x: pubkeys[6],
            reserve_y: pubkeys[7],
            token_x_mint: pubkeys[8],
            token_y_mint: pubkeys[9],
            oracle: pubkeys[10],
            token_x_program: pubkeys[11],
            token_y_program: pubkeys[12],
            dlmm_event_authority: pubkeys[13],
            crank_fee_whitelist: pubkeys[14],
            crank_fee_receiver: pubkeys[15],
            cranker: pubkeys[16],
            system_program: pubkeys[17],
            memo_program: pubkeys[18],
            event_authority: pubkeys[19],
            program: pubkeys[20],
        }
    }
}
impl<'info> From<FillDlmmAccounts<'_, 'info>>
for [AccountInfo<'info>; FILL_DLMM_IX_ACCOUNTS_LEN] {
    fn from(accounts: FillDlmmAccounts<'_, 'info>) -> Self {
        [
            accounts.vault.clone(),
            accounts.token_vault.clone(),
            accounts.token_out_vault.clone(),
            accounts.amm_program.clone(),
            accounts.pool.clone(),
            accounts.bin_array_bitmap_extension.clone(),
            accounts.reserve_x.clone(),
            accounts.reserve_y.clone(),
            accounts.token_x_mint.clone(),
            accounts.token_y_mint.clone(),
            accounts.oracle.clone(),
            accounts.token_x_program.clone(),
            accounts.token_y_program.clone(),
            accounts.dlmm_event_authority.clone(),
            accounts.crank_fee_whitelist.clone(),
            accounts.crank_fee_receiver.clone(),
            accounts.cranker.clone(),
            accounts.system_program.clone(),
            accounts.memo_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; FILL_DLMM_IX_ACCOUNTS_LEN]>
for FillDlmmAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; FILL_DLMM_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            vault: &arr[0],
            token_vault: &arr[1],
            token_out_vault: &arr[2],
            amm_program: &arr[3],
            pool: &arr[4],
            bin_array_bitmap_extension: &arr[5],
            reserve_x: &arr[6],
            reserve_y: &arr[7],
            token_x_mint: &arr[8],
            token_y_mint: &arr[9],
            oracle: &arr[10],
            token_x_program: &arr[11],
            token_y_program: &arr[12],
            dlmm_event_authority: &arr[13],
            crank_fee_whitelist: &arr[14],
            crank_fee_receiver: &arr[15],
            cranker: &arr[16],
            system_program: &arr[17],
            memo_program: &arr[18],
            event_authority: &arr[19],
            program: &arr[20],
        }
    }
}
pub const FILL_DLMM_IX_DISCM: [u8; 8usize] = [1, 108, 141, 11, 4, 126, 251, 222];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct FillDlmmIxArgs {
    pub max_amount: u64,
    pub remaining_accounts_info: RemainingAccountsInfo,
}
#[derive(Clone, Debug, PartialEq)]
pub struct FillDlmmIxData(pub FillDlmmIxArgs);
impl From<FillDlmmIxArgs> for FillDlmmIxData {
    fn from(args: FillDlmmIxArgs) -> Self {
        Self(args)
    }
}
impl FillDlmmIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != FILL_DLMM_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let max_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let remaining_accounts_info = if reader.is_empty() {
            Default::default()
        } else {
            <RemainingAccountsInfo>::deserialize(&mut reader)?
        };
        Ok(
            Self(FillDlmmIxArgs {
                max_amount,
                remaining_accounts_info,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&FILL_DLMM_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.max_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.remaining_accounts_info, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn fill_dlmm_ix_with_program_id(
    program_id: Pubkey,
    keys: FillDlmmKeys,
    args: FillDlmmIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; FILL_DLMM_IX_ACCOUNTS_LEN] = keys.into();
    let data: FillDlmmIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn fill_dlmm_ix(
    keys: FillDlmmKeys,
    args: FillDlmmIxArgs,
) -> std::io::Result<Instruction> {
    fill_dlmm_ix_with_program_id(ALPHA_VAULT_PROGRAM_ID, keys, args)
}
pub fn fill_dlmm_invoke_with_program_id(
    program_id: Pubkey,
    accounts: FillDlmmAccounts<'_, '_>,
    args: FillDlmmIxArgs,
) -> ProgramResult {
    let keys: FillDlmmKeys = accounts.into();
    let ix = fill_dlmm_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn fill_dlmm_invoke(
    accounts: FillDlmmAccounts<'_, '_>,
    args: FillDlmmIxArgs,
) -> ProgramResult {
    fill_dlmm_invoke_with_program_id(ALPHA_VAULT_PROGRAM_ID, accounts, args)
}
pub fn fill_dlmm_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: FillDlmmAccounts<'_, '_>,
    args: FillDlmmIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: FillDlmmKeys = accounts.into();
    let ix = fill_dlmm_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn fill_dlmm_invoke_signed(
    accounts: FillDlmmAccounts<'_, '_>,
    args: FillDlmmIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    fill_dlmm_invoke_signed_with_program_id(
        ALPHA_VAULT_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn fill_dlmm_verify_account_keys(
    accounts: FillDlmmAccounts<'_, '_>,
    keys: FillDlmmKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.vault.key, keys.vault),
        (*accounts.token_vault.key, keys.token_vault),
        (*accounts.token_out_vault.key, keys.token_out_vault),
        (*accounts.amm_program.key, keys.amm_program),
        (*accounts.pool.key, keys.pool),
        (*accounts.bin_array_bitmap_extension.key, keys.bin_array_bitmap_extension),
        (*accounts.reserve_x.key, keys.reserve_x),
        (*accounts.reserve_y.key, keys.reserve_y),
        (*accounts.token_x_mint.key, keys.token_x_mint),
        (*accounts.token_y_mint.key, keys.token_y_mint),
        (*accounts.oracle.key, keys.oracle),
        (*accounts.token_x_program.key, keys.token_x_program),
        (*accounts.token_y_program.key, keys.token_y_program),
        (*accounts.dlmm_event_authority.key, keys.dlmm_event_authority),
        (*accounts.crank_fee_whitelist.key, keys.crank_fee_whitelist),
        (*accounts.crank_fee_receiver.key, keys.crank_fee_receiver),
        (*accounts.cranker.key, keys.cranker),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.memo_program.key, keys.memo_program),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn fill_dlmm_verify_writable_privileges<'me, 'info>(
    accounts: FillDlmmAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.vault,
        accounts.token_vault,
        accounts.token_out_vault,
        accounts.pool,
        accounts.reserve_x,
        accounts.reserve_y,
        accounts.oracle,
        accounts.crank_fee_receiver,
        accounts.cranker,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn fill_dlmm_verify_signer_privileges<'me, 'info>(
    accounts: FillDlmmAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.cranker] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn fill_dlmm_verify_account_privileges<'me, 'info>(
    accounts: FillDlmmAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    fill_dlmm_verify_writable_privileges(accounts)?;
    fill_dlmm_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const FILL_DYNAMIC_AMM_IX_ACCOUNTS_LEN: usize = 22;
#[derive(Copy, Clone, Debug)]
pub struct FillDynamicAmmAccounts<'me, 'info> {
    pub vault: &'me AccountInfo<'info>,
    pub token_vault: &'me AccountInfo<'info>,
    pub token_out_vault: &'me AccountInfo<'info>,
    pub amm_program: &'me AccountInfo<'info>,
    pub pool: &'me AccountInfo<'info>,
    pub a_vault: &'me AccountInfo<'info>,
    pub b_vault: &'me AccountInfo<'info>,
    pub a_token_vault: &'me AccountInfo<'info>,
    pub b_token_vault: &'me AccountInfo<'info>,
    pub a_vault_lp_mint: &'me AccountInfo<'info>,
    pub b_vault_lp_mint: &'me AccountInfo<'info>,
    pub a_vault_lp: &'me AccountInfo<'info>,
    pub b_vault_lp: &'me AccountInfo<'info>,
    pub admin_token_fee: &'me AccountInfo<'info>,
    pub vault_program: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub crank_fee_whitelist: &'me AccountInfo<'info>,
    pub crank_fee_receiver: &'me AccountInfo<'info>,
    pub cranker: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FillDynamicAmmKeys {
    pub vault: Pubkey,
    pub token_vault: Pubkey,
    pub token_out_vault: Pubkey,
    pub amm_program: Pubkey,
    pub pool: Pubkey,
    pub a_vault: Pubkey,
    pub b_vault: Pubkey,
    pub a_token_vault: Pubkey,
    pub b_token_vault: Pubkey,
    pub a_vault_lp_mint: Pubkey,
    pub b_vault_lp_mint: Pubkey,
    pub a_vault_lp: Pubkey,
    pub b_vault_lp: Pubkey,
    pub admin_token_fee: Pubkey,
    pub vault_program: Pubkey,
    pub token_program: Pubkey,
    pub crank_fee_whitelist: Pubkey,
    pub crank_fee_receiver: Pubkey,
    pub cranker: Pubkey,
    pub system_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<FillDynamicAmmAccounts<'_, '_>> for FillDynamicAmmKeys {
    fn from(accounts: FillDynamicAmmAccounts) -> Self {
        Self {
            vault: *accounts.vault.key,
            token_vault: *accounts.token_vault.key,
            token_out_vault: *accounts.token_out_vault.key,
            amm_program: *accounts.amm_program.key,
            pool: *accounts.pool.key,
            a_vault: *accounts.a_vault.key,
            b_vault: *accounts.b_vault.key,
            a_token_vault: *accounts.a_token_vault.key,
            b_token_vault: *accounts.b_token_vault.key,
            a_vault_lp_mint: *accounts.a_vault_lp_mint.key,
            b_vault_lp_mint: *accounts.b_vault_lp_mint.key,
            a_vault_lp: *accounts.a_vault_lp.key,
            b_vault_lp: *accounts.b_vault_lp.key,
            admin_token_fee: *accounts.admin_token_fee.key,
            vault_program: *accounts.vault_program.key,
            token_program: *accounts.token_program.key,
            crank_fee_whitelist: *accounts.crank_fee_whitelist.key,
            crank_fee_receiver: *accounts.crank_fee_receiver.key,
            cranker: *accounts.cranker.key,
            system_program: *accounts.system_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<FillDynamicAmmKeys> for [AccountMeta; FILL_DYNAMIC_AMM_IX_ACCOUNTS_LEN] {
    fn from(keys: FillDynamicAmmKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_out_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.amm_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.a_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.b_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.a_token_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.b_token_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.a_vault_lp_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.b_vault_lp_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.a_vault_lp,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.b_vault_lp,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.admin_token_fee,
                is_signer: false,
                is_writable: true,
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
                pubkey: keys.crank_fee_whitelist,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.crank_fee_receiver,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.cranker,
                is_signer: true,
                is_writable: true,
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
impl From<[Pubkey; FILL_DYNAMIC_AMM_IX_ACCOUNTS_LEN]> for FillDynamicAmmKeys {
    fn from(pubkeys: [Pubkey; FILL_DYNAMIC_AMM_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            vault: pubkeys[0],
            token_vault: pubkeys[1],
            token_out_vault: pubkeys[2],
            amm_program: pubkeys[3],
            pool: pubkeys[4],
            a_vault: pubkeys[5],
            b_vault: pubkeys[6],
            a_token_vault: pubkeys[7],
            b_token_vault: pubkeys[8],
            a_vault_lp_mint: pubkeys[9],
            b_vault_lp_mint: pubkeys[10],
            a_vault_lp: pubkeys[11],
            b_vault_lp: pubkeys[12],
            admin_token_fee: pubkeys[13],
            vault_program: pubkeys[14],
            token_program: pubkeys[15],
            crank_fee_whitelist: pubkeys[16],
            crank_fee_receiver: pubkeys[17],
            cranker: pubkeys[18],
            system_program: pubkeys[19],
            event_authority: pubkeys[20],
            program: pubkeys[21],
        }
    }
}
impl<'info> From<FillDynamicAmmAccounts<'_, 'info>>
for [AccountInfo<'info>; FILL_DYNAMIC_AMM_IX_ACCOUNTS_LEN] {
    fn from(accounts: FillDynamicAmmAccounts<'_, 'info>) -> Self {
        [
            accounts.vault.clone(),
            accounts.token_vault.clone(),
            accounts.token_out_vault.clone(),
            accounts.amm_program.clone(),
            accounts.pool.clone(),
            accounts.a_vault.clone(),
            accounts.b_vault.clone(),
            accounts.a_token_vault.clone(),
            accounts.b_token_vault.clone(),
            accounts.a_vault_lp_mint.clone(),
            accounts.b_vault_lp_mint.clone(),
            accounts.a_vault_lp.clone(),
            accounts.b_vault_lp.clone(),
            accounts.admin_token_fee.clone(),
            accounts.vault_program.clone(),
            accounts.token_program.clone(),
            accounts.crank_fee_whitelist.clone(),
            accounts.crank_fee_receiver.clone(),
            accounts.cranker.clone(),
            accounts.system_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; FILL_DYNAMIC_AMM_IX_ACCOUNTS_LEN]>
for FillDynamicAmmAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; FILL_DYNAMIC_AMM_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            vault: &arr[0],
            token_vault: &arr[1],
            token_out_vault: &arr[2],
            amm_program: &arr[3],
            pool: &arr[4],
            a_vault: &arr[5],
            b_vault: &arr[6],
            a_token_vault: &arr[7],
            b_token_vault: &arr[8],
            a_vault_lp_mint: &arr[9],
            b_vault_lp_mint: &arr[10],
            a_vault_lp: &arr[11],
            b_vault_lp: &arr[12],
            admin_token_fee: &arr[13],
            vault_program: &arr[14],
            token_program: &arr[15],
            crank_fee_whitelist: &arr[16],
            crank_fee_receiver: &arr[17],
            cranker: &arr[18],
            system_program: &arr[19],
            event_authority: &arr[20],
            program: &arr[21],
        }
    }
}
pub const FILL_DYNAMIC_AMM_IX_DISCM: [u8; 8usize] = [224, 226, 223, 80, 36, 50, 70, 231];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct FillDynamicAmmIxArgs {
    pub max_amount: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct FillDynamicAmmIxData(pub FillDynamicAmmIxArgs);
impl From<FillDynamicAmmIxArgs> for FillDynamicAmmIxData {
    fn from(args: FillDynamicAmmIxArgs) -> Self {
        Self(args)
    }
}
impl FillDynamicAmmIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != FILL_DYNAMIC_AMM_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let max_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(Self(FillDynamicAmmIxArgs { max_amount }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&FILL_DYNAMIC_AMM_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.max_amount, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn fill_dynamic_amm_ix_with_program_id(
    program_id: Pubkey,
    keys: FillDynamicAmmKeys,
    args: FillDynamicAmmIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; FILL_DYNAMIC_AMM_IX_ACCOUNTS_LEN] = keys.into();
    let data: FillDynamicAmmIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn fill_dynamic_amm_ix(
    keys: FillDynamicAmmKeys,
    args: FillDynamicAmmIxArgs,
) -> std::io::Result<Instruction> {
    fill_dynamic_amm_ix_with_program_id(ALPHA_VAULT_PROGRAM_ID, keys, args)
}
pub fn fill_dynamic_amm_invoke_with_program_id(
    program_id: Pubkey,
    accounts: FillDynamicAmmAccounts<'_, '_>,
    args: FillDynamicAmmIxArgs,
) -> ProgramResult {
    let keys: FillDynamicAmmKeys = accounts.into();
    let ix = fill_dynamic_amm_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn fill_dynamic_amm_invoke(
    accounts: FillDynamicAmmAccounts<'_, '_>,
    args: FillDynamicAmmIxArgs,
) -> ProgramResult {
    fill_dynamic_amm_invoke_with_program_id(ALPHA_VAULT_PROGRAM_ID, accounts, args)
}
pub fn fill_dynamic_amm_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: FillDynamicAmmAccounts<'_, '_>,
    args: FillDynamicAmmIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: FillDynamicAmmKeys = accounts.into();
    let ix = fill_dynamic_amm_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn fill_dynamic_amm_invoke_signed(
    accounts: FillDynamicAmmAccounts<'_, '_>,
    args: FillDynamicAmmIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    fill_dynamic_amm_invoke_signed_with_program_id(
        ALPHA_VAULT_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn fill_dynamic_amm_verify_account_keys(
    accounts: FillDynamicAmmAccounts<'_, '_>,
    keys: FillDynamicAmmKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.vault.key, keys.vault),
        (*accounts.token_vault.key, keys.token_vault),
        (*accounts.token_out_vault.key, keys.token_out_vault),
        (*accounts.amm_program.key, keys.amm_program),
        (*accounts.pool.key, keys.pool),
        (*accounts.a_vault.key, keys.a_vault),
        (*accounts.b_vault.key, keys.b_vault),
        (*accounts.a_token_vault.key, keys.a_token_vault),
        (*accounts.b_token_vault.key, keys.b_token_vault),
        (*accounts.a_vault_lp_mint.key, keys.a_vault_lp_mint),
        (*accounts.b_vault_lp_mint.key, keys.b_vault_lp_mint),
        (*accounts.a_vault_lp.key, keys.a_vault_lp),
        (*accounts.b_vault_lp.key, keys.b_vault_lp),
        (*accounts.admin_token_fee.key, keys.admin_token_fee),
        (*accounts.vault_program.key, keys.vault_program),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.crank_fee_whitelist.key, keys.crank_fee_whitelist),
        (*accounts.crank_fee_receiver.key, keys.crank_fee_receiver),
        (*accounts.cranker.key, keys.cranker),
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
pub fn fill_dynamic_amm_verify_writable_privileges<'me, 'info>(
    accounts: FillDynamicAmmAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.vault,
        accounts.token_vault,
        accounts.token_out_vault,
        accounts.pool,
        accounts.a_vault,
        accounts.b_vault,
        accounts.a_token_vault,
        accounts.b_token_vault,
        accounts.a_vault_lp_mint,
        accounts.b_vault_lp_mint,
        accounts.a_vault_lp,
        accounts.b_vault_lp,
        accounts.admin_token_fee,
        accounts.crank_fee_receiver,
        accounts.cranker,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn fill_dynamic_amm_verify_signer_privileges<'me, 'info>(
    accounts: FillDynamicAmmAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.cranker] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn fill_dynamic_amm_verify_account_privileges<'me, 'info>(
    accounts: FillDynamicAmmAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    fill_dynamic_amm_verify_writable_privileges(accounts)?;
    fill_dynamic_amm_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const INITIALIZE_FCFS_VAULT_IX_ACCOUNTS_LEN: usize = 7;
#[derive(Copy, Clone, Debug)]
pub struct InitializeFcfsVaultAccounts<'me, 'info> {
    pub vault: &'me AccountInfo<'info>,
    pub pool: &'me AccountInfo<'info>,
    pub funder: &'me AccountInfo<'info>,
    pub base: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct InitializeFcfsVaultKeys {
    pub vault: Pubkey,
    pub pool: Pubkey,
    pub funder: Pubkey,
    pub base: Pubkey,
    pub system_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<InitializeFcfsVaultAccounts<'_, '_>> for InitializeFcfsVaultKeys {
    fn from(accounts: InitializeFcfsVaultAccounts) -> Self {
        Self {
            vault: *accounts.vault.key,
            pool: *accounts.pool.key,
            funder: *accounts.funder.key,
            base: *accounts.base.key,
            system_program: *accounts.system_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<InitializeFcfsVaultKeys>
for [AccountMeta; INITIALIZE_FCFS_VAULT_IX_ACCOUNTS_LEN] {
    fn from(keys: InitializeFcfsVaultKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pool,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.funder,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.base,
                is_signer: true,
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
impl From<[Pubkey; INITIALIZE_FCFS_VAULT_IX_ACCOUNTS_LEN]> for InitializeFcfsVaultKeys {
    fn from(pubkeys: [Pubkey; INITIALIZE_FCFS_VAULT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            vault: pubkeys[0],
            pool: pubkeys[1],
            funder: pubkeys[2],
            base: pubkeys[3],
            system_program: pubkeys[4],
            event_authority: pubkeys[5],
            program: pubkeys[6],
        }
    }
}
impl<'info> From<InitializeFcfsVaultAccounts<'_, 'info>>
for [AccountInfo<'info>; INITIALIZE_FCFS_VAULT_IX_ACCOUNTS_LEN] {
    fn from(accounts: InitializeFcfsVaultAccounts<'_, 'info>) -> Self {
        [
            accounts.vault.clone(),
            accounts.pool.clone(),
            accounts.funder.clone(),
            accounts.base.clone(),
            accounts.system_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; INITIALIZE_FCFS_VAULT_IX_ACCOUNTS_LEN]>
for InitializeFcfsVaultAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; INITIALIZE_FCFS_VAULT_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            vault: &arr[0],
            pool: &arr[1],
            funder: &arr[2],
            base: &arr[3],
            system_program: &arr[4],
            event_authority: &arr[5],
            program: &arr[6],
        }
    }
}
pub const INITIALIZE_FCFS_VAULT_IX_DISCM: [u8; 8usize] = [
    163, 205, 69, 145, 235, 71, 47, 21,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct InitializeFcfsVaultIxArgs {
    pub params: InitializeFcfsVaultParams,
}
#[derive(Clone, Debug, PartialEq)]
pub struct InitializeFcfsVaultIxData(pub InitializeFcfsVaultIxArgs);
impl From<InitializeFcfsVaultIxArgs> for InitializeFcfsVaultIxData {
    fn from(args: InitializeFcfsVaultIxArgs) -> Self {
        Self(args)
    }
}
impl InitializeFcfsVaultIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INITIALIZE_FCFS_VAULT_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let params = if reader.is_empty() {
            Default::default()
        } else {
            <InitializeFcfsVaultParams>::deserialize(&mut reader)?
        };
        Ok(
            Self(InitializeFcfsVaultIxArgs {
                params,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INITIALIZE_FCFS_VAULT_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.params, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn initialize_fcfs_vault_ix_with_program_id(
    program_id: Pubkey,
    keys: InitializeFcfsVaultKeys,
    args: InitializeFcfsVaultIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; INITIALIZE_FCFS_VAULT_IX_ACCOUNTS_LEN] = keys.into();
    let data: InitializeFcfsVaultIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn initialize_fcfs_vault_ix(
    keys: InitializeFcfsVaultKeys,
    args: InitializeFcfsVaultIxArgs,
) -> std::io::Result<Instruction> {
    initialize_fcfs_vault_ix_with_program_id(ALPHA_VAULT_PROGRAM_ID, keys, args)
}
pub fn initialize_fcfs_vault_invoke_with_program_id(
    program_id: Pubkey,
    accounts: InitializeFcfsVaultAccounts<'_, '_>,
    args: InitializeFcfsVaultIxArgs,
) -> ProgramResult {
    let keys: InitializeFcfsVaultKeys = accounts.into();
    let ix = initialize_fcfs_vault_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn initialize_fcfs_vault_invoke(
    accounts: InitializeFcfsVaultAccounts<'_, '_>,
    args: InitializeFcfsVaultIxArgs,
) -> ProgramResult {
    initialize_fcfs_vault_invoke_with_program_id(ALPHA_VAULT_PROGRAM_ID, accounts, args)
}
pub fn initialize_fcfs_vault_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: InitializeFcfsVaultAccounts<'_, '_>,
    args: InitializeFcfsVaultIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: InitializeFcfsVaultKeys = accounts.into();
    let ix = initialize_fcfs_vault_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn initialize_fcfs_vault_invoke_signed(
    accounts: InitializeFcfsVaultAccounts<'_, '_>,
    args: InitializeFcfsVaultIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    initialize_fcfs_vault_invoke_signed_with_program_id(
        ALPHA_VAULT_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn initialize_fcfs_vault_verify_account_keys(
    accounts: InitializeFcfsVaultAccounts<'_, '_>,
    keys: InitializeFcfsVaultKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.vault.key, keys.vault),
        (*accounts.pool.key, keys.pool),
        (*accounts.funder.key, keys.funder),
        (*accounts.base.key, keys.base),
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
pub fn initialize_fcfs_vault_verify_writable_privileges<'me, 'info>(
    accounts: InitializeFcfsVaultAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.vault, accounts.funder] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn initialize_fcfs_vault_verify_signer_privileges<'me, 'info>(
    accounts: InitializeFcfsVaultAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.funder, accounts.base] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn initialize_fcfs_vault_verify_account_privileges<'me, 'info>(
    accounts: InitializeFcfsVaultAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    initialize_fcfs_vault_verify_writable_privileges(accounts)?;
    initialize_fcfs_vault_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const INITIALIZE_PRORATA_VAULT_IX_ACCOUNTS_LEN: usize = 7;
#[derive(Copy, Clone, Debug)]
pub struct InitializeProrataVaultAccounts<'me, 'info> {
    pub vault: &'me AccountInfo<'info>,
    pub pool: &'me AccountInfo<'info>,
    pub funder: &'me AccountInfo<'info>,
    pub base: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct InitializeProrataVaultKeys {
    pub vault: Pubkey,
    pub pool: Pubkey,
    pub funder: Pubkey,
    pub base: Pubkey,
    pub system_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<InitializeProrataVaultAccounts<'_, '_>> for InitializeProrataVaultKeys {
    fn from(accounts: InitializeProrataVaultAccounts) -> Self {
        Self {
            vault: *accounts.vault.key,
            pool: *accounts.pool.key,
            funder: *accounts.funder.key,
            base: *accounts.base.key,
            system_program: *accounts.system_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<InitializeProrataVaultKeys>
for [AccountMeta; INITIALIZE_PRORATA_VAULT_IX_ACCOUNTS_LEN] {
    fn from(keys: InitializeProrataVaultKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pool,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.funder,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.base,
                is_signer: true,
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
impl From<[Pubkey; INITIALIZE_PRORATA_VAULT_IX_ACCOUNTS_LEN]>
for InitializeProrataVaultKeys {
    fn from(pubkeys: [Pubkey; INITIALIZE_PRORATA_VAULT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            vault: pubkeys[0],
            pool: pubkeys[1],
            funder: pubkeys[2],
            base: pubkeys[3],
            system_program: pubkeys[4],
            event_authority: pubkeys[5],
            program: pubkeys[6],
        }
    }
}
impl<'info> From<InitializeProrataVaultAccounts<'_, 'info>>
for [AccountInfo<'info>; INITIALIZE_PRORATA_VAULT_IX_ACCOUNTS_LEN] {
    fn from(accounts: InitializeProrataVaultAccounts<'_, 'info>) -> Self {
        [
            accounts.vault.clone(),
            accounts.pool.clone(),
            accounts.funder.clone(),
            accounts.base.clone(),
            accounts.system_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; INITIALIZE_PRORATA_VAULT_IX_ACCOUNTS_LEN]>
for InitializeProrataVaultAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; INITIALIZE_PRORATA_VAULT_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            vault: &arr[0],
            pool: &arr[1],
            funder: &arr[2],
            base: &arr[3],
            system_program: &arr[4],
            event_authority: &arr[5],
            program: &arr[6],
        }
    }
}
pub const INITIALIZE_PRORATA_VAULT_IX_DISCM: [u8; 8usize] = [
    178, 180, 176, 247, 128, 186, 43, 9,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct InitializeProrataVaultIxArgs {
    pub params: InitializeProrataVaultParams,
}
#[derive(Clone, Debug, PartialEq)]
pub struct InitializeProrataVaultIxData(pub InitializeProrataVaultIxArgs);
impl From<InitializeProrataVaultIxArgs> for InitializeProrataVaultIxData {
    fn from(args: InitializeProrataVaultIxArgs) -> Self {
        Self(args)
    }
}
impl InitializeProrataVaultIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INITIALIZE_PRORATA_VAULT_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let params = if reader.is_empty() {
            Default::default()
        } else {
            <InitializeProrataVaultParams>::deserialize(&mut reader)?
        };
        Ok(
            Self(InitializeProrataVaultIxArgs {
                params,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INITIALIZE_PRORATA_VAULT_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.params, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn initialize_prorata_vault_ix_with_program_id(
    program_id: Pubkey,
    keys: InitializeProrataVaultKeys,
    args: InitializeProrataVaultIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; INITIALIZE_PRORATA_VAULT_IX_ACCOUNTS_LEN] = keys.into();
    let data: InitializeProrataVaultIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn initialize_prorata_vault_ix(
    keys: InitializeProrataVaultKeys,
    args: InitializeProrataVaultIxArgs,
) -> std::io::Result<Instruction> {
    initialize_prorata_vault_ix_with_program_id(ALPHA_VAULT_PROGRAM_ID, keys, args)
}
pub fn initialize_prorata_vault_invoke_with_program_id(
    program_id: Pubkey,
    accounts: InitializeProrataVaultAccounts<'_, '_>,
    args: InitializeProrataVaultIxArgs,
) -> ProgramResult {
    let keys: InitializeProrataVaultKeys = accounts.into();
    let ix = initialize_prorata_vault_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn initialize_prorata_vault_invoke(
    accounts: InitializeProrataVaultAccounts<'_, '_>,
    args: InitializeProrataVaultIxArgs,
) -> ProgramResult {
    initialize_prorata_vault_invoke_with_program_id(
        ALPHA_VAULT_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn initialize_prorata_vault_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: InitializeProrataVaultAccounts<'_, '_>,
    args: InitializeProrataVaultIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: InitializeProrataVaultKeys = accounts.into();
    let ix = initialize_prorata_vault_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn initialize_prorata_vault_invoke_signed(
    accounts: InitializeProrataVaultAccounts<'_, '_>,
    args: InitializeProrataVaultIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    initialize_prorata_vault_invoke_signed_with_program_id(
        ALPHA_VAULT_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn initialize_prorata_vault_verify_account_keys(
    accounts: InitializeProrataVaultAccounts<'_, '_>,
    keys: InitializeProrataVaultKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.vault.key, keys.vault),
        (*accounts.pool.key, keys.pool),
        (*accounts.funder.key, keys.funder),
        (*accounts.base.key, keys.base),
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
pub fn initialize_prorata_vault_verify_writable_privileges<'me, 'info>(
    accounts: InitializeProrataVaultAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.vault, accounts.funder] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn initialize_prorata_vault_verify_signer_privileges<'me, 'info>(
    accounts: InitializeProrataVaultAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.funder, accounts.base] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn initialize_prorata_vault_verify_account_privileges<'me, 'info>(
    accounts: InitializeProrataVaultAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    initialize_prorata_vault_verify_writable_privileges(accounts)?;
    initialize_prorata_vault_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const INITIALIZE_VAULT_WITH_FCFS_CONFIG_IX_ACCOUNTS_LEN: usize = 8;
#[derive(Copy, Clone, Debug)]
pub struct InitializeVaultWithFcfsConfigAccounts<'me, 'info> {
    pub vault: &'me AccountInfo<'info>,
    pub pool: &'me AccountInfo<'info>,
    pub quote_mint: &'me AccountInfo<'info>,
    pub funder: &'me AccountInfo<'info>,
    pub config: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct InitializeVaultWithFcfsConfigKeys {
    pub vault: Pubkey,
    pub pool: Pubkey,
    pub quote_mint: Pubkey,
    pub funder: Pubkey,
    pub config: Pubkey,
    pub system_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<InitializeVaultWithFcfsConfigAccounts<'_, '_>>
for InitializeVaultWithFcfsConfigKeys {
    fn from(accounts: InitializeVaultWithFcfsConfigAccounts) -> Self {
        Self {
            vault: *accounts.vault.key,
            pool: *accounts.pool.key,
            quote_mint: *accounts.quote_mint.key,
            funder: *accounts.funder.key,
            config: *accounts.config.key,
            system_program: *accounts.system_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<InitializeVaultWithFcfsConfigKeys>
for [AccountMeta; INITIALIZE_VAULT_WITH_FCFS_CONFIG_IX_ACCOUNTS_LEN] {
    fn from(keys: InitializeVaultWithFcfsConfigKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pool,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.quote_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.funder,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.config,
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
impl From<[Pubkey; INITIALIZE_VAULT_WITH_FCFS_CONFIG_IX_ACCOUNTS_LEN]>
for InitializeVaultWithFcfsConfigKeys {
    fn from(
        pubkeys: [Pubkey; INITIALIZE_VAULT_WITH_FCFS_CONFIG_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            vault: pubkeys[0],
            pool: pubkeys[1],
            quote_mint: pubkeys[2],
            funder: pubkeys[3],
            config: pubkeys[4],
            system_program: pubkeys[5],
            event_authority: pubkeys[6],
            program: pubkeys[7],
        }
    }
}
impl<'info> From<InitializeVaultWithFcfsConfigAccounts<'_, 'info>>
for [AccountInfo<'info>; INITIALIZE_VAULT_WITH_FCFS_CONFIG_IX_ACCOUNTS_LEN] {
    fn from(accounts: InitializeVaultWithFcfsConfigAccounts<'_, 'info>) -> Self {
        [
            accounts.vault.clone(),
            accounts.pool.clone(),
            accounts.quote_mint.clone(),
            accounts.funder.clone(),
            accounts.config.clone(),
            accounts.system_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; INITIALIZE_VAULT_WITH_FCFS_CONFIG_IX_ACCOUNTS_LEN]>
for InitializeVaultWithFcfsConfigAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; INITIALIZE_VAULT_WITH_FCFS_CONFIG_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            vault: &arr[0],
            pool: &arr[1],
            quote_mint: &arr[2],
            funder: &arr[3],
            config: &arr[4],
            system_program: &arr[5],
            event_authority: &arr[6],
            program: &arr[7],
        }
    }
}
pub const INITIALIZE_VAULT_WITH_FCFS_CONFIG_IX_DISCM: [u8; 8usize] = [
    189, 251, 92, 104, 235, 21, 81, 182,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct InitializeVaultWithFcfsConfigIxArgs {
    pub params: InitializeVaultWithConfigParams,
}
#[derive(Clone, Debug, PartialEq)]
pub struct InitializeVaultWithFcfsConfigIxData(pub InitializeVaultWithFcfsConfigIxArgs);
impl From<InitializeVaultWithFcfsConfigIxArgs> for InitializeVaultWithFcfsConfigIxData {
    fn from(args: InitializeVaultWithFcfsConfigIxArgs) -> Self {
        Self(args)
    }
}
impl InitializeVaultWithFcfsConfigIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INITIALIZE_VAULT_WITH_FCFS_CONFIG_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let params = if reader.is_empty() {
            Default::default()
        } else {
            <InitializeVaultWithConfigParams>::deserialize(&mut reader)?
        };
        Ok(
            Self(InitializeVaultWithFcfsConfigIxArgs {
                params,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INITIALIZE_VAULT_WITH_FCFS_CONFIG_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.params, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn initialize_vault_with_fcfs_config_ix_with_program_id(
    program_id: Pubkey,
    keys: InitializeVaultWithFcfsConfigKeys,
    args: InitializeVaultWithFcfsConfigIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; INITIALIZE_VAULT_WITH_FCFS_CONFIG_IX_ACCOUNTS_LEN] = keys
        .into();
    let data: InitializeVaultWithFcfsConfigIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn initialize_vault_with_fcfs_config_ix(
    keys: InitializeVaultWithFcfsConfigKeys,
    args: InitializeVaultWithFcfsConfigIxArgs,
) -> std::io::Result<Instruction> {
    initialize_vault_with_fcfs_config_ix_with_program_id(
        ALPHA_VAULT_PROGRAM_ID,
        keys,
        args,
    )
}
pub fn initialize_vault_with_fcfs_config_invoke_with_program_id(
    program_id: Pubkey,
    accounts: InitializeVaultWithFcfsConfigAccounts<'_, '_>,
    args: InitializeVaultWithFcfsConfigIxArgs,
) -> ProgramResult {
    let keys: InitializeVaultWithFcfsConfigKeys = accounts.into();
    let ix = initialize_vault_with_fcfs_config_ix_with_program_id(
        program_id,
        keys,
        args,
    )?;
    invoke_instruction(&ix, accounts)
}
pub fn initialize_vault_with_fcfs_config_invoke(
    accounts: InitializeVaultWithFcfsConfigAccounts<'_, '_>,
    args: InitializeVaultWithFcfsConfigIxArgs,
) -> ProgramResult {
    initialize_vault_with_fcfs_config_invoke_with_program_id(
        ALPHA_VAULT_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn initialize_vault_with_fcfs_config_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: InitializeVaultWithFcfsConfigAccounts<'_, '_>,
    args: InitializeVaultWithFcfsConfigIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: InitializeVaultWithFcfsConfigKeys = accounts.into();
    let ix = initialize_vault_with_fcfs_config_ix_with_program_id(
        program_id,
        keys,
        args,
    )?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn initialize_vault_with_fcfs_config_invoke_signed(
    accounts: InitializeVaultWithFcfsConfigAccounts<'_, '_>,
    args: InitializeVaultWithFcfsConfigIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    initialize_vault_with_fcfs_config_invoke_signed_with_program_id(
        ALPHA_VAULT_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn initialize_vault_with_fcfs_config_verify_account_keys(
    accounts: InitializeVaultWithFcfsConfigAccounts<'_, '_>,
    keys: InitializeVaultWithFcfsConfigKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.vault.key, keys.vault),
        (*accounts.pool.key, keys.pool),
        (*accounts.quote_mint.key, keys.quote_mint),
        (*accounts.funder.key, keys.funder),
        (*accounts.config.key, keys.config),
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
pub fn initialize_vault_with_fcfs_config_verify_writable_privileges<'me, 'info>(
    accounts: InitializeVaultWithFcfsConfigAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.vault, accounts.funder] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn initialize_vault_with_fcfs_config_verify_signer_privileges<'me, 'info>(
    accounts: InitializeVaultWithFcfsConfigAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.funder] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn initialize_vault_with_fcfs_config_verify_account_privileges<'me, 'info>(
    accounts: InitializeVaultWithFcfsConfigAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    initialize_vault_with_fcfs_config_verify_writable_privileges(accounts)?;
    initialize_vault_with_fcfs_config_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const INITIALIZE_VAULT_WITH_PRORATA_CONFIG_IX_ACCOUNTS_LEN: usize = 8;
#[derive(Copy, Clone, Debug)]
pub struct InitializeVaultWithProrataConfigAccounts<'me, 'info> {
    pub vault: &'me AccountInfo<'info>,
    pub pool: &'me AccountInfo<'info>,
    pub quote_mint: &'me AccountInfo<'info>,
    pub funder: &'me AccountInfo<'info>,
    pub config: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct InitializeVaultWithProrataConfigKeys {
    pub vault: Pubkey,
    pub pool: Pubkey,
    pub quote_mint: Pubkey,
    pub funder: Pubkey,
    pub config: Pubkey,
    pub system_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<InitializeVaultWithProrataConfigAccounts<'_, '_>>
for InitializeVaultWithProrataConfigKeys {
    fn from(accounts: InitializeVaultWithProrataConfigAccounts) -> Self {
        Self {
            vault: *accounts.vault.key,
            pool: *accounts.pool.key,
            quote_mint: *accounts.quote_mint.key,
            funder: *accounts.funder.key,
            config: *accounts.config.key,
            system_program: *accounts.system_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<InitializeVaultWithProrataConfigKeys>
for [AccountMeta; INITIALIZE_VAULT_WITH_PRORATA_CONFIG_IX_ACCOUNTS_LEN] {
    fn from(keys: InitializeVaultWithProrataConfigKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pool,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.quote_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.funder,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.config,
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
impl From<[Pubkey; INITIALIZE_VAULT_WITH_PRORATA_CONFIG_IX_ACCOUNTS_LEN]>
for InitializeVaultWithProrataConfigKeys {
    fn from(
        pubkeys: [Pubkey; INITIALIZE_VAULT_WITH_PRORATA_CONFIG_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            vault: pubkeys[0],
            pool: pubkeys[1],
            quote_mint: pubkeys[2],
            funder: pubkeys[3],
            config: pubkeys[4],
            system_program: pubkeys[5],
            event_authority: pubkeys[6],
            program: pubkeys[7],
        }
    }
}
impl<'info> From<InitializeVaultWithProrataConfigAccounts<'_, 'info>>
for [AccountInfo<'info>; INITIALIZE_VAULT_WITH_PRORATA_CONFIG_IX_ACCOUNTS_LEN] {
    fn from(accounts: InitializeVaultWithProrataConfigAccounts<'_, 'info>) -> Self {
        [
            accounts.vault.clone(),
            accounts.pool.clone(),
            accounts.quote_mint.clone(),
            accounts.funder.clone(),
            accounts.config.clone(),
            accounts.system_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; INITIALIZE_VAULT_WITH_PRORATA_CONFIG_IX_ACCOUNTS_LEN]>
for InitializeVaultWithProrataConfigAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<
            'info,
        >; INITIALIZE_VAULT_WITH_PRORATA_CONFIG_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            vault: &arr[0],
            pool: &arr[1],
            quote_mint: &arr[2],
            funder: &arr[3],
            config: &arr[4],
            system_program: &arr[5],
            event_authority: &arr[6],
            program: &arr[7],
        }
    }
}
pub const INITIALIZE_VAULT_WITH_PRORATA_CONFIG_IX_DISCM: [u8; 8usize] = [
    155, 216, 34, 162, 103, 242, 236, 211,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct InitializeVaultWithProrataConfigIxArgs {
    pub params: InitializeVaultWithConfigParams,
}
#[derive(Clone, Debug, PartialEq)]
pub struct InitializeVaultWithProrataConfigIxData(
    pub InitializeVaultWithProrataConfigIxArgs,
);
impl From<InitializeVaultWithProrataConfigIxArgs>
for InitializeVaultWithProrataConfigIxData {
    fn from(args: InitializeVaultWithProrataConfigIxArgs) -> Self {
        Self(args)
    }
}
impl InitializeVaultWithProrataConfigIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INITIALIZE_VAULT_WITH_PRORATA_CONFIG_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let params = if reader.is_empty() {
            Default::default()
        } else {
            <InitializeVaultWithConfigParams>::deserialize(&mut reader)?
        };
        Ok(
            Self(InitializeVaultWithProrataConfigIxArgs {
                params,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INITIALIZE_VAULT_WITH_PRORATA_CONFIG_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.params, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn initialize_vault_with_prorata_config_ix_with_program_id(
    program_id: Pubkey,
    keys: InitializeVaultWithProrataConfigKeys,
    args: InitializeVaultWithProrataConfigIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; INITIALIZE_VAULT_WITH_PRORATA_CONFIG_IX_ACCOUNTS_LEN] = keys
        .into();
    let data: InitializeVaultWithProrataConfigIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn initialize_vault_with_prorata_config_ix(
    keys: InitializeVaultWithProrataConfigKeys,
    args: InitializeVaultWithProrataConfigIxArgs,
) -> std::io::Result<Instruction> {
    initialize_vault_with_prorata_config_ix_with_program_id(
        ALPHA_VAULT_PROGRAM_ID,
        keys,
        args,
    )
}
pub fn initialize_vault_with_prorata_config_invoke_with_program_id(
    program_id: Pubkey,
    accounts: InitializeVaultWithProrataConfigAccounts<'_, '_>,
    args: InitializeVaultWithProrataConfigIxArgs,
) -> ProgramResult {
    let keys: InitializeVaultWithProrataConfigKeys = accounts.into();
    let ix = initialize_vault_with_prorata_config_ix_with_program_id(
        program_id,
        keys,
        args,
    )?;
    invoke_instruction(&ix, accounts)
}
pub fn initialize_vault_with_prorata_config_invoke(
    accounts: InitializeVaultWithProrataConfigAccounts<'_, '_>,
    args: InitializeVaultWithProrataConfigIxArgs,
) -> ProgramResult {
    initialize_vault_with_prorata_config_invoke_with_program_id(
        ALPHA_VAULT_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn initialize_vault_with_prorata_config_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: InitializeVaultWithProrataConfigAccounts<'_, '_>,
    args: InitializeVaultWithProrataConfigIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: InitializeVaultWithProrataConfigKeys = accounts.into();
    let ix = initialize_vault_with_prorata_config_ix_with_program_id(
        program_id,
        keys,
        args,
    )?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn initialize_vault_with_prorata_config_invoke_signed(
    accounts: InitializeVaultWithProrataConfigAccounts<'_, '_>,
    args: InitializeVaultWithProrataConfigIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    initialize_vault_with_prorata_config_invoke_signed_with_program_id(
        ALPHA_VAULT_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn initialize_vault_with_prorata_config_verify_account_keys(
    accounts: InitializeVaultWithProrataConfigAccounts<'_, '_>,
    keys: InitializeVaultWithProrataConfigKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.vault.key, keys.vault),
        (*accounts.pool.key, keys.pool),
        (*accounts.quote_mint.key, keys.quote_mint),
        (*accounts.funder.key, keys.funder),
        (*accounts.config.key, keys.config),
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
pub fn initialize_vault_with_prorata_config_verify_writable_privileges<'me, 'info>(
    accounts: InitializeVaultWithProrataConfigAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.vault, accounts.funder] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn initialize_vault_with_prorata_config_verify_signer_privileges<'me, 'info>(
    accounts: InitializeVaultWithProrataConfigAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.funder] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn initialize_vault_with_prorata_config_verify_account_privileges<'me, 'info>(
    accounts: InitializeVaultWithProrataConfigAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    initialize_vault_with_prorata_config_verify_writable_privileges(accounts)?;
    initialize_vault_with_prorata_config_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const TRANSFER_VAULT_AUTHORITY_IX_ACCOUNTS_LEN: usize = 2;
#[derive(Copy, Clone, Debug)]
pub struct TransferVaultAuthorityAccounts<'me, 'info> {
    pub vault: &'me AccountInfo<'info>,
    pub vault_authority: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct TransferVaultAuthorityKeys {
    pub vault: Pubkey,
    pub vault_authority: Pubkey,
}
impl From<TransferVaultAuthorityAccounts<'_, '_>> for TransferVaultAuthorityKeys {
    fn from(accounts: TransferVaultAuthorityAccounts) -> Self {
        Self {
            vault: *accounts.vault.key,
            vault_authority: *accounts.vault_authority.key,
        }
    }
}
impl From<TransferVaultAuthorityKeys>
for [AccountMeta; TRANSFER_VAULT_AUTHORITY_IX_ACCOUNTS_LEN] {
    fn from(keys: TransferVaultAuthorityKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vault_authority,
                is_signer: true,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; TRANSFER_VAULT_AUTHORITY_IX_ACCOUNTS_LEN]>
for TransferVaultAuthorityKeys {
    fn from(pubkeys: [Pubkey; TRANSFER_VAULT_AUTHORITY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            vault: pubkeys[0],
            vault_authority: pubkeys[1],
        }
    }
}
impl<'info> From<TransferVaultAuthorityAccounts<'_, 'info>>
for [AccountInfo<'info>; TRANSFER_VAULT_AUTHORITY_IX_ACCOUNTS_LEN] {
    fn from(accounts: TransferVaultAuthorityAccounts<'_, 'info>) -> Self {
        [accounts.vault.clone(), accounts.vault_authority.clone()]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; TRANSFER_VAULT_AUTHORITY_IX_ACCOUNTS_LEN]>
for TransferVaultAuthorityAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; TRANSFER_VAULT_AUTHORITY_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            vault: &arr[0],
            vault_authority: &arr[1],
        }
    }
}
pub const TRANSFER_VAULT_AUTHORITY_IX_DISCM: [u8; 8usize] = [
    139, 35, 83, 88, 52, 186, 162, 110,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TransferVaultAuthorityIxArgs {
    pub new_authority: Pubkey,
}
#[derive(Clone, Debug, PartialEq)]
pub struct TransferVaultAuthorityIxData(pub TransferVaultAuthorityIxArgs);
impl From<TransferVaultAuthorityIxArgs> for TransferVaultAuthorityIxData {
    fn from(args: TransferVaultAuthorityIxArgs) -> Self {
        Self(args)
    }
}
impl TransferVaultAuthorityIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != TRANSFER_VAULT_AUTHORITY_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let new_authority: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(TransferVaultAuthorityIxArgs {
                new_authority,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&TRANSFER_VAULT_AUTHORITY_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.new_authority, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn transfer_vault_authority_ix_with_program_id(
    program_id: Pubkey,
    keys: TransferVaultAuthorityKeys,
    args: TransferVaultAuthorityIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; TRANSFER_VAULT_AUTHORITY_IX_ACCOUNTS_LEN] = keys.into();
    let data: TransferVaultAuthorityIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn transfer_vault_authority_ix(
    keys: TransferVaultAuthorityKeys,
    args: TransferVaultAuthorityIxArgs,
) -> std::io::Result<Instruction> {
    transfer_vault_authority_ix_with_program_id(ALPHA_VAULT_PROGRAM_ID, keys, args)
}
pub fn transfer_vault_authority_invoke_with_program_id(
    program_id: Pubkey,
    accounts: TransferVaultAuthorityAccounts<'_, '_>,
    args: TransferVaultAuthorityIxArgs,
) -> ProgramResult {
    let keys: TransferVaultAuthorityKeys = accounts.into();
    let ix = transfer_vault_authority_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn transfer_vault_authority_invoke(
    accounts: TransferVaultAuthorityAccounts<'_, '_>,
    args: TransferVaultAuthorityIxArgs,
) -> ProgramResult {
    transfer_vault_authority_invoke_with_program_id(
        ALPHA_VAULT_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn transfer_vault_authority_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: TransferVaultAuthorityAccounts<'_, '_>,
    args: TransferVaultAuthorityIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: TransferVaultAuthorityKeys = accounts.into();
    let ix = transfer_vault_authority_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn transfer_vault_authority_invoke_signed(
    accounts: TransferVaultAuthorityAccounts<'_, '_>,
    args: TransferVaultAuthorityIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    transfer_vault_authority_invoke_signed_with_program_id(
        ALPHA_VAULT_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn transfer_vault_authority_verify_account_keys(
    accounts: TransferVaultAuthorityAccounts<'_, '_>,
    keys: TransferVaultAuthorityKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.vault.key, keys.vault),
        (*accounts.vault_authority.key, keys.vault_authority),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn transfer_vault_authority_verify_writable_privileges<'me, 'info>(
    accounts: TransferVaultAuthorityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.vault] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn transfer_vault_authority_verify_signer_privileges<'me, 'info>(
    accounts: TransferVaultAuthorityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.vault_authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn transfer_vault_authority_verify_account_privileges<'me, 'info>(
    accounts: TransferVaultAuthorityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    transfer_vault_authority_verify_writable_privileges(accounts)?;
    transfer_vault_authority_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const UPDATE_FCFS_VAULT_PARAMETERS_IX_ACCOUNTS_LEN: usize = 5;
#[derive(Copy, Clone, Debug)]
pub struct UpdateFcfsVaultParametersAccounts<'me, 'info> {
    pub vault: &'me AccountInfo<'info>,
    pub pool: &'me AccountInfo<'info>,
    pub admin: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UpdateFcfsVaultParametersKeys {
    pub vault: Pubkey,
    pub pool: Pubkey,
    pub admin: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<UpdateFcfsVaultParametersAccounts<'_, '_>> for UpdateFcfsVaultParametersKeys {
    fn from(accounts: UpdateFcfsVaultParametersAccounts) -> Self {
        Self {
            vault: *accounts.vault.key,
            pool: *accounts.pool.key,
            admin: *accounts.admin.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<UpdateFcfsVaultParametersKeys>
for [AccountMeta; UPDATE_FCFS_VAULT_PARAMETERS_IX_ACCOUNTS_LEN] {
    fn from(keys: UpdateFcfsVaultParametersKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pool,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.admin,
                is_signer: true,
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
impl From<[Pubkey; UPDATE_FCFS_VAULT_PARAMETERS_IX_ACCOUNTS_LEN]>
for UpdateFcfsVaultParametersKeys {
    fn from(pubkeys: [Pubkey; UPDATE_FCFS_VAULT_PARAMETERS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            vault: pubkeys[0],
            pool: pubkeys[1],
            admin: pubkeys[2],
            event_authority: pubkeys[3],
            program: pubkeys[4],
        }
    }
}
impl<'info> From<UpdateFcfsVaultParametersAccounts<'_, 'info>>
for [AccountInfo<'info>; UPDATE_FCFS_VAULT_PARAMETERS_IX_ACCOUNTS_LEN] {
    fn from(accounts: UpdateFcfsVaultParametersAccounts<'_, 'info>) -> Self {
        [
            accounts.vault.clone(),
            accounts.pool.clone(),
            accounts.admin.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; UPDATE_FCFS_VAULT_PARAMETERS_IX_ACCOUNTS_LEN]>
for UpdateFcfsVaultParametersAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; UPDATE_FCFS_VAULT_PARAMETERS_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            vault: &arr[0],
            pool: &arr[1],
            admin: &arr[2],
            event_authority: &arr[3],
            program: &arr[4],
        }
    }
}
pub const UPDATE_FCFS_VAULT_PARAMETERS_IX_DISCM: [u8; 8usize] = [
    172, 23, 13, 143, 18, 133, 104, 174,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UpdateFcfsVaultParametersIxArgs {
    pub params: UpdateFcfsVaultParams,
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateFcfsVaultParametersIxData(pub UpdateFcfsVaultParametersIxArgs);
impl From<UpdateFcfsVaultParametersIxArgs> for UpdateFcfsVaultParametersIxData {
    fn from(args: UpdateFcfsVaultParametersIxArgs) -> Self {
        Self(args)
    }
}
impl UpdateFcfsVaultParametersIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_FCFS_VAULT_PARAMETERS_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let params = if reader.is_empty() {
            Default::default()
        } else {
            <UpdateFcfsVaultParams>::deserialize(&mut reader)?
        };
        Ok(
            Self(UpdateFcfsVaultParametersIxArgs {
                params,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_FCFS_VAULT_PARAMETERS_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.params, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn update_fcfs_vault_parameters_ix_with_program_id(
    program_id: Pubkey,
    keys: UpdateFcfsVaultParametersKeys,
    args: UpdateFcfsVaultParametersIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; UPDATE_FCFS_VAULT_PARAMETERS_IX_ACCOUNTS_LEN] = keys.into();
    let data: UpdateFcfsVaultParametersIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_fcfs_vault_parameters_ix(
    keys: UpdateFcfsVaultParametersKeys,
    args: UpdateFcfsVaultParametersIxArgs,
) -> std::io::Result<Instruction> {
    update_fcfs_vault_parameters_ix_with_program_id(ALPHA_VAULT_PROGRAM_ID, keys, args)
}
pub fn update_fcfs_vault_parameters_invoke_with_program_id(
    program_id: Pubkey,
    accounts: UpdateFcfsVaultParametersAccounts<'_, '_>,
    args: UpdateFcfsVaultParametersIxArgs,
) -> ProgramResult {
    let keys: UpdateFcfsVaultParametersKeys = accounts.into();
    let ix = update_fcfs_vault_parameters_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn update_fcfs_vault_parameters_invoke(
    accounts: UpdateFcfsVaultParametersAccounts<'_, '_>,
    args: UpdateFcfsVaultParametersIxArgs,
) -> ProgramResult {
    update_fcfs_vault_parameters_invoke_with_program_id(
        ALPHA_VAULT_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn update_fcfs_vault_parameters_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: UpdateFcfsVaultParametersAccounts<'_, '_>,
    args: UpdateFcfsVaultParametersIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: UpdateFcfsVaultParametersKeys = accounts.into();
    let ix = update_fcfs_vault_parameters_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn update_fcfs_vault_parameters_invoke_signed(
    accounts: UpdateFcfsVaultParametersAccounts<'_, '_>,
    args: UpdateFcfsVaultParametersIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    update_fcfs_vault_parameters_invoke_signed_with_program_id(
        ALPHA_VAULT_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn update_fcfs_vault_parameters_verify_account_keys(
    accounts: UpdateFcfsVaultParametersAccounts<'_, '_>,
    keys: UpdateFcfsVaultParametersKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.vault.key, keys.vault),
        (*accounts.pool.key, keys.pool),
        (*accounts.admin.key, keys.admin),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn update_fcfs_vault_parameters_verify_writable_privileges<'me, 'info>(
    accounts: UpdateFcfsVaultParametersAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.vault] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn update_fcfs_vault_parameters_verify_signer_privileges<'me, 'info>(
    accounts: UpdateFcfsVaultParametersAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn update_fcfs_vault_parameters_verify_account_privileges<'me, 'info>(
    accounts: UpdateFcfsVaultParametersAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    update_fcfs_vault_parameters_verify_writable_privileges(accounts)?;
    update_fcfs_vault_parameters_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const UPDATE_PRORATA_VAULT_PARAMETERS_IX_ACCOUNTS_LEN: usize = 5;
#[derive(Copy, Clone, Debug)]
pub struct UpdateProrataVaultParametersAccounts<'me, 'info> {
    pub vault: &'me AccountInfo<'info>,
    pub pool: &'me AccountInfo<'info>,
    pub admin: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UpdateProrataVaultParametersKeys {
    pub vault: Pubkey,
    pub pool: Pubkey,
    pub admin: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<UpdateProrataVaultParametersAccounts<'_, '_>>
for UpdateProrataVaultParametersKeys {
    fn from(accounts: UpdateProrataVaultParametersAccounts) -> Self {
        Self {
            vault: *accounts.vault.key,
            pool: *accounts.pool.key,
            admin: *accounts.admin.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<UpdateProrataVaultParametersKeys>
for [AccountMeta; UPDATE_PRORATA_VAULT_PARAMETERS_IX_ACCOUNTS_LEN] {
    fn from(keys: UpdateProrataVaultParametersKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pool,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.admin,
                is_signer: true,
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
impl From<[Pubkey; UPDATE_PRORATA_VAULT_PARAMETERS_IX_ACCOUNTS_LEN]>
for UpdateProrataVaultParametersKeys {
    fn from(pubkeys: [Pubkey; UPDATE_PRORATA_VAULT_PARAMETERS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            vault: pubkeys[0],
            pool: pubkeys[1],
            admin: pubkeys[2],
            event_authority: pubkeys[3],
            program: pubkeys[4],
        }
    }
}
impl<'info> From<UpdateProrataVaultParametersAccounts<'_, 'info>>
for [AccountInfo<'info>; UPDATE_PRORATA_VAULT_PARAMETERS_IX_ACCOUNTS_LEN] {
    fn from(accounts: UpdateProrataVaultParametersAccounts<'_, 'info>) -> Self {
        [
            accounts.vault.clone(),
            accounts.pool.clone(),
            accounts.admin.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; UPDATE_PRORATA_VAULT_PARAMETERS_IX_ACCOUNTS_LEN]>
for UpdateProrataVaultParametersAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; UPDATE_PRORATA_VAULT_PARAMETERS_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            vault: &arr[0],
            pool: &arr[1],
            admin: &arr[2],
            event_authority: &arr[3],
            program: &arr[4],
        }
    }
}
pub const UPDATE_PRORATA_VAULT_PARAMETERS_IX_DISCM: [u8; 8usize] = [
    177, 39, 151, 50, 253, 249, 5, 74,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UpdateProrataVaultParametersIxArgs {
    pub params: UpdateProrataVaultParams,
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateProrataVaultParametersIxData(pub UpdateProrataVaultParametersIxArgs);
impl From<UpdateProrataVaultParametersIxArgs> for UpdateProrataVaultParametersIxData {
    fn from(args: UpdateProrataVaultParametersIxArgs) -> Self {
        Self(args)
    }
}
impl UpdateProrataVaultParametersIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_PRORATA_VAULT_PARAMETERS_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let params = if reader.is_empty() {
            Default::default()
        } else {
            <UpdateProrataVaultParams>::deserialize(&mut reader)?
        };
        Ok(
            Self(UpdateProrataVaultParametersIxArgs {
                params,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_PRORATA_VAULT_PARAMETERS_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.params, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn update_prorata_vault_parameters_ix_with_program_id(
    program_id: Pubkey,
    keys: UpdateProrataVaultParametersKeys,
    args: UpdateProrataVaultParametersIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; UPDATE_PRORATA_VAULT_PARAMETERS_IX_ACCOUNTS_LEN] = keys
        .into();
    let data: UpdateProrataVaultParametersIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_prorata_vault_parameters_ix(
    keys: UpdateProrataVaultParametersKeys,
    args: UpdateProrataVaultParametersIxArgs,
) -> std::io::Result<Instruction> {
    update_prorata_vault_parameters_ix_with_program_id(
        ALPHA_VAULT_PROGRAM_ID,
        keys,
        args,
    )
}
pub fn update_prorata_vault_parameters_invoke_with_program_id(
    program_id: Pubkey,
    accounts: UpdateProrataVaultParametersAccounts<'_, '_>,
    args: UpdateProrataVaultParametersIxArgs,
) -> ProgramResult {
    let keys: UpdateProrataVaultParametersKeys = accounts.into();
    let ix = update_prorata_vault_parameters_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn update_prorata_vault_parameters_invoke(
    accounts: UpdateProrataVaultParametersAccounts<'_, '_>,
    args: UpdateProrataVaultParametersIxArgs,
) -> ProgramResult {
    update_prorata_vault_parameters_invoke_with_program_id(
        ALPHA_VAULT_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn update_prorata_vault_parameters_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: UpdateProrataVaultParametersAccounts<'_, '_>,
    args: UpdateProrataVaultParametersIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: UpdateProrataVaultParametersKeys = accounts.into();
    let ix = update_prorata_vault_parameters_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn update_prorata_vault_parameters_invoke_signed(
    accounts: UpdateProrataVaultParametersAccounts<'_, '_>,
    args: UpdateProrataVaultParametersIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    update_prorata_vault_parameters_invoke_signed_with_program_id(
        ALPHA_VAULT_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn update_prorata_vault_parameters_verify_account_keys(
    accounts: UpdateProrataVaultParametersAccounts<'_, '_>,
    keys: UpdateProrataVaultParametersKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.vault.key, keys.vault),
        (*accounts.pool.key, keys.pool),
        (*accounts.admin.key, keys.admin),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn update_prorata_vault_parameters_verify_writable_privileges<'me, 'info>(
    accounts: UpdateProrataVaultParametersAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.vault] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn update_prorata_vault_parameters_verify_signer_privileges<'me, 'info>(
    accounts: UpdateProrataVaultParametersAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn update_prorata_vault_parameters_verify_account_privileges<'me, 'info>(
    accounts: UpdateProrataVaultParametersAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    update_prorata_vault_parameters_verify_writable_privileges(accounts)?;
    update_prorata_vault_parameters_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const WITHDRAW_IX_ACCOUNTS_LEN: usize = 10;
#[derive(Copy, Clone, Debug)]
pub struct WithdrawAccounts<'me, 'info> {
    pub vault: &'me AccountInfo<'info>,
    pub pool: &'me AccountInfo<'info>,
    pub escrow: &'me AccountInfo<'info>,
    pub destination_token: &'me AccountInfo<'info>,
    pub token_vault: &'me AccountInfo<'info>,
    pub token_mint: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub owner: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct WithdrawKeys {
    pub vault: Pubkey,
    pub pool: Pubkey,
    pub escrow: Pubkey,
    pub destination_token: Pubkey,
    pub token_vault: Pubkey,
    pub token_mint: Pubkey,
    pub token_program: Pubkey,
    pub owner: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<WithdrawAccounts<'_, '_>> for WithdrawKeys {
    fn from(accounts: WithdrawAccounts) -> Self {
        Self {
            vault: *accounts.vault.key,
            pool: *accounts.pool.key,
            escrow: *accounts.escrow.key,
            destination_token: *accounts.destination_token.key,
            token_vault: *accounts.token_vault.key,
            token_mint: *accounts.token_mint.key,
            token_program: *accounts.token_program.key,
            owner: *accounts.owner.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<WithdrawKeys> for [AccountMeta; WITHDRAW_IX_ACCOUNTS_LEN] {
    fn from(keys: WithdrawKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pool,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.escrow,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.destination_token,
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
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.owner,
                is_signer: true,
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
            vault: pubkeys[0],
            pool: pubkeys[1],
            escrow: pubkeys[2],
            destination_token: pubkeys[3],
            token_vault: pubkeys[4],
            token_mint: pubkeys[5],
            token_program: pubkeys[6],
            owner: pubkeys[7],
            event_authority: pubkeys[8],
            program: pubkeys[9],
        }
    }
}
impl<'info> From<WithdrawAccounts<'_, 'info>>
for [AccountInfo<'info>; WITHDRAW_IX_ACCOUNTS_LEN] {
    fn from(accounts: WithdrawAccounts<'_, 'info>) -> Self {
        [
            accounts.vault.clone(),
            accounts.pool.clone(),
            accounts.escrow.clone(),
            accounts.destination_token.clone(),
            accounts.token_vault.clone(),
            accounts.token_mint.clone(),
            accounts.token_program.clone(),
            accounts.owner.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; WITHDRAW_IX_ACCOUNTS_LEN]>
for WithdrawAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; WITHDRAW_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            vault: &arr[0],
            pool: &arr[1],
            escrow: &arr[2],
            destination_token: &arr[3],
            token_vault: &arr[4],
            token_mint: &arr[5],
            token_program: &arr[6],
            owner: &arr[7],
            event_authority: &arr[8],
            program: &arr[9],
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
    withdraw_ix_with_program_id(ALPHA_VAULT_PROGRAM_ID, keys, args)
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
    withdraw_invoke_with_program_id(ALPHA_VAULT_PROGRAM_ID, accounts, args)
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
    withdraw_invoke_signed_with_program_id(ALPHA_VAULT_PROGRAM_ID, accounts, args, seeds)
}
pub fn withdraw_verify_account_keys(
    accounts: WithdrawAccounts<'_, '_>,
    keys: WithdrawKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.vault.key, keys.vault),
        (*accounts.pool.key, keys.pool),
        (*accounts.escrow.key, keys.escrow),
        (*accounts.destination_token.key, keys.destination_token),
        (*accounts.token_vault.key, keys.token_vault),
        (*accounts.token_mint.key, keys.token_mint),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.owner.key, keys.owner),
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
        accounts.vault,
        accounts.escrow,
        accounts.destination_token,
        accounts.token_vault,
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
    for should_be_signer in [accounts.owner] {
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
pub const WITHDRAW_REMAINING_QUOTE_IX_ACCOUNTS_LEN: usize = 10;
#[derive(Copy, Clone, Debug)]
pub struct WithdrawRemainingQuoteAccounts<'me, 'info> {
    pub vault: &'me AccountInfo<'info>,
    pub pool: &'me AccountInfo<'info>,
    pub escrow: &'me AccountInfo<'info>,
    pub token_vault: &'me AccountInfo<'info>,
    pub destination_token: &'me AccountInfo<'info>,
    pub token_mint: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub owner: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct WithdrawRemainingQuoteKeys {
    pub vault: Pubkey,
    pub pool: Pubkey,
    pub escrow: Pubkey,
    pub token_vault: Pubkey,
    pub destination_token: Pubkey,
    pub token_mint: Pubkey,
    pub token_program: Pubkey,
    pub owner: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<WithdrawRemainingQuoteAccounts<'_, '_>> for WithdrawRemainingQuoteKeys {
    fn from(accounts: WithdrawRemainingQuoteAccounts) -> Self {
        Self {
            vault: *accounts.vault.key,
            pool: *accounts.pool.key,
            escrow: *accounts.escrow.key,
            token_vault: *accounts.token_vault.key,
            destination_token: *accounts.destination_token.key,
            token_mint: *accounts.token_mint.key,
            token_program: *accounts.token_program.key,
            owner: *accounts.owner.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<WithdrawRemainingQuoteKeys>
for [AccountMeta; WITHDRAW_REMAINING_QUOTE_IX_ACCOUNTS_LEN] {
    fn from(keys: WithdrawRemainingQuoteKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pool,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.escrow,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.destination_token,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.owner,
                is_signer: true,
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
impl From<[Pubkey; WITHDRAW_REMAINING_QUOTE_IX_ACCOUNTS_LEN]>
for WithdrawRemainingQuoteKeys {
    fn from(pubkeys: [Pubkey; WITHDRAW_REMAINING_QUOTE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            vault: pubkeys[0],
            pool: pubkeys[1],
            escrow: pubkeys[2],
            token_vault: pubkeys[3],
            destination_token: pubkeys[4],
            token_mint: pubkeys[5],
            token_program: pubkeys[6],
            owner: pubkeys[7],
            event_authority: pubkeys[8],
            program: pubkeys[9],
        }
    }
}
impl<'info> From<WithdrawRemainingQuoteAccounts<'_, 'info>>
for [AccountInfo<'info>; WITHDRAW_REMAINING_QUOTE_IX_ACCOUNTS_LEN] {
    fn from(accounts: WithdrawRemainingQuoteAccounts<'_, 'info>) -> Self {
        [
            accounts.vault.clone(),
            accounts.pool.clone(),
            accounts.escrow.clone(),
            accounts.token_vault.clone(),
            accounts.destination_token.clone(),
            accounts.token_mint.clone(),
            accounts.token_program.clone(),
            accounts.owner.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; WITHDRAW_REMAINING_QUOTE_IX_ACCOUNTS_LEN]>
for WithdrawRemainingQuoteAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; WITHDRAW_REMAINING_QUOTE_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            vault: &arr[0],
            pool: &arr[1],
            escrow: &arr[2],
            token_vault: &arr[3],
            destination_token: &arr[4],
            token_mint: &arr[5],
            token_program: &arr[6],
            owner: &arr[7],
            event_authority: &arr[8],
            program: &arr[9],
        }
    }
}
pub const WITHDRAW_REMAINING_QUOTE_IX_DISCM: [u8; 8usize] = [
    54, 253, 188, 34, 100, 145, 59, 127,
];
#[derive(Clone, Debug, PartialEq)]
pub struct WithdrawRemainingQuoteIxData;
impl WithdrawRemainingQuoteIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != WITHDRAW_REMAINING_QUOTE_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&WITHDRAW_REMAINING_QUOTE_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn withdraw_remaining_quote_ix_with_program_id(
    program_id: Pubkey,
    keys: WithdrawRemainingQuoteKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; WITHDRAW_REMAINING_QUOTE_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: WithdrawRemainingQuoteIxData.try_to_vec()?,
    })
}
pub fn withdraw_remaining_quote_ix(
    keys: WithdrawRemainingQuoteKeys,
) -> std::io::Result<Instruction> {
    withdraw_remaining_quote_ix_with_program_id(ALPHA_VAULT_PROGRAM_ID, keys)
}
pub fn withdraw_remaining_quote_invoke_with_program_id(
    program_id: Pubkey,
    accounts: WithdrawRemainingQuoteAccounts<'_, '_>,
) -> ProgramResult {
    let keys: WithdrawRemainingQuoteKeys = accounts.into();
    let ix = withdraw_remaining_quote_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn withdraw_remaining_quote_invoke(
    accounts: WithdrawRemainingQuoteAccounts<'_, '_>,
) -> ProgramResult {
    withdraw_remaining_quote_invoke_with_program_id(ALPHA_VAULT_PROGRAM_ID, accounts)
}
pub fn withdraw_remaining_quote_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: WithdrawRemainingQuoteAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: WithdrawRemainingQuoteKeys = accounts.into();
    let ix = withdraw_remaining_quote_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn withdraw_remaining_quote_invoke_signed(
    accounts: WithdrawRemainingQuoteAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    withdraw_remaining_quote_invoke_signed_with_program_id(
        ALPHA_VAULT_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn withdraw_remaining_quote_verify_account_keys(
    accounts: WithdrawRemainingQuoteAccounts<'_, '_>,
    keys: WithdrawRemainingQuoteKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.vault.key, keys.vault),
        (*accounts.pool.key, keys.pool),
        (*accounts.escrow.key, keys.escrow),
        (*accounts.token_vault.key, keys.token_vault),
        (*accounts.destination_token.key, keys.destination_token),
        (*accounts.token_mint.key, keys.token_mint),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.owner.key, keys.owner),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn withdraw_remaining_quote_verify_writable_privileges<'me, 'info>(
    accounts: WithdrawRemainingQuoteAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.vault,
        accounts.escrow,
        accounts.token_vault,
        accounts.destination_token,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn withdraw_remaining_quote_verify_signer_privileges<'me, 'info>(
    accounts: WithdrawRemainingQuoteAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.owner] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn withdraw_remaining_quote_verify_account_privileges<'me, 'info>(
    accounts: WithdrawRemainingQuoteAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    withdraw_remaining_quote_verify_writable_privileges(accounts)?;
    withdraw_remaining_quote_verify_signer_privileges(accounts)?;
    Ok(())
}
