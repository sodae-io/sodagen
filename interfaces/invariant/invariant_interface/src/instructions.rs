use solana_pubkey::Pubkey;
use solana_cpi::{invoke, invoke_signed};
use solana_instruction::{AccountMeta, Instruction};
use solana_account_info::AccountInfo;
use solana_program_error::ProgramError;
use std::io::Read;
#[allow(unused_imports)]
use crate::*;
#[derive(Clone, Debug, PartialEq)]
pub enum InvariantProgramIx {
    CreateState(CreateStateIxArgs),
    CreateFeeTier(CreateFeeTierIxArgs),
    CreatePool(CreatePoolIxArgs),
    Swap(SwapIxArgs),
    InitializeOracle,
    CreateTick(CreateTickIxArgs),
    CreatePositionList,
    CreatePosition(CreatePositionIxArgs),
    RemovePosition(RemovePositionIxArgs),
    TransferPositionOwnership(TransferPositionOwnershipIxArgs),
    ClaimFee(ClaimFeeIxArgs),
    UpdateSecondsPerLiquidity(UpdateSecondsPerLiquidityIxArgs),
    WithdrawProtocolFee,
    ChangeProtocolFee(ChangeProtocolFeeIxArgs),
    ChangeFeeReceiver,
}
impl InvariantProgramIx {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        if buf.starts_with(&CREATE_STATE_IX_DISCM) {
            let mut reader = &buf[CREATE_STATE_IX_DISCM.len()..];
            let nonce: u8 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(Self::CreateState(CreateStateIxArgs { nonce }));
        }
        if buf.starts_with(&CREATE_FEE_TIER_IX_DISCM) {
            let mut reader = &buf[CREATE_FEE_TIER_IX_DISCM.len()..];
            let fee: u128 = crate::borsh_de_or_default(&mut reader)?;
            let tick_spacing: u16 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::CreateFeeTier(CreateFeeTierIxArgs {
                    fee,
                    tick_spacing,
                }),
            );
        }
        if buf.starts_with(&CREATE_POOL_IX_DISCM) {
            let mut reader = &buf[CREATE_POOL_IX_DISCM.len()..];
            let init_tick: i32 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(Self::CreatePool(CreatePoolIxArgs { init_tick }));
        }
        if buf.starts_with(&SWAP_IX_DISCM) {
            let mut reader = &buf[SWAP_IX_DISCM.len()..];
            let x_to_y: bool = crate::borsh_de_or_default(&mut reader)?;
            let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            let by_amount_in: bool = crate::borsh_de_or_default(&mut reader)?;
            let sqrt_price_limit: u128 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::Swap(SwapIxArgs {
                    x_to_y,
                    amount,
                    by_amount_in,
                    sqrt_price_limit,
                }),
            );
        }
        if buf.starts_with(&INITIALIZE_ORACLE_IX_DISCM) {
            return Ok(Self::InitializeOracle);
        }
        if buf.starts_with(&CREATE_TICK_IX_DISCM) {
            let mut reader = &buf[CREATE_TICK_IX_DISCM.len()..];
            let index: i32 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(Self::CreateTick(CreateTickIxArgs { index }));
        }
        if buf.starts_with(&CREATE_POSITION_LIST_IX_DISCM) {
            return Ok(Self::CreatePositionList);
        }
        if buf.starts_with(&CREATE_POSITION_IX_DISCM) {
            let mut reader = &buf[CREATE_POSITION_IX_DISCM.len()..];
            let lower_tick_index: i32 = crate::borsh_de_or_default(&mut reader)?;
            let upper_tick_index: i32 = crate::borsh_de_or_default(&mut reader)?;
            let liquidity_delta = if reader.is_empty() {
                Default::default()
            } else {
                <Liquidity>::deserialize(&mut reader)?
            };
            let slippage_limit_lower = if reader.is_empty() {
                Default::default()
            } else {
                <Price>::deserialize(&mut reader)?
            };
            let slippage_limit_upper = if reader.is_empty() {
                Default::default()
            } else {
                <Price>::deserialize(&mut reader)?
            };
            return Ok(
                Self::CreatePosition(CreatePositionIxArgs {
                    lower_tick_index,
                    upper_tick_index,
                    liquidity_delta,
                    slippage_limit_lower,
                    slippage_limit_upper,
                }),
            );
        }
        if buf.starts_with(&REMOVE_POSITION_IX_DISCM) {
            let mut reader = &buf[REMOVE_POSITION_IX_DISCM.len()..];
            let index: u32 = crate::borsh_de_or_default(&mut reader)?;
            let lower_tick_index: i32 = crate::borsh_de_or_default(&mut reader)?;
            let upper_tick_index: i32 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::RemovePosition(RemovePositionIxArgs {
                    index,
                    lower_tick_index,
                    upper_tick_index,
                }),
            );
        }
        if buf.starts_with(&TRANSFER_POSITION_OWNERSHIP_IX_DISCM) {
            let mut reader = &buf[TRANSFER_POSITION_OWNERSHIP_IX_DISCM.len()..];
            let index: u32 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::TransferPositionOwnership(TransferPositionOwnershipIxArgs {
                    index,
                }),
            );
        }
        if buf.starts_with(&CLAIM_FEE_IX_DISCM) {
            let mut reader = &buf[CLAIM_FEE_IX_DISCM.len()..];
            let index: u32 = crate::borsh_de_or_default(&mut reader)?;
            let lower_tick_index: i32 = crate::borsh_de_or_default(&mut reader)?;
            let upper_tick_index: i32 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::ClaimFee(ClaimFeeIxArgs {
                    index,
                    lower_tick_index,
                    upper_tick_index,
                }),
            );
        }
        if buf.starts_with(&UPDATE_SECONDS_PER_LIQUIDITY_IX_DISCM) {
            let mut reader = &buf[UPDATE_SECONDS_PER_LIQUIDITY_IX_DISCM.len()..];
            let lower_tick_index: i32 = crate::borsh_de_or_default(&mut reader)?;
            let upper_tick_index: i32 = crate::borsh_de_or_default(&mut reader)?;
            let index: i32 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::UpdateSecondsPerLiquidity(UpdateSecondsPerLiquidityIxArgs {
                    lower_tick_index,
                    upper_tick_index,
                    index,
                }),
            );
        }
        if buf.starts_with(&WITHDRAW_PROTOCOL_FEE_IX_DISCM) {
            return Ok(Self::WithdrawProtocolFee);
        }
        if buf.starts_with(&CHANGE_PROTOCOL_FEE_IX_DISCM) {
            let mut reader = &buf[CHANGE_PROTOCOL_FEE_IX_DISCM.len()..];
            let protocol_fee = if reader.is_empty() {
                Default::default()
            } else {
                <FixedPoint>::deserialize(&mut reader)?
            };
            return Ok(
                Self::ChangeProtocolFee(ChangeProtocolFeeIxArgs {
                    protocol_fee,
                }),
            );
        }
        if buf.starts_with(&CHANGE_FEE_RECEIVER_IX_DISCM) {
            return Ok(Self::ChangeFeeReceiver);
        }
        Err(std::io::Error::from(std::io::ErrorKind::InvalidData))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        match self {
            Self::CreateState(args) => {
                writer.write_all(&CREATE_STATE_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.nonce, &mut writer)?;
                Ok(())
            }
            Self::CreateFeeTier(args) => {
                writer.write_all(&CREATE_FEE_TIER_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.fee, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.tick_spacing, &mut writer)?;
                Ok(())
            }
            Self::CreatePool(args) => {
                writer.write_all(&CREATE_POOL_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.init_tick, &mut writer)?;
                Ok(())
            }
            Self::Swap(args) => {
                writer.write_all(&SWAP_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.x_to_y, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.amount, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.by_amount_in, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.sqrt_price_limit, &mut writer)?;
                Ok(())
            }
            Self::InitializeOracle => writer.write_all(&INITIALIZE_ORACLE_IX_DISCM),
            Self::CreateTick(args) => {
                writer.write_all(&CREATE_TICK_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.index, &mut writer)?;
                Ok(())
            }
            Self::CreatePositionList => writer.write_all(&CREATE_POSITION_LIST_IX_DISCM),
            Self::CreatePosition(args) => {
                writer.write_all(&CREATE_POSITION_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.lower_tick_index, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.upper_tick_index, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.liquidity_delta, &mut writer)?;
                borsh::BorshSerialize::serialize(
                    &args.slippage_limit_lower,
                    &mut writer,
                )?;
                borsh::BorshSerialize::serialize(
                    &args.slippage_limit_upper,
                    &mut writer,
                )?;
                Ok(())
            }
            Self::RemovePosition(args) => {
                writer.write_all(&REMOVE_POSITION_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.index, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.lower_tick_index, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.upper_tick_index, &mut writer)?;
                Ok(())
            }
            Self::TransferPositionOwnership(args) => {
                writer.write_all(&TRANSFER_POSITION_OWNERSHIP_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.index, &mut writer)?;
                Ok(())
            }
            Self::ClaimFee(args) => {
                writer.write_all(&CLAIM_FEE_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.index, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.lower_tick_index, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.upper_tick_index, &mut writer)?;
                Ok(())
            }
            Self::UpdateSecondsPerLiquidity(args) => {
                writer.write_all(&UPDATE_SECONDS_PER_LIQUIDITY_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.lower_tick_index, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.upper_tick_index, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.index, &mut writer)?;
                Ok(())
            }
            Self::WithdrawProtocolFee => {
                writer.write_all(&WITHDRAW_PROTOCOL_FEE_IX_DISCM)
            }
            Self::ChangeProtocolFee(args) => {
                writer.write_all(&CHANGE_PROTOCOL_FEE_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.protocol_fee, &mut writer)?;
                Ok(())
            }
            Self::ChangeFeeReceiver => writer.write_all(&CHANGE_FEE_RECEIVER_IX_DISCM),
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
pub const CREATE_STATE_IX_ACCOUNTS_LEN: usize = 5;
#[derive(Copy, Clone, Debug)]
pub struct CreateStateAccounts<'me, 'info> {
    pub state: &'me AccountInfo<'info>,
    pub admin: &'me AccountInfo<'info>,
    pub program_authority: &'me AccountInfo<'info>,
    pub rent: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CreateStateKeys {
    pub state: Pubkey,
    pub admin: Pubkey,
    pub program_authority: Pubkey,
    pub rent: Pubkey,
    pub system_program: Pubkey,
}
impl From<CreateStateAccounts<'_, '_>> for CreateStateKeys {
    fn from(accounts: CreateStateAccounts) -> Self {
        Self {
            state: *accounts.state.key,
            admin: *accounts.admin.key,
            program_authority: *accounts.program_authority.key,
            rent: *accounts.rent.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<CreateStateKeys> for [AccountMeta; CREATE_STATE_IX_ACCOUNTS_LEN] {
    fn from(keys: CreateStateKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.admin,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.program_authority,
                is_signer: false,
                is_writable: false,
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
impl From<[Pubkey; CREATE_STATE_IX_ACCOUNTS_LEN]> for CreateStateKeys {
    fn from(pubkeys: [Pubkey; CREATE_STATE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            state: pubkeys[0],
            admin: pubkeys[1],
            program_authority: pubkeys[2],
            rent: pubkeys[3],
            system_program: pubkeys[4],
        }
    }
}
impl<'info> From<CreateStateAccounts<'_, 'info>>
for [AccountInfo<'info>; CREATE_STATE_IX_ACCOUNTS_LEN] {
    fn from(accounts: CreateStateAccounts<'_, 'info>) -> Self {
        [
            accounts.state.clone(),
            accounts.admin.clone(),
            accounts.program_authority.clone(),
            accounts.rent.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CREATE_STATE_IX_ACCOUNTS_LEN]>
for CreateStateAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; CREATE_STATE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            state: &arr[0],
            admin: &arr[1],
            program_authority: &arr[2],
            rent: &arr[3],
            system_program: &arr[4],
        }
    }
}
pub const CREATE_STATE_IX_DISCM: [u8; 8usize] = [214, 211, 209, 79, 107, 105, 247, 222];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CreateStateIxArgs {
    pub nonce: u8,
}
#[derive(Clone, Debug, PartialEq)]
pub struct CreateStateIxData(pub CreateStateIxArgs);
impl From<CreateStateIxArgs> for CreateStateIxData {
    fn from(args: CreateStateIxArgs) -> Self {
        Self(args)
    }
}
impl CreateStateIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CREATE_STATE_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let nonce: u8 = crate::borsh_de_or_default(&mut reader)?;
        Ok(Self(CreateStateIxArgs { nonce }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CREATE_STATE_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.nonce, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn create_state_ix_with_program_id(
    program_id: Pubkey,
    keys: CreateStateKeys,
    args: CreateStateIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CREATE_STATE_IX_ACCOUNTS_LEN] = keys.into();
    let data: CreateStateIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn create_state_ix(
    keys: CreateStateKeys,
    args: CreateStateIxArgs,
) -> std::io::Result<Instruction> {
    create_state_ix_with_program_id(INVARIANT_PROGRAM_ID, keys, args)
}
pub fn create_state_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CreateStateAccounts<'_, '_>,
    args: CreateStateIxArgs,
) -> ProgramResult {
    let keys: CreateStateKeys = accounts.into();
    let ix = create_state_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn create_state_invoke(
    accounts: CreateStateAccounts<'_, '_>,
    args: CreateStateIxArgs,
) -> ProgramResult {
    create_state_invoke_with_program_id(INVARIANT_PROGRAM_ID, accounts, args)
}
pub fn create_state_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CreateStateAccounts<'_, '_>,
    args: CreateStateIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CreateStateKeys = accounts.into();
    let ix = create_state_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn create_state_invoke_signed(
    accounts: CreateStateAccounts<'_, '_>,
    args: CreateStateIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    create_state_invoke_signed_with_program_id(
        INVARIANT_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn create_state_verify_account_keys(
    accounts: CreateStateAccounts<'_, '_>,
    keys: CreateStateKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.state.key, keys.state),
        (*accounts.admin.key, keys.admin),
        (*accounts.program_authority.key, keys.program_authority),
        (*accounts.rent.key, keys.rent),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn create_state_verify_writable_privileges<'me, 'info>(
    accounts: CreateStateAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.state, accounts.admin] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn create_state_verify_signer_privileges<'me, 'info>(
    accounts: CreateStateAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn create_state_verify_account_privileges<'me, 'info>(
    accounts: CreateStateAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    create_state_verify_writable_privileges(accounts)?;
    create_state_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CREATE_FEE_TIER_IX_ACCOUNTS_LEN: usize = 5;
#[derive(Copy, Clone, Debug)]
pub struct CreateFeeTierAccounts<'me, 'info> {
    pub fee_tier: &'me AccountInfo<'info>,
    pub state: &'me AccountInfo<'info>,
    pub admin: &'me AccountInfo<'info>,
    pub rent: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CreateFeeTierKeys {
    pub fee_tier: Pubkey,
    pub state: Pubkey,
    pub admin: Pubkey,
    pub rent: Pubkey,
    pub system_program: Pubkey,
}
impl From<CreateFeeTierAccounts<'_, '_>> for CreateFeeTierKeys {
    fn from(accounts: CreateFeeTierAccounts) -> Self {
        Self {
            fee_tier: *accounts.fee_tier.key,
            state: *accounts.state.key,
            admin: *accounts.admin.key,
            rent: *accounts.rent.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<CreateFeeTierKeys> for [AccountMeta; CREATE_FEE_TIER_IX_ACCOUNTS_LEN] {
    fn from(keys: CreateFeeTierKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.fee_tier,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.state,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.admin,
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
impl From<[Pubkey; CREATE_FEE_TIER_IX_ACCOUNTS_LEN]> for CreateFeeTierKeys {
    fn from(pubkeys: [Pubkey; CREATE_FEE_TIER_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            fee_tier: pubkeys[0],
            state: pubkeys[1],
            admin: pubkeys[2],
            rent: pubkeys[3],
            system_program: pubkeys[4],
        }
    }
}
impl<'info> From<CreateFeeTierAccounts<'_, 'info>>
for [AccountInfo<'info>; CREATE_FEE_TIER_IX_ACCOUNTS_LEN] {
    fn from(accounts: CreateFeeTierAccounts<'_, 'info>) -> Self {
        [
            accounts.fee_tier.clone(),
            accounts.state.clone(),
            accounts.admin.clone(),
            accounts.rent.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CREATE_FEE_TIER_IX_ACCOUNTS_LEN]>
for CreateFeeTierAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; CREATE_FEE_TIER_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            fee_tier: &arr[0],
            state: &arr[1],
            admin: &arr[2],
            rent: &arr[3],
            system_program: &arr[4],
        }
    }
}
pub const CREATE_FEE_TIER_IX_DISCM: [u8; 8usize] = [150, 158, 85, 114, 219, 75, 212, 91];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CreateFeeTierIxArgs {
    pub fee: u128,
    pub tick_spacing: u16,
}
#[derive(Clone, Debug, PartialEq)]
pub struct CreateFeeTierIxData(pub CreateFeeTierIxArgs);
impl From<CreateFeeTierIxArgs> for CreateFeeTierIxData {
    fn from(args: CreateFeeTierIxArgs) -> Self {
        Self(args)
    }
}
impl CreateFeeTierIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CREATE_FEE_TIER_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let fee: u128 = crate::borsh_de_or_default(&mut reader)?;
        let tick_spacing: u16 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(CreateFeeTierIxArgs {
                fee,
                tick_spacing,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CREATE_FEE_TIER_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.fee, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.tick_spacing, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn create_fee_tier_ix_with_program_id(
    program_id: Pubkey,
    keys: CreateFeeTierKeys,
    args: CreateFeeTierIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CREATE_FEE_TIER_IX_ACCOUNTS_LEN] = keys.into();
    let data: CreateFeeTierIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn create_fee_tier_ix(
    keys: CreateFeeTierKeys,
    args: CreateFeeTierIxArgs,
) -> std::io::Result<Instruction> {
    create_fee_tier_ix_with_program_id(INVARIANT_PROGRAM_ID, keys, args)
}
pub fn create_fee_tier_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CreateFeeTierAccounts<'_, '_>,
    args: CreateFeeTierIxArgs,
) -> ProgramResult {
    let keys: CreateFeeTierKeys = accounts.into();
    let ix = create_fee_tier_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn create_fee_tier_invoke(
    accounts: CreateFeeTierAccounts<'_, '_>,
    args: CreateFeeTierIxArgs,
) -> ProgramResult {
    create_fee_tier_invoke_with_program_id(INVARIANT_PROGRAM_ID, accounts, args)
}
pub fn create_fee_tier_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CreateFeeTierAccounts<'_, '_>,
    args: CreateFeeTierIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CreateFeeTierKeys = accounts.into();
    let ix = create_fee_tier_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn create_fee_tier_invoke_signed(
    accounts: CreateFeeTierAccounts<'_, '_>,
    args: CreateFeeTierIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    create_fee_tier_invoke_signed_with_program_id(
        INVARIANT_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn create_fee_tier_verify_account_keys(
    accounts: CreateFeeTierAccounts<'_, '_>,
    keys: CreateFeeTierKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.fee_tier.key, keys.fee_tier),
        (*accounts.state.key, keys.state),
        (*accounts.admin.key, keys.admin),
        (*accounts.rent.key, keys.rent),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn create_fee_tier_verify_writable_privileges<'me, 'info>(
    accounts: CreateFeeTierAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.fee_tier, accounts.admin] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn create_fee_tier_verify_signer_privileges<'me, 'info>(
    accounts: CreateFeeTierAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn create_fee_tier_verify_account_privileges<'me, 'info>(
    accounts: CreateFeeTierAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    create_fee_tier_verify_writable_privileges(accounts)?;
    create_fee_tier_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CREATE_POOL_IX_ACCOUNTS_LEN: usize = 13;
#[derive(Copy, Clone, Debug)]
pub struct CreatePoolAccounts<'me, 'info> {
    pub state: &'me AccountInfo<'info>,
    pub pool: &'me AccountInfo<'info>,
    pub fee_tier: &'me AccountInfo<'info>,
    pub tickmap: &'me AccountInfo<'info>,
    pub token_x: &'me AccountInfo<'info>,
    pub token_y: &'me AccountInfo<'info>,
    pub token_x_reserve: &'me AccountInfo<'info>,
    pub token_y_reserve: &'me AccountInfo<'info>,
    pub payer: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub rent: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CreatePoolKeys {
    pub state: Pubkey,
    pub pool: Pubkey,
    pub fee_tier: Pubkey,
    pub tickmap: Pubkey,
    pub token_x: Pubkey,
    pub token_y: Pubkey,
    pub token_x_reserve: Pubkey,
    pub token_y_reserve: Pubkey,
    pub payer: Pubkey,
    pub authority: Pubkey,
    pub token_program: Pubkey,
    pub rent: Pubkey,
    pub system_program: Pubkey,
}
impl From<CreatePoolAccounts<'_, '_>> for CreatePoolKeys {
    fn from(accounts: CreatePoolAccounts) -> Self {
        Self {
            state: *accounts.state.key,
            pool: *accounts.pool.key,
            fee_tier: *accounts.fee_tier.key,
            tickmap: *accounts.tickmap.key,
            token_x: *accounts.token_x.key,
            token_y: *accounts.token_y.key,
            token_x_reserve: *accounts.token_x_reserve.key,
            token_y_reserve: *accounts.token_y_reserve.key,
            payer: *accounts.payer.key,
            authority: *accounts.authority.key,
            token_program: *accounts.token_program.key,
            rent: *accounts.rent.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<CreatePoolKeys> for [AccountMeta; CREATE_POOL_IX_ACCOUNTS_LEN] {
    fn from(keys: CreatePoolKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.state,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.fee_tier,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.tickmap,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_x,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_y,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_x_reserve,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_y_reserve,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.payer,
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
impl From<[Pubkey; CREATE_POOL_IX_ACCOUNTS_LEN]> for CreatePoolKeys {
    fn from(pubkeys: [Pubkey; CREATE_POOL_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            state: pubkeys[0],
            pool: pubkeys[1],
            fee_tier: pubkeys[2],
            tickmap: pubkeys[3],
            token_x: pubkeys[4],
            token_y: pubkeys[5],
            token_x_reserve: pubkeys[6],
            token_y_reserve: pubkeys[7],
            payer: pubkeys[8],
            authority: pubkeys[9],
            token_program: pubkeys[10],
            rent: pubkeys[11],
            system_program: pubkeys[12],
        }
    }
}
impl<'info> From<CreatePoolAccounts<'_, 'info>>
for [AccountInfo<'info>; CREATE_POOL_IX_ACCOUNTS_LEN] {
    fn from(accounts: CreatePoolAccounts<'_, 'info>) -> Self {
        [
            accounts.state.clone(),
            accounts.pool.clone(),
            accounts.fee_tier.clone(),
            accounts.tickmap.clone(),
            accounts.token_x.clone(),
            accounts.token_y.clone(),
            accounts.token_x_reserve.clone(),
            accounts.token_y_reserve.clone(),
            accounts.payer.clone(),
            accounts.authority.clone(),
            accounts.token_program.clone(),
            accounts.rent.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CREATE_POOL_IX_ACCOUNTS_LEN]>
for CreatePoolAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; CREATE_POOL_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            state: &arr[0],
            pool: &arr[1],
            fee_tier: &arr[2],
            tickmap: &arr[3],
            token_x: &arr[4],
            token_y: &arr[5],
            token_x_reserve: &arr[6],
            token_y_reserve: &arr[7],
            payer: &arr[8],
            authority: &arr[9],
            token_program: &arr[10],
            rent: &arr[11],
            system_program: &arr[12],
        }
    }
}
pub const CREATE_POOL_IX_DISCM: [u8; 8usize] = [233, 146, 209, 142, 207, 104, 64, 188];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CreatePoolIxArgs {
    pub init_tick: i32,
}
#[derive(Clone, Debug, PartialEq)]
pub struct CreatePoolIxData(pub CreatePoolIxArgs);
impl From<CreatePoolIxArgs> for CreatePoolIxData {
    fn from(args: CreatePoolIxArgs) -> Self {
        Self(args)
    }
}
impl CreatePoolIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CREATE_POOL_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let init_tick: i32 = crate::borsh_de_or_default(&mut reader)?;
        Ok(Self(CreatePoolIxArgs { init_tick }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CREATE_POOL_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.init_tick, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn create_pool_ix_with_program_id(
    program_id: Pubkey,
    keys: CreatePoolKeys,
    args: CreatePoolIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CREATE_POOL_IX_ACCOUNTS_LEN] = keys.into();
    let data: CreatePoolIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn create_pool_ix(
    keys: CreatePoolKeys,
    args: CreatePoolIxArgs,
) -> std::io::Result<Instruction> {
    create_pool_ix_with_program_id(INVARIANT_PROGRAM_ID, keys, args)
}
pub fn create_pool_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CreatePoolAccounts<'_, '_>,
    args: CreatePoolIxArgs,
) -> ProgramResult {
    let keys: CreatePoolKeys = accounts.into();
    let ix = create_pool_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn create_pool_invoke(
    accounts: CreatePoolAccounts<'_, '_>,
    args: CreatePoolIxArgs,
) -> ProgramResult {
    create_pool_invoke_with_program_id(INVARIANT_PROGRAM_ID, accounts, args)
}
pub fn create_pool_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CreatePoolAccounts<'_, '_>,
    args: CreatePoolIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CreatePoolKeys = accounts.into();
    let ix = create_pool_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn create_pool_invoke_signed(
    accounts: CreatePoolAccounts<'_, '_>,
    args: CreatePoolIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    create_pool_invoke_signed_with_program_id(
        INVARIANT_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn create_pool_verify_account_keys(
    accounts: CreatePoolAccounts<'_, '_>,
    keys: CreatePoolKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.state.key, keys.state),
        (*accounts.pool.key, keys.pool),
        (*accounts.fee_tier.key, keys.fee_tier),
        (*accounts.tickmap.key, keys.tickmap),
        (*accounts.token_x.key, keys.token_x),
        (*accounts.token_y.key, keys.token_y),
        (*accounts.token_x_reserve.key, keys.token_x_reserve),
        (*accounts.token_y_reserve.key, keys.token_y_reserve),
        (*accounts.payer.key, keys.payer),
        (*accounts.authority.key, keys.authority),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.rent.key, keys.rent),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn create_pool_verify_writable_privileges<'me, 'info>(
    accounts: CreatePoolAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.pool,
        accounts.tickmap,
        accounts.token_x_reserve,
        accounts.token_y_reserve,
        accounts.payer,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn create_pool_verify_signer_privileges<'me, 'info>(
    accounts: CreatePoolAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [
        accounts.token_x_reserve,
        accounts.token_y_reserve,
        accounts.payer,
    ] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn create_pool_verify_account_privileges<'me, 'info>(
    accounts: CreatePoolAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    create_pool_verify_writable_privileges(accounts)?;
    create_pool_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SWAP_IX_ACCOUNTS_LEN: usize = 10;
#[derive(Copy, Clone, Debug)]
pub struct SwapAccounts<'me, 'info> {
    pub state: &'me AccountInfo<'info>,
    pub pool: &'me AccountInfo<'info>,
    pub tickmap: &'me AccountInfo<'info>,
    pub account_x: &'me AccountInfo<'info>,
    pub account_y: &'me AccountInfo<'info>,
    pub reserve_x: &'me AccountInfo<'info>,
    pub reserve_y: &'me AccountInfo<'info>,
    pub owner: &'me AccountInfo<'info>,
    pub program_authority: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SwapKeys {
    pub state: Pubkey,
    pub pool: Pubkey,
    pub tickmap: Pubkey,
    pub account_x: Pubkey,
    pub account_y: Pubkey,
    pub reserve_x: Pubkey,
    pub reserve_y: Pubkey,
    pub owner: Pubkey,
    pub program_authority: Pubkey,
    pub token_program: Pubkey,
}
impl From<SwapAccounts<'_, '_>> for SwapKeys {
    fn from(accounts: SwapAccounts) -> Self {
        Self {
            state: *accounts.state.key,
            pool: *accounts.pool.key,
            tickmap: *accounts.tickmap.key,
            account_x: *accounts.account_x.key,
            account_y: *accounts.account_y.key,
            reserve_x: *accounts.reserve_x.key,
            reserve_y: *accounts.reserve_y.key,
            owner: *accounts.owner.key,
            program_authority: *accounts.program_authority.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<SwapKeys> for [AccountMeta; SWAP_IX_ACCOUNTS_LEN] {
    fn from(keys: SwapKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.state,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.tickmap,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.account_x,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.account_y,
                is_signer: false,
                is_writable: true,
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
                pubkey: keys.owner,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.program_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; SWAP_IX_ACCOUNTS_LEN]> for SwapKeys {
    fn from(pubkeys: [Pubkey; SWAP_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            state: pubkeys[0],
            pool: pubkeys[1],
            tickmap: pubkeys[2],
            account_x: pubkeys[3],
            account_y: pubkeys[4],
            reserve_x: pubkeys[5],
            reserve_y: pubkeys[6],
            owner: pubkeys[7],
            program_authority: pubkeys[8],
            token_program: pubkeys[9],
        }
    }
}
impl<'info> From<SwapAccounts<'_, 'info>>
for [AccountInfo<'info>; SWAP_IX_ACCOUNTS_LEN] {
    fn from(accounts: SwapAccounts<'_, 'info>) -> Self {
        [
            accounts.state.clone(),
            accounts.pool.clone(),
            accounts.tickmap.clone(),
            accounts.account_x.clone(),
            accounts.account_y.clone(),
            accounts.reserve_x.clone(),
            accounts.reserve_y.clone(),
            accounts.owner.clone(),
            accounts.program_authority.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SWAP_IX_ACCOUNTS_LEN]>
for SwapAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; SWAP_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            state: &arr[0],
            pool: &arr[1],
            tickmap: &arr[2],
            account_x: &arr[3],
            account_y: &arr[4],
            reserve_x: &arr[5],
            reserve_y: &arr[6],
            owner: &arr[7],
            program_authority: &arr[8],
            token_program: &arr[9],
        }
    }
}
pub const SWAP_IX_DISCM: [u8; 8usize] = [248, 198, 158, 145, 225, 117, 135, 200];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SwapIxArgs {
    pub x_to_y: bool,
    pub amount: u64,
    pub by_amount_in: bool,
    pub sqrt_price_limit: u128,
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
        let x_to_y: bool = crate::borsh_de_or_default(&mut reader)?;
        let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let by_amount_in: bool = crate::borsh_de_or_default(&mut reader)?;
        let sqrt_price_limit: u128 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(SwapIxArgs {
                x_to_y,
                amount,
                by_amount_in,
                sqrt_price_limit,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SWAP_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.x_to_y, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.by_amount_in, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.sqrt_price_limit, &mut writer)?;
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
    swap_ix_with_program_id(INVARIANT_PROGRAM_ID, keys, args)
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
    swap_invoke_with_program_id(INVARIANT_PROGRAM_ID, accounts, args)
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
    swap_invoke_signed_with_program_id(INVARIANT_PROGRAM_ID, accounts, args, seeds)
}
pub fn swap_verify_account_keys(
    accounts: SwapAccounts<'_, '_>,
    keys: SwapKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.state.key, keys.state),
        (*accounts.pool.key, keys.pool),
        (*accounts.tickmap.key, keys.tickmap),
        (*accounts.account_x.key, keys.account_x),
        (*accounts.account_y.key, keys.account_y),
        (*accounts.reserve_x.key, keys.reserve_x),
        (*accounts.reserve_y.key, keys.reserve_y),
        (*accounts.owner.key, keys.owner),
        (*accounts.program_authority.key, keys.program_authority),
        (*accounts.token_program.key, keys.token_program),
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
        accounts.tickmap,
        accounts.account_x,
        accounts.account_y,
        accounts.reserve_x,
        accounts.reserve_y,
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
    for should_be_signer in [accounts.owner] {
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
pub const INITIALIZE_ORACLE_IX_ACCOUNTS_LEN: usize = 7;
#[derive(Copy, Clone, Debug)]
pub struct InitializeOracleAccounts<'me, 'info> {
    pub pool: &'me AccountInfo<'info>,
    pub oracle: &'me AccountInfo<'info>,
    pub token_x: &'me AccountInfo<'info>,
    pub token_y: &'me AccountInfo<'info>,
    pub payer: &'me AccountInfo<'info>,
    pub rent: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct InitializeOracleKeys {
    pub pool: Pubkey,
    pub oracle: Pubkey,
    pub token_x: Pubkey,
    pub token_y: Pubkey,
    pub payer: Pubkey,
    pub rent: Pubkey,
    pub system_program: Pubkey,
}
impl From<InitializeOracleAccounts<'_, '_>> for InitializeOracleKeys {
    fn from(accounts: InitializeOracleAccounts) -> Self {
        Self {
            pool: *accounts.pool.key,
            oracle: *accounts.oracle.key,
            token_x: *accounts.token_x.key,
            token_y: *accounts.token_y.key,
            payer: *accounts.payer.key,
            rent: *accounts.rent.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<InitializeOracleKeys> for [AccountMeta; INITIALIZE_ORACLE_IX_ACCOUNTS_LEN] {
    fn from(keys: InitializeOracleKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.oracle,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_x,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_y,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.payer,
                is_signer: true,
                is_writable: false,
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
impl From<[Pubkey; INITIALIZE_ORACLE_IX_ACCOUNTS_LEN]> for InitializeOracleKeys {
    fn from(pubkeys: [Pubkey; INITIALIZE_ORACLE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            pool: pubkeys[0],
            oracle: pubkeys[1],
            token_x: pubkeys[2],
            token_y: pubkeys[3],
            payer: pubkeys[4],
            rent: pubkeys[5],
            system_program: pubkeys[6],
        }
    }
}
impl<'info> From<InitializeOracleAccounts<'_, 'info>>
for [AccountInfo<'info>; INITIALIZE_ORACLE_IX_ACCOUNTS_LEN] {
    fn from(accounts: InitializeOracleAccounts<'_, 'info>) -> Self {
        [
            accounts.pool.clone(),
            accounts.oracle.clone(),
            accounts.token_x.clone(),
            accounts.token_y.clone(),
            accounts.payer.clone(),
            accounts.rent.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; INITIALIZE_ORACLE_IX_ACCOUNTS_LEN]>
for InitializeOracleAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; INITIALIZE_ORACLE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            pool: &arr[0],
            oracle: &arr[1],
            token_x: &arr[2],
            token_y: &arr[3],
            payer: &arr[4],
            rent: &arr[5],
            system_program: &arr[6],
        }
    }
}
pub const INITIALIZE_ORACLE_IX_DISCM: [u8; 8usize] = [
    144, 223, 131, 120, 196, 253, 181, 99,
];
#[derive(Clone, Debug, PartialEq)]
pub struct InitializeOracleIxData;
impl InitializeOracleIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INITIALIZE_ORACLE_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INITIALIZE_ORACLE_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn initialize_oracle_ix_with_program_id(
    program_id: Pubkey,
    keys: InitializeOracleKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; INITIALIZE_ORACLE_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: InitializeOracleIxData.try_to_vec()?,
    })
}
pub fn initialize_oracle_ix(keys: InitializeOracleKeys) -> std::io::Result<Instruction> {
    initialize_oracle_ix_with_program_id(INVARIANT_PROGRAM_ID, keys)
}
pub fn initialize_oracle_invoke_with_program_id(
    program_id: Pubkey,
    accounts: InitializeOracleAccounts<'_, '_>,
) -> ProgramResult {
    let keys: InitializeOracleKeys = accounts.into();
    let ix = initialize_oracle_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn initialize_oracle_invoke(
    accounts: InitializeOracleAccounts<'_, '_>,
) -> ProgramResult {
    initialize_oracle_invoke_with_program_id(INVARIANT_PROGRAM_ID, accounts)
}
pub fn initialize_oracle_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: InitializeOracleAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: InitializeOracleKeys = accounts.into();
    let ix = initialize_oracle_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn initialize_oracle_invoke_signed(
    accounts: InitializeOracleAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    initialize_oracle_invoke_signed_with_program_id(
        INVARIANT_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn initialize_oracle_verify_account_keys(
    accounts: InitializeOracleAccounts<'_, '_>,
    keys: InitializeOracleKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.pool.key, keys.pool),
        (*accounts.oracle.key, keys.oracle),
        (*accounts.token_x.key, keys.token_x),
        (*accounts.token_y.key, keys.token_y),
        (*accounts.payer.key, keys.payer),
        (*accounts.rent.key, keys.rent),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn initialize_oracle_verify_writable_privileges<'me, 'info>(
    accounts: InitializeOracleAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.pool, accounts.oracle] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn initialize_oracle_verify_signer_privileges<'me, 'info>(
    accounts: InitializeOracleAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.payer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn initialize_oracle_verify_account_privileges<'me, 'info>(
    accounts: InitializeOracleAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    initialize_oracle_verify_writable_privileges(accounts)?;
    initialize_oracle_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CREATE_TICK_IX_ACCOUNTS_LEN: usize = 8;
#[derive(Copy, Clone, Debug)]
pub struct CreateTickAccounts<'me, 'info> {
    pub tick: &'me AccountInfo<'info>,
    pub pool: &'me AccountInfo<'info>,
    pub tickmap: &'me AccountInfo<'info>,
    pub payer: &'me AccountInfo<'info>,
    pub token_x: &'me AccountInfo<'info>,
    pub token_y: &'me AccountInfo<'info>,
    pub rent: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CreateTickKeys {
    pub tick: Pubkey,
    pub pool: Pubkey,
    pub tickmap: Pubkey,
    pub payer: Pubkey,
    pub token_x: Pubkey,
    pub token_y: Pubkey,
    pub rent: Pubkey,
    pub system_program: Pubkey,
}
impl From<CreateTickAccounts<'_, '_>> for CreateTickKeys {
    fn from(accounts: CreateTickAccounts) -> Self {
        Self {
            tick: *accounts.tick.key,
            pool: *accounts.pool.key,
            tickmap: *accounts.tickmap.key,
            payer: *accounts.payer.key,
            token_x: *accounts.token_x.key,
            token_y: *accounts.token_y.key,
            rent: *accounts.rent.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<CreateTickKeys> for [AccountMeta; CREATE_TICK_IX_ACCOUNTS_LEN] {
    fn from(keys: CreateTickKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.tick,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pool,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.tickmap,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.payer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_x,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_y,
                is_signer: false,
                is_writable: false,
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
impl From<[Pubkey; CREATE_TICK_IX_ACCOUNTS_LEN]> for CreateTickKeys {
    fn from(pubkeys: [Pubkey; CREATE_TICK_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            tick: pubkeys[0],
            pool: pubkeys[1],
            tickmap: pubkeys[2],
            payer: pubkeys[3],
            token_x: pubkeys[4],
            token_y: pubkeys[5],
            rent: pubkeys[6],
            system_program: pubkeys[7],
        }
    }
}
impl<'info> From<CreateTickAccounts<'_, 'info>>
for [AccountInfo<'info>; CREATE_TICK_IX_ACCOUNTS_LEN] {
    fn from(accounts: CreateTickAccounts<'_, 'info>) -> Self {
        [
            accounts.tick.clone(),
            accounts.pool.clone(),
            accounts.tickmap.clone(),
            accounts.payer.clone(),
            accounts.token_x.clone(),
            accounts.token_y.clone(),
            accounts.rent.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CREATE_TICK_IX_ACCOUNTS_LEN]>
for CreateTickAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; CREATE_TICK_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            tick: &arr[0],
            pool: &arr[1],
            tickmap: &arr[2],
            payer: &arr[3],
            token_x: &arr[4],
            token_y: &arr[5],
            rent: &arr[6],
            system_program: &arr[7],
        }
    }
}
pub const CREATE_TICK_IX_DISCM: [u8; 8usize] = [227, 158, 200, 168, 122, 104, 133, 81];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CreateTickIxArgs {
    pub index: i32,
}
#[derive(Clone, Debug, PartialEq)]
pub struct CreateTickIxData(pub CreateTickIxArgs);
impl From<CreateTickIxArgs> for CreateTickIxData {
    fn from(args: CreateTickIxArgs) -> Self {
        Self(args)
    }
}
impl CreateTickIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CREATE_TICK_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let index: i32 = crate::borsh_de_or_default(&mut reader)?;
        Ok(Self(CreateTickIxArgs { index }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CREATE_TICK_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.index, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn create_tick_ix_with_program_id(
    program_id: Pubkey,
    keys: CreateTickKeys,
    args: CreateTickIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CREATE_TICK_IX_ACCOUNTS_LEN] = keys.into();
    let data: CreateTickIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn create_tick_ix(
    keys: CreateTickKeys,
    args: CreateTickIxArgs,
) -> std::io::Result<Instruction> {
    create_tick_ix_with_program_id(INVARIANT_PROGRAM_ID, keys, args)
}
pub fn create_tick_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CreateTickAccounts<'_, '_>,
    args: CreateTickIxArgs,
) -> ProgramResult {
    let keys: CreateTickKeys = accounts.into();
    let ix = create_tick_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn create_tick_invoke(
    accounts: CreateTickAccounts<'_, '_>,
    args: CreateTickIxArgs,
) -> ProgramResult {
    create_tick_invoke_with_program_id(INVARIANT_PROGRAM_ID, accounts, args)
}
pub fn create_tick_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CreateTickAccounts<'_, '_>,
    args: CreateTickIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CreateTickKeys = accounts.into();
    let ix = create_tick_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn create_tick_invoke_signed(
    accounts: CreateTickAccounts<'_, '_>,
    args: CreateTickIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    create_tick_invoke_signed_with_program_id(
        INVARIANT_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn create_tick_verify_account_keys(
    accounts: CreateTickAccounts<'_, '_>,
    keys: CreateTickKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.tick.key, keys.tick),
        (*accounts.pool.key, keys.pool),
        (*accounts.tickmap.key, keys.tickmap),
        (*accounts.payer.key, keys.payer),
        (*accounts.token_x.key, keys.token_x),
        (*accounts.token_y.key, keys.token_y),
        (*accounts.rent.key, keys.rent),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn create_tick_verify_writable_privileges<'me, 'info>(
    accounts: CreateTickAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.tick, accounts.tickmap, accounts.payer] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn create_tick_verify_signer_privileges<'me, 'info>(
    accounts: CreateTickAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.payer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn create_tick_verify_account_privileges<'me, 'info>(
    accounts: CreateTickAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    create_tick_verify_writable_privileges(accounts)?;
    create_tick_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CREATE_POSITION_LIST_IX_ACCOUNTS_LEN: usize = 5;
#[derive(Copy, Clone, Debug)]
pub struct CreatePositionListAccounts<'me, 'info> {
    pub position_list: &'me AccountInfo<'info>,
    pub owner: &'me AccountInfo<'info>,
    pub signer: &'me AccountInfo<'info>,
    pub rent: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CreatePositionListKeys {
    pub position_list: Pubkey,
    pub owner: Pubkey,
    pub signer: Pubkey,
    pub rent: Pubkey,
    pub system_program: Pubkey,
}
impl From<CreatePositionListAccounts<'_, '_>> for CreatePositionListKeys {
    fn from(accounts: CreatePositionListAccounts) -> Self {
        Self {
            position_list: *accounts.position_list.key,
            owner: *accounts.owner.key,
            signer: *accounts.signer.key,
            rent: *accounts.rent.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<CreatePositionListKeys>
for [AccountMeta; CREATE_POSITION_LIST_IX_ACCOUNTS_LEN] {
    fn from(keys: CreatePositionListKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.position_list,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.owner,
                is_signer: false,
                is_writable: false,
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
impl From<[Pubkey; CREATE_POSITION_LIST_IX_ACCOUNTS_LEN]> for CreatePositionListKeys {
    fn from(pubkeys: [Pubkey; CREATE_POSITION_LIST_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            position_list: pubkeys[0],
            owner: pubkeys[1],
            signer: pubkeys[2],
            rent: pubkeys[3],
            system_program: pubkeys[4],
        }
    }
}
impl<'info> From<CreatePositionListAccounts<'_, 'info>>
for [AccountInfo<'info>; CREATE_POSITION_LIST_IX_ACCOUNTS_LEN] {
    fn from(accounts: CreatePositionListAccounts<'_, 'info>) -> Self {
        [
            accounts.position_list.clone(),
            accounts.owner.clone(),
            accounts.signer.clone(),
            accounts.rent.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CREATE_POSITION_LIST_IX_ACCOUNTS_LEN]>
for CreatePositionListAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; CREATE_POSITION_LIST_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            position_list: &arr[0],
            owner: &arr[1],
            signer: &arr[2],
            rent: &arr[3],
            system_program: &arr[4],
        }
    }
}
pub const CREATE_POSITION_LIST_IX_DISCM: [u8; 8usize] = [
    135, 165, 83, 94, 175, 24, 149, 4,
];
#[derive(Clone, Debug, PartialEq)]
pub struct CreatePositionListIxData;
impl CreatePositionListIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CREATE_POSITION_LIST_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CREATE_POSITION_LIST_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn create_position_list_ix_with_program_id(
    program_id: Pubkey,
    keys: CreatePositionListKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CREATE_POSITION_LIST_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: CreatePositionListIxData.try_to_vec()?,
    })
}
pub fn create_position_list_ix(
    keys: CreatePositionListKeys,
) -> std::io::Result<Instruction> {
    create_position_list_ix_with_program_id(INVARIANT_PROGRAM_ID, keys)
}
pub fn create_position_list_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CreatePositionListAccounts<'_, '_>,
) -> ProgramResult {
    let keys: CreatePositionListKeys = accounts.into();
    let ix = create_position_list_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn create_position_list_invoke(
    accounts: CreatePositionListAccounts<'_, '_>,
) -> ProgramResult {
    create_position_list_invoke_with_program_id(INVARIANT_PROGRAM_ID, accounts)
}
pub fn create_position_list_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CreatePositionListAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CreatePositionListKeys = accounts.into();
    let ix = create_position_list_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn create_position_list_invoke_signed(
    accounts: CreatePositionListAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    create_position_list_invoke_signed_with_program_id(
        INVARIANT_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn create_position_list_verify_account_keys(
    accounts: CreatePositionListAccounts<'_, '_>,
    keys: CreatePositionListKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.position_list.key, keys.position_list),
        (*accounts.owner.key, keys.owner),
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
pub fn create_position_list_verify_writable_privileges<'me, 'info>(
    accounts: CreatePositionListAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.position_list, accounts.signer] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn create_position_list_verify_signer_privileges<'me, 'info>(
    accounts: CreatePositionListAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.signer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn create_position_list_verify_account_privileges<'me, 'info>(
    accounts: CreatePositionListAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    create_position_list_verify_writable_privileges(accounts)?;
    create_position_list_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CREATE_POSITION_IX_ACCOUNTS_LEN: usize = 19;
#[derive(Copy, Clone, Debug)]
pub struct CreatePositionAccounts<'me, 'info> {
    pub state: &'me AccountInfo<'info>,
    pub position: &'me AccountInfo<'info>,
    pub pool: &'me AccountInfo<'info>,
    pub position_list: &'me AccountInfo<'info>,
    pub payer: &'me AccountInfo<'info>,
    pub owner: &'me AccountInfo<'info>,
    pub lower_tick: &'me AccountInfo<'info>,
    pub upper_tick: &'me AccountInfo<'info>,
    pub tickmap: &'me AccountInfo<'info>,
    pub token_x: &'me AccountInfo<'info>,
    pub token_y: &'me AccountInfo<'info>,
    pub account_x: &'me AccountInfo<'info>,
    pub account_y: &'me AccountInfo<'info>,
    pub reserve_x: &'me AccountInfo<'info>,
    pub reserve_y: &'me AccountInfo<'info>,
    pub program_authority: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub rent: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CreatePositionKeys {
    pub state: Pubkey,
    pub position: Pubkey,
    pub pool: Pubkey,
    pub position_list: Pubkey,
    pub payer: Pubkey,
    pub owner: Pubkey,
    pub lower_tick: Pubkey,
    pub upper_tick: Pubkey,
    pub tickmap: Pubkey,
    pub token_x: Pubkey,
    pub token_y: Pubkey,
    pub account_x: Pubkey,
    pub account_y: Pubkey,
    pub reserve_x: Pubkey,
    pub reserve_y: Pubkey,
    pub program_authority: Pubkey,
    pub token_program: Pubkey,
    pub rent: Pubkey,
    pub system_program: Pubkey,
}
impl From<CreatePositionAccounts<'_, '_>> for CreatePositionKeys {
    fn from(accounts: CreatePositionAccounts) -> Self {
        Self {
            state: *accounts.state.key,
            position: *accounts.position.key,
            pool: *accounts.pool.key,
            position_list: *accounts.position_list.key,
            payer: *accounts.payer.key,
            owner: *accounts.owner.key,
            lower_tick: *accounts.lower_tick.key,
            upper_tick: *accounts.upper_tick.key,
            tickmap: *accounts.tickmap.key,
            token_x: *accounts.token_x.key,
            token_y: *accounts.token_y.key,
            account_x: *accounts.account_x.key,
            account_y: *accounts.account_y.key,
            reserve_x: *accounts.reserve_x.key,
            reserve_y: *accounts.reserve_y.key,
            program_authority: *accounts.program_authority.key,
            token_program: *accounts.token_program.key,
            rent: *accounts.rent.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<CreatePositionKeys> for [AccountMeta; CREATE_POSITION_IX_ACCOUNTS_LEN] {
    fn from(keys: CreatePositionKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.state,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.position,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.position_list,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.payer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.owner,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.lower_tick,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.upper_tick,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.tickmap,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_x,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_y,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.account_x,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.account_y,
                is_signer: false,
                is_writable: true,
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
                pubkey: keys.program_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
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
impl From<[Pubkey; CREATE_POSITION_IX_ACCOUNTS_LEN]> for CreatePositionKeys {
    fn from(pubkeys: [Pubkey; CREATE_POSITION_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            state: pubkeys[0],
            position: pubkeys[1],
            pool: pubkeys[2],
            position_list: pubkeys[3],
            payer: pubkeys[4],
            owner: pubkeys[5],
            lower_tick: pubkeys[6],
            upper_tick: pubkeys[7],
            tickmap: pubkeys[8],
            token_x: pubkeys[9],
            token_y: pubkeys[10],
            account_x: pubkeys[11],
            account_y: pubkeys[12],
            reserve_x: pubkeys[13],
            reserve_y: pubkeys[14],
            program_authority: pubkeys[15],
            token_program: pubkeys[16],
            rent: pubkeys[17],
            system_program: pubkeys[18],
        }
    }
}
impl<'info> From<CreatePositionAccounts<'_, 'info>>
for [AccountInfo<'info>; CREATE_POSITION_IX_ACCOUNTS_LEN] {
    fn from(accounts: CreatePositionAccounts<'_, 'info>) -> Self {
        [
            accounts.state.clone(),
            accounts.position.clone(),
            accounts.pool.clone(),
            accounts.position_list.clone(),
            accounts.payer.clone(),
            accounts.owner.clone(),
            accounts.lower_tick.clone(),
            accounts.upper_tick.clone(),
            accounts.tickmap.clone(),
            accounts.token_x.clone(),
            accounts.token_y.clone(),
            accounts.account_x.clone(),
            accounts.account_y.clone(),
            accounts.reserve_x.clone(),
            accounts.reserve_y.clone(),
            accounts.program_authority.clone(),
            accounts.token_program.clone(),
            accounts.rent.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CREATE_POSITION_IX_ACCOUNTS_LEN]>
for CreatePositionAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; CREATE_POSITION_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            state: &arr[0],
            position: &arr[1],
            pool: &arr[2],
            position_list: &arr[3],
            payer: &arr[4],
            owner: &arr[5],
            lower_tick: &arr[6],
            upper_tick: &arr[7],
            tickmap: &arr[8],
            token_x: &arr[9],
            token_y: &arr[10],
            account_x: &arr[11],
            account_y: &arr[12],
            reserve_x: &arr[13],
            reserve_y: &arr[14],
            program_authority: &arr[15],
            token_program: &arr[16],
            rent: &arr[17],
            system_program: &arr[18],
        }
    }
}
pub const CREATE_POSITION_IX_DISCM: [u8; 8usize] = [
    48, 215, 197, 153, 96, 203, 180, 133,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CreatePositionIxArgs {
    pub lower_tick_index: i32,
    pub upper_tick_index: i32,
    pub liquidity_delta: Liquidity,
    pub slippage_limit_lower: Price,
    pub slippage_limit_upper: Price,
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
        let lower_tick_index: i32 = crate::borsh_de_or_default(&mut reader)?;
        let upper_tick_index: i32 = crate::borsh_de_or_default(&mut reader)?;
        let liquidity_delta = if reader.is_empty() {
            Default::default()
        } else {
            <Liquidity>::deserialize(&mut reader)?
        };
        let slippage_limit_lower = if reader.is_empty() {
            Default::default()
        } else {
            <Price>::deserialize(&mut reader)?
        };
        let slippage_limit_upper = if reader.is_empty() {
            Default::default()
        } else {
            <Price>::deserialize(&mut reader)?
        };
        Ok(
            Self(CreatePositionIxArgs {
                lower_tick_index,
                upper_tick_index,
                liquidity_delta,
                slippage_limit_lower,
                slippage_limit_upper,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CREATE_POSITION_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.lower_tick_index, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.upper_tick_index, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.liquidity_delta, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.slippage_limit_lower, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.slippage_limit_upper, &mut writer)?;
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
    create_position_ix_with_program_id(INVARIANT_PROGRAM_ID, keys, args)
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
    create_position_invoke_with_program_id(INVARIANT_PROGRAM_ID, accounts, args)
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
        INVARIANT_PROGRAM_ID,
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
        (*accounts.state.key, keys.state),
        (*accounts.position.key, keys.position),
        (*accounts.pool.key, keys.pool),
        (*accounts.position_list.key, keys.position_list),
        (*accounts.payer.key, keys.payer),
        (*accounts.owner.key, keys.owner),
        (*accounts.lower_tick.key, keys.lower_tick),
        (*accounts.upper_tick.key, keys.upper_tick),
        (*accounts.tickmap.key, keys.tickmap),
        (*accounts.token_x.key, keys.token_x),
        (*accounts.token_y.key, keys.token_y),
        (*accounts.account_x.key, keys.account_x),
        (*accounts.account_y.key, keys.account_y),
        (*accounts.reserve_x.key, keys.reserve_x),
        (*accounts.reserve_y.key, keys.reserve_y),
        (*accounts.program_authority.key, keys.program_authority),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.rent.key, keys.rent),
        (*accounts.system_program.key, keys.system_program),
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
        accounts.pool,
        accounts.position_list,
        accounts.payer,
        accounts.lower_tick,
        accounts.upper_tick,
        accounts.tickmap,
        accounts.account_x,
        accounts.account_y,
        accounts.reserve_x,
        accounts.reserve_y,
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
    for should_be_signer in [accounts.payer, accounts.owner] {
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
pub const REMOVE_POSITION_IX_ACCOUNTS_LEN: usize = 17;
#[derive(Copy, Clone, Debug)]
pub struct RemovePositionAccounts<'me, 'info> {
    pub state: &'me AccountInfo<'info>,
    pub removed_position: &'me AccountInfo<'info>,
    pub position_list: &'me AccountInfo<'info>,
    pub last_position: &'me AccountInfo<'info>,
    pub pool: &'me AccountInfo<'info>,
    pub tickmap: &'me AccountInfo<'info>,
    pub lower_tick: &'me AccountInfo<'info>,
    pub upper_tick: &'me AccountInfo<'info>,
    pub owner: &'me AccountInfo<'info>,
    pub token_x: &'me AccountInfo<'info>,
    pub token_y: &'me AccountInfo<'info>,
    pub account_x: &'me AccountInfo<'info>,
    pub account_y: &'me AccountInfo<'info>,
    pub reserve_x: &'me AccountInfo<'info>,
    pub reserve_y: &'me AccountInfo<'info>,
    pub program_authority: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct RemovePositionKeys {
    pub state: Pubkey,
    pub removed_position: Pubkey,
    pub position_list: Pubkey,
    pub last_position: Pubkey,
    pub pool: Pubkey,
    pub tickmap: Pubkey,
    pub lower_tick: Pubkey,
    pub upper_tick: Pubkey,
    pub owner: Pubkey,
    pub token_x: Pubkey,
    pub token_y: Pubkey,
    pub account_x: Pubkey,
    pub account_y: Pubkey,
    pub reserve_x: Pubkey,
    pub reserve_y: Pubkey,
    pub program_authority: Pubkey,
    pub token_program: Pubkey,
}
impl From<RemovePositionAccounts<'_, '_>> for RemovePositionKeys {
    fn from(accounts: RemovePositionAccounts) -> Self {
        Self {
            state: *accounts.state.key,
            removed_position: *accounts.removed_position.key,
            position_list: *accounts.position_list.key,
            last_position: *accounts.last_position.key,
            pool: *accounts.pool.key,
            tickmap: *accounts.tickmap.key,
            lower_tick: *accounts.lower_tick.key,
            upper_tick: *accounts.upper_tick.key,
            owner: *accounts.owner.key,
            token_x: *accounts.token_x.key,
            token_y: *accounts.token_y.key,
            account_x: *accounts.account_x.key,
            account_y: *accounts.account_y.key,
            reserve_x: *accounts.reserve_x.key,
            reserve_y: *accounts.reserve_y.key,
            program_authority: *accounts.program_authority.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<RemovePositionKeys> for [AccountMeta; REMOVE_POSITION_IX_ACCOUNTS_LEN] {
    fn from(keys: RemovePositionKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.state,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.removed_position,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.position_list,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.last_position,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.tickmap,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.lower_tick,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.upper_tick,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.owner,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_x,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_y,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.account_x,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.account_y,
                is_signer: false,
                is_writable: true,
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
                pubkey: keys.program_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; REMOVE_POSITION_IX_ACCOUNTS_LEN]> for RemovePositionKeys {
    fn from(pubkeys: [Pubkey; REMOVE_POSITION_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            state: pubkeys[0],
            removed_position: pubkeys[1],
            position_list: pubkeys[2],
            last_position: pubkeys[3],
            pool: pubkeys[4],
            tickmap: pubkeys[5],
            lower_tick: pubkeys[6],
            upper_tick: pubkeys[7],
            owner: pubkeys[8],
            token_x: pubkeys[9],
            token_y: pubkeys[10],
            account_x: pubkeys[11],
            account_y: pubkeys[12],
            reserve_x: pubkeys[13],
            reserve_y: pubkeys[14],
            program_authority: pubkeys[15],
            token_program: pubkeys[16],
        }
    }
}
impl<'info> From<RemovePositionAccounts<'_, 'info>>
for [AccountInfo<'info>; REMOVE_POSITION_IX_ACCOUNTS_LEN] {
    fn from(accounts: RemovePositionAccounts<'_, 'info>) -> Self {
        [
            accounts.state.clone(),
            accounts.removed_position.clone(),
            accounts.position_list.clone(),
            accounts.last_position.clone(),
            accounts.pool.clone(),
            accounts.tickmap.clone(),
            accounts.lower_tick.clone(),
            accounts.upper_tick.clone(),
            accounts.owner.clone(),
            accounts.token_x.clone(),
            accounts.token_y.clone(),
            accounts.account_x.clone(),
            accounts.account_y.clone(),
            accounts.reserve_x.clone(),
            accounts.reserve_y.clone(),
            accounts.program_authority.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; REMOVE_POSITION_IX_ACCOUNTS_LEN]>
for RemovePositionAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; REMOVE_POSITION_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            state: &arr[0],
            removed_position: &arr[1],
            position_list: &arr[2],
            last_position: &arr[3],
            pool: &arr[4],
            tickmap: &arr[5],
            lower_tick: &arr[6],
            upper_tick: &arr[7],
            owner: &arr[8],
            token_x: &arr[9],
            token_y: &arr[10],
            account_x: &arr[11],
            account_y: &arr[12],
            reserve_x: &arr[13],
            reserve_y: &arr[14],
            program_authority: &arr[15],
            token_program: &arr[16],
        }
    }
}
pub const REMOVE_POSITION_IX_DISCM: [u8; 8usize] = [219, 24, 236, 110, 138, 80, 129, 6];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct RemovePositionIxArgs {
    pub index: u32,
    pub lower_tick_index: i32,
    pub upper_tick_index: i32,
}
#[derive(Clone, Debug, PartialEq)]
pub struct RemovePositionIxData(pub RemovePositionIxArgs);
impl From<RemovePositionIxArgs> for RemovePositionIxData {
    fn from(args: RemovePositionIxArgs) -> Self {
        Self(args)
    }
}
impl RemovePositionIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != REMOVE_POSITION_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let index: u32 = crate::borsh_de_or_default(&mut reader)?;
        let lower_tick_index: i32 = crate::borsh_de_or_default(&mut reader)?;
        let upper_tick_index: i32 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(RemovePositionIxArgs {
                index,
                lower_tick_index,
                upper_tick_index,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&REMOVE_POSITION_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.index, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.lower_tick_index, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.upper_tick_index, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn remove_position_ix_with_program_id(
    program_id: Pubkey,
    keys: RemovePositionKeys,
    args: RemovePositionIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; REMOVE_POSITION_IX_ACCOUNTS_LEN] = keys.into();
    let data: RemovePositionIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn remove_position_ix(
    keys: RemovePositionKeys,
    args: RemovePositionIxArgs,
) -> std::io::Result<Instruction> {
    remove_position_ix_with_program_id(INVARIANT_PROGRAM_ID, keys, args)
}
pub fn remove_position_invoke_with_program_id(
    program_id: Pubkey,
    accounts: RemovePositionAccounts<'_, '_>,
    args: RemovePositionIxArgs,
) -> ProgramResult {
    let keys: RemovePositionKeys = accounts.into();
    let ix = remove_position_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn remove_position_invoke(
    accounts: RemovePositionAccounts<'_, '_>,
    args: RemovePositionIxArgs,
) -> ProgramResult {
    remove_position_invoke_with_program_id(INVARIANT_PROGRAM_ID, accounts, args)
}
pub fn remove_position_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: RemovePositionAccounts<'_, '_>,
    args: RemovePositionIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: RemovePositionKeys = accounts.into();
    let ix = remove_position_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn remove_position_invoke_signed(
    accounts: RemovePositionAccounts<'_, '_>,
    args: RemovePositionIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    remove_position_invoke_signed_with_program_id(
        INVARIANT_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn remove_position_verify_account_keys(
    accounts: RemovePositionAccounts<'_, '_>,
    keys: RemovePositionKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.state.key, keys.state),
        (*accounts.removed_position.key, keys.removed_position),
        (*accounts.position_list.key, keys.position_list),
        (*accounts.last_position.key, keys.last_position),
        (*accounts.pool.key, keys.pool),
        (*accounts.tickmap.key, keys.tickmap),
        (*accounts.lower_tick.key, keys.lower_tick),
        (*accounts.upper_tick.key, keys.upper_tick),
        (*accounts.owner.key, keys.owner),
        (*accounts.token_x.key, keys.token_x),
        (*accounts.token_y.key, keys.token_y),
        (*accounts.account_x.key, keys.account_x),
        (*accounts.account_y.key, keys.account_y),
        (*accounts.reserve_x.key, keys.reserve_x),
        (*accounts.reserve_y.key, keys.reserve_y),
        (*accounts.program_authority.key, keys.program_authority),
        (*accounts.token_program.key, keys.token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn remove_position_verify_writable_privileges<'me, 'info>(
    accounts: RemovePositionAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.removed_position,
        accounts.position_list,
        accounts.last_position,
        accounts.pool,
        accounts.tickmap,
        accounts.lower_tick,
        accounts.upper_tick,
        accounts.owner,
        accounts.account_x,
        accounts.account_y,
        accounts.reserve_x,
        accounts.reserve_y,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn remove_position_verify_signer_privileges<'me, 'info>(
    accounts: RemovePositionAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.owner] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn remove_position_verify_account_privileges<'me, 'info>(
    accounts: RemovePositionAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    remove_position_verify_writable_privileges(accounts)?;
    remove_position_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const TRANSFER_POSITION_OWNERSHIP_IX_ACCOUNTS_LEN: usize = 9;
#[derive(Copy, Clone, Debug)]
pub struct TransferPositionOwnershipAccounts<'me, 'info> {
    pub owner_list: &'me AccountInfo<'info>,
    pub recipient_list: &'me AccountInfo<'info>,
    pub new_position: &'me AccountInfo<'info>,
    pub removed_position: &'me AccountInfo<'info>,
    pub last_position: &'me AccountInfo<'info>,
    pub owner: &'me AccountInfo<'info>,
    pub recipient: &'me AccountInfo<'info>,
    pub rent: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct TransferPositionOwnershipKeys {
    pub owner_list: Pubkey,
    pub recipient_list: Pubkey,
    pub new_position: Pubkey,
    pub removed_position: Pubkey,
    pub last_position: Pubkey,
    pub owner: Pubkey,
    pub recipient: Pubkey,
    pub rent: Pubkey,
    pub system_program: Pubkey,
}
impl From<TransferPositionOwnershipAccounts<'_, '_>> for TransferPositionOwnershipKeys {
    fn from(accounts: TransferPositionOwnershipAccounts) -> Self {
        Self {
            owner_list: *accounts.owner_list.key,
            recipient_list: *accounts.recipient_list.key,
            new_position: *accounts.new_position.key,
            removed_position: *accounts.removed_position.key,
            last_position: *accounts.last_position.key,
            owner: *accounts.owner.key,
            recipient: *accounts.recipient.key,
            rent: *accounts.rent.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<TransferPositionOwnershipKeys>
for [AccountMeta; TRANSFER_POSITION_OWNERSHIP_IX_ACCOUNTS_LEN] {
    fn from(keys: TransferPositionOwnershipKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.owner_list,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.recipient_list,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.new_position,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.removed_position,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.last_position,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.owner,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.recipient,
                is_signer: false,
                is_writable: false,
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
impl From<[Pubkey; TRANSFER_POSITION_OWNERSHIP_IX_ACCOUNTS_LEN]>
for TransferPositionOwnershipKeys {
    fn from(pubkeys: [Pubkey; TRANSFER_POSITION_OWNERSHIP_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            owner_list: pubkeys[0],
            recipient_list: pubkeys[1],
            new_position: pubkeys[2],
            removed_position: pubkeys[3],
            last_position: pubkeys[4],
            owner: pubkeys[5],
            recipient: pubkeys[6],
            rent: pubkeys[7],
            system_program: pubkeys[8],
        }
    }
}
impl<'info> From<TransferPositionOwnershipAccounts<'_, 'info>>
for [AccountInfo<'info>; TRANSFER_POSITION_OWNERSHIP_IX_ACCOUNTS_LEN] {
    fn from(accounts: TransferPositionOwnershipAccounts<'_, 'info>) -> Self {
        [
            accounts.owner_list.clone(),
            accounts.recipient_list.clone(),
            accounts.new_position.clone(),
            accounts.removed_position.clone(),
            accounts.last_position.clone(),
            accounts.owner.clone(),
            accounts.recipient.clone(),
            accounts.rent.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; TRANSFER_POSITION_OWNERSHIP_IX_ACCOUNTS_LEN]>
for TransferPositionOwnershipAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; TRANSFER_POSITION_OWNERSHIP_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            owner_list: &arr[0],
            recipient_list: &arr[1],
            new_position: &arr[2],
            removed_position: &arr[3],
            last_position: &arr[4],
            owner: &arr[5],
            recipient: &arr[6],
            rent: &arr[7],
            system_program: &arr[8],
        }
    }
}
pub const TRANSFER_POSITION_OWNERSHIP_IX_DISCM: [u8; 8usize] = [
    99, 194, 166, 162, 172, 182, 45, 228,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TransferPositionOwnershipIxArgs {
    pub index: u32,
}
#[derive(Clone, Debug, PartialEq)]
pub struct TransferPositionOwnershipIxData(pub TransferPositionOwnershipIxArgs);
impl From<TransferPositionOwnershipIxArgs> for TransferPositionOwnershipIxData {
    fn from(args: TransferPositionOwnershipIxArgs) -> Self {
        Self(args)
    }
}
impl TransferPositionOwnershipIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != TRANSFER_POSITION_OWNERSHIP_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let index: u32 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(TransferPositionOwnershipIxArgs {
                index,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&TRANSFER_POSITION_OWNERSHIP_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.index, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn transfer_position_ownership_ix_with_program_id(
    program_id: Pubkey,
    keys: TransferPositionOwnershipKeys,
    args: TransferPositionOwnershipIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; TRANSFER_POSITION_OWNERSHIP_IX_ACCOUNTS_LEN] = keys.into();
    let data: TransferPositionOwnershipIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn transfer_position_ownership_ix(
    keys: TransferPositionOwnershipKeys,
    args: TransferPositionOwnershipIxArgs,
) -> std::io::Result<Instruction> {
    transfer_position_ownership_ix_with_program_id(INVARIANT_PROGRAM_ID, keys, args)
}
pub fn transfer_position_ownership_invoke_with_program_id(
    program_id: Pubkey,
    accounts: TransferPositionOwnershipAccounts<'_, '_>,
    args: TransferPositionOwnershipIxArgs,
) -> ProgramResult {
    let keys: TransferPositionOwnershipKeys = accounts.into();
    let ix = transfer_position_ownership_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn transfer_position_ownership_invoke(
    accounts: TransferPositionOwnershipAccounts<'_, '_>,
    args: TransferPositionOwnershipIxArgs,
) -> ProgramResult {
    transfer_position_ownership_invoke_with_program_id(
        INVARIANT_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn transfer_position_ownership_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: TransferPositionOwnershipAccounts<'_, '_>,
    args: TransferPositionOwnershipIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: TransferPositionOwnershipKeys = accounts.into();
    let ix = transfer_position_ownership_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn transfer_position_ownership_invoke_signed(
    accounts: TransferPositionOwnershipAccounts<'_, '_>,
    args: TransferPositionOwnershipIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    transfer_position_ownership_invoke_signed_with_program_id(
        INVARIANT_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn transfer_position_ownership_verify_account_keys(
    accounts: TransferPositionOwnershipAccounts<'_, '_>,
    keys: TransferPositionOwnershipKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.owner_list.key, keys.owner_list),
        (*accounts.recipient_list.key, keys.recipient_list),
        (*accounts.new_position.key, keys.new_position),
        (*accounts.removed_position.key, keys.removed_position),
        (*accounts.last_position.key, keys.last_position),
        (*accounts.owner.key, keys.owner),
        (*accounts.recipient.key, keys.recipient),
        (*accounts.rent.key, keys.rent),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn transfer_position_ownership_verify_writable_privileges<'me, 'info>(
    accounts: TransferPositionOwnershipAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.owner_list,
        accounts.recipient_list,
        accounts.new_position,
        accounts.removed_position,
        accounts.last_position,
        accounts.owner,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn transfer_position_ownership_verify_signer_privileges<'me, 'info>(
    accounts: TransferPositionOwnershipAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.owner] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn transfer_position_ownership_verify_account_privileges<'me, 'info>(
    accounts: TransferPositionOwnershipAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    transfer_position_ownership_verify_writable_privileges(accounts)?;
    transfer_position_ownership_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CLAIM_FEE_IX_ACCOUNTS_LEN: usize = 14;
#[derive(Copy, Clone, Debug)]
pub struct ClaimFeeAccounts<'me, 'info> {
    pub state: &'me AccountInfo<'info>,
    pub pool: &'me AccountInfo<'info>,
    pub position: &'me AccountInfo<'info>,
    pub lower_tick: &'me AccountInfo<'info>,
    pub upper_tick: &'me AccountInfo<'info>,
    pub owner: &'me AccountInfo<'info>,
    pub token_x: &'me AccountInfo<'info>,
    pub token_y: &'me AccountInfo<'info>,
    pub account_x: &'me AccountInfo<'info>,
    pub account_y: &'me AccountInfo<'info>,
    pub reserve_x: &'me AccountInfo<'info>,
    pub reserve_y: &'me AccountInfo<'info>,
    pub program_authority: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ClaimFeeKeys {
    pub state: Pubkey,
    pub pool: Pubkey,
    pub position: Pubkey,
    pub lower_tick: Pubkey,
    pub upper_tick: Pubkey,
    pub owner: Pubkey,
    pub token_x: Pubkey,
    pub token_y: Pubkey,
    pub account_x: Pubkey,
    pub account_y: Pubkey,
    pub reserve_x: Pubkey,
    pub reserve_y: Pubkey,
    pub program_authority: Pubkey,
    pub token_program: Pubkey,
}
impl From<ClaimFeeAccounts<'_, '_>> for ClaimFeeKeys {
    fn from(accounts: ClaimFeeAccounts) -> Self {
        Self {
            state: *accounts.state.key,
            pool: *accounts.pool.key,
            position: *accounts.position.key,
            lower_tick: *accounts.lower_tick.key,
            upper_tick: *accounts.upper_tick.key,
            owner: *accounts.owner.key,
            token_x: *accounts.token_x.key,
            token_y: *accounts.token_y.key,
            account_x: *accounts.account_x.key,
            account_y: *accounts.account_y.key,
            reserve_x: *accounts.reserve_x.key,
            reserve_y: *accounts.reserve_y.key,
            program_authority: *accounts.program_authority.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<ClaimFeeKeys> for [AccountMeta; CLAIM_FEE_IX_ACCOUNTS_LEN] {
    fn from(keys: ClaimFeeKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.state,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.position,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.lower_tick,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.upper_tick,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.owner,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_x,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_y,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.account_x,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.account_y,
                is_signer: false,
                is_writable: true,
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
                pubkey: keys.program_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; CLAIM_FEE_IX_ACCOUNTS_LEN]> for ClaimFeeKeys {
    fn from(pubkeys: [Pubkey; CLAIM_FEE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            state: pubkeys[0],
            pool: pubkeys[1],
            position: pubkeys[2],
            lower_tick: pubkeys[3],
            upper_tick: pubkeys[4],
            owner: pubkeys[5],
            token_x: pubkeys[6],
            token_y: pubkeys[7],
            account_x: pubkeys[8],
            account_y: pubkeys[9],
            reserve_x: pubkeys[10],
            reserve_y: pubkeys[11],
            program_authority: pubkeys[12],
            token_program: pubkeys[13],
        }
    }
}
impl<'info> From<ClaimFeeAccounts<'_, 'info>>
for [AccountInfo<'info>; CLAIM_FEE_IX_ACCOUNTS_LEN] {
    fn from(accounts: ClaimFeeAccounts<'_, 'info>) -> Self {
        [
            accounts.state.clone(),
            accounts.pool.clone(),
            accounts.position.clone(),
            accounts.lower_tick.clone(),
            accounts.upper_tick.clone(),
            accounts.owner.clone(),
            accounts.token_x.clone(),
            accounts.token_y.clone(),
            accounts.account_x.clone(),
            accounts.account_y.clone(),
            accounts.reserve_x.clone(),
            accounts.reserve_y.clone(),
            accounts.program_authority.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CLAIM_FEE_IX_ACCOUNTS_LEN]>
for ClaimFeeAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; CLAIM_FEE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            state: &arr[0],
            pool: &arr[1],
            position: &arr[2],
            lower_tick: &arr[3],
            upper_tick: &arr[4],
            owner: &arr[5],
            token_x: &arr[6],
            token_y: &arr[7],
            account_x: &arr[8],
            account_y: &arr[9],
            reserve_x: &arr[10],
            reserve_y: &arr[11],
            program_authority: &arr[12],
            token_program: &arr[13],
        }
    }
}
pub const CLAIM_FEE_IX_DISCM: [u8; 8usize] = [169, 32, 79, 137, 136, 232, 70, 137];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ClaimFeeIxArgs {
    pub index: u32,
    pub lower_tick_index: i32,
    pub upper_tick_index: i32,
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
        let index: u32 = crate::borsh_de_or_default(&mut reader)?;
        let lower_tick_index: i32 = crate::borsh_de_or_default(&mut reader)?;
        let upper_tick_index: i32 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(ClaimFeeIxArgs {
                index,
                lower_tick_index,
                upper_tick_index,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CLAIM_FEE_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.index, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.lower_tick_index, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.upper_tick_index, &mut writer)?;
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
    claim_fee_ix_with_program_id(INVARIANT_PROGRAM_ID, keys, args)
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
    claim_fee_invoke_with_program_id(INVARIANT_PROGRAM_ID, accounts, args)
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
    claim_fee_invoke_signed_with_program_id(INVARIANT_PROGRAM_ID, accounts, args, seeds)
}
pub fn claim_fee_verify_account_keys(
    accounts: ClaimFeeAccounts<'_, '_>,
    keys: ClaimFeeKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.state.key, keys.state),
        (*accounts.pool.key, keys.pool),
        (*accounts.position.key, keys.position),
        (*accounts.lower_tick.key, keys.lower_tick),
        (*accounts.upper_tick.key, keys.upper_tick),
        (*accounts.owner.key, keys.owner),
        (*accounts.token_x.key, keys.token_x),
        (*accounts.token_y.key, keys.token_y),
        (*accounts.account_x.key, keys.account_x),
        (*accounts.account_y.key, keys.account_y),
        (*accounts.reserve_x.key, keys.reserve_x),
        (*accounts.reserve_y.key, keys.reserve_y),
        (*accounts.program_authority.key, keys.program_authority),
        (*accounts.token_program.key, keys.token_program),
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
        accounts.pool,
        accounts.position,
        accounts.lower_tick,
        accounts.upper_tick,
        accounts.account_x,
        accounts.account_y,
        accounts.reserve_x,
        accounts.reserve_y,
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
    for should_be_signer in [accounts.owner] {
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
pub const UPDATE_SECONDS_PER_LIQUIDITY_IX_ACCOUNTS_LEN: usize = 10;
#[derive(Copy, Clone, Debug)]
pub struct UpdateSecondsPerLiquidityAccounts<'me, 'info> {
    pub pool: &'me AccountInfo<'info>,
    pub lower_tick: &'me AccountInfo<'info>,
    pub upper_tick: &'me AccountInfo<'info>,
    pub position: &'me AccountInfo<'info>,
    pub token_x: &'me AccountInfo<'info>,
    pub token_y: &'me AccountInfo<'info>,
    pub owner: &'me AccountInfo<'info>,
    pub signer: &'me AccountInfo<'info>,
    pub rent: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UpdateSecondsPerLiquidityKeys {
    pub pool: Pubkey,
    pub lower_tick: Pubkey,
    pub upper_tick: Pubkey,
    pub position: Pubkey,
    pub token_x: Pubkey,
    pub token_y: Pubkey,
    pub owner: Pubkey,
    pub signer: Pubkey,
    pub rent: Pubkey,
    pub system_program: Pubkey,
}
impl From<UpdateSecondsPerLiquidityAccounts<'_, '_>> for UpdateSecondsPerLiquidityKeys {
    fn from(accounts: UpdateSecondsPerLiquidityAccounts) -> Self {
        Self {
            pool: *accounts.pool.key,
            lower_tick: *accounts.lower_tick.key,
            upper_tick: *accounts.upper_tick.key,
            position: *accounts.position.key,
            token_x: *accounts.token_x.key,
            token_y: *accounts.token_y.key,
            owner: *accounts.owner.key,
            signer: *accounts.signer.key,
            rent: *accounts.rent.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<UpdateSecondsPerLiquidityKeys>
for [AccountMeta; UPDATE_SECONDS_PER_LIQUIDITY_IX_ACCOUNTS_LEN] {
    fn from(keys: UpdateSecondsPerLiquidityKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.lower_tick,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.upper_tick,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.position,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_x,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_y,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.owner,
                is_signer: false,
                is_writable: false,
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
impl From<[Pubkey; UPDATE_SECONDS_PER_LIQUIDITY_IX_ACCOUNTS_LEN]>
for UpdateSecondsPerLiquidityKeys {
    fn from(pubkeys: [Pubkey; UPDATE_SECONDS_PER_LIQUIDITY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            pool: pubkeys[0],
            lower_tick: pubkeys[1],
            upper_tick: pubkeys[2],
            position: pubkeys[3],
            token_x: pubkeys[4],
            token_y: pubkeys[5],
            owner: pubkeys[6],
            signer: pubkeys[7],
            rent: pubkeys[8],
            system_program: pubkeys[9],
        }
    }
}
impl<'info> From<UpdateSecondsPerLiquidityAccounts<'_, 'info>>
for [AccountInfo<'info>; UPDATE_SECONDS_PER_LIQUIDITY_IX_ACCOUNTS_LEN] {
    fn from(accounts: UpdateSecondsPerLiquidityAccounts<'_, 'info>) -> Self {
        [
            accounts.pool.clone(),
            accounts.lower_tick.clone(),
            accounts.upper_tick.clone(),
            accounts.position.clone(),
            accounts.token_x.clone(),
            accounts.token_y.clone(),
            accounts.owner.clone(),
            accounts.signer.clone(),
            accounts.rent.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; UPDATE_SECONDS_PER_LIQUIDITY_IX_ACCOUNTS_LEN]>
for UpdateSecondsPerLiquidityAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; UPDATE_SECONDS_PER_LIQUIDITY_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            pool: &arr[0],
            lower_tick: &arr[1],
            upper_tick: &arr[2],
            position: &arr[3],
            token_x: &arr[4],
            token_y: &arr[5],
            owner: &arr[6],
            signer: &arr[7],
            rent: &arr[8],
            system_program: &arr[9],
        }
    }
}
pub const UPDATE_SECONDS_PER_LIQUIDITY_IX_DISCM: [u8; 8usize] = [
    189, 141, 35, 129, 86, 57, 205, 219,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UpdateSecondsPerLiquidityIxArgs {
    pub lower_tick_index: i32,
    pub upper_tick_index: i32,
    pub index: i32,
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateSecondsPerLiquidityIxData(pub UpdateSecondsPerLiquidityIxArgs);
impl From<UpdateSecondsPerLiquidityIxArgs> for UpdateSecondsPerLiquidityIxData {
    fn from(args: UpdateSecondsPerLiquidityIxArgs) -> Self {
        Self(args)
    }
}
impl UpdateSecondsPerLiquidityIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_SECONDS_PER_LIQUIDITY_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let lower_tick_index: i32 = crate::borsh_de_or_default(&mut reader)?;
        let upper_tick_index: i32 = crate::borsh_de_or_default(&mut reader)?;
        let index: i32 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(UpdateSecondsPerLiquidityIxArgs {
                lower_tick_index,
                upper_tick_index,
                index,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_SECONDS_PER_LIQUIDITY_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.lower_tick_index, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.upper_tick_index, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.index, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn update_seconds_per_liquidity_ix_with_program_id(
    program_id: Pubkey,
    keys: UpdateSecondsPerLiquidityKeys,
    args: UpdateSecondsPerLiquidityIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; UPDATE_SECONDS_PER_LIQUIDITY_IX_ACCOUNTS_LEN] = keys.into();
    let data: UpdateSecondsPerLiquidityIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_seconds_per_liquidity_ix(
    keys: UpdateSecondsPerLiquidityKeys,
    args: UpdateSecondsPerLiquidityIxArgs,
) -> std::io::Result<Instruction> {
    update_seconds_per_liquidity_ix_with_program_id(INVARIANT_PROGRAM_ID, keys, args)
}
pub fn update_seconds_per_liquidity_invoke_with_program_id(
    program_id: Pubkey,
    accounts: UpdateSecondsPerLiquidityAccounts<'_, '_>,
    args: UpdateSecondsPerLiquidityIxArgs,
) -> ProgramResult {
    let keys: UpdateSecondsPerLiquidityKeys = accounts.into();
    let ix = update_seconds_per_liquidity_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn update_seconds_per_liquidity_invoke(
    accounts: UpdateSecondsPerLiquidityAccounts<'_, '_>,
    args: UpdateSecondsPerLiquidityIxArgs,
) -> ProgramResult {
    update_seconds_per_liquidity_invoke_with_program_id(
        INVARIANT_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn update_seconds_per_liquidity_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: UpdateSecondsPerLiquidityAccounts<'_, '_>,
    args: UpdateSecondsPerLiquidityIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: UpdateSecondsPerLiquidityKeys = accounts.into();
    let ix = update_seconds_per_liquidity_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn update_seconds_per_liquidity_invoke_signed(
    accounts: UpdateSecondsPerLiquidityAccounts<'_, '_>,
    args: UpdateSecondsPerLiquidityIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    update_seconds_per_liquidity_invoke_signed_with_program_id(
        INVARIANT_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn update_seconds_per_liquidity_verify_account_keys(
    accounts: UpdateSecondsPerLiquidityAccounts<'_, '_>,
    keys: UpdateSecondsPerLiquidityKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.pool.key, keys.pool),
        (*accounts.lower_tick.key, keys.lower_tick),
        (*accounts.upper_tick.key, keys.upper_tick),
        (*accounts.position.key, keys.position),
        (*accounts.token_x.key, keys.token_x),
        (*accounts.token_y.key, keys.token_y),
        (*accounts.owner.key, keys.owner),
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
pub fn update_seconds_per_liquidity_verify_writable_privileges<'me, 'info>(
    accounts: UpdateSecondsPerLiquidityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.pool, accounts.position, accounts.signer] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn update_seconds_per_liquidity_verify_signer_privileges<'me, 'info>(
    accounts: UpdateSecondsPerLiquidityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.signer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn update_seconds_per_liquidity_verify_account_privileges<'me, 'info>(
    accounts: UpdateSecondsPerLiquidityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    update_seconds_per_liquidity_verify_writable_privileges(accounts)?;
    update_seconds_per_liquidity_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const WITHDRAW_PROTOCOL_FEE_IX_ACCOUNTS_LEN: usize = 11;
#[derive(Copy, Clone, Debug)]
pub struct WithdrawProtocolFeeAccounts<'me, 'info> {
    pub state: &'me AccountInfo<'info>,
    pub pool: &'me AccountInfo<'info>,
    pub token_x: &'me AccountInfo<'info>,
    pub token_y: &'me AccountInfo<'info>,
    pub account_x: &'me AccountInfo<'info>,
    pub account_y: &'me AccountInfo<'info>,
    pub reserve_x: &'me AccountInfo<'info>,
    pub reserve_y: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub program_authority: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct WithdrawProtocolFeeKeys {
    pub state: Pubkey,
    pub pool: Pubkey,
    pub token_x: Pubkey,
    pub token_y: Pubkey,
    pub account_x: Pubkey,
    pub account_y: Pubkey,
    pub reserve_x: Pubkey,
    pub reserve_y: Pubkey,
    pub authority: Pubkey,
    pub program_authority: Pubkey,
    pub token_program: Pubkey,
}
impl From<WithdrawProtocolFeeAccounts<'_, '_>> for WithdrawProtocolFeeKeys {
    fn from(accounts: WithdrawProtocolFeeAccounts) -> Self {
        Self {
            state: *accounts.state.key,
            pool: *accounts.pool.key,
            token_x: *accounts.token_x.key,
            token_y: *accounts.token_y.key,
            account_x: *accounts.account_x.key,
            account_y: *accounts.account_y.key,
            reserve_x: *accounts.reserve_x.key,
            reserve_y: *accounts.reserve_y.key,
            authority: *accounts.authority.key,
            program_authority: *accounts.program_authority.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<WithdrawProtocolFeeKeys>
for [AccountMeta; WITHDRAW_PROTOCOL_FEE_IX_ACCOUNTS_LEN] {
    fn from(keys: WithdrawProtocolFeeKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.state,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_x,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_y,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.account_x,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.account_y,
                is_signer: false,
                is_writable: true,
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
                pubkey: keys.authority,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.program_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; WITHDRAW_PROTOCOL_FEE_IX_ACCOUNTS_LEN]> for WithdrawProtocolFeeKeys {
    fn from(pubkeys: [Pubkey; WITHDRAW_PROTOCOL_FEE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            state: pubkeys[0],
            pool: pubkeys[1],
            token_x: pubkeys[2],
            token_y: pubkeys[3],
            account_x: pubkeys[4],
            account_y: pubkeys[5],
            reserve_x: pubkeys[6],
            reserve_y: pubkeys[7],
            authority: pubkeys[8],
            program_authority: pubkeys[9],
            token_program: pubkeys[10],
        }
    }
}
impl<'info> From<WithdrawProtocolFeeAccounts<'_, 'info>>
for [AccountInfo<'info>; WITHDRAW_PROTOCOL_FEE_IX_ACCOUNTS_LEN] {
    fn from(accounts: WithdrawProtocolFeeAccounts<'_, 'info>) -> Self {
        [
            accounts.state.clone(),
            accounts.pool.clone(),
            accounts.token_x.clone(),
            accounts.token_y.clone(),
            accounts.account_x.clone(),
            accounts.account_y.clone(),
            accounts.reserve_x.clone(),
            accounts.reserve_y.clone(),
            accounts.authority.clone(),
            accounts.program_authority.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; WITHDRAW_PROTOCOL_FEE_IX_ACCOUNTS_LEN]>
for WithdrawProtocolFeeAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; WITHDRAW_PROTOCOL_FEE_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            state: &arr[0],
            pool: &arr[1],
            token_x: &arr[2],
            token_y: &arr[3],
            account_x: &arr[4],
            account_y: &arr[5],
            reserve_x: &arr[6],
            reserve_y: &arr[7],
            authority: &arr[8],
            program_authority: &arr[9],
            token_program: &arr[10],
        }
    }
}
pub const WITHDRAW_PROTOCOL_FEE_IX_DISCM: [u8; 8usize] = [
    158, 201, 158, 189, 33, 93, 162, 103,
];
#[derive(Clone, Debug, PartialEq)]
pub struct WithdrawProtocolFeeIxData;
impl WithdrawProtocolFeeIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != WITHDRAW_PROTOCOL_FEE_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&WITHDRAW_PROTOCOL_FEE_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn withdraw_protocol_fee_ix_with_program_id(
    program_id: Pubkey,
    keys: WithdrawProtocolFeeKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; WITHDRAW_PROTOCOL_FEE_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: WithdrawProtocolFeeIxData.try_to_vec()?,
    })
}
pub fn withdraw_protocol_fee_ix(
    keys: WithdrawProtocolFeeKeys,
) -> std::io::Result<Instruction> {
    withdraw_protocol_fee_ix_with_program_id(INVARIANT_PROGRAM_ID, keys)
}
pub fn withdraw_protocol_fee_invoke_with_program_id(
    program_id: Pubkey,
    accounts: WithdrawProtocolFeeAccounts<'_, '_>,
) -> ProgramResult {
    let keys: WithdrawProtocolFeeKeys = accounts.into();
    let ix = withdraw_protocol_fee_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn withdraw_protocol_fee_invoke(
    accounts: WithdrawProtocolFeeAccounts<'_, '_>,
) -> ProgramResult {
    withdraw_protocol_fee_invoke_with_program_id(INVARIANT_PROGRAM_ID, accounts)
}
pub fn withdraw_protocol_fee_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: WithdrawProtocolFeeAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: WithdrawProtocolFeeKeys = accounts.into();
    let ix = withdraw_protocol_fee_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn withdraw_protocol_fee_invoke_signed(
    accounts: WithdrawProtocolFeeAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    withdraw_protocol_fee_invoke_signed_with_program_id(
        INVARIANT_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn withdraw_protocol_fee_verify_account_keys(
    accounts: WithdrawProtocolFeeAccounts<'_, '_>,
    keys: WithdrawProtocolFeeKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.state.key, keys.state),
        (*accounts.pool.key, keys.pool),
        (*accounts.token_x.key, keys.token_x),
        (*accounts.token_y.key, keys.token_y),
        (*accounts.account_x.key, keys.account_x),
        (*accounts.account_y.key, keys.account_y),
        (*accounts.reserve_x.key, keys.reserve_x),
        (*accounts.reserve_y.key, keys.reserve_y),
        (*accounts.authority.key, keys.authority),
        (*accounts.program_authority.key, keys.program_authority),
        (*accounts.token_program.key, keys.token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn withdraw_protocol_fee_verify_writable_privileges<'me, 'info>(
    accounts: WithdrawProtocolFeeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.pool,
        accounts.account_x,
        accounts.account_y,
        accounts.reserve_x,
        accounts.reserve_y,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn withdraw_protocol_fee_verify_signer_privileges<'me, 'info>(
    accounts: WithdrawProtocolFeeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn withdraw_protocol_fee_verify_account_privileges<'me, 'info>(
    accounts: WithdrawProtocolFeeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    withdraw_protocol_fee_verify_writable_privileges(accounts)?;
    withdraw_protocol_fee_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CHANGE_PROTOCOL_FEE_IX_ACCOUNTS_LEN: usize = 6;
#[derive(Copy, Clone, Debug)]
pub struct ChangeProtocolFeeAccounts<'me, 'info> {
    pub state: &'me AccountInfo<'info>,
    pub pool: &'me AccountInfo<'info>,
    pub token_x: &'me AccountInfo<'info>,
    pub token_y: &'me AccountInfo<'info>,
    pub admin: &'me AccountInfo<'info>,
    pub program_authority: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ChangeProtocolFeeKeys {
    pub state: Pubkey,
    pub pool: Pubkey,
    pub token_x: Pubkey,
    pub token_y: Pubkey,
    pub admin: Pubkey,
    pub program_authority: Pubkey,
}
impl From<ChangeProtocolFeeAccounts<'_, '_>> for ChangeProtocolFeeKeys {
    fn from(accounts: ChangeProtocolFeeAccounts) -> Self {
        Self {
            state: *accounts.state.key,
            pool: *accounts.pool.key,
            token_x: *accounts.token_x.key,
            token_y: *accounts.token_y.key,
            admin: *accounts.admin.key,
            program_authority: *accounts.program_authority.key,
        }
    }
}
impl From<ChangeProtocolFeeKeys> for [AccountMeta; CHANGE_PROTOCOL_FEE_IX_ACCOUNTS_LEN] {
    fn from(keys: ChangeProtocolFeeKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.state,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_x,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_y,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.admin,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.program_authority,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; CHANGE_PROTOCOL_FEE_IX_ACCOUNTS_LEN]> for ChangeProtocolFeeKeys {
    fn from(pubkeys: [Pubkey; CHANGE_PROTOCOL_FEE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            state: pubkeys[0],
            pool: pubkeys[1],
            token_x: pubkeys[2],
            token_y: pubkeys[3],
            admin: pubkeys[4],
            program_authority: pubkeys[5],
        }
    }
}
impl<'info> From<ChangeProtocolFeeAccounts<'_, 'info>>
for [AccountInfo<'info>; CHANGE_PROTOCOL_FEE_IX_ACCOUNTS_LEN] {
    fn from(accounts: ChangeProtocolFeeAccounts<'_, 'info>) -> Self {
        [
            accounts.state.clone(),
            accounts.pool.clone(),
            accounts.token_x.clone(),
            accounts.token_y.clone(),
            accounts.admin.clone(),
            accounts.program_authority.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CHANGE_PROTOCOL_FEE_IX_ACCOUNTS_LEN]>
for ChangeProtocolFeeAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; CHANGE_PROTOCOL_FEE_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            state: &arr[0],
            pool: &arr[1],
            token_x: &arr[2],
            token_y: &arr[3],
            admin: &arr[4],
            program_authority: &arr[5],
        }
    }
}
pub const CHANGE_PROTOCOL_FEE_IX_DISCM: [u8; 8usize] = [
    16, 252, 253, 159, 48, 242, 32, 84,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ChangeProtocolFeeIxArgs {
    pub protocol_fee: FixedPoint,
}
#[derive(Clone, Debug, PartialEq)]
pub struct ChangeProtocolFeeIxData(pub ChangeProtocolFeeIxArgs);
impl From<ChangeProtocolFeeIxArgs> for ChangeProtocolFeeIxData {
    fn from(args: ChangeProtocolFeeIxArgs) -> Self {
        Self(args)
    }
}
impl ChangeProtocolFeeIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CHANGE_PROTOCOL_FEE_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let protocol_fee = if reader.is_empty() {
            Default::default()
        } else {
            <FixedPoint>::deserialize(&mut reader)?
        };
        Ok(
            Self(ChangeProtocolFeeIxArgs {
                protocol_fee,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CHANGE_PROTOCOL_FEE_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.protocol_fee, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn change_protocol_fee_ix_with_program_id(
    program_id: Pubkey,
    keys: ChangeProtocolFeeKeys,
    args: ChangeProtocolFeeIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CHANGE_PROTOCOL_FEE_IX_ACCOUNTS_LEN] = keys.into();
    let data: ChangeProtocolFeeIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn change_protocol_fee_ix(
    keys: ChangeProtocolFeeKeys,
    args: ChangeProtocolFeeIxArgs,
) -> std::io::Result<Instruction> {
    change_protocol_fee_ix_with_program_id(INVARIANT_PROGRAM_ID, keys, args)
}
pub fn change_protocol_fee_invoke_with_program_id(
    program_id: Pubkey,
    accounts: ChangeProtocolFeeAccounts<'_, '_>,
    args: ChangeProtocolFeeIxArgs,
) -> ProgramResult {
    let keys: ChangeProtocolFeeKeys = accounts.into();
    let ix = change_protocol_fee_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn change_protocol_fee_invoke(
    accounts: ChangeProtocolFeeAccounts<'_, '_>,
    args: ChangeProtocolFeeIxArgs,
) -> ProgramResult {
    change_protocol_fee_invoke_with_program_id(INVARIANT_PROGRAM_ID, accounts, args)
}
pub fn change_protocol_fee_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: ChangeProtocolFeeAccounts<'_, '_>,
    args: ChangeProtocolFeeIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: ChangeProtocolFeeKeys = accounts.into();
    let ix = change_protocol_fee_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn change_protocol_fee_invoke_signed(
    accounts: ChangeProtocolFeeAccounts<'_, '_>,
    args: ChangeProtocolFeeIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    change_protocol_fee_invoke_signed_with_program_id(
        INVARIANT_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn change_protocol_fee_verify_account_keys(
    accounts: ChangeProtocolFeeAccounts<'_, '_>,
    keys: ChangeProtocolFeeKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.state.key, keys.state),
        (*accounts.pool.key, keys.pool),
        (*accounts.token_x.key, keys.token_x),
        (*accounts.token_y.key, keys.token_y),
        (*accounts.admin.key, keys.admin),
        (*accounts.program_authority.key, keys.program_authority),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn change_protocol_fee_verify_writable_privileges<'me, 'info>(
    accounts: ChangeProtocolFeeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.pool] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn change_protocol_fee_verify_signer_privileges<'me, 'info>(
    accounts: ChangeProtocolFeeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn change_protocol_fee_verify_account_privileges<'me, 'info>(
    accounts: ChangeProtocolFeeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    change_protocol_fee_verify_writable_privileges(accounts)?;
    change_protocol_fee_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CHANGE_FEE_RECEIVER_IX_ACCOUNTS_LEN: usize = 6;
#[derive(Copy, Clone, Debug)]
pub struct ChangeFeeReceiverAccounts<'me, 'info> {
    pub state: &'me AccountInfo<'info>,
    pub pool: &'me AccountInfo<'info>,
    pub token_x: &'me AccountInfo<'info>,
    pub token_y: &'me AccountInfo<'info>,
    pub admin: &'me AccountInfo<'info>,
    pub fee_receiver: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ChangeFeeReceiverKeys {
    pub state: Pubkey,
    pub pool: Pubkey,
    pub token_x: Pubkey,
    pub token_y: Pubkey,
    pub admin: Pubkey,
    pub fee_receiver: Pubkey,
}
impl From<ChangeFeeReceiverAccounts<'_, '_>> for ChangeFeeReceiverKeys {
    fn from(accounts: ChangeFeeReceiverAccounts) -> Self {
        Self {
            state: *accounts.state.key,
            pool: *accounts.pool.key,
            token_x: *accounts.token_x.key,
            token_y: *accounts.token_y.key,
            admin: *accounts.admin.key,
            fee_receiver: *accounts.fee_receiver.key,
        }
    }
}
impl From<ChangeFeeReceiverKeys> for [AccountMeta; CHANGE_FEE_RECEIVER_IX_ACCOUNTS_LEN] {
    fn from(keys: ChangeFeeReceiverKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.state,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_x,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_y,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.admin,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.fee_receiver,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; CHANGE_FEE_RECEIVER_IX_ACCOUNTS_LEN]> for ChangeFeeReceiverKeys {
    fn from(pubkeys: [Pubkey; CHANGE_FEE_RECEIVER_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            state: pubkeys[0],
            pool: pubkeys[1],
            token_x: pubkeys[2],
            token_y: pubkeys[3],
            admin: pubkeys[4],
            fee_receiver: pubkeys[5],
        }
    }
}
impl<'info> From<ChangeFeeReceiverAccounts<'_, 'info>>
for [AccountInfo<'info>; CHANGE_FEE_RECEIVER_IX_ACCOUNTS_LEN] {
    fn from(accounts: ChangeFeeReceiverAccounts<'_, 'info>) -> Self {
        [
            accounts.state.clone(),
            accounts.pool.clone(),
            accounts.token_x.clone(),
            accounts.token_y.clone(),
            accounts.admin.clone(),
            accounts.fee_receiver.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CHANGE_FEE_RECEIVER_IX_ACCOUNTS_LEN]>
for ChangeFeeReceiverAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; CHANGE_FEE_RECEIVER_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            state: &arr[0],
            pool: &arr[1],
            token_x: &arr[2],
            token_y: &arr[3],
            admin: &arr[4],
            fee_receiver: &arr[5],
        }
    }
}
pub const CHANGE_FEE_RECEIVER_IX_DISCM: [u8; 8usize] = [
    92, 14, 241, 152, 58, 92, 188, 57,
];
#[derive(Clone, Debug, PartialEq)]
pub struct ChangeFeeReceiverIxData;
impl ChangeFeeReceiverIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CHANGE_FEE_RECEIVER_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CHANGE_FEE_RECEIVER_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn change_fee_receiver_ix_with_program_id(
    program_id: Pubkey,
    keys: ChangeFeeReceiverKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CHANGE_FEE_RECEIVER_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: ChangeFeeReceiverIxData.try_to_vec()?,
    })
}
pub fn change_fee_receiver_ix(
    keys: ChangeFeeReceiverKeys,
) -> std::io::Result<Instruction> {
    change_fee_receiver_ix_with_program_id(INVARIANT_PROGRAM_ID, keys)
}
pub fn change_fee_receiver_invoke_with_program_id(
    program_id: Pubkey,
    accounts: ChangeFeeReceiverAccounts<'_, '_>,
) -> ProgramResult {
    let keys: ChangeFeeReceiverKeys = accounts.into();
    let ix = change_fee_receiver_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn change_fee_receiver_invoke(
    accounts: ChangeFeeReceiverAccounts<'_, '_>,
) -> ProgramResult {
    change_fee_receiver_invoke_with_program_id(INVARIANT_PROGRAM_ID, accounts)
}
pub fn change_fee_receiver_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: ChangeFeeReceiverAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: ChangeFeeReceiverKeys = accounts.into();
    let ix = change_fee_receiver_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn change_fee_receiver_invoke_signed(
    accounts: ChangeFeeReceiverAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    change_fee_receiver_invoke_signed_with_program_id(
        INVARIANT_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn change_fee_receiver_verify_account_keys(
    accounts: ChangeFeeReceiverAccounts<'_, '_>,
    keys: ChangeFeeReceiverKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.state.key, keys.state),
        (*accounts.pool.key, keys.pool),
        (*accounts.token_x.key, keys.token_x),
        (*accounts.token_y.key, keys.token_y),
        (*accounts.admin.key, keys.admin),
        (*accounts.fee_receiver.key, keys.fee_receiver),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn change_fee_receiver_verify_writable_privileges<'me, 'info>(
    accounts: ChangeFeeReceiverAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.pool] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn change_fee_receiver_verify_signer_privileges<'me, 'info>(
    accounts: ChangeFeeReceiverAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn change_fee_receiver_verify_account_privileges<'me, 'info>(
    accounts: ChangeFeeReceiverAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    change_fee_receiver_verify_writable_privileges(accounts)?;
    change_fee_receiver_verify_signer_privileges(accounts)?;
    Ok(())
}
