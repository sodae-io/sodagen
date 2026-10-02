use solana_pubkey::Pubkey;
use solana_cpi::{invoke, invoke_signed};
use solana_instruction::{AccountMeta, Instruction};
use solana_account_info::AccountInfo;
use solana_program_error::ProgramError;
use std::io::Read;
#[allow(unused_imports)]
use crate::*;
#[derive(Clone, Debug, PartialEq)]
pub enum OmnipairProgramIx {
    AddCollateral(AddCollateralIxArgs),
    AddLiquidity(AddLiquidityIxArgs),
    Borrow(BorrowIxArgs),
    ClaimProtocolFees,
    CreateRateModel(CreateRateModelIxArgs),
    Flashloan(FlashloanIxArgs),
    InitFutarchyAuthority(InitFutarchyAuthorityIxArgs),
    Initialize(InitializeIxArgs),
    Liquidate,
    RemoveCollateral(RemoveCollateralIxArgs),
    RemoveLiquidity(RemoveLiquidityIxArgs),
    Repay(RepayIxArgs),
    SetGlobalReduceOnly(SetGlobalReduceOnlyIxArgs),
    SetPairRateModel,
    SetPairReduceOnly(SetPairReduceOnlyIxArgs),
    Swap(SwapIxArgs),
    UpdateFutarchyAuthority(UpdateFutarchyAuthorityIxArgs),
    UpdateProtocolRevenue(UpdateProtocolRevenueIxArgs),
    UpdateRevenueRecipients(UpdateRevenueRecipientsIxArgs),
    ViewPairData(ViewPairDataIxArgs),
    ViewUserPositionData(ViewUserPositionDataIxArgs),
}
impl OmnipairProgramIx {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        if buf.starts_with(&ADD_COLLATERAL_IX_DISCM) {
            let mut reader = &buf[ADD_COLLATERAL_IX_DISCM.len()..];
            let args = if reader.is_empty() {
                Default::default()
            } else {
                <AdjustCollateralArgs>::deserialize(&mut reader)?
            };
            return Ok(Self::AddCollateral(AddCollateralIxArgs { args }));
        }
        if buf.starts_with(&ADD_LIQUIDITY_IX_DISCM) {
            let mut reader = &buf[ADD_LIQUIDITY_IX_DISCM.len()..];
            let args = if reader.is_empty() {
                Default::default()
            } else {
                <AddLiquidityArgs>::deserialize(&mut reader)?
            };
            return Ok(Self::AddLiquidity(AddLiquidityIxArgs { args }));
        }
        if buf.starts_with(&BORROW_IX_DISCM) {
            let mut reader = &buf[BORROW_IX_DISCM.len()..];
            let args = if reader.is_empty() {
                Default::default()
            } else {
                <AdjustDebtArgs>::deserialize(&mut reader)?
            };
            return Ok(Self::Borrow(BorrowIxArgs { args }));
        }
        if buf.starts_with(&CLAIM_PROTOCOL_FEES_IX_DISCM) {
            return Ok(Self::ClaimProtocolFees);
        }
        if buf.starts_with(&CREATE_RATE_MODEL_IX_DISCM) {
            let mut reader = &buf[CREATE_RATE_MODEL_IX_DISCM.len()..];
            let args = if reader.is_empty() {
                Default::default()
            } else {
                <CreateRateModelArgs>::deserialize(&mut reader)?
            };
            return Ok(Self::CreateRateModel(CreateRateModelIxArgs { args }));
        }
        if buf.starts_with(&FLASHLOAN_IX_DISCM) {
            let mut reader = &buf[FLASHLOAN_IX_DISCM.len()..];
            let args = if reader.is_empty() {
                Default::default()
            } else {
                <FlashloanArgs>::deserialize(&mut reader)?
            };
            return Ok(Self::Flashloan(FlashloanIxArgs { args }));
        }
        if buf.starts_with(&INIT_FUTARCHY_AUTHORITY_IX_DISCM) {
            let mut reader = &buf[INIT_FUTARCHY_AUTHORITY_IX_DISCM.len()..];
            let args = if reader.is_empty() {
                Default::default()
            } else {
                <InitFutarchyAuthorityArgs>::deserialize(&mut reader)?
            };
            return Ok(
                Self::InitFutarchyAuthority(InitFutarchyAuthorityIxArgs {
                    args,
                }),
            );
        }
        if buf.starts_with(&INITIALIZE_IX_DISCM) {
            let mut reader = &buf[INITIALIZE_IX_DISCM.len()..];
            let args = if reader.is_empty() {
                Default::default()
            } else {
                <InitializeAndBootstrapArgs>::deserialize(&mut reader)?
            };
            return Ok(Self::Initialize(InitializeIxArgs { args }));
        }
        if buf.starts_with(&LIQUIDATE_IX_DISCM) {
            return Ok(Self::Liquidate);
        }
        if buf.starts_with(&REMOVE_COLLATERAL_IX_DISCM) {
            let mut reader = &buf[REMOVE_COLLATERAL_IX_DISCM.len()..];
            let args = if reader.is_empty() {
                Default::default()
            } else {
                <AdjustCollateralArgs>::deserialize(&mut reader)?
            };
            return Ok(Self::RemoveCollateral(RemoveCollateralIxArgs { args }));
        }
        if buf.starts_with(&REMOVE_LIQUIDITY_IX_DISCM) {
            let mut reader = &buf[REMOVE_LIQUIDITY_IX_DISCM.len()..];
            let args = if reader.is_empty() {
                Default::default()
            } else {
                <RemoveLiquidityArgs>::deserialize(&mut reader)?
            };
            return Ok(Self::RemoveLiquidity(RemoveLiquidityIxArgs { args }));
        }
        if buf.starts_with(&REPAY_IX_DISCM) {
            let mut reader = &buf[REPAY_IX_DISCM.len()..];
            let args = if reader.is_empty() {
                Default::default()
            } else {
                <AdjustDebtArgs>::deserialize(&mut reader)?
            };
            return Ok(Self::Repay(RepayIxArgs { args }));
        }
        if buf.starts_with(&SET_GLOBAL_REDUCE_ONLY_IX_DISCM) {
            let mut reader = &buf[SET_GLOBAL_REDUCE_ONLY_IX_DISCM.len()..];
            let args = if reader.is_empty() {
                Default::default()
            } else {
                <SetGlobalReduceOnlyArgs>::deserialize(&mut reader)?
            };
            return Ok(Self::SetGlobalReduceOnly(SetGlobalReduceOnlyIxArgs { args }));
        }
        if buf.starts_with(&SET_PAIR_RATE_MODEL_IX_DISCM) {
            return Ok(Self::SetPairRateModel);
        }
        if buf.starts_with(&SET_PAIR_REDUCE_ONLY_IX_DISCM) {
            let mut reader = &buf[SET_PAIR_REDUCE_ONLY_IX_DISCM.len()..];
            let args = if reader.is_empty() {
                Default::default()
            } else {
                <SetPairReduceOnlyArgs>::deserialize(&mut reader)?
            };
            return Ok(Self::SetPairReduceOnly(SetPairReduceOnlyIxArgs { args }));
        }
        if buf.starts_with(&SWAP_IX_DISCM) {
            let mut reader = &buf[SWAP_IX_DISCM.len()..];
            let args = if reader.is_empty() {
                Default::default()
            } else {
                <SwapArgs>::deserialize(&mut reader)?
            };
            return Ok(Self::Swap(SwapIxArgs { args }));
        }
        if buf.starts_with(&UPDATE_FUTARCHY_AUTHORITY_IX_DISCM) {
            let mut reader = &buf[UPDATE_FUTARCHY_AUTHORITY_IX_DISCM.len()..];
            let args = if reader.is_empty() {
                Default::default()
            } else {
                <UpdateFutarchyAuthorityArgs>::deserialize(&mut reader)?
            };
            return Ok(
                Self::UpdateFutarchyAuthority(UpdateFutarchyAuthorityIxArgs {
                    args,
                }),
            );
        }
        if buf.starts_with(&UPDATE_PROTOCOL_REVENUE_IX_DISCM) {
            let mut reader = &buf[UPDATE_PROTOCOL_REVENUE_IX_DISCM.len()..];
            let args = if reader.is_empty() {
                Default::default()
            } else {
                <UpdateProtocolRevenueArgs>::deserialize(&mut reader)?
            };
            return Ok(
                Self::UpdateProtocolRevenue(UpdateProtocolRevenueIxArgs {
                    args,
                }),
            );
        }
        if buf.starts_with(&UPDATE_REVENUE_RECIPIENTS_IX_DISCM) {
            let mut reader = &buf[UPDATE_REVENUE_RECIPIENTS_IX_DISCM.len()..];
            let args = if reader.is_empty() {
                Default::default()
            } else {
                <UpdateRevenueRecipientsArgs>::deserialize(&mut reader)?
            };
            return Ok(
                Self::UpdateRevenueRecipients(UpdateRevenueRecipientsIxArgs {
                    args,
                }),
            );
        }
        if buf.starts_with(&VIEW_PAIR_DATA_IX_DISCM) {
            let mut reader = &buf[VIEW_PAIR_DATA_IX_DISCM.len()..];
            let getter: PairViewKind = crate::borsh_de_or_default(&mut reader)?;
            let args = if reader.is_empty() {
                Default::default()
            } else {
                <EmitValueArgs>::deserialize(&mut reader)?
            };
            return Ok(Self::ViewPairData(ViewPairDataIxArgs { getter, args }));
        }
        if buf.starts_with(&VIEW_USER_POSITION_DATA_IX_DISCM) {
            let mut reader = &buf[VIEW_USER_POSITION_DATA_IX_DISCM.len()..];
            let getter: UserPositionViewKind = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::ViewUserPositionData(ViewUserPositionDataIxArgs {
                    getter,
                }),
            );
        }
        Err(std::io::Error::from(std::io::ErrorKind::InvalidData))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        match self {
            Self::AddCollateral(args) => {
                writer.write_all(&ADD_COLLATERAL_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.args, &mut writer)?;
                Ok(())
            }
            Self::AddLiquidity(args) => {
                writer.write_all(&ADD_LIQUIDITY_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.args, &mut writer)?;
                Ok(())
            }
            Self::Borrow(args) => {
                writer.write_all(&BORROW_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.args, &mut writer)?;
                Ok(())
            }
            Self::ClaimProtocolFees => writer.write_all(&CLAIM_PROTOCOL_FEES_IX_DISCM),
            Self::CreateRateModel(args) => {
                writer.write_all(&CREATE_RATE_MODEL_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.args, &mut writer)?;
                Ok(())
            }
            Self::Flashloan(args) => {
                writer.write_all(&FLASHLOAN_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.args, &mut writer)?;
                Ok(())
            }
            Self::InitFutarchyAuthority(args) => {
                writer.write_all(&INIT_FUTARCHY_AUTHORITY_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.args, &mut writer)?;
                Ok(())
            }
            Self::Initialize(args) => {
                writer.write_all(&INITIALIZE_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.args, &mut writer)?;
                Ok(())
            }
            Self::Liquidate => writer.write_all(&LIQUIDATE_IX_DISCM),
            Self::RemoveCollateral(args) => {
                writer.write_all(&REMOVE_COLLATERAL_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.args, &mut writer)?;
                Ok(())
            }
            Self::RemoveLiquidity(args) => {
                writer.write_all(&REMOVE_LIQUIDITY_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.args, &mut writer)?;
                Ok(())
            }
            Self::Repay(args) => {
                writer.write_all(&REPAY_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.args, &mut writer)?;
                Ok(())
            }
            Self::SetGlobalReduceOnly(args) => {
                writer.write_all(&SET_GLOBAL_REDUCE_ONLY_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.args, &mut writer)?;
                Ok(())
            }
            Self::SetPairRateModel => writer.write_all(&SET_PAIR_RATE_MODEL_IX_DISCM),
            Self::SetPairReduceOnly(args) => {
                writer.write_all(&SET_PAIR_REDUCE_ONLY_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.args, &mut writer)?;
                Ok(())
            }
            Self::Swap(args) => {
                writer.write_all(&SWAP_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.args, &mut writer)?;
                Ok(())
            }
            Self::UpdateFutarchyAuthority(args) => {
                writer.write_all(&UPDATE_FUTARCHY_AUTHORITY_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.args, &mut writer)?;
                Ok(())
            }
            Self::UpdateProtocolRevenue(args) => {
                writer.write_all(&UPDATE_PROTOCOL_REVENUE_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.args, &mut writer)?;
                Ok(())
            }
            Self::UpdateRevenueRecipients(args) => {
                writer.write_all(&UPDATE_REVENUE_RECIPIENTS_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.args, &mut writer)?;
                Ok(())
            }
            Self::ViewPairData(args) => {
                writer.write_all(&VIEW_PAIR_DATA_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.getter, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.args, &mut writer)?;
                Ok(())
            }
            Self::ViewUserPositionData(args) => {
                writer.write_all(&VIEW_USER_POSITION_DATA_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.getter, &mut writer)?;
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
pub const ADD_COLLATERAL_IX_ACCOUNTS_LEN: usize = 13;
#[derive(Copy, Clone, Debug)]
pub struct AddCollateralAccounts<'me, 'info> {
    pub pair: &'me AccountInfo<'info>,
    pub rate_model: &'me AccountInfo<'info>,
    pub futarchy_authority: &'me AccountInfo<'info>,
    pub user_position: &'me AccountInfo<'info>,
    pub collateral_vault: &'me AccountInfo<'info>,
    pub user_collateral_token_account: &'me AccountInfo<'info>,
    pub collateral_token_mint: &'me AccountInfo<'info>,
    pub user: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub token_2022_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct AddCollateralKeys {
    pub pair: Pubkey,
    pub rate_model: Pubkey,
    pub futarchy_authority: Pubkey,
    pub user_position: Pubkey,
    pub collateral_vault: Pubkey,
    pub user_collateral_token_account: Pubkey,
    pub collateral_token_mint: Pubkey,
    pub user: Pubkey,
    pub token_program: Pubkey,
    pub token_2022_program: Pubkey,
    pub system_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<AddCollateralAccounts<'_, '_>> for AddCollateralKeys {
    fn from(accounts: AddCollateralAccounts) -> Self {
        Self {
            pair: *accounts.pair.key,
            rate_model: *accounts.rate_model.key,
            futarchy_authority: *accounts.futarchy_authority.key,
            user_position: *accounts.user_position.key,
            collateral_vault: *accounts.collateral_vault.key,
            user_collateral_token_account: *accounts.user_collateral_token_account.key,
            collateral_token_mint: *accounts.collateral_token_mint.key,
            user: *accounts.user.key,
            token_program: *accounts.token_program.key,
            token_2022_program: *accounts.token_2022_program.key,
            system_program: *accounts.system_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<AddCollateralKeys> for [AccountMeta; ADD_COLLATERAL_IX_ACCOUNTS_LEN] {
    fn from(keys: AddCollateralKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.pair,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.rate_model,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.futarchy_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.user_position,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.collateral_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_collateral_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.collateral_token_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.user,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_2022_program,
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
impl From<[Pubkey; ADD_COLLATERAL_IX_ACCOUNTS_LEN]> for AddCollateralKeys {
    fn from(pubkeys: [Pubkey; ADD_COLLATERAL_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            pair: pubkeys[0],
            rate_model: pubkeys[1],
            futarchy_authority: pubkeys[2],
            user_position: pubkeys[3],
            collateral_vault: pubkeys[4],
            user_collateral_token_account: pubkeys[5],
            collateral_token_mint: pubkeys[6],
            user: pubkeys[7],
            token_program: pubkeys[8],
            token_2022_program: pubkeys[9],
            system_program: pubkeys[10],
            event_authority: pubkeys[11],
            program: pubkeys[12],
        }
    }
}
impl<'info> From<AddCollateralAccounts<'_, 'info>>
for [AccountInfo<'info>; ADD_COLLATERAL_IX_ACCOUNTS_LEN] {
    fn from(accounts: AddCollateralAccounts<'_, 'info>) -> Self {
        [
            accounts.pair.clone(),
            accounts.rate_model.clone(),
            accounts.futarchy_authority.clone(),
            accounts.user_position.clone(),
            accounts.collateral_vault.clone(),
            accounts.user_collateral_token_account.clone(),
            accounts.collateral_token_mint.clone(),
            accounts.user.clone(),
            accounts.token_program.clone(),
            accounts.token_2022_program.clone(),
            accounts.system_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; ADD_COLLATERAL_IX_ACCOUNTS_LEN]>
for AddCollateralAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; ADD_COLLATERAL_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            pair: &arr[0],
            rate_model: &arr[1],
            futarchy_authority: &arr[2],
            user_position: &arr[3],
            collateral_vault: &arr[4],
            user_collateral_token_account: &arr[5],
            collateral_token_mint: &arr[6],
            user: &arr[7],
            token_program: &arr[8],
            token_2022_program: &arr[9],
            system_program: &arr[10],
            event_authority: &arr[11],
            program: &arr[12],
        }
    }
}
pub const ADD_COLLATERAL_IX_DISCM: [u8; 8usize] = [127, 82, 121, 42, 161, 176, 249, 206];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AddCollateralIxArgs {
    pub args: AdjustCollateralArgs,
}
#[derive(Clone, Debug, PartialEq)]
pub struct AddCollateralIxData(pub AddCollateralIxArgs);
impl From<AddCollateralIxArgs> for AddCollateralIxData {
    fn from(args: AddCollateralIxArgs) -> Self {
        Self(args)
    }
}
impl AddCollateralIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != ADD_COLLATERAL_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let args = if reader.is_empty() {
            Default::default()
        } else {
            <AdjustCollateralArgs>::deserialize(&mut reader)?
        };
        Ok(Self(AddCollateralIxArgs { args }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&ADD_COLLATERAL_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.args, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn add_collateral_ix_with_program_id(
    program_id: Pubkey,
    keys: AddCollateralKeys,
    args: AddCollateralIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; ADD_COLLATERAL_IX_ACCOUNTS_LEN] = keys.into();
    let data: AddCollateralIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn add_collateral_ix(
    keys: AddCollateralKeys,
    args: AddCollateralIxArgs,
) -> std::io::Result<Instruction> {
    add_collateral_ix_with_program_id(OMNIPAIR_PROGRAM_ID, keys, args)
}
pub fn add_collateral_invoke_with_program_id(
    program_id: Pubkey,
    accounts: AddCollateralAccounts<'_, '_>,
    args: AddCollateralIxArgs,
) -> ProgramResult {
    let keys: AddCollateralKeys = accounts.into();
    let ix = add_collateral_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn add_collateral_invoke(
    accounts: AddCollateralAccounts<'_, '_>,
    args: AddCollateralIxArgs,
) -> ProgramResult {
    add_collateral_invoke_with_program_id(OMNIPAIR_PROGRAM_ID, accounts, args)
}
pub fn add_collateral_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: AddCollateralAccounts<'_, '_>,
    args: AddCollateralIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: AddCollateralKeys = accounts.into();
    let ix = add_collateral_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn add_collateral_invoke_signed(
    accounts: AddCollateralAccounts<'_, '_>,
    args: AddCollateralIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    add_collateral_invoke_signed_with_program_id(
        OMNIPAIR_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn add_collateral_verify_account_keys(
    accounts: AddCollateralAccounts<'_, '_>,
    keys: AddCollateralKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.pair.key, keys.pair),
        (*accounts.rate_model.key, keys.rate_model),
        (*accounts.futarchy_authority.key, keys.futarchy_authority),
        (*accounts.user_position.key, keys.user_position),
        (*accounts.collateral_vault.key, keys.collateral_vault),
        (
            *accounts.user_collateral_token_account.key,
            keys.user_collateral_token_account,
        ),
        (*accounts.collateral_token_mint.key, keys.collateral_token_mint),
        (*accounts.user.key, keys.user),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.token_2022_program.key, keys.token_2022_program),
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
pub fn add_collateral_verify_writable_privileges<'me, 'info>(
    accounts: AddCollateralAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.pair,
        accounts.rate_model,
        accounts.user_position,
        accounts.collateral_vault,
        accounts.user_collateral_token_account,
        accounts.user,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn add_collateral_verify_signer_privileges<'me, 'info>(
    accounts: AddCollateralAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.user] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn add_collateral_verify_account_privileges<'me, 'info>(
    accounts: AddCollateralAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    add_collateral_verify_writable_privileges(accounts)?;
    add_collateral_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const ADD_LIQUIDITY_IX_ACCOUNTS_LEN: usize = 19;
#[derive(Copy, Clone, Debug)]
pub struct AddLiquidityAccounts<'me, 'info> {
    pub pair: &'me AccountInfo<'info>,
    pub rate_model: &'me AccountInfo<'info>,
    pub futarchy_authority: &'me AccountInfo<'info>,
    pub reserve0_vault: &'me AccountInfo<'info>,
    pub reserve1_vault: &'me AccountInfo<'info>,
    pub user_token0_account: &'me AccountInfo<'info>,
    pub user_token1_account: &'me AccountInfo<'info>,
    pub token0_mint: &'me AccountInfo<'info>,
    pub token1_mint: &'me AccountInfo<'info>,
    pub lp_mint: &'me AccountInfo<'info>,
    pub user_lp_token_account: &'me AccountInfo<'info>,
    pub user: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub token_2022_program: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub instructions_sysvar: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct AddLiquidityKeys {
    pub pair: Pubkey,
    pub rate_model: Pubkey,
    pub futarchy_authority: Pubkey,
    pub reserve0_vault: Pubkey,
    pub reserve1_vault: Pubkey,
    pub user_token0_account: Pubkey,
    pub user_token1_account: Pubkey,
    pub token0_mint: Pubkey,
    pub token1_mint: Pubkey,
    pub lp_mint: Pubkey,
    pub user_lp_token_account: Pubkey,
    pub user: Pubkey,
    pub token_program: Pubkey,
    pub token_2022_program: Pubkey,
    pub associated_token_program: Pubkey,
    pub system_program: Pubkey,
    pub instructions_sysvar: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<AddLiquidityAccounts<'_, '_>> for AddLiquidityKeys {
    fn from(accounts: AddLiquidityAccounts) -> Self {
        Self {
            pair: *accounts.pair.key,
            rate_model: *accounts.rate_model.key,
            futarchy_authority: *accounts.futarchy_authority.key,
            reserve0_vault: *accounts.reserve0_vault.key,
            reserve1_vault: *accounts.reserve1_vault.key,
            user_token0_account: *accounts.user_token0_account.key,
            user_token1_account: *accounts.user_token1_account.key,
            token0_mint: *accounts.token0_mint.key,
            token1_mint: *accounts.token1_mint.key,
            lp_mint: *accounts.lp_mint.key,
            user_lp_token_account: *accounts.user_lp_token_account.key,
            user: *accounts.user.key,
            token_program: *accounts.token_program.key,
            token_2022_program: *accounts.token_2022_program.key,
            associated_token_program: *accounts.associated_token_program.key,
            system_program: *accounts.system_program.key,
            instructions_sysvar: *accounts.instructions_sysvar.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<AddLiquidityKeys> for [AccountMeta; ADD_LIQUIDITY_IX_ACCOUNTS_LEN] {
    fn from(keys: AddLiquidityKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.pair,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.rate_model,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.futarchy_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.reserve0_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.reserve1_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_token0_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_token1_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token0_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token1_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.lp_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_lp_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_2022_program,
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
                pubkey: keys.instructions_sysvar,
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
impl From<[Pubkey; ADD_LIQUIDITY_IX_ACCOUNTS_LEN]> for AddLiquidityKeys {
    fn from(pubkeys: [Pubkey; ADD_LIQUIDITY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            pair: pubkeys[0],
            rate_model: pubkeys[1],
            futarchy_authority: pubkeys[2],
            reserve0_vault: pubkeys[3],
            reserve1_vault: pubkeys[4],
            user_token0_account: pubkeys[5],
            user_token1_account: pubkeys[6],
            token0_mint: pubkeys[7],
            token1_mint: pubkeys[8],
            lp_mint: pubkeys[9],
            user_lp_token_account: pubkeys[10],
            user: pubkeys[11],
            token_program: pubkeys[12],
            token_2022_program: pubkeys[13],
            associated_token_program: pubkeys[14],
            system_program: pubkeys[15],
            instructions_sysvar: pubkeys[16],
            event_authority: pubkeys[17],
            program: pubkeys[18],
        }
    }
}
impl<'info> From<AddLiquidityAccounts<'_, 'info>>
for [AccountInfo<'info>; ADD_LIQUIDITY_IX_ACCOUNTS_LEN] {
    fn from(accounts: AddLiquidityAccounts<'_, 'info>) -> Self {
        [
            accounts.pair.clone(),
            accounts.rate_model.clone(),
            accounts.futarchy_authority.clone(),
            accounts.reserve0_vault.clone(),
            accounts.reserve1_vault.clone(),
            accounts.user_token0_account.clone(),
            accounts.user_token1_account.clone(),
            accounts.token0_mint.clone(),
            accounts.token1_mint.clone(),
            accounts.lp_mint.clone(),
            accounts.user_lp_token_account.clone(),
            accounts.user.clone(),
            accounts.token_program.clone(),
            accounts.token_2022_program.clone(),
            accounts.associated_token_program.clone(),
            accounts.system_program.clone(),
            accounts.instructions_sysvar.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; ADD_LIQUIDITY_IX_ACCOUNTS_LEN]>
for AddLiquidityAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; ADD_LIQUIDITY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            pair: &arr[0],
            rate_model: &arr[1],
            futarchy_authority: &arr[2],
            reserve0_vault: &arr[3],
            reserve1_vault: &arr[4],
            user_token0_account: &arr[5],
            user_token1_account: &arr[6],
            token0_mint: &arr[7],
            token1_mint: &arr[8],
            lp_mint: &arr[9],
            user_lp_token_account: &arr[10],
            user: &arr[11],
            token_program: &arr[12],
            token_2022_program: &arr[13],
            associated_token_program: &arr[14],
            system_program: &arr[15],
            instructions_sysvar: &arr[16],
            event_authority: &arr[17],
            program: &arr[18],
        }
    }
}
pub const ADD_LIQUIDITY_IX_DISCM: [u8; 8usize] = [181, 157, 89, 67, 143, 182, 52, 72];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AddLiquidityIxArgs {
    pub args: AddLiquidityArgs,
}
#[derive(Clone, Debug, PartialEq)]
pub struct AddLiquidityIxData(pub AddLiquidityIxArgs);
impl From<AddLiquidityIxArgs> for AddLiquidityIxData {
    fn from(args: AddLiquidityIxArgs) -> Self {
        Self(args)
    }
}
impl AddLiquidityIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != ADD_LIQUIDITY_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let args = if reader.is_empty() {
            Default::default()
        } else {
            <AddLiquidityArgs>::deserialize(&mut reader)?
        };
        Ok(Self(AddLiquidityIxArgs { args }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&ADD_LIQUIDITY_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.args, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn add_liquidity_ix_with_program_id(
    program_id: Pubkey,
    keys: AddLiquidityKeys,
    args: AddLiquidityIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; ADD_LIQUIDITY_IX_ACCOUNTS_LEN] = keys.into();
    let data: AddLiquidityIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn add_liquidity_ix(
    keys: AddLiquidityKeys,
    args: AddLiquidityIxArgs,
) -> std::io::Result<Instruction> {
    add_liquidity_ix_with_program_id(OMNIPAIR_PROGRAM_ID, keys, args)
}
pub fn add_liquidity_invoke_with_program_id(
    program_id: Pubkey,
    accounts: AddLiquidityAccounts<'_, '_>,
    args: AddLiquidityIxArgs,
) -> ProgramResult {
    let keys: AddLiquidityKeys = accounts.into();
    let ix = add_liquidity_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn add_liquidity_invoke(
    accounts: AddLiquidityAccounts<'_, '_>,
    args: AddLiquidityIxArgs,
) -> ProgramResult {
    add_liquidity_invoke_with_program_id(OMNIPAIR_PROGRAM_ID, accounts, args)
}
pub fn add_liquidity_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: AddLiquidityAccounts<'_, '_>,
    args: AddLiquidityIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: AddLiquidityKeys = accounts.into();
    let ix = add_liquidity_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn add_liquidity_invoke_signed(
    accounts: AddLiquidityAccounts<'_, '_>,
    args: AddLiquidityIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    add_liquidity_invoke_signed_with_program_id(
        OMNIPAIR_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn add_liquidity_verify_account_keys(
    accounts: AddLiquidityAccounts<'_, '_>,
    keys: AddLiquidityKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.pair.key, keys.pair),
        (*accounts.rate_model.key, keys.rate_model),
        (*accounts.futarchy_authority.key, keys.futarchy_authority),
        (*accounts.reserve0_vault.key, keys.reserve0_vault),
        (*accounts.reserve1_vault.key, keys.reserve1_vault),
        (*accounts.user_token0_account.key, keys.user_token0_account),
        (*accounts.user_token1_account.key, keys.user_token1_account),
        (*accounts.token0_mint.key, keys.token0_mint),
        (*accounts.token1_mint.key, keys.token1_mint),
        (*accounts.lp_mint.key, keys.lp_mint),
        (*accounts.user_lp_token_account.key, keys.user_lp_token_account),
        (*accounts.user.key, keys.user),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.token_2022_program.key, keys.token_2022_program),
        (*accounts.associated_token_program.key, keys.associated_token_program),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.instructions_sysvar.key, keys.instructions_sysvar),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn add_liquidity_verify_writable_privileges<'me, 'info>(
    accounts: AddLiquidityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.pair,
        accounts.rate_model,
        accounts.reserve0_vault,
        accounts.reserve1_vault,
        accounts.user_token0_account,
        accounts.user_token1_account,
        accounts.lp_mint,
        accounts.user_lp_token_account,
        accounts.user,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn add_liquidity_verify_signer_privileges<'me, 'info>(
    accounts: AddLiquidityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.user] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn add_liquidity_verify_account_privileges<'me, 'info>(
    accounts: AddLiquidityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    add_liquidity_verify_writable_privileges(accounts)?;
    add_liquidity_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const BORROW_IX_ACCOUNTS_LEN: usize = 14;
#[derive(Copy, Clone, Debug)]
pub struct BorrowAccounts<'me, 'info> {
    pub pair: &'me AccountInfo<'info>,
    pub user_position: &'me AccountInfo<'info>,
    pub rate_model: &'me AccountInfo<'info>,
    pub futarchy_authority: &'me AccountInfo<'info>,
    pub reserve_vault: &'me AccountInfo<'info>,
    pub user_reserve_token_account: &'me AccountInfo<'info>,
    pub reserve_token_mint: &'me AccountInfo<'info>,
    pub user: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub token_2022_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub instructions_sysvar: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct BorrowKeys {
    pub pair: Pubkey,
    pub user_position: Pubkey,
    pub rate_model: Pubkey,
    pub futarchy_authority: Pubkey,
    pub reserve_vault: Pubkey,
    pub user_reserve_token_account: Pubkey,
    pub reserve_token_mint: Pubkey,
    pub user: Pubkey,
    pub token_program: Pubkey,
    pub token_2022_program: Pubkey,
    pub system_program: Pubkey,
    pub instructions_sysvar: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<BorrowAccounts<'_, '_>> for BorrowKeys {
    fn from(accounts: BorrowAccounts) -> Self {
        Self {
            pair: *accounts.pair.key,
            user_position: *accounts.user_position.key,
            rate_model: *accounts.rate_model.key,
            futarchy_authority: *accounts.futarchy_authority.key,
            reserve_vault: *accounts.reserve_vault.key,
            user_reserve_token_account: *accounts.user_reserve_token_account.key,
            reserve_token_mint: *accounts.reserve_token_mint.key,
            user: *accounts.user.key,
            token_program: *accounts.token_program.key,
            token_2022_program: *accounts.token_2022_program.key,
            system_program: *accounts.system_program.key,
            instructions_sysvar: *accounts.instructions_sysvar.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<BorrowKeys> for [AccountMeta; BORROW_IX_ACCOUNTS_LEN] {
    fn from(keys: BorrowKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.pair,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_position,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.rate_model,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.futarchy_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.reserve_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_reserve_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.reserve_token_mint,
                is_signer: false,
                is_writable: false,
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
                pubkey: keys.token_2022_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.system_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.instructions_sysvar,
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
impl From<[Pubkey; BORROW_IX_ACCOUNTS_LEN]> for BorrowKeys {
    fn from(pubkeys: [Pubkey; BORROW_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            pair: pubkeys[0],
            user_position: pubkeys[1],
            rate_model: pubkeys[2],
            futarchy_authority: pubkeys[3],
            reserve_vault: pubkeys[4],
            user_reserve_token_account: pubkeys[5],
            reserve_token_mint: pubkeys[6],
            user: pubkeys[7],
            token_program: pubkeys[8],
            token_2022_program: pubkeys[9],
            system_program: pubkeys[10],
            instructions_sysvar: pubkeys[11],
            event_authority: pubkeys[12],
            program: pubkeys[13],
        }
    }
}
impl<'info> From<BorrowAccounts<'_, 'info>>
for [AccountInfo<'info>; BORROW_IX_ACCOUNTS_LEN] {
    fn from(accounts: BorrowAccounts<'_, 'info>) -> Self {
        [
            accounts.pair.clone(),
            accounts.user_position.clone(),
            accounts.rate_model.clone(),
            accounts.futarchy_authority.clone(),
            accounts.reserve_vault.clone(),
            accounts.user_reserve_token_account.clone(),
            accounts.reserve_token_mint.clone(),
            accounts.user.clone(),
            accounts.token_program.clone(),
            accounts.token_2022_program.clone(),
            accounts.system_program.clone(),
            accounts.instructions_sysvar.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; BORROW_IX_ACCOUNTS_LEN]>
for BorrowAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; BORROW_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            pair: &arr[0],
            user_position: &arr[1],
            rate_model: &arr[2],
            futarchy_authority: &arr[3],
            reserve_vault: &arr[4],
            user_reserve_token_account: &arr[5],
            reserve_token_mint: &arr[6],
            user: &arr[7],
            token_program: &arr[8],
            token_2022_program: &arr[9],
            system_program: &arr[10],
            instructions_sysvar: &arr[11],
            event_authority: &arr[12],
            program: &arr[13],
        }
    }
}
pub const BORROW_IX_DISCM: [u8; 8usize] = [228, 253, 131, 202, 207, 116, 89, 18];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct BorrowIxArgs {
    pub args: AdjustDebtArgs,
}
#[derive(Clone, Debug, PartialEq)]
pub struct BorrowIxData(pub BorrowIxArgs);
impl From<BorrowIxArgs> for BorrowIxData {
    fn from(args: BorrowIxArgs) -> Self {
        Self(args)
    }
}
impl BorrowIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != BORROW_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let args = if reader.is_empty() {
            Default::default()
        } else {
            <AdjustDebtArgs>::deserialize(&mut reader)?
        };
        Ok(Self(BorrowIxArgs { args }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&BORROW_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.args, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn borrow_ix_with_program_id(
    program_id: Pubkey,
    keys: BorrowKeys,
    args: BorrowIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; BORROW_IX_ACCOUNTS_LEN] = keys.into();
    let data: BorrowIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn borrow_ix(keys: BorrowKeys, args: BorrowIxArgs) -> std::io::Result<Instruction> {
    borrow_ix_with_program_id(OMNIPAIR_PROGRAM_ID, keys, args)
}
pub fn borrow_invoke_with_program_id(
    program_id: Pubkey,
    accounts: BorrowAccounts<'_, '_>,
    args: BorrowIxArgs,
) -> ProgramResult {
    let keys: BorrowKeys = accounts.into();
    let ix = borrow_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn borrow_invoke(
    accounts: BorrowAccounts<'_, '_>,
    args: BorrowIxArgs,
) -> ProgramResult {
    borrow_invoke_with_program_id(OMNIPAIR_PROGRAM_ID, accounts, args)
}
pub fn borrow_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: BorrowAccounts<'_, '_>,
    args: BorrowIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: BorrowKeys = accounts.into();
    let ix = borrow_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn borrow_invoke_signed(
    accounts: BorrowAccounts<'_, '_>,
    args: BorrowIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    borrow_invoke_signed_with_program_id(OMNIPAIR_PROGRAM_ID, accounts, args, seeds)
}
pub fn borrow_verify_account_keys(
    accounts: BorrowAccounts<'_, '_>,
    keys: BorrowKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.pair.key, keys.pair),
        (*accounts.user_position.key, keys.user_position),
        (*accounts.rate_model.key, keys.rate_model),
        (*accounts.futarchy_authority.key, keys.futarchy_authority),
        (*accounts.reserve_vault.key, keys.reserve_vault),
        (*accounts.user_reserve_token_account.key, keys.user_reserve_token_account),
        (*accounts.reserve_token_mint.key, keys.reserve_token_mint),
        (*accounts.user.key, keys.user),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.token_2022_program.key, keys.token_2022_program),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.instructions_sysvar.key, keys.instructions_sysvar),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn borrow_verify_writable_privileges<'me, 'info>(
    accounts: BorrowAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.pair,
        accounts.user_position,
        accounts.rate_model,
        accounts.reserve_vault,
        accounts.user_reserve_token_account,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn borrow_verify_signer_privileges<'me, 'info>(
    accounts: BorrowAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.user] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn borrow_verify_account_privileges<'me, 'info>(
    accounts: BorrowAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    borrow_verify_writable_privileges(accounts)?;
    borrow_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CLAIM_PROTOCOL_FEES_IX_ACCOUNTS_LEN: usize = 23;
#[derive(Copy, Clone, Debug)]
pub struct ClaimProtocolFeesAccounts<'me, 'info> {
    pub caller: &'me AccountInfo<'info>,
    pub pair: &'me AccountInfo<'info>,
    pub rate_model: &'me AccountInfo<'info>,
    pub futarchy_authority: &'me AccountInfo<'info>,
    pub reserve0_vault: &'me AccountInfo<'info>,
    pub reserve1_vault: &'me AccountInfo<'info>,
    pub token0_mint: &'me AccountInfo<'info>,
    pub token1_mint: &'me AccountInfo<'info>,
    pub futarchy_treasury_token0: &'me AccountInfo<'info>,
    pub futarchy_treasury_token1: &'me AccountInfo<'info>,
    pub futarchy_treasury: &'me AccountInfo<'info>,
    pub buybacks_vault_token0: &'me AccountInfo<'info>,
    pub buybacks_vault_token1: &'me AccountInfo<'info>,
    pub buybacks_vault: &'me AccountInfo<'info>,
    pub team_treasury_token0: &'me AccountInfo<'info>,
    pub team_treasury_token1: &'me AccountInfo<'info>,
    pub team_treasury: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub token_2022_program: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ClaimProtocolFeesKeys {
    pub caller: Pubkey,
    pub pair: Pubkey,
    pub rate_model: Pubkey,
    pub futarchy_authority: Pubkey,
    pub reserve0_vault: Pubkey,
    pub reserve1_vault: Pubkey,
    pub token0_mint: Pubkey,
    pub token1_mint: Pubkey,
    pub futarchy_treasury_token0: Pubkey,
    pub futarchy_treasury_token1: Pubkey,
    pub futarchy_treasury: Pubkey,
    pub buybacks_vault_token0: Pubkey,
    pub buybacks_vault_token1: Pubkey,
    pub buybacks_vault: Pubkey,
    pub team_treasury_token0: Pubkey,
    pub team_treasury_token1: Pubkey,
    pub team_treasury: Pubkey,
    pub token_program: Pubkey,
    pub token_2022_program: Pubkey,
    pub associated_token_program: Pubkey,
    pub system_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<ClaimProtocolFeesAccounts<'_, '_>> for ClaimProtocolFeesKeys {
    fn from(accounts: ClaimProtocolFeesAccounts) -> Self {
        Self {
            caller: *accounts.caller.key,
            pair: *accounts.pair.key,
            rate_model: *accounts.rate_model.key,
            futarchy_authority: *accounts.futarchy_authority.key,
            reserve0_vault: *accounts.reserve0_vault.key,
            reserve1_vault: *accounts.reserve1_vault.key,
            token0_mint: *accounts.token0_mint.key,
            token1_mint: *accounts.token1_mint.key,
            futarchy_treasury_token0: *accounts.futarchy_treasury_token0.key,
            futarchy_treasury_token1: *accounts.futarchy_treasury_token1.key,
            futarchy_treasury: *accounts.futarchy_treasury.key,
            buybacks_vault_token0: *accounts.buybacks_vault_token0.key,
            buybacks_vault_token1: *accounts.buybacks_vault_token1.key,
            buybacks_vault: *accounts.buybacks_vault.key,
            team_treasury_token0: *accounts.team_treasury_token0.key,
            team_treasury_token1: *accounts.team_treasury_token1.key,
            team_treasury: *accounts.team_treasury.key,
            token_program: *accounts.token_program.key,
            token_2022_program: *accounts.token_2022_program.key,
            associated_token_program: *accounts.associated_token_program.key,
            system_program: *accounts.system_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<ClaimProtocolFeesKeys> for [AccountMeta; CLAIM_PROTOCOL_FEES_IX_ACCOUNTS_LEN] {
    fn from(keys: ClaimProtocolFeesKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.caller,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pair,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.rate_model,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.futarchy_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.reserve0_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.reserve1_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token0_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token1_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.futarchy_treasury_token0,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.futarchy_treasury_token1,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.futarchy_treasury,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.buybacks_vault_token0,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.buybacks_vault_token1,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.buybacks_vault,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.team_treasury_token0,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.team_treasury_token1,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.team_treasury,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_2022_program,
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
impl From<[Pubkey; CLAIM_PROTOCOL_FEES_IX_ACCOUNTS_LEN]> for ClaimProtocolFeesKeys {
    fn from(pubkeys: [Pubkey; CLAIM_PROTOCOL_FEES_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            caller: pubkeys[0],
            pair: pubkeys[1],
            rate_model: pubkeys[2],
            futarchy_authority: pubkeys[3],
            reserve0_vault: pubkeys[4],
            reserve1_vault: pubkeys[5],
            token0_mint: pubkeys[6],
            token1_mint: pubkeys[7],
            futarchy_treasury_token0: pubkeys[8],
            futarchy_treasury_token1: pubkeys[9],
            futarchy_treasury: pubkeys[10],
            buybacks_vault_token0: pubkeys[11],
            buybacks_vault_token1: pubkeys[12],
            buybacks_vault: pubkeys[13],
            team_treasury_token0: pubkeys[14],
            team_treasury_token1: pubkeys[15],
            team_treasury: pubkeys[16],
            token_program: pubkeys[17],
            token_2022_program: pubkeys[18],
            associated_token_program: pubkeys[19],
            system_program: pubkeys[20],
            event_authority: pubkeys[21],
            program: pubkeys[22],
        }
    }
}
impl<'info> From<ClaimProtocolFeesAccounts<'_, 'info>>
for [AccountInfo<'info>; CLAIM_PROTOCOL_FEES_IX_ACCOUNTS_LEN] {
    fn from(accounts: ClaimProtocolFeesAccounts<'_, 'info>) -> Self {
        [
            accounts.caller.clone(),
            accounts.pair.clone(),
            accounts.rate_model.clone(),
            accounts.futarchy_authority.clone(),
            accounts.reserve0_vault.clone(),
            accounts.reserve1_vault.clone(),
            accounts.token0_mint.clone(),
            accounts.token1_mint.clone(),
            accounts.futarchy_treasury_token0.clone(),
            accounts.futarchy_treasury_token1.clone(),
            accounts.futarchy_treasury.clone(),
            accounts.buybacks_vault_token0.clone(),
            accounts.buybacks_vault_token1.clone(),
            accounts.buybacks_vault.clone(),
            accounts.team_treasury_token0.clone(),
            accounts.team_treasury_token1.clone(),
            accounts.team_treasury.clone(),
            accounts.token_program.clone(),
            accounts.token_2022_program.clone(),
            accounts.associated_token_program.clone(),
            accounts.system_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CLAIM_PROTOCOL_FEES_IX_ACCOUNTS_LEN]>
for ClaimProtocolFeesAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; CLAIM_PROTOCOL_FEES_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            caller: &arr[0],
            pair: &arr[1],
            rate_model: &arr[2],
            futarchy_authority: &arr[3],
            reserve0_vault: &arr[4],
            reserve1_vault: &arr[5],
            token0_mint: &arr[6],
            token1_mint: &arr[7],
            futarchy_treasury_token0: &arr[8],
            futarchy_treasury_token1: &arr[9],
            futarchy_treasury: &arr[10],
            buybacks_vault_token0: &arr[11],
            buybacks_vault_token1: &arr[12],
            buybacks_vault: &arr[13],
            team_treasury_token0: &arr[14],
            team_treasury_token1: &arr[15],
            team_treasury: &arr[16],
            token_program: &arr[17],
            token_2022_program: &arr[18],
            associated_token_program: &arr[19],
            system_program: &arr[20],
            event_authority: &arr[21],
            program: &arr[22],
        }
    }
}
pub const CLAIM_PROTOCOL_FEES_IX_DISCM: [u8; 8usize] = [
    34, 142, 219, 112, 109, 54, 133, 23,
];
#[derive(Clone, Debug, PartialEq)]
pub struct ClaimProtocolFeesIxData;
impl ClaimProtocolFeesIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CLAIM_PROTOCOL_FEES_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CLAIM_PROTOCOL_FEES_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn claim_protocol_fees_ix_with_program_id(
    program_id: Pubkey,
    keys: ClaimProtocolFeesKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CLAIM_PROTOCOL_FEES_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: ClaimProtocolFeesIxData.try_to_vec()?,
    })
}
pub fn claim_protocol_fees_ix(
    keys: ClaimProtocolFeesKeys,
) -> std::io::Result<Instruction> {
    claim_protocol_fees_ix_with_program_id(OMNIPAIR_PROGRAM_ID, keys)
}
pub fn claim_protocol_fees_invoke_with_program_id(
    program_id: Pubkey,
    accounts: ClaimProtocolFeesAccounts<'_, '_>,
) -> ProgramResult {
    let keys: ClaimProtocolFeesKeys = accounts.into();
    let ix = claim_protocol_fees_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn claim_protocol_fees_invoke(
    accounts: ClaimProtocolFeesAccounts<'_, '_>,
) -> ProgramResult {
    claim_protocol_fees_invoke_with_program_id(OMNIPAIR_PROGRAM_ID, accounts)
}
pub fn claim_protocol_fees_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: ClaimProtocolFeesAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: ClaimProtocolFeesKeys = accounts.into();
    let ix = claim_protocol_fees_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn claim_protocol_fees_invoke_signed(
    accounts: ClaimProtocolFeesAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    claim_protocol_fees_invoke_signed_with_program_id(
        OMNIPAIR_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn claim_protocol_fees_verify_account_keys(
    accounts: ClaimProtocolFeesAccounts<'_, '_>,
    keys: ClaimProtocolFeesKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.caller.key, keys.caller),
        (*accounts.pair.key, keys.pair),
        (*accounts.rate_model.key, keys.rate_model),
        (*accounts.futarchy_authority.key, keys.futarchy_authority),
        (*accounts.reserve0_vault.key, keys.reserve0_vault),
        (*accounts.reserve1_vault.key, keys.reserve1_vault),
        (*accounts.token0_mint.key, keys.token0_mint),
        (*accounts.token1_mint.key, keys.token1_mint),
        (*accounts.futarchy_treasury_token0.key, keys.futarchy_treasury_token0),
        (*accounts.futarchy_treasury_token1.key, keys.futarchy_treasury_token1),
        (*accounts.futarchy_treasury.key, keys.futarchy_treasury),
        (*accounts.buybacks_vault_token0.key, keys.buybacks_vault_token0),
        (*accounts.buybacks_vault_token1.key, keys.buybacks_vault_token1),
        (*accounts.buybacks_vault.key, keys.buybacks_vault),
        (*accounts.team_treasury_token0.key, keys.team_treasury_token0),
        (*accounts.team_treasury_token1.key, keys.team_treasury_token1),
        (*accounts.team_treasury.key, keys.team_treasury),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.token_2022_program.key, keys.token_2022_program),
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
pub fn claim_protocol_fees_verify_writable_privileges<'me, 'info>(
    accounts: ClaimProtocolFeesAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.caller,
        accounts.pair,
        accounts.rate_model,
        accounts.reserve0_vault,
        accounts.reserve1_vault,
        accounts.futarchy_treasury_token0,
        accounts.futarchy_treasury_token1,
        accounts.buybacks_vault_token0,
        accounts.buybacks_vault_token1,
        accounts.team_treasury_token0,
        accounts.team_treasury_token1,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn claim_protocol_fees_verify_signer_privileges<'me, 'info>(
    accounts: ClaimProtocolFeesAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.caller] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn claim_protocol_fees_verify_account_privileges<'me, 'info>(
    accounts: ClaimProtocolFeesAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    claim_protocol_fees_verify_writable_privileges(accounts)?;
    claim_protocol_fees_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CREATE_RATE_MODEL_IX_ACCOUNTS_LEN: usize = 4;
#[derive(Copy, Clone, Debug)]
pub struct CreateRateModelAccounts<'me, 'info> {
    pub authority_signer: &'me AccountInfo<'info>,
    pub futarchy_authority: &'me AccountInfo<'info>,
    pub rate_model: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CreateRateModelKeys {
    pub authority_signer: Pubkey,
    pub futarchy_authority: Pubkey,
    pub rate_model: Pubkey,
    pub system_program: Pubkey,
}
impl From<CreateRateModelAccounts<'_, '_>> for CreateRateModelKeys {
    fn from(accounts: CreateRateModelAccounts) -> Self {
        Self {
            authority_signer: *accounts.authority_signer.key,
            futarchy_authority: *accounts.futarchy_authority.key,
            rate_model: *accounts.rate_model.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<CreateRateModelKeys> for [AccountMeta; CREATE_RATE_MODEL_IX_ACCOUNTS_LEN] {
    fn from(keys: CreateRateModelKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.authority_signer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.futarchy_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.rate_model,
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
impl From<[Pubkey; CREATE_RATE_MODEL_IX_ACCOUNTS_LEN]> for CreateRateModelKeys {
    fn from(pubkeys: [Pubkey; CREATE_RATE_MODEL_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            authority_signer: pubkeys[0],
            futarchy_authority: pubkeys[1],
            rate_model: pubkeys[2],
            system_program: pubkeys[3],
        }
    }
}
impl<'info> From<CreateRateModelAccounts<'_, 'info>>
for [AccountInfo<'info>; CREATE_RATE_MODEL_IX_ACCOUNTS_LEN] {
    fn from(accounts: CreateRateModelAccounts<'_, 'info>) -> Self {
        [
            accounts.authority_signer.clone(),
            accounts.futarchy_authority.clone(),
            accounts.rate_model.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CREATE_RATE_MODEL_IX_ACCOUNTS_LEN]>
for CreateRateModelAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; CREATE_RATE_MODEL_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            authority_signer: &arr[0],
            futarchy_authority: &arr[1],
            rate_model: &arr[2],
            system_program: &arr[3],
        }
    }
}
pub const CREATE_RATE_MODEL_IX_DISCM: [u8; 8usize] = [
    128, 221, 68, 197, 26, 158, 219, 171,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CreateRateModelIxArgs {
    pub args: CreateRateModelArgs,
}
#[derive(Clone, Debug, PartialEq)]
pub struct CreateRateModelIxData(pub CreateRateModelIxArgs);
impl From<CreateRateModelIxArgs> for CreateRateModelIxData {
    fn from(args: CreateRateModelIxArgs) -> Self {
        Self(args)
    }
}
impl CreateRateModelIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CREATE_RATE_MODEL_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let args = if reader.is_empty() {
            Default::default()
        } else {
            <CreateRateModelArgs>::deserialize(&mut reader)?
        };
        Ok(Self(CreateRateModelIxArgs { args }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CREATE_RATE_MODEL_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.args, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn create_rate_model_ix_with_program_id(
    program_id: Pubkey,
    keys: CreateRateModelKeys,
    args: CreateRateModelIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CREATE_RATE_MODEL_IX_ACCOUNTS_LEN] = keys.into();
    let data: CreateRateModelIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn create_rate_model_ix(
    keys: CreateRateModelKeys,
    args: CreateRateModelIxArgs,
) -> std::io::Result<Instruction> {
    create_rate_model_ix_with_program_id(OMNIPAIR_PROGRAM_ID, keys, args)
}
pub fn create_rate_model_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CreateRateModelAccounts<'_, '_>,
    args: CreateRateModelIxArgs,
) -> ProgramResult {
    let keys: CreateRateModelKeys = accounts.into();
    let ix = create_rate_model_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn create_rate_model_invoke(
    accounts: CreateRateModelAccounts<'_, '_>,
    args: CreateRateModelIxArgs,
) -> ProgramResult {
    create_rate_model_invoke_with_program_id(OMNIPAIR_PROGRAM_ID, accounts, args)
}
pub fn create_rate_model_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CreateRateModelAccounts<'_, '_>,
    args: CreateRateModelIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CreateRateModelKeys = accounts.into();
    let ix = create_rate_model_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn create_rate_model_invoke_signed(
    accounts: CreateRateModelAccounts<'_, '_>,
    args: CreateRateModelIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    create_rate_model_invoke_signed_with_program_id(
        OMNIPAIR_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn create_rate_model_verify_account_keys(
    accounts: CreateRateModelAccounts<'_, '_>,
    keys: CreateRateModelKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.authority_signer.key, keys.authority_signer),
        (*accounts.futarchy_authority.key, keys.futarchy_authority),
        (*accounts.rate_model.key, keys.rate_model),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn create_rate_model_verify_writable_privileges<'me, 'info>(
    accounts: CreateRateModelAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.authority_signer, accounts.rate_model] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn create_rate_model_verify_signer_privileges<'me, 'info>(
    accounts: CreateRateModelAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority_signer, accounts.rate_model] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn create_rate_model_verify_account_privileges<'me, 'info>(
    accounts: CreateRateModelAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    create_rate_model_verify_writable_privileges(accounts)?;
    create_rate_model_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const FLASHLOAN_IX_ACCOUNTS_LEN: usize = 16;
#[derive(Copy, Clone, Debug)]
pub struct FlashloanAccounts<'me, 'info> {
    pub pair: &'me AccountInfo<'info>,
    pub rate_model: &'me AccountInfo<'info>,
    pub futarchy_authority: &'me AccountInfo<'info>,
    pub reserve0_vault: &'me AccountInfo<'info>,
    pub reserve1_vault: &'me AccountInfo<'info>,
    pub token0_mint: &'me AccountInfo<'info>,
    pub token1_mint: &'me AccountInfo<'info>,
    pub receiver_token0_account: &'me AccountInfo<'info>,
    pub receiver_token1_account: &'me AccountInfo<'info>,
    pub receiver_program: &'me AccountInfo<'info>,
    pub user: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub token_2022_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FlashloanKeys {
    pub pair: Pubkey,
    pub rate_model: Pubkey,
    pub futarchy_authority: Pubkey,
    pub reserve0_vault: Pubkey,
    pub reserve1_vault: Pubkey,
    pub token0_mint: Pubkey,
    pub token1_mint: Pubkey,
    pub receiver_token0_account: Pubkey,
    pub receiver_token1_account: Pubkey,
    pub receiver_program: Pubkey,
    pub user: Pubkey,
    pub token_program: Pubkey,
    pub token_2022_program: Pubkey,
    pub system_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<FlashloanAccounts<'_, '_>> for FlashloanKeys {
    fn from(accounts: FlashloanAccounts) -> Self {
        Self {
            pair: *accounts.pair.key,
            rate_model: *accounts.rate_model.key,
            futarchy_authority: *accounts.futarchy_authority.key,
            reserve0_vault: *accounts.reserve0_vault.key,
            reserve1_vault: *accounts.reserve1_vault.key,
            token0_mint: *accounts.token0_mint.key,
            token1_mint: *accounts.token1_mint.key,
            receiver_token0_account: *accounts.receiver_token0_account.key,
            receiver_token1_account: *accounts.receiver_token1_account.key,
            receiver_program: *accounts.receiver_program.key,
            user: *accounts.user.key,
            token_program: *accounts.token_program.key,
            token_2022_program: *accounts.token_2022_program.key,
            system_program: *accounts.system_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<FlashloanKeys> for [AccountMeta; FLASHLOAN_IX_ACCOUNTS_LEN] {
    fn from(keys: FlashloanKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.pair,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.rate_model,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.futarchy_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.reserve0_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.reserve1_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token0_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token1_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.receiver_token0_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.receiver_token1_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.receiver_program,
                is_signer: false,
                is_writable: false,
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
                pubkey: keys.token_2022_program,
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
impl From<[Pubkey; FLASHLOAN_IX_ACCOUNTS_LEN]> for FlashloanKeys {
    fn from(pubkeys: [Pubkey; FLASHLOAN_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            pair: pubkeys[0],
            rate_model: pubkeys[1],
            futarchy_authority: pubkeys[2],
            reserve0_vault: pubkeys[3],
            reserve1_vault: pubkeys[4],
            token0_mint: pubkeys[5],
            token1_mint: pubkeys[6],
            receiver_token0_account: pubkeys[7],
            receiver_token1_account: pubkeys[8],
            receiver_program: pubkeys[9],
            user: pubkeys[10],
            token_program: pubkeys[11],
            token_2022_program: pubkeys[12],
            system_program: pubkeys[13],
            event_authority: pubkeys[14],
            program: pubkeys[15],
        }
    }
}
impl<'info> From<FlashloanAccounts<'_, 'info>>
for [AccountInfo<'info>; FLASHLOAN_IX_ACCOUNTS_LEN] {
    fn from(accounts: FlashloanAccounts<'_, 'info>) -> Self {
        [
            accounts.pair.clone(),
            accounts.rate_model.clone(),
            accounts.futarchy_authority.clone(),
            accounts.reserve0_vault.clone(),
            accounts.reserve1_vault.clone(),
            accounts.token0_mint.clone(),
            accounts.token1_mint.clone(),
            accounts.receiver_token0_account.clone(),
            accounts.receiver_token1_account.clone(),
            accounts.receiver_program.clone(),
            accounts.user.clone(),
            accounts.token_program.clone(),
            accounts.token_2022_program.clone(),
            accounts.system_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; FLASHLOAN_IX_ACCOUNTS_LEN]>
for FlashloanAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; FLASHLOAN_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            pair: &arr[0],
            rate_model: &arr[1],
            futarchy_authority: &arr[2],
            reserve0_vault: &arr[3],
            reserve1_vault: &arr[4],
            token0_mint: &arr[5],
            token1_mint: &arr[6],
            receiver_token0_account: &arr[7],
            receiver_token1_account: &arr[8],
            receiver_program: &arr[9],
            user: &arr[10],
            token_program: &arr[11],
            token_2022_program: &arr[12],
            system_program: &arr[13],
            event_authority: &arr[14],
            program: &arr[15],
        }
    }
}
pub const FLASHLOAN_IX_DISCM: [u8; 8usize] = [105, 33, 1, 3, 42, 158, 246, 67];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct FlashloanIxArgs {
    pub args: FlashloanArgs,
}
#[derive(Clone, Debug, PartialEq)]
pub struct FlashloanIxData(pub FlashloanIxArgs);
impl From<FlashloanIxArgs> for FlashloanIxData {
    fn from(args: FlashloanIxArgs) -> Self {
        Self(args)
    }
}
impl FlashloanIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != FLASHLOAN_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let args = if reader.is_empty() {
            Default::default()
        } else {
            <FlashloanArgs>::deserialize(&mut reader)?
        };
        Ok(Self(FlashloanIxArgs { args }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&FLASHLOAN_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.args, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn flashloan_ix_with_program_id(
    program_id: Pubkey,
    keys: FlashloanKeys,
    args: FlashloanIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; FLASHLOAN_IX_ACCOUNTS_LEN] = keys.into();
    let data: FlashloanIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn flashloan_ix(
    keys: FlashloanKeys,
    args: FlashloanIxArgs,
) -> std::io::Result<Instruction> {
    flashloan_ix_with_program_id(OMNIPAIR_PROGRAM_ID, keys, args)
}
pub fn flashloan_invoke_with_program_id(
    program_id: Pubkey,
    accounts: FlashloanAccounts<'_, '_>,
    args: FlashloanIxArgs,
) -> ProgramResult {
    let keys: FlashloanKeys = accounts.into();
    let ix = flashloan_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn flashloan_invoke(
    accounts: FlashloanAccounts<'_, '_>,
    args: FlashloanIxArgs,
) -> ProgramResult {
    flashloan_invoke_with_program_id(OMNIPAIR_PROGRAM_ID, accounts, args)
}
pub fn flashloan_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: FlashloanAccounts<'_, '_>,
    args: FlashloanIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: FlashloanKeys = accounts.into();
    let ix = flashloan_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn flashloan_invoke_signed(
    accounts: FlashloanAccounts<'_, '_>,
    args: FlashloanIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    flashloan_invoke_signed_with_program_id(OMNIPAIR_PROGRAM_ID, accounts, args, seeds)
}
pub fn flashloan_verify_account_keys(
    accounts: FlashloanAccounts<'_, '_>,
    keys: FlashloanKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.pair.key, keys.pair),
        (*accounts.rate_model.key, keys.rate_model),
        (*accounts.futarchy_authority.key, keys.futarchy_authority),
        (*accounts.reserve0_vault.key, keys.reserve0_vault),
        (*accounts.reserve1_vault.key, keys.reserve1_vault),
        (*accounts.token0_mint.key, keys.token0_mint),
        (*accounts.token1_mint.key, keys.token1_mint),
        (*accounts.receiver_token0_account.key, keys.receiver_token0_account),
        (*accounts.receiver_token1_account.key, keys.receiver_token1_account),
        (*accounts.receiver_program.key, keys.receiver_program),
        (*accounts.user.key, keys.user),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.token_2022_program.key, keys.token_2022_program),
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
pub fn flashloan_verify_writable_privileges<'me, 'info>(
    accounts: FlashloanAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.pair,
        accounts.rate_model,
        accounts.reserve0_vault,
        accounts.reserve1_vault,
        accounts.receiver_token0_account,
        accounts.receiver_token1_account,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn flashloan_verify_signer_privileges<'me, 'info>(
    accounts: FlashloanAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.user] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn flashloan_verify_account_privileges<'me, 'info>(
    accounts: FlashloanAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    flashloan_verify_writable_privileges(accounts)?;
    flashloan_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const INIT_FUTARCHY_AUTHORITY_IX_ACCOUNTS_LEN: usize = 4;
#[derive(Copy, Clone, Debug)]
pub struct InitFutarchyAuthorityAccounts<'me, 'info> {
    pub deployer: &'me AccountInfo<'info>,
    pub futarchy_authority: &'me AccountInfo<'info>,
    pub program_data: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct InitFutarchyAuthorityKeys {
    pub deployer: Pubkey,
    pub futarchy_authority: Pubkey,
    pub program_data: Pubkey,
    pub system_program: Pubkey,
}
impl From<InitFutarchyAuthorityAccounts<'_, '_>> for InitFutarchyAuthorityKeys {
    fn from(accounts: InitFutarchyAuthorityAccounts) -> Self {
        Self {
            deployer: *accounts.deployer.key,
            futarchy_authority: *accounts.futarchy_authority.key,
            program_data: *accounts.program_data.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<InitFutarchyAuthorityKeys>
for [AccountMeta; INIT_FUTARCHY_AUTHORITY_IX_ACCOUNTS_LEN] {
    fn from(keys: InitFutarchyAuthorityKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.deployer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.futarchy_authority,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.program_data,
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
impl From<[Pubkey; INIT_FUTARCHY_AUTHORITY_IX_ACCOUNTS_LEN]>
for InitFutarchyAuthorityKeys {
    fn from(pubkeys: [Pubkey; INIT_FUTARCHY_AUTHORITY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            deployer: pubkeys[0],
            futarchy_authority: pubkeys[1],
            program_data: pubkeys[2],
            system_program: pubkeys[3],
        }
    }
}
impl<'info> From<InitFutarchyAuthorityAccounts<'_, 'info>>
for [AccountInfo<'info>; INIT_FUTARCHY_AUTHORITY_IX_ACCOUNTS_LEN] {
    fn from(accounts: InitFutarchyAuthorityAccounts<'_, 'info>) -> Self {
        [
            accounts.deployer.clone(),
            accounts.futarchy_authority.clone(),
            accounts.program_data.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; INIT_FUTARCHY_AUTHORITY_IX_ACCOUNTS_LEN]>
for InitFutarchyAuthorityAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; INIT_FUTARCHY_AUTHORITY_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            deployer: &arr[0],
            futarchy_authority: &arr[1],
            program_data: &arr[2],
            system_program: &arr[3],
        }
    }
}
pub const INIT_FUTARCHY_AUTHORITY_IX_DISCM: [u8; 8usize] = [
    133, 110, 154, 29, 240, 206, 71, 100,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct InitFutarchyAuthorityIxArgs {
    pub args: InitFutarchyAuthorityArgs,
}
#[derive(Clone, Debug, PartialEq)]
pub struct InitFutarchyAuthorityIxData(pub InitFutarchyAuthorityIxArgs);
impl From<InitFutarchyAuthorityIxArgs> for InitFutarchyAuthorityIxData {
    fn from(args: InitFutarchyAuthorityIxArgs) -> Self {
        Self(args)
    }
}
impl InitFutarchyAuthorityIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INIT_FUTARCHY_AUTHORITY_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let args = if reader.is_empty() {
            Default::default()
        } else {
            <InitFutarchyAuthorityArgs>::deserialize(&mut reader)?
        };
        Ok(
            Self(InitFutarchyAuthorityIxArgs {
                args,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INIT_FUTARCHY_AUTHORITY_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.args, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn init_futarchy_authority_ix_with_program_id(
    program_id: Pubkey,
    keys: InitFutarchyAuthorityKeys,
    args: InitFutarchyAuthorityIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; INIT_FUTARCHY_AUTHORITY_IX_ACCOUNTS_LEN] = keys.into();
    let data: InitFutarchyAuthorityIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn init_futarchy_authority_ix(
    keys: InitFutarchyAuthorityKeys,
    args: InitFutarchyAuthorityIxArgs,
) -> std::io::Result<Instruction> {
    init_futarchy_authority_ix_with_program_id(OMNIPAIR_PROGRAM_ID, keys, args)
}
pub fn init_futarchy_authority_invoke_with_program_id(
    program_id: Pubkey,
    accounts: InitFutarchyAuthorityAccounts<'_, '_>,
    args: InitFutarchyAuthorityIxArgs,
) -> ProgramResult {
    let keys: InitFutarchyAuthorityKeys = accounts.into();
    let ix = init_futarchy_authority_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn init_futarchy_authority_invoke(
    accounts: InitFutarchyAuthorityAccounts<'_, '_>,
    args: InitFutarchyAuthorityIxArgs,
) -> ProgramResult {
    init_futarchy_authority_invoke_with_program_id(OMNIPAIR_PROGRAM_ID, accounts, args)
}
pub fn init_futarchy_authority_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: InitFutarchyAuthorityAccounts<'_, '_>,
    args: InitFutarchyAuthorityIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: InitFutarchyAuthorityKeys = accounts.into();
    let ix = init_futarchy_authority_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn init_futarchy_authority_invoke_signed(
    accounts: InitFutarchyAuthorityAccounts<'_, '_>,
    args: InitFutarchyAuthorityIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    init_futarchy_authority_invoke_signed_with_program_id(
        OMNIPAIR_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn init_futarchy_authority_verify_account_keys(
    accounts: InitFutarchyAuthorityAccounts<'_, '_>,
    keys: InitFutarchyAuthorityKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.deployer.key, keys.deployer),
        (*accounts.futarchy_authority.key, keys.futarchy_authority),
        (*accounts.program_data.key, keys.program_data),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn init_futarchy_authority_verify_writable_privileges<'me, 'info>(
    accounts: InitFutarchyAuthorityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.deployer, accounts.futarchy_authority] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn init_futarchy_authority_verify_signer_privileges<'me, 'info>(
    accounts: InitFutarchyAuthorityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.deployer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn init_futarchy_authority_verify_account_privileges<'me, 'info>(
    accounts: InitFutarchyAuthorityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    init_futarchy_authority_verify_writable_privileges(accounts)?;
    init_futarchy_authority_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const INITIALIZE_IX_ACCOUNTS_LEN: usize = 25;
#[derive(Copy, Clone, Debug)]
pub struct InitializeAccounts<'me, 'info> {
    pub deployer: &'me AccountInfo<'info>,
    pub token0_mint: &'me AccountInfo<'info>,
    pub token1_mint: &'me AccountInfo<'info>,
    pub pair: &'me AccountInfo<'info>,
    pub futarchy_authority: &'me AccountInfo<'info>,
    pub rate_model: &'me AccountInfo<'info>,
    pub lp_mint: &'me AccountInfo<'info>,
    pub lp_token_metadata: &'me AccountInfo<'info>,
    pub deployer_lp_token_account: &'me AccountInfo<'info>,
    pub reserve0_vault: &'me AccountInfo<'info>,
    pub reserve1_vault: &'me AccountInfo<'info>,
    pub collateral0_vault: &'me AccountInfo<'info>,
    pub collateral1_vault: &'me AccountInfo<'info>,
    pub deployer_token0_account: &'me AccountInfo<'info>,
    pub deployer_token1_account: &'me AccountInfo<'info>,
    pub team_treasury: &'me AccountInfo<'info>,
    pub team_treasury_wsol_account: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub token_2022_program: &'me AccountInfo<'info>,
    pub token_metadata_program: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
    pub rent: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct InitializeKeys {
    pub deployer: Pubkey,
    pub token0_mint: Pubkey,
    pub token1_mint: Pubkey,
    pub pair: Pubkey,
    pub futarchy_authority: Pubkey,
    pub rate_model: Pubkey,
    pub lp_mint: Pubkey,
    pub lp_token_metadata: Pubkey,
    pub deployer_lp_token_account: Pubkey,
    pub reserve0_vault: Pubkey,
    pub reserve1_vault: Pubkey,
    pub collateral0_vault: Pubkey,
    pub collateral1_vault: Pubkey,
    pub deployer_token0_account: Pubkey,
    pub deployer_token1_account: Pubkey,
    pub team_treasury: Pubkey,
    pub team_treasury_wsol_account: Pubkey,
    pub system_program: Pubkey,
    pub token_program: Pubkey,
    pub token_2022_program: Pubkey,
    pub token_metadata_program: Pubkey,
    pub associated_token_program: Pubkey,
    pub rent: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<InitializeAccounts<'_, '_>> for InitializeKeys {
    fn from(accounts: InitializeAccounts) -> Self {
        Self {
            deployer: *accounts.deployer.key,
            token0_mint: *accounts.token0_mint.key,
            token1_mint: *accounts.token1_mint.key,
            pair: *accounts.pair.key,
            futarchy_authority: *accounts.futarchy_authority.key,
            rate_model: *accounts.rate_model.key,
            lp_mint: *accounts.lp_mint.key,
            lp_token_metadata: *accounts.lp_token_metadata.key,
            deployer_lp_token_account: *accounts.deployer_lp_token_account.key,
            reserve0_vault: *accounts.reserve0_vault.key,
            reserve1_vault: *accounts.reserve1_vault.key,
            collateral0_vault: *accounts.collateral0_vault.key,
            collateral1_vault: *accounts.collateral1_vault.key,
            deployer_token0_account: *accounts.deployer_token0_account.key,
            deployer_token1_account: *accounts.deployer_token1_account.key,
            team_treasury: *accounts.team_treasury.key,
            team_treasury_wsol_account: *accounts.team_treasury_wsol_account.key,
            system_program: *accounts.system_program.key,
            token_program: *accounts.token_program.key,
            token_2022_program: *accounts.token_2022_program.key,
            token_metadata_program: *accounts.token_metadata_program.key,
            associated_token_program: *accounts.associated_token_program.key,
            rent: *accounts.rent.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<InitializeKeys> for [AccountMeta; INITIALIZE_IX_ACCOUNTS_LEN] {
    fn from(keys: InitializeKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.deployer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token0_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token1_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pair,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.futarchy_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.rate_model,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.lp_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.lp_token_metadata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.deployer_lp_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.reserve0_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.reserve1_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.collateral0_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.collateral1_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.deployer_token0_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.deployer_token1_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.team_treasury,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.team_treasury_wsol_account,
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
                pubkey: keys.token_2022_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_metadata_program,
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
impl From<[Pubkey; INITIALIZE_IX_ACCOUNTS_LEN]> for InitializeKeys {
    fn from(pubkeys: [Pubkey; INITIALIZE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            deployer: pubkeys[0],
            token0_mint: pubkeys[1],
            token1_mint: pubkeys[2],
            pair: pubkeys[3],
            futarchy_authority: pubkeys[4],
            rate_model: pubkeys[5],
            lp_mint: pubkeys[6],
            lp_token_metadata: pubkeys[7],
            deployer_lp_token_account: pubkeys[8],
            reserve0_vault: pubkeys[9],
            reserve1_vault: pubkeys[10],
            collateral0_vault: pubkeys[11],
            collateral1_vault: pubkeys[12],
            deployer_token0_account: pubkeys[13],
            deployer_token1_account: pubkeys[14],
            team_treasury: pubkeys[15],
            team_treasury_wsol_account: pubkeys[16],
            system_program: pubkeys[17],
            token_program: pubkeys[18],
            token_2022_program: pubkeys[19],
            token_metadata_program: pubkeys[20],
            associated_token_program: pubkeys[21],
            rent: pubkeys[22],
            event_authority: pubkeys[23],
            program: pubkeys[24],
        }
    }
}
impl<'info> From<InitializeAccounts<'_, 'info>>
for [AccountInfo<'info>; INITIALIZE_IX_ACCOUNTS_LEN] {
    fn from(accounts: InitializeAccounts<'_, 'info>) -> Self {
        [
            accounts.deployer.clone(),
            accounts.token0_mint.clone(),
            accounts.token1_mint.clone(),
            accounts.pair.clone(),
            accounts.futarchy_authority.clone(),
            accounts.rate_model.clone(),
            accounts.lp_mint.clone(),
            accounts.lp_token_metadata.clone(),
            accounts.deployer_lp_token_account.clone(),
            accounts.reserve0_vault.clone(),
            accounts.reserve1_vault.clone(),
            accounts.collateral0_vault.clone(),
            accounts.collateral1_vault.clone(),
            accounts.deployer_token0_account.clone(),
            accounts.deployer_token1_account.clone(),
            accounts.team_treasury.clone(),
            accounts.team_treasury_wsol_account.clone(),
            accounts.system_program.clone(),
            accounts.token_program.clone(),
            accounts.token_2022_program.clone(),
            accounts.token_metadata_program.clone(),
            accounts.associated_token_program.clone(),
            accounts.rent.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; INITIALIZE_IX_ACCOUNTS_LEN]>
for InitializeAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; INITIALIZE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            deployer: &arr[0],
            token0_mint: &arr[1],
            token1_mint: &arr[2],
            pair: &arr[3],
            futarchy_authority: &arr[4],
            rate_model: &arr[5],
            lp_mint: &arr[6],
            lp_token_metadata: &arr[7],
            deployer_lp_token_account: &arr[8],
            reserve0_vault: &arr[9],
            reserve1_vault: &arr[10],
            collateral0_vault: &arr[11],
            collateral1_vault: &arr[12],
            deployer_token0_account: &arr[13],
            deployer_token1_account: &arr[14],
            team_treasury: &arr[15],
            team_treasury_wsol_account: &arr[16],
            system_program: &arr[17],
            token_program: &arr[18],
            token_2022_program: &arr[19],
            token_metadata_program: &arr[20],
            associated_token_program: &arr[21],
            rent: &arr[22],
            event_authority: &arr[23],
            program: &arr[24],
        }
    }
}
pub const INITIALIZE_IX_DISCM: [u8; 8usize] = [175, 175, 109, 31, 13, 152, 155, 237];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct InitializeIxArgs {
    pub args: InitializeAndBootstrapArgs,
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
        let args = if reader.is_empty() {
            Default::default()
        } else {
            <InitializeAndBootstrapArgs>::deserialize(&mut reader)?
        };
        Ok(Self(InitializeIxArgs { args }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INITIALIZE_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.args, &mut writer)?;
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
    initialize_ix_with_program_id(OMNIPAIR_PROGRAM_ID, keys, args)
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
    initialize_invoke_with_program_id(OMNIPAIR_PROGRAM_ID, accounts, args)
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
    initialize_invoke_signed_with_program_id(OMNIPAIR_PROGRAM_ID, accounts, args, seeds)
}
pub fn initialize_verify_account_keys(
    accounts: InitializeAccounts<'_, '_>,
    keys: InitializeKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.deployer.key, keys.deployer),
        (*accounts.token0_mint.key, keys.token0_mint),
        (*accounts.token1_mint.key, keys.token1_mint),
        (*accounts.pair.key, keys.pair),
        (*accounts.futarchy_authority.key, keys.futarchy_authority),
        (*accounts.rate_model.key, keys.rate_model),
        (*accounts.lp_mint.key, keys.lp_mint),
        (*accounts.lp_token_metadata.key, keys.lp_token_metadata),
        (*accounts.deployer_lp_token_account.key, keys.deployer_lp_token_account),
        (*accounts.reserve0_vault.key, keys.reserve0_vault),
        (*accounts.reserve1_vault.key, keys.reserve1_vault),
        (*accounts.collateral0_vault.key, keys.collateral0_vault),
        (*accounts.collateral1_vault.key, keys.collateral1_vault),
        (*accounts.deployer_token0_account.key, keys.deployer_token0_account),
        (*accounts.deployer_token1_account.key, keys.deployer_token1_account),
        (*accounts.team_treasury.key, keys.team_treasury),
        (*accounts.team_treasury_wsol_account.key, keys.team_treasury_wsol_account),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.token_2022_program.key, keys.token_2022_program),
        (*accounts.token_metadata_program.key, keys.token_metadata_program),
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
pub fn initialize_verify_writable_privileges<'me, 'info>(
    accounts: InitializeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.deployer,
        accounts.pair,
        accounts.rate_model,
        accounts.lp_mint,
        accounts.lp_token_metadata,
        accounts.deployer_lp_token_account,
        accounts.reserve0_vault,
        accounts.reserve1_vault,
        accounts.collateral0_vault,
        accounts.collateral1_vault,
        accounts.deployer_token0_account,
        accounts.deployer_token1_account,
        accounts.team_treasury_wsol_account,
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
    for should_be_signer in [accounts.deployer] {
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
pub const LIQUIDATE_IX_ACCOUNTS_LEN: usize = 15;
#[derive(Copy, Clone, Debug)]
pub struct LiquidateAccounts<'me, 'info> {
    pub pair: &'me AccountInfo<'info>,
    pub user_position: &'me AccountInfo<'info>,
    pub rate_model: &'me AccountInfo<'info>,
    pub futarchy_authority: &'me AccountInfo<'info>,
    pub collateral_vault: &'me AccountInfo<'info>,
    pub caller_token_account: &'me AccountInfo<'info>,
    pub collateral_token_mint: &'me AccountInfo<'info>,
    pub reserve_vault: &'me AccountInfo<'info>,
    pub position_owner: &'me AccountInfo<'info>,
    pub payer: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub token_2022_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LiquidateKeys {
    pub pair: Pubkey,
    pub user_position: Pubkey,
    pub rate_model: Pubkey,
    pub futarchy_authority: Pubkey,
    pub collateral_vault: Pubkey,
    pub caller_token_account: Pubkey,
    pub collateral_token_mint: Pubkey,
    pub reserve_vault: Pubkey,
    pub position_owner: Pubkey,
    pub payer: Pubkey,
    pub token_program: Pubkey,
    pub token_2022_program: Pubkey,
    pub system_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<LiquidateAccounts<'_, '_>> for LiquidateKeys {
    fn from(accounts: LiquidateAccounts) -> Self {
        Self {
            pair: *accounts.pair.key,
            user_position: *accounts.user_position.key,
            rate_model: *accounts.rate_model.key,
            futarchy_authority: *accounts.futarchy_authority.key,
            collateral_vault: *accounts.collateral_vault.key,
            caller_token_account: *accounts.caller_token_account.key,
            collateral_token_mint: *accounts.collateral_token_mint.key,
            reserve_vault: *accounts.reserve_vault.key,
            position_owner: *accounts.position_owner.key,
            payer: *accounts.payer.key,
            token_program: *accounts.token_program.key,
            token_2022_program: *accounts.token_2022_program.key,
            system_program: *accounts.system_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<LiquidateKeys> for [AccountMeta; LIQUIDATE_IX_ACCOUNTS_LEN] {
    fn from(keys: LiquidateKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.pair,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_position,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.rate_model,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.futarchy_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.collateral_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.caller_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.collateral_token_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.reserve_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.position_owner,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.payer,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_2022_program,
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
impl From<[Pubkey; LIQUIDATE_IX_ACCOUNTS_LEN]> for LiquidateKeys {
    fn from(pubkeys: [Pubkey; LIQUIDATE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            pair: pubkeys[0],
            user_position: pubkeys[1],
            rate_model: pubkeys[2],
            futarchy_authority: pubkeys[3],
            collateral_vault: pubkeys[4],
            caller_token_account: pubkeys[5],
            collateral_token_mint: pubkeys[6],
            reserve_vault: pubkeys[7],
            position_owner: pubkeys[8],
            payer: pubkeys[9],
            token_program: pubkeys[10],
            token_2022_program: pubkeys[11],
            system_program: pubkeys[12],
            event_authority: pubkeys[13],
            program: pubkeys[14],
        }
    }
}
impl<'info> From<LiquidateAccounts<'_, 'info>>
for [AccountInfo<'info>; LIQUIDATE_IX_ACCOUNTS_LEN] {
    fn from(accounts: LiquidateAccounts<'_, 'info>) -> Self {
        [
            accounts.pair.clone(),
            accounts.user_position.clone(),
            accounts.rate_model.clone(),
            accounts.futarchy_authority.clone(),
            accounts.collateral_vault.clone(),
            accounts.caller_token_account.clone(),
            accounts.collateral_token_mint.clone(),
            accounts.reserve_vault.clone(),
            accounts.position_owner.clone(),
            accounts.payer.clone(),
            accounts.token_program.clone(),
            accounts.token_2022_program.clone(),
            accounts.system_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; LIQUIDATE_IX_ACCOUNTS_LEN]>
for LiquidateAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; LIQUIDATE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            pair: &arr[0],
            user_position: &arr[1],
            rate_model: &arr[2],
            futarchy_authority: &arr[3],
            collateral_vault: &arr[4],
            caller_token_account: &arr[5],
            collateral_token_mint: &arr[6],
            reserve_vault: &arr[7],
            position_owner: &arr[8],
            payer: &arr[9],
            token_program: &arr[10],
            token_2022_program: &arr[11],
            system_program: &arr[12],
            event_authority: &arr[13],
            program: &arr[14],
        }
    }
}
pub const LIQUIDATE_IX_DISCM: [u8; 8usize] = [223, 179, 226, 125, 48, 46, 39, 74];
#[derive(Clone, Debug, PartialEq)]
pub struct LiquidateIxData;
impl LiquidateIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != LIQUIDATE_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&LIQUIDATE_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn liquidate_ix_with_program_id(
    program_id: Pubkey,
    keys: LiquidateKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; LIQUIDATE_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: LiquidateIxData.try_to_vec()?,
    })
}
pub fn liquidate_ix(keys: LiquidateKeys) -> std::io::Result<Instruction> {
    liquidate_ix_with_program_id(OMNIPAIR_PROGRAM_ID, keys)
}
pub fn liquidate_invoke_with_program_id(
    program_id: Pubkey,
    accounts: LiquidateAccounts<'_, '_>,
) -> ProgramResult {
    let keys: LiquidateKeys = accounts.into();
    let ix = liquidate_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn liquidate_invoke(accounts: LiquidateAccounts<'_, '_>) -> ProgramResult {
    liquidate_invoke_with_program_id(OMNIPAIR_PROGRAM_ID, accounts)
}
pub fn liquidate_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: LiquidateAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: LiquidateKeys = accounts.into();
    let ix = liquidate_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn liquidate_invoke_signed(
    accounts: LiquidateAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    liquidate_invoke_signed_with_program_id(OMNIPAIR_PROGRAM_ID, accounts, seeds)
}
pub fn liquidate_verify_account_keys(
    accounts: LiquidateAccounts<'_, '_>,
    keys: LiquidateKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.pair.key, keys.pair),
        (*accounts.user_position.key, keys.user_position),
        (*accounts.rate_model.key, keys.rate_model),
        (*accounts.futarchy_authority.key, keys.futarchy_authority),
        (*accounts.collateral_vault.key, keys.collateral_vault),
        (*accounts.caller_token_account.key, keys.caller_token_account),
        (*accounts.collateral_token_mint.key, keys.collateral_token_mint),
        (*accounts.reserve_vault.key, keys.reserve_vault),
        (*accounts.position_owner.key, keys.position_owner),
        (*accounts.payer.key, keys.payer),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.token_2022_program.key, keys.token_2022_program),
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
pub fn liquidate_verify_writable_privileges<'me, 'info>(
    accounts: LiquidateAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.pair,
        accounts.user_position,
        accounts.rate_model,
        accounts.collateral_vault,
        accounts.caller_token_account,
        accounts.reserve_vault,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn liquidate_verify_signer_privileges<'me, 'info>(
    accounts: LiquidateAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.payer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn liquidate_verify_account_privileges<'me, 'info>(
    accounts: LiquidateAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    liquidate_verify_writable_privileges(accounts)?;
    liquidate_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const REMOVE_COLLATERAL_IX_ACCOUNTS_LEN: usize = 14;
#[derive(Copy, Clone, Debug)]
pub struct RemoveCollateralAccounts<'me, 'info> {
    pub pair: &'me AccountInfo<'info>,
    pub user_position: &'me AccountInfo<'info>,
    pub rate_model: &'me AccountInfo<'info>,
    pub futarchy_authority: &'me AccountInfo<'info>,
    pub collateral_vault: &'me AccountInfo<'info>,
    pub user_collateral_token_account: &'me AccountInfo<'info>,
    pub collateral_token_mint: &'me AccountInfo<'info>,
    pub user: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub token_2022_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub instructions_sysvar: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct RemoveCollateralKeys {
    pub pair: Pubkey,
    pub user_position: Pubkey,
    pub rate_model: Pubkey,
    pub futarchy_authority: Pubkey,
    pub collateral_vault: Pubkey,
    pub user_collateral_token_account: Pubkey,
    pub collateral_token_mint: Pubkey,
    pub user: Pubkey,
    pub token_program: Pubkey,
    pub token_2022_program: Pubkey,
    pub system_program: Pubkey,
    pub instructions_sysvar: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<RemoveCollateralAccounts<'_, '_>> for RemoveCollateralKeys {
    fn from(accounts: RemoveCollateralAccounts) -> Self {
        Self {
            pair: *accounts.pair.key,
            user_position: *accounts.user_position.key,
            rate_model: *accounts.rate_model.key,
            futarchy_authority: *accounts.futarchy_authority.key,
            collateral_vault: *accounts.collateral_vault.key,
            user_collateral_token_account: *accounts.user_collateral_token_account.key,
            collateral_token_mint: *accounts.collateral_token_mint.key,
            user: *accounts.user.key,
            token_program: *accounts.token_program.key,
            token_2022_program: *accounts.token_2022_program.key,
            system_program: *accounts.system_program.key,
            instructions_sysvar: *accounts.instructions_sysvar.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<RemoveCollateralKeys> for [AccountMeta; REMOVE_COLLATERAL_IX_ACCOUNTS_LEN] {
    fn from(keys: RemoveCollateralKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.pair,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_position,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.rate_model,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.futarchy_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.collateral_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_collateral_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.collateral_token_mint,
                is_signer: false,
                is_writable: false,
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
                pubkey: keys.token_2022_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.system_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.instructions_sysvar,
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
impl From<[Pubkey; REMOVE_COLLATERAL_IX_ACCOUNTS_LEN]> for RemoveCollateralKeys {
    fn from(pubkeys: [Pubkey; REMOVE_COLLATERAL_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            pair: pubkeys[0],
            user_position: pubkeys[1],
            rate_model: pubkeys[2],
            futarchy_authority: pubkeys[3],
            collateral_vault: pubkeys[4],
            user_collateral_token_account: pubkeys[5],
            collateral_token_mint: pubkeys[6],
            user: pubkeys[7],
            token_program: pubkeys[8],
            token_2022_program: pubkeys[9],
            system_program: pubkeys[10],
            instructions_sysvar: pubkeys[11],
            event_authority: pubkeys[12],
            program: pubkeys[13],
        }
    }
}
impl<'info> From<RemoveCollateralAccounts<'_, 'info>>
for [AccountInfo<'info>; REMOVE_COLLATERAL_IX_ACCOUNTS_LEN] {
    fn from(accounts: RemoveCollateralAccounts<'_, 'info>) -> Self {
        [
            accounts.pair.clone(),
            accounts.user_position.clone(),
            accounts.rate_model.clone(),
            accounts.futarchy_authority.clone(),
            accounts.collateral_vault.clone(),
            accounts.user_collateral_token_account.clone(),
            accounts.collateral_token_mint.clone(),
            accounts.user.clone(),
            accounts.token_program.clone(),
            accounts.token_2022_program.clone(),
            accounts.system_program.clone(),
            accounts.instructions_sysvar.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; REMOVE_COLLATERAL_IX_ACCOUNTS_LEN]>
for RemoveCollateralAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; REMOVE_COLLATERAL_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            pair: &arr[0],
            user_position: &arr[1],
            rate_model: &arr[2],
            futarchy_authority: &arr[3],
            collateral_vault: &arr[4],
            user_collateral_token_account: &arr[5],
            collateral_token_mint: &arr[6],
            user: &arr[7],
            token_program: &arr[8],
            token_2022_program: &arr[9],
            system_program: &arr[10],
            instructions_sysvar: &arr[11],
            event_authority: &arr[12],
            program: &arr[13],
        }
    }
}
pub const REMOVE_COLLATERAL_IX_DISCM: [u8; 8usize] = [86, 222, 130, 86, 92, 20, 72, 65];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct RemoveCollateralIxArgs {
    pub args: AdjustCollateralArgs,
}
#[derive(Clone, Debug, PartialEq)]
pub struct RemoveCollateralIxData(pub RemoveCollateralIxArgs);
impl From<RemoveCollateralIxArgs> for RemoveCollateralIxData {
    fn from(args: RemoveCollateralIxArgs) -> Self {
        Self(args)
    }
}
impl RemoveCollateralIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != REMOVE_COLLATERAL_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let args = if reader.is_empty() {
            Default::default()
        } else {
            <AdjustCollateralArgs>::deserialize(&mut reader)?
        };
        Ok(Self(RemoveCollateralIxArgs { args }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&REMOVE_COLLATERAL_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.args, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn remove_collateral_ix_with_program_id(
    program_id: Pubkey,
    keys: RemoveCollateralKeys,
    args: RemoveCollateralIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; REMOVE_COLLATERAL_IX_ACCOUNTS_LEN] = keys.into();
    let data: RemoveCollateralIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn remove_collateral_ix(
    keys: RemoveCollateralKeys,
    args: RemoveCollateralIxArgs,
) -> std::io::Result<Instruction> {
    remove_collateral_ix_with_program_id(OMNIPAIR_PROGRAM_ID, keys, args)
}
pub fn remove_collateral_invoke_with_program_id(
    program_id: Pubkey,
    accounts: RemoveCollateralAccounts<'_, '_>,
    args: RemoveCollateralIxArgs,
) -> ProgramResult {
    let keys: RemoveCollateralKeys = accounts.into();
    let ix = remove_collateral_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn remove_collateral_invoke(
    accounts: RemoveCollateralAccounts<'_, '_>,
    args: RemoveCollateralIxArgs,
) -> ProgramResult {
    remove_collateral_invoke_with_program_id(OMNIPAIR_PROGRAM_ID, accounts, args)
}
pub fn remove_collateral_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: RemoveCollateralAccounts<'_, '_>,
    args: RemoveCollateralIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: RemoveCollateralKeys = accounts.into();
    let ix = remove_collateral_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn remove_collateral_invoke_signed(
    accounts: RemoveCollateralAccounts<'_, '_>,
    args: RemoveCollateralIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    remove_collateral_invoke_signed_with_program_id(
        OMNIPAIR_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn remove_collateral_verify_account_keys(
    accounts: RemoveCollateralAccounts<'_, '_>,
    keys: RemoveCollateralKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.pair.key, keys.pair),
        (*accounts.user_position.key, keys.user_position),
        (*accounts.rate_model.key, keys.rate_model),
        (*accounts.futarchy_authority.key, keys.futarchy_authority),
        (*accounts.collateral_vault.key, keys.collateral_vault),
        (
            *accounts.user_collateral_token_account.key,
            keys.user_collateral_token_account,
        ),
        (*accounts.collateral_token_mint.key, keys.collateral_token_mint),
        (*accounts.user.key, keys.user),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.token_2022_program.key, keys.token_2022_program),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.instructions_sysvar.key, keys.instructions_sysvar),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn remove_collateral_verify_writable_privileges<'me, 'info>(
    accounts: RemoveCollateralAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.pair,
        accounts.user_position,
        accounts.rate_model,
        accounts.collateral_vault,
        accounts.user_collateral_token_account,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn remove_collateral_verify_signer_privileges<'me, 'info>(
    accounts: RemoveCollateralAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.user] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn remove_collateral_verify_account_privileges<'me, 'info>(
    accounts: RemoveCollateralAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    remove_collateral_verify_writable_privileges(accounts)?;
    remove_collateral_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const REMOVE_LIQUIDITY_IX_ACCOUNTS_LEN: usize = 19;
#[derive(Copy, Clone, Debug)]
pub struct RemoveLiquidityAccounts<'me, 'info> {
    pub pair: &'me AccountInfo<'info>,
    pub rate_model: &'me AccountInfo<'info>,
    pub futarchy_authority: &'me AccountInfo<'info>,
    pub reserve0_vault: &'me AccountInfo<'info>,
    pub reserve1_vault: &'me AccountInfo<'info>,
    pub user_token0_account: &'me AccountInfo<'info>,
    pub user_token1_account: &'me AccountInfo<'info>,
    pub token0_mint: &'me AccountInfo<'info>,
    pub token1_mint: &'me AccountInfo<'info>,
    pub lp_mint: &'me AccountInfo<'info>,
    pub user_lp_token_account: &'me AccountInfo<'info>,
    pub user: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub token_2022_program: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub instructions_sysvar: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct RemoveLiquidityKeys {
    pub pair: Pubkey,
    pub rate_model: Pubkey,
    pub futarchy_authority: Pubkey,
    pub reserve0_vault: Pubkey,
    pub reserve1_vault: Pubkey,
    pub user_token0_account: Pubkey,
    pub user_token1_account: Pubkey,
    pub token0_mint: Pubkey,
    pub token1_mint: Pubkey,
    pub lp_mint: Pubkey,
    pub user_lp_token_account: Pubkey,
    pub user: Pubkey,
    pub token_program: Pubkey,
    pub token_2022_program: Pubkey,
    pub associated_token_program: Pubkey,
    pub system_program: Pubkey,
    pub instructions_sysvar: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<RemoveLiquidityAccounts<'_, '_>> for RemoveLiquidityKeys {
    fn from(accounts: RemoveLiquidityAccounts) -> Self {
        Self {
            pair: *accounts.pair.key,
            rate_model: *accounts.rate_model.key,
            futarchy_authority: *accounts.futarchy_authority.key,
            reserve0_vault: *accounts.reserve0_vault.key,
            reserve1_vault: *accounts.reserve1_vault.key,
            user_token0_account: *accounts.user_token0_account.key,
            user_token1_account: *accounts.user_token1_account.key,
            token0_mint: *accounts.token0_mint.key,
            token1_mint: *accounts.token1_mint.key,
            lp_mint: *accounts.lp_mint.key,
            user_lp_token_account: *accounts.user_lp_token_account.key,
            user: *accounts.user.key,
            token_program: *accounts.token_program.key,
            token_2022_program: *accounts.token_2022_program.key,
            associated_token_program: *accounts.associated_token_program.key,
            system_program: *accounts.system_program.key,
            instructions_sysvar: *accounts.instructions_sysvar.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<RemoveLiquidityKeys> for [AccountMeta; REMOVE_LIQUIDITY_IX_ACCOUNTS_LEN] {
    fn from(keys: RemoveLiquidityKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.pair,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.rate_model,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.futarchy_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.reserve0_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.reserve1_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_token0_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_token1_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token0_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token1_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.lp_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_lp_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_2022_program,
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
                pubkey: keys.instructions_sysvar,
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
impl From<[Pubkey; REMOVE_LIQUIDITY_IX_ACCOUNTS_LEN]> for RemoveLiquidityKeys {
    fn from(pubkeys: [Pubkey; REMOVE_LIQUIDITY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            pair: pubkeys[0],
            rate_model: pubkeys[1],
            futarchy_authority: pubkeys[2],
            reserve0_vault: pubkeys[3],
            reserve1_vault: pubkeys[4],
            user_token0_account: pubkeys[5],
            user_token1_account: pubkeys[6],
            token0_mint: pubkeys[7],
            token1_mint: pubkeys[8],
            lp_mint: pubkeys[9],
            user_lp_token_account: pubkeys[10],
            user: pubkeys[11],
            token_program: pubkeys[12],
            token_2022_program: pubkeys[13],
            associated_token_program: pubkeys[14],
            system_program: pubkeys[15],
            instructions_sysvar: pubkeys[16],
            event_authority: pubkeys[17],
            program: pubkeys[18],
        }
    }
}
impl<'info> From<RemoveLiquidityAccounts<'_, 'info>>
for [AccountInfo<'info>; REMOVE_LIQUIDITY_IX_ACCOUNTS_LEN] {
    fn from(accounts: RemoveLiquidityAccounts<'_, 'info>) -> Self {
        [
            accounts.pair.clone(),
            accounts.rate_model.clone(),
            accounts.futarchy_authority.clone(),
            accounts.reserve0_vault.clone(),
            accounts.reserve1_vault.clone(),
            accounts.user_token0_account.clone(),
            accounts.user_token1_account.clone(),
            accounts.token0_mint.clone(),
            accounts.token1_mint.clone(),
            accounts.lp_mint.clone(),
            accounts.user_lp_token_account.clone(),
            accounts.user.clone(),
            accounts.token_program.clone(),
            accounts.token_2022_program.clone(),
            accounts.associated_token_program.clone(),
            accounts.system_program.clone(),
            accounts.instructions_sysvar.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; REMOVE_LIQUIDITY_IX_ACCOUNTS_LEN]>
for RemoveLiquidityAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; REMOVE_LIQUIDITY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            pair: &arr[0],
            rate_model: &arr[1],
            futarchy_authority: &arr[2],
            reserve0_vault: &arr[3],
            reserve1_vault: &arr[4],
            user_token0_account: &arr[5],
            user_token1_account: &arr[6],
            token0_mint: &arr[7],
            token1_mint: &arr[8],
            lp_mint: &arr[9],
            user_lp_token_account: &arr[10],
            user: &arr[11],
            token_program: &arr[12],
            token_2022_program: &arr[13],
            associated_token_program: &arr[14],
            system_program: &arr[15],
            instructions_sysvar: &arr[16],
            event_authority: &arr[17],
            program: &arr[18],
        }
    }
}
pub const REMOVE_LIQUIDITY_IX_DISCM: [u8; 8usize] = [80, 85, 209, 72, 24, 206, 177, 108];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct RemoveLiquidityIxArgs {
    pub args: RemoveLiquidityArgs,
}
#[derive(Clone, Debug, PartialEq)]
pub struct RemoveLiquidityIxData(pub RemoveLiquidityIxArgs);
impl From<RemoveLiquidityIxArgs> for RemoveLiquidityIxData {
    fn from(args: RemoveLiquidityIxArgs) -> Self {
        Self(args)
    }
}
impl RemoveLiquidityIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != REMOVE_LIQUIDITY_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let args = if reader.is_empty() {
            Default::default()
        } else {
            <RemoveLiquidityArgs>::deserialize(&mut reader)?
        };
        Ok(Self(RemoveLiquidityIxArgs { args }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&REMOVE_LIQUIDITY_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.args, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn remove_liquidity_ix_with_program_id(
    program_id: Pubkey,
    keys: RemoveLiquidityKeys,
    args: RemoveLiquidityIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; REMOVE_LIQUIDITY_IX_ACCOUNTS_LEN] = keys.into();
    let data: RemoveLiquidityIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn remove_liquidity_ix(
    keys: RemoveLiquidityKeys,
    args: RemoveLiquidityIxArgs,
) -> std::io::Result<Instruction> {
    remove_liquidity_ix_with_program_id(OMNIPAIR_PROGRAM_ID, keys, args)
}
pub fn remove_liquidity_invoke_with_program_id(
    program_id: Pubkey,
    accounts: RemoveLiquidityAccounts<'_, '_>,
    args: RemoveLiquidityIxArgs,
) -> ProgramResult {
    let keys: RemoveLiquidityKeys = accounts.into();
    let ix = remove_liquidity_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn remove_liquidity_invoke(
    accounts: RemoveLiquidityAccounts<'_, '_>,
    args: RemoveLiquidityIxArgs,
) -> ProgramResult {
    remove_liquidity_invoke_with_program_id(OMNIPAIR_PROGRAM_ID, accounts, args)
}
pub fn remove_liquidity_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: RemoveLiquidityAccounts<'_, '_>,
    args: RemoveLiquidityIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: RemoveLiquidityKeys = accounts.into();
    let ix = remove_liquidity_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn remove_liquidity_invoke_signed(
    accounts: RemoveLiquidityAccounts<'_, '_>,
    args: RemoveLiquidityIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    remove_liquidity_invoke_signed_with_program_id(
        OMNIPAIR_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn remove_liquidity_verify_account_keys(
    accounts: RemoveLiquidityAccounts<'_, '_>,
    keys: RemoveLiquidityKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.pair.key, keys.pair),
        (*accounts.rate_model.key, keys.rate_model),
        (*accounts.futarchy_authority.key, keys.futarchy_authority),
        (*accounts.reserve0_vault.key, keys.reserve0_vault),
        (*accounts.reserve1_vault.key, keys.reserve1_vault),
        (*accounts.user_token0_account.key, keys.user_token0_account),
        (*accounts.user_token1_account.key, keys.user_token1_account),
        (*accounts.token0_mint.key, keys.token0_mint),
        (*accounts.token1_mint.key, keys.token1_mint),
        (*accounts.lp_mint.key, keys.lp_mint),
        (*accounts.user_lp_token_account.key, keys.user_lp_token_account),
        (*accounts.user.key, keys.user),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.token_2022_program.key, keys.token_2022_program),
        (*accounts.associated_token_program.key, keys.associated_token_program),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.instructions_sysvar.key, keys.instructions_sysvar),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn remove_liquidity_verify_writable_privileges<'me, 'info>(
    accounts: RemoveLiquidityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.pair,
        accounts.rate_model,
        accounts.reserve0_vault,
        accounts.reserve1_vault,
        accounts.user_token0_account,
        accounts.user_token1_account,
        accounts.lp_mint,
        accounts.user_lp_token_account,
        accounts.user,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn remove_liquidity_verify_signer_privileges<'me, 'info>(
    accounts: RemoveLiquidityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.user] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn remove_liquidity_verify_account_privileges<'me, 'info>(
    accounts: RemoveLiquidityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    remove_liquidity_verify_writable_privileges(accounts)?;
    remove_liquidity_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const REPAY_IX_ACCOUNTS_LEN: usize = 13;
#[derive(Copy, Clone, Debug)]
pub struct RepayAccounts<'me, 'info> {
    pub pair: &'me AccountInfo<'info>,
    pub user_position: &'me AccountInfo<'info>,
    pub rate_model: &'me AccountInfo<'info>,
    pub futarchy_authority: &'me AccountInfo<'info>,
    pub reserve_vault: &'me AccountInfo<'info>,
    pub user_reserve_token_account: &'me AccountInfo<'info>,
    pub reserve_token_mint: &'me AccountInfo<'info>,
    pub user: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub token_2022_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct RepayKeys {
    pub pair: Pubkey,
    pub user_position: Pubkey,
    pub rate_model: Pubkey,
    pub futarchy_authority: Pubkey,
    pub reserve_vault: Pubkey,
    pub user_reserve_token_account: Pubkey,
    pub reserve_token_mint: Pubkey,
    pub user: Pubkey,
    pub token_program: Pubkey,
    pub token_2022_program: Pubkey,
    pub system_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<RepayAccounts<'_, '_>> for RepayKeys {
    fn from(accounts: RepayAccounts) -> Self {
        Self {
            pair: *accounts.pair.key,
            user_position: *accounts.user_position.key,
            rate_model: *accounts.rate_model.key,
            futarchy_authority: *accounts.futarchy_authority.key,
            reserve_vault: *accounts.reserve_vault.key,
            user_reserve_token_account: *accounts.user_reserve_token_account.key,
            reserve_token_mint: *accounts.reserve_token_mint.key,
            user: *accounts.user.key,
            token_program: *accounts.token_program.key,
            token_2022_program: *accounts.token_2022_program.key,
            system_program: *accounts.system_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<RepayKeys> for [AccountMeta; REPAY_IX_ACCOUNTS_LEN] {
    fn from(keys: RepayKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.pair,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_position,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.rate_model,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.futarchy_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.reserve_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_reserve_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.reserve_token_mint,
                is_signer: false,
                is_writable: false,
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
                pubkey: keys.token_2022_program,
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
impl From<[Pubkey; REPAY_IX_ACCOUNTS_LEN]> for RepayKeys {
    fn from(pubkeys: [Pubkey; REPAY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            pair: pubkeys[0],
            user_position: pubkeys[1],
            rate_model: pubkeys[2],
            futarchy_authority: pubkeys[3],
            reserve_vault: pubkeys[4],
            user_reserve_token_account: pubkeys[5],
            reserve_token_mint: pubkeys[6],
            user: pubkeys[7],
            token_program: pubkeys[8],
            token_2022_program: pubkeys[9],
            system_program: pubkeys[10],
            event_authority: pubkeys[11],
            program: pubkeys[12],
        }
    }
}
impl<'info> From<RepayAccounts<'_, 'info>>
for [AccountInfo<'info>; REPAY_IX_ACCOUNTS_LEN] {
    fn from(accounts: RepayAccounts<'_, 'info>) -> Self {
        [
            accounts.pair.clone(),
            accounts.user_position.clone(),
            accounts.rate_model.clone(),
            accounts.futarchy_authority.clone(),
            accounts.reserve_vault.clone(),
            accounts.user_reserve_token_account.clone(),
            accounts.reserve_token_mint.clone(),
            accounts.user.clone(),
            accounts.token_program.clone(),
            accounts.token_2022_program.clone(),
            accounts.system_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; REPAY_IX_ACCOUNTS_LEN]>
for RepayAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; REPAY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            pair: &arr[0],
            user_position: &arr[1],
            rate_model: &arr[2],
            futarchy_authority: &arr[3],
            reserve_vault: &arr[4],
            user_reserve_token_account: &arr[5],
            reserve_token_mint: &arr[6],
            user: &arr[7],
            token_program: &arr[8],
            token_2022_program: &arr[9],
            system_program: &arr[10],
            event_authority: &arr[11],
            program: &arr[12],
        }
    }
}
pub const REPAY_IX_DISCM: [u8; 8usize] = [234, 103, 67, 82, 208, 234, 219, 166];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct RepayIxArgs {
    pub args: AdjustDebtArgs,
}
#[derive(Clone, Debug, PartialEq)]
pub struct RepayIxData(pub RepayIxArgs);
impl From<RepayIxArgs> for RepayIxData {
    fn from(args: RepayIxArgs) -> Self {
        Self(args)
    }
}
impl RepayIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != REPAY_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let args = if reader.is_empty() {
            Default::default()
        } else {
            <AdjustDebtArgs>::deserialize(&mut reader)?
        };
        Ok(Self(RepayIxArgs { args }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&REPAY_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.args, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn repay_ix_with_program_id(
    program_id: Pubkey,
    keys: RepayKeys,
    args: RepayIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; REPAY_IX_ACCOUNTS_LEN] = keys.into();
    let data: RepayIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn repay_ix(keys: RepayKeys, args: RepayIxArgs) -> std::io::Result<Instruction> {
    repay_ix_with_program_id(OMNIPAIR_PROGRAM_ID, keys, args)
}
pub fn repay_invoke_with_program_id(
    program_id: Pubkey,
    accounts: RepayAccounts<'_, '_>,
    args: RepayIxArgs,
) -> ProgramResult {
    let keys: RepayKeys = accounts.into();
    let ix = repay_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn repay_invoke(
    accounts: RepayAccounts<'_, '_>,
    args: RepayIxArgs,
) -> ProgramResult {
    repay_invoke_with_program_id(OMNIPAIR_PROGRAM_ID, accounts, args)
}
pub fn repay_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: RepayAccounts<'_, '_>,
    args: RepayIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: RepayKeys = accounts.into();
    let ix = repay_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn repay_invoke_signed(
    accounts: RepayAccounts<'_, '_>,
    args: RepayIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    repay_invoke_signed_with_program_id(OMNIPAIR_PROGRAM_ID, accounts, args, seeds)
}
pub fn repay_verify_account_keys(
    accounts: RepayAccounts<'_, '_>,
    keys: RepayKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.pair.key, keys.pair),
        (*accounts.user_position.key, keys.user_position),
        (*accounts.rate_model.key, keys.rate_model),
        (*accounts.futarchy_authority.key, keys.futarchy_authority),
        (*accounts.reserve_vault.key, keys.reserve_vault),
        (*accounts.user_reserve_token_account.key, keys.user_reserve_token_account),
        (*accounts.reserve_token_mint.key, keys.reserve_token_mint),
        (*accounts.user.key, keys.user),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.token_2022_program.key, keys.token_2022_program),
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
pub fn repay_verify_writable_privileges<'me, 'info>(
    accounts: RepayAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.pair,
        accounts.user_position,
        accounts.rate_model,
        accounts.reserve_vault,
        accounts.user_reserve_token_account,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn repay_verify_signer_privileges<'me, 'info>(
    accounts: RepayAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.user] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn repay_verify_account_privileges<'me, 'info>(
    accounts: RepayAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    repay_verify_writable_privileges(accounts)?;
    repay_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SET_GLOBAL_REDUCE_ONLY_IX_ACCOUNTS_LEN: usize = 2;
#[derive(Copy, Clone, Debug)]
pub struct SetGlobalReduceOnlyAccounts<'me, 'info> {
    pub authority_signer: &'me AccountInfo<'info>,
    pub futarchy_authority: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SetGlobalReduceOnlyKeys {
    pub authority_signer: Pubkey,
    pub futarchy_authority: Pubkey,
}
impl From<SetGlobalReduceOnlyAccounts<'_, '_>> for SetGlobalReduceOnlyKeys {
    fn from(accounts: SetGlobalReduceOnlyAccounts) -> Self {
        Self {
            authority_signer: *accounts.authority_signer.key,
            futarchy_authority: *accounts.futarchy_authority.key,
        }
    }
}
impl From<SetGlobalReduceOnlyKeys>
for [AccountMeta; SET_GLOBAL_REDUCE_ONLY_IX_ACCOUNTS_LEN] {
    fn from(keys: SetGlobalReduceOnlyKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.authority_signer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.futarchy_authority,
                is_signer: false,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; SET_GLOBAL_REDUCE_ONLY_IX_ACCOUNTS_LEN]> for SetGlobalReduceOnlyKeys {
    fn from(pubkeys: [Pubkey; SET_GLOBAL_REDUCE_ONLY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            authority_signer: pubkeys[0],
            futarchy_authority: pubkeys[1],
        }
    }
}
impl<'info> From<SetGlobalReduceOnlyAccounts<'_, 'info>>
for [AccountInfo<'info>; SET_GLOBAL_REDUCE_ONLY_IX_ACCOUNTS_LEN] {
    fn from(accounts: SetGlobalReduceOnlyAccounts<'_, 'info>) -> Self {
        [accounts.authority_signer.clone(), accounts.futarchy_authority.clone()]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SET_GLOBAL_REDUCE_ONLY_IX_ACCOUNTS_LEN]>
for SetGlobalReduceOnlyAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; SET_GLOBAL_REDUCE_ONLY_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            authority_signer: &arr[0],
            futarchy_authority: &arr[1],
        }
    }
}
pub const SET_GLOBAL_REDUCE_ONLY_IX_DISCM: [u8; 8usize] = [
    242, 151, 123, 139, 239, 87, 249, 98,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SetGlobalReduceOnlyIxArgs {
    pub args: SetGlobalReduceOnlyArgs,
}
#[derive(Clone, Debug, PartialEq)]
pub struct SetGlobalReduceOnlyIxData(pub SetGlobalReduceOnlyIxArgs);
impl From<SetGlobalReduceOnlyIxArgs> for SetGlobalReduceOnlyIxData {
    fn from(args: SetGlobalReduceOnlyIxArgs) -> Self {
        Self(args)
    }
}
impl SetGlobalReduceOnlyIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SET_GLOBAL_REDUCE_ONLY_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let args = if reader.is_empty() {
            Default::default()
        } else {
            <SetGlobalReduceOnlyArgs>::deserialize(&mut reader)?
        };
        Ok(Self(SetGlobalReduceOnlyIxArgs { args }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SET_GLOBAL_REDUCE_ONLY_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.args, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn set_global_reduce_only_ix_with_program_id(
    program_id: Pubkey,
    keys: SetGlobalReduceOnlyKeys,
    args: SetGlobalReduceOnlyIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SET_GLOBAL_REDUCE_ONLY_IX_ACCOUNTS_LEN] = keys.into();
    let data: SetGlobalReduceOnlyIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn set_global_reduce_only_ix(
    keys: SetGlobalReduceOnlyKeys,
    args: SetGlobalReduceOnlyIxArgs,
) -> std::io::Result<Instruction> {
    set_global_reduce_only_ix_with_program_id(OMNIPAIR_PROGRAM_ID, keys, args)
}
pub fn set_global_reduce_only_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SetGlobalReduceOnlyAccounts<'_, '_>,
    args: SetGlobalReduceOnlyIxArgs,
) -> ProgramResult {
    let keys: SetGlobalReduceOnlyKeys = accounts.into();
    let ix = set_global_reduce_only_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn set_global_reduce_only_invoke(
    accounts: SetGlobalReduceOnlyAccounts<'_, '_>,
    args: SetGlobalReduceOnlyIxArgs,
) -> ProgramResult {
    set_global_reduce_only_invoke_with_program_id(OMNIPAIR_PROGRAM_ID, accounts, args)
}
pub fn set_global_reduce_only_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SetGlobalReduceOnlyAccounts<'_, '_>,
    args: SetGlobalReduceOnlyIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SetGlobalReduceOnlyKeys = accounts.into();
    let ix = set_global_reduce_only_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn set_global_reduce_only_invoke_signed(
    accounts: SetGlobalReduceOnlyAccounts<'_, '_>,
    args: SetGlobalReduceOnlyIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    set_global_reduce_only_invoke_signed_with_program_id(
        OMNIPAIR_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn set_global_reduce_only_verify_account_keys(
    accounts: SetGlobalReduceOnlyAccounts<'_, '_>,
    keys: SetGlobalReduceOnlyKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.authority_signer.key, keys.authority_signer),
        (*accounts.futarchy_authority.key, keys.futarchy_authority),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn set_global_reduce_only_verify_writable_privileges<'me, 'info>(
    accounts: SetGlobalReduceOnlyAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.authority_signer, accounts.futarchy_authority] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn set_global_reduce_only_verify_signer_privileges<'me, 'info>(
    accounts: SetGlobalReduceOnlyAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority_signer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn set_global_reduce_only_verify_account_privileges<'me, 'info>(
    accounts: SetGlobalReduceOnlyAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    set_global_reduce_only_verify_writable_privileges(accounts)?;
    set_global_reduce_only_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SET_PAIR_RATE_MODEL_IX_ACCOUNTS_LEN: usize = 5;
#[derive(Copy, Clone, Debug)]
pub struct SetPairRateModelAccounts<'me, 'info> {
    pub authority_signer: &'me AccountInfo<'info>,
    pub futarchy_authority: &'me AccountInfo<'info>,
    pub pair: &'me AccountInfo<'info>,
    pub new_rate_model: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SetPairRateModelKeys {
    pub authority_signer: Pubkey,
    pub futarchy_authority: Pubkey,
    pub pair: Pubkey,
    pub new_rate_model: Pubkey,
    pub system_program: Pubkey,
}
impl From<SetPairRateModelAccounts<'_, '_>> for SetPairRateModelKeys {
    fn from(accounts: SetPairRateModelAccounts) -> Self {
        Self {
            authority_signer: *accounts.authority_signer.key,
            futarchy_authority: *accounts.futarchy_authority.key,
            pair: *accounts.pair.key,
            new_rate_model: *accounts.new_rate_model.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<SetPairRateModelKeys> for [AccountMeta; SET_PAIR_RATE_MODEL_IX_ACCOUNTS_LEN] {
    fn from(keys: SetPairRateModelKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.authority_signer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.futarchy_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pair,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.new_rate_model,
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
impl From<[Pubkey; SET_PAIR_RATE_MODEL_IX_ACCOUNTS_LEN]> for SetPairRateModelKeys {
    fn from(pubkeys: [Pubkey; SET_PAIR_RATE_MODEL_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            authority_signer: pubkeys[0],
            futarchy_authority: pubkeys[1],
            pair: pubkeys[2],
            new_rate_model: pubkeys[3],
            system_program: pubkeys[4],
        }
    }
}
impl<'info> From<SetPairRateModelAccounts<'_, 'info>>
for [AccountInfo<'info>; SET_PAIR_RATE_MODEL_IX_ACCOUNTS_LEN] {
    fn from(accounts: SetPairRateModelAccounts<'_, 'info>) -> Self {
        [
            accounts.authority_signer.clone(),
            accounts.futarchy_authority.clone(),
            accounts.pair.clone(),
            accounts.new_rate_model.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SET_PAIR_RATE_MODEL_IX_ACCOUNTS_LEN]>
for SetPairRateModelAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; SET_PAIR_RATE_MODEL_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            authority_signer: &arr[0],
            futarchy_authority: &arr[1],
            pair: &arr[2],
            new_rate_model: &arr[3],
            system_program: &arr[4],
        }
    }
}
pub const SET_PAIR_RATE_MODEL_IX_DISCM: [u8; 8usize] = [56, 70, 171, 88, 3, 24, 54, 97];
#[derive(Clone, Debug, PartialEq)]
pub struct SetPairRateModelIxData;
impl SetPairRateModelIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SET_PAIR_RATE_MODEL_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SET_PAIR_RATE_MODEL_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn set_pair_rate_model_ix_with_program_id(
    program_id: Pubkey,
    keys: SetPairRateModelKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SET_PAIR_RATE_MODEL_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: SetPairRateModelIxData.try_to_vec()?,
    })
}
pub fn set_pair_rate_model_ix(
    keys: SetPairRateModelKeys,
) -> std::io::Result<Instruction> {
    set_pair_rate_model_ix_with_program_id(OMNIPAIR_PROGRAM_ID, keys)
}
pub fn set_pair_rate_model_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SetPairRateModelAccounts<'_, '_>,
) -> ProgramResult {
    let keys: SetPairRateModelKeys = accounts.into();
    let ix = set_pair_rate_model_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn set_pair_rate_model_invoke(
    accounts: SetPairRateModelAccounts<'_, '_>,
) -> ProgramResult {
    set_pair_rate_model_invoke_with_program_id(OMNIPAIR_PROGRAM_ID, accounts)
}
pub fn set_pair_rate_model_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SetPairRateModelAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SetPairRateModelKeys = accounts.into();
    let ix = set_pair_rate_model_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn set_pair_rate_model_invoke_signed(
    accounts: SetPairRateModelAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    set_pair_rate_model_invoke_signed_with_program_id(
        OMNIPAIR_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn set_pair_rate_model_verify_account_keys(
    accounts: SetPairRateModelAccounts<'_, '_>,
    keys: SetPairRateModelKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.authority_signer.key, keys.authority_signer),
        (*accounts.futarchy_authority.key, keys.futarchy_authority),
        (*accounts.pair.key, keys.pair),
        (*accounts.new_rate_model.key, keys.new_rate_model),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn set_pair_rate_model_verify_writable_privileges<'me, 'info>(
    accounts: SetPairRateModelAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.authority_signer, accounts.pair] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn set_pair_rate_model_verify_signer_privileges<'me, 'info>(
    accounts: SetPairRateModelAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority_signer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn set_pair_rate_model_verify_account_privileges<'me, 'info>(
    accounts: SetPairRateModelAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    set_pair_rate_model_verify_writable_privileges(accounts)?;
    set_pair_rate_model_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SET_PAIR_REDUCE_ONLY_IX_ACCOUNTS_LEN: usize = 2;
#[derive(Copy, Clone, Debug)]
pub struct SetPairReduceOnlyAccounts<'me, 'info> {
    pub authority_signer: &'me AccountInfo<'info>,
    pub pair: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SetPairReduceOnlyKeys {
    pub authority_signer: Pubkey,
    pub pair: Pubkey,
}
impl From<SetPairReduceOnlyAccounts<'_, '_>> for SetPairReduceOnlyKeys {
    fn from(accounts: SetPairReduceOnlyAccounts) -> Self {
        Self {
            authority_signer: *accounts.authority_signer.key,
            pair: *accounts.pair.key,
        }
    }
}
impl From<SetPairReduceOnlyKeys>
for [AccountMeta; SET_PAIR_REDUCE_ONLY_IX_ACCOUNTS_LEN] {
    fn from(keys: SetPairReduceOnlyKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.authority_signer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pair,
                is_signer: false,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; SET_PAIR_REDUCE_ONLY_IX_ACCOUNTS_LEN]> for SetPairReduceOnlyKeys {
    fn from(pubkeys: [Pubkey; SET_PAIR_REDUCE_ONLY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            authority_signer: pubkeys[0],
            pair: pubkeys[1],
        }
    }
}
impl<'info> From<SetPairReduceOnlyAccounts<'_, 'info>>
for [AccountInfo<'info>; SET_PAIR_REDUCE_ONLY_IX_ACCOUNTS_LEN] {
    fn from(accounts: SetPairReduceOnlyAccounts<'_, 'info>) -> Self {
        [accounts.authority_signer.clone(), accounts.pair.clone()]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SET_PAIR_REDUCE_ONLY_IX_ACCOUNTS_LEN]>
for SetPairReduceOnlyAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; SET_PAIR_REDUCE_ONLY_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            authority_signer: &arr[0],
            pair: &arr[1],
        }
    }
}
pub const SET_PAIR_REDUCE_ONLY_IX_DISCM: [u8; 8usize] = [
    147, 113, 16, 50, 64, 88, 175, 18,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SetPairReduceOnlyIxArgs {
    pub args: SetPairReduceOnlyArgs,
}
#[derive(Clone, Debug, PartialEq)]
pub struct SetPairReduceOnlyIxData(pub SetPairReduceOnlyIxArgs);
impl From<SetPairReduceOnlyIxArgs> for SetPairReduceOnlyIxData {
    fn from(args: SetPairReduceOnlyIxArgs) -> Self {
        Self(args)
    }
}
impl SetPairReduceOnlyIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SET_PAIR_REDUCE_ONLY_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let args = if reader.is_empty() {
            Default::default()
        } else {
            <SetPairReduceOnlyArgs>::deserialize(&mut reader)?
        };
        Ok(Self(SetPairReduceOnlyIxArgs { args }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SET_PAIR_REDUCE_ONLY_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.args, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn set_pair_reduce_only_ix_with_program_id(
    program_id: Pubkey,
    keys: SetPairReduceOnlyKeys,
    args: SetPairReduceOnlyIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SET_PAIR_REDUCE_ONLY_IX_ACCOUNTS_LEN] = keys.into();
    let data: SetPairReduceOnlyIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn set_pair_reduce_only_ix(
    keys: SetPairReduceOnlyKeys,
    args: SetPairReduceOnlyIxArgs,
) -> std::io::Result<Instruction> {
    set_pair_reduce_only_ix_with_program_id(OMNIPAIR_PROGRAM_ID, keys, args)
}
pub fn set_pair_reduce_only_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SetPairReduceOnlyAccounts<'_, '_>,
    args: SetPairReduceOnlyIxArgs,
) -> ProgramResult {
    let keys: SetPairReduceOnlyKeys = accounts.into();
    let ix = set_pair_reduce_only_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn set_pair_reduce_only_invoke(
    accounts: SetPairReduceOnlyAccounts<'_, '_>,
    args: SetPairReduceOnlyIxArgs,
) -> ProgramResult {
    set_pair_reduce_only_invoke_with_program_id(OMNIPAIR_PROGRAM_ID, accounts, args)
}
pub fn set_pair_reduce_only_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SetPairReduceOnlyAccounts<'_, '_>,
    args: SetPairReduceOnlyIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SetPairReduceOnlyKeys = accounts.into();
    let ix = set_pair_reduce_only_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn set_pair_reduce_only_invoke_signed(
    accounts: SetPairReduceOnlyAccounts<'_, '_>,
    args: SetPairReduceOnlyIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    set_pair_reduce_only_invoke_signed_with_program_id(
        OMNIPAIR_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn set_pair_reduce_only_verify_account_keys(
    accounts: SetPairReduceOnlyAccounts<'_, '_>,
    keys: SetPairReduceOnlyKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.authority_signer.key, keys.authority_signer),
        (*accounts.pair.key, keys.pair),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn set_pair_reduce_only_verify_writable_privileges<'me, 'info>(
    accounts: SetPairReduceOnlyAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.authority_signer, accounts.pair] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn set_pair_reduce_only_verify_signer_privileges<'me, 'info>(
    accounts: SetPairReduceOnlyAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority_signer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn set_pair_reduce_only_verify_account_privileges<'me, 'info>(
    accounts: SetPairReduceOnlyAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    set_pair_reduce_only_verify_writable_privileges(accounts)?;
    set_pair_reduce_only_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SWAP_IX_ACCOUNTS_LEN: usize = 14;
#[derive(Copy, Clone, Debug)]
pub struct SwapAccounts<'me, 'info> {
    pub pair: &'me AccountInfo<'info>,
    pub rate_model: &'me AccountInfo<'info>,
    pub futarchy_authority: &'me AccountInfo<'info>,
    pub token_in_vault: &'me AccountInfo<'info>,
    pub token_out_vault: &'me AccountInfo<'info>,
    pub user_token_in_account: &'me AccountInfo<'info>,
    pub user_token_out_account: &'me AccountInfo<'info>,
    pub token_in_mint: &'me AccountInfo<'info>,
    pub token_out_mint: &'me AccountInfo<'info>,
    pub user: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub token_2022_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SwapKeys {
    pub pair: Pubkey,
    pub rate_model: Pubkey,
    pub futarchy_authority: Pubkey,
    pub token_in_vault: Pubkey,
    pub token_out_vault: Pubkey,
    pub user_token_in_account: Pubkey,
    pub user_token_out_account: Pubkey,
    pub token_in_mint: Pubkey,
    pub token_out_mint: Pubkey,
    pub user: Pubkey,
    pub token_program: Pubkey,
    pub token_2022_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<SwapAccounts<'_, '_>> for SwapKeys {
    fn from(accounts: SwapAccounts) -> Self {
        Self {
            pair: *accounts.pair.key,
            rate_model: *accounts.rate_model.key,
            futarchy_authority: *accounts.futarchy_authority.key,
            token_in_vault: *accounts.token_in_vault.key,
            token_out_vault: *accounts.token_out_vault.key,
            user_token_in_account: *accounts.user_token_in_account.key,
            user_token_out_account: *accounts.user_token_out_account.key,
            token_in_mint: *accounts.token_in_mint.key,
            token_out_mint: *accounts.token_out_mint.key,
            user: *accounts.user.key,
            token_program: *accounts.token_program.key,
            token_2022_program: *accounts.token_2022_program.key,
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
                pubkey: keys.rate_model,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.futarchy_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_in_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_out_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_token_in_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_token_out_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_in_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_out_mint,
                is_signer: false,
                is_writable: false,
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
                pubkey: keys.token_2022_program,
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
            rate_model: pubkeys[1],
            futarchy_authority: pubkeys[2],
            token_in_vault: pubkeys[3],
            token_out_vault: pubkeys[4],
            user_token_in_account: pubkeys[5],
            user_token_out_account: pubkeys[6],
            token_in_mint: pubkeys[7],
            token_out_mint: pubkeys[8],
            user: pubkeys[9],
            token_program: pubkeys[10],
            token_2022_program: pubkeys[11],
            event_authority: pubkeys[12],
            program: pubkeys[13],
        }
    }
}
impl<'info> From<SwapAccounts<'_, 'info>>
for [AccountInfo<'info>; SWAP_IX_ACCOUNTS_LEN] {
    fn from(accounts: SwapAccounts<'_, 'info>) -> Self {
        [
            accounts.pair.clone(),
            accounts.rate_model.clone(),
            accounts.futarchy_authority.clone(),
            accounts.token_in_vault.clone(),
            accounts.token_out_vault.clone(),
            accounts.user_token_in_account.clone(),
            accounts.user_token_out_account.clone(),
            accounts.token_in_mint.clone(),
            accounts.token_out_mint.clone(),
            accounts.user.clone(),
            accounts.token_program.clone(),
            accounts.token_2022_program.clone(),
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
            rate_model: &arr[1],
            futarchy_authority: &arr[2],
            token_in_vault: &arr[3],
            token_out_vault: &arr[4],
            user_token_in_account: &arr[5],
            user_token_out_account: &arr[6],
            token_in_mint: &arr[7],
            token_out_mint: &arr[8],
            user: &arr[9],
            token_program: &arr[10],
            token_2022_program: &arr[11],
            event_authority: &arr[12],
            program: &arr[13],
        }
    }
}
pub const SWAP_IX_DISCM: [u8; 8usize] = [248, 198, 158, 145, 225, 117, 135, 200];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SwapIxArgs {
    pub args: SwapArgs,
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
        let args = if reader.is_empty() {
            Default::default()
        } else {
            <SwapArgs>::deserialize(&mut reader)?
        };
        Ok(Self(SwapIxArgs { args }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SWAP_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.args, &mut writer)?;
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
    swap_ix_with_program_id(OMNIPAIR_PROGRAM_ID, keys, args)
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
    swap_invoke_with_program_id(OMNIPAIR_PROGRAM_ID, accounts, args)
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
    swap_invoke_signed_with_program_id(OMNIPAIR_PROGRAM_ID, accounts, args, seeds)
}
pub fn swap_verify_account_keys(
    accounts: SwapAccounts<'_, '_>,
    keys: SwapKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.pair.key, keys.pair),
        (*accounts.rate_model.key, keys.rate_model),
        (*accounts.futarchy_authority.key, keys.futarchy_authority),
        (*accounts.token_in_vault.key, keys.token_in_vault),
        (*accounts.token_out_vault.key, keys.token_out_vault),
        (*accounts.user_token_in_account.key, keys.user_token_in_account),
        (*accounts.user_token_out_account.key, keys.user_token_out_account),
        (*accounts.token_in_mint.key, keys.token_in_mint),
        (*accounts.token_out_mint.key, keys.token_out_mint),
        (*accounts.user.key, keys.user),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.token_2022_program.key, keys.token_2022_program),
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
        accounts.rate_model,
        accounts.token_in_vault,
        accounts.token_out_vault,
        accounts.user_token_in_account,
        accounts.user_token_out_account,
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
pub const UPDATE_FUTARCHY_AUTHORITY_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct UpdateFutarchyAuthorityAccounts<'me, 'info> {
    pub authority_signer: &'me AccountInfo<'info>,
    pub futarchy_authority: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UpdateFutarchyAuthorityKeys {
    pub authority_signer: Pubkey,
    pub futarchy_authority: Pubkey,
    pub system_program: Pubkey,
}
impl From<UpdateFutarchyAuthorityAccounts<'_, '_>> for UpdateFutarchyAuthorityKeys {
    fn from(accounts: UpdateFutarchyAuthorityAccounts) -> Self {
        Self {
            authority_signer: *accounts.authority_signer.key,
            futarchy_authority: *accounts.futarchy_authority.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<UpdateFutarchyAuthorityKeys>
for [AccountMeta; UPDATE_FUTARCHY_AUTHORITY_IX_ACCOUNTS_LEN] {
    fn from(keys: UpdateFutarchyAuthorityKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.authority_signer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.futarchy_authority,
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
impl From<[Pubkey; UPDATE_FUTARCHY_AUTHORITY_IX_ACCOUNTS_LEN]>
for UpdateFutarchyAuthorityKeys {
    fn from(pubkeys: [Pubkey; UPDATE_FUTARCHY_AUTHORITY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            authority_signer: pubkeys[0],
            futarchy_authority: pubkeys[1],
            system_program: pubkeys[2],
        }
    }
}
impl<'info> From<UpdateFutarchyAuthorityAccounts<'_, 'info>>
for [AccountInfo<'info>; UPDATE_FUTARCHY_AUTHORITY_IX_ACCOUNTS_LEN] {
    fn from(accounts: UpdateFutarchyAuthorityAccounts<'_, 'info>) -> Self {
        [
            accounts.authority_signer.clone(),
            accounts.futarchy_authority.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; UPDATE_FUTARCHY_AUTHORITY_IX_ACCOUNTS_LEN]>
for UpdateFutarchyAuthorityAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; UPDATE_FUTARCHY_AUTHORITY_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            authority_signer: &arr[0],
            futarchy_authority: &arr[1],
            system_program: &arr[2],
        }
    }
}
pub const UPDATE_FUTARCHY_AUTHORITY_IX_DISCM: [u8; 8usize] = [
    15, 196, 157, 217, 113, 226, 89, 25,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UpdateFutarchyAuthorityIxArgs {
    pub args: UpdateFutarchyAuthorityArgs,
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateFutarchyAuthorityIxData(pub UpdateFutarchyAuthorityIxArgs);
impl From<UpdateFutarchyAuthorityIxArgs> for UpdateFutarchyAuthorityIxData {
    fn from(args: UpdateFutarchyAuthorityIxArgs) -> Self {
        Self(args)
    }
}
impl UpdateFutarchyAuthorityIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_FUTARCHY_AUTHORITY_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let args = if reader.is_empty() {
            Default::default()
        } else {
            <UpdateFutarchyAuthorityArgs>::deserialize(&mut reader)?
        };
        Ok(
            Self(UpdateFutarchyAuthorityIxArgs {
                args,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_FUTARCHY_AUTHORITY_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.args, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn update_futarchy_authority_ix_with_program_id(
    program_id: Pubkey,
    keys: UpdateFutarchyAuthorityKeys,
    args: UpdateFutarchyAuthorityIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; UPDATE_FUTARCHY_AUTHORITY_IX_ACCOUNTS_LEN] = keys.into();
    let data: UpdateFutarchyAuthorityIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_futarchy_authority_ix(
    keys: UpdateFutarchyAuthorityKeys,
    args: UpdateFutarchyAuthorityIxArgs,
) -> std::io::Result<Instruction> {
    update_futarchy_authority_ix_with_program_id(OMNIPAIR_PROGRAM_ID, keys, args)
}
pub fn update_futarchy_authority_invoke_with_program_id(
    program_id: Pubkey,
    accounts: UpdateFutarchyAuthorityAccounts<'_, '_>,
    args: UpdateFutarchyAuthorityIxArgs,
) -> ProgramResult {
    let keys: UpdateFutarchyAuthorityKeys = accounts.into();
    let ix = update_futarchy_authority_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn update_futarchy_authority_invoke(
    accounts: UpdateFutarchyAuthorityAccounts<'_, '_>,
    args: UpdateFutarchyAuthorityIxArgs,
) -> ProgramResult {
    update_futarchy_authority_invoke_with_program_id(OMNIPAIR_PROGRAM_ID, accounts, args)
}
pub fn update_futarchy_authority_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: UpdateFutarchyAuthorityAccounts<'_, '_>,
    args: UpdateFutarchyAuthorityIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: UpdateFutarchyAuthorityKeys = accounts.into();
    let ix = update_futarchy_authority_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn update_futarchy_authority_invoke_signed(
    accounts: UpdateFutarchyAuthorityAccounts<'_, '_>,
    args: UpdateFutarchyAuthorityIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    update_futarchy_authority_invoke_signed_with_program_id(
        OMNIPAIR_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn update_futarchy_authority_verify_account_keys(
    accounts: UpdateFutarchyAuthorityAccounts<'_, '_>,
    keys: UpdateFutarchyAuthorityKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.authority_signer.key, keys.authority_signer),
        (*accounts.futarchy_authority.key, keys.futarchy_authority),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn update_futarchy_authority_verify_writable_privileges<'me, 'info>(
    accounts: UpdateFutarchyAuthorityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.authority_signer, accounts.futarchy_authority] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn update_futarchy_authority_verify_signer_privileges<'me, 'info>(
    accounts: UpdateFutarchyAuthorityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority_signer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn update_futarchy_authority_verify_account_privileges<'me, 'info>(
    accounts: UpdateFutarchyAuthorityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    update_futarchy_authority_verify_writable_privileges(accounts)?;
    update_futarchy_authority_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const UPDATE_PROTOCOL_REVENUE_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct UpdateProtocolRevenueAccounts<'me, 'info> {
    pub authority_signer: &'me AccountInfo<'info>,
    pub futarchy_authority: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UpdateProtocolRevenueKeys {
    pub authority_signer: Pubkey,
    pub futarchy_authority: Pubkey,
    pub system_program: Pubkey,
}
impl From<UpdateProtocolRevenueAccounts<'_, '_>> for UpdateProtocolRevenueKeys {
    fn from(accounts: UpdateProtocolRevenueAccounts) -> Self {
        Self {
            authority_signer: *accounts.authority_signer.key,
            futarchy_authority: *accounts.futarchy_authority.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<UpdateProtocolRevenueKeys>
for [AccountMeta; UPDATE_PROTOCOL_REVENUE_IX_ACCOUNTS_LEN] {
    fn from(keys: UpdateProtocolRevenueKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.authority_signer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.futarchy_authority,
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
impl From<[Pubkey; UPDATE_PROTOCOL_REVENUE_IX_ACCOUNTS_LEN]>
for UpdateProtocolRevenueKeys {
    fn from(pubkeys: [Pubkey; UPDATE_PROTOCOL_REVENUE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            authority_signer: pubkeys[0],
            futarchy_authority: pubkeys[1],
            system_program: pubkeys[2],
        }
    }
}
impl<'info> From<UpdateProtocolRevenueAccounts<'_, 'info>>
for [AccountInfo<'info>; UPDATE_PROTOCOL_REVENUE_IX_ACCOUNTS_LEN] {
    fn from(accounts: UpdateProtocolRevenueAccounts<'_, 'info>) -> Self {
        [
            accounts.authority_signer.clone(),
            accounts.futarchy_authority.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; UPDATE_PROTOCOL_REVENUE_IX_ACCOUNTS_LEN]>
for UpdateProtocolRevenueAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; UPDATE_PROTOCOL_REVENUE_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            authority_signer: &arr[0],
            futarchy_authority: &arr[1],
            system_program: &arr[2],
        }
    }
}
pub const UPDATE_PROTOCOL_REVENUE_IX_DISCM: [u8; 8usize] = [
    176, 139, 131, 197, 40, 225, 125, 200,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UpdateProtocolRevenueIxArgs {
    pub args: UpdateProtocolRevenueArgs,
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateProtocolRevenueIxData(pub UpdateProtocolRevenueIxArgs);
impl From<UpdateProtocolRevenueIxArgs> for UpdateProtocolRevenueIxData {
    fn from(args: UpdateProtocolRevenueIxArgs) -> Self {
        Self(args)
    }
}
impl UpdateProtocolRevenueIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_PROTOCOL_REVENUE_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let args = if reader.is_empty() {
            Default::default()
        } else {
            <UpdateProtocolRevenueArgs>::deserialize(&mut reader)?
        };
        Ok(
            Self(UpdateProtocolRevenueIxArgs {
                args,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_PROTOCOL_REVENUE_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.args, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn update_protocol_revenue_ix_with_program_id(
    program_id: Pubkey,
    keys: UpdateProtocolRevenueKeys,
    args: UpdateProtocolRevenueIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; UPDATE_PROTOCOL_REVENUE_IX_ACCOUNTS_LEN] = keys.into();
    let data: UpdateProtocolRevenueIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_protocol_revenue_ix(
    keys: UpdateProtocolRevenueKeys,
    args: UpdateProtocolRevenueIxArgs,
) -> std::io::Result<Instruction> {
    update_protocol_revenue_ix_with_program_id(OMNIPAIR_PROGRAM_ID, keys, args)
}
pub fn update_protocol_revenue_invoke_with_program_id(
    program_id: Pubkey,
    accounts: UpdateProtocolRevenueAccounts<'_, '_>,
    args: UpdateProtocolRevenueIxArgs,
) -> ProgramResult {
    let keys: UpdateProtocolRevenueKeys = accounts.into();
    let ix = update_protocol_revenue_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn update_protocol_revenue_invoke(
    accounts: UpdateProtocolRevenueAccounts<'_, '_>,
    args: UpdateProtocolRevenueIxArgs,
) -> ProgramResult {
    update_protocol_revenue_invoke_with_program_id(OMNIPAIR_PROGRAM_ID, accounts, args)
}
pub fn update_protocol_revenue_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: UpdateProtocolRevenueAccounts<'_, '_>,
    args: UpdateProtocolRevenueIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: UpdateProtocolRevenueKeys = accounts.into();
    let ix = update_protocol_revenue_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn update_protocol_revenue_invoke_signed(
    accounts: UpdateProtocolRevenueAccounts<'_, '_>,
    args: UpdateProtocolRevenueIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    update_protocol_revenue_invoke_signed_with_program_id(
        OMNIPAIR_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn update_protocol_revenue_verify_account_keys(
    accounts: UpdateProtocolRevenueAccounts<'_, '_>,
    keys: UpdateProtocolRevenueKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.authority_signer.key, keys.authority_signer),
        (*accounts.futarchy_authority.key, keys.futarchy_authority),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn update_protocol_revenue_verify_writable_privileges<'me, 'info>(
    accounts: UpdateProtocolRevenueAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.authority_signer, accounts.futarchy_authority] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn update_protocol_revenue_verify_signer_privileges<'me, 'info>(
    accounts: UpdateProtocolRevenueAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority_signer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn update_protocol_revenue_verify_account_privileges<'me, 'info>(
    accounts: UpdateProtocolRevenueAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    update_protocol_revenue_verify_writable_privileges(accounts)?;
    update_protocol_revenue_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const UPDATE_REVENUE_RECIPIENTS_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct UpdateRevenueRecipientsAccounts<'me, 'info> {
    pub authority_signer: &'me AccountInfo<'info>,
    pub futarchy_authority: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UpdateRevenueRecipientsKeys {
    pub authority_signer: Pubkey,
    pub futarchy_authority: Pubkey,
    pub system_program: Pubkey,
}
impl From<UpdateRevenueRecipientsAccounts<'_, '_>> for UpdateRevenueRecipientsKeys {
    fn from(accounts: UpdateRevenueRecipientsAccounts) -> Self {
        Self {
            authority_signer: *accounts.authority_signer.key,
            futarchy_authority: *accounts.futarchy_authority.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<UpdateRevenueRecipientsKeys>
for [AccountMeta; UPDATE_REVENUE_RECIPIENTS_IX_ACCOUNTS_LEN] {
    fn from(keys: UpdateRevenueRecipientsKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.authority_signer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.futarchy_authority,
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
impl From<[Pubkey; UPDATE_REVENUE_RECIPIENTS_IX_ACCOUNTS_LEN]>
for UpdateRevenueRecipientsKeys {
    fn from(pubkeys: [Pubkey; UPDATE_REVENUE_RECIPIENTS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            authority_signer: pubkeys[0],
            futarchy_authority: pubkeys[1],
            system_program: pubkeys[2],
        }
    }
}
impl<'info> From<UpdateRevenueRecipientsAccounts<'_, 'info>>
for [AccountInfo<'info>; UPDATE_REVENUE_RECIPIENTS_IX_ACCOUNTS_LEN] {
    fn from(accounts: UpdateRevenueRecipientsAccounts<'_, 'info>) -> Self {
        [
            accounts.authority_signer.clone(),
            accounts.futarchy_authority.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; UPDATE_REVENUE_RECIPIENTS_IX_ACCOUNTS_LEN]>
for UpdateRevenueRecipientsAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; UPDATE_REVENUE_RECIPIENTS_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            authority_signer: &arr[0],
            futarchy_authority: &arr[1],
            system_program: &arr[2],
        }
    }
}
pub const UPDATE_REVENUE_RECIPIENTS_IX_DISCM: [u8; 8usize] = [
    116, 179, 137, 47, 118, 167, 65, 217,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UpdateRevenueRecipientsIxArgs {
    pub args: UpdateRevenueRecipientsArgs,
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateRevenueRecipientsIxData(pub UpdateRevenueRecipientsIxArgs);
impl From<UpdateRevenueRecipientsIxArgs> for UpdateRevenueRecipientsIxData {
    fn from(args: UpdateRevenueRecipientsIxArgs) -> Self {
        Self(args)
    }
}
impl UpdateRevenueRecipientsIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_REVENUE_RECIPIENTS_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let args = if reader.is_empty() {
            Default::default()
        } else {
            <UpdateRevenueRecipientsArgs>::deserialize(&mut reader)?
        };
        Ok(
            Self(UpdateRevenueRecipientsIxArgs {
                args,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_REVENUE_RECIPIENTS_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.args, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn update_revenue_recipients_ix_with_program_id(
    program_id: Pubkey,
    keys: UpdateRevenueRecipientsKeys,
    args: UpdateRevenueRecipientsIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; UPDATE_REVENUE_RECIPIENTS_IX_ACCOUNTS_LEN] = keys.into();
    let data: UpdateRevenueRecipientsIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_revenue_recipients_ix(
    keys: UpdateRevenueRecipientsKeys,
    args: UpdateRevenueRecipientsIxArgs,
) -> std::io::Result<Instruction> {
    update_revenue_recipients_ix_with_program_id(OMNIPAIR_PROGRAM_ID, keys, args)
}
pub fn update_revenue_recipients_invoke_with_program_id(
    program_id: Pubkey,
    accounts: UpdateRevenueRecipientsAccounts<'_, '_>,
    args: UpdateRevenueRecipientsIxArgs,
) -> ProgramResult {
    let keys: UpdateRevenueRecipientsKeys = accounts.into();
    let ix = update_revenue_recipients_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn update_revenue_recipients_invoke(
    accounts: UpdateRevenueRecipientsAccounts<'_, '_>,
    args: UpdateRevenueRecipientsIxArgs,
) -> ProgramResult {
    update_revenue_recipients_invoke_with_program_id(OMNIPAIR_PROGRAM_ID, accounts, args)
}
pub fn update_revenue_recipients_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: UpdateRevenueRecipientsAccounts<'_, '_>,
    args: UpdateRevenueRecipientsIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: UpdateRevenueRecipientsKeys = accounts.into();
    let ix = update_revenue_recipients_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn update_revenue_recipients_invoke_signed(
    accounts: UpdateRevenueRecipientsAccounts<'_, '_>,
    args: UpdateRevenueRecipientsIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    update_revenue_recipients_invoke_signed_with_program_id(
        OMNIPAIR_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn update_revenue_recipients_verify_account_keys(
    accounts: UpdateRevenueRecipientsAccounts<'_, '_>,
    keys: UpdateRevenueRecipientsKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.authority_signer.key, keys.authority_signer),
        (*accounts.futarchy_authority.key, keys.futarchy_authority),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn update_revenue_recipients_verify_writable_privileges<'me, 'info>(
    accounts: UpdateRevenueRecipientsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.authority_signer, accounts.futarchy_authority] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn update_revenue_recipients_verify_signer_privileges<'me, 'info>(
    accounts: UpdateRevenueRecipientsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority_signer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn update_revenue_recipients_verify_account_privileges<'me, 'info>(
    accounts: UpdateRevenueRecipientsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    update_revenue_recipients_verify_writable_privileges(accounts)?;
    update_revenue_recipients_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const VIEW_PAIR_DATA_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct ViewPairDataAccounts<'me, 'info> {
    pub pair: &'me AccountInfo<'info>,
    pub rate_model: &'me AccountInfo<'info>,
    pub futarchy_authority: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ViewPairDataKeys {
    pub pair: Pubkey,
    pub rate_model: Pubkey,
    pub futarchy_authority: Pubkey,
}
impl From<ViewPairDataAccounts<'_, '_>> for ViewPairDataKeys {
    fn from(accounts: ViewPairDataAccounts) -> Self {
        Self {
            pair: *accounts.pair.key,
            rate_model: *accounts.rate_model.key,
            futarchy_authority: *accounts.futarchy_authority.key,
        }
    }
}
impl From<ViewPairDataKeys> for [AccountMeta; VIEW_PAIR_DATA_IX_ACCOUNTS_LEN] {
    fn from(keys: ViewPairDataKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.pair,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.rate_model,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.futarchy_authority,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; VIEW_PAIR_DATA_IX_ACCOUNTS_LEN]> for ViewPairDataKeys {
    fn from(pubkeys: [Pubkey; VIEW_PAIR_DATA_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            pair: pubkeys[0],
            rate_model: pubkeys[1],
            futarchy_authority: pubkeys[2],
        }
    }
}
impl<'info> From<ViewPairDataAccounts<'_, 'info>>
for [AccountInfo<'info>; VIEW_PAIR_DATA_IX_ACCOUNTS_LEN] {
    fn from(accounts: ViewPairDataAccounts<'_, 'info>) -> Self {
        [
            accounts.pair.clone(),
            accounts.rate_model.clone(),
            accounts.futarchy_authority.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; VIEW_PAIR_DATA_IX_ACCOUNTS_LEN]>
for ViewPairDataAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; VIEW_PAIR_DATA_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            pair: &arr[0],
            rate_model: &arr[1],
            futarchy_authority: &arr[2],
        }
    }
}
pub const VIEW_PAIR_DATA_IX_DISCM: [u8; 8usize] = [30, 231, 169, 73, 19, 161, 44, 252];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ViewPairDataIxArgs {
    pub getter: PairViewKind,
    pub args: EmitValueArgs,
}
#[derive(Clone, Debug, PartialEq)]
pub struct ViewPairDataIxData(pub ViewPairDataIxArgs);
impl From<ViewPairDataIxArgs> for ViewPairDataIxData {
    fn from(args: ViewPairDataIxArgs) -> Self {
        Self(args)
    }
}
impl ViewPairDataIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != VIEW_PAIR_DATA_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let getter: PairViewKind = crate::borsh_de_or_default(&mut reader)?;
        let args = if reader.is_empty() {
            Default::default()
        } else {
            <EmitValueArgs>::deserialize(&mut reader)?
        };
        Ok(Self(ViewPairDataIxArgs { getter, args }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&VIEW_PAIR_DATA_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.getter, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.args, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn view_pair_data_ix_with_program_id(
    program_id: Pubkey,
    keys: ViewPairDataKeys,
    args: ViewPairDataIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; VIEW_PAIR_DATA_IX_ACCOUNTS_LEN] = keys.into();
    let data: ViewPairDataIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn view_pair_data_ix(
    keys: ViewPairDataKeys,
    args: ViewPairDataIxArgs,
) -> std::io::Result<Instruction> {
    view_pair_data_ix_with_program_id(OMNIPAIR_PROGRAM_ID, keys, args)
}
pub fn view_pair_data_invoke_with_program_id(
    program_id: Pubkey,
    accounts: ViewPairDataAccounts<'_, '_>,
    args: ViewPairDataIxArgs,
) -> ProgramResult {
    let keys: ViewPairDataKeys = accounts.into();
    let ix = view_pair_data_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn view_pair_data_invoke(
    accounts: ViewPairDataAccounts<'_, '_>,
    args: ViewPairDataIxArgs,
) -> ProgramResult {
    view_pair_data_invoke_with_program_id(OMNIPAIR_PROGRAM_ID, accounts, args)
}
pub fn view_pair_data_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: ViewPairDataAccounts<'_, '_>,
    args: ViewPairDataIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: ViewPairDataKeys = accounts.into();
    let ix = view_pair_data_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn view_pair_data_invoke_signed(
    accounts: ViewPairDataAccounts<'_, '_>,
    args: ViewPairDataIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    view_pair_data_invoke_signed_with_program_id(
        OMNIPAIR_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn view_pair_data_verify_account_keys(
    accounts: ViewPairDataAccounts<'_, '_>,
    keys: ViewPairDataKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.pair.key, keys.pair),
        (*accounts.rate_model.key, keys.rate_model),
        (*accounts.futarchy_authority.key, keys.futarchy_authority),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub const VIEW_USER_POSITION_DATA_IX_ACCOUNTS_LEN: usize = 4;
#[derive(Copy, Clone, Debug)]
pub struct ViewUserPositionDataAccounts<'me, 'info> {
    pub pair: &'me AccountInfo<'info>,
    pub user_position: &'me AccountInfo<'info>,
    pub rate_model: &'me AccountInfo<'info>,
    pub futarchy_authority: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ViewUserPositionDataKeys {
    pub pair: Pubkey,
    pub user_position: Pubkey,
    pub rate_model: Pubkey,
    pub futarchy_authority: Pubkey,
}
impl From<ViewUserPositionDataAccounts<'_, '_>> for ViewUserPositionDataKeys {
    fn from(accounts: ViewUserPositionDataAccounts) -> Self {
        Self {
            pair: *accounts.pair.key,
            user_position: *accounts.user_position.key,
            rate_model: *accounts.rate_model.key,
            futarchy_authority: *accounts.futarchy_authority.key,
        }
    }
}
impl From<ViewUserPositionDataKeys>
for [AccountMeta; VIEW_USER_POSITION_DATA_IX_ACCOUNTS_LEN] {
    fn from(keys: ViewUserPositionDataKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.pair,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.user_position,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.rate_model,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.futarchy_authority,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; VIEW_USER_POSITION_DATA_IX_ACCOUNTS_LEN]>
for ViewUserPositionDataKeys {
    fn from(pubkeys: [Pubkey; VIEW_USER_POSITION_DATA_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            pair: pubkeys[0],
            user_position: pubkeys[1],
            rate_model: pubkeys[2],
            futarchy_authority: pubkeys[3],
        }
    }
}
impl<'info> From<ViewUserPositionDataAccounts<'_, 'info>>
for [AccountInfo<'info>; VIEW_USER_POSITION_DATA_IX_ACCOUNTS_LEN] {
    fn from(accounts: ViewUserPositionDataAccounts<'_, 'info>) -> Self {
        [
            accounts.pair.clone(),
            accounts.user_position.clone(),
            accounts.rate_model.clone(),
            accounts.futarchy_authority.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; VIEW_USER_POSITION_DATA_IX_ACCOUNTS_LEN]>
for ViewUserPositionDataAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; VIEW_USER_POSITION_DATA_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            pair: &arr[0],
            user_position: &arr[1],
            rate_model: &arr[2],
            futarchy_authority: &arr[3],
        }
    }
}
pub const VIEW_USER_POSITION_DATA_IX_DISCM: [u8; 8usize] = [
    203, 218, 173, 213, 43, 31, 211, 152,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ViewUserPositionDataIxArgs {
    pub getter: UserPositionViewKind,
}
#[derive(Clone, Debug, PartialEq)]
pub struct ViewUserPositionDataIxData(pub ViewUserPositionDataIxArgs);
impl From<ViewUserPositionDataIxArgs> for ViewUserPositionDataIxData {
    fn from(args: ViewUserPositionDataIxArgs) -> Self {
        Self(args)
    }
}
impl ViewUserPositionDataIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != VIEW_USER_POSITION_DATA_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let getter: UserPositionViewKind = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(ViewUserPositionDataIxArgs {
                getter,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&VIEW_USER_POSITION_DATA_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.getter, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn view_user_position_data_ix_with_program_id(
    program_id: Pubkey,
    keys: ViewUserPositionDataKeys,
    args: ViewUserPositionDataIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; VIEW_USER_POSITION_DATA_IX_ACCOUNTS_LEN] = keys.into();
    let data: ViewUserPositionDataIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn view_user_position_data_ix(
    keys: ViewUserPositionDataKeys,
    args: ViewUserPositionDataIxArgs,
) -> std::io::Result<Instruction> {
    view_user_position_data_ix_with_program_id(OMNIPAIR_PROGRAM_ID, keys, args)
}
pub fn view_user_position_data_invoke_with_program_id(
    program_id: Pubkey,
    accounts: ViewUserPositionDataAccounts<'_, '_>,
    args: ViewUserPositionDataIxArgs,
) -> ProgramResult {
    let keys: ViewUserPositionDataKeys = accounts.into();
    let ix = view_user_position_data_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn view_user_position_data_invoke(
    accounts: ViewUserPositionDataAccounts<'_, '_>,
    args: ViewUserPositionDataIxArgs,
) -> ProgramResult {
    view_user_position_data_invoke_with_program_id(OMNIPAIR_PROGRAM_ID, accounts, args)
}
pub fn view_user_position_data_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: ViewUserPositionDataAccounts<'_, '_>,
    args: ViewUserPositionDataIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: ViewUserPositionDataKeys = accounts.into();
    let ix = view_user_position_data_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn view_user_position_data_invoke_signed(
    accounts: ViewUserPositionDataAccounts<'_, '_>,
    args: ViewUserPositionDataIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    view_user_position_data_invoke_signed_with_program_id(
        OMNIPAIR_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn view_user_position_data_verify_account_keys(
    accounts: ViewUserPositionDataAccounts<'_, '_>,
    keys: ViewUserPositionDataKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.pair.key, keys.pair),
        (*accounts.user_position.key, keys.user_position),
        (*accounts.rate_model.key, keys.rate_model),
        (*accounts.futarchy_authority.key, keys.futarchy_authority),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
