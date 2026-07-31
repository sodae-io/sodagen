use solana_pubkey::Pubkey;
use solana_cpi::{invoke, invoke_signed};
use solana_instruction::{AccountMeta, Instruction};
use solana_account_info::AccountInfo;
use solana_program_error::ProgramError;
use std::io::Read;
#[allow(unused_imports)]
use crate::*;
#[derive(Clone, Debug, PartialEq)]
pub enum StabbleClmmProgramIx {
    ClosePosition,
    CloseProtocolPosition,
    CollectFundFee(CollectFundFeeIxArgs),
    CollectProtocolFee(CollectProtocolFeeIxArgs),
    CollectRemainingRewards(CollectRemainingRewardsIxArgs),
    CreateAmmConfig(CreateAmmConfigIxArgs),
    CreateOperationAccount,
    CreatePool(CreatePoolIxArgs),
    CreateSupportMintAssociated,
    DecreaseLiquidityV2(DecreaseLiquidityV2IxArgs),
    IncreaseLiquidityV2(IncreaseLiquidityV2IxArgs),
    InitializeReward(InitializeRewardIxArgs),
    OpenPositionWithToken22Nft(OpenPositionWithToken22NftIxArgs),
    SetRewardParams(SetRewardParamsIxArgs),
    SwapRouterBaseIn(SwapRouterBaseInIxArgs),
    SwapV2(SwapV2IxArgs),
    TransferRewardOwner(TransferRewardOwnerIxArgs),
    UpdateAmmConfig(UpdateAmmConfigIxArgs),
    UpdateOperationAccount(UpdateOperationAccountIxArgs),
    UpdatePoolStatus(UpdatePoolStatusIxArgs),
    UpdateRewardInfos,
}
impl StabbleClmmProgramIx {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        if buf.starts_with(&CLOSE_POSITION_IX_DISCM) {
            return Ok(Self::ClosePosition);
        }
        if buf.starts_with(&CLOSE_PROTOCOL_POSITION_IX_DISCM) {
            return Ok(Self::CloseProtocolPosition);
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
        if buf.starts_with(&COLLECT_REMAINING_REWARDS_IX_DISCM) {
            let mut reader = &buf[COLLECT_REMAINING_REWARDS_IX_DISCM.len()..];
            let reward_index: u8 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::CollectRemainingRewards(CollectRemainingRewardsIxArgs {
                    reward_index,
                }),
            );
        }
        if buf.starts_with(&CREATE_AMM_CONFIG_IX_DISCM) {
            let mut reader = &buf[CREATE_AMM_CONFIG_IX_DISCM.len()..];
            let index: u16 = crate::borsh_de_or_default(&mut reader)?;
            let tick_spacing: u16 = crate::borsh_de_or_default(&mut reader)?;
            let trade_fee_rate: u32 = crate::borsh_de_or_default(&mut reader)?;
            let protocol_fee_rate: u32 = crate::borsh_de_or_default(&mut reader)?;
            let fund_fee_rate: u32 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::CreateAmmConfig(CreateAmmConfigIxArgs {
                    index,
                    tick_spacing,
                    trade_fee_rate,
                    protocol_fee_rate,
                    fund_fee_rate,
                }),
            );
        }
        if buf.starts_with(&CREATE_OPERATION_ACCOUNT_IX_DISCM) {
            return Ok(Self::CreateOperationAccount);
        }
        if buf.starts_with(&CREATE_POOL_IX_DISCM) {
            let mut reader = &buf[CREATE_POOL_IX_DISCM.len()..];
            let sqrt_price_x64: u128 = crate::borsh_de_or_default(&mut reader)?;
            let open_time: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::CreatePool(CreatePoolIxArgs {
                    sqrt_price_x64,
                    open_time,
                }),
            );
        }
        if buf.starts_with(&CREATE_SUPPORT_MINT_ASSOCIATED_IX_DISCM) {
            return Ok(Self::CreateSupportMintAssociated);
        }
        if buf.starts_with(&DECREASE_LIQUIDITY_V2_IX_DISCM) {
            let mut reader = &buf[DECREASE_LIQUIDITY_V2_IX_DISCM.len()..];
            let liquidity: u128 = crate::borsh_de_or_default(&mut reader)?;
            let amount_0_min: u64 = crate::borsh_de_or_default(&mut reader)?;
            let amount_1_min: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::DecreaseLiquidityV2(DecreaseLiquidityV2IxArgs {
                    liquidity,
                    amount_0_min,
                    amount_1_min,
                }),
            );
        }
        if buf.starts_with(&INCREASE_LIQUIDITY_V2_IX_DISCM) {
            let mut reader = &buf[INCREASE_LIQUIDITY_V2_IX_DISCM.len()..];
            let liquidity: u128 = crate::borsh_de_or_default(&mut reader)?;
            let amount_0_max: u64 = crate::borsh_de_or_default(&mut reader)?;
            let amount_1_max: u64 = crate::borsh_de_or_default(&mut reader)?;
            let base_flag: Option<bool> = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::IncreaseLiquidityV2(IncreaseLiquidityV2IxArgs {
                    liquidity,
                    amount_0_max,
                    amount_1_max,
                    base_flag,
                }),
            );
        }
        if buf.starts_with(&INITIALIZE_REWARD_IX_DISCM) {
            let mut reader = &buf[INITIALIZE_REWARD_IX_DISCM.len()..];
            let param = if reader.is_empty() {
                Default::default()
            } else {
                <InitializeRewardParam>::deserialize(&mut reader)?
            };
            return Ok(Self::InitializeReward(InitializeRewardIxArgs { param }));
        }
        if buf.starts_with(&OPEN_POSITION_WITH_TOKEN22_NFT_IX_DISCM) {
            let mut reader = &buf[OPEN_POSITION_WITH_TOKEN22_NFT_IX_DISCM.len()..];
            let tick_lower_index: i32 = crate::borsh_de_or_default(&mut reader)?;
            let tick_upper_index: i32 = crate::borsh_de_or_default(&mut reader)?;
            let tick_array_lower_start_index: i32 = crate::borsh_de_or_default(
                &mut reader,
            )?;
            let tick_array_upper_start_index: i32 = crate::borsh_de_or_default(
                &mut reader,
            )?;
            let liquidity: u128 = crate::borsh_de_or_default(&mut reader)?;
            let amount_0_max: u64 = crate::borsh_de_or_default(&mut reader)?;
            let amount_1_max: u64 = crate::borsh_de_or_default(&mut reader)?;
            let with_metadata: bool = crate::borsh_de_or_default(&mut reader)?;
            let base_flag: Option<bool> = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::OpenPositionWithToken22Nft(OpenPositionWithToken22NftIxArgs {
                    tick_lower_index,
                    tick_upper_index,
                    tick_array_lower_start_index,
                    tick_array_upper_start_index,
                    liquidity,
                    amount_0_max,
                    amount_1_max,
                    with_metadata,
                    base_flag,
                }),
            );
        }
        if buf.starts_with(&SET_REWARD_PARAMS_IX_DISCM) {
            let mut reader = &buf[SET_REWARD_PARAMS_IX_DISCM.len()..];
            let reward_index: u8 = crate::borsh_de_or_default(&mut reader)?;
            let emissions_per_second_x64: u128 = crate::borsh_de_or_default(
                &mut reader,
            )?;
            let open_time: u64 = crate::borsh_de_or_default(&mut reader)?;
            let end_time: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::SetRewardParams(SetRewardParamsIxArgs {
                    reward_index,
                    emissions_per_second_x64,
                    open_time,
                    end_time,
                }),
            );
        }
        if buf.starts_with(&SWAP_ROUTER_BASE_IN_IX_DISCM) {
            let mut reader = &buf[SWAP_ROUTER_BASE_IN_IX_DISCM.len()..];
            let amount_in: u64 = crate::borsh_de_or_default(&mut reader)?;
            let amount_out_minimum: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::SwapRouterBaseIn(SwapRouterBaseInIxArgs {
                    amount_in,
                    amount_out_minimum,
                }),
            );
        }
        if buf.starts_with(&SWAP_V2_IX_DISCM) {
            let mut reader = &buf[SWAP_V2_IX_DISCM.len()..];
            let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            let other_amount_threshold: u64 = crate::borsh_de_or_default(&mut reader)?;
            let sqrt_price_limit_x64: u128 = crate::borsh_de_or_default(&mut reader)?;
            let is_base_input: bool = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::SwapV2(SwapV2IxArgs {
                    amount,
                    other_amount_threshold,
                    sqrt_price_limit_x64,
                    is_base_input,
                }),
            );
        }
        if buf.starts_with(&TRANSFER_REWARD_OWNER_IX_DISCM) {
            let mut reader = &buf[TRANSFER_REWARD_OWNER_IX_DISCM.len()..];
            let new_owner: Pubkey = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::TransferRewardOwner(TransferRewardOwnerIxArgs {
                    new_owner,
                }),
            );
        }
        if buf.starts_with(&UPDATE_AMM_CONFIG_IX_DISCM) {
            let mut reader = &buf[UPDATE_AMM_CONFIG_IX_DISCM.len()..];
            let param: u8 = crate::borsh_de_or_default(&mut reader)?;
            let value: u32 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::UpdateAmmConfig(UpdateAmmConfigIxArgs {
                    param,
                    value,
                }),
            );
        }
        if buf.starts_with(&UPDATE_OPERATION_ACCOUNT_IX_DISCM) {
            let mut reader = &buf[UPDATE_OPERATION_ACCOUNT_IX_DISCM.len()..];
            let param: u8 = crate::borsh_de_or_default(&mut reader)?;
            let keys: Vec<Pubkey> = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::UpdateOperationAccount(UpdateOperationAccountIxArgs {
                    param,
                    keys,
                }),
            );
        }
        if buf.starts_with(&UPDATE_POOL_STATUS_IX_DISCM) {
            let mut reader = &buf[UPDATE_POOL_STATUS_IX_DISCM.len()..];
            let status: u8 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(Self::UpdatePoolStatus(UpdatePoolStatusIxArgs { status }));
        }
        if buf.starts_with(&UPDATE_REWARD_INFOS_IX_DISCM) {
            return Ok(Self::UpdateRewardInfos);
        }
        Err(std::io::Error::from(std::io::ErrorKind::InvalidData))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        match self {
            Self::ClosePosition => writer.write_all(&CLOSE_POSITION_IX_DISCM),
            Self::CloseProtocolPosition => {
                writer.write_all(&CLOSE_PROTOCOL_POSITION_IX_DISCM)
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
            Self::CollectRemainingRewards(args) => {
                writer.write_all(&COLLECT_REMAINING_REWARDS_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.reward_index, &mut writer)?;
                Ok(())
            }
            Self::CreateAmmConfig(args) => {
                writer.write_all(&CREATE_AMM_CONFIG_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.index, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.tick_spacing, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.trade_fee_rate, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.protocol_fee_rate, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.fund_fee_rate, &mut writer)?;
                Ok(())
            }
            Self::CreateOperationAccount => {
                writer.write_all(&CREATE_OPERATION_ACCOUNT_IX_DISCM)
            }
            Self::CreatePool(args) => {
                writer.write_all(&CREATE_POOL_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.sqrt_price_x64, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.open_time, &mut writer)?;
                Ok(())
            }
            Self::CreateSupportMintAssociated => {
                writer.write_all(&CREATE_SUPPORT_MINT_ASSOCIATED_IX_DISCM)
            }
            Self::DecreaseLiquidityV2(args) => {
                writer.write_all(&DECREASE_LIQUIDITY_V2_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.liquidity, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.amount_0_min, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.amount_1_min, &mut writer)?;
                Ok(())
            }
            Self::IncreaseLiquidityV2(args) => {
                writer.write_all(&INCREASE_LIQUIDITY_V2_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.liquidity, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.amount_0_max, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.amount_1_max, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.base_flag, &mut writer)?;
                Ok(())
            }
            Self::InitializeReward(args) => {
                writer.write_all(&INITIALIZE_REWARD_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.param, &mut writer)?;
                Ok(())
            }
            Self::OpenPositionWithToken22Nft(args) => {
                writer.write_all(&OPEN_POSITION_WITH_TOKEN22_NFT_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.tick_lower_index, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.tick_upper_index, &mut writer)?;
                borsh::BorshSerialize::serialize(
                    &args.tick_array_lower_start_index,
                    &mut writer,
                )?;
                borsh::BorshSerialize::serialize(
                    &args.tick_array_upper_start_index,
                    &mut writer,
                )?;
                borsh::BorshSerialize::serialize(&args.liquidity, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.amount_0_max, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.amount_1_max, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.with_metadata, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.base_flag, &mut writer)?;
                Ok(())
            }
            Self::SetRewardParams(args) => {
                writer.write_all(&SET_REWARD_PARAMS_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.reward_index, &mut writer)?;
                borsh::BorshSerialize::serialize(
                    &args.emissions_per_second_x64,
                    &mut writer,
                )?;
                borsh::BorshSerialize::serialize(&args.open_time, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.end_time, &mut writer)?;
                Ok(())
            }
            Self::SwapRouterBaseIn(args) => {
                writer.write_all(&SWAP_ROUTER_BASE_IN_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.amount_in, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.amount_out_minimum, &mut writer)?;
                Ok(())
            }
            Self::SwapV2(args) => {
                writer.write_all(&SWAP_V2_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.amount, &mut writer)?;
                borsh::BorshSerialize::serialize(
                    &args.other_amount_threshold,
                    &mut writer,
                )?;
                borsh::BorshSerialize::serialize(
                    &args.sqrt_price_limit_x64,
                    &mut writer,
                )?;
                borsh::BorshSerialize::serialize(&args.is_base_input, &mut writer)?;
                Ok(())
            }
            Self::TransferRewardOwner(args) => {
                writer.write_all(&TRANSFER_REWARD_OWNER_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.new_owner, &mut writer)?;
                Ok(())
            }
            Self::UpdateAmmConfig(args) => {
                writer.write_all(&UPDATE_AMM_CONFIG_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.param, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.value, &mut writer)?;
                Ok(())
            }
            Self::UpdateOperationAccount(args) => {
                writer.write_all(&UPDATE_OPERATION_ACCOUNT_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.param, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.keys, &mut writer)?;
                Ok(())
            }
            Self::UpdatePoolStatus(args) => {
                writer.write_all(&UPDATE_POOL_STATUS_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.status, &mut writer)?;
                Ok(())
            }
            Self::UpdateRewardInfos => writer.write_all(&UPDATE_REWARD_INFOS_IX_DISCM),
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
pub const CLOSE_POSITION_IX_ACCOUNTS_LEN: usize = 6;
#[derive(Copy, Clone, Debug)]
pub struct ClosePositionAccounts<'me, 'info> {
    pub nft_owner: &'me AccountInfo<'info>,
    pub position_nft_mint: &'me AccountInfo<'info>,
    pub position_nft_account: &'me AccountInfo<'info>,
    pub personal_position: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ClosePositionKeys {
    pub nft_owner: Pubkey,
    pub position_nft_mint: Pubkey,
    pub position_nft_account: Pubkey,
    pub personal_position: Pubkey,
    pub system_program: Pubkey,
    pub token_program: Pubkey,
}
impl From<ClosePositionAccounts<'_, '_>> for ClosePositionKeys {
    fn from(accounts: ClosePositionAccounts) -> Self {
        Self {
            nft_owner: *accounts.nft_owner.key,
            position_nft_mint: *accounts.position_nft_mint.key,
            position_nft_account: *accounts.position_nft_account.key,
            personal_position: *accounts.personal_position.key,
            system_program: *accounts.system_program.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<ClosePositionKeys> for [AccountMeta; CLOSE_POSITION_IX_ACCOUNTS_LEN] {
    fn from(keys: ClosePositionKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.nft_owner,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.position_nft_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.position_nft_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.personal_position,
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
        ]
    }
}
impl From<[Pubkey; CLOSE_POSITION_IX_ACCOUNTS_LEN]> for ClosePositionKeys {
    fn from(pubkeys: [Pubkey; CLOSE_POSITION_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            nft_owner: pubkeys[0],
            position_nft_mint: pubkeys[1],
            position_nft_account: pubkeys[2],
            personal_position: pubkeys[3],
            system_program: pubkeys[4],
            token_program: pubkeys[5],
        }
    }
}
impl<'info> From<ClosePositionAccounts<'_, 'info>>
for [AccountInfo<'info>; CLOSE_POSITION_IX_ACCOUNTS_LEN] {
    fn from(accounts: ClosePositionAccounts<'_, 'info>) -> Self {
        [
            accounts.nft_owner.clone(),
            accounts.position_nft_mint.clone(),
            accounts.position_nft_account.clone(),
            accounts.personal_position.clone(),
            accounts.system_program.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CLOSE_POSITION_IX_ACCOUNTS_LEN]>
for ClosePositionAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; CLOSE_POSITION_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            nft_owner: &arr[0],
            position_nft_mint: &arr[1],
            position_nft_account: &arr[2],
            personal_position: &arr[3],
            system_program: &arr[4],
            token_program: &arr[5],
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
    close_position_ix_with_program_id(STABBLE_CLMM_PROGRAM_ID, keys)
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
    close_position_invoke_with_program_id(STABBLE_CLMM_PROGRAM_ID, accounts)
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
        STABBLE_CLMM_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn close_position_verify_account_keys(
    accounts: ClosePositionAccounts<'_, '_>,
    keys: ClosePositionKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.nft_owner.key, keys.nft_owner),
        (*accounts.position_nft_mint.key, keys.position_nft_mint),
        (*accounts.position_nft_account.key, keys.position_nft_account),
        (*accounts.personal_position.key, keys.personal_position),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.token_program.key, keys.token_program),
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
        accounts.nft_owner,
        accounts.position_nft_mint,
        accounts.position_nft_account,
        accounts.personal_position,
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
    for should_be_signer in [accounts.nft_owner] {
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
pub const CLOSE_PROTOCOL_POSITION_IX_ACCOUNTS_LEN: usize = 2;
#[derive(Copy, Clone, Debug)]
pub struct CloseProtocolPositionAccounts<'me, 'info> {
    pub admin: &'me AccountInfo<'info>,
    pub protocol_position: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CloseProtocolPositionKeys {
    pub admin: Pubkey,
    pub protocol_position: Pubkey,
}
impl From<CloseProtocolPositionAccounts<'_, '_>> for CloseProtocolPositionKeys {
    fn from(accounts: CloseProtocolPositionAccounts) -> Self {
        Self {
            admin: *accounts.admin.key,
            protocol_position: *accounts.protocol_position.key,
        }
    }
}
impl From<CloseProtocolPositionKeys>
for [AccountMeta; CLOSE_PROTOCOL_POSITION_IX_ACCOUNTS_LEN] {
    fn from(keys: CloseProtocolPositionKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.admin,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.protocol_position,
                is_signer: false,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; CLOSE_PROTOCOL_POSITION_IX_ACCOUNTS_LEN]>
for CloseProtocolPositionKeys {
    fn from(pubkeys: [Pubkey; CLOSE_PROTOCOL_POSITION_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin: pubkeys[0],
            protocol_position: pubkeys[1],
        }
    }
}
impl<'info> From<CloseProtocolPositionAccounts<'_, 'info>>
for [AccountInfo<'info>; CLOSE_PROTOCOL_POSITION_IX_ACCOUNTS_LEN] {
    fn from(accounts: CloseProtocolPositionAccounts<'_, 'info>) -> Self {
        [accounts.admin.clone(), accounts.protocol_position.clone()]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CLOSE_PROTOCOL_POSITION_IX_ACCOUNTS_LEN]>
for CloseProtocolPositionAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; CLOSE_PROTOCOL_POSITION_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            admin: &arr[0],
            protocol_position: &arr[1],
        }
    }
}
pub const CLOSE_PROTOCOL_POSITION_IX_DISCM: [u8; 8usize] = [
    201, 117, 152, 144, 85, 85, 108, 178,
];
#[derive(Clone, Debug, PartialEq)]
pub struct CloseProtocolPositionIxData;
impl CloseProtocolPositionIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CLOSE_PROTOCOL_POSITION_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CLOSE_PROTOCOL_POSITION_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn close_protocol_position_ix_with_program_id(
    program_id: Pubkey,
    keys: CloseProtocolPositionKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CLOSE_PROTOCOL_POSITION_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: CloseProtocolPositionIxData.try_to_vec()?,
    })
}
pub fn close_protocol_position_ix(
    keys: CloseProtocolPositionKeys,
) -> std::io::Result<Instruction> {
    close_protocol_position_ix_with_program_id(STABBLE_CLMM_PROGRAM_ID, keys)
}
pub fn close_protocol_position_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CloseProtocolPositionAccounts<'_, '_>,
) -> ProgramResult {
    let keys: CloseProtocolPositionKeys = accounts.into();
    let ix = close_protocol_position_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn close_protocol_position_invoke(
    accounts: CloseProtocolPositionAccounts<'_, '_>,
) -> ProgramResult {
    close_protocol_position_invoke_with_program_id(STABBLE_CLMM_PROGRAM_ID, accounts)
}
pub fn close_protocol_position_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CloseProtocolPositionAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CloseProtocolPositionKeys = accounts.into();
    let ix = close_protocol_position_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn close_protocol_position_invoke_signed(
    accounts: CloseProtocolPositionAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    close_protocol_position_invoke_signed_with_program_id(
        STABBLE_CLMM_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn close_protocol_position_verify_account_keys(
    accounts: CloseProtocolPositionAccounts<'_, '_>,
    keys: CloseProtocolPositionKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.admin.key, keys.admin),
        (*accounts.protocol_position.key, keys.protocol_position),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn close_protocol_position_verify_writable_privileges<'me, 'info>(
    accounts: CloseProtocolPositionAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.admin, accounts.protocol_position] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn close_protocol_position_verify_signer_privileges<'me, 'info>(
    accounts: CloseProtocolPositionAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn close_protocol_position_verify_account_privileges<'me, 'info>(
    accounts: CloseProtocolPositionAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    close_protocol_position_verify_writable_privileges(accounts)?;
    close_protocol_position_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const COLLECT_FUND_FEE_IX_ACCOUNTS_LEN: usize = 11;
#[derive(Copy, Clone, Debug)]
pub struct CollectFundFeeAccounts<'me, 'info> {
    pub owner: &'me AccountInfo<'info>,
    pub pool_state: &'me AccountInfo<'info>,
    pub amm_config: &'me AccountInfo<'info>,
    pub token_vault_0: &'me AccountInfo<'info>,
    pub token_vault_1: &'me AccountInfo<'info>,
    pub vault_0_mint: &'me AccountInfo<'info>,
    pub vault_1_mint: &'me AccountInfo<'info>,
    pub recipient_token_account_0: &'me AccountInfo<'info>,
    pub recipient_token_account_1: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub token_program_2022: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CollectFundFeeKeys {
    pub owner: Pubkey,
    pub pool_state: Pubkey,
    pub amm_config: Pubkey,
    pub token_vault_0: Pubkey,
    pub token_vault_1: Pubkey,
    pub vault_0_mint: Pubkey,
    pub vault_1_mint: Pubkey,
    pub recipient_token_account_0: Pubkey,
    pub recipient_token_account_1: Pubkey,
    pub token_program: Pubkey,
    pub token_program_2022: Pubkey,
}
impl From<CollectFundFeeAccounts<'_, '_>> for CollectFundFeeKeys {
    fn from(accounts: CollectFundFeeAccounts) -> Self {
        Self {
            owner: *accounts.owner.key,
            pool_state: *accounts.pool_state.key,
            amm_config: *accounts.amm_config.key,
            token_vault_0: *accounts.token_vault_0.key,
            token_vault_1: *accounts.token_vault_1.key,
            vault_0_mint: *accounts.vault_0_mint.key,
            vault_1_mint: *accounts.vault_1_mint.key,
            recipient_token_account_0: *accounts.recipient_token_account_0.key,
            recipient_token_account_1: *accounts.recipient_token_account_1.key,
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
                pubkey: keys.token_vault_0,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_vault_1,
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
                pubkey: keys.recipient_token_account_0,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.recipient_token_account_1,
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
            pool_state: pubkeys[1],
            amm_config: pubkeys[2],
            token_vault_0: pubkeys[3],
            token_vault_1: pubkeys[4],
            vault_0_mint: pubkeys[5],
            vault_1_mint: pubkeys[6],
            recipient_token_account_0: pubkeys[7],
            recipient_token_account_1: pubkeys[8],
            token_program: pubkeys[9],
            token_program_2022: pubkeys[10],
        }
    }
}
impl<'info> From<CollectFundFeeAccounts<'_, 'info>>
for [AccountInfo<'info>; COLLECT_FUND_FEE_IX_ACCOUNTS_LEN] {
    fn from(accounts: CollectFundFeeAccounts<'_, 'info>) -> Self {
        [
            accounts.owner.clone(),
            accounts.pool_state.clone(),
            accounts.amm_config.clone(),
            accounts.token_vault_0.clone(),
            accounts.token_vault_1.clone(),
            accounts.vault_0_mint.clone(),
            accounts.vault_1_mint.clone(),
            accounts.recipient_token_account_0.clone(),
            accounts.recipient_token_account_1.clone(),
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
            pool_state: &arr[1],
            amm_config: &arr[2],
            token_vault_0: &arr[3],
            token_vault_1: &arr[4],
            vault_0_mint: &arr[5],
            vault_1_mint: &arr[6],
            recipient_token_account_0: &arr[7],
            recipient_token_account_1: &arr[8],
            token_program: &arr[9],
            token_program_2022: &arr[10],
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
    collect_fund_fee_ix_with_program_id(STABBLE_CLMM_PROGRAM_ID, keys, args)
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
    collect_fund_fee_invoke_with_program_id(STABBLE_CLMM_PROGRAM_ID, accounts, args)
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
        STABBLE_CLMM_PROGRAM_ID,
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
        (*accounts.pool_state.key, keys.pool_state),
        (*accounts.amm_config.key, keys.amm_config),
        (*accounts.token_vault_0.key, keys.token_vault_0),
        (*accounts.token_vault_1.key, keys.token_vault_1),
        (*accounts.vault_0_mint.key, keys.vault_0_mint),
        (*accounts.vault_1_mint.key, keys.vault_1_mint),
        (*accounts.recipient_token_account_0.key, keys.recipient_token_account_0),
        (*accounts.recipient_token_account_1.key, keys.recipient_token_account_1),
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
        accounts.token_vault_0,
        accounts.token_vault_1,
        accounts.recipient_token_account_0,
        accounts.recipient_token_account_1,
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
pub const COLLECT_PROTOCOL_FEE_IX_ACCOUNTS_LEN: usize = 11;
#[derive(Copy, Clone, Debug)]
pub struct CollectProtocolFeeAccounts<'me, 'info> {
    pub owner: &'me AccountInfo<'info>,
    pub pool_state: &'me AccountInfo<'info>,
    pub amm_config: &'me AccountInfo<'info>,
    pub token_vault_0: &'me AccountInfo<'info>,
    pub token_vault_1: &'me AccountInfo<'info>,
    pub vault_0_mint: &'me AccountInfo<'info>,
    pub vault_1_mint: &'me AccountInfo<'info>,
    pub recipient_token_account_0: &'me AccountInfo<'info>,
    pub recipient_token_account_1: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub token_program_2022: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CollectProtocolFeeKeys {
    pub owner: Pubkey,
    pub pool_state: Pubkey,
    pub amm_config: Pubkey,
    pub token_vault_0: Pubkey,
    pub token_vault_1: Pubkey,
    pub vault_0_mint: Pubkey,
    pub vault_1_mint: Pubkey,
    pub recipient_token_account_0: Pubkey,
    pub recipient_token_account_1: Pubkey,
    pub token_program: Pubkey,
    pub token_program_2022: Pubkey,
}
impl From<CollectProtocolFeeAccounts<'_, '_>> for CollectProtocolFeeKeys {
    fn from(accounts: CollectProtocolFeeAccounts) -> Self {
        Self {
            owner: *accounts.owner.key,
            pool_state: *accounts.pool_state.key,
            amm_config: *accounts.amm_config.key,
            token_vault_0: *accounts.token_vault_0.key,
            token_vault_1: *accounts.token_vault_1.key,
            vault_0_mint: *accounts.vault_0_mint.key,
            vault_1_mint: *accounts.vault_1_mint.key,
            recipient_token_account_0: *accounts.recipient_token_account_0.key,
            recipient_token_account_1: *accounts.recipient_token_account_1.key,
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
                pubkey: keys.token_vault_0,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_vault_1,
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
                pubkey: keys.recipient_token_account_0,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.recipient_token_account_1,
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
            pool_state: pubkeys[1],
            amm_config: pubkeys[2],
            token_vault_0: pubkeys[3],
            token_vault_1: pubkeys[4],
            vault_0_mint: pubkeys[5],
            vault_1_mint: pubkeys[6],
            recipient_token_account_0: pubkeys[7],
            recipient_token_account_1: pubkeys[8],
            token_program: pubkeys[9],
            token_program_2022: pubkeys[10],
        }
    }
}
impl<'info> From<CollectProtocolFeeAccounts<'_, 'info>>
for [AccountInfo<'info>; COLLECT_PROTOCOL_FEE_IX_ACCOUNTS_LEN] {
    fn from(accounts: CollectProtocolFeeAccounts<'_, 'info>) -> Self {
        [
            accounts.owner.clone(),
            accounts.pool_state.clone(),
            accounts.amm_config.clone(),
            accounts.token_vault_0.clone(),
            accounts.token_vault_1.clone(),
            accounts.vault_0_mint.clone(),
            accounts.vault_1_mint.clone(),
            accounts.recipient_token_account_0.clone(),
            accounts.recipient_token_account_1.clone(),
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
            pool_state: &arr[1],
            amm_config: &arr[2],
            token_vault_0: &arr[3],
            token_vault_1: &arr[4],
            vault_0_mint: &arr[5],
            vault_1_mint: &arr[6],
            recipient_token_account_0: &arr[7],
            recipient_token_account_1: &arr[8],
            token_program: &arr[9],
            token_program_2022: &arr[10],
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
    collect_protocol_fee_ix_with_program_id(STABBLE_CLMM_PROGRAM_ID, keys, args)
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
    collect_protocol_fee_invoke_with_program_id(STABBLE_CLMM_PROGRAM_ID, accounts, args)
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
        STABBLE_CLMM_PROGRAM_ID,
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
        (*accounts.pool_state.key, keys.pool_state),
        (*accounts.amm_config.key, keys.amm_config),
        (*accounts.token_vault_0.key, keys.token_vault_0),
        (*accounts.token_vault_1.key, keys.token_vault_1),
        (*accounts.vault_0_mint.key, keys.vault_0_mint),
        (*accounts.vault_1_mint.key, keys.vault_1_mint),
        (*accounts.recipient_token_account_0.key, keys.recipient_token_account_0),
        (*accounts.recipient_token_account_1.key, keys.recipient_token_account_1),
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
        accounts.token_vault_0,
        accounts.token_vault_1,
        accounts.recipient_token_account_0,
        accounts.recipient_token_account_1,
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
pub const COLLECT_REMAINING_REWARDS_IX_ACCOUNTS_LEN: usize = 8;
#[derive(Copy, Clone, Debug)]
pub struct CollectRemainingRewardsAccounts<'me, 'info> {
    pub reward_funder: &'me AccountInfo<'info>,
    pub funder_token_account: &'me AccountInfo<'info>,
    pub pool_state: &'me AccountInfo<'info>,
    pub reward_token_vault: &'me AccountInfo<'info>,
    pub reward_vault_mint: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub token_program_2022: &'me AccountInfo<'info>,
    pub memo_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CollectRemainingRewardsKeys {
    pub reward_funder: Pubkey,
    pub funder_token_account: Pubkey,
    pub pool_state: Pubkey,
    pub reward_token_vault: Pubkey,
    pub reward_vault_mint: Pubkey,
    pub token_program: Pubkey,
    pub token_program_2022: Pubkey,
    pub memo_program: Pubkey,
}
impl From<CollectRemainingRewardsAccounts<'_, '_>> for CollectRemainingRewardsKeys {
    fn from(accounts: CollectRemainingRewardsAccounts) -> Self {
        Self {
            reward_funder: *accounts.reward_funder.key,
            funder_token_account: *accounts.funder_token_account.key,
            pool_state: *accounts.pool_state.key,
            reward_token_vault: *accounts.reward_token_vault.key,
            reward_vault_mint: *accounts.reward_vault_mint.key,
            token_program: *accounts.token_program.key,
            token_program_2022: *accounts.token_program_2022.key,
            memo_program: *accounts.memo_program.key,
        }
    }
}
impl From<CollectRemainingRewardsKeys>
for [AccountMeta; COLLECT_REMAINING_REWARDS_IX_ACCOUNTS_LEN] {
    fn from(keys: CollectRemainingRewardsKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.reward_funder,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.funder_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pool_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.reward_token_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.reward_vault_mint,
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
        ]
    }
}
impl From<[Pubkey; COLLECT_REMAINING_REWARDS_IX_ACCOUNTS_LEN]>
for CollectRemainingRewardsKeys {
    fn from(pubkeys: [Pubkey; COLLECT_REMAINING_REWARDS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            reward_funder: pubkeys[0],
            funder_token_account: pubkeys[1],
            pool_state: pubkeys[2],
            reward_token_vault: pubkeys[3],
            reward_vault_mint: pubkeys[4],
            token_program: pubkeys[5],
            token_program_2022: pubkeys[6],
            memo_program: pubkeys[7],
        }
    }
}
impl<'info> From<CollectRemainingRewardsAccounts<'_, 'info>>
for [AccountInfo<'info>; COLLECT_REMAINING_REWARDS_IX_ACCOUNTS_LEN] {
    fn from(accounts: CollectRemainingRewardsAccounts<'_, 'info>) -> Self {
        [
            accounts.reward_funder.clone(),
            accounts.funder_token_account.clone(),
            accounts.pool_state.clone(),
            accounts.reward_token_vault.clone(),
            accounts.reward_vault_mint.clone(),
            accounts.token_program.clone(),
            accounts.token_program_2022.clone(),
            accounts.memo_program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; COLLECT_REMAINING_REWARDS_IX_ACCOUNTS_LEN]>
for CollectRemainingRewardsAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; COLLECT_REMAINING_REWARDS_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            reward_funder: &arr[0],
            funder_token_account: &arr[1],
            pool_state: &arr[2],
            reward_token_vault: &arr[3],
            reward_vault_mint: &arr[4],
            token_program: &arr[5],
            token_program_2022: &arr[6],
            memo_program: &arr[7],
        }
    }
}
pub const COLLECT_REMAINING_REWARDS_IX_DISCM: [u8; 8usize] = [
    18, 237, 166, 197, 34, 16, 213, 144,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CollectRemainingRewardsIxArgs {
    pub reward_index: u8,
}
#[derive(Clone, Debug, PartialEq)]
pub struct CollectRemainingRewardsIxData(pub CollectRemainingRewardsIxArgs);
impl From<CollectRemainingRewardsIxArgs> for CollectRemainingRewardsIxData {
    fn from(args: CollectRemainingRewardsIxArgs) -> Self {
        Self(args)
    }
}
impl CollectRemainingRewardsIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != COLLECT_REMAINING_REWARDS_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let reward_index: u8 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(CollectRemainingRewardsIxArgs {
                reward_index,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&COLLECT_REMAINING_REWARDS_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.reward_index, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn collect_remaining_rewards_ix_with_program_id(
    program_id: Pubkey,
    keys: CollectRemainingRewardsKeys,
    args: CollectRemainingRewardsIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; COLLECT_REMAINING_REWARDS_IX_ACCOUNTS_LEN] = keys.into();
    let data: CollectRemainingRewardsIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn collect_remaining_rewards_ix(
    keys: CollectRemainingRewardsKeys,
    args: CollectRemainingRewardsIxArgs,
) -> std::io::Result<Instruction> {
    collect_remaining_rewards_ix_with_program_id(STABBLE_CLMM_PROGRAM_ID, keys, args)
}
pub fn collect_remaining_rewards_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CollectRemainingRewardsAccounts<'_, '_>,
    args: CollectRemainingRewardsIxArgs,
) -> ProgramResult {
    let keys: CollectRemainingRewardsKeys = accounts.into();
    let ix = collect_remaining_rewards_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn collect_remaining_rewards_invoke(
    accounts: CollectRemainingRewardsAccounts<'_, '_>,
    args: CollectRemainingRewardsIxArgs,
) -> ProgramResult {
    collect_remaining_rewards_invoke_with_program_id(
        STABBLE_CLMM_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn collect_remaining_rewards_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CollectRemainingRewardsAccounts<'_, '_>,
    args: CollectRemainingRewardsIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CollectRemainingRewardsKeys = accounts.into();
    let ix = collect_remaining_rewards_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn collect_remaining_rewards_invoke_signed(
    accounts: CollectRemainingRewardsAccounts<'_, '_>,
    args: CollectRemainingRewardsIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    collect_remaining_rewards_invoke_signed_with_program_id(
        STABBLE_CLMM_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn collect_remaining_rewards_verify_account_keys(
    accounts: CollectRemainingRewardsAccounts<'_, '_>,
    keys: CollectRemainingRewardsKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.reward_funder.key, keys.reward_funder),
        (*accounts.funder_token_account.key, keys.funder_token_account),
        (*accounts.pool_state.key, keys.pool_state),
        (*accounts.reward_token_vault.key, keys.reward_token_vault),
        (*accounts.reward_vault_mint.key, keys.reward_vault_mint),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.token_program_2022.key, keys.token_program_2022),
        (*accounts.memo_program.key, keys.memo_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn collect_remaining_rewards_verify_writable_privileges<'me, 'info>(
    accounts: CollectRemainingRewardsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.funder_token_account,
        accounts.pool_state,
        accounts.reward_token_vault,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn collect_remaining_rewards_verify_signer_privileges<'me, 'info>(
    accounts: CollectRemainingRewardsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.reward_funder] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn collect_remaining_rewards_verify_account_privileges<'me, 'info>(
    accounts: CollectRemainingRewardsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    collect_remaining_rewards_verify_writable_privileges(accounts)?;
    collect_remaining_rewards_verify_signer_privileges(accounts)?;
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
    pub tick_spacing: u16,
    pub trade_fee_rate: u32,
    pub protocol_fee_rate: u32,
    pub fund_fee_rate: u32,
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
        let tick_spacing: u16 = crate::borsh_de_or_default(&mut reader)?;
        let trade_fee_rate: u32 = crate::borsh_de_or_default(&mut reader)?;
        let protocol_fee_rate: u32 = crate::borsh_de_or_default(&mut reader)?;
        let fund_fee_rate: u32 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(CreateAmmConfigIxArgs {
                index,
                tick_spacing,
                trade_fee_rate,
                protocol_fee_rate,
                fund_fee_rate,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CREATE_AMM_CONFIG_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.index, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.tick_spacing, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.trade_fee_rate, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.protocol_fee_rate, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.fund_fee_rate, &mut writer)?;
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
    create_amm_config_ix_with_program_id(STABBLE_CLMM_PROGRAM_ID, keys, args)
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
    create_amm_config_invoke_with_program_id(STABBLE_CLMM_PROGRAM_ID, accounts, args)
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
        STABBLE_CLMM_PROGRAM_ID,
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
pub const CREATE_OPERATION_ACCOUNT_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct CreateOperationAccountAccounts<'me, 'info> {
    pub owner: &'me AccountInfo<'info>,
    pub operation_state: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CreateOperationAccountKeys {
    pub owner: Pubkey,
    pub operation_state: Pubkey,
    pub system_program: Pubkey,
}
impl From<CreateOperationAccountAccounts<'_, '_>> for CreateOperationAccountKeys {
    fn from(accounts: CreateOperationAccountAccounts) -> Self {
        Self {
            owner: *accounts.owner.key,
            operation_state: *accounts.operation_state.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<CreateOperationAccountKeys>
for [AccountMeta; CREATE_OPERATION_ACCOUNT_IX_ACCOUNTS_LEN] {
    fn from(keys: CreateOperationAccountKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.owner,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.operation_state,
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
impl From<[Pubkey; CREATE_OPERATION_ACCOUNT_IX_ACCOUNTS_LEN]>
for CreateOperationAccountKeys {
    fn from(pubkeys: [Pubkey; CREATE_OPERATION_ACCOUNT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            owner: pubkeys[0],
            operation_state: pubkeys[1],
            system_program: pubkeys[2],
        }
    }
}
impl<'info> From<CreateOperationAccountAccounts<'_, 'info>>
for [AccountInfo<'info>; CREATE_OPERATION_ACCOUNT_IX_ACCOUNTS_LEN] {
    fn from(accounts: CreateOperationAccountAccounts<'_, 'info>) -> Self {
        [
            accounts.owner.clone(),
            accounts.operation_state.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; CREATE_OPERATION_ACCOUNT_IX_ACCOUNTS_LEN]>
for CreateOperationAccountAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; CREATE_OPERATION_ACCOUNT_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            owner: &arr[0],
            operation_state: &arr[1],
            system_program: &arr[2],
        }
    }
}
pub const CREATE_OPERATION_ACCOUNT_IX_DISCM: [u8; 8usize] = [
    63, 87, 148, 33, 109, 35, 8, 104,
];
#[derive(Clone, Debug, PartialEq)]
pub struct CreateOperationAccountIxData;
impl CreateOperationAccountIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CREATE_OPERATION_ACCOUNT_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CREATE_OPERATION_ACCOUNT_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn create_operation_account_ix_with_program_id(
    program_id: Pubkey,
    keys: CreateOperationAccountKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CREATE_OPERATION_ACCOUNT_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: CreateOperationAccountIxData.try_to_vec()?,
    })
}
pub fn create_operation_account_ix(
    keys: CreateOperationAccountKeys,
) -> std::io::Result<Instruction> {
    create_operation_account_ix_with_program_id(STABBLE_CLMM_PROGRAM_ID, keys)
}
pub fn create_operation_account_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CreateOperationAccountAccounts<'_, '_>,
) -> ProgramResult {
    let keys: CreateOperationAccountKeys = accounts.into();
    let ix = create_operation_account_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn create_operation_account_invoke(
    accounts: CreateOperationAccountAccounts<'_, '_>,
) -> ProgramResult {
    create_operation_account_invoke_with_program_id(STABBLE_CLMM_PROGRAM_ID, accounts)
}
pub fn create_operation_account_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CreateOperationAccountAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CreateOperationAccountKeys = accounts.into();
    let ix = create_operation_account_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn create_operation_account_invoke_signed(
    accounts: CreateOperationAccountAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    create_operation_account_invoke_signed_with_program_id(
        STABBLE_CLMM_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn create_operation_account_verify_account_keys(
    accounts: CreateOperationAccountAccounts<'_, '_>,
    keys: CreateOperationAccountKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.owner.key, keys.owner),
        (*accounts.operation_state.key, keys.operation_state),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn create_operation_account_verify_writable_privileges<'me, 'info>(
    accounts: CreateOperationAccountAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.owner, accounts.operation_state] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn create_operation_account_verify_signer_privileges<'me, 'info>(
    accounts: CreateOperationAccountAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.owner] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn create_operation_account_verify_account_privileges<'me, 'info>(
    accounts: CreateOperationAccountAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    create_operation_account_verify_writable_privileges(accounts)?;
    create_operation_account_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CREATE_POOL_IX_ACCOUNTS_LEN: usize = 13;
#[derive(Copy, Clone, Debug)]
pub struct CreatePoolAccounts<'me, 'info> {
    pub pool_creator: &'me AccountInfo<'info>,
    pub amm_config: &'me AccountInfo<'info>,
    pub pool_state: &'me AccountInfo<'info>,
    pub token_mint_0: &'me AccountInfo<'info>,
    pub token_mint_1: &'me AccountInfo<'info>,
    pub token_vault_0: &'me AccountInfo<'info>,
    pub token_vault_1: &'me AccountInfo<'info>,
    pub observation_state: &'me AccountInfo<'info>,
    pub tick_array_bitmap: &'me AccountInfo<'info>,
    pub token_program_0: &'me AccountInfo<'info>,
    pub token_program_1: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub rent: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CreatePoolKeys {
    pub pool_creator: Pubkey,
    pub amm_config: Pubkey,
    pub pool_state: Pubkey,
    pub token_mint_0: Pubkey,
    pub token_mint_1: Pubkey,
    pub token_vault_0: Pubkey,
    pub token_vault_1: Pubkey,
    pub observation_state: Pubkey,
    pub tick_array_bitmap: Pubkey,
    pub token_program_0: Pubkey,
    pub token_program_1: Pubkey,
    pub system_program: Pubkey,
    pub rent: Pubkey,
}
impl From<CreatePoolAccounts<'_, '_>> for CreatePoolKeys {
    fn from(accounts: CreatePoolAccounts) -> Self {
        Self {
            pool_creator: *accounts.pool_creator.key,
            amm_config: *accounts.amm_config.key,
            pool_state: *accounts.pool_state.key,
            token_mint_0: *accounts.token_mint_0.key,
            token_mint_1: *accounts.token_mint_1.key,
            token_vault_0: *accounts.token_vault_0.key,
            token_vault_1: *accounts.token_vault_1.key,
            observation_state: *accounts.observation_state.key,
            tick_array_bitmap: *accounts.tick_array_bitmap.key,
            token_program_0: *accounts.token_program_0.key,
            token_program_1: *accounts.token_program_1.key,
            system_program: *accounts.system_program.key,
            rent: *accounts.rent.key,
        }
    }
}
impl From<CreatePoolKeys> for [AccountMeta; CREATE_POOL_IX_ACCOUNTS_LEN] {
    fn from(keys: CreatePoolKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.pool_creator,
                is_signer: true,
                is_writable: true,
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
                pubkey: keys.token_mint_0,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_mint_1,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_vault_0,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_vault_1,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.observation_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.tick_array_bitmap,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_program_0,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_program_1,
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
impl From<[Pubkey; CREATE_POOL_IX_ACCOUNTS_LEN]> for CreatePoolKeys {
    fn from(pubkeys: [Pubkey; CREATE_POOL_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            pool_creator: pubkeys[0],
            amm_config: pubkeys[1],
            pool_state: pubkeys[2],
            token_mint_0: pubkeys[3],
            token_mint_1: pubkeys[4],
            token_vault_0: pubkeys[5],
            token_vault_1: pubkeys[6],
            observation_state: pubkeys[7],
            tick_array_bitmap: pubkeys[8],
            token_program_0: pubkeys[9],
            token_program_1: pubkeys[10],
            system_program: pubkeys[11],
            rent: pubkeys[12],
        }
    }
}
impl<'info> From<CreatePoolAccounts<'_, 'info>>
for [AccountInfo<'info>; CREATE_POOL_IX_ACCOUNTS_LEN] {
    fn from(accounts: CreatePoolAccounts<'_, 'info>) -> Self {
        [
            accounts.pool_creator.clone(),
            accounts.amm_config.clone(),
            accounts.pool_state.clone(),
            accounts.token_mint_0.clone(),
            accounts.token_mint_1.clone(),
            accounts.token_vault_0.clone(),
            accounts.token_vault_1.clone(),
            accounts.observation_state.clone(),
            accounts.tick_array_bitmap.clone(),
            accounts.token_program_0.clone(),
            accounts.token_program_1.clone(),
            accounts.system_program.clone(),
            accounts.rent.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CREATE_POOL_IX_ACCOUNTS_LEN]>
for CreatePoolAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; CREATE_POOL_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            pool_creator: &arr[0],
            amm_config: &arr[1],
            pool_state: &arr[2],
            token_mint_0: &arr[3],
            token_mint_1: &arr[4],
            token_vault_0: &arr[5],
            token_vault_1: &arr[6],
            observation_state: &arr[7],
            tick_array_bitmap: &arr[8],
            token_program_0: &arr[9],
            token_program_1: &arr[10],
            system_program: &arr[11],
            rent: &arr[12],
        }
    }
}
pub const CREATE_POOL_IX_DISCM: [u8; 8usize] = [233, 146, 209, 142, 207, 104, 64, 188];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CreatePoolIxArgs {
    pub sqrt_price_x64: u128,
    pub open_time: u64,
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
        let sqrt_price_x64: u128 = crate::borsh_de_or_default(&mut reader)?;
        let open_time: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(CreatePoolIxArgs {
                sqrt_price_x64,
                open_time,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CREATE_POOL_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.sqrt_price_x64, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.open_time, &mut writer)?;
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
    create_pool_ix_with_program_id(STABBLE_CLMM_PROGRAM_ID, keys, args)
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
    create_pool_invoke_with_program_id(STABBLE_CLMM_PROGRAM_ID, accounts, args)
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
        STABBLE_CLMM_PROGRAM_ID,
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
        (*accounts.pool_creator.key, keys.pool_creator),
        (*accounts.amm_config.key, keys.amm_config),
        (*accounts.pool_state.key, keys.pool_state),
        (*accounts.token_mint_0.key, keys.token_mint_0),
        (*accounts.token_mint_1.key, keys.token_mint_1),
        (*accounts.token_vault_0.key, keys.token_vault_0),
        (*accounts.token_vault_1.key, keys.token_vault_1),
        (*accounts.observation_state.key, keys.observation_state),
        (*accounts.tick_array_bitmap.key, keys.tick_array_bitmap),
        (*accounts.token_program_0.key, keys.token_program_0),
        (*accounts.token_program_1.key, keys.token_program_1),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.rent.key, keys.rent),
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
        accounts.pool_creator,
        accounts.pool_state,
        accounts.token_vault_0,
        accounts.token_vault_1,
        accounts.observation_state,
        accounts.tick_array_bitmap,
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
    for should_be_signer in [accounts.pool_creator] {
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
    create_support_mint_associated_ix_with_program_id(STABBLE_CLMM_PROGRAM_ID, keys)
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
        STABBLE_CLMM_PROGRAM_ID,
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
        STABBLE_CLMM_PROGRAM_ID,
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
pub const DECREASE_LIQUIDITY_V2_IX_ACCOUNTS_LEN: usize = 16;
#[derive(Copy, Clone, Debug)]
pub struct DecreaseLiquidityV2Accounts<'me, 'info> {
    pub nft_owner: &'me AccountInfo<'info>,
    pub nft_account: &'me AccountInfo<'info>,
    pub personal_position: &'me AccountInfo<'info>,
    pub pool_state: &'me AccountInfo<'info>,
    pub protocol_position: &'me AccountInfo<'info>,
    pub token_vault_0: &'me AccountInfo<'info>,
    pub token_vault_1: &'me AccountInfo<'info>,
    pub tick_array_lower: &'me AccountInfo<'info>,
    pub tick_array_upper: &'me AccountInfo<'info>,
    pub recipient_token_account_0: &'me AccountInfo<'info>,
    pub recipient_token_account_1: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub token_program_2022: &'me AccountInfo<'info>,
    pub memo_program: &'me AccountInfo<'info>,
    pub vault_0_mint: &'me AccountInfo<'info>,
    pub vault_1_mint: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct DecreaseLiquidityV2Keys {
    pub nft_owner: Pubkey,
    pub nft_account: Pubkey,
    pub personal_position: Pubkey,
    pub pool_state: Pubkey,
    pub protocol_position: Pubkey,
    pub token_vault_0: Pubkey,
    pub token_vault_1: Pubkey,
    pub tick_array_lower: Pubkey,
    pub tick_array_upper: Pubkey,
    pub recipient_token_account_0: Pubkey,
    pub recipient_token_account_1: Pubkey,
    pub token_program: Pubkey,
    pub token_program_2022: Pubkey,
    pub memo_program: Pubkey,
    pub vault_0_mint: Pubkey,
    pub vault_1_mint: Pubkey,
}
impl From<DecreaseLiquidityV2Accounts<'_, '_>> for DecreaseLiquidityV2Keys {
    fn from(accounts: DecreaseLiquidityV2Accounts) -> Self {
        Self {
            nft_owner: *accounts.nft_owner.key,
            nft_account: *accounts.nft_account.key,
            personal_position: *accounts.personal_position.key,
            pool_state: *accounts.pool_state.key,
            protocol_position: *accounts.protocol_position.key,
            token_vault_0: *accounts.token_vault_0.key,
            token_vault_1: *accounts.token_vault_1.key,
            tick_array_lower: *accounts.tick_array_lower.key,
            tick_array_upper: *accounts.tick_array_upper.key,
            recipient_token_account_0: *accounts.recipient_token_account_0.key,
            recipient_token_account_1: *accounts.recipient_token_account_1.key,
            token_program: *accounts.token_program.key,
            token_program_2022: *accounts.token_program_2022.key,
            memo_program: *accounts.memo_program.key,
            vault_0_mint: *accounts.vault_0_mint.key,
            vault_1_mint: *accounts.vault_1_mint.key,
        }
    }
}
impl From<DecreaseLiquidityV2Keys>
for [AccountMeta; DECREASE_LIQUIDITY_V2_IX_ACCOUNTS_LEN] {
    fn from(keys: DecreaseLiquidityV2Keys) -> Self {
        [
            AccountMeta {
                pubkey: keys.nft_owner,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.nft_account,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.personal_position,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pool_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.protocol_position,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_vault_0,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_vault_1,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.tick_array_lower,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.tick_array_upper,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.recipient_token_account_0,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.recipient_token_account_1,
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
                pubkey: keys.memo_program,
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
        ]
    }
}
impl From<[Pubkey; DECREASE_LIQUIDITY_V2_IX_ACCOUNTS_LEN]> for DecreaseLiquidityV2Keys {
    fn from(pubkeys: [Pubkey; DECREASE_LIQUIDITY_V2_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            nft_owner: pubkeys[0],
            nft_account: pubkeys[1],
            personal_position: pubkeys[2],
            pool_state: pubkeys[3],
            protocol_position: pubkeys[4],
            token_vault_0: pubkeys[5],
            token_vault_1: pubkeys[6],
            tick_array_lower: pubkeys[7],
            tick_array_upper: pubkeys[8],
            recipient_token_account_0: pubkeys[9],
            recipient_token_account_1: pubkeys[10],
            token_program: pubkeys[11],
            token_program_2022: pubkeys[12],
            memo_program: pubkeys[13],
            vault_0_mint: pubkeys[14],
            vault_1_mint: pubkeys[15],
        }
    }
}
impl<'info> From<DecreaseLiquidityV2Accounts<'_, 'info>>
for [AccountInfo<'info>; DECREASE_LIQUIDITY_V2_IX_ACCOUNTS_LEN] {
    fn from(accounts: DecreaseLiquidityV2Accounts<'_, 'info>) -> Self {
        [
            accounts.nft_owner.clone(),
            accounts.nft_account.clone(),
            accounts.personal_position.clone(),
            accounts.pool_state.clone(),
            accounts.protocol_position.clone(),
            accounts.token_vault_0.clone(),
            accounts.token_vault_1.clone(),
            accounts.tick_array_lower.clone(),
            accounts.tick_array_upper.clone(),
            accounts.recipient_token_account_0.clone(),
            accounts.recipient_token_account_1.clone(),
            accounts.token_program.clone(),
            accounts.token_program_2022.clone(),
            accounts.memo_program.clone(),
            accounts.vault_0_mint.clone(),
            accounts.vault_1_mint.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; DECREASE_LIQUIDITY_V2_IX_ACCOUNTS_LEN]>
for DecreaseLiquidityV2Accounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; DECREASE_LIQUIDITY_V2_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            nft_owner: &arr[0],
            nft_account: &arr[1],
            personal_position: &arr[2],
            pool_state: &arr[3],
            protocol_position: &arr[4],
            token_vault_0: &arr[5],
            token_vault_1: &arr[6],
            tick_array_lower: &arr[7],
            tick_array_upper: &arr[8],
            recipient_token_account_0: &arr[9],
            recipient_token_account_1: &arr[10],
            token_program: &arr[11],
            token_program_2022: &arr[12],
            memo_program: &arr[13],
            vault_0_mint: &arr[14],
            vault_1_mint: &arr[15],
        }
    }
}
pub const DECREASE_LIQUIDITY_V2_IX_DISCM: [u8; 8usize] = [
    58, 127, 188, 62, 79, 82, 196, 96,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DecreaseLiquidityV2IxArgs {
    pub liquidity: u128,
    pub amount_0_min: u64,
    pub amount_1_min: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct DecreaseLiquidityV2IxData(pub DecreaseLiquidityV2IxArgs);
impl From<DecreaseLiquidityV2IxArgs> for DecreaseLiquidityV2IxData {
    fn from(args: DecreaseLiquidityV2IxArgs) -> Self {
        Self(args)
    }
}
impl DecreaseLiquidityV2IxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != DECREASE_LIQUIDITY_V2_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let liquidity: u128 = crate::borsh_de_or_default(&mut reader)?;
        let amount_0_min: u64 = crate::borsh_de_or_default(&mut reader)?;
        let amount_1_min: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(DecreaseLiquidityV2IxArgs {
                liquidity,
                amount_0_min,
                amount_1_min,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&DECREASE_LIQUIDITY_V2_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.liquidity, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.amount_0_min, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.amount_1_min, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn decrease_liquidity_v2_ix_with_program_id(
    program_id: Pubkey,
    keys: DecreaseLiquidityV2Keys,
    args: DecreaseLiquidityV2IxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; DECREASE_LIQUIDITY_V2_IX_ACCOUNTS_LEN] = keys.into();
    let data: DecreaseLiquidityV2IxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn decrease_liquidity_v2_ix(
    keys: DecreaseLiquidityV2Keys,
    args: DecreaseLiquidityV2IxArgs,
) -> std::io::Result<Instruction> {
    decrease_liquidity_v2_ix_with_program_id(STABBLE_CLMM_PROGRAM_ID, keys, args)
}
pub fn decrease_liquidity_v2_invoke_with_program_id(
    program_id: Pubkey,
    accounts: DecreaseLiquidityV2Accounts<'_, '_>,
    args: DecreaseLiquidityV2IxArgs,
) -> ProgramResult {
    let keys: DecreaseLiquidityV2Keys = accounts.into();
    let ix = decrease_liquidity_v2_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn decrease_liquidity_v2_invoke(
    accounts: DecreaseLiquidityV2Accounts<'_, '_>,
    args: DecreaseLiquidityV2IxArgs,
) -> ProgramResult {
    decrease_liquidity_v2_invoke_with_program_id(STABBLE_CLMM_PROGRAM_ID, accounts, args)
}
pub fn decrease_liquidity_v2_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: DecreaseLiquidityV2Accounts<'_, '_>,
    args: DecreaseLiquidityV2IxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: DecreaseLiquidityV2Keys = accounts.into();
    let ix = decrease_liquidity_v2_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn decrease_liquidity_v2_invoke_signed(
    accounts: DecreaseLiquidityV2Accounts<'_, '_>,
    args: DecreaseLiquidityV2IxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    decrease_liquidity_v2_invoke_signed_with_program_id(
        STABBLE_CLMM_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn decrease_liquidity_v2_verify_account_keys(
    accounts: DecreaseLiquidityV2Accounts<'_, '_>,
    keys: DecreaseLiquidityV2Keys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.nft_owner.key, keys.nft_owner),
        (*accounts.nft_account.key, keys.nft_account),
        (*accounts.personal_position.key, keys.personal_position),
        (*accounts.pool_state.key, keys.pool_state),
        (*accounts.protocol_position.key, keys.protocol_position),
        (*accounts.token_vault_0.key, keys.token_vault_0),
        (*accounts.token_vault_1.key, keys.token_vault_1),
        (*accounts.tick_array_lower.key, keys.tick_array_lower),
        (*accounts.tick_array_upper.key, keys.tick_array_upper),
        (*accounts.recipient_token_account_0.key, keys.recipient_token_account_0),
        (*accounts.recipient_token_account_1.key, keys.recipient_token_account_1),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.token_program_2022.key, keys.token_program_2022),
        (*accounts.memo_program.key, keys.memo_program),
        (*accounts.vault_0_mint.key, keys.vault_0_mint),
        (*accounts.vault_1_mint.key, keys.vault_1_mint),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn decrease_liquidity_v2_verify_writable_privileges<'me, 'info>(
    accounts: DecreaseLiquidityV2Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.personal_position,
        accounts.pool_state,
        accounts.token_vault_0,
        accounts.token_vault_1,
        accounts.tick_array_lower,
        accounts.tick_array_upper,
        accounts.recipient_token_account_0,
        accounts.recipient_token_account_1,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn decrease_liquidity_v2_verify_signer_privileges<'me, 'info>(
    accounts: DecreaseLiquidityV2Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.nft_owner] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn decrease_liquidity_v2_verify_account_privileges<'me, 'info>(
    accounts: DecreaseLiquidityV2Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    decrease_liquidity_v2_verify_writable_privileges(accounts)?;
    decrease_liquidity_v2_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const INCREASE_LIQUIDITY_V2_IX_ACCOUNTS_LEN: usize = 15;
#[derive(Copy, Clone, Debug)]
pub struct IncreaseLiquidityV2Accounts<'me, 'info> {
    pub nft_owner: &'me AccountInfo<'info>,
    pub nft_account: &'me AccountInfo<'info>,
    pub pool_state: &'me AccountInfo<'info>,
    pub protocol_position: &'me AccountInfo<'info>,
    pub personal_position: &'me AccountInfo<'info>,
    pub tick_array_lower: &'me AccountInfo<'info>,
    pub tick_array_upper: &'me AccountInfo<'info>,
    pub token_account_0: &'me AccountInfo<'info>,
    pub token_account_1: &'me AccountInfo<'info>,
    pub token_vault_0: &'me AccountInfo<'info>,
    pub token_vault_1: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub token_program_2022: &'me AccountInfo<'info>,
    pub vault_0_mint: &'me AccountInfo<'info>,
    pub vault_1_mint: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct IncreaseLiquidityV2Keys {
    pub nft_owner: Pubkey,
    pub nft_account: Pubkey,
    pub pool_state: Pubkey,
    pub protocol_position: Pubkey,
    pub personal_position: Pubkey,
    pub tick_array_lower: Pubkey,
    pub tick_array_upper: Pubkey,
    pub token_account_0: Pubkey,
    pub token_account_1: Pubkey,
    pub token_vault_0: Pubkey,
    pub token_vault_1: Pubkey,
    pub token_program: Pubkey,
    pub token_program_2022: Pubkey,
    pub vault_0_mint: Pubkey,
    pub vault_1_mint: Pubkey,
}
impl From<IncreaseLiquidityV2Accounts<'_, '_>> for IncreaseLiquidityV2Keys {
    fn from(accounts: IncreaseLiquidityV2Accounts) -> Self {
        Self {
            nft_owner: *accounts.nft_owner.key,
            nft_account: *accounts.nft_account.key,
            pool_state: *accounts.pool_state.key,
            protocol_position: *accounts.protocol_position.key,
            personal_position: *accounts.personal_position.key,
            tick_array_lower: *accounts.tick_array_lower.key,
            tick_array_upper: *accounts.tick_array_upper.key,
            token_account_0: *accounts.token_account_0.key,
            token_account_1: *accounts.token_account_1.key,
            token_vault_0: *accounts.token_vault_0.key,
            token_vault_1: *accounts.token_vault_1.key,
            token_program: *accounts.token_program.key,
            token_program_2022: *accounts.token_program_2022.key,
            vault_0_mint: *accounts.vault_0_mint.key,
            vault_1_mint: *accounts.vault_1_mint.key,
        }
    }
}
impl From<IncreaseLiquidityV2Keys>
for [AccountMeta; INCREASE_LIQUIDITY_V2_IX_ACCOUNTS_LEN] {
    fn from(keys: IncreaseLiquidityV2Keys) -> Self {
        [
            AccountMeta {
                pubkey: keys.nft_owner,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.nft_account,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.protocol_position,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.personal_position,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.tick_array_lower,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.tick_array_upper,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_account_0,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_account_1,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_vault_0,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_vault_1,
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
        ]
    }
}
impl From<[Pubkey; INCREASE_LIQUIDITY_V2_IX_ACCOUNTS_LEN]> for IncreaseLiquidityV2Keys {
    fn from(pubkeys: [Pubkey; INCREASE_LIQUIDITY_V2_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            nft_owner: pubkeys[0],
            nft_account: pubkeys[1],
            pool_state: pubkeys[2],
            protocol_position: pubkeys[3],
            personal_position: pubkeys[4],
            tick_array_lower: pubkeys[5],
            tick_array_upper: pubkeys[6],
            token_account_0: pubkeys[7],
            token_account_1: pubkeys[8],
            token_vault_0: pubkeys[9],
            token_vault_1: pubkeys[10],
            token_program: pubkeys[11],
            token_program_2022: pubkeys[12],
            vault_0_mint: pubkeys[13],
            vault_1_mint: pubkeys[14],
        }
    }
}
impl<'info> From<IncreaseLiquidityV2Accounts<'_, 'info>>
for [AccountInfo<'info>; INCREASE_LIQUIDITY_V2_IX_ACCOUNTS_LEN] {
    fn from(accounts: IncreaseLiquidityV2Accounts<'_, 'info>) -> Self {
        [
            accounts.nft_owner.clone(),
            accounts.nft_account.clone(),
            accounts.pool_state.clone(),
            accounts.protocol_position.clone(),
            accounts.personal_position.clone(),
            accounts.tick_array_lower.clone(),
            accounts.tick_array_upper.clone(),
            accounts.token_account_0.clone(),
            accounts.token_account_1.clone(),
            accounts.token_vault_0.clone(),
            accounts.token_vault_1.clone(),
            accounts.token_program.clone(),
            accounts.token_program_2022.clone(),
            accounts.vault_0_mint.clone(),
            accounts.vault_1_mint.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; INCREASE_LIQUIDITY_V2_IX_ACCOUNTS_LEN]>
for IncreaseLiquidityV2Accounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; INCREASE_LIQUIDITY_V2_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            nft_owner: &arr[0],
            nft_account: &arr[1],
            pool_state: &arr[2],
            protocol_position: &arr[3],
            personal_position: &arr[4],
            tick_array_lower: &arr[5],
            tick_array_upper: &arr[6],
            token_account_0: &arr[7],
            token_account_1: &arr[8],
            token_vault_0: &arr[9],
            token_vault_1: &arr[10],
            token_program: &arr[11],
            token_program_2022: &arr[12],
            vault_0_mint: &arr[13],
            vault_1_mint: &arr[14],
        }
    }
}
pub const INCREASE_LIQUIDITY_V2_IX_DISCM: [u8; 8usize] = [
    133, 29, 89, 223, 69, 238, 176, 10,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct IncreaseLiquidityV2IxArgs {
    pub liquidity: u128,
    pub amount_0_max: u64,
    pub amount_1_max: u64,
    pub base_flag: Option<bool>,
}
#[derive(Clone, Debug, PartialEq)]
pub struct IncreaseLiquidityV2IxData(pub IncreaseLiquidityV2IxArgs);
impl From<IncreaseLiquidityV2IxArgs> for IncreaseLiquidityV2IxData {
    fn from(args: IncreaseLiquidityV2IxArgs) -> Self {
        Self(args)
    }
}
impl IncreaseLiquidityV2IxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INCREASE_LIQUIDITY_V2_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let liquidity: u128 = crate::borsh_de_or_default(&mut reader)?;
        let amount_0_max: u64 = crate::borsh_de_or_default(&mut reader)?;
        let amount_1_max: u64 = crate::borsh_de_or_default(&mut reader)?;
        let base_flag: Option<bool> = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(IncreaseLiquidityV2IxArgs {
                liquidity,
                amount_0_max,
                amount_1_max,
                base_flag,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INCREASE_LIQUIDITY_V2_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.liquidity, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.amount_0_max, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.amount_1_max, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.base_flag, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn increase_liquidity_v2_ix_with_program_id(
    program_id: Pubkey,
    keys: IncreaseLiquidityV2Keys,
    args: IncreaseLiquidityV2IxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; INCREASE_LIQUIDITY_V2_IX_ACCOUNTS_LEN] = keys.into();
    let data: IncreaseLiquidityV2IxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn increase_liquidity_v2_ix(
    keys: IncreaseLiquidityV2Keys,
    args: IncreaseLiquidityV2IxArgs,
) -> std::io::Result<Instruction> {
    increase_liquidity_v2_ix_with_program_id(STABBLE_CLMM_PROGRAM_ID, keys, args)
}
pub fn increase_liquidity_v2_invoke_with_program_id(
    program_id: Pubkey,
    accounts: IncreaseLiquidityV2Accounts<'_, '_>,
    args: IncreaseLiquidityV2IxArgs,
) -> ProgramResult {
    let keys: IncreaseLiquidityV2Keys = accounts.into();
    let ix = increase_liquidity_v2_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn increase_liquidity_v2_invoke(
    accounts: IncreaseLiquidityV2Accounts<'_, '_>,
    args: IncreaseLiquidityV2IxArgs,
) -> ProgramResult {
    increase_liquidity_v2_invoke_with_program_id(STABBLE_CLMM_PROGRAM_ID, accounts, args)
}
pub fn increase_liquidity_v2_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: IncreaseLiquidityV2Accounts<'_, '_>,
    args: IncreaseLiquidityV2IxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: IncreaseLiquidityV2Keys = accounts.into();
    let ix = increase_liquidity_v2_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn increase_liquidity_v2_invoke_signed(
    accounts: IncreaseLiquidityV2Accounts<'_, '_>,
    args: IncreaseLiquidityV2IxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    increase_liquidity_v2_invoke_signed_with_program_id(
        STABBLE_CLMM_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn increase_liquidity_v2_verify_account_keys(
    accounts: IncreaseLiquidityV2Accounts<'_, '_>,
    keys: IncreaseLiquidityV2Keys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.nft_owner.key, keys.nft_owner),
        (*accounts.nft_account.key, keys.nft_account),
        (*accounts.pool_state.key, keys.pool_state),
        (*accounts.protocol_position.key, keys.protocol_position),
        (*accounts.personal_position.key, keys.personal_position),
        (*accounts.tick_array_lower.key, keys.tick_array_lower),
        (*accounts.tick_array_upper.key, keys.tick_array_upper),
        (*accounts.token_account_0.key, keys.token_account_0),
        (*accounts.token_account_1.key, keys.token_account_1),
        (*accounts.token_vault_0.key, keys.token_vault_0),
        (*accounts.token_vault_1.key, keys.token_vault_1),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.token_program_2022.key, keys.token_program_2022),
        (*accounts.vault_0_mint.key, keys.vault_0_mint),
        (*accounts.vault_1_mint.key, keys.vault_1_mint),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn increase_liquidity_v2_verify_writable_privileges<'me, 'info>(
    accounts: IncreaseLiquidityV2Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.pool_state,
        accounts.personal_position,
        accounts.tick_array_lower,
        accounts.tick_array_upper,
        accounts.token_account_0,
        accounts.token_account_1,
        accounts.token_vault_0,
        accounts.token_vault_1,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn increase_liquidity_v2_verify_signer_privileges<'me, 'info>(
    accounts: IncreaseLiquidityV2Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.nft_owner] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn increase_liquidity_v2_verify_account_privileges<'me, 'info>(
    accounts: IncreaseLiquidityV2Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    increase_liquidity_v2_verify_writable_privileges(accounts)?;
    increase_liquidity_v2_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const INITIALIZE_REWARD_IX_ACCOUNTS_LEN: usize = 10;
#[derive(Copy, Clone, Debug)]
pub struct InitializeRewardAccounts<'me, 'info> {
    pub reward_funder: &'me AccountInfo<'info>,
    pub funder_token_account: &'me AccountInfo<'info>,
    pub amm_config: &'me AccountInfo<'info>,
    pub pool_state: &'me AccountInfo<'info>,
    pub operation_state: &'me AccountInfo<'info>,
    pub reward_token_mint: &'me AccountInfo<'info>,
    pub reward_token_vault: &'me AccountInfo<'info>,
    pub reward_token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub rent: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct InitializeRewardKeys {
    pub reward_funder: Pubkey,
    pub funder_token_account: Pubkey,
    pub amm_config: Pubkey,
    pub pool_state: Pubkey,
    pub operation_state: Pubkey,
    pub reward_token_mint: Pubkey,
    pub reward_token_vault: Pubkey,
    pub reward_token_program: Pubkey,
    pub system_program: Pubkey,
    pub rent: Pubkey,
}
impl From<InitializeRewardAccounts<'_, '_>> for InitializeRewardKeys {
    fn from(accounts: InitializeRewardAccounts) -> Self {
        Self {
            reward_funder: *accounts.reward_funder.key,
            funder_token_account: *accounts.funder_token_account.key,
            amm_config: *accounts.amm_config.key,
            pool_state: *accounts.pool_state.key,
            operation_state: *accounts.operation_state.key,
            reward_token_mint: *accounts.reward_token_mint.key,
            reward_token_vault: *accounts.reward_token_vault.key,
            reward_token_program: *accounts.reward_token_program.key,
            system_program: *accounts.system_program.key,
            rent: *accounts.rent.key,
        }
    }
}
impl From<InitializeRewardKeys> for [AccountMeta; INITIALIZE_REWARD_IX_ACCOUNTS_LEN] {
    fn from(keys: InitializeRewardKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.reward_funder,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.funder_token_account,
                is_signer: false,
                is_writable: true,
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
                pubkey: keys.operation_state,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.reward_token_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.reward_token_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.reward_token_program,
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
impl From<[Pubkey; INITIALIZE_REWARD_IX_ACCOUNTS_LEN]> for InitializeRewardKeys {
    fn from(pubkeys: [Pubkey; INITIALIZE_REWARD_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            reward_funder: pubkeys[0],
            funder_token_account: pubkeys[1],
            amm_config: pubkeys[2],
            pool_state: pubkeys[3],
            operation_state: pubkeys[4],
            reward_token_mint: pubkeys[5],
            reward_token_vault: pubkeys[6],
            reward_token_program: pubkeys[7],
            system_program: pubkeys[8],
            rent: pubkeys[9],
        }
    }
}
impl<'info> From<InitializeRewardAccounts<'_, 'info>>
for [AccountInfo<'info>; INITIALIZE_REWARD_IX_ACCOUNTS_LEN] {
    fn from(accounts: InitializeRewardAccounts<'_, 'info>) -> Self {
        [
            accounts.reward_funder.clone(),
            accounts.funder_token_account.clone(),
            accounts.amm_config.clone(),
            accounts.pool_state.clone(),
            accounts.operation_state.clone(),
            accounts.reward_token_mint.clone(),
            accounts.reward_token_vault.clone(),
            accounts.reward_token_program.clone(),
            accounts.system_program.clone(),
            accounts.rent.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; INITIALIZE_REWARD_IX_ACCOUNTS_LEN]>
for InitializeRewardAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; INITIALIZE_REWARD_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            reward_funder: &arr[0],
            funder_token_account: &arr[1],
            amm_config: &arr[2],
            pool_state: &arr[3],
            operation_state: &arr[4],
            reward_token_mint: &arr[5],
            reward_token_vault: &arr[6],
            reward_token_program: &arr[7],
            system_program: &arr[8],
            rent: &arr[9],
        }
    }
}
pub const INITIALIZE_REWARD_IX_DISCM: [u8; 8usize] = [
    95, 135, 192, 196, 242, 129, 230, 68,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct InitializeRewardIxArgs {
    pub param: InitializeRewardParam,
}
#[derive(Clone, Debug, PartialEq)]
pub struct InitializeRewardIxData(pub InitializeRewardIxArgs);
impl From<InitializeRewardIxArgs> for InitializeRewardIxData {
    fn from(args: InitializeRewardIxArgs) -> Self {
        Self(args)
    }
}
impl InitializeRewardIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INITIALIZE_REWARD_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let param = if reader.is_empty() {
            Default::default()
        } else {
            <InitializeRewardParam>::deserialize(&mut reader)?
        };
        Ok(Self(InitializeRewardIxArgs { param }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INITIALIZE_REWARD_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.param, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn initialize_reward_ix_with_program_id(
    program_id: Pubkey,
    keys: InitializeRewardKeys,
    args: InitializeRewardIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; INITIALIZE_REWARD_IX_ACCOUNTS_LEN] = keys.into();
    let data: InitializeRewardIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn initialize_reward_ix(
    keys: InitializeRewardKeys,
    args: InitializeRewardIxArgs,
) -> std::io::Result<Instruction> {
    initialize_reward_ix_with_program_id(STABBLE_CLMM_PROGRAM_ID, keys, args)
}
pub fn initialize_reward_invoke_with_program_id(
    program_id: Pubkey,
    accounts: InitializeRewardAccounts<'_, '_>,
    args: InitializeRewardIxArgs,
) -> ProgramResult {
    let keys: InitializeRewardKeys = accounts.into();
    let ix = initialize_reward_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn initialize_reward_invoke(
    accounts: InitializeRewardAccounts<'_, '_>,
    args: InitializeRewardIxArgs,
) -> ProgramResult {
    initialize_reward_invoke_with_program_id(STABBLE_CLMM_PROGRAM_ID, accounts, args)
}
pub fn initialize_reward_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: InitializeRewardAccounts<'_, '_>,
    args: InitializeRewardIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: InitializeRewardKeys = accounts.into();
    let ix = initialize_reward_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn initialize_reward_invoke_signed(
    accounts: InitializeRewardAccounts<'_, '_>,
    args: InitializeRewardIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    initialize_reward_invoke_signed_with_program_id(
        STABBLE_CLMM_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn initialize_reward_verify_account_keys(
    accounts: InitializeRewardAccounts<'_, '_>,
    keys: InitializeRewardKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.reward_funder.key, keys.reward_funder),
        (*accounts.funder_token_account.key, keys.funder_token_account),
        (*accounts.amm_config.key, keys.amm_config),
        (*accounts.pool_state.key, keys.pool_state),
        (*accounts.operation_state.key, keys.operation_state),
        (*accounts.reward_token_mint.key, keys.reward_token_mint),
        (*accounts.reward_token_vault.key, keys.reward_token_vault),
        (*accounts.reward_token_program.key, keys.reward_token_program),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.rent.key, keys.rent),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn initialize_reward_verify_writable_privileges<'me, 'info>(
    accounts: InitializeRewardAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.reward_funder,
        accounts.funder_token_account,
        accounts.pool_state,
        accounts.reward_token_vault,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn initialize_reward_verify_signer_privileges<'me, 'info>(
    accounts: InitializeRewardAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.reward_funder] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn initialize_reward_verify_account_privileges<'me, 'info>(
    accounts: InitializeRewardAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    initialize_reward_verify_writable_privileges(accounts)?;
    initialize_reward_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const OPEN_POSITION_WITH_TOKEN22_NFT_IX_ACCOUNTS_LEN: usize = 20;
#[derive(Copy, Clone, Debug)]
pub struct OpenPositionWithToken22NftAccounts<'me, 'info> {
    pub payer: &'me AccountInfo<'info>,
    pub position_nft_owner: &'me AccountInfo<'info>,
    pub position_nft_mint: &'me AccountInfo<'info>,
    pub position_nft_account: &'me AccountInfo<'info>,
    pub pool_state: &'me AccountInfo<'info>,
    pub protocol_position: &'me AccountInfo<'info>,
    pub tick_array_lower: &'me AccountInfo<'info>,
    pub tick_array_upper: &'me AccountInfo<'info>,
    pub personal_position: &'me AccountInfo<'info>,
    pub token_account_0: &'me AccountInfo<'info>,
    pub token_account_1: &'me AccountInfo<'info>,
    pub token_vault_0: &'me AccountInfo<'info>,
    pub token_vault_1: &'me AccountInfo<'info>,
    pub rent: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
    pub token_program_2022: &'me AccountInfo<'info>,
    pub vault_0_mint: &'me AccountInfo<'info>,
    pub vault_1_mint: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct OpenPositionWithToken22NftKeys {
    pub payer: Pubkey,
    pub position_nft_owner: Pubkey,
    pub position_nft_mint: Pubkey,
    pub position_nft_account: Pubkey,
    pub pool_state: Pubkey,
    pub protocol_position: Pubkey,
    pub tick_array_lower: Pubkey,
    pub tick_array_upper: Pubkey,
    pub personal_position: Pubkey,
    pub token_account_0: Pubkey,
    pub token_account_1: Pubkey,
    pub token_vault_0: Pubkey,
    pub token_vault_1: Pubkey,
    pub rent: Pubkey,
    pub system_program: Pubkey,
    pub token_program: Pubkey,
    pub associated_token_program: Pubkey,
    pub token_program_2022: Pubkey,
    pub vault_0_mint: Pubkey,
    pub vault_1_mint: Pubkey,
}
impl From<OpenPositionWithToken22NftAccounts<'_, '_>>
for OpenPositionWithToken22NftKeys {
    fn from(accounts: OpenPositionWithToken22NftAccounts) -> Self {
        Self {
            payer: *accounts.payer.key,
            position_nft_owner: *accounts.position_nft_owner.key,
            position_nft_mint: *accounts.position_nft_mint.key,
            position_nft_account: *accounts.position_nft_account.key,
            pool_state: *accounts.pool_state.key,
            protocol_position: *accounts.protocol_position.key,
            tick_array_lower: *accounts.tick_array_lower.key,
            tick_array_upper: *accounts.tick_array_upper.key,
            personal_position: *accounts.personal_position.key,
            token_account_0: *accounts.token_account_0.key,
            token_account_1: *accounts.token_account_1.key,
            token_vault_0: *accounts.token_vault_0.key,
            token_vault_1: *accounts.token_vault_1.key,
            rent: *accounts.rent.key,
            system_program: *accounts.system_program.key,
            token_program: *accounts.token_program.key,
            associated_token_program: *accounts.associated_token_program.key,
            token_program_2022: *accounts.token_program_2022.key,
            vault_0_mint: *accounts.vault_0_mint.key,
            vault_1_mint: *accounts.vault_1_mint.key,
        }
    }
}
impl From<OpenPositionWithToken22NftKeys>
for [AccountMeta; OPEN_POSITION_WITH_TOKEN22_NFT_IX_ACCOUNTS_LEN] {
    fn from(keys: OpenPositionWithToken22NftKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.payer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.position_nft_owner,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.position_nft_mint,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.position_nft_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pool_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.protocol_position,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.tick_array_lower,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.tick_array_upper,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.personal_position,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_account_0,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_account_1,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_vault_0,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_vault_1,
                is_signer: false,
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
        ]
    }
}
impl From<[Pubkey; OPEN_POSITION_WITH_TOKEN22_NFT_IX_ACCOUNTS_LEN]>
for OpenPositionWithToken22NftKeys {
    fn from(pubkeys: [Pubkey; OPEN_POSITION_WITH_TOKEN22_NFT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            payer: pubkeys[0],
            position_nft_owner: pubkeys[1],
            position_nft_mint: pubkeys[2],
            position_nft_account: pubkeys[3],
            pool_state: pubkeys[4],
            protocol_position: pubkeys[5],
            tick_array_lower: pubkeys[6],
            tick_array_upper: pubkeys[7],
            personal_position: pubkeys[8],
            token_account_0: pubkeys[9],
            token_account_1: pubkeys[10],
            token_vault_0: pubkeys[11],
            token_vault_1: pubkeys[12],
            rent: pubkeys[13],
            system_program: pubkeys[14],
            token_program: pubkeys[15],
            associated_token_program: pubkeys[16],
            token_program_2022: pubkeys[17],
            vault_0_mint: pubkeys[18],
            vault_1_mint: pubkeys[19],
        }
    }
}
impl<'info> From<OpenPositionWithToken22NftAccounts<'_, 'info>>
for [AccountInfo<'info>; OPEN_POSITION_WITH_TOKEN22_NFT_IX_ACCOUNTS_LEN] {
    fn from(accounts: OpenPositionWithToken22NftAccounts<'_, 'info>) -> Self {
        [
            accounts.payer.clone(),
            accounts.position_nft_owner.clone(),
            accounts.position_nft_mint.clone(),
            accounts.position_nft_account.clone(),
            accounts.pool_state.clone(),
            accounts.protocol_position.clone(),
            accounts.tick_array_lower.clone(),
            accounts.tick_array_upper.clone(),
            accounts.personal_position.clone(),
            accounts.token_account_0.clone(),
            accounts.token_account_1.clone(),
            accounts.token_vault_0.clone(),
            accounts.token_vault_1.clone(),
            accounts.rent.clone(),
            accounts.system_program.clone(),
            accounts.token_program.clone(),
            accounts.associated_token_program.clone(),
            accounts.token_program_2022.clone(),
            accounts.vault_0_mint.clone(),
            accounts.vault_1_mint.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; OPEN_POSITION_WITH_TOKEN22_NFT_IX_ACCOUNTS_LEN]>
for OpenPositionWithToken22NftAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; OPEN_POSITION_WITH_TOKEN22_NFT_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            payer: &arr[0],
            position_nft_owner: &arr[1],
            position_nft_mint: &arr[2],
            position_nft_account: &arr[3],
            pool_state: &arr[4],
            protocol_position: &arr[5],
            tick_array_lower: &arr[6],
            tick_array_upper: &arr[7],
            personal_position: &arr[8],
            token_account_0: &arr[9],
            token_account_1: &arr[10],
            token_vault_0: &arr[11],
            token_vault_1: &arr[12],
            rent: &arr[13],
            system_program: &arr[14],
            token_program: &arr[15],
            associated_token_program: &arr[16],
            token_program_2022: &arr[17],
            vault_0_mint: &arr[18],
            vault_1_mint: &arr[19],
        }
    }
}
pub const OPEN_POSITION_WITH_TOKEN22_NFT_IX_DISCM: [u8; 8usize] = [
    77, 255, 174, 82, 125, 29, 201, 46,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct OpenPositionWithToken22NftIxArgs {
    pub tick_lower_index: i32,
    pub tick_upper_index: i32,
    pub tick_array_lower_start_index: i32,
    pub tick_array_upper_start_index: i32,
    pub liquidity: u128,
    pub amount_0_max: u64,
    pub amount_1_max: u64,
    pub with_metadata: bool,
    pub base_flag: Option<bool>,
}
#[derive(Clone, Debug, PartialEq)]
pub struct OpenPositionWithToken22NftIxData(pub OpenPositionWithToken22NftIxArgs);
impl From<OpenPositionWithToken22NftIxArgs> for OpenPositionWithToken22NftIxData {
    fn from(args: OpenPositionWithToken22NftIxArgs) -> Self {
        Self(args)
    }
}
impl OpenPositionWithToken22NftIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != OPEN_POSITION_WITH_TOKEN22_NFT_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let tick_lower_index: i32 = crate::borsh_de_or_default(&mut reader)?;
        let tick_upper_index: i32 = crate::borsh_de_or_default(&mut reader)?;
        let tick_array_lower_start_index: i32 = crate::borsh_de_or_default(&mut reader)?;
        let tick_array_upper_start_index: i32 = crate::borsh_de_or_default(&mut reader)?;
        let liquidity: u128 = crate::borsh_de_or_default(&mut reader)?;
        let amount_0_max: u64 = crate::borsh_de_or_default(&mut reader)?;
        let amount_1_max: u64 = crate::borsh_de_or_default(&mut reader)?;
        let with_metadata: bool = crate::borsh_de_or_default(&mut reader)?;
        let base_flag: Option<bool> = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(OpenPositionWithToken22NftIxArgs {
                tick_lower_index,
                tick_upper_index,
                tick_array_lower_start_index,
                tick_array_upper_start_index,
                liquidity,
                amount_0_max,
                amount_1_max,
                with_metadata,
                base_flag,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&OPEN_POSITION_WITH_TOKEN22_NFT_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.tick_lower_index, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.tick_upper_index, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.0.tick_array_lower_start_index,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.0.tick_array_upper_start_index,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.0.liquidity, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.amount_0_max, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.amount_1_max, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.with_metadata, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.base_flag, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn open_position_with_token22_nft_ix_with_program_id(
    program_id: Pubkey,
    keys: OpenPositionWithToken22NftKeys,
    args: OpenPositionWithToken22NftIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; OPEN_POSITION_WITH_TOKEN22_NFT_IX_ACCOUNTS_LEN] = keys
        .into();
    let data: OpenPositionWithToken22NftIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn open_position_with_token22_nft_ix(
    keys: OpenPositionWithToken22NftKeys,
    args: OpenPositionWithToken22NftIxArgs,
) -> std::io::Result<Instruction> {
    open_position_with_token22_nft_ix_with_program_id(
        STABBLE_CLMM_PROGRAM_ID,
        keys,
        args,
    )
}
pub fn open_position_with_token22_nft_invoke_with_program_id(
    program_id: Pubkey,
    accounts: OpenPositionWithToken22NftAccounts<'_, '_>,
    args: OpenPositionWithToken22NftIxArgs,
) -> ProgramResult {
    let keys: OpenPositionWithToken22NftKeys = accounts.into();
    let ix = open_position_with_token22_nft_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn open_position_with_token22_nft_invoke(
    accounts: OpenPositionWithToken22NftAccounts<'_, '_>,
    args: OpenPositionWithToken22NftIxArgs,
) -> ProgramResult {
    open_position_with_token22_nft_invoke_with_program_id(
        STABBLE_CLMM_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn open_position_with_token22_nft_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: OpenPositionWithToken22NftAccounts<'_, '_>,
    args: OpenPositionWithToken22NftIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: OpenPositionWithToken22NftKeys = accounts.into();
    let ix = open_position_with_token22_nft_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn open_position_with_token22_nft_invoke_signed(
    accounts: OpenPositionWithToken22NftAccounts<'_, '_>,
    args: OpenPositionWithToken22NftIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    open_position_with_token22_nft_invoke_signed_with_program_id(
        STABBLE_CLMM_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn open_position_with_token22_nft_verify_account_keys(
    accounts: OpenPositionWithToken22NftAccounts<'_, '_>,
    keys: OpenPositionWithToken22NftKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.payer.key, keys.payer),
        (*accounts.position_nft_owner.key, keys.position_nft_owner),
        (*accounts.position_nft_mint.key, keys.position_nft_mint),
        (*accounts.position_nft_account.key, keys.position_nft_account),
        (*accounts.pool_state.key, keys.pool_state),
        (*accounts.protocol_position.key, keys.protocol_position),
        (*accounts.tick_array_lower.key, keys.tick_array_lower),
        (*accounts.tick_array_upper.key, keys.tick_array_upper),
        (*accounts.personal_position.key, keys.personal_position),
        (*accounts.token_account_0.key, keys.token_account_0),
        (*accounts.token_account_1.key, keys.token_account_1),
        (*accounts.token_vault_0.key, keys.token_vault_0),
        (*accounts.token_vault_1.key, keys.token_vault_1),
        (*accounts.rent.key, keys.rent),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.associated_token_program.key, keys.associated_token_program),
        (*accounts.token_program_2022.key, keys.token_program_2022),
        (*accounts.vault_0_mint.key, keys.vault_0_mint),
        (*accounts.vault_1_mint.key, keys.vault_1_mint),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn open_position_with_token22_nft_verify_writable_privileges<'me, 'info>(
    accounts: OpenPositionWithToken22NftAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.payer,
        accounts.position_nft_mint,
        accounts.position_nft_account,
        accounts.pool_state,
        accounts.tick_array_lower,
        accounts.tick_array_upper,
        accounts.personal_position,
        accounts.token_account_0,
        accounts.token_account_1,
        accounts.token_vault_0,
        accounts.token_vault_1,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn open_position_with_token22_nft_verify_signer_privileges<'me, 'info>(
    accounts: OpenPositionWithToken22NftAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.payer, accounts.position_nft_mint] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn open_position_with_token22_nft_verify_account_privileges<'me, 'info>(
    accounts: OpenPositionWithToken22NftAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    open_position_with_token22_nft_verify_writable_privileges(accounts)?;
    open_position_with_token22_nft_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SET_REWARD_PARAMS_IX_ACCOUNTS_LEN: usize = 6;
#[derive(Copy, Clone, Debug)]
pub struct SetRewardParamsAccounts<'me, 'info> {
    pub authority: &'me AccountInfo<'info>,
    pub amm_config: &'me AccountInfo<'info>,
    pub pool_state: &'me AccountInfo<'info>,
    pub operation_state: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub token_program_2022: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SetRewardParamsKeys {
    pub authority: Pubkey,
    pub amm_config: Pubkey,
    pub pool_state: Pubkey,
    pub operation_state: Pubkey,
    pub token_program: Pubkey,
    pub token_program_2022: Pubkey,
}
impl From<SetRewardParamsAccounts<'_, '_>> for SetRewardParamsKeys {
    fn from(accounts: SetRewardParamsAccounts) -> Self {
        Self {
            authority: *accounts.authority.key,
            amm_config: *accounts.amm_config.key,
            pool_state: *accounts.pool_state.key,
            operation_state: *accounts.operation_state.key,
            token_program: *accounts.token_program.key,
            token_program_2022: *accounts.token_program_2022.key,
        }
    }
}
impl From<SetRewardParamsKeys> for [AccountMeta; SET_REWARD_PARAMS_IX_ACCOUNTS_LEN] {
    fn from(keys: SetRewardParamsKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.authority,
                is_signer: true,
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
                pubkey: keys.operation_state,
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
impl From<[Pubkey; SET_REWARD_PARAMS_IX_ACCOUNTS_LEN]> for SetRewardParamsKeys {
    fn from(pubkeys: [Pubkey; SET_REWARD_PARAMS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            authority: pubkeys[0],
            amm_config: pubkeys[1],
            pool_state: pubkeys[2],
            operation_state: pubkeys[3],
            token_program: pubkeys[4],
            token_program_2022: pubkeys[5],
        }
    }
}
impl<'info> From<SetRewardParamsAccounts<'_, 'info>>
for [AccountInfo<'info>; SET_REWARD_PARAMS_IX_ACCOUNTS_LEN] {
    fn from(accounts: SetRewardParamsAccounts<'_, 'info>) -> Self {
        [
            accounts.authority.clone(),
            accounts.amm_config.clone(),
            accounts.pool_state.clone(),
            accounts.operation_state.clone(),
            accounts.token_program.clone(),
            accounts.token_program_2022.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SET_REWARD_PARAMS_IX_ACCOUNTS_LEN]>
for SetRewardParamsAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; SET_REWARD_PARAMS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            authority: &arr[0],
            amm_config: &arr[1],
            pool_state: &arr[2],
            operation_state: &arr[3],
            token_program: &arr[4],
            token_program_2022: &arr[5],
        }
    }
}
pub const SET_REWARD_PARAMS_IX_DISCM: [u8; 8usize] = [
    112, 52, 167, 75, 32, 201, 211, 137,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SetRewardParamsIxArgs {
    pub reward_index: u8,
    pub emissions_per_second_x64: u128,
    pub open_time: u64,
    pub end_time: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct SetRewardParamsIxData(pub SetRewardParamsIxArgs);
impl From<SetRewardParamsIxArgs> for SetRewardParamsIxData {
    fn from(args: SetRewardParamsIxArgs) -> Self {
        Self(args)
    }
}
impl SetRewardParamsIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SET_REWARD_PARAMS_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let reward_index: u8 = crate::borsh_de_or_default(&mut reader)?;
        let emissions_per_second_x64: u128 = crate::borsh_de_or_default(&mut reader)?;
        let open_time: u64 = crate::borsh_de_or_default(&mut reader)?;
        let end_time: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(SetRewardParamsIxArgs {
                reward_index,
                emissions_per_second_x64,
                open_time,
                end_time,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SET_REWARD_PARAMS_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.reward_index, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.emissions_per_second_x64, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.open_time, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.end_time, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn set_reward_params_ix_with_program_id(
    program_id: Pubkey,
    keys: SetRewardParamsKeys,
    args: SetRewardParamsIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SET_REWARD_PARAMS_IX_ACCOUNTS_LEN] = keys.into();
    let data: SetRewardParamsIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn set_reward_params_ix(
    keys: SetRewardParamsKeys,
    args: SetRewardParamsIxArgs,
) -> std::io::Result<Instruction> {
    set_reward_params_ix_with_program_id(STABBLE_CLMM_PROGRAM_ID, keys, args)
}
pub fn set_reward_params_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SetRewardParamsAccounts<'_, '_>,
    args: SetRewardParamsIxArgs,
) -> ProgramResult {
    let keys: SetRewardParamsKeys = accounts.into();
    let ix = set_reward_params_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn set_reward_params_invoke(
    accounts: SetRewardParamsAccounts<'_, '_>,
    args: SetRewardParamsIxArgs,
) -> ProgramResult {
    set_reward_params_invoke_with_program_id(STABBLE_CLMM_PROGRAM_ID, accounts, args)
}
pub fn set_reward_params_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SetRewardParamsAccounts<'_, '_>,
    args: SetRewardParamsIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SetRewardParamsKeys = accounts.into();
    let ix = set_reward_params_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn set_reward_params_invoke_signed(
    accounts: SetRewardParamsAccounts<'_, '_>,
    args: SetRewardParamsIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    set_reward_params_invoke_signed_with_program_id(
        STABBLE_CLMM_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn set_reward_params_verify_account_keys(
    accounts: SetRewardParamsAccounts<'_, '_>,
    keys: SetRewardParamsKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.authority.key, keys.authority),
        (*accounts.amm_config.key, keys.amm_config),
        (*accounts.pool_state.key, keys.pool_state),
        (*accounts.operation_state.key, keys.operation_state),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.token_program_2022.key, keys.token_program_2022),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn set_reward_params_verify_writable_privileges<'me, 'info>(
    accounts: SetRewardParamsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.pool_state] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn set_reward_params_verify_signer_privileges<'me, 'info>(
    accounts: SetRewardParamsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn set_reward_params_verify_account_privileges<'me, 'info>(
    accounts: SetRewardParamsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    set_reward_params_verify_writable_privileges(accounts)?;
    set_reward_params_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SWAP_ROUTER_BASE_IN_IX_ACCOUNTS_LEN: usize = 6;
#[derive(Copy, Clone, Debug)]
pub struct SwapRouterBaseInAccounts<'me, 'info> {
    pub payer: &'me AccountInfo<'info>,
    pub input_token_account: &'me AccountInfo<'info>,
    pub input_token_mint: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub token_program_2022: &'me AccountInfo<'info>,
    pub memo_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SwapRouterBaseInKeys {
    pub payer: Pubkey,
    pub input_token_account: Pubkey,
    pub input_token_mint: Pubkey,
    pub token_program: Pubkey,
    pub token_program_2022: Pubkey,
    pub memo_program: Pubkey,
}
impl From<SwapRouterBaseInAccounts<'_, '_>> for SwapRouterBaseInKeys {
    fn from(accounts: SwapRouterBaseInAccounts) -> Self {
        Self {
            payer: *accounts.payer.key,
            input_token_account: *accounts.input_token_account.key,
            input_token_mint: *accounts.input_token_mint.key,
            token_program: *accounts.token_program.key,
            token_program_2022: *accounts.token_program_2022.key,
            memo_program: *accounts.memo_program.key,
        }
    }
}
impl From<SwapRouterBaseInKeys> for [AccountMeta; SWAP_ROUTER_BASE_IN_IX_ACCOUNTS_LEN] {
    fn from(keys: SwapRouterBaseInKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.payer,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.input_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.input_token_mint,
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
                pubkey: keys.memo_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; SWAP_ROUTER_BASE_IN_IX_ACCOUNTS_LEN]> for SwapRouterBaseInKeys {
    fn from(pubkeys: [Pubkey; SWAP_ROUTER_BASE_IN_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            payer: pubkeys[0],
            input_token_account: pubkeys[1],
            input_token_mint: pubkeys[2],
            token_program: pubkeys[3],
            token_program_2022: pubkeys[4],
            memo_program: pubkeys[5],
        }
    }
}
impl<'info> From<SwapRouterBaseInAccounts<'_, 'info>>
for [AccountInfo<'info>; SWAP_ROUTER_BASE_IN_IX_ACCOUNTS_LEN] {
    fn from(accounts: SwapRouterBaseInAccounts<'_, 'info>) -> Self {
        [
            accounts.payer.clone(),
            accounts.input_token_account.clone(),
            accounts.input_token_mint.clone(),
            accounts.token_program.clone(),
            accounts.token_program_2022.clone(),
            accounts.memo_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SWAP_ROUTER_BASE_IN_IX_ACCOUNTS_LEN]>
for SwapRouterBaseInAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; SWAP_ROUTER_BASE_IN_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            payer: &arr[0],
            input_token_account: &arr[1],
            input_token_mint: &arr[2],
            token_program: &arr[3],
            token_program_2022: &arr[4],
            memo_program: &arr[5],
        }
    }
}
pub const SWAP_ROUTER_BASE_IN_IX_DISCM: [u8; 8usize] = [
    69, 125, 115, 218, 245, 186, 242, 196,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SwapRouterBaseInIxArgs {
    pub amount_in: u64,
    pub amount_out_minimum: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct SwapRouterBaseInIxData(pub SwapRouterBaseInIxArgs);
impl From<SwapRouterBaseInIxArgs> for SwapRouterBaseInIxData {
    fn from(args: SwapRouterBaseInIxArgs) -> Self {
        Self(args)
    }
}
impl SwapRouterBaseInIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SWAP_ROUTER_BASE_IN_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let amount_in: u64 = crate::borsh_de_or_default(&mut reader)?;
        let amount_out_minimum: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(SwapRouterBaseInIxArgs {
                amount_in,
                amount_out_minimum,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SWAP_ROUTER_BASE_IN_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.amount_in, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.amount_out_minimum, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn swap_router_base_in_ix_with_program_id(
    program_id: Pubkey,
    keys: SwapRouterBaseInKeys,
    args: SwapRouterBaseInIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SWAP_ROUTER_BASE_IN_IX_ACCOUNTS_LEN] = keys.into();
    let data: SwapRouterBaseInIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn swap_router_base_in_ix(
    keys: SwapRouterBaseInKeys,
    args: SwapRouterBaseInIxArgs,
) -> std::io::Result<Instruction> {
    swap_router_base_in_ix_with_program_id(STABBLE_CLMM_PROGRAM_ID, keys, args)
}
pub fn swap_router_base_in_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SwapRouterBaseInAccounts<'_, '_>,
    args: SwapRouterBaseInIxArgs,
) -> ProgramResult {
    let keys: SwapRouterBaseInKeys = accounts.into();
    let ix = swap_router_base_in_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn swap_router_base_in_invoke(
    accounts: SwapRouterBaseInAccounts<'_, '_>,
    args: SwapRouterBaseInIxArgs,
) -> ProgramResult {
    swap_router_base_in_invoke_with_program_id(STABBLE_CLMM_PROGRAM_ID, accounts, args)
}
pub fn swap_router_base_in_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SwapRouterBaseInAccounts<'_, '_>,
    args: SwapRouterBaseInIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SwapRouterBaseInKeys = accounts.into();
    let ix = swap_router_base_in_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn swap_router_base_in_invoke_signed(
    accounts: SwapRouterBaseInAccounts<'_, '_>,
    args: SwapRouterBaseInIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    swap_router_base_in_invoke_signed_with_program_id(
        STABBLE_CLMM_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn swap_router_base_in_verify_account_keys(
    accounts: SwapRouterBaseInAccounts<'_, '_>,
    keys: SwapRouterBaseInKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.payer.key, keys.payer),
        (*accounts.input_token_account.key, keys.input_token_account),
        (*accounts.input_token_mint.key, keys.input_token_mint),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.token_program_2022.key, keys.token_program_2022),
        (*accounts.memo_program.key, keys.memo_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn swap_router_base_in_verify_writable_privileges<'me, 'info>(
    accounts: SwapRouterBaseInAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.input_token_account, accounts.input_token_mint] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn swap_router_base_in_verify_signer_privileges<'me, 'info>(
    accounts: SwapRouterBaseInAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.payer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn swap_router_base_in_verify_account_privileges<'me, 'info>(
    accounts: SwapRouterBaseInAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    swap_router_base_in_verify_writable_privileges(accounts)?;
    swap_router_base_in_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SWAP_V2_IX_ACCOUNTS_LEN: usize = 13;
#[derive(Copy, Clone, Debug)]
pub struct SwapV2Accounts<'me, 'info> {
    pub payer: &'me AccountInfo<'info>,
    pub amm_config: &'me AccountInfo<'info>,
    pub pool_state: &'me AccountInfo<'info>,
    pub input_token_account: &'me AccountInfo<'info>,
    pub output_token_account: &'me AccountInfo<'info>,
    pub input_vault: &'me AccountInfo<'info>,
    pub output_vault: &'me AccountInfo<'info>,
    pub observation_state: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub token_program_2022: &'me AccountInfo<'info>,
    pub memo_program: &'me AccountInfo<'info>,
    pub input_vault_mint: &'me AccountInfo<'info>,
    pub output_vault_mint: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SwapV2Keys {
    pub payer: Pubkey,
    pub amm_config: Pubkey,
    pub pool_state: Pubkey,
    pub input_token_account: Pubkey,
    pub output_token_account: Pubkey,
    pub input_vault: Pubkey,
    pub output_vault: Pubkey,
    pub observation_state: Pubkey,
    pub token_program: Pubkey,
    pub token_program_2022: Pubkey,
    pub memo_program: Pubkey,
    pub input_vault_mint: Pubkey,
    pub output_vault_mint: Pubkey,
}
impl From<SwapV2Accounts<'_, '_>> for SwapV2Keys {
    fn from(accounts: SwapV2Accounts) -> Self {
        Self {
            payer: *accounts.payer.key,
            amm_config: *accounts.amm_config.key,
            pool_state: *accounts.pool_state.key,
            input_token_account: *accounts.input_token_account.key,
            output_token_account: *accounts.output_token_account.key,
            input_vault: *accounts.input_vault.key,
            output_vault: *accounts.output_vault.key,
            observation_state: *accounts.observation_state.key,
            token_program: *accounts.token_program.key,
            token_program_2022: *accounts.token_program_2022.key,
            memo_program: *accounts.memo_program.key,
            input_vault_mint: *accounts.input_vault_mint.key,
            output_vault_mint: *accounts.output_vault_mint.key,
        }
    }
}
impl From<SwapV2Keys> for [AccountMeta; SWAP_V2_IX_ACCOUNTS_LEN] {
    fn from(keys: SwapV2Keys) -> Self {
        [
            AccountMeta {
                pubkey: keys.payer,
                is_signer: true,
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
                pubkey: keys.input_vault_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.output_vault_mint,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; SWAP_V2_IX_ACCOUNTS_LEN]> for SwapV2Keys {
    fn from(pubkeys: [Pubkey; SWAP_V2_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            payer: pubkeys[0],
            amm_config: pubkeys[1],
            pool_state: pubkeys[2],
            input_token_account: pubkeys[3],
            output_token_account: pubkeys[4],
            input_vault: pubkeys[5],
            output_vault: pubkeys[6],
            observation_state: pubkeys[7],
            token_program: pubkeys[8],
            token_program_2022: pubkeys[9],
            memo_program: pubkeys[10],
            input_vault_mint: pubkeys[11],
            output_vault_mint: pubkeys[12],
        }
    }
}
impl<'info> From<SwapV2Accounts<'_, 'info>>
for [AccountInfo<'info>; SWAP_V2_IX_ACCOUNTS_LEN] {
    fn from(accounts: SwapV2Accounts<'_, 'info>) -> Self {
        [
            accounts.payer.clone(),
            accounts.amm_config.clone(),
            accounts.pool_state.clone(),
            accounts.input_token_account.clone(),
            accounts.output_token_account.clone(),
            accounts.input_vault.clone(),
            accounts.output_vault.clone(),
            accounts.observation_state.clone(),
            accounts.token_program.clone(),
            accounts.token_program_2022.clone(),
            accounts.memo_program.clone(),
            accounts.input_vault_mint.clone(),
            accounts.output_vault_mint.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SWAP_V2_IX_ACCOUNTS_LEN]>
for SwapV2Accounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; SWAP_V2_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            payer: &arr[0],
            amm_config: &arr[1],
            pool_state: &arr[2],
            input_token_account: &arr[3],
            output_token_account: &arr[4],
            input_vault: &arr[5],
            output_vault: &arr[6],
            observation_state: &arr[7],
            token_program: &arr[8],
            token_program_2022: &arr[9],
            memo_program: &arr[10],
            input_vault_mint: &arr[11],
            output_vault_mint: &arr[12],
        }
    }
}
pub const SWAP_V2_IX_DISCM: [u8; 8usize] = [43, 4, 237, 11, 26, 201, 30, 98];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SwapV2IxArgs {
    pub amount: u64,
    pub other_amount_threshold: u64,
    pub sqrt_price_limit_x64: u128,
    pub is_base_input: bool,
}
#[derive(Clone, Debug, PartialEq)]
pub struct SwapV2IxData(pub SwapV2IxArgs);
impl From<SwapV2IxArgs> for SwapV2IxData {
    fn from(args: SwapV2IxArgs) -> Self {
        Self(args)
    }
}
impl SwapV2IxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SWAP_V2_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let other_amount_threshold: u64 = crate::borsh_de_or_default(&mut reader)?;
        let sqrt_price_limit_x64: u128 = crate::borsh_de_or_default(&mut reader)?;
        let is_base_input: bool = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(SwapV2IxArgs {
                amount,
                other_amount_threshold,
                sqrt_price_limit_x64,
                is_base_input,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SWAP_V2_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.other_amount_threshold, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.sqrt_price_limit_x64, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.is_base_input, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn swap_v2_ix_with_program_id(
    program_id: Pubkey,
    keys: SwapV2Keys,
    args: SwapV2IxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SWAP_V2_IX_ACCOUNTS_LEN] = keys.into();
    let data: SwapV2IxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn swap_v2_ix(keys: SwapV2Keys, args: SwapV2IxArgs) -> std::io::Result<Instruction> {
    swap_v2_ix_with_program_id(STABBLE_CLMM_PROGRAM_ID, keys, args)
}
pub fn swap_v2_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SwapV2Accounts<'_, '_>,
    args: SwapV2IxArgs,
) -> ProgramResult {
    let keys: SwapV2Keys = accounts.into();
    let ix = swap_v2_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn swap_v2_invoke(
    accounts: SwapV2Accounts<'_, '_>,
    args: SwapV2IxArgs,
) -> ProgramResult {
    swap_v2_invoke_with_program_id(STABBLE_CLMM_PROGRAM_ID, accounts, args)
}
pub fn swap_v2_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SwapV2Accounts<'_, '_>,
    args: SwapV2IxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SwapV2Keys = accounts.into();
    let ix = swap_v2_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn swap_v2_invoke_signed(
    accounts: SwapV2Accounts<'_, '_>,
    args: SwapV2IxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    swap_v2_invoke_signed_with_program_id(STABBLE_CLMM_PROGRAM_ID, accounts, args, seeds)
}
pub fn swap_v2_verify_account_keys(
    accounts: SwapV2Accounts<'_, '_>,
    keys: SwapV2Keys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.payer.key, keys.payer),
        (*accounts.amm_config.key, keys.amm_config),
        (*accounts.pool_state.key, keys.pool_state),
        (*accounts.input_token_account.key, keys.input_token_account),
        (*accounts.output_token_account.key, keys.output_token_account),
        (*accounts.input_vault.key, keys.input_vault),
        (*accounts.output_vault.key, keys.output_vault),
        (*accounts.observation_state.key, keys.observation_state),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.token_program_2022.key, keys.token_program_2022),
        (*accounts.memo_program.key, keys.memo_program),
        (*accounts.input_vault_mint.key, keys.input_vault_mint),
        (*accounts.output_vault_mint.key, keys.output_vault_mint),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn swap_v2_verify_writable_privileges<'me, 'info>(
    accounts: SwapV2Accounts<'me, 'info>,
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
pub fn swap_v2_verify_signer_privileges<'me, 'info>(
    accounts: SwapV2Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.payer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn swap_v2_verify_account_privileges<'me, 'info>(
    accounts: SwapV2Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    swap_v2_verify_writable_privileges(accounts)?;
    swap_v2_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const TRANSFER_REWARD_OWNER_IX_ACCOUNTS_LEN: usize = 2;
#[derive(Copy, Clone, Debug)]
pub struct TransferRewardOwnerAccounts<'me, 'info> {
    pub authority: &'me AccountInfo<'info>,
    pub pool_state: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct TransferRewardOwnerKeys {
    pub authority: Pubkey,
    pub pool_state: Pubkey,
}
impl From<TransferRewardOwnerAccounts<'_, '_>> for TransferRewardOwnerKeys {
    fn from(accounts: TransferRewardOwnerAccounts) -> Self {
        Self {
            authority: *accounts.authority.key,
            pool_state: *accounts.pool_state.key,
        }
    }
}
impl From<TransferRewardOwnerKeys>
for [AccountMeta; TRANSFER_REWARD_OWNER_IX_ACCOUNTS_LEN] {
    fn from(keys: TransferRewardOwnerKeys) -> Self {
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
impl From<[Pubkey; TRANSFER_REWARD_OWNER_IX_ACCOUNTS_LEN]> for TransferRewardOwnerKeys {
    fn from(pubkeys: [Pubkey; TRANSFER_REWARD_OWNER_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            authority: pubkeys[0],
            pool_state: pubkeys[1],
        }
    }
}
impl<'info> From<TransferRewardOwnerAccounts<'_, 'info>>
for [AccountInfo<'info>; TRANSFER_REWARD_OWNER_IX_ACCOUNTS_LEN] {
    fn from(accounts: TransferRewardOwnerAccounts<'_, 'info>) -> Self {
        [accounts.authority.clone(), accounts.pool_state.clone()]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; TRANSFER_REWARD_OWNER_IX_ACCOUNTS_LEN]>
for TransferRewardOwnerAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; TRANSFER_REWARD_OWNER_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            authority: &arr[0],
            pool_state: &arr[1],
        }
    }
}
pub const TRANSFER_REWARD_OWNER_IX_DISCM: [u8; 8usize] = [
    7, 22, 12, 83, 242, 43, 48, 121,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TransferRewardOwnerIxArgs {
    pub new_owner: Pubkey,
}
#[derive(Clone, Debug, PartialEq)]
pub struct TransferRewardOwnerIxData(pub TransferRewardOwnerIxArgs);
impl From<TransferRewardOwnerIxArgs> for TransferRewardOwnerIxData {
    fn from(args: TransferRewardOwnerIxArgs) -> Self {
        Self(args)
    }
}
impl TransferRewardOwnerIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != TRANSFER_REWARD_OWNER_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let new_owner: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(TransferRewardOwnerIxArgs {
                new_owner,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&TRANSFER_REWARD_OWNER_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.new_owner, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn transfer_reward_owner_ix_with_program_id(
    program_id: Pubkey,
    keys: TransferRewardOwnerKeys,
    args: TransferRewardOwnerIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; TRANSFER_REWARD_OWNER_IX_ACCOUNTS_LEN] = keys.into();
    let data: TransferRewardOwnerIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn transfer_reward_owner_ix(
    keys: TransferRewardOwnerKeys,
    args: TransferRewardOwnerIxArgs,
) -> std::io::Result<Instruction> {
    transfer_reward_owner_ix_with_program_id(STABBLE_CLMM_PROGRAM_ID, keys, args)
}
pub fn transfer_reward_owner_invoke_with_program_id(
    program_id: Pubkey,
    accounts: TransferRewardOwnerAccounts<'_, '_>,
    args: TransferRewardOwnerIxArgs,
) -> ProgramResult {
    let keys: TransferRewardOwnerKeys = accounts.into();
    let ix = transfer_reward_owner_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn transfer_reward_owner_invoke(
    accounts: TransferRewardOwnerAccounts<'_, '_>,
    args: TransferRewardOwnerIxArgs,
) -> ProgramResult {
    transfer_reward_owner_invoke_with_program_id(STABBLE_CLMM_PROGRAM_ID, accounts, args)
}
pub fn transfer_reward_owner_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: TransferRewardOwnerAccounts<'_, '_>,
    args: TransferRewardOwnerIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: TransferRewardOwnerKeys = accounts.into();
    let ix = transfer_reward_owner_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn transfer_reward_owner_invoke_signed(
    accounts: TransferRewardOwnerAccounts<'_, '_>,
    args: TransferRewardOwnerIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    transfer_reward_owner_invoke_signed_with_program_id(
        STABBLE_CLMM_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn transfer_reward_owner_verify_account_keys(
    accounts: TransferRewardOwnerAccounts<'_, '_>,
    keys: TransferRewardOwnerKeys,
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
pub fn transfer_reward_owner_verify_writable_privileges<'me, 'info>(
    accounts: TransferRewardOwnerAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.pool_state] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn transfer_reward_owner_verify_signer_privileges<'me, 'info>(
    accounts: TransferRewardOwnerAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn transfer_reward_owner_verify_account_privileges<'me, 'info>(
    accounts: TransferRewardOwnerAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    transfer_reward_owner_verify_writable_privileges(accounts)?;
    transfer_reward_owner_verify_signer_privileges(accounts)?;
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
    pub value: u32,
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
        let value: u32 = crate::borsh_de_or_default(&mut reader)?;
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
    update_amm_config_ix_with_program_id(STABBLE_CLMM_PROGRAM_ID, keys, args)
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
    update_amm_config_invoke_with_program_id(STABBLE_CLMM_PROGRAM_ID, accounts, args)
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
        STABBLE_CLMM_PROGRAM_ID,
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
pub const UPDATE_OPERATION_ACCOUNT_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct UpdateOperationAccountAccounts<'me, 'info> {
    pub owner: &'me AccountInfo<'info>,
    pub operation_state: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UpdateOperationAccountKeys {
    pub owner: Pubkey,
    pub operation_state: Pubkey,
    pub system_program: Pubkey,
}
impl From<UpdateOperationAccountAccounts<'_, '_>> for UpdateOperationAccountKeys {
    fn from(accounts: UpdateOperationAccountAccounts) -> Self {
        Self {
            owner: *accounts.owner.key,
            operation_state: *accounts.operation_state.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<UpdateOperationAccountKeys>
for [AccountMeta; UPDATE_OPERATION_ACCOUNT_IX_ACCOUNTS_LEN] {
    fn from(keys: UpdateOperationAccountKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.owner,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.operation_state,
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
impl From<[Pubkey; UPDATE_OPERATION_ACCOUNT_IX_ACCOUNTS_LEN]>
for UpdateOperationAccountKeys {
    fn from(pubkeys: [Pubkey; UPDATE_OPERATION_ACCOUNT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            owner: pubkeys[0],
            operation_state: pubkeys[1],
            system_program: pubkeys[2],
        }
    }
}
impl<'info> From<UpdateOperationAccountAccounts<'_, 'info>>
for [AccountInfo<'info>; UPDATE_OPERATION_ACCOUNT_IX_ACCOUNTS_LEN] {
    fn from(accounts: UpdateOperationAccountAccounts<'_, 'info>) -> Self {
        [
            accounts.owner.clone(),
            accounts.operation_state.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; UPDATE_OPERATION_ACCOUNT_IX_ACCOUNTS_LEN]>
for UpdateOperationAccountAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; UPDATE_OPERATION_ACCOUNT_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            owner: &arr[0],
            operation_state: &arr[1],
            system_program: &arr[2],
        }
    }
}
pub const UPDATE_OPERATION_ACCOUNT_IX_DISCM: [u8; 8usize] = [
    127, 70, 119, 40, 188, 227, 61, 7,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UpdateOperationAccountIxArgs {
    pub param: u8,
    pub keys: Vec<Pubkey>,
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateOperationAccountIxData(pub UpdateOperationAccountIxArgs);
impl From<UpdateOperationAccountIxArgs> for UpdateOperationAccountIxData {
    fn from(args: UpdateOperationAccountIxArgs) -> Self {
        Self(args)
    }
}
impl UpdateOperationAccountIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_OPERATION_ACCOUNT_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let param: u8 = crate::borsh_de_or_default(&mut reader)?;
        let keys: Vec<Pubkey> = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(UpdateOperationAccountIxArgs {
                param,
                keys,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_OPERATION_ACCOUNT_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.param, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.keys, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn update_operation_account_ix_with_program_id(
    program_id: Pubkey,
    keys: UpdateOperationAccountKeys,
    args: UpdateOperationAccountIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; UPDATE_OPERATION_ACCOUNT_IX_ACCOUNTS_LEN] = keys.into();
    let data: UpdateOperationAccountIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_operation_account_ix(
    keys: UpdateOperationAccountKeys,
    args: UpdateOperationAccountIxArgs,
) -> std::io::Result<Instruction> {
    update_operation_account_ix_with_program_id(STABBLE_CLMM_PROGRAM_ID, keys, args)
}
pub fn update_operation_account_invoke_with_program_id(
    program_id: Pubkey,
    accounts: UpdateOperationAccountAccounts<'_, '_>,
    args: UpdateOperationAccountIxArgs,
) -> ProgramResult {
    let keys: UpdateOperationAccountKeys = accounts.into();
    let ix = update_operation_account_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn update_operation_account_invoke(
    accounts: UpdateOperationAccountAccounts<'_, '_>,
    args: UpdateOperationAccountIxArgs,
) -> ProgramResult {
    update_operation_account_invoke_with_program_id(
        STABBLE_CLMM_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn update_operation_account_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: UpdateOperationAccountAccounts<'_, '_>,
    args: UpdateOperationAccountIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: UpdateOperationAccountKeys = accounts.into();
    let ix = update_operation_account_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn update_operation_account_invoke_signed(
    accounts: UpdateOperationAccountAccounts<'_, '_>,
    args: UpdateOperationAccountIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    update_operation_account_invoke_signed_with_program_id(
        STABBLE_CLMM_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn update_operation_account_verify_account_keys(
    accounts: UpdateOperationAccountAccounts<'_, '_>,
    keys: UpdateOperationAccountKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.owner.key, keys.owner),
        (*accounts.operation_state.key, keys.operation_state),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn update_operation_account_verify_writable_privileges<'me, 'info>(
    accounts: UpdateOperationAccountAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.operation_state] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn update_operation_account_verify_signer_privileges<'me, 'info>(
    accounts: UpdateOperationAccountAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.owner] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn update_operation_account_verify_account_privileges<'me, 'info>(
    accounts: UpdateOperationAccountAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    update_operation_account_verify_writable_privileges(accounts)?;
    update_operation_account_verify_signer_privileges(accounts)?;
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
    update_pool_status_ix_with_program_id(STABBLE_CLMM_PROGRAM_ID, keys, args)
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
    update_pool_status_invoke_with_program_id(STABBLE_CLMM_PROGRAM_ID, accounts, args)
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
        STABBLE_CLMM_PROGRAM_ID,
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
pub const UPDATE_REWARD_INFOS_IX_ACCOUNTS_LEN: usize = 1;
#[derive(Copy, Clone, Debug)]
pub struct UpdateRewardInfosAccounts<'me, 'info> {
    pub pool_state: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UpdateRewardInfosKeys {
    pub pool_state: Pubkey,
}
impl From<UpdateRewardInfosAccounts<'_, '_>> for UpdateRewardInfosKeys {
    fn from(accounts: UpdateRewardInfosAccounts) -> Self {
        Self {
            pool_state: *accounts.pool_state.key,
        }
    }
}
impl From<UpdateRewardInfosKeys> for [AccountMeta; UPDATE_REWARD_INFOS_IX_ACCOUNTS_LEN] {
    fn from(keys: UpdateRewardInfosKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.pool_state,
                is_signer: false,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; UPDATE_REWARD_INFOS_IX_ACCOUNTS_LEN]> for UpdateRewardInfosKeys {
    fn from(pubkeys: [Pubkey; UPDATE_REWARD_INFOS_IX_ACCOUNTS_LEN]) -> Self {
        Self { pool_state: pubkeys[0] }
    }
}
impl<'info> From<UpdateRewardInfosAccounts<'_, 'info>>
for [AccountInfo<'info>; UPDATE_REWARD_INFOS_IX_ACCOUNTS_LEN] {
    fn from(accounts: UpdateRewardInfosAccounts<'_, 'info>) -> Self {
        [accounts.pool_state.clone()]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; UPDATE_REWARD_INFOS_IX_ACCOUNTS_LEN]>
for UpdateRewardInfosAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; UPDATE_REWARD_INFOS_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self { pool_state: &arr[0] }
    }
}
pub const UPDATE_REWARD_INFOS_IX_DISCM: [u8; 8usize] = [
    163, 172, 224, 52, 11, 154, 106, 223,
];
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateRewardInfosIxData;
impl UpdateRewardInfosIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_REWARD_INFOS_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_REWARD_INFOS_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn update_reward_infos_ix_with_program_id(
    program_id: Pubkey,
    keys: UpdateRewardInfosKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; UPDATE_REWARD_INFOS_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: UpdateRewardInfosIxData.try_to_vec()?,
    })
}
pub fn update_reward_infos_ix(
    keys: UpdateRewardInfosKeys,
) -> std::io::Result<Instruction> {
    update_reward_infos_ix_with_program_id(STABBLE_CLMM_PROGRAM_ID, keys)
}
pub fn update_reward_infos_invoke_with_program_id(
    program_id: Pubkey,
    accounts: UpdateRewardInfosAccounts<'_, '_>,
) -> ProgramResult {
    let keys: UpdateRewardInfosKeys = accounts.into();
    let ix = update_reward_infos_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn update_reward_infos_invoke(
    accounts: UpdateRewardInfosAccounts<'_, '_>,
) -> ProgramResult {
    update_reward_infos_invoke_with_program_id(STABBLE_CLMM_PROGRAM_ID, accounts)
}
pub fn update_reward_infos_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: UpdateRewardInfosAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: UpdateRewardInfosKeys = accounts.into();
    let ix = update_reward_infos_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn update_reward_infos_invoke_signed(
    accounts: UpdateRewardInfosAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    update_reward_infos_invoke_signed_with_program_id(
        STABBLE_CLMM_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn update_reward_infos_verify_account_keys(
    accounts: UpdateRewardInfosAccounts<'_, '_>,
    keys: UpdateRewardInfosKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [(*accounts.pool_state.key, keys.pool_state)] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn update_reward_infos_verify_writable_privileges<'me, 'info>(
    accounts: UpdateRewardInfosAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.pool_state] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn update_reward_infos_verify_account_privileges<'me, 'info>(
    accounts: UpdateRewardInfosAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    update_reward_infos_verify_writable_privileges(accounts)?;
    Ok(())
}
