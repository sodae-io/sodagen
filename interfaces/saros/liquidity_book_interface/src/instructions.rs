use solana_pubkey::Pubkey;
use solana_cpi::{invoke, invoke_signed};
use solana_instruction::{AccountMeta, Instruction};
use solana_account_info::AccountInfo;
use solana_program_error::ProgramError;
use std::io::Read;
#[allow(unused_imports)]
use crate::*;
#[derive(Clone, Debug, PartialEq)]
pub enum LiquidityBookProgramIx {
    AcceptConfigOwnership,
    ClosePosition,
    CreatePosition(CreatePositionIxArgs),
    DecreasePosition(DecreasePositionIxArgs),
    IncreasePosition(IncreasePositionIxArgs),
    InitializeBinArray(InitializeBinArrayIxArgs),
    InitializeBinStepConfig(InitializeBinStepConfigIxArgs),
    InitializeConfig(InitializeConfigIxArgs),
    InitializePair(InitializePairIxArgs),
    InitializeQuoteAssetBadge,
    SetHook,
    Swap(SwapIxArgs),
    TransferConfigOwnership(TransferConfigOwnershipIxArgs),
    UpdateBinStepConfig(UpdateBinStepConfigIxArgs),
    UpdatePairStaticFeeParameters(UpdatePairStaticFeeParametersIxArgs),
    UpdateQuoteAssetBadge(UpdateQuoteAssetBadgeIxArgs),
    WithdrawProtocolFees,
}
impl LiquidityBookProgramIx {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        if buf.starts_with(&ACCEPT_CONFIG_OWNERSHIP_IX_DISCM) {
            return Ok(Self::AcceptConfigOwnership);
        }
        if buf.starts_with(&CLOSE_POSITION_IX_DISCM) {
            return Ok(Self::ClosePosition);
        }
        if buf.starts_with(&CREATE_POSITION_IX_DISCM) {
            let mut reader = &buf[CREATE_POSITION_IX_DISCM.len()..];
            let relative_bin_id_left: i32 = crate::borsh_de_or_default(&mut reader)?;
            let relative_bin_in_right: i32 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::CreatePosition(CreatePositionIxArgs {
                    relative_bin_id_left,
                    relative_bin_in_right,
                }),
            );
        }
        if buf.starts_with(&DECREASE_POSITION_IX_DISCM) {
            let mut reader = &buf[DECREASE_POSITION_IX_DISCM.len()..];
            let shares: Vec<u128> = crate::borsh_de_or_default(&mut reader)?;
            return Ok(Self::DecreasePosition(DecreasePositionIxArgs { shares }));
        }
        if buf.starts_with(&INCREASE_POSITION_IX_DISCM) {
            let mut reader = &buf[INCREASE_POSITION_IX_DISCM.len()..];
            let amount_x: u64 = crate::borsh_de_or_default(&mut reader)?;
            let amount_y: u64 = crate::borsh_de_or_default(&mut reader)?;
            let liquidity_distribution: Vec<BinLiquidityDistribution> = crate::borsh_de_or_default(
                &mut reader,
            )?;
            return Ok(
                Self::IncreasePosition(IncreasePositionIxArgs {
                    amount_x,
                    amount_y,
                    liquidity_distribution,
                }),
            );
        }
        if buf.starts_with(&INITIALIZE_BIN_ARRAY_IX_DISCM) {
            let mut reader = &buf[INITIALIZE_BIN_ARRAY_IX_DISCM.len()..];
            let id: u32 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(Self::InitializeBinArray(InitializeBinArrayIxArgs { id }));
        }
        if buf.starts_with(&INITIALIZE_BIN_STEP_CONFIG_IX_DISCM) {
            let mut reader = &buf[INITIALIZE_BIN_STEP_CONFIG_IX_DISCM.len()..];
            let bin_step: u8 = crate::borsh_de_or_default(&mut reader)?;
            let availability: ConfigAvailability = crate::borsh_de_or_default(
                &mut reader,
            )?;
            let fee_parameters = if reader.is_empty() {
                Default::default()
            } else {
                <StaticFeeParameters>::deserialize(&mut reader)?
            };
            return Ok(
                Self::InitializeBinStepConfig(InitializeBinStepConfigIxArgs {
                    bin_step,
                    availability,
                    fee_parameters,
                }),
            );
        }
        if buf.starts_with(&INITIALIZE_CONFIG_IX_DISCM) {
            let mut reader = &buf[INITIALIZE_CONFIG_IX_DISCM.len()..];
            let fee_authority: Pubkey = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::InitializeConfig(InitializeConfigIxArgs {
                    fee_authority,
                }),
            );
        }
        if buf.starts_with(&INITIALIZE_PAIR_IX_DISCM) {
            let mut reader = &buf[INITIALIZE_PAIR_IX_DISCM.len()..];
            let active_id: u32 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(Self::InitializePair(InitializePairIxArgs { active_id }));
        }
        if buf.starts_with(&INITIALIZE_QUOTE_ASSET_BADGE_IX_DISCM) {
            return Ok(Self::InitializeQuoteAssetBadge);
        }
        if buf.starts_with(&SET_HOOK_IX_DISCM) {
            return Ok(Self::SetHook);
        }
        if buf.starts_with(&SWAP_IX_DISCM) {
            let mut reader = &buf[SWAP_IX_DISCM.len()..];
            let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            let other_amount_threshold: u64 = crate::borsh_de_or_default(&mut reader)?;
            let swap_for_y: bool = crate::borsh_de_or_default(&mut reader)?;
            let swap_type: SwapType = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::Swap(SwapIxArgs {
                    amount,
                    other_amount_threshold,
                    swap_for_y,
                    swap_type,
                }),
            );
        }
        if buf.starts_with(&TRANSFER_CONFIG_OWNERSHIP_IX_DISCM) {
            let mut reader = &buf[TRANSFER_CONFIG_OWNERSHIP_IX_DISCM.len()..];
            let new_authority: Option<Pubkey> = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::TransferConfigOwnership(TransferConfigOwnershipIxArgs {
                    new_authority,
                }),
            );
        }
        if buf.starts_with(&UPDATE_BIN_STEP_CONFIG_IX_DISCM) {
            let mut reader = &buf[UPDATE_BIN_STEP_CONFIG_IX_DISCM.len()..];
            let status: ConfigStatus = crate::borsh_de_or_default(&mut reader)?;
            let availability: ConfigAvailability = crate::borsh_de_or_default(
                &mut reader,
            )?;
            let fee_parameters = if reader.is_empty() {
                Default::default()
            } else {
                <StaticFeeParameters>::deserialize(&mut reader)?
            };
            return Ok(
                Self::UpdateBinStepConfig(UpdateBinStepConfigIxArgs {
                    status,
                    availability,
                    fee_parameters,
                }),
            );
        }
        if buf.starts_with(&UPDATE_PAIR_STATIC_FEE_PARAMETERS_IX_DISCM) {
            let mut reader = &buf[UPDATE_PAIR_STATIC_FEE_PARAMETERS_IX_DISCM.len()..];
            let fee_parameters = if reader.is_empty() {
                Default::default()
            } else {
                <StaticFeeParameters>::deserialize(&mut reader)?
            };
            return Ok(
                Self::UpdatePairStaticFeeParameters(UpdatePairStaticFeeParametersIxArgs {
                    fee_parameters,
                }),
            );
        }
        if buf.starts_with(&UPDATE_QUOTE_ASSET_BADGE_IX_DISCM) {
            let mut reader = &buf[UPDATE_QUOTE_ASSET_BADGE_IX_DISCM.len()..];
            let status: QuoteAssetBadgeStatus = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::UpdateQuoteAssetBadge(UpdateQuoteAssetBadgeIxArgs {
                    status,
                }),
            );
        }
        if buf.starts_with(&WITHDRAW_PROTOCOL_FEES_IX_DISCM) {
            return Ok(Self::WithdrawProtocolFees);
        }
        Err(std::io::Error::from(std::io::ErrorKind::InvalidData))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        match self {
            Self::AcceptConfigOwnership => {
                writer.write_all(&ACCEPT_CONFIG_OWNERSHIP_IX_DISCM)
            }
            Self::ClosePosition => writer.write_all(&CLOSE_POSITION_IX_DISCM),
            Self::CreatePosition(args) => {
                writer.write_all(&CREATE_POSITION_IX_DISCM)?;
                borsh::BorshSerialize::serialize(
                    &args.relative_bin_id_left,
                    &mut writer,
                )?;
                borsh::BorshSerialize::serialize(
                    &args.relative_bin_in_right,
                    &mut writer,
                )?;
                Ok(())
            }
            Self::DecreasePosition(args) => {
                writer.write_all(&DECREASE_POSITION_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.shares, &mut writer)?;
                Ok(())
            }
            Self::IncreasePosition(args) => {
                writer.write_all(&INCREASE_POSITION_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.amount_x, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.amount_y, &mut writer)?;
                borsh::BorshSerialize::serialize(
                    &args.liquidity_distribution,
                    &mut writer,
                )?;
                Ok(())
            }
            Self::InitializeBinArray(args) => {
                writer.write_all(&INITIALIZE_BIN_ARRAY_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.id, &mut writer)?;
                Ok(())
            }
            Self::InitializeBinStepConfig(args) => {
                writer.write_all(&INITIALIZE_BIN_STEP_CONFIG_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.bin_step, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.availability, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.fee_parameters, &mut writer)?;
                Ok(())
            }
            Self::InitializeConfig(args) => {
                writer.write_all(&INITIALIZE_CONFIG_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.fee_authority, &mut writer)?;
                Ok(())
            }
            Self::InitializePair(args) => {
                writer.write_all(&INITIALIZE_PAIR_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.active_id, &mut writer)?;
                Ok(())
            }
            Self::InitializeQuoteAssetBadge => {
                writer.write_all(&INITIALIZE_QUOTE_ASSET_BADGE_IX_DISCM)
            }
            Self::SetHook => writer.write_all(&SET_HOOK_IX_DISCM),
            Self::Swap(args) => {
                writer.write_all(&SWAP_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.amount, &mut writer)?;
                borsh::BorshSerialize::serialize(
                    &args.other_amount_threshold,
                    &mut writer,
                )?;
                borsh::BorshSerialize::serialize(&args.swap_for_y, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.swap_type, &mut writer)?;
                Ok(())
            }
            Self::TransferConfigOwnership(args) => {
                writer.write_all(&TRANSFER_CONFIG_OWNERSHIP_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.new_authority, &mut writer)?;
                Ok(())
            }
            Self::UpdateBinStepConfig(args) => {
                writer.write_all(&UPDATE_BIN_STEP_CONFIG_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.status, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.availability, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.fee_parameters, &mut writer)?;
                Ok(())
            }
            Self::UpdatePairStaticFeeParameters(args) => {
                writer.write_all(&UPDATE_PAIR_STATIC_FEE_PARAMETERS_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.fee_parameters, &mut writer)?;
                Ok(())
            }
            Self::UpdateQuoteAssetBadge(args) => {
                writer.write_all(&UPDATE_QUOTE_ASSET_BADGE_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.status, &mut writer)?;
                Ok(())
            }
            Self::WithdrawProtocolFees => {
                writer.write_all(&WITHDRAW_PROTOCOL_FEES_IX_DISCM)
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
pub const ACCEPT_CONFIG_OWNERSHIP_IX_ACCOUNTS_LEN: usize = 4;
#[derive(Copy, Clone, Debug)]
pub struct AcceptConfigOwnershipAccounts<'me, 'info> {
    pub liquidity_book_config: &'me AccountInfo<'info>,
    pub pending_preset_authority: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct AcceptConfigOwnershipKeys {
    pub liquidity_book_config: Pubkey,
    pub pending_preset_authority: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<AcceptConfigOwnershipAccounts<'_, '_>> for AcceptConfigOwnershipKeys {
    fn from(accounts: AcceptConfigOwnershipAccounts) -> Self {
        Self {
            liquidity_book_config: *accounts.liquidity_book_config.key,
            pending_preset_authority: *accounts.pending_preset_authority.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<AcceptConfigOwnershipKeys>
for [AccountMeta; ACCEPT_CONFIG_OWNERSHIP_IX_ACCOUNTS_LEN] {
    fn from(keys: AcceptConfigOwnershipKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.liquidity_book_config,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pending_preset_authority,
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
impl From<[Pubkey; ACCEPT_CONFIG_OWNERSHIP_IX_ACCOUNTS_LEN]>
for AcceptConfigOwnershipKeys {
    fn from(pubkeys: [Pubkey; ACCEPT_CONFIG_OWNERSHIP_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            liquidity_book_config: pubkeys[0],
            pending_preset_authority: pubkeys[1],
            event_authority: pubkeys[2],
            program: pubkeys[3],
        }
    }
}
impl<'info> From<AcceptConfigOwnershipAccounts<'_, 'info>>
for [AccountInfo<'info>; ACCEPT_CONFIG_OWNERSHIP_IX_ACCOUNTS_LEN] {
    fn from(accounts: AcceptConfigOwnershipAccounts<'_, 'info>) -> Self {
        [
            accounts.liquidity_book_config.clone(),
            accounts.pending_preset_authority.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; ACCEPT_CONFIG_OWNERSHIP_IX_ACCOUNTS_LEN]>
for AcceptConfigOwnershipAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; ACCEPT_CONFIG_OWNERSHIP_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            liquidity_book_config: &arr[0],
            pending_preset_authority: &arr[1],
            event_authority: &arr[2],
            program: &arr[3],
        }
    }
}
pub const ACCEPT_CONFIG_OWNERSHIP_IX_DISCM: [u8; 8usize] = [
    6, 212, 14, 48, 229, 38, 62, 241,
];
#[derive(Clone, Debug, PartialEq)]
pub struct AcceptConfigOwnershipIxData;
impl AcceptConfigOwnershipIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != ACCEPT_CONFIG_OWNERSHIP_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&ACCEPT_CONFIG_OWNERSHIP_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn accept_config_ownership_ix_with_program_id(
    program_id: Pubkey,
    keys: AcceptConfigOwnershipKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; ACCEPT_CONFIG_OWNERSHIP_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: AcceptConfigOwnershipIxData.try_to_vec()?,
    })
}
pub fn accept_config_ownership_ix(
    keys: AcceptConfigOwnershipKeys,
) -> std::io::Result<Instruction> {
    accept_config_ownership_ix_with_program_id(LIQUIDITY_BOOK_PROGRAM_ID, keys)
}
pub fn accept_config_ownership_invoke_with_program_id(
    program_id: Pubkey,
    accounts: AcceptConfigOwnershipAccounts<'_, '_>,
) -> ProgramResult {
    let keys: AcceptConfigOwnershipKeys = accounts.into();
    let ix = accept_config_ownership_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn accept_config_ownership_invoke(
    accounts: AcceptConfigOwnershipAccounts<'_, '_>,
) -> ProgramResult {
    accept_config_ownership_invoke_with_program_id(LIQUIDITY_BOOK_PROGRAM_ID, accounts)
}
pub fn accept_config_ownership_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: AcceptConfigOwnershipAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: AcceptConfigOwnershipKeys = accounts.into();
    let ix = accept_config_ownership_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn accept_config_ownership_invoke_signed(
    accounts: AcceptConfigOwnershipAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    accept_config_ownership_invoke_signed_with_program_id(
        LIQUIDITY_BOOK_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn accept_config_ownership_verify_account_keys(
    accounts: AcceptConfigOwnershipAccounts<'_, '_>,
    keys: AcceptConfigOwnershipKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.liquidity_book_config.key, keys.liquidity_book_config),
        (*accounts.pending_preset_authority.key, keys.pending_preset_authority),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn accept_config_ownership_verify_writable_privileges<'me, 'info>(
    accounts: AcceptConfigOwnershipAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.liquidity_book_config] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn accept_config_ownership_verify_signer_privileges<'me, 'info>(
    accounts: AcceptConfigOwnershipAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.pending_preset_authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn accept_config_ownership_verify_account_privileges<'me, 'info>(
    accounts: AcceptConfigOwnershipAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    accept_config_ownership_verify_writable_privileges(accounts)?;
    accept_config_ownership_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CLOSE_POSITION_IX_ACCOUNTS_LEN: usize = 22;
#[derive(Copy, Clone, Debug)]
pub struct ClosePositionAccounts<'me, 'info> {
    pub pair: &'me AccountInfo<'info>,
    pub position: &'me AccountInfo<'info>,
    pub position_mint: &'me AccountInfo<'info>,
    pub position_token_account: &'me AccountInfo<'info>,
    pub bin_array_lower: &'me AccountInfo<'info>,
    pub bin_array_upper: &'me AccountInfo<'info>,
    pub token_mint_x: &'me AccountInfo<'info>,
    pub token_mint_y: &'me AccountInfo<'info>,
    pub token_vault_x: &'me AccountInfo<'info>,
    pub token_vault_y: &'me AccountInfo<'info>,
    pub user_vault_x: &'me AccountInfo<'info>,
    pub user_vault_y: &'me AccountInfo<'info>,
    pub user: &'me AccountInfo<'info>,
    pub token_program_x: &'me AccountInfo<'info>,
    pub token_program_y: &'me AccountInfo<'info>,
    pub position_token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub memo_program: &'me AccountInfo<'info>,
    pub hook: &'me AccountInfo<'info>,
    pub hooks_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ClosePositionKeys {
    pub pair: Pubkey,
    pub position: Pubkey,
    pub position_mint: Pubkey,
    pub position_token_account: Pubkey,
    pub bin_array_lower: Pubkey,
    pub bin_array_upper: Pubkey,
    pub token_mint_x: Pubkey,
    pub token_mint_y: Pubkey,
    pub token_vault_x: Pubkey,
    pub token_vault_y: Pubkey,
    pub user_vault_x: Pubkey,
    pub user_vault_y: Pubkey,
    pub user: Pubkey,
    pub token_program_x: Pubkey,
    pub token_program_y: Pubkey,
    pub position_token_program: Pubkey,
    pub system_program: Pubkey,
    pub memo_program: Pubkey,
    pub hook: Pubkey,
    pub hooks_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<ClosePositionAccounts<'_, '_>> for ClosePositionKeys {
    fn from(accounts: ClosePositionAccounts) -> Self {
        Self {
            pair: *accounts.pair.key,
            position: *accounts.position.key,
            position_mint: *accounts.position_mint.key,
            position_token_account: *accounts.position_token_account.key,
            bin_array_lower: *accounts.bin_array_lower.key,
            bin_array_upper: *accounts.bin_array_upper.key,
            token_mint_x: *accounts.token_mint_x.key,
            token_mint_y: *accounts.token_mint_y.key,
            token_vault_x: *accounts.token_vault_x.key,
            token_vault_y: *accounts.token_vault_y.key,
            user_vault_x: *accounts.user_vault_x.key,
            user_vault_y: *accounts.user_vault_y.key,
            user: *accounts.user.key,
            token_program_x: *accounts.token_program_x.key,
            token_program_y: *accounts.token_program_y.key,
            position_token_program: *accounts.position_token_program.key,
            system_program: *accounts.system_program.key,
            memo_program: *accounts.memo_program.key,
            hook: *accounts.hook.key,
            hooks_program: *accounts.hooks_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<ClosePositionKeys> for [AccountMeta; CLOSE_POSITION_IX_ACCOUNTS_LEN] {
    fn from(keys: ClosePositionKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.pair,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.position,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.position_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.position_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.bin_array_lower,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.bin_array_upper,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_mint_x,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_mint_y,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_vault_x,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_vault_y,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_vault_x,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_vault_y,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_program_x,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_program_y,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.position_token_program,
                is_signer: false,
                is_writable: false,
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
                pubkey: keys.hook,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.hooks_program,
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
impl From<[Pubkey; CLOSE_POSITION_IX_ACCOUNTS_LEN]> for ClosePositionKeys {
    fn from(pubkeys: [Pubkey; CLOSE_POSITION_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            pair: pubkeys[0],
            position: pubkeys[1],
            position_mint: pubkeys[2],
            position_token_account: pubkeys[3],
            bin_array_lower: pubkeys[4],
            bin_array_upper: pubkeys[5],
            token_mint_x: pubkeys[6],
            token_mint_y: pubkeys[7],
            token_vault_x: pubkeys[8],
            token_vault_y: pubkeys[9],
            user_vault_x: pubkeys[10],
            user_vault_y: pubkeys[11],
            user: pubkeys[12],
            token_program_x: pubkeys[13],
            token_program_y: pubkeys[14],
            position_token_program: pubkeys[15],
            system_program: pubkeys[16],
            memo_program: pubkeys[17],
            hook: pubkeys[18],
            hooks_program: pubkeys[19],
            event_authority: pubkeys[20],
            program: pubkeys[21],
        }
    }
}
impl<'info> From<ClosePositionAccounts<'_, 'info>>
for [AccountInfo<'info>; CLOSE_POSITION_IX_ACCOUNTS_LEN] {
    fn from(accounts: ClosePositionAccounts<'_, 'info>) -> Self {
        [
            accounts.pair.clone(),
            accounts.position.clone(),
            accounts.position_mint.clone(),
            accounts.position_token_account.clone(),
            accounts.bin_array_lower.clone(),
            accounts.bin_array_upper.clone(),
            accounts.token_mint_x.clone(),
            accounts.token_mint_y.clone(),
            accounts.token_vault_x.clone(),
            accounts.token_vault_y.clone(),
            accounts.user_vault_x.clone(),
            accounts.user_vault_y.clone(),
            accounts.user.clone(),
            accounts.token_program_x.clone(),
            accounts.token_program_y.clone(),
            accounts.position_token_program.clone(),
            accounts.system_program.clone(),
            accounts.memo_program.clone(),
            accounts.hook.clone(),
            accounts.hooks_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CLOSE_POSITION_IX_ACCOUNTS_LEN]>
for ClosePositionAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; CLOSE_POSITION_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            pair: &arr[0],
            position: &arr[1],
            position_mint: &arr[2],
            position_token_account: &arr[3],
            bin_array_lower: &arr[4],
            bin_array_upper: &arr[5],
            token_mint_x: &arr[6],
            token_mint_y: &arr[7],
            token_vault_x: &arr[8],
            token_vault_y: &arr[9],
            user_vault_x: &arr[10],
            user_vault_y: &arr[11],
            user: &arr[12],
            token_program_x: &arr[13],
            token_program_y: &arr[14],
            position_token_program: &arr[15],
            system_program: &arr[16],
            memo_program: &arr[17],
            hook: &arr[18],
            hooks_program: &arr[19],
            event_authority: &arr[20],
            program: &arr[21],
        }
    }
}
pub const CLOSE_POSITION_IX_DISCM: [u8; 8usize] = [123, 134, 81, 0, 49, 68, 98, 98];
#[derive(Clone, Debug, PartialEq)]
pub struct ClosePositionIxData;
impl ClosePositionIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CLOSE_POSITION_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CLOSE_POSITION_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn close_position_ix_with_program_id(
    program_id: Pubkey,
    keys: ClosePositionKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CLOSE_POSITION_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: ClosePositionIxData.try_to_vec()?,
    })
}
pub fn close_position_ix(keys: ClosePositionKeys) -> std::io::Result<Instruction> {
    close_position_ix_with_program_id(LIQUIDITY_BOOK_PROGRAM_ID, keys)
}
pub fn close_position_invoke_with_program_id(
    program_id: Pubkey,
    accounts: ClosePositionAccounts<'_, '_>,
) -> ProgramResult {
    let keys: ClosePositionKeys = accounts.into();
    let ix = close_position_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn close_position_invoke(accounts: ClosePositionAccounts<'_, '_>) -> ProgramResult {
    close_position_invoke_with_program_id(LIQUIDITY_BOOK_PROGRAM_ID, accounts)
}
pub fn close_position_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: ClosePositionAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: ClosePositionKeys = accounts.into();
    let ix = close_position_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn close_position_invoke_signed(
    accounts: ClosePositionAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    close_position_invoke_signed_with_program_id(
        LIQUIDITY_BOOK_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn close_position_verify_account_keys(
    accounts: ClosePositionAccounts<'_, '_>,
    keys: ClosePositionKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.pair.key, keys.pair),
        (*accounts.position.key, keys.position),
        (*accounts.position_mint.key, keys.position_mint),
        (*accounts.position_token_account.key, keys.position_token_account),
        (*accounts.bin_array_lower.key, keys.bin_array_lower),
        (*accounts.bin_array_upper.key, keys.bin_array_upper),
        (*accounts.token_mint_x.key, keys.token_mint_x),
        (*accounts.token_mint_y.key, keys.token_mint_y),
        (*accounts.token_vault_x.key, keys.token_vault_x),
        (*accounts.token_vault_y.key, keys.token_vault_y),
        (*accounts.user_vault_x.key, keys.user_vault_x),
        (*accounts.user_vault_y.key, keys.user_vault_y),
        (*accounts.user.key, keys.user),
        (*accounts.token_program_x.key, keys.token_program_x),
        (*accounts.token_program_y.key, keys.token_program_y),
        (*accounts.position_token_program.key, keys.position_token_program),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.memo_program.key, keys.memo_program),
        (*accounts.hook.key, keys.hook),
        (*accounts.hooks_program.key, keys.hooks_program),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn close_position_verify_writable_privileges<'me, 'info>(
    accounts: ClosePositionAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.pair,
        accounts.position,
        accounts.position_mint,
        accounts.position_token_account,
        accounts.bin_array_lower,
        accounts.bin_array_upper,
        accounts.token_vault_x,
        accounts.token_vault_y,
        accounts.user_vault_x,
        accounts.user_vault_y,
        accounts.hook,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn close_position_verify_signer_privileges<'me, 'info>(
    accounts: ClosePositionAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.user] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn close_position_verify_account_privileges<'me, 'info>(
    accounts: ClosePositionAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    close_position_verify_writable_privileges(accounts)?;
    close_position_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CREATE_POSITION_IX_ACCOUNTS_LEN: usize = 10;
#[derive(Copy, Clone, Debug)]
pub struct CreatePositionAccounts<'me, 'info> {
    pub pair: &'me AccountInfo<'info>,
    pub position: &'me AccountInfo<'info>,
    pub position_mint: &'me AccountInfo<'info>,
    pub position_token_account: &'me AccountInfo<'info>,
    pub user: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CreatePositionKeys {
    pub pair: Pubkey,
    pub position: Pubkey,
    pub position_mint: Pubkey,
    pub position_token_account: Pubkey,
    pub user: Pubkey,
    pub system_program: Pubkey,
    pub token_program: Pubkey,
    pub associated_token_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<CreatePositionAccounts<'_, '_>> for CreatePositionKeys {
    fn from(accounts: CreatePositionAccounts) -> Self {
        Self {
            pair: *accounts.pair.key,
            position: *accounts.position.key,
            position_mint: *accounts.position_mint.key,
            position_token_account: *accounts.position_token_account.key,
            user: *accounts.user.key,
            system_program: *accounts.system_program.key,
            token_program: *accounts.token_program.key,
            associated_token_program: *accounts.associated_token_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<CreatePositionKeys> for [AccountMeta; CREATE_POSITION_IX_ACCOUNTS_LEN] {
    fn from(keys: CreatePositionKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.pair,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.position,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.position_mint,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.position_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user,
                is_signer: true,
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
impl From<[Pubkey; CREATE_POSITION_IX_ACCOUNTS_LEN]> for CreatePositionKeys {
    fn from(pubkeys: [Pubkey; CREATE_POSITION_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            pair: pubkeys[0],
            position: pubkeys[1],
            position_mint: pubkeys[2],
            position_token_account: pubkeys[3],
            user: pubkeys[4],
            system_program: pubkeys[5],
            token_program: pubkeys[6],
            associated_token_program: pubkeys[7],
            event_authority: pubkeys[8],
            program: pubkeys[9],
        }
    }
}
impl<'info> From<CreatePositionAccounts<'_, 'info>>
for [AccountInfo<'info>; CREATE_POSITION_IX_ACCOUNTS_LEN] {
    fn from(accounts: CreatePositionAccounts<'_, 'info>) -> Self {
        [
            accounts.pair.clone(),
            accounts.position.clone(),
            accounts.position_mint.clone(),
            accounts.position_token_account.clone(),
            accounts.user.clone(),
            accounts.system_program.clone(),
            accounts.token_program.clone(),
            accounts.associated_token_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CREATE_POSITION_IX_ACCOUNTS_LEN]>
for CreatePositionAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; CREATE_POSITION_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            pair: &arr[0],
            position: &arr[1],
            position_mint: &arr[2],
            position_token_account: &arr[3],
            user: &arr[4],
            system_program: &arr[5],
            token_program: &arr[6],
            associated_token_program: &arr[7],
            event_authority: &arr[8],
            program: &arr[9],
        }
    }
}
pub const CREATE_POSITION_IX_DISCM: [u8; 8usize] = [
    48, 215, 197, 153, 96, 203, 180, 133,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CreatePositionIxArgs {
    pub relative_bin_id_left: i32,
    pub relative_bin_in_right: i32,
}
#[derive(Clone, Debug, PartialEq)]
pub struct CreatePositionIxData(pub CreatePositionIxArgs);
impl From<CreatePositionIxArgs> for CreatePositionIxData {
    fn from(args: CreatePositionIxArgs) -> Self {
        Self(args)
    }
}
impl CreatePositionIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CREATE_POSITION_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let relative_bin_id_left: i32 = crate::borsh_de_or_default(&mut reader)?;
        let relative_bin_in_right: i32 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(CreatePositionIxArgs {
                relative_bin_id_left,
                relative_bin_in_right,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CREATE_POSITION_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.relative_bin_id_left, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.relative_bin_in_right, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn create_position_ix_with_program_id(
    program_id: Pubkey,
    keys: CreatePositionKeys,
    args: CreatePositionIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CREATE_POSITION_IX_ACCOUNTS_LEN] = keys.into();
    let data: CreatePositionIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn create_position_ix(
    keys: CreatePositionKeys,
    args: CreatePositionIxArgs,
) -> std::io::Result<Instruction> {
    create_position_ix_with_program_id(LIQUIDITY_BOOK_PROGRAM_ID, keys, args)
}
pub fn create_position_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CreatePositionAccounts<'_, '_>,
    args: CreatePositionIxArgs,
) -> ProgramResult {
    let keys: CreatePositionKeys = accounts.into();
    let ix = create_position_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn create_position_invoke(
    accounts: CreatePositionAccounts<'_, '_>,
    args: CreatePositionIxArgs,
) -> ProgramResult {
    create_position_invoke_with_program_id(LIQUIDITY_BOOK_PROGRAM_ID, accounts, args)
}
pub fn create_position_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CreatePositionAccounts<'_, '_>,
    args: CreatePositionIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CreatePositionKeys = accounts.into();
    let ix = create_position_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn create_position_invoke_signed(
    accounts: CreatePositionAccounts<'_, '_>,
    args: CreatePositionIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    create_position_invoke_signed_with_program_id(
        LIQUIDITY_BOOK_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn create_position_verify_account_keys(
    accounts: CreatePositionAccounts<'_, '_>,
    keys: CreatePositionKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.pair.key, keys.pair),
        (*accounts.position.key, keys.position),
        (*accounts.position_mint.key, keys.position_mint),
        (*accounts.position_token_account.key, keys.position_token_account),
        (*accounts.user.key, keys.user),
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
pub fn create_position_verify_writable_privileges<'me, 'info>(
    accounts: CreatePositionAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.position,
        accounts.position_mint,
        accounts.position_token_account,
        accounts.user,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn create_position_verify_signer_privileges<'me, 'info>(
    accounts: CreatePositionAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.position_mint, accounts.user] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn create_position_verify_account_privileges<'me, 'info>(
    accounts: CreatePositionAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    create_position_verify_writable_privileges(accounts)?;
    create_position_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const DECREASE_POSITION_IX_ACCOUNTS_LEN: usize = 22;
#[derive(Copy, Clone, Debug)]
pub struct DecreasePositionAccounts<'me, 'info> {
    pub pair: &'me AccountInfo<'info>,
    pub position: &'me AccountInfo<'info>,
    pub position_mint: &'me AccountInfo<'info>,
    pub position_token_account: &'me AccountInfo<'info>,
    pub bin_array_lower: &'me AccountInfo<'info>,
    pub bin_array_upper: &'me AccountInfo<'info>,
    pub token_mint_x: &'me AccountInfo<'info>,
    pub token_mint_y: &'me AccountInfo<'info>,
    pub token_vault_x: &'me AccountInfo<'info>,
    pub token_vault_y: &'me AccountInfo<'info>,
    pub user_vault_x: &'me AccountInfo<'info>,
    pub user_vault_y: &'me AccountInfo<'info>,
    pub user: &'me AccountInfo<'info>,
    pub token_program_x: &'me AccountInfo<'info>,
    pub token_program_y: &'me AccountInfo<'info>,
    pub position_token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub memo_program: &'me AccountInfo<'info>,
    pub hook: &'me AccountInfo<'info>,
    pub hooks_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct DecreasePositionKeys {
    pub pair: Pubkey,
    pub position: Pubkey,
    pub position_mint: Pubkey,
    pub position_token_account: Pubkey,
    pub bin_array_lower: Pubkey,
    pub bin_array_upper: Pubkey,
    pub token_mint_x: Pubkey,
    pub token_mint_y: Pubkey,
    pub token_vault_x: Pubkey,
    pub token_vault_y: Pubkey,
    pub user_vault_x: Pubkey,
    pub user_vault_y: Pubkey,
    pub user: Pubkey,
    pub token_program_x: Pubkey,
    pub token_program_y: Pubkey,
    pub position_token_program: Pubkey,
    pub system_program: Pubkey,
    pub memo_program: Pubkey,
    pub hook: Pubkey,
    pub hooks_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<DecreasePositionAccounts<'_, '_>> for DecreasePositionKeys {
    fn from(accounts: DecreasePositionAccounts) -> Self {
        Self {
            pair: *accounts.pair.key,
            position: *accounts.position.key,
            position_mint: *accounts.position_mint.key,
            position_token_account: *accounts.position_token_account.key,
            bin_array_lower: *accounts.bin_array_lower.key,
            bin_array_upper: *accounts.bin_array_upper.key,
            token_mint_x: *accounts.token_mint_x.key,
            token_mint_y: *accounts.token_mint_y.key,
            token_vault_x: *accounts.token_vault_x.key,
            token_vault_y: *accounts.token_vault_y.key,
            user_vault_x: *accounts.user_vault_x.key,
            user_vault_y: *accounts.user_vault_y.key,
            user: *accounts.user.key,
            token_program_x: *accounts.token_program_x.key,
            token_program_y: *accounts.token_program_y.key,
            position_token_program: *accounts.position_token_program.key,
            system_program: *accounts.system_program.key,
            memo_program: *accounts.memo_program.key,
            hook: *accounts.hook.key,
            hooks_program: *accounts.hooks_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<DecreasePositionKeys> for [AccountMeta; DECREASE_POSITION_IX_ACCOUNTS_LEN] {
    fn from(keys: DecreasePositionKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.pair,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.position,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.position_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.position_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.bin_array_lower,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.bin_array_upper,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_mint_x,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_mint_y,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_vault_x,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_vault_y,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_vault_x,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_vault_y,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_program_x,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_program_y,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.position_token_program,
                is_signer: false,
                is_writable: false,
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
                pubkey: keys.hook,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.hooks_program,
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
impl From<[Pubkey; DECREASE_POSITION_IX_ACCOUNTS_LEN]> for DecreasePositionKeys {
    fn from(pubkeys: [Pubkey; DECREASE_POSITION_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            pair: pubkeys[0],
            position: pubkeys[1],
            position_mint: pubkeys[2],
            position_token_account: pubkeys[3],
            bin_array_lower: pubkeys[4],
            bin_array_upper: pubkeys[5],
            token_mint_x: pubkeys[6],
            token_mint_y: pubkeys[7],
            token_vault_x: pubkeys[8],
            token_vault_y: pubkeys[9],
            user_vault_x: pubkeys[10],
            user_vault_y: pubkeys[11],
            user: pubkeys[12],
            token_program_x: pubkeys[13],
            token_program_y: pubkeys[14],
            position_token_program: pubkeys[15],
            system_program: pubkeys[16],
            memo_program: pubkeys[17],
            hook: pubkeys[18],
            hooks_program: pubkeys[19],
            event_authority: pubkeys[20],
            program: pubkeys[21],
        }
    }
}
impl<'info> From<DecreasePositionAccounts<'_, 'info>>
for [AccountInfo<'info>; DECREASE_POSITION_IX_ACCOUNTS_LEN] {
    fn from(accounts: DecreasePositionAccounts<'_, 'info>) -> Self {
        [
            accounts.pair.clone(),
            accounts.position.clone(),
            accounts.position_mint.clone(),
            accounts.position_token_account.clone(),
            accounts.bin_array_lower.clone(),
            accounts.bin_array_upper.clone(),
            accounts.token_mint_x.clone(),
            accounts.token_mint_y.clone(),
            accounts.token_vault_x.clone(),
            accounts.token_vault_y.clone(),
            accounts.user_vault_x.clone(),
            accounts.user_vault_y.clone(),
            accounts.user.clone(),
            accounts.token_program_x.clone(),
            accounts.token_program_y.clone(),
            accounts.position_token_program.clone(),
            accounts.system_program.clone(),
            accounts.memo_program.clone(),
            accounts.hook.clone(),
            accounts.hooks_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; DECREASE_POSITION_IX_ACCOUNTS_LEN]>
for DecreasePositionAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; DECREASE_POSITION_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            pair: &arr[0],
            position: &arr[1],
            position_mint: &arr[2],
            position_token_account: &arr[3],
            bin_array_lower: &arr[4],
            bin_array_upper: &arr[5],
            token_mint_x: &arr[6],
            token_mint_y: &arr[7],
            token_vault_x: &arr[8],
            token_vault_y: &arr[9],
            user_vault_x: &arr[10],
            user_vault_y: &arr[11],
            user: &arr[12],
            token_program_x: &arr[13],
            token_program_y: &arr[14],
            position_token_program: &arr[15],
            system_program: &arr[16],
            memo_program: &arr[17],
            hook: &arr[18],
            hooks_program: &arr[19],
            event_authority: &arr[20],
            program: &arr[21],
        }
    }
}
pub const DECREASE_POSITION_IX_DISCM: [u8; 8usize] = [
    57, 125, 21, 59, 200, 137, 179, 108,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DecreasePositionIxArgs {
    pub shares: Vec<u128>,
}
#[derive(Clone, Debug, PartialEq)]
pub struct DecreasePositionIxData(pub DecreasePositionIxArgs);
impl From<DecreasePositionIxArgs> for DecreasePositionIxData {
    fn from(args: DecreasePositionIxArgs) -> Self {
        Self(args)
    }
}
impl DecreasePositionIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != DECREASE_POSITION_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let shares: Vec<u128> = crate::borsh_de_or_default(&mut reader)?;
        Ok(Self(DecreasePositionIxArgs { shares }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&DECREASE_POSITION_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.shares, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn decrease_position_ix_with_program_id(
    program_id: Pubkey,
    keys: DecreasePositionKeys,
    args: DecreasePositionIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; DECREASE_POSITION_IX_ACCOUNTS_LEN] = keys.into();
    let data: DecreasePositionIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn decrease_position_ix(
    keys: DecreasePositionKeys,
    args: DecreasePositionIxArgs,
) -> std::io::Result<Instruction> {
    decrease_position_ix_with_program_id(LIQUIDITY_BOOK_PROGRAM_ID, keys, args)
}
pub fn decrease_position_invoke_with_program_id(
    program_id: Pubkey,
    accounts: DecreasePositionAccounts<'_, '_>,
    args: DecreasePositionIxArgs,
) -> ProgramResult {
    let keys: DecreasePositionKeys = accounts.into();
    let ix = decrease_position_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn decrease_position_invoke(
    accounts: DecreasePositionAccounts<'_, '_>,
    args: DecreasePositionIxArgs,
) -> ProgramResult {
    decrease_position_invoke_with_program_id(LIQUIDITY_BOOK_PROGRAM_ID, accounts, args)
}
pub fn decrease_position_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: DecreasePositionAccounts<'_, '_>,
    args: DecreasePositionIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: DecreasePositionKeys = accounts.into();
    let ix = decrease_position_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn decrease_position_invoke_signed(
    accounts: DecreasePositionAccounts<'_, '_>,
    args: DecreasePositionIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    decrease_position_invoke_signed_with_program_id(
        LIQUIDITY_BOOK_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn decrease_position_verify_account_keys(
    accounts: DecreasePositionAccounts<'_, '_>,
    keys: DecreasePositionKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.pair.key, keys.pair),
        (*accounts.position.key, keys.position),
        (*accounts.position_mint.key, keys.position_mint),
        (*accounts.position_token_account.key, keys.position_token_account),
        (*accounts.bin_array_lower.key, keys.bin_array_lower),
        (*accounts.bin_array_upper.key, keys.bin_array_upper),
        (*accounts.token_mint_x.key, keys.token_mint_x),
        (*accounts.token_mint_y.key, keys.token_mint_y),
        (*accounts.token_vault_x.key, keys.token_vault_x),
        (*accounts.token_vault_y.key, keys.token_vault_y),
        (*accounts.user_vault_x.key, keys.user_vault_x),
        (*accounts.user_vault_y.key, keys.user_vault_y),
        (*accounts.user.key, keys.user),
        (*accounts.token_program_x.key, keys.token_program_x),
        (*accounts.token_program_y.key, keys.token_program_y),
        (*accounts.position_token_program.key, keys.position_token_program),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.memo_program.key, keys.memo_program),
        (*accounts.hook.key, keys.hook),
        (*accounts.hooks_program.key, keys.hooks_program),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn decrease_position_verify_writable_privileges<'me, 'info>(
    accounts: DecreasePositionAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.pair,
        accounts.position,
        accounts.position_mint,
        accounts.position_token_account,
        accounts.bin_array_lower,
        accounts.bin_array_upper,
        accounts.token_vault_x,
        accounts.token_vault_y,
        accounts.user_vault_x,
        accounts.user_vault_y,
        accounts.hook,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn decrease_position_verify_signer_privileges<'me, 'info>(
    accounts: DecreasePositionAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.user] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn decrease_position_verify_account_privileges<'me, 'info>(
    accounts: DecreasePositionAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    decrease_position_verify_writable_privileges(accounts)?;
    decrease_position_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const INCREASE_POSITION_IX_ACCOUNTS_LEN: usize = 22;
#[derive(Copy, Clone, Debug)]
pub struct IncreasePositionAccounts<'me, 'info> {
    pub pair: &'me AccountInfo<'info>,
    pub position: &'me AccountInfo<'info>,
    pub position_mint: &'me AccountInfo<'info>,
    pub position_token_account: &'me AccountInfo<'info>,
    pub bin_array_lower: &'me AccountInfo<'info>,
    pub bin_array_upper: &'me AccountInfo<'info>,
    pub token_mint_x: &'me AccountInfo<'info>,
    pub token_mint_y: &'me AccountInfo<'info>,
    pub token_vault_x: &'me AccountInfo<'info>,
    pub token_vault_y: &'me AccountInfo<'info>,
    pub user_vault_x: &'me AccountInfo<'info>,
    pub user_vault_y: &'me AccountInfo<'info>,
    pub user: &'me AccountInfo<'info>,
    pub token_program_x: &'me AccountInfo<'info>,
    pub token_program_y: &'me AccountInfo<'info>,
    pub position_token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub memo_program: &'me AccountInfo<'info>,
    pub hook: &'me AccountInfo<'info>,
    pub hooks_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct IncreasePositionKeys {
    pub pair: Pubkey,
    pub position: Pubkey,
    pub position_mint: Pubkey,
    pub position_token_account: Pubkey,
    pub bin_array_lower: Pubkey,
    pub bin_array_upper: Pubkey,
    pub token_mint_x: Pubkey,
    pub token_mint_y: Pubkey,
    pub token_vault_x: Pubkey,
    pub token_vault_y: Pubkey,
    pub user_vault_x: Pubkey,
    pub user_vault_y: Pubkey,
    pub user: Pubkey,
    pub token_program_x: Pubkey,
    pub token_program_y: Pubkey,
    pub position_token_program: Pubkey,
    pub system_program: Pubkey,
    pub memo_program: Pubkey,
    pub hook: Pubkey,
    pub hooks_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<IncreasePositionAccounts<'_, '_>> for IncreasePositionKeys {
    fn from(accounts: IncreasePositionAccounts) -> Self {
        Self {
            pair: *accounts.pair.key,
            position: *accounts.position.key,
            position_mint: *accounts.position_mint.key,
            position_token_account: *accounts.position_token_account.key,
            bin_array_lower: *accounts.bin_array_lower.key,
            bin_array_upper: *accounts.bin_array_upper.key,
            token_mint_x: *accounts.token_mint_x.key,
            token_mint_y: *accounts.token_mint_y.key,
            token_vault_x: *accounts.token_vault_x.key,
            token_vault_y: *accounts.token_vault_y.key,
            user_vault_x: *accounts.user_vault_x.key,
            user_vault_y: *accounts.user_vault_y.key,
            user: *accounts.user.key,
            token_program_x: *accounts.token_program_x.key,
            token_program_y: *accounts.token_program_y.key,
            position_token_program: *accounts.position_token_program.key,
            system_program: *accounts.system_program.key,
            memo_program: *accounts.memo_program.key,
            hook: *accounts.hook.key,
            hooks_program: *accounts.hooks_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<IncreasePositionKeys> for [AccountMeta; INCREASE_POSITION_IX_ACCOUNTS_LEN] {
    fn from(keys: IncreasePositionKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.pair,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.position,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.position_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.position_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.bin_array_lower,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.bin_array_upper,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_mint_x,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_mint_y,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_vault_x,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_vault_y,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_vault_x,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_vault_y,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_program_x,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_program_y,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.position_token_program,
                is_signer: false,
                is_writable: false,
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
                pubkey: keys.hook,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.hooks_program,
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
impl From<[Pubkey; INCREASE_POSITION_IX_ACCOUNTS_LEN]> for IncreasePositionKeys {
    fn from(pubkeys: [Pubkey; INCREASE_POSITION_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            pair: pubkeys[0],
            position: pubkeys[1],
            position_mint: pubkeys[2],
            position_token_account: pubkeys[3],
            bin_array_lower: pubkeys[4],
            bin_array_upper: pubkeys[5],
            token_mint_x: pubkeys[6],
            token_mint_y: pubkeys[7],
            token_vault_x: pubkeys[8],
            token_vault_y: pubkeys[9],
            user_vault_x: pubkeys[10],
            user_vault_y: pubkeys[11],
            user: pubkeys[12],
            token_program_x: pubkeys[13],
            token_program_y: pubkeys[14],
            position_token_program: pubkeys[15],
            system_program: pubkeys[16],
            memo_program: pubkeys[17],
            hook: pubkeys[18],
            hooks_program: pubkeys[19],
            event_authority: pubkeys[20],
            program: pubkeys[21],
        }
    }
}
impl<'info> From<IncreasePositionAccounts<'_, 'info>>
for [AccountInfo<'info>; INCREASE_POSITION_IX_ACCOUNTS_LEN] {
    fn from(accounts: IncreasePositionAccounts<'_, 'info>) -> Self {
        [
            accounts.pair.clone(),
            accounts.position.clone(),
            accounts.position_mint.clone(),
            accounts.position_token_account.clone(),
            accounts.bin_array_lower.clone(),
            accounts.bin_array_upper.clone(),
            accounts.token_mint_x.clone(),
            accounts.token_mint_y.clone(),
            accounts.token_vault_x.clone(),
            accounts.token_vault_y.clone(),
            accounts.user_vault_x.clone(),
            accounts.user_vault_y.clone(),
            accounts.user.clone(),
            accounts.token_program_x.clone(),
            accounts.token_program_y.clone(),
            accounts.position_token_program.clone(),
            accounts.system_program.clone(),
            accounts.memo_program.clone(),
            accounts.hook.clone(),
            accounts.hooks_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; INCREASE_POSITION_IX_ACCOUNTS_LEN]>
for IncreasePositionAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; INCREASE_POSITION_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            pair: &arr[0],
            position: &arr[1],
            position_mint: &arr[2],
            position_token_account: &arr[3],
            bin_array_lower: &arr[4],
            bin_array_upper: &arr[5],
            token_mint_x: &arr[6],
            token_mint_y: &arr[7],
            token_vault_x: &arr[8],
            token_vault_y: &arr[9],
            user_vault_x: &arr[10],
            user_vault_y: &arr[11],
            user: &arr[12],
            token_program_x: &arr[13],
            token_program_y: &arr[14],
            position_token_program: &arr[15],
            system_program: &arr[16],
            memo_program: &arr[17],
            hook: &arr[18],
            hooks_program: &arr[19],
            event_authority: &arr[20],
            program: &arr[21],
        }
    }
}
pub const INCREASE_POSITION_IX_DISCM: [u8; 8usize] = [
    253, 234, 128, 104, 192, 188, 45, 91,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct IncreasePositionIxArgs {
    pub amount_x: u64,
    pub amount_y: u64,
    pub liquidity_distribution: Vec<BinLiquidityDistribution>,
}
#[derive(Clone, Debug, PartialEq)]
pub struct IncreasePositionIxData(pub IncreasePositionIxArgs);
impl From<IncreasePositionIxArgs> for IncreasePositionIxData {
    fn from(args: IncreasePositionIxArgs) -> Self {
        Self(args)
    }
}
impl IncreasePositionIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INCREASE_POSITION_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let amount_x: u64 = crate::borsh_de_or_default(&mut reader)?;
        let amount_y: u64 = crate::borsh_de_or_default(&mut reader)?;
        let liquidity_distribution: Vec<BinLiquidityDistribution> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        Ok(
            Self(IncreasePositionIxArgs {
                amount_x,
                amount_y,
                liquidity_distribution,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INCREASE_POSITION_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.amount_x, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.amount_y, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.liquidity_distribution, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn increase_position_ix_with_program_id(
    program_id: Pubkey,
    keys: IncreasePositionKeys,
    args: IncreasePositionIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; INCREASE_POSITION_IX_ACCOUNTS_LEN] = keys.into();
    let data: IncreasePositionIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn increase_position_ix(
    keys: IncreasePositionKeys,
    args: IncreasePositionIxArgs,
) -> std::io::Result<Instruction> {
    increase_position_ix_with_program_id(LIQUIDITY_BOOK_PROGRAM_ID, keys, args)
}
pub fn increase_position_invoke_with_program_id(
    program_id: Pubkey,
    accounts: IncreasePositionAccounts<'_, '_>,
    args: IncreasePositionIxArgs,
) -> ProgramResult {
    let keys: IncreasePositionKeys = accounts.into();
    let ix = increase_position_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn increase_position_invoke(
    accounts: IncreasePositionAccounts<'_, '_>,
    args: IncreasePositionIxArgs,
) -> ProgramResult {
    increase_position_invoke_with_program_id(LIQUIDITY_BOOK_PROGRAM_ID, accounts, args)
}
pub fn increase_position_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: IncreasePositionAccounts<'_, '_>,
    args: IncreasePositionIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: IncreasePositionKeys = accounts.into();
    let ix = increase_position_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn increase_position_invoke_signed(
    accounts: IncreasePositionAccounts<'_, '_>,
    args: IncreasePositionIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    increase_position_invoke_signed_with_program_id(
        LIQUIDITY_BOOK_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn increase_position_verify_account_keys(
    accounts: IncreasePositionAccounts<'_, '_>,
    keys: IncreasePositionKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.pair.key, keys.pair),
        (*accounts.position.key, keys.position),
        (*accounts.position_mint.key, keys.position_mint),
        (*accounts.position_token_account.key, keys.position_token_account),
        (*accounts.bin_array_lower.key, keys.bin_array_lower),
        (*accounts.bin_array_upper.key, keys.bin_array_upper),
        (*accounts.token_mint_x.key, keys.token_mint_x),
        (*accounts.token_mint_y.key, keys.token_mint_y),
        (*accounts.token_vault_x.key, keys.token_vault_x),
        (*accounts.token_vault_y.key, keys.token_vault_y),
        (*accounts.user_vault_x.key, keys.user_vault_x),
        (*accounts.user_vault_y.key, keys.user_vault_y),
        (*accounts.user.key, keys.user),
        (*accounts.token_program_x.key, keys.token_program_x),
        (*accounts.token_program_y.key, keys.token_program_y),
        (*accounts.position_token_program.key, keys.position_token_program),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.memo_program.key, keys.memo_program),
        (*accounts.hook.key, keys.hook),
        (*accounts.hooks_program.key, keys.hooks_program),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn increase_position_verify_writable_privileges<'me, 'info>(
    accounts: IncreasePositionAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.pair,
        accounts.position,
        accounts.position_mint,
        accounts.position_token_account,
        accounts.bin_array_lower,
        accounts.bin_array_upper,
        accounts.token_vault_x,
        accounts.token_vault_y,
        accounts.user_vault_x,
        accounts.user_vault_y,
        accounts.hook,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn increase_position_verify_signer_privileges<'me, 'info>(
    accounts: IncreasePositionAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.user] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn increase_position_verify_account_privileges<'me, 'info>(
    accounts: IncreasePositionAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    increase_position_verify_writable_privileges(accounts)?;
    increase_position_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const INITIALIZE_BIN_ARRAY_IX_ACCOUNTS_LEN: usize = 6;
#[derive(Copy, Clone, Debug)]
pub struct InitializeBinArrayAccounts<'me, 'info> {
    pub pair: &'me AccountInfo<'info>,
    pub bin_array: &'me AccountInfo<'info>,
    pub user: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct InitializeBinArrayKeys {
    pub pair: Pubkey,
    pub bin_array: Pubkey,
    pub user: Pubkey,
    pub system_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<InitializeBinArrayAccounts<'_, '_>> for InitializeBinArrayKeys {
    fn from(accounts: InitializeBinArrayAccounts) -> Self {
        Self {
            pair: *accounts.pair.key,
            bin_array: *accounts.bin_array.key,
            user: *accounts.user.key,
            system_program: *accounts.system_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<InitializeBinArrayKeys>
for [AccountMeta; INITIALIZE_BIN_ARRAY_IX_ACCOUNTS_LEN] {
    fn from(keys: InitializeBinArrayKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.pair,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.bin_array,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user,
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
impl From<[Pubkey; INITIALIZE_BIN_ARRAY_IX_ACCOUNTS_LEN]> for InitializeBinArrayKeys {
    fn from(pubkeys: [Pubkey; INITIALIZE_BIN_ARRAY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            pair: pubkeys[0],
            bin_array: pubkeys[1],
            user: pubkeys[2],
            system_program: pubkeys[3],
            event_authority: pubkeys[4],
            program: pubkeys[5],
        }
    }
}
impl<'info> From<InitializeBinArrayAccounts<'_, 'info>>
for [AccountInfo<'info>; INITIALIZE_BIN_ARRAY_IX_ACCOUNTS_LEN] {
    fn from(accounts: InitializeBinArrayAccounts<'_, 'info>) -> Self {
        [
            accounts.pair.clone(),
            accounts.bin_array.clone(),
            accounts.user.clone(),
            accounts.system_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; INITIALIZE_BIN_ARRAY_IX_ACCOUNTS_LEN]>
for InitializeBinArrayAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; INITIALIZE_BIN_ARRAY_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            pair: &arr[0],
            bin_array: &arr[1],
            user: &arr[2],
            system_program: &arr[3],
            event_authority: &arr[4],
            program: &arr[5],
        }
    }
}
pub const INITIALIZE_BIN_ARRAY_IX_DISCM: [u8; 8usize] = [
    35, 86, 19, 185, 78, 212, 75, 211,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct InitializeBinArrayIxArgs {
    pub id: u32,
}
#[derive(Clone, Debug, PartialEq)]
pub struct InitializeBinArrayIxData(pub InitializeBinArrayIxArgs);
impl From<InitializeBinArrayIxArgs> for InitializeBinArrayIxData {
    fn from(args: InitializeBinArrayIxArgs) -> Self {
        Self(args)
    }
}
impl InitializeBinArrayIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INITIALIZE_BIN_ARRAY_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let id: u32 = crate::borsh_de_or_default(&mut reader)?;
        Ok(Self(InitializeBinArrayIxArgs { id }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INITIALIZE_BIN_ARRAY_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.id, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn initialize_bin_array_ix_with_program_id(
    program_id: Pubkey,
    keys: InitializeBinArrayKeys,
    args: InitializeBinArrayIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; INITIALIZE_BIN_ARRAY_IX_ACCOUNTS_LEN] = keys.into();
    let data: InitializeBinArrayIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn initialize_bin_array_ix(
    keys: InitializeBinArrayKeys,
    args: InitializeBinArrayIxArgs,
) -> std::io::Result<Instruction> {
    initialize_bin_array_ix_with_program_id(LIQUIDITY_BOOK_PROGRAM_ID, keys, args)
}
pub fn initialize_bin_array_invoke_with_program_id(
    program_id: Pubkey,
    accounts: InitializeBinArrayAccounts<'_, '_>,
    args: InitializeBinArrayIxArgs,
) -> ProgramResult {
    let keys: InitializeBinArrayKeys = accounts.into();
    let ix = initialize_bin_array_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn initialize_bin_array_invoke(
    accounts: InitializeBinArrayAccounts<'_, '_>,
    args: InitializeBinArrayIxArgs,
) -> ProgramResult {
    initialize_bin_array_invoke_with_program_id(
        LIQUIDITY_BOOK_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn initialize_bin_array_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: InitializeBinArrayAccounts<'_, '_>,
    args: InitializeBinArrayIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: InitializeBinArrayKeys = accounts.into();
    let ix = initialize_bin_array_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn initialize_bin_array_invoke_signed(
    accounts: InitializeBinArrayAccounts<'_, '_>,
    args: InitializeBinArrayIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    initialize_bin_array_invoke_signed_with_program_id(
        LIQUIDITY_BOOK_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn initialize_bin_array_verify_account_keys(
    accounts: InitializeBinArrayAccounts<'_, '_>,
    keys: InitializeBinArrayKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.pair.key, keys.pair),
        (*accounts.bin_array.key, keys.bin_array),
        (*accounts.user.key, keys.user),
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
pub fn initialize_bin_array_verify_writable_privileges<'me, 'info>(
    accounts: InitializeBinArrayAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.bin_array, accounts.user] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn initialize_bin_array_verify_signer_privileges<'me, 'info>(
    accounts: InitializeBinArrayAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.user] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn initialize_bin_array_verify_account_privileges<'me, 'info>(
    accounts: InitializeBinArrayAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    initialize_bin_array_verify_writable_privileges(accounts)?;
    initialize_bin_array_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const INITIALIZE_BIN_STEP_CONFIG_IX_ACCOUNTS_LEN: usize = 6;
#[derive(Copy, Clone, Debug)]
pub struct InitializeBinStepConfigAccounts<'me, 'info> {
    pub liquidity_book_config: &'me AccountInfo<'info>,
    pub bin_step_config: &'me AccountInfo<'info>,
    pub preset_authority: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct InitializeBinStepConfigKeys {
    pub liquidity_book_config: Pubkey,
    pub bin_step_config: Pubkey,
    pub preset_authority: Pubkey,
    pub system_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<InitializeBinStepConfigAccounts<'_, '_>> for InitializeBinStepConfigKeys {
    fn from(accounts: InitializeBinStepConfigAccounts) -> Self {
        Self {
            liquidity_book_config: *accounts.liquidity_book_config.key,
            bin_step_config: *accounts.bin_step_config.key,
            preset_authority: *accounts.preset_authority.key,
            system_program: *accounts.system_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<InitializeBinStepConfigKeys>
for [AccountMeta; INITIALIZE_BIN_STEP_CONFIG_IX_ACCOUNTS_LEN] {
    fn from(keys: InitializeBinStepConfigKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.liquidity_book_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.bin_step_config,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.preset_authority,
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
impl From<[Pubkey; INITIALIZE_BIN_STEP_CONFIG_IX_ACCOUNTS_LEN]>
for InitializeBinStepConfigKeys {
    fn from(pubkeys: [Pubkey; INITIALIZE_BIN_STEP_CONFIG_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            liquidity_book_config: pubkeys[0],
            bin_step_config: pubkeys[1],
            preset_authority: pubkeys[2],
            system_program: pubkeys[3],
            event_authority: pubkeys[4],
            program: pubkeys[5],
        }
    }
}
impl<'info> From<InitializeBinStepConfigAccounts<'_, 'info>>
for [AccountInfo<'info>; INITIALIZE_BIN_STEP_CONFIG_IX_ACCOUNTS_LEN] {
    fn from(accounts: InitializeBinStepConfigAccounts<'_, 'info>) -> Self {
        [
            accounts.liquidity_book_config.clone(),
            accounts.bin_step_config.clone(),
            accounts.preset_authority.clone(),
            accounts.system_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; INITIALIZE_BIN_STEP_CONFIG_IX_ACCOUNTS_LEN]>
for InitializeBinStepConfigAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; INITIALIZE_BIN_STEP_CONFIG_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            liquidity_book_config: &arr[0],
            bin_step_config: &arr[1],
            preset_authority: &arr[2],
            system_program: &arr[3],
            event_authority: &arr[4],
            program: &arr[5],
        }
    }
}
pub const INITIALIZE_BIN_STEP_CONFIG_IX_DISCM: [u8; 8usize] = [
    2, 168, 136, 251, 163, 9, 132, 255,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct InitializeBinStepConfigIxArgs {
    pub bin_step: u8,
    pub availability: ConfigAvailability,
    pub fee_parameters: StaticFeeParameters,
}
#[derive(Clone, Debug, PartialEq)]
pub struct InitializeBinStepConfigIxData(pub InitializeBinStepConfigIxArgs);
impl From<InitializeBinStepConfigIxArgs> for InitializeBinStepConfigIxData {
    fn from(args: InitializeBinStepConfigIxArgs) -> Self {
        Self(args)
    }
}
impl InitializeBinStepConfigIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INITIALIZE_BIN_STEP_CONFIG_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let bin_step: u8 = crate::borsh_de_or_default(&mut reader)?;
        let availability: ConfigAvailability = crate::borsh_de_or_default(&mut reader)?;
        let fee_parameters = if reader.is_empty() {
            Default::default()
        } else {
            <StaticFeeParameters>::deserialize(&mut reader)?
        };
        Ok(
            Self(InitializeBinStepConfigIxArgs {
                bin_step,
                availability,
                fee_parameters,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INITIALIZE_BIN_STEP_CONFIG_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.bin_step, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.availability, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.fee_parameters, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn initialize_bin_step_config_ix_with_program_id(
    program_id: Pubkey,
    keys: InitializeBinStepConfigKeys,
    args: InitializeBinStepConfigIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; INITIALIZE_BIN_STEP_CONFIG_IX_ACCOUNTS_LEN] = keys.into();
    let data: InitializeBinStepConfigIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn initialize_bin_step_config_ix(
    keys: InitializeBinStepConfigKeys,
    args: InitializeBinStepConfigIxArgs,
) -> std::io::Result<Instruction> {
    initialize_bin_step_config_ix_with_program_id(LIQUIDITY_BOOK_PROGRAM_ID, keys, args)
}
pub fn initialize_bin_step_config_invoke_with_program_id(
    program_id: Pubkey,
    accounts: InitializeBinStepConfigAccounts<'_, '_>,
    args: InitializeBinStepConfigIxArgs,
) -> ProgramResult {
    let keys: InitializeBinStepConfigKeys = accounts.into();
    let ix = initialize_bin_step_config_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn initialize_bin_step_config_invoke(
    accounts: InitializeBinStepConfigAccounts<'_, '_>,
    args: InitializeBinStepConfigIxArgs,
) -> ProgramResult {
    initialize_bin_step_config_invoke_with_program_id(
        LIQUIDITY_BOOK_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn initialize_bin_step_config_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: InitializeBinStepConfigAccounts<'_, '_>,
    args: InitializeBinStepConfigIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: InitializeBinStepConfigKeys = accounts.into();
    let ix = initialize_bin_step_config_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn initialize_bin_step_config_invoke_signed(
    accounts: InitializeBinStepConfigAccounts<'_, '_>,
    args: InitializeBinStepConfigIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    initialize_bin_step_config_invoke_signed_with_program_id(
        LIQUIDITY_BOOK_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn initialize_bin_step_config_verify_account_keys(
    accounts: InitializeBinStepConfigAccounts<'_, '_>,
    keys: InitializeBinStepConfigKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.liquidity_book_config.key, keys.liquidity_book_config),
        (*accounts.bin_step_config.key, keys.bin_step_config),
        (*accounts.preset_authority.key, keys.preset_authority),
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
pub fn initialize_bin_step_config_verify_writable_privileges<'me, 'info>(
    accounts: InitializeBinStepConfigAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.bin_step_config, accounts.preset_authority] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn initialize_bin_step_config_verify_signer_privileges<'me, 'info>(
    accounts: InitializeBinStepConfigAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.preset_authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn initialize_bin_step_config_verify_account_privileges<'me, 'info>(
    accounts: InitializeBinStepConfigAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    initialize_bin_step_config_verify_writable_privileges(accounts)?;
    initialize_bin_step_config_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const INITIALIZE_CONFIG_IX_ACCOUNTS_LEN: usize = 5;
#[derive(Copy, Clone, Debug)]
pub struct InitializeConfigAccounts<'me, 'info> {
    pub config: &'me AccountInfo<'info>,
    pub payer: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct InitializeConfigKeys {
    pub config: Pubkey,
    pub payer: Pubkey,
    pub system_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<InitializeConfigAccounts<'_, '_>> for InitializeConfigKeys {
    fn from(accounts: InitializeConfigAccounts) -> Self {
        Self {
            config: *accounts.config.key,
            payer: *accounts.payer.key,
            system_program: *accounts.system_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<InitializeConfigKeys> for [AccountMeta; INITIALIZE_CONFIG_IX_ACCOUNTS_LEN] {
    fn from(keys: InitializeConfigKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.config,
                is_signer: true,
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
impl From<[Pubkey; INITIALIZE_CONFIG_IX_ACCOUNTS_LEN]> for InitializeConfigKeys {
    fn from(pubkeys: [Pubkey; INITIALIZE_CONFIG_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            config: pubkeys[0],
            payer: pubkeys[1],
            system_program: pubkeys[2],
            event_authority: pubkeys[3],
            program: pubkeys[4],
        }
    }
}
impl<'info> From<InitializeConfigAccounts<'_, 'info>>
for [AccountInfo<'info>; INITIALIZE_CONFIG_IX_ACCOUNTS_LEN] {
    fn from(accounts: InitializeConfigAccounts<'_, 'info>) -> Self {
        [
            accounts.config.clone(),
            accounts.payer.clone(),
            accounts.system_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; INITIALIZE_CONFIG_IX_ACCOUNTS_LEN]>
for InitializeConfigAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; INITIALIZE_CONFIG_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            config: &arr[0],
            payer: &arr[1],
            system_program: &arr[2],
            event_authority: &arr[3],
            program: &arr[4],
        }
    }
}
pub const INITIALIZE_CONFIG_IX_DISCM: [u8; 8usize] = [
    208, 127, 21, 1, 194, 190, 196, 70,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct InitializeConfigIxArgs {
    pub fee_authority: Pubkey,
}
#[derive(Clone, Debug, PartialEq)]
pub struct InitializeConfigIxData(pub InitializeConfigIxArgs);
impl From<InitializeConfigIxArgs> for InitializeConfigIxData {
    fn from(args: InitializeConfigIxArgs) -> Self {
        Self(args)
    }
}
impl InitializeConfigIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INITIALIZE_CONFIG_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let fee_authority: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(InitializeConfigIxArgs {
                fee_authority,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INITIALIZE_CONFIG_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.fee_authority, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn initialize_config_ix_with_program_id(
    program_id: Pubkey,
    keys: InitializeConfigKeys,
    args: InitializeConfigIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; INITIALIZE_CONFIG_IX_ACCOUNTS_LEN] = keys.into();
    let data: InitializeConfigIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn initialize_config_ix(
    keys: InitializeConfigKeys,
    args: InitializeConfigIxArgs,
) -> std::io::Result<Instruction> {
    initialize_config_ix_with_program_id(LIQUIDITY_BOOK_PROGRAM_ID, keys, args)
}
pub fn initialize_config_invoke_with_program_id(
    program_id: Pubkey,
    accounts: InitializeConfigAccounts<'_, '_>,
    args: InitializeConfigIxArgs,
) -> ProgramResult {
    let keys: InitializeConfigKeys = accounts.into();
    let ix = initialize_config_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn initialize_config_invoke(
    accounts: InitializeConfigAccounts<'_, '_>,
    args: InitializeConfigIxArgs,
) -> ProgramResult {
    initialize_config_invoke_with_program_id(LIQUIDITY_BOOK_PROGRAM_ID, accounts, args)
}
pub fn initialize_config_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: InitializeConfigAccounts<'_, '_>,
    args: InitializeConfigIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: InitializeConfigKeys = accounts.into();
    let ix = initialize_config_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn initialize_config_invoke_signed(
    accounts: InitializeConfigAccounts<'_, '_>,
    args: InitializeConfigIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    initialize_config_invoke_signed_with_program_id(
        LIQUIDITY_BOOK_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn initialize_config_verify_account_keys(
    accounts: InitializeConfigAccounts<'_, '_>,
    keys: InitializeConfigKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.config.key, keys.config),
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
pub fn initialize_config_verify_writable_privileges<'me, 'info>(
    accounts: InitializeConfigAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.config, accounts.payer] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn initialize_config_verify_signer_privileges<'me, 'info>(
    accounts: InitializeConfigAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.config, accounts.payer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn initialize_config_verify_account_privileges<'me, 'info>(
    accounts: InitializeConfigAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    initialize_config_verify_writable_privileges(accounts)?;
    initialize_config_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const INITIALIZE_PAIR_IX_ACCOUNTS_LEN: usize = 10;
#[derive(Copy, Clone, Debug)]
pub struct InitializePairAccounts<'me, 'info> {
    pub liquidity_book_config: &'me AccountInfo<'info>,
    pub token_mint_x: &'me AccountInfo<'info>,
    pub token_mint_y: &'me AccountInfo<'info>,
    pub bin_step_config: &'me AccountInfo<'info>,
    pub quote_asset_badge: &'me AccountInfo<'info>,
    pub pair: &'me AccountInfo<'info>,
    pub user: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct InitializePairKeys {
    pub liquidity_book_config: Pubkey,
    pub token_mint_x: Pubkey,
    pub token_mint_y: Pubkey,
    pub bin_step_config: Pubkey,
    pub quote_asset_badge: Pubkey,
    pub pair: Pubkey,
    pub user: Pubkey,
    pub system_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<InitializePairAccounts<'_, '_>> for InitializePairKeys {
    fn from(accounts: InitializePairAccounts) -> Self {
        Self {
            liquidity_book_config: *accounts.liquidity_book_config.key,
            token_mint_x: *accounts.token_mint_x.key,
            token_mint_y: *accounts.token_mint_y.key,
            bin_step_config: *accounts.bin_step_config.key,
            quote_asset_badge: *accounts.quote_asset_badge.key,
            pair: *accounts.pair.key,
            user: *accounts.user.key,
            system_program: *accounts.system_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<InitializePairKeys> for [AccountMeta; INITIALIZE_PAIR_IX_ACCOUNTS_LEN] {
    fn from(keys: InitializePairKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.liquidity_book_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_mint_x,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_mint_y,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.bin_step_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.quote_asset_badge,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pair,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user,
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
impl From<[Pubkey; INITIALIZE_PAIR_IX_ACCOUNTS_LEN]> for InitializePairKeys {
    fn from(pubkeys: [Pubkey; INITIALIZE_PAIR_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            liquidity_book_config: pubkeys[0],
            token_mint_x: pubkeys[1],
            token_mint_y: pubkeys[2],
            bin_step_config: pubkeys[3],
            quote_asset_badge: pubkeys[4],
            pair: pubkeys[5],
            user: pubkeys[6],
            system_program: pubkeys[7],
            event_authority: pubkeys[8],
            program: pubkeys[9],
        }
    }
}
impl<'info> From<InitializePairAccounts<'_, 'info>>
for [AccountInfo<'info>; INITIALIZE_PAIR_IX_ACCOUNTS_LEN] {
    fn from(accounts: InitializePairAccounts<'_, 'info>) -> Self {
        [
            accounts.liquidity_book_config.clone(),
            accounts.token_mint_x.clone(),
            accounts.token_mint_y.clone(),
            accounts.bin_step_config.clone(),
            accounts.quote_asset_badge.clone(),
            accounts.pair.clone(),
            accounts.user.clone(),
            accounts.system_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; INITIALIZE_PAIR_IX_ACCOUNTS_LEN]>
for InitializePairAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; INITIALIZE_PAIR_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            liquidity_book_config: &arr[0],
            token_mint_x: &arr[1],
            token_mint_y: &arr[2],
            bin_step_config: &arr[3],
            quote_asset_badge: &arr[4],
            pair: &arr[5],
            user: &arr[6],
            system_program: &arr[7],
            event_authority: &arr[8],
            program: &arr[9],
        }
    }
}
pub const INITIALIZE_PAIR_IX_DISCM: [u8; 8usize] = [177, 114, 226, 34, 186, 150, 5, 245];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct InitializePairIxArgs {
    pub active_id: u32,
}
#[derive(Clone, Debug, PartialEq)]
pub struct InitializePairIxData(pub InitializePairIxArgs);
impl From<InitializePairIxArgs> for InitializePairIxData {
    fn from(args: InitializePairIxArgs) -> Self {
        Self(args)
    }
}
impl InitializePairIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INITIALIZE_PAIR_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let active_id: u32 = crate::borsh_de_or_default(&mut reader)?;
        Ok(Self(InitializePairIxArgs { active_id }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INITIALIZE_PAIR_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.active_id, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn initialize_pair_ix_with_program_id(
    program_id: Pubkey,
    keys: InitializePairKeys,
    args: InitializePairIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; INITIALIZE_PAIR_IX_ACCOUNTS_LEN] = keys.into();
    let data: InitializePairIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn initialize_pair_ix(
    keys: InitializePairKeys,
    args: InitializePairIxArgs,
) -> std::io::Result<Instruction> {
    initialize_pair_ix_with_program_id(LIQUIDITY_BOOK_PROGRAM_ID, keys, args)
}
pub fn initialize_pair_invoke_with_program_id(
    program_id: Pubkey,
    accounts: InitializePairAccounts<'_, '_>,
    args: InitializePairIxArgs,
) -> ProgramResult {
    let keys: InitializePairKeys = accounts.into();
    let ix = initialize_pair_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn initialize_pair_invoke(
    accounts: InitializePairAccounts<'_, '_>,
    args: InitializePairIxArgs,
) -> ProgramResult {
    initialize_pair_invoke_with_program_id(LIQUIDITY_BOOK_PROGRAM_ID, accounts, args)
}
pub fn initialize_pair_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: InitializePairAccounts<'_, '_>,
    args: InitializePairIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: InitializePairKeys = accounts.into();
    let ix = initialize_pair_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn initialize_pair_invoke_signed(
    accounts: InitializePairAccounts<'_, '_>,
    args: InitializePairIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    initialize_pair_invoke_signed_with_program_id(
        LIQUIDITY_BOOK_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn initialize_pair_verify_account_keys(
    accounts: InitializePairAccounts<'_, '_>,
    keys: InitializePairKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.liquidity_book_config.key, keys.liquidity_book_config),
        (*accounts.token_mint_x.key, keys.token_mint_x),
        (*accounts.token_mint_y.key, keys.token_mint_y),
        (*accounts.bin_step_config.key, keys.bin_step_config),
        (*accounts.quote_asset_badge.key, keys.quote_asset_badge),
        (*accounts.pair.key, keys.pair),
        (*accounts.user.key, keys.user),
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
pub fn initialize_pair_verify_writable_privileges<'me, 'info>(
    accounts: InitializePairAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.pair, accounts.user] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn initialize_pair_verify_signer_privileges<'me, 'info>(
    accounts: InitializePairAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.user] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn initialize_pair_verify_account_privileges<'me, 'info>(
    accounts: InitializePairAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    initialize_pair_verify_writable_privileges(accounts)?;
    initialize_pair_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const INITIALIZE_QUOTE_ASSET_BADGE_IX_ACCOUNTS_LEN: usize = 7;
#[derive(Copy, Clone, Debug)]
pub struct InitializeQuoteAssetBadgeAccounts<'me, 'info> {
    pub liquidity_book_config: &'me AccountInfo<'info>,
    pub quote_asset_badge: &'me AccountInfo<'info>,
    pub token_mint: &'me AccountInfo<'info>,
    pub preset_authority: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct InitializeQuoteAssetBadgeKeys {
    pub liquidity_book_config: Pubkey,
    pub quote_asset_badge: Pubkey,
    pub token_mint: Pubkey,
    pub preset_authority: Pubkey,
    pub system_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<InitializeQuoteAssetBadgeAccounts<'_, '_>> for InitializeQuoteAssetBadgeKeys {
    fn from(accounts: InitializeQuoteAssetBadgeAccounts) -> Self {
        Self {
            liquidity_book_config: *accounts.liquidity_book_config.key,
            quote_asset_badge: *accounts.quote_asset_badge.key,
            token_mint: *accounts.token_mint.key,
            preset_authority: *accounts.preset_authority.key,
            system_program: *accounts.system_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<InitializeQuoteAssetBadgeKeys>
for [AccountMeta; INITIALIZE_QUOTE_ASSET_BADGE_IX_ACCOUNTS_LEN] {
    fn from(keys: InitializeQuoteAssetBadgeKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.liquidity_book_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.quote_asset_badge,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.preset_authority,
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
impl From<[Pubkey; INITIALIZE_QUOTE_ASSET_BADGE_IX_ACCOUNTS_LEN]>
for InitializeQuoteAssetBadgeKeys {
    fn from(pubkeys: [Pubkey; INITIALIZE_QUOTE_ASSET_BADGE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            liquidity_book_config: pubkeys[0],
            quote_asset_badge: pubkeys[1],
            token_mint: pubkeys[2],
            preset_authority: pubkeys[3],
            system_program: pubkeys[4],
            event_authority: pubkeys[5],
            program: pubkeys[6],
        }
    }
}
impl<'info> From<InitializeQuoteAssetBadgeAccounts<'_, 'info>>
for [AccountInfo<'info>; INITIALIZE_QUOTE_ASSET_BADGE_IX_ACCOUNTS_LEN] {
    fn from(accounts: InitializeQuoteAssetBadgeAccounts<'_, 'info>) -> Self {
        [
            accounts.liquidity_book_config.clone(),
            accounts.quote_asset_badge.clone(),
            accounts.token_mint.clone(),
            accounts.preset_authority.clone(),
            accounts.system_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; INITIALIZE_QUOTE_ASSET_BADGE_IX_ACCOUNTS_LEN]>
for InitializeQuoteAssetBadgeAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; INITIALIZE_QUOTE_ASSET_BADGE_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            liquidity_book_config: &arr[0],
            quote_asset_badge: &arr[1],
            token_mint: &arr[2],
            preset_authority: &arr[3],
            system_program: &arr[4],
            event_authority: &arr[5],
            program: &arr[6],
        }
    }
}
pub const INITIALIZE_QUOTE_ASSET_BADGE_IX_DISCM: [u8; 8usize] = [
    115, 174, 34, 42, 176, 5, 229, 207,
];
#[derive(Clone, Debug, PartialEq)]
pub struct InitializeQuoteAssetBadgeIxData;
impl InitializeQuoteAssetBadgeIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INITIALIZE_QUOTE_ASSET_BADGE_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INITIALIZE_QUOTE_ASSET_BADGE_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn initialize_quote_asset_badge_ix_with_program_id(
    program_id: Pubkey,
    keys: InitializeQuoteAssetBadgeKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; INITIALIZE_QUOTE_ASSET_BADGE_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: InitializeQuoteAssetBadgeIxData.try_to_vec()?,
    })
}
pub fn initialize_quote_asset_badge_ix(
    keys: InitializeQuoteAssetBadgeKeys,
) -> std::io::Result<Instruction> {
    initialize_quote_asset_badge_ix_with_program_id(LIQUIDITY_BOOK_PROGRAM_ID, keys)
}
pub fn initialize_quote_asset_badge_invoke_with_program_id(
    program_id: Pubkey,
    accounts: InitializeQuoteAssetBadgeAccounts<'_, '_>,
) -> ProgramResult {
    let keys: InitializeQuoteAssetBadgeKeys = accounts.into();
    let ix = initialize_quote_asset_badge_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn initialize_quote_asset_badge_invoke(
    accounts: InitializeQuoteAssetBadgeAccounts<'_, '_>,
) -> ProgramResult {
    initialize_quote_asset_badge_invoke_with_program_id(
        LIQUIDITY_BOOK_PROGRAM_ID,
        accounts,
    )
}
pub fn initialize_quote_asset_badge_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: InitializeQuoteAssetBadgeAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: InitializeQuoteAssetBadgeKeys = accounts.into();
    let ix = initialize_quote_asset_badge_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn initialize_quote_asset_badge_invoke_signed(
    accounts: InitializeQuoteAssetBadgeAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    initialize_quote_asset_badge_invoke_signed_with_program_id(
        LIQUIDITY_BOOK_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn initialize_quote_asset_badge_verify_account_keys(
    accounts: InitializeQuoteAssetBadgeAccounts<'_, '_>,
    keys: InitializeQuoteAssetBadgeKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.liquidity_book_config.key, keys.liquidity_book_config),
        (*accounts.quote_asset_badge.key, keys.quote_asset_badge),
        (*accounts.token_mint.key, keys.token_mint),
        (*accounts.preset_authority.key, keys.preset_authority),
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
pub fn initialize_quote_asset_badge_verify_writable_privileges<'me, 'info>(
    accounts: InitializeQuoteAssetBadgeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.quote_asset_badge, accounts.preset_authority] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn initialize_quote_asset_badge_verify_signer_privileges<'me, 'info>(
    accounts: InitializeQuoteAssetBadgeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.preset_authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn initialize_quote_asset_badge_verify_account_privileges<'me, 'info>(
    accounts: InitializeQuoteAssetBadgeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    initialize_quote_asset_badge_verify_writable_privileges(accounts)?;
    initialize_quote_asset_badge_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SET_HOOK_IX_ACCOUNTS_LEN: usize = 6;
#[derive(Copy, Clone, Debug)]
pub struct SetHookAccounts<'me, 'info> {
    pub liquidity_book_config: &'me AccountInfo<'info>,
    pub pair: &'me AccountInfo<'info>,
    pub preset_authority: &'me AccountInfo<'info>,
    pub hook: &'me AccountInfo<'info>,
    pub hooks_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SetHookKeys {
    pub liquidity_book_config: Pubkey,
    pub pair: Pubkey,
    pub preset_authority: Pubkey,
    pub hook: Pubkey,
    pub hooks_program: Pubkey,
    pub system_program: Pubkey,
}
impl From<SetHookAccounts<'_, '_>> for SetHookKeys {
    fn from(accounts: SetHookAccounts) -> Self {
        Self {
            liquidity_book_config: *accounts.liquidity_book_config.key,
            pair: *accounts.pair.key,
            preset_authority: *accounts.preset_authority.key,
            hook: *accounts.hook.key,
            hooks_program: *accounts.hooks_program.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<SetHookKeys> for [AccountMeta; SET_HOOK_IX_ACCOUNTS_LEN] {
    fn from(keys: SetHookKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.liquidity_book_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pair,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.preset_authority,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.hook,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.hooks_program,
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
impl From<[Pubkey; SET_HOOK_IX_ACCOUNTS_LEN]> for SetHookKeys {
    fn from(pubkeys: [Pubkey; SET_HOOK_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            liquidity_book_config: pubkeys[0],
            pair: pubkeys[1],
            preset_authority: pubkeys[2],
            hook: pubkeys[3],
            hooks_program: pubkeys[4],
            system_program: pubkeys[5],
        }
    }
}
impl<'info> From<SetHookAccounts<'_, 'info>>
for [AccountInfo<'info>; SET_HOOK_IX_ACCOUNTS_LEN] {
    fn from(accounts: SetHookAccounts<'_, 'info>) -> Self {
        [
            accounts.liquidity_book_config.clone(),
            accounts.pair.clone(),
            accounts.preset_authority.clone(),
            accounts.hook.clone(),
            accounts.hooks_program.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SET_HOOK_IX_ACCOUNTS_LEN]>
for SetHookAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; SET_HOOK_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            liquidity_book_config: &arr[0],
            pair: &arr[1],
            preset_authority: &arr[2],
            hook: &arr[3],
            hooks_program: &arr[4],
            system_program: &arr[5],
        }
    }
}
pub const SET_HOOK_IX_DISCM: [u8; 8usize] = [175, 16, 187, 252, 19, 54, 111, 221];
#[derive(Clone, Debug, PartialEq)]
pub struct SetHookIxData;
impl SetHookIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SET_HOOK_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SET_HOOK_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn set_hook_ix_with_program_id(
    program_id: Pubkey,
    keys: SetHookKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SET_HOOK_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: SetHookIxData.try_to_vec()?,
    })
}
pub fn set_hook_ix(keys: SetHookKeys) -> std::io::Result<Instruction> {
    set_hook_ix_with_program_id(LIQUIDITY_BOOK_PROGRAM_ID, keys)
}
pub fn set_hook_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SetHookAccounts<'_, '_>,
) -> ProgramResult {
    let keys: SetHookKeys = accounts.into();
    let ix = set_hook_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn set_hook_invoke(accounts: SetHookAccounts<'_, '_>) -> ProgramResult {
    set_hook_invoke_with_program_id(LIQUIDITY_BOOK_PROGRAM_ID, accounts)
}
pub fn set_hook_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SetHookAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SetHookKeys = accounts.into();
    let ix = set_hook_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn set_hook_invoke_signed(
    accounts: SetHookAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    set_hook_invoke_signed_with_program_id(LIQUIDITY_BOOK_PROGRAM_ID, accounts, seeds)
}
pub fn set_hook_verify_account_keys(
    accounts: SetHookAccounts<'_, '_>,
    keys: SetHookKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.liquidity_book_config.key, keys.liquidity_book_config),
        (*accounts.pair.key, keys.pair),
        (*accounts.preset_authority.key, keys.preset_authority),
        (*accounts.hook.key, keys.hook),
        (*accounts.hooks_program.key, keys.hooks_program),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn set_hook_verify_writable_privileges<'me, 'info>(
    accounts: SetHookAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.pair, accounts.hook] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn set_hook_verify_signer_privileges<'me, 'info>(
    accounts: SetHookAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.preset_authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn set_hook_verify_account_privileges<'me, 'info>(
    accounts: SetHookAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    set_hook_verify_writable_privileges(accounts)?;
    set_hook_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SWAP_IX_ACCOUNTS_LEN: usize = 17;
#[derive(Copy, Clone, Debug)]
pub struct SwapAccounts<'me, 'info> {
    pub pair: &'me AccountInfo<'info>,
    pub token_mint_x: &'me AccountInfo<'info>,
    pub token_mint_y: &'me AccountInfo<'info>,
    pub bin_array_lower: &'me AccountInfo<'info>,
    pub bin_array_upper: &'me AccountInfo<'info>,
    pub token_vault_x: &'me AccountInfo<'info>,
    pub token_vault_y: &'me AccountInfo<'info>,
    pub user_vault_x: &'me AccountInfo<'info>,
    pub user_vault_y: &'me AccountInfo<'info>,
    pub user: &'me AccountInfo<'info>,
    pub token_program_x: &'me AccountInfo<'info>,
    pub token_program_y: &'me AccountInfo<'info>,
    pub memo_program: &'me AccountInfo<'info>,
    pub hook: &'me AccountInfo<'info>,
    pub hooks_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SwapKeys {
    pub pair: Pubkey,
    pub token_mint_x: Pubkey,
    pub token_mint_y: Pubkey,
    pub bin_array_lower: Pubkey,
    pub bin_array_upper: Pubkey,
    pub token_vault_x: Pubkey,
    pub token_vault_y: Pubkey,
    pub user_vault_x: Pubkey,
    pub user_vault_y: Pubkey,
    pub user: Pubkey,
    pub token_program_x: Pubkey,
    pub token_program_y: Pubkey,
    pub memo_program: Pubkey,
    pub hook: Pubkey,
    pub hooks_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<SwapAccounts<'_, '_>> for SwapKeys {
    fn from(accounts: SwapAccounts) -> Self {
        Self {
            pair: *accounts.pair.key,
            token_mint_x: *accounts.token_mint_x.key,
            token_mint_y: *accounts.token_mint_y.key,
            bin_array_lower: *accounts.bin_array_lower.key,
            bin_array_upper: *accounts.bin_array_upper.key,
            token_vault_x: *accounts.token_vault_x.key,
            token_vault_y: *accounts.token_vault_y.key,
            user_vault_x: *accounts.user_vault_x.key,
            user_vault_y: *accounts.user_vault_y.key,
            user: *accounts.user.key,
            token_program_x: *accounts.token_program_x.key,
            token_program_y: *accounts.token_program_y.key,
            memo_program: *accounts.memo_program.key,
            hook: *accounts.hook.key,
            hooks_program: *accounts.hooks_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<SwapKeys> for [AccountMeta; SWAP_IX_ACCOUNTS_LEN] {
    fn from(keys: SwapKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.pair,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_mint_x,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_mint_y,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.bin_array_lower,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.bin_array_upper,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_vault_x,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_vault_y,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_vault_x,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_vault_y,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_program_x,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_program_y,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.memo_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.hook,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.hooks_program,
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
impl From<[Pubkey; SWAP_IX_ACCOUNTS_LEN]> for SwapKeys {
    fn from(pubkeys: [Pubkey; SWAP_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            pair: pubkeys[0],
            token_mint_x: pubkeys[1],
            token_mint_y: pubkeys[2],
            bin_array_lower: pubkeys[3],
            bin_array_upper: pubkeys[4],
            token_vault_x: pubkeys[5],
            token_vault_y: pubkeys[6],
            user_vault_x: pubkeys[7],
            user_vault_y: pubkeys[8],
            user: pubkeys[9],
            token_program_x: pubkeys[10],
            token_program_y: pubkeys[11],
            memo_program: pubkeys[12],
            hook: pubkeys[13],
            hooks_program: pubkeys[14],
            event_authority: pubkeys[15],
            program: pubkeys[16],
        }
    }
}
impl<'info> From<SwapAccounts<'_, 'info>>
for [AccountInfo<'info>; SWAP_IX_ACCOUNTS_LEN] {
    fn from(accounts: SwapAccounts<'_, 'info>) -> Self {
        [
            accounts.pair.clone(),
            accounts.token_mint_x.clone(),
            accounts.token_mint_y.clone(),
            accounts.bin_array_lower.clone(),
            accounts.bin_array_upper.clone(),
            accounts.token_vault_x.clone(),
            accounts.token_vault_y.clone(),
            accounts.user_vault_x.clone(),
            accounts.user_vault_y.clone(),
            accounts.user.clone(),
            accounts.token_program_x.clone(),
            accounts.token_program_y.clone(),
            accounts.memo_program.clone(),
            accounts.hook.clone(),
            accounts.hooks_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SWAP_IX_ACCOUNTS_LEN]>
for SwapAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; SWAP_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            pair: &arr[0],
            token_mint_x: &arr[1],
            token_mint_y: &arr[2],
            bin_array_lower: &arr[3],
            bin_array_upper: &arr[4],
            token_vault_x: &arr[5],
            token_vault_y: &arr[6],
            user_vault_x: &arr[7],
            user_vault_y: &arr[8],
            user: &arr[9],
            token_program_x: &arr[10],
            token_program_y: &arr[11],
            memo_program: &arr[12],
            hook: &arr[13],
            hooks_program: &arr[14],
            event_authority: &arr[15],
            program: &arr[16],
        }
    }
}
pub const SWAP_IX_DISCM: [u8; 8usize] = [248, 198, 158, 145, 225, 117, 135, 200];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SwapIxArgs {
    pub amount: u64,
    pub other_amount_threshold: u64,
    pub swap_for_y: bool,
    pub swap_type: SwapType,
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
        let other_amount_threshold: u64 = crate::borsh_de_or_default(&mut reader)?;
        let swap_for_y: bool = crate::borsh_de_or_default(&mut reader)?;
        let swap_type: SwapType = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(SwapIxArgs {
                amount,
                other_amount_threshold,
                swap_for_y,
                swap_type,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SWAP_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.other_amount_threshold, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.swap_for_y, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.swap_type, &mut writer)?;
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
    swap_ix_with_program_id(LIQUIDITY_BOOK_PROGRAM_ID, keys, args)
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
    swap_invoke_with_program_id(LIQUIDITY_BOOK_PROGRAM_ID, accounts, args)
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
    swap_invoke_signed_with_program_id(LIQUIDITY_BOOK_PROGRAM_ID, accounts, args, seeds)
}
pub fn swap_verify_account_keys(
    accounts: SwapAccounts<'_, '_>,
    keys: SwapKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.pair.key, keys.pair),
        (*accounts.token_mint_x.key, keys.token_mint_x),
        (*accounts.token_mint_y.key, keys.token_mint_y),
        (*accounts.bin_array_lower.key, keys.bin_array_lower),
        (*accounts.bin_array_upper.key, keys.bin_array_upper),
        (*accounts.token_vault_x.key, keys.token_vault_x),
        (*accounts.token_vault_y.key, keys.token_vault_y),
        (*accounts.user_vault_x.key, keys.user_vault_x),
        (*accounts.user_vault_y.key, keys.user_vault_y),
        (*accounts.user.key, keys.user),
        (*accounts.token_program_x.key, keys.token_program_x),
        (*accounts.token_program_y.key, keys.token_program_y),
        (*accounts.memo_program.key, keys.memo_program),
        (*accounts.hook.key, keys.hook),
        (*accounts.hooks_program.key, keys.hooks_program),
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
        accounts.pair,
        accounts.bin_array_lower,
        accounts.bin_array_upper,
        accounts.token_vault_x,
        accounts.token_vault_y,
        accounts.user_vault_x,
        accounts.user_vault_y,
        accounts.hook,
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
    for should_be_signer in [accounts.user] {
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
pub const TRANSFER_CONFIG_OWNERSHIP_IX_ACCOUNTS_LEN: usize = 4;
#[derive(Copy, Clone, Debug)]
pub struct TransferConfigOwnershipAccounts<'me, 'info> {
    pub liquidity_book_config: &'me AccountInfo<'info>,
    pub preset_authority: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct TransferConfigOwnershipKeys {
    pub liquidity_book_config: Pubkey,
    pub preset_authority: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<TransferConfigOwnershipAccounts<'_, '_>> for TransferConfigOwnershipKeys {
    fn from(accounts: TransferConfigOwnershipAccounts) -> Self {
        Self {
            liquidity_book_config: *accounts.liquidity_book_config.key,
            preset_authority: *accounts.preset_authority.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<TransferConfigOwnershipKeys>
for [AccountMeta; TRANSFER_CONFIG_OWNERSHIP_IX_ACCOUNTS_LEN] {
    fn from(keys: TransferConfigOwnershipKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.liquidity_book_config,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.preset_authority,
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
impl From<[Pubkey; TRANSFER_CONFIG_OWNERSHIP_IX_ACCOUNTS_LEN]>
for TransferConfigOwnershipKeys {
    fn from(pubkeys: [Pubkey; TRANSFER_CONFIG_OWNERSHIP_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            liquidity_book_config: pubkeys[0],
            preset_authority: pubkeys[1],
            event_authority: pubkeys[2],
            program: pubkeys[3],
        }
    }
}
impl<'info> From<TransferConfigOwnershipAccounts<'_, 'info>>
for [AccountInfo<'info>; TRANSFER_CONFIG_OWNERSHIP_IX_ACCOUNTS_LEN] {
    fn from(accounts: TransferConfigOwnershipAccounts<'_, 'info>) -> Self {
        [
            accounts.liquidity_book_config.clone(),
            accounts.preset_authority.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; TRANSFER_CONFIG_OWNERSHIP_IX_ACCOUNTS_LEN]>
for TransferConfigOwnershipAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; TRANSFER_CONFIG_OWNERSHIP_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            liquidity_book_config: &arr[0],
            preset_authority: &arr[1],
            event_authority: &arr[2],
            program: &arr[3],
        }
    }
}
pub const TRANSFER_CONFIG_OWNERSHIP_IX_DISCM: [u8; 8usize] = [
    53, 124, 67, 226, 108, 130, 19, 12,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TransferConfigOwnershipIxArgs {
    pub new_authority: Option<Pubkey>,
}
#[derive(Clone, Debug, PartialEq)]
pub struct TransferConfigOwnershipIxData(pub TransferConfigOwnershipIxArgs);
impl From<TransferConfigOwnershipIxArgs> for TransferConfigOwnershipIxData {
    fn from(args: TransferConfigOwnershipIxArgs) -> Self {
        Self(args)
    }
}
impl TransferConfigOwnershipIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != TRANSFER_CONFIG_OWNERSHIP_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let new_authority: Option<Pubkey> = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(TransferConfigOwnershipIxArgs {
                new_authority,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&TRANSFER_CONFIG_OWNERSHIP_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.new_authority, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn transfer_config_ownership_ix_with_program_id(
    program_id: Pubkey,
    keys: TransferConfigOwnershipKeys,
    args: TransferConfigOwnershipIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; TRANSFER_CONFIG_OWNERSHIP_IX_ACCOUNTS_LEN] = keys.into();
    let data: TransferConfigOwnershipIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn transfer_config_ownership_ix(
    keys: TransferConfigOwnershipKeys,
    args: TransferConfigOwnershipIxArgs,
) -> std::io::Result<Instruction> {
    transfer_config_ownership_ix_with_program_id(LIQUIDITY_BOOK_PROGRAM_ID, keys, args)
}
pub fn transfer_config_ownership_invoke_with_program_id(
    program_id: Pubkey,
    accounts: TransferConfigOwnershipAccounts<'_, '_>,
    args: TransferConfigOwnershipIxArgs,
) -> ProgramResult {
    let keys: TransferConfigOwnershipKeys = accounts.into();
    let ix = transfer_config_ownership_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn transfer_config_ownership_invoke(
    accounts: TransferConfigOwnershipAccounts<'_, '_>,
    args: TransferConfigOwnershipIxArgs,
) -> ProgramResult {
    transfer_config_ownership_invoke_with_program_id(
        LIQUIDITY_BOOK_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn transfer_config_ownership_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: TransferConfigOwnershipAccounts<'_, '_>,
    args: TransferConfigOwnershipIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: TransferConfigOwnershipKeys = accounts.into();
    let ix = transfer_config_ownership_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn transfer_config_ownership_invoke_signed(
    accounts: TransferConfigOwnershipAccounts<'_, '_>,
    args: TransferConfigOwnershipIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    transfer_config_ownership_invoke_signed_with_program_id(
        LIQUIDITY_BOOK_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn transfer_config_ownership_verify_account_keys(
    accounts: TransferConfigOwnershipAccounts<'_, '_>,
    keys: TransferConfigOwnershipKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.liquidity_book_config.key, keys.liquidity_book_config),
        (*accounts.preset_authority.key, keys.preset_authority),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn transfer_config_ownership_verify_writable_privileges<'me, 'info>(
    accounts: TransferConfigOwnershipAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.liquidity_book_config] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn transfer_config_ownership_verify_signer_privileges<'me, 'info>(
    accounts: TransferConfigOwnershipAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.preset_authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn transfer_config_ownership_verify_account_privileges<'me, 'info>(
    accounts: TransferConfigOwnershipAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    transfer_config_ownership_verify_writable_privileges(accounts)?;
    transfer_config_ownership_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const UPDATE_BIN_STEP_CONFIG_IX_ACCOUNTS_LEN: usize = 5;
#[derive(Copy, Clone, Debug)]
pub struct UpdateBinStepConfigAccounts<'me, 'info> {
    pub liquidity_book_config: &'me AccountInfo<'info>,
    pub bin_step_config: &'me AccountInfo<'info>,
    pub preset_authority: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UpdateBinStepConfigKeys {
    pub liquidity_book_config: Pubkey,
    pub bin_step_config: Pubkey,
    pub preset_authority: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<UpdateBinStepConfigAccounts<'_, '_>> for UpdateBinStepConfigKeys {
    fn from(accounts: UpdateBinStepConfigAccounts) -> Self {
        Self {
            liquidity_book_config: *accounts.liquidity_book_config.key,
            bin_step_config: *accounts.bin_step_config.key,
            preset_authority: *accounts.preset_authority.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<UpdateBinStepConfigKeys>
for [AccountMeta; UPDATE_BIN_STEP_CONFIG_IX_ACCOUNTS_LEN] {
    fn from(keys: UpdateBinStepConfigKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.liquidity_book_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.bin_step_config,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.preset_authority,
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
impl From<[Pubkey; UPDATE_BIN_STEP_CONFIG_IX_ACCOUNTS_LEN]> for UpdateBinStepConfigKeys {
    fn from(pubkeys: [Pubkey; UPDATE_BIN_STEP_CONFIG_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            liquidity_book_config: pubkeys[0],
            bin_step_config: pubkeys[1],
            preset_authority: pubkeys[2],
            event_authority: pubkeys[3],
            program: pubkeys[4],
        }
    }
}
impl<'info> From<UpdateBinStepConfigAccounts<'_, 'info>>
for [AccountInfo<'info>; UPDATE_BIN_STEP_CONFIG_IX_ACCOUNTS_LEN] {
    fn from(accounts: UpdateBinStepConfigAccounts<'_, 'info>) -> Self {
        [
            accounts.liquidity_book_config.clone(),
            accounts.bin_step_config.clone(),
            accounts.preset_authority.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; UPDATE_BIN_STEP_CONFIG_IX_ACCOUNTS_LEN]>
for UpdateBinStepConfigAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; UPDATE_BIN_STEP_CONFIG_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            liquidity_book_config: &arr[0],
            bin_step_config: &arr[1],
            preset_authority: &arr[2],
            event_authority: &arr[3],
            program: &arr[4],
        }
    }
}
pub const UPDATE_BIN_STEP_CONFIG_IX_DISCM: [u8; 8usize] = [
    205, 204, 206, 220, 251, 239, 19, 238,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UpdateBinStepConfigIxArgs {
    pub status: ConfigStatus,
    pub availability: ConfigAvailability,
    pub fee_parameters: StaticFeeParameters,
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateBinStepConfigIxData(pub UpdateBinStepConfigIxArgs);
impl From<UpdateBinStepConfigIxArgs> for UpdateBinStepConfigIxData {
    fn from(args: UpdateBinStepConfigIxArgs) -> Self {
        Self(args)
    }
}
impl UpdateBinStepConfigIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_BIN_STEP_CONFIG_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let status: ConfigStatus = crate::borsh_de_or_default(&mut reader)?;
        let availability: ConfigAvailability = crate::borsh_de_or_default(&mut reader)?;
        let fee_parameters = if reader.is_empty() {
            Default::default()
        } else {
            <StaticFeeParameters>::deserialize(&mut reader)?
        };
        Ok(
            Self(UpdateBinStepConfigIxArgs {
                status,
                availability,
                fee_parameters,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_BIN_STEP_CONFIG_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.status, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.availability, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.fee_parameters, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn update_bin_step_config_ix_with_program_id(
    program_id: Pubkey,
    keys: UpdateBinStepConfigKeys,
    args: UpdateBinStepConfigIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; UPDATE_BIN_STEP_CONFIG_IX_ACCOUNTS_LEN] = keys.into();
    let data: UpdateBinStepConfigIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_bin_step_config_ix(
    keys: UpdateBinStepConfigKeys,
    args: UpdateBinStepConfigIxArgs,
) -> std::io::Result<Instruction> {
    update_bin_step_config_ix_with_program_id(LIQUIDITY_BOOK_PROGRAM_ID, keys, args)
}
pub fn update_bin_step_config_invoke_with_program_id(
    program_id: Pubkey,
    accounts: UpdateBinStepConfigAccounts<'_, '_>,
    args: UpdateBinStepConfigIxArgs,
) -> ProgramResult {
    let keys: UpdateBinStepConfigKeys = accounts.into();
    let ix = update_bin_step_config_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn update_bin_step_config_invoke(
    accounts: UpdateBinStepConfigAccounts<'_, '_>,
    args: UpdateBinStepConfigIxArgs,
) -> ProgramResult {
    update_bin_step_config_invoke_with_program_id(
        LIQUIDITY_BOOK_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn update_bin_step_config_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: UpdateBinStepConfigAccounts<'_, '_>,
    args: UpdateBinStepConfigIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: UpdateBinStepConfigKeys = accounts.into();
    let ix = update_bin_step_config_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn update_bin_step_config_invoke_signed(
    accounts: UpdateBinStepConfigAccounts<'_, '_>,
    args: UpdateBinStepConfigIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    update_bin_step_config_invoke_signed_with_program_id(
        LIQUIDITY_BOOK_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn update_bin_step_config_verify_account_keys(
    accounts: UpdateBinStepConfigAccounts<'_, '_>,
    keys: UpdateBinStepConfigKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.liquidity_book_config.key, keys.liquidity_book_config),
        (*accounts.bin_step_config.key, keys.bin_step_config),
        (*accounts.preset_authority.key, keys.preset_authority),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn update_bin_step_config_verify_writable_privileges<'me, 'info>(
    accounts: UpdateBinStepConfigAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.bin_step_config] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn update_bin_step_config_verify_signer_privileges<'me, 'info>(
    accounts: UpdateBinStepConfigAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.preset_authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn update_bin_step_config_verify_account_privileges<'me, 'info>(
    accounts: UpdateBinStepConfigAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    update_bin_step_config_verify_writable_privileges(accounts)?;
    update_bin_step_config_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const UPDATE_PAIR_STATIC_FEE_PARAMETERS_IX_ACCOUNTS_LEN: usize = 6;
#[derive(Copy, Clone, Debug)]
pub struct UpdatePairStaticFeeParametersAccounts<'me, 'info> {
    pub liquidity_book_config: &'me AccountInfo<'info>,
    pub pair: &'me AccountInfo<'info>,
    pub preset_authority: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UpdatePairStaticFeeParametersKeys {
    pub liquidity_book_config: Pubkey,
    pub pair: Pubkey,
    pub preset_authority: Pubkey,
    pub system_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<UpdatePairStaticFeeParametersAccounts<'_, '_>>
for UpdatePairStaticFeeParametersKeys {
    fn from(accounts: UpdatePairStaticFeeParametersAccounts) -> Self {
        Self {
            liquidity_book_config: *accounts.liquidity_book_config.key,
            pair: *accounts.pair.key,
            preset_authority: *accounts.preset_authority.key,
            system_program: *accounts.system_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<UpdatePairStaticFeeParametersKeys>
for [AccountMeta; UPDATE_PAIR_STATIC_FEE_PARAMETERS_IX_ACCOUNTS_LEN] {
    fn from(keys: UpdatePairStaticFeeParametersKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.liquidity_book_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pair,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.preset_authority,
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
impl From<[Pubkey; UPDATE_PAIR_STATIC_FEE_PARAMETERS_IX_ACCOUNTS_LEN]>
for UpdatePairStaticFeeParametersKeys {
    fn from(
        pubkeys: [Pubkey; UPDATE_PAIR_STATIC_FEE_PARAMETERS_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            liquidity_book_config: pubkeys[0],
            pair: pubkeys[1],
            preset_authority: pubkeys[2],
            system_program: pubkeys[3],
            event_authority: pubkeys[4],
            program: pubkeys[5],
        }
    }
}
impl<'info> From<UpdatePairStaticFeeParametersAccounts<'_, 'info>>
for [AccountInfo<'info>; UPDATE_PAIR_STATIC_FEE_PARAMETERS_IX_ACCOUNTS_LEN] {
    fn from(accounts: UpdatePairStaticFeeParametersAccounts<'_, 'info>) -> Self {
        [
            accounts.liquidity_book_config.clone(),
            accounts.pair.clone(),
            accounts.preset_authority.clone(),
            accounts.system_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; UPDATE_PAIR_STATIC_FEE_PARAMETERS_IX_ACCOUNTS_LEN]>
for UpdatePairStaticFeeParametersAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; UPDATE_PAIR_STATIC_FEE_PARAMETERS_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            liquidity_book_config: &arr[0],
            pair: &arr[1],
            preset_authority: &arr[2],
            system_program: &arr[3],
            event_authority: &arr[4],
            program: &arr[5],
        }
    }
}
pub const UPDATE_PAIR_STATIC_FEE_PARAMETERS_IX_DISCM: [u8; 8usize] = [
    20, 223, 186, 73, 199, 65, 45, 80,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UpdatePairStaticFeeParametersIxArgs {
    pub fee_parameters: StaticFeeParameters,
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdatePairStaticFeeParametersIxData(pub UpdatePairStaticFeeParametersIxArgs);
impl From<UpdatePairStaticFeeParametersIxArgs> for UpdatePairStaticFeeParametersIxData {
    fn from(args: UpdatePairStaticFeeParametersIxArgs) -> Self {
        Self(args)
    }
}
impl UpdatePairStaticFeeParametersIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_PAIR_STATIC_FEE_PARAMETERS_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let fee_parameters = if reader.is_empty() {
            Default::default()
        } else {
            <StaticFeeParameters>::deserialize(&mut reader)?
        };
        Ok(
            Self(UpdatePairStaticFeeParametersIxArgs {
                fee_parameters,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_PAIR_STATIC_FEE_PARAMETERS_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.fee_parameters, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn update_pair_static_fee_parameters_ix_with_program_id(
    program_id: Pubkey,
    keys: UpdatePairStaticFeeParametersKeys,
    args: UpdatePairStaticFeeParametersIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; UPDATE_PAIR_STATIC_FEE_PARAMETERS_IX_ACCOUNTS_LEN] = keys
        .into();
    let data: UpdatePairStaticFeeParametersIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_pair_static_fee_parameters_ix(
    keys: UpdatePairStaticFeeParametersKeys,
    args: UpdatePairStaticFeeParametersIxArgs,
) -> std::io::Result<Instruction> {
    update_pair_static_fee_parameters_ix_with_program_id(
        LIQUIDITY_BOOK_PROGRAM_ID,
        keys,
        args,
    )
}
pub fn update_pair_static_fee_parameters_invoke_with_program_id(
    program_id: Pubkey,
    accounts: UpdatePairStaticFeeParametersAccounts<'_, '_>,
    args: UpdatePairStaticFeeParametersIxArgs,
) -> ProgramResult {
    let keys: UpdatePairStaticFeeParametersKeys = accounts.into();
    let ix = update_pair_static_fee_parameters_ix_with_program_id(
        program_id,
        keys,
        args,
    )?;
    invoke_instruction(&ix, accounts)
}
pub fn update_pair_static_fee_parameters_invoke(
    accounts: UpdatePairStaticFeeParametersAccounts<'_, '_>,
    args: UpdatePairStaticFeeParametersIxArgs,
) -> ProgramResult {
    update_pair_static_fee_parameters_invoke_with_program_id(
        LIQUIDITY_BOOK_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn update_pair_static_fee_parameters_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: UpdatePairStaticFeeParametersAccounts<'_, '_>,
    args: UpdatePairStaticFeeParametersIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: UpdatePairStaticFeeParametersKeys = accounts.into();
    let ix = update_pair_static_fee_parameters_ix_with_program_id(
        program_id,
        keys,
        args,
    )?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn update_pair_static_fee_parameters_invoke_signed(
    accounts: UpdatePairStaticFeeParametersAccounts<'_, '_>,
    args: UpdatePairStaticFeeParametersIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    update_pair_static_fee_parameters_invoke_signed_with_program_id(
        LIQUIDITY_BOOK_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn update_pair_static_fee_parameters_verify_account_keys(
    accounts: UpdatePairStaticFeeParametersAccounts<'_, '_>,
    keys: UpdatePairStaticFeeParametersKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.liquidity_book_config.key, keys.liquidity_book_config),
        (*accounts.pair.key, keys.pair),
        (*accounts.preset_authority.key, keys.preset_authority),
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
pub fn update_pair_static_fee_parameters_verify_writable_privileges<'me, 'info>(
    accounts: UpdatePairStaticFeeParametersAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.pair] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn update_pair_static_fee_parameters_verify_signer_privileges<'me, 'info>(
    accounts: UpdatePairStaticFeeParametersAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.preset_authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn update_pair_static_fee_parameters_verify_account_privileges<'me, 'info>(
    accounts: UpdatePairStaticFeeParametersAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    update_pair_static_fee_parameters_verify_writable_privileges(accounts)?;
    update_pair_static_fee_parameters_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const UPDATE_QUOTE_ASSET_BADGE_IX_ACCOUNTS_LEN: usize = 6;
#[derive(Copy, Clone, Debug)]
pub struct UpdateQuoteAssetBadgeAccounts<'me, 'info> {
    pub liquidity_book_config: &'me AccountInfo<'info>,
    pub quote_asset_badge: &'me AccountInfo<'info>,
    pub token_mint: &'me AccountInfo<'info>,
    pub preset_authority: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UpdateQuoteAssetBadgeKeys {
    pub liquidity_book_config: Pubkey,
    pub quote_asset_badge: Pubkey,
    pub token_mint: Pubkey,
    pub preset_authority: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<UpdateQuoteAssetBadgeAccounts<'_, '_>> for UpdateQuoteAssetBadgeKeys {
    fn from(accounts: UpdateQuoteAssetBadgeAccounts) -> Self {
        Self {
            liquidity_book_config: *accounts.liquidity_book_config.key,
            quote_asset_badge: *accounts.quote_asset_badge.key,
            token_mint: *accounts.token_mint.key,
            preset_authority: *accounts.preset_authority.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<UpdateQuoteAssetBadgeKeys>
for [AccountMeta; UPDATE_QUOTE_ASSET_BADGE_IX_ACCOUNTS_LEN] {
    fn from(keys: UpdateQuoteAssetBadgeKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.liquidity_book_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.quote_asset_badge,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.preset_authority,
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
impl From<[Pubkey; UPDATE_QUOTE_ASSET_BADGE_IX_ACCOUNTS_LEN]>
for UpdateQuoteAssetBadgeKeys {
    fn from(pubkeys: [Pubkey; UPDATE_QUOTE_ASSET_BADGE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            liquidity_book_config: pubkeys[0],
            quote_asset_badge: pubkeys[1],
            token_mint: pubkeys[2],
            preset_authority: pubkeys[3],
            event_authority: pubkeys[4],
            program: pubkeys[5],
        }
    }
}
impl<'info> From<UpdateQuoteAssetBadgeAccounts<'_, 'info>>
for [AccountInfo<'info>; UPDATE_QUOTE_ASSET_BADGE_IX_ACCOUNTS_LEN] {
    fn from(accounts: UpdateQuoteAssetBadgeAccounts<'_, 'info>) -> Self {
        [
            accounts.liquidity_book_config.clone(),
            accounts.quote_asset_badge.clone(),
            accounts.token_mint.clone(),
            accounts.preset_authority.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; UPDATE_QUOTE_ASSET_BADGE_IX_ACCOUNTS_LEN]>
for UpdateQuoteAssetBadgeAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; UPDATE_QUOTE_ASSET_BADGE_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            liquidity_book_config: &arr[0],
            quote_asset_badge: &arr[1],
            token_mint: &arr[2],
            preset_authority: &arr[3],
            event_authority: &arr[4],
            program: &arr[5],
        }
    }
}
pub const UPDATE_QUOTE_ASSET_BADGE_IX_DISCM: [u8; 8usize] = [
    42, 12, 208, 17, 29, 174, 196, 103,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UpdateQuoteAssetBadgeIxArgs {
    pub status: QuoteAssetBadgeStatus,
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateQuoteAssetBadgeIxData(pub UpdateQuoteAssetBadgeIxArgs);
impl From<UpdateQuoteAssetBadgeIxArgs> for UpdateQuoteAssetBadgeIxData {
    fn from(args: UpdateQuoteAssetBadgeIxArgs) -> Self {
        Self(args)
    }
}
impl UpdateQuoteAssetBadgeIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_QUOTE_ASSET_BADGE_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let status: QuoteAssetBadgeStatus = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(UpdateQuoteAssetBadgeIxArgs {
                status,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_QUOTE_ASSET_BADGE_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.status, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn update_quote_asset_badge_ix_with_program_id(
    program_id: Pubkey,
    keys: UpdateQuoteAssetBadgeKeys,
    args: UpdateQuoteAssetBadgeIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; UPDATE_QUOTE_ASSET_BADGE_IX_ACCOUNTS_LEN] = keys.into();
    let data: UpdateQuoteAssetBadgeIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_quote_asset_badge_ix(
    keys: UpdateQuoteAssetBadgeKeys,
    args: UpdateQuoteAssetBadgeIxArgs,
) -> std::io::Result<Instruction> {
    update_quote_asset_badge_ix_with_program_id(LIQUIDITY_BOOK_PROGRAM_ID, keys, args)
}
pub fn update_quote_asset_badge_invoke_with_program_id(
    program_id: Pubkey,
    accounts: UpdateQuoteAssetBadgeAccounts<'_, '_>,
    args: UpdateQuoteAssetBadgeIxArgs,
) -> ProgramResult {
    let keys: UpdateQuoteAssetBadgeKeys = accounts.into();
    let ix = update_quote_asset_badge_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn update_quote_asset_badge_invoke(
    accounts: UpdateQuoteAssetBadgeAccounts<'_, '_>,
    args: UpdateQuoteAssetBadgeIxArgs,
) -> ProgramResult {
    update_quote_asset_badge_invoke_with_program_id(
        LIQUIDITY_BOOK_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn update_quote_asset_badge_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: UpdateQuoteAssetBadgeAccounts<'_, '_>,
    args: UpdateQuoteAssetBadgeIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: UpdateQuoteAssetBadgeKeys = accounts.into();
    let ix = update_quote_asset_badge_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn update_quote_asset_badge_invoke_signed(
    accounts: UpdateQuoteAssetBadgeAccounts<'_, '_>,
    args: UpdateQuoteAssetBadgeIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    update_quote_asset_badge_invoke_signed_with_program_id(
        LIQUIDITY_BOOK_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn update_quote_asset_badge_verify_account_keys(
    accounts: UpdateQuoteAssetBadgeAccounts<'_, '_>,
    keys: UpdateQuoteAssetBadgeKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.liquidity_book_config.key, keys.liquidity_book_config),
        (*accounts.quote_asset_badge.key, keys.quote_asset_badge),
        (*accounts.token_mint.key, keys.token_mint),
        (*accounts.preset_authority.key, keys.preset_authority),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn update_quote_asset_badge_verify_writable_privileges<'me, 'info>(
    accounts: UpdateQuoteAssetBadgeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.quote_asset_badge] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn update_quote_asset_badge_verify_signer_privileges<'me, 'info>(
    accounts: UpdateQuoteAssetBadgeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.preset_authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn update_quote_asset_badge_verify_account_privileges<'me, 'info>(
    accounts: UpdateQuoteAssetBadgeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    update_quote_asset_badge_verify_writable_privileges(accounts)?;
    update_quote_asset_badge_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const WITHDRAW_PROTOCOL_FEES_IX_ACCOUNTS_LEN: usize = 14;
#[derive(Copy, Clone, Debug)]
pub struct WithdrawProtocolFeesAccounts<'me, 'info> {
    pub liquidity_book_config: &'me AccountInfo<'info>,
    pub pair: &'me AccountInfo<'info>,
    pub token_mint_x: &'me AccountInfo<'info>,
    pub token_mint_y: &'me AccountInfo<'info>,
    pub token_vault_x: &'me AccountInfo<'info>,
    pub token_vault_y: &'me AccountInfo<'info>,
    pub protocol_vault_x: &'me AccountInfo<'info>,
    pub protocol_vault_y: &'me AccountInfo<'info>,
    pub token_program_x: &'me AccountInfo<'info>,
    pub token_program_y: &'me AccountInfo<'info>,
    pub preset_authority: &'me AccountInfo<'info>,
    pub memo_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct WithdrawProtocolFeesKeys {
    pub liquidity_book_config: Pubkey,
    pub pair: Pubkey,
    pub token_mint_x: Pubkey,
    pub token_mint_y: Pubkey,
    pub token_vault_x: Pubkey,
    pub token_vault_y: Pubkey,
    pub protocol_vault_x: Pubkey,
    pub protocol_vault_y: Pubkey,
    pub token_program_x: Pubkey,
    pub token_program_y: Pubkey,
    pub preset_authority: Pubkey,
    pub memo_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<WithdrawProtocolFeesAccounts<'_, '_>> for WithdrawProtocolFeesKeys {
    fn from(accounts: WithdrawProtocolFeesAccounts) -> Self {
        Self {
            liquidity_book_config: *accounts.liquidity_book_config.key,
            pair: *accounts.pair.key,
            token_mint_x: *accounts.token_mint_x.key,
            token_mint_y: *accounts.token_mint_y.key,
            token_vault_x: *accounts.token_vault_x.key,
            token_vault_y: *accounts.token_vault_y.key,
            protocol_vault_x: *accounts.protocol_vault_x.key,
            protocol_vault_y: *accounts.protocol_vault_y.key,
            token_program_x: *accounts.token_program_x.key,
            token_program_y: *accounts.token_program_y.key,
            preset_authority: *accounts.preset_authority.key,
            memo_program: *accounts.memo_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<WithdrawProtocolFeesKeys>
for [AccountMeta; WITHDRAW_PROTOCOL_FEES_IX_ACCOUNTS_LEN] {
    fn from(keys: WithdrawProtocolFeesKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.liquidity_book_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pair,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_mint_x,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_mint_y,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_vault_x,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_vault_y,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.protocol_vault_x,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.protocol_vault_y,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_program_x,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_program_y,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.preset_authority,
                is_signer: true,
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
impl From<[Pubkey; WITHDRAW_PROTOCOL_FEES_IX_ACCOUNTS_LEN]>
for WithdrawProtocolFeesKeys {
    fn from(pubkeys: [Pubkey; WITHDRAW_PROTOCOL_FEES_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            liquidity_book_config: pubkeys[0],
            pair: pubkeys[1],
            token_mint_x: pubkeys[2],
            token_mint_y: pubkeys[3],
            token_vault_x: pubkeys[4],
            token_vault_y: pubkeys[5],
            protocol_vault_x: pubkeys[6],
            protocol_vault_y: pubkeys[7],
            token_program_x: pubkeys[8],
            token_program_y: pubkeys[9],
            preset_authority: pubkeys[10],
            memo_program: pubkeys[11],
            event_authority: pubkeys[12],
            program: pubkeys[13],
        }
    }
}
impl<'info> From<WithdrawProtocolFeesAccounts<'_, 'info>>
for [AccountInfo<'info>; WITHDRAW_PROTOCOL_FEES_IX_ACCOUNTS_LEN] {
    fn from(accounts: WithdrawProtocolFeesAccounts<'_, 'info>) -> Self {
        [
            accounts.liquidity_book_config.clone(),
            accounts.pair.clone(),
            accounts.token_mint_x.clone(),
            accounts.token_mint_y.clone(),
            accounts.token_vault_x.clone(),
            accounts.token_vault_y.clone(),
            accounts.protocol_vault_x.clone(),
            accounts.protocol_vault_y.clone(),
            accounts.token_program_x.clone(),
            accounts.token_program_y.clone(),
            accounts.preset_authority.clone(),
            accounts.memo_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; WITHDRAW_PROTOCOL_FEES_IX_ACCOUNTS_LEN]>
for WithdrawProtocolFeesAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; WITHDRAW_PROTOCOL_FEES_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            liquidity_book_config: &arr[0],
            pair: &arr[1],
            token_mint_x: &arr[2],
            token_mint_y: &arr[3],
            token_vault_x: &arr[4],
            token_vault_y: &arr[5],
            protocol_vault_x: &arr[6],
            protocol_vault_y: &arr[7],
            token_program_x: &arr[8],
            token_program_y: &arr[9],
            preset_authority: &arr[10],
            memo_program: &arr[11],
            event_authority: &arr[12],
            program: &arr[13],
        }
    }
}
pub const WITHDRAW_PROTOCOL_FEES_IX_DISCM: [u8; 8usize] = [
    11, 68, 165, 98, 18, 208, 134, 73,
];
#[derive(Clone, Debug, PartialEq)]
pub struct WithdrawProtocolFeesIxData;
impl WithdrawProtocolFeesIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != WITHDRAW_PROTOCOL_FEES_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&WITHDRAW_PROTOCOL_FEES_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn withdraw_protocol_fees_ix_with_program_id(
    program_id: Pubkey,
    keys: WithdrawProtocolFeesKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; WITHDRAW_PROTOCOL_FEES_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: WithdrawProtocolFeesIxData.try_to_vec()?,
    })
}
pub fn withdraw_protocol_fees_ix(
    keys: WithdrawProtocolFeesKeys,
) -> std::io::Result<Instruction> {
    withdraw_protocol_fees_ix_with_program_id(LIQUIDITY_BOOK_PROGRAM_ID, keys)
}
pub fn withdraw_protocol_fees_invoke_with_program_id(
    program_id: Pubkey,
    accounts: WithdrawProtocolFeesAccounts<'_, '_>,
) -> ProgramResult {
    let keys: WithdrawProtocolFeesKeys = accounts.into();
    let ix = withdraw_protocol_fees_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn withdraw_protocol_fees_invoke(
    accounts: WithdrawProtocolFeesAccounts<'_, '_>,
) -> ProgramResult {
    withdraw_protocol_fees_invoke_with_program_id(LIQUIDITY_BOOK_PROGRAM_ID, accounts)
}
pub fn withdraw_protocol_fees_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: WithdrawProtocolFeesAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: WithdrawProtocolFeesKeys = accounts.into();
    let ix = withdraw_protocol_fees_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn withdraw_protocol_fees_invoke_signed(
    accounts: WithdrawProtocolFeesAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    withdraw_protocol_fees_invoke_signed_with_program_id(
        LIQUIDITY_BOOK_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn withdraw_protocol_fees_verify_account_keys(
    accounts: WithdrawProtocolFeesAccounts<'_, '_>,
    keys: WithdrawProtocolFeesKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.liquidity_book_config.key, keys.liquidity_book_config),
        (*accounts.pair.key, keys.pair),
        (*accounts.token_mint_x.key, keys.token_mint_x),
        (*accounts.token_mint_y.key, keys.token_mint_y),
        (*accounts.token_vault_x.key, keys.token_vault_x),
        (*accounts.token_vault_y.key, keys.token_vault_y),
        (*accounts.protocol_vault_x.key, keys.protocol_vault_x),
        (*accounts.protocol_vault_y.key, keys.protocol_vault_y),
        (*accounts.token_program_x.key, keys.token_program_x),
        (*accounts.token_program_y.key, keys.token_program_y),
        (*accounts.preset_authority.key, keys.preset_authority),
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
pub fn withdraw_protocol_fees_verify_writable_privileges<'me, 'info>(
    accounts: WithdrawProtocolFeesAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.pair,
        accounts.token_vault_x,
        accounts.token_vault_y,
        accounts.protocol_vault_x,
        accounts.protocol_vault_y,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn withdraw_protocol_fees_verify_signer_privileges<'me, 'info>(
    accounts: WithdrawProtocolFeesAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.preset_authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn withdraw_protocol_fees_verify_account_privileges<'me, 'info>(
    accounts: WithdrawProtocolFeesAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    withdraw_protocol_fees_verify_writable_privileges(accounts)?;
    withdraw_protocol_fees_verify_signer_privileges(accounts)?;
    Ok(())
}
