use solana_pubkey::Pubkey;
use solana_cpi::{invoke, invoke_signed};
use solana_instruction::{AccountMeta, Instruction};
use solana_account_info::AccountInfo;
use solana_program_error::ProgramError;
use std::io::Read;
#[allow(unused_imports)]
use crate::*;
#[derive(Clone, Debug, PartialEq)]
pub enum CykuraProgramIx {
    InitFactory,
    SetOwner,
    EnableFeeAmount(EnableFeeAmountIxArgs),
    CreateAndInitPool(CreateAndInitPoolIxArgs),
    IncreaseObservationCardinalityNext(IncreaseObservationCardinalityNextIxArgs),
    SetFeeProtocol(SetFeeProtocolIxArgs),
    CollectProtocol(CollectProtocolIxArgs),
    InitTickAccount(InitTickAccountIxArgs),
    CloseTickAccount,
    InitBitmapAccount(InitBitmapAccountIxArgs),
    InitPositionAccount,
    MintCallback(MintCallbackIxArgs),
    SwapCallback(SwapCallbackIxArgs),
    Mint(MintIxArgs),
    Burn(BurnIxArgs),
    Collect(CollectIxArgs),
    Swap(SwapIxArgs),
    MintTokenizedPosition(MintTokenizedPositionIxArgs),
    AddMetaplexMetadata,
    IncreaseLiquidity(IncreaseLiquidityIxArgs),
    DecreaseLiquidity(DecreaseLiquidityIxArgs),
    CollectFromTokenized(CollectFromTokenizedIxArgs),
    ExactInputSingle(ExactInputSingleIxArgs),
    ExactInput(ExactInputIxArgs),
}
impl CykuraProgramIx {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        if buf.starts_with(&INIT_FACTORY_IX_DISCM) {
            return Ok(Self::InitFactory);
        }
        if buf.starts_with(&SET_OWNER_IX_DISCM) {
            return Ok(Self::SetOwner);
        }
        if buf.starts_with(&ENABLE_FEE_AMOUNT_IX_DISCM) {
            let mut reader = &buf[ENABLE_FEE_AMOUNT_IX_DISCM.len()..];
            let fee: u32 = crate::borsh_de_or_default(&mut reader)?;
            let tick_spacing: u16 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::EnableFeeAmount(EnableFeeAmountIxArgs {
                    fee,
                    tick_spacing,
                }),
            );
        }
        if buf.starts_with(&CREATE_AND_INIT_POOL_IX_DISCM) {
            let mut reader = &buf[CREATE_AND_INIT_POOL_IX_DISCM.len()..];
            let sqrt_price_x32: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::CreateAndInitPool(CreateAndInitPoolIxArgs {
                    sqrt_price_x32,
                }),
            );
        }
        if buf.starts_with(&INCREASE_OBSERVATION_CARDINALITY_NEXT_IX_DISCM) {
            let mut reader = &buf[INCREASE_OBSERVATION_CARDINALITY_NEXT_IX_DISCM
                .len()..];
            let observation_account_bumps: Vec<u8> = crate::borsh_de_or_default(
                &mut reader,
            )?;
            return Ok(
                Self::IncreaseObservationCardinalityNext(IncreaseObservationCardinalityNextIxArgs {
                    observation_account_bumps,
                }),
            );
        }
        if buf.starts_with(&SET_FEE_PROTOCOL_IX_DISCM) {
            let mut reader = &buf[SET_FEE_PROTOCOL_IX_DISCM.len()..];
            let fee_protocol: u8 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::SetFeeProtocol(SetFeeProtocolIxArgs {
                    fee_protocol,
                }),
            );
        }
        if buf.starts_with(&COLLECT_PROTOCOL_IX_DISCM) {
            let mut reader = &buf[COLLECT_PROTOCOL_IX_DISCM.len()..];
            let amount0_requested: u64 = crate::borsh_de_or_default(&mut reader)?;
            let amount1_requested: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::CollectProtocol(CollectProtocolIxArgs {
                    amount0_requested,
                    amount1_requested,
                }),
            );
        }
        if buf.starts_with(&INIT_TICK_ACCOUNT_IX_DISCM) {
            let mut reader = &buf[INIT_TICK_ACCOUNT_IX_DISCM.len()..];
            let tick: i32 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(Self::InitTickAccount(InitTickAccountIxArgs { tick }));
        }
        if buf.starts_with(&CLOSE_TICK_ACCOUNT_IX_DISCM) {
            return Ok(Self::CloseTickAccount);
        }
        if buf.starts_with(&INIT_BITMAP_ACCOUNT_IX_DISCM) {
            let mut reader = &buf[INIT_BITMAP_ACCOUNT_IX_DISCM.len()..];
            let word_pos: i16 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::InitBitmapAccount(InitBitmapAccountIxArgs {
                    word_pos,
                }),
            );
        }
        if buf.starts_with(&INIT_POSITION_ACCOUNT_IX_DISCM) {
            return Ok(Self::InitPositionAccount);
        }
        if buf.starts_with(&MINT_CALLBACK_IX_DISCM) {
            let mut reader = &buf[MINT_CALLBACK_IX_DISCM.len()..];
            let amount0_owed: u64 = crate::borsh_de_or_default(&mut reader)?;
            let amount1_owed: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::MintCallback(MintCallbackIxArgs {
                    amount0_owed,
                    amount1_owed,
                }),
            );
        }
        if buf.starts_with(&SWAP_CALLBACK_IX_DISCM) {
            let mut reader = &buf[SWAP_CALLBACK_IX_DISCM.len()..];
            let amount0_delta: i64 = crate::borsh_de_or_default(&mut reader)?;
            let amount1_delta: i64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::SwapCallback(SwapCallbackIxArgs {
                    amount0_delta,
                    amount1_delta,
                }),
            );
        }
        if buf.starts_with(&MINT_IX_DISCM) {
            let mut reader = &buf[MINT_IX_DISCM.len()..];
            let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(Self::Mint(MintIxArgs { amount }));
        }
        if buf.starts_with(&BURN_IX_DISCM) {
            let mut reader = &buf[BURN_IX_DISCM.len()..];
            let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(Self::Burn(BurnIxArgs { amount }));
        }
        if buf.starts_with(&COLLECT_IX_DISCM) {
            let mut reader = &buf[COLLECT_IX_DISCM.len()..];
            let amount0_requested: u64 = crate::borsh_de_or_default(&mut reader)?;
            let amount1_requested: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::Collect(CollectIxArgs {
                    amount0_requested,
                    amount1_requested,
                }),
            );
        }
        if buf.starts_with(&SWAP_IX_DISCM) {
            let mut reader = &buf[SWAP_IX_DISCM.len()..];
            let amount_specified: i64 = crate::borsh_de_or_default(&mut reader)?;
            let sqrt_price_limit_x32: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::Swap(SwapIxArgs {
                    amount_specified,
                    sqrt_price_limit_x32,
                }),
            );
        }
        if buf.starts_with(&MINT_TOKENIZED_POSITION_IX_DISCM) {
            let mut reader = &buf[MINT_TOKENIZED_POSITION_IX_DISCM.len()..];
            let amount0_desired: u64 = crate::borsh_de_or_default(&mut reader)?;
            let amount1_desired: u64 = crate::borsh_de_or_default(&mut reader)?;
            let amount0_min: u64 = crate::borsh_de_or_default(&mut reader)?;
            let amount1_min: u64 = crate::borsh_de_or_default(&mut reader)?;
            let deadline: i64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::MintTokenizedPosition(MintTokenizedPositionIxArgs {
                    amount0_desired,
                    amount1_desired,
                    amount0_min,
                    amount1_min,
                    deadline,
                }),
            );
        }
        if buf.starts_with(&ADD_METAPLEX_METADATA_IX_DISCM) {
            return Ok(Self::AddMetaplexMetadata);
        }
        if buf.starts_with(&INCREASE_LIQUIDITY_IX_DISCM) {
            let mut reader = &buf[INCREASE_LIQUIDITY_IX_DISCM.len()..];
            let amount0_desired: u64 = crate::borsh_de_or_default(&mut reader)?;
            let amount1_desired: u64 = crate::borsh_de_or_default(&mut reader)?;
            let amount0_min: u64 = crate::borsh_de_or_default(&mut reader)?;
            let amount1_min: u64 = crate::borsh_de_or_default(&mut reader)?;
            let deadline: i64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::IncreaseLiquidity(IncreaseLiquidityIxArgs {
                    amount0_desired,
                    amount1_desired,
                    amount0_min,
                    amount1_min,
                    deadline,
                }),
            );
        }
        if buf.starts_with(&DECREASE_LIQUIDITY_IX_DISCM) {
            let mut reader = &buf[DECREASE_LIQUIDITY_IX_DISCM.len()..];
            let liquidity: u64 = crate::borsh_de_or_default(&mut reader)?;
            let amount0_min: u64 = crate::borsh_de_or_default(&mut reader)?;
            let amount1_min: u64 = crate::borsh_de_or_default(&mut reader)?;
            let deadline: i64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::DecreaseLiquidity(DecreaseLiquidityIxArgs {
                    liquidity,
                    amount0_min,
                    amount1_min,
                    deadline,
                }),
            );
        }
        if buf.starts_with(&COLLECT_FROM_TOKENIZED_IX_DISCM) {
            let mut reader = &buf[COLLECT_FROM_TOKENIZED_IX_DISCM.len()..];
            let amount0_max: u64 = crate::borsh_de_or_default(&mut reader)?;
            let amount1_max: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::CollectFromTokenized(CollectFromTokenizedIxArgs {
                    amount0_max,
                    amount1_max,
                }),
            );
        }
        if buf.starts_with(&EXACT_INPUT_SINGLE_IX_DISCM) {
            let mut reader = &buf[EXACT_INPUT_SINGLE_IX_DISCM.len()..];
            let deadline: i64 = crate::borsh_de_or_default(&mut reader)?;
            let amount_in: u64 = crate::borsh_de_or_default(&mut reader)?;
            let amount_out_minimum: u64 = crate::borsh_de_or_default(&mut reader)?;
            let sqrt_price_limit_x32: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::ExactInputSingle(ExactInputSingleIxArgs {
                    deadline,
                    amount_in,
                    amount_out_minimum,
                    sqrt_price_limit_x32,
                }),
            );
        }
        if buf.starts_with(&EXACT_INPUT_IX_DISCM) {
            let mut reader = &buf[EXACT_INPUT_IX_DISCM.len()..];
            let deadline: i64 = crate::borsh_de_or_default(&mut reader)?;
            let amount_in: u64 = crate::borsh_de_or_default(&mut reader)?;
            let amount_out_minimum: u64 = crate::borsh_de_or_default(&mut reader)?;
            let additional_accounts_per_pool: Vec<u8> = crate::borsh_de_or_default(
                &mut reader,
            )?;
            return Ok(
                Self::ExactInput(ExactInputIxArgs {
                    deadline,
                    amount_in,
                    amount_out_minimum,
                    additional_accounts_per_pool,
                }),
            );
        }
        Err(std::io::Error::from(std::io::ErrorKind::InvalidData))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        match self {
            Self::InitFactory => writer.write_all(&INIT_FACTORY_IX_DISCM),
            Self::SetOwner => writer.write_all(&SET_OWNER_IX_DISCM),
            Self::EnableFeeAmount(args) => {
                writer.write_all(&ENABLE_FEE_AMOUNT_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.fee, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.tick_spacing, &mut writer)?;
                Ok(())
            }
            Self::CreateAndInitPool(args) => {
                writer.write_all(&CREATE_AND_INIT_POOL_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.sqrt_price_x32, &mut writer)?;
                Ok(())
            }
            Self::IncreaseObservationCardinalityNext(args) => {
                writer.write_all(&INCREASE_OBSERVATION_CARDINALITY_NEXT_IX_DISCM)?;
                borsh::BorshSerialize::serialize(
                    &args.observation_account_bumps,
                    &mut writer,
                )?;
                Ok(())
            }
            Self::SetFeeProtocol(args) => {
                writer.write_all(&SET_FEE_PROTOCOL_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.fee_protocol, &mut writer)?;
                Ok(())
            }
            Self::CollectProtocol(args) => {
                writer.write_all(&COLLECT_PROTOCOL_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.amount0_requested, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.amount1_requested, &mut writer)?;
                Ok(())
            }
            Self::InitTickAccount(args) => {
                writer.write_all(&INIT_TICK_ACCOUNT_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.tick, &mut writer)?;
                Ok(())
            }
            Self::CloseTickAccount => writer.write_all(&CLOSE_TICK_ACCOUNT_IX_DISCM),
            Self::InitBitmapAccount(args) => {
                writer.write_all(&INIT_BITMAP_ACCOUNT_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.word_pos, &mut writer)?;
                Ok(())
            }
            Self::InitPositionAccount => {
                writer.write_all(&INIT_POSITION_ACCOUNT_IX_DISCM)
            }
            Self::MintCallback(args) => {
                writer.write_all(&MINT_CALLBACK_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.amount0_owed, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.amount1_owed, &mut writer)?;
                Ok(())
            }
            Self::SwapCallback(args) => {
                writer.write_all(&SWAP_CALLBACK_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.amount0_delta, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.amount1_delta, &mut writer)?;
                Ok(())
            }
            Self::Mint(args) => {
                writer.write_all(&MINT_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.amount, &mut writer)?;
                Ok(())
            }
            Self::Burn(args) => {
                writer.write_all(&BURN_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.amount, &mut writer)?;
                Ok(())
            }
            Self::Collect(args) => {
                writer.write_all(&COLLECT_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.amount0_requested, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.amount1_requested, &mut writer)?;
                Ok(())
            }
            Self::Swap(args) => {
                writer.write_all(&SWAP_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.amount_specified, &mut writer)?;
                borsh::BorshSerialize::serialize(
                    &args.sqrt_price_limit_x32,
                    &mut writer,
                )?;
                Ok(())
            }
            Self::MintTokenizedPosition(args) => {
                writer.write_all(&MINT_TOKENIZED_POSITION_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.amount0_desired, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.amount1_desired, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.amount0_min, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.amount1_min, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.deadline, &mut writer)?;
                Ok(())
            }
            Self::AddMetaplexMetadata => {
                writer.write_all(&ADD_METAPLEX_METADATA_IX_DISCM)
            }
            Self::IncreaseLiquidity(args) => {
                writer.write_all(&INCREASE_LIQUIDITY_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.amount0_desired, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.amount1_desired, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.amount0_min, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.amount1_min, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.deadline, &mut writer)?;
                Ok(())
            }
            Self::DecreaseLiquidity(args) => {
                writer.write_all(&DECREASE_LIQUIDITY_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.liquidity, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.amount0_min, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.amount1_min, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.deadline, &mut writer)?;
                Ok(())
            }
            Self::CollectFromTokenized(args) => {
                writer.write_all(&COLLECT_FROM_TOKENIZED_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.amount0_max, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.amount1_max, &mut writer)?;
                Ok(())
            }
            Self::ExactInputSingle(args) => {
                writer.write_all(&EXACT_INPUT_SINGLE_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.deadline, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.amount_in, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.amount_out_minimum, &mut writer)?;
                borsh::BorshSerialize::serialize(
                    &args.sqrt_price_limit_x32,
                    &mut writer,
                )?;
                Ok(())
            }
            Self::ExactInput(args) => {
                writer.write_all(&EXACT_INPUT_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.deadline, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.amount_in, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.amount_out_minimum, &mut writer)?;
                borsh::BorshSerialize::serialize(
                    &args.additional_accounts_per_pool,
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
pub const INIT_FACTORY_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct InitFactoryAccounts<'me, 'info> {
    pub owner: &'me AccountInfo<'info>,
    pub factory_state: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct InitFactoryKeys {
    pub owner: Pubkey,
    pub factory_state: Pubkey,
    pub system_program: Pubkey,
}
impl From<InitFactoryAccounts<'_, '_>> for InitFactoryKeys {
    fn from(accounts: InitFactoryAccounts) -> Self {
        Self {
            owner: *accounts.owner.key,
            factory_state: *accounts.factory_state.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<InitFactoryKeys> for [AccountMeta; INIT_FACTORY_IX_ACCOUNTS_LEN] {
    fn from(keys: InitFactoryKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.owner,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.factory_state,
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
impl From<[Pubkey; INIT_FACTORY_IX_ACCOUNTS_LEN]> for InitFactoryKeys {
    fn from(pubkeys: [Pubkey; INIT_FACTORY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            owner: pubkeys[0],
            factory_state: pubkeys[1],
            system_program: pubkeys[2],
        }
    }
}
impl<'info> From<InitFactoryAccounts<'_, 'info>>
for [AccountInfo<'info>; INIT_FACTORY_IX_ACCOUNTS_LEN] {
    fn from(accounts: InitFactoryAccounts<'_, 'info>) -> Self {
        [
            accounts.owner.clone(),
            accounts.factory_state.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; INIT_FACTORY_IX_ACCOUNTS_LEN]>
for InitFactoryAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; INIT_FACTORY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            owner: &arr[0],
            factory_state: &arr[1],
            system_program: &arr[2],
        }
    }
}
pub const INIT_FACTORY_IX_DISCM: [u8; 8usize] = [65, 136, 219, 177, 234, 197, 24, 39];
#[derive(Clone, Debug, PartialEq)]
pub struct InitFactoryIxData;
impl InitFactoryIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INIT_FACTORY_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INIT_FACTORY_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn init_factory_ix_with_program_id(
    program_id: Pubkey,
    keys: InitFactoryKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; INIT_FACTORY_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: InitFactoryIxData.try_to_vec()?,
    })
}
pub fn init_factory_ix(keys: InitFactoryKeys) -> std::io::Result<Instruction> {
    init_factory_ix_with_program_id(CYKURA_PROGRAM_ID, keys)
}
pub fn init_factory_invoke_with_program_id(
    program_id: Pubkey,
    accounts: InitFactoryAccounts<'_, '_>,
) -> ProgramResult {
    let keys: InitFactoryKeys = accounts.into();
    let ix = init_factory_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn init_factory_invoke(accounts: InitFactoryAccounts<'_, '_>) -> ProgramResult {
    init_factory_invoke_with_program_id(CYKURA_PROGRAM_ID, accounts)
}
pub fn init_factory_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: InitFactoryAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: InitFactoryKeys = accounts.into();
    let ix = init_factory_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn init_factory_invoke_signed(
    accounts: InitFactoryAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    init_factory_invoke_signed_with_program_id(CYKURA_PROGRAM_ID, accounts, seeds)
}
pub fn init_factory_verify_account_keys(
    accounts: InitFactoryAccounts<'_, '_>,
    keys: InitFactoryKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.owner.key, keys.owner),
        (*accounts.factory_state.key, keys.factory_state),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn init_factory_verify_writable_privileges<'me, 'info>(
    accounts: InitFactoryAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.owner, accounts.factory_state] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn init_factory_verify_signer_privileges<'me, 'info>(
    accounts: InitFactoryAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.owner] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn init_factory_verify_account_privileges<'me, 'info>(
    accounts: InitFactoryAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    init_factory_verify_writable_privileges(accounts)?;
    init_factory_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SET_OWNER_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct SetOwnerAccounts<'me, 'info> {
    pub owner: &'me AccountInfo<'info>,
    pub new_owner: &'me AccountInfo<'info>,
    pub factory_state: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SetOwnerKeys {
    pub owner: Pubkey,
    pub new_owner: Pubkey,
    pub factory_state: Pubkey,
}
impl From<SetOwnerAccounts<'_, '_>> for SetOwnerKeys {
    fn from(accounts: SetOwnerAccounts) -> Self {
        Self {
            owner: *accounts.owner.key,
            new_owner: *accounts.new_owner.key,
            factory_state: *accounts.factory_state.key,
        }
    }
}
impl From<SetOwnerKeys> for [AccountMeta; SET_OWNER_IX_ACCOUNTS_LEN] {
    fn from(keys: SetOwnerKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.owner,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.new_owner,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.factory_state,
                is_signer: false,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; SET_OWNER_IX_ACCOUNTS_LEN]> for SetOwnerKeys {
    fn from(pubkeys: [Pubkey; SET_OWNER_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            owner: pubkeys[0],
            new_owner: pubkeys[1],
            factory_state: pubkeys[2],
        }
    }
}
impl<'info> From<SetOwnerAccounts<'_, 'info>>
for [AccountInfo<'info>; SET_OWNER_IX_ACCOUNTS_LEN] {
    fn from(accounts: SetOwnerAccounts<'_, 'info>) -> Self {
        [
            accounts.owner.clone(),
            accounts.new_owner.clone(),
            accounts.factory_state.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SET_OWNER_IX_ACCOUNTS_LEN]>
for SetOwnerAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; SET_OWNER_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            owner: &arr[0],
            new_owner: &arr[1],
            factory_state: &arr[2],
        }
    }
}
pub const SET_OWNER_IX_DISCM: [u8; 8usize] = [72, 202, 120, 52, 77, 128, 96, 197];
#[derive(Clone, Debug, PartialEq)]
pub struct SetOwnerIxData;
impl SetOwnerIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SET_OWNER_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SET_OWNER_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn set_owner_ix_with_program_id(
    program_id: Pubkey,
    keys: SetOwnerKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SET_OWNER_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: SetOwnerIxData.try_to_vec()?,
    })
}
pub fn set_owner_ix(keys: SetOwnerKeys) -> std::io::Result<Instruction> {
    set_owner_ix_with_program_id(CYKURA_PROGRAM_ID, keys)
}
pub fn set_owner_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SetOwnerAccounts<'_, '_>,
) -> ProgramResult {
    let keys: SetOwnerKeys = accounts.into();
    let ix = set_owner_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn set_owner_invoke(accounts: SetOwnerAccounts<'_, '_>) -> ProgramResult {
    set_owner_invoke_with_program_id(CYKURA_PROGRAM_ID, accounts)
}
pub fn set_owner_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SetOwnerAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SetOwnerKeys = accounts.into();
    let ix = set_owner_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn set_owner_invoke_signed(
    accounts: SetOwnerAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    set_owner_invoke_signed_with_program_id(CYKURA_PROGRAM_ID, accounts, seeds)
}
pub fn set_owner_verify_account_keys(
    accounts: SetOwnerAccounts<'_, '_>,
    keys: SetOwnerKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.owner.key, keys.owner),
        (*accounts.new_owner.key, keys.new_owner),
        (*accounts.factory_state.key, keys.factory_state),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn set_owner_verify_writable_privileges<'me, 'info>(
    accounts: SetOwnerAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.factory_state] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn set_owner_verify_signer_privileges<'me, 'info>(
    accounts: SetOwnerAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.owner] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn set_owner_verify_account_privileges<'me, 'info>(
    accounts: SetOwnerAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    set_owner_verify_writable_privileges(accounts)?;
    set_owner_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const ENABLE_FEE_AMOUNT_IX_ACCOUNTS_LEN: usize = 4;
#[derive(Copy, Clone, Debug)]
pub struct EnableFeeAmountAccounts<'me, 'info> {
    pub owner: &'me AccountInfo<'info>,
    pub factory_state: &'me AccountInfo<'info>,
    pub fee_state: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct EnableFeeAmountKeys {
    pub owner: Pubkey,
    pub factory_state: Pubkey,
    pub fee_state: Pubkey,
    pub system_program: Pubkey,
}
impl From<EnableFeeAmountAccounts<'_, '_>> for EnableFeeAmountKeys {
    fn from(accounts: EnableFeeAmountAccounts) -> Self {
        Self {
            owner: *accounts.owner.key,
            factory_state: *accounts.factory_state.key,
            fee_state: *accounts.fee_state.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<EnableFeeAmountKeys> for [AccountMeta; ENABLE_FEE_AMOUNT_IX_ACCOUNTS_LEN] {
    fn from(keys: EnableFeeAmountKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.owner,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.factory_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.fee_state,
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
impl From<[Pubkey; ENABLE_FEE_AMOUNT_IX_ACCOUNTS_LEN]> for EnableFeeAmountKeys {
    fn from(pubkeys: [Pubkey; ENABLE_FEE_AMOUNT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            owner: pubkeys[0],
            factory_state: pubkeys[1],
            fee_state: pubkeys[2],
            system_program: pubkeys[3],
        }
    }
}
impl<'info> From<EnableFeeAmountAccounts<'_, 'info>>
for [AccountInfo<'info>; ENABLE_FEE_AMOUNT_IX_ACCOUNTS_LEN] {
    fn from(accounts: EnableFeeAmountAccounts<'_, 'info>) -> Self {
        [
            accounts.owner.clone(),
            accounts.factory_state.clone(),
            accounts.fee_state.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; ENABLE_FEE_AMOUNT_IX_ACCOUNTS_LEN]>
for EnableFeeAmountAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; ENABLE_FEE_AMOUNT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            owner: &arr[0],
            factory_state: &arr[1],
            fee_state: &arr[2],
            system_program: &arr[3],
        }
    }
}
pub const ENABLE_FEE_AMOUNT_IX_DISCM: [u8; 8usize] = [
    187, 207, 166, 107, 160, 19, 109, 252,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct EnableFeeAmountIxArgs {
    pub fee: u32,
    pub tick_spacing: u16,
}
#[derive(Clone, Debug, PartialEq)]
pub struct EnableFeeAmountIxData(pub EnableFeeAmountIxArgs);
impl From<EnableFeeAmountIxArgs> for EnableFeeAmountIxData {
    fn from(args: EnableFeeAmountIxArgs) -> Self {
        Self(args)
    }
}
impl EnableFeeAmountIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != ENABLE_FEE_AMOUNT_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let fee: u32 = crate::borsh_de_or_default(&mut reader)?;
        let tick_spacing: u16 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(EnableFeeAmountIxArgs {
                fee,
                tick_spacing,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&ENABLE_FEE_AMOUNT_IX_DISCM)?;
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
pub fn enable_fee_amount_ix_with_program_id(
    program_id: Pubkey,
    keys: EnableFeeAmountKeys,
    args: EnableFeeAmountIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; ENABLE_FEE_AMOUNT_IX_ACCOUNTS_LEN] = keys.into();
    let data: EnableFeeAmountIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn enable_fee_amount_ix(
    keys: EnableFeeAmountKeys,
    args: EnableFeeAmountIxArgs,
) -> std::io::Result<Instruction> {
    enable_fee_amount_ix_with_program_id(CYKURA_PROGRAM_ID, keys, args)
}
pub fn enable_fee_amount_invoke_with_program_id(
    program_id: Pubkey,
    accounts: EnableFeeAmountAccounts<'_, '_>,
    args: EnableFeeAmountIxArgs,
) -> ProgramResult {
    let keys: EnableFeeAmountKeys = accounts.into();
    let ix = enable_fee_amount_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn enable_fee_amount_invoke(
    accounts: EnableFeeAmountAccounts<'_, '_>,
    args: EnableFeeAmountIxArgs,
) -> ProgramResult {
    enable_fee_amount_invoke_with_program_id(CYKURA_PROGRAM_ID, accounts, args)
}
pub fn enable_fee_amount_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: EnableFeeAmountAccounts<'_, '_>,
    args: EnableFeeAmountIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: EnableFeeAmountKeys = accounts.into();
    let ix = enable_fee_amount_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn enable_fee_amount_invoke_signed(
    accounts: EnableFeeAmountAccounts<'_, '_>,
    args: EnableFeeAmountIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    enable_fee_amount_invoke_signed_with_program_id(
        CYKURA_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn enable_fee_amount_verify_account_keys(
    accounts: EnableFeeAmountAccounts<'_, '_>,
    keys: EnableFeeAmountKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.owner.key, keys.owner),
        (*accounts.factory_state.key, keys.factory_state),
        (*accounts.fee_state.key, keys.fee_state),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn enable_fee_amount_verify_writable_privileges<'me, 'info>(
    accounts: EnableFeeAmountAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.owner,
        accounts.factory_state,
        accounts.fee_state,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn enable_fee_amount_verify_signer_privileges<'me, 'info>(
    accounts: EnableFeeAmountAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.owner] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn enable_fee_amount_verify_account_privileges<'me, 'info>(
    accounts: EnableFeeAmountAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    enable_fee_amount_verify_writable_privileges(accounts)?;
    enable_fee_amount_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CREATE_AND_INIT_POOL_IX_ACCOUNTS_LEN: usize = 8;
#[derive(Copy, Clone, Debug)]
pub struct CreateAndInitPoolAccounts<'me, 'info> {
    pub pool_creator: &'me AccountInfo<'info>,
    pub token0: &'me AccountInfo<'info>,
    pub token1: &'me AccountInfo<'info>,
    pub fee_state: &'me AccountInfo<'info>,
    pub pool_state: &'me AccountInfo<'info>,
    pub initial_observation_state: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub rent: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CreateAndInitPoolKeys {
    pub pool_creator: Pubkey,
    pub token0: Pubkey,
    pub token1: Pubkey,
    pub fee_state: Pubkey,
    pub pool_state: Pubkey,
    pub initial_observation_state: Pubkey,
    pub system_program: Pubkey,
    pub rent: Pubkey,
}
impl From<CreateAndInitPoolAccounts<'_, '_>> for CreateAndInitPoolKeys {
    fn from(accounts: CreateAndInitPoolAccounts) -> Self {
        Self {
            pool_creator: *accounts.pool_creator.key,
            token0: *accounts.token0.key,
            token1: *accounts.token1.key,
            fee_state: *accounts.fee_state.key,
            pool_state: *accounts.pool_state.key,
            initial_observation_state: *accounts.initial_observation_state.key,
            system_program: *accounts.system_program.key,
            rent: *accounts.rent.key,
        }
    }
}
impl From<CreateAndInitPoolKeys>
for [AccountMeta; CREATE_AND_INIT_POOL_IX_ACCOUNTS_LEN] {
    fn from(keys: CreateAndInitPoolKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.pool_creator,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token0,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token1,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.fee_state,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.initial_observation_state,
                is_signer: false,
                is_writable: true,
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
impl From<[Pubkey; CREATE_AND_INIT_POOL_IX_ACCOUNTS_LEN]> for CreateAndInitPoolKeys {
    fn from(pubkeys: [Pubkey; CREATE_AND_INIT_POOL_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            pool_creator: pubkeys[0],
            token0: pubkeys[1],
            token1: pubkeys[2],
            fee_state: pubkeys[3],
            pool_state: pubkeys[4],
            initial_observation_state: pubkeys[5],
            system_program: pubkeys[6],
            rent: pubkeys[7],
        }
    }
}
impl<'info> From<CreateAndInitPoolAccounts<'_, 'info>>
for [AccountInfo<'info>; CREATE_AND_INIT_POOL_IX_ACCOUNTS_LEN] {
    fn from(accounts: CreateAndInitPoolAccounts<'_, 'info>) -> Self {
        [
            accounts.pool_creator.clone(),
            accounts.token0.clone(),
            accounts.token1.clone(),
            accounts.fee_state.clone(),
            accounts.pool_state.clone(),
            accounts.initial_observation_state.clone(),
            accounts.system_program.clone(),
            accounts.rent.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CREATE_AND_INIT_POOL_IX_ACCOUNTS_LEN]>
for CreateAndInitPoolAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; CREATE_AND_INIT_POOL_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            pool_creator: &arr[0],
            token0: &arr[1],
            token1: &arr[2],
            fee_state: &arr[3],
            pool_state: &arr[4],
            initial_observation_state: &arr[5],
            system_program: &arr[6],
            rent: &arr[7],
        }
    }
}
pub const CREATE_AND_INIT_POOL_IX_DISCM: [u8; 8usize] = [
    162, 191, 255, 254, 196, 184, 230, 132,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CreateAndInitPoolIxArgs {
    pub sqrt_price_x32: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct CreateAndInitPoolIxData(pub CreateAndInitPoolIxArgs);
impl From<CreateAndInitPoolIxArgs> for CreateAndInitPoolIxData {
    fn from(args: CreateAndInitPoolIxArgs) -> Self {
        Self(args)
    }
}
impl CreateAndInitPoolIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CREATE_AND_INIT_POOL_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let sqrt_price_x32: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(CreateAndInitPoolIxArgs {
                sqrt_price_x32,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CREATE_AND_INIT_POOL_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.sqrt_price_x32, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn create_and_init_pool_ix_with_program_id(
    program_id: Pubkey,
    keys: CreateAndInitPoolKeys,
    args: CreateAndInitPoolIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CREATE_AND_INIT_POOL_IX_ACCOUNTS_LEN] = keys.into();
    let data: CreateAndInitPoolIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn create_and_init_pool_ix(
    keys: CreateAndInitPoolKeys,
    args: CreateAndInitPoolIxArgs,
) -> std::io::Result<Instruction> {
    create_and_init_pool_ix_with_program_id(CYKURA_PROGRAM_ID, keys, args)
}
pub fn create_and_init_pool_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CreateAndInitPoolAccounts<'_, '_>,
    args: CreateAndInitPoolIxArgs,
) -> ProgramResult {
    let keys: CreateAndInitPoolKeys = accounts.into();
    let ix = create_and_init_pool_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn create_and_init_pool_invoke(
    accounts: CreateAndInitPoolAccounts<'_, '_>,
    args: CreateAndInitPoolIxArgs,
) -> ProgramResult {
    create_and_init_pool_invoke_with_program_id(CYKURA_PROGRAM_ID, accounts, args)
}
pub fn create_and_init_pool_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CreateAndInitPoolAccounts<'_, '_>,
    args: CreateAndInitPoolIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CreateAndInitPoolKeys = accounts.into();
    let ix = create_and_init_pool_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn create_and_init_pool_invoke_signed(
    accounts: CreateAndInitPoolAccounts<'_, '_>,
    args: CreateAndInitPoolIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    create_and_init_pool_invoke_signed_with_program_id(
        CYKURA_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn create_and_init_pool_verify_account_keys(
    accounts: CreateAndInitPoolAccounts<'_, '_>,
    keys: CreateAndInitPoolKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.pool_creator.key, keys.pool_creator),
        (*accounts.token0.key, keys.token0),
        (*accounts.token1.key, keys.token1),
        (*accounts.fee_state.key, keys.fee_state),
        (*accounts.pool_state.key, keys.pool_state),
        (*accounts.initial_observation_state.key, keys.initial_observation_state),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.rent.key, keys.rent),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn create_and_init_pool_verify_writable_privileges<'me, 'info>(
    accounts: CreateAndInitPoolAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.pool_creator,
        accounts.pool_state,
        accounts.initial_observation_state,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn create_and_init_pool_verify_signer_privileges<'me, 'info>(
    accounts: CreateAndInitPoolAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.pool_creator] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn create_and_init_pool_verify_account_privileges<'me, 'info>(
    accounts: CreateAndInitPoolAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    create_and_init_pool_verify_writable_privileges(accounts)?;
    create_and_init_pool_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const INCREASE_OBSERVATION_CARDINALITY_NEXT_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct IncreaseObservationCardinalityNextAccounts<'me, 'info> {
    pub payer: &'me AccountInfo<'info>,
    pub pool_state: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct IncreaseObservationCardinalityNextKeys {
    pub payer: Pubkey,
    pub pool_state: Pubkey,
    pub system_program: Pubkey,
}
impl From<IncreaseObservationCardinalityNextAccounts<'_, '_>>
for IncreaseObservationCardinalityNextKeys {
    fn from(accounts: IncreaseObservationCardinalityNextAccounts) -> Self {
        Self {
            payer: *accounts.payer.key,
            pool_state: *accounts.pool_state.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<IncreaseObservationCardinalityNextKeys>
for [AccountMeta; INCREASE_OBSERVATION_CARDINALITY_NEXT_IX_ACCOUNTS_LEN] {
    fn from(keys: IncreaseObservationCardinalityNextKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.payer,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_state,
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
impl From<[Pubkey; INCREASE_OBSERVATION_CARDINALITY_NEXT_IX_ACCOUNTS_LEN]>
for IncreaseObservationCardinalityNextKeys {
    fn from(
        pubkeys: [Pubkey; INCREASE_OBSERVATION_CARDINALITY_NEXT_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            payer: pubkeys[0],
            pool_state: pubkeys[1],
            system_program: pubkeys[2],
        }
    }
}
impl<'info> From<IncreaseObservationCardinalityNextAccounts<'_, 'info>>
for [AccountInfo<'info>; INCREASE_OBSERVATION_CARDINALITY_NEXT_IX_ACCOUNTS_LEN] {
    fn from(accounts: IncreaseObservationCardinalityNextAccounts<'_, 'info>) -> Self {
        [
            accounts.payer.clone(),
            accounts.pool_state.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; INCREASE_OBSERVATION_CARDINALITY_NEXT_IX_ACCOUNTS_LEN]>
for IncreaseObservationCardinalityNextAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<
            'info,
        >; INCREASE_OBSERVATION_CARDINALITY_NEXT_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            payer: &arr[0],
            pool_state: &arr[1],
            system_program: &arr[2],
        }
    }
}
pub const INCREASE_OBSERVATION_CARDINALITY_NEXT_IX_DISCM: [u8; 8usize] = [
    182, 210, 154, 179, 182, 228, 163, 202,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct IncreaseObservationCardinalityNextIxArgs {
    pub observation_account_bumps: Vec<u8>,
}
#[derive(Clone, Debug, PartialEq)]
pub struct IncreaseObservationCardinalityNextIxData(
    pub IncreaseObservationCardinalityNextIxArgs,
);
impl From<IncreaseObservationCardinalityNextIxArgs>
for IncreaseObservationCardinalityNextIxData {
    fn from(args: IncreaseObservationCardinalityNextIxArgs) -> Self {
        Self(args)
    }
}
impl IncreaseObservationCardinalityNextIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INCREASE_OBSERVATION_CARDINALITY_NEXT_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let observation_account_bumps: Vec<u8> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        Ok(
            Self(IncreaseObservationCardinalityNextIxArgs {
                observation_account_bumps,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INCREASE_OBSERVATION_CARDINALITY_NEXT_IX_DISCM)?;
        borsh::BorshSerialize::serialize(
            &self.0.observation_account_bumps,
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
pub fn increase_observation_cardinality_next_ix_with_program_id(
    program_id: Pubkey,
    keys: IncreaseObservationCardinalityNextKeys,
    args: IncreaseObservationCardinalityNextIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; INCREASE_OBSERVATION_CARDINALITY_NEXT_IX_ACCOUNTS_LEN] = keys
        .into();
    let data: IncreaseObservationCardinalityNextIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn increase_observation_cardinality_next_ix(
    keys: IncreaseObservationCardinalityNextKeys,
    args: IncreaseObservationCardinalityNextIxArgs,
) -> std::io::Result<Instruction> {
    increase_observation_cardinality_next_ix_with_program_id(
        CYKURA_PROGRAM_ID,
        keys,
        args,
    )
}
pub fn increase_observation_cardinality_next_invoke_with_program_id(
    program_id: Pubkey,
    accounts: IncreaseObservationCardinalityNextAccounts<'_, '_>,
    args: IncreaseObservationCardinalityNextIxArgs,
) -> ProgramResult {
    let keys: IncreaseObservationCardinalityNextKeys = accounts.into();
    let ix = increase_observation_cardinality_next_ix_with_program_id(
        program_id,
        keys,
        args,
    )?;
    invoke_instruction(&ix, accounts)
}
pub fn increase_observation_cardinality_next_invoke(
    accounts: IncreaseObservationCardinalityNextAccounts<'_, '_>,
    args: IncreaseObservationCardinalityNextIxArgs,
) -> ProgramResult {
    increase_observation_cardinality_next_invoke_with_program_id(
        CYKURA_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn increase_observation_cardinality_next_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: IncreaseObservationCardinalityNextAccounts<'_, '_>,
    args: IncreaseObservationCardinalityNextIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: IncreaseObservationCardinalityNextKeys = accounts.into();
    let ix = increase_observation_cardinality_next_ix_with_program_id(
        program_id,
        keys,
        args,
    )?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn increase_observation_cardinality_next_invoke_signed(
    accounts: IncreaseObservationCardinalityNextAccounts<'_, '_>,
    args: IncreaseObservationCardinalityNextIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    increase_observation_cardinality_next_invoke_signed_with_program_id(
        CYKURA_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn increase_observation_cardinality_next_verify_account_keys(
    accounts: IncreaseObservationCardinalityNextAccounts<'_, '_>,
    keys: IncreaseObservationCardinalityNextKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.payer.key, keys.payer),
        (*accounts.pool_state.key, keys.pool_state),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn increase_observation_cardinality_next_verify_writable_privileges<'me, 'info>(
    accounts: IncreaseObservationCardinalityNextAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.pool_state] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn increase_observation_cardinality_next_verify_signer_privileges<'me, 'info>(
    accounts: IncreaseObservationCardinalityNextAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.payer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn increase_observation_cardinality_next_verify_account_privileges<'me, 'info>(
    accounts: IncreaseObservationCardinalityNextAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    increase_observation_cardinality_next_verify_writable_privileges(accounts)?;
    increase_observation_cardinality_next_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SET_FEE_PROTOCOL_IX_ACCOUNTS_LEN: usize = 2;
#[derive(Copy, Clone, Debug)]
pub struct SetFeeProtocolAccounts<'me, 'info> {
    pub owner: &'me AccountInfo<'info>,
    pub factory_state: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SetFeeProtocolKeys {
    pub owner: Pubkey,
    pub factory_state: Pubkey,
}
impl From<SetFeeProtocolAccounts<'_, '_>> for SetFeeProtocolKeys {
    fn from(accounts: SetFeeProtocolAccounts) -> Self {
        Self {
            owner: *accounts.owner.key,
            factory_state: *accounts.factory_state.key,
        }
    }
}
impl From<SetFeeProtocolKeys> for [AccountMeta; SET_FEE_PROTOCOL_IX_ACCOUNTS_LEN] {
    fn from(keys: SetFeeProtocolKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.owner,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.factory_state,
                is_signer: false,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; SET_FEE_PROTOCOL_IX_ACCOUNTS_LEN]> for SetFeeProtocolKeys {
    fn from(pubkeys: [Pubkey; SET_FEE_PROTOCOL_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            owner: pubkeys[0],
            factory_state: pubkeys[1],
        }
    }
}
impl<'info> From<SetFeeProtocolAccounts<'_, 'info>>
for [AccountInfo<'info>; SET_FEE_PROTOCOL_IX_ACCOUNTS_LEN] {
    fn from(accounts: SetFeeProtocolAccounts<'_, 'info>) -> Self {
        [accounts.owner.clone(), accounts.factory_state.clone()]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SET_FEE_PROTOCOL_IX_ACCOUNTS_LEN]>
for SetFeeProtocolAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; SET_FEE_PROTOCOL_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            owner: &arr[0],
            factory_state: &arr[1],
        }
    }
}
pub const SET_FEE_PROTOCOL_IX_DISCM: [u8; 8usize] = [88, 10, 24, 94, 217, 176, 124, 153];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SetFeeProtocolIxArgs {
    pub fee_protocol: u8,
}
#[derive(Clone, Debug, PartialEq)]
pub struct SetFeeProtocolIxData(pub SetFeeProtocolIxArgs);
impl From<SetFeeProtocolIxArgs> for SetFeeProtocolIxData {
    fn from(args: SetFeeProtocolIxArgs) -> Self {
        Self(args)
    }
}
impl SetFeeProtocolIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SET_FEE_PROTOCOL_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let fee_protocol: u8 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(SetFeeProtocolIxArgs {
                fee_protocol,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SET_FEE_PROTOCOL_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.fee_protocol, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn set_fee_protocol_ix_with_program_id(
    program_id: Pubkey,
    keys: SetFeeProtocolKeys,
    args: SetFeeProtocolIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SET_FEE_PROTOCOL_IX_ACCOUNTS_LEN] = keys.into();
    let data: SetFeeProtocolIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn set_fee_protocol_ix(
    keys: SetFeeProtocolKeys,
    args: SetFeeProtocolIxArgs,
) -> std::io::Result<Instruction> {
    set_fee_protocol_ix_with_program_id(CYKURA_PROGRAM_ID, keys, args)
}
pub fn set_fee_protocol_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SetFeeProtocolAccounts<'_, '_>,
    args: SetFeeProtocolIxArgs,
) -> ProgramResult {
    let keys: SetFeeProtocolKeys = accounts.into();
    let ix = set_fee_protocol_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn set_fee_protocol_invoke(
    accounts: SetFeeProtocolAccounts<'_, '_>,
    args: SetFeeProtocolIxArgs,
) -> ProgramResult {
    set_fee_protocol_invoke_with_program_id(CYKURA_PROGRAM_ID, accounts, args)
}
pub fn set_fee_protocol_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SetFeeProtocolAccounts<'_, '_>,
    args: SetFeeProtocolIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SetFeeProtocolKeys = accounts.into();
    let ix = set_fee_protocol_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn set_fee_protocol_invoke_signed(
    accounts: SetFeeProtocolAccounts<'_, '_>,
    args: SetFeeProtocolIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    set_fee_protocol_invoke_signed_with_program_id(
        CYKURA_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn set_fee_protocol_verify_account_keys(
    accounts: SetFeeProtocolAccounts<'_, '_>,
    keys: SetFeeProtocolKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.owner.key, keys.owner),
        (*accounts.factory_state.key, keys.factory_state),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn set_fee_protocol_verify_writable_privileges<'me, 'info>(
    accounts: SetFeeProtocolAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.factory_state] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn set_fee_protocol_verify_signer_privileges<'me, 'info>(
    accounts: SetFeeProtocolAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.owner] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn set_fee_protocol_verify_account_privileges<'me, 'info>(
    accounts: SetFeeProtocolAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    set_fee_protocol_verify_writable_privileges(accounts)?;
    set_fee_protocol_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const COLLECT_PROTOCOL_IX_ACCOUNTS_LEN: usize = 8;
#[derive(Copy, Clone, Debug)]
pub struct CollectProtocolAccounts<'me, 'info> {
    pub owner: &'me AccountInfo<'info>,
    pub factory_state: &'me AccountInfo<'info>,
    pub pool_state: &'me AccountInfo<'info>,
    pub vault0: &'me AccountInfo<'info>,
    pub vault1: &'me AccountInfo<'info>,
    pub recipient_wallet0: &'me AccountInfo<'info>,
    pub recipient_wallet1: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CollectProtocolKeys {
    pub owner: Pubkey,
    pub factory_state: Pubkey,
    pub pool_state: Pubkey,
    pub vault0: Pubkey,
    pub vault1: Pubkey,
    pub recipient_wallet0: Pubkey,
    pub recipient_wallet1: Pubkey,
    pub token_program: Pubkey,
}
impl From<CollectProtocolAccounts<'_, '_>> for CollectProtocolKeys {
    fn from(accounts: CollectProtocolAccounts) -> Self {
        Self {
            owner: *accounts.owner.key,
            factory_state: *accounts.factory_state.key,
            pool_state: *accounts.pool_state.key,
            vault0: *accounts.vault0.key,
            vault1: *accounts.vault1.key,
            recipient_wallet0: *accounts.recipient_wallet0.key,
            recipient_wallet1: *accounts.recipient_wallet1.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<CollectProtocolKeys> for [AccountMeta; COLLECT_PROTOCOL_IX_ACCOUNTS_LEN] {
    fn from(keys: CollectProtocolKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.owner,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.factory_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pool_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vault0,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vault1,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.recipient_wallet0,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.recipient_wallet1,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; COLLECT_PROTOCOL_IX_ACCOUNTS_LEN]> for CollectProtocolKeys {
    fn from(pubkeys: [Pubkey; COLLECT_PROTOCOL_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            owner: pubkeys[0],
            factory_state: pubkeys[1],
            pool_state: pubkeys[2],
            vault0: pubkeys[3],
            vault1: pubkeys[4],
            recipient_wallet0: pubkeys[5],
            recipient_wallet1: pubkeys[6],
            token_program: pubkeys[7],
        }
    }
}
impl<'info> From<CollectProtocolAccounts<'_, 'info>>
for [AccountInfo<'info>; COLLECT_PROTOCOL_IX_ACCOUNTS_LEN] {
    fn from(accounts: CollectProtocolAccounts<'_, 'info>) -> Self {
        [
            accounts.owner.clone(),
            accounts.factory_state.clone(),
            accounts.pool_state.clone(),
            accounts.vault0.clone(),
            accounts.vault1.clone(),
            accounts.recipient_wallet0.clone(),
            accounts.recipient_wallet1.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; COLLECT_PROTOCOL_IX_ACCOUNTS_LEN]>
for CollectProtocolAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; COLLECT_PROTOCOL_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            owner: &arr[0],
            factory_state: &arr[1],
            pool_state: &arr[2],
            vault0: &arr[3],
            vault1: &arr[4],
            recipient_wallet0: &arr[5],
            recipient_wallet1: &arr[6],
            token_program: &arr[7],
        }
    }
}
pub const COLLECT_PROTOCOL_IX_DISCM: [u8; 8usize] = [30, 17, 219, 243, 39, 164, 93, 96];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CollectProtocolIxArgs {
    pub amount0_requested: u64,
    pub amount1_requested: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct CollectProtocolIxData(pub CollectProtocolIxArgs);
impl From<CollectProtocolIxArgs> for CollectProtocolIxData {
    fn from(args: CollectProtocolIxArgs) -> Self {
        Self(args)
    }
}
impl CollectProtocolIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != COLLECT_PROTOCOL_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let amount0_requested: u64 = crate::borsh_de_or_default(&mut reader)?;
        let amount1_requested: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(CollectProtocolIxArgs {
                amount0_requested,
                amount1_requested,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&COLLECT_PROTOCOL_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.amount0_requested, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.amount1_requested, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn collect_protocol_ix_with_program_id(
    program_id: Pubkey,
    keys: CollectProtocolKeys,
    args: CollectProtocolIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; COLLECT_PROTOCOL_IX_ACCOUNTS_LEN] = keys.into();
    let data: CollectProtocolIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn collect_protocol_ix(
    keys: CollectProtocolKeys,
    args: CollectProtocolIxArgs,
) -> std::io::Result<Instruction> {
    collect_protocol_ix_with_program_id(CYKURA_PROGRAM_ID, keys, args)
}
pub fn collect_protocol_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CollectProtocolAccounts<'_, '_>,
    args: CollectProtocolIxArgs,
) -> ProgramResult {
    let keys: CollectProtocolKeys = accounts.into();
    let ix = collect_protocol_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn collect_protocol_invoke(
    accounts: CollectProtocolAccounts<'_, '_>,
    args: CollectProtocolIxArgs,
) -> ProgramResult {
    collect_protocol_invoke_with_program_id(CYKURA_PROGRAM_ID, accounts, args)
}
pub fn collect_protocol_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CollectProtocolAccounts<'_, '_>,
    args: CollectProtocolIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CollectProtocolKeys = accounts.into();
    let ix = collect_protocol_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn collect_protocol_invoke_signed(
    accounts: CollectProtocolAccounts<'_, '_>,
    args: CollectProtocolIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    collect_protocol_invoke_signed_with_program_id(
        CYKURA_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn collect_protocol_verify_account_keys(
    accounts: CollectProtocolAccounts<'_, '_>,
    keys: CollectProtocolKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.owner.key, keys.owner),
        (*accounts.factory_state.key, keys.factory_state),
        (*accounts.pool_state.key, keys.pool_state),
        (*accounts.vault0.key, keys.vault0),
        (*accounts.vault1.key, keys.vault1),
        (*accounts.recipient_wallet0.key, keys.recipient_wallet0),
        (*accounts.recipient_wallet1.key, keys.recipient_wallet1),
        (*accounts.token_program.key, keys.token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn collect_protocol_verify_writable_privileges<'me, 'info>(
    accounts: CollectProtocolAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.factory_state,
        accounts.pool_state,
        accounts.vault0,
        accounts.vault1,
        accounts.recipient_wallet0,
        accounts.recipient_wallet1,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn collect_protocol_verify_signer_privileges<'me, 'info>(
    accounts: CollectProtocolAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.owner] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn collect_protocol_verify_account_privileges<'me, 'info>(
    accounts: CollectProtocolAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    collect_protocol_verify_writable_privileges(accounts)?;
    collect_protocol_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const INIT_TICK_ACCOUNT_IX_ACCOUNTS_LEN: usize = 4;
#[derive(Copy, Clone, Debug)]
pub struct InitTickAccountAccounts<'me, 'info> {
    pub signer: &'me AccountInfo<'info>,
    pub pool_state: &'me AccountInfo<'info>,
    pub tick_state: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct InitTickAccountKeys {
    pub signer: Pubkey,
    pub pool_state: Pubkey,
    pub tick_state: Pubkey,
    pub system_program: Pubkey,
}
impl From<InitTickAccountAccounts<'_, '_>> for InitTickAccountKeys {
    fn from(accounts: InitTickAccountAccounts) -> Self {
        Self {
            signer: *accounts.signer.key,
            pool_state: *accounts.pool_state.key,
            tick_state: *accounts.tick_state.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<InitTickAccountKeys> for [AccountMeta; INIT_TICK_ACCOUNT_IX_ACCOUNTS_LEN] {
    fn from(keys: InitTickAccountKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.signer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pool_state,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.tick_state,
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
impl From<[Pubkey; INIT_TICK_ACCOUNT_IX_ACCOUNTS_LEN]> for InitTickAccountKeys {
    fn from(pubkeys: [Pubkey; INIT_TICK_ACCOUNT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            signer: pubkeys[0],
            pool_state: pubkeys[1],
            tick_state: pubkeys[2],
            system_program: pubkeys[3],
        }
    }
}
impl<'info> From<InitTickAccountAccounts<'_, 'info>>
for [AccountInfo<'info>; INIT_TICK_ACCOUNT_IX_ACCOUNTS_LEN] {
    fn from(accounts: InitTickAccountAccounts<'_, 'info>) -> Self {
        [
            accounts.signer.clone(),
            accounts.pool_state.clone(),
            accounts.tick_state.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; INIT_TICK_ACCOUNT_IX_ACCOUNTS_LEN]>
for InitTickAccountAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; INIT_TICK_ACCOUNT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            signer: &arr[0],
            pool_state: &arr[1],
            tick_state: &arr[2],
            system_program: &arr[3],
        }
    }
}
pub const INIT_TICK_ACCOUNT_IX_DISCM: [u8; 8usize] = [
    201, 21, 130, 106, 197, 224, 18, 214,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct InitTickAccountIxArgs {
    pub tick: i32,
}
#[derive(Clone, Debug, PartialEq)]
pub struct InitTickAccountIxData(pub InitTickAccountIxArgs);
impl From<InitTickAccountIxArgs> for InitTickAccountIxData {
    fn from(args: InitTickAccountIxArgs) -> Self {
        Self(args)
    }
}
impl InitTickAccountIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INIT_TICK_ACCOUNT_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let tick: i32 = crate::borsh_de_or_default(&mut reader)?;
        Ok(Self(InitTickAccountIxArgs { tick }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INIT_TICK_ACCOUNT_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.tick, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn init_tick_account_ix_with_program_id(
    program_id: Pubkey,
    keys: InitTickAccountKeys,
    args: InitTickAccountIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; INIT_TICK_ACCOUNT_IX_ACCOUNTS_LEN] = keys.into();
    let data: InitTickAccountIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn init_tick_account_ix(
    keys: InitTickAccountKeys,
    args: InitTickAccountIxArgs,
) -> std::io::Result<Instruction> {
    init_tick_account_ix_with_program_id(CYKURA_PROGRAM_ID, keys, args)
}
pub fn init_tick_account_invoke_with_program_id(
    program_id: Pubkey,
    accounts: InitTickAccountAccounts<'_, '_>,
    args: InitTickAccountIxArgs,
) -> ProgramResult {
    let keys: InitTickAccountKeys = accounts.into();
    let ix = init_tick_account_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn init_tick_account_invoke(
    accounts: InitTickAccountAccounts<'_, '_>,
    args: InitTickAccountIxArgs,
) -> ProgramResult {
    init_tick_account_invoke_with_program_id(CYKURA_PROGRAM_ID, accounts, args)
}
pub fn init_tick_account_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: InitTickAccountAccounts<'_, '_>,
    args: InitTickAccountIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: InitTickAccountKeys = accounts.into();
    let ix = init_tick_account_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn init_tick_account_invoke_signed(
    accounts: InitTickAccountAccounts<'_, '_>,
    args: InitTickAccountIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    init_tick_account_invoke_signed_with_program_id(
        CYKURA_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn init_tick_account_verify_account_keys(
    accounts: InitTickAccountAccounts<'_, '_>,
    keys: InitTickAccountKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.signer.key, keys.signer),
        (*accounts.pool_state.key, keys.pool_state),
        (*accounts.tick_state.key, keys.tick_state),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn init_tick_account_verify_writable_privileges<'me, 'info>(
    accounts: InitTickAccountAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.signer, accounts.tick_state] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn init_tick_account_verify_signer_privileges<'me, 'info>(
    accounts: InitTickAccountAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.signer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn init_tick_account_verify_account_privileges<'me, 'info>(
    accounts: InitTickAccountAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    init_tick_account_verify_writable_privileges(accounts)?;
    init_tick_account_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CLOSE_TICK_ACCOUNT_IX_ACCOUNTS_LEN: usize = 2;
#[derive(Copy, Clone, Debug)]
pub struct CloseTickAccountAccounts<'me, 'info> {
    pub tick_state: &'me AccountInfo<'info>,
    pub recipient: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CloseTickAccountKeys {
    pub tick_state: Pubkey,
    pub recipient: Pubkey,
}
impl From<CloseTickAccountAccounts<'_, '_>> for CloseTickAccountKeys {
    fn from(accounts: CloseTickAccountAccounts) -> Self {
        Self {
            tick_state: *accounts.tick_state.key,
            recipient: *accounts.recipient.key,
        }
    }
}
impl From<CloseTickAccountKeys> for [AccountMeta; CLOSE_TICK_ACCOUNT_IX_ACCOUNTS_LEN] {
    fn from(keys: CloseTickAccountKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.tick_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.recipient,
                is_signer: false,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; CLOSE_TICK_ACCOUNT_IX_ACCOUNTS_LEN]> for CloseTickAccountKeys {
    fn from(pubkeys: [Pubkey; CLOSE_TICK_ACCOUNT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            tick_state: pubkeys[0],
            recipient: pubkeys[1],
        }
    }
}
impl<'info> From<CloseTickAccountAccounts<'_, 'info>>
for [AccountInfo<'info>; CLOSE_TICK_ACCOUNT_IX_ACCOUNTS_LEN] {
    fn from(accounts: CloseTickAccountAccounts<'_, 'info>) -> Self {
        [accounts.tick_state.clone(), accounts.recipient.clone()]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CLOSE_TICK_ACCOUNT_IX_ACCOUNTS_LEN]>
for CloseTickAccountAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; CLOSE_TICK_ACCOUNT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            tick_state: &arr[0],
            recipient: &arr[1],
        }
    }
}
pub const CLOSE_TICK_ACCOUNT_IX_DISCM: [u8; 8usize] = [
    225, 243, 157, 252, 220, 169, 12, 214,
];
#[derive(Clone, Debug, PartialEq)]
pub struct CloseTickAccountIxData;
impl CloseTickAccountIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CLOSE_TICK_ACCOUNT_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CLOSE_TICK_ACCOUNT_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn close_tick_account_ix_with_program_id(
    program_id: Pubkey,
    keys: CloseTickAccountKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CLOSE_TICK_ACCOUNT_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: CloseTickAccountIxData.try_to_vec()?,
    })
}
pub fn close_tick_account_ix(
    keys: CloseTickAccountKeys,
) -> std::io::Result<Instruction> {
    close_tick_account_ix_with_program_id(CYKURA_PROGRAM_ID, keys)
}
pub fn close_tick_account_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CloseTickAccountAccounts<'_, '_>,
) -> ProgramResult {
    let keys: CloseTickAccountKeys = accounts.into();
    let ix = close_tick_account_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn close_tick_account_invoke(
    accounts: CloseTickAccountAccounts<'_, '_>,
) -> ProgramResult {
    close_tick_account_invoke_with_program_id(CYKURA_PROGRAM_ID, accounts)
}
pub fn close_tick_account_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CloseTickAccountAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CloseTickAccountKeys = accounts.into();
    let ix = close_tick_account_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn close_tick_account_invoke_signed(
    accounts: CloseTickAccountAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    close_tick_account_invoke_signed_with_program_id(CYKURA_PROGRAM_ID, accounts, seeds)
}
pub fn close_tick_account_verify_account_keys(
    accounts: CloseTickAccountAccounts<'_, '_>,
    keys: CloseTickAccountKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.tick_state.key, keys.tick_state),
        (*accounts.recipient.key, keys.recipient),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn close_tick_account_verify_writable_privileges<'me, 'info>(
    accounts: CloseTickAccountAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.tick_state, accounts.recipient] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn close_tick_account_verify_account_privileges<'me, 'info>(
    accounts: CloseTickAccountAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    close_tick_account_verify_writable_privileges(accounts)?;
    Ok(())
}
pub const INIT_BITMAP_ACCOUNT_IX_ACCOUNTS_LEN: usize = 4;
#[derive(Copy, Clone, Debug)]
pub struct InitBitmapAccountAccounts<'me, 'info> {
    pub signer: &'me AccountInfo<'info>,
    pub pool_state: &'me AccountInfo<'info>,
    pub bitmap_state: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct InitBitmapAccountKeys {
    pub signer: Pubkey,
    pub pool_state: Pubkey,
    pub bitmap_state: Pubkey,
    pub system_program: Pubkey,
}
impl From<InitBitmapAccountAccounts<'_, '_>> for InitBitmapAccountKeys {
    fn from(accounts: InitBitmapAccountAccounts) -> Self {
        Self {
            signer: *accounts.signer.key,
            pool_state: *accounts.pool_state.key,
            bitmap_state: *accounts.bitmap_state.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<InitBitmapAccountKeys> for [AccountMeta; INIT_BITMAP_ACCOUNT_IX_ACCOUNTS_LEN] {
    fn from(keys: InitBitmapAccountKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.signer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pool_state,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.bitmap_state,
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
impl From<[Pubkey; INIT_BITMAP_ACCOUNT_IX_ACCOUNTS_LEN]> for InitBitmapAccountKeys {
    fn from(pubkeys: [Pubkey; INIT_BITMAP_ACCOUNT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            signer: pubkeys[0],
            pool_state: pubkeys[1],
            bitmap_state: pubkeys[2],
            system_program: pubkeys[3],
        }
    }
}
impl<'info> From<InitBitmapAccountAccounts<'_, 'info>>
for [AccountInfo<'info>; INIT_BITMAP_ACCOUNT_IX_ACCOUNTS_LEN] {
    fn from(accounts: InitBitmapAccountAccounts<'_, 'info>) -> Self {
        [
            accounts.signer.clone(),
            accounts.pool_state.clone(),
            accounts.bitmap_state.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; INIT_BITMAP_ACCOUNT_IX_ACCOUNTS_LEN]>
for InitBitmapAccountAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; INIT_BITMAP_ACCOUNT_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            signer: &arr[0],
            pool_state: &arr[1],
            bitmap_state: &arr[2],
            system_program: &arr[3],
        }
    }
}
pub const INIT_BITMAP_ACCOUNT_IX_DISCM: [u8; 8usize] = [
    11, 242, 106, 1, 255, 103, 156, 212,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct InitBitmapAccountIxArgs {
    pub word_pos: i16,
}
#[derive(Clone, Debug, PartialEq)]
pub struct InitBitmapAccountIxData(pub InitBitmapAccountIxArgs);
impl From<InitBitmapAccountIxArgs> for InitBitmapAccountIxData {
    fn from(args: InitBitmapAccountIxArgs) -> Self {
        Self(args)
    }
}
impl InitBitmapAccountIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INIT_BITMAP_ACCOUNT_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let word_pos: i16 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(InitBitmapAccountIxArgs {
                word_pos,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INIT_BITMAP_ACCOUNT_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.word_pos, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn init_bitmap_account_ix_with_program_id(
    program_id: Pubkey,
    keys: InitBitmapAccountKeys,
    args: InitBitmapAccountIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; INIT_BITMAP_ACCOUNT_IX_ACCOUNTS_LEN] = keys.into();
    let data: InitBitmapAccountIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn init_bitmap_account_ix(
    keys: InitBitmapAccountKeys,
    args: InitBitmapAccountIxArgs,
) -> std::io::Result<Instruction> {
    init_bitmap_account_ix_with_program_id(CYKURA_PROGRAM_ID, keys, args)
}
pub fn init_bitmap_account_invoke_with_program_id(
    program_id: Pubkey,
    accounts: InitBitmapAccountAccounts<'_, '_>,
    args: InitBitmapAccountIxArgs,
) -> ProgramResult {
    let keys: InitBitmapAccountKeys = accounts.into();
    let ix = init_bitmap_account_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn init_bitmap_account_invoke(
    accounts: InitBitmapAccountAccounts<'_, '_>,
    args: InitBitmapAccountIxArgs,
) -> ProgramResult {
    init_bitmap_account_invoke_with_program_id(CYKURA_PROGRAM_ID, accounts, args)
}
pub fn init_bitmap_account_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: InitBitmapAccountAccounts<'_, '_>,
    args: InitBitmapAccountIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: InitBitmapAccountKeys = accounts.into();
    let ix = init_bitmap_account_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn init_bitmap_account_invoke_signed(
    accounts: InitBitmapAccountAccounts<'_, '_>,
    args: InitBitmapAccountIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    init_bitmap_account_invoke_signed_with_program_id(
        CYKURA_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn init_bitmap_account_verify_account_keys(
    accounts: InitBitmapAccountAccounts<'_, '_>,
    keys: InitBitmapAccountKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.signer.key, keys.signer),
        (*accounts.pool_state.key, keys.pool_state),
        (*accounts.bitmap_state.key, keys.bitmap_state),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn init_bitmap_account_verify_writable_privileges<'me, 'info>(
    accounts: InitBitmapAccountAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.signer, accounts.bitmap_state] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn init_bitmap_account_verify_signer_privileges<'me, 'info>(
    accounts: InitBitmapAccountAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.signer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn init_bitmap_account_verify_account_privileges<'me, 'info>(
    accounts: InitBitmapAccountAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    init_bitmap_account_verify_writable_privileges(accounts)?;
    init_bitmap_account_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const INIT_POSITION_ACCOUNT_IX_ACCOUNTS_LEN: usize = 7;
#[derive(Copy, Clone, Debug)]
pub struct InitPositionAccountAccounts<'me, 'info> {
    pub signer: &'me AccountInfo<'info>,
    pub recipient: &'me AccountInfo<'info>,
    pub pool_state: &'me AccountInfo<'info>,
    pub tick_lower_state: &'me AccountInfo<'info>,
    pub tick_upper_state: &'me AccountInfo<'info>,
    pub position_state: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct InitPositionAccountKeys {
    pub signer: Pubkey,
    pub recipient: Pubkey,
    pub pool_state: Pubkey,
    pub tick_lower_state: Pubkey,
    pub tick_upper_state: Pubkey,
    pub position_state: Pubkey,
    pub system_program: Pubkey,
}
impl From<InitPositionAccountAccounts<'_, '_>> for InitPositionAccountKeys {
    fn from(accounts: InitPositionAccountAccounts) -> Self {
        Self {
            signer: *accounts.signer.key,
            recipient: *accounts.recipient.key,
            pool_state: *accounts.pool_state.key,
            tick_lower_state: *accounts.tick_lower_state.key,
            tick_upper_state: *accounts.tick_upper_state.key,
            position_state: *accounts.position_state.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<InitPositionAccountKeys>
for [AccountMeta; INIT_POSITION_ACCOUNT_IX_ACCOUNTS_LEN] {
    fn from(keys: InitPositionAccountKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.signer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.recipient,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_state,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.tick_lower_state,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.tick_upper_state,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.position_state,
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
impl From<[Pubkey; INIT_POSITION_ACCOUNT_IX_ACCOUNTS_LEN]> for InitPositionAccountKeys {
    fn from(pubkeys: [Pubkey; INIT_POSITION_ACCOUNT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            signer: pubkeys[0],
            recipient: pubkeys[1],
            pool_state: pubkeys[2],
            tick_lower_state: pubkeys[3],
            tick_upper_state: pubkeys[4],
            position_state: pubkeys[5],
            system_program: pubkeys[6],
        }
    }
}
impl<'info> From<InitPositionAccountAccounts<'_, 'info>>
for [AccountInfo<'info>; INIT_POSITION_ACCOUNT_IX_ACCOUNTS_LEN] {
    fn from(accounts: InitPositionAccountAccounts<'_, 'info>) -> Self {
        [
            accounts.signer.clone(),
            accounts.recipient.clone(),
            accounts.pool_state.clone(),
            accounts.tick_lower_state.clone(),
            accounts.tick_upper_state.clone(),
            accounts.position_state.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; INIT_POSITION_ACCOUNT_IX_ACCOUNTS_LEN]>
for InitPositionAccountAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; INIT_POSITION_ACCOUNT_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            signer: &arr[0],
            recipient: &arr[1],
            pool_state: &arr[2],
            tick_lower_state: &arr[3],
            tick_upper_state: &arr[4],
            position_state: &arr[5],
            system_program: &arr[6],
        }
    }
}
pub const INIT_POSITION_ACCOUNT_IX_DISCM: [u8; 8usize] = [
    34, 23, 74, 12, 241, 183, 27, 102,
];
#[derive(Clone, Debug, PartialEq)]
pub struct InitPositionAccountIxData;
impl InitPositionAccountIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INIT_POSITION_ACCOUNT_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INIT_POSITION_ACCOUNT_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn init_position_account_ix_with_program_id(
    program_id: Pubkey,
    keys: InitPositionAccountKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; INIT_POSITION_ACCOUNT_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: InitPositionAccountIxData.try_to_vec()?,
    })
}
pub fn init_position_account_ix(
    keys: InitPositionAccountKeys,
) -> std::io::Result<Instruction> {
    init_position_account_ix_with_program_id(CYKURA_PROGRAM_ID, keys)
}
pub fn init_position_account_invoke_with_program_id(
    program_id: Pubkey,
    accounts: InitPositionAccountAccounts<'_, '_>,
) -> ProgramResult {
    let keys: InitPositionAccountKeys = accounts.into();
    let ix = init_position_account_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn init_position_account_invoke(
    accounts: InitPositionAccountAccounts<'_, '_>,
) -> ProgramResult {
    init_position_account_invoke_with_program_id(CYKURA_PROGRAM_ID, accounts)
}
pub fn init_position_account_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: InitPositionAccountAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: InitPositionAccountKeys = accounts.into();
    let ix = init_position_account_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn init_position_account_invoke_signed(
    accounts: InitPositionAccountAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    init_position_account_invoke_signed_with_program_id(
        CYKURA_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn init_position_account_verify_account_keys(
    accounts: InitPositionAccountAccounts<'_, '_>,
    keys: InitPositionAccountKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.signer.key, keys.signer),
        (*accounts.recipient.key, keys.recipient),
        (*accounts.pool_state.key, keys.pool_state),
        (*accounts.tick_lower_state.key, keys.tick_lower_state),
        (*accounts.tick_upper_state.key, keys.tick_upper_state),
        (*accounts.position_state.key, keys.position_state),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn init_position_account_verify_writable_privileges<'me, 'info>(
    accounts: InitPositionAccountAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.signer, accounts.position_state] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn init_position_account_verify_signer_privileges<'me, 'info>(
    accounts: InitPositionAccountAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.signer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn init_position_account_verify_account_privileges<'me, 'info>(
    accounts: InitPositionAccountAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    init_position_account_verify_writable_privileges(accounts)?;
    init_position_account_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const MINT_CALLBACK_IX_ACCOUNTS_LEN: usize = 6;
#[derive(Copy, Clone, Debug)]
pub struct MintCallbackAccounts<'me, 'info> {
    pub minter: &'me AccountInfo<'info>,
    pub token_account0: &'me AccountInfo<'info>,
    pub token_account1: &'me AccountInfo<'info>,
    pub vault0: &'me AccountInfo<'info>,
    pub vault1: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MintCallbackKeys {
    pub minter: Pubkey,
    pub token_account0: Pubkey,
    pub token_account1: Pubkey,
    pub vault0: Pubkey,
    pub vault1: Pubkey,
    pub token_program: Pubkey,
}
impl From<MintCallbackAccounts<'_, '_>> for MintCallbackKeys {
    fn from(accounts: MintCallbackAccounts) -> Self {
        Self {
            minter: *accounts.minter.key,
            token_account0: *accounts.token_account0.key,
            token_account1: *accounts.token_account1.key,
            vault0: *accounts.vault0.key,
            vault1: *accounts.vault1.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<MintCallbackKeys> for [AccountMeta; MINT_CALLBACK_IX_ACCOUNTS_LEN] {
    fn from(keys: MintCallbackKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.minter,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_account0,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_account1,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault0,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault1,
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
impl From<[Pubkey; MINT_CALLBACK_IX_ACCOUNTS_LEN]> for MintCallbackKeys {
    fn from(pubkeys: [Pubkey; MINT_CALLBACK_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            minter: pubkeys[0],
            token_account0: pubkeys[1],
            token_account1: pubkeys[2],
            vault0: pubkeys[3],
            vault1: pubkeys[4],
            token_program: pubkeys[5],
        }
    }
}
impl<'info> From<MintCallbackAccounts<'_, 'info>>
for [AccountInfo<'info>; MINT_CALLBACK_IX_ACCOUNTS_LEN] {
    fn from(accounts: MintCallbackAccounts<'_, 'info>) -> Self {
        [
            accounts.minter.clone(),
            accounts.token_account0.clone(),
            accounts.token_account1.clone(),
            accounts.vault0.clone(),
            accounts.vault1.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; MINT_CALLBACK_IX_ACCOUNTS_LEN]>
for MintCallbackAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; MINT_CALLBACK_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            minter: &arr[0],
            token_account0: &arr[1],
            token_account1: &arr[2],
            vault0: &arr[3],
            vault1: &arr[4],
            token_program: &arr[5],
        }
    }
}
pub const MINT_CALLBACK_IX_DISCM: [u8; 8usize] = [221, 131, 183, 240, 176, 158, 43, 35];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MintCallbackIxArgs {
    pub amount0_owed: u64,
    pub amount1_owed: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct MintCallbackIxData(pub MintCallbackIxArgs);
impl From<MintCallbackIxArgs> for MintCallbackIxData {
    fn from(args: MintCallbackIxArgs) -> Self {
        Self(args)
    }
}
impl MintCallbackIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != MINT_CALLBACK_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let amount0_owed: u64 = crate::borsh_de_or_default(&mut reader)?;
        let amount1_owed: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(MintCallbackIxArgs {
                amount0_owed,
                amount1_owed,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&MINT_CALLBACK_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.amount0_owed, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.amount1_owed, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn mint_callback_ix_with_program_id(
    program_id: Pubkey,
    keys: MintCallbackKeys,
    args: MintCallbackIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; MINT_CALLBACK_IX_ACCOUNTS_LEN] = keys.into();
    let data: MintCallbackIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn mint_callback_ix(
    keys: MintCallbackKeys,
    args: MintCallbackIxArgs,
) -> std::io::Result<Instruction> {
    mint_callback_ix_with_program_id(CYKURA_PROGRAM_ID, keys, args)
}
pub fn mint_callback_invoke_with_program_id(
    program_id: Pubkey,
    accounts: MintCallbackAccounts<'_, '_>,
    args: MintCallbackIxArgs,
) -> ProgramResult {
    let keys: MintCallbackKeys = accounts.into();
    let ix = mint_callback_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn mint_callback_invoke(
    accounts: MintCallbackAccounts<'_, '_>,
    args: MintCallbackIxArgs,
) -> ProgramResult {
    mint_callback_invoke_with_program_id(CYKURA_PROGRAM_ID, accounts, args)
}
pub fn mint_callback_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: MintCallbackAccounts<'_, '_>,
    args: MintCallbackIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: MintCallbackKeys = accounts.into();
    let ix = mint_callback_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn mint_callback_invoke_signed(
    accounts: MintCallbackAccounts<'_, '_>,
    args: MintCallbackIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    mint_callback_invoke_signed_with_program_id(CYKURA_PROGRAM_ID, accounts, args, seeds)
}
pub fn mint_callback_verify_account_keys(
    accounts: MintCallbackAccounts<'_, '_>,
    keys: MintCallbackKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.minter.key, keys.minter),
        (*accounts.token_account0.key, keys.token_account0),
        (*accounts.token_account1.key, keys.token_account1),
        (*accounts.vault0.key, keys.vault0),
        (*accounts.vault1.key, keys.vault1),
        (*accounts.token_program.key, keys.token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn mint_callback_verify_signer_privileges<'me, 'info>(
    accounts: MintCallbackAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.minter] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn mint_callback_verify_account_privileges<'me, 'info>(
    accounts: MintCallbackAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    mint_callback_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SWAP_CALLBACK_IX_ACCOUNTS_LEN: usize = 6;
#[derive(Copy, Clone, Debug)]
pub struct SwapCallbackAccounts<'me, 'info> {
    pub signer: &'me AccountInfo<'info>,
    pub input_token_account: &'me AccountInfo<'info>,
    pub output_token_account: &'me AccountInfo<'info>,
    pub input_vault: &'me AccountInfo<'info>,
    pub output_vault: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SwapCallbackKeys {
    pub signer: Pubkey,
    pub input_token_account: Pubkey,
    pub output_token_account: Pubkey,
    pub input_vault: Pubkey,
    pub output_vault: Pubkey,
    pub token_program: Pubkey,
}
impl From<SwapCallbackAccounts<'_, '_>> for SwapCallbackKeys {
    fn from(accounts: SwapCallbackAccounts) -> Self {
        Self {
            signer: *accounts.signer.key,
            input_token_account: *accounts.input_token_account.key,
            output_token_account: *accounts.output_token_account.key,
            input_vault: *accounts.input_vault.key,
            output_vault: *accounts.output_vault.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<SwapCallbackKeys> for [AccountMeta; SWAP_CALLBACK_IX_ACCOUNTS_LEN] {
    fn from(keys: SwapCallbackKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.signer,
                is_signer: true,
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
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; SWAP_CALLBACK_IX_ACCOUNTS_LEN]> for SwapCallbackKeys {
    fn from(pubkeys: [Pubkey; SWAP_CALLBACK_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            signer: pubkeys[0],
            input_token_account: pubkeys[1],
            output_token_account: pubkeys[2],
            input_vault: pubkeys[3],
            output_vault: pubkeys[4],
            token_program: pubkeys[5],
        }
    }
}
impl<'info> From<SwapCallbackAccounts<'_, 'info>>
for [AccountInfo<'info>; SWAP_CALLBACK_IX_ACCOUNTS_LEN] {
    fn from(accounts: SwapCallbackAccounts<'_, 'info>) -> Self {
        [
            accounts.signer.clone(),
            accounts.input_token_account.clone(),
            accounts.output_token_account.clone(),
            accounts.input_vault.clone(),
            accounts.output_vault.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SWAP_CALLBACK_IX_ACCOUNTS_LEN]>
for SwapCallbackAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; SWAP_CALLBACK_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            signer: &arr[0],
            input_token_account: &arr[1],
            output_token_account: &arr[2],
            input_vault: &arr[3],
            output_vault: &arr[4],
            token_program: &arr[5],
        }
    }
}
pub const SWAP_CALLBACK_IX_DISCM: [u8; 8usize] = [103, 125, 93, 121, 54, 18, 238, 141];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SwapCallbackIxArgs {
    pub amount0_delta: i64,
    pub amount1_delta: i64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct SwapCallbackIxData(pub SwapCallbackIxArgs);
impl From<SwapCallbackIxArgs> for SwapCallbackIxData {
    fn from(args: SwapCallbackIxArgs) -> Self {
        Self(args)
    }
}
impl SwapCallbackIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SWAP_CALLBACK_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let amount0_delta: i64 = crate::borsh_de_or_default(&mut reader)?;
        let amount1_delta: i64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(SwapCallbackIxArgs {
                amount0_delta,
                amount1_delta,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SWAP_CALLBACK_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.amount0_delta, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.amount1_delta, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn swap_callback_ix_with_program_id(
    program_id: Pubkey,
    keys: SwapCallbackKeys,
    args: SwapCallbackIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SWAP_CALLBACK_IX_ACCOUNTS_LEN] = keys.into();
    let data: SwapCallbackIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn swap_callback_ix(
    keys: SwapCallbackKeys,
    args: SwapCallbackIxArgs,
) -> std::io::Result<Instruction> {
    swap_callback_ix_with_program_id(CYKURA_PROGRAM_ID, keys, args)
}
pub fn swap_callback_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SwapCallbackAccounts<'_, '_>,
    args: SwapCallbackIxArgs,
) -> ProgramResult {
    let keys: SwapCallbackKeys = accounts.into();
    let ix = swap_callback_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn swap_callback_invoke(
    accounts: SwapCallbackAccounts<'_, '_>,
    args: SwapCallbackIxArgs,
) -> ProgramResult {
    swap_callback_invoke_with_program_id(CYKURA_PROGRAM_ID, accounts, args)
}
pub fn swap_callback_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SwapCallbackAccounts<'_, '_>,
    args: SwapCallbackIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SwapCallbackKeys = accounts.into();
    let ix = swap_callback_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn swap_callback_invoke_signed(
    accounts: SwapCallbackAccounts<'_, '_>,
    args: SwapCallbackIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    swap_callback_invoke_signed_with_program_id(CYKURA_PROGRAM_ID, accounts, args, seeds)
}
pub fn swap_callback_verify_account_keys(
    accounts: SwapCallbackAccounts<'_, '_>,
    keys: SwapCallbackKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.signer.key, keys.signer),
        (*accounts.input_token_account.key, keys.input_token_account),
        (*accounts.output_token_account.key, keys.output_token_account),
        (*accounts.input_vault.key, keys.input_vault),
        (*accounts.output_vault.key, keys.output_vault),
        (*accounts.token_program.key, keys.token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn swap_callback_verify_writable_privileges<'me, 'info>(
    accounts: SwapCallbackAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.input_token_account,
        accounts.output_token_account,
        accounts.input_vault,
        accounts.output_vault,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn swap_callback_verify_signer_privileges<'me, 'info>(
    accounts: SwapCallbackAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.signer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn swap_callback_verify_account_privileges<'me, 'info>(
    accounts: SwapCallbackAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    swap_callback_verify_writable_privileges(accounts)?;
    swap_callback_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const MINT_IX_ACCOUNTS_LEN: usize = 15;
#[derive(Copy, Clone, Debug)]
pub struct MintAccounts<'me, 'info> {
    pub minter: &'me AccountInfo<'info>,
    pub token_account0: &'me AccountInfo<'info>,
    pub token_account1: &'me AccountInfo<'info>,
    pub vault0: &'me AccountInfo<'info>,
    pub vault1: &'me AccountInfo<'info>,
    pub recipient: &'me AccountInfo<'info>,
    pub pool_state: &'me AccountInfo<'info>,
    pub tick_lower_state: &'me AccountInfo<'info>,
    pub tick_upper_state: &'me AccountInfo<'info>,
    pub bitmap_lower_state: &'me AccountInfo<'info>,
    pub bitmap_upper_state: &'me AccountInfo<'info>,
    pub position_state: &'me AccountInfo<'info>,
    pub last_observation_state: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub callback_handler: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MintKeys {
    pub minter: Pubkey,
    pub token_account0: Pubkey,
    pub token_account1: Pubkey,
    pub vault0: Pubkey,
    pub vault1: Pubkey,
    pub recipient: Pubkey,
    pub pool_state: Pubkey,
    pub tick_lower_state: Pubkey,
    pub tick_upper_state: Pubkey,
    pub bitmap_lower_state: Pubkey,
    pub bitmap_upper_state: Pubkey,
    pub position_state: Pubkey,
    pub last_observation_state: Pubkey,
    pub token_program: Pubkey,
    pub callback_handler: Pubkey,
}
impl From<MintAccounts<'_, '_>> for MintKeys {
    fn from(accounts: MintAccounts) -> Self {
        Self {
            minter: *accounts.minter.key,
            token_account0: *accounts.token_account0.key,
            token_account1: *accounts.token_account1.key,
            vault0: *accounts.vault0.key,
            vault1: *accounts.vault1.key,
            recipient: *accounts.recipient.key,
            pool_state: *accounts.pool_state.key,
            tick_lower_state: *accounts.tick_lower_state.key,
            tick_upper_state: *accounts.tick_upper_state.key,
            bitmap_lower_state: *accounts.bitmap_lower_state.key,
            bitmap_upper_state: *accounts.bitmap_upper_state.key,
            position_state: *accounts.position_state.key,
            last_observation_state: *accounts.last_observation_state.key,
            token_program: *accounts.token_program.key,
            callback_handler: *accounts.callback_handler.key,
        }
    }
}
impl From<MintKeys> for [AccountMeta; MINT_IX_ACCOUNTS_LEN] {
    fn from(keys: MintKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.minter,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_account0,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_account1,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vault0,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vault1,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.recipient,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.tick_lower_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.tick_upper_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.bitmap_lower_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.bitmap_upper_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.position_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.last_observation_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.callback_handler,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; MINT_IX_ACCOUNTS_LEN]> for MintKeys {
    fn from(pubkeys: [Pubkey; MINT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            minter: pubkeys[0],
            token_account0: pubkeys[1],
            token_account1: pubkeys[2],
            vault0: pubkeys[3],
            vault1: pubkeys[4],
            recipient: pubkeys[5],
            pool_state: pubkeys[6],
            tick_lower_state: pubkeys[7],
            tick_upper_state: pubkeys[8],
            bitmap_lower_state: pubkeys[9],
            bitmap_upper_state: pubkeys[10],
            position_state: pubkeys[11],
            last_observation_state: pubkeys[12],
            token_program: pubkeys[13],
            callback_handler: pubkeys[14],
        }
    }
}
impl<'info> From<MintAccounts<'_, 'info>>
for [AccountInfo<'info>; MINT_IX_ACCOUNTS_LEN] {
    fn from(accounts: MintAccounts<'_, 'info>) -> Self {
        [
            accounts.minter.clone(),
            accounts.token_account0.clone(),
            accounts.token_account1.clone(),
            accounts.vault0.clone(),
            accounts.vault1.clone(),
            accounts.recipient.clone(),
            accounts.pool_state.clone(),
            accounts.tick_lower_state.clone(),
            accounts.tick_upper_state.clone(),
            accounts.bitmap_lower_state.clone(),
            accounts.bitmap_upper_state.clone(),
            accounts.position_state.clone(),
            accounts.last_observation_state.clone(),
            accounts.token_program.clone(),
            accounts.callback_handler.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; MINT_IX_ACCOUNTS_LEN]>
for MintAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; MINT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            minter: &arr[0],
            token_account0: &arr[1],
            token_account1: &arr[2],
            vault0: &arr[3],
            vault1: &arr[4],
            recipient: &arr[5],
            pool_state: &arr[6],
            tick_lower_state: &arr[7],
            tick_upper_state: &arr[8],
            bitmap_lower_state: &arr[9],
            bitmap_upper_state: &arr[10],
            position_state: &arr[11],
            last_observation_state: &arr[12],
            token_program: &arr[13],
            callback_handler: &arr[14],
        }
    }
}
pub const MINT_IX_DISCM: [u8; 8usize] = [51, 57, 225, 47, 182, 146, 137, 166];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MintIxArgs {
    pub amount: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct MintIxData(pub MintIxArgs);
impl From<MintIxArgs> for MintIxData {
    fn from(args: MintIxArgs) -> Self {
        Self(args)
    }
}
impl MintIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != MINT_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(Self(MintIxArgs { amount }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&MINT_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.amount, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn mint_ix_with_program_id(
    program_id: Pubkey,
    keys: MintKeys,
    args: MintIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; MINT_IX_ACCOUNTS_LEN] = keys.into();
    let data: MintIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn mint_ix(keys: MintKeys, args: MintIxArgs) -> std::io::Result<Instruction> {
    mint_ix_with_program_id(CYKURA_PROGRAM_ID, keys, args)
}
pub fn mint_invoke_with_program_id(
    program_id: Pubkey,
    accounts: MintAccounts<'_, '_>,
    args: MintIxArgs,
) -> ProgramResult {
    let keys: MintKeys = accounts.into();
    let ix = mint_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn mint_invoke(accounts: MintAccounts<'_, '_>, args: MintIxArgs) -> ProgramResult {
    mint_invoke_with_program_id(CYKURA_PROGRAM_ID, accounts, args)
}
pub fn mint_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: MintAccounts<'_, '_>,
    args: MintIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: MintKeys = accounts.into();
    let ix = mint_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn mint_invoke_signed(
    accounts: MintAccounts<'_, '_>,
    args: MintIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    mint_invoke_signed_with_program_id(CYKURA_PROGRAM_ID, accounts, args, seeds)
}
pub fn mint_verify_account_keys(
    accounts: MintAccounts<'_, '_>,
    keys: MintKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.minter.key, keys.minter),
        (*accounts.token_account0.key, keys.token_account0),
        (*accounts.token_account1.key, keys.token_account1),
        (*accounts.vault0.key, keys.vault0),
        (*accounts.vault1.key, keys.vault1),
        (*accounts.recipient.key, keys.recipient),
        (*accounts.pool_state.key, keys.pool_state),
        (*accounts.tick_lower_state.key, keys.tick_lower_state),
        (*accounts.tick_upper_state.key, keys.tick_upper_state),
        (*accounts.bitmap_lower_state.key, keys.bitmap_lower_state),
        (*accounts.bitmap_upper_state.key, keys.bitmap_upper_state),
        (*accounts.position_state.key, keys.position_state),
        (*accounts.last_observation_state.key, keys.last_observation_state),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.callback_handler.key, keys.callback_handler),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn mint_verify_writable_privileges<'me, 'info>(
    accounts: MintAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.token_account0,
        accounts.token_account1,
        accounts.vault0,
        accounts.vault1,
        accounts.pool_state,
        accounts.tick_lower_state,
        accounts.tick_upper_state,
        accounts.bitmap_lower_state,
        accounts.bitmap_upper_state,
        accounts.position_state,
        accounts.last_observation_state,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn mint_verify_signer_privileges<'me, 'info>(
    accounts: MintAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.minter] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn mint_verify_account_privileges<'me, 'info>(
    accounts: MintAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    mint_verify_writable_privileges(accounts)?;
    mint_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const BURN_IX_ACCOUNTS_LEN: usize = 8;
#[derive(Copy, Clone, Debug)]
pub struct BurnAccounts<'me, 'info> {
    pub owner: &'me AccountInfo<'info>,
    pub pool_state: &'me AccountInfo<'info>,
    pub tick_lower_state: &'me AccountInfo<'info>,
    pub tick_upper_state: &'me AccountInfo<'info>,
    pub bitmap_lower_state: &'me AccountInfo<'info>,
    pub bitmap_upper_state: &'me AccountInfo<'info>,
    pub position_state: &'me AccountInfo<'info>,
    pub last_observation_state: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct BurnKeys {
    pub owner: Pubkey,
    pub pool_state: Pubkey,
    pub tick_lower_state: Pubkey,
    pub tick_upper_state: Pubkey,
    pub bitmap_lower_state: Pubkey,
    pub bitmap_upper_state: Pubkey,
    pub position_state: Pubkey,
    pub last_observation_state: Pubkey,
}
impl From<BurnAccounts<'_, '_>> for BurnKeys {
    fn from(accounts: BurnAccounts) -> Self {
        Self {
            owner: *accounts.owner.key,
            pool_state: *accounts.pool_state.key,
            tick_lower_state: *accounts.tick_lower_state.key,
            tick_upper_state: *accounts.tick_upper_state.key,
            bitmap_lower_state: *accounts.bitmap_lower_state.key,
            bitmap_upper_state: *accounts.bitmap_upper_state.key,
            position_state: *accounts.position_state.key,
            last_observation_state: *accounts.last_observation_state.key,
        }
    }
}
impl From<BurnKeys> for [AccountMeta; BURN_IX_ACCOUNTS_LEN] {
    fn from(keys: BurnKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.owner,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.tick_lower_state,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.tick_upper_state,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.bitmap_lower_state,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.bitmap_upper_state,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.position_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.last_observation_state,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; BURN_IX_ACCOUNTS_LEN]> for BurnKeys {
    fn from(pubkeys: [Pubkey; BURN_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            owner: pubkeys[0],
            pool_state: pubkeys[1],
            tick_lower_state: pubkeys[2],
            tick_upper_state: pubkeys[3],
            bitmap_lower_state: pubkeys[4],
            bitmap_upper_state: pubkeys[5],
            position_state: pubkeys[6],
            last_observation_state: pubkeys[7],
        }
    }
}
impl<'info> From<BurnAccounts<'_, 'info>>
for [AccountInfo<'info>; BURN_IX_ACCOUNTS_LEN] {
    fn from(accounts: BurnAccounts<'_, 'info>) -> Self {
        [
            accounts.owner.clone(),
            accounts.pool_state.clone(),
            accounts.tick_lower_state.clone(),
            accounts.tick_upper_state.clone(),
            accounts.bitmap_lower_state.clone(),
            accounts.bitmap_upper_state.clone(),
            accounts.position_state.clone(),
            accounts.last_observation_state.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; BURN_IX_ACCOUNTS_LEN]>
for BurnAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; BURN_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            owner: &arr[0],
            pool_state: &arr[1],
            tick_lower_state: &arr[2],
            tick_upper_state: &arr[3],
            bitmap_lower_state: &arr[4],
            bitmap_upper_state: &arr[5],
            position_state: &arr[6],
            last_observation_state: &arr[7],
        }
    }
}
pub const BURN_IX_DISCM: [u8; 8usize] = [116, 110, 29, 56, 107, 219, 42, 93];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct BurnIxArgs {
    pub amount: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct BurnIxData(pub BurnIxArgs);
impl From<BurnIxArgs> for BurnIxData {
    fn from(args: BurnIxArgs) -> Self {
        Self(args)
    }
}
impl BurnIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != BURN_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(Self(BurnIxArgs { amount }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&BURN_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.amount, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn burn_ix_with_program_id(
    program_id: Pubkey,
    keys: BurnKeys,
    args: BurnIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; BURN_IX_ACCOUNTS_LEN] = keys.into();
    let data: BurnIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn burn_ix(keys: BurnKeys, args: BurnIxArgs) -> std::io::Result<Instruction> {
    burn_ix_with_program_id(CYKURA_PROGRAM_ID, keys, args)
}
pub fn burn_invoke_with_program_id(
    program_id: Pubkey,
    accounts: BurnAccounts<'_, '_>,
    args: BurnIxArgs,
) -> ProgramResult {
    let keys: BurnKeys = accounts.into();
    let ix = burn_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn burn_invoke(accounts: BurnAccounts<'_, '_>, args: BurnIxArgs) -> ProgramResult {
    burn_invoke_with_program_id(CYKURA_PROGRAM_ID, accounts, args)
}
pub fn burn_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: BurnAccounts<'_, '_>,
    args: BurnIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: BurnKeys = accounts.into();
    let ix = burn_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn burn_invoke_signed(
    accounts: BurnAccounts<'_, '_>,
    args: BurnIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    burn_invoke_signed_with_program_id(CYKURA_PROGRAM_ID, accounts, args, seeds)
}
pub fn burn_verify_account_keys(
    accounts: BurnAccounts<'_, '_>,
    keys: BurnKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.owner.key, keys.owner),
        (*accounts.pool_state.key, keys.pool_state),
        (*accounts.tick_lower_state.key, keys.tick_lower_state),
        (*accounts.tick_upper_state.key, keys.tick_upper_state),
        (*accounts.bitmap_lower_state.key, keys.bitmap_lower_state),
        (*accounts.bitmap_upper_state.key, keys.bitmap_upper_state),
        (*accounts.position_state.key, keys.position_state),
        (*accounts.last_observation_state.key, keys.last_observation_state),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn burn_verify_writable_privileges<'me, 'info>(
    accounts: BurnAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.pool_state, accounts.position_state] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn burn_verify_signer_privileges<'me, 'info>(
    accounts: BurnAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.owner] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn burn_verify_account_privileges<'me, 'info>(
    accounts: BurnAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    burn_verify_writable_privileges(accounts)?;
    burn_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const COLLECT_IX_ACCOUNTS_LEN: usize = 10;
#[derive(Copy, Clone, Debug)]
pub struct CollectAccounts<'me, 'info> {
    pub owner: &'me AccountInfo<'info>,
    pub pool_state: &'me AccountInfo<'info>,
    pub tick_lower_state: &'me AccountInfo<'info>,
    pub tick_upper_state: &'me AccountInfo<'info>,
    pub position_state: &'me AccountInfo<'info>,
    pub vault0: &'me AccountInfo<'info>,
    pub vault1: &'me AccountInfo<'info>,
    pub recipient_wallet0: &'me AccountInfo<'info>,
    pub recipient_wallet1: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CollectKeys {
    pub owner: Pubkey,
    pub pool_state: Pubkey,
    pub tick_lower_state: Pubkey,
    pub tick_upper_state: Pubkey,
    pub position_state: Pubkey,
    pub vault0: Pubkey,
    pub vault1: Pubkey,
    pub recipient_wallet0: Pubkey,
    pub recipient_wallet1: Pubkey,
    pub token_program: Pubkey,
}
impl From<CollectAccounts<'_, '_>> for CollectKeys {
    fn from(accounts: CollectAccounts) -> Self {
        Self {
            owner: *accounts.owner.key,
            pool_state: *accounts.pool_state.key,
            tick_lower_state: *accounts.tick_lower_state.key,
            tick_upper_state: *accounts.tick_upper_state.key,
            position_state: *accounts.position_state.key,
            vault0: *accounts.vault0.key,
            vault1: *accounts.vault1.key,
            recipient_wallet0: *accounts.recipient_wallet0.key,
            recipient_wallet1: *accounts.recipient_wallet1.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<CollectKeys> for [AccountMeta; COLLECT_IX_ACCOUNTS_LEN] {
    fn from(keys: CollectKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.owner,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.tick_lower_state,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.tick_upper_state,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.position_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vault0,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vault1,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.recipient_wallet0,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.recipient_wallet1,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; COLLECT_IX_ACCOUNTS_LEN]> for CollectKeys {
    fn from(pubkeys: [Pubkey; COLLECT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            owner: pubkeys[0],
            pool_state: pubkeys[1],
            tick_lower_state: pubkeys[2],
            tick_upper_state: pubkeys[3],
            position_state: pubkeys[4],
            vault0: pubkeys[5],
            vault1: pubkeys[6],
            recipient_wallet0: pubkeys[7],
            recipient_wallet1: pubkeys[8],
            token_program: pubkeys[9],
        }
    }
}
impl<'info> From<CollectAccounts<'_, 'info>>
for [AccountInfo<'info>; COLLECT_IX_ACCOUNTS_LEN] {
    fn from(accounts: CollectAccounts<'_, 'info>) -> Self {
        [
            accounts.owner.clone(),
            accounts.pool_state.clone(),
            accounts.tick_lower_state.clone(),
            accounts.tick_upper_state.clone(),
            accounts.position_state.clone(),
            accounts.vault0.clone(),
            accounts.vault1.clone(),
            accounts.recipient_wallet0.clone(),
            accounts.recipient_wallet1.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; COLLECT_IX_ACCOUNTS_LEN]>
for CollectAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; COLLECT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            owner: &arr[0],
            pool_state: &arr[1],
            tick_lower_state: &arr[2],
            tick_upper_state: &arr[3],
            position_state: &arr[4],
            vault0: &arr[5],
            vault1: &arr[6],
            recipient_wallet0: &arr[7],
            recipient_wallet1: &arr[8],
            token_program: &arr[9],
        }
    }
}
pub const COLLECT_IX_DISCM: [u8; 8usize] = [208, 47, 194, 155, 17, 98, 82, 236];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CollectIxArgs {
    pub amount0_requested: u64,
    pub amount1_requested: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct CollectIxData(pub CollectIxArgs);
impl From<CollectIxArgs> for CollectIxData {
    fn from(args: CollectIxArgs) -> Self {
        Self(args)
    }
}
impl CollectIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != COLLECT_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let amount0_requested: u64 = crate::borsh_de_or_default(&mut reader)?;
        let amount1_requested: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(CollectIxArgs {
                amount0_requested,
                amount1_requested,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&COLLECT_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.amount0_requested, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.amount1_requested, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn collect_ix_with_program_id(
    program_id: Pubkey,
    keys: CollectKeys,
    args: CollectIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; COLLECT_IX_ACCOUNTS_LEN] = keys.into();
    let data: CollectIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn collect_ix(
    keys: CollectKeys,
    args: CollectIxArgs,
) -> std::io::Result<Instruction> {
    collect_ix_with_program_id(CYKURA_PROGRAM_ID, keys, args)
}
pub fn collect_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CollectAccounts<'_, '_>,
    args: CollectIxArgs,
) -> ProgramResult {
    let keys: CollectKeys = accounts.into();
    let ix = collect_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn collect_invoke(
    accounts: CollectAccounts<'_, '_>,
    args: CollectIxArgs,
) -> ProgramResult {
    collect_invoke_with_program_id(CYKURA_PROGRAM_ID, accounts, args)
}
pub fn collect_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CollectAccounts<'_, '_>,
    args: CollectIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CollectKeys = accounts.into();
    let ix = collect_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn collect_invoke_signed(
    accounts: CollectAccounts<'_, '_>,
    args: CollectIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    collect_invoke_signed_with_program_id(CYKURA_PROGRAM_ID, accounts, args, seeds)
}
pub fn collect_verify_account_keys(
    accounts: CollectAccounts<'_, '_>,
    keys: CollectKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.owner.key, keys.owner),
        (*accounts.pool_state.key, keys.pool_state),
        (*accounts.tick_lower_state.key, keys.tick_lower_state),
        (*accounts.tick_upper_state.key, keys.tick_upper_state),
        (*accounts.position_state.key, keys.position_state),
        (*accounts.vault0.key, keys.vault0),
        (*accounts.vault1.key, keys.vault1),
        (*accounts.recipient_wallet0.key, keys.recipient_wallet0),
        (*accounts.recipient_wallet1.key, keys.recipient_wallet1),
        (*accounts.token_program.key, keys.token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn collect_verify_writable_privileges<'me, 'info>(
    accounts: CollectAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.pool_state,
        accounts.position_state,
        accounts.vault0,
        accounts.vault1,
        accounts.recipient_wallet0,
        accounts.recipient_wallet1,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn collect_verify_signer_privileges<'me, 'info>(
    accounts: CollectAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.owner] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn collect_verify_account_privileges<'me, 'info>(
    accounts: CollectAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    collect_verify_writable_privileges(accounts)?;
    collect_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SWAP_IX_ACCOUNTS_LEN: usize = 10;
#[derive(Copy, Clone, Debug)]
pub struct SwapAccounts<'me, 'info> {
    pub signer: &'me AccountInfo<'info>,
    pub input_token_account: &'me AccountInfo<'info>,
    pub output_token_account: &'me AccountInfo<'info>,
    pub input_vault: &'me AccountInfo<'info>,
    pub output_vault: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub factory_state: &'me AccountInfo<'info>,
    pub pool_state: &'me AccountInfo<'info>,
    pub last_observation_state: &'me AccountInfo<'info>,
    pub callback_handler: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SwapKeys {
    pub signer: Pubkey,
    pub input_token_account: Pubkey,
    pub output_token_account: Pubkey,
    pub input_vault: Pubkey,
    pub output_vault: Pubkey,
    pub token_program: Pubkey,
    pub factory_state: Pubkey,
    pub pool_state: Pubkey,
    pub last_observation_state: Pubkey,
    pub callback_handler: Pubkey,
}
impl From<SwapAccounts<'_, '_>> for SwapKeys {
    fn from(accounts: SwapAccounts) -> Self {
        Self {
            signer: *accounts.signer.key,
            input_token_account: *accounts.input_token_account.key,
            output_token_account: *accounts.output_token_account.key,
            input_vault: *accounts.input_vault.key,
            output_vault: *accounts.output_vault.key,
            token_program: *accounts.token_program.key,
            factory_state: *accounts.factory_state.key,
            pool_state: *accounts.pool_state.key,
            last_observation_state: *accounts.last_observation_state.key,
            callback_handler: *accounts.callback_handler.key,
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
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.factory_state,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.last_observation_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.callback_handler,
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
            input_token_account: pubkeys[1],
            output_token_account: pubkeys[2],
            input_vault: pubkeys[3],
            output_vault: pubkeys[4],
            token_program: pubkeys[5],
            factory_state: pubkeys[6],
            pool_state: pubkeys[7],
            last_observation_state: pubkeys[8],
            callback_handler: pubkeys[9],
        }
    }
}
impl<'info> From<SwapAccounts<'_, 'info>>
for [AccountInfo<'info>; SWAP_IX_ACCOUNTS_LEN] {
    fn from(accounts: SwapAccounts<'_, 'info>) -> Self {
        [
            accounts.signer.clone(),
            accounts.input_token_account.clone(),
            accounts.output_token_account.clone(),
            accounts.input_vault.clone(),
            accounts.output_vault.clone(),
            accounts.token_program.clone(),
            accounts.factory_state.clone(),
            accounts.pool_state.clone(),
            accounts.last_observation_state.clone(),
            accounts.callback_handler.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SWAP_IX_ACCOUNTS_LEN]>
for SwapAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; SWAP_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            signer: &arr[0],
            input_token_account: &arr[1],
            output_token_account: &arr[2],
            input_vault: &arr[3],
            output_vault: &arr[4],
            token_program: &arr[5],
            factory_state: &arr[6],
            pool_state: &arr[7],
            last_observation_state: &arr[8],
            callback_handler: &arr[9],
        }
    }
}
pub const SWAP_IX_DISCM: [u8; 8usize] = [248, 198, 158, 145, 225, 117, 135, 200];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SwapIxArgs {
    pub amount_specified: i64,
    pub sqrt_price_limit_x32: u64,
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
        let amount_specified: i64 = crate::borsh_de_or_default(&mut reader)?;
        let sqrt_price_limit_x32: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(SwapIxArgs {
                amount_specified,
                sqrt_price_limit_x32,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SWAP_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.amount_specified, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.sqrt_price_limit_x32, &mut writer)?;
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
    swap_ix_with_program_id(CYKURA_PROGRAM_ID, keys, args)
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
    swap_invoke_with_program_id(CYKURA_PROGRAM_ID, accounts, args)
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
    swap_invoke_signed_with_program_id(CYKURA_PROGRAM_ID, accounts, args, seeds)
}
pub fn swap_verify_account_keys(
    accounts: SwapAccounts<'_, '_>,
    keys: SwapKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.signer.key, keys.signer),
        (*accounts.input_token_account.key, keys.input_token_account),
        (*accounts.output_token_account.key, keys.output_token_account),
        (*accounts.input_vault.key, keys.input_vault),
        (*accounts.output_vault.key, keys.output_vault),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.factory_state.key, keys.factory_state),
        (*accounts.pool_state.key, keys.pool_state),
        (*accounts.last_observation_state.key, keys.last_observation_state),
        (*accounts.callback_handler.key, keys.callback_handler),
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
        accounts.input_token_account,
        accounts.output_token_account,
        accounts.input_vault,
        accounts.output_vault,
        accounts.pool_state,
        accounts.last_observation_state,
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
    for should_be_signer in [accounts.signer] {
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
pub const MINT_TOKENIZED_POSITION_IX_ACCOUNTS_LEN: usize = 22;
#[derive(Copy, Clone, Debug)]
pub struct MintTokenizedPositionAccounts<'me, 'info> {
    pub minter: &'me AccountInfo<'info>,
    pub recipient: &'me AccountInfo<'info>,
    pub factory_state: &'me AccountInfo<'info>,
    pub nft_mint: &'me AccountInfo<'info>,
    pub nft_account: &'me AccountInfo<'info>,
    pub pool_state: &'me AccountInfo<'info>,
    pub core_position_state: &'me AccountInfo<'info>,
    pub tick_lower_state: &'me AccountInfo<'info>,
    pub tick_upper_state: &'me AccountInfo<'info>,
    pub bitmap_lower_state: &'me AccountInfo<'info>,
    pub bitmap_upper_state: &'me AccountInfo<'info>,
    pub tokenized_position_state: &'me AccountInfo<'info>,
    pub token_account0: &'me AccountInfo<'info>,
    pub token_account1: &'me AccountInfo<'info>,
    pub vault0: &'me AccountInfo<'info>,
    pub vault1: &'me AccountInfo<'info>,
    pub last_observation_state: &'me AccountInfo<'info>,
    pub rent: &'me AccountInfo<'info>,
    pub core_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MintTokenizedPositionKeys {
    pub minter: Pubkey,
    pub recipient: Pubkey,
    pub factory_state: Pubkey,
    pub nft_mint: Pubkey,
    pub nft_account: Pubkey,
    pub pool_state: Pubkey,
    pub core_position_state: Pubkey,
    pub tick_lower_state: Pubkey,
    pub tick_upper_state: Pubkey,
    pub bitmap_lower_state: Pubkey,
    pub bitmap_upper_state: Pubkey,
    pub tokenized_position_state: Pubkey,
    pub token_account0: Pubkey,
    pub token_account1: Pubkey,
    pub vault0: Pubkey,
    pub vault1: Pubkey,
    pub last_observation_state: Pubkey,
    pub rent: Pubkey,
    pub core_program: Pubkey,
    pub system_program: Pubkey,
    pub token_program: Pubkey,
    pub associated_token_program: Pubkey,
}
impl From<MintTokenizedPositionAccounts<'_, '_>> for MintTokenizedPositionKeys {
    fn from(accounts: MintTokenizedPositionAccounts) -> Self {
        Self {
            minter: *accounts.minter.key,
            recipient: *accounts.recipient.key,
            factory_state: *accounts.factory_state.key,
            nft_mint: *accounts.nft_mint.key,
            nft_account: *accounts.nft_account.key,
            pool_state: *accounts.pool_state.key,
            core_position_state: *accounts.core_position_state.key,
            tick_lower_state: *accounts.tick_lower_state.key,
            tick_upper_state: *accounts.tick_upper_state.key,
            bitmap_lower_state: *accounts.bitmap_lower_state.key,
            bitmap_upper_state: *accounts.bitmap_upper_state.key,
            tokenized_position_state: *accounts.tokenized_position_state.key,
            token_account0: *accounts.token_account0.key,
            token_account1: *accounts.token_account1.key,
            vault0: *accounts.vault0.key,
            vault1: *accounts.vault1.key,
            last_observation_state: *accounts.last_observation_state.key,
            rent: *accounts.rent.key,
            core_program: *accounts.core_program.key,
            system_program: *accounts.system_program.key,
            token_program: *accounts.token_program.key,
            associated_token_program: *accounts.associated_token_program.key,
        }
    }
}
impl From<MintTokenizedPositionKeys>
for [AccountMeta; MINT_TOKENIZED_POSITION_IX_ACCOUNTS_LEN] {
    fn from(keys: MintTokenizedPositionKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.minter,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.recipient,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.factory_state,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.nft_mint,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.nft_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pool_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.core_position_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.tick_lower_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.tick_upper_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.bitmap_lower_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.bitmap_upper_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.tokenized_position_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_account0,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_account1,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vault0,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vault1,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.last_observation_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.rent,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.core_program,
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
        ]
    }
}
impl From<[Pubkey; MINT_TOKENIZED_POSITION_IX_ACCOUNTS_LEN]>
for MintTokenizedPositionKeys {
    fn from(pubkeys: [Pubkey; MINT_TOKENIZED_POSITION_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            minter: pubkeys[0],
            recipient: pubkeys[1],
            factory_state: pubkeys[2],
            nft_mint: pubkeys[3],
            nft_account: pubkeys[4],
            pool_state: pubkeys[5],
            core_position_state: pubkeys[6],
            tick_lower_state: pubkeys[7],
            tick_upper_state: pubkeys[8],
            bitmap_lower_state: pubkeys[9],
            bitmap_upper_state: pubkeys[10],
            tokenized_position_state: pubkeys[11],
            token_account0: pubkeys[12],
            token_account1: pubkeys[13],
            vault0: pubkeys[14],
            vault1: pubkeys[15],
            last_observation_state: pubkeys[16],
            rent: pubkeys[17],
            core_program: pubkeys[18],
            system_program: pubkeys[19],
            token_program: pubkeys[20],
            associated_token_program: pubkeys[21],
        }
    }
}
impl<'info> From<MintTokenizedPositionAccounts<'_, 'info>>
for [AccountInfo<'info>; MINT_TOKENIZED_POSITION_IX_ACCOUNTS_LEN] {
    fn from(accounts: MintTokenizedPositionAccounts<'_, 'info>) -> Self {
        [
            accounts.minter.clone(),
            accounts.recipient.clone(),
            accounts.factory_state.clone(),
            accounts.nft_mint.clone(),
            accounts.nft_account.clone(),
            accounts.pool_state.clone(),
            accounts.core_position_state.clone(),
            accounts.tick_lower_state.clone(),
            accounts.tick_upper_state.clone(),
            accounts.bitmap_lower_state.clone(),
            accounts.bitmap_upper_state.clone(),
            accounts.tokenized_position_state.clone(),
            accounts.token_account0.clone(),
            accounts.token_account1.clone(),
            accounts.vault0.clone(),
            accounts.vault1.clone(),
            accounts.last_observation_state.clone(),
            accounts.rent.clone(),
            accounts.core_program.clone(),
            accounts.system_program.clone(),
            accounts.token_program.clone(),
            accounts.associated_token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; MINT_TOKENIZED_POSITION_IX_ACCOUNTS_LEN]>
for MintTokenizedPositionAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; MINT_TOKENIZED_POSITION_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            minter: &arr[0],
            recipient: &arr[1],
            factory_state: &arr[2],
            nft_mint: &arr[3],
            nft_account: &arr[4],
            pool_state: &arr[5],
            core_position_state: &arr[6],
            tick_lower_state: &arr[7],
            tick_upper_state: &arr[8],
            bitmap_lower_state: &arr[9],
            bitmap_upper_state: &arr[10],
            tokenized_position_state: &arr[11],
            token_account0: &arr[12],
            token_account1: &arr[13],
            vault0: &arr[14],
            vault1: &arr[15],
            last_observation_state: &arr[16],
            rent: &arr[17],
            core_program: &arr[18],
            system_program: &arr[19],
            token_program: &arr[20],
            associated_token_program: &arr[21],
        }
    }
}
pub const MINT_TOKENIZED_POSITION_IX_DISCM: [u8; 8usize] = [
    165, 199, 148, 197, 212, 172, 211, 229,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MintTokenizedPositionIxArgs {
    pub amount0_desired: u64,
    pub amount1_desired: u64,
    pub amount0_min: u64,
    pub amount1_min: u64,
    pub deadline: i64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct MintTokenizedPositionIxData(pub MintTokenizedPositionIxArgs);
impl From<MintTokenizedPositionIxArgs> for MintTokenizedPositionIxData {
    fn from(args: MintTokenizedPositionIxArgs) -> Self {
        Self(args)
    }
}
impl MintTokenizedPositionIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != MINT_TOKENIZED_POSITION_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let amount0_desired: u64 = crate::borsh_de_or_default(&mut reader)?;
        let amount1_desired: u64 = crate::borsh_de_or_default(&mut reader)?;
        let amount0_min: u64 = crate::borsh_de_or_default(&mut reader)?;
        let amount1_min: u64 = crate::borsh_de_or_default(&mut reader)?;
        let deadline: i64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(MintTokenizedPositionIxArgs {
                amount0_desired,
                amount1_desired,
                amount0_min,
                amount1_min,
                deadline,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&MINT_TOKENIZED_POSITION_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.amount0_desired, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.amount1_desired, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.amount0_min, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.amount1_min, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.deadline, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn mint_tokenized_position_ix_with_program_id(
    program_id: Pubkey,
    keys: MintTokenizedPositionKeys,
    args: MintTokenizedPositionIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; MINT_TOKENIZED_POSITION_IX_ACCOUNTS_LEN] = keys.into();
    let data: MintTokenizedPositionIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn mint_tokenized_position_ix(
    keys: MintTokenizedPositionKeys,
    args: MintTokenizedPositionIxArgs,
) -> std::io::Result<Instruction> {
    mint_tokenized_position_ix_with_program_id(CYKURA_PROGRAM_ID, keys, args)
}
pub fn mint_tokenized_position_invoke_with_program_id(
    program_id: Pubkey,
    accounts: MintTokenizedPositionAccounts<'_, '_>,
    args: MintTokenizedPositionIxArgs,
) -> ProgramResult {
    let keys: MintTokenizedPositionKeys = accounts.into();
    let ix = mint_tokenized_position_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn mint_tokenized_position_invoke(
    accounts: MintTokenizedPositionAccounts<'_, '_>,
    args: MintTokenizedPositionIxArgs,
) -> ProgramResult {
    mint_tokenized_position_invoke_with_program_id(CYKURA_PROGRAM_ID, accounts, args)
}
pub fn mint_tokenized_position_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: MintTokenizedPositionAccounts<'_, '_>,
    args: MintTokenizedPositionIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: MintTokenizedPositionKeys = accounts.into();
    let ix = mint_tokenized_position_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn mint_tokenized_position_invoke_signed(
    accounts: MintTokenizedPositionAccounts<'_, '_>,
    args: MintTokenizedPositionIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    mint_tokenized_position_invoke_signed_with_program_id(
        CYKURA_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn mint_tokenized_position_verify_account_keys(
    accounts: MintTokenizedPositionAccounts<'_, '_>,
    keys: MintTokenizedPositionKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.minter.key, keys.minter),
        (*accounts.recipient.key, keys.recipient),
        (*accounts.factory_state.key, keys.factory_state),
        (*accounts.nft_mint.key, keys.nft_mint),
        (*accounts.nft_account.key, keys.nft_account),
        (*accounts.pool_state.key, keys.pool_state),
        (*accounts.core_position_state.key, keys.core_position_state),
        (*accounts.tick_lower_state.key, keys.tick_lower_state),
        (*accounts.tick_upper_state.key, keys.tick_upper_state),
        (*accounts.bitmap_lower_state.key, keys.bitmap_lower_state),
        (*accounts.bitmap_upper_state.key, keys.bitmap_upper_state),
        (*accounts.tokenized_position_state.key, keys.tokenized_position_state),
        (*accounts.token_account0.key, keys.token_account0),
        (*accounts.token_account1.key, keys.token_account1),
        (*accounts.vault0.key, keys.vault0),
        (*accounts.vault1.key, keys.vault1),
        (*accounts.last_observation_state.key, keys.last_observation_state),
        (*accounts.rent.key, keys.rent),
        (*accounts.core_program.key, keys.core_program),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.associated_token_program.key, keys.associated_token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn mint_tokenized_position_verify_writable_privileges<'me, 'info>(
    accounts: MintTokenizedPositionAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.minter,
        accounts.nft_mint,
        accounts.nft_account,
        accounts.pool_state,
        accounts.core_position_state,
        accounts.tick_lower_state,
        accounts.tick_upper_state,
        accounts.bitmap_lower_state,
        accounts.bitmap_upper_state,
        accounts.tokenized_position_state,
        accounts.token_account0,
        accounts.token_account1,
        accounts.vault0,
        accounts.vault1,
        accounts.last_observation_state,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn mint_tokenized_position_verify_signer_privileges<'me, 'info>(
    accounts: MintTokenizedPositionAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.minter, accounts.nft_mint] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn mint_tokenized_position_verify_account_privileges<'me, 'info>(
    accounts: MintTokenizedPositionAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    mint_tokenized_position_verify_writable_privileges(accounts)?;
    mint_tokenized_position_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const ADD_METAPLEX_METADATA_IX_ACCOUNTS_LEN: usize = 9;
#[derive(Copy, Clone, Debug)]
pub struct AddMetaplexMetadataAccounts<'me, 'info> {
    pub payer: &'me AccountInfo<'info>,
    pub factory_state: &'me AccountInfo<'info>,
    pub nft_mint: &'me AccountInfo<'info>,
    pub tokenized_position_state: &'me AccountInfo<'info>,
    pub metadata_account: &'me AccountInfo<'info>,
    pub rent: &'me AccountInfo<'info>,
    pub metadata_program: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct AddMetaplexMetadataKeys {
    pub payer: Pubkey,
    pub factory_state: Pubkey,
    pub nft_mint: Pubkey,
    pub tokenized_position_state: Pubkey,
    pub metadata_account: Pubkey,
    pub rent: Pubkey,
    pub metadata_program: Pubkey,
    pub token_program: Pubkey,
    pub system_program: Pubkey,
}
impl From<AddMetaplexMetadataAccounts<'_, '_>> for AddMetaplexMetadataKeys {
    fn from(accounts: AddMetaplexMetadataAccounts) -> Self {
        Self {
            payer: *accounts.payer.key,
            factory_state: *accounts.factory_state.key,
            nft_mint: *accounts.nft_mint.key,
            tokenized_position_state: *accounts.tokenized_position_state.key,
            metadata_account: *accounts.metadata_account.key,
            rent: *accounts.rent.key,
            metadata_program: *accounts.metadata_program.key,
            token_program: *accounts.token_program.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<AddMetaplexMetadataKeys>
for [AccountMeta; ADD_METAPLEX_METADATA_IX_ACCOUNTS_LEN] {
    fn from(keys: AddMetaplexMetadataKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.payer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.factory_state,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.nft_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.tokenized_position_state,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.metadata_account,
                is_signer: false,
                is_writable: true,
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
                pubkey: keys.token_program,
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
impl From<[Pubkey; ADD_METAPLEX_METADATA_IX_ACCOUNTS_LEN]> for AddMetaplexMetadataKeys {
    fn from(pubkeys: [Pubkey; ADD_METAPLEX_METADATA_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            payer: pubkeys[0],
            factory_state: pubkeys[1],
            nft_mint: pubkeys[2],
            tokenized_position_state: pubkeys[3],
            metadata_account: pubkeys[4],
            rent: pubkeys[5],
            metadata_program: pubkeys[6],
            token_program: pubkeys[7],
            system_program: pubkeys[8],
        }
    }
}
impl<'info> From<AddMetaplexMetadataAccounts<'_, 'info>>
for [AccountInfo<'info>; ADD_METAPLEX_METADATA_IX_ACCOUNTS_LEN] {
    fn from(accounts: AddMetaplexMetadataAccounts<'_, 'info>) -> Self {
        [
            accounts.payer.clone(),
            accounts.factory_state.clone(),
            accounts.nft_mint.clone(),
            accounts.tokenized_position_state.clone(),
            accounts.metadata_account.clone(),
            accounts.rent.clone(),
            accounts.metadata_program.clone(),
            accounts.token_program.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; ADD_METAPLEX_METADATA_IX_ACCOUNTS_LEN]>
for AddMetaplexMetadataAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; ADD_METAPLEX_METADATA_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            payer: &arr[0],
            factory_state: &arr[1],
            nft_mint: &arr[2],
            tokenized_position_state: &arr[3],
            metadata_account: &arr[4],
            rent: &arr[5],
            metadata_program: &arr[6],
            token_program: &arr[7],
            system_program: &arr[8],
        }
    }
}
pub const ADD_METAPLEX_METADATA_IX_DISCM: [u8; 8usize] = [
    147, 78, 151, 93, 131, 33, 84, 69,
];
#[derive(Clone, Debug, PartialEq)]
pub struct AddMetaplexMetadataIxData;
impl AddMetaplexMetadataIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != ADD_METAPLEX_METADATA_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&ADD_METAPLEX_METADATA_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn add_metaplex_metadata_ix_with_program_id(
    program_id: Pubkey,
    keys: AddMetaplexMetadataKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; ADD_METAPLEX_METADATA_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: AddMetaplexMetadataIxData.try_to_vec()?,
    })
}
pub fn add_metaplex_metadata_ix(
    keys: AddMetaplexMetadataKeys,
) -> std::io::Result<Instruction> {
    add_metaplex_metadata_ix_with_program_id(CYKURA_PROGRAM_ID, keys)
}
pub fn add_metaplex_metadata_invoke_with_program_id(
    program_id: Pubkey,
    accounts: AddMetaplexMetadataAccounts<'_, '_>,
) -> ProgramResult {
    let keys: AddMetaplexMetadataKeys = accounts.into();
    let ix = add_metaplex_metadata_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn add_metaplex_metadata_invoke(
    accounts: AddMetaplexMetadataAccounts<'_, '_>,
) -> ProgramResult {
    add_metaplex_metadata_invoke_with_program_id(CYKURA_PROGRAM_ID, accounts)
}
pub fn add_metaplex_metadata_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: AddMetaplexMetadataAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: AddMetaplexMetadataKeys = accounts.into();
    let ix = add_metaplex_metadata_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn add_metaplex_metadata_invoke_signed(
    accounts: AddMetaplexMetadataAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    add_metaplex_metadata_invoke_signed_with_program_id(
        CYKURA_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn add_metaplex_metadata_verify_account_keys(
    accounts: AddMetaplexMetadataAccounts<'_, '_>,
    keys: AddMetaplexMetadataKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.payer.key, keys.payer),
        (*accounts.factory_state.key, keys.factory_state),
        (*accounts.nft_mint.key, keys.nft_mint),
        (*accounts.tokenized_position_state.key, keys.tokenized_position_state),
        (*accounts.metadata_account.key, keys.metadata_account),
        (*accounts.rent.key, keys.rent),
        (*accounts.metadata_program.key, keys.metadata_program),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn add_metaplex_metadata_verify_writable_privileges<'me, 'info>(
    accounts: AddMetaplexMetadataAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.payer,
        accounts.nft_mint,
        accounts.metadata_account,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn add_metaplex_metadata_verify_signer_privileges<'me, 'info>(
    accounts: AddMetaplexMetadataAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.payer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn add_metaplex_metadata_verify_account_privileges<'me, 'info>(
    accounts: AddMetaplexMetadataAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    add_metaplex_metadata_verify_writable_privileges(accounts)?;
    add_metaplex_metadata_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const INCREASE_LIQUIDITY_IX_ACCOUNTS_LEN: usize = 16;
#[derive(Copy, Clone, Debug)]
pub struct IncreaseLiquidityAccounts<'me, 'info> {
    pub payer: &'me AccountInfo<'info>,
    pub factory_state: &'me AccountInfo<'info>,
    pub tokenized_position_state: &'me AccountInfo<'info>,
    pub pool_state: &'me AccountInfo<'info>,
    pub core_position_state: &'me AccountInfo<'info>,
    pub tick_lower_state: &'me AccountInfo<'info>,
    pub tick_upper_state: &'me AccountInfo<'info>,
    pub bitmap_lower_state: &'me AccountInfo<'info>,
    pub bitmap_upper_state: &'me AccountInfo<'info>,
    pub token_account0: &'me AccountInfo<'info>,
    pub token_account1: &'me AccountInfo<'info>,
    pub vault0: &'me AccountInfo<'info>,
    pub vault1: &'me AccountInfo<'info>,
    pub last_observation_state: &'me AccountInfo<'info>,
    pub core_program: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct IncreaseLiquidityKeys {
    pub payer: Pubkey,
    pub factory_state: Pubkey,
    pub tokenized_position_state: Pubkey,
    pub pool_state: Pubkey,
    pub core_position_state: Pubkey,
    pub tick_lower_state: Pubkey,
    pub tick_upper_state: Pubkey,
    pub bitmap_lower_state: Pubkey,
    pub bitmap_upper_state: Pubkey,
    pub token_account0: Pubkey,
    pub token_account1: Pubkey,
    pub vault0: Pubkey,
    pub vault1: Pubkey,
    pub last_observation_state: Pubkey,
    pub core_program: Pubkey,
    pub token_program: Pubkey,
}
impl From<IncreaseLiquidityAccounts<'_, '_>> for IncreaseLiquidityKeys {
    fn from(accounts: IncreaseLiquidityAccounts) -> Self {
        Self {
            payer: *accounts.payer.key,
            factory_state: *accounts.factory_state.key,
            tokenized_position_state: *accounts.tokenized_position_state.key,
            pool_state: *accounts.pool_state.key,
            core_position_state: *accounts.core_position_state.key,
            tick_lower_state: *accounts.tick_lower_state.key,
            tick_upper_state: *accounts.tick_upper_state.key,
            bitmap_lower_state: *accounts.bitmap_lower_state.key,
            bitmap_upper_state: *accounts.bitmap_upper_state.key,
            token_account0: *accounts.token_account0.key,
            token_account1: *accounts.token_account1.key,
            vault0: *accounts.vault0.key,
            vault1: *accounts.vault1.key,
            last_observation_state: *accounts.last_observation_state.key,
            core_program: *accounts.core_program.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<IncreaseLiquidityKeys> for [AccountMeta; INCREASE_LIQUIDITY_IX_ACCOUNTS_LEN] {
    fn from(keys: IncreaseLiquidityKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.payer,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.factory_state,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.tokenized_position_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pool_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.core_position_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.tick_lower_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.tick_upper_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.bitmap_lower_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.bitmap_upper_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_account0,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_account1,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vault0,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vault1,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.last_observation_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.core_program,
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
impl From<[Pubkey; INCREASE_LIQUIDITY_IX_ACCOUNTS_LEN]> for IncreaseLiquidityKeys {
    fn from(pubkeys: [Pubkey; INCREASE_LIQUIDITY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            payer: pubkeys[0],
            factory_state: pubkeys[1],
            tokenized_position_state: pubkeys[2],
            pool_state: pubkeys[3],
            core_position_state: pubkeys[4],
            tick_lower_state: pubkeys[5],
            tick_upper_state: pubkeys[6],
            bitmap_lower_state: pubkeys[7],
            bitmap_upper_state: pubkeys[8],
            token_account0: pubkeys[9],
            token_account1: pubkeys[10],
            vault0: pubkeys[11],
            vault1: pubkeys[12],
            last_observation_state: pubkeys[13],
            core_program: pubkeys[14],
            token_program: pubkeys[15],
        }
    }
}
impl<'info> From<IncreaseLiquidityAccounts<'_, 'info>>
for [AccountInfo<'info>; INCREASE_LIQUIDITY_IX_ACCOUNTS_LEN] {
    fn from(accounts: IncreaseLiquidityAccounts<'_, 'info>) -> Self {
        [
            accounts.payer.clone(),
            accounts.factory_state.clone(),
            accounts.tokenized_position_state.clone(),
            accounts.pool_state.clone(),
            accounts.core_position_state.clone(),
            accounts.tick_lower_state.clone(),
            accounts.tick_upper_state.clone(),
            accounts.bitmap_lower_state.clone(),
            accounts.bitmap_upper_state.clone(),
            accounts.token_account0.clone(),
            accounts.token_account1.clone(),
            accounts.vault0.clone(),
            accounts.vault1.clone(),
            accounts.last_observation_state.clone(),
            accounts.core_program.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; INCREASE_LIQUIDITY_IX_ACCOUNTS_LEN]>
for IncreaseLiquidityAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; INCREASE_LIQUIDITY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            payer: &arr[0],
            factory_state: &arr[1],
            tokenized_position_state: &arr[2],
            pool_state: &arr[3],
            core_position_state: &arr[4],
            tick_lower_state: &arr[5],
            tick_upper_state: &arr[6],
            bitmap_lower_state: &arr[7],
            bitmap_upper_state: &arr[8],
            token_account0: &arr[9],
            token_account1: &arr[10],
            vault0: &arr[11],
            vault1: &arr[12],
            last_observation_state: &arr[13],
            core_program: &arr[14],
            token_program: &arr[15],
        }
    }
}
pub const INCREASE_LIQUIDITY_IX_DISCM: [u8; 8usize] = [
    46, 156, 243, 118, 13, 205, 251, 178,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct IncreaseLiquidityIxArgs {
    pub amount0_desired: u64,
    pub amount1_desired: u64,
    pub amount0_min: u64,
    pub amount1_min: u64,
    pub deadline: i64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct IncreaseLiquidityIxData(pub IncreaseLiquidityIxArgs);
impl From<IncreaseLiquidityIxArgs> for IncreaseLiquidityIxData {
    fn from(args: IncreaseLiquidityIxArgs) -> Self {
        Self(args)
    }
}
impl IncreaseLiquidityIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INCREASE_LIQUIDITY_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let amount0_desired: u64 = crate::borsh_de_or_default(&mut reader)?;
        let amount1_desired: u64 = crate::borsh_de_or_default(&mut reader)?;
        let amount0_min: u64 = crate::borsh_de_or_default(&mut reader)?;
        let amount1_min: u64 = crate::borsh_de_or_default(&mut reader)?;
        let deadline: i64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(IncreaseLiquidityIxArgs {
                amount0_desired,
                amount1_desired,
                amount0_min,
                amount1_min,
                deadline,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INCREASE_LIQUIDITY_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.amount0_desired, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.amount1_desired, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.amount0_min, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.amount1_min, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.deadline, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn increase_liquidity_ix_with_program_id(
    program_id: Pubkey,
    keys: IncreaseLiquidityKeys,
    args: IncreaseLiquidityIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; INCREASE_LIQUIDITY_IX_ACCOUNTS_LEN] = keys.into();
    let data: IncreaseLiquidityIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn increase_liquidity_ix(
    keys: IncreaseLiquidityKeys,
    args: IncreaseLiquidityIxArgs,
) -> std::io::Result<Instruction> {
    increase_liquidity_ix_with_program_id(CYKURA_PROGRAM_ID, keys, args)
}
pub fn increase_liquidity_invoke_with_program_id(
    program_id: Pubkey,
    accounts: IncreaseLiquidityAccounts<'_, '_>,
    args: IncreaseLiquidityIxArgs,
) -> ProgramResult {
    let keys: IncreaseLiquidityKeys = accounts.into();
    let ix = increase_liquidity_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn increase_liquidity_invoke(
    accounts: IncreaseLiquidityAccounts<'_, '_>,
    args: IncreaseLiquidityIxArgs,
) -> ProgramResult {
    increase_liquidity_invoke_with_program_id(CYKURA_PROGRAM_ID, accounts, args)
}
pub fn increase_liquidity_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: IncreaseLiquidityAccounts<'_, '_>,
    args: IncreaseLiquidityIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: IncreaseLiquidityKeys = accounts.into();
    let ix = increase_liquidity_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn increase_liquidity_invoke_signed(
    accounts: IncreaseLiquidityAccounts<'_, '_>,
    args: IncreaseLiquidityIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    increase_liquidity_invoke_signed_with_program_id(
        CYKURA_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn increase_liquidity_verify_account_keys(
    accounts: IncreaseLiquidityAccounts<'_, '_>,
    keys: IncreaseLiquidityKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.payer.key, keys.payer),
        (*accounts.factory_state.key, keys.factory_state),
        (*accounts.tokenized_position_state.key, keys.tokenized_position_state),
        (*accounts.pool_state.key, keys.pool_state),
        (*accounts.core_position_state.key, keys.core_position_state),
        (*accounts.tick_lower_state.key, keys.tick_lower_state),
        (*accounts.tick_upper_state.key, keys.tick_upper_state),
        (*accounts.bitmap_lower_state.key, keys.bitmap_lower_state),
        (*accounts.bitmap_upper_state.key, keys.bitmap_upper_state),
        (*accounts.token_account0.key, keys.token_account0),
        (*accounts.token_account1.key, keys.token_account1),
        (*accounts.vault0.key, keys.vault0),
        (*accounts.vault1.key, keys.vault1),
        (*accounts.last_observation_state.key, keys.last_observation_state),
        (*accounts.core_program.key, keys.core_program),
        (*accounts.token_program.key, keys.token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn increase_liquidity_verify_writable_privileges<'me, 'info>(
    accounts: IncreaseLiquidityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.tokenized_position_state,
        accounts.pool_state,
        accounts.core_position_state,
        accounts.tick_lower_state,
        accounts.tick_upper_state,
        accounts.bitmap_lower_state,
        accounts.bitmap_upper_state,
        accounts.token_account0,
        accounts.token_account1,
        accounts.vault0,
        accounts.vault1,
        accounts.last_observation_state,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn increase_liquidity_verify_signer_privileges<'me, 'info>(
    accounts: IncreaseLiquidityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.payer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn increase_liquidity_verify_account_privileges<'me, 'info>(
    accounts: IncreaseLiquidityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    increase_liquidity_verify_writable_privileges(accounts)?;
    increase_liquidity_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const DECREASE_LIQUIDITY_IX_ACCOUNTS_LEN: usize = 12;
#[derive(Copy, Clone, Debug)]
pub struct DecreaseLiquidityAccounts<'me, 'info> {
    pub owner_or_delegate: &'me AccountInfo<'info>,
    pub nft_account: &'me AccountInfo<'info>,
    pub tokenized_position_state: &'me AccountInfo<'info>,
    pub factory_state: &'me AccountInfo<'info>,
    pub pool_state: &'me AccountInfo<'info>,
    pub core_position_state: &'me AccountInfo<'info>,
    pub tick_lower_state: &'me AccountInfo<'info>,
    pub tick_upper_state: &'me AccountInfo<'info>,
    pub bitmap_lower_state: &'me AccountInfo<'info>,
    pub bitmap_upper_state: &'me AccountInfo<'info>,
    pub last_observation_state: &'me AccountInfo<'info>,
    pub core_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct DecreaseLiquidityKeys {
    pub owner_or_delegate: Pubkey,
    pub nft_account: Pubkey,
    pub tokenized_position_state: Pubkey,
    pub factory_state: Pubkey,
    pub pool_state: Pubkey,
    pub core_position_state: Pubkey,
    pub tick_lower_state: Pubkey,
    pub tick_upper_state: Pubkey,
    pub bitmap_lower_state: Pubkey,
    pub bitmap_upper_state: Pubkey,
    pub last_observation_state: Pubkey,
    pub core_program: Pubkey,
}
impl From<DecreaseLiquidityAccounts<'_, '_>> for DecreaseLiquidityKeys {
    fn from(accounts: DecreaseLiquidityAccounts) -> Self {
        Self {
            owner_or_delegate: *accounts.owner_or_delegate.key,
            nft_account: *accounts.nft_account.key,
            tokenized_position_state: *accounts.tokenized_position_state.key,
            factory_state: *accounts.factory_state.key,
            pool_state: *accounts.pool_state.key,
            core_position_state: *accounts.core_position_state.key,
            tick_lower_state: *accounts.tick_lower_state.key,
            tick_upper_state: *accounts.tick_upper_state.key,
            bitmap_lower_state: *accounts.bitmap_lower_state.key,
            bitmap_upper_state: *accounts.bitmap_upper_state.key,
            last_observation_state: *accounts.last_observation_state.key,
            core_program: *accounts.core_program.key,
        }
    }
}
impl From<DecreaseLiquidityKeys> for [AccountMeta; DECREASE_LIQUIDITY_IX_ACCOUNTS_LEN] {
    fn from(keys: DecreaseLiquidityKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.owner_or_delegate,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.nft_account,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.tokenized_position_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.factory_state,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.core_position_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.tick_lower_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.tick_upper_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.bitmap_lower_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.bitmap_upper_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.last_observation_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.core_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; DECREASE_LIQUIDITY_IX_ACCOUNTS_LEN]> for DecreaseLiquidityKeys {
    fn from(pubkeys: [Pubkey; DECREASE_LIQUIDITY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            owner_or_delegate: pubkeys[0],
            nft_account: pubkeys[1],
            tokenized_position_state: pubkeys[2],
            factory_state: pubkeys[3],
            pool_state: pubkeys[4],
            core_position_state: pubkeys[5],
            tick_lower_state: pubkeys[6],
            tick_upper_state: pubkeys[7],
            bitmap_lower_state: pubkeys[8],
            bitmap_upper_state: pubkeys[9],
            last_observation_state: pubkeys[10],
            core_program: pubkeys[11],
        }
    }
}
impl<'info> From<DecreaseLiquidityAccounts<'_, 'info>>
for [AccountInfo<'info>; DECREASE_LIQUIDITY_IX_ACCOUNTS_LEN] {
    fn from(accounts: DecreaseLiquidityAccounts<'_, 'info>) -> Self {
        [
            accounts.owner_or_delegate.clone(),
            accounts.nft_account.clone(),
            accounts.tokenized_position_state.clone(),
            accounts.factory_state.clone(),
            accounts.pool_state.clone(),
            accounts.core_position_state.clone(),
            accounts.tick_lower_state.clone(),
            accounts.tick_upper_state.clone(),
            accounts.bitmap_lower_state.clone(),
            accounts.bitmap_upper_state.clone(),
            accounts.last_observation_state.clone(),
            accounts.core_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; DECREASE_LIQUIDITY_IX_ACCOUNTS_LEN]>
for DecreaseLiquidityAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; DECREASE_LIQUIDITY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            owner_or_delegate: &arr[0],
            nft_account: &arr[1],
            tokenized_position_state: &arr[2],
            factory_state: &arr[3],
            pool_state: &arr[4],
            core_position_state: &arr[5],
            tick_lower_state: &arr[6],
            tick_upper_state: &arr[7],
            bitmap_lower_state: &arr[8],
            bitmap_upper_state: &arr[9],
            last_observation_state: &arr[10],
            core_program: &arr[11],
        }
    }
}
pub const DECREASE_LIQUIDITY_IX_DISCM: [u8; 8usize] = [
    160, 38, 208, 111, 104, 91, 44, 1,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DecreaseLiquidityIxArgs {
    pub liquidity: u64,
    pub amount0_min: u64,
    pub amount1_min: u64,
    pub deadline: i64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct DecreaseLiquidityIxData(pub DecreaseLiquidityIxArgs);
impl From<DecreaseLiquidityIxArgs> for DecreaseLiquidityIxData {
    fn from(args: DecreaseLiquidityIxArgs) -> Self {
        Self(args)
    }
}
impl DecreaseLiquidityIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != DECREASE_LIQUIDITY_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let liquidity: u64 = crate::borsh_de_or_default(&mut reader)?;
        let amount0_min: u64 = crate::borsh_de_or_default(&mut reader)?;
        let amount1_min: u64 = crate::borsh_de_or_default(&mut reader)?;
        let deadline: i64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(DecreaseLiquidityIxArgs {
                liquidity,
                amount0_min,
                amount1_min,
                deadline,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&DECREASE_LIQUIDITY_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.liquidity, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.amount0_min, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.amount1_min, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.deadline, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn decrease_liquidity_ix_with_program_id(
    program_id: Pubkey,
    keys: DecreaseLiquidityKeys,
    args: DecreaseLiquidityIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; DECREASE_LIQUIDITY_IX_ACCOUNTS_LEN] = keys.into();
    let data: DecreaseLiquidityIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn decrease_liquidity_ix(
    keys: DecreaseLiquidityKeys,
    args: DecreaseLiquidityIxArgs,
) -> std::io::Result<Instruction> {
    decrease_liquidity_ix_with_program_id(CYKURA_PROGRAM_ID, keys, args)
}
pub fn decrease_liquidity_invoke_with_program_id(
    program_id: Pubkey,
    accounts: DecreaseLiquidityAccounts<'_, '_>,
    args: DecreaseLiquidityIxArgs,
) -> ProgramResult {
    let keys: DecreaseLiquidityKeys = accounts.into();
    let ix = decrease_liquidity_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn decrease_liquidity_invoke(
    accounts: DecreaseLiquidityAccounts<'_, '_>,
    args: DecreaseLiquidityIxArgs,
) -> ProgramResult {
    decrease_liquidity_invoke_with_program_id(CYKURA_PROGRAM_ID, accounts, args)
}
pub fn decrease_liquidity_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: DecreaseLiquidityAccounts<'_, '_>,
    args: DecreaseLiquidityIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: DecreaseLiquidityKeys = accounts.into();
    let ix = decrease_liquidity_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn decrease_liquidity_invoke_signed(
    accounts: DecreaseLiquidityAccounts<'_, '_>,
    args: DecreaseLiquidityIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    decrease_liquidity_invoke_signed_with_program_id(
        CYKURA_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn decrease_liquidity_verify_account_keys(
    accounts: DecreaseLiquidityAccounts<'_, '_>,
    keys: DecreaseLiquidityKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.owner_or_delegate.key, keys.owner_or_delegate),
        (*accounts.nft_account.key, keys.nft_account),
        (*accounts.tokenized_position_state.key, keys.tokenized_position_state),
        (*accounts.factory_state.key, keys.factory_state),
        (*accounts.pool_state.key, keys.pool_state),
        (*accounts.core_position_state.key, keys.core_position_state),
        (*accounts.tick_lower_state.key, keys.tick_lower_state),
        (*accounts.tick_upper_state.key, keys.tick_upper_state),
        (*accounts.bitmap_lower_state.key, keys.bitmap_lower_state),
        (*accounts.bitmap_upper_state.key, keys.bitmap_upper_state),
        (*accounts.last_observation_state.key, keys.last_observation_state),
        (*accounts.core_program.key, keys.core_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn decrease_liquidity_verify_writable_privileges<'me, 'info>(
    accounts: DecreaseLiquidityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.tokenized_position_state,
        accounts.pool_state,
        accounts.core_position_state,
        accounts.tick_lower_state,
        accounts.tick_upper_state,
        accounts.bitmap_lower_state,
        accounts.bitmap_upper_state,
        accounts.last_observation_state,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn decrease_liquidity_verify_signer_privileges<'me, 'info>(
    accounts: DecreaseLiquidityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.owner_or_delegate] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn decrease_liquidity_verify_account_privileges<'me, 'info>(
    accounts: DecreaseLiquidityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    decrease_liquidity_verify_writable_privileges(accounts)?;
    decrease_liquidity_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const COLLECT_FROM_TOKENIZED_IX_ACCOUNTS_LEN: usize = 17;
#[derive(Copy, Clone, Debug)]
pub struct CollectFromTokenizedAccounts<'me, 'info> {
    pub owner_or_delegate: &'me AccountInfo<'info>,
    pub nft_account: &'me AccountInfo<'info>,
    pub tokenized_position_state: &'me AccountInfo<'info>,
    pub factory_state: &'me AccountInfo<'info>,
    pub pool_state: &'me AccountInfo<'info>,
    pub core_position_state: &'me AccountInfo<'info>,
    pub tick_lower_state: &'me AccountInfo<'info>,
    pub tick_upper_state: &'me AccountInfo<'info>,
    pub bitmap_lower_state: &'me AccountInfo<'info>,
    pub bitmap_upper_state: &'me AccountInfo<'info>,
    pub last_observation_state: &'me AccountInfo<'info>,
    pub vault0: &'me AccountInfo<'info>,
    pub vault1: &'me AccountInfo<'info>,
    pub recipient_wallet0: &'me AccountInfo<'info>,
    pub recipient_wallet1: &'me AccountInfo<'info>,
    pub core_program: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CollectFromTokenizedKeys {
    pub owner_or_delegate: Pubkey,
    pub nft_account: Pubkey,
    pub tokenized_position_state: Pubkey,
    pub factory_state: Pubkey,
    pub pool_state: Pubkey,
    pub core_position_state: Pubkey,
    pub tick_lower_state: Pubkey,
    pub tick_upper_state: Pubkey,
    pub bitmap_lower_state: Pubkey,
    pub bitmap_upper_state: Pubkey,
    pub last_observation_state: Pubkey,
    pub vault0: Pubkey,
    pub vault1: Pubkey,
    pub recipient_wallet0: Pubkey,
    pub recipient_wallet1: Pubkey,
    pub core_program: Pubkey,
    pub token_program: Pubkey,
}
impl From<CollectFromTokenizedAccounts<'_, '_>> for CollectFromTokenizedKeys {
    fn from(accounts: CollectFromTokenizedAccounts) -> Self {
        Self {
            owner_or_delegate: *accounts.owner_or_delegate.key,
            nft_account: *accounts.nft_account.key,
            tokenized_position_state: *accounts.tokenized_position_state.key,
            factory_state: *accounts.factory_state.key,
            pool_state: *accounts.pool_state.key,
            core_position_state: *accounts.core_position_state.key,
            tick_lower_state: *accounts.tick_lower_state.key,
            tick_upper_state: *accounts.tick_upper_state.key,
            bitmap_lower_state: *accounts.bitmap_lower_state.key,
            bitmap_upper_state: *accounts.bitmap_upper_state.key,
            last_observation_state: *accounts.last_observation_state.key,
            vault0: *accounts.vault0.key,
            vault1: *accounts.vault1.key,
            recipient_wallet0: *accounts.recipient_wallet0.key,
            recipient_wallet1: *accounts.recipient_wallet1.key,
            core_program: *accounts.core_program.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<CollectFromTokenizedKeys>
for [AccountMeta; COLLECT_FROM_TOKENIZED_IX_ACCOUNTS_LEN] {
    fn from(keys: CollectFromTokenizedKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.owner_or_delegate,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.nft_account,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.tokenized_position_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.factory_state,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.core_position_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.tick_lower_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.tick_upper_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.bitmap_lower_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.bitmap_upper_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.last_observation_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vault0,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vault1,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.recipient_wallet0,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.recipient_wallet1,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.core_program,
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
impl From<[Pubkey; COLLECT_FROM_TOKENIZED_IX_ACCOUNTS_LEN]>
for CollectFromTokenizedKeys {
    fn from(pubkeys: [Pubkey; COLLECT_FROM_TOKENIZED_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            owner_or_delegate: pubkeys[0],
            nft_account: pubkeys[1],
            tokenized_position_state: pubkeys[2],
            factory_state: pubkeys[3],
            pool_state: pubkeys[4],
            core_position_state: pubkeys[5],
            tick_lower_state: pubkeys[6],
            tick_upper_state: pubkeys[7],
            bitmap_lower_state: pubkeys[8],
            bitmap_upper_state: pubkeys[9],
            last_observation_state: pubkeys[10],
            vault0: pubkeys[11],
            vault1: pubkeys[12],
            recipient_wallet0: pubkeys[13],
            recipient_wallet1: pubkeys[14],
            core_program: pubkeys[15],
            token_program: pubkeys[16],
        }
    }
}
impl<'info> From<CollectFromTokenizedAccounts<'_, 'info>>
for [AccountInfo<'info>; COLLECT_FROM_TOKENIZED_IX_ACCOUNTS_LEN] {
    fn from(accounts: CollectFromTokenizedAccounts<'_, 'info>) -> Self {
        [
            accounts.owner_or_delegate.clone(),
            accounts.nft_account.clone(),
            accounts.tokenized_position_state.clone(),
            accounts.factory_state.clone(),
            accounts.pool_state.clone(),
            accounts.core_position_state.clone(),
            accounts.tick_lower_state.clone(),
            accounts.tick_upper_state.clone(),
            accounts.bitmap_lower_state.clone(),
            accounts.bitmap_upper_state.clone(),
            accounts.last_observation_state.clone(),
            accounts.vault0.clone(),
            accounts.vault1.clone(),
            accounts.recipient_wallet0.clone(),
            accounts.recipient_wallet1.clone(),
            accounts.core_program.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; COLLECT_FROM_TOKENIZED_IX_ACCOUNTS_LEN]>
for CollectFromTokenizedAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; COLLECT_FROM_TOKENIZED_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            owner_or_delegate: &arr[0],
            nft_account: &arr[1],
            tokenized_position_state: &arr[2],
            factory_state: &arr[3],
            pool_state: &arr[4],
            core_position_state: &arr[5],
            tick_lower_state: &arr[6],
            tick_upper_state: &arr[7],
            bitmap_lower_state: &arr[8],
            bitmap_upper_state: &arr[9],
            last_observation_state: &arr[10],
            vault0: &arr[11],
            vault1: &arr[12],
            recipient_wallet0: &arr[13],
            recipient_wallet1: &arr[14],
            core_program: &arr[15],
            token_program: &arr[16],
        }
    }
}
pub const COLLECT_FROM_TOKENIZED_IX_DISCM: [u8; 8usize] = [
    48, 252, 233, 120, 127, 103, 33, 176,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CollectFromTokenizedIxArgs {
    pub amount0_max: u64,
    pub amount1_max: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct CollectFromTokenizedIxData(pub CollectFromTokenizedIxArgs);
impl From<CollectFromTokenizedIxArgs> for CollectFromTokenizedIxData {
    fn from(args: CollectFromTokenizedIxArgs) -> Self {
        Self(args)
    }
}
impl CollectFromTokenizedIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != COLLECT_FROM_TOKENIZED_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let amount0_max: u64 = crate::borsh_de_or_default(&mut reader)?;
        let amount1_max: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(CollectFromTokenizedIxArgs {
                amount0_max,
                amount1_max,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&COLLECT_FROM_TOKENIZED_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.amount0_max, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.amount1_max, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn collect_from_tokenized_ix_with_program_id(
    program_id: Pubkey,
    keys: CollectFromTokenizedKeys,
    args: CollectFromTokenizedIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; COLLECT_FROM_TOKENIZED_IX_ACCOUNTS_LEN] = keys.into();
    let data: CollectFromTokenizedIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn collect_from_tokenized_ix(
    keys: CollectFromTokenizedKeys,
    args: CollectFromTokenizedIxArgs,
) -> std::io::Result<Instruction> {
    collect_from_tokenized_ix_with_program_id(CYKURA_PROGRAM_ID, keys, args)
}
pub fn collect_from_tokenized_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CollectFromTokenizedAccounts<'_, '_>,
    args: CollectFromTokenizedIxArgs,
) -> ProgramResult {
    let keys: CollectFromTokenizedKeys = accounts.into();
    let ix = collect_from_tokenized_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn collect_from_tokenized_invoke(
    accounts: CollectFromTokenizedAccounts<'_, '_>,
    args: CollectFromTokenizedIxArgs,
) -> ProgramResult {
    collect_from_tokenized_invoke_with_program_id(CYKURA_PROGRAM_ID, accounts, args)
}
pub fn collect_from_tokenized_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CollectFromTokenizedAccounts<'_, '_>,
    args: CollectFromTokenizedIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CollectFromTokenizedKeys = accounts.into();
    let ix = collect_from_tokenized_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn collect_from_tokenized_invoke_signed(
    accounts: CollectFromTokenizedAccounts<'_, '_>,
    args: CollectFromTokenizedIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    collect_from_tokenized_invoke_signed_with_program_id(
        CYKURA_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn collect_from_tokenized_verify_account_keys(
    accounts: CollectFromTokenizedAccounts<'_, '_>,
    keys: CollectFromTokenizedKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.owner_or_delegate.key, keys.owner_or_delegate),
        (*accounts.nft_account.key, keys.nft_account),
        (*accounts.tokenized_position_state.key, keys.tokenized_position_state),
        (*accounts.factory_state.key, keys.factory_state),
        (*accounts.pool_state.key, keys.pool_state),
        (*accounts.core_position_state.key, keys.core_position_state),
        (*accounts.tick_lower_state.key, keys.tick_lower_state),
        (*accounts.tick_upper_state.key, keys.tick_upper_state),
        (*accounts.bitmap_lower_state.key, keys.bitmap_lower_state),
        (*accounts.bitmap_upper_state.key, keys.bitmap_upper_state),
        (*accounts.last_observation_state.key, keys.last_observation_state),
        (*accounts.vault0.key, keys.vault0),
        (*accounts.vault1.key, keys.vault1),
        (*accounts.recipient_wallet0.key, keys.recipient_wallet0),
        (*accounts.recipient_wallet1.key, keys.recipient_wallet1),
        (*accounts.core_program.key, keys.core_program),
        (*accounts.token_program.key, keys.token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn collect_from_tokenized_verify_writable_privileges<'me, 'info>(
    accounts: CollectFromTokenizedAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.tokenized_position_state,
        accounts.pool_state,
        accounts.core_position_state,
        accounts.tick_lower_state,
        accounts.tick_upper_state,
        accounts.bitmap_lower_state,
        accounts.bitmap_upper_state,
        accounts.last_observation_state,
        accounts.vault0,
        accounts.vault1,
        accounts.recipient_wallet0,
        accounts.recipient_wallet1,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn collect_from_tokenized_verify_signer_privileges<'me, 'info>(
    accounts: CollectFromTokenizedAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.owner_or_delegate] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn collect_from_tokenized_verify_account_privileges<'me, 'info>(
    accounts: CollectFromTokenizedAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    collect_from_tokenized_verify_writable_privileges(accounts)?;
    collect_from_tokenized_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const EXACT_INPUT_SINGLE_IX_ACCOUNTS_LEN: usize = 10;
#[derive(Copy, Clone, Debug)]
pub struct ExactInputSingleAccounts<'me, 'info> {
    pub signer: &'me AccountInfo<'info>,
    pub factory_state: &'me AccountInfo<'info>,
    pub pool_state: &'me AccountInfo<'info>,
    pub input_token_account: &'me AccountInfo<'info>,
    pub output_token_account: &'me AccountInfo<'info>,
    pub input_vault: &'me AccountInfo<'info>,
    pub output_vault: &'me AccountInfo<'info>,
    pub last_observation_state: &'me AccountInfo<'info>,
    pub core_program: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ExactInputSingleKeys {
    pub signer: Pubkey,
    pub factory_state: Pubkey,
    pub pool_state: Pubkey,
    pub input_token_account: Pubkey,
    pub output_token_account: Pubkey,
    pub input_vault: Pubkey,
    pub output_vault: Pubkey,
    pub last_observation_state: Pubkey,
    pub core_program: Pubkey,
    pub token_program: Pubkey,
}
impl From<ExactInputSingleAccounts<'_, '_>> for ExactInputSingleKeys {
    fn from(accounts: ExactInputSingleAccounts) -> Self {
        Self {
            signer: *accounts.signer.key,
            factory_state: *accounts.factory_state.key,
            pool_state: *accounts.pool_state.key,
            input_token_account: *accounts.input_token_account.key,
            output_token_account: *accounts.output_token_account.key,
            input_vault: *accounts.input_vault.key,
            output_vault: *accounts.output_vault.key,
            last_observation_state: *accounts.last_observation_state.key,
            core_program: *accounts.core_program.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<ExactInputSingleKeys> for [AccountMeta; EXACT_INPUT_SINGLE_IX_ACCOUNTS_LEN] {
    fn from(keys: ExactInputSingleKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.signer,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.factory_state,
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
                pubkey: keys.last_observation_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.core_program,
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
impl From<[Pubkey; EXACT_INPUT_SINGLE_IX_ACCOUNTS_LEN]> for ExactInputSingleKeys {
    fn from(pubkeys: [Pubkey; EXACT_INPUT_SINGLE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            signer: pubkeys[0],
            factory_state: pubkeys[1],
            pool_state: pubkeys[2],
            input_token_account: pubkeys[3],
            output_token_account: pubkeys[4],
            input_vault: pubkeys[5],
            output_vault: pubkeys[6],
            last_observation_state: pubkeys[7],
            core_program: pubkeys[8],
            token_program: pubkeys[9],
        }
    }
}
impl<'info> From<ExactInputSingleAccounts<'_, 'info>>
for [AccountInfo<'info>; EXACT_INPUT_SINGLE_IX_ACCOUNTS_LEN] {
    fn from(accounts: ExactInputSingleAccounts<'_, 'info>) -> Self {
        [
            accounts.signer.clone(),
            accounts.factory_state.clone(),
            accounts.pool_state.clone(),
            accounts.input_token_account.clone(),
            accounts.output_token_account.clone(),
            accounts.input_vault.clone(),
            accounts.output_vault.clone(),
            accounts.last_observation_state.clone(),
            accounts.core_program.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; EXACT_INPUT_SINGLE_IX_ACCOUNTS_LEN]>
for ExactInputSingleAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; EXACT_INPUT_SINGLE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            signer: &arr[0],
            factory_state: &arr[1],
            pool_state: &arr[2],
            input_token_account: &arr[3],
            output_token_account: &arr[4],
            input_vault: &arr[5],
            output_vault: &arr[6],
            last_observation_state: &arr[7],
            core_program: &arr[8],
            token_program: &arr[9],
        }
    }
}
pub const EXACT_INPUT_SINGLE_IX_DISCM: [u8; 8usize] = [
    23, 113, 90, 161, 237, 143, 153, 13,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ExactInputSingleIxArgs {
    pub deadline: i64,
    pub amount_in: u64,
    pub amount_out_minimum: u64,
    pub sqrt_price_limit_x32: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct ExactInputSingleIxData(pub ExactInputSingleIxArgs);
impl From<ExactInputSingleIxArgs> for ExactInputSingleIxData {
    fn from(args: ExactInputSingleIxArgs) -> Self {
        Self(args)
    }
}
impl ExactInputSingleIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != EXACT_INPUT_SINGLE_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let deadline: i64 = crate::borsh_de_or_default(&mut reader)?;
        let amount_in: u64 = crate::borsh_de_or_default(&mut reader)?;
        let amount_out_minimum: u64 = crate::borsh_de_or_default(&mut reader)?;
        let sqrt_price_limit_x32: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(ExactInputSingleIxArgs {
                deadline,
                amount_in,
                amount_out_minimum,
                sqrt_price_limit_x32,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&EXACT_INPUT_SINGLE_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.deadline, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.amount_in, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.amount_out_minimum, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.sqrt_price_limit_x32, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn exact_input_single_ix_with_program_id(
    program_id: Pubkey,
    keys: ExactInputSingleKeys,
    args: ExactInputSingleIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; EXACT_INPUT_SINGLE_IX_ACCOUNTS_LEN] = keys.into();
    let data: ExactInputSingleIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn exact_input_single_ix(
    keys: ExactInputSingleKeys,
    args: ExactInputSingleIxArgs,
) -> std::io::Result<Instruction> {
    exact_input_single_ix_with_program_id(CYKURA_PROGRAM_ID, keys, args)
}
pub fn exact_input_single_invoke_with_program_id(
    program_id: Pubkey,
    accounts: ExactInputSingleAccounts<'_, '_>,
    args: ExactInputSingleIxArgs,
) -> ProgramResult {
    let keys: ExactInputSingleKeys = accounts.into();
    let ix = exact_input_single_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn exact_input_single_invoke(
    accounts: ExactInputSingleAccounts<'_, '_>,
    args: ExactInputSingleIxArgs,
) -> ProgramResult {
    exact_input_single_invoke_with_program_id(CYKURA_PROGRAM_ID, accounts, args)
}
pub fn exact_input_single_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: ExactInputSingleAccounts<'_, '_>,
    args: ExactInputSingleIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: ExactInputSingleKeys = accounts.into();
    let ix = exact_input_single_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn exact_input_single_invoke_signed(
    accounts: ExactInputSingleAccounts<'_, '_>,
    args: ExactInputSingleIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    exact_input_single_invoke_signed_with_program_id(
        CYKURA_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn exact_input_single_verify_account_keys(
    accounts: ExactInputSingleAccounts<'_, '_>,
    keys: ExactInputSingleKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.signer.key, keys.signer),
        (*accounts.factory_state.key, keys.factory_state),
        (*accounts.pool_state.key, keys.pool_state),
        (*accounts.input_token_account.key, keys.input_token_account),
        (*accounts.output_token_account.key, keys.output_token_account),
        (*accounts.input_vault.key, keys.input_vault),
        (*accounts.output_vault.key, keys.output_vault),
        (*accounts.last_observation_state.key, keys.last_observation_state),
        (*accounts.core_program.key, keys.core_program),
        (*accounts.token_program.key, keys.token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn exact_input_single_verify_writable_privileges<'me, 'info>(
    accounts: ExactInputSingleAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.pool_state,
        accounts.input_token_account,
        accounts.output_token_account,
        accounts.input_vault,
        accounts.output_vault,
        accounts.last_observation_state,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn exact_input_single_verify_signer_privileges<'me, 'info>(
    accounts: ExactInputSingleAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.signer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn exact_input_single_verify_account_privileges<'me, 'info>(
    accounts: ExactInputSingleAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    exact_input_single_verify_writable_privileges(accounts)?;
    exact_input_single_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const EXACT_INPUT_IX_ACCOUNTS_LEN: usize = 5;
#[derive(Copy, Clone, Debug)]
pub struct ExactInputAccounts<'me, 'info> {
    pub signer: &'me AccountInfo<'info>,
    pub factory_state: &'me AccountInfo<'info>,
    pub input_token_account: &'me AccountInfo<'info>,
    pub core_program: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ExactInputKeys {
    pub signer: Pubkey,
    pub factory_state: Pubkey,
    pub input_token_account: Pubkey,
    pub core_program: Pubkey,
    pub token_program: Pubkey,
}
impl From<ExactInputAccounts<'_, '_>> for ExactInputKeys {
    fn from(accounts: ExactInputAccounts) -> Self {
        Self {
            signer: *accounts.signer.key,
            factory_state: *accounts.factory_state.key,
            input_token_account: *accounts.input_token_account.key,
            core_program: *accounts.core_program.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<ExactInputKeys> for [AccountMeta; EXACT_INPUT_IX_ACCOUNTS_LEN] {
    fn from(keys: ExactInputKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.signer,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.factory_state,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.input_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.core_program,
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
impl From<[Pubkey; EXACT_INPUT_IX_ACCOUNTS_LEN]> for ExactInputKeys {
    fn from(pubkeys: [Pubkey; EXACT_INPUT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            signer: pubkeys[0],
            factory_state: pubkeys[1],
            input_token_account: pubkeys[2],
            core_program: pubkeys[3],
            token_program: pubkeys[4],
        }
    }
}
impl<'info> From<ExactInputAccounts<'_, 'info>>
for [AccountInfo<'info>; EXACT_INPUT_IX_ACCOUNTS_LEN] {
    fn from(accounts: ExactInputAccounts<'_, 'info>) -> Self {
        [
            accounts.signer.clone(),
            accounts.factory_state.clone(),
            accounts.input_token_account.clone(),
            accounts.core_program.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; EXACT_INPUT_IX_ACCOUNTS_LEN]>
for ExactInputAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; EXACT_INPUT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            signer: &arr[0],
            factory_state: &arr[1],
            input_token_account: &arr[2],
            core_program: &arr[3],
            token_program: &arr[4],
        }
    }
}
pub const EXACT_INPUT_IX_DISCM: [u8; 8usize] = [239, 115, 34, 128, 115, 191, 197, 185];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ExactInputIxArgs {
    pub deadline: i64,
    pub amount_in: u64,
    pub amount_out_minimum: u64,
    pub additional_accounts_per_pool: Vec<u8>,
}
#[derive(Clone, Debug, PartialEq)]
pub struct ExactInputIxData(pub ExactInputIxArgs);
impl From<ExactInputIxArgs> for ExactInputIxData {
    fn from(args: ExactInputIxArgs) -> Self {
        Self(args)
    }
}
impl ExactInputIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != EXACT_INPUT_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let deadline: i64 = crate::borsh_de_or_default(&mut reader)?;
        let amount_in: u64 = crate::borsh_de_or_default(&mut reader)?;
        let amount_out_minimum: u64 = crate::borsh_de_or_default(&mut reader)?;
        let additional_accounts_per_pool: Vec<u8> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        Ok(
            Self(ExactInputIxArgs {
                deadline,
                amount_in,
                amount_out_minimum,
                additional_accounts_per_pool,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&EXACT_INPUT_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.deadline, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.amount_in, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.amount_out_minimum, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.0.additional_accounts_per_pool,
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
pub fn exact_input_ix_with_program_id(
    program_id: Pubkey,
    keys: ExactInputKeys,
    args: ExactInputIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; EXACT_INPUT_IX_ACCOUNTS_LEN] = keys.into();
    let data: ExactInputIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn exact_input_ix(
    keys: ExactInputKeys,
    args: ExactInputIxArgs,
) -> std::io::Result<Instruction> {
    exact_input_ix_with_program_id(CYKURA_PROGRAM_ID, keys, args)
}
pub fn exact_input_invoke_with_program_id(
    program_id: Pubkey,
    accounts: ExactInputAccounts<'_, '_>,
    args: ExactInputIxArgs,
) -> ProgramResult {
    let keys: ExactInputKeys = accounts.into();
    let ix = exact_input_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn exact_input_invoke(
    accounts: ExactInputAccounts<'_, '_>,
    args: ExactInputIxArgs,
) -> ProgramResult {
    exact_input_invoke_with_program_id(CYKURA_PROGRAM_ID, accounts, args)
}
pub fn exact_input_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: ExactInputAccounts<'_, '_>,
    args: ExactInputIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: ExactInputKeys = accounts.into();
    let ix = exact_input_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn exact_input_invoke_signed(
    accounts: ExactInputAccounts<'_, '_>,
    args: ExactInputIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    exact_input_invoke_signed_with_program_id(CYKURA_PROGRAM_ID, accounts, args, seeds)
}
pub fn exact_input_verify_account_keys(
    accounts: ExactInputAccounts<'_, '_>,
    keys: ExactInputKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.signer.key, keys.signer),
        (*accounts.factory_state.key, keys.factory_state),
        (*accounts.input_token_account.key, keys.input_token_account),
        (*accounts.core_program.key, keys.core_program),
        (*accounts.token_program.key, keys.token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn exact_input_verify_writable_privileges<'me, 'info>(
    accounts: ExactInputAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.input_token_account] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn exact_input_verify_signer_privileges<'me, 'info>(
    accounts: ExactInputAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.signer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn exact_input_verify_account_privileges<'me, 'info>(
    accounts: ExactInputAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    exact_input_verify_writable_privileges(accounts)?;
    exact_input_verify_signer_privileges(accounts)?;
    Ok(())
}
