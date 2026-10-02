use solana_pubkey::Pubkey;
use solana_cpi::{invoke, invoke_signed};
use solana_instruction::{AccountMeta, Instruction};
use solana_account_info::AccountInfo;
use solana_program_error::ProgramError;
use std::io::Read;
#[allow(unused_imports)]
use crate::*;
#[derive(Clone, Debug, PartialEq)]
pub enum RaydiumCpSwapProgramIx {
    ClosePermissionPda,
    CloseSupportMintAssociated,
    CollectCreatorFee,
    CollectCreatorFeePermissionless,
    CollectExcessLamports,
    CollectFundFee(CollectFundFeeIxArgs),
    CollectProtocolFee(CollectProtocolFeeIxArgs),
    CreateAmmConfig(CreateAmmConfigIxArgs),
    CreatePermissionPda,
    CreateSupportMintAssociated,
    Deposit(DepositIxArgs),
    Initialize(InitializeIxArgs),
    InitializeWithPermission(InitializeWithPermissionIxArgs),
    SwapBaseInput(SwapBaseInputIxArgs),
    SwapBaseOutput(SwapBaseOutputIxArgs),
    UpdateAmmConfig(UpdateAmmConfigIxArgs),
    UpdatePoolStatus(UpdatePoolStatusIxArgs),
    Withdraw(WithdrawIxArgs),
}
impl RaydiumCpSwapProgramIx {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        if buf.starts_with(&CLOSE_PERMISSION_PDA_IX_DISCM) {
            return Ok(Self::ClosePermissionPda);
        }
        if buf.starts_with(&CLOSE_SUPPORT_MINT_ASSOCIATED_IX_DISCM) {
            return Ok(Self::CloseSupportMintAssociated);
        }
        if buf.starts_with(&COLLECT_CREATOR_FEE_IX_DISCM) {
            return Ok(Self::CollectCreatorFee);
        }
        if buf.starts_with(&COLLECT_CREATOR_FEE_PERMISSIONLESS_IX_DISCM) {
            return Ok(Self::CollectCreatorFeePermissionless);
        }
        if buf.starts_with(&COLLECT_EXCESS_LAMPORTS_IX_DISCM) {
            return Ok(Self::CollectExcessLamports);
        }
        if buf.starts_with(&COLLECT_FUND_FEE_IX_DISCM) {
            let mut reader = &buf[COLLECT_FUND_FEE_IX_DISCM.len()..];
            let amount_0_requested: u64 = crate::borsh_de_or_default(&mut reader)?;
            let amount_1_requested: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::CollectFundFee(CollectFundFeeIxArgs {
                    amount_0_requested,
                    amount_1_requested,
                }),
            );
        }
        if buf.starts_with(&COLLECT_PROTOCOL_FEE_IX_DISCM) {
            let mut reader = &buf[COLLECT_PROTOCOL_FEE_IX_DISCM.len()..];
            let amount_0_requested: u64 = crate::borsh_de_or_default(&mut reader)?;
            let amount_1_requested: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::CollectProtocolFee(CollectProtocolFeeIxArgs {
                    amount_0_requested,
                    amount_1_requested,
                }),
            );
        }
        if buf.starts_with(&CREATE_AMM_CONFIG_IX_DISCM) {
            let mut reader = &buf[CREATE_AMM_CONFIG_IX_DISCM.len()..];
            let index: u16 = crate::borsh_de_or_default(&mut reader)?;
            let trade_fee_rate: u64 = crate::borsh_de_or_default(&mut reader)?;
            let protocol_fee_rate: u64 = crate::borsh_de_or_default(&mut reader)?;
            let fund_fee_rate: u64 = crate::borsh_de_or_default(&mut reader)?;
            let create_pool_fee: u64 = crate::borsh_de_or_default(&mut reader)?;
            let creator_fee_rate: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::CreateAmmConfig(CreateAmmConfigIxArgs {
                    index,
                    trade_fee_rate,
                    protocol_fee_rate,
                    fund_fee_rate,
                    create_pool_fee,
                    creator_fee_rate,
                }),
            );
        }
        if buf.starts_with(&CREATE_PERMISSION_PDA_IX_DISCM) {
            return Ok(Self::CreatePermissionPda);
        }
        if buf.starts_with(&CREATE_SUPPORT_MINT_ASSOCIATED_IX_DISCM) {
            return Ok(Self::CreateSupportMintAssociated);
        }
        if buf.starts_with(&DEPOSIT_IX_DISCM) {
            let mut reader = &buf[DEPOSIT_IX_DISCM.len()..];
            let lp_token_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            let maximum_token_0_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            let maximum_token_1_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::Deposit(DepositIxArgs {
                    lp_token_amount,
                    maximum_token_0_amount,
                    maximum_token_1_amount,
                }),
            );
        }
        if buf.starts_with(&INITIALIZE_IX_DISCM) {
            let mut reader = &buf[INITIALIZE_IX_DISCM.len()..];
            let init_amount_0: u64 = crate::borsh_de_or_default(&mut reader)?;
            let init_amount_1: u64 = crate::borsh_de_or_default(&mut reader)?;
            let open_time: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::Initialize(InitializeIxArgs {
                    init_amount_0,
                    init_amount_1,
                    open_time,
                }),
            );
        }
        if buf.starts_with(&INITIALIZE_WITH_PERMISSION_IX_DISCM) {
            let mut reader = &buf[INITIALIZE_WITH_PERMISSION_IX_DISCM.len()..];
            let init_amount_0: u64 = crate::borsh_de_or_default(&mut reader)?;
            let init_amount_1: u64 = crate::borsh_de_or_default(&mut reader)?;
            let open_time: u64 = crate::borsh_de_or_default(&mut reader)?;
            let creator_fee_on: CreatorFeeOn = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::InitializeWithPermission(InitializeWithPermissionIxArgs {
                    init_amount_0,
                    init_amount_1,
                    open_time,
                    creator_fee_on,
                }),
            );
        }
        if buf.starts_with(&SWAP_BASE_INPUT_IX_DISCM) {
            let mut reader = &buf[SWAP_BASE_INPUT_IX_DISCM.len()..];
            let amount_in: u64 = crate::borsh_de_or_default(&mut reader)?;
            let minimum_amount_out: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::SwapBaseInput(SwapBaseInputIxArgs {
                    amount_in,
                    minimum_amount_out,
                }),
            );
        }
        if buf.starts_with(&SWAP_BASE_OUTPUT_IX_DISCM) {
            let mut reader = &buf[SWAP_BASE_OUTPUT_IX_DISCM.len()..];
            let max_amount_in: u64 = crate::borsh_de_or_default(&mut reader)?;
            let amount_out: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::SwapBaseOutput(SwapBaseOutputIxArgs {
                    max_amount_in,
                    amount_out,
                }),
            );
        }
        if buf.starts_with(&UPDATE_AMM_CONFIG_IX_DISCM) {
            let mut reader = &buf[UPDATE_AMM_CONFIG_IX_DISCM.len()..];
            let param: u8 = crate::borsh_de_or_default(&mut reader)?;
            let value: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::UpdateAmmConfig(UpdateAmmConfigIxArgs {
                    param,
                    value,
                }),
            );
        }
        if buf.starts_with(&UPDATE_POOL_STATUS_IX_DISCM) {
            let mut reader = &buf[UPDATE_POOL_STATUS_IX_DISCM.len()..];
            let status: u8 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(Self::UpdatePoolStatus(UpdatePoolStatusIxArgs { status }));
        }
        if buf.starts_with(&WITHDRAW_IX_DISCM) {
            let mut reader = &buf[WITHDRAW_IX_DISCM.len()..];
            let lp_token_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            let minimum_token_0_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            let minimum_token_1_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::Withdraw(WithdrawIxArgs {
                    lp_token_amount,
                    minimum_token_0_amount,
                    minimum_token_1_amount,
                }),
            );
        }
        Err(std::io::Error::from(std::io::ErrorKind::InvalidData))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        match self {
            Self::ClosePermissionPda => writer.write_all(&CLOSE_PERMISSION_PDA_IX_DISCM),
            Self::CloseSupportMintAssociated => {
                writer.write_all(&CLOSE_SUPPORT_MINT_ASSOCIATED_IX_DISCM)
            }
            Self::CollectCreatorFee => writer.write_all(&COLLECT_CREATOR_FEE_IX_DISCM),
            Self::CollectCreatorFeePermissionless => {
                writer.write_all(&COLLECT_CREATOR_FEE_PERMISSIONLESS_IX_DISCM)
            }
            Self::CollectExcessLamports => {
                writer.write_all(&COLLECT_EXCESS_LAMPORTS_IX_DISCM)
            }
            Self::CollectFundFee(args) => {
                writer.write_all(&COLLECT_FUND_FEE_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.amount_0_requested, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.amount_1_requested, &mut writer)?;
                Ok(())
            }
            Self::CollectProtocolFee(args) => {
                writer.write_all(&COLLECT_PROTOCOL_FEE_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.amount_0_requested, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.amount_1_requested, &mut writer)?;
                Ok(())
            }
            Self::CreateAmmConfig(args) => {
                writer.write_all(&CREATE_AMM_CONFIG_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.index, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.trade_fee_rate, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.protocol_fee_rate, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.fund_fee_rate, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.create_pool_fee, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.creator_fee_rate, &mut writer)?;
                Ok(())
            }
            Self::CreatePermissionPda => {
                writer.write_all(&CREATE_PERMISSION_PDA_IX_DISCM)
            }
            Self::CreateSupportMintAssociated => {
                writer.write_all(&CREATE_SUPPORT_MINT_ASSOCIATED_IX_DISCM)
            }
            Self::Deposit(args) => {
                writer.write_all(&DEPOSIT_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.lp_token_amount, &mut writer)?;
                borsh::BorshSerialize::serialize(
                    &args.maximum_token_0_amount,
                    &mut writer,
                )?;
                borsh::BorshSerialize::serialize(
                    &args.maximum_token_1_amount,
                    &mut writer,
                )?;
                Ok(())
            }
            Self::Initialize(args) => {
                writer.write_all(&INITIALIZE_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.init_amount_0, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.init_amount_1, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.open_time, &mut writer)?;
                Ok(())
            }
            Self::InitializeWithPermission(args) => {
                writer.write_all(&INITIALIZE_WITH_PERMISSION_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.init_amount_0, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.init_amount_1, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.open_time, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.creator_fee_on, &mut writer)?;
                Ok(())
            }
            Self::SwapBaseInput(args) => {
                writer.write_all(&SWAP_BASE_INPUT_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.amount_in, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.minimum_amount_out, &mut writer)?;
                Ok(())
            }
            Self::SwapBaseOutput(args) => {
                writer.write_all(&SWAP_BASE_OUTPUT_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.max_amount_in, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.amount_out, &mut writer)?;
                Ok(())
            }
            Self::UpdateAmmConfig(args) => {
                writer.write_all(&UPDATE_AMM_CONFIG_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.param, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.value, &mut writer)?;
                Ok(())
            }
            Self::UpdatePoolStatus(args) => {
                writer.write_all(&UPDATE_POOL_STATUS_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.status, &mut writer)?;
                Ok(())
            }
            Self::Withdraw(args) => {
                writer.write_all(&WITHDRAW_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.lp_token_amount, &mut writer)?;
                borsh::BorshSerialize::serialize(
                    &args.minimum_token_0_amount,
                    &mut writer,
                )?;
                borsh::BorshSerialize::serialize(
                    &args.minimum_token_1_amount,
                    &mut writer,
                )?;
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
pub const CLOSE_PERMISSION_PDA_IX_ACCOUNTS_LEN: usize = 4;
#[derive(Copy, Clone, Debug)]
pub struct ClosePermissionPdaAccounts<'me, 'info> {
    pub owner: &'me AccountInfo<'info>,
    pub permission_authority: &'me AccountInfo<'info>,
    pub permission: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ClosePermissionPdaKeys {
    pub owner: Pubkey,
    pub permission_authority: Pubkey,
    pub permission: Pubkey,
    pub system_program: Pubkey,
}
impl From<ClosePermissionPdaAccounts<'_, '_>> for ClosePermissionPdaKeys {
    fn from(accounts: ClosePermissionPdaAccounts) -> Self {
        Self {
            owner: *accounts.owner.key,
            permission_authority: *accounts.permission_authority.key,
            permission: *accounts.permission.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<ClosePermissionPdaKeys>
for [AccountMeta; CLOSE_PERMISSION_PDA_IX_ACCOUNTS_LEN] {
    fn from(keys: ClosePermissionPdaKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.owner,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.permission_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.permission,
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
impl From<[Pubkey; CLOSE_PERMISSION_PDA_IX_ACCOUNTS_LEN]> for ClosePermissionPdaKeys {
    fn from(pubkeys: [Pubkey; CLOSE_PERMISSION_PDA_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            owner: pubkeys[0],
            permission_authority: pubkeys[1],
            permission: pubkeys[2],
            system_program: pubkeys[3],
        }
    }
}
impl<'info> From<ClosePermissionPdaAccounts<'_, 'info>>
for [AccountInfo<'info>; CLOSE_PERMISSION_PDA_IX_ACCOUNTS_LEN] {
    fn from(accounts: ClosePermissionPdaAccounts<'_, 'info>) -> Self {
        [
            accounts.owner.clone(),
            accounts.permission_authority.clone(),
            accounts.permission.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CLOSE_PERMISSION_PDA_IX_ACCOUNTS_LEN]>
for ClosePermissionPdaAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; CLOSE_PERMISSION_PDA_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            owner: &arr[0],
            permission_authority: &arr[1],
            permission: &arr[2],
            system_program: &arr[3],
        }
    }
}
pub const CLOSE_PERMISSION_PDA_IX_DISCM: [u8; 8usize] = [
    156, 84, 32, 118, 69, 135, 70, 123,
];
#[derive(Clone, Debug, PartialEq)]
pub struct ClosePermissionPdaIxData;
impl ClosePermissionPdaIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CLOSE_PERMISSION_PDA_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CLOSE_PERMISSION_PDA_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn close_permission_pda_ix_with_program_id(
    program_id: Pubkey,
    keys: ClosePermissionPdaKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CLOSE_PERMISSION_PDA_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: ClosePermissionPdaIxData.try_to_vec()?,
    })
}
pub fn close_permission_pda_ix(
    keys: ClosePermissionPdaKeys,
) -> std::io::Result<Instruction> {
    close_permission_pda_ix_with_program_id(RAYDIUM_CP_SWAP_PROGRAM_ID, keys)
}
pub fn close_permission_pda_invoke_with_program_id(
    program_id: Pubkey,
    accounts: ClosePermissionPdaAccounts<'_, '_>,
) -> ProgramResult {
    let keys: ClosePermissionPdaKeys = accounts.into();
    let ix = close_permission_pda_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn close_permission_pda_invoke(
    accounts: ClosePermissionPdaAccounts<'_, '_>,
) -> ProgramResult {
    close_permission_pda_invoke_with_program_id(RAYDIUM_CP_SWAP_PROGRAM_ID, accounts)
}
pub fn close_permission_pda_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: ClosePermissionPdaAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: ClosePermissionPdaKeys = accounts.into();
    let ix = close_permission_pda_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn close_permission_pda_invoke_signed(
    accounts: ClosePermissionPdaAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    close_permission_pda_invoke_signed_with_program_id(
        RAYDIUM_CP_SWAP_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn close_permission_pda_verify_account_keys(
    accounts: ClosePermissionPdaAccounts<'_, '_>,
    keys: ClosePermissionPdaKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.owner.key, keys.owner),
        (*accounts.permission_authority.key, keys.permission_authority),
        (*accounts.permission.key, keys.permission),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn close_permission_pda_verify_writable_privileges<'me, 'info>(
    accounts: ClosePermissionPdaAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.owner, accounts.permission] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn close_permission_pda_verify_signer_privileges<'me, 'info>(
    accounts: ClosePermissionPdaAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.owner] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn close_permission_pda_verify_account_privileges<'me, 'info>(
    accounts: ClosePermissionPdaAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    close_permission_pda_verify_writable_privileges(accounts)?;
    close_permission_pda_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CLOSE_SUPPORT_MINT_ASSOCIATED_IX_ACCOUNTS_LEN: usize = 4;
#[derive(Copy, Clone, Debug)]
pub struct CloseSupportMintAssociatedAccounts<'me, 'info> {
    pub owner: &'me AccountInfo<'info>,
    pub token_mint: &'me AccountInfo<'info>,
    pub support_mint_associated: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CloseSupportMintAssociatedKeys {
    pub owner: Pubkey,
    pub token_mint: Pubkey,
    pub support_mint_associated: Pubkey,
    pub system_program: Pubkey,
}
impl From<CloseSupportMintAssociatedAccounts<'_, '_>>
for CloseSupportMintAssociatedKeys {
    fn from(accounts: CloseSupportMintAssociatedAccounts) -> Self {
        Self {
            owner: *accounts.owner.key,
            token_mint: *accounts.token_mint.key,
            support_mint_associated: *accounts.support_mint_associated.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<CloseSupportMintAssociatedKeys>
for [AccountMeta; CLOSE_SUPPORT_MINT_ASSOCIATED_IX_ACCOUNTS_LEN] {
    fn from(keys: CloseSupportMintAssociatedKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.owner,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.support_mint_associated,
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
impl From<[Pubkey; CLOSE_SUPPORT_MINT_ASSOCIATED_IX_ACCOUNTS_LEN]>
for CloseSupportMintAssociatedKeys {
    fn from(pubkeys: [Pubkey; CLOSE_SUPPORT_MINT_ASSOCIATED_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            owner: pubkeys[0],
            token_mint: pubkeys[1],
            support_mint_associated: pubkeys[2],
            system_program: pubkeys[3],
        }
    }
}
impl<'info> From<CloseSupportMintAssociatedAccounts<'_, 'info>>
for [AccountInfo<'info>; CLOSE_SUPPORT_MINT_ASSOCIATED_IX_ACCOUNTS_LEN] {
    fn from(accounts: CloseSupportMintAssociatedAccounts<'_, 'info>) -> Self {
        [
            accounts.owner.clone(),
            accounts.token_mint.clone(),
            accounts.support_mint_associated.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; CLOSE_SUPPORT_MINT_ASSOCIATED_IX_ACCOUNTS_LEN]>
for CloseSupportMintAssociatedAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; CLOSE_SUPPORT_MINT_ASSOCIATED_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            owner: &arr[0],
            token_mint: &arr[1],
            support_mint_associated: &arr[2],
            system_program: &arr[3],
        }
    }
}
pub const CLOSE_SUPPORT_MINT_ASSOCIATED_IX_DISCM: [u8; 8usize] = [
    96, 136, 183, 99, 72, 152, 54, 131,
];
#[derive(Clone, Debug, PartialEq)]
pub struct CloseSupportMintAssociatedIxData;
impl CloseSupportMintAssociatedIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CLOSE_SUPPORT_MINT_ASSOCIATED_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CLOSE_SUPPORT_MINT_ASSOCIATED_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn close_support_mint_associated_ix_with_program_id(
    program_id: Pubkey,
    keys: CloseSupportMintAssociatedKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CLOSE_SUPPORT_MINT_ASSOCIATED_IX_ACCOUNTS_LEN] = keys
        .into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: CloseSupportMintAssociatedIxData.try_to_vec()?,
    })
}
pub fn close_support_mint_associated_ix(
    keys: CloseSupportMintAssociatedKeys,
) -> std::io::Result<Instruction> {
    close_support_mint_associated_ix_with_program_id(RAYDIUM_CP_SWAP_PROGRAM_ID, keys)
}
pub fn close_support_mint_associated_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CloseSupportMintAssociatedAccounts<'_, '_>,
) -> ProgramResult {
    let keys: CloseSupportMintAssociatedKeys = accounts.into();
    let ix = close_support_mint_associated_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn close_support_mint_associated_invoke(
    accounts: CloseSupportMintAssociatedAccounts<'_, '_>,
) -> ProgramResult {
    close_support_mint_associated_invoke_with_program_id(
        RAYDIUM_CP_SWAP_PROGRAM_ID,
        accounts,
    )
}
pub fn close_support_mint_associated_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CloseSupportMintAssociatedAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CloseSupportMintAssociatedKeys = accounts.into();
    let ix = close_support_mint_associated_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn close_support_mint_associated_invoke_signed(
    accounts: CloseSupportMintAssociatedAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    close_support_mint_associated_invoke_signed_with_program_id(
        RAYDIUM_CP_SWAP_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn close_support_mint_associated_verify_account_keys(
    accounts: CloseSupportMintAssociatedAccounts<'_, '_>,
    keys: CloseSupportMintAssociatedKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.owner.key, keys.owner),
        (*accounts.token_mint.key, keys.token_mint),
        (*accounts.support_mint_associated.key, keys.support_mint_associated),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn close_support_mint_associated_verify_writable_privileges<'me, 'info>(
    accounts: CloseSupportMintAssociatedAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.owner, accounts.support_mint_associated] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn close_support_mint_associated_verify_signer_privileges<'me, 'info>(
    accounts: CloseSupportMintAssociatedAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.owner] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn close_support_mint_associated_verify_account_privileges<'me, 'info>(
    accounts: CloseSupportMintAssociatedAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    close_support_mint_associated_verify_writable_privileges(accounts)?;
    close_support_mint_associated_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const COLLECT_CREATOR_FEE_IX_ACCOUNTS_LEN: usize = 14;
#[derive(Copy, Clone, Debug)]
pub struct CollectCreatorFeeAccounts<'me, 'info> {
    pub creator: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub pool_state: &'me AccountInfo<'info>,
    pub amm_config: &'me AccountInfo<'info>,
    pub token_0_vault: &'me AccountInfo<'info>,
    pub token_1_vault: &'me AccountInfo<'info>,
    pub vault_0_mint: &'me AccountInfo<'info>,
    pub vault_1_mint: &'me AccountInfo<'info>,
    pub creator_token_0: &'me AccountInfo<'info>,
    pub creator_token_1: &'me AccountInfo<'info>,
    pub token_0_program: &'me AccountInfo<'info>,
    pub token_1_program: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CollectCreatorFeeKeys {
    pub creator: Pubkey,
    pub authority: Pubkey,
    pub pool_state: Pubkey,
    pub amm_config: Pubkey,
    pub token_0_vault: Pubkey,
    pub token_1_vault: Pubkey,
    pub vault_0_mint: Pubkey,
    pub vault_1_mint: Pubkey,
    pub creator_token_0: Pubkey,
    pub creator_token_1: Pubkey,
    pub token_0_program: Pubkey,
    pub token_1_program: Pubkey,
    pub associated_token_program: Pubkey,
    pub system_program: Pubkey,
}
impl From<CollectCreatorFeeAccounts<'_, '_>> for CollectCreatorFeeKeys {
    fn from(accounts: CollectCreatorFeeAccounts) -> Self {
        Self {
            creator: *accounts.creator.key,
            authority: *accounts.authority.key,
            pool_state: *accounts.pool_state.key,
            amm_config: *accounts.amm_config.key,
            token_0_vault: *accounts.token_0_vault.key,
            token_1_vault: *accounts.token_1_vault.key,
            vault_0_mint: *accounts.vault_0_mint.key,
            vault_1_mint: *accounts.vault_1_mint.key,
            creator_token_0: *accounts.creator_token_0.key,
            creator_token_1: *accounts.creator_token_1.key,
            token_0_program: *accounts.token_0_program.key,
            token_1_program: *accounts.token_1_program.key,
            associated_token_program: *accounts.associated_token_program.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<CollectCreatorFeeKeys> for [AccountMeta; COLLECT_CREATOR_FEE_IX_ACCOUNTS_LEN] {
    fn from(keys: CollectCreatorFeeKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.creator,
                is_signer: true,
                is_writable: true,
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
                pubkey: keys.amm_config,
                is_signer: false,
                is_writable: false,
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
        ]
    }
}
impl From<[Pubkey; COLLECT_CREATOR_FEE_IX_ACCOUNTS_LEN]> for CollectCreatorFeeKeys {
    fn from(pubkeys: [Pubkey; COLLECT_CREATOR_FEE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            creator: pubkeys[0],
            authority: pubkeys[1],
            pool_state: pubkeys[2],
            amm_config: pubkeys[3],
            token_0_vault: pubkeys[4],
            token_1_vault: pubkeys[5],
            vault_0_mint: pubkeys[6],
            vault_1_mint: pubkeys[7],
            creator_token_0: pubkeys[8],
            creator_token_1: pubkeys[9],
            token_0_program: pubkeys[10],
            token_1_program: pubkeys[11],
            associated_token_program: pubkeys[12],
            system_program: pubkeys[13],
        }
    }
}
impl<'info> From<CollectCreatorFeeAccounts<'_, 'info>>
for [AccountInfo<'info>; COLLECT_CREATOR_FEE_IX_ACCOUNTS_LEN] {
    fn from(accounts: CollectCreatorFeeAccounts<'_, 'info>) -> Self {
        [
            accounts.creator.clone(),
            accounts.authority.clone(),
            accounts.pool_state.clone(),
            accounts.amm_config.clone(),
            accounts.token_0_vault.clone(),
            accounts.token_1_vault.clone(),
            accounts.vault_0_mint.clone(),
            accounts.vault_1_mint.clone(),
            accounts.creator_token_0.clone(),
            accounts.creator_token_1.clone(),
            accounts.token_0_program.clone(),
            accounts.token_1_program.clone(),
            accounts.associated_token_program.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; COLLECT_CREATOR_FEE_IX_ACCOUNTS_LEN]>
for CollectCreatorFeeAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; COLLECT_CREATOR_FEE_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            creator: &arr[0],
            authority: &arr[1],
            pool_state: &arr[2],
            amm_config: &arr[3],
            token_0_vault: &arr[4],
            token_1_vault: &arr[5],
            vault_0_mint: &arr[6],
            vault_1_mint: &arr[7],
            creator_token_0: &arr[8],
            creator_token_1: &arr[9],
            token_0_program: &arr[10],
            token_1_program: &arr[11],
            associated_token_program: &arr[12],
            system_program: &arr[13],
        }
    }
}
pub const COLLECT_CREATOR_FEE_IX_DISCM: [u8; 8usize] = [
    20, 22, 86, 123, 198, 28, 219, 132,
];
#[derive(Clone, Debug, PartialEq)]
pub struct CollectCreatorFeeIxData;
impl CollectCreatorFeeIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != COLLECT_CREATOR_FEE_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&COLLECT_CREATOR_FEE_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn collect_creator_fee_ix_with_program_id(
    program_id: Pubkey,
    keys: CollectCreatorFeeKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; COLLECT_CREATOR_FEE_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: CollectCreatorFeeIxData.try_to_vec()?,
    })
}
pub fn collect_creator_fee_ix(
    keys: CollectCreatorFeeKeys,
) -> std::io::Result<Instruction> {
    collect_creator_fee_ix_with_program_id(RAYDIUM_CP_SWAP_PROGRAM_ID, keys)
}
pub fn collect_creator_fee_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CollectCreatorFeeAccounts<'_, '_>,
) -> ProgramResult {
    let keys: CollectCreatorFeeKeys = accounts.into();
    let ix = collect_creator_fee_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn collect_creator_fee_invoke(
    accounts: CollectCreatorFeeAccounts<'_, '_>,
) -> ProgramResult {
    collect_creator_fee_invoke_with_program_id(RAYDIUM_CP_SWAP_PROGRAM_ID, accounts)
}
pub fn collect_creator_fee_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CollectCreatorFeeAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CollectCreatorFeeKeys = accounts.into();
    let ix = collect_creator_fee_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn collect_creator_fee_invoke_signed(
    accounts: CollectCreatorFeeAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    collect_creator_fee_invoke_signed_with_program_id(
        RAYDIUM_CP_SWAP_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn collect_creator_fee_verify_account_keys(
    accounts: CollectCreatorFeeAccounts<'_, '_>,
    keys: CollectCreatorFeeKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.creator.key, keys.creator),
        (*accounts.authority.key, keys.authority),
        (*accounts.pool_state.key, keys.pool_state),
        (*accounts.amm_config.key, keys.amm_config),
        (*accounts.token_0_vault.key, keys.token_0_vault),
        (*accounts.token_1_vault.key, keys.token_1_vault),
        (*accounts.vault_0_mint.key, keys.vault_0_mint),
        (*accounts.vault_1_mint.key, keys.vault_1_mint),
        (*accounts.creator_token_0.key, keys.creator_token_0),
        (*accounts.creator_token_1.key, keys.creator_token_1),
        (*accounts.token_0_program.key, keys.token_0_program),
        (*accounts.token_1_program.key, keys.token_1_program),
        (*accounts.associated_token_program.key, keys.associated_token_program),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn collect_creator_fee_verify_writable_privileges<'me, 'info>(
    accounts: CollectCreatorFeeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.creator,
        accounts.pool_state,
        accounts.token_0_vault,
        accounts.token_1_vault,
        accounts.creator_token_0,
        accounts.creator_token_1,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn collect_creator_fee_verify_signer_privileges<'me, 'info>(
    accounts: CollectCreatorFeeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.creator] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn collect_creator_fee_verify_account_privileges<'me, 'info>(
    accounts: CollectCreatorFeeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    collect_creator_fee_verify_writable_privileges(accounts)?;
    collect_creator_fee_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const COLLECT_CREATOR_FEE_PERMISSIONLESS_IX_ACCOUNTS_LEN: usize = 14;
#[derive(Copy, Clone, Debug)]
pub struct CollectCreatorFeePermissionlessAccounts<'me, 'info> {
    pub payer: &'me AccountInfo<'info>,
    pub creator: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub pool_state: &'me AccountInfo<'info>,
    pub token_0_vault: &'me AccountInfo<'info>,
    pub token_1_vault: &'me AccountInfo<'info>,
    pub vault_0_mint: &'me AccountInfo<'info>,
    pub vault_1_mint: &'me AccountInfo<'info>,
    pub creator_token_0: &'me AccountInfo<'info>,
    pub creator_token_1: &'me AccountInfo<'info>,
    pub token_0_program: &'me AccountInfo<'info>,
    pub token_1_program: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CollectCreatorFeePermissionlessKeys {
    pub payer: Pubkey,
    pub creator: Pubkey,
    pub authority: Pubkey,
    pub pool_state: Pubkey,
    pub token_0_vault: Pubkey,
    pub token_1_vault: Pubkey,
    pub vault_0_mint: Pubkey,
    pub vault_1_mint: Pubkey,
    pub creator_token_0: Pubkey,
    pub creator_token_1: Pubkey,
    pub token_0_program: Pubkey,
    pub token_1_program: Pubkey,
    pub associated_token_program: Pubkey,
    pub system_program: Pubkey,
}
impl From<CollectCreatorFeePermissionlessAccounts<'_, '_>>
for CollectCreatorFeePermissionlessKeys {
    fn from(accounts: CollectCreatorFeePermissionlessAccounts) -> Self {
        Self {
            payer: *accounts.payer.key,
            creator: *accounts.creator.key,
            authority: *accounts.authority.key,
            pool_state: *accounts.pool_state.key,
            token_0_vault: *accounts.token_0_vault.key,
            token_1_vault: *accounts.token_1_vault.key,
            vault_0_mint: *accounts.vault_0_mint.key,
            vault_1_mint: *accounts.vault_1_mint.key,
            creator_token_0: *accounts.creator_token_0.key,
            creator_token_1: *accounts.creator_token_1.key,
            token_0_program: *accounts.token_0_program.key,
            token_1_program: *accounts.token_1_program.key,
            associated_token_program: *accounts.associated_token_program.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<CollectCreatorFeePermissionlessKeys>
for [AccountMeta; COLLECT_CREATOR_FEE_PERMISSIONLESS_IX_ACCOUNTS_LEN] {
    fn from(keys: CollectCreatorFeePermissionlessKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.payer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.creator,
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
        ]
    }
}
impl From<[Pubkey; COLLECT_CREATOR_FEE_PERMISSIONLESS_IX_ACCOUNTS_LEN]>
for CollectCreatorFeePermissionlessKeys {
    fn from(
        pubkeys: [Pubkey; COLLECT_CREATOR_FEE_PERMISSIONLESS_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            payer: pubkeys[0],
            creator: pubkeys[1],
            authority: pubkeys[2],
            pool_state: pubkeys[3],
            token_0_vault: pubkeys[4],
            token_1_vault: pubkeys[5],
            vault_0_mint: pubkeys[6],
            vault_1_mint: pubkeys[7],
            creator_token_0: pubkeys[8],
            creator_token_1: pubkeys[9],
            token_0_program: pubkeys[10],
            token_1_program: pubkeys[11],
            associated_token_program: pubkeys[12],
            system_program: pubkeys[13],
        }
    }
}
impl<'info> From<CollectCreatorFeePermissionlessAccounts<'_, 'info>>
for [AccountInfo<'info>; COLLECT_CREATOR_FEE_PERMISSIONLESS_IX_ACCOUNTS_LEN] {
    fn from(accounts: CollectCreatorFeePermissionlessAccounts<'_, 'info>) -> Self {
        [
            accounts.payer.clone(),
            accounts.creator.clone(),
            accounts.authority.clone(),
            accounts.pool_state.clone(),
            accounts.token_0_vault.clone(),
            accounts.token_1_vault.clone(),
            accounts.vault_0_mint.clone(),
            accounts.vault_1_mint.clone(),
            accounts.creator_token_0.clone(),
            accounts.creator_token_1.clone(),
            accounts.token_0_program.clone(),
            accounts.token_1_program.clone(),
            accounts.associated_token_program.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; COLLECT_CREATOR_FEE_PERMISSIONLESS_IX_ACCOUNTS_LEN]>
for CollectCreatorFeePermissionlessAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<
            'info,
        >; COLLECT_CREATOR_FEE_PERMISSIONLESS_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            payer: &arr[0],
            creator: &arr[1],
            authority: &arr[2],
            pool_state: &arr[3],
            token_0_vault: &arr[4],
            token_1_vault: &arr[5],
            vault_0_mint: &arr[6],
            vault_1_mint: &arr[7],
            creator_token_0: &arr[8],
            creator_token_1: &arr[9],
            token_0_program: &arr[10],
            token_1_program: &arr[11],
            associated_token_program: &arr[12],
            system_program: &arr[13],
        }
    }
}
pub const COLLECT_CREATOR_FEE_PERMISSIONLESS_IX_DISCM: [u8; 8usize] = [
    202, 202, 34, 83, 226, 122, 145, 229,
];
#[derive(Clone, Debug, PartialEq)]
pub struct CollectCreatorFeePermissionlessIxData;
impl CollectCreatorFeePermissionlessIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != COLLECT_CREATOR_FEE_PERMISSIONLESS_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&COLLECT_CREATOR_FEE_PERMISSIONLESS_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn collect_creator_fee_permissionless_ix_with_program_id(
    program_id: Pubkey,
    keys: CollectCreatorFeePermissionlessKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; COLLECT_CREATOR_FEE_PERMISSIONLESS_IX_ACCOUNTS_LEN] = keys
        .into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: CollectCreatorFeePermissionlessIxData.try_to_vec()?,
    })
}
pub fn collect_creator_fee_permissionless_ix(
    keys: CollectCreatorFeePermissionlessKeys,
) -> std::io::Result<Instruction> {
    collect_creator_fee_permissionless_ix_with_program_id(
        RAYDIUM_CP_SWAP_PROGRAM_ID,
        keys,
    )
}
pub fn collect_creator_fee_permissionless_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CollectCreatorFeePermissionlessAccounts<'_, '_>,
) -> ProgramResult {
    let keys: CollectCreatorFeePermissionlessKeys = accounts.into();
    let ix = collect_creator_fee_permissionless_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn collect_creator_fee_permissionless_invoke(
    accounts: CollectCreatorFeePermissionlessAccounts<'_, '_>,
) -> ProgramResult {
    collect_creator_fee_permissionless_invoke_with_program_id(
        RAYDIUM_CP_SWAP_PROGRAM_ID,
        accounts,
    )
}
pub fn collect_creator_fee_permissionless_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CollectCreatorFeePermissionlessAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CollectCreatorFeePermissionlessKeys = accounts.into();
    let ix = collect_creator_fee_permissionless_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn collect_creator_fee_permissionless_invoke_signed(
    accounts: CollectCreatorFeePermissionlessAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    collect_creator_fee_permissionless_invoke_signed_with_program_id(
        RAYDIUM_CP_SWAP_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn collect_creator_fee_permissionless_verify_account_keys(
    accounts: CollectCreatorFeePermissionlessAccounts<'_, '_>,
    keys: CollectCreatorFeePermissionlessKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.payer.key, keys.payer),
        (*accounts.creator.key, keys.creator),
        (*accounts.authority.key, keys.authority),
        (*accounts.pool_state.key, keys.pool_state),
        (*accounts.token_0_vault.key, keys.token_0_vault),
        (*accounts.token_1_vault.key, keys.token_1_vault),
        (*accounts.vault_0_mint.key, keys.vault_0_mint),
        (*accounts.vault_1_mint.key, keys.vault_1_mint),
        (*accounts.creator_token_0.key, keys.creator_token_0),
        (*accounts.creator_token_1.key, keys.creator_token_1),
        (*accounts.token_0_program.key, keys.token_0_program),
        (*accounts.token_1_program.key, keys.token_1_program),
        (*accounts.associated_token_program.key, keys.associated_token_program),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn collect_creator_fee_permissionless_verify_writable_privileges<'me, 'info>(
    accounts: CollectCreatorFeePermissionlessAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.payer,
        accounts.pool_state,
        accounts.token_0_vault,
        accounts.token_1_vault,
        accounts.creator_token_0,
        accounts.creator_token_1,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn collect_creator_fee_permissionless_verify_signer_privileges<'me, 'info>(
    accounts: CollectCreatorFeePermissionlessAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.payer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn collect_creator_fee_permissionless_verify_account_privileges<'me, 'info>(
    accounts: CollectCreatorFeePermissionlessAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    collect_creator_fee_permissionless_verify_writable_privileges(accounts)?;
    collect_creator_fee_permissionless_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const COLLECT_EXCESS_LAMPORTS_IX_ACCOUNTS_LEN: usize = 4;
#[derive(Copy, Clone, Debug)]
pub struct CollectExcessLamportsAccounts<'me, 'info> {
    pub collect_lamports_wallet: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub token_program_2022: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CollectExcessLamportsKeys {
    pub collect_lamports_wallet: Pubkey,
    pub authority: Pubkey,
    pub token_program: Pubkey,
    pub token_program_2022: Pubkey,
}
impl From<CollectExcessLamportsAccounts<'_, '_>> for CollectExcessLamportsKeys {
    fn from(accounts: CollectExcessLamportsAccounts) -> Self {
        Self {
            collect_lamports_wallet: *accounts.collect_lamports_wallet.key,
            authority: *accounts.authority.key,
            token_program: *accounts.token_program.key,
            token_program_2022: *accounts.token_program_2022.key,
        }
    }
}
impl From<CollectExcessLamportsKeys>
for [AccountMeta; COLLECT_EXCESS_LAMPORTS_IX_ACCOUNTS_LEN] {
    fn from(keys: CollectExcessLamportsKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.collect_lamports_wallet,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.authority,
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
        ]
    }
}
impl From<[Pubkey; COLLECT_EXCESS_LAMPORTS_IX_ACCOUNTS_LEN]>
for CollectExcessLamportsKeys {
    fn from(pubkeys: [Pubkey; COLLECT_EXCESS_LAMPORTS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            collect_lamports_wallet: pubkeys[0],
            authority: pubkeys[1],
            token_program: pubkeys[2],
            token_program_2022: pubkeys[3],
        }
    }
}
impl<'info> From<CollectExcessLamportsAccounts<'_, 'info>>
for [AccountInfo<'info>; COLLECT_EXCESS_LAMPORTS_IX_ACCOUNTS_LEN] {
    fn from(accounts: CollectExcessLamportsAccounts<'_, 'info>) -> Self {
        [
            accounts.collect_lamports_wallet.clone(),
            accounts.authority.clone(),
            accounts.token_program.clone(),
            accounts.token_program_2022.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; COLLECT_EXCESS_LAMPORTS_IX_ACCOUNTS_LEN]>
for CollectExcessLamportsAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; COLLECT_EXCESS_LAMPORTS_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            collect_lamports_wallet: &arr[0],
            authority: &arr[1],
            token_program: &arr[2],
            token_program_2022: &arr[3],
        }
    }
}
pub const COLLECT_EXCESS_LAMPORTS_IX_DISCM: [u8; 8usize] = [
    28, 189, 19, 40, 164, 176, 118, 17,
];
#[derive(Clone, Debug, PartialEq)]
pub struct CollectExcessLamportsIxData;
impl CollectExcessLamportsIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != COLLECT_EXCESS_LAMPORTS_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&COLLECT_EXCESS_LAMPORTS_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn collect_excess_lamports_ix_with_program_id(
    program_id: Pubkey,
    keys: CollectExcessLamportsKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; COLLECT_EXCESS_LAMPORTS_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: CollectExcessLamportsIxData.try_to_vec()?,
    })
}
pub fn collect_excess_lamports_ix(
    keys: CollectExcessLamportsKeys,
) -> std::io::Result<Instruction> {
    collect_excess_lamports_ix_with_program_id(RAYDIUM_CP_SWAP_PROGRAM_ID, keys)
}
pub fn collect_excess_lamports_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CollectExcessLamportsAccounts<'_, '_>,
) -> ProgramResult {
    let keys: CollectExcessLamportsKeys = accounts.into();
    let ix = collect_excess_lamports_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn collect_excess_lamports_invoke(
    accounts: CollectExcessLamportsAccounts<'_, '_>,
) -> ProgramResult {
    collect_excess_lamports_invoke_with_program_id(RAYDIUM_CP_SWAP_PROGRAM_ID, accounts)
}
pub fn collect_excess_lamports_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CollectExcessLamportsAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CollectExcessLamportsKeys = accounts.into();
    let ix = collect_excess_lamports_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn collect_excess_lamports_invoke_signed(
    accounts: CollectExcessLamportsAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    collect_excess_lamports_invoke_signed_with_program_id(
        RAYDIUM_CP_SWAP_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn collect_excess_lamports_verify_account_keys(
    accounts: CollectExcessLamportsAccounts<'_, '_>,
    keys: CollectExcessLamportsKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.collect_lamports_wallet.key, keys.collect_lamports_wallet),
        (*accounts.authority.key, keys.authority),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.token_program_2022.key, keys.token_program_2022),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn collect_excess_lamports_verify_writable_privileges<'me, 'info>(
    accounts: CollectExcessLamportsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.collect_lamports_wallet] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn collect_excess_lamports_verify_signer_privileges<'me, 'info>(
    accounts: CollectExcessLamportsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.collect_lamports_wallet] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn collect_excess_lamports_verify_account_privileges<'me, 'info>(
    accounts: CollectExcessLamportsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    collect_excess_lamports_verify_writable_privileges(accounts)?;
    collect_excess_lamports_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const COLLECT_FUND_FEE_IX_ACCOUNTS_LEN: usize = 12;
#[derive(Copy, Clone, Debug)]
pub struct CollectFundFeeAccounts<'me, 'info> {
    pub owner: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub pool_state: &'me AccountInfo<'info>,
    pub amm_config: &'me AccountInfo<'info>,
    pub token_0_vault: &'me AccountInfo<'info>,
    pub token_1_vault: &'me AccountInfo<'info>,
    pub vault_0_mint: &'me AccountInfo<'info>,
    pub vault_1_mint: &'me AccountInfo<'info>,
    pub recipient_token_0_account: &'me AccountInfo<'info>,
    pub recipient_token_1_account: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub token_program_2022: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CollectFundFeeKeys {
    pub owner: Pubkey,
    pub authority: Pubkey,
    pub pool_state: Pubkey,
    pub amm_config: Pubkey,
    pub token_0_vault: Pubkey,
    pub token_1_vault: Pubkey,
    pub vault_0_mint: Pubkey,
    pub vault_1_mint: Pubkey,
    pub recipient_token_0_account: Pubkey,
    pub recipient_token_1_account: Pubkey,
    pub token_program: Pubkey,
    pub token_program_2022: Pubkey,
}
impl From<CollectFundFeeAccounts<'_, '_>> for CollectFundFeeKeys {
    fn from(accounts: CollectFundFeeAccounts) -> Self {
        Self {
            owner: *accounts.owner.key,
            authority: *accounts.authority.key,
            pool_state: *accounts.pool_state.key,
            amm_config: *accounts.amm_config.key,
            token_0_vault: *accounts.token_0_vault.key,
            token_1_vault: *accounts.token_1_vault.key,
            vault_0_mint: *accounts.vault_0_mint.key,
            vault_1_mint: *accounts.vault_1_mint.key,
            recipient_token_0_account: *accounts.recipient_token_0_account.key,
            recipient_token_1_account: *accounts.recipient_token_1_account.key,
            token_program: *accounts.token_program.key,
            token_program_2022: *accounts.token_program_2022.key,
        }
    }
}
impl From<CollectFundFeeKeys> for [AccountMeta; COLLECT_FUND_FEE_IX_ACCOUNTS_LEN] {
    fn from(keys: CollectFundFeeKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.owner,
                is_signer: true,
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
                pubkey: keys.amm_config,
                is_signer: false,
                is_writable: false,
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
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault_1_mint,
                is_signer: false,
                is_writable: false,
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
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_program_2022,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; COLLECT_FUND_FEE_IX_ACCOUNTS_LEN]> for CollectFundFeeKeys {
    fn from(pubkeys: [Pubkey; COLLECT_FUND_FEE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            owner: pubkeys[0],
            authority: pubkeys[1],
            pool_state: pubkeys[2],
            amm_config: pubkeys[3],
            token_0_vault: pubkeys[4],
            token_1_vault: pubkeys[5],
            vault_0_mint: pubkeys[6],
            vault_1_mint: pubkeys[7],
            recipient_token_0_account: pubkeys[8],
            recipient_token_1_account: pubkeys[9],
            token_program: pubkeys[10],
            token_program_2022: pubkeys[11],
        }
    }
}
impl<'info> From<CollectFundFeeAccounts<'_, 'info>>
for [AccountInfo<'info>; COLLECT_FUND_FEE_IX_ACCOUNTS_LEN] {
    fn from(accounts: CollectFundFeeAccounts<'_, 'info>) -> Self {
        [
            accounts.owner.clone(),
            accounts.authority.clone(),
            accounts.pool_state.clone(),
            accounts.amm_config.clone(),
            accounts.token_0_vault.clone(),
            accounts.token_1_vault.clone(),
            accounts.vault_0_mint.clone(),
            accounts.vault_1_mint.clone(),
            accounts.recipient_token_0_account.clone(),
            accounts.recipient_token_1_account.clone(),
            accounts.token_program.clone(),
            accounts.token_program_2022.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; COLLECT_FUND_FEE_IX_ACCOUNTS_LEN]>
for CollectFundFeeAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; COLLECT_FUND_FEE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            owner: &arr[0],
            authority: &arr[1],
            pool_state: &arr[2],
            amm_config: &arr[3],
            token_0_vault: &arr[4],
            token_1_vault: &arr[5],
            vault_0_mint: &arr[6],
            vault_1_mint: &arr[7],
            recipient_token_0_account: &arr[8],
            recipient_token_1_account: &arr[9],
            token_program: &arr[10],
            token_program_2022: &arr[11],
        }
    }
}
pub const COLLECT_FUND_FEE_IX_DISCM: [u8; 8usize] = [
    167, 138, 78, 149, 223, 194, 6, 126,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CollectFundFeeIxArgs {
    pub amount_0_requested: u64,
    pub amount_1_requested: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct CollectFundFeeIxData(pub CollectFundFeeIxArgs);
impl From<CollectFundFeeIxArgs> for CollectFundFeeIxData {
    fn from(args: CollectFundFeeIxArgs) -> Self {
        Self(args)
    }
}
impl CollectFundFeeIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != COLLECT_FUND_FEE_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let amount_0_requested: u64 = crate::borsh_de_or_default(&mut reader)?;
        let amount_1_requested: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(CollectFundFeeIxArgs {
                amount_0_requested,
                amount_1_requested,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&COLLECT_FUND_FEE_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.amount_0_requested, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.amount_1_requested, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn collect_fund_fee_ix_with_program_id(
    program_id: Pubkey,
    keys: CollectFundFeeKeys,
    args: CollectFundFeeIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; COLLECT_FUND_FEE_IX_ACCOUNTS_LEN] = keys.into();
    let data: CollectFundFeeIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn collect_fund_fee_ix(
    keys: CollectFundFeeKeys,
    args: CollectFundFeeIxArgs,
) -> std::io::Result<Instruction> {
    collect_fund_fee_ix_with_program_id(RAYDIUM_CP_SWAP_PROGRAM_ID, keys, args)
}
pub fn collect_fund_fee_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CollectFundFeeAccounts<'_, '_>,
    args: CollectFundFeeIxArgs,
) -> ProgramResult {
    let keys: CollectFundFeeKeys = accounts.into();
    let ix = collect_fund_fee_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn collect_fund_fee_invoke(
    accounts: CollectFundFeeAccounts<'_, '_>,
    args: CollectFundFeeIxArgs,
) -> ProgramResult {
    collect_fund_fee_invoke_with_program_id(RAYDIUM_CP_SWAP_PROGRAM_ID, accounts, args)
}
pub fn collect_fund_fee_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CollectFundFeeAccounts<'_, '_>,
    args: CollectFundFeeIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CollectFundFeeKeys = accounts.into();
    let ix = collect_fund_fee_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn collect_fund_fee_invoke_signed(
    accounts: CollectFundFeeAccounts<'_, '_>,
    args: CollectFundFeeIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    collect_fund_fee_invoke_signed_with_program_id(
        RAYDIUM_CP_SWAP_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn collect_fund_fee_verify_account_keys(
    accounts: CollectFundFeeAccounts<'_, '_>,
    keys: CollectFundFeeKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.owner.key, keys.owner),
        (*accounts.authority.key, keys.authority),
        (*accounts.pool_state.key, keys.pool_state),
        (*accounts.amm_config.key, keys.amm_config),
        (*accounts.token_0_vault.key, keys.token_0_vault),
        (*accounts.token_1_vault.key, keys.token_1_vault),
        (*accounts.vault_0_mint.key, keys.vault_0_mint),
        (*accounts.vault_1_mint.key, keys.vault_1_mint),
        (*accounts.recipient_token_0_account.key, keys.recipient_token_0_account),
        (*accounts.recipient_token_1_account.key, keys.recipient_token_1_account),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.token_program_2022.key, keys.token_program_2022),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn collect_fund_fee_verify_writable_privileges<'me, 'info>(
    accounts: CollectFundFeeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.pool_state,
        accounts.token_0_vault,
        accounts.token_1_vault,
        accounts.recipient_token_0_account,
        accounts.recipient_token_1_account,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn collect_fund_fee_verify_signer_privileges<'me, 'info>(
    accounts: CollectFundFeeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.owner] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn collect_fund_fee_verify_account_privileges<'me, 'info>(
    accounts: CollectFundFeeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    collect_fund_fee_verify_writable_privileges(accounts)?;
    collect_fund_fee_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const COLLECT_PROTOCOL_FEE_IX_ACCOUNTS_LEN: usize = 12;
#[derive(Copy, Clone, Debug)]
pub struct CollectProtocolFeeAccounts<'me, 'info> {
    pub owner: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub pool_state: &'me AccountInfo<'info>,
    pub amm_config: &'me AccountInfo<'info>,
    pub token_0_vault: &'me AccountInfo<'info>,
    pub token_1_vault: &'me AccountInfo<'info>,
    pub vault_0_mint: &'me AccountInfo<'info>,
    pub vault_1_mint: &'me AccountInfo<'info>,
    pub recipient_token_0_account: &'me AccountInfo<'info>,
    pub recipient_token_1_account: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub token_program_2022: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CollectProtocolFeeKeys {
    pub owner: Pubkey,
    pub authority: Pubkey,
    pub pool_state: Pubkey,
    pub amm_config: Pubkey,
    pub token_0_vault: Pubkey,
    pub token_1_vault: Pubkey,
    pub vault_0_mint: Pubkey,
    pub vault_1_mint: Pubkey,
    pub recipient_token_0_account: Pubkey,
    pub recipient_token_1_account: Pubkey,
    pub token_program: Pubkey,
    pub token_program_2022: Pubkey,
}
impl From<CollectProtocolFeeAccounts<'_, '_>> for CollectProtocolFeeKeys {
    fn from(accounts: CollectProtocolFeeAccounts) -> Self {
        Self {
            owner: *accounts.owner.key,
            authority: *accounts.authority.key,
            pool_state: *accounts.pool_state.key,
            amm_config: *accounts.amm_config.key,
            token_0_vault: *accounts.token_0_vault.key,
            token_1_vault: *accounts.token_1_vault.key,
            vault_0_mint: *accounts.vault_0_mint.key,
            vault_1_mint: *accounts.vault_1_mint.key,
            recipient_token_0_account: *accounts.recipient_token_0_account.key,
            recipient_token_1_account: *accounts.recipient_token_1_account.key,
            token_program: *accounts.token_program.key,
            token_program_2022: *accounts.token_program_2022.key,
        }
    }
}
impl From<CollectProtocolFeeKeys>
for [AccountMeta; COLLECT_PROTOCOL_FEE_IX_ACCOUNTS_LEN] {
    fn from(keys: CollectProtocolFeeKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.owner,
                is_signer: true,
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
                pubkey: keys.amm_config,
                is_signer: false,
                is_writable: false,
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
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault_1_mint,
                is_signer: false,
                is_writable: false,
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
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_program_2022,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; COLLECT_PROTOCOL_FEE_IX_ACCOUNTS_LEN]> for CollectProtocolFeeKeys {
    fn from(pubkeys: [Pubkey; COLLECT_PROTOCOL_FEE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            owner: pubkeys[0],
            authority: pubkeys[1],
            pool_state: pubkeys[2],
            amm_config: pubkeys[3],
            token_0_vault: pubkeys[4],
            token_1_vault: pubkeys[5],
            vault_0_mint: pubkeys[6],
            vault_1_mint: pubkeys[7],
            recipient_token_0_account: pubkeys[8],
            recipient_token_1_account: pubkeys[9],
            token_program: pubkeys[10],
            token_program_2022: pubkeys[11],
        }
    }
}
impl<'info> From<CollectProtocolFeeAccounts<'_, 'info>>
for [AccountInfo<'info>; COLLECT_PROTOCOL_FEE_IX_ACCOUNTS_LEN] {
    fn from(accounts: CollectProtocolFeeAccounts<'_, 'info>) -> Self {
        [
            accounts.owner.clone(),
            accounts.authority.clone(),
            accounts.pool_state.clone(),
            accounts.amm_config.clone(),
            accounts.token_0_vault.clone(),
            accounts.token_1_vault.clone(),
            accounts.vault_0_mint.clone(),
            accounts.vault_1_mint.clone(),
            accounts.recipient_token_0_account.clone(),
            accounts.recipient_token_1_account.clone(),
            accounts.token_program.clone(),
            accounts.token_program_2022.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; COLLECT_PROTOCOL_FEE_IX_ACCOUNTS_LEN]>
for CollectProtocolFeeAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; COLLECT_PROTOCOL_FEE_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            owner: &arr[0],
            authority: &arr[1],
            pool_state: &arr[2],
            amm_config: &arr[3],
            token_0_vault: &arr[4],
            token_1_vault: &arr[5],
            vault_0_mint: &arr[6],
            vault_1_mint: &arr[7],
            recipient_token_0_account: &arr[8],
            recipient_token_1_account: &arr[9],
            token_program: &arr[10],
            token_program_2022: &arr[11],
        }
    }
}
pub const COLLECT_PROTOCOL_FEE_IX_DISCM: [u8; 8usize] = [
    136, 136, 252, 221, 194, 66, 126, 89,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CollectProtocolFeeIxArgs {
    pub amount_0_requested: u64,
    pub amount_1_requested: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct CollectProtocolFeeIxData(pub CollectProtocolFeeIxArgs);
impl From<CollectProtocolFeeIxArgs> for CollectProtocolFeeIxData {
    fn from(args: CollectProtocolFeeIxArgs) -> Self {
        Self(args)
    }
}
impl CollectProtocolFeeIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != COLLECT_PROTOCOL_FEE_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let amount_0_requested: u64 = crate::borsh_de_or_default(&mut reader)?;
        let amount_1_requested: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(CollectProtocolFeeIxArgs {
                amount_0_requested,
                amount_1_requested,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&COLLECT_PROTOCOL_FEE_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.amount_0_requested, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.amount_1_requested, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn collect_protocol_fee_ix_with_program_id(
    program_id: Pubkey,
    keys: CollectProtocolFeeKeys,
    args: CollectProtocolFeeIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; COLLECT_PROTOCOL_FEE_IX_ACCOUNTS_LEN] = keys.into();
    let data: CollectProtocolFeeIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn collect_protocol_fee_ix(
    keys: CollectProtocolFeeKeys,
    args: CollectProtocolFeeIxArgs,
) -> std::io::Result<Instruction> {
    collect_protocol_fee_ix_with_program_id(RAYDIUM_CP_SWAP_PROGRAM_ID, keys, args)
}
pub fn collect_protocol_fee_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CollectProtocolFeeAccounts<'_, '_>,
    args: CollectProtocolFeeIxArgs,
) -> ProgramResult {
    let keys: CollectProtocolFeeKeys = accounts.into();
    let ix = collect_protocol_fee_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn collect_protocol_fee_invoke(
    accounts: CollectProtocolFeeAccounts<'_, '_>,
    args: CollectProtocolFeeIxArgs,
) -> ProgramResult {
    collect_protocol_fee_invoke_with_program_id(
        RAYDIUM_CP_SWAP_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn collect_protocol_fee_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CollectProtocolFeeAccounts<'_, '_>,
    args: CollectProtocolFeeIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CollectProtocolFeeKeys = accounts.into();
    let ix = collect_protocol_fee_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn collect_protocol_fee_invoke_signed(
    accounts: CollectProtocolFeeAccounts<'_, '_>,
    args: CollectProtocolFeeIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    collect_protocol_fee_invoke_signed_with_program_id(
        RAYDIUM_CP_SWAP_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn collect_protocol_fee_verify_account_keys(
    accounts: CollectProtocolFeeAccounts<'_, '_>,
    keys: CollectProtocolFeeKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.owner.key, keys.owner),
        (*accounts.authority.key, keys.authority),
        (*accounts.pool_state.key, keys.pool_state),
        (*accounts.amm_config.key, keys.amm_config),
        (*accounts.token_0_vault.key, keys.token_0_vault),
        (*accounts.token_1_vault.key, keys.token_1_vault),
        (*accounts.vault_0_mint.key, keys.vault_0_mint),
        (*accounts.vault_1_mint.key, keys.vault_1_mint),
        (*accounts.recipient_token_0_account.key, keys.recipient_token_0_account),
        (*accounts.recipient_token_1_account.key, keys.recipient_token_1_account),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.token_program_2022.key, keys.token_program_2022),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn collect_protocol_fee_verify_writable_privileges<'me, 'info>(
    accounts: CollectProtocolFeeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.pool_state,
        accounts.token_0_vault,
        accounts.token_1_vault,
        accounts.recipient_token_0_account,
        accounts.recipient_token_1_account,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn collect_protocol_fee_verify_signer_privileges<'me, 'info>(
    accounts: CollectProtocolFeeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.owner] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn collect_protocol_fee_verify_account_privileges<'me, 'info>(
    accounts: CollectProtocolFeeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    collect_protocol_fee_verify_writable_privileges(accounts)?;
    collect_protocol_fee_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CREATE_AMM_CONFIG_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct CreateAmmConfigAccounts<'me, 'info> {
    pub owner: &'me AccountInfo<'info>,
    pub amm_config: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CreateAmmConfigKeys {
    pub owner: Pubkey,
    pub amm_config: Pubkey,
    pub system_program: Pubkey,
}
impl From<CreateAmmConfigAccounts<'_, '_>> for CreateAmmConfigKeys {
    fn from(accounts: CreateAmmConfigAccounts) -> Self {
        Self {
            owner: *accounts.owner.key,
            amm_config: *accounts.amm_config.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<CreateAmmConfigKeys> for [AccountMeta; CREATE_AMM_CONFIG_IX_ACCOUNTS_LEN] {
    fn from(keys: CreateAmmConfigKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.owner,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.amm_config,
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
impl From<[Pubkey; CREATE_AMM_CONFIG_IX_ACCOUNTS_LEN]> for CreateAmmConfigKeys {
    fn from(pubkeys: [Pubkey; CREATE_AMM_CONFIG_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            owner: pubkeys[0],
            amm_config: pubkeys[1],
            system_program: pubkeys[2],
        }
    }
}
impl<'info> From<CreateAmmConfigAccounts<'_, 'info>>
for [AccountInfo<'info>; CREATE_AMM_CONFIG_IX_ACCOUNTS_LEN] {
    fn from(accounts: CreateAmmConfigAccounts<'_, 'info>) -> Self {
        [
            accounts.owner.clone(),
            accounts.amm_config.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CREATE_AMM_CONFIG_IX_ACCOUNTS_LEN]>
for CreateAmmConfigAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; CREATE_AMM_CONFIG_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            owner: &arr[0],
            amm_config: &arr[1],
            system_program: &arr[2],
        }
    }
}
pub const CREATE_AMM_CONFIG_IX_DISCM: [u8; 8usize] = [
    137, 52, 237, 212, 215, 117, 108, 104,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CreateAmmConfigIxArgs {
    pub index: u16,
    pub trade_fee_rate: u64,
    pub protocol_fee_rate: u64,
    pub fund_fee_rate: u64,
    pub create_pool_fee: u64,
    pub creator_fee_rate: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct CreateAmmConfigIxData(pub CreateAmmConfigIxArgs);
impl From<CreateAmmConfigIxArgs> for CreateAmmConfigIxData {
    fn from(args: CreateAmmConfigIxArgs) -> Self {
        Self(args)
    }
}
impl CreateAmmConfigIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CREATE_AMM_CONFIG_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let index: u16 = crate::borsh_de_or_default(&mut reader)?;
        let trade_fee_rate: u64 = crate::borsh_de_or_default(&mut reader)?;
        let protocol_fee_rate: u64 = crate::borsh_de_or_default(&mut reader)?;
        let fund_fee_rate: u64 = crate::borsh_de_or_default(&mut reader)?;
        let create_pool_fee: u64 = crate::borsh_de_or_default(&mut reader)?;
        let creator_fee_rate: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(CreateAmmConfigIxArgs {
                index,
                trade_fee_rate,
                protocol_fee_rate,
                fund_fee_rate,
                create_pool_fee,
                creator_fee_rate,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CREATE_AMM_CONFIG_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.index, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.trade_fee_rate, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.protocol_fee_rate, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.fund_fee_rate, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.create_pool_fee, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.creator_fee_rate, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn create_amm_config_ix_with_program_id(
    program_id: Pubkey,
    keys: CreateAmmConfigKeys,
    args: CreateAmmConfigIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CREATE_AMM_CONFIG_IX_ACCOUNTS_LEN] = keys.into();
    let data: CreateAmmConfigIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn create_amm_config_ix(
    keys: CreateAmmConfigKeys,
    args: CreateAmmConfigIxArgs,
) -> std::io::Result<Instruction> {
    create_amm_config_ix_with_program_id(RAYDIUM_CP_SWAP_PROGRAM_ID, keys, args)
}
pub fn create_amm_config_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CreateAmmConfigAccounts<'_, '_>,
    args: CreateAmmConfigIxArgs,
) -> ProgramResult {
    let keys: CreateAmmConfigKeys = accounts.into();
    let ix = create_amm_config_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn create_amm_config_invoke(
    accounts: CreateAmmConfigAccounts<'_, '_>,
    args: CreateAmmConfigIxArgs,
) -> ProgramResult {
    create_amm_config_invoke_with_program_id(RAYDIUM_CP_SWAP_PROGRAM_ID, accounts, args)
}
pub fn create_amm_config_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CreateAmmConfigAccounts<'_, '_>,
    args: CreateAmmConfigIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CreateAmmConfigKeys = accounts.into();
    let ix = create_amm_config_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn create_amm_config_invoke_signed(
    accounts: CreateAmmConfigAccounts<'_, '_>,
    args: CreateAmmConfigIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    create_amm_config_invoke_signed_with_program_id(
        RAYDIUM_CP_SWAP_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn create_amm_config_verify_account_keys(
    accounts: CreateAmmConfigAccounts<'_, '_>,
    keys: CreateAmmConfigKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.owner.key, keys.owner),
        (*accounts.amm_config.key, keys.amm_config),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn create_amm_config_verify_writable_privileges<'me, 'info>(
    accounts: CreateAmmConfigAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.owner, accounts.amm_config] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn create_amm_config_verify_signer_privileges<'me, 'info>(
    accounts: CreateAmmConfigAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.owner] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn create_amm_config_verify_account_privileges<'me, 'info>(
    accounts: CreateAmmConfigAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    create_amm_config_verify_writable_privileges(accounts)?;
    create_amm_config_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CREATE_PERMISSION_PDA_IX_ACCOUNTS_LEN: usize = 4;
#[derive(Copy, Clone, Debug)]
pub struct CreatePermissionPdaAccounts<'me, 'info> {
    pub owner: &'me AccountInfo<'info>,
    pub permission_authority: &'me AccountInfo<'info>,
    pub permission: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CreatePermissionPdaKeys {
    pub owner: Pubkey,
    pub permission_authority: Pubkey,
    pub permission: Pubkey,
    pub system_program: Pubkey,
}
impl From<CreatePermissionPdaAccounts<'_, '_>> for CreatePermissionPdaKeys {
    fn from(accounts: CreatePermissionPdaAccounts) -> Self {
        Self {
            owner: *accounts.owner.key,
            permission_authority: *accounts.permission_authority.key,
            permission: *accounts.permission.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<CreatePermissionPdaKeys>
for [AccountMeta; CREATE_PERMISSION_PDA_IX_ACCOUNTS_LEN] {
    fn from(keys: CreatePermissionPdaKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.owner,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.permission_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.permission,
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
impl From<[Pubkey; CREATE_PERMISSION_PDA_IX_ACCOUNTS_LEN]> for CreatePermissionPdaKeys {
    fn from(pubkeys: [Pubkey; CREATE_PERMISSION_PDA_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            owner: pubkeys[0],
            permission_authority: pubkeys[1],
            permission: pubkeys[2],
            system_program: pubkeys[3],
        }
    }
}
impl<'info> From<CreatePermissionPdaAccounts<'_, 'info>>
for [AccountInfo<'info>; CREATE_PERMISSION_PDA_IX_ACCOUNTS_LEN] {
    fn from(accounts: CreatePermissionPdaAccounts<'_, 'info>) -> Self {
        [
            accounts.owner.clone(),
            accounts.permission_authority.clone(),
            accounts.permission.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CREATE_PERMISSION_PDA_IX_ACCOUNTS_LEN]>
for CreatePermissionPdaAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; CREATE_PERMISSION_PDA_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            owner: &arr[0],
            permission_authority: &arr[1],
            permission: &arr[2],
            system_program: &arr[3],
        }
    }
}
pub const CREATE_PERMISSION_PDA_IX_DISCM: [u8; 8usize] = [
    135, 136, 2, 216, 137, 169, 181, 202,
];
#[derive(Clone, Debug, PartialEq)]
pub struct CreatePermissionPdaIxData;
impl CreatePermissionPdaIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CREATE_PERMISSION_PDA_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CREATE_PERMISSION_PDA_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn create_permission_pda_ix_with_program_id(
    program_id: Pubkey,
    keys: CreatePermissionPdaKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CREATE_PERMISSION_PDA_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: CreatePermissionPdaIxData.try_to_vec()?,
    })
}
pub fn create_permission_pda_ix(
    keys: CreatePermissionPdaKeys,
) -> std::io::Result<Instruction> {
    create_permission_pda_ix_with_program_id(RAYDIUM_CP_SWAP_PROGRAM_ID, keys)
}
pub fn create_permission_pda_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CreatePermissionPdaAccounts<'_, '_>,
) -> ProgramResult {
    let keys: CreatePermissionPdaKeys = accounts.into();
    let ix = create_permission_pda_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn create_permission_pda_invoke(
    accounts: CreatePermissionPdaAccounts<'_, '_>,
) -> ProgramResult {
    create_permission_pda_invoke_with_program_id(RAYDIUM_CP_SWAP_PROGRAM_ID, accounts)
}
pub fn create_permission_pda_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CreatePermissionPdaAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CreatePermissionPdaKeys = accounts.into();
    let ix = create_permission_pda_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn create_permission_pda_invoke_signed(
    accounts: CreatePermissionPdaAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    create_permission_pda_invoke_signed_with_program_id(
        RAYDIUM_CP_SWAP_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn create_permission_pda_verify_account_keys(
    accounts: CreatePermissionPdaAccounts<'_, '_>,
    keys: CreatePermissionPdaKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.owner.key, keys.owner),
        (*accounts.permission_authority.key, keys.permission_authority),
        (*accounts.permission.key, keys.permission),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn create_permission_pda_verify_writable_privileges<'me, 'info>(
    accounts: CreatePermissionPdaAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.owner, accounts.permission] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn create_permission_pda_verify_signer_privileges<'me, 'info>(
    accounts: CreatePermissionPdaAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.owner] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn create_permission_pda_verify_account_privileges<'me, 'info>(
    accounts: CreatePermissionPdaAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    create_permission_pda_verify_writable_privileges(accounts)?;
    create_permission_pda_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CREATE_SUPPORT_MINT_ASSOCIATED_IX_ACCOUNTS_LEN: usize = 4;
#[derive(Copy, Clone, Debug)]
pub struct CreateSupportMintAssociatedAccounts<'me, 'info> {
    pub owner: &'me AccountInfo<'info>,
    pub token_mint: &'me AccountInfo<'info>,
    pub support_mint_associated: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CreateSupportMintAssociatedKeys {
    pub owner: Pubkey,
    pub token_mint: Pubkey,
    pub support_mint_associated: Pubkey,
    pub system_program: Pubkey,
}
impl From<CreateSupportMintAssociatedAccounts<'_, '_>>
for CreateSupportMintAssociatedKeys {
    fn from(accounts: CreateSupportMintAssociatedAccounts) -> Self {
        Self {
            owner: *accounts.owner.key,
            token_mint: *accounts.token_mint.key,
            support_mint_associated: *accounts.support_mint_associated.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<CreateSupportMintAssociatedKeys>
for [AccountMeta; CREATE_SUPPORT_MINT_ASSOCIATED_IX_ACCOUNTS_LEN] {
    fn from(keys: CreateSupportMintAssociatedKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.owner,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.support_mint_associated,
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
impl From<[Pubkey; CREATE_SUPPORT_MINT_ASSOCIATED_IX_ACCOUNTS_LEN]>
for CreateSupportMintAssociatedKeys {
    fn from(pubkeys: [Pubkey; CREATE_SUPPORT_MINT_ASSOCIATED_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            owner: pubkeys[0],
            token_mint: pubkeys[1],
            support_mint_associated: pubkeys[2],
            system_program: pubkeys[3],
        }
    }
}
impl<'info> From<CreateSupportMintAssociatedAccounts<'_, 'info>>
for [AccountInfo<'info>; CREATE_SUPPORT_MINT_ASSOCIATED_IX_ACCOUNTS_LEN] {
    fn from(accounts: CreateSupportMintAssociatedAccounts<'_, 'info>) -> Self {
        [
            accounts.owner.clone(),
            accounts.token_mint.clone(),
            accounts.support_mint_associated.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; CREATE_SUPPORT_MINT_ASSOCIATED_IX_ACCOUNTS_LEN]>
for CreateSupportMintAssociatedAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; CREATE_SUPPORT_MINT_ASSOCIATED_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            owner: &arr[0],
            token_mint: &arr[1],
            support_mint_associated: &arr[2],
            system_program: &arr[3],
        }
    }
}
pub const CREATE_SUPPORT_MINT_ASSOCIATED_IX_DISCM: [u8; 8usize] = [
    17, 251, 65, 92, 136, 242, 14, 169,
];
#[derive(Clone, Debug, PartialEq)]
pub struct CreateSupportMintAssociatedIxData;
impl CreateSupportMintAssociatedIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CREATE_SUPPORT_MINT_ASSOCIATED_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CREATE_SUPPORT_MINT_ASSOCIATED_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn create_support_mint_associated_ix_with_program_id(
    program_id: Pubkey,
    keys: CreateSupportMintAssociatedKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CREATE_SUPPORT_MINT_ASSOCIATED_IX_ACCOUNTS_LEN] = keys
        .into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: CreateSupportMintAssociatedIxData.try_to_vec()?,
    })
}
pub fn create_support_mint_associated_ix(
    keys: CreateSupportMintAssociatedKeys,
) -> std::io::Result<Instruction> {
    create_support_mint_associated_ix_with_program_id(RAYDIUM_CP_SWAP_PROGRAM_ID, keys)
}
pub fn create_support_mint_associated_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CreateSupportMintAssociatedAccounts<'_, '_>,
) -> ProgramResult {
    let keys: CreateSupportMintAssociatedKeys = accounts.into();
    let ix = create_support_mint_associated_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn create_support_mint_associated_invoke(
    accounts: CreateSupportMintAssociatedAccounts<'_, '_>,
) -> ProgramResult {
    create_support_mint_associated_invoke_with_program_id(
        RAYDIUM_CP_SWAP_PROGRAM_ID,
        accounts,
    )
}
pub fn create_support_mint_associated_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CreateSupportMintAssociatedAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CreateSupportMintAssociatedKeys = accounts.into();
    let ix = create_support_mint_associated_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn create_support_mint_associated_invoke_signed(
    accounts: CreateSupportMintAssociatedAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    create_support_mint_associated_invoke_signed_with_program_id(
        RAYDIUM_CP_SWAP_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn create_support_mint_associated_verify_account_keys(
    accounts: CreateSupportMintAssociatedAccounts<'_, '_>,
    keys: CreateSupportMintAssociatedKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.owner.key, keys.owner),
        (*accounts.token_mint.key, keys.token_mint),
        (*accounts.support_mint_associated.key, keys.support_mint_associated),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn create_support_mint_associated_verify_writable_privileges<'me, 'info>(
    accounts: CreateSupportMintAssociatedAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.owner, accounts.support_mint_associated] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn create_support_mint_associated_verify_signer_privileges<'me, 'info>(
    accounts: CreateSupportMintAssociatedAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.owner] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn create_support_mint_associated_verify_account_privileges<'me, 'info>(
    accounts: CreateSupportMintAssociatedAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    create_support_mint_associated_verify_writable_privileges(accounts)?;
    create_support_mint_associated_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const DEPOSIT_IX_ACCOUNTS_LEN: usize = 13;
#[derive(Copy, Clone, Debug)]
pub struct DepositAccounts<'me, 'info> {
    pub owner: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub pool_state: &'me AccountInfo<'info>,
    pub owner_lp_token: &'me AccountInfo<'info>,
    pub token_0_account: &'me AccountInfo<'info>,
    pub token_1_account: &'me AccountInfo<'info>,
    pub token_0_vault: &'me AccountInfo<'info>,
    pub token_1_vault: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub token_program_2022: &'me AccountInfo<'info>,
    pub vault_0_mint: &'me AccountInfo<'info>,
    pub vault_1_mint: &'me AccountInfo<'info>,
    pub lp_mint: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct DepositKeys {
    pub owner: Pubkey,
    pub authority: Pubkey,
    pub pool_state: Pubkey,
    pub owner_lp_token: Pubkey,
    pub token_0_account: Pubkey,
    pub token_1_account: Pubkey,
    pub token_0_vault: Pubkey,
    pub token_1_vault: Pubkey,
    pub token_program: Pubkey,
    pub token_program_2022: Pubkey,
    pub vault_0_mint: Pubkey,
    pub vault_1_mint: Pubkey,
    pub lp_mint: Pubkey,
}
impl From<DepositAccounts<'_, '_>> for DepositKeys {
    fn from(accounts: DepositAccounts) -> Self {
        Self {
            owner: *accounts.owner.key,
            authority: *accounts.authority.key,
            pool_state: *accounts.pool_state.key,
            owner_lp_token: *accounts.owner_lp_token.key,
            token_0_account: *accounts.token_0_account.key,
            token_1_account: *accounts.token_1_account.key,
            token_0_vault: *accounts.token_0_vault.key,
            token_1_vault: *accounts.token_1_vault.key,
            token_program: *accounts.token_program.key,
            token_program_2022: *accounts.token_program_2022.key,
            vault_0_mint: *accounts.vault_0_mint.key,
            vault_1_mint: *accounts.vault_1_mint.key,
            lp_mint: *accounts.lp_mint.key,
        }
    }
}
impl From<DepositKeys> for [AccountMeta; DEPOSIT_IX_ACCOUNTS_LEN] {
    fn from(keys: DepositKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.owner,
                is_signer: true,
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
                pubkey: keys.owner_lp_token,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_0_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_1_account,
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
                pubkey: keys.lp_mint,
                is_signer: false,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; DEPOSIT_IX_ACCOUNTS_LEN]> for DepositKeys {
    fn from(pubkeys: [Pubkey; DEPOSIT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            owner: pubkeys[0],
            authority: pubkeys[1],
            pool_state: pubkeys[2],
            owner_lp_token: pubkeys[3],
            token_0_account: pubkeys[4],
            token_1_account: pubkeys[5],
            token_0_vault: pubkeys[6],
            token_1_vault: pubkeys[7],
            token_program: pubkeys[8],
            token_program_2022: pubkeys[9],
            vault_0_mint: pubkeys[10],
            vault_1_mint: pubkeys[11],
            lp_mint: pubkeys[12],
        }
    }
}
impl<'info> From<DepositAccounts<'_, 'info>>
for [AccountInfo<'info>; DEPOSIT_IX_ACCOUNTS_LEN] {
    fn from(accounts: DepositAccounts<'_, 'info>) -> Self {
        [
            accounts.owner.clone(),
            accounts.authority.clone(),
            accounts.pool_state.clone(),
            accounts.owner_lp_token.clone(),
            accounts.token_0_account.clone(),
            accounts.token_1_account.clone(),
            accounts.token_0_vault.clone(),
            accounts.token_1_vault.clone(),
            accounts.token_program.clone(),
            accounts.token_program_2022.clone(),
            accounts.vault_0_mint.clone(),
            accounts.vault_1_mint.clone(),
            accounts.lp_mint.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; DEPOSIT_IX_ACCOUNTS_LEN]>
for DepositAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; DEPOSIT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            owner: &arr[0],
            authority: &arr[1],
            pool_state: &arr[2],
            owner_lp_token: &arr[3],
            token_0_account: &arr[4],
            token_1_account: &arr[5],
            token_0_vault: &arr[6],
            token_1_vault: &arr[7],
            token_program: &arr[8],
            token_program_2022: &arr[9],
            vault_0_mint: &arr[10],
            vault_1_mint: &arr[11],
            lp_mint: &arr[12],
        }
    }
}
pub const DEPOSIT_IX_DISCM: [u8; 8usize] = [242, 35, 198, 137, 82, 225, 242, 182];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DepositIxArgs {
    pub lp_token_amount: u64,
    pub maximum_token_0_amount: u64,
    pub maximum_token_1_amount: u64,
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
        let lp_token_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let maximum_token_0_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let maximum_token_1_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(DepositIxArgs {
                lp_token_amount,
                maximum_token_0_amount,
                maximum_token_1_amount,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&DEPOSIT_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.lp_token_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.maximum_token_0_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.maximum_token_1_amount, &mut writer)?;
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
    deposit_ix_with_program_id(RAYDIUM_CP_SWAP_PROGRAM_ID, keys, args)
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
    deposit_invoke_with_program_id(RAYDIUM_CP_SWAP_PROGRAM_ID, accounts, args)
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
    deposit_invoke_signed_with_program_id(
        RAYDIUM_CP_SWAP_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn deposit_verify_account_keys(
    accounts: DepositAccounts<'_, '_>,
    keys: DepositKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.owner.key, keys.owner),
        (*accounts.authority.key, keys.authority),
        (*accounts.pool_state.key, keys.pool_state),
        (*accounts.owner_lp_token.key, keys.owner_lp_token),
        (*accounts.token_0_account.key, keys.token_0_account),
        (*accounts.token_1_account.key, keys.token_1_account),
        (*accounts.token_0_vault.key, keys.token_0_vault),
        (*accounts.token_1_vault.key, keys.token_1_vault),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.token_program_2022.key, keys.token_program_2022),
        (*accounts.vault_0_mint.key, keys.vault_0_mint),
        (*accounts.vault_1_mint.key, keys.vault_1_mint),
        (*accounts.lp_mint.key, keys.lp_mint),
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
        accounts.pool_state,
        accounts.owner_lp_token,
        accounts.token_0_account,
        accounts.token_1_account,
        accounts.token_0_vault,
        accounts.token_1_vault,
        accounts.lp_mint,
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
pub const INITIALIZE_IX_ACCOUNTS_LEN: usize = 20;
#[derive(Copy, Clone, Debug)]
pub struct InitializeAccounts<'me, 'info> {
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
    pub token_program: &'me AccountInfo<'info>,
    pub token_0_program: &'me AccountInfo<'info>,
    pub token_1_program: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub rent: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct InitializeKeys {
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
    pub token_program: Pubkey,
    pub token_0_program: Pubkey,
    pub token_1_program: Pubkey,
    pub associated_token_program: Pubkey,
    pub system_program: Pubkey,
    pub rent: Pubkey,
}
impl From<InitializeAccounts<'_, '_>> for InitializeKeys {
    fn from(accounts: InitializeAccounts) -> Self {
        Self {
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
            token_program: *accounts.token_program.key,
            token_0_program: *accounts.token_0_program.key,
            token_1_program: *accounts.token_1_program.key,
            associated_token_program: *accounts.associated_token_program.key,
            system_program: *accounts.system_program.key,
            rent: *accounts.rent.key,
        }
    }
}
impl From<InitializeKeys> for [AccountMeta; INITIALIZE_IX_ACCOUNTS_LEN] {
    fn from(keys: InitializeKeys) -> Self {
        [
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
                pubkey: keys.token_program,
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
        ]
    }
}
impl From<[Pubkey; INITIALIZE_IX_ACCOUNTS_LEN]> for InitializeKeys {
    fn from(pubkeys: [Pubkey; INITIALIZE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            creator: pubkeys[0],
            amm_config: pubkeys[1],
            authority: pubkeys[2],
            pool_state: pubkeys[3],
            token_0_mint: pubkeys[4],
            token_1_mint: pubkeys[5],
            lp_mint: pubkeys[6],
            creator_token_0: pubkeys[7],
            creator_token_1: pubkeys[8],
            creator_lp_token: pubkeys[9],
            token_0_vault: pubkeys[10],
            token_1_vault: pubkeys[11],
            create_pool_fee: pubkeys[12],
            observation_state: pubkeys[13],
            token_program: pubkeys[14],
            token_0_program: pubkeys[15],
            token_1_program: pubkeys[16],
            associated_token_program: pubkeys[17],
            system_program: pubkeys[18],
            rent: pubkeys[19],
        }
    }
}
impl<'info> From<InitializeAccounts<'_, 'info>>
for [AccountInfo<'info>; INITIALIZE_IX_ACCOUNTS_LEN] {
    fn from(accounts: InitializeAccounts<'_, 'info>) -> Self {
        [
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
            accounts.token_program.clone(),
            accounts.token_0_program.clone(),
            accounts.token_1_program.clone(),
            accounts.associated_token_program.clone(),
            accounts.system_program.clone(),
            accounts.rent.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; INITIALIZE_IX_ACCOUNTS_LEN]>
for InitializeAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; INITIALIZE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            creator: &arr[0],
            amm_config: &arr[1],
            authority: &arr[2],
            pool_state: &arr[3],
            token_0_mint: &arr[4],
            token_1_mint: &arr[5],
            lp_mint: &arr[6],
            creator_token_0: &arr[7],
            creator_token_1: &arr[8],
            creator_lp_token: &arr[9],
            token_0_vault: &arr[10],
            token_1_vault: &arr[11],
            create_pool_fee: &arr[12],
            observation_state: &arr[13],
            token_program: &arr[14],
            token_0_program: &arr[15],
            token_1_program: &arr[16],
            associated_token_program: &arr[17],
            system_program: &arr[18],
            rent: &arr[19],
        }
    }
}
pub const INITIALIZE_IX_DISCM: [u8; 8usize] = [175, 175, 109, 31, 13, 152, 155, 237];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct InitializeIxArgs {
    pub init_amount_0: u64,
    pub init_amount_1: u64,
    pub open_time: u64,
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
        let init_amount_0: u64 = crate::borsh_de_or_default(&mut reader)?;
        let init_amount_1: u64 = crate::borsh_de_or_default(&mut reader)?;
        let open_time: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(InitializeIxArgs {
                init_amount_0,
                init_amount_1,
                open_time,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INITIALIZE_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.init_amount_0, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.init_amount_1, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.open_time, &mut writer)?;
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
    initialize_ix_with_program_id(RAYDIUM_CP_SWAP_PROGRAM_ID, keys, args)
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
    initialize_invoke_with_program_id(RAYDIUM_CP_SWAP_PROGRAM_ID, accounts, args)
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
        RAYDIUM_CP_SWAP_PROGRAM_ID,
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
        (*accounts.token_program.key, keys.token_program),
        (*accounts.token_0_program.key, keys.token_0_program),
        (*accounts.token_1_program.key, keys.token_1_program),
        (*accounts.associated_token_program.key, keys.associated_token_program),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.rent.key, keys.rent),
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
    for should_be_writable in [
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
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn initialize_verify_signer_privileges<'me, 'info>(
    accounts: InitializeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.creator] {
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
pub const INITIALIZE_WITH_PERMISSION_IX_ACCOUNTS_LEN: usize = 21;
#[derive(Copy, Clone, Debug)]
pub struct InitializeWithPermissionAccounts<'me, 'info> {
    pub payer: &'me AccountInfo<'info>,
    pub creator: &'me AccountInfo<'info>,
    pub amm_config: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub pool_state: &'me AccountInfo<'info>,
    pub token_0_mint: &'me AccountInfo<'info>,
    pub token_1_mint: &'me AccountInfo<'info>,
    pub lp_mint: &'me AccountInfo<'info>,
    pub payer_token_0: &'me AccountInfo<'info>,
    pub payer_token_1: &'me AccountInfo<'info>,
    pub payer_lp_token: &'me AccountInfo<'info>,
    pub token_0_vault: &'me AccountInfo<'info>,
    pub token_1_vault: &'me AccountInfo<'info>,
    pub create_pool_fee: &'me AccountInfo<'info>,
    pub observation_state: &'me AccountInfo<'info>,
    pub permission: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub token_0_program: &'me AccountInfo<'info>,
    pub token_1_program: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct InitializeWithPermissionKeys {
    pub payer: Pubkey,
    pub creator: Pubkey,
    pub amm_config: Pubkey,
    pub authority: Pubkey,
    pub pool_state: Pubkey,
    pub token_0_mint: Pubkey,
    pub token_1_mint: Pubkey,
    pub lp_mint: Pubkey,
    pub payer_token_0: Pubkey,
    pub payer_token_1: Pubkey,
    pub payer_lp_token: Pubkey,
    pub token_0_vault: Pubkey,
    pub token_1_vault: Pubkey,
    pub create_pool_fee: Pubkey,
    pub observation_state: Pubkey,
    pub permission: Pubkey,
    pub token_program: Pubkey,
    pub token_0_program: Pubkey,
    pub token_1_program: Pubkey,
    pub associated_token_program: Pubkey,
    pub system_program: Pubkey,
}
impl From<InitializeWithPermissionAccounts<'_, '_>> for InitializeWithPermissionKeys {
    fn from(accounts: InitializeWithPermissionAccounts) -> Self {
        Self {
            payer: *accounts.payer.key,
            creator: *accounts.creator.key,
            amm_config: *accounts.amm_config.key,
            authority: *accounts.authority.key,
            pool_state: *accounts.pool_state.key,
            token_0_mint: *accounts.token_0_mint.key,
            token_1_mint: *accounts.token_1_mint.key,
            lp_mint: *accounts.lp_mint.key,
            payer_token_0: *accounts.payer_token_0.key,
            payer_token_1: *accounts.payer_token_1.key,
            payer_lp_token: *accounts.payer_lp_token.key,
            token_0_vault: *accounts.token_0_vault.key,
            token_1_vault: *accounts.token_1_vault.key,
            create_pool_fee: *accounts.create_pool_fee.key,
            observation_state: *accounts.observation_state.key,
            permission: *accounts.permission.key,
            token_program: *accounts.token_program.key,
            token_0_program: *accounts.token_0_program.key,
            token_1_program: *accounts.token_1_program.key,
            associated_token_program: *accounts.associated_token_program.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<InitializeWithPermissionKeys>
for [AccountMeta; INITIALIZE_WITH_PERMISSION_IX_ACCOUNTS_LEN] {
    fn from(keys: InitializeWithPermissionKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.payer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.creator,
                is_signer: false,
                is_writable: false,
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
                pubkey: keys.payer_token_0,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.payer_token_1,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.payer_lp_token,
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
                pubkey: keys.permission,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_program,
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
        ]
    }
}
impl From<[Pubkey; INITIALIZE_WITH_PERMISSION_IX_ACCOUNTS_LEN]>
for InitializeWithPermissionKeys {
    fn from(pubkeys: [Pubkey; INITIALIZE_WITH_PERMISSION_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            payer: pubkeys[0],
            creator: pubkeys[1],
            amm_config: pubkeys[2],
            authority: pubkeys[3],
            pool_state: pubkeys[4],
            token_0_mint: pubkeys[5],
            token_1_mint: pubkeys[6],
            lp_mint: pubkeys[7],
            payer_token_0: pubkeys[8],
            payer_token_1: pubkeys[9],
            payer_lp_token: pubkeys[10],
            token_0_vault: pubkeys[11],
            token_1_vault: pubkeys[12],
            create_pool_fee: pubkeys[13],
            observation_state: pubkeys[14],
            permission: pubkeys[15],
            token_program: pubkeys[16],
            token_0_program: pubkeys[17],
            token_1_program: pubkeys[18],
            associated_token_program: pubkeys[19],
            system_program: pubkeys[20],
        }
    }
}
impl<'info> From<InitializeWithPermissionAccounts<'_, 'info>>
for [AccountInfo<'info>; INITIALIZE_WITH_PERMISSION_IX_ACCOUNTS_LEN] {
    fn from(accounts: InitializeWithPermissionAccounts<'_, 'info>) -> Self {
        [
            accounts.payer.clone(),
            accounts.creator.clone(),
            accounts.amm_config.clone(),
            accounts.authority.clone(),
            accounts.pool_state.clone(),
            accounts.token_0_mint.clone(),
            accounts.token_1_mint.clone(),
            accounts.lp_mint.clone(),
            accounts.payer_token_0.clone(),
            accounts.payer_token_1.clone(),
            accounts.payer_lp_token.clone(),
            accounts.token_0_vault.clone(),
            accounts.token_1_vault.clone(),
            accounts.create_pool_fee.clone(),
            accounts.observation_state.clone(),
            accounts.permission.clone(),
            accounts.token_program.clone(),
            accounts.token_0_program.clone(),
            accounts.token_1_program.clone(),
            accounts.associated_token_program.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; INITIALIZE_WITH_PERMISSION_IX_ACCOUNTS_LEN]>
for InitializeWithPermissionAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; INITIALIZE_WITH_PERMISSION_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            payer: &arr[0],
            creator: &arr[1],
            amm_config: &arr[2],
            authority: &arr[3],
            pool_state: &arr[4],
            token_0_mint: &arr[5],
            token_1_mint: &arr[6],
            lp_mint: &arr[7],
            payer_token_0: &arr[8],
            payer_token_1: &arr[9],
            payer_lp_token: &arr[10],
            token_0_vault: &arr[11],
            token_1_vault: &arr[12],
            create_pool_fee: &arr[13],
            observation_state: &arr[14],
            permission: &arr[15],
            token_program: &arr[16],
            token_0_program: &arr[17],
            token_1_program: &arr[18],
            associated_token_program: &arr[19],
            system_program: &arr[20],
        }
    }
}
pub const INITIALIZE_WITH_PERMISSION_IX_DISCM: [u8; 8usize] = [
    63, 55, 254, 65, 49, 178, 89, 121,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct InitializeWithPermissionIxArgs {
    pub init_amount_0: u64,
    pub init_amount_1: u64,
    pub open_time: u64,
    pub creator_fee_on: CreatorFeeOn,
}
#[derive(Clone, Debug, PartialEq)]
pub struct InitializeWithPermissionIxData(pub InitializeWithPermissionIxArgs);
impl From<InitializeWithPermissionIxArgs> for InitializeWithPermissionIxData {
    fn from(args: InitializeWithPermissionIxArgs) -> Self {
        Self(args)
    }
}
impl InitializeWithPermissionIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INITIALIZE_WITH_PERMISSION_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let init_amount_0: u64 = crate::borsh_de_or_default(&mut reader)?;
        let init_amount_1: u64 = crate::borsh_de_or_default(&mut reader)?;
        let open_time: u64 = crate::borsh_de_or_default(&mut reader)?;
        let creator_fee_on: CreatorFeeOn = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(InitializeWithPermissionIxArgs {
                init_amount_0,
                init_amount_1,
                open_time,
                creator_fee_on,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INITIALIZE_WITH_PERMISSION_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.init_amount_0, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.init_amount_1, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.open_time, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.creator_fee_on, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn initialize_with_permission_ix_with_program_id(
    program_id: Pubkey,
    keys: InitializeWithPermissionKeys,
    args: InitializeWithPermissionIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; INITIALIZE_WITH_PERMISSION_IX_ACCOUNTS_LEN] = keys.into();
    let data: InitializeWithPermissionIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn initialize_with_permission_ix(
    keys: InitializeWithPermissionKeys,
    args: InitializeWithPermissionIxArgs,
) -> std::io::Result<Instruction> {
    initialize_with_permission_ix_with_program_id(RAYDIUM_CP_SWAP_PROGRAM_ID, keys, args)
}
pub fn initialize_with_permission_invoke_with_program_id(
    program_id: Pubkey,
    accounts: InitializeWithPermissionAccounts<'_, '_>,
    args: InitializeWithPermissionIxArgs,
) -> ProgramResult {
    let keys: InitializeWithPermissionKeys = accounts.into();
    let ix = initialize_with_permission_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn initialize_with_permission_invoke(
    accounts: InitializeWithPermissionAccounts<'_, '_>,
    args: InitializeWithPermissionIxArgs,
) -> ProgramResult {
    initialize_with_permission_invoke_with_program_id(
        RAYDIUM_CP_SWAP_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn initialize_with_permission_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: InitializeWithPermissionAccounts<'_, '_>,
    args: InitializeWithPermissionIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: InitializeWithPermissionKeys = accounts.into();
    let ix = initialize_with_permission_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn initialize_with_permission_invoke_signed(
    accounts: InitializeWithPermissionAccounts<'_, '_>,
    args: InitializeWithPermissionIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    initialize_with_permission_invoke_signed_with_program_id(
        RAYDIUM_CP_SWAP_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn initialize_with_permission_verify_account_keys(
    accounts: InitializeWithPermissionAccounts<'_, '_>,
    keys: InitializeWithPermissionKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.payer.key, keys.payer),
        (*accounts.creator.key, keys.creator),
        (*accounts.amm_config.key, keys.amm_config),
        (*accounts.authority.key, keys.authority),
        (*accounts.pool_state.key, keys.pool_state),
        (*accounts.token_0_mint.key, keys.token_0_mint),
        (*accounts.token_1_mint.key, keys.token_1_mint),
        (*accounts.lp_mint.key, keys.lp_mint),
        (*accounts.payer_token_0.key, keys.payer_token_0),
        (*accounts.payer_token_1.key, keys.payer_token_1),
        (*accounts.payer_lp_token.key, keys.payer_lp_token),
        (*accounts.token_0_vault.key, keys.token_0_vault),
        (*accounts.token_1_vault.key, keys.token_1_vault),
        (*accounts.create_pool_fee.key, keys.create_pool_fee),
        (*accounts.observation_state.key, keys.observation_state),
        (*accounts.permission.key, keys.permission),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.token_0_program.key, keys.token_0_program),
        (*accounts.token_1_program.key, keys.token_1_program),
        (*accounts.associated_token_program.key, keys.associated_token_program),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn initialize_with_permission_verify_writable_privileges<'me, 'info>(
    accounts: InitializeWithPermissionAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.payer,
        accounts.pool_state,
        accounts.lp_mint,
        accounts.payer_token_0,
        accounts.payer_token_1,
        accounts.payer_lp_token,
        accounts.token_0_vault,
        accounts.token_1_vault,
        accounts.create_pool_fee,
        accounts.observation_state,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn initialize_with_permission_verify_signer_privileges<'me, 'info>(
    accounts: InitializeWithPermissionAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.payer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn initialize_with_permission_verify_account_privileges<'me, 'info>(
    accounts: InitializeWithPermissionAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    initialize_with_permission_verify_writable_privileges(accounts)?;
    initialize_with_permission_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SWAP_BASE_INPUT_IX_ACCOUNTS_LEN: usize = 13;
#[derive(Copy, Clone, Debug)]
pub struct SwapBaseInputAccounts<'me, 'info> {
    pub payer: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub amm_config: &'me AccountInfo<'info>,
    pub pool_state: &'me AccountInfo<'info>,
    pub input_token_account: &'me AccountInfo<'info>,
    pub output_token_account: &'me AccountInfo<'info>,
    pub input_vault: &'me AccountInfo<'info>,
    pub output_vault: &'me AccountInfo<'info>,
    pub input_token_program: &'me AccountInfo<'info>,
    pub output_token_program: &'me AccountInfo<'info>,
    pub input_token_mint: &'me AccountInfo<'info>,
    pub output_token_mint: &'me AccountInfo<'info>,
    pub observation_state: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SwapBaseInputKeys {
    pub payer: Pubkey,
    pub authority: Pubkey,
    pub amm_config: Pubkey,
    pub pool_state: Pubkey,
    pub input_token_account: Pubkey,
    pub output_token_account: Pubkey,
    pub input_vault: Pubkey,
    pub output_vault: Pubkey,
    pub input_token_program: Pubkey,
    pub output_token_program: Pubkey,
    pub input_token_mint: Pubkey,
    pub output_token_mint: Pubkey,
    pub observation_state: Pubkey,
}
impl From<SwapBaseInputAccounts<'_, '_>> for SwapBaseInputKeys {
    fn from(accounts: SwapBaseInputAccounts) -> Self {
        Self {
            payer: *accounts.payer.key,
            authority: *accounts.authority.key,
            amm_config: *accounts.amm_config.key,
            pool_state: *accounts.pool_state.key,
            input_token_account: *accounts.input_token_account.key,
            output_token_account: *accounts.output_token_account.key,
            input_vault: *accounts.input_vault.key,
            output_vault: *accounts.output_vault.key,
            input_token_program: *accounts.input_token_program.key,
            output_token_program: *accounts.output_token_program.key,
            input_token_mint: *accounts.input_token_mint.key,
            output_token_mint: *accounts.output_token_mint.key,
            observation_state: *accounts.observation_state.key,
        }
    }
}
impl From<SwapBaseInputKeys> for [AccountMeta; SWAP_BASE_INPUT_IX_ACCOUNTS_LEN] {
    fn from(keys: SwapBaseInputKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.payer,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.amm_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_state,
                is_signer: false,
                is_writable: true,
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
                pubkey: keys.input_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.output_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.input_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.output_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.input_token_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.output_token_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.observation_state,
                is_signer: false,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; SWAP_BASE_INPUT_IX_ACCOUNTS_LEN]> for SwapBaseInputKeys {
    fn from(pubkeys: [Pubkey; SWAP_BASE_INPUT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            payer: pubkeys[0],
            authority: pubkeys[1],
            amm_config: pubkeys[2],
            pool_state: pubkeys[3],
            input_token_account: pubkeys[4],
            output_token_account: pubkeys[5],
            input_vault: pubkeys[6],
            output_vault: pubkeys[7],
            input_token_program: pubkeys[8],
            output_token_program: pubkeys[9],
            input_token_mint: pubkeys[10],
            output_token_mint: pubkeys[11],
            observation_state: pubkeys[12],
        }
    }
}
impl<'info> From<SwapBaseInputAccounts<'_, 'info>>
for [AccountInfo<'info>; SWAP_BASE_INPUT_IX_ACCOUNTS_LEN] {
    fn from(accounts: SwapBaseInputAccounts<'_, 'info>) -> Self {
        [
            accounts.payer.clone(),
            accounts.authority.clone(),
            accounts.amm_config.clone(),
            accounts.pool_state.clone(),
            accounts.input_token_account.clone(),
            accounts.output_token_account.clone(),
            accounts.input_vault.clone(),
            accounts.output_vault.clone(),
            accounts.input_token_program.clone(),
            accounts.output_token_program.clone(),
            accounts.input_token_mint.clone(),
            accounts.output_token_mint.clone(),
            accounts.observation_state.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SWAP_BASE_INPUT_IX_ACCOUNTS_LEN]>
for SwapBaseInputAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; SWAP_BASE_INPUT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            payer: &arr[0],
            authority: &arr[1],
            amm_config: &arr[2],
            pool_state: &arr[3],
            input_token_account: &arr[4],
            output_token_account: &arr[5],
            input_vault: &arr[6],
            output_vault: &arr[7],
            input_token_program: &arr[8],
            output_token_program: &arr[9],
            input_token_mint: &arr[10],
            output_token_mint: &arr[11],
            observation_state: &arr[12],
        }
    }
}
pub const SWAP_BASE_INPUT_IX_DISCM: [u8; 8usize] = [143, 190, 90, 218, 196, 30, 51, 222];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SwapBaseInputIxArgs {
    pub amount_in: u64,
    pub minimum_amount_out: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct SwapBaseInputIxData(pub SwapBaseInputIxArgs);
impl From<SwapBaseInputIxArgs> for SwapBaseInputIxData {
    fn from(args: SwapBaseInputIxArgs) -> Self {
        Self(args)
    }
}
impl SwapBaseInputIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SWAP_BASE_INPUT_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let amount_in: u64 = crate::borsh_de_or_default(&mut reader)?;
        let minimum_amount_out: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(SwapBaseInputIxArgs {
                amount_in,
                minimum_amount_out,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SWAP_BASE_INPUT_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.amount_in, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.minimum_amount_out, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn swap_base_input_ix_with_program_id(
    program_id: Pubkey,
    keys: SwapBaseInputKeys,
    args: SwapBaseInputIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SWAP_BASE_INPUT_IX_ACCOUNTS_LEN] = keys.into();
    let data: SwapBaseInputIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn swap_base_input_ix(
    keys: SwapBaseInputKeys,
    args: SwapBaseInputIxArgs,
) -> std::io::Result<Instruction> {
    swap_base_input_ix_with_program_id(RAYDIUM_CP_SWAP_PROGRAM_ID, keys, args)
}
pub fn swap_base_input_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SwapBaseInputAccounts<'_, '_>,
    args: SwapBaseInputIxArgs,
) -> ProgramResult {
    let keys: SwapBaseInputKeys = accounts.into();
    let ix = swap_base_input_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn swap_base_input_invoke(
    accounts: SwapBaseInputAccounts<'_, '_>,
    args: SwapBaseInputIxArgs,
) -> ProgramResult {
    swap_base_input_invoke_with_program_id(RAYDIUM_CP_SWAP_PROGRAM_ID, accounts, args)
}
pub fn swap_base_input_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SwapBaseInputAccounts<'_, '_>,
    args: SwapBaseInputIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SwapBaseInputKeys = accounts.into();
    let ix = swap_base_input_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn swap_base_input_invoke_signed(
    accounts: SwapBaseInputAccounts<'_, '_>,
    args: SwapBaseInputIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    swap_base_input_invoke_signed_with_program_id(
        RAYDIUM_CP_SWAP_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn swap_base_input_verify_account_keys(
    accounts: SwapBaseInputAccounts<'_, '_>,
    keys: SwapBaseInputKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.payer.key, keys.payer),
        (*accounts.authority.key, keys.authority),
        (*accounts.amm_config.key, keys.amm_config),
        (*accounts.pool_state.key, keys.pool_state),
        (*accounts.input_token_account.key, keys.input_token_account),
        (*accounts.output_token_account.key, keys.output_token_account),
        (*accounts.input_vault.key, keys.input_vault),
        (*accounts.output_vault.key, keys.output_vault),
        (*accounts.input_token_program.key, keys.input_token_program),
        (*accounts.output_token_program.key, keys.output_token_program),
        (*accounts.input_token_mint.key, keys.input_token_mint),
        (*accounts.output_token_mint.key, keys.output_token_mint),
        (*accounts.observation_state.key, keys.observation_state),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn swap_base_input_verify_writable_privileges<'me, 'info>(
    accounts: SwapBaseInputAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.pool_state,
        accounts.input_token_account,
        accounts.output_token_account,
        accounts.input_vault,
        accounts.output_vault,
        accounts.observation_state,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn swap_base_input_verify_signer_privileges<'me, 'info>(
    accounts: SwapBaseInputAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.payer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn swap_base_input_verify_account_privileges<'me, 'info>(
    accounts: SwapBaseInputAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    swap_base_input_verify_writable_privileges(accounts)?;
    swap_base_input_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SWAP_BASE_OUTPUT_IX_ACCOUNTS_LEN: usize = 13;
#[derive(Copy, Clone, Debug)]
pub struct SwapBaseOutputAccounts<'me, 'info> {
    pub payer: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub amm_config: &'me AccountInfo<'info>,
    pub pool_state: &'me AccountInfo<'info>,
    pub input_token_account: &'me AccountInfo<'info>,
    pub output_token_account: &'me AccountInfo<'info>,
    pub input_vault: &'me AccountInfo<'info>,
    pub output_vault: &'me AccountInfo<'info>,
    pub input_token_program: &'me AccountInfo<'info>,
    pub output_token_program: &'me AccountInfo<'info>,
    pub input_token_mint: &'me AccountInfo<'info>,
    pub output_token_mint: &'me AccountInfo<'info>,
    pub observation_state: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SwapBaseOutputKeys {
    pub payer: Pubkey,
    pub authority: Pubkey,
    pub amm_config: Pubkey,
    pub pool_state: Pubkey,
    pub input_token_account: Pubkey,
    pub output_token_account: Pubkey,
    pub input_vault: Pubkey,
    pub output_vault: Pubkey,
    pub input_token_program: Pubkey,
    pub output_token_program: Pubkey,
    pub input_token_mint: Pubkey,
    pub output_token_mint: Pubkey,
    pub observation_state: Pubkey,
}
impl From<SwapBaseOutputAccounts<'_, '_>> for SwapBaseOutputKeys {
    fn from(accounts: SwapBaseOutputAccounts) -> Self {
        Self {
            payer: *accounts.payer.key,
            authority: *accounts.authority.key,
            amm_config: *accounts.amm_config.key,
            pool_state: *accounts.pool_state.key,
            input_token_account: *accounts.input_token_account.key,
            output_token_account: *accounts.output_token_account.key,
            input_vault: *accounts.input_vault.key,
            output_vault: *accounts.output_vault.key,
            input_token_program: *accounts.input_token_program.key,
            output_token_program: *accounts.output_token_program.key,
            input_token_mint: *accounts.input_token_mint.key,
            output_token_mint: *accounts.output_token_mint.key,
            observation_state: *accounts.observation_state.key,
        }
    }
}
impl From<SwapBaseOutputKeys> for [AccountMeta; SWAP_BASE_OUTPUT_IX_ACCOUNTS_LEN] {
    fn from(keys: SwapBaseOutputKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.payer,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.amm_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_state,
                is_signer: false,
                is_writable: true,
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
                pubkey: keys.input_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.output_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.input_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.output_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.input_token_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.output_token_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.observation_state,
                is_signer: false,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; SWAP_BASE_OUTPUT_IX_ACCOUNTS_LEN]> for SwapBaseOutputKeys {
    fn from(pubkeys: [Pubkey; SWAP_BASE_OUTPUT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            payer: pubkeys[0],
            authority: pubkeys[1],
            amm_config: pubkeys[2],
            pool_state: pubkeys[3],
            input_token_account: pubkeys[4],
            output_token_account: pubkeys[5],
            input_vault: pubkeys[6],
            output_vault: pubkeys[7],
            input_token_program: pubkeys[8],
            output_token_program: pubkeys[9],
            input_token_mint: pubkeys[10],
            output_token_mint: pubkeys[11],
            observation_state: pubkeys[12],
        }
    }
}
impl<'info> From<SwapBaseOutputAccounts<'_, 'info>>
for [AccountInfo<'info>; SWAP_BASE_OUTPUT_IX_ACCOUNTS_LEN] {
    fn from(accounts: SwapBaseOutputAccounts<'_, 'info>) -> Self {
        [
            accounts.payer.clone(),
            accounts.authority.clone(),
            accounts.amm_config.clone(),
            accounts.pool_state.clone(),
            accounts.input_token_account.clone(),
            accounts.output_token_account.clone(),
            accounts.input_vault.clone(),
            accounts.output_vault.clone(),
            accounts.input_token_program.clone(),
            accounts.output_token_program.clone(),
            accounts.input_token_mint.clone(),
            accounts.output_token_mint.clone(),
            accounts.observation_state.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SWAP_BASE_OUTPUT_IX_ACCOUNTS_LEN]>
for SwapBaseOutputAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; SWAP_BASE_OUTPUT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            payer: &arr[0],
            authority: &arr[1],
            amm_config: &arr[2],
            pool_state: &arr[3],
            input_token_account: &arr[4],
            output_token_account: &arr[5],
            input_vault: &arr[6],
            output_vault: &arr[7],
            input_token_program: &arr[8],
            output_token_program: &arr[9],
            input_token_mint: &arr[10],
            output_token_mint: &arr[11],
            observation_state: &arr[12],
        }
    }
}
pub const SWAP_BASE_OUTPUT_IX_DISCM: [u8; 8usize] = [55, 217, 98, 86, 163, 74, 180, 173];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SwapBaseOutputIxArgs {
    pub max_amount_in: u64,
    pub amount_out: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct SwapBaseOutputIxData(pub SwapBaseOutputIxArgs);
impl From<SwapBaseOutputIxArgs> for SwapBaseOutputIxData {
    fn from(args: SwapBaseOutputIxArgs) -> Self {
        Self(args)
    }
}
impl SwapBaseOutputIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SWAP_BASE_OUTPUT_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let max_amount_in: u64 = crate::borsh_de_or_default(&mut reader)?;
        let amount_out: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(SwapBaseOutputIxArgs {
                max_amount_in,
                amount_out,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SWAP_BASE_OUTPUT_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.max_amount_in, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.amount_out, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn swap_base_output_ix_with_program_id(
    program_id: Pubkey,
    keys: SwapBaseOutputKeys,
    args: SwapBaseOutputIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SWAP_BASE_OUTPUT_IX_ACCOUNTS_LEN] = keys.into();
    let data: SwapBaseOutputIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn swap_base_output_ix(
    keys: SwapBaseOutputKeys,
    args: SwapBaseOutputIxArgs,
) -> std::io::Result<Instruction> {
    swap_base_output_ix_with_program_id(RAYDIUM_CP_SWAP_PROGRAM_ID, keys, args)
}
pub fn swap_base_output_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SwapBaseOutputAccounts<'_, '_>,
    args: SwapBaseOutputIxArgs,
) -> ProgramResult {
    let keys: SwapBaseOutputKeys = accounts.into();
    let ix = swap_base_output_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn swap_base_output_invoke(
    accounts: SwapBaseOutputAccounts<'_, '_>,
    args: SwapBaseOutputIxArgs,
) -> ProgramResult {
    swap_base_output_invoke_with_program_id(RAYDIUM_CP_SWAP_PROGRAM_ID, accounts, args)
}
pub fn swap_base_output_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SwapBaseOutputAccounts<'_, '_>,
    args: SwapBaseOutputIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SwapBaseOutputKeys = accounts.into();
    let ix = swap_base_output_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn swap_base_output_invoke_signed(
    accounts: SwapBaseOutputAccounts<'_, '_>,
    args: SwapBaseOutputIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    swap_base_output_invoke_signed_with_program_id(
        RAYDIUM_CP_SWAP_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn swap_base_output_verify_account_keys(
    accounts: SwapBaseOutputAccounts<'_, '_>,
    keys: SwapBaseOutputKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.payer.key, keys.payer),
        (*accounts.authority.key, keys.authority),
        (*accounts.amm_config.key, keys.amm_config),
        (*accounts.pool_state.key, keys.pool_state),
        (*accounts.input_token_account.key, keys.input_token_account),
        (*accounts.output_token_account.key, keys.output_token_account),
        (*accounts.input_vault.key, keys.input_vault),
        (*accounts.output_vault.key, keys.output_vault),
        (*accounts.input_token_program.key, keys.input_token_program),
        (*accounts.output_token_program.key, keys.output_token_program),
        (*accounts.input_token_mint.key, keys.input_token_mint),
        (*accounts.output_token_mint.key, keys.output_token_mint),
        (*accounts.observation_state.key, keys.observation_state),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn swap_base_output_verify_writable_privileges<'me, 'info>(
    accounts: SwapBaseOutputAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.pool_state,
        accounts.input_token_account,
        accounts.output_token_account,
        accounts.input_vault,
        accounts.output_vault,
        accounts.observation_state,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn swap_base_output_verify_signer_privileges<'me, 'info>(
    accounts: SwapBaseOutputAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.payer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn swap_base_output_verify_account_privileges<'me, 'info>(
    accounts: SwapBaseOutputAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    swap_base_output_verify_writable_privileges(accounts)?;
    swap_base_output_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const UPDATE_AMM_CONFIG_IX_ACCOUNTS_LEN: usize = 2;
#[derive(Copy, Clone, Debug)]
pub struct UpdateAmmConfigAccounts<'me, 'info> {
    pub owner: &'me AccountInfo<'info>,
    pub amm_config: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UpdateAmmConfigKeys {
    pub owner: Pubkey,
    pub amm_config: Pubkey,
}
impl From<UpdateAmmConfigAccounts<'_, '_>> for UpdateAmmConfigKeys {
    fn from(accounts: UpdateAmmConfigAccounts) -> Self {
        Self {
            owner: *accounts.owner.key,
            amm_config: *accounts.amm_config.key,
        }
    }
}
impl From<UpdateAmmConfigKeys> for [AccountMeta; UPDATE_AMM_CONFIG_IX_ACCOUNTS_LEN] {
    fn from(keys: UpdateAmmConfigKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.owner,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.amm_config,
                is_signer: false,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; UPDATE_AMM_CONFIG_IX_ACCOUNTS_LEN]> for UpdateAmmConfigKeys {
    fn from(pubkeys: [Pubkey; UPDATE_AMM_CONFIG_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            owner: pubkeys[0],
            amm_config: pubkeys[1],
        }
    }
}
impl<'info> From<UpdateAmmConfigAccounts<'_, 'info>>
for [AccountInfo<'info>; UPDATE_AMM_CONFIG_IX_ACCOUNTS_LEN] {
    fn from(accounts: UpdateAmmConfigAccounts<'_, 'info>) -> Self {
        [accounts.owner.clone(), accounts.amm_config.clone()]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; UPDATE_AMM_CONFIG_IX_ACCOUNTS_LEN]>
for UpdateAmmConfigAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; UPDATE_AMM_CONFIG_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            owner: &arr[0],
            amm_config: &arr[1],
        }
    }
}
pub const UPDATE_AMM_CONFIG_IX_DISCM: [u8; 8usize] = [
    49, 60, 174, 136, 154, 28, 116, 200,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UpdateAmmConfigIxArgs {
    pub param: u8,
    pub value: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateAmmConfigIxData(pub UpdateAmmConfigIxArgs);
impl From<UpdateAmmConfigIxArgs> for UpdateAmmConfigIxData {
    fn from(args: UpdateAmmConfigIxArgs) -> Self {
        Self(args)
    }
}
impl UpdateAmmConfigIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_AMM_CONFIG_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let param: u8 = crate::borsh_de_or_default(&mut reader)?;
        let value: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(UpdateAmmConfigIxArgs {
                param,
                value,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_AMM_CONFIG_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.param, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.value, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn update_amm_config_ix_with_program_id(
    program_id: Pubkey,
    keys: UpdateAmmConfigKeys,
    args: UpdateAmmConfigIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; UPDATE_AMM_CONFIG_IX_ACCOUNTS_LEN] = keys.into();
    let data: UpdateAmmConfigIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_amm_config_ix(
    keys: UpdateAmmConfigKeys,
    args: UpdateAmmConfigIxArgs,
) -> std::io::Result<Instruction> {
    update_amm_config_ix_with_program_id(RAYDIUM_CP_SWAP_PROGRAM_ID, keys, args)
}
pub fn update_amm_config_invoke_with_program_id(
    program_id: Pubkey,
    accounts: UpdateAmmConfigAccounts<'_, '_>,
    args: UpdateAmmConfigIxArgs,
) -> ProgramResult {
    let keys: UpdateAmmConfigKeys = accounts.into();
    let ix = update_amm_config_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn update_amm_config_invoke(
    accounts: UpdateAmmConfigAccounts<'_, '_>,
    args: UpdateAmmConfigIxArgs,
) -> ProgramResult {
    update_amm_config_invoke_with_program_id(RAYDIUM_CP_SWAP_PROGRAM_ID, accounts, args)
}
pub fn update_amm_config_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: UpdateAmmConfigAccounts<'_, '_>,
    args: UpdateAmmConfigIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: UpdateAmmConfigKeys = accounts.into();
    let ix = update_amm_config_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn update_amm_config_invoke_signed(
    accounts: UpdateAmmConfigAccounts<'_, '_>,
    args: UpdateAmmConfigIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    update_amm_config_invoke_signed_with_program_id(
        RAYDIUM_CP_SWAP_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn update_amm_config_verify_account_keys(
    accounts: UpdateAmmConfigAccounts<'_, '_>,
    keys: UpdateAmmConfigKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.owner.key, keys.owner),
        (*accounts.amm_config.key, keys.amm_config),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn update_amm_config_verify_writable_privileges<'me, 'info>(
    accounts: UpdateAmmConfigAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.amm_config] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn update_amm_config_verify_signer_privileges<'me, 'info>(
    accounts: UpdateAmmConfigAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.owner] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn update_amm_config_verify_account_privileges<'me, 'info>(
    accounts: UpdateAmmConfigAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    update_amm_config_verify_writable_privileges(accounts)?;
    update_amm_config_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const UPDATE_POOL_STATUS_IX_ACCOUNTS_LEN: usize = 2;
#[derive(Copy, Clone, Debug)]
pub struct UpdatePoolStatusAccounts<'me, 'info> {
    pub authority: &'me AccountInfo<'info>,
    pub pool_state: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UpdatePoolStatusKeys {
    pub authority: Pubkey,
    pub pool_state: Pubkey,
}
impl From<UpdatePoolStatusAccounts<'_, '_>> for UpdatePoolStatusKeys {
    fn from(accounts: UpdatePoolStatusAccounts) -> Self {
        Self {
            authority: *accounts.authority.key,
            pool_state: *accounts.pool_state.key,
        }
    }
}
impl From<UpdatePoolStatusKeys> for [AccountMeta; UPDATE_POOL_STATUS_IX_ACCOUNTS_LEN] {
    fn from(keys: UpdatePoolStatusKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.authority,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_state,
                is_signer: false,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; UPDATE_POOL_STATUS_IX_ACCOUNTS_LEN]> for UpdatePoolStatusKeys {
    fn from(pubkeys: [Pubkey; UPDATE_POOL_STATUS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            authority: pubkeys[0],
            pool_state: pubkeys[1],
        }
    }
}
impl<'info> From<UpdatePoolStatusAccounts<'_, 'info>>
for [AccountInfo<'info>; UPDATE_POOL_STATUS_IX_ACCOUNTS_LEN] {
    fn from(accounts: UpdatePoolStatusAccounts<'_, 'info>) -> Self {
        [accounts.authority.clone(), accounts.pool_state.clone()]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; UPDATE_POOL_STATUS_IX_ACCOUNTS_LEN]>
for UpdatePoolStatusAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; UPDATE_POOL_STATUS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            authority: &arr[0],
            pool_state: &arr[1],
        }
    }
}
pub const UPDATE_POOL_STATUS_IX_DISCM: [u8; 8usize] = [
    130, 87, 108, 6, 46, 224, 117, 123,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UpdatePoolStatusIxArgs {
    pub status: u8,
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdatePoolStatusIxData(pub UpdatePoolStatusIxArgs);
impl From<UpdatePoolStatusIxArgs> for UpdatePoolStatusIxData {
    fn from(args: UpdatePoolStatusIxArgs) -> Self {
        Self(args)
    }
}
impl UpdatePoolStatusIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_POOL_STATUS_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let status: u8 = crate::borsh_de_or_default(&mut reader)?;
        Ok(Self(UpdatePoolStatusIxArgs { status }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_POOL_STATUS_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.status, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn update_pool_status_ix_with_program_id(
    program_id: Pubkey,
    keys: UpdatePoolStatusKeys,
    args: UpdatePoolStatusIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; UPDATE_POOL_STATUS_IX_ACCOUNTS_LEN] = keys.into();
    let data: UpdatePoolStatusIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_pool_status_ix(
    keys: UpdatePoolStatusKeys,
    args: UpdatePoolStatusIxArgs,
) -> std::io::Result<Instruction> {
    update_pool_status_ix_with_program_id(RAYDIUM_CP_SWAP_PROGRAM_ID, keys, args)
}
pub fn update_pool_status_invoke_with_program_id(
    program_id: Pubkey,
    accounts: UpdatePoolStatusAccounts<'_, '_>,
    args: UpdatePoolStatusIxArgs,
) -> ProgramResult {
    let keys: UpdatePoolStatusKeys = accounts.into();
    let ix = update_pool_status_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn update_pool_status_invoke(
    accounts: UpdatePoolStatusAccounts<'_, '_>,
    args: UpdatePoolStatusIxArgs,
) -> ProgramResult {
    update_pool_status_invoke_with_program_id(RAYDIUM_CP_SWAP_PROGRAM_ID, accounts, args)
}
pub fn update_pool_status_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: UpdatePoolStatusAccounts<'_, '_>,
    args: UpdatePoolStatusIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: UpdatePoolStatusKeys = accounts.into();
    let ix = update_pool_status_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn update_pool_status_invoke_signed(
    accounts: UpdatePoolStatusAccounts<'_, '_>,
    args: UpdatePoolStatusIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    update_pool_status_invoke_signed_with_program_id(
        RAYDIUM_CP_SWAP_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn update_pool_status_verify_account_keys(
    accounts: UpdatePoolStatusAccounts<'_, '_>,
    keys: UpdatePoolStatusKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.authority.key, keys.authority),
        (*accounts.pool_state.key, keys.pool_state),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn update_pool_status_verify_writable_privileges<'me, 'info>(
    accounts: UpdatePoolStatusAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.pool_state] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn update_pool_status_verify_signer_privileges<'me, 'info>(
    accounts: UpdatePoolStatusAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn update_pool_status_verify_account_privileges<'me, 'info>(
    accounts: UpdatePoolStatusAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    update_pool_status_verify_writable_privileges(accounts)?;
    update_pool_status_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const WITHDRAW_IX_ACCOUNTS_LEN: usize = 14;
#[derive(Copy, Clone, Debug)]
pub struct WithdrawAccounts<'me, 'info> {
    pub owner: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub pool_state: &'me AccountInfo<'info>,
    pub owner_lp_token: &'me AccountInfo<'info>,
    pub token_0_account: &'me AccountInfo<'info>,
    pub token_1_account: &'me AccountInfo<'info>,
    pub token_0_vault: &'me AccountInfo<'info>,
    pub token_1_vault: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub token_program_2022: &'me AccountInfo<'info>,
    pub vault_0_mint: &'me AccountInfo<'info>,
    pub vault_1_mint: &'me AccountInfo<'info>,
    pub lp_mint: &'me AccountInfo<'info>,
    pub memo_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct WithdrawKeys {
    pub owner: Pubkey,
    pub authority: Pubkey,
    pub pool_state: Pubkey,
    pub owner_lp_token: Pubkey,
    pub token_0_account: Pubkey,
    pub token_1_account: Pubkey,
    pub token_0_vault: Pubkey,
    pub token_1_vault: Pubkey,
    pub token_program: Pubkey,
    pub token_program_2022: Pubkey,
    pub vault_0_mint: Pubkey,
    pub vault_1_mint: Pubkey,
    pub lp_mint: Pubkey,
    pub memo_program: Pubkey,
}
impl From<WithdrawAccounts<'_, '_>> for WithdrawKeys {
    fn from(accounts: WithdrawAccounts) -> Self {
        Self {
            owner: *accounts.owner.key,
            authority: *accounts.authority.key,
            pool_state: *accounts.pool_state.key,
            owner_lp_token: *accounts.owner_lp_token.key,
            token_0_account: *accounts.token_0_account.key,
            token_1_account: *accounts.token_1_account.key,
            token_0_vault: *accounts.token_0_vault.key,
            token_1_vault: *accounts.token_1_vault.key,
            token_program: *accounts.token_program.key,
            token_program_2022: *accounts.token_program_2022.key,
            vault_0_mint: *accounts.vault_0_mint.key,
            vault_1_mint: *accounts.vault_1_mint.key,
            lp_mint: *accounts.lp_mint.key,
            memo_program: *accounts.memo_program.key,
        }
    }
}
impl From<WithdrawKeys> for [AccountMeta; WITHDRAW_IX_ACCOUNTS_LEN] {
    fn from(keys: WithdrawKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.owner,
                is_signer: true,
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
                pubkey: keys.owner_lp_token,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_0_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_1_account,
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
                pubkey: keys.lp_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.memo_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; WITHDRAW_IX_ACCOUNTS_LEN]> for WithdrawKeys {
    fn from(pubkeys: [Pubkey; WITHDRAW_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            owner: pubkeys[0],
            authority: pubkeys[1],
            pool_state: pubkeys[2],
            owner_lp_token: pubkeys[3],
            token_0_account: pubkeys[4],
            token_1_account: pubkeys[5],
            token_0_vault: pubkeys[6],
            token_1_vault: pubkeys[7],
            token_program: pubkeys[8],
            token_program_2022: pubkeys[9],
            vault_0_mint: pubkeys[10],
            vault_1_mint: pubkeys[11],
            lp_mint: pubkeys[12],
            memo_program: pubkeys[13],
        }
    }
}
impl<'info> From<WithdrawAccounts<'_, 'info>>
for [AccountInfo<'info>; WITHDRAW_IX_ACCOUNTS_LEN] {
    fn from(accounts: WithdrawAccounts<'_, 'info>) -> Self {
        [
            accounts.owner.clone(),
            accounts.authority.clone(),
            accounts.pool_state.clone(),
            accounts.owner_lp_token.clone(),
            accounts.token_0_account.clone(),
            accounts.token_1_account.clone(),
            accounts.token_0_vault.clone(),
            accounts.token_1_vault.clone(),
            accounts.token_program.clone(),
            accounts.token_program_2022.clone(),
            accounts.vault_0_mint.clone(),
            accounts.vault_1_mint.clone(),
            accounts.lp_mint.clone(),
            accounts.memo_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; WITHDRAW_IX_ACCOUNTS_LEN]>
for WithdrawAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; WITHDRAW_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            owner: &arr[0],
            authority: &arr[1],
            pool_state: &arr[2],
            owner_lp_token: &arr[3],
            token_0_account: &arr[4],
            token_1_account: &arr[5],
            token_0_vault: &arr[6],
            token_1_vault: &arr[7],
            token_program: &arr[8],
            token_program_2022: &arr[9],
            vault_0_mint: &arr[10],
            vault_1_mint: &arr[11],
            lp_mint: &arr[12],
            memo_program: &arr[13],
        }
    }
}
pub const WITHDRAW_IX_DISCM: [u8; 8usize] = [183, 18, 70, 156, 148, 109, 161, 34];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WithdrawIxArgs {
    pub lp_token_amount: u64,
    pub minimum_token_0_amount: u64,
    pub minimum_token_1_amount: u64,
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
        let lp_token_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let minimum_token_0_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let minimum_token_1_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(WithdrawIxArgs {
                lp_token_amount,
                minimum_token_0_amount,
                minimum_token_1_amount,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&WITHDRAW_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.lp_token_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.minimum_token_0_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.minimum_token_1_amount, &mut writer)?;
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
    withdraw_ix_with_program_id(RAYDIUM_CP_SWAP_PROGRAM_ID, keys, args)
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
    withdraw_invoke_with_program_id(RAYDIUM_CP_SWAP_PROGRAM_ID, accounts, args)
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
    withdraw_invoke_signed_with_program_id(
        RAYDIUM_CP_SWAP_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn withdraw_verify_account_keys(
    accounts: WithdrawAccounts<'_, '_>,
    keys: WithdrawKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.owner.key, keys.owner),
        (*accounts.authority.key, keys.authority),
        (*accounts.pool_state.key, keys.pool_state),
        (*accounts.owner_lp_token.key, keys.owner_lp_token),
        (*accounts.token_0_account.key, keys.token_0_account),
        (*accounts.token_1_account.key, keys.token_1_account),
        (*accounts.token_0_vault.key, keys.token_0_vault),
        (*accounts.token_1_vault.key, keys.token_1_vault),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.token_program_2022.key, keys.token_program_2022),
        (*accounts.vault_0_mint.key, keys.vault_0_mint),
        (*accounts.vault_1_mint.key, keys.vault_1_mint),
        (*accounts.lp_mint.key, keys.lp_mint),
        (*accounts.memo_program.key, keys.memo_program),
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
        accounts.pool_state,
        accounts.owner_lp_token,
        accounts.token_0_account,
        accounts.token_1_account,
        accounts.token_0_vault,
        accounts.token_1_vault,
        accounts.lp_mint,
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
