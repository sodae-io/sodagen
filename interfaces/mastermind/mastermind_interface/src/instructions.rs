use solana_pubkey::Pubkey;
use solana_cpi::{invoke, invoke_signed};
use solana_instruction::{AccountMeta, Instruction};
use solana_account_info::AccountInfo;
use solana_program_error::ProgramError;
use std::io::Read;
#[allow(unused_imports)]
use crate::*;
#[derive(Clone, Debug, PartialEq)]
pub enum MastermindProgramIx {
    BecomeAuthority,
    BuyExactIn(BuyExactInIxArgs),
    BuyExactOut(BuyExactOutIxArgs),
    BuyMaxOut(BuyMaxOutIxArgs),
    CollectCpFees(CollectCpFeesIxArgs),
    Create(CreateIxArgs),
    Initialize(InitializeIxArgs),
    MigrateToRadium,
    SellExactIn(SellExactInIxArgs),
    SellExactOut(SellExactOutIxArgs),
    SetCurveParams(SetCurveParamsIxArgs),
    SetFeeBps(SetFeeBpsIxArgs),
    SetFeeReceiver(SetFeeReceiverIxArgs),
    SetMigrationLotSize(SetMigrationLotSizeIxArgs),
    SetMigrationRefund(SetMigrationRefundIxArgs),
    SetMigrator(SetMigratorIxArgs),
    SetPendingAuthority(SetPendingAuthorityIxArgs),
    SetWrapperMint(SetWrapperMintIxArgs),
    SyncWrapper,
    WrapSolForCurve,
}
impl MastermindProgramIx {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        if buf.starts_with(&BECOME_AUTHORITY_IX_DISCM) {
            return Ok(Self::BecomeAuthority);
        }
        if buf.starts_with(&BUY_EXACT_IN_IX_DISCM) {
            let mut reader = &buf[BUY_EXACT_IN_IX_DISCM.len()..];
            let bumps = if reader.is_empty() {
                Default::default()
            } else {
                <InstructionBumps>::deserialize(&mut reader)?
            };
            let sol_amount_input: u64 = crate::borsh_de_or_default(&mut reader)?;
            let min_tokens_output: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::BuyExactIn(BuyExactInIxArgs {
                    bumps,
                    sol_amount_input,
                    min_tokens_output,
                }),
            );
        }
        if buf.starts_with(&BUY_EXACT_OUT_IX_DISCM) {
            let mut reader = &buf[BUY_EXACT_OUT_IX_DISCM.len()..];
            let bumps = if reader.is_empty() {
                Default::default()
            } else {
                <InstructionBumps>::deserialize(&mut reader)?
            };
            let max_sol_amount_input: u64 = crate::borsh_de_or_default(&mut reader)?;
            let tokens_output: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::BuyExactOut(BuyExactOutIxArgs {
                    bumps,
                    max_sol_amount_input,
                    tokens_output,
                }),
            );
        }
        if buf.starts_with(&BUY_MAX_OUT_IX_DISCM) {
            let mut reader = &buf[BUY_MAX_OUT_IX_DISCM.len()..];
            let bumps = if reader.is_empty() {
                Default::default()
            } else {
                <InstructionBumps>::deserialize(&mut reader)?
            };
            let max_sol_amount_input: u64 = crate::borsh_de_or_default(&mut reader)?;
            let min_tokens_output: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::BuyMaxOut(BuyMaxOutIxArgs {
                    bumps,
                    max_sol_amount_input,
                    min_tokens_output,
                }),
            );
        }
        if buf.starts_with(&COLLECT_CP_FEES_IX_DISCM) {
            let mut reader = &buf[COLLECT_CP_FEES_IX_DISCM.len()..];
            let fee_nft_owner_bump: u8 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::CollectCpFees(CollectCpFeesIxArgs {
                    fee_nft_owner_bump,
                }),
            );
        }
        if buf.starts_with(&CREATE_IX_DISCM) {
            let mut reader = &buf[CREATE_IX_DISCM.len()..];
            let name: String = crate::borsh_de_or_default(&mut reader)?;
            let symbol: String = crate::borsh_de_or_default(&mut reader)?;
            let uri: String = crate::borsh_de_or_default(&mut reader)?;
            let migration_kind: MigrationKind = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::Create(CreateIxArgs {
                    name,
                    symbol,
                    uri,
                    migration_kind,
                }),
            );
        }
        if buf.starts_with(&INITIALIZE_IX_DISCM) {
            let mut reader = &buf[INITIALIZE_IX_DISCM.len()..];
            let migrator: Pubkey = crate::borsh_de_or_default(&mut reader)?;
            let wrapper_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
            let fee_receiver: Pubkey = crate::borsh_de_or_default(&mut reader)?;
            let fee_bps: u64 = crate::borsh_de_or_default(&mut reader)?;
            let initial_virtual_token_reserve: u64 = crate::borsh_de_or_default(
                &mut reader,
            )?;
            let initial_virtual_sol_reserve: u64 = crate::borsh_de_or_default(
                &mut reader,
            )?;
            let coin_lot_size: u64 = crate::borsh_de_or_default(&mut reader)?;
            let pc_lot_size: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::Initialize(InitializeIxArgs {
                    migrator,
                    wrapper_mint,
                    fee_receiver,
                    fee_bps,
                    initial_virtual_token_reserve,
                    initial_virtual_sol_reserve,
                    coin_lot_size,
                    pc_lot_size,
                }),
            );
        }
        if buf.starts_with(&MIGRATE_TO_RADIUM_IX_DISCM) {
            return Ok(Self::MigrateToRadium);
        }
        if buf.starts_with(&SELL_EXACT_IN_IX_DISCM) {
            let mut reader = &buf[SELL_EXACT_IN_IX_DISCM.len()..];
            let bumps = if reader.is_empty() {
                Default::default()
            } else {
                <InstructionBumps>::deserialize(&mut reader)?
            };
            let tokens_input: u64 = crate::borsh_de_or_default(&mut reader)?;
            let min_sol_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::SellExactIn(SellExactInIxArgs {
                    bumps,
                    tokens_input,
                    min_sol_amount,
                }),
            );
        }
        if buf.starts_with(&SELL_EXACT_OUT_IX_DISCM) {
            let mut reader = &buf[SELL_EXACT_OUT_IX_DISCM.len()..];
            let bumps = if reader.is_empty() {
                Default::default()
            } else {
                <InstructionBumps>::deserialize(&mut reader)?
            };
            let max_tokens_input: u64 = crate::borsh_de_or_default(&mut reader)?;
            let sol_amount_output: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::SellExactOut(SellExactOutIxArgs {
                    bumps,
                    max_tokens_input,
                    sol_amount_output,
                }),
            );
        }
        if buf.starts_with(&SET_CURVE_PARAMS_IX_DISCM) {
            let mut reader = &buf[SET_CURVE_PARAMS_IX_DISCM.len()..];
            let initial_virtual_token_reserve: u64 = crate::borsh_de_or_default(
                &mut reader,
            )?;
            let initial_virtual_sol_reserve: u64 = crate::borsh_de_or_default(
                &mut reader,
            )?;
            return Ok(
                Self::SetCurveParams(SetCurveParamsIxArgs {
                    initial_virtual_token_reserve,
                    initial_virtual_sol_reserve,
                }),
            );
        }
        if buf.starts_with(&SET_FEE_BPS_IX_DISCM) {
            let mut reader = &buf[SET_FEE_BPS_IX_DISCM.len()..];
            let fee_bps: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(Self::SetFeeBps(SetFeeBpsIxArgs { fee_bps }));
        }
        if buf.starts_with(&SET_FEE_RECEIVER_IX_DISCM) {
            let mut reader = &buf[SET_FEE_RECEIVER_IX_DISCM.len()..];
            let fee_receiver: Pubkey = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::SetFeeReceiver(SetFeeReceiverIxArgs {
                    fee_receiver,
                }),
            );
        }
        if buf.starts_with(&SET_MIGRATION_LOT_SIZE_IX_DISCM) {
            let mut reader = &buf[SET_MIGRATION_LOT_SIZE_IX_DISCM.len()..];
            let coin_lot_size: u64 = crate::borsh_de_or_default(&mut reader)?;
            let pc_lot_size: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::SetMigrationLotSize(SetMigrationLotSizeIxArgs {
                    coin_lot_size,
                    pc_lot_size,
                }),
            );
        }
        if buf.starts_with(&SET_MIGRATION_REFUND_IX_DISCM) {
            let mut reader = &buf[SET_MIGRATION_REFUND_IX_DISCM.len()..];
            let migration_refund: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::SetMigrationRefund(SetMigrationRefundIxArgs {
                    migration_refund,
                }),
            );
        }
        if buf.starts_with(&SET_MIGRATOR_IX_DISCM) {
            let mut reader = &buf[SET_MIGRATOR_IX_DISCM.len()..];
            let migrator: Pubkey = crate::borsh_de_or_default(&mut reader)?;
            return Ok(Self::SetMigrator(SetMigratorIxArgs { migrator }));
        }
        if buf.starts_with(&SET_PENDING_AUTHORITY_IX_DISCM) {
            let mut reader = &buf[SET_PENDING_AUTHORITY_IX_DISCM.len()..];
            let pending_authority: Pubkey = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::SetPendingAuthority(SetPendingAuthorityIxArgs {
                    pending_authority,
                }),
            );
        }
        if buf.starts_with(&SET_WRAPPER_MINT_IX_DISCM) {
            let mut reader = &buf[SET_WRAPPER_MINT_IX_DISCM.len()..];
            let wrapper_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::SetWrapperMint(SetWrapperMintIxArgs {
                    wrapper_mint,
                }),
            );
        }
        if buf.starts_with(&SYNC_WRAPPER_IX_DISCM) {
            return Ok(Self::SyncWrapper);
        }
        if buf.starts_with(&WRAP_SOL_FOR_CURVE_IX_DISCM) {
            return Ok(Self::WrapSolForCurve);
        }
        Err(std::io::Error::from(std::io::ErrorKind::InvalidData))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        match self {
            Self::BecomeAuthority => writer.write_all(&BECOME_AUTHORITY_IX_DISCM),
            Self::BuyExactIn(args) => {
                writer.write_all(&BUY_EXACT_IN_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.bumps, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.sol_amount_input, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.min_tokens_output, &mut writer)?;
                Ok(())
            }
            Self::BuyExactOut(args) => {
                writer.write_all(&BUY_EXACT_OUT_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.bumps, &mut writer)?;
                borsh::BorshSerialize::serialize(
                    &args.max_sol_amount_input,
                    &mut writer,
                )?;
                borsh::BorshSerialize::serialize(&args.tokens_output, &mut writer)?;
                Ok(())
            }
            Self::BuyMaxOut(args) => {
                writer.write_all(&BUY_MAX_OUT_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.bumps, &mut writer)?;
                borsh::BorshSerialize::serialize(
                    &args.max_sol_amount_input,
                    &mut writer,
                )?;
                borsh::BorshSerialize::serialize(&args.min_tokens_output, &mut writer)?;
                Ok(())
            }
            Self::CollectCpFees(args) => {
                writer.write_all(&COLLECT_CP_FEES_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.fee_nft_owner_bump, &mut writer)?;
                Ok(())
            }
            Self::Create(args) => {
                writer.write_all(&CREATE_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.name, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.symbol, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.uri, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.migration_kind, &mut writer)?;
                Ok(())
            }
            Self::Initialize(args) => {
                writer.write_all(&INITIALIZE_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.migrator, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.wrapper_mint, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.fee_receiver, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.fee_bps, &mut writer)?;
                borsh::BorshSerialize::serialize(
                    &args.initial_virtual_token_reserve,
                    &mut writer,
                )?;
                borsh::BorshSerialize::serialize(
                    &args.initial_virtual_sol_reserve,
                    &mut writer,
                )?;
                borsh::BorshSerialize::serialize(&args.coin_lot_size, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.pc_lot_size, &mut writer)?;
                Ok(())
            }
            Self::MigrateToRadium => writer.write_all(&MIGRATE_TO_RADIUM_IX_DISCM),
            Self::SellExactIn(args) => {
                writer.write_all(&SELL_EXACT_IN_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.bumps, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.tokens_input, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.min_sol_amount, &mut writer)?;
                Ok(())
            }
            Self::SellExactOut(args) => {
                writer.write_all(&SELL_EXACT_OUT_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.bumps, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.max_tokens_input, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.sol_amount_output, &mut writer)?;
                Ok(())
            }
            Self::SetCurveParams(args) => {
                writer.write_all(&SET_CURVE_PARAMS_IX_DISCM)?;
                borsh::BorshSerialize::serialize(
                    &args.initial_virtual_token_reserve,
                    &mut writer,
                )?;
                borsh::BorshSerialize::serialize(
                    &args.initial_virtual_sol_reserve,
                    &mut writer,
                )?;
                Ok(())
            }
            Self::SetFeeBps(args) => {
                writer.write_all(&SET_FEE_BPS_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.fee_bps, &mut writer)?;
                Ok(())
            }
            Self::SetFeeReceiver(args) => {
                writer.write_all(&SET_FEE_RECEIVER_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.fee_receiver, &mut writer)?;
                Ok(())
            }
            Self::SetMigrationLotSize(args) => {
                writer.write_all(&SET_MIGRATION_LOT_SIZE_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.coin_lot_size, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.pc_lot_size, &mut writer)?;
                Ok(())
            }
            Self::SetMigrationRefund(args) => {
                writer.write_all(&SET_MIGRATION_REFUND_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.migration_refund, &mut writer)?;
                Ok(())
            }
            Self::SetMigrator(args) => {
                writer.write_all(&SET_MIGRATOR_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.migrator, &mut writer)?;
                Ok(())
            }
            Self::SetPendingAuthority(args) => {
                writer.write_all(&SET_PENDING_AUTHORITY_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.pending_authority, &mut writer)?;
                Ok(())
            }
            Self::SetWrapperMint(args) => {
                writer.write_all(&SET_WRAPPER_MINT_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.wrapper_mint, &mut writer)?;
                Ok(())
            }
            Self::SyncWrapper => writer.write_all(&SYNC_WRAPPER_IX_DISCM),
            Self::WrapSolForCurve => writer.write_all(&WRAP_SOL_FOR_CURVE_IX_DISCM),
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
pub const BECOME_AUTHORITY_IX_ACCOUNTS_LEN: usize = 2;
#[derive(Copy, Clone, Debug)]
pub struct BecomeAuthorityAccounts<'me, 'info> {
    pub state: &'me AccountInfo<'info>,
    pub pending_authority: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct BecomeAuthorityKeys {
    pub state: Pubkey,
    pub pending_authority: Pubkey,
}
impl From<BecomeAuthorityAccounts<'_, '_>> for BecomeAuthorityKeys {
    fn from(accounts: BecomeAuthorityAccounts) -> Self {
        Self {
            state: *accounts.state.key,
            pending_authority: *accounts.pending_authority.key,
        }
    }
}
impl From<BecomeAuthorityKeys> for [AccountMeta; BECOME_AUTHORITY_IX_ACCOUNTS_LEN] {
    fn from(keys: BecomeAuthorityKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pending_authority,
                is_signer: true,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; BECOME_AUTHORITY_IX_ACCOUNTS_LEN]> for BecomeAuthorityKeys {
    fn from(pubkeys: [Pubkey; BECOME_AUTHORITY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            state: pubkeys[0],
            pending_authority: pubkeys[1],
        }
    }
}
impl<'info> From<BecomeAuthorityAccounts<'_, 'info>>
for [AccountInfo<'info>; BECOME_AUTHORITY_IX_ACCOUNTS_LEN] {
    fn from(accounts: BecomeAuthorityAccounts<'_, 'info>) -> Self {
        [accounts.state.clone(), accounts.pending_authority.clone()]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; BECOME_AUTHORITY_IX_ACCOUNTS_LEN]>
for BecomeAuthorityAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; BECOME_AUTHORITY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            state: &arr[0],
            pending_authority: &arr[1],
        }
    }
}
pub const BECOME_AUTHORITY_IX_DISCM: [u8; 8usize] = [
    25, 249, 222, 93, 121, 106, 104, 46,
];
#[derive(Clone, Debug, PartialEq)]
pub struct BecomeAuthorityIxData;
impl BecomeAuthorityIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != BECOME_AUTHORITY_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&BECOME_AUTHORITY_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn become_authority_ix_with_program_id(
    program_id: Pubkey,
    keys: BecomeAuthorityKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; BECOME_AUTHORITY_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: BecomeAuthorityIxData.try_to_vec()?,
    })
}
pub fn become_authority_ix(keys: BecomeAuthorityKeys) -> std::io::Result<Instruction> {
    become_authority_ix_with_program_id(MASTERMIND_PROGRAM_ID, keys)
}
pub fn become_authority_invoke_with_program_id(
    program_id: Pubkey,
    accounts: BecomeAuthorityAccounts<'_, '_>,
) -> ProgramResult {
    let keys: BecomeAuthorityKeys = accounts.into();
    let ix = become_authority_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn become_authority_invoke(
    accounts: BecomeAuthorityAccounts<'_, '_>,
) -> ProgramResult {
    become_authority_invoke_with_program_id(MASTERMIND_PROGRAM_ID, accounts)
}
pub fn become_authority_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: BecomeAuthorityAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: BecomeAuthorityKeys = accounts.into();
    let ix = become_authority_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn become_authority_invoke_signed(
    accounts: BecomeAuthorityAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    become_authority_invoke_signed_with_program_id(
        MASTERMIND_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn become_authority_verify_account_keys(
    accounts: BecomeAuthorityAccounts<'_, '_>,
    keys: BecomeAuthorityKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.state.key, keys.state),
        (*accounts.pending_authority.key, keys.pending_authority),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn become_authority_verify_writable_privileges<'me, 'info>(
    accounts: BecomeAuthorityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.state, accounts.pending_authority] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn become_authority_verify_signer_privileges<'me, 'info>(
    accounts: BecomeAuthorityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.pending_authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn become_authority_verify_account_privileges<'me, 'info>(
    accounts: BecomeAuthorityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    become_authority_verify_writable_privileges(accounts)?;
    become_authority_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const BUY_EXACT_IN_IX_ACCOUNTS_LEN: usize = 15;
#[derive(Copy, Clone, Debug)]
pub struct BuyExactInAccounts<'me, 'info> {
    pub state: &'me AccountInfo<'info>,
    pub mint: &'me AccountInfo<'info>,
    pub bonding_curve: &'me AccountInfo<'info>,
    pub bonding_curve_sol_associated_account: &'me AccountInfo<'info>,
    pub bonding_curve_token_associated_account: &'me AccountInfo<'info>,
    pub receiver_associated_account: &'me AccountInfo<'info>,
    pub payer: &'me AccountInfo<'info>,
    pub receiver: &'me AccountInfo<'info>,
    pub fee_receiver: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub rent: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct BuyExactInKeys {
    pub state: Pubkey,
    pub mint: Pubkey,
    pub bonding_curve: Pubkey,
    pub bonding_curve_sol_associated_account: Pubkey,
    pub bonding_curve_token_associated_account: Pubkey,
    pub receiver_associated_account: Pubkey,
    pub payer: Pubkey,
    pub receiver: Pubkey,
    pub fee_receiver: Pubkey,
    pub token_program: Pubkey,
    pub associated_token_program: Pubkey,
    pub system_program: Pubkey,
    pub rent: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<BuyExactInAccounts<'_, '_>> for BuyExactInKeys {
    fn from(accounts: BuyExactInAccounts) -> Self {
        Self {
            state: *accounts.state.key,
            mint: *accounts.mint.key,
            bonding_curve: *accounts.bonding_curve.key,
            bonding_curve_sol_associated_account: *accounts
                .bonding_curve_sol_associated_account
                .key,
            bonding_curve_token_associated_account: *accounts
                .bonding_curve_token_associated_account
                .key,
            receiver_associated_account: *accounts.receiver_associated_account.key,
            payer: *accounts.payer.key,
            receiver: *accounts.receiver.key,
            fee_receiver: *accounts.fee_receiver.key,
            token_program: *accounts.token_program.key,
            associated_token_program: *accounts.associated_token_program.key,
            system_program: *accounts.system_program.key,
            rent: *accounts.rent.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<BuyExactInKeys> for [AccountMeta; BUY_EXACT_IN_IX_ACCOUNTS_LEN] {
    fn from(keys: BuyExactInKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.state,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.bonding_curve,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.bonding_curve_sol_associated_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.bonding_curve_token_associated_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.receiver_associated_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.payer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.receiver,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.fee_receiver,
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
            AccountMeta {
                pubkey: keys.rent,
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
impl From<[Pubkey; BUY_EXACT_IN_IX_ACCOUNTS_LEN]> for BuyExactInKeys {
    fn from(pubkeys: [Pubkey; BUY_EXACT_IN_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            state: pubkeys[0],
            mint: pubkeys[1],
            bonding_curve: pubkeys[2],
            bonding_curve_sol_associated_account: pubkeys[3],
            bonding_curve_token_associated_account: pubkeys[4],
            receiver_associated_account: pubkeys[5],
            payer: pubkeys[6],
            receiver: pubkeys[7],
            fee_receiver: pubkeys[8],
            token_program: pubkeys[9],
            associated_token_program: pubkeys[10],
            system_program: pubkeys[11],
            rent: pubkeys[12],
            event_authority: pubkeys[13],
            program: pubkeys[14],
        }
    }
}
impl<'info> From<BuyExactInAccounts<'_, 'info>>
for [AccountInfo<'info>; BUY_EXACT_IN_IX_ACCOUNTS_LEN] {
    fn from(accounts: BuyExactInAccounts<'_, 'info>) -> Self {
        [
            accounts.state.clone(),
            accounts.mint.clone(),
            accounts.bonding_curve.clone(),
            accounts.bonding_curve_sol_associated_account.clone(),
            accounts.bonding_curve_token_associated_account.clone(),
            accounts.receiver_associated_account.clone(),
            accounts.payer.clone(),
            accounts.receiver.clone(),
            accounts.fee_receiver.clone(),
            accounts.token_program.clone(),
            accounts.associated_token_program.clone(),
            accounts.system_program.clone(),
            accounts.rent.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; BUY_EXACT_IN_IX_ACCOUNTS_LEN]>
for BuyExactInAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; BUY_EXACT_IN_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            state: &arr[0],
            mint: &arr[1],
            bonding_curve: &arr[2],
            bonding_curve_sol_associated_account: &arr[3],
            bonding_curve_token_associated_account: &arr[4],
            receiver_associated_account: &arr[5],
            payer: &arr[6],
            receiver: &arr[7],
            fee_receiver: &arr[8],
            token_program: &arr[9],
            associated_token_program: &arr[10],
            system_program: &arr[11],
            rent: &arr[12],
            event_authority: &arr[13],
            program: &arr[14],
        }
    }
}
pub const BUY_EXACT_IN_IX_DISCM: [u8; 8usize] = [250, 234, 13, 123, 213, 156, 19, 236];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct BuyExactInIxArgs {
    pub bumps: InstructionBumps,
    pub sol_amount_input: u64,
    pub min_tokens_output: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct BuyExactInIxData(pub BuyExactInIxArgs);
impl From<BuyExactInIxArgs> for BuyExactInIxData {
    fn from(args: BuyExactInIxArgs) -> Self {
        Self(args)
    }
}
impl BuyExactInIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != BUY_EXACT_IN_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let bumps = if reader.is_empty() {
            Default::default()
        } else {
            <InstructionBumps>::deserialize(&mut reader)?
        };
        let sol_amount_input: u64 = crate::borsh_de_or_default(&mut reader)?;
        let min_tokens_output: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(BuyExactInIxArgs {
                bumps,
                sol_amount_input,
                min_tokens_output,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&BUY_EXACT_IN_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.bumps, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.sol_amount_input, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.min_tokens_output, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn buy_exact_in_ix_with_program_id(
    program_id: Pubkey,
    keys: BuyExactInKeys,
    args: BuyExactInIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; BUY_EXACT_IN_IX_ACCOUNTS_LEN] = keys.into();
    let data: BuyExactInIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn buy_exact_in_ix(
    keys: BuyExactInKeys,
    args: BuyExactInIxArgs,
) -> std::io::Result<Instruction> {
    buy_exact_in_ix_with_program_id(MASTERMIND_PROGRAM_ID, keys, args)
}
pub fn buy_exact_in_invoke_with_program_id(
    program_id: Pubkey,
    accounts: BuyExactInAccounts<'_, '_>,
    args: BuyExactInIxArgs,
) -> ProgramResult {
    let keys: BuyExactInKeys = accounts.into();
    let ix = buy_exact_in_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn buy_exact_in_invoke(
    accounts: BuyExactInAccounts<'_, '_>,
    args: BuyExactInIxArgs,
) -> ProgramResult {
    buy_exact_in_invoke_with_program_id(MASTERMIND_PROGRAM_ID, accounts, args)
}
pub fn buy_exact_in_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: BuyExactInAccounts<'_, '_>,
    args: BuyExactInIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: BuyExactInKeys = accounts.into();
    let ix = buy_exact_in_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn buy_exact_in_invoke_signed(
    accounts: BuyExactInAccounts<'_, '_>,
    args: BuyExactInIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    buy_exact_in_invoke_signed_with_program_id(
        MASTERMIND_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn buy_exact_in_verify_account_keys(
    accounts: BuyExactInAccounts<'_, '_>,
    keys: BuyExactInKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.state.key, keys.state),
        (*accounts.mint.key, keys.mint),
        (*accounts.bonding_curve.key, keys.bonding_curve),
        (
            *accounts.bonding_curve_sol_associated_account.key,
            keys.bonding_curve_sol_associated_account,
        ),
        (
            *accounts.bonding_curve_token_associated_account.key,
            keys.bonding_curve_token_associated_account,
        ),
        (*accounts.receiver_associated_account.key, keys.receiver_associated_account),
        (*accounts.payer.key, keys.payer),
        (*accounts.receiver.key, keys.receiver),
        (*accounts.fee_receiver.key, keys.fee_receiver),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.associated_token_program.key, keys.associated_token_program),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.rent.key, keys.rent),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn buy_exact_in_verify_writable_privileges<'me, 'info>(
    accounts: BuyExactInAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.bonding_curve,
        accounts.bonding_curve_sol_associated_account,
        accounts.bonding_curve_token_associated_account,
        accounts.receiver_associated_account,
        accounts.payer,
        accounts.fee_receiver,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn buy_exact_in_verify_signer_privileges<'me, 'info>(
    accounts: BuyExactInAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.payer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn buy_exact_in_verify_account_privileges<'me, 'info>(
    accounts: BuyExactInAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    buy_exact_in_verify_writable_privileges(accounts)?;
    buy_exact_in_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const BUY_EXACT_OUT_IX_ACCOUNTS_LEN: usize = 15;
#[derive(Copy, Clone, Debug)]
pub struct BuyExactOutAccounts<'me, 'info> {
    pub state: &'me AccountInfo<'info>,
    pub mint: &'me AccountInfo<'info>,
    pub bonding_curve: &'me AccountInfo<'info>,
    pub bonding_curve_sol_associated_account: &'me AccountInfo<'info>,
    pub bonding_curve_token_associated_account: &'me AccountInfo<'info>,
    pub receiver_associated_account: &'me AccountInfo<'info>,
    pub payer: &'me AccountInfo<'info>,
    pub receiver: &'me AccountInfo<'info>,
    pub fee_receiver: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub rent: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct BuyExactOutKeys {
    pub state: Pubkey,
    pub mint: Pubkey,
    pub bonding_curve: Pubkey,
    pub bonding_curve_sol_associated_account: Pubkey,
    pub bonding_curve_token_associated_account: Pubkey,
    pub receiver_associated_account: Pubkey,
    pub payer: Pubkey,
    pub receiver: Pubkey,
    pub fee_receiver: Pubkey,
    pub token_program: Pubkey,
    pub associated_token_program: Pubkey,
    pub system_program: Pubkey,
    pub rent: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<BuyExactOutAccounts<'_, '_>> for BuyExactOutKeys {
    fn from(accounts: BuyExactOutAccounts) -> Self {
        Self {
            state: *accounts.state.key,
            mint: *accounts.mint.key,
            bonding_curve: *accounts.bonding_curve.key,
            bonding_curve_sol_associated_account: *accounts
                .bonding_curve_sol_associated_account
                .key,
            bonding_curve_token_associated_account: *accounts
                .bonding_curve_token_associated_account
                .key,
            receiver_associated_account: *accounts.receiver_associated_account.key,
            payer: *accounts.payer.key,
            receiver: *accounts.receiver.key,
            fee_receiver: *accounts.fee_receiver.key,
            token_program: *accounts.token_program.key,
            associated_token_program: *accounts.associated_token_program.key,
            system_program: *accounts.system_program.key,
            rent: *accounts.rent.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<BuyExactOutKeys> for [AccountMeta; BUY_EXACT_OUT_IX_ACCOUNTS_LEN] {
    fn from(keys: BuyExactOutKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.state,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.bonding_curve,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.bonding_curve_sol_associated_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.bonding_curve_token_associated_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.receiver_associated_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.payer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.receiver,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.fee_receiver,
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
            AccountMeta {
                pubkey: keys.rent,
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
impl From<[Pubkey; BUY_EXACT_OUT_IX_ACCOUNTS_LEN]> for BuyExactOutKeys {
    fn from(pubkeys: [Pubkey; BUY_EXACT_OUT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            state: pubkeys[0],
            mint: pubkeys[1],
            bonding_curve: pubkeys[2],
            bonding_curve_sol_associated_account: pubkeys[3],
            bonding_curve_token_associated_account: pubkeys[4],
            receiver_associated_account: pubkeys[5],
            payer: pubkeys[6],
            receiver: pubkeys[7],
            fee_receiver: pubkeys[8],
            token_program: pubkeys[9],
            associated_token_program: pubkeys[10],
            system_program: pubkeys[11],
            rent: pubkeys[12],
            event_authority: pubkeys[13],
            program: pubkeys[14],
        }
    }
}
impl<'info> From<BuyExactOutAccounts<'_, 'info>>
for [AccountInfo<'info>; BUY_EXACT_OUT_IX_ACCOUNTS_LEN] {
    fn from(accounts: BuyExactOutAccounts<'_, 'info>) -> Self {
        [
            accounts.state.clone(),
            accounts.mint.clone(),
            accounts.bonding_curve.clone(),
            accounts.bonding_curve_sol_associated_account.clone(),
            accounts.bonding_curve_token_associated_account.clone(),
            accounts.receiver_associated_account.clone(),
            accounts.payer.clone(),
            accounts.receiver.clone(),
            accounts.fee_receiver.clone(),
            accounts.token_program.clone(),
            accounts.associated_token_program.clone(),
            accounts.system_program.clone(),
            accounts.rent.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; BUY_EXACT_OUT_IX_ACCOUNTS_LEN]>
for BuyExactOutAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; BUY_EXACT_OUT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            state: &arr[0],
            mint: &arr[1],
            bonding_curve: &arr[2],
            bonding_curve_sol_associated_account: &arr[3],
            bonding_curve_token_associated_account: &arr[4],
            receiver_associated_account: &arr[5],
            payer: &arr[6],
            receiver: &arr[7],
            fee_receiver: &arr[8],
            token_program: &arr[9],
            associated_token_program: &arr[10],
            system_program: &arr[11],
            rent: &arr[12],
            event_authority: &arr[13],
            program: &arr[14],
        }
    }
}
pub const BUY_EXACT_OUT_IX_DISCM: [u8; 8usize] = [24, 211, 116, 40, 105, 3, 153, 56];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct BuyExactOutIxArgs {
    pub bumps: InstructionBumps,
    pub max_sol_amount_input: u64,
    pub tokens_output: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct BuyExactOutIxData(pub BuyExactOutIxArgs);
impl From<BuyExactOutIxArgs> for BuyExactOutIxData {
    fn from(args: BuyExactOutIxArgs) -> Self {
        Self(args)
    }
}
impl BuyExactOutIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != BUY_EXACT_OUT_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let bumps = if reader.is_empty() {
            Default::default()
        } else {
            <InstructionBumps>::deserialize(&mut reader)?
        };
        let max_sol_amount_input: u64 = crate::borsh_de_or_default(&mut reader)?;
        let tokens_output: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(BuyExactOutIxArgs {
                bumps,
                max_sol_amount_input,
                tokens_output,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&BUY_EXACT_OUT_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.bumps, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.max_sol_amount_input, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.tokens_output, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn buy_exact_out_ix_with_program_id(
    program_id: Pubkey,
    keys: BuyExactOutKeys,
    args: BuyExactOutIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; BUY_EXACT_OUT_IX_ACCOUNTS_LEN] = keys.into();
    let data: BuyExactOutIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn buy_exact_out_ix(
    keys: BuyExactOutKeys,
    args: BuyExactOutIxArgs,
) -> std::io::Result<Instruction> {
    buy_exact_out_ix_with_program_id(MASTERMIND_PROGRAM_ID, keys, args)
}
pub fn buy_exact_out_invoke_with_program_id(
    program_id: Pubkey,
    accounts: BuyExactOutAccounts<'_, '_>,
    args: BuyExactOutIxArgs,
) -> ProgramResult {
    let keys: BuyExactOutKeys = accounts.into();
    let ix = buy_exact_out_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn buy_exact_out_invoke(
    accounts: BuyExactOutAccounts<'_, '_>,
    args: BuyExactOutIxArgs,
) -> ProgramResult {
    buy_exact_out_invoke_with_program_id(MASTERMIND_PROGRAM_ID, accounts, args)
}
pub fn buy_exact_out_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: BuyExactOutAccounts<'_, '_>,
    args: BuyExactOutIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: BuyExactOutKeys = accounts.into();
    let ix = buy_exact_out_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn buy_exact_out_invoke_signed(
    accounts: BuyExactOutAccounts<'_, '_>,
    args: BuyExactOutIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    buy_exact_out_invoke_signed_with_program_id(
        MASTERMIND_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn buy_exact_out_verify_account_keys(
    accounts: BuyExactOutAccounts<'_, '_>,
    keys: BuyExactOutKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.state.key, keys.state),
        (*accounts.mint.key, keys.mint),
        (*accounts.bonding_curve.key, keys.bonding_curve),
        (
            *accounts.bonding_curve_sol_associated_account.key,
            keys.bonding_curve_sol_associated_account,
        ),
        (
            *accounts.bonding_curve_token_associated_account.key,
            keys.bonding_curve_token_associated_account,
        ),
        (*accounts.receiver_associated_account.key, keys.receiver_associated_account),
        (*accounts.payer.key, keys.payer),
        (*accounts.receiver.key, keys.receiver),
        (*accounts.fee_receiver.key, keys.fee_receiver),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.associated_token_program.key, keys.associated_token_program),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.rent.key, keys.rent),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn buy_exact_out_verify_writable_privileges<'me, 'info>(
    accounts: BuyExactOutAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.bonding_curve,
        accounts.bonding_curve_sol_associated_account,
        accounts.bonding_curve_token_associated_account,
        accounts.receiver_associated_account,
        accounts.payer,
        accounts.fee_receiver,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn buy_exact_out_verify_signer_privileges<'me, 'info>(
    accounts: BuyExactOutAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.payer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn buy_exact_out_verify_account_privileges<'me, 'info>(
    accounts: BuyExactOutAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    buy_exact_out_verify_writable_privileges(accounts)?;
    buy_exact_out_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const BUY_MAX_OUT_IX_ACCOUNTS_LEN: usize = 15;
#[derive(Copy, Clone, Debug)]
pub struct BuyMaxOutAccounts<'me, 'info> {
    pub state: &'me AccountInfo<'info>,
    pub mint: &'me AccountInfo<'info>,
    pub bonding_curve: &'me AccountInfo<'info>,
    pub bonding_curve_sol_associated_account: &'me AccountInfo<'info>,
    pub bonding_curve_token_associated_account: &'me AccountInfo<'info>,
    pub receiver_associated_account: &'me AccountInfo<'info>,
    pub payer: &'me AccountInfo<'info>,
    pub receiver: &'me AccountInfo<'info>,
    pub fee_receiver: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub rent: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct BuyMaxOutKeys {
    pub state: Pubkey,
    pub mint: Pubkey,
    pub bonding_curve: Pubkey,
    pub bonding_curve_sol_associated_account: Pubkey,
    pub bonding_curve_token_associated_account: Pubkey,
    pub receiver_associated_account: Pubkey,
    pub payer: Pubkey,
    pub receiver: Pubkey,
    pub fee_receiver: Pubkey,
    pub token_program: Pubkey,
    pub associated_token_program: Pubkey,
    pub system_program: Pubkey,
    pub rent: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<BuyMaxOutAccounts<'_, '_>> for BuyMaxOutKeys {
    fn from(accounts: BuyMaxOutAccounts) -> Self {
        Self {
            state: *accounts.state.key,
            mint: *accounts.mint.key,
            bonding_curve: *accounts.bonding_curve.key,
            bonding_curve_sol_associated_account: *accounts
                .bonding_curve_sol_associated_account
                .key,
            bonding_curve_token_associated_account: *accounts
                .bonding_curve_token_associated_account
                .key,
            receiver_associated_account: *accounts.receiver_associated_account.key,
            payer: *accounts.payer.key,
            receiver: *accounts.receiver.key,
            fee_receiver: *accounts.fee_receiver.key,
            token_program: *accounts.token_program.key,
            associated_token_program: *accounts.associated_token_program.key,
            system_program: *accounts.system_program.key,
            rent: *accounts.rent.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<BuyMaxOutKeys> for [AccountMeta; BUY_MAX_OUT_IX_ACCOUNTS_LEN] {
    fn from(keys: BuyMaxOutKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.state,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.bonding_curve,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.bonding_curve_sol_associated_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.bonding_curve_token_associated_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.receiver_associated_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.payer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.receiver,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.fee_receiver,
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
            AccountMeta {
                pubkey: keys.rent,
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
impl From<[Pubkey; BUY_MAX_OUT_IX_ACCOUNTS_LEN]> for BuyMaxOutKeys {
    fn from(pubkeys: [Pubkey; BUY_MAX_OUT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            state: pubkeys[0],
            mint: pubkeys[1],
            bonding_curve: pubkeys[2],
            bonding_curve_sol_associated_account: pubkeys[3],
            bonding_curve_token_associated_account: pubkeys[4],
            receiver_associated_account: pubkeys[5],
            payer: pubkeys[6],
            receiver: pubkeys[7],
            fee_receiver: pubkeys[8],
            token_program: pubkeys[9],
            associated_token_program: pubkeys[10],
            system_program: pubkeys[11],
            rent: pubkeys[12],
            event_authority: pubkeys[13],
            program: pubkeys[14],
        }
    }
}
impl<'info> From<BuyMaxOutAccounts<'_, 'info>>
for [AccountInfo<'info>; BUY_MAX_OUT_IX_ACCOUNTS_LEN] {
    fn from(accounts: BuyMaxOutAccounts<'_, 'info>) -> Self {
        [
            accounts.state.clone(),
            accounts.mint.clone(),
            accounts.bonding_curve.clone(),
            accounts.bonding_curve_sol_associated_account.clone(),
            accounts.bonding_curve_token_associated_account.clone(),
            accounts.receiver_associated_account.clone(),
            accounts.payer.clone(),
            accounts.receiver.clone(),
            accounts.fee_receiver.clone(),
            accounts.token_program.clone(),
            accounts.associated_token_program.clone(),
            accounts.system_program.clone(),
            accounts.rent.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; BUY_MAX_OUT_IX_ACCOUNTS_LEN]>
for BuyMaxOutAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; BUY_MAX_OUT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            state: &arr[0],
            mint: &arr[1],
            bonding_curve: &arr[2],
            bonding_curve_sol_associated_account: &arr[3],
            bonding_curve_token_associated_account: &arr[4],
            receiver_associated_account: &arr[5],
            payer: &arr[6],
            receiver: &arr[7],
            fee_receiver: &arr[8],
            token_program: &arr[9],
            associated_token_program: &arr[10],
            system_program: &arr[11],
            rent: &arr[12],
            event_authority: &arr[13],
            program: &arr[14],
        }
    }
}
pub const BUY_MAX_OUT_IX_DISCM: [u8; 8usize] = [96, 177, 203, 117, 183, 65, 196, 177];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct BuyMaxOutIxArgs {
    pub bumps: InstructionBumps,
    pub max_sol_amount_input: u64,
    pub min_tokens_output: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct BuyMaxOutIxData(pub BuyMaxOutIxArgs);
impl From<BuyMaxOutIxArgs> for BuyMaxOutIxData {
    fn from(args: BuyMaxOutIxArgs) -> Self {
        Self(args)
    }
}
impl BuyMaxOutIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != BUY_MAX_OUT_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let bumps = if reader.is_empty() {
            Default::default()
        } else {
            <InstructionBumps>::deserialize(&mut reader)?
        };
        let max_sol_amount_input: u64 = crate::borsh_de_or_default(&mut reader)?;
        let min_tokens_output: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(BuyMaxOutIxArgs {
                bumps,
                max_sol_amount_input,
                min_tokens_output,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&BUY_MAX_OUT_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.bumps, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.max_sol_amount_input, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.min_tokens_output, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn buy_max_out_ix_with_program_id(
    program_id: Pubkey,
    keys: BuyMaxOutKeys,
    args: BuyMaxOutIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; BUY_MAX_OUT_IX_ACCOUNTS_LEN] = keys.into();
    let data: BuyMaxOutIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn buy_max_out_ix(
    keys: BuyMaxOutKeys,
    args: BuyMaxOutIxArgs,
) -> std::io::Result<Instruction> {
    buy_max_out_ix_with_program_id(MASTERMIND_PROGRAM_ID, keys, args)
}
pub fn buy_max_out_invoke_with_program_id(
    program_id: Pubkey,
    accounts: BuyMaxOutAccounts<'_, '_>,
    args: BuyMaxOutIxArgs,
) -> ProgramResult {
    let keys: BuyMaxOutKeys = accounts.into();
    let ix = buy_max_out_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn buy_max_out_invoke(
    accounts: BuyMaxOutAccounts<'_, '_>,
    args: BuyMaxOutIxArgs,
) -> ProgramResult {
    buy_max_out_invoke_with_program_id(MASTERMIND_PROGRAM_ID, accounts, args)
}
pub fn buy_max_out_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: BuyMaxOutAccounts<'_, '_>,
    args: BuyMaxOutIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: BuyMaxOutKeys = accounts.into();
    let ix = buy_max_out_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn buy_max_out_invoke_signed(
    accounts: BuyMaxOutAccounts<'_, '_>,
    args: BuyMaxOutIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    buy_max_out_invoke_signed_with_program_id(
        MASTERMIND_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn buy_max_out_verify_account_keys(
    accounts: BuyMaxOutAccounts<'_, '_>,
    keys: BuyMaxOutKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.state.key, keys.state),
        (*accounts.mint.key, keys.mint),
        (*accounts.bonding_curve.key, keys.bonding_curve),
        (
            *accounts.bonding_curve_sol_associated_account.key,
            keys.bonding_curve_sol_associated_account,
        ),
        (
            *accounts.bonding_curve_token_associated_account.key,
            keys.bonding_curve_token_associated_account,
        ),
        (*accounts.receiver_associated_account.key, keys.receiver_associated_account),
        (*accounts.payer.key, keys.payer),
        (*accounts.receiver.key, keys.receiver),
        (*accounts.fee_receiver.key, keys.fee_receiver),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.associated_token_program.key, keys.associated_token_program),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.rent.key, keys.rent),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn buy_max_out_verify_writable_privileges<'me, 'info>(
    accounts: BuyMaxOutAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.bonding_curve,
        accounts.bonding_curve_sol_associated_account,
        accounts.bonding_curve_token_associated_account,
        accounts.receiver_associated_account,
        accounts.payer,
        accounts.fee_receiver,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn buy_max_out_verify_signer_privileges<'me, 'info>(
    accounts: BuyMaxOutAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.payer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn buy_max_out_verify_account_privileges<'me, 'info>(
    accounts: BuyMaxOutAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    buy_max_out_verify_writable_privileges(accounts)?;
    buy_max_out_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const COLLECT_CP_FEES_IX_ACCOUNTS_LEN: usize = 32;
#[derive(Copy, Clone, Debug)]
pub struct CollectCpFeesAccounts<'me, 'info> {
    pub state: &'me AccountInfo<'info>,
    pub locking_program: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub fee_nft_owner: &'me AccountInfo<'info>,
    pub fee_nft_account: &'me AccountInfo<'info>,
    pub locked_liquidity: &'me AccountInfo<'info>,
    pub cpmm_program: &'me AccountInfo<'info>,
    pub cp_authority: &'me AccountInfo<'info>,
    pub pool_state: &'me AccountInfo<'info>,
    pub lp_mint: &'me AccountInfo<'info>,
    pub referral_pool_sol_account: &'me AccountInfo<'info>,
    pub wrapper_program: &'me AccountInfo<'info>,
    pub wrapper_state: &'me AccountInfo<'info>,
    pub wrapper_mint: &'me AccountInfo<'info>,
    pub recipient_token_0_account: &'me AccountInfo<'info>,
    pub recipient_token_1_account: &'me AccountInfo<'info>,
    pub token_0_vault: &'me AccountInfo<'info>,
    pub token_1_vault: &'me AccountInfo<'info>,
    pub vault_0_mint: &'me AccountInfo<'info>,
    pub vault_1_mint: &'me AccountInfo<'info>,
    pub locked_lp_vault: &'me AccountInfo<'info>,
    pub token_0_program: &'me AccountInfo<'info>,
    pub token_1_program: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub token_program_2022: &'me AccountInfo<'info>,
    pub memo_program: &'me AccountInfo<'info>,
    pub payer: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
    pub rent: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CollectCpFeesKeys {
    pub state: Pubkey,
    pub locking_program: Pubkey,
    pub authority: Pubkey,
    pub fee_nft_owner: Pubkey,
    pub fee_nft_account: Pubkey,
    pub locked_liquidity: Pubkey,
    pub cpmm_program: Pubkey,
    pub cp_authority: Pubkey,
    pub pool_state: Pubkey,
    pub lp_mint: Pubkey,
    pub referral_pool_sol_account: Pubkey,
    pub wrapper_program: Pubkey,
    pub wrapper_state: Pubkey,
    pub wrapper_mint: Pubkey,
    pub recipient_token_0_account: Pubkey,
    pub recipient_token_1_account: Pubkey,
    pub token_0_vault: Pubkey,
    pub token_1_vault: Pubkey,
    pub vault_0_mint: Pubkey,
    pub vault_1_mint: Pubkey,
    pub locked_lp_vault: Pubkey,
    pub token_0_program: Pubkey,
    pub token_1_program: Pubkey,
    pub token_program: Pubkey,
    pub token_program_2022: Pubkey,
    pub memo_program: Pubkey,
    pub payer: Pubkey,
    pub system_program: Pubkey,
    pub associated_token_program: Pubkey,
    pub rent: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<CollectCpFeesAccounts<'_, '_>> for CollectCpFeesKeys {
    fn from(accounts: CollectCpFeesAccounts) -> Self {
        Self {
            state: *accounts.state.key,
            locking_program: *accounts.locking_program.key,
            authority: *accounts.authority.key,
            fee_nft_owner: *accounts.fee_nft_owner.key,
            fee_nft_account: *accounts.fee_nft_account.key,
            locked_liquidity: *accounts.locked_liquidity.key,
            cpmm_program: *accounts.cpmm_program.key,
            cp_authority: *accounts.cp_authority.key,
            pool_state: *accounts.pool_state.key,
            lp_mint: *accounts.lp_mint.key,
            referral_pool_sol_account: *accounts.referral_pool_sol_account.key,
            wrapper_program: *accounts.wrapper_program.key,
            wrapper_state: *accounts.wrapper_state.key,
            wrapper_mint: *accounts.wrapper_mint.key,
            recipient_token_0_account: *accounts.recipient_token_0_account.key,
            recipient_token_1_account: *accounts.recipient_token_1_account.key,
            token_0_vault: *accounts.token_0_vault.key,
            token_1_vault: *accounts.token_1_vault.key,
            vault_0_mint: *accounts.vault_0_mint.key,
            vault_1_mint: *accounts.vault_1_mint.key,
            locked_lp_vault: *accounts.locked_lp_vault.key,
            token_0_program: *accounts.token_0_program.key,
            token_1_program: *accounts.token_1_program.key,
            token_program: *accounts.token_program.key,
            token_program_2022: *accounts.token_program_2022.key,
            memo_program: *accounts.memo_program.key,
            payer: *accounts.payer.key,
            system_program: *accounts.system_program.key,
            associated_token_program: *accounts.associated_token_program.key,
            rent: *accounts.rent.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<CollectCpFeesKeys> for [AccountMeta; COLLECT_CP_FEES_IX_ACCOUNTS_LEN] {
    fn from(keys: CollectCpFeesKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.state,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.locking_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.fee_nft_owner,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.fee_nft_account,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.locked_liquidity,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.cpmm_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.cp_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.lp_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.referral_pool_sol_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.wrapper_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.wrapper_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.wrapper_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.recipient_token_0_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.recipient_token_1_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_0_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_1_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vault_0_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vault_1_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.locked_lp_vault,
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
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_program_2022,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.memo_program,
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
                pubkey: keys.associated_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.rent,
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
impl From<[Pubkey; COLLECT_CP_FEES_IX_ACCOUNTS_LEN]> for CollectCpFeesKeys {
    fn from(pubkeys: [Pubkey; COLLECT_CP_FEES_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            state: pubkeys[0],
            locking_program: pubkeys[1],
            authority: pubkeys[2],
            fee_nft_owner: pubkeys[3],
            fee_nft_account: pubkeys[4],
            locked_liquidity: pubkeys[5],
            cpmm_program: pubkeys[6],
            cp_authority: pubkeys[7],
            pool_state: pubkeys[8],
            lp_mint: pubkeys[9],
            referral_pool_sol_account: pubkeys[10],
            wrapper_program: pubkeys[11],
            wrapper_state: pubkeys[12],
            wrapper_mint: pubkeys[13],
            recipient_token_0_account: pubkeys[14],
            recipient_token_1_account: pubkeys[15],
            token_0_vault: pubkeys[16],
            token_1_vault: pubkeys[17],
            vault_0_mint: pubkeys[18],
            vault_1_mint: pubkeys[19],
            locked_lp_vault: pubkeys[20],
            token_0_program: pubkeys[21],
            token_1_program: pubkeys[22],
            token_program: pubkeys[23],
            token_program_2022: pubkeys[24],
            memo_program: pubkeys[25],
            payer: pubkeys[26],
            system_program: pubkeys[27],
            associated_token_program: pubkeys[28],
            rent: pubkeys[29],
            event_authority: pubkeys[30],
            program: pubkeys[31],
        }
    }
}
impl<'info> From<CollectCpFeesAccounts<'_, 'info>>
for [AccountInfo<'info>; COLLECT_CP_FEES_IX_ACCOUNTS_LEN] {
    fn from(accounts: CollectCpFeesAccounts<'_, 'info>) -> Self {
        [
            accounts.state.clone(),
            accounts.locking_program.clone(),
            accounts.authority.clone(),
            accounts.fee_nft_owner.clone(),
            accounts.fee_nft_account.clone(),
            accounts.locked_liquidity.clone(),
            accounts.cpmm_program.clone(),
            accounts.cp_authority.clone(),
            accounts.pool_state.clone(),
            accounts.lp_mint.clone(),
            accounts.referral_pool_sol_account.clone(),
            accounts.wrapper_program.clone(),
            accounts.wrapper_state.clone(),
            accounts.wrapper_mint.clone(),
            accounts.recipient_token_0_account.clone(),
            accounts.recipient_token_1_account.clone(),
            accounts.token_0_vault.clone(),
            accounts.token_1_vault.clone(),
            accounts.vault_0_mint.clone(),
            accounts.vault_1_mint.clone(),
            accounts.locked_lp_vault.clone(),
            accounts.token_0_program.clone(),
            accounts.token_1_program.clone(),
            accounts.token_program.clone(),
            accounts.token_program_2022.clone(),
            accounts.memo_program.clone(),
            accounts.payer.clone(),
            accounts.system_program.clone(),
            accounts.associated_token_program.clone(),
            accounts.rent.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; COLLECT_CP_FEES_IX_ACCOUNTS_LEN]>
for CollectCpFeesAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; COLLECT_CP_FEES_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            state: &arr[0],
            locking_program: &arr[1],
            authority: &arr[2],
            fee_nft_owner: &arr[3],
            fee_nft_account: &arr[4],
            locked_liquidity: &arr[5],
            cpmm_program: &arr[6],
            cp_authority: &arr[7],
            pool_state: &arr[8],
            lp_mint: &arr[9],
            referral_pool_sol_account: &arr[10],
            wrapper_program: &arr[11],
            wrapper_state: &arr[12],
            wrapper_mint: &arr[13],
            recipient_token_0_account: &arr[14],
            recipient_token_1_account: &arr[15],
            token_0_vault: &arr[16],
            token_1_vault: &arr[17],
            vault_0_mint: &arr[18],
            vault_1_mint: &arr[19],
            locked_lp_vault: &arr[20],
            token_0_program: &arr[21],
            token_1_program: &arr[22],
            token_program: &arr[23],
            token_program_2022: &arr[24],
            memo_program: &arr[25],
            payer: &arr[26],
            system_program: &arr[27],
            associated_token_program: &arr[28],
            rent: &arr[29],
            event_authority: &arr[30],
            program: &arr[31],
        }
    }
}
pub const COLLECT_CP_FEES_IX_DISCM: [u8; 8usize] = [8, 30, 51, 199, 209, 184, 247, 133];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CollectCpFeesIxArgs {
    pub fee_nft_owner_bump: u8,
}
#[derive(Clone, Debug, PartialEq)]
pub struct CollectCpFeesIxData(pub CollectCpFeesIxArgs);
impl From<CollectCpFeesIxArgs> for CollectCpFeesIxData {
    fn from(args: CollectCpFeesIxArgs) -> Self {
        Self(args)
    }
}
impl CollectCpFeesIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != COLLECT_CP_FEES_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let fee_nft_owner_bump: u8 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(CollectCpFeesIxArgs {
                fee_nft_owner_bump,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&COLLECT_CP_FEES_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.fee_nft_owner_bump, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn collect_cp_fees_ix_with_program_id(
    program_id: Pubkey,
    keys: CollectCpFeesKeys,
    args: CollectCpFeesIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; COLLECT_CP_FEES_IX_ACCOUNTS_LEN] = keys.into();
    let data: CollectCpFeesIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn collect_cp_fees_ix(
    keys: CollectCpFeesKeys,
    args: CollectCpFeesIxArgs,
) -> std::io::Result<Instruction> {
    collect_cp_fees_ix_with_program_id(MASTERMIND_PROGRAM_ID, keys, args)
}
pub fn collect_cp_fees_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CollectCpFeesAccounts<'_, '_>,
    args: CollectCpFeesIxArgs,
) -> ProgramResult {
    let keys: CollectCpFeesKeys = accounts.into();
    let ix = collect_cp_fees_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn collect_cp_fees_invoke(
    accounts: CollectCpFeesAccounts<'_, '_>,
    args: CollectCpFeesIxArgs,
) -> ProgramResult {
    collect_cp_fees_invoke_with_program_id(MASTERMIND_PROGRAM_ID, accounts, args)
}
pub fn collect_cp_fees_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CollectCpFeesAccounts<'_, '_>,
    args: CollectCpFeesIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CollectCpFeesKeys = accounts.into();
    let ix = collect_cp_fees_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn collect_cp_fees_invoke_signed(
    accounts: CollectCpFeesAccounts<'_, '_>,
    args: CollectCpFeesIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    collect_cp_fees_invoke_signed_with_program_id(
        MASTERMIND_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn collect_cp_fees_verify_account_keys(
    accounts: CollectCpFeesAccounts<'_, '_>,
    keys: CollectCpFeesKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.state.key, keys.state),
        (*accounts.locking_program.key, keys.locking_program),
        (*accounts.authority.key, keys.authority),
        (*accounts.fee_nft_owner.key, keys.fee_nft_owner),
        (*accounts.fee_nft_account.key, keys.fee_nft_account),
        (*accounts.locked_liquidity.key, keys.locked_liquidity),
        (*accounts.cpmm_program.key, keys.cpmm_program),
        (*accounts.cp_authority.key, keys.cp_authority),
        (*accounts.pool_state.key, keys.pool_state),
        (*accounts.lp_mint.key, keys.lp_mint),
        (*accounts.referral_pool_sol_account.key, keys.referral_pool_sol_account),
        (*accounts.wrapper_program.key, keys.wrapper_program),
        (*accounts.wrapper_state.key, keys.wrapper_state),
        (*accounts.wrapper_mint.key, keys.wrapper_mint),
        (*accounts.recipient_token_0_account.key, keys.recipient_token_0_account),
        (*accounts.recipient_token_1_account.key, keys.recipient_token_1_account),
        (*accounts.token_0_vault.key, keys.token_0_vault),
        (*accounts.token_1_vault.key, keys.token_1_vault),
        (*accounts.vault_0_mint.key, keys.vault_0_mint),
        (*accounts.vault_1_mint.key, keys.vault_1_mint),
        (*accounts.locked_lp_vault.key, keys.locked_lp_vault),
        (*accounts.token_0_program.key, keys.token_0_program),
        (*accounts.token_1_program.key, keys.token_1_program),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.token_program_2022.key, keys.token_program_2022),
        (*accounts.memo_program.key, keys.memo_program),
        (*accounts.payer.key, keys.payer),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.associated_token_program.key, keys.associated_token_program),
        (*accounts.rent.key, keys.rent),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn collect_cp_fees_verify_writable_privileges<'me, 'info>(
    accounts: CollectCpFeesAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.fee_nft_owner,
        accounts.locked_liquidity,
        accounts.pool_state,
        accounts.lp_mint,
        accounts.referral_pool_sol_account,
        accounts.wrapper_state,
        accounts.wrapper_mint,
        accounts.recipient_token_0_account,
        accounts.recipient_token_1_account,
        accounts.token_0_vault,
        accounts.token_1_vault,
        accounts.vault_0_mint,
        accounts.vault_1_mint,
        accounts.locked_lp_vault,
        accounts.payer,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn collect_cp_fees_verify_signer_privileges<'me, 'info>(
    accounts: CollectCpFeesAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.payer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn collect_cp_fees_verify_account_privileges<'me, 'info>(
    accounts: CollectCpFeesAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    collect_cp_fees_verify_writable_privileges(accounts)?;
    collect_cp_fees_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CREATE_IX_ACCOUNTS_LEN: usize = 12;
#[derive(Copy, Clone, Debug)]
pub struct CreateAccounts<'me, 'info> {
    pub state: &'me AccountInfo<'info>,
    pub bonding_curve: &'me AccountInfo<'info>,
    pub mint: &'me AccountInfo<'info>,
    pub bonding_curve_sol_vault: &'me AccountInfo<'info>,
    pub bonding_curve_token_associated_account: &'me AccountInfo<'info>,
    pub payer: &'me AccountInfo<'info>,
    pub token_program_2022: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub rent: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CreateKeys {
    pub state: Pubkey,
    pub bonding_curve: Pubkey,
    pub mint: Pubkey,
    pub bonding_curve_sol_vault: Pubkey,
    pub bonding_curve_token_associated_account: Pubkey,
    pub payer: Pubkey,
    pub token_program_2022: Pubkey,
    pub associated_token_program: Pubkey,
    pub system_program: Pubkey,
    pub rent: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<CreateAccounts<'_, '_>> for CreateKeys {
    fn from(accounts: CreateAccounts) -> Self {
        Self {
            state: *accounts.state.key,
            bonding_curve: *accounts.bonding_curve.key,
            mint: *accounts.mint.key,
            bonding_curve_sol_vault: *accounts.bonding_curve_sol_vault.key,
            bonding_curve_token_associated_account: *accounts
                .bonding_curve_token_associated_account
                .key,
            payer: *accounts.payer.key,
            token_program_2022: *accounts.token_program_2022.key,
            associated_token_program: *accounts.associated_token_program.key,
            system_program: *accounts.system_program.key,
            rent: *accounts.rent.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<CreateKeys> for [AccountMeta; CREATE_IX_ACCOUNTS_LEN] {
    fn from(keys: CreateKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.state,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.bonding_curve,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.bonding_curve_sol_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.bonding_curve_token_associated_account,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.payer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_program_2022,
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
                pubkey: keys.rent,
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
impl From<[Pubkey; CREATE_IX_ACCOUNTS_LEN]> for CreateKeys {
    fn from(pubkeys: [Pubkey; CREATE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            state: pubkeys[0],
            bonding_curve: pubkeys[1],
            mint: pubkeys[2],
            bonding_curve_sol_vault: pubkeys[3],
            bonding_curve_token_associated_account: pubkeys[4],
            payer: pubkeys[5],
            token_program_2022: pubkeys[6],
            associated_token_program: pubkeys[7],
            system_program: pubkeys[8],
            rent: pubkeys[9],
            event_authority: pubkeys[10],
            program: pubkeys[11],
        }
    }
}
impl<'info> From<CreateAccounts<'_, 'info>>
for [AccountInfo<'info>; CREATE_IX_ACCOUNTS_LEN] {
    fn from(accounts: CreateAccounts<'_, 'info>) -> Self {
        [
            accounts.state.clone(),
            accounts.bonding_curve.clone(),
            accounts.mint.clone(),
            accounts.bonding_curve_sol_vault.clone(),
            accounts.bonding_curve_token_associated_account.clone(),
            accounts.payer.clone(),
            accounts.token_program_2022.clone(),
            accounts.associated_token_program.clone(),
            accounts.system_program.clone(),
            accounts.rent.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CREATE_IX_ACCOUNTS_LEN]>
for CreateAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; CREATE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            state: &arr[0],
            bonding_curve: &arr[1],
            mint: &arr[2],
            bonding_curve_sol_vault: &arr[3],
            bonding_curve_token_associated_account: &arr[4],
            payer: &arr[5],
            token_program_2022: &arr[6],
            associated_token_program: &arr[7],
            system_program: &arr[8],
            rent: &arr[9],
            event_authority: &arr[10],
            program: &arr[11],
        }
    }
}
pub const CREATE_IX_DISCM: [u8; 8usize] = [24, 30, 200, 40, 5, 28, 7, 119];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CreateIxArgs {
    pub name: String,
    pub symbol: String,
    pub uri: String,
    pub migration_kind: MigrationKind,
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
        let symbol: String = crate::borsh_de_or_default(&mut reader)?;
        let uri: String = crate::borsh_de_or_default(&mut reader)?;
        let migration_kind: MigrationKind = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(CreateIxArgs {
                name,
                symbol,
                uri,
                migration_kind,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CREATE_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.name, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.symbol, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.uri, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.migration_kind, &mut writer)?;
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
    create_ix_with_program_id(MASTERMIND_PROGRAM_ID, keys, args)
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
    create_invoke_with_program_id(MASTERMIND_PROGRAM_ID, accounts, args)
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
    create_invoke_signed_with_program_id(MASTERMIND_PROGRAM_ID, accounts, args, seeds)
}
pub fn create_verify_account_keys(
    accounts: CreateAccounts<'_, '_>,
    keys: CreateKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.state.key, keys.state),
        (*accounts.bonding_curve.key, keys.bonding_curve),
        (*accounts.mint.key, keys.mint),
        (*accounts.bonding_curve_sol_vault.key, keys.bonding_curve_sol_vault),
        (
            *accounts.bonding_curve_token_associated_account.key,
            keys.bonding_curve_token_associated_account,
        ),
        (*accounts.payer.key, keys.payer),
        (*accounts.token_program_2022.key, keys.token_program_2022),
        (*accounts.associated_token_program.key, keys.associated_token_program),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.rent.key, keys.rent),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
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
        accounts.bonding_curve,
        accounts.bonding_curve_sol_vault,
        accounts.payer,
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
    for should_be_signer in [accounts.payer] {
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
pub const INITIALIZE_IX_ACCOUNTS_LEN: usize = 4;
#[derive(Copy, Clone, Debug)]
pub struct InitializeAccounts<'me, 'info> {
    pub state: &'me AccountInfo<'info>,
    pub signer: &'me AccountInfo<'info>,
    pub rent: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct InitializeKeys {
    pub state: Pubkey,
    pub signer: Pubkey,
    pub rent: Pubkey,
    pub system_program: Pubkey,
}
impl From<InitializeAccounts<'_, '_>> for InitializeKeys {
    fn from(accounts: InitializeAccounts) -> Self {
        Self {
            state: *accounts.state.key,
            signer: *accounts.signer.key,
            rent: *accounts.rent.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<InitializeKeys> for [AccountMeta; INITIALIZE_IX_ACCOUNTS_LEN] {
    fn from(keys: InitializeKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.signer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.rent,
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
impl From<[Pubkey; INITIALIZE_IX_ACCOUNTS_LEN]> for InitializeKeys {
    fn from(pubkeys: [Pubkey; INITIALIZE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            state: pubkeys[0],
            signer: pubkeys[1],
            rent: pubkeys[2],
            system_program: pubkeys[3],
        }
    }
}
impl<'info> From<InitializeAccounts<'_, 'info>>
for [AccountInfo<'info>; INITIALIZE_IX_ACCOUNTS_LEN] {
    fn from(accounts: InitializeAccounts<'_, 'info>) -> Self {
        [
            accounts.state.clone(),
            accounts.signer.clone(),
            accounts.rent.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; INITIALIZE_IX_ACCOUNTS_LEN]>
for InitializeAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; INITIALIZE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            state: &arr[0],
            signer: &arr[1],
            rent: &arr[2],
            system_program: &arr[3],
        }
    }
}
pub const INITIALIZE_IX_DISCM: [u8; 8usize] = [175, 175, 109, 31, 13, 152, 155, 237];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct InitializeIxArgs {
    pub migrator: Pubkey,
    pub wrapper_mint: Pubkey,
    pub fee_receiver: Pubkey,
    pub fee_bps: u64,
    pub initial_virtual_token_reserve: u64,
    pub initial_virtual_sol_reserve: u64,
    pub coin_lot_size: u64,
    pub pc_lot_size: u64,
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
        let migrator: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let wrapper_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let fee_receiver: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let fee_bps: u64 = crate::borsh_de_or_default(&mut reader)?;
        let initial_virtual_token_reserve: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let initial_virtual_sol_reserve: u64 = crate::borsh_de_or_default(&mut reader)?;
        let coin_lot_size: u64 = crate::borsh_de_or_default(&mut reader)?;
        let pc_lot_size: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(InitializeIxArgs {
                migrator,
                wrapper_mint,
                fee_receiver,
                fee_bps,
                initial_virtual_token_reserve,
                initial_virtual_sol_reserve,
                coin_lot_size,
                pc_lot_size,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INITIALIZE_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.migrator, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.wrapper_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.fee_receiver, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.fee_bps, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.0.initial_virtual_token_reserve,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.0.initial_virtual_sol_reserve,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.0.coin_lot_size, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.pc_lot_size, &mut writer)?;
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
    initialize_ix_with_program_id(MASTERMIND_PROGRAM_ID, keys, args)
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
    initialize_invoke_with_program_id(MASTERMIND_PROGRAM_ID, accounts, args)
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
        MASTERMIND_PROGRAM_ID,
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
        (*accounts.state.key, keys.state),
        (*accounts.signer.key, keys.signer),
        (*accounts.rent.key, keys.rent),
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
    for should_be_writable in [accounts.state, accounts.signer] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn initialize_verify_signer_privileges<'me, 'info>(
    accounts: InitializeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.signer] {
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
pub const MIGRATE_TO_RADIUM_IX_ACCOUNTS_LEN: usize = 45;
#[derive(Copy, Clone, Debug)]
pub struct MigrateToRadiumAccounts<'me, 'info> {
    pub state: &'me AccountInfo<'info>,
    pub token_mint: &'me AccountInfo<'info>,
    pub wrapper_mint: &'me AccountInfo<'info>,
    pub bonding_curve: &'me AccountInfo<'info>,
    pub bonding_curve_wrapper_associated_account: &'me AccountInfo<'info>,
    pub bonding_curve_token_associated_account: &'me AccountInfo<'info>,
    pub wrapper_program: &'me AccountInfo<'info>,
    pub wrapper_state: &'me AccountInfo<'info>,
    pub wrapper_mint_authority: &'me AccountInfo<'info>,
    pub payer_wrapper_associated_account: &'me AccountInfo<'info>,
    pub payer_token_associated_account: &'me AccountInfo<'info>,
    pub cp_swap_program: &'me AccountInfo<'info>,
    pub creator: &'me AccountInfo<'info>,
    pub amm_config: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub pool_state: &'me AccountInfo<'info>,
    pub token_0_mint: &'me AccountInfo<'info>,
    pub token_1_mint: &'me AccountInfo<'info>,
    pub lp_mint: &'me AccountInfo<'info>,
    pub creator_token_0: &'me AccountInfo<'info>,
    pub creator_token_1: &'me AccountInfo<'info>,
    pub creator_lp_token: &'me AccountInfo<'info>,
    pub token_0_vault: &'me AccountInfo<'info>,
    pub token_1_vault: &'me AccountInfo<'info>,
    pub create_pool_fee: &'me AccountInfo<'info>,
    pub observation_state: &'me AccountInfo<'info>,
    pub locking_program: &'me AccountInfo<'info>,
    pub locking_authority: &'me AccountInfo<'info>,
    pub fee_nft_owner: &'me AccountInfo<'info>,
    pub fee_nft_mint: &'me AccountInfo<'info>,
    pub fee_nft_account: &'me AccountInfo<'info>,
    pub locked_liquidity: &'me AccountInfo<'info>,
    pub locked_lp_vault: &'me AccountInfo<'info>,
    pub metadata_account: &'me AccountInfo<'info>,
    pub payer: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub token_program_2022: &'me AccountInfo<'info>,
    pub token_0_program: &'me AccountInfo<'info>,
    pub token_1_program: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub rent: &'me AccountInfo<'info>,
    pub metadata_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MigrateToRadiumKeys {
    pub state: Pubkey,
    pub token_mint: Pubkey,
    pub wrapper_mint: Pubkey,
    pub bonding_curve: Pubkey,
    pub bonding_curve_wrapper_associated_account: Pubkey,
    pub bonding_curve_token_associated_account: Pubkey,
    pub wrapper_program: Pubkey,
    pub wrapper_state: Pubkey,
    pub wrapper_mint_authority: Pubkey,
    pub payer_wrapper_associated_account: Pubkey,
    pub payer_token_associated_account: Pubkey,
    pub cp_swap_program: Pubkey,
    pub creator: Pubkey,
    pub amm_config: Pubkey,
    pub authority: Pubkey,
    pub pool_state: Pubkey,
    pub token_0_mint: Pubkey,
    pub token_1_mint: Pubkey,
    pub lp_mint: Pubkey,
    pub creator_token_0: Pubkey,
    pub creator_token_1: Pubkey,
    pub creator_lp_token: Pubkey,
    pub token_0_vault: Pubkey,
    pub token_1_vault: Pubkey,
    pub create_pool_fee: Pubkey,
    pub observation_state: Pubkey,
    pub locking_program: Pubkey,
    pub locking_authority: Pubkey,
    pub fee_nft_owner: Pubkey,
    pub fee_nft_mint: Pubkey,
    pub fee_nft_account: Pubkey,
    pub locked_liquidity: Pubkey,
    pub locked_lp_vault: Pubkey,
    pub metadata_account: Pubkey,
    pub payer: Pubkey,
    pub token_program: Pubkey,
    pub token_program_2022: Pubkey,
    pub token_0_program: Pubkey,
    pub token_1_program: Pubkey,
    pub associated_token_program: Pubkey,
    pub system_program: Pubkey,
    pub rent: Pubkey,
    pub metadata_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<MigrateToRadiumAccounts<'_, '_>> for MigrateToRadiumKeys {
    fn from(accounts: MigrateToRadiumAccounts) -> Self {
        Self {
            state: *accounts.state.key,
            token_mint: *accounts.token_mint.key,
            wrapper_mint: *accounts.wrapper_mint.key,
            bonding_curve: *accounts.bonding_curve.key,
            bonding_curve_wrapper_associated_account: *accounts
                .bonding_curve_wrapper_associated_account
                .key,
            bonding_curve_token_associated_account: *accounts
                .bonding_curve_token_associated_account
                .key,
            wrapper_program: *accounts.wrapper_program.key,
            wrapper_state: *accounts.wrapper_state.key,
            wrapper_mint_authority: *accounts.wrapper_mint_authority.key,
            payer_wrapper_associated_account: *accounts
                .payer_wrapper_associated_account
                .key,
            payer_token_associated_account: *accounts.payer_token_associated_account.key,
            cp_swap_program: *accounts.cp_swap_program.key,
            creator: *accounts.creator.key,
            amm_config: *accounts.amm_config.key,
            authority: *accounts.authority.key,
            pool_state: *accounts.pool_state.key,
            token_0_mint: *accounts.token_0_mint.key,
            token_1_mint: *accounts.token_1_mint.key,
            lp_mint: *accounts.lp_mint.key,
            creator_token_0: *accounts.creator_token_0.key,
            creator_token_1: *accounts.creator_token_1.key,
            creator_lp_token: *accounts.creator_lp_token.key,
            token_0_vault: *accounts.token_0_vault.key,
            token_1_vault: *accounts.token_1_vault.key,
            create_pool_fee: *accounts.create_pool_fee.key,
            observation_state: *accounts.observation_state.key,
            locking_program: *accounts.locking_program.key,
            locking_authority: *accounts.locking_authority.key,
            fee_nft_owner: *accounts.fee_nft_owner.key,
            fee_nft_mint: *accounts.fee_nft_mint.key,
            fee_nft_account: *accounts.fee_nft_account.key,
            locked_liquidity: *accounts.locked_liquidity.key,
            locked_lp_vault: *accounts.locked_lp_vault.key,
            metadata_account: *accounts.metadata_account.key,
            payer: *accounts.payer.key,
            token_program: *accounts.token_program.key,
            token_program_2022: *accounts.token_program_2022.key,
            token_0_program: *accounts.token_0_program.key,
            token_1_program: *accounts.token_1_program.key,
            associated_token_program: *accounts.associated_token_program.key,
            system_program: *accounts.system_program.key,
            rent: *accounts.rent.key,
            metadata_program: *accounts.metadata_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<MigrateToRadiumKeys> for [AccountMeta; MIGRATE_TO_RADIUM_IX_ACCOUNTS_LEN] {
    fn from(keys: MigrateToRadiumKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.wrapper_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.bonding_curve,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.bonding_curve_wrapper_associated_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.bonding_curve_token_associated_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.wrapper_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.wrapper_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.wrapper_mint_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.payer_wrapper_associated_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.payer_token_associated_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.cp_swap_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.creator,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.amm_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_0_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_1_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.lp_mint,
                is_signer: false,
                is_writable: true,
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
                pubkey: keys.creator_lp_token,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_0_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_1_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.create_pool_fee,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.observation_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.locking_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.locking_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.fee_nft_owner,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.fee_nft_mint,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.fee_nft_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.locked_liquidity,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.locked_lp_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.metadata_account,
                is_signer: false,
                is_writable: true,
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
                pubkey: keys.token_program_2022,
                is_signer: false,
                is_writable: false,
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
                pubkey: keys.rent,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.metadata_program,
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
impl From<[Pubkey; MIGRATE_TO_RADIUM_IX_ACCOUNTS_LEN]> for MigrateToRadiumKeys {
    fn from(pubkeys: [Pubkey; MIGRATE_TO_RADIUM_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            state: pubkeys[0],
            token_mint: pubkeys[1],
            wrapper_mint: pubkeys[2],
            bonding_curve: pubkeys[3],
            bonding_curve_wrapper_associated_account: pubkeys[4],
            bonding_curve_token_associated_account: pubkeys[5],
            wrapper_program: pubkeys[6],
            wrapper_state: pubkeys[7],
            wrapper_mint_authority: pubkeys[8],
            payer_wrapper_associated_account: pubkeys[9],
            payer_token_associated_account: pubkeys[10],
            cp_swap_program: pubkeys[11],
            creator: pubkeys[12],
            amm_config: pubkeys[13],
            authority: pubkeys[14],
            pool_state: pubkeys[15],
            token_0_mint: pubkeys[16],
            token_1_mint: pubkeys[17],
            lp_mint: pubkeys[18],
            creator_token_0: pubkeys[19],
            creator_token_1: pubkeys[20],
            creator_lp_token: pubkeys[21],
            token_0_vault: pubkeys[22],
            token_1_vault: pubkeys[23],
            create_pool_fee: pubkeys[24],
            observation_state: pubkeys[25],
            locking_program: pubkeys[26],
            locking_authority: pubkeys[27],
            fee_nft_owner: pubkeys[28],
            fee_nft_mint: pubkeys[29],
            fee_nft_account: pubkeys[30],
            locked_liquidity: pubkeys[31],
            locked_lp_vault: pubkeys[32],
            metadata_account: pubkeys[33],
            payer: pubkeys[34],
            token_program: pubkeys[35],
            token_program_2022: pubkeys[36],
            token_0_program: pubkeys[37],
            token_1_program: pubkeys[38],
            associated_token_program: pubkeys[39],
            system_program: pubkeys[40],
            rent: pubkeys[41],
            metadata_program: pubkeys[42],
            event_authority: pubkeys[43],
            program: pubkeys[44],
        }
    }
}
impl<'info> From<MigrateToRadiumAccounts<'_, 'info>>
for [AccountInfo<'info>; MIGRATE_TO_RADIUM_IX_ACCOUNTS_LEN] {
    fn from(accounts: MigrateToRadiumAccounts<'_, 'info>) -> Self {
        [
            accounts.state.clone(),
            accounts.token_mint.clone(),
            accounts.wrapper_mint.clone(),
            accounts.bonding_curve.clone(),
            accounts.bonding_curve_wrapper_associated_account.clone(),
            accounts.bonding_curve_token_associated_account.clone(),
            accounts.wrapper_program.clone(),
            accounts.wrapper_state.clone(),
            accounts.wrapper_mint_authority.clone(),
            accounts.payer_wrapper_associated_account.clone(),
            accounts.payer_token_associated_account.clone(),
            accounts.cp_swap_program.clone(),
            accounts.creator.clone(),
            accounts.amm_config.clone(),
            accounts.authority.clone(),
            accounts.pool_state.clone(),
            accounts.token_0_mint.clone(),
            accounts.token_1_mint.clone(),
            accounts.lp_mint.clone(),
            accounts.creator_token_0.clone(),
            accounts.creator_token_1.clone(),
            accounts.creator_lp_token.clone(),
            accounts.token_0_vault.clone(),
            accounts.token_1_vault.clone(),
            accounts.create_pool_fee.clone(),
            accounts.observation_state.clone(),
            accounts.locking_program.clone(),
            accounts.locking_authority.clone(),
            accounts.fee_nft_owner.clone(),
            accounts.fee_nft_mint.clone(),
            accounts.fee_nft_account.clone(),
            accounts.locked_liquidity.clone(),
            accounts.locked_lp_vault.clone(),
            accounts.metadata_account.clone(),
            accounts.payer.clone(),
            accounts.token_program.clone(),
            accounts.token_program_2022.clone(),
            accounts.token_0_program.clone(),
            accounts.token_1_program.clone(),
            accounts.associated_token_program.clone(),
            accounts.system_program.clone(),
            accounts.rent.clone(),
            accounts.metadata_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; MIGRATE_TO_RADIUM_IX_ACCOUNTS_LEN]>
for MigrateToRadiumAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; MIGRATE_TO_RADIUM_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            state: &arr[0],
            token_mint: &arr[1],
            wrapper_mint: &arr[2],
            bonding_curve: &arr[3],
            bonding_curve_wrapper_associated_account: &arr[4],
            bonding_curve_token_associated_account: &arr[5],
            wrapper_program: &arr[6],
            wrapper_state: &arr[7],
            wrapper_mint_authority: &arr[8],
            payer_wrapper_associated_account: &arr[9],
            payer_token_associated_account: &arr[10],
            cp_swap_program: &arr[11],
            creator: &arr[12],
            amm_config: &arr[13],
            authority: &arr[14],
            pool_state: &arr[15],
            token_0_mint: &arr[16],
            token_1_mint: &arr[17],
            lp_mint: &arr[18],
            creator_token_0: &arr[19],
            creator_token_1: &arr[20],
            creator_lp_token: &arr[21],
            token_0_vault: &arr[22],
            token_1_vault: &arr[23],
            create_pool_fee: &arr[24],
            observation_state: &arr[25],
            locking_program: &arr[26],
            locking_authority: &arr[27],
            fee_nft_owner: &arr[28],
            fee_nft_mint: &arr[29],
            fee_nft_account: &arr[30],
            locked_liquidity: &arr[31],
            locked_lp_vault: &arr[32],
            metadata_account: &arr[33],
            payer: &arr[34],
            token_program: &arr[35],
            token_program_2022: &arr[36],
            token_0_program: &arr[37],
            token_1_program: &arr[38],
            associated_token_program: &arr[39],
            system_program: &arr[40],
            rent: &arr[41],
            metadata_program: &arr[42],
            event_authority: &arr[43],
            program: &arr[44],
        }
    }
}
pub const MIGRATE_TO_RADIUM_IX_DISCM: [u8; 8usize] = [
    96, 230, 91, 140, 139, 40, 235, 142,
];
#[derive(Clone, Debug, PartialEq)]
pub struct MigrateToRadiumIxData;
impl MigrateToRadiumIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != MIGRATE_TO_RADIUM_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&MIGRATE_TO_RADIUM_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn migrate_to_radium_ix_with_program_id(
    program_id: Pubkey,
    keys: MigrateToRadiumKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; MIGRATE_TO_RADIUM_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: MigrateToRadiumIxData.try_to_vec()?,
    })
}
pub fn migrate_to_radium_ix(keys: MigrateToRadiumKeys) -> std::io::Result<Instruction> {
    migrate_to_radium_ix_with_program_id(MASTERMIND_PROGRAM_ID, keys)
}
pub fn migrate_to_radium_invoke_with_program_id(
    program_id: Pubkey,
    accounts: MigrateToRadiumAccounts<'_, '_>,
) -> ProgramResult {
    let keys: MigrateToRadiumKeys = accounts.into();
    let ix = migrate_to_radium_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn migrate_to_radium_invoke(
    accounts: MigrateToRadiumAccounts<'_, '_>,
) -> ProgramResult {
    migrate_to_radium_invoke_with_program_id(MASTERMIND_PROGRAM_ID, accounts)
}
pub fn migrate_to_radium_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: MigrateToRadiumAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: MigrateToRadiumKeys = accounts.into();
    let ix = migrate_to_radium_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn migrate_to_radium_invoke_signed(
    accounts: MigrateToRadiumAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    migrate_to_radium_invoke_signed_with_program_id(
        MASTERMIND_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn migrate_to_radium_verify_account_keys(
    accounts: MigrateToRadiumAccounts<'_, '_>,
    keys: MigrateToRadiumKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.state.key, keys.state),
        (*accounts.token_mint.key, keys.token_mint),
        (*accounts.wrapper_mint.key, keys.wrapper_mint),
        (*accounts.bonding_curve.key, keys.bonding_curve),
        (
            *accounts.bonding_curve_wrapper_associated_account.key,
            keys.bonding_curve_wrapper_associated_account,
        ),
        (
            *accounts.bonding_curve_token_associated_account.key,
            keys.bonding_curve_token_associated_account,
        ),
        (*accounts.wrapper_program.key, keys.wrapper_program),
        (*accounts.wrapper_state.key, keys.wrapper_state),
        (*accounts.wrapper_mint_authority.key, keys.wrapper_mint_authority),
        (
            *accounts.payer_wrapper_associated_account.key,
            keys.payer_wrapper_associated_account,
        ),
        (
            *accounts.payer_token_associated_account.key,
            keys.payer_token_associated_account,
        ),
        (*accounts.cp_swap_program.key, keys.cp_swap_program),
        (*accounts.creator.key, keys.creator),
        (*accounts.amm_config.key, keys.amm_config),
        (*accounts.authority.key, keys.authority),
        (*accounts.pool_state.key, keys.pool_state),
        (*accounts.token_0_mint.key, keys.token_0_mint),
        (*accounts.token_1_mint.key, keys.token_1_mint),
        (*accounts.lp_mint.key, keys.lp_mint),
        (*accounts.creator_token_0.key, keys.creator_token_0),
        (*accounts.creator_token_1.key, keys.creator_token_1),
        (*accounts.creator_lp_token.key, keys.creator_lp_token),
        (*accounts.token_0_vault.key, keys.token_0_vault),
        (*accounts.token_1_vault.key, keys.token_1_vault),
        (*accounts.create_pool_fee.key, keys.create_pool_fee),
        (*accounts.observation_state.key, keys.observation_state),
        (*accounts.locking_program.key, keys.locking_program),
        (*accounts.locking_authority.key, keys.locking_authority),
        (*accounts.fee_nft_owner.key, keys.fee_nft_owner),
        (*accounts.fee_nft_mint.key, keys.fee_nft_mint),
        (*accounts.fee_nft_account.key, keys.fee_nft_account),
        (*accounts.locked_liquidity.key, keys.locked_liquidity),
        (*accounts.locked_lp_vault.key, keys.locked_lp_vault),
        (*accounts.metadata_account.key, keys.metadata_account),
        (*accounts.payer.key, keys.payer),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.token_program_2022.key, keys.token_program_2022),
        (*accounts.token_0_program.key, keys.token_0_program),
        (*accounts.token_1_program.key, keys.token_1_program),
        (*accounts.associated_token_program.key, keys.associated_token_program),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.rent.key, keys.rent),
        (*accounts.metadata_program.key, keys.metadata_program),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn migrate_to_radium_verify_writable_privileges<'me, 'info>(
    accounts: MigrateToRadiumAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.state,
        accounts.token_mint,
        accounts.wrapper_mint,
        accounts.bonding_curve,
        accounts.bonding_curve_wrapper_associated_account,
        accounts.bonding_curve_token_associated_account,
        accounts.wrapper_state,
        accounts.payer_wrapper_associated_account,
        accounts.payer_token_associated_account,
        accounts.creator,
        accounts.pool_state,
        accounts.lp_mint,
        accounts.creator_token_0,
        accounts.creator_token_1,
        accounts.creator_lp_token,
        accounts.token_0_vault,
        accounts.token_1_vault,
        accounts.create_pool_fee,
        accounts.observation_state,
        accounts.fee_nft_mint,
        accounts.fee_nft_account,
        accounts.locked_liquidity,
        accounts.locked_lp_vault,
        accounts.metadata_account,
        accounts.payer,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn migrate_to_radium_verify_signer_privileges<'me, 'info>(
    accounts: MigrateToRadiumAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.creator, accounts.fee_nft_mint, accounts.payer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn migrate_to_radium_verify_account_privileges<'me, 'info>(
    accounts: MigrateToRadiumAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    migrate_to_radium_verify_writable_privileges(accounts)?;
    migrate_to_radium_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SELL_EXACT_IN_IX_ACCOUNTS_LEN: usize = 15;
#[derive(Copy, Clone, Debug)]
pub struct SellExactInAccounts<'me, 'info> {
    pub state: &'me AccountInfo<'info>,
    pub mint: &'me AccountInfo<'info>,
    pub bonding_curve: &'me AccountInfo<'info>,
    pub bonding_curve_sol_associated_account: &'me AccountInfo<'info>,
    pub bonding_curve_token_associated_account: &'me AccountInfo<'info>,
    pub payer_associated_account: &'me AccountInfo<'info>,
    pub payer: &'me AccountInfo<'info>,
    pub receiver: &'me AccountInfo<'info>,
    pub fee_receiver: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub rent: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SellExactInKeys {
    pub state: Pubkey,
    pub mint: Pubkey,
    pub bonding_curve: Pubkey,
    pub bonding_curve_sol_associated_account: Pubkey,
    pub bonding_curve_token_associated_account: Pubkey,
    pub payer_associated_account: Pubkey,
    pub payer: Pubkey,
    pub receiver: Pubkey,
    pub fee_receiver: Pubkey,
    pub token_program: Pubkey,
    pub associated_token_program: Pubkey,
    pub system_program: Pubkey,
    pub rent: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<SellExactInAccounts<'_, '_>> for SellExactInKeys {
    fn from(accounts: SellExactInAccounts) -> Self {
        Self {
            state: *accounts.state.key,
            mint: *accounts.mint.key,
            bonding_curve: *accounts.bonding_curve.key,
            bonding_curve_sol_associated_account: *accounts
                .bonding_curve_sol_associated_account
                .key,
            bonding_curve_token_associated_account: *accounts
                .bonding_curve_token_associated_account
                .key,
            payer_associated_account: *accounts.payer_associated_account.key,
            payer: *accounts.payer.key,
            receiver: *accounts.receiver.key,
            fee_receiver: *accounts.fee_receiver.key,
            token_program: *accounts.token_program.key,
            associated_token_program: *accounts.associated_token_program.key,
            system_program: *accounts.system_program.key,
            rent: *accounts.rent.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<SellExactInKeys> for [AccountMeta; SELL_EXACT_IN_IX_ACCOUNTS_LEN] {
    fn from(keys: SellExactInKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.state,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.bonding_curve,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.bonding_curve_sol_associated_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.bonding_curve_token_associated_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.payer_associated_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.payer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.receiver,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.fee_receiver,
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
            AccountMeta {
                pubkey: keys.rent,
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
impl From<[Pubkey; SELL_EXACT_IN_IX_ACCOUNTS_LEN]> for SellExactInKeys {
    fn from(pubkeys: [Pubkey; SELL_EXACT_IN_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            state: pubkeys[0],
            mint: pubkeys[1],
            bonding_curve: pubkeys[2],
            bonding_curve_sol_associated_account: pubkeys[3],
            bonding_curve_token_associated_account: pubkeys[4],
            payer_associated_account: pubkeys[5],
            payer: pubkeys[6],
            receiver: pubkeys[7],
            fee_receiver: pubkeys[8],
            token_program: pubkeys[9],
            associated_token_program: pubkeys[10],
            system_program: pubkeys[11],
            rent: pubkeys[12],
            event_authority: pubkeys[13],
            program: pubkeys[14],
        }
    }
}
impl<'info> From<SellExactInAccounts<'_, 'info>>
for [AccountInfo<'info>; SELL_EXACT_IN_IX_ACCOUNTS_LEN] {
    fn from(accounts: SellExactInAccounts<'_, 'info>) -> Self {
        [
            accounts.state.clone(),
            accounts.mint.clone(),
            accounts.bonding_curve.clone(),
            accounts.bonding_curve_sol_associated_account.clone(),
            accounts.bonding_curve_token_associated_account.clone(),
            accounts.payer_associated_account.clone(),
            accounts.payer.clone(),
            accounts.receiver.clone(),
            accounts.fee_receiver.clone(),
            accounts.token_program.clone(),
            accounts.associated_token_program.clone(),
            accounts.system_program.clone(),
            accounts.rent.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SELL_EXACT_IN_IX_ACCOUNTS_LEN]>
for SellExactInAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; SELL_EXACT_IN_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            state: &arr[0],
            mint: &arr[1],
            bonding_curve: &arr[2],
            bonding_curve_sol_associated_account: &arr[3],
            bonding_curve_token_associated_account: &arr[4],
            payer_associated_account: &arr[5],
            payer: &arr[6],
            receiver: &arr[7],
            fee_receiver: &arr[8],
            token_program: &arr[9],
            associated_token_program: &arr[10],
            system_program: &arr[11],
            rent: &arr[12],
            event_authority: &arr[13],
            program: &arr[14],
        }
    }
}
pub const SELL_EXACT_IN_IX_DISCM: [u8; 8usize] = [149, 39, 222, 155, 211, 124, 152, 26];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SellExactInIxArgs {
    pub bumps: InstructionBumps,
    pub tokens_input: u64,
    pub min_sol_amount: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct SellExactInIxData(pub SellExactInIxArgs);
impl From<SellExactInIxArgs> for SellExactInIxData {
    fn from(args: SellExactInIxArgs) -> Self {
        Self(args)
    }
}
impl SellExactInIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SELL_EXACT_IN_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let bumps = if reader.is_empty() {
            Default::default()
        } else {
            <InstructionBumps>::deserialize(&mut reader)?
        };
        let tokens_input: u64 = crate::borsh_de_or_default(&mut reader)?;
        let min_sol_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(SellExactInIxArgs {
                bumps,
                tokens_input,
                min_sol_amount,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SELL_EXACT_IN_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.bumps, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.tokens_input, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.min_sol_amount, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn sell_exact_in_ix_with_program_id(
    program_id: Pubkey,
    keys: SellExactInKeys,
    args: SellExactInIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SELL_EXACT_IN_IX_ACCOUNTS_LEN] = keys.into();
    let data: SellExactInIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn sell_exact_in_ix(
    keys: SellExactInKeys,
    args: SellExactInIxArgs,
) -> std::io::Result<Instruction> {
    sell_exact_in_ix_with_program_id(MASTERMIND_PROGRAM_ID, keys, args)
}
pub fn sell_exact_in_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SellExactInAccounts<'_, '_>,
    args: SellExactInIxArgs,
) -> ProgramResult {
    let keys: SellExactInKeys = accounts.into();
    let ix = sell_exact_in_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn sell_exact_in_invoke(
    accounts: SellExactInAccounts<'_, '_>,
    args: SellExactInIxArgs,
) -> ProgramResult {
    sell_exact_in_invoke_with_program_id(MASTERMIND_PROGRAM_ID, accounts, args)
}
pub fn sell_exact_in_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SellExactInAccounts<'_, '_>,
    args: SellExactInIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SellExactInKeys = accounts.into();
    let ix = sell_exact_in_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn sell_exact_in_invoke_signed(
    accounts: SellExactInAccounts<'_, '_>,
    args: SellExactInIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    sell_exact_in_invoke_signed_with_program_id(
        MASTERMIND_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn sell_exact_in_verify_account_keys(
    accounts: SellExactInAccounts<'_, '_>,
    keys: SellExactInKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.state.key, keys.state),
        (*accounts.mint.key, keys.mint),
        (*accounts.bonding_curve.key, keys.bonding_curve),
        (
            *accounts.bonding_curve_sol_associated_account.key,
            keys.bonding_curve_sol_associated_account,
        ),
        (
            *accounts.bonding_curve_token_associated_account.key,
            keys.bonding_curve_token_associated_account,
        ),
        (*accounts.payer_associated_account.key, keys.payer_associated_account),
        (*accounts.payer.key, keys.payer),
        (*accounts.receiver.key, keys.receiver),
        (*accounts.fee_receiver.key, keys.fee_receiver),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.associated_token_program.key, keys.associated_token_program),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.rent.key, keys.rent),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn sell_exact_in_verify_writable_privileges<'me, 'info>(
    accounts: SellExactInAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.bonding_curve,
        accounts.bonding_curve_sol_associated_account,
        accounts.bonding_curve_token_associated_account,
        accounts.payer_associated_account,
        accounts.payer,
        accounts.receiver,
        accounts.fee_receiver,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn sell_exact_in_verify_signer_privileges<'me, 'info>(
    accounts: SellExactInAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.payer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn sell_exact_in_verify_account_privileges<'me, 'info>(
    accounts: SellExactInAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    sell_exact_in_verify_writable_privileges(accounts)?;
    sell_exact_in_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SELL_EXACT_OUT_IX_ACCOUNTS_LEN: usize = 15;
#[derive(Copy, Clone, Debug)]
pub struct SellExactOutAccounts<'me, 'info> {
    pub state: &'me AccountInfo<'info>,
    pub mint: &'me AccountInfo<'info>,
    pub bonding_curve: &'me AccountInfo<'info>,
    pub bonding_curve_sol_associated_account: &'me AccountInfo<'info>,
    pub bonding_curve_token_associated_account: &'me AccountInfo<'info>,
    pub payer_associated_account: &'me AccountInfo<'info>,
    pub payer: &'me AccountInfo<'info>,
    pub receiver: &'me AccountInfo<'info>,
    pub fee_receiver: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub rent: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SellExactOutKeys {
    pub state: Pubkey,
    pub mint: Pubkey,
    pub bonding_curve: Pubkey,
    pub bonding_curve_sol_associated_account: Pubkey,
    pub bonding_curve_token_associated_account: Pubkey,
    pub payer_associated_account: Pubkey,
    pub payer: Pubkey,
    pub receiver: Pubkey,
    pub fee_receiver: Pubkey,
    pub token_program: Pubkey,
    pub associated_token_program: Pubkey,
    pub system_program: Pubkey,
    pub rent: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<SellExactOutAccounts<'_, '_>> for SellExactOutKeys {
    fn from(accounts: SellExactOutAccounts) -> Self {
        Self {
            state: *accounts.state.key,
            mint: *accounts.mint.key,
            bonding_curve: *accounts.bonding_curve.key,
            bonding_curve_sol_associated_account: *accounts
                .bonding_curve_sol_associated_account
                .key,
            bonding_curve_token_associated_account: *accounts
                .bonding_curve_token_associated_account
                .key,
            payer_associated_account: *accounts.payer_associated_account.key,
            payer: *accounts.payer.key,
            receiver: *accounts.receiver.key,
            fee_receiver: *accounts.fee_receiver.key,
            token_program: *accounts.token_program.key,
            associated_token_program: *accounts.associated_token_program.key,
            system_program: *accounts.system_program.key,
            rent: *accounts.rent.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<SellExactOutKeys> for [AccountMeta; SELL_EXACT_OUT_IX_ACCOUNTS_LEN] {
    fn from(keys: SellExactOutKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.state,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.bonding_curve,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.bonding_curve_sol_associated_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.bonding_curve_token_associated_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.payer_associated_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.payer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.receiver,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.fee_receiver,
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
            AccountMeta {
                pubkey: keys.rent,
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
impl From<[Pubkey; SELL_EXACT_OUT_IX_ACCOUNTS_LEN]> for SellExactOutKeys {
    fn from(pubkeys: [Pubkey; SELL_EXACT_OUT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            state: pubkeys[0],
            mint: pubkeys[1],
            bonding_curve: pubkeys[2],
            bonding_curve_sol_associated_account: pubkeys[3],
            bonding_curve_token_associated_account: pubkeys[4],
            payer_associated_account: pubkeys[5],
            payer: pubkeys[6],
            receiver: pubkeys[7],
            fee_receiver: pubkeys[8],
            token_program: pubkeys[9],
            associated_token_program: pubkeys[10],
            system_program: pubkeys[11],
            rent: pubkeys[12],
            event_authority: pubkeys[13],
            program: pubkeys[14],
        }
    }
}
impl<'info> From<SellExactOutAccounts<'_, 'info>>
for [AccountInfo<'info>; SELL_EXACT_OUT_IX_ACCOUNTS_LEN] {
    fn from(accounts: SellExactOutAccounts<'_, 'info>) -> Self {
        [
            accounts.state.clone(),
            accounts.mint.clone(),
            accounts.bonding_curve.clone(),
            accounts.bonding_curve_sol_associated_account.clone(),
            accounts.bonding_curve_token_associated_account.clone(),
            accounts.payer_associated_account.clone(),
            accounts.payer.clone(),
            accounts.receiver.clone(),
            accounts.fee_receiver.clone(),
            accounts.token_program.clone(),
            accounts.associated_token_program.clone(),
            accounts.system_program.clone(),
            accounts.rent.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SELL_EXACT_OUT_IX_ACCOUNTS_LEN]>
for SellExactOutAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; SELL_EXACT_OUT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            state: &arr[0],
            mint: &arr[1],
            bonding_curve: &arr[2],
            bonding_curve_sol_associated_account: &arr[3],
            bonding_curve_token_associated_account: &arr[4],
            payer_associated_account: &arr[5],
            payer: &arr[6],
            receiver: &arr[7],
            fee_receiver: &arr[8],
            token_program: &arr[9],
            associated_token_program: &arr[10],
            system_program: &arr[11],
            rent: &arr[12],
            event_authority: &arr[13],
            program: &arr[14],
        }
    }
}
pub const SELL_EXACT_OUT_IX_DISCM: [u8; 8usize] = [95, 200, 71, 34, 8, 9, 11, 166];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SellExactOutIxArgs {
    pub bumps: InstructionBumps,
    pub max_tokens_input: u64,
    pub sol_amount_output: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct SellExactOutIxData(pub SellExactOutIxArgs);
impl From<SellExactOutIxArgs> for SellExactOutIxData {
    fn from(args: SellExactOutIxArgs) -> Self {
        Self(args)
    }
}
impl SellExactOutIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SELL_EXACT_OUT_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let bumps = if reader.is_empty() {
            Default::default()
        } else {
            <InstructionBumps>::deserialize(&mut reader)?
        };
        let max_tokens_input: u64 = crate::borsh_de_or_default(&mut reader)?;
        let sol_amount_output: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(SellExactOutIxArgs {
                bumps,
                max_tokens_input,
                sol_amount_output,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SELL_EXACT_OUT_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.bumps, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.max_tokens_input, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.sol_amount_output, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn sell_exact_out_ix_with_program_id(
    program_id: Pubkey,
    keys: SellExactOutKeys,
    args: SellExactOutIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SELL_EXACT_OUT_IX_ACCOUNTS_LEN] = keys.into();
    let data: SellExactOutIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn sell_exact_out_ix(
    keys: SellExactOutKeys,
    args: SellExactOutIxArgs,
) -> std::io::Result<Instruction> {
    sell_exact_out_ix_with_program_id(MASTERMIND_PROGRAM_ID, keys, args)
}
pub fn sell_exact_out_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SellExactOutAccounts<'_, '_>,
    args: SellExactOutIxArgs,
) -> ProgramResult {
    let keys: SellExactOutKeys = accounts.into();
    let ix = sell_exact_out_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn sell_exact_out_invoke(
    accounts: SellExactOutAccounts<'_, '_>,
    args: SellExactOutIxArgs,
) -> ProgramResult {
    sell_exact_out_invoke_with_program_id(MASTERMIND_PROGRAM_ID, accounts, args)
}
pub fn sell_exact_out_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SellExactOutAccounts<'_, '_>,
    args: SellExactOutIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SellExactOutKeys = accounts.into();
    let ix = sell_exact_out_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn sell_exact_out_invoke_signed(
    accounts: SellExactOutAccounts<'_, '_>,
    args: SellExactOutIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    sell_exact_out_invoke_signed_with_program_id(
        MASTERMIND_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn sell_exact_out_verify_account_keys(
    accounts: SellExactOutAccounts<'_, '_>,
    keys: SellExactOutKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.state.key, keys.state),
        (*accounts.mint.key, keys.mint),
        (*accounts.bonding_curve.key, keys.bonding_curve),
        (
            *accounts.bonding_curve_sol_associated_account.key,
            keys.bonding_curve_sol_associated_account,
        ),
        (
            *accounts.bonding_curve_token_associated_account.key,
            keys.bonding_curve_token_associated_account,
        ),
        (*accounts.payer_associated_account.key, keys.payer_associated_account),
        (*accounts.payer.key, keys.payer),
        (*accounts.receiver.key, keys.receiver),
        (*accounts.fee_receiver.key, keys.fee_receiver),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.associated_token_program.key, keys.associated_token_program),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.rent.key, keys.rent),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn sell_exact_out_verify_writable_privileges<'me, 'info>(
    accounts: SellExactOutAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.bonding_curve,
        accounts.bonding_curve_sol_associated_account,
        accounts.bonding_curve_token_associated_account,
        accounts.payer_associated_account,
        accounts.payer,
        accounts.receiver,
        accounts.fee_receiver,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn sell_exact_out_verify_signer_privileges<'me, 'info>(
    accounts: SellExactOutAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.payer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn sell_exact_out_verify_account_privileges<'me, 'info>(
    accounts: SellExactOutAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    sell_exact_out_verify_writable_privileges(accounts)?;
    sell_exact_out_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SET_CURVE_PARAMS_IX_ACCOUNTS_LEN: usize = 2;
#[derive(Copy, Clone, Debug)]
pub struct SetCurveParamsAccounts<'me, 'info> {
    pub state: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SetCurveParamsKeys {
    pub state: Pubkey,
    pub authority: Pubkey,
}
impl From<SetCurveParamsAccounts<'_, '_>> for SetCurveParamsKeys {
    fn from(accounts: SetCurveParamsAccounts) -> Self {
        Self {
            state: *accounts.state.key,
            authority: *accounts.authority.key,
        }
    }
}
impl From<SetCurveParamsKeys> for [AccountMeta; SET_CURVE_PARAMS_IX_ACCOUNTS_LEN] {
    fn from(keys: SetCurveParamsKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.authority,
                is_signer: true,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; SET_CURVE_PARAMS_IX_ACCOUNTS_LEN]> for SetCurveParamsKeys {
    fn from(pubkeys: [Pubkey; SET_CURVE_PARAMS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            state: pubkeys[0],
            authority: pubkeys[1],
        }
    }
}
impl<'info> From<SetCurveParamsAccounts<'_, 'info>>
for [AccountInfo<'info>; SET_CURVE_PARAMS_IX_ACCOUNTS_LEN] {
    fn from(accounts: SetCurveParamsAccounts<'_, 'info>) -> Self {
        [accounts.state.clone(), accounts.authority.clone()]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SET_CURVE_PARAMS_IX_ACCOUNTS_LEN]>
for SetCurveParamsAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; SET_CURVE_PARAMS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            state: &arr[0],
            authority: &arr[1],
        }
    }
}
pub const SET_CURVE_PARAMS_IX_DISCM: [u8; 8usize] = [133, 183, 61, 96, 198, 233, 62, 13];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SetCurveParamsIxArgs {
    pub initial_virtual_token_reserve: u64,
    pub initial_virtual_sol_reserve: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct SetCurveParamsIxData(pub SetCurveParamsIxArgs);
impl From<SetCurveParamsIxArgs> for SetCurveParamsIxData {
    fn from(args: SetCurveParamsIxArgs) -> Self {
        Self(args)
    }
}
impl SetCurveParamsIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SET_CURVE_PARAMS_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let initial_virtual_token_reserve: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let initial_virtual_sol_reserve: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(SetCurveParamsIxArgs {
                initial_virtual_token_reserve,
                initial_virtual_sol_reserve,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SET_CURVE_PARAMS_IX_DISCM)?;
        borsh::BorshSerialize::serialize(
            &self.0.initial_virtual_token_reserve,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.0.initial_virtual_sol_reserve,
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
pub fn set_curve_params_ix_with_program_id(
    program_id: Pubkey,
    keys: SetCurveParamsKeys,
    args: SetCurveParamsIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SET_CURVE_PARAMS_IX_ACCOUNTS_LEN] = keys.into();
    let data: SetCurveParamsIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn set_curve_params_ix(
    keys: SetCurveParamsKeys,
    args: SetCurveParamsIxArgs,
) -> std::io::Result<Instruction> {
    set_curve_params_ix_with_program_id(MASTERMIND_PROGRAM_ID, keys, args)
}
pub fn set_curve_params_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SetCurveParamsAccounts<'_, '_>,
    args: SetCurveParamsIxArgs,
) -> ProgramResult {
    let keys: SetCurveParamsKeys = accounts.into();
    let ix = set_curve_params_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn set_curve_params_invoke(
    accounts: SetCurveParamsAccounts<'_, '_>,
    args: SetCurveParamsIxArgs,
) -> ProgramResult {
    set_curve_params_invoke_with_program_id(MASTERMIND_PROGRAM_ID, accounts, args)
}
pub fn set_curve_params_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SetCurveParamsAccounts<'_, '_>,
    args: SetCurveParamsIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SetCurveParamsKeys = accounts.into();
    let ix = set_curve_params_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn set_curve_params_invoke_signed(
    accounts: SetCurveParamsAccounts<'_, '_>,
    args: SetCurveParamsIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    set_curve_params_invoke_signed_with_program_id(
        MASTERMIND_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn set_curve_params_verify_account_keys(
    accounts: SetCurveParamsAccounts<'_, '_>,
    keys: SetCurveParamsKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.state.key, keys.state),
        (*accounts.authority.key, keys.authority),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn set_curve_params_verify_writable_privileges<'me, 'info>(
    accounts: SetCurveParamsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.state, accounts.authority] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn set_curve_params_verify_signer_privileges<'me, 'info>(
    accounts: SetCurveParamsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn set_curve_params_verify_account_privileges<'me, 'info>(
    accounts: SetCurveParamsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    set_curve_params_verify_writable_privileges(accounts)?;
    set_curve_params_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SET_FEE_BPS_IX_ACCOUNTS_LEN: usize = 2;
#[derive(Copy, Clone, Debug)]
pub struct SetFeeBpsAccounts<'me, 'info> {
    pub state: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SetFeeBpsKeys {
    pub state: Pubkey,
    pub authority: Pubkey,
}
impl From<SetFeeBpsAccounts<'_, '_>> for SetFeeBpsKeys {
    fn from(accounts: SetFeeBpsAccounts) -> Self {
        Self {
            state: *accounts.state.key,
            authority: *accounts.authority.key,
        }
    }
}
impl From<SetFeeBpsKeys> for [AccountMeta; SET_FEE_BPS_IX_ACCOUNTS_LEN] {
    fn from(keys: SetFeeBpsKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.authority,
                is_signer: true,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; SET_FEE_BPS_IX_ACCOUNTS_LEN]> for SetFeeBpsKeys {
    fn from(pubkeys: [Pubkey; SET_FEE_BPS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            state: pubkeys[0],
            authority: pubkeys[1],
        }
    }
}
impl<'info> From<SetFeeBpsAccounts<'_, 'info>>
for [AccountInfo<'info>; SET_FEE_BPS_IX_ACCOUNTS_LEN] {
    fn from(accounts: SetFeeBpsAccounts<'_, 'info>) -> Self {
        [accounts.state.clone(), accounts.authority.clone()]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SET_FEE_BPS_IX_ACCOUNTS_LEN]>
for SetFeeBpsAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; SET_FEE_BPS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            state: &arr[0],
            authority: &arr[1],
        }
    }
}
pub const SET_FEE_BPS_IX_DISCM: [u8; 8usize] = [2, 161, 245, 141, 111, 32, 39, 198];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SetFeeBpsIxArgs {
    pub fee_bps: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct SetFeeBpsIxData(pub SetFeeBpsIxArgs);
impl From<SetFeeBpsIxArgs> for SetFeeBpsIxData {
    fn from(args: SetFeeBpsIxArgs) -> Self {
        Self(args)
    }
}
impl SetFeeBpsIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SET_FEE_BPS_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let fee_bps: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(Self(SetFeeBpsIxArgs { fee_bps }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SET_FEE_BPS_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.fee_bps, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn set_fee_bps_ix_with_program_id(
    program_id: Pubkey,
    keys: SetFeeBpsKeys,
    args: SetFeeBpsIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SET_FEE_BPS_IX_ACCOUNTS_LEN] = keys.into();
    let data: SetFeeBpsIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn set_fee_bps_ix(
    keys: SetFeeBpsKeys,
    args: SetFeeBpsIxArgs,
) -> std::io::Result<Instruction> {
    set_fee_bps_ix_with_program_id(MASTERMIND_PROGRAM_ID, keys, args)
}
pub fn set_fee_bps_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SetFeeBpsAccounts<'_, '_>,
    args: SetFeeBpsIxArgs,
) -> ProgramResult {
    let keys: SetFeeBpsKeys = accounts.into();
    let ix = set_fee_bps_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn set_fee_bps_invoke(
    accounts: SetFeeBpsAccounts<'_, '_>,
    args: SetFeeBpsIxArgs,
) -> ProgramResult {
    set_fee_bps_invoke_with_program_id(MASTERMIND_PROGRAM_ID, accounts, args)
}
pub fn set_fee_bps_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SetFeeBpsAccounts<'_, '_>,
    args: SetFeeBpsIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SetFeeBpsKeys = accounts.into();
    let ix = set_fee_bps_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn set_fee_bps_invoke_signed(
    accounts: SetFeeBpsAccounts<'_, '_>,
    args: SetFeeBpsIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    set_fee_bps_invoke_signed_with_program_id(
        MASTERMIND_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn set_fee_bps_verify_account_keys(
    accounts: SetFeeBpsAccounts<'_, '_>,
    keys: SetFeeBpsKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.state.key, keys.state),
        (*accounts.authority.key, keys.authority),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn set_fee_bps_verify_writable_privileges<'me, 'info>(
    accounts: SetFeeBpsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.state, accounts.authority] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn set_fee_bps_verify_signer_privileges<'me, 'info>(
    accounts: SetFeeBpsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn set_fee_bps_verify_account_privileges<'me, 'info>(
    accounts: SetFeeBpsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    set_fee_bps_verify_writable_privileges(accounts)?;
    set_fee_bps_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SET_FEE_RECEIVER_IX_ACCOUNTS_LEN: usize = 2;
#[derive(Copy, Clone, Debug)]
pub struct SetFeeReceiverAccounts<'me, 'info> {
    pub state: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SetFeeReceiverKeys {
    pub state: Pubkey,
    pub authority: Pubkey,
}
impl From<SetFeeReceiverAccounts<'_, '_>> for SetFeeReceiverKeys {
    fn from(accounts: SetFeeReceiverAccounts) -> Self {
        Self {
            state: *accounts.state.key,
            authority: *accounts.authority.key,
        }
    }
}
impl From<SetFeeReceiverKeys> for [AccountMeta; SET_FEE_RECEIVER_IX_ACCOUNTS_LEN] {
    fn from(keys: SetFeeReceiverKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.authority,
                is_signer: true,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; SET_FEE_RECEIVER_IX_ACCOUNTS_LEN]> for SetFeeReceiverKeys {
    fn from(pubkeys: [Pubkey; SET_FEE_RECEIVER_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            state: pubkeys[0],
            authority: pubkeys[1],
        }
    }
}
impl<'info> From<SetFeeReceiverAccounts<'_, 'info>>
for [AccountInfo<'info>; SET_FEE_RECEIVER_IX_ACCOUNTS_LEN] {
    fn from(accounts: SetFeeReceiverAccounts<'_, 'info>) -> Self {
        [accounts.state.clone(), accounts.authority.clone()]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SET_FEE_RECEIVER_IX_ACCOUNTS_LEN]>
for SetFeeReceiverAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; SET_FEE_RECEIVER_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            state: &arr[0],
            authority: &arr[1],
        }
    }
}
pub const SET_FEE_RECEIVER_IX_DISCM: [u8; 8usize] = [
    222, 168, 176, 101, 242, 87, 191, 16,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SetFeeReceiverIxArgs {
    pub fee_receiver: Pubkey,
}
#[derive(Clone, Debug, PartialEq)]
pub struct SetFeeReceiverIxData(pub SetFeeReceiverIxArgs);
impl From<SetFeeReceiverIxArgs> for SetFeeReceiverIxData {
    fn from(args: SetFeeReceiverIxArgs) -> Self {
        Self(args)
    }
}
impl SetFeeReceiverIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SET_FEE_RECEIVER_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let fee_receiver: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(SetFeeReceiverIxArgs {
                fee_receiver,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SET_FEE_RECEIVER_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.fee_receiver, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn set_fee_receiver_ix_with_program_id(
    program_id: Pubkey,
    keys: SetFeeReceiverKeys,
    args: SetFeeReceiverIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SET_FEE_RECEIVER_IX_ACCOUNTS_LEN] = keys.into();
    let data: SetFeeReceiverIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn set_fee_receiver_ix(
    keys: SetFeeReceiverKeys,
    args: SetFeeReceiverIxArgs,
) -> std::io::Result<Instruction> {
    set_fee_receiver_ix_with_program_id(MASTERMIND_PROGRAM_ID, keys, args)
}
pub fn set_fee_receiver_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SetFeeReceiverAccounts<'_, '_>,
    args: SetFeeReceiverIxArgs,
) -> ProgramResult {
    let keys: SetFeeReceiverKeys = accounts.into();
    let ix = set_fee_receiver_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn set_fee_receiver_invoke(
    accounts: SetFeeReceiverAccounts<'_, '_>,
    args: SetFeeReceiverIxArgs,
) -> ProgramResult {
    set_fee_receiver_invoke_with_program_id(MASTERMIND_PROGRAM_ID, accounts, args)
}
pub fn set_fee_receiver_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SetFeeReceiverAccounts<'_, '_>,
    args: SetFeeReceiverIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SetFeeReceiverKeys = accounts.into();
    let ix = set_fee_receiver_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn set_fee_receiver_invoke_signed(
    accounts: SetFeeReceiverAccounts<'_, '_>,
    args: SetFeeReceiverIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    set_fee_receiver_invoke_signed_with_program_id(
        MASTERMIND_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn set_fee_receiver_verify_account_keys(
    accounts: SetFeeReceiverAccounts<'_, '_>,
    keys: SetFeeReceiverKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.state.key, keys.state),
        (*accounts.authority.key, keys.authority),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn set_fee_receiver_verify_writable_privileges<'me, 'info>(
    accounts: SetFeeReceiverAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.state, accounts.authority] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn set_fee_receiver_verify_signer_privileges<'me, 'info>(
    accounts: SetFeeReceiverAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn set_fee_receiver_verify_account_privileges<'me, 'info>(
    accounts: SetFeeReceiverAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    set_fee_receiver_verify_writable_privileges(accounts)?;
    set_fee_receiver_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SET_MIGRATION_LOT_SIZE_IX_ACCOUNTS_LEN: usize = 2;
#[derive(Copy, Clone, Debug)]
pub struct SetMigrationLotSizeAccounts<'me, 'info> {
    pub state: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SetMigrationLotSizeKeys {
    pub state: Pubkey,
    pub authority: Pubkey,
}
impl From<SetMigrationLotSizeAccounts<'_, '_>> for SetMigrationLotSizeKeys {
    fn from(accounts: SetMigrationLotSizeAccounts) -> Self {
        Self {
            state: *accounts.state.key,
            authority: *accounts.authority.key,
        }
    }
}
impl From<SetMigrationLotSizeKeys>
for [AccountMeta; SET_MIGRATION_LOT_SIZE_IX_ACCOUNTS_LEN] {
    fn from(keys: SetMigrationLotSizeKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.authority,
                is_signer: true,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; SET_MIGRATION_LOT_SIZE_IX_ACCOUNTS_LEN]> for SetMigrationLotSizeKeys {
    fn from(pubkeys: [Pubkey; SET_MIGRATION_LOT_SIZE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            state: pubkeys[0],
            authority: pubkeys[1],
        }
    }
}
impl<'info> From<SetMigrationLotSizeAccounts<'_, 'info>>
for [AccountInfo<'info>; SET_MIGRATION_LOT_SIZE_IX_ACCOUNTS_LEN] {
    fn from(accounts: SetMigrationLotSizeAccounts<'_, 'info>) -> Self {
        [accounts.state.clone(), accounts.authority.clone()]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SET_MIGRATION_LOT_SIZE_IX_ACCOUNTS_LEN]>
for SetMigrationLotSizeAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; SET_MIGRATION_LOT_SIZE_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            state: &arr[0],
            authority: &arr[1],
        }
    }
}
pub const SET_MIGRATION_LOT_SIZE_IX_DISCM: [u8; 8usize] = [
    224, 245, 51, 3, 173, 186, 250, 230,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SetMigrationLotSizeIxArgs {
    pub coin_lot_size: u64,
    pub pc_lot_size: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct SetMigrationLotSizeIxData(pub SetMigrationLotSizeIxArgs);
impl From<SetMigrationLotSizeIxArgs> for SetMigrationLotSizeIxData {
    fn from(args: SetMigrationLotSizeIxArgs) -> Self {
        Self(args)
    }
}
impl SetMigrationLotSizeIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SET_MIGRATION_LOT_SIZE_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let coin_lot_size: u64 = crate::borsh_de_or_default(&mut reader)?;
        let pc_lot_size: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(SetMigrationLotSizeIxArgs {
                coin_lot_size,
                pc_lot_size,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SET_MIGRATION_LOT_SIZE_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.coin_lot_size, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.pc_lot_size, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn set_migration_lot_size_ix_with_program_id(
    program_id: Pubkey,
    keys: SetMigrationLotSizeKeys,
    args: SetMigrationLotSizeIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SET_MIGRATION_LOT_SIZE_IX_ACCOUNTS_LEN] = keys.into();
    let data: SetMigrationLotSizeIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn set_migration_lot_size_ix(
    keys: SetMigrationLotSizeKeys,
    args: SetMigrationLotSizeIxArgs,
) -> std::io::Result<Instruction> {
    set_migration_lot_size_ix_with_program_id(MASTERMIND_PROGRAM_ID, keys, args)
}
pub fn set_migration_lot_size_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SetMigrationLotSizeAccounts<'_, '_>,
    args: SetMigrationLotSizeIxArgs,
) -> ProgramResult {
    let keys: SetMigrationLotSizeKeys = accounts.into();
    let ix = set_migration_lot_size_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn set_migration_lot_size_invoke(
    accounts: SetMigrationLotSizeAccounts<'_, '_>,
    args: SetMigrationLotSizeIxArgs,
) -> ProgramResult {
    set_migration_lot_size_invoke_with_program_id(MASTERMIND_PROGRAM_ID, accounts, args)
}
pub fn set_migration_lot_size_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SetMigrationLotSizeAccounts<'_, '_>,
    args: SetMigrationLotSizeIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SetMigrationLotSizeKeys = accounts.into();
    let ix = set_migration_lot_size_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn set_migration_lot_size_invoke_signed(
    accounts: SetMigrationLotSizeAccounts<'_, '_>,
    args: SetMigrationLotSizeIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    set_migration_lot_size_invoke_signed_with_program_id(
        MASTERMIND_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn set_migration_lot_size_verify_account_keys(
    accounts: SetMigrationLotSizeAccounts<'_, '_>,
    keys: SetMigrationLotSizeKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.state.key, keys.state),
        (*accounts.authority.key, keys.authority),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn set_migration_lot_size_verify_writable_privileges<'me, 'info>(
    accounts: SetMigrationLotSizeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.state, accounts.authority] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn set_migration_lot_size_verify_signer_privileges<'me, 'info>(
    accounts: SetMigrationLotSizeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn set_migration_lot_size_verify_account_privileges<'me, 'info>(
    accounts: SetMigrationLotSizeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    set_migration_lot_size_verify_writable_privileges(accounts)?;
    set_migration_lot_size_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SET_MIGRATION_REFUND_IX_ACCOUNTS_LEN: usize = 2;
#[derive(Copy, Clone, Debug)]
pub struct SetMigrationRefundAccounts<'me, 'info> {
    pub state: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SetMigrationRefundKeys {
    pub state: Pubkey,
    pub authority: Pubkey,
}
impl From<SetMigrationRefundAccounts<'_, '_>> for SetMigrationRefundKeys {
    fn from(accounts: SetMigrationRefundAccounts) -> Self {
        Self {
            state: *accounts.state.key,
            authority: *accounts.authority.key,
        }
    }
}
impl From<SetMigrationRefundKeys>
for [AccountMeta; SET_MIGRATION_REFUND_IX_ACCOUNTS_LEN] {
    fn from(keys: SetMigrationRefundKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.authority,
                is_signer: true,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; SET_MIGRATION_REFUND_IX_ACCOUNTS_LEN]> for SetMigrationRefundKeys {
    fn from(pubkeys: [Pubkey; SET_MIGRATION_REFUND_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            state: pubkeys[0],
            authority: pubkeys[1],
        }
    }
}
impl<'info> From<SetMigrationRefundAccounts<'_, 'info>>
for [AccountInfo<'info>; SET_MIGRATION_REFUND_IX_ACCOUNTS_LEN] {
    fn from(accounts: SetMigrationRefundAccounts<'_, 'info>) -> Self {
        [accounts.state.clone(), accounts.authority.clone()]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SET_MIGRATION_REFUND_IX_ACCOUNTS_LEN]>
for SetMigrationRefundAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; SET_MIGRATION_REFUND_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            state: &arr[0],
            authority: &arr[1],
        }
    }
}
pub const SET_MIGRATION_REFUND_IX_DISCM: [u8; 8usize] = [
    148, 239, 194, 248, 85, 104, 213, 52,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SetMigrationRefundIxArgs {
    pub migration_refund: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct SetMigrationRefundIxData(pub SetMigrationRefundIxArgs);
impl From<SetMigrationRefundIxArgs> for SetMigrationRefundIxData {
    fn from(args: SetMigrationRefundIxArgs) -> Self {
        Self(args)
    }
}
impl SetMigrationRefundIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SET_MIGRATION_REFUND_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let migration_refund: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(SetMigrationRefundIxArgs {
                migration_refund,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SET_MIGRATION_REFUND_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.migration_refund, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn set_migration_refund_ix_with_program_id(
    program_id: Pubkey,
    keys: SetMigrationRefundKeys,
    args: SetMigrationRefundIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SET_MIGRATION_REFUND_IX_ACCOUNTS_LEN] = keys.into();
    let data: SetMigrationRefundIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn set_migration_refund_ix(
    keys: SetMigrationRefundKeys,
    args: SetMigrationRefundIxArgs,
) -> std::io::Result<Instruction> {
    set_migration_refund_ix_with_program_id(MASTERMIND_PROGRAM_ID, keys, args)
}
pub fn set_migration_refund_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SetMigrationRefundAccounts<'_, '_>,
    args: SetMigrationRefundIxArgs,
) -> ProgramResult {
    let keys: SetMigrationRefundKeys = accounts.into();
    let ix = set_migration_refund_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn set_migration_refund_invoke(
    accounts: SetMigrationRefundAccounts<'_, '_>,
    args: SetMigrationRefundIxArgs,
) -> ProgramResult {
    set_migration_refund_invoke_with_program_id(MASTERMIND_PROGRAM_ID, accounts, args)
}
pub fn set_migration_refund_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SetMigrationRefundAccounts<'_, '_>,
    args: SetMigrationRefundIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SetMigrationRefundKeys = accounts.into();
    let ix = set_migration_refund_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn set_migration_refund_invoke_signed(
    accounts: SetMigrationRefundAccounts<'_, '_>,
    args: SetMigrationRefundIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    set_migration_refund_invoke_signed_with_program_id(
        MASTERMIND_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn set_migration_refund_verify_account_keys(
    accounts: SetMigrationRefundAccounts<'_, '_>,
    keys: SetMigrationRefundKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.state.key, keys.state),
        (*accounts.authority.key, keys.authority),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn set_migration_refund_verify_writable_privileges<'me, 'info>(
    accounts: SetMigrationRefundAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.state, accounts.authority] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn set_migration_refund_verify_signer_privileges<'me, 'info>(
    accounts: SetMigrationRefundAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn set_migration_refund_verify_account_privileges<'me, 'info>(
    accounts: SetMigrationRefundAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    set_migration_refund_verify_writable_privileges(accounts)?;
    set_migration_refund_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SET_MIGRATOR_IX_ACCOUNTS_LEN: usize = 2;
#[derive(Copy, Clone, Debug)]
pub struct SetMigratorAccounts<'me, 'info> {
    pub state: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SetMigratorKeys {
    pub state: Pubkey,
    pub authority: Pubkey,
}
impl From<SetMigratorAccounts<'_, '_>> for SetMigratorKeys {
    fn from(accounts: SetMigratorAccounts) -> Self {
        Self {
            state: *accounts.state.key,
            authority: *accounts.authority.key,
        }
    }
}
impl From<SetMigratorKeys> for [AccountMeta; SET_MIGRATOR_IX_ACCOUNTS_LEN] {
    fn from(keys: SetMigratorKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.authority,
                is_signer: true,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; SET_MIGRATOR_IX_ACCOUNTS_LEN]> for SetMigratorKeys {
    fn from(pubkeys: [Pubkey; SET_MIGRATOR_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            state: pubkeys[0],
            authority: pubkeys[1],
        }
    }
}
impl<'info> From<SetMigratorAccounts<'_, 'info>>
for [AccountInfo<'info>; SET_MIGRATOR_IX_ACCOUNTS_LEN] {
    fn from(accounts: SetMigratorAccounts<'_, 'info>) -> Self {
        [accounts.state.clone(), accounts.authority.clone()]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SET_MIGRATOR_IX_ACCOUNTS_LEN]>
for SetMigratorAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; SET_MIGRATOR_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            state: &arr[0],
            authority: &arr[1],
        }
    }
}
pub const SET_MIGRATOR_IX_DISCM: [u8; 8usize] = [209, 164, 101, 138, 178, 62, 195, 249];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SetMigratorIxArgs {
    pub migrator: Pubkey,
}
#[derive(Clone, Debug, PartialEq)]
pub struct SetMigratorIxData(pub SetMigratorIxArgs);
impl From<SetMigratorIxArgs> for SetMigratorIxData {
    fn from(args: SetMigratorIxArgs) -> Self {
        Self(args)
    }
}
impl SetMigratorIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SET_MIGRATOR_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let migrator: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        Ok(Self(SetMigratorIxArgs { migrator }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SET_MIGRATOR_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.migrator, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn set_migrator_ix_with_program_id(
    program_id: Pubkey,
    keys: SetMigratorKeys,
    args: SetMigratorIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SET_MIGRATOR_IX_ACCOUNTS_LEN] = keys.into();
    let data: SetMigratorIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn set_migrator_ix(
    keys: SetMigratorKeys,
    args: SetMigratorIxArgs,
) -> std::io::Result<Instruction> {
    set_migrator_ix_with_program_id(MASTERMIND_PROGRAM_ID, keys, args)
}
pub fn set_migrator_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SetMigratorAccounts<'_, '_>,
    args: SetMigratorIxArgs,
) -> ProgramResult {
    let keys: SetMigratorKeys = accounts.into();
    let ix = set_migrator_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn set_migrator_invoke(
    accounts: SetMigratorAccounts<'_, '_>,
    args: SetMigratorIxArgs,
) -> ProgramResult {
    set_migrator_invoke_with_program_id(MASTERMIND_PROGRAM_ID, accounts, args)
}
pub fn set_migrator_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SetMigratorAccounts<'_, '_>,
    args: SetMigratorIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SetMigratorKeys = accounts.into();
    let ix = set_migrator_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn set_migrator_invoke_signed(
    accounts: SetMigratorAccounts<'_, '_>,
    args: SetMigratorIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    set_migrator_invoke_signed_with_program_id(
        MASTERMIND_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn set_migrator_verify_account_keys(
    accounts: SetMigratorAccounts<'_, '_>,
    keys: SetMigratorKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.state.key, keys.state),
        (*accounts.authority.key, keys.authority),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn set_migrator_verify_writable_privileges<'me, 'info>(
    accounts: SetMigratorAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.state, accounts.authority] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn set_migrator_verify_signer_privileges<'me, 'info>(
    accounts: SetMigratorAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn set_migrator_verify_account_privileges<'me, 'info>(
    accounts: SetMigratorAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    set_migrator_verify_writable_privileges(accounts)?;
    set_migrator_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SET_PENDING_AUTHORITY_IX_ACCOUNTS_LEN: usize = 2;
#[derive(Copy, Clone, Debug)]
pub struct SetPendingAuthorityAccounts<'me, 'info> {
    pub state: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SetPendingAuthorityKeys {
    pub state: Pubkey,
    pub authority: Pubkey,
}
impl From<SetPendingAuthorityAccounts<'_, '_>> for SetPendingAuthorityKeys {
    fn from(accounts: SetPendingAuthorityAccounts) -> Self {
        Self {
            state: *accounts.state.key,
            authority: *accounts.authority.key,
        }
    }
}
impl From<SetPendingAuthorityKeys>
for [AccountMeta; SET_PENDING_AUTHORITY_IX_ACCOUNTS_LEN] {
    fn from(keys: SetPendingAuthorityKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.authority,
                is_signer: true,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; SET_PENDING_AUTHORITY_IX_ACCOUNTS_LEN]> for SetPendingAuthorityKeys {
    fn from(pubkeys: [Pubkey; SET_PENDING_AUTHORITY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            state: pubkeys[0],
            authority: pubkeys[1],
        }
    }
}
impl<'info> From<SetPendingAuthorityAccounts<'_, 'info>>
for [AccountInfo<'info>; SET_PENDING_AUTHORITY_IX_ACCOUNTS_LEN] {
    fn from(accounts: SetPendingAuthorityAccounts<'_, 'info>) -> Self {
        [accounts.state.clone(), accounts.authority.clone()]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SET_PENDING_AUTHORITY_IX_ACCOUNTS_LEN]>
for SetPendingAuthorityAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; SET_PENDING_AUTHORITY_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            state: &arr[0],
            authority: &arr[1],
        }
    }
}
pub const SET_PENDING_AUTHORITY_IX_DISCM: [u8; 8usize] = [
    175, 71, 167, 223, 49, 144, 102, 193,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SetPendingAuthorityIxArgs {
    pub pending_authority: Pubkey,
}
#[derive(Clone, Debug, PartialEq)]
pub struct SetPendingAuthorityIxData(pub SetPendingAuthorityIxArgs);
impl From<SetPendingAuthorityIxArgs> for SetPendingAuthorityIxData {
    fn from(args: SetPendingAuthorityIxArgs) -> Self {
        Self(args)
    }
}
impl SetPendingAuthorityIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SET_PENDING_AUTHORITY_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let pending_authority: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(SetPendingAuthorityIxArgs {
                pending_authority,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SET_PENDING_AUTHORITY_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.pending_authority, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn set_pending_authority_ix_with_program_id(
    program_id: Pubkey,
    keys: SetPendingAuthorityKeys,
    args: SetPendingAuthorityIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SET_PENDING_AUTHORITY_IX_ACCOUNTS_LEN] = keys.into();
    let data: SetPendingAuthorityIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn set_pending_authority_ix(
    keys: SetPendingAuthorityKeys,
    args: SetPendingAuthorityIxArgs,
) -> std::io::Result<Instruction> {
    set_pending_authority_ix_with_program_id(MASTERMIND_PROGRAM_ID, keys, args)
}
pub fn set_pending_authority_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SetPendingAuthorityAccounts<'_, '_>,
    args: SetPendingAuthorityIxArgs,
) -> ProgramResult {
    let keys: SetPendingAuthorityKeys = accounts.into();
    let ix = set_pending_authority_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn set_pending_authority_invoke(
    accounts: SetPendingAuthorityAccounts<'_, '_>,
    args: SetPendingAuthorityIxArgs,
) -> ProgramResult {
    set_pending_authority_invoke_with_program_id(MASTERMIND_PROGRAM_ID, accounts, args)
}
pub fn set_pending_authority_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SetPendingAuthorityAccounts<'_, '_>,
    args: SetPendingAuthorityIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SetPendingAuthorityKeys = accounts.into();
    let ix = set_pending_authority_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn set_pending_authority_invoke_signed(
    accounts: SetPendingAuthorityAccounts<'_, '_>,
    args: SetPendingAuthorityIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    set_pending_authority_invoke_signed_with_program_id(
        MASTERMIND_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn set_pending_authority_verify_account_keys(
    accounts: SetPendingAuthorityAccounts<'_, '_>,
    keys: SetPendingAuthorityKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.state.key, keys.state),
        (*accounts.authority.key, keys.authority),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn set_pending_authority_verify_writable_privileges<'me, 'info>(
    accounts: SetPendingAuthorityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.state, accounts.authority] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn set_pending_authority_verify_signer_privileges<'me, 'info>(
    accounts: SetPendingAuthorityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn set_pending_authority_verify_account_privileges<'me, 'info>(
    accounts: SetPendingAuthorityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    set_pending_authority_verify_writable_privileges(accounts)?;
    set_pending_authority_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SET_WRAPPER_MINT_IX_ACCOUNTS_LEN: usize = 2;
#[derive(Copy, Clone, Debug)]
pub struct SetWrapperMintAccounts<'me, 'info> {
    pub state: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SetWrapperMintKeys {
    pub state: Pubkey,
    pub authority: Pubkey,
}
impl From<SetWrapperMintAccounts<'_, '_>> for SetWrapperMintKeys {
    fn from(accounts: SetWrapperMintAccounts) -> Self {
        Self {
            state: *accounts.state.key,
            authority: *accounts.authority.key,
        }
    }
}
impl From<SetWrapperMintKeys> for [AccountMeta; SET_WRAPPER_MINT_IX_ACCOUNTS_LEN] {
    fn from(keys: SetWrapperMintKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.authority,
                is_signer: true,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; SET_WRAPPER_MINT_IX_ACCOUNTS_LEN]> for SetWrapperMintKeys {
    fn from(pubkeys: [Pubkey; SET_WRAPPER_MINT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            state: pubkeys[0],
            authority: pubkeys[1],
        }
    }
}
impl<'info> From<SetWrapperMintAccounts<'_, 'info>>
for [AccountInfo<'info>; SET_WRAPPER_MINT_IX_ACCOUNTS_LEN] {
    fn from(accounts: SetWrapperMintAccounts<'_, 'info>) -> Self {
        [accounts.state.clone(), accounts.authority.clone()]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SET_WRAPPER_MINT_IX_ACCOUNTS_LEN]>
for SetWrapperMintAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; SET_WRAPPER_MINT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            state: &arr[0],
            authority: &arr[1],
        }
    }
}
pub const SET_WRAPPER_MINT_IX_DISCM: [u8; 8usize] = [65, 169, 74, 24, 18, 148, 174, 154];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SetWrapperMintIxArgs {
    pub wrapper_mint: Pubkey,
}
#[derive(Clone, Debug, PartialEq)]
pub struct SetWrapperMintIxData(pub SetWrapperMintIxArgs);
impl From<SetWrapperMintIxArgs> for SetWrapperMintIxData {
    fn from(args: SetWrapperMintIxArgs) -> Self {
        Self(args)
    }
}
impl SetWrapperMintIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SET_WRAPPER_MINT_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let wrapper_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(SetWrapperMintIxArgs {
                wrapper_mint,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SET_WRAPPER_MINT_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.wrapper_mint, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn set_wrapper_mint_ix_with_program_id(
    program_id: Pubkey,
    keys: SetWrapperMintKeys,
    args: SetWrapperMintIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SET_WRAPPER_MINT_IX_ACCOUNTS_LEN] = keys.into();
    let data: SetWrapperMintIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn set_wrapper_mint_ix(
    keys: SetWrapperMintKeys,
    args: SetWrapperMintIxArgs,
) -> std::io::Result<Instruction> {
    set_wrapper_mint_ix_with_program_id(MASTERMIND_PROGRAM_ID, keys, args)
}
pub fn set_wrapper_mint_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SetWrapperMintAccounts<'_, '_>,
    args: SetWrapperMintIxArgs,
) -> ProgramResult {
    let keys: SetWrapperMintKeys = accounts.into();
    let ix = set_wrapper_mint_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn set_wrapper_mint_invoke(
    accounts: SetWrapperMintAccounts<'_, '_>,
    args: SetWrapperMintIxArgs,
) -> ProgramResult {
    set_wrapper_mint_invoke_with_program_id(MASTERMIND_PROGRAM_ID, accounts, args)
}
pub fn set_wrapper_mint_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SetWrapperMintAccounts<'_, '_>,
    args: SetWrapperMintIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SetWrapperMintKeys = accounts.into();
    let ix = set_wrapper_mint_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn set_wrapper_mint_invoke_signed(
    accounts: SetWrapperMintAccounts<'_, '_>,
    args: SetWrapperMintIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    set_wrapper_mint_invoke_signed_with_program_id(
        MASTERMIND_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn set_wrapper_mint_verify_account_keys(
    accounts: SetWrapperMintAccounts<'_, '_>,
    keys: SetWrapperMintKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.state.key, keys.state),
        (*accounts.authority.key, keys.authority),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn set_wrapper_mint_verify_writable_privileges<'me, 'info>(
    accounts: SetWrapperMintAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.state, accounts.authority] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn set_wrapper_mint_verify_signer_privileges<'me, 'info>(
    accounts: SetWrapperMintAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn set_wrapper_mint_verify_account_privileges<'me, 'info>(
    accounts: SetWrapperMintAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    set_wrapper_mint_verify_writable_privileges(accounts)?;
    set_wrapper_mint_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SYNC_WRAPPER_IX_ACCOUNTS_LEN: usize = 14;
#[derive(Copy, Clone, Debug)]
pub struct SyncWrapperAccounts<'me, 'info> {
    pub state: &'me AccountInfo<'info>,
    pub wrapper_program: &'me AccountInfo<'info>,
    pub wrapper_state: &'me AccountInfo<'info>,
    pub wrapper_mint: &'me AccountInfo<'info>,
    pub wrapper_mint_authority: &'me AccountInfo<'info>,
    pub fee_receiver_token_account: &'me AccountInfo<'info>,
    pub payer: &'me AccountInfo<'info>,
    pub fee_receiver: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub rent: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SyncWrapperKeys {
    pub state: Pubkey,
    pub wrapper_program: Pubkey,
    pub wrapper_state: Pubkey,
    pub wrapper_mint: Pubkey,
    pub wrapper_mint_authority: Pubkey,
    pub fee_receiver_token_account: Pubkey,
    pub payer: Pubkey,
    pub fee_receiver: Pubkey,
    pub token_program: Pubkey,
    pub associated_token_program: Pubkey,
    pub system_program: Pubkey,
    pub rent: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<SyncWrapperAccounts<'_, '_>> for SyncWrapperKeys {
    fn from(accounts: SyncWrapperAccounts) -> Self {
        Self {
            state: *accounts.state.key,
            wrapper_program: *accounts.wrapper_program.key,
            wrapper_state: *accounts.wrapper_state.key,
            wrapper_mint: *accounts.wrapper_mint.key,
            wrapper_mint_authority: *accounts.wrapper_mint_authority.key,
            fee_receiver_token_account: *accounts.fee_receiver_token_account.key,
            payer: *accounts.payer.key,
            fee_receiver: *accounts.fee_receiver.key,
            token_program: *accounts.token_program.key,
            associated_token_program: *accounts.associated_token_program.key,
            system_program: *accounts.system_program.key,
            rent: *accounts.rent.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<SyncWrapperKeys> for [AccountMeta; SYNC_WRAPPER_IX_ACCOUNTS_LEN] {
    fn from(keys: SyncWrapperKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.wrapper_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.wrapper_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.wrapper_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.wrapper_mint_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.fee_receiver_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.payer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.fee_receiver,
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
            AccountMeta {
                pubkey: keys.rent,
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
impl From<[Pubkey; SYNC_WRAPPER_IX_ACCOUNTS_LEN]> for SyncWrapperKeys {
    fn from(pubkeys: [Pubkey; SYNC_WRAPPER_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            state: pubkeys[0],
            wrapper_program: pubkeys[1],
            wrapper_state: pubkeys[2],
            wrapper_mint: pubkeys[3],
            wrapper_mint_authority: pubkeys[4],
            fee_receiver_token_account: pubkeys[5],
            payer: pubkeys[6],
            fee_receiver: pubkeys[7],
            token_program: pubkeys[8],
            associated_token_program: pubkeys[9],
            system_program: pubkeys[10],
            rent: pubkeys[11],
            event_authority: pubkeys[12],
            program: pubkeys[13],
        }
    }
}
impl<'info> From<SyncWrapperAccounts<'_, 'info>>
for [AccountInfo<'info>; SYNC_WRAPPER_IX_ACCOUNTS_LEN] {
    fn from(accounts: SyncWrapperAccounts<'_, 'info>) -> Self {
        [
            accounts.state.clone(),
            accounts.wrapper_program.clone(),
            accounts.wrapper_state.clone(),
            accounts.wrapper_mint.clone(),
            accounts.wrapper_mint_authority.clone(),
            accounts.fee_receiver_token_account.clone(),
            accounts.payer.clone(),
            accounts.fee_receiver.clone(),
            accounts.token_program.clone(),
            accounts.associated_token_program.clone(),
            accounts.system_program.clone(),
            accounts.rent.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SYNC_WRAPPER_IX_ACCOUNTS_LEN]>
for SyncWrapperAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; SYNC_WRAPPER_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            state: &arr[0],
            wrapper_program: &arr[1],
            wrapper_state: &arr[2],
            wrapper_mint: &arr[3],
            wrapper_mint_authority: &arr[4],
            fee_receiver_token_account: &arr[5],
            payer: &arr[6],
            fee_receiver: &arr[7],
            token_program: &arr[8],
            associated_token_program: &arr[9],
            system_program: &arr[10],
            rent: &arr[11],
            event_authority: &arr[12],
            program: &arr[13],
        }
    }
}
pub const SYNC_WRAPPER_IX_DISCM: [u8; 8usize] = [58, 56, 206, 153, 224, 221, 169, 86];
#[derive(Clone, Debug, PartialEq)]
pub struct SyncWrapperIxData;
impl SyncWrapperIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SYNC_WRAPPER_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SYNC_WRAPPER_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn sync_wrapper_ix_with_program_id(
    program_id: Pubkey,
    keys: SyncWrapperKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SYNC_WRAPPER_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: SyncWrapperIxData.try_to_vec()?,
    })
}
pub fn sync_wrapper_ix(keys: SyncWrapperKeys) -> std::io::Result<Instruction> {
    sync_wrapper_ix_with_program_id(MASTERMIND_PROGRAM_ID, keys)
}
pub fn sync_wrapper_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SyncWrapperAccounts<'_, '_>,
) -> ProgramResult {
    let keys: SyncWrapperKeys = accounts.into();
    let ix = sync_wrapper_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn sync_wrapper_invoke(accounts: SyncWrapperAccounts<'_, '_>) -> ProgramResult {
    sync_wrapper_invoke_with_program_id(MASTERMIND_PROGRAM_ID, accounts)
}
pub fn sync_wrapper_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SyncWrapperAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SyncWrapperKeys = accounts.into();
    let ix = sync_wrapper_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn sync_wrapper_invoke_signed(
    accounts: SyncWrapperAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    sync_wrapper_invoke_signed_with_program_id(MASTERMIND_PROGRAM_ID, accounts, seeds)
}
pub fn sync_wrapper_verify_account_keys(
    accounts: SyncWrapperAccounts<'_, '_>,
    keys: SyncWrapperKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.state.key, keys.state),
        (*accounts.wrapper_program.key, keys.wrapper_program),
        (*accounts.wrapper_state.key, keys.wrapper_state),
        (*accounts.wrapper_mint.key, keys.wrapper_mint),
        (*accounts.wrapper_mint_authority.key, keys.wrapper_mint_authority),
        (*accounts.fee_receiver_token_account.key, keys.fee_receiver_token_account),
        (*accounts.payer.key, keys.payer),
        (*accounts.fee_receiver.key, keys.fee_receiver),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.associated_token_program.key, keys.associated_token_program),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.rent.key, keys.rent),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn sync_wrapper_verify_writable_privileges<'me, 'info>(
    accounts: SyncWrapperAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.state,
        accounts.wrapper_state,
        accounts.wrapper_mint,
        accounts.fee_receiver_token_account,
        accounts.payer,
        accounts.fee_receiver,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn sync_wrapper_verify_signer_privileges<'me, 'info>(
    accounts: SyncWrapperAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.payer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn sync_wrapper_verify_account_privileges<'me, 'info>(
    accounts: SyncWrapperAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    sync_wrapper_verify_writable_privileges(accounts)?;
    sync_wrapper_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const WRAP_SOL_FOR_CURVE_IX_ACCOUNTS_LEN: usize = 16;
#[derive(Copy, Clone, Debug)]
pub struct WrapSolForCurveAccounts<'me, 'info> {
    pub state: &'me AccountInfo<'info>,
    pub token_mint: &'me AccountInfo<'info>,
    pub bonding_curve: &'me AccountInfo<'info>,
    pub bonding_curve_sol_vault: &'me AccountInfo<'info>,
    pub wrapper_program: &'me AccountInfo<'info>,
    pub wrapper_state: &'me AccountInfo<'info>,
    pub wrapper_mint: &'me AccountInfo<'info>,
    pub wrapper_mint_authority: &'me AccountInfo<'info>,
    pub wrapper_bonding_curve_token_account: &'me AccountInfo<'info>,
    pub payer: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub rent: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct WrapSolForCurveKeys {
    pub state: Pubkey,
    pub token_mint: Pubkey,
    pub bonding_curve: Pubkey,
    pub bonding_curve_sol_vault: Pubkey,
    pub wrapper_program: Pubkey,
    pub wrapper_state: Pubkey,
    pub wrapper_mint: Pubkey,
    pub wrapper_mint_authority: Pubkey,
    pub wrapper_bonding_curve_token_account: Pubkey,
    pub payer: Pubkey,
    pub token_program: Pubkey,
    pub associated_token_program: Pubkey,
    pub system_program: Pubkey,
    pub rent: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<WrapSolForCurveAccounts<'_, '_>> for WrapSolForCurveKeys {
    fn from(accounts: WrapSolForCurveAccounts) -> Self {
        Self {
            state: *accounts.state.key,
            token_mint: *accounts.token_mint.key,
            bonding_curve: *accounts.bonding_curve.key,
            bonding_curve_sol_vault: *accounts.bonding_curve_sol_vault.key,
            wrapper_program: *accounts.wrapper_program.key,
            wrapper_state: *accounts.wrapper_state.key,
            wrapper_mint: *accounts.wrapper_mint.key,
            wrapper_mint_authority: *accounts.wrapper_mint_authority.key,
            wrapper_bonding_curve_token_account: *accounts
                .wrapper_bonding_curve_token_account
                .key,
            payer: *accounts.payer.key,
            token_program: *accounts.token_program.key,
            associated_token_program: *accounts.associated_token_program.key,
            system_program: *accounts.system_program.key,
            rent: *accounts.rent.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<WrapSolForCurveKeys> for [AccountMeta; WRAP_SOL_FOR_CURVE_IX_ACCOUNTS_LEN] {
    fn from(keys: WrapSolForCurveKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.state,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.bonding_curve,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.bonding_curve_sol_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.wrapper_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.wrapper_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.wrapper_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.wrapper_mint_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.wrapper_bonding_curve_token_account,
                is_signer: false,
                is_writable: true,
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
                pubkey: keys.rent,
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
impl From<[Pubkey; WRAP_SOL_FOR_CURVE_IX_ACCOUNTS_LEN]> for WrapSolForCurveKeys {
    fn from(pubkeys: [Pubkey; WRAP_SOL_FOR_CURVE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            state: pubkeys[0],
            token_mint: pubkeys[1],
            bonding_curve: pubkeys[2],
            bonding_curve_sol_vault: pubkeys[3],
            wrapper_program: pubkeys[4],
            wrapper_state: pubkeys[5],
            wrapper_mint: pubkeys[6],
            wrapper_mint_authority: pubkeys[7],
            wrapper_bonding_curve_token_account: pubkeys[8],
            payer: pubkeys[9],
            token_program: pubkeys[10],
            associated_token_program: pubkeys[11],
            system_program: pubkeys[12],
            rent: pubkeys[13],
            event_authority: pubkeys[14],
            program: pubkeys[15],
        }
    }
}
impl<'info> From<WrapSolForCurveAccounts<'_, 'info>>
for [AccountInfo<'info>; WRAP_SOL_FOR_CURVE_IX_ACCOUNTS_LEN] {
    fn from(accounts: WrapSolForCurveAccounts<'_, 'info>) -> Self {
        [
            accounts.state.clone(),
            accounts.token_mint.clone(),
            accounts.bonding_curve.clone(),
            accounts.bonding_curve_sol_vault.clone(),
            accounts.wrapper_program.clone(),
            accounts.wrapper_state.clone(),
            accounts.wrapper_mint.clone(),
            accounts.wrapper_mint_authority.clone(),
            accounts.wrapper_bonding_curve_token_account.clone(),
            accounts.payer.clone(),
            accounts.token_program.clone(),
            accounts.associated_token_program.clone(),
            accounts.system_program.clone(),
            accounts.rent.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; WRAP_SOL_FOR_CURVE_IX_ACCOUNTS_LEN]>
for WrapSolForCurveAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; WRAP_SOL_FOR_CURVE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            state: &arr[0],
            token_mint: &arr[1],
            bonding_curve: &arr[2],
            bonding_curve_sol_vault: &arr[3],
            wrapper_program: &arr[4],
            wrapper_state: &arr[5],
            wrapper_mint: &arr[6],
            wrapper_mint_authority: &arr[7],
            wrapper_bonding_curve_token_account: &arr[8],
            payer: &arr[9],
            token_program: &arr[10],
            associated_token_program: &arr[11],
            system_program: &arr[12],
            rent: &arr[13],
            event_authority: &arr[14],
            program: &arr[15],
        }
    }
}
pub const WRAP_SOL_FOR_CURVE_IX_DISCM: [u8; 8usize] = [
    32, 147, 121, 169, 176, 40, 51, 95,
];
#[derive(Clone, Debug, PartialEq)]
pub struct WrapSolForCurveIxData;
impl WrapSolForCurveIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != WRAP_SOL_FOR_CURVE_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&WRAP_SOL_FOR_CURVE_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn wrap_sol_for_curve_ix_with_program_id(
    program_id: Pubkey,
    keys: WrapSolForCurveKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; WRAP_SOL_FOR_CURVE_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: WrapSolForCurveIxData.try_to_vec()?,
    })
}
pub fn wrap_sol_for_curve_ix(keys: WrapSolForCurveKeys) -> std::io::Result<Instruction> {
    wrap_sol_for_curve_ix_with_program_id(MASTERMIND_PROGRAM_ID, keys)
}
pub fn wrap_sol_for_curve_invoke_with_program_id(
    program_id: Pubkey,
    accounts: WrapSolForCurveAccounts<'_, '_>,
) -> ProgramResult {
    let keys: WrapSolForCurveKeys = accounts.into();
    let ix = wrap_sol_for_curve_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn wrap_sol_for_curve_invoke(
    accounts: WrapSolForCurveAccounts<'_, '_>,
) -> ProgramResult {
    wrap_sol_for_curve_invoke_with_program_id(MASTERMIND_PROGRAM_ID, accounts)
}
pub fn wrap_sol_for_curve_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: WrapSolForCurveAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: WrapSolForCurveKeys = accounts.into();
    let ix = wrap_sol_for_curve_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn wrap_sol_for_curve_invoke_signed(
    accounts: WrapSolForCurveAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    wrap_sol_for_curve_invoke_signed_with_program_id(
        MASTERMIND_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn wrap_sol_for_curve_verify_account_keys(
    accounts: WrapSolForCurveAccounts<'_, '_>,
    keys: WrapSolForCurveKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.state.key, keys.state),
        (*accounts.token_mint.key, keys.token_mint),
        (*accounts.bonding_curve.key, keys.bonding_curve),
        (*accounts.bonding_curve_sol_vault.key, keys.bonding_curve_sol_vault),
        (*accounts.wrapper_program.key, keys.wrapper_program),
        (*accounts.wrapper_state.key, keys.wrapper_state),
        (*accounts.wrapper_mint.key, keys.wrapper_mint),
        (*accounts.wrapper_mint_authority.key, keys.wrapper_mint_authority),
        (
            *accounts.wrapper_bonding_curve_token_account.key,
            keys.wrapper_bonding_curve_token_account,
        ),
        (*accounts.payer.key, keys.payer),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.associated_token_program.key, keys.associated_token_program),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.rent.key, keys.rent),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn wrap_sol_for_curve_verify_writable_privileges<'me, 'info>(
    accounts: WrapSolForCurveAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.token_mint,
        accounts.bonding_curve,
        accounts.bonding_curve_sol_vault,
        accounts.wrapper_state,
        accounts.wrapper_mint,
        accounts.wrapper_bonding_curve_token_account,
        accounts.payer,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn wrap_sol_for_curve_verify_signer_privileges<'me, 'info>(
    accounts: WrapSolForCurveAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.payer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn wrap_sol_for_curve_verify_account_privileges<'me, 'info>(
    accounts: WrapSolForCurveAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    wrap_sol_for_curve_verify_writable_privileges(accounts)?;
    wrap_sol_for_curve_verify_signer_privileges(accounts)?;
    Ok(())
}
