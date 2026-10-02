use solana_pubkey::Pubkey;
use solana_cpi::{invoke, invoke_signed};
use solana_instruction::{AccountMeta, Instruction};
use solana_account_info::AccountInfo;
use solana_program_error::ProgramError;
use std::io::Read;
#[allow(unused_imports)]
use crate::*;
#[derive(Clone, Debug, PartialEq)]
pub enum FutarchyProgramIx {
    InitializeDao(InitializeDaoIxArgs),
    InitializeProposal,
    StakeToProposal(StakeToProposalIxArgs),
    UnstakeFromProposal(UnstakeFromProposalIxArgs),
    LaunchProposal,
    FinalizeProposal,
    UpdateDao(UpdateDaoIxArgs),
    ResizeDao,
    SpotSwap(SpotSwapIxArgs),
    ConditionalSwap(ConditionalSwapIxArgs),
    ProvideLiquidity(ProvideLiquidityIxArgs),
    WithdrawLiquidity(WithdrawLiquidityIxArgs),
    CollectFees,
    ExecuteSpendingLimitChange,
    SponsorProposal,
    CollectMeteoraDammFees,
    InitiateVaultSpendOptimisticProposal(InitiateVaultSpendOptimisticProposalIxArgs),
    FinalizeOptimisticProposal,
    AdminEnqueueMultisigProposalApproval(AdminEnqueueMultisigProposalApprovalIxArgs),
    ExecuteMultisigProposalApproval,
    AdminExecuteMultisigProposal,
    AdminCancelProposal,
    AdminRemoveProposal,
    AdminApproveMultisigProposal(AdminApproveMultisigProposalIxArgs),
}
impl FutarchyProgramIx {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        if buf.starts_with(&INITIALIZE_DAO_IX_DISCM) {
            let mut reader = &buf[INITIALIZE_DAO_IX_DISCM.len()..];
            let params = if reader.is_empty() {
                Default::default()
            } else {
                <InitializeDaoParams>::deserialize(&mut reader)?
            };
            return Ok(Self::InitializeDao(InitializeDaoIxArgs { params }));
        }
        if buf.starts_with(&INITIALIZE_PROPOSAL_IX_DISCM) {
            return Ok(Self::InitializeProposal);
        }
        if buf.starts_with(&STAKE_TO_PROPOSAL_IX_DISCM) {
            let mut reader = &buf[STAKE_TO_PROPOSAL_IX_DISCM.len()..];
            let params = if reader.is_empty() {
                Default::default()
            } else {
                <StakeToProposalParams>::deserialize(&mut reader)?
            };
            return Ok(Self::StakeToProposal(StakeToProposalIxArgs { params }));
        }
        if buf.starts_with(&UNSTAKE_FROM_PROPOSAL_IX_DISCM) {
            let mut reader = &buf[UNSTAKE_FROM_PROPOSAL_IX_DISCM.len()..];
            let params = if reader.is_empty() {
                Default::default()
            } else {
                <UnstakeFromProposalParams>::deserialize(&mut reader)?
            };
            return Ok(
                Self::UnstakeFromProposal(UnstakeFromProposalIxArgs {
                    params,
                }),
            );
        }
        if buf.starts_with(&LAUNCH_PROPOSAL_IX_DISCM) {
            return Ok(Self::LaunchProposal);
        }
        if buf.starts_with(&FINALIZE_PROPOSAL_IX_DISCM) {
            return Ok(Self::FinalizeProposal);
        }
        if buf.starts_with(&UPDATE_DAO_IX_DISCM) {
            let mut reader = &buf[UPDATE_DAO_IX_DISCM.len()..];
            let dao_params = if reader.is_empty() {
                Default::default()
            } else {
                <UpdateDaoParams>::deserialize(&mut reader)?
            };
            return Ok(Self::UpdateDao(UpdateDaoIxArgs { dao_params }));
        }
        if buf.starts_with(&RESIZE_DAO_IX_DISCM) {
            return Ok(Self::ResizeDao);
        }
        if buf.starts_with(&SPOT_SWAP_IX_DISCM) {
            let mut reader = &buf[SPOT_SWAP_IX_DISCM.len()..];
            let params = if reader.is_empty() {
                Default::default()
            } else {
                <SpotSwapParams>::deserialize(&mut reader)?
            };
            return Ok(Self::SpotSwap(SpotSwapIxArgs { params }));
        }
        if buf.starts_with(&CONDITIONAL_SWAP_IX_DISCM) {
            let mut reader = &buf[CONDITIONAL_SWAP_IX_DISCM.len()..];
            let params = if reader.is_empty() {
                Default::default()
            } else {
                <ConditionalSwapParams>::deserialize(&mut reader)?
            };
            return Ok(Self::ConditionalSwap(ConditionalSwapIxArgs { params }));
        }
        if buf.starts_with(&PROVIDE_LIQUIDITY_IX_DISCM) {
            let mut reader = &buf[PROVIDE_LIQUIDITY_IX_DISCM.len()..];
            let params = if reader.is_empty() {
                Default::default()
            } else {
                <ProvideLiquidityParams>::deserialize(&mut reader)?
            };
            return Ok(Self::ProvideLiquidity(ProvideLiquidityIxArgs { params }));
        }
        if buf.starts_with(&WITHDRAW_LIQUIDITY_IX_DISCM) {
            let mut reader = &buf[WITHDRAW_LIQUIDITY_IX_DISCM.len()..];
            let params = if reader.is_empty() {
                Default::default()
            } else {
                <WithdrawLiquidityParams>::deserialize(&mut reader)?
            };
            return Ok(Self::WithdrawLiquidity(WithdrawLiquidityIxArgs { params }));
        }
        if buf.starts_with(&COLLECT_FEES_IX_DISCM) {
            return Ok(Self::CollectFees);
        }
        if buf.starts_with(&EXECUTE_SPENDING_LIMIT_CHANGE_IX_DISCM) {
            return Ok(Self::ExecuteSpendingLimitChange);
        }
        if buf.starts_with(&SPONSOR_PROPOSAL_IX_DISCM) {
            return Ok(Self::SponsorProposal);
        }
        if buf.starts_with(&COLLECT_METEORA_DAMM_FEES_IX_DISCM) {
            return Ok(Self::CollectMeteoraDammFees);
        }
        if buf.starts_with(&INITIATE_VAULT_SPEND_OPTIMISTIC_PROPOSAL_IX_DISCM) {
            let mut reader = &buf[INITIATE_VAULT_SPEND_OPTIMISTIC_PROPOSAL_IX_DISCM
                .len()..];
            let params = if reader.is_empty() {
                Default::default()
            } else {
                <InitiateVaultSpendOptimisticProposalParams>::deserialize(&mut reader)?
            };
            return Ok(
                Self::InitiateVaultSpendOptimisticProposal(InitiateVaultSpendOptimisticProposalIxArgs {
                    params,
                }),
            );
        }
        if buf.starts_with(&FINALIZE_OPTIMISTIC_PROPOSAL_IX_DISCM) {
            return Ok(Self::FinalizeOptimisticProposal);
        }
        if buf.starts_with(&ADMIN_ENQUEUE_MULTISIG_PROPOSAL_APPROVAL_IX_DISCM) {
            let mut reader = &buf[ADMIN_ENQUEUE_MULTISIG_PROPOSAL_APPROVAL_IX_DISCM
                .len()..];
            let args = if reader.is_empty() {
                Default::default()
            } else {
                <AdminEnqueueMultisigProposalApprovalArgs>::deserialize(&mut reader)?
            };
            return Ok(
                Self::AdminEnqueueMultisigProposalApproval(AdminEnqueueMultisigProposalApprovalIxArgs {
                    args,
                }),
            );
        }
        if buf.starts_with(&EXECUTE_MULTISIG_PROPOSAL_APPROVAL_IX_DISCM) {
            return Ok(Self::ExecuteMultisigProposalApproval);
        }
        if buf.starts_with(&ADMIN_EXECUTE_MULTISIG_PROPOSAL_IX_DISCM) {
            return Ok(Self::AdminExecuteMultisigProposal);
        }
        if buf.starts_with(&ADMIN_CANCEL_PROPOSAL_IX_DISCM) {
            return Ok(Self::AdminCancelProposal);
        }
        if buf.starts_with(&ADMIN_REMOVE_PROPOSAL_IX_DISCM) {
            return Ok(Self::AdminRemoveProposal);
        }
        if buf.starts_with(&ADMIN_APPROVE_MULTISIG_PROPOSAL_IX_DISCM) {
            let mut reader = &buf[ADMIN_APPROVE_MULTISIG_PROPOSAL_IX_DISCM.len()..];
            let args = if reader.is_empty() {
                Default::default()
            } else {
                <AdminApproveMultisigProposalArgs>::deserialize(&mut reader)?
            };
            return Ok(
                Self::AdminApproveMultisigProposal(AdminApproveMultisigProposalIxArgs {
                    args,
                }),
            );
        }
        Err(std::io::Error::from(std::io::ErrorKind::InvalidData))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        match self {
            Self::InitializeDao(args) => {
                writer.write_all(&INITIALIZE_DAO_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.params, &mut writer)?;
                Ok(())
            }
            Self::InitializeProposal => writer.write_all(&INITIALIZE_PROPOSAL_IX_DISCM),
            Self::StakeToProposal(args) => {
                writer.write_all(&STAKE_TO_PROPOSAL_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.params, &mut writer)?;
                Ok(())
            }
            Self::UnstakeFromProposal(args) => {
                writer.write_all(&UNSTAKE_FROM_PROPOSAL_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.params, &mut writer)?;
                Ok(())
            }
            Self::LaunchProposal => writer.write_all(&LAUNCH_PROPOSAL_IX_DISCM),
            Self::FinalizeProposal => writer.write_all(&FINALIZE_PROPOSAL_IX_DISCM),
            Self::UpdateDao(args) => {
                writer.write_all(&UPDATE_DAO_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.dao_params, &mut writer)?;
                Ok(())
            }
            Self::ResizeDao => writer.write_all(&RESIZE_DAO_IX_DISCM),
            Self::SpotSwap(args) => {
                writer.write_all(&SPOT_SWAP_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.params, &mut writer)?;
                Ok(())
            }
            Self::ConditionalSwap(args) => {
                writer.write_all(&CONDITIONAL_SWAP_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.params, &mut writer)?;
                Ok(())
            }
            Self::ProvideLiquidity(args) => {
                writer.write_all(&PROVIDE_LIQUIDITY_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.params, &mut writer)?;
                Ok(())
            }
            Self::WithdrawLiquidity(args) => {
                writer.write_all(&WITHDRAW_LIQUIDITY_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.params, &mut writer)?;
                Ok(())
            }
            Self::CollectFees => writer.write_all(&COLLECT_FEES_IX_DISCM),
            Self::ExecuteSpendingLimitChange => {
                writer.write_all(&EXECUTE_SPENDING_LIMIT_CHANGE_IX_DISCM)
            }
            Self::SponsorProposal => writer.write_all(&SPONSOR_PROPOSAL_IX_DISCM),
            Self::CollectMeteoraDammFees => {
                writer.write_all(&COLLECT_METEORA_DAMM_FEES_IX_DISCM)
            }
            Self::InitiateVaultSpendOptimisticProposal(args) => {
                writer.write_all(&INITIATE_VAULT_SPEND_OPTIMISTIC_PROPOSAL_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.params, &mut writer)?;
                Ok(())
            }
            Self::FinalizeOptimisticProposal => {
                writer.write_all(&FINALIZE_OPTIMISTIC_PROPOSAL_IX_DISCM)
            }
            Self::AdminEnqueueMultisigProposalApproval(args) => {
                writer.write_all(&ADMIN_ENQUEUE_MULTISIG_PROPOSAL_APPROVAL_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.args, &mut writer)?;
                Ok(())
            }
            Self::ExecuteMultisigProposalApproval => {
                writer.write_all(&EXECUTE_MULTISIG_PROPOSAL_APPROVAL_IX_DISCM)
            }
            Self::AdminExecuteMultisigProposal => {
                writer.write_all(&ADMIN_EXECUTE_MULTISIG_PROPOSAL_IX_DISCM)
            }
            Self::AdminCancelProposal => {
                writer.write_all(&ADMIN_CANCEL_PROPOSAL_IX_DISCM)
            }
            Self::AdminRemoveProposal => {
                writer.write_all(&ADMIN_REMOVE_PROPOSAL_IX_DISCM)
            }
            Self::AdminApproveMultisigProposal(args) => {
                writer.write_all(&ADMIN_APPROVE_MULTISIG_PROPOSAL_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.args, &mut writer)?;
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
pub const INITIALIZE_DAO_IX_ACCOUNTS_LEN: usize = 18;
#[derive(Copy, Clone, Debug)]
pub struct InitializeDaoAccounts<'me, 'info> {
    pub dao: &'me AccountInfo<'info>,
    pub dao_creator: &'me AccountInfo<'info>,
    pub payer: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub base_mint: &'me AccountInfo<'info>,
    pub quote_mint: &'me AccountInfo<'info>,
    pub squads_multisig: &'me AccountInfo<'info>,
    pub squads_multisig_vault: &'me AccountInfo<'info>,
    pub squads_program: &'me AccountInfo<'info>,
    pub squads_program_config: &'me AccountInfo<'info>,
    pub squads_program_config_treasury: &'me AccountInfo<'info>,
    pub spending_limit: &'me AccountInfo<'info>,
    pub futarchy_amm_base_vault: &'me AccountInfo<'info>,
    pub futarchy_amm_quote_vault: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct InitializeDaoKeys {
    pub dao: Pubkey,
    pub dao_creator: Pubkey,
    pub payer: Pubkey,
    pub system_program: Pubkey,
    pub base_mint: Pubkey,
    pub quote_mint: Pubkey,
    pub squads_multisig: Pubkey,
    pub squads_multisig_vault: Pubkey,
    pub squads_program: Pubkey,
    pub squads_program_config: Pubkey,
    pub squads_program_config_treasury: Pubkey,
    pub spending_limit: Pubkey,
    pub futarchy_amm_base_vault: Pubkey,
    pub futarchy_amm_quote_vault: Pubkey,
    pub token_program: Pubkey,
    pub associated_token_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<InitializeDaoAccounts<'_, '_>> for InitializeDaoKeys {
    fn from(accounts: InitializeDaoAccounts) -> Self {
        Self {
            dao: *accounts.dao.key,
            dao_creator: *accounts.dao_creator.key,
            payer: *accounts.payer.key,
            system_program: *accounts.system_program.key,
            base_mint: *accounts.base_mint.key,
            quote_mint: *accounts.quote_mint.key,
            squads_multisig: *accounts.squads_multisig.key,
            squads_multisig_vault: *accounts.squads_multisig_vault.key,
            squads_program: *accounts.squads_program.key,
            squads_program_config: *accounts.squads_program_config.key,
            squads_program_config_treasury: *accounts.squads_program_config_treasury.key,
            spending_limit: *accounts.spending_limit.key,
            futarchy_amm_base_vault: *accounts.futarchy_amm_base_vault.key,
            futarchy_amm_quote_vault: *accounts.futarchy_amm_quote_vault.key,
            token_program: *accounts.token_program.key,
            associated_token_program: *accounts.associated_token_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<InitializeDaoKeys> for [AccountMeta; INITIALIZE_DAO_IX_ACCOUNTS_LEN] {
    fn from(keys: InitializeDaoKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.dao,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.dao_creator,
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
                pubkey: keys.squads_multisig,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.squads_multisig_vault,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.squads_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.squads_program_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.squads_program_config_treasury,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.spending_limit,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.futarchy_amm_base_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.futarchy_amm_quote_vault,
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
impl From<[Pubkey; INITIALIZE_DAO_IX_ACCOUNTS_LEN]> for InitializeDaoKeys {
    fn from(pubkeys: [Pubkey; INITIALIZE_DAO_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            dao: pubkeys[0],
            dao_creator: pubkeys[1],
            payer: pubkeys[2],
            system_program: pubkeys[3],
            base_mint: pubkeys[4],
            quote_mint: pubkeys[5],
            squads_multisig: pubkeys[6],
            squads_multisig_vault: pubkeys[7],
            squads_program: pubkeys[8],
            squads_program_config: pubkeys[9],
            squads_program_config_treasury: pubkeys[10],
            spending_limit: pubkeys[11],
            futarchy_amm_base_vault: pubkeys[12],
            futarchy_amm_quote_vault: pubkeys[13],
            token_program: pubkeys[14],
            associated_token_program: pubkeys[15],
            event_authority: pubkeys[16],
            program: pubkeys[17],
        }
    }
}
impl<'info> From<InitializeDaoAccounts<'_, 'info>>
for [AccountInfo<'info>; INITIALIZE_DAO_IX_ACCOUNTS_LEN] {
    fn from(accounts: InitializeDaoAccounts<'_, 'info>) -> Self {
        [
            accounts.dao.clone(),
            accounts.dao_creator.clone(),
            accounts.payer.clone(),
            accounts.system_program.clone(),
            accounts.base_mint.clone(),
            accounts.quote_mint.clone(),
            accounts.squads_multisig.clone(),
            accounts.squads_multisig_vault.clone(),
            accounts.squads_program.clone(),
            accounts.squads_program_config.clone(),
            accounts.squads_program_config_treasury.clone(),
            accounts.spending_limit.clone(),
            accounts.futarchy_amm_base_vault.clone(),
            accounts.futarchy_amm_quote_vault.clone(),
            accounts.token_program.clone(),
            accounts.associated_token_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; INITIALIZE_DAO_IX_ACCOUNTS_LEN]>
for InitializeDaoAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; INITIALIZE_DAO_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            dao: &arr[0],
            dao_creator: &arr[1],
            payer: &arr[2],
            system_program: &arr[3],
            base_mint: &arr[4],
            quote_mint: &arr[5],
            squads_multisig: &arr[6],
            squads_multisig_vault: &arr[7],
            squads_program: &arr[8],
            squads_program_config: &arr[9],
            squads_program_config_treasury: &arr[10],
            spending_limit: &arr[11],
            futarchy_amm_base_vault: &arr[12],
            futarchy_amm_quote_vault: &arr[13],
            token_program: &arr[14],
            associated_token_program: &arr[15],
            event_authority: &arr[16],
            program: &arr[17],
        }
    }
}
pub const INITIALIZE_DAO_IX_DISCM: [u8; 8usize] = [128, 226, 96, 90, 39, 56, 24, 196];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct InitializeDaoIxArgs {
    pub params: InitializeDaoParams,
}
#[derive(Clone, Debug, PartialEq)]
pub struct InitializeDaoIxData(pub InitializeDaoIxArgs);
impl From<InitializeDaoIxArgs> for InitializeDaoIxData {
    fn from(args: InitializeDaoIxArgs) -> Self {
        Self(args)
    }
}
impl InitializeDaoIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INITIALIZE_DAO_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let params = if reader.is_empty() {
            Default::default()
        } else {
            <InitializeDaoParams>::deserialize(&mut reader)?
        };
        Ok(Self(InitializeDaoIxArgs { params }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INITIALIZE_DAO_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.params, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn initialize_dao_ix_with_program_id(
    program_id: Pubkey,
    keys: InitializeDaoKeys,
    args: InitializeDaoIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; INITIALIZE_DAO_IX_ACCOUNTS_LEN] = keys.into();
    let data: InitializeDaoIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn initialize_dao_ix(
    keys: InitializeDaoKeys,
    args: InitializeDaoIxArgs,
) -> std::io::Result<Instruction> {
    initialize_dao_ix_with_program_id(FUTARCHY_PROGRAM_ID, keys, args)
}
pub fn initialize_dao_invoke_with_program_id(
    program_id: Pubkey,
    accounts: InitializeDaoAccounts<'_, '_>,
    args: InitializeDaoIxArgs,
) -> ProgramResult {
    let keys: InitializeDaoKeys = accounts.into();
    let ix = initialize_dao_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn initialize_dao_invoke(
    accounts: InitializeDaoAccounts<'_, '_>,
    args: InitializeDaoIxArgs,
) -> ProgramResult {
    initialize_dao_invoke_with_program_id(FUTARCHY_PROGRAM_ID, accounts, args)
}
pub fn initialize_dao_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: InitializeDaoAccounts<'_, '_>,
    args: InitializeDaoIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: InitializeDaoKeys = accounts.into();
    let ix = initialize_dao_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn initialize_dao_invoke_signed(
    accounts: InitializeDaoAccounts<'_, '_>,
    args: InitializeDaoIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    initialize_dao_invoke_signed_with_program_id(
        FUTARCHY_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn initialize_dao_verify_account_keys(
    accounts: InitializeDaoAccounts<'_, '_>,
    keys: InitializeDaoKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.dao.key, keys.dao),
        (*accounts.dao_creator.key, keys.dao_creator),
        (*accounts.payer.key, keys.payer),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.base_mint.key, keys.base_mint),
        (*accounts.quote_mint.key, keys.quote_mint),
        (*accounts.squads_multisig.key, keys.squads_multisig),
        (*accounts.squads_multisig_vault.key, keys.squads_multisig_vault),
        (*accounts.squads_program.key, keys.squads_program),
        (*accounts.squads_program_config.key, keys.squads_program_config),
        (
            *accounts.squads_program_config_treasury.key,
            keys.squads_program_config_treasury,
        ),
        (*accounts.spending_limit.key, keys.spending_limit),
        (*accounts.futarchy_amm_base_vault.key, keys.futarchy_amm_base_vault),
        (*accounts.futarchy_amm_quote_vault.key, keys.futarchy_amm_quote_vault),
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
pub fn initialize_dao_verify_writable_privileges<'me, 'info>(
    accounts: InitializeDaoAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.dao,
        accounts.payer,
        accounts.squads_multisig,
        accounts.squads_program_config_treasury,
        accounts.spending_limit,
        accounts.futarchy_amm_base_vault,
        accounts.futarchy_amm_quote_vault,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn initialize_dao_verify_signer_privileges<'me, 'info>(
    accounts: InitializeDaoAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.dao_creator, accounts.payer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn initialize_dao_verify_account_privileges<'me, 'info>(
    accounts: InitializeDaoAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    initialize_dao_verify_writable_privileges(accounts)?;
    initialize_dao_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const INITIALIZE_PROPOSAL_IX_ACCOUNTS_LEN: usize = 12;
#[derive(Copy, Clone, Debug)]
pub struct InitializeProposalAccounts<'me, 'info> {
    pub proposal: &'me AccountInfo<'info>,
    pub squads_proposal: &'me AccountInfo<'info>,
    pub squads_multisig: &'me AccountInfo<'info>,
    pub dao: &'me AccountInfo<'info>,
    pub question: &'me AccountInfo<'info>,
    pub quote_vault: &'me AccountInfo<'info>,
    pub base_vault: &'me AccountInfo<'info>,
    pub proposer: &'me AccountInfo<'info>,
    pub payer: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct InitializeProposalKeys {
    pub proposal: Pubkey,
    pub squads_proposal: Pubkey,
    pub squads_multisig: Pubkey,
    pub dao: Pubkey,
    pub question: Pubkey,
    pub quote_vault: Pubkey,
    pub base_vault: Pubkey,
    pub proposer: Pubkey,
    pub payer: Pubkey,
    pub system_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<InitializeProposalAccounts<'_, '_>> for InitializeProposalKeys {
    fn from(accounts: InitializeProposalAccounts) -> Self {
        Self {
            proposal: *accounts.proposal.key,
            squads_proposal: *accounts.squads_proposal.key,
            squads_multisig: *accounts.squads_multisig.key,
            dao: *accounts.dao.key,
            question: *accounts.question.key,
            quote_vault: *accounts.quote_vault.key,
            base_vault: *accounts.base_vault.key,
            proposer: *accounts.proposer.key,
            payer: *accounts.payer.key,
            system_program: *accounts.system_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<InitializeProposalKeys>
for [AccountMeta; INITIALIZE_PROPOSAL_IX_ACCOUNTS_LEN] {
    fn from(keys: InitializeProposalKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.proposal,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.squads_proposal,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.squads_multisig,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.dao,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.question,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.quote_vault,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.base_vault,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.proposer,
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
impl From<[Pubkey; INITIALIZE_PROPOSAL_IX_ACCOUNTS_LEN]> for InitializeProposalKeys {
    fn from(pubkeys: [Pubkey; INITIALIZE_PROPOSAL_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            proposal: pubkeys[0],
            squads_proposal: pubkeys[1],
            squads_multisig: pubkeys[2],
            dao: pubkeys[3],
            question: pubkeys[4],
            quote_vault: pubkeys[5],
            base_vault: pubkeys[6],
            proposer: pubkeys[7],
            payer: pubkeys[8],
            system_program: pubkeys[9],
            event_authority: pubkeys[10],
            program: pubkeys[11],
        }
    }
}
impl<'info> From<InitializeProposalAccounts<'_, 'info>>
for [AccountInfo<'info>; INITIALIZE_PROPOSAL_IX_ACCOUNTS_LEN] {
    fn from(accounts: InitializeProposalAccounts<'_, 'info>) -> Self {
        [
            accounts.proposal.clone(),
            accounts.squads_proposal.clone(),
            accounts.squads_multisig.clone(),
            accounts.dao.clone(),
            accounts.question.clone(),
            accounts.quote_vault.clone(),
            accounts.base_vault.clone(),
            accounts.proposer.clone(),
            accounts.payer.clone(),
            accounts.system_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; INITIALIZE_PROPOSAL_IX_ACCOUNTS_LEN]>
for InitializeProposalAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; INITIALIZE_PROPOSAL_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            proposal: &arr[0],
            squads_proposal: &arr[1],
            squads_multisig: &arr[2],
            dao: &arr[3],
            question: &arr[4],
            quote_vault: &arr[5],
            base_vault: &arr[6],
            proposer: &arr[7],
            payer: &arr[8],
            system_program: &arr[9],
            event_authority: &arr[10],
            program: &arr[11],
        }
    }
}
pub const INITIALIZE_PROPOSAL_IX_DISCM: [u8; 8usize] = [
    50, 73, 156, 98, 129, 149, 21, 158,
];
#[derive(Clone, Debug, PartialEq)]
pub struct InitializeProposalIxData;
impl InitializeProposalIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INITIALIZE_PROPOSAL_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INITIALIZE_PROPOSAL_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn initialize_proposal_ix_with_program_id(
    program_id: Pubkey,
    keys: InitializeProposalKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; INITIALIZE_PROPOSAL_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: InitializeProposalIxData.try_to_vec()?,
    })
}
pub fn initialize_proposal_ix(
    keys: InitializeProposalKeys,
) -> std::io::Result<Instruction> {
    initialize_proposal_ix_with_program_id(FUTARCHY_PROGRAM_ID, keys)
}
pub fn initialize_proposal_invoke_with_program_id(
    program_id: Pubkey,
    accounts: InitializeProposalAccounts<'_, '_>,
) -> ProgramResult {
    let keys: InitializeProposalKeys = accounts.into();
    let ix = initialize_proposal_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn initialize_proposal_invoke(
    accounts: InitializeProposalAccounts<'_, '_>,
) -> ProgramResult {
    initialize_proposal_invoke_with_program_id(FUTARCHY_PROGRAM_ID, accounts)
}
pub fn initialize_proposal_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: InitializeProposalAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: InitializeProposalKeys = accounts.into();
    let ix = initialize_proposal_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn initialize_proposal_invoke_signed(
    accounts: InitializeProposalAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    initialize_proposal_invoke_signed_with_program_id(
        FUTARCHY_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn initialize_proposal_verify_account_keys(
    accounts: InitializeProposalAccounts<'_, '_>,
    keys: InitializeProposalKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.proposal.key, keys.proposal),
        (*accounts.squads_proposal.key, keys.squads_proposal),
        (*accounts.squads_multisig.key, keys.squads_multisig),
        (*accounts.dao.key, keys.dao),
        (*accounts.question.key, keys.question),
        (*accounts.quote_vault.key, keys.quote_vault),
        (*accounts.base_vault.key, keys.base_vault),
        (*accounts.proposer.key, keys.proposer),
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
pub fn initialize_proposal_verify_writable_privileges<'me, 'info>(
    accounts: InitializeProposalAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.proposal, accounts.dao, accounts.payer] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn initialize_proposal_verify_signer_privileges<'me, 'info>(
    accounts: InitializeProposalAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.proposer, accounts.payer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn initialize_proposal_verify_account_privileges<'me, 'info>(
    accounts: InitializeProposalAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    initialize_proposal_verify_writable_privileges(accounts)?;
    initialize_proposal_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const STAKE_TO_PROPOSAL_IX_ACCOUNTS_LEN: usize = 11;
#[derive(Copy, Clone, Debug)]
pub struct StakeToProposalAccounts<'me, 'info> {
    pub proposal: &'me AccountInfo<'info>,
    pub dao: &'me AccountInfo<'info>,
    pub staker_base_account: &'me AccountInfo<'info>,
    pub proposal_base_account: &'me AccountInfo<'info>,
    pub stake_account: &'me AccountInfo<'info>,
    pub staker: &'me AccountInfo<'info>,
    pub payer: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct StakeToProposalKeys {
    pub proposal: Pubkey,
    pub dao: Pubkey,
    pub staker_base_account: Pubkey,
    pub proposal_base_account: Pubkey,
    pub stake_account: Pubkey,
    pub staker: Pubkey,
    pub payer: Pubkey,
    pub token_program: Pubkey,
    pub system_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<StakeToProposalAccounts<'_, '_>> for StakeToProposalKeys {
    fn from(accounts: StakeToProposalAccounts) -> Self {
        Self {
            proposal: *accounts.proposal.key,
            dao: *accounts.dao.key,
            staker_base_account: *accounts.staker_base_account.key,
            proposal_base_account: *accounts.proposal_base_account.key,
            stake_account: *accounts.stake_account.key,
            staker: *accounts.staker.key,
            payer: *accounts.payer.key,
            token_program: *accounts.token_program.key,
            system_program: *accounts.system_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<StakeToProposalKeys> for [AccountMeta; STAKE_TO_PROPOSAL_IX_ACCOUNTS_LEN] {
    fn from(keys: StakeToProposalKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.proposal,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.dao,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.staker_base_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.proposal_base_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.stake_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.staker,
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
impl From<[Pubkey; STAKE_TO_PROPOSAL_IX_ACCOUNTS_LEN]> for StakeToProposalKeys {
    fn from(pubkeys: [Pubkey; STAKE_TO_PROPOSAL_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            proposal: pubkeys[0],
            dao: pubkeys[1],
            staker_base_account: pubkeys[2],
            proposal_base_account: pubkeys[3],
            stake_account: pubkeys[4],
            staker: pubkeys[5],
            payer: pubkeys[6],
            token_program: pubkeys[7],
            system_program: pubkeys[8],
            event_authority: pubkeys[9],
            program: pubkeys[10],
        }
    }
}
impl<'info> From<StakeToProposalAccounts<'_, 'info>>
for [AccountInfo<'info>; STAKE_TO_PROPOSAL_IX_ACCOUNTS_LEN] {
    fn from(accounts: StakeToProposalAccounts<'_, 'info>) -> Self {
        [
            accounts.proposal.clone(),
            accounts.dao.clone(),
            accounts.staker_base_account.clone(),
            accounts.proposal_base_account.clone(),
            accounts.stake_account.clone(),
            accounts.staker.clone(),
            accounts.payer.clone(),
            accounts.token_program.clone(),
            accounts.system_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; STAKE_TO_PROPOSAL_IX_ACCOUNTS_LEN]>
for StakeToProposalAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; STAKE_TO_PROPOSAL_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            proposal: &arr[0],
            dao: &arr[1],
            staker_base_account: &arr[2],
            proposal_base_account: &arr[3],
            stake_account: &arr[4],
            staker: &arr[5],
            payer: &arr[6],
            token_program: &arr[7],
            system_program: &arr[8],
            event_authority: &arr[9],
            program: &arr[10],
        }
    }
}
pub const STAKE_TO_PROPOSAL_IX_DISCM: [u8; 8usize] = [
    10, 169, 175, 238, 80, 221, 37, 16,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct StakeToProposalIxArgs {
    pub params: StakeToProposalParams,
}
#[derive(Clone, Debug, PartialEq)]
pub struct StakeToProposalIxData(pub StakeToProposalIxArgs);
impl From<StakeToProposalIxArgs> for StakeToProposalIxData {
    fn from(args: StakeToProposalIxArgs) -> Self {
        Self(args)
    }
}
impl StakeToProposalIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != STAKE_TO_PROPOSAL_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let params = if reader.is_empty() {
            Default::default()
        } else {
            <StakeToProposalParams>::deserialize(&mut reader)?
        };
        Ok(Self(StakeToProposalIxArgs { params }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&STAKE_TO_PROPOSAL_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.params, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn stake_to_proposal_ix_with_program_id(
    program_id: Pubkey,
    keys: StakeToProposalKeys,
    args: StakeToProposalIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; STAKE_TO_PROPOSAL_IX_ACCOUNTS_LEN] = keys.into();
    let data: StakeToProposalIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn stake_to_proposal_ix(
    keys: StakeToProposalKeys,
    args: StakeToProposalIxArgs,
) -> std::io::Result<Instruction> {
    stake_to_proposal_ix_with_program_id(FUTARCHY_PROGRAM_ID, keys, args)
}
pub fn stake_to_proposal_invoke_with_program_id(
    program_id: Pubkey,
    accounts: StakeToProposalAccounts<'_, '_>,
    args: StakeToProposalIxArgs,
) -> ProgramResult {
    let keys: StakeToProposalKeys = accounts.into();
    let ix = stake_to_proposal_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn stake_to_proposal_invoke(
    accounts: StakeToProposalAccounts<'_, '_>,
    args: StakeToProposalIxArgs,
) -> ProgramResult {
    stake_to_proposal_invoke_with_program_id(FUTARCHY_PROGRAM_ID, accounts, args)
}
pub fn stake_to_proposal_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: StakeToProposalAccounts<'_, '_>,
    args: StakeToProposalIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: StakeToProposalKeys = accounts.into();
    let ix = stake_to_proposal_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn stake_to_proposal_invoke_signed(
    accounts: StakeToProposalAccounts<'_, '_>,
    args: StakeToProposalIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    stake_to_proposal_invoke_signed_with_program_id(
        FUTARCHY_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn stake_to_proposal_verify_account_keys(
    accounts: StakeToProposalAccounts<'_, '_>,
    keys: StakeToProposalKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.proposal.key, keys.proposal),
        (*accounts.dao.key, keys.dao),
        (*accounts.staker_base_account.key, keys.staker_base_account),
        (*accounts.proposal_base_account.key, keys.proposal_base_account),
        (*accounts.stake_account.key, keys.stake_account),
        (*accounts.staker.key, keys.staker),
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
pub fn stake_to_proposal_verify_writable_privileges<'me, 'info>(
    accounts: StakeToProposalAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.proposal,
        accounts.dao,
        accounts.staker_base_account,
        accounts.proposal_base_account,
        accounts.stake_account,
        accounts.payer,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn stake_to_proposal_verify_signer_privileges<'me, 'info>(
    accounts: StakeToProposalAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.staker, accounts.payer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn stake_to_proposal_verify_account_privileges<'me, 'info>(
    accounts: StakeToProposalAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    stake_to_proposal_verify_writable_privileges(accounts)?;
    stake_to_proposal_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const UNSTAKE_FROM_PROPOSAL_IX_ACCOUNTS_LEN: usize = 12;
#[derive(Copy, Clone, Debug)]
pub struct UnstakeFromProposalAccounts<'me, 'info> {
    pub proposal: &'me AccountInfo<'info>,
    pub dao: &'me AccountInfo<'info>,
    pub staker_base_account: &'me AccountInfo<'info>,
    pub proposal_base_account: &'me AccountInfo<'info>,
    pub stake_account: &'me AccountInfo<'info>,
    pub base_mint: &'me AccountInfo<'info>,
    pub staker: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UnstakeFromProposalKeys {
    pub proposal: Pubkey,
    pub dao: Pubkey,
    pub staker_base_account: Pubkey,
    pub proposal_base_account: Pubkey,
    pub stake_account: Pubkey,
    pub base_mint: Pubkey,
    pub staker: Pubkey,
    pub token_program: Pubkey,
    pub system_program: Pubkey,
    pub associated_token_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<UnstakeFromProposalAccounts<'_, '_>> for UnstakeFromProposalKeys {
    fn from(accounts: UnstakeFromProposalAccounts) -> Self {
        Self {
            proposal: *accounts.proposal.key,
            dao: *accounts.dao.key,
            staker_base_account: *accounts.staker_base_account.key,
            proposal_base_account: *accounts.proposal_base_account.key,
            stake_account: *accounts.stake_account.key,
            base_mint: *accounts.base_mint.key,
            staker: *accounts.staker.key,
            token_program: *accounts.token_program.key,
            system_program: *accounts.system_program.key,
            associated_token_program: *accounts.associated_token_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<UnstakeFromProposalKeys>
for [AccountMeta; UNSTAKE_FROM_PROPOSAL_IX_ACCOUNTS_LEN] {
    fn from(keys: UnstakeFromProposalKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.proposal,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.dao,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.staker_base_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.proposal_base_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.stake_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.base_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.staker,
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
impl From<[Pubkey; UNSTAKE_FROM_PROPOSAL_IX_ACCOUNTS_LEN]> for UnstakeFromProposalKeys {
    fn from(pubkeys: [Pubkey; UNSTAKE_FROM_PROPOSAL_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            proposal: pubkeys[0],
            dao: pubkeys[1],
            staker_base_account: pubkeys[2],
            proposal_base_account: pubkeys[3],
            stake_account: pubkeys[4],
            base_mint: pubkeys[5],
            staker: pubkeys[6],
            token_program: pubkeys[7],
            system_program: pubkeys[8],
            associated_token_program: pubkeys[9],
            event_authority: pubkeys[10],
            program: pubkeys[11],
        }
    }
}
impl<'info> From<UnstakeFromProposalAccounts<'_, 'info>>
for [AccountInfo<'info>; UNSTAKE_FROM_PROPOSAL_IX_ACCOUNTS_LEN] {
    fn from(accounts: UnstakeFromProposalAccounts<'_, 'info>) -> Self {
        [
            accounts.proposal.clone(),
            accounts.dao.clone(),
            accounts.staker_base_account.clone(),
            accounts.proposal_base_account.clone(),
            accounts.stake_account.clone(),
            accounts.base_mint.clone(),
            accounts.staker.clone(),
            accounts.token_program.clone(),
            accounts.system_program.clone(),
            accounts.associated_token_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; UNSTAKE_FROM_PROPOSAL_IX_ACCOUNTS_LEN]>
for UnstakeFromProposalAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; UNSTAKE_FROM_PROPOSAL_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            proposal: &arr[0],
            dao: &arr[1],
            staker_base_account: &arr[2],
            proposal_base_account: &arr[3],
            stake_account: &arr[4],
            base_mint: &arr[5],
            staker: &arr[6],
            token_program: &arr[7],
            system_program: &arr[8],
            associated_token_program: &arr[9],
            event_authority: &arr[10],
            program: &arr[11],
        }
    }
}
pub const UNSTAKE_FROM_PROPOSAL_IX_DISCM: [u8; 8usize] = [
    179, 220, 186, 86, 2, 96, 50, 161,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UnstakeFromProposalIxArgs {
    pub params: UnstakeFromProposalParams,
}
#[derive(Clone, Debug, PartialEq)]
pub struct UnstakeFromProposalIxData(pub UnstakeFromProposalIxArgs);
impl From<UnstakeFromProposalIxArgs> for UnstakeFromProposalIxData {
    fn from(args: UnstakeFromProposalIxArgs) -> Self {
        Self(args)
    }
}
impl UnstakeFromProposalIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UNSTAKE_FROM_PROPOSAL_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let params = if reader.is_empty() {
            Default::default()
        } else {
            <UnstakeFromProposalParams>::deserialize(&mut reader)?
        };
        Ok(
            Self(UnstakeFromProposalIxArgs {
                params,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UNSTAKE_FROM_PROPOSAL_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.params, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn unstake_from_proposal_ix_with_program_id(
    program_id: Pubkey,
    keys: UnstakeFromProposalKeys,
    args: UnstakeFromProposalIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; UNSTAKE_FROM_PROPOSAL_IX_ACCOUNTS_LEN] = keys.into();
    let data: UnstakeFromProposalIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn unstake_from_proposal_ix(
    keys: UnstakeFromProposalKeys,
    args: UnstakeFromProposalIxArgs,
) -> std::io::Result<Instruction> {
    unstake_from_proposal_ix_with_program_id(FUTARCHY_PROGRAM_ID, keys, args)
}
pub fn unstake_from_proposal_invoke_with_program_id(
    program_id: Pubkey,
    accounts: UnstakeFromProposalAccounts<'_, '_>,
    args: UnstakeFromProposalIxArgs,
) -> ProgramResult {
    let keys: UnstakeFromProposalKeys = accounts.into();
    let ix = unstake_from_proposal_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn unstake_from_proposal_invoke(
    accounts: UnstakeFromProposalAccounts<'_, '_>,
    args: UnstakeFromProposalIxArgs,
) -> ProgramResult {
    unstake_from_proposal_invoke_with_program_id(FUTARCHY_PROGRAM_ID, accounts, args)
}
pub fn unstake_from_proposal_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: UnstakeFromProposalAccounts<'_, '_>,
    args: UnstakeFromProposalIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: UnstakeFromProposalKeys = accounts.into();
    let ix = unstake_from_proposal_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn unstake_from_proposal_invoke_signed(
    accounts: UnstakeFromProposalAccounts<'_, '_>,
    args: UnstakeFromProposalIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    unstake_from_proposal_invoke_signed_with_program_id(
        FUTARCHY_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn unstake_from_proposal_verify_account_keys(
    accounts: UnstakeFromProposalAccounts<'_, '_>,
    keys: UnstakeFromProposalKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.proposal.key, keys.proposal),
        (*accounts.dao.key, keys.dao),
        (*accounts.staker_base_account.key, keys.staker_base_account),
        (*accounts.proposal_base_account.key, keys.proposal_base_account),
        (*accounts.stake_account.key, keys.stake_account),
        (*accounts.base_mint.key, keys.base_mint),
        (*accounts.staker.key, keys.staker),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.system_program.key, keys.system_program),
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
pub fn unstake_from_proposal_verify_writable_privileges<'me, 'info>(
    accounts: UnstakeFromProposalAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.proposal,
        accounts.dao,
        accounts.staker_base_account,
        accounts.proposal_base_account,
        accounts.stake_account,
        accounts.staker,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn unstake_from_proposal_verify_signer_privileges<'me, 'info>(
    accounts: UnstakeFromProposalAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.staker] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn unstake_from_proposal_verify_account_privileges<'me, 'info>(
    accounts: UnstakeFromProposalAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    unstake_from_proposal_verify_writable_privileges(accounts)?;
    unstake_from_proposal_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const LAUNCH_PROPOSAL_IX_ACCOUNTS_LEN: usize = 20;
#[derive(Copy, Clone, Debug)]
pub struct LaunchProposalAccounts<'me, 'info> {
    pub proposal: &'me AccountInfo<'info>,
    pub base_vault: &'me AccountInfo<'info>,
    pub quote_vault: &'me AccountInfo<'info>,
    pub pass_base_mint: &'me AccountInfo<'info>,
    pub pass_quote_mint: &'me AccountInfo<'info>,
    pub fail_base_mint: &'me AccountInfo<'info>,
    pub fail_quote_mint: &'me AccountInfo<'info>,
    pub dao: &'me AccountInfo<'info>,
    pub payer: &'me AccountInfo<'info>,
    pub amm_pass_base_vault: &'me AccountInfo<'info>,
    pub amm_pass_quote_vault: &'me AccountInfo<'info>,
    pub amm_fail_base_vault: &'me AccountInfo<'info>,
    pub amm_fail_quote_vault: &'me AccountInfo<'info>,
    pub squads_multisig: &'me AccountInfo<'info>,
    pub squads_proposal: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LaunchProposalKeys {
    pub proposal: Pubkey,
    pub base_vault: Pubkey,
    pub quote_vault: Pubkey,
    pub pass_base_mint: Pubkey,
    pub pass_quote_mint: Pubkey,
    pub fail_base_mint: Pubkey,
    pub fail_quote_mint: Pubkey,
    pub dao: Pubkey,
    pub payer: Pubkey,
    pub amm_pass_base_vault: Pubkey,
    pub amm_pass_quote_vault: Pubkey,
    pub amm_fail_base_vault: Pubkey,
    pub amm_fail_quote_vault: Pubkey,
    pub squads_multisig: Pubkey,
    pub squads_proposal: Pubkey,
    pub system_program: Pubkey,
    pub token_program: Pubkey,
    pub associated_token_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<LaunchProposalAccounts<'_, '_>> for LaunchProposalKeys {
    fn from(accounts: LaunchProposalAccounts) -> Self {
        Self {
            proposal: *accounts.proposal.key,
            base_vault: *accounts.base_vault.key,
            quote_vault: *accounts.quote_vault.key,
            pass_base_mint: *accounts.pass_base_mint.key,
            pass_quote_mint: *accounts.pass_quote_mint.key,
            fail_base_mint: *accounts.fail_base_mint.key,
            fail_quote_mint: *accounts.fail_quote_mint.key,
            dao: *accounts.dao.key,
            payer: *accounts.payer.key,
            amm_pass_base_vault: *accounts.amm_pass_base_vault.key,
            amm_pass_quote_vault: *accounts.amm_pass_quote_vault.key,
            amm_fail_base_vault: *accounts.amm_fail_base_vault.key,
            amm_fail_quote_vault: *accounts.amm_fail_quote_vault.key,
            squads_multisig: *accounts.squads_multisig.key,
            squads_proposal: *accounts.squads_proposal.key,
            system_program: *accounts.system_program.key,
            token_program: *accounts.token_program.key,
            associated_token_program: *accounts.associated_token_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<LaunchProposalKeys> for [AccountMeta; LAUNCH_PROPOSAL_IX_ACCOUNTS_LEN] {
    fn from(keys: LaunchProposalKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.proposal,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.base_vault,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.quote_vault,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pass_base_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pass_quote_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.fail_base_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.fail_quote_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.dao,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.payer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.amm_pass_base_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.amm_pass_quote_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.amm_fail_base_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.amm_fail_quote_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.squads_multisig,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.squads_proposal,
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
impl From<[Pubkey; LAUNCH_PROPOSAL_IX_ACCOUNTS_LEN]> for LaunchProposalKeys {
    fn from(pubkeys: [Pubkey; LAUNCH_PROPOSAL_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            proposal: pubkeys[0],
            base_vault: pubkeys[1],
            quote_vault: pubkeys[2],
            pass_base_mint: pubkeys[3],
            pass_quote_mint: pubkeys[4],
            fail_base_mint: pubkeys[5],
            fail_quote_mint: pubkeys[6],
            dao: pubkeys[7],
            payer: pubkeys[8],
            amm_pass_base_vault: pubkeys[9],
            amm_pass_quote_vault: pubkeys[10],
            amm_fail_base_vault: pubkeys[11],
            amm_fail_quote_vault: pubkeys[12],
            squads_multisig: pubkeys[13],
            squads_proposal: pubkeys[14],
            system_program: pubkeys[15],
            token_program: pubkeys[16],
            associated_token_program: pubkeys[17],
            event_authority: pubkeys[18],
            program: pubkeys[19],
        }
    }
}
impl<'info> From<LaunchProposalAccounts<'_, 'info>>
for [AccountInfo<'info>; LAUNCH_PROPOSAL_IX_ACCOUNTS_LEN] {
    fn from(accounts: LaunchProposalAccounts<'_, 'info>) -> Self {
        [
            accounts.proposal.clone(),
            accounts.base_vault.clone(),
            accounts.quote_vault.clone(),
            accounts.pass_base_mint.clone(),
            accounts.pass_quote_mint.clone(),
            accounts.fail_base_mint.clone(),
            accounts.fail_quote_mint.clone(),
            accounts.dao.clone(),
            accounts.payer.clone(),
            accounts.amm_pass_base_vault.clone(),
            accounts.amm_pass_quote_vault.clone(),
            accounts.amm_fail_base_vault.clone(),
            accounts.amm_fail_quote_vault.clone(),
            accounts.squads_multisig.clone(),
            accounts.squads_proposal.clone(),
            accounts.system_program.clone(),
            accounts.token_program.clone(),
            accounts.associated_token_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; LAUNCH_PROPOSAL_IX_ACCOUNTS_LEN]>
for LaunchProposalAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; LAUNCH_PROPOSAL_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            proposal: &arr[0],
            base_vault: &arr[1],
            quote_vault: &arr[2],
            pass_base_mint: &arr[3],
            pass_quote_mint: &arr[4],
            fail_base_mint: &arr[5],
            fail_quote_mint: &arr[6],
            dao: &arr[7],
            payer: &arr[8],
            amm_pass_base_vault: &arr[9],
            amm_pass_quote_vault: &arr[10],
            amm_fail_base_vault: &arr[11],
            amm_fail_quote_vault: &arr[12],
            squads_multisig: &arr[13],
            squads_proposal: &arr[14],
            system_program: &arr[15],
            token_program: &arr[16],
            associated_token_program: &arr[17],
            event_authority: &arr[18],
            program: &arr[19],
        }
    }
}
pub const LAUNCH_PROPOSAL_IX_DISCM: [u8; 8usize] = [16, 211, 189, 119, 245, 72, 0, 229];
#[derive(Clone, Debug, PartialEq)]
pub struct LaunchProposalIxData;
impl LaunchProposalIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != LAUNCH_PROPOSAL_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&LAUNCH_PROPOSAL_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn launch_proposal_ix_with_program_id(
    program_id: Pubkey,
    keys: LaunchProposalKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; LAUNCH_PROPOSAL_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: LaunchProposalIxData.try_to_vec()?,
    })
}
pub fn launch_proposal_ix(keys: LaunchProposalKeys) -> std::io::Result<Instruction> {
    launch_proposal_ix_with_program_id(FUTARCHY_PROGRAM_ID, keys)
}
pub fn launch_proposal_invoke_with_program_id(
    program_id: Pubkey,
    accounts: LaunchProposalAccounts<'_, '_>,
) -> ProgramResult {
    let keys: LaunchProposalKeys = accounts.into();
    let ix = launch_proposal_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn launch_proposal_invoke(
    accounts: LaunchProposalAccounts<'_, '_>,
) -> ProgramResult {
    launch_proposal_invoke_with_program_id(FUTARCHY_PROGRAM_ID, accounts)
}
pub fn launch_proposal_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: LaunchProposalAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: LaunchProposalKeys = accounts.into();
    let ix = launch_proposal_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn launch_proposal_invoke_signed(
    accounts: LaunchProposalAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    launch_proposal_invoke_signed_with_program_id(FUTARCHY_PROGRAM_ID, accounts, seeds)
}
pub fn launch_proposal_verify_account_keys(
    accounts: LaunchProposalAccounts<'_, '_>,
    keys: LaunchProposalKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.proposal.key, keys.proposal),
        (*accounts.base_vault.key, keys.base_vault),
        (*accounts.quote_vault.key, keys.quote_vault),
        (*accounts.pass_base_mint.key, keys.pass_base_mint),
        (*accounts.pass_quote_mint.key, keys.pass_quote_mint),
        (*accounts.fail_base_mint.key, keys.fail_base_mint),
        (*accounts.fail_quote_mint.key, keys.fail_quote_mint),
        (*accounts.dao.key, keys.dao),
        (*accounts.payer.key, keys.payer),
        (*accounts.amm_pass_base_vault.key, keys.amm_pass_base_vault),
        (*accounts.amm_pass_quote_vault.key, keys.amm_pass_quote_vault),
        (*accounts.amm_fail_base_vault.key, keys.amm_fail_base_vault),
        (*accounts.amm_fail_quote_vault.key, keys.amm_fail_quote_vault),
        (*accounts.squads_multisig.key, keys.squads_multisig),
        (*accounts.squads_proposal.key, keys.squads_proposal),
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
pub fn launch_proposal_verify_writable_privileges<'me, 'info>(
    accounts: LaunchProposalAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.proposal,
        accounts.dao,
        accounts.payer,
        accounts.amm_pass_base_vault,
        accounts.amm_pass_quote_vault,
        accounts.amm_fail_base_vault,
        accounts.amm_fail_quote_vault,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn launch_proposal_verify_signer_privileges<'me, 'info>(
    accounts: LaunchProposalAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.payer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn launch_proposal_verify_account_privileges<'me, 'info>(
    accounts: LaunchProposalAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    launch_proposal_verify_writable_privileges(accounts)?;
    launch_proposal_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const FINALIZE_PROPOSAL_IX_ACCOUNTS_LEN: usize = 25;
#[derive(Copy, Clone, Debug)]
pub struct FinalizeProposalAccounts<'me, 'info> {
    pub proposal: &'me AccountInfo<'info>,
    pub dao: &'me AccountInfo<'info>,
    pub question: &'me AccountInfo<'info>,
    pub squads_proposal: &'me AccountInfo<'info>,
    pub squads_multisig: &'me AccountInfo<'info>,
    pub squads_multisig_program: &'me AccountInfo<'info>,
    pub amm_pass_base_vault: &'me AccountInfo<'info>,
    pub amm_pass_quote_vault: &'me AccountInfo<'info>,
    pub amm_fail_base_vault: &'me AccountInfo<'info>,
    pub amm_fail_quote_vault: &'me AccountInfo<'info>,
    pub amm_base_vault: &'me AccountInfo<'info>,
    pub amm_quote_vault: &'me AccountInfo<'info>,
    pub vault_program: &'me AccountInfo<'info>,
    pub vault_event_authority: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub quote_vault: &'me AccountInfo<'info>,
    pub quote_vault_underlying_token_account: &'me AccountInfo<'info>,
    pub pass_quote_mint: &'me AccountInfo<'info>,
    pub fail_quote_mint: &'me AccountInfo<'info>,
    pub pass_base_mint: &'me AccountInfo<'info>,
    pub fail_base_mint: &'me AccountInfo<'info>,
    pub base_vault: &'me AccountInfo<'info>,
    pub base_vault_underlying_token_account: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FinalizeProposalKeys {
    pub proposal: Pubkey,
    pub dao: Pubkey,
    pub question: Pubkey,
    pub squads_proposal: Pubkey,
    pub squads_multisig: Pubkey,
    pub squads_multisig_program: Pubkey,
    pub amm_pass_base_vault: Pubkey,
    pub amm_pass_quote_vault: Pubkey,
    pub amm_fail_base_vault: Pubkey,
    pub amm_fail_quote_vault: Pubkey,
    pub amm_base_vault: Pubkey,
    pub amm_quote_vault: Pubkey,
    pub vault_program: Pubkey,
    pub vault_event_authority: Pubkey,
    pub token_program: Pubkey,
    pub quote_vault: Pubkey,
    pub quote_vault_underlying_token_account: Pubkey,
    pub pass_quote_mint: Pubkey,
    pub fail_quote_mint: Pubkey,
    pub pass_base_mint: Pubkey,
    pub fail_base_mint: Pubkey,
    pub base_vault: Pubkey,
    pub base_vault_underlying_token_account: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<FinalizeProposalAccounts<'_, '_>> for FinalizeProposalKeys {
    fn from(accounts: FinalizeProposalAccounts) -> Self {
        Self {
            proposal: *accounts.proposal.key,
            dao: *accounts.dao.key,
            question: *accounts.question.key,
            squads_proposal: *accounts.squads_proposal.key,
            squads_multisig: *accounts.squads_multisig.key,
            squads_multisig_program: *accounts.squads_multisig_program.key,
            amm_pass_base_vault: *accounts.amm_pass_base_vault.key,
            amm_pass_quote_vault: *accounts.amm_pass_quote_vault.key,
            amm_fail_base_vault: *accounts.amm_fail_base_vault.key,
            amm_fail_quote_vault: *accounts.amm_fail_quote_vault.key,
            amm_base_vault: *accounts.amm_base_vault.key,
            amm_quote_vault: *accounts.amm_quote_vault.key,
            vault_program: *accounts.vault_program.key,
            vault_event_authority: *accounts.vault_event_authority.key,
            token_program: *accounts.token_program.key,
            quote_vault: *accounts.quote_vault.key,
            quote_vault_underlying_token_account: *accounts
                .quote_vault_underlying_token_account
                .key,
            pass_quote_mint: *accounts.pass_quote_mint.key,
            fail_quote_mint: *accounts.fail_quote_mint.key,
            pass_base_mint: *accounts.pass_base_mint.key,
            fail_base_mint: *accounts.fail_base_mint.key,
            base_vault: *accounts.base_vault.key,
            base_vault_underlying_token_account: *accounts
                .base_vault_underlying_token_account
                .key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<FinalizeProposalKeys> for [AccountMeta; FINALIZE_PROPOSAL_IX_ACCOUNTS_LEN] {
    fn from(keys: FinalizeProposalKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.proposal,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.dao,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.question,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.squads_proposal,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.squads_multisig,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.squads_multisig_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.amm_pass_base_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.amm_pass_quote_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.amm_fail_base_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.amm_fail_quote_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.amm_base_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.amm_quote_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vault_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault_event_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.quote_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.quote_vault_underlying_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pass_quote_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.fail_quote_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pass_base_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.fail_base_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.base_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.base_vault_underlying_token_account,
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
impl From<[Pubkey; FINALIZE_PROPOSAL_IX_ACCOUNTS_LEN]> for FinalizeProposalKeys {
    fn from(pubkeys: [Pubkey; FINALIZE_PROPOSAL_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            proposal: pubkeys[0],
            dao: pubkeys[1],
            question: pubkeys[2],
            squads_proposal: pubkeys[3],
            squads_multisig: pubkeys[4],
            squads_multisig_program: pubkeys[5],
            amm_pass_base_vault: pubkeys[6],
            amm_pass_quote_vault: pubkeys[7],
            amm_fail_base_vault: pubkeys[8],
            amm_fail_quote_vault: pubkeys[9],
            amm_base_vault: pubkeys[10],
            amm_quote_vault: pubkeys[11],
            vault_program: pubkeys[12],
            vault_event_authority: pubkeys[13],
            token_program: pubkeys[14],
            quote_vault: pubkeys[15],
            quote_vault_underlying_token_account: pubkeys[16],
            pass_quote_mint: pubkeys[17],
            fail_quote_mint: pubkeys[18],
            pass_base_mint: pubkeys[19],
            fail_base_mint: pubkeys[20],
            base_vault: pubkeys[21],
            base_vault_underlying_token_account: pubkeys[22],
            event_authority: pubkeys[23],
            program: pubkeys[24],
        }
    }
}
impl<'info> From<FinalizeProposalAccounts<'_, 'info>>
for [AccountInfo<'info>; FINALIZE_PROPOSAL_IX_ACCOUNTS_LEN] {
    fn from(accounts: FinalizeProposalAccounts<'_, 'info>) -> Self {
        [
            accounts.proposal.clone(),
            accounts.dao.clone(),
            accounts.question.clone(),
            accounts.squads_proposal.clone(),
            accounts.squads_multisig.clone(),
            accounts.squads_multisig_program.clone(),
            accounts.amm_pass_base_vault.clone(),
            accounts.amm_pass_quote_vault.clone(),
            accounts.amm_fail_base_vault.clone(),
            accounts.amm_fail_quote_vault.clone(),
            accounts.amm_base_vault.clone(),
            accounts.amm_quote_vault.clone(),
            accounts.vault_program.clone(),
            accounts.vault_event_authority.clone(),
            accounts.token_program.clone(),
            accounts.quote_vault.clone(),
            accounts.quote_vault_underlying_token_account.clone(),
            accounts.pass_quote_mint.clone(),
            accounts.fail_quote_mint.clone(),
            accounts.pass_base_mint.clone(),
            accounts.fail_base_mint.clone(),
            accounts.base_vault.clone(),
            accounts.base_vault_underlying_token_account.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; FINALIZE_PROPOSAL_IX_ACCOUNTS_LEN]>
for FinalizeProposalAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; FINALIZE_PROPOSAL_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            proposal: &arr[0],
            dao: &arr[1],
            question: &arr[2],
            squads_proposal: &arr[3],
            squads_multisig: &arr[4],
            squads_multisig_program: &arr[5],
            amm_pass_base_vault: &arr[6],
            amm_pass_quote_vault: &arr[7],
            amm_fail_base_vault: &arr[8],
            amm_fail_quote_vault: &arr[9],
            amm_base_vault: &arr[10],
            amm_quote_vault: &arr[11],
            vault_program: &arr[12],
            vault_event_authority: &arr[13],
            token_program: &arr[14],
            quote_vault: &arr[15],
            quote_vault_underlying_token_account: &arr[16],
            pass_quote_mint: &arr[17],
            fail_quote_mint: &arr[18],
            pass_base_mint: &arr[19],
            fail_base_mint: &arr[20],
            base_vault: &arr[21],
            base_vault_underlying_token_account: &arr[22],
            event_authority: &arr[23],
            program: &arr[24],
        }
    }
}
pub const FINALIZE_PROPOSAL_IX_DISCM: [u8; 8usize] = [
    23, 68, 51, 167, 109, 173, 187, 164,
];
#[derive(Clone, Debug, PartialEq)]
pub struct FinalizeProposalIxData;
impl FinalizeProposalIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != FINALIZE_PROPOSAL_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&FINALIZE_PROPOSAL_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn finalize_proposal_ix_with_program_id(
    program_id: Pubkey,
    keys: FinalizeProposalKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; FINALIZE_PROPOSAL_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: FinalizeProposalIxData.try_to_vec()?,
    })
}
pub fn finalize_proposal_ix(keys: FinalizeProposalKeys) -> std::io::Result<Instruction> {
    finalize_proposal_ix_with_program_id(FUTARCHY_PROGRAM_ID, keys)
}
pub fn finalize_proposal_invoke_with_program_id(
    program_id: Pubkey,
    accounts: FinalizeProposalAccounts<'_, '_>,
) -> ProgramResult {
    let keys: FinalizeProposalKeys = accounts.into();
    let ix = finalize_proposal_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn finalize_proposal_invoke(
    accounts: FinalizeProposalAccounts<'_, '_>,
) -> ProgramResult {
    finalize_proposal_invoke_with_program_id(FUTARCHY_PROGRAM_ID, accounts)
}
pub fn finalize_proposal_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: FinalizeProposalAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: FinalizeProposalKeys = accounts.into();
    let ix = finalize_proposal_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn finalize_proposal_invoke_signed(
    accounts: FinalizeProposalAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    finalize_proposal_invoke_signed_with_program_id(FUTARCHY_PROGRAM_ID, accounts, seeds)
}
pub fn finalize_proposal_verify_account_keys(
    accounts: FinalizeProposalAccounts<'_, '_>,
    keys: FinalizeProposalKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.proposal.key, keys.proposal),
        (*accounts.dao.key, keys.dao),
        (*accounts.question.key, keys.question),
        (*accounts.squads_proposal.key, keys.squads_proposal),
        (*accounts.squads_multisig.key, keys.squads_multisig),
        (*accounts.squads_multisig_program.key, keys.squads_multisig_program),
        (*accounts.amm_pass_base_vault.key, keys.amm_pass_base_vault),
        (*accounts.amm_pass_quote_vault.key, keys.amm_pass_quote_vault),
        (*accounts.amm_fail_base_vault.key, keys.amm_fail_base_vault),
        (*accounts.amm_fail_quote_vault.key, keys.amm_fail_quote_vault),
        (*accounts.amm_base_vault.key, keys.amm_base_vault),
        (*accounts.amm_quote_vault.key, keys.amm_quote_vault),
        (*accounts.vault_program.key, keys.vault_program),
        (*accounts.vault_event_authority.key, keys.vault_event_authority),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.quote_vault.key, keys.quote_vault),
        (
            *accounts.quote_vault_underlying_token_account.key,
            keys.quote_vault_underlying_token_account,
        ),
        (*accounts.pass_quote_mint.key, keys.pass_quote_mint),
        (*accounts.fail_quote_mint.key, keys.fail_quote_mint),
        (*accounts.pass_base_mint.key, keys.pass_base_mint),
        (*accounts.fail_base_mint.key, keys.fail_base_mint),
        (*accounts.base_vault.key, keys.base_vault),
        (
            *accounts.base_vault_underlying_token_account.key,
            keys.base_vault_underlying_token_account,
        ),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn finalize_proposal_verify_writable_privileges<'me, 'info>(
    accounts: FinalizeProposalAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.proposal,
        accounts.dao,
        accounts.question,
        accounts.squads_proposal,
        accounts.amm_pass_base_vault,
        accounts.amm_pass_quote_vault,
        accounts.amm_fail_base_vault,
        accounts.amm_fail_quote_vault,
        accounts.amm_base_vault,
        accounts.amm_quote_vault,
        accounts.quote_vault,
        accounts.quote_vault_underlying_token_account,
        accounts.pass_quote_mint,
        accounts.fail_quote_mint,
        accounts.pass_base_mint,
        accounts.fail_base_mint,
        accounts.base_vault,
        accounts.base_vault_underlying_token_account,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn finalize_proposal_verify_account_privileges<'me, 'info>(
    accounts: FinalizeProposalAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    finalize_proposal_verify_writable_privileges(accounts)?;
    Ok(())
}
pub const UPDATE_DAO_IX_ACCOUNTS_LEN: usize = 4;
#[derive(Copy, Clone, Debug)]
pub struct UpdateDaoAccounts<'me, 'info> {
    pub dao: &'me AccountInfo<'info>,
    pub squads_multisig_vault: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UpdateDaoKeys {
    pub dao: Pubkey,
    pub squads_multisig_vault: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<UpdateDaoAccounts<'_, '_>> for UpdateDaoKeys {
    fn from(accounts: UpdateDaoAccounts) -> Self {
        Self {
            dao: *accounts.dao.key,
            squads_multisig_vault: *accounts.squads_multisig_vault.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<UpdateDaoKeys> for [AccountMeta; UPDATE_DAO_IX_ACCOUNTS_LEN] {
    fn from(keys: UpdateDaoKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.dao,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.squads_multisig_vault,
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
impl From<[Pubkey; UPDATE_DAO_IX_ACCOUNTS_LEN]> for UpdateDaoKeys {
    fn from(pubkeys: [Pubkey; UPDATE_DAO_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            dao: pubkeys[0],
            squads_multisig_vault: pubkeys[1],
            event_authority: pubkeys[2],
            program: pubkeys[3],
        }
    }
}
impl<'info> From<UpdateDaoAccounts<'_, 'info>>
for [AccountInfo<'info>; UPDATE_DAO_IX_ACCOUNTS_LEN] {
    fn from(accounts: UpdateDaoAccounts<'_, 'info>) -> Self {
        [
            accounts.dao.clone(),
            accounts.squads_multisig_vault.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; UPDATE_DAO_IX_ACCOUNTS_LEN]>
for UpdateDaoAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; UPDATE_DAO_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            dao: &arr[0],
            squads_multisig_vault: &arr[1],
            event_authority: &arr[2],
            program: &arr[3],
        }
    }
}
pub const UPDATE_DAO_IX_DISCM: [u8; 8usize] = [131, 72, 75, 25, 112, 210, 109, 2];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UpdateDaoIxArgs {
    pub dao_params: UpdateDaoParams,
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateDaoIxData(pub UpdateDaoIxArgs);
impl From<UpdateDaoIxArgs> for UpdateDaoIxData {
    fn from(args: UpdateDaoIxArgs) -> Self {
        Self(args)
    }
}
impl UpdateDaoIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_DAO_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let dao_params = if reader.is_empty() {
            Default::default()
        } else {
            <UpdateDaoParams>::deserialize(&mut reader)?
        };
        Ok(Self(UpdateDaoIxArgs { dao_params }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_DAO_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.dao_params, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn update_dao_ix_with_program_id(
    program_id: Pubkey,
    keys: UpdateDaoKeys,
    args: UpdateDaoIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; UPDATE_DAO_IX_ACCOUNTS_LEN] = keys.into();
    let data: UpdateDaoIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_dao_ix(
    keys: UpdateDaoKeys,
    args: UpdateDaoIxArgs,
) -> std::io::Result<Instruction> {
    update_dao_ix_with_program_id(FUTARCHY_PROGRAM_ID, keys, args)
}
pub fn update_dao_invoke_with_program_id(
    program_id: Pubkey,
    accounts: UpdateDaoAccounts<'_, '_>,
    args: UpdateDaoIxArgs,
) -> ProgramResult {
    let keys: UpdateDaoKeys = accounts.into();
    let ix = update_dao_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn update_dao_invoke(
    accounts: UpdateDaoAccounts<'_, '_>,
    args: UpdateDaoIxArgs,
) -> ProgramResult {
    update_dao_invoke_with_program_id(FUTARCHY_PROGRAM_ID, accounts, args)
}
pub fn update_dao_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: UpdateDaoAccounts<'_, '_>,
    args: UpdateDaoIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: UpdateDaoKeys = accounts.into();
    let ix = update_dao_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn update_dao_invoke_signed(
    accounts: UpdateDaoAccounts<'_, '_>,
    args: UpdateDaoIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    update_dao_invoke_signed_with_program_id(FUTARCHY_PROGRAM_ID, accounts, args, seeds)
}
pub fn update_dao_verify_account_keys(
    accounts: UpdateDaoAccounts<'_, '_>,
    keys: UpdateDaoKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.dao.key, keys.dao),
        (*accounts.squads_multisig_vault.key, keys.squads_multisig_vault),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn update_dao_verify_writable_privileges<'me, 'info>(
    accounts: UpdateDaoAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.dao] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn update_dao_verify_signer_privileges<'me, 'info>(
    accounts: UpdateDaoAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.squads_multisig_vault] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn update_dao_verify_account_privileges<'me, 'info>(
    accounts: UpdateDaoAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    update_dao_verify_writable_privileges(accounts)?;
    update_dao_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const RESIZE_DAO_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct ResizeDaoAccounts<'me, 'info> {
    pub dao: &'me AccountInfo<'info>,
    pub payer: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ResizeDaoKeys {
    pub dao: Pubkey,
    pub payer: Pubkey,
    pub system_program: Pubkey,
}
impl From<ResizeDaoAccounts<'_, '_>> for ResizeDaoKeys {
    fn from(accounts: ResizeDaoAccounts) -> Self {
        Self {
            dao: *accounts.dao.key,
            payer: *accounts.payer.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<ResizeDaoKeys> for [AccountMeta; RESIZE_DAO_IX_ACCOUNTS_LEN] {
    fn from(keys: ResizeDaoKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.dao,
                is_signer: false,
                is_writable: true,
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
impl From<[Pubkey; RESIZE_DAO_IX_ACCOUNTS_LEN]> for ResizeDaoKeys {
    fn from(pubkeys: [Pubkey; RESIZE_DAO_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            dao: pubkeys[0],
            payer: pubkeys[1],
            system_program: pubkeys[2],
        }
    }
}
impl<'info> From<ResizeDaoAccounts<'_, 'info>>
for [AccountInfo<'info>; RESIZE_DAO_IX_ACCOUNTS_LEN] {
    fn from(accounts: ResizeDaoAccounts<'_, 'info>) -> Self {
        [accounts.dao.clone(), accounts.payer.clone(), accounts.system_program.clone()]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; RESIZE_DAO_IX_ACCOUNTS_LEN]>
for ResizeDaoAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; RESIZE_DAO_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            dao: &arr[0],
            payer: &arr[1],
            system_program: &arr[2],
        }
    }
}
pub const RESIZE_DAO_IX_DISCM: [u8; 8usize] = [142, 52, 68, 81, 113, 14, 90, 40];
#[derive(Clone, Debug, PartialEq)]
pub struct ResizeDaoIxData;
impl ResizeDaoIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != RESIZE_DAO_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&RESIZE_DAO_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn resize_dao_ix_with_program_id(
    program_id: Pubkey,
    keys: ResizeDaoKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; RESIZE_DAO_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: ResizeDaoIxData.try_to_vec()?,
    })
}
pub fn resize_dao_ix(keys: ResizeDaoKeys) -> std::io::Result<Instruction> {
    resize_dao_ix_with_program_id(FUTARCHY_PROGRAM_ID, keys)
}
pub fn resize_dao_invoke_with_program_id(
    program_id: Pubkey,
    accounts: ResizeDaoAccounts<'_, '_>,
) -> ProgramResult {
    let keys: ResizeDaoKeys = accounts.into();
    let ix = resize_dao_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn resize_dao_invoke(accounts: ResizeDaoAccounts<'_, '_>) -> ProgramResult {
    resize_dao_invoke_with_program_id(FUTARCHY_PROGRAM_ID, accounts)
}
pub fn resize_dao_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: ResizeDaoAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: ResizeDaoKeys = accounts.into();
    let ix = resize_dao_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn resize_dao_invoke_signed(
    accounts: ResizeDaoAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    resize_dao_invoke_signed_with_program_id(FUTARCHY_PROGRAM_ID, accounts, seeds)
}
pub fn resize_dao_verify_account_keys(
    accounts: ResizeDaoAccounts<'_, '_>,
    keys: ResizeDaoKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.dao.key, keys.dao),
        (*accounts.payer.key, keys.payer),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn resize_dao_verify_writable_privileges<'me, 'info>(
    accounts: ResizeDaoAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.dao, accounts.payer] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn resize_dao_verify_signer_privileges<'me, 'info>(
    accounts: ResizeDaoAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.payer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn resize_dao_verify_account_privileges<'me, 'info>(
    accounts: ResizeDaoAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    resize_dao_verify_writable_privileges(accounts)?;
    resize_dao_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SPOT_SWAP_IX_ACCOUNTS_LEN: usize = 9;
#[derive(Copy, Clone, Debug)]
pub struct SpotSwapAccounts<'me, 'info> {
    pub dao: &'me AccountInfo<'info>,
    pub user_base_account: &'me AccountInfo<'info>,
    pub user_quote_account: &'me AccountInfo<'info>,
    pub amm_base_vault: &'me AccountInfo<'info>,
    pub amm_quote_vault: &'me AccountInfo<'info>,
    pub user: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SpotSwapKeys {
    pub dao: Pubkey,
    pub user_base_account: Pubkey,
    pub user_quote_account: Pubkey,
    pub amm_base_vault: Pubkey,
    pub amm_quote_vault: Pubkey,
    pub user: Pubkey,
    pub token_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<SpotSwapAccounts<'_, '_>> for SpotSwapKeys {
    fn from(accounts: SpotSwapAccounts) -> Self {
        Self {
            dao: *accounts.dao.key,
            user_base_account: *accounts.user_base_account.key,
            user_quote_account: *accounts.user_quote_account.key,
            amm_base_vault: *accounts.amm_base_vault.key,
            amm_quote_vault: *accounts.amm_quote_vault.key,
            user: *accounts.user.key,
            token_program: *accounts.token_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<SpotSwapKeys> for [AccountMeta; SPOT_SWAP_IX_ACCOUNTS_LEN] {
    fn from(keys: SpotSwapKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.dao,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_base_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_quote_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.amm_base_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.amm_quote_vault,
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
impl From<[Pubkey; SPOT_SWAP_IX_ACCOUNTS_LEN]> for SpotSwapKeys {
    fn from(pubkeys: [Pubkey; SPOT_SWAP_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            dao: pubkeys[0],
            user_base_account: pubkeys[1],
            user_quote_account: pubkeys[2],
            amm_base_vault: pubkeys[3],
            amm_quote_vault: pubkeys[4],
            user: pubkeys[5],
            token_program: pubkeys[6],
            event_authority: pubkeys[7],
            program: pubkeys[8],
        }
    }
}
impl<'info> From<SpotSwapAccounts<'_, 'info>>
for [AccountInfo<'info>; SPOT_SWAP_IX_ACCOUNTS_LEN] {
    fn from(accounts: SpotSwapAccounts<'_, 'info>) -> Self {
        [
            accounts.dao.clone(),
            accounts.user_base_account.clone(),
            accounts.user_quote_account.clone(),
            accounts.amm_base_vault.clone(),
            accounts.amm_quote_vault.clone(),
            accounts.user.clone(),
            accounts.token_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SPOT_SWAP_IX_ACCOUNTS_LEN]>
for SpotSwapAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; SPOT_SWAP_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            dao: &arr[0],
            user_base_account: &arr[1],
            user_quote_account: &arr[2],
            amm_base_vault: &arr[3],
            amm_quote_vault: &arr[4],
            user: &arr[5],
            token_program: &arr[6],
            event_authority: &arr[7],
            program: &arr[8],
        }
    }
}
pub const SPOT_SWAP_IX_DISCM: [u8; 8usize] = [167, 97, 12, 231, 237, 78, 166, 251];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SpotSwapIxArgs {
    pub params: SpotSwapParams,
}
#[derive(Clone, Debug, PartialEq)]
pub struct SpotSwapIxData(pub SpotSwapIxArgs);
impl From<SpotSwapIxArgs> for SpotSwapIxData {
    fn from(args: SpotSwapIxArgs) -> Self {
        Self(args)
    }
}
impl SpotSwapIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SPOT_SWAP_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let params = if reader.is_empty() {
            Default::default()
        } else {
            <SpotSwapParams>::deserialize(&mut reader)?
        };
        Ok(Self(SpotSwapIxArgs { params }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SPOT_SWAP_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.params, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn spot_swap_ix_with_program_id(
    program_id: Pubkey,
    keys: SpotSwapKeys,
    args: SpotSwapIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SPOT_SWAP_IX_ACCOUNTS_LEN] = keys.into();
    let data: SpotSwapIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn spot_swap_ix(
    keys: SpotSwapKeys,
    args: SpotSwapIxArgs,
) -> std::io::Result<Instruction> {
    spot_swap_ix_with_program_id(FUTARCHY_PROGRAM_ID, keys, args)
}
pub fn spot_swap_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SpotSwapAccounts<'_, '_>,
    args: SpotSwapIxArgs,
) -> ProgramResult {
    let keys: SpotSwapKeys = accounts.into();
    let ix = spot_swap_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn spot_swap_invoke(
    accounts: SpotSwapAccounts<'_, '_>,
    args: SpotSwapIxArgs,
) -> ProgramResult {
    spot_swap_invoke_with_program_id(FUTARCHY_PROGRAM_ID, accounts, args)
}
pub fn spot_swap_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SpotSwapAccounts<'_, '_>,
    args: SpotSwapIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SpotSwapKeys = accounts.into();
    let ix = spot_swap_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn spot_swap_invoke_signed(
    accounts: SpotSwapAccounts<'_, '_>,
    args: SpotSwapIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    spot_swap_invoke_signed_with_program_id(FUTARCHY_PROGRAM_ID, accounts, args, seeds)
}
pub fn spot_swap_verify_account_keys(
    accounts: SpotSwapAccounts<'_, '_>,
    keys: SpotSwapKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.dao.key, keys.dao),
        (*accounts.user_base_account.key, keys.user_base_account),
        (*accounts.user_quote_account.key, keys.user_quote_account),
        (*accounts.amm_base_vault.key, keys.amm_base_vault),
        (*accounts.amm_quote_vault.key, keys.amm_quote_vault),
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
pub fn spot_swap_verify_writable_privileges<'me, 'info>(
    accounts: SpotSwapAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.dao,
        accounts.user_base_account,
        accounts.user_quote_account,
        accounts.amm_base_vault,
        accounts.amm_quote_vault,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn spot_swap_verify_signer_privileges<'me, 'info>(
    accounts: SpotSwapAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.user] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn spot_swap_verify_account_privileges<'me, 'info>(
    accounts: SpotSwapAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    spot_swap_verify_writable_privileges(accounts)?;
    spot_swap_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CONDITIONAL_SWAP_IX_ACCOUNTS_LEN: usize = 25;
#[derive(Copy, Clone, Debug)]
pub struct ConditionalSwapAccounts<'me, 'info> {
    pub dao: &'me AccountInfo<'info>,
    pub amm_base_vault: &'me AccountInfo<'info>,
    pub amm_quote_vault: &'me AccountInfo<'info>,
    pub proposal: &'me AccountInfo<'info>,
    pub amm_pass_base_vault: &'me AccountInfo<'info>,
    pub amm_pass_quote_vault: &'me AccountInfo<'info>,
    pub amm_fail_base_vault: &'me AccountInfo<'info>,
    pub amm_fail_quote_vault: &'me AccountInfo<'info>,
    pub trader: &'me AccountInfo<'info>,
    pub user_input_account: &'me AccountInfo<'info>,
    pub user_output_account: &'me AccountInfo<'info>,
    pub base_vault: &'me AccountInfo<'info>,
    pub base_vault_underlying_token_account: &'me AccountInfo<'info>,
    pub quote_vault: &'me AccountInfo<'info>,
    pub quote_vault_underlying_token_account: &'me AccountInfo<'info>,
    pub pass_base_mint: &'me AccountInfo<'info>,
    pub fail_base_mint: &'me AccountInfo<'info>,
    pub pass_quote_mint: &'me AccountInfo<'info>,
    pub fail_quote_mint: &'me AccountInfo<'info>,
    pub conditional_vault_program: &'me AccountInfo<'info>,
    pub vault_event_authority: &'me AccountInfo<'info>,
    pub question: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ConditionalSwapKeys {
    pub dao: Pubkey,
    pub amm_base_vault: Pubkey,
    pub amm_quote_vault: Pubkey,
    pub proposal: Pubkey,
    pub amm_pass_base_vault: Pubkey,
    pub amm_pass_quote_vault: Pubkey,
    pub amm_fail_base_vault: Pubkey,
    pub amm_fail_quote_vault: Pubkey,
    pub trader: Pubkey,
    pub user_input_account: Pubkey,
    pub user_output_account: Pubkey,
    pub base_vault: Pubkey,
    pub base_vault_underlying_token_account: Pubkey,
    pub quote_vault: Pubkey,
    pub quote_vault_underlying_token_account: Pubkey,
    pub pass_base_mint: Pubkey,
    pub fail_base_mint: Pubkey,
    pub pass_quote_mint: Pubkey,
    pub fail_quote_mint: Pubkey,
    pub conditional_vault_program: Pubkey,
    pub vault_event_authority: Pubkey,
    pub question: Pubkey,
    pub token_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<ConditionalSwapAccounts<'_, '_>> for ConditionalSwapKeys {
    fn from(accounts: ConditionalSwapAccounts) -> Self {
        Self {
            dao: *accounts.dao.key,
            amm_base_vault: *accounts.amm_base_vault.key,
            amm_quote_vault: *accounts.amm_quote_vault.key,
            proposal: *accounts.proposal.key,
            amm_pass_base_vault: *accounts.amm_pass_base_vault.key,
            amm_pass_quote_vault: *accounts.amm_pass_quote_vault.key,
            amm_fail_base_vault: *accounts.amm_fail_base_vault.key,
            amm_fail_quote_vault: *accounts.amm_fail_quote_vault.key,
            trader: *accounts.trader.key,
            user_input_account: *accounts.user_input_account.key,
            user_output_account: *accounts.user_output_account.key,
            base_vault: *accounts.base_vault.key,
            base_vault_underlying_token_account: *accounts
                .base_vault_underlying_token_account
                .key,
            quote_vault: *accounts.quote_vault.key,
            quote_vault_underlying_token_account: *accounts
                .quote_vault_underlying_token_account
                .key,
            pass_base_mint: *accounts.pass_base_mint.key,
            fail_base_mint: *accounts.fail_base_mint.key,
            pass_quote_mint: *accounts.pass_quote_mint.key,
            fail_quote_mint: *accounts.fail_quote_mint.key,
            conditional_vault_program: *accounts.conditional_vault_program.key,
            vault_event_authority: *accounts.vault_event_authority.key,
            question: *accounts.question.key,
            token_program: *accounts.token_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<ConditionalSwapKeys> for [AccountMeta; CONDITIONAL_SWAP_IX_ACCOUNTS_LEN] {
    fn from(keys: ConditionalSwapKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.dao,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.amm_base_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.amm_quote_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.proposal,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.amm_pass_base_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.amm_pass_quote_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.amm_fail_base_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.amm_fail_quote_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.trader,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.user_input_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_output_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.base_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.base_vault_underlying_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.quote_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.quote_vault_underlying_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pass_base_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.fail_base_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pass_quote_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.fail_quote_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.conditional_vault_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault_event_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.question,
                is_signer: false,
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
impl From<[Pubkey; CONDITIONAL_SWAP_IX_ACCOUNTS_LEN]> for ConditionalSwapKeys {
    fn from(pubkeys: [Pubkey; CONDITIONAL_SWAP_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            dao: pubkeys[0],
            amm_base_vault: pubkeys[1],
            amm_quote_vault: pubkeys[2],
            proposal: pubkeys[3],
            amm_pass_base_vault: pubkeys[4],
            amm_pass_quote_vault: pubkeys[5],
            amm_fail_base_vault: pubkeys[6],
            amm_fail_quote_vault: pubkeys[7],
            trader: pubkeys[8],
            user_input_account: pubkeys[9],
            user_output_account: pubkeys[10],
            base_vault: pubkeys[11],
            base_vault_underlying_token_account: pubkeys[12],
            quote_vault: pubkeys[13],
            quote_vault_underlying_token_account: pubkeys[14],
            pass_base_mint: pubkeys[15],
            fail_base_mint: pubkeys[16],
            pass_quote_mint: pubkeys[17],
            fail_quote_mint: pubkeys[18],
            conditional_vault_program: pubkeys[19],
            vault_event_authority: pubkeys[20],
            question: pubkeys[21],
            token_program: pubkeys[22],
            event_authority: pubkeys[23],
            program: pubkeys[24],
        }
    }
}
impl<'info> From<ConditionalSwapAccounts<'_, 'info>>
for [AccountInfo<'info>; CONDITIONAL_SWAP_IX_ACCOUNTS_LEN] {
    fn from(accounts: ConditionalSwapAccounts<'_, 'info>) -> Self {
        [
            accounts.dao.clone(),
            accounts.amm_base_vault.clone(),
            accounts.amm_quote_vault.clone(),
            accounts.proposal.clone(),
            accounts.amm_pass_base_vault.clone(),
            accounts.amm_pass_quote_vault.clone(),
            accounts.amm_fail_base_vault.clone(),
            accounts.amm_fail_quote_vault.clone(),
            accounts.trader.clone(),
            accounts.user_input_account.clone(),
            accounts.user_output_account.clone(),
            accounts.base_vault.clone(),
            accounts.base_vault_underlying_token_account.clone(),
            accounts.quote_vault.clone(),
            accounts.quote_vault_underlying_token_account.clone(),
            accounts.pass_base_mint.clone(),
            accounts.fail_base_mint.clone(),
            accounts.pass_quote_mint.clone(),
            accounts.fail_quote_mint.clone(),
            accounts.conditional_vault_program.clone(),
            accounts.vault_event_authority.clone(),
            accounts.question.clone(),
            accounts.token_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CONDITIONAL_SWAP_IX_ACCOUNTS_LEN]>
for ConditionalSwapAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; CONDITIONAL_SWAP_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            dao: &arr[0],
            amm_base_vault: &arr[1],
            amm_quote_vault: &arr[2],
            proposal: &arr[3],
            amm_pass_base_vault: &arr[4],
            amm_pass_quote_vault: &arr[5],
            amm_fail_base_vault: &arr[6],
            amm_fail_quote_vault: &arr[7],
            trader: &arr[8],
            user_input_account: &arr[9],
            user_output_account: &arr[10],
            base_vault: &arr[11],
            base_vault_underlying_token_account: &arr[12],
            quote_vault: &arr[13],
            quote_vault_underlying_token_account: &arr[14],
            pass_base_mint: &arr[15],
            fail_base_mint: &arr[16],
            pass_quote_mint: &arr[17],
            fail_quote_mint: &arr[18],
            conditional_vault_program: &arr[19],
            vault_event_authority: &arr[20],
            question: &arr[21],
            token_program: &arr[22],
            event_authority: &arr[23],
            program: &arr[24],
        }
    }
}
pub const CONDITIONAL_SWAP_IX_DISCM: [u8; 8usize] = [
    194, 136, 220, 89, 242, 169, 130, 157,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ConditionalSwapIxArgs {
    pub params: ConditionalSwapParams,
}
#[derive(Clone, Debug, PartialEq)]
pub struct ConditionalSwapIxData(pub ConditionalSwapIxArgs);
impl From<ConditionalSwapIxArgs> for ConditionalSwapIxData {
    fn from(args: ConditionalSwapIxArgs) -> Self {
        Self(args)
    }
}
impl ConditionalSwapIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CONDITIONAL_SWAP_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let params = if reader.is_empty() {
            Default::default()
        } else {
            <ConditionalSwapParams>::deserialize(&mut reader)?
        };
        Ok(Self(ConditionalSwapIxArgs { params }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CONDITIONAL_SWAP_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.params, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn conditional_swap_ix_with_program_id(
    program_id: Pubkey,
    keys: ConditionalSwapKeys,
    args: ConditionalSwapIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CONDITIONAL_SWAP_IX_ACCOUNTS_LEN] = keys.into();
    let data: ConditionalSwapIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn conditional_swap_ix(
    keys: ConditionalSwapKeys,
    args: ConditionalSwapIxArgs,
) -> std::io::Result<Instruction> {
    conditional_swap_ix_with_program_id(FUTARCHY_PROGRAM_ID, keys, args)
}
pub fn conditional_swap_invoke_with_program_id(
    program_id: Pubkey,
    accounts: ConditionalSwapAccounts<'_, '_>,
    args: ConditionalSwapIxArgs,
) -> ProgramResult {
    let keys: ConditionalSwapKeys = accounts.into();
    let ix = conditional_swap_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn conditional_swap_invoke(
    accounts: ConditionalSwapAccounts<'_, '_>,
    args: ConditionalSwapIxArgs,
) -> ProgramResult {
    conditional_swap_invoke_with_program_id(FUTARCHY_PROGRAM_ID, accounts, args)
}
pub fn conditional_swap_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: ConditionalSwapAccounts<'_, '_>,
    args: ConditionalSwapIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: ConditionalSwapKeys = accounts.into();
    let ix = conditional_swap_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn conditional_swap_invoke_signed(
    accounts: ConditionalSwapAccounts<'_, '_>,
    args: ConditionalSwapIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    conditional_swap_invoke_signed_with_program_id(
        FUTARCHY_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn conditional_swap_verify_account_keys(
    accounts: ConditionalSwapAccounts<'_, '_>,
    keys: ConditionalSwapKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.dao.key, keys.dao),
        (*accounts.amm_base_vault.key, keys.amm_base_vault),
        (*accounts.amm_quote_vault.key, keys.amm_quote_vault),
        (*accounts.proposal.key, keys.proposal),
        (*accounts.amm_pass_base_vault.key, keys.amm_pass_base_vault),
        (*accounts.amm_pass_quote_vault.key, keys.amm_pass_quote_vault),
        (*accounts.amm_fail_base_vault.key, keys.amm_fail_base_vault),
        (*accounts.amm_fail_quote_vault.key, keys.amm_fail_quote_vault),
        (*accounts.trader.key, keys.trader),
        (*accounts.user_input_account.key, keys.user_input_account),
        (*accounts.user_output_account.key, keys.user_output_account),
        (*accounts.base_vault.key, keys.base_vault),
        (
            *accounts.base_vault_underlying_token_account.key,
            keys.base_vault_underlying_token_account,
        ),
        (*accounts.quote_vault.key, keys.quote_vault),
        (
            *accounts.quote_vault_underlying_token_account.key,
            keys.quote_vault_underlying_token_account,
        ),
        (*accounts.pass_base_mint.key, keys.pass_base_mint),
        (*accounts.fail_base_mint.key, keys.fail_base_mint),
        (*accounts.pass_quote_mint.key, keys.pass_quote_mint),
        (*accounts.fail_quote_mint.key, keys.fail_quote_mint),
        (*accounts.conditional_vault_program.key, keys.conditional_vault_program),
        (*accounts.vault_event_authority.key, keys.vault_event_authority),
        (*accounts.question.key, keys.question),
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
pub fn conditional_swap_verify_writable_privileges<'me, 'info>(
    accounts: ConditionalSwapAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.dao,
        accounts.amm_base_vault,
        accounts.amm_quote_vault,
        accounts.amm_pass_base_vault,
        accounts.amm_pass_quote_vault,
        accounts.amm_fail_base_vault,
        accounts.amm_fail_quote_vault,
        accounts.user_input_account,
        accounts.user_output_account,
        accounts.base_vault,
        accounts.base_vault_underlying_token_account,
        accounts.quote_vault,
        accounts.quote_vault_underlying_token_account,
        accounts.pass_base_mint,
        accounts.fail_base_mint,
        accounts.pass_quote_mint,
        accounts.fail_quote_mint,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn conditional_swap_verify_signer_privileges<'me, 'info>(
    accounts: ConditionalSwapAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.trader] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn conditional_swap_verify_account_privileges<'me, 'info>(
    accounts: ConditionalSwapAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    conditional_swap_verify_writable_privileges(accounts)?;
    conditional_swap_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const PROVIDE_LIQUIDITY_IX_ACCOUNTS_LEN: usize = 12;
#[derive(Copy, Clone, Debug)]
pub struct ProvideLiquidityAccounts<'me, 'info> {
    pub dao: &'me AccountInfo<'info>,
    pub liquidity_provider: &'me AccountInfo<'info>,
    pub liquidity_provider_base_account: &'me AccountInfo<'info>,
    pub liquidity_provider_quote_account: &'me AccountInfo<'info>,
    pub payer: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub amm_base_vault: &'me AccountInfo<'info>,
    pub amm_quote_vault: &'me AccountInfo<'info>,
    pub amm_position: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ProvideLiquidityKeys {
    pub dao: Pubkey,
    pub liquidity_provider: Pubkey,
    pub liquidity_provider_base_account: Pubkey,
    pub liquidity_provider_quote_account: Pubkey,
    pub payer: Pubkey,
    pub system_program: Pubkey,
    pub amm_base_vault: Pubkey,
    pub amm_quote_vault: Pubkey,
    pub amm_position: Pubkey,
    pub token_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<ProvideLiquidityAccounts<'_, '_>> for ProvideLiquidityKeys {
    fn from(accounts: ProvideLiquidityAccounts) -> Self {
        Self {
            dao: *accounts.dao.key,
            liquidity_provider: *accounts.liquidity_provider.key,
            liquidity_provider_base_account: *accounts
                .liquidity_provider_base_account
                .key,
            liquidity_provider_quote_account: *accounts
                .liquidity_provider_quote_account
                .key,
            payer: *accounts.payer.key,
            system_program: *accounts.system_program.key,
            amm_base_vault: *accounts.amm_base_vault.key,
            amm_quote_vault: *accounts.amm_quote_vault.key,
            amm_position: *accounts.amm_position.key,
            token_program: *accounts.token_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<ProvideLiquidityKeys> for [AccountMeta; PROVIDE_LIQUIDITY_IX_ACCOUNTS_LEN] {
    fn from(keys: ProvideLiquidityKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.dao,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.liquidity_provider,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.liquidity_provider_base_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.liquidity_provider_quote_account,
                is_signer: false,
                is_writable: true,
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
                pubkey: keys.amm_base_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.amm_quote_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.amm_position,
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
impl From<[Pubkey; PROVIDE_LIQUIDITY_IX_ACCOUNTS_LEN]> for ProvideLiquidityKeys {
    fn from(pubkeys: [Pubkey; PROVIDE_LIQUIDITY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            dao: pubkeys[0],
            liquidity_provider: pubkeys[1],
            liquidity_provider_base_account: pubkeys[2],
            liquidity_provider_quote_account: pubkeys[3],
            payer: pubkeys[4],
            system_program: pubkeys[5],
            amm_base_vault: pubkeys[6],
            amm_quote_vault: pubkeys[7],
            amm_position: pubkeys[8],
            token_program: pubkeys[9],
            event_authority: pubkeys[10],
            program: pubkeys[11],
        }
    }
}
impl<'info> From<ProvideLiquidityAccounts<'_, 'info>>
for [AccountInfo<'info>; PROVIDE_LIQUIDITY_IX_ACCOUNTS_LEN] {
    fn from(accounts: ProvideLiquidityAccounts<'_, 'info>) -> Self {
        [
            accounts.dao.clone(),
            accounts.liquidity_provider.clone(),
            accounts.liquidity_provider_base_account.clone(),
            accounts.liquidity_provider_quote_account.clone(),
            accounts.payer.clone(),
            accounts.system_program.clone(),
            accounts.amm_base_vault.clone(),
            accounts.amm_quote_vault.clone(),
            accounts.amm_position.clone(),
            accounts.token_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; PROVIDE_LIQUIDITY_IX_ACCOUNTS_LEN]>
for ProvideLiquidityAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; PROVIDE_LIQUIDITY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            dao: &arr[0],
            liquidity_provider: &arr[1],
            liquidity_provider_base_account: &arr[2],
            liquidity_provider_quote_account: &arr[3],
            payer: &arr[4],
            system_program: &arr[5],
            amm_base_vault: &arr[6],
            amm_quote_vault: &arr[7],
            amm_position: &arr[8],
            token_program: &arr[9],
            event_authority: &arr[10],
            program: &arr[11],
        }
    }
}
pub const PROVIDE_LIQUIDITY_IX_DISCM: [u8; 8usize] = [
    40, 110, 107, 116, 174, 127, 97, 204,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ProvideLiquidityIxArgs {
    pub params: ProvideLiquidityParams,
}
#[derive(Clone, Debug, PartialEq)]
pub struct ProvideLiquidityIxData(pub ProvideLiquidityIxArgs);
impl From<ProvideLiquidityIxArgs> for ProvideLiquidityIxData {
    fn from(args: ProvideLiquidityIxArgs) -> Self {
        Self(args)
    }
}
impl ProvideLiquidityIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != PROVIDE_LIQUIDITY_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let params = if reader.is_empty() {
            Default::default()
        } else {
            <ProvideLiquidityParams>::deserialize(&mut reader)?
        };
        Ok(Self(ProvideLiquidityIxArgs { params }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&PROVIDE_LIQUIDITY_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.params, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn provide_liquidity_ix_with_program_id(
    program_id: Pubkey,
    keys: ProvideLiquidityKeys,
    args: ProvideLiquidityIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; PROVIDE_LIQUIDITY_IX_ACCOUNTS_LEN] = keys.into();
    let data: ProvideLiquidityIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn provide_liquidity_ix(
    keys: ProvideLiquidityKeys,
    args: ProvideLiquidityIxArgs,
) -> std::io::Result<Instruction> {
    provide_liquidity_ix_with_program_id(FUTARCHY_PROGRAM_ID, keys, args)
}
pub fn provide_liquidity_invoke_with_program_id(
    program_id: Pubkey,
    accounts: ProvideLiquidityAccounts<'_, '_>,
    args: ProvideLiquidityIxArgs,
) -> ProgramResult {
    let keys: ProvideLiquidityKeys = accounts.into();
    let ix = provide_liquidity_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn provide_liquidity_invoke(
    accounts: ProvideLiquidityAccounts<'_, '_>,
    args: ProvideLiquidityIxArgs,
) -> ProgramResult {
    provide_liquidity_invoke_with_program_id(FUTARCHY_PROGRAM_ID, accounts, args)
}
pub fn provide_liquidity_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: ProvideLiquidityAccounts<'_, '_>,
    args: ProvideLiquidityIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: ProvideLiquidityKeys = accounts.into();
    let ix = provide_liquidity_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn provide_liquidity_invoke_signed(
    accounts: ProvideLiquidityAccounts<'_, '_>,
    args: ProvideLiquidityIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    provide_liquidity_invoke_signed_with_program_id(
        FUTARCHY_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn provide_liquidity_verify_account_keys(
    accounts: ProvideLiquidityAccounts<'_, '_>,
    keys: ProvideLiquidityKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.dao.key, keys.dao),
        (*accounts.liquidity_provider.key, keys.liquidity_provider),
        (
            *accounts.liquidity_provider_base_account.key,
            keys.liquidity_provider_base_account,
        ),
        (
            *accounts.liquidity_provider_quote_account.key,
            keys.liquidity_provider_quote_account,
        ),
        (*accounts.payer.key, keys.payer),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.amm_base_vault.key, keys.amm_base_vault),
        (*accounts.amm_quote_vault.key, keys.amm_quote_vault),
        (*accounts.amm_position.key, keys.amm_position),
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
pub fn provide_liquidity_verify_writable_privileges<'me, 'info>(
    accounts: ProvideLiquidityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.dao,
        accounts.liquidity_provider_base_account,
        accounts.liquidity_provider_quote_account,
        accounts.payer,
        accounts.amm_base_vault,
        accounts.amm_quote_vault,
        accounts.amm_position,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn provide_liquidity_verify_signer_privileges<'me, 'info>(
    accounts: ProvideLiquidityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.liquidity_provider, accounts.payer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn provide_liquidity_verify_account_privileges<'me, 'info>(
    accounts: ProvideLiquidityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    provide_liquidity_verify_writable_privileges(accounts)?;
    provide_liquidity_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const WITHDRAW_LIQUIDITY_IX_ACCOUNTS_LEN: usize = 10;
#[derive(Copy, Clone, Debug)]
pub struct WithdrawLiquidityAccounts<'me, 'info> {
    pub dao: &'me AccountInfo<'info>,
    pub position_authority: &'me AccountInfo<'info>,
    pub liquidity_provider_base_account: &'me AccountInfo<'info>,
    pub liquidity_provider_quote_account: &'me AccountInfo<'info>,
    pub amm_base_vault: &'me AccountInfo<'info>,
    pub amm_quote_vault: &'me AccountInfo<'info>,
    pub amm_position: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct WithdrawLiquidityKeys {
    pub dao: Pubkey,
    pub position_authority: Pubkey,
    pub liquidity_provider_base_account: Pubkey,
    pub liquidity_provider_quote_account: Pubkey,
    pub amm_base_vault: Pubkey,
    pub amm_quote_vault: Pubkey,
    pub amm_position: Pubkey,
    pub token_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<WithdrawLiquidityAccounts<'_, '_>> for WithdrawLiquidityKeys {
    fn from(accounts: WithdrawLiquidityAccounts) -> Self {
        Self {
            dao: *accounts.dao.key,
            position_authority: *accounts.position_authority.key,
            liquidity_provider_base_account: *accounts
                .liquidity_provider_base_account
                .key,
            liquidity_provider_quote_account: *accounts
                .liquidity_provider_quote_account
                .key,
            amm_base_vault: *accounts.amm_base_vault.key,
            amm_quote_vault: *accounts.amm_quote_vault.key,
            amm_position: *accounts.amm_position.key,
            token_program: *accounts.token_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<WithdrawLiquidityKeys> for [AccountMeta; WITHDRAW_LIQUIDITY_IX_ACCOUNTS_LEN] {
    fn from(keys: WithdrawLiquidityKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.dao,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.position_authority,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.liquidity_provider_base_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.liquidity_provider_quote_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.amm_base_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.amm_quote_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.amm_position,
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
impl From<[Pubkey; WITHDRAW_LIQUIDITY_IX_ACCOUNTS_LEN]> for WithdrawLiquidityKeys {
    fn from(pubkeys: [Pubkey; WITHDRAW_LIQUIDITY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            dao: pubkeys[0],
            position_authority: pubkeys[1],
            liquidity_provider_base_account: pubkeys[2],
            liquidity_provider_quote_account: pubkeys[3],
            amm_base_vault: pubkeys[4],
            amm_quote_vault: pubkeys[5],
            amm_position: pubkeys[6],
            token_program: pubkeys[7],
            event_authority: pubkeys[8],
            program: pubkeys[9],
        }
    }
}
impl<'info> From<WithdrawLiquidityAccounts<'_, 'info>>
for [AccountInfo<'info>; WITHDRAW_LIQUIDITY_IX_ACCOUNTS_LEN] {
    fn from(accounts: WithdrawLiquidityAccounts<'_, 'info>) -> Self {
        [
            accounts.dao.clone(),
            accounts.position_authority.clone(),
            accounts.liquidity_provider_base_account.clone(),
            accounts.liquidity_provider_quote_account.clone(),
            accounts.amm_base_vault.clone(),
            accounts.amm_quote_vault.clone(),
            accounts.amm_position.clone(),
            accounts.token_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; WITHDRAW_LIQUIDITY_IX_ACCOUNTS_LEN]>
for WithdrawLiquidityAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; WITHDRAW_LIQUIDITY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            dao: &arr[0],
            position_authority: &arr[1],
            liquidity_provider_base_account: &arr[2],
            liquidity_provider_quote_account: &arr[3],
            amm_base_vault: &arr[4],
            amm_quote_vault: &arr[5],
            amm_position: &arr[6],
            token_program: &arr[7],
            event_authority: &arr[8],
            program: &arr[9],
        }
    }
}
pub const WITHDRAW_LIQUIDITY_IX_DISCM: [u8; 8usize] = [
    149, 158, 33, 185, 47, 243, 253, 31,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WithdrawLiquidityIxArgs {
    pub params: WithdrawLiquidityParams,
}
#[derive(Clone, Debug, PartialEq)]
pub struct WithdrawLiquidityIxData(pub WithdrawLiquidityIxArgs);
impl From<WithdrawLiquidityIxArgs> for WithdrawLiquidityIxData {
    fn from(args: WithdrawLiquidityIxArgs) -> Self {
        Self(args)
    }
}
impl WithdrawLiquidityIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != WITHDRAW_LIQUIDITY_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let params = if reader.is_empty() {
            Default::default()
        } else {
            <WithdrawLiquidityParams>::deserialize(&mut reader)?
        };
        Ok(Self(WithdrawLiquidityIxArgs { params }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&WITHDRAW_LIQUIDITY_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.params, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn withdraw_liquidity_ix_with_program_id(
    program_id: Pubkey,
    keys: WithdrawLiquidityKeys,
    args: WithdrawLiquidityIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; WITHDRAW_LIQUIDITY_IX_ACCOUNTS_LEN] = keys.into();
    let data: WithdrawLiquidityIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn withdraw_liquidity_ix(
    keys: WithdrawLiquidityKeys,
    args: WithdrawLiquidityIxArgs,
) -> std::io::Result<Instruction> {
    withdraw_liquidity_ix_with_program_id(FUTARCHY_PROGRAM_ID, keys, args)
}
pub fn withdraw_liquidity_invoke_with_program_id(
    program_id: Pubkey,
    accounts: WithdrawLiquidityAccounts<'_, '_>,
    args: WithdrawLiquidityIxArgs,
) -> ProgramResult {
    let keys: WithdrawLiquidityKeys = accounts.into();
    let ix = withdraw_liquidity_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn withdraw_liquidity_invoke(
    accounts: WithdrawLiquidityAccounts<'_, '_>,
    args: WithdrawLiquidityIxArgs,
) -> ProgramResult {
    withdraw_liquidity_invoke_with_program_id(FUTARCHY_PROGRAM_ID, accounts, args)
}
pub fn withdraw_liquidity_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: WithdrawLiquidityAccounts<'_, '_>,
    args: WithdrawLiquidityIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: WithdrawLiquidityKeys = accounts.into();
    let ix = withdraw_liquidity_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn withdraw_liquidity_invoke_signed(
    accounts: WithdrawLiquidityAccounts<'_, '_>,
    args: WithdrawLiquidityIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    withdraw_liquidity_invoke_signed_with_program_id(
        FUTARCHY_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn withdraw_liquidity_verify_account_keys(
    accounts: WithdrawLiquidityAccounts<'_, '_>,
    keys: WithdrawLiquidityKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.dao.key, keys.dao),
        (*accounts.position_authority.key, keys.position_authority),
        (
            *accounts.liquidity_provider_base_account.key,
            keys.liquidity_provider_base_account,
        ),
        (
            *accounts.liquidity_provider_quote_account.key,
            keys.liquidity_provider_quote_account,
        ),
        (*accounts.amm_base_vault.key, keys.amm_base_vault),
        (*accounts.amm_quote_vault.key, keys.amm_quote_vault),
        (*accounts.amm_position.key, keys.amm_position),
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
pub fn withdraw_liquidity_verify_writable_privileges<'me, 'info>(
    accounts: WithdrawLiquidityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.dao,
        accounts.liquidity_provider_base_account,
        accounts.liquidity_provider_quote_account,
        accounts.amm_base_vault,
        accounts.amm_quote_vault,
        accounts.amm_position,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn withdraw_liquidity_verify_signer_privileges<'me, 'info>(
    accounts: WithdrawLiquidityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.position_authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn withdraw_liquidity_verify_account_privileges<'me, 'info>(
    accounts: WithdrawLiquidityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    withdraw_liquidity_verify_writable_privileges(accounts)?;
    withdraw_liquidity_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const COLLECT_FEES_IX_ACCOUNTS_LEN: usize = 9;
#[derive(Copy, Clone, Debug)]
pub struct CollectFeesAccounts<'me, 'info> {
    pub dao: &'me AccountInfo<'info>,
    pub admin: &'me AccountInfo<'info>,
    pub base_token_account: &'me AccountInfo<'info>,
    pub quote_token_account: &'me AccountInfo<'info>,
    pub amm_base_vault: &'me AccountInfo<'info>,
    pub amm_quote_vault: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CollectFeesKeys {
    pub dao: Pubkey,
    pub admin: Pubkey,
    pub base_token_account: Pubkey,
    pub quote_token_account: Pubkey,
    pub amm_base_vault: Pubkey,
    pub amm_quote_vault: Pubkey,
    pub token_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<CollectFeesAccounts<'_, '_>> for CollectFeesKeys {
    fn from(accounts: CollectFeesAccounts) -> Self {
        Self {
            dao: *accounts.dao.key,
            admin: *accounts.admin.key,
            base_token_account: *accounts.base_token_account.key,
            quote_token_account: *accounts.quote_token_account.key,
            amm_base_vault: *accounts.amm_base_vault.key,
            amm_quote_vault: *accounts.amm_quote_vault.key,
            token_program: *accounts.token_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<CollectFeesKeys> for [AccountMeta; COLLECT_FEES_IX_ACCOUNTS_LEN] {
    fn from(keys: CollectFeesKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.dao,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.admin,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.base_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.quote_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.amm_base_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.amm_quote_vault,
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
impl From<[Pubkey; COLLECT_FEES_IX_ACCOUNTS_LEN]> for CollectFeesKeys {
    fn from(pubkeys: [Pubkey; COLLECT_FEES_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            dao: pubkeys[0],
            admin: pubkeys[1],
            base_token_account: pubkeys[2],
            quote_token_account: pubkeys[3],
            amm_base_vault: pubkeys[4],
            amm_quote_vault: pubkeys[5],
            token_program: pubkeys[6],
            event_authority: pubkeys[7],
            program: pubkeys[8],
        }
    }
}
impl<'info> From<CollectFeesAccounts<'_, 'info>>
for [AccountInfo<'info>; COLLECT_FEES_IX_ACCOUNTS_LEN] {
    fn from(accounts: CollectFeesAccounts<'_, 'info>) -> Self {
        [
            accounts.dao.clone(),
            accounts.admin.clone(),
            accounts.base_token_account.clone(),
            accounts.quote_token_account.clone(),
            accounts.amm_base_vault.clone(),
            accounts.amm_quote_vault.clone(),
            accounts.token_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; COLLECT_FEES_IX_ACCOUNTS_LEN]>
for CollectFeesAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; COLLECT_FEES_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            dao: &arr[0],
            admin: &arr[1],
            base_token_account: &arr[2],
            quote_token_account: &arr[3],
            amm_base_vault: &arr[4],
            amm_quote_vault: &arr[5],
            token_program: &arr[6],
            event_authority: &arr[7],
            program: &arr[8],
        }
    }
}
pub const COLLECT_FEES_IX_DISCM: [u8; 8usize] = [164, 152, 207, 99, 30, 186, 19, 182];
#[derive(Clone, Debug, PartialEq)]
pub struct CollectFeesIxData;
impl CollectFeesIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != COLLECT_FEES_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&COLLECT_FEES_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn collect_fees_ix_with_program_id(
    program_id: Pubkey,
    keys: CollectFeesKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; COLLECT_FEES_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: CollectFeesIxData.try_to_vec()?,
    })
}
pub fn collect_fees_ix(keys: CollectFeesKeys) -> std::io::Result<Instruction> {
    collect_fees_ix_with_program_id(FUTARCHY_PROGRAM_ID, keys)
}
pub fn collect_fees_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CollectFeesAccounts<'_, '_>,
) -> ProgramResult {
    let keys: CollectFeesKeys = accounts.into();
    let ix = collect_fees_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn collect_fees_invoke(accounts: CollectFeesAccounts<'_, '_>) -> ProgramResult {
    collect_fees_invoke_with_program_id(FUTARCHY_PROGRAM_ID, accounts)
}
pub fn collect_fees_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CollectFeesAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CollectFeesKeys = accounts.into();
    let ix = collect_fees_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn collect_fees_invoke_signed(
    accounts: CollectFeesAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    collect_fees_invoke_signed_with_program_id(FUTARCHY_PROGRAM_ID, accounts, seeds)
}
pub fn collect_fees_verify_account_keys(
    accounts: CollectFeesAccounts<'_, '_>,
    keys: CollectFeesKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.dao.key, keys.dao),
        (*accounts.admin.key, keys.admin),
        (*accounts.base_token_account.key, keys.base_token_account),
        (*accounts.quote_token_account.key, keys.quote_token_account),
        (*accounts.amm_base_vault.key, keys.amm_base_vault),
        (*accounts.amm_quote_vault.key, keys.amm_quote_vault),
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
pub fn collect_fees_verify_writable_privileges<'me, 'info>(
    accounts: CollectFeesAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.dao,
        accounts.base_token_account,
        accounts.quote_token_account,
        accounts.amm_base_vault,
        accounts.amm_quote_vault,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn collect_fees_verify_signer_privileges<'me, 'info>(
    accounts: CollectFeesAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn collect_fees_verify_account_privileges<'me, 'info>(
    accounts: CollectFeesAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    collect_fees_verify_writable_privileges(accounts)?;
    collect_fees_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const EXECUTE_SPENDING_LIMIT_CHANGE_IX_ACCOUNTS_LEN: usize = 6;
#[derive(Copy, Clone, Debug)]
pub struct ExecuteSpendingLimitChangeAccounts<'me, 'info> {
    pub proposal: &'me AccountInfo<'info>,
    pub dao: &'me AccountInfo<'info>,
    pub squads_proposal: &'me AccountInfo<'info>,
    pub squads_multisig: &'me AccountInfo<'info>,
    pub squads_multisig_program: &'me AccountInfo<'info>,
    pub vault_transaction: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ExecuteSpendingLimitChangeKeys {
    pub proposal: Pubkey,
    pub dao: Pubkey,
    pub squads_proposal: Pubkey,
    pub squads_multisig: Pubkey,
    pub squads_multisig_program: Pubkey,
    pub vault_transaction: Pubkey,
}
impl From<ExecuteSpendingLimitChangeAccounts<'_, '_>>
for ExecuteSpendingLimitChangeKeys {
    fn from(accounts: ExecuteSpendingLimitChangeAccounts) -> Self {
        Self {
            proposal: *accounts.proposal.key,
            dao: *accounts.dao.key,
            squads_proposal: *accounts.squads_proposal.key,
            squads_multisig: *accounts.squads_multisig.key,
            squads_multisig_program: *accounts.squads_multisig_program.key,
            vault_transaction: *accounts.vault_transaction.key,
        }
    }
}
impl From<ExecuteSpendingLimitChangeKeys>
for [AccountMeta; EXECUTE_SPENDING_LIMIT_CHANGE_IX_ACCOUNTS_LEN] {
    fn from(keys: ExecuteSpendingLimitChangeKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.proposal,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.dao,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.squads_proposal,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.squads_multisig,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.squads_multisig_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault_transaction,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; EXECUTE_SPENDING_LIMIT_CHANGE_IX_ACCOUNTS_LEN]>
for ExecuteSpendingLimitChangeKeys {
    fn from(pubkeys: [Pubkey; EXECUTE_SPENDING_LIMIT_CHANGE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            proposal: pubkeys[0],
            dao: pubkeys[1],
            squads_proposal: pubkeys[2],
            squads_multisig: pubkeys[3],
            squads_multisig_program: pubkeys[4],
            vault_transaction: pubkeys[5],
        }
    }
}
impl<'info> From<ExecuteSpendingLimitChangeAccounts<'_, 'info>>
for [AccountInfo<'info>; EXECUTE_SPENDING_LIMIT_CHANGE_IX_ACCOUNTS_LEN] {
    fn from(accounts: ExecuteSpendingLimitChangeAccounts<'_, 'info>) -> Self {
        [
            accounts.proposal.clone(),
            accounts.dao.clone(),
            accounts.squads_proposal.clone(),
            accounts.squads_multisig.clone(),
            accounts.squads_multisig_program.clone(),
            accounts.vault_transaction.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; EXECUTE_SPENDING_LIMIT_CHANGE_IX_ACCOUNTS_LEN]>
for ExecuteSpendingLimitChangeAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; EXECUTE_SPENDING_LIMIT_CHANGE_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            proposal: &arr[0],
            dao: &arr[1],
            squads_proposal: &arr[2],
            squads_multisig: &arr[3],
            squads_multisig_program: &arr[4],
            vault_transaction: &arr[5],
        }
    }
}
pub const EXECUTE_SPENDING_LIMIT_CHANGE_IX_DISCM: [u8; 8usize] = [
    146, 175, 145, 31, 184, 129, 252, 79,
];
#[derive(Clone, Debug, PartialEq)]
pub struct ExecuteSpendingLimitChangeIxData;
impl ExecuteSpendingLimitChangeIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != EXECUTE_SPENDING_LIMIT_CHANGE_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&EXECUTE_SPENDING_LIMIT_CHANGE_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn execute_spending_limit_change_ix_with_program_id(
    program_id: Pubkey,
    keys: ExecuteSpendingLimitChangeKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; EXECUTE_SPENDING_LIMIT_CHANGE_IX_ACCOUNTS_LEN] = keys
        .into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: ExecuteSpendingLimitChangeIxData.try_to_vec()?,
    })
}
pub fn execute_spending_limit_change_ix(
    keys: ExecuteSpendingLimitChangeKeys,
) -> std::io::Result<Instruction> {
    execute_spending_limit_change_ix_with_program_id(FUTARCHY_PROGRAM_ID, keys)
}
pub fn execute_spending_limit_change_invoke_with_program_id(
    program_id: Pubkey,
    accounts: ExecuteSpendingLimitChangeAccounts<'_, '_>,
) -> ProgramResult {
    let keys: ExecuteSpendingLimitChangeKeys = accounts.into();
    let ix = execute_spending_limit_change_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn execute_spending_limit_change_invoke(
    accounts: ExecuteSpendingLimitChangeAccounts<'_, '_>,
) -> ProgramResult {
    execute_spending_limit_change_invoke_with_program_id(FUTARCHY_PROGRAM_ID, accounts)
}
pub fn execute_spending_limit_change_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: ExecuteSpendingLimitChangeAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: ExecuteSpendingLimitChangeKeys = accounts.into();
    let ix = execute_spending_limit_change_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn execute_spending_limit_change_invoke_signed(
    accounts: ExecuteSpendingLimitChangeAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    execute_spending_limit_change_invoke_signed_with_program_id(
        FUTARCHY_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn execute_spending_limit_change_verify_account_keys(
    accounts: ExecuteSpendingLimitChangeAccounts<'_, '_>,
    keys: ExecuteSpendingLimitChangeKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.proposal.key, keys.proposal),
        (*accounts.dao.key, keys.dao),
        (*accounts.squads_proposal.key, keys.squads_proposal),
        (*accounts.squads_multisig.key, keys.squads_multisig),
        (*accounts.squads_multisig_program.key, keys.squads_multisig_program),
        (*accounts.vault_transaction.key, keys.vault_transaction),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn execute_spending_limit_change_verify_writable_privileges<'me, 'info>(
    accounts: ExecuteSpendingLimitChangeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.proposal,
        accounts.dao,
        accounts.squads_proposal,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn execute_spending_limit_change_verify_account_privileges<'me, 'info>(
    accounts: ExecuteSpendingLimitChangeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    execute_spending_limit_change_verify_writable_privileges(accounts)?;
    Ok(())
}
pub const SPONSOR_PROPOSAL_IX_ACCOUNTS_LEN: usize = 5;
#[derive(Copy, Clone, Debug)]
pub struct SponsorProposalAccounts<'me, 'info> {
    pub proposal: &'me AccountInfo<'info>,
    pub dao: &'me AccountInfo<'info>,
    pub team_address: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SponsorProposalKeys {
    pub proposal: Pubkey,
    pub dao: Pubkey,
    pub team_address: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<SponsorProposalAccounts<'_, '_>> for SponsorProposalKeys {
    fn from(accounts: SponsorProposalAccounts) -> Self {
        Self {
            proposal: *accounts.proposal.key,
            dao: *accounts.dao.key,
            team_address: *accounts.team_address.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<SponsorProposalKeys> for [AccountMeta; SPONSOR_PROPOSAL_IX_ACCOUNTS_LEN] {
    fn from(keys: SponsorProposalKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.proposal,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.dao,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.team_address,
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
impl From<[Pubkey; SPONSOR_PROPOSAL_IX_ACCOUNTS_LEN]> for SponsorProposalKeys {
    fn from(pubkeys: [Pubkey; SPONSOR_PROPOSAL_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            proposal: pubkeys[0],
            dao: pubkeys[1],
            team_address: pubkeys[2],
            event_authority: pubkeys[3],
            program: pubkeys[4],
        }
    }
}
impl<'info> From<SponsorProposalAccounts<'_, 'info>>
for [AccountInfo<'info>; SPONSOR_PROPOSAL_IX_ACCOUNTS_LEN] {
    fn from(accounts: SponsorProposalAccounts<'_, 'info>) -> Self {
        [
            accounts.proposal.clone(),
            accounts.dao.clone(),
            accounts.team_address.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SPONSOR_PROPOSAL_IX_ACCOUNTS_LEN]>
for SponsorProposalAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; SPONSOR_PROPOSAL_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            proposal: &arr[0],
            dao: &arr[1],
            team_address: &arr[2],
            event_authority: &arr[3],
            program: &arr[4],
        }
    }
}
pub const SPONSOR_PROPOSAL_IX_DISCM: [u8; 8usize] = [
    193, 57, 170, 136, 101, 196, 58, 173,
];
#[derive(Clone, Debug, PartialEq)]
pub struct SponsorProposalIxData;
impl SponsorProposalIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SPONSOR_PROPOSAL_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SPONSOR_PROPOSAL_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn sponsor_proposal_ix_with_program_id(
    program_id: Pubkey,
    keys: SponsorProposalKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SPONSOR_PROPOSAL_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: SponsorProposalIxData.try_to_vec()?,
    })
}
pub fn sponsor_proposal_ix(keys: SponsorProposalKeys) -> std::io::Result<Instruction> {
    sponsor_proposal_ix_with_program_id(FUTARCHY_PROGRAM_ID, keys)
}
pub fn sponsor_proposal_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SponsorProposalAccounts<'_, '_>,
) -> ProgramResult {
    let keys: SponsorProposalKeys = accounts.into();
    let ix = sponsor_proposal_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn sponsor_proposal_invoke(
    accounts: SponsorProposalAccounts<'_, '_>,
) -> ProgramResult {
    sponsor_proposal_invoke_with_program_id(FUTARCHY_PROGRAM_ID, accounts)
}
pub fn sponsor_proposal_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SponsorProposalAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SponsorProposalKeys = accounts.into();
    let ix = sponsor_proposal_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn sponsor_proposal_invoke_signed(
    accounts: SponsorProposalAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    sponsor_proposal_invoke_signed_with_program_id(FUTARCHY_PROGRAM_ID, accounts, seeds)
}
pub fn sponsor_proposal_verify_account_keys(
    accounts: SponsorProposalAccounts<'_, '_>,
    keys: SponsorProposalKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.proposal.key, keys.proposal),
        (*accounts.dao.key, keys.dao),
        (*accounts.team_address.key, keys.team_address),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn sponsor_proposal_verify_writable_privileges<'me, 'info>(
    accounts: SponsorProposalAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.proposal, accounts.dao] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn sponsor_proposal_verify_signer_privileges<'me, 'info>(
    accounts: SponsorProposalAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.team_address] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn sponsor_proposal_verify_account_privileges<'me, 'info>(
    accounts: SponsorProposalAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    sponsor_proposal_verify_writable_privileges(accounts)?;
    sponsor_proposal_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const COLLECT_METEORA_DAMM_FEES_IX_ACCOUNTS_LEN: usize = 27;
#[derive(Copy, Clone, Debug)]
pub struct CollectMeteoraDammFeesAccounts<'me, 'info> {
    pub dao: &'me AccountInfo<'info>,
    pub admin: &'me AccountInfo<'info>,
    pub squads_multisig: &'me AccountInfo<'info>,
    pub squads_multisig_vault: &'me AccountInfo<'info>,
    pub squads_multisig_vault_transaction: &'me AccountInfo<'info>,
    pub squads_multisig_proposal: &'me AccountInfo<'info>,
    pub squads_multisig_permissionless_account: &'me AccountInfo<'info>,
    pub meteora_claim_position_fees_accounts_damm_v2_program: &'me AccountInfo<'info>,
    pub meteora_claim_position_fees_accounts_damm_v2_event_authority: &'me AccountInfo<
        'info,
    >,
    pub meteora_claim_position_fees_accounts_pool_authority: &'me AccountInfo<'info>,
    pub meteora_claim_position_fees_accounts_pool: &'me AccountInfo<'info>,
    pub meteora_claim_position_fees_accounts_position: &'me AccountInfo<'info>,
    pub meteora_claim_position_fees_accounts_token_a_account: &'me AccountInfo<'info>,
    pub meteora_claim_position_fees_accounts_token_b_account: &'me AccountInfo<'info>,
    pub meteora_claim_position_fees_accounts_token_a_vault: &'me AccountInfo<'info>,
    pub meteora_claim_position_fees_accounts_token_b_vault: &'me AccountInfo<'info>,
    pub meteora_claim_position_fees_accounts_token_a_mint: &'me AccountInfo<'info>,
    pub meteora_claim_position_fees_accounts_token_b_mint: &'me AccountInfo<'info>,
    pub meteora_claim_position_fees_accounts_position_nft_account: &'me AccountInfo<
        'info,
    >,
    pub meteora_claim_position_fees_accounts_owner: &'me AccountInfo<'info>,
    pub meteora_claim_position_fees_accounts_token_a_program: &'me AccountInfo<'info>,
    pub meteora_claim_position_fees_accounts_token_b_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub squads_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CollectMeteoraDammFeesKeys {
    pub dao: Pubkey,
    pub admin: Pubkey,
    pub squads_multisig: Pubkey,
    pub squads_multisig_vault: Pubkey,
    pub squads_multisig_vault_transaction: Pubkey,
    pub squads_multisig_proposal: Pubkey,
    pub squads_multisig_permissionless_account: Pubkey,
    pub meteora_claim_position_fees_accounts_damm_v2_program: Pubkey,
    pub meteora_claim_position_fees_accounts_damm_v2_event_authority: Pubkey,
    pub meteora_claim_position_fees_accounts_pool_authority: Pubkey,
    pub meteora_claim_position_fees_accounts_pool: Pubkey,
    pub meteora_claim_position_fees_accounts_position: Pubkey,
    pub meteora_claim_position_fees_accounts_token_a_account: Pubkey,
    pub meteora_claim_position_fees_accounts_token_b_account: Pubkey,
    pub meteora_claim_position_fees_accounts_token_a_vault: Pubkey,
    pub meteora_claim_position_fees_accounts_token_b_vault: Pubkey,
    pub meteora_claim_position_fees_accounts_token_a_mint: Pubkey,
    pub meteora_claim_position_fees_accounts_token_b_mint: Pubkey,
    pub meteora_claim_position_fees_accounts_position_nft_account: Pubkey,
    pub meteora_claim_position_fees_accounts_owner: Pubkey,
    pub meteora_claim_position_fees_accounts_token_a_program: Pubkey,
    pub meteora_claim_position_fees_accounts_token_b_program: Pubkey,
    pub system_program: Pubkey,
    pub token_program: Pubkey,
    pub squads_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<CollectMeteoraDammFeesAccounts<'_, '_>> for CollectMeteoraDammFeesKeys {
    fn from(accounts: CollectMeteoraDammFeesAccounts) -> Self {
        Self {
            dao: *accounts.dao.key,
            admin: *accounts.admin.key,
            squads_multisig: *accounts.squads_multisig.key,
            squads_multisig_vault: *accounts.squads_multisig_vault.key,
            squads_multisig_vault_transaction: *accounts
                .squads_multisig_vault_transaction
                .key,
            squads_multisig_proposal: *accounts.squads_multisig_proposal.key,
            squads_multisig_permissionless_account: *accounts
                .squads_multisig_permissionless_account
                .key,
            meteora_claim_position_fees_accounts_damm_v2_program: *accounts
                .meteora_claim_position_fees_accounts_damm_v2_program
                .key,
            meteora_claim_position_fees_accounts_damm_v2_event_authority: *accounts
                .meteora_claim_position_fees_accounts_damm_v2_event_authority
                .key,
            meteora_claim_position_fees_accounts_pool_authority: *accounts
                .meteora_claim_position_fees_accounts_pool_authority
                .key,
            meteora_claim_position_fees_accounts_pool: *accounts
                .meteora_claim_position_fees_accounts_pool
                .key,
            meteora_claim_position_fees_accounts_position: *accounts
                .meteora_claim_position_fees_accounts_position
                .key,
            meteora_claim_position_fees_accounts_token_a_account: *accounts
                .meteora_claim_position_fees_accounts_token_a_account
                .key,
            meteora_claim_position_fees_accounts_token_b_account: *accounts
                .meteora_claim_position_fees_accounts_token_b_account
                .key,
            meteora_claim_position_fees_accounts_token_a_vault: *accounts
                .meteora_claim_position_fees_accounts_token_a_vault
                .key,
            meteora_claim_position_fees_accounts_token_b_vault: *accounts
                .meteora_claim_position_fees_accounts_token_b_vault
                .key,
            meteora_claim_position_fees_accounts_token_a_mint: *accounts
                .meteora_claim_position_fees_accounts_token_a_mint
                .key,
            meteora_claim_position_fees_accounts_token_b_mint: *accounts
                .meteora_claim_position_fees_accounts_token_b_mint
                .key,
            meteora_claim_position_fees_accounts_position_nft_account: *accounts
                .meteora_claim_position_fees_accounts_position_nft_account
                .key,
            meteora_claim_position_fees_accounts_owner: *accounts
                .meteora_claim_position_fees_accounts_owner
                .key,
            meteora_claim_position_fees_accounts_token_a_program: *accounts
                .meteora_claim_position_fees_accounts_token_a_program
                .key,
            meteora_claim_position_fees_accounts_token_b_program: *accounts
                .meteora_claim_position_fees_accounts_token_b_program
                .key,
            system_program: *accounts.system_program.key,
            token_program: *accounts.token_program.key,
            squads_program: *accounts.squads_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<CollectMeteoraDammFeesKeys>
for [AccountMeta; COLLECT_METEORA_DAMM_FEES_IX_ACCOUNTS_LEN] {
    fn from(keys: CollectMeteoraDammFeesKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.dao,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.admin,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.squads_multisig,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.squads_multisig_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.squads_multisig_vault_transaction,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.squads_multisig_proposal,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.squads_multisig_permissionless_account,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.meteora_claim_position_fees_accounts_damm_v2_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys
                    .meteora_claim_position_fees_accounts_damm_v2_event_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.meteora_claim_position_fees_accounts_pool_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.meteora_claim_position_fees_accounts_pool,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.meteora_claim_position_fees_accounts_position,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.meteora_claim_position_fees_accounts_token_a_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.meteora_claim_position_fees_accounts_token_b_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.meteora_claim_position_fees_accounts_token_a_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.meteora_claim_position_fees_accounts_token_b_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.meteora_claim_position_fees_accounts_token_a_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.meteora_claim_position_fees_accounts_token_b_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.meteora_claim_position_fees_accounts_position_nft_account,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.meteora_claim_position_fees_accounts_owner,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.meteora_claim_position_fees_accounts_token_a_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.meteora_claim_position_fees_accounts_token_b_program,
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
                pubkey: keys.squads_program,
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
impl From<[Pubkey; COLLECT_METEORA_DAMM_FEES_IX_ACCOUNTS_LEN]>
for CollectMeteoraDammFeesKeys {
    fn from(pubkeys: [Pubkey; COLLECT_METEORA_DAMM_FEES_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            dao: pubkeys[0],
            admin: pubkeys[1],
            squads_multisig: pubkeys[2],
            squads_multisig_vault: pubkeys[3],
            squads_multisig_vault_transaction: pubkeys[4],
            squads_multisig_proposal: pubkeys[5],
            squads_multisig_permissionless_account: pubkeys[6],
            meteora_claim_position_fees_accounts_damm_v2_program: pubkeys[7],
            meteora_claim_position_fees_accounts_damm_v2_event_authority: pubkeys[8],
            meteora_claim_position_fees_accounts_pool_authority: pubkeys[9],
            meteora_claim_position_fees_accounts_pool: pubkeys[10],
            meteora_claim_position_fees_accounts_position: pubkeys[11],
            meteora_claim_position_fees_accounts_token_a_account: pubkeys[12],
            meteora_claim_position_fees_accounts_token_b_account: pubkeys[13],
            meteora_claim_position_fees_accounts_token_a_vault: pubkeys[14],
            meteora_claim_position_fees_accounts_token_b_vault: pubkeys[15],
            meteora_claim_position_fees_accounts_token_a_mint: pubkeys[16],
            meteora_claim_position_fees_accounts_token_b_mint: pubkeys[17],
            meteora_claim_position_fees_accounts_position_nft_account: pubkeys[18],
            meteora_claim_position_fees_accounts_owner: pubkeys[19],
            meteora_claim_position_fees_accounts_token_a_program: pubkeys[20],
            meteora_claim_position_fees_accounts_token_b_program: pubkeys[21],
            system_program: pubkeys[22],
            token_program: pubkeys[23],
            squads_program: pubkeys[24],
            event_authority: pubkeys[25],
            program: pubkeys[26],
        }
    }
}
impl<'info> From<CollectMeteoraDammFeesAccounts<'_, 'info>>
for [AccountInfo<'info>; COLLECT_METEORA_DAMM_FEES_IX_ACCOUNTS_LEN] {
    fn from(accounts: CollectMeteoraDammFeesAccounts<'_, 'info>) -> Self {
        [
            accounts.dao.clone(),
            accounts.admin.clone(),
            accounts.squads_multisig.clone(),
            accounts.squads_multisig_vault.clone(),
            accounts.squads_multisig_vault_transaction.clone(),
            accounts.squads_multisig_proposal.clone(),
            accounts.squads_multisig_permissionless_account.clone(),
            accounts.meteora_claim_position_fees_accounts_damm_v2_program.clone(),
            accounts
                .meteora_claim_position_fees_accounts_damm_v2_event_authority
                .clone(),
            accounts.meteora_claim_position_fees_accounts_pool_authority.clone(),
            accounts.meteora_claim_position_fees_accounts_pool.clone(),
            accounts.meteora_claim_position_fees_accounts_position.clone(),
            accounts.meteora_claim_position_fees_accounts_token_a_account.clone(),
            accounts.meteora_claim_position_fees_accounts_token_b_account.clone(),
            accounts.meteora_claim_position_fees_accounts_token_a_vault.clone(),
            accounts.meteora_claim_position_fees_accounts_token_b_vault.clone(),
            accounts.meteora_claim_position_fees_accounts_token_a_mint.clone(),
            accounts.meteora_claim_position_fees_accounts_token_b_mint.clone(),
            accounts.meteora_claim_position_fees_accounts_position_nft_account.clone(),
            accounts.meteora_claim_position_fees_accounts_owner.clone(),
            accounts.meteora_claim_position_fees_accounts_token_a_program.clone(),
            accounts.meteora_claim_position_fees_accounts_token_b_program.clone(),
            accounts.system_program.clone(),
            accounts.token_program.clone(),
            accounts.squads_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; COLLECT_METEORA_DAMM_FEES_IX_ACCOUNTS_LEN]>
for CollectMeteoraDammFeesAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; COLLECT_METEORA_DAMM_FEES_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            dao: &arr[0],
            admin: &arr[1],
            squads_multisig: &arr[2],
            squads_multisig_vault: &arr[3],
            squads_multisig_vault_transaction: &arr[4],
            squads_multisig_proposal: &arr[5],
            squads_multisig_permissionless_account: &arr[6],
            meteora_claim_position_fees_accounts_damm_v2_program: &arr[7],
            meteora_claim_position_fees_accounts_damm_v2_event_authority: &arr[8],
            meteora_claim_position_fees_accounts_pool_authority: &arr[9],
            meteora_claim_position_fees_accounts_pool: &arr[10],
            meteora_claim_position_fees_accounts_position: &arr[11],
            meteora_claim_position_fees_accounts_token_a_account: &arr[12],
            meteora_claim_position_fees_accounts_token_b_account: &arr[13],
            meteora_claim_position_fees_accounts_token_a_vault: &arr[14],
            meteora_claim_position_fees_accounts_token_b_vault: &arr[15],
            meteora_claim_position_fees_accounts_token_a_mint: &arr[16],
            meteora_claim_position_fees_accounts_token_b_mint: &arr[17],
            meteora_claim_position_fees_accounts_position_nft_account: &arr[18],
            meteora_claim_position_fees_accounts_owner: &arr[19],
            meteora_claim_position_fees_accounts_token_a_program: &arr[20],
            meteora_claim_position_fees_accounts_token_b_program: &arr[21],
            system_program: &arr[22],
            token_program: &arr[23],
            squads_program: &arr[24],
            event_authority: &arr[25],
            program: &arr[26],
        }
    }
}
pub const COLLECT_METEORA_DAMM_FEES_IX_DISCM: [u8; 8usize] = [
    139, 212, 105, 118, 126, 54, 214, 143,
];
#[derive(Clone, Debug, PartialEq)]
pub struct CollectMeteoraDammFeesIxData;
impl CollectMeteoraDammFeesIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != COLLECT_METEORA_DAMM_FEES_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&COLLECT_METEORA_DAMM_FEES_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn collect_meteora_damm_fees_ix_with_program_id(
    program_id: Pubkey,
    keys: CollectMeteoraDammFeesKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; COLLECT_METEORA_DAMM_FEES_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: CollectMeteoraDammFeesIxData.try_to_vec()?,
    })
}
pub fn collect_meteora_damm_fees_ix(
    keys: CollectMeteoraDammFeesKeys,
) -> std::io::Result<Instruction> {
    collect_meteora_damm_fees_ix_with_program_id(FUTARCHY_PROGRAM_ID, keys)
}
pub fn collect_meteora_damm_fees_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CollectMeteoraDammFeesAccounts<'_, '_>,
) -> ProgramResult {
    let keys: CollectMeteoraDammFeesKeys = accounts.into();
    let ix = collect_meteora_damm_fees_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn collect_meteora_damm_fees_invoke(
    accounts: CollectMeteoraDammFeesAccounts<'_, '_>,
) -> ProgramResult {
    collect_meteora_damm_fees_invoke_with_program_id(FUTARCHY_PROGRAM_ID, accounts)
}
pub fn collect_meteora_damm_fees_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CollectMeteoraDammFeesAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CollectMeteoraDammFeesKeys = accounts.into();
    let ix = collect_meteora_damm_fees_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn collect_meteora_damm_fees_invoke_signed(
    accounts: CollectMeteoraDammFeesAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    collect_meteora_damm_fees_invoke_signed_with_program_id(
        FUTARCHY_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn collect_meteora_damm_fees_verify_account_keys(
    accounts: CollectMeteoraDammFeesAccounts<'_, '_>,
    keys: CollectMeteoraDammFeesKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.dao.key, keys.dao),
        (*accounts.admin.key, keys.admin),
        (*accounts.squads_multisig.key, keys.squads_multisig),
        (*accounts.squads_multisig_vault.key, keys.squads_multisig_vault),
        (
            *accounts.squads_multisig_vault_transaction.key,
            keys.squads_multisig_vault_transaction,
        ),
        (*accounts.squads_multisig_proposal.key, keys.squads_multisig_proposal),
        (
            *accounts.squads_multisig_permissionless_account.key,
            keys.squads_multisig_permissionless_account,
        ),
        (
            *accounts.meteora_claim_position_fees_accounts_damm_v2_program.key,
            keys.meteora_claim_position_fees_accounts_damm_v2_program,
        ),
        (
            *accounts.meteora_claim_position_fees_accounts_damm_v2_event_authority.key,
            keys.meteora_claim_position_fees_accounts_damm_v2_event_authority,
        ),
        (
            *accounts.meteora_claim_position_fees_accounts_pool_authority.key,
            keys.meteora_claim_position_fees_accounts_pool_authority,
        ),
        (
            *accounts.meteora_claim_position_fees_accounts_pool.key,
            keys.meteora_claim_position_fees_accounts_pool,
        ),
        (
            *accounts.meteora_claim_position_fees_accounts_position.key,
            keys.meteora_claim_position_fees_accounts_position,
        ),
        (
            *accounts.meteora_claim_position_fees_accounts_token_a_account.key,
            keys.meteora_claim_position_fees_accounts_token_a_account,
        ),
        (
            *accounts.meteora_claim_position_fees_accounts_token_b_account.key,
            keys.meteora_claim_position_fees_accounts_token_b_account,
        ),
        (
            *accounts.meteora_claim_position_fees_accounts_token_a_vault.key,
            keys.meteora_claim_position_fees_accounts_token_a_vault,
        ),
        (
            *accounts.meteora_claim_position_fees_accounts_token_b_vault.key,
            keys.meteora_claim_position_fees_accounts_token_b_vault,
        ),
        (
            *accounts.meteora_claim_position_fees_accounts_token_a_mint.key,
            keys.meteora_claim_position_fees_accounts_token_a_mint,
        ),
        (
            *accounts.meteora_claim_position_fees_accounts_token_b_mint.key,
            keys.meteora_claim_position_fees_accounts_token_b_mint,
        ),
        (
            *accounts.meteora_claim_position_fees_accounts_position_nft_account.key,
            keys.meteora_claim_position_fees_accounts_position_nft_account,
        ),
        (
            *accounts.meteora_claim_position_fees_accounts_owner.key,
            keys.meteora_claim_position_fees_accounts_owner,
        ),
        (
            *accounts.meteora_claim_position_fees_accounts_token_a_program.key,
            keys.meteora_claim_position_fees_accounts_token_a_program,
        ),
        (
            *accounts.meteora_claim_position_fees_accounts_token_b_program.key,
            keys.meteora_claim_position_fees_accounts_token_b_program,
        ),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.squads_program.key, keys.squads_program),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn collect_meteora_damm_fees_verify_writable_privileges<'me, 'info>(
    accounts: CollectMeteoraDammFeesAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.dao,
        accounts.admin,
        accounts.squads_multisig,
        accounts.squads_multisig_vault,
        accounts.squads_multisig_vault_transaction,
        accounts.squads_multisig_proposal,
        accounts.meteora_claim_position_fees_accounts_position,
        accounts.meteora_claim_position_fees_accounts_token_a_account,
        accounts.meteora_claim_position_fees_accounts_token_b_account,
        accounts.meteora_claim_position_fees_accounts_token_a_vault,
        accounts.meteora_claim_position_fees_accounts_token_b_vault,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn collect_meteora_damm_fees_verify_signer_privileges<'me, 'info>(
    accounts: CollectMeteoraDammFeesAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [
        accounts.admin,
        accounts.squads_multisig_permissionless_account,
    ] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn collect_meteora_damm_fees_verify_account_privileges<'me, 'info>(
    accounts: CollectMeteoraDammFeesAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    collect_meteora_damm_fees_verify_writable_privileges(accounts)?;
    collect_meteora_damm_fees_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const INITIATE_VAULT_SPEND_OPTIMISTIC_PROPOSAL_IX_ACCOUNTS_LEN: usize = 14;
#[derive(Copy, Clone, Debug)]
pub struct InitiateVaultSpendOptimisticProposalAccounts<'me, 'info> {
    pub squads_multisig: &'me AccountInfo<'info>,
    pub squads_multisig_vault: &'me AccountInfo<'info>,
    pub squads_spending_limit: &'me AccountInfo<'info>,
    pub squads_proposal: &'me AccountInfo<'info>,
    pub squads_vault_transaction: &'me AccountInfo<'info>,
    pub dao: &'me AccountInfo<'info>,
    pub dao_quote_vault_account: &'me AccountInfo<'info>,
    pub proposer: &'me AccountInfo<'info>,
    pub recipient: &'me AccountInfo<'info>,
    pub recipient_quote_account: &'me AccountInfo<'info>,
    pub squads_program: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct InitiateVaultSpendOptimisticProposalKeys {
    pub squads_multisig: Pubkey,
    pub squads_multisig_vault: Pubkey,
    pub squads_spending_limit: Pubkey,
    pub squads_proposal: Pubkey,
    pub squads_vault_transaction: Pubkey,
    pub dao: Pubkey,
    pub dao_quote_vault_account: Pubkey,
    pub proposer: Pubkey,
    pub recipient: Pubkey,
    pub recipient_quote_account: Pubkey,
    pub squads_program: Pubkey,
    pub token_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<InitiateVaultSpendOptimisticProposalAccounts<'_, '_>>
for InitiateVaultSpendOptimisticProposalKeys {
    fn from(accounts: InitiateVaultSpendOptimisticProposalAccounts) -> Self {
        Self {
            squads_multisig: *accounts.squads_multisig.key,
            squads_multisig_vault: *accounts.squads_multisig_vault.key,
            squads_spending_limit: *accounts.squads_spending_limit.key,
            squads_proposal: *accounts.squads_proposal.key,
            squads_vault_transaction: *accounts.squads_vault_transaction.key,
            dao: *accounts.dao.key,
            dao_quote_vault_account: *accounts.dao_quote_vault_account.key,
            proposer: *accounts.proposer.key,
            recipient: *accounts.recipient.key,
            recipient_quote_account: *accounts.recipient_quote_account.key,
            squads_program: *accounts.squads_program.key,
            token_program: *accounts.token_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<InitiateVaultSpendOptimisticProposalKeys>
for [AccountMeta; INITIATE_VAULT_SPEND_OPTIMISTIC_PROPOSAL_IX_ACCOUNTS_LEN] {
    fn from(keys: InitiateVaultSpendOptimisticProposalKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.squads_multisig,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.squads_multisig_vault,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.squads_spending_limit,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.squads_proposal,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.squads_vault_transaction,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.dao,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.dao_quote_vault_account,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.proposer,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.recipient,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.recipient_quote_account,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.squads_program,
                is_signer: false,
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
impl From<[Pubkey; INITIATE_VAULT_SPEND_OPTIMISTIC_PROPOSAL_IX_ACCOUNTS_LEN]>
for InitiateVaultSpendOptimisticProposalKeys {
    fn from(
        pubkeys: [Pubkey; INITIATE_VAULT_SPEND_OPTIMISTIC_PROPOSAL_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            squads_multisig: pubkeys[0],
            squads_multisig_vault: pubkeys[1],
            squads_spending_limit: pubkeys[2],
            squads_proposal: pubkeys[3],
            squads_vault_transaction: pubkeys[4],
            dao: pubkeys[5],
            dao_quote_vault_account: pubkeys[6],
            proposer: pubkeys[7],
            recipient: pubkeys[8],
            recipient_quote_account: pubkeys[9],
            squads_program: pubkeys[10],
            token_program: pubkeys[11],
            event_authority: pubkeys[12],
            program: pubkeys[13],
        }
    }
}
impl<'info> From<InitiateVaultSpendOptimisticProposalAccounts<'_, 'info>>
for [AccountInfo<'info>; INITIATE_VAULT_SPEND_OPTIMISTIC_PROPOSAL_IX_ACCOUNTS_LEN] {
    fn from(accounts: InitiateVaultSpendOptimisticProposalAccounts<'_, 'info>) -> Self {
        [
            accounts.squads_multisig.clone(),
            accounts.squads_multisig_vault.clone(),
            accounts.squads_spending_limit.clone(),
            accounts.squads_proposal.clone(),
            accounts.squads_vault_transaction.clone(),
            accounts.dao.clone(),
            accounts.dao_quote_vault_account.clone(),
            accounts.proposer.clone(),
            accounts.recipient.clone(),
            accounts.recipient_quote_account.clone(),
            accounts.squads_program.clone(),
            accounts.token_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<
    &'me [AccountInfo<'info>; INITIATE_VAULT_SPEND_OPTIMISTIC_PROPOSAL_IX_ACCOUNTS_LEN],
> for InitiateVaultSpendOptimisticProposalAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<
            'info,
        >; INITIATE_VAULT_SPEND_OPTIMISTIC_PROPOSAL_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            squads_multisig: &arr[0],
            squads_multisig_vault: &arr[1],
            squads_spending_limit: &arr[2],
            squads_proposal: &arr[3],
            squads_vault_transaction: &arr[4],
            dao: &arr[5],
            dao_quote_vault_account: &arr[6],
            proposer: &arr[7],
            recipient: &arr[8],
            recipient_quote_account: &arr[9],
            squads_program: &arr[10],
            token_program: &arr[11],
            event_authority: &arr[12],
            program: &arr[13],
        }
    }
}
pub const INITIATE_VAULT_SPEND_OPTIMISTIC_PROPOSAL_IX_DISCM: [u8; 8usize] = [
    83, 24, 40, 200, 88, 239, 19, 255,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct InitiateVaultSpendOptimisticProposalIxArgs {
    pub params: InitiateVaultSpendOptimisticProposalParams,
}
#[derive(Clone, Debug, PartialEq)]
pub struct InitiateVaultSpendOptimisticProposalIxData(
    pub InitiateVaultSpendOptimisticProposalIxArgs,
);
impl From<InitiateVaultSpendOptimisticProposalIxArgs>
for InitiateVaultSpendOptimisticProposalIxData {
    fn from(args: InitiateVaultSpendOptimisticProposalIxArgs) -> Self {
        Self(args)
    }
}
impl InitiateVaultSpendOptimisticProposalIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INITIATE_VAULT_SPEND_OPTIMISTIC_PROPOSAL_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let params = if reader.is_empty() {
            Default::default()
        } else {
            <InitiateVaultSpendOptimisticProposalParams>::deserialize(&mut reader)?
        };
        Ok(
            Self(InitiateVaultSpendOptimisticProposalIxArgs {
                params,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INITIATE_VAULT_SPEND_OPTIMISTIC_PROPOSAL_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.params, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn initiate_vault_spend_optimistic_proposal_ix_with_program_id(
    program_id: Pubkey,
    keys: InitiateVaultSpendOptimisticProposalKeys,
    args: InitiateVaultSpendOptimisticProposalIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; INITIATE_VAULT_SPEND_OPTIMISTIC_PROPOSAL_IX_ACCOUNTS_LEN] = keys
        .into();
    let data: InitiateVaultSpendOptimisticProposalIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn initiate_vault_spend_optimistic_proposal_ix(
    keys: InitiateVaultSpendOptimisticProposalKeys,
    args: InitiateVaultSpendOptimisticProposalIxArgs,
) -> std::io::Result<Instruction> {
    initiate_vault_spend_optimistic_proposal_ix_with_program_id(
        FUTARCHY_PROGRAM_ID,
        keys,
        args,
    )
}
pub fn initiate_vault_spend_optimistic_proposal_invoke_with_program_id(
    program_id: Pubkey,
    accounts: InitiateVaultSpendOptimisticProposalAccounts<'_, '_>,
    args: InitiateVaultSpendOptimisticProposalIxArgs,
) -> ProgramResult {
    let keys: InitiateVaultSpendOptimisticProposalKeys = accounts.into();
    let ix = initiate_vault_spend_optimistic_proposal_ix_with_program_id(
        program_id,
        keys,
        args,
    )?;
    invoke_instruction(&ix, accounts)
}
pub fn initiate_vault_spend_optimistic_proposal_invoke(
    accounts: InitiateVaultSpendOptimisticProposalAccounts<'_, '_>,
    args: InitiateVaultSpendOptimisticProposalIxArgs,
) -> ProgramResult {
    initiate_vault_spend_optimistic_proposal_invoke_with_program_id(
        FUTARCHY_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn initiate_vault_spend_optimistic_proposal_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: InitiateVaultSpendOptimisticProposalAccounts<'_, '_>,
    args: InitiateVaultSpendOptimisticProposalIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: InitiateVaultSpendOptimisticProposalKeys = accounts.into();
    let ix = initiate_vault_spend_optimistic_proposal_ix_with_program_id(
        program_id,
        keys,
        args,
    )?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn initiate_vault_spend_optimistic_proposal_invoke_signed(
    accounts: InitiateVaultSpendOptimisticProposalAccounts<'_, '_>,
    args: InitiateVaultSpendOptimisticProposalIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    initiate_vault_spend_optimistic_proposal_invoke_signed_with_program_id(
        FUTARCHY_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn initiate_vault_spend_optimistic_proposal_verify_account_keys(
    accounts: InitiateVaultSpendOptimisticProposalAccounts<'_, '_>,
    keys: InitiateVaultSpendOptimisticProposalKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.squads_multisig.key, keys.squads_multisig),
        (*accounts.squads_multisig_vault.key, keys.squads_multisig_vault),
        (*accounts.squads_spending_limit.key, keys.squads_spending_limit),
        (*accounts.squads_proposal.key, keys.squads_proposal),
        (*accounts.squads_vault_transaction.key, keys.squads_vault_transaction),
        (*accounts.dao.key, keys.dao),
        (*accounts.dao_quote_vault_account.key, keys.dao_quote_vault_account),
        (*accounts.proposer.key, keys.proposer),
        (*accounts.recipient.key, keys.recipient),
        (*accounts.recipient_quote_account.key, keys.recipient_quote_account),
        (*accounts.squads_program.key, keys.squads_program),
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
pub fn initiate_vault_spend_optimistic_proposal_verify_writable_privileges<'me, 'info>(
    accounts: InitiateVaultSpendOptimisticProposalAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.dao] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn initiate_vault_spend_optimistic_proposal_verify_signer_privileges<'me, 'info>(
    accounts: InitiateVaultSpendOptimisticProposalAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.proposer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn initiate_vault_spend_optimistic_proposal_verify_account_privileges<'me, 'info>(
    accounts: InitiateVaultSpendOptimisticProposalAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    initiate_vault_spend_optimistic_proposal_verify_writable_privileges(accounts)?;
    initiate_vault_spend_optimistic_proposal_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const FINALIZE_OPTIMISTIC_PROPOSAL_IX_ACCOUNTS_LEN: usize = 6;
#[derive(Copy, Clone, Debug)]
pub struct FinalizeOptimisticProposalAccounts<'me, 'info> {
    pub squads_multisig: &'me AccountInfo<'info>,
    pub squads_proposal: &'me AccountInfo<'info>,
    pub dao: &'me AccountInfo<'info>,
    pub squads_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FinalizeOptimisticProposalKeys {
    pub squads_multisig: Pubkey,
    pub squads_proposal: Pubkey,
    pub dao: Pubkey,
    pub squads_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<FinalizeOptimisticProposalAccounts<'_, '_>>
for FinalizeOptimisticProposalKeys {
    fn from(accounts: FinalizeOptimisticProposalAccounts) -> Self {
        Self {
            squads_multisig: *accounts.squads_multisig.key,
            squads_proposal: *accounts.squads_proposal.key,
            dao: *accounts.dao.key,
            squads_program: *accounts.squads_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<FinalizeOptimisticProposalKeys>
for [AccountMeta; FINALIZE_OPTIMISTIC_PROPOSAL_IX_ACCOUNTS_LEN] {
    fn from(keys: FinalizeOptimisticProposalKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.squads_multisig,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.squads_proposal,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.dao,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.squads_program,
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
impl From<[Pubkey; FINALIZE_OPTIMISTIC_PROPOSAL_IX_ACCOUNTS_LEN]>
for FinalizeOptimisticProposalKeys {
    fn from(pubkeys: [Pubkey; FINALIZE_OPTIMISTIC_PROPOSAL_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            squads_multisig: pubkeys[0],
            squads_proposal: pubkeys[1],
            dao: pubkeys[2],
            squads_program: pubkeys[3],
            event_authority: pubkeys[4],
            program: pubkeys[5],
        }
    }
}
impl<'info> From<FinalizeOptimisticProposalAccounts<'_, 'info>>
for [AccountInfo<'info>; FINALIZE_OPTIMISTIC_PROPOSAL_IX_ACCOUNTS_LEN] {
    fn from(accounts: FinalizeOptimisticProposalAccounts<'_, 'info>) -> Self {
        [
            accounts.squads_multisig.clone(),
            accounts.squads_proposal.clone(),
            accounts.dao.clone(),
            accounts.squads_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; FINALIZE_OPTIMISTIC_PROPOSAL_IX_ACCOUNTS_LEN]>
for FinalizeOptimisticProposalAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; FINALIZE_OPTIMISTIC_PROPOSAL_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            squads_multisig: &arr[0],
            squads_proposal: &arr[1],
            dao: &arr[2],
            squads_program: &arr[3],
            event_authority: &arr[4],
            program: &arr[5],
        }
    }
}
pub const FINALIZE_OPTIMISTIC_PROPOSAL_IX_DISCM: [u8; 8usize] = [
    71, 146, 38, 160, 89, 15, 48, 19,
];
#[derive(Clone, Debug, PartialEq)]
pub struct FinalizeOptimisticProposalIxData;
impl FinalizeOptimisticProposalIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != FINALIZE_OPTIMISTIC_PROPOSAL_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&FINALIZE_OPTIMISTIC_PROPOSAL_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn finalize_optimistic_proposal_ix_with_program_id(
    program_id: Pubkey,
    keys: FinalizeOptimisticProposalKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; FINALIZE_OPTIMISTIC_PROPOSAL_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: FinalizeOptimisticProposalIxData.try_to_vec()?,
    })
}
pub fn finalize_optimistic_proposal_ix(
    keys: FinalizeOptimisticProposalKeys,
) -> std::io::Result<Instruction> {
    finalize_optimistic_proposal_ix_with_program_id(FUTARCHY_PROGRAM_ID, keys)
}
pub fn finalize_optimistic_proposal_invoke_with_program_id(
    program_id: Pubkey,
    accounts: FinalizeOptimisticProposalAccounts<'_, '_>,
) -> ProgramResult {
    let keys: FinalizeOptimisticProposalKeys = accounts.into();
    let ix = finalize_optimistic_proposal_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn finalize_optimistic_proposal_invoke(
    accounts: FinalizeOptimisticProposalAccounts<'_, '_>,
) -> ProgramResult {
    finalize_optimistic_proposal_invoke_with_program_id(FUTARCHY_PROGRAM_ID, accounts)
}
pub fn finalize_optimistic_proposal_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: FinalizeOptimisticProposalAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: FinalizeOptimisticProposalKeys = accounts.into();
    let ix = finalize_optimistic_proposal_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn finalize_optimistic_proposal_invoke_signed(
    accounts: FinalizeOptimisticProposalAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    finalize_optimistic_proposal_invoke_signed_with_program_id(
        FUTARCHY_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn finalize_optimistic_proposal_verify_account_keys(
    accounts: FinalizeOptimisticProposalAccounts<'_, '_>,
    keys: FinalizeOptimisticProposalKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.squads_multisig.key, keys.squads_multisig),
        (*accounts.squads_proposal.key, keys.squads_proposal),
        (*accounts.dao.key, keys.dao),
        (*accounts.squads_program.key, keys.squads_program),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn finalize_optimistic_proposal_verify_writable_privileges<'me, 'info>(
    accounts: FinalizeOptimisticProposalAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.squads_multisig,
        accounts.squads_proposal,
        accounts.dao,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn finalize_optimistic_proposal_verify_account_privileges<'me, 'info>(
    accounts: FinalizeOptimisticProposalAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    finalize_optimistic_proposal_verify_writable_privileges(accounts)?;
    Ok(())
}
pub const ADMIN_ENQUEUE_MULTISIG_PROPOSAL_APPROVAL_IX_ACCOUNTS_LEN: usize = 6;
#[derive(Copy, Clone, Debug)]
pub struct AdminEnqueueMultisigProposalApprovalAccounts<'me, 'info> {
    pub dao: &'me AccountInfo<'info>,
    pub admin: &'me AccountInfo<'info>,
    pub squads_multisig: &'me AccountInfo<'info>,
    pub squads_multisig_proposal: &'me AccountInfo<'info>,
    pub enqueued_approval: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct AdminEnqueueMultisigProposalApprovalKeys {
    pub dao: Pubkey,
    pub admin: Pubkey,
    pub squads_multisig: Pubkey,
    pub squads_multisig_proposal: Pubkey,
    pub enqueued_approval: Pubkey,
    pub system_program: Pubkey,
}
impl From<AdminEnqueueMultisigProposalApprovalAccounts<'_, '_>>
for AdminEnqueueMultisigProposalApprovalKeys {
    fn from(accounts: AdminEnqueueMultisigProposalApprovalAccounts) -> Self {
        Self {
            dao: *accounts.dao.key,
            admin: *accounts.admin.key,
            squads_multisig: *accounts.squads_multisig.key,
            squads_multisig_proposal: *accounts.squads_multisig_proposal.key,
            enqueued_approval: *accounts.enqueued_approval.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<AdminEnqueueMultisigProposalApprovalKeys>
for [AccountMeta; ADMIN_ENQUEUE_MULTISIG_PROPOSAL_APPROVAL_IX_ACCOUNTS_LEN] {
    fn from(keys: AdminEnqueueMultisigProposalApprovalKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.dao,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.admin,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.squads_multisig,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.squads_multisig_proposal,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.enqueued_approval,
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
impl From<[Pubkey; ADMIN_ENQUEUE_MULTISIG_PROPOSAL_APPROVAL_IX_ACCOUNTS_LEN]>
for AdminEnqueueMultisigProposalApprovalKeys {
    fn from(
        pubkeys: [Pubkey; ADMIN_ENQUEUE_MULTISIG_PROPOSAL_APPROVAL_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            dao: pubkeys[0],
            admin: pubkeys[1],
            squads_multisig: pubkeys[2],
            squads_multisig_proposal: pubkeys[3],
            enqueued_approval: pubkeys[4],
            system_program: pubkeys[5],
        }
    }
}
impl<'info> From<AdminEnqueueMultisigProposalApprovalAccounts<'_, 'info>>
for [AccountInfo<'info>; ADMIN_ENQUEUE_MULTISIG_PROPOSAL_APPROVAL_IX_ACCOUNTS_LEN] {
    fn from(accounts: AdminEnqueueMultisigProposalApprovalAccounts<'_, 'info>) -> Self {
        [
            accounts.dao.clone(),
            accounts.admin.clone(),
            accounts.squads_multisig.clone(),
            accounts.squads_multisig_proposal.clone(),
            accounts.enqueued_approval.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<
    &'me [AccountInfo<'info>; ADMIN_ENQUEUE_MULTISIG_PROPOSAL_APPROVAL_IX_ACCOUNTS_LEN],
> for AdminEnqueueMultisigProposalApprovalAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<
            'info,
        >; ADMIN_ENQUEUE_MULTISIG_PROPOSAL_APPROVAL_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            dao: &arr[0],
            admin: &arr[1],
            squads_multisig: &arr[2],
            squads_multisig_proposal: &arr[3],
            enqueued_approval: &arr[4],
            system_program: &arr[5],
        }
    }
}
pub const ADMIN_ENQUEUE_MULTISIG_PROPOSAL_APPROVAL_IX_DISCM: [u8; 8usize] = [
    61, 199, 241, 100, 238, 220, 84, 78,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AdminEnqueueMultisigProposalApprovalIxArgs {
    pub args: AdminEnqueueMultisigProposalApprovalArgs,
}
#[derive(Clone, Debug, PartialEq)]
pub struct AdminEnqueueMultisigProposalApprovalIxData(
    pub AdminEnqueueMultisigProposalApprovalIxArgs,
);
impl From<AdminEnqueueMultisigProposalApprovalIxArgs>
for AdminEnqueueMultisigProposalApprovalIxData {
    fn from(args: AdminEnqueueMultisigProposalApprovalIxArgs) -> Self {
        Self(args)
    }
}
impl AdminEnqueueMultisigProposalApprovalIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != ADMIN_ENQUEUE_MULTISIG_PROPOSAL_APPROVAL_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let args = if reader.is_empty() {
            Default::default()
        } else {
            <AdminEnqueueMultisigProposalApprovalArgs>::deserialize(&mut reader)?
        };
        Ok(
            Self(AdminEnqueueMultisigProposalApprovalIxArgs {
                args,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&ADMIN_ENQUEUE_MULTISIG_PROPOSAL_APPROVAL_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.args, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn admin_enqueue_multisig_proposal_approval_ix_with_program_id(
    program_id: Pubkey,
    keys: AdminEnqueueMultisigProposalApprovalKeys,
    args: AdminEnqueueMultisigProposalApprovalIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; ADMIN_ENQUEUE_MULTISIG_PROPOSAL_APPROVAL_IX_ACCOUNTS_LEN] = keys
        .into();
    let data: AdminEnqueueMultisigProposalApprovalIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn admin_enqueue_multisig_proposal_approval_ix(
    keys: AdminEnqueueMultisigProposalApprovalKeys,
    args: AdminEnqueueMultisigProposalApprovalIxArgs,
) -> std::io::Result<Instruction> {
    admin_enqueue_multisig_proposal_approval_ix_with_program_id(
        FUTARCHY_PROGRAM_ID,
        keys,
        args,
    )
}
pub fn admin_enqueue_multisig_proposal_approval_invoke_with_program_id(
    program_id: Pubkey,
    accounts: AdminEnqueueMultisigProposalApprovalAccounts<'_, '_>,
    args: AdminEnqueueMultisigProposalApprovalIxArgs,
) -> ProgramResult {
    let keys: AdminEnqueueMultisigProposalApprovalKeys = accounts.into();
    let ix = admin_enqueue_multisig_proposal_approval_ix_with_program_id(
        program_id,
        keys,
        args,
    )?;
    invoke_instruction(&ix, accounts)
}
pub fn admin_enqueue_multisig_proposal_approval_invoke(
    accounts: AdminEnqueueMultisigProposalApprovalAccounts<'_, '_>,
    args: AdminEnqueueMultisigProposalApprovalIxArgs,
) -> ProgramResult {
    admin_enqueue_multisig_proposal_approval_invoke_with_program_id(
        FUTARCHY_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn admin_enqueue_multisig_proposal_approval_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: AdminEnqueueMultisigProposalApprovalAccounts<'_, '_>,
    args: AdminEnqueueMultisigProposalApprovalIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: AdminEnqueueMultisigProposalApprovalKeys = accounts.into();
    let ix = admin_enqueue_multisig_proposal_approval_ix_with_program_id(
        program_id,
        keys,
        args,
    )?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn admin_enqueue_multisig_proposal_approval_invoke_signed(
    accounts: AdminEnqueueMultisigProposalApprovalAccounts<'_, '_>,
    args: AdminEnqueueMultisigProposalApprovalIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    admin_enqueue_multisig_proposal_approval_invoke_signed_with_program_id(
        FUTARCHY_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn admin_enqueue_multisig_proposal_approval_verify_account_keys(
    accounts: AdminEnqueueMultisigProposalApprovalAccounts<'_, '_>,
    keys: AdminEnqueueMultisigProposalApprovalKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.dao.key, keys.dao),
        (*accounts.admin.key, keys.admin),
        (*accounts.squads_multisig.key, keys.squads_multisig),
        (*accounts.squads_multisig_proposal.key, keys.squads_multisig_proposal),
        (*accounts.enqueued_approval.key, keys.enqueued_approval),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn admin_enqueue_multisig_proposal_approval_verify_writable_privileges<'me, 'info>(
    accounts: AdminEnqueueMultisigProposalApprovalAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.admin, accounts.enqueued_approval] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn admin_enqueue_multisig_proposal_approval_verify_signer_privileges<'me, 'info>(
    accounts: AdminEnqueueMultisigProposalApprovalAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn admin_enqueue_multisig_proposal_approval_verify_account_privileges<'me, 'info>(
    accounts: AdminEnqueueMultisigProposalApprovalAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    admin_enqueue_multisig_proposal_approval_verify_writable_privileges(accounts)?;
    admin_enqueue_multisig_proposal_approval_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const EXECUTE_MULTISIG_PROPOSAL_APPROVAL_IX_ACCOUNTS_LEN: usize = 6;
#[derive(Copy, Clone, Debug)]
pub struct ExecuteMultisigProposalApprovalAccounts<'me, 'info> {
    pub dao: &'me AccountInfo<'info>,
    pub rent_receiver: &'me AccountInfo<'info>,
    pub squads_multisig: &'me AccountInfo<'info>,
    pub squads_multisig_proposal: &'me AccountInfo<'info>,
    pub enqueued_approval: &'me AccountInfo<'info>,
    pub squads_multisig_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ExecuteMultisigProposalApprovalKeys {
    pub dao: Pubkey,
    pub rent_receiver: Pubkey,
    pub squads_multisig: Pubkey,
    pub squads_multisig_proposal: Pubkey,
    pub enqueued_approval: Pubkey,
    pub squads_multisig_program: Pubkey,
}
impl From<ExecuteMultisigProposalApprovalAccounts<'_, '_>>
for ExecuteMultisigProposalApprovalKeys {
    fn from(accounts: ExecuteMultisigProposalApprovalAccounts) -> Self {
        Self {
            dao: *accounts.dao.key,
            rent_receiver: *accounts.rent_receiver.key,
            squads_multisig: *accounts.squads_multisig.key,
            squads_multisig_proposal: *accounts.squads_multisig_proposal.key,
            enqueued_approval: *accounts.enqueued_approval.key,
            squads_multisig_program: *accounts.squads_multisig_program.key,
        }
    }
}
impl From<ExecuteMultisigProposalApprovalKeys>
for [AccountMeta; EXECUTE_MULTISIG_PROPOSAL_APPROVAL_IX_ACCOUNTS_LEN] {
    fn from(keys: ExecuteMultisigProposalApprovalKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.dao,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.rent_receiver,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.squads_multisig,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.squads_multisig_proposal,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.enqueued_approval,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.squads_multisig_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; EXECUTE_MULTISIG_PROPOSAL_APPROVAL_IX_ACCOUNTS_LEN]>
for ExecuteMultisigProposalApprovalKeys {
    fn from(
        pubkeys: [Pubkey; EXECUTE_MULTISIG_PROPOSAL_APPROVAL_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            dao: pubkeys[0],
            rent_receiver: pubkeys[1],
            squads_multisig: pubkeys[2],
            squads_multisig_proposal: pubkeys[3],
            enqueued_approval: pubkeys[4],
            squads_multisig_program: pubkeys[5],
        }
    }
}
impl<'info> From<ExecuteMultisigProposalApprovalAccounts<'_, 'info>>
for [AccountInfo<'info>; EXECUTE_MULTISIG_PROPOSAL_APPROVAL_IX_ACCOUNTS_LEN] {
    fn from(accounts: ExecuteMultisigProposalApprovalAccounts<'_, 'info>) -> Self {
        [
            accounts.dao.clone(),
            accounts.rent_receiver.clone(),
            accounts.squads_multisig.clone(),
            accounts.squads_multisig_proposal.clone(),
            accounts.enqueued_approval.clone(),
            accounts.squads_multisig_program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; EXECUTE_MULTISIG_PROPOSAL_APPROVAL_IX_ACCOUNTS_LEN]>
for ExecuteMultisigProposalApprovalAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<
            'info,
        >; EXECUTE_MULTISIG_PROPOSAL_APPROVAL_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            dao: &arr[0],
            rent_receiver: &arr[1],
            squads_multisig: &arr[2],
            squads_multisig_proposal: &arr[3],
            enqueued_approval: &arr[4],
            squads_multisig_program: &arr[5],
        }
    }
}
pub const EXECUTE_MULTISIG_PROPOSAL_APPROVAL_IX_DISCM: [u8; 8usize] = [
    124, 144, 201, 164, 182, 226, 193, 224,
];
#[derive(Clone, Debug, PartialEq)]
pub struct ExecuteMultisigProposalApprovalIxData;
impl ExecuteMultisigProposalApprovalIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != EXECUTE_MULTISIG_PROPOSAL_APPROVAL_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&EXECUTE_MULTISIG_PROPOSAL_APPROVAL_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn execute_multisig_proposal_approval_ix_with_program_id(
    program_id: Pubkey,
    keys: ExecuteMultisigProposalApprovalKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; EXECUTE_MULTISIG_PROPOSAL_APPROVAL_IX_ACCOUNTS_LEN] = keys
        .into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: ExecuteMultisigProposalApprovalIxData.try_to_vec()?,
    })
}
pub fn execute_multisig_proposal_approval_ix(
    keys: ExecuteMultisigProposalApprovalKeys,
) -> std::io::Result<Instruction> {
    execute_multisig_proposal_approval_ix_with_program_id(FUTARCHY_PROGRAM_ID, keys)
}
pub fn execute_multisig_proposal_approval_invoke_with_program_id(
    program_id: Pubkey,
    accounts: ExecuteMultisigProposalApprovalAccounts<'_, '_>,
) -> ProgramResult {
    let keys: ExecuteMultisigProposalApprovalKeys = accounts.into();
    let ix = execute_multisig_proposal_approval_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn execute_multisig_proposal_approval_invoke(
    accounts: ExecuteMultisigProposalApprovalAccounts<'_, '_>,
) -> ProgramResult {
    execute_multisig_proposal_approval_invoke_with_program_id(
        FUTARCHY_PROGRAM_ID,
        accounts,
    )
}
pub fn execute_multisig_proposal_approval_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: ExecuteMultisigProposalApprovalAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: ExecuteMultisigProposalApprovalKeys = accounts.into();
    let ix = execute_multisig_proposal_approval_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn execute_multisig_proposal_approval_invoke_signed(
    accounts: ExecuteMultisigProposalApprovalAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    execute_multisig_proposal_approval_invoke_signed_with_program_id(
        FUTARCHY_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn execute_multisig_proposal_approval_verify_account_keys(
    accounts: ExecuteMultisigProposalApprovalAccounts<'_, '_>,
    keys: ExecuteMultisigProposalApprovalKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.dao.key, keys.dao),
        (*accounts.rent_receiver.key, keys.rent_receiver),
        (*accounts.squads_multisig.key, keys.squads_multisig),
        (*accounts.squads_multisig_proposal.key, keys.squads_multisig_proposal),
        (*accounts.enqueued_approval.key, keys.enqueued_approval),
        (*accounts.squads_multisig_program.key, keys.squads_multisig_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn execute_multisig_proposal_approval_verify_writable_privileges<'me, 'info>(
    accounts: ExecuteMultisigProposalApprovalAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.dao,
        accounts.rent_receiver,
        accounts.squads_multisig,
        accounts.squads_multisig_proposal,
        accounts.enqueued_approval,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn execute_multisig_proposal_approval_verify_signer_privileges<'me, 'info>(
    accounts: ExecuteMultisigProposalApprovalAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.rent_receiver] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn execute_multisig_proposal_approval_verify_account_privileges<'me, 'info>(
    accounts: ExecuteMultisigProposalApprovalAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    execute_multisig_proposal_approval_verify_writable_privileges(accounts)?;
    execute_multisig_proposal_approval_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const ADMIN_EXECUTE_MULTISIG_PROPOSAL_IX_ACCOUNTS_LEN: usize = 6;
#[derive(Copy, Clone, Debug)]
pub struct AdminExecuteMultisigProposalAccounts<'me, 'info> {
    pub dao: &'me AccountInfo<'info>,
    pub admin: &'me AccountInfo<'info>,
    pub squads_multisig: &'me AccountInfo<'info>,
    pub squads_multisig_proposal: &'me AccountInfo<'info>,
    pub squads_multisig_vault_transaction: &'me AccountInfo<'info>,
    pub squads_multisig_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct AdminExecuteMultisigProposalKeys {
    pub dao: Pubkey,
    pub admin: Pubkey,
    pub squads_multisig: Pubkey,
    pub squads_multisig_proposal: Pubkey,
    pub squads_multisig_vault_transaction: Pubkey,
    pub squads_multisig_program: Pubkey,
}
impl From<AdminExecuteMultisigProposalAccounts<'_, '_>>
for AdminExecuteMultisigProposalKeys {
    fn from(accounts: AdminExecuteMultisigProposalAccounts) -> Self {
        Self {
            dao: *accounts.dao.key,
            admin: *accounts.admin.key,
            squads_multisig: *accounts.squads_multisig.key,
            squads_multisig_proposal: *accounts.squads_multisig_proposal.key,
            squads_multisig_vault_transaction: *accounts
                .squads_multisig_vault_transaction
                .key,
            squads_multisig_program: *accounts.squads_multisig_program.key,
        }
    }
}
impl From<AdminExecuteMultisigProposalKeys>
for [AccountMeta; ADMIN_EXECUTE_MULTISIG_PROPOSAL_IX_ACCOUNTS_LEN] {
    fn from(keys: AdminExecuteMultisigProposalKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.dao,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.admin,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.squads_multisig,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.squads_multisig_proposal,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.squads_multisig_vault_transaction,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.squads_multisig_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; ADMIN_EXECUTE_MULTISIG_PROPOSAL_IX_ACCOUNTS_LEN]>
for AdminExecuteMultisigProposalKeys {
    fn from(pubkeys: [Pubkey; ADMIN_EXECUTE_MULTISIG_PROPOSAL_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            dao: pubkeys[0],
            admin: pubkeys[1],
            squads_multisig: pubkeys[2],
            squads_multisig_proposal: pubkeys[3],
            squads_multisig_vault_transaction: pubkeys[4],
            squads_multisig_program: pubkeys[5],
        }
    }
}
impl<'info> From<AdminExecuteMultisigProposalAccounts<'_, 'info>>
for [AccountInfo<'info>; ADMIN_EXECUTE_MULTISIG_PROPOSAL_IX_ACCOUNTS_LEN] {
    fn from(accounts: AdminExecuteMultisigProposalAccounts<'_, 'info>) -> Self {
        [
            accounts.dao.clone(),
            accounts.admin.clone(),
            accounts.squads_multisig.clone(),
            accounts.squads_multisig_proposal.clone(),
            accounts.squads_multisig_vault_transaction.clone(),
            accounts.squads_multisig_program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; ADMIN_EXECUTE_MULTISIG_PROPOSAL_IX_ACCOUNTS_LEN]>
for AdminExecuteMultisigProposalAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; ADMIN_EXECUTE_MULTISIG_PROPOSAL_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            dao: &arr[0],
            admin: &arr[1],
            squads_multisig: &arr[2],
            squads_multisig_proposal: &arr[3],
            squads_multisig_vault_transaction: &arr[4],
            squads_multisig_program: &arr[5],
        }
    }
}
pub const ADMIN_EXECUTE_MULTISIG_PROPOSAL_IX_DISCM: [u8; 8usize] = [
    226, 221, 40, 160, 169, 90, 48, 42,
];
#[derive(Clone, Debug, PartialEq)]
pub struct AdminExecuteMultisigProposalIxData;
impl AdminExecuteMultisigProposalIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != ADMIN_EXECUTE_MULTISIG_PROPOSAL_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&ADMIN_EXECUTE_MULTISIG_PROPOSAL_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn admin_execute_multisig_proposal_ix_with_program_id(
    program_id: Pubkey,
    keys: AdminExecuteMultisigProposalKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; ADMIN_EXECUTE_MULTISIG_PROPOSAL_IX_ACCOUNTS_LEN] = keys
        .into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: AdminExecuteMultisigProposalIxData.try_to_vec()?,
    })
}
pub fn admin_execute_multisig_proposal_ix(
    keys: AdminExecuteMultisigProposalKeys,
) -> std::io::Result<Instruction> {
    admin_execute_multisig_proposal_ix_with_program_id(FUTARCHY_PROGRAM_ID, keys)
}
pub fn admin_execute_multisig_proposal_invoke_with_program_id(
    program_id: Pubkey,
    accounts: AdminExecuteMultisigProposalAccounts<'_, '_>,
) -> ProgramResult {
    let keys: AdminExecuteMultisigProposalKeys = accounts.into();
    let ix = admin_execute_multisig_proposal_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn admin_execute_multisig_proposal_invoke(
    accounts: AdminExecuteMultisigProposalAccounts<'_, '_>,
) -> ProgramResult {
    admin_execute_multisig_proposal_invoke_with_program_id(FUTARCHY_PROGRAM_ID, accounts)
}
pub fn admin_execute_multisig_proposal_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: AdminExecuteMultisigProposalAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: AdminExecuteMultisigProposalKeys = accounts.into();
    let ix = admin_execute_multisig_proposal_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn admin_execute_multisig_proposal_invoke_signed(
    accounts: AdminExecuteMultisigProposalAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    admin_execute_multisig_proposal_invoke_signed_with_program_id(
        FUTARCHY_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn admin_execute_multisig_proposal_verify_account_keys(
    accounts: AdminExecuteMultisigProposalAccounts<'_, '_>,
    keys: AdminExecuteMultisigProposalKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.dao.key, keys.dao),
        (*accounts.admin.key, keys.admin),
        (*accounts.squads_multisig.key, keys.squads_multisig),
        (*accounts.squads_multisig_proposal.key, keys.squads_multisig_proposal),
        (
            *accounts.squads_multisig_vault_transaction.key,
            keys.squads_multisig_vault_transaction,
        ),
        (*accounts.squads_multisig_program.key, keys.squads_multisig_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn admin_execute_multisig_proposal_verify_writable_privileges<'me, 'info>(
    accounts: AdminExecuteMultisigProposalAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.dao,
        accounts.admin,
        accounts.squads_multisig,
        accounts.squads_multisig_proposal,
        accounts.squads_multisig_vault_transaction,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn admin_execute_multisig_proposal_verify_signer_privileges<'me, 'info>(
    accounts: AdminExecuteMultisigProposalAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn admin_execute_multisig_proposal_verify_account_privileges<'me, 'info>(
    accounts: AdminExecuteMultisigProposalAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    admin_execute_multisig_proposal_verify_writable_privileges(accounts)?;
    admin_execute_multisig_proposal_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const ADMIN_CANCEL_PROPOSAL_IX_ACCOUNTS_LEN: usize = 26;
#[derive(Copy, Clone, Debug)]
pub struct AdminCancelProposalAccounts<'me, 'info> {
    pub proposal: &'me AccountInfo<'info>,
    pub dao: &'me AccountInfo<'info>,
    pub question: &'me AccountInfo<'info>,
    pub squads_proposal: &'me AccountInfo<'info>,
    pub squads_multisig: &'me AccountInfo<'info>,
    pub squads_multisig_program: &'me AccountInfo<'info>,
    pub amm_pass_base_vault: &'me AccountInfo<'info>,
    pub amm_pass_quote_vault: &'me AccountInfo<'info>,
    pub amm_fail_base_vault: &'me AccountInfo<'info>,
    pub amm_fail_quote_vault: &'me AccountInfo<'info>,
    pub amm_base_vault: &'me AccountInfo<'info>,
    pub amm_quote_vault: &'me AccountInfo<'info>,
    pub vault_program: &'me AccountInfo<'info>,
    pub vault_event_authority: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub quote_vault: &'me AccountInfo<'info>,
    pub quote_vault_underlying_token_account: &'me AccountInfo<'info>,
    pub pass_quote_mint: &'me AccountInfo<'info>,
    pub fail_quote_mint: &'me AccountInfo<'info>,
    pub pass_base_mint: &'me AccountInfo<'info>,
    pub fail_base_mint: &'me AccountInfo<'info>,
    pub base_vault: &'me AccountInfo<'info>,
    pub base_vault_underlying_token_account: &'me AccountInfo<'info>,
    pub admin: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct AdminCancelProposalKeys {
    pub proposal: Pubkey,
    pub dao: Pubkey,
    pub question: Pubkey,
    pub squads_proposal: Pubkey,
    pub squads_multisig: Pubkey,
    pub squads_multisig_program: Pubkey,
    pub amm_pass_base_vault: Pubkey,
    pub amm_pass_quote_vault: Pubkey,
    pub amm_fail_base_vault: Pubkey,
    pub amm_fail_quote_vault: Pubkey,
    pub amm_base_vault: Pubkey,
    pub amm_quote_vault: Pubkey,
    pub vault_program: Pubkey,
    pub vault_event_authority: Pubkey,
    pub token_program: Pubkey,
    pub quote_vault: Pubkey,
    pub quote_vault_underlying_token_account: Pubkey,
    pub pass_quote_mint: Pubkey,
    pub fail_quote_mint: Pubkey,
    pub pass_base_mint: Pubkey,
    pub fail_base_mint: Pubkey,
    pub base_vault: Pubkey,
    pub base_vault_underlying_token_account: Pubkey,
    pub admin: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<AdminCancelProposalAccounts<'_, '_>> for AdminCancelProposalKeys {
    fn from(accounts: AdminCancelProposalAccounts) -> Self {
        Self {
            proposal: *accounts.proposal.key,
            dao: *accounts.dao.key,
            question: *accounts.question.key,
            squads_proposal: *accounts.squads_proposal.key,
            squads_multisig: *accounts.squads_multisig.key,
            squads_multisig_program: *accounts.squads_multisig_program.key,
            amm_pass_base_vault: *accounts.amm_pass_base_vault.key,
            amm_pass_quote_vault: *accounts.amm_pass_quote_vault.key,
            amm_fail_base_vault: *accounts.amm_fail_base_vault.key,
            amm_fail_quote_vault: *accounts.amm_fail_quote_vault.key,
            amm_base_vault: *accounts.amm_base_vault.key,
            amm_quote_vault: *accounts.amm_quote_vault.key,
            vault_program: *accounts.vault_program.key,
            vault_event_authority: *accounts.vault_event_authority.key,
            token_program: *accounts.token_program.key,
            quote_vault: *accounts.quote_vault.key,
            quote_vault_underlying_token_account: *accounts
                .quote_vault_underlying_token_account
                .key,
            pass_quote_mint: *accounts.pass_quote_mint.key,
            fail_quote_mint: *accounts.fail_quote_mint.key,
            pass_base_mint: *accounts.pass_base_mint.key,
            fail_base_mint: *accounts.fail_base_mint.key,
            base_vault: *accounts.base_vault.key,
            base_vault_underlying_token_account: *accounts
                .base_vault_underlying_token_account
                .key,
            admin: *accounts.admin.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<AdminCancelProposalKeys>
for [AccountMeta; ADMIN_CANCEL_PROPOSAL_IX_ACCOUNTS_LEN] {
    fn from(keys: AdminCancelProposalKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.proposal,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.dao,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.question,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.squads_proposal,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.squads_multisig,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.squads_multisig_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.amm_pass_base_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.amm_pass_quote_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.amm_fail_base_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.amm_fail_quote_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.amm_base_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.amm_quote_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vault_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault_event_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.quote_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.quote_vault_underlying_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pass_quote_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.fail_quote_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pass_base_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.fail_base_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.base_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.base_vault_underlying_token_account,
                is_signer: false,
                is_writable: true,
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
impl From<[Pubkey; ADMIN_CANCEL_PROPOSAL_IX_ACCOUNTS_LEN]> for AdminCancelProposalKeys {
    fn from(pubkeys: [Pubkey; ADMIN_CANCEL_PROPOSAL_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            proposal: pubkeys[0],
            dao: pubkeys[1],
            question: pubkeys[2],
            squads_proposal: pubkeys[3],
            squads_multisig: pubkeys[4],
            squads_multisig_program: pubkeys[5],
            amm_pass_base_vault: pubkeys[6],
            amm_pass_quote_vault: pubkeys[7],
            amm_fail_base_vault: pubkeys[8],
            amm_fail_quote_vault: pubkeys[9],
            amm_base_vault: pubkeys[10],
            amm_quote_vault: pubkeys[11],
            vault_program: pubkeys[12],
            vault_event_authority: pubkeys[13],
            token_program: pubkeys[14],
            quote_vault: pubkeys[15],
            quote_vault_underlying_token_account: pubkeys[16],
            pass_quote_mint: pubkeys[17],
            fail_quote_mint: pubkeys[18],
            pass_base_mint: pubkeys[19],
            fail_base_mint: pubkeys[20],
            base_vault: pubkeys[21],
            base_vault_underlying_token_account: pubkeys[22],
            admin: pubkeys[23],
            event_authority: pubkeys[24],
            program: pubkeys[25],
        }
    }
}
impl<'info> From<AdminCancelProposalAccounts<'_, 'info>>
for [AccountInfo<'info>; ADMIN_CANCEL_PROPOSAL_IX_ACCOUNTS_LEN] {
    fn from(accounts: AdminCancelProposalAccounts<'_, 'info>) -> Self {
        [
            accounts.proposal.clone(),
            accounts.dao.clone(),
            accounts.question.clone(),
            accounts.squads_proposal.clone(),
            accounts.squads_multisig.clone(),
            accounts.squads_multisig_program.clone(),
            accounts.amm_pass_base_vault.clone(),
            accounts.amm_pass_quote_vault.clone(),
            accounts.amm_fail_base_vault.clone(),
            accounts.amm_fail_quote_vault.clone(),
            accounts.amm_base_vault.clone(),
            accounts.amm_quote_vault.clone(),
            accounts.vault_program.clone(),
            accounts.vault_event_authority.clone(),
            accounts.token_program.clone(),
            accounts.quote_vault.clone(),
            accounts.quote_vault_underlying_token_account.clone(),
            accounts.pass_quote_mint.clone(),
            accounts.fail_quote_mint.clone(),
            accounts.pass_base_mint.clone(),
            accounts.fail_base_mint.clone(),
            accounts.base_vault.clone(),
            accounts.base_vault_underlying_token_account.clone(),
            accounts.admin.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; ADMIN_CANCEL_PROPOSAL_IX_ACCOUNTS_LEN]>
for AdminCancelProposalAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; ADMIN_CANCEL_PROPOSAL_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            proposal: &arr[0],
            dao: &arr[1],
            question: &arr[2],
            squads_proposal: &arr[3],
            squads_multisig: &arr[4],
            squads_multisig_program: &arr[5],
            amm_pass_base_vault: &arr[6],
            amm_pass_quote_vault: &arr[7],
            amm_fail_base_vault: &arr[8],
            amm_fail_quote_vault: &arr[9],
            amm_base_vault: &arr[10],
            amm_quote_vault: &arr[11],
            vault_program: &arr[12],
            vault_event_authority: &arr[13],
            token_program: &arr[14],
            quote_vault: &arr[15],
            quote_vault_underlying_token_account: &arr[16],
            pass_quote_mint: &arr[17],
            fail_quote_mint: &arr[18],
            pass_base_mint: &arr[19],
            fail_base_mint: &arr[20],
            base_vault: &arr[21],
            base_vault_underlying_token_account: &arr[22],
            admin: &arr[23],
            event_authority: &arr[24],
            program: &arr[25],
        }
    }
}
pub const ADMIN_CANCEL_PROPOSAL_IX_DISCM: [u8; 8usize] = [
    95, 233, 121, 193, 90, 80, 147, 255,
];
#[derive(Clone, Debug, PartialEq)]
pub struct AdminCancelProposalIxData;
impl AdminCancelProposalIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != ADMIN_CANCEL_PROPOSAL_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&ADMIN_CANCEL_PROPOSAL_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn admin_cancel_proposal_ix_with_program_id(
    program_id: Pubkey,
    keys: AdminCancelProposalKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; ADMIN_CANCEL_PROPOSAL_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: AdminCancelProposalIxData.try_to_vec()?,
    })
}
pub fn admin_cancel_proposal_ix(
    keys: AdminCancelProposalKeys,
) -> std::io::Result<Instruction> {
    admin_cancel_proposal_ix_with_program_id(FUTARCHY_PROGRAM_ID, keys)
}
pub fn admin_cancel_proposal_invoke_with_program_id(
    program_id: Pubkey,
    accounts: AdminCancelProposalAccounts<'_, '_>,
) -> ProgramResult {
    let keys: AdminCancelProposalKeys = accounts.into();
    let ix = admin_cancel_proposal_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn admin_cancel_proposal_invoke(
    accounts: AdminCancelProposalAccounts<'_, '_>,
) -> ProgramResult {
    admin_cancel_proposal_invoke_with_program_id(FUTARCHY_PROGRAM_ID, accounts)
}
pub fn admin_cancel_proposal_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: AdminCancelProposalAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: AdminCancelProposalKeys = accounts.into();
    let ix = admin_cancel_proposal_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn admin_cancel_proposal_invoke_signed(
    accounts: AdminCancelProposalAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    admin_cancel_proposal_invoke_signed_with_program_id(
        FUTARCHY_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn admin_cancel_proposal_verify_account_keys(
    accounts: AdminCancelProposalAccounts<'_, '_>,
    keys: AdminCancelProposalKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.proposal.key, keys.proposal),
        (*accounts.dao.key, keys.dao),
        (*accounts.question.key, keys.question),
        (*accounts.squads_proposal.key, keys.squads_proposal),
        (*accounts.squads_multisig.key, keys.squads_multisig),
        (*accounts.squads_multisig_program.key, keys.squads_multisig_program),
        (*accounts.amm_pass_base_vault.key, keys.amm_pass_base_vault),
        (*accounts.amm_pass_quote_vault.key, keys.amm_pass_quote_vault),
        (*accounts.amm_fail_base_vault.key, keys.amm_fail_base_vault),
        (*accounts.amm_fail_quote_vault.key, keys.amm_fail_quote_vault),
        (*accounts.amm_base_vault.key, keys.amm_base_vault),
        (*accounts.amm_quote_vault.key, keys.amm_quote_vault),
        (*accounts.vault_program.key, keys.vault_program),
        (*accounts.vault_event_authority.key, keys.vault_event_authority),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.quote_vault.key, keys.quote_vault),
        (
            *accounts.quote_vault_underlying_token_account.key,
            keys.quote_vault_underlying_token_account,
        ),
        (*accounts.pass_quote_mint.key, keys.pass_quote_mint),
        (*accounts.fail_quote_mint.key, keys.fail_quote_mint),
        (*accounts.pass_base_mint.key, keys.pass_base_mint),
        (*accounts.fail_base_mint.key, keys.fail_base_mint),
        (*accounts.base_vault.key, keys.base_vault),
        (
            *accounts.base_vault_underlying_token_account.key,
            keys.base_vault_underlying_token_account,
        ),
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
pub fn admin_cancel_proposal_verify_writable_privileges<'me, 'info>(
    accounts: AdminCancelProposalAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.proposal,
        accounts.dao,
        accounts.question,
        accounts.squads_proposal,
        accounts.amm_pass_base_vault,
        accounts.amm_pass_quote_vault,
        accounts.amm_fail_base_vault,
        accounts.amm_fail_quote_vault,
        accounts.amm_base_vault,
        accounts.amm_quote_vault,
        accounts.quote_vault,
        accounts.quote_vault_underlying_token_account,
        accounts.pass_quote_mint,
        accounts.fail_quote_mint,
        accounts.pass_base_mint,
        accounts.fail_base_mint,
        accounts.base_vault,
        accounts.base_vault_underlying_token_account,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn admin_cancel_proposal_verify_signer_privileges<'me, 'info>(
    accounts: AdminCancelProposalAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn admin_cancel_proposal_verify_account_privileges<'me, 'info>(
    accounts: AdminCancelProposalAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    admin_cancel_proposal_verify_writable_privileges(accounts)?;
    admin_cancel_proposal_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const ADMIN_REMOVE_PROPOSAL_IX_ACCOUNTS_LEN: usize = 5;
#[derive(Copy, Clone, Debug)]
pub struct AdminRemoveProposalAccounts<'me, 'info> {
    pub proposal: &'me AccountInfo<'info>,
    pub dao: &'me AccountInfo<'info>,
    pub admin: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct AdminRemoveProposalKeys {
    pub proposal: Pubkey,
    pub dao: Pubkey,
    pub admin: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<AdminRemoveProposalAccounts<'_, '_>> for AdminRemoveProposalKeys {
    fn from(accounts: AdminRemoveProposalAccounts) -> Self {
        Self {
            proposal: *accounts.proposal.key,
            dao: *accounts.dao.key,
            admin: *accounts.admin.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<AdminRemoveProposalKeys>
for [AccountMeta; ADMIN_REMOVE_PROPOSAL_IX_ACCOUNTS_LEN] {
    fn from(keys: AdminRemoveProposalKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.proposal,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.dao,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.admin,
                is_signer: true,
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
impl From<[Pubkey; ADMIN_REMOVE_PROPOSAL_IX_ACCOUNTS_LEN]> for AdminRemoveProposalKeys {
    fn from(pubkeys: [Pubkey; ADMIN_REMOVE_PROPOSAL_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            proposal: pubkeys[0],
            dao: pubkeys[1],
            admin: pubkeys[2],
            event_authority: pubkeys[3],
            program: pubkeys[4],
        }
    }
}
impl<'info> From<AdminRemoveProposalAccounts<'_, 'info>>
for [AccountInfo<'info>; ADMIN_REMOVE_PROPOSAL_IX_ACCOUNTS_LEN] {
    fn from(accounts: AdminRemoveProposalAccounts<'_, 'info>) -> Self {
        [
            accounts.proposal.clone(),
            accounts.dao.clone(),
            accounts.admin.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; ADMIN_REMOVE_PROPOSAL_IX_ACCOUNTS_LEN]>
for AdminRemoveProposalAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; ADMIN_REMOVE_PROPOSAL_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            proposal: &arr[0],
            dao: &arr[1],
            admin: &arr[2],
            event_authority: &arr[3],
            program: &arr[4],
        }
    }
}
pub const ADMIN_REMOVE_PROPOSAL_IX_DISCM: [u8; 8usize] = [
    242, 199, 27, 28, 7, 108, 122, 73,
];
#[derive(Clone, Debug, PartialEq)]
pub struct AdminRemoveProposalIxData;
impl AdminRemoveProposalIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != ADMIN_REMOVE_PROPOSAL_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&ADMIN_REMOVE_PROPOSAL_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn admin_remove_proposal_ix_with_program_id(
    program_id: Pubkey,
    keys: AdminRemoveProposalKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; ADMIN_REMOVE_PROPOSAL_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: AdminRemoveProposalIxData.try_to_vec()?,
    })
}
pub fn admin_remove_proposal_ix(
    keys: AdminRemoveProposalKeys,
) -> std::io::Result<Instruction> {
    admin_remove_proposal_ix_with_program_id(FUTARCHY_PROGRAM_ID, keys)
}
pub fn admin_remove_proposal_invoke_with_program_id(
    program_id: Pubkey,
    accounts: AdminRemoveProposalAccounts<'_, '_>,
) -> ProgramResult {
    let keys: AdminRemoveProposalKeys = accounts.into();
    let ix = admin_remove_proposal_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn admin_remove_proposal_invoke(
    accounts: AdminRemoveProposalAccounts<'_, '_>,
) -> ProgramResult {
    admin_remove_proposal_invoke_with_program_id(FUTARCHY_PROGRAM_ID, accounts)
}
pub fn admin_remove_proposal_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: AdminRemoveProposalAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: AdminRemoveProposalKeys = accounts.into();
    let ix = admin_remove_proposal_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn admin_remove_proposal_invoke_signed(
    accounts: AdminRemoveProposalAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    admin_remove_proposal_invoke_signed_with_program_id(
        FUTARCHY_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn admin_remove_proposal_verify_account_keys(
    accounts: AdminRemoveProposalAccounts<'_, '_>,
    keys: AdminRemoveProposalKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.proposal.key, keys.proposal),
        (*accounts.dao.key, keys.dao),
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
pub fn admin_remove_proposal_verify_writable_privileges<'me, 'info>(
    accounts: AdminRemoveProposalAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.proposal, accounts.dao, accounts.admin] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn admin_remove_proposal_verify_signer_privileges<'me, 'info>(
    accounts: AdminRemoveProposalAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn admin_remove_proposal_verify_account_privileges<'me, 'info>(
    accounts: AdminRemoveProposalAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    admin_remove_proposal_verify_writable_privileges(accounts)?;
    admin_remove_proposal_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const ADMIN_APPROVE_MULTISIG_PROPOSAL_IX_ACCOUNTS_LEN: usize = 5;
#[derive(Copy, Clone, Debug)]
pub struct AdminApproveMultisigProposalAccounts<'me, 'info> {
    pub dao: &'me AccountInfo<'info>,
    pub admin: &'me AccountInfo<'info>,
    pub squads_multisig: &'me AccountInfo<'info>,
    pub squads_multisig_proposal: &'me AccountInfo<'info>,
    pub squads_multisig_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct AdminApproveMultisigProposalKeys {
    pub dao: Pubkey,
    pub admin: Pubkey,
    pub squads_multisig: Pubkey,
    pub squads_multisig_proposal: Pubkey,
    pub squads_multisig_program: Pubkey,
}
impl From<AdminApproveMultisigProposalAccounts<'_, '_>>
for AdminApproveMultisigProposalKeys {
    fn from(accounts: AdminApproveMultisigProposalAccounts) -> Self {
        Self {
            dao: *accounts.dao.key,
            admin: *accounts.admin.key,
            squads_multisig: *accounts.squads_multisig.key,
            squads_multisig_proposal: *accounts.squads_multisig_proposal.key,
            squads_multisig_program: *accounts.squads_multisig_program.key,
        }
    }
}
impl From<AdminApproveMultisigProposalKeys>
for [AccountMeta; ADMIN_APPROVE_MULTISIG_PROPOSAL_IX_ACCOUNTS_LEN] {
    fn from(keys: AdminApproveMultisigProposalKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.dao,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.admin,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.squads_multisig,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.squads_multisig_proposal,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.squads_multisig_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; ADMIN_APPROVE_MULTISIG_PROPOSAL_IX_ACCOUNTS_LEN]>
for AdminApproveMultisigProposalKeys {
    fn from(pubkeys: [Pubkey; ADMIN_APPROVE_MULTISIG_PROPOSAL_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            dao: pubkeys[0],
            admin: pubkeys[1],
            squads_multisig: pubkeys[2],
            squads_multisig_proposal: pubkeys[3],
            squads_multisig_program: pubkeys[4],
        }
    }
}
impl<'info> From<AdminApproveMultisigProposalAccounts<'_, 'info>>
for [AccountInfo<'info>; ADMIN_APPROVE_MULTISIG_PROPOSAL_IX_ACCOUNTS_LEN] {
    fn from(accounts: AdminApproveMultisigProposalAccounts<'_, 'info>) -> Self {
        [
            accounts.dao.clone(),
            accounts.admin.clone(),
            accounts.squads_multisig.clone(),
            accounts.squads_multisig_proposal.clone(),
            accounts.squads_multisig_program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; ADMIN_APPROVE_MULTISIG_PROPOSAL_IX_ACCOUNTS_LEN]>
for AdminApproveMultisigProposalAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; ADMIN_APPROVE_MULTISIG_PROPOSAL_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            dao: &arr[0],
            admin: &arr[1],
            squads_multisig: &arr[2],
            squads_multisig_proposal: &arr[3],
            squads_multisig_program: &arr[4],
        }
    }
}
pub const ADMIN_APPROVE_MULTISIG_PROPOSAL_IX_DISCM: [u8; 8usize] = [
    157, 1, 93, 74, 82, 60, 30, 203,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AdminApproveMultisigProposalIxArgs {
    pub args: AdminApproveMultisigProposalArgs,
}
#[derive(Clone, Debug, PartialEq)]
pub struct AdminApproveMultisigProposalIxData(pub AdminApproveMultisigProposalIxArgs);
impl From<AdminApproveMultisigProposalIxArgs> for AdminApproveMultisigProposalIxData {
    fn from(args: AdminApproveMultisigProposalIxArgs) -> Self {
        Self(args)
    }
}
impl AdminApproveMultisigProposalIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != ADMIN_APPROVE_MULTISIG_PROPOSAL_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let args = if reader.is_empty() {
            Default::default()
        } else {
            <AdminApproveMultisigProposalArgs>::deserialize(&mut reader)?
        };
        Ok(
            Self(AdminApproveMultisigProposalIxArgs {
                args,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&ADMIN_APPROVE_MULTISIG_PROPOSAL_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.args, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn admin_approve_multisig_proposal_ix_with_program_id(
    program_id: Pubkey,
    keys: AdminApproveMultisigProposalKeys,
    args: AdminApproveMultisigProposalIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; ADMIN_APPROVE_MULTISIG_PROPOSAL_IX_ACCOUNTS_LEN] = keys
        .into();
    let data: AdminApproveMultisigProposalIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn admin_approve_multisig_proposal_ix(
    keys: AdminApproveMultisigProposalKeys,
    args: AdminApproveMultisigProposalIxArgs,
) -> std::io::Result<Instruction> {
    admin_approve_multisig_proposal_ix_with_program_id(FUTARCHY_PROGRAM_ID, keys, args)
}
pub fn admin_approve_multisig_proposal_invoke_with_program_id(
    program_id: Pubkey,
    accounts: AdminApproveMultisigProposalAccounts<'_, '_>,
    args: AdminApproveMultisigProposalIxArgs,
) -> ProgramResult {
    let keys: AdminApproveMultisigProposalKeys = accounts.into();
    let ix = admin_approve_multisig_proposal_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn admin_approve_multisig_proposal_invoke(
    accounts: AdminApproveMultisigProposalAccounts<'_, '_>,
    args: AdminApproveMultisigProposalIxArgs,
) -> ProgramResult {
    admin_approve_multisig_proposal_invoke_with_program_id(
        FUTARCHY_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn admin_approve_multisig_proposal_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: AdminApproveMultisigProposalAccounts<'_, '_>,
    args: AdminApproveMultisigProposalIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: AdminApproveMultisigProposalKeys = accounts.into();
    let ix = admin_approve_multisig_proposal_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn admin_approve_multisig_proposal_invoke_signed(
    accounts: AdminApproveMultisigProposalAccounts<'_, '_>,
    args: AdminApproveMultisigProposalIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    admin_approve_multisig_proposal_invoke_signed_with_program_id(
        FUTARCHY_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn admin_approve_multisig_proposal_verify_account_keys(
    accounts: AdminApproveMultisigProposalAccounts<'_, '_>,
    keys: AdminApproveMultisigProposalKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.dao.key, keys.dao),
        (*accounts.admin.key, keys.admin),
        (*accounts.squads_multisig.key, keys.squads_multisig),
        (*accounts.squads_multisig_proposal.key, keys.squads_multisig_proposal),
        (*accounts.squads_multisig_program.key, keys.squads_multisig_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn admin_approve_multisig_proposal_verify_writable_privileges<'me, 'info>(
    accounts: AdminApproveMultisigProposalAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.dao,
        accounts.admin,
        accounts.squads_multisig,
        accounts.squads_multisig_proposal,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn admin_approve_multisig_proposal_verify_signer_privileges<'me, 'info>(
    accounts: AdminApproveMultisigProposalAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn admin_approve_multisig_proposal_verify_account_privileges<'me, 'info>(
    accounts: AdminApproveMultisigProposalAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    admin_approve_multisig_proposal_verify_writable_privileges(accounts)?;
    admin_approve_multisig_proposal_verify_signer_privileges(accounts)?;
    Ok(())
}
