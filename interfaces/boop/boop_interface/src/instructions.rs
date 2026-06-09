use solana_pubkey::Pubkey;
use solana_cpi::{invoke, invoke_signed};
use solana_instruction::{AccountMeta, Instruction};
use solana_account_info::AccountInfo;
use solana_program_error::ProgramError;
use std::io::Read;
#[allow(unused_imports)]
use crate::*;
#[derive(Clone, Debug, PartialEq)]
pub enum BoopProgramIx {
    AddOperators(AddOperatorsIxArgs),
    BuyToken(BuyTokenIxArgs),
    CancelAuthorityTransfer,
    CloseBondingCurveVault,
    CollectMeteoraTradingFees,
    CollectMeteoraTradingFeesV2,
    CollectTradingFees,
    CollectTradingFeesV2,
    CompleteAuthorityTransfer,
    CreateMeteoraPool,
    CreateRaydiumPool,
    CreateRaydiumRandomPool,
    CreateToken(CreateTokenIxArgs),
    CreateTokenFallback(CreateTokenFallbackIxArgs),
    DeployBondingCurve(DeployBondingCurveIxArgs),
    DeployBondingCurveFallback(DeployBondingCurveFallbackIxArgs),
    DepositIntoRaydium(DepositIntoRaydiumIxArgs),
    Graduate,
    Initialize(InitializeIxArgs),
    InitiateAuthorityTransfer(InitiateAuthorityTransferIxArgs),
    LockRaydiumLiquidity,
    RemoveOperators(RemoveOperatorsIxArgs),
    SellToken(SellTokenIxArgs),
    SplitPostGraduationTradingFees,
    SplitTradingFees,
    SwapSolForTokensOnRaydium(SwapSolForTokensOnRaydiumIxArgs),
    SwapTokensForSolOnRaydium(SwapTokensForSolOnRaydiumIxArgs),
    TogglePaused,
    UpdateConfig(UpdateConfigIxArgs),
}
impl BoopProgramIx {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        if buf.starts_with(&ADD_OPERATORS_IX_DISCM) {
            let mut reader = &buf[ADD_OPERATORS_IX_DISCM.len()..];
            let operators: Vec<Pubkey> = crate::borsh_de_or_default(&mut reader)?;
            return Ok(Self::AddOperators(AddOperatorsIxArgs { operators }));
        }
        if buf.starts_with(&BUY_TOKEN_IX_DISCM) {
            let mut reader = &buf[BUY_TOKEN_IX_DISCM.len()..];
            let buy_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            let amount_out_min: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::BuyToken(BuyTokenIxArgs {
                    buy_amount,
                    amount_out_min,
                }),
            );
        }
        if buf.starts_with(&CANCEL_AUTHORITY_TRANSFER_IX_DISCM) {
            return Ok(Self::CancelAuthorityTransfer);
        }
        if buf.starts_with(&CLOSE_BONDING_CURVE_VAULT_IX_DISCM) {
            return Ok(Self::CloseBondingCurveVault);
        }
        if buf.starts_with(&COLLECT_METEORA_TRADING_FEES_IX_DISCM) {
            return Ok(Self::CollectMeteoraTradingFees);
        }
        if buf.starts_with(&COLLECT_METEORA_TRADING_FEES_V2_IX_DISCM) {
            return Ok(Self::CollectMeteoraTradingFeesV2);
        }
        if buf.starts_with(&COLLECT_TRADING_FEES_IX_DISCM) {
            return Ok(Self::CollectTradingFees);
        }
        if buf.starts_with(&COLLECT_TRADING_FEES_V2_IX_DISCM) {
            return Ok(Self::CollectTradingFeesV2);
        }
        if buf.starts_with(&COMPLETE_AUTHORITY_TRANSFER_IX_DISCM) {
            return Ok(Self::CompleteAuthorityTransfer);
        }
        if buf.starts_with(&CREATE_METEORA_POOL_IX_DISCM) {
            return Ok(Self::CreateMeteoraPool);
        }
        if buf.starts_with(&CREATE_RAYDIUM_POOL_IX_DISCM) {
            return Ok(Self::CreateRaydiumPool);
        }
        if buf.starts_with(&CREATE_RAYDIUM_RANDOM_POOL_IX_DISCM) {
            return Ok(Self::CreateRaydiumRandomPool);
        }
        if buf.starts_with(&CREATE_TOKEN_IX_DISCM) {
            let mut reader = &buf[CREATE_TOKEN_IX_DISCM.len()..];
            let salt: u64 = crate::borsh_de_or_default(&mut reader)?;
            let name: String = crate::borsh_de_or_default(&mut reader)?;
            let symbol: String = crate::borsh_de_or_default(&mut reader)?;
            let uri: String = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::CreateToken(CreateTokenIxArgs {
                    salt,
                    name,
                    symbol,
                    uri,
                }),
            );
        }
        if buf.starts_with(&CREATE_TOKEN_FALLBACK_IX_DISCM) {
            let mut reader = &buf[CREATE_TOKEN_FALLBACK_IX_DISCM.len()..];
            let salt: u64 = crate::borsh_de_or_default(&mut reader)?;
            let name: String = crate::borsh_de_or_default(&mut reader)?;
            let symbol: String = crate::borsh_de_or_default(&mut reader)?;
            let uri: String = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::CreateTokenFallback(CreateTokenFallbackIxArgs {
                    salt,
                    name,
                    symbol,
                    uri,
                }),
            );
        }
        if buf.starts_with(&DEPLOY_BONDING_CURVE_IX_DISCM) {
            let mut reader = &buf[DEPLOY_BONDING_CURVE_IX_DISCM.len()..];
            let creator: Pubkey = crate::borsh_de_or_default(&mut reader)?;
            let salt: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::DeployBondingCurve(DeployBondingCurveIxArgs {
                    creator,
                    salt,
                }),
            );
        }
        if buf.starts_with(&DEPLOY_BONDING_CURVE_FALLBACK_IX_DISCM) {
            let mut reader = &buf[DEPLOY_BONDING_CURVE_FALLBACK_IX_DISCM.len()..];
            let creator: Pubkey = crate::borsh_de_or_default(&mut reader)?;
            let salt: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::DeployBondingCurveFallback(DeployBondingCurveFallbackIxArgs {
                    creator,
                    salt,
                }),
            );
        }
        if buf.starts_with(&DEPOSIT_INTO_RAYDIUM_IX_DISCM) {
            let mut reader = &buf[DEPOSIT_INTO_RAYDIUM_IX_DISCM.len()..];
            let lp_token_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            let maximum_token_0_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            let maximum_token_1_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::DepositIntoRaydium(DepositIntoRaydiumIxArgs {
                    lp_token_amount,
                    maximum_token_0_amount,
                    maximum_token_1_amount,
                }),
            );
        }
        if buf.starts_with(&GRADUATE_IX_DISCM) {
            return Ok(Self::Graduate);
        }
        if buf.starts_with(&INITIALIZE_IX_DISCM) {
            let mut reader = &buf[INITIALIZE_IX_DISCM.len()..];
            let protocol_fee_recipient: Pubkey = crate::borsh_de_or_default(
                &mut reader,
            )?;
            let token_distributor: Pubkey = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::Initialize(InitializeIxArgs {
                    protocol_fee_recipient,
                    token_distributor,
                }),
            );
        }
        if buf.starts_with(&INITIATE_AUTHORITY_TRANSFER_IX_DISCM) {
            let mut reader = &buf[INITIATE_AUTHORITY_TRANSFER_IX_DISCM.len()..];
            let new_authority: Pubkey = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::InitiateAuthorityTransfer(InitiateAuthorityTransferIxArgs {
                    new_authority,
                }),
            );
        }
        if buf.starts_with(&LOCK_RAYDIUM_LIQUIDITY_IX_DISCM) {
            return Ok(Self::LockRaydiumLiquidity);
        }
        if buf.starts_with(&REMOVE_OPERATORS_IX_DISCM) {
            let mut reader = &buf[REMOVE_OPERATORS_IX_DISCM.len()..];
            let operators: Vec<Pubkey> = crate::borsh_de_or_default(&mut reader)?;
            return Ok(Self::RemoveOperators(RemoveOperatorsIxArgs { operators }));
        }
        if buf.starts_with(&SELL_TOKEN_IX_DISCM) {
            let mut reader = &buf[SELL_TOKEN_IX_DISCM.len()..];
            let sell_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            let amount_out_min: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::SellToken(SellTokenIxArgs {
                    sell_amount,
                    amount_out_min,
                }),
            );
        }
        if buf.starts_with(&SPLIT_POST_GRADUATION_TRADING_FEES_IX_DISCM) {
            return Ok(Self::SplitPostGraduationTradingFees);
        }
        if buf.starts_with(&SPLIT_TRADING_FEES_IX_DISCM) {
            return Ok(Self::SplitTradingFees);
        }
        if buf.starts_with(&SWAP_SOL_FOR_TOKENS_ON_RAYDIUM_IX_DISCM) {
            let mut reader = &buf[SWAP_SOL_FOR_TOKENS_ON_RAYDIUM_IX_DISCM.len()..];
            let amount_in: u64 = crate::borsh_de_or_default(&mut reader)?;
            let minimum_amount_out: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::SwapSolForTokensOnRaydium(SwapSolForTokensOnRaydiumIxArgs {
                    amount_in,
                    minimum_amount_out,
                }),
            );
        }
        if buf.starts_with(&SWAP_TOKENS_FOR_SOL_ON_RAYDIUM_IX_DISCM) {
            let mut reader = &buf[SWAP_TOKENS_FOR_SOL_ON_RAYDIUM_IX_DISCM.len()..];
            let amount_in: u64 = crate::borsh_de_or_default(&mut reader)?;
            let minimum_amount_out: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::SwapTokensForSolOnRaydium(SwapTokensForSolOnRaydiumIxArgs {
                    amount_in,
                    minimum_amount_out,
                }),
            );
        }
        if buf.starts_with(&TOGGLE_PAUSED_IX_DISCM) {
            return Ok(Self::TogglePaused);
        }
        if buf.starts_with(&UPDATE_CONFIG_IX_DISCM) {
            let mut reader = &buf[UPDATE_CONFIG_IX_DISCM.len()..];
            let new_protocol_fee_recipient: Pubkey = crate::borsh_de_or_default(
                &mut reader,
            )?;
            let new_virtual_sol_reserves: u64 = crate::borsh_de_or_default(&mut reader)?;
            let new_virtual_token_reserves: u64 = crate::borsh_de_or_default(
                &mut reader,
            )?;
            let new_graduation_target: u64 = crate::borsh_de_or_default(&mut reader)?;
            let new_graduation_fee: u64 = crate::borsh_de_or_default(&mut reader)?;
            let new_damping_term: u8 = crate::borsh_de_or_default(&mut reader)?;
            let new_swap_fee_basis_points: u8 = crate::borsh_de_or_default(&mut reader)?;
            let new_token_for_stakers_basis_points: u16 = crate::borsh_de_or_default(
                &mut reader,
            )?;
            let new_token_amount_for_raydium_liquidity: u64 = crate::borsh_de_or_default(
                &mut reader,
            )?;
            let new_max_graduation_price_deviation_basis_points: u16 = crate::borsh_de_or_default(
                &mut reader,
            )?;
            let new_max_swap_amount_for_pool_price_correction_basis_points: u16 = crate::borsh_de_or_default(
                &mut reader,
            )?;
            return Ok(
                Self::UpdateConfig(UpdateConfigIxArgs {
                    new_protocol_fee_recipient,
                    new_virtual_sol_reserves,
                    new_virtual_token_reserves,
                    new_graduation_target,
                    new_graduation_fee,
                    new_damping_term,
                    new_swap_fee_basis_points,
                    new_token_for_stakers_basis_points,
                    new_token_amount_for_raydium_liquidity,
                    new_max_graduation_price_deviation_basis_points,
                    new_max_swap_amount_for_pool_price_correction_basis_points,
                }),
            );
        }
        Err(std::io::Error::from(std::io::ErrorKind::InvalidData))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        match self {
            Self::AddOperators(args) => {
                writer.write_all(&ADD_OPERATORS_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.operators, &mut writer)?;
                Ok(())
            }
            Self::BuyToken(args) => {
                writer.write_all(&BUY_TOKEN_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.buy_amount, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.amount_out_min, &mut writer)?;
                Ok(())
            }
            Self::CancelAuthorityTransfer => {
                writer.write_all(&CANCEL_AUTHORITY_TRANSFER_IX_DISCM)
            }
            Self::CloseBondingCurveVault => {
                writer.write_all(&CLOSE_BONDING_CURVE_VAULT_IX_DISCM)
            }
            Self::CollectMeteoraTradingFees => {
                writer.write_all(&COLLECT_METEORA_TRADING_FEES_IX_DISCM)
            }
            Self::CollectMeteoraTradingFeesV2 => {
                writer.write_all(&COLLECT_METEORA_TRADING_FEES_V2_IX_DISCM)
            }
            Self::CollectTradingFees => writer.write_all(&COLLECT_TRADING_FEES_IX_DISCM),
            Self::CollectTradingFeesV2 => {
                writer.write_all(&COLLECT_TRADING_FEES_V2_IX_DISCM)
            }
            Self::CompleteAuthorityTransfer => {
                writer.write_all(&COMPLETE_AUTHORITY_TRANSFER_IX_DISCM)
            }
            Self::CreateMeteoraPool => writer.write_all(&CREATE_METEORA_POOL_IX_DISCM),
            Self::CreateRaydiumPool => writer.write_all(&CREATE_RAYDIUM_POOL_IX_DISCM),
            Self::CreateRaydiumRandomPool => {
                writer.write_all(&CREATE_RAYDIUM_RANDOM_POOL_IX_DISCM)
            }
            Self::CreateToken(args) => {
                writer.write_all(&CREATE_TOKEN_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.salt, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.name, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.symbol, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.uri, &mut writer)?;
                Ok(())
            }
            Self::CreateTokenFallback(args) => {
                writer.write_all(&CREATE_TOKEN_FALLBACK_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.salt, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.name, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.symbol, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.uri, &mut writer)?;
                Ok(())
            }
            Self::DeployBondingCurve(args) => {
                writer.write_all(&DEPLOY_BONDING_CURVE_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.creator, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.salt, &mut writer)?;
                Ok(())
            }
            Self::DeployBondingCurveFallback(args) => {
                writer.write_all(&DEPLOY_BONDING_CURVE_FALLBACK_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.creator, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.salt, &mut writer)?;
                Ok(())
            }
            Self::DepositIntoRaydium(args) => {
                writer.write_all(&DEPOSIT_INTO_RAYDIUM_IX_DISCM)?;
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
            Self::Graduate => writer.write_all(&GRADUATE_IX_DISCM),
            Self::Initialize(args) => {
                writer.write_all(&INITIALIZE_IX_DISCM)?;
                borsh::BorshSerialize::serialize(
                    &args.protocol_fee_recipient,
                    &mut writer,
                )?;
                borsh::BorshSerialize::serialize(&args.token_distributor, &mut writer)?;
                Ok(())
            }
            Self::InitiateAuthorityTransfer(args) => {
                writer.write_all(&INITIATE_AUTHORITY_TRANSFER_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.new_authority, &mut writer)?;
                Ok(())
            }
            Self::LockRaydiumLiquidity => {
                writer.write_all(&LOCK_RAYDIUM_LIQUIDITY_IX_DISCM)
            }
            Self::RemoveOperators(args) => {
                writer.write_all(&REMOVE_OPERATORS_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.operators, &mut writer)?;
                Ok(())
            }
            Self::SellToken(args) => {
                writer.write_all(&SELL_TOKEN_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.sell_amount, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.amount_out_min, &mut writer)?;
                Ok(())
            }
            Self::SplitPostGraduationTradingFees => {
                writer.write_all(&SPLIT_POST_GRADUATION_TRADING_FEES_IX_DISCM)
            }
            Self::SplitTradingFees => writer.write_all(&SPLIT_TRADING_FEES_IX_DISCM),
            Self::SwapSolForTokensOnRaydium(args) => {
                writer.write_all(&SWAP_SOL_FOR_TOKENS_ON_RAYDIUM_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.amount_in, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.minimum_amount_out, &mut writer)?;
                Ok(())
            }
            Self::SwapTokensForSolOnRaydium(args) => {
                writer.write_all(&SWAP_TOKENS_FOR_SOL_ON_RAYDIUM_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.amount_in, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.minimum_amount_out, &mut writer)?;
                Ok(())
            }
            Self::TogglePaused => writer.write_all(&TOGGLE_PAUSED_IX_DISCM),
            Self::UpdateConfig(args) => {
                writer.write_all(&UPDATE_CONFIG_IX_DISCM)?;
                borsh::BorshSerialize::serialize(
                    &args.new_protocol_fee_recipient,
                    &mut writer,
                )?;
                borsh::BorshSerialize::serialize(
                    &args.new_virtual_sol_reserves,
                    &mut writer,
                )?;
                borsh::BorshSerialize::serialize(
                    &args.new_virtual_token_reserves,
                    &mut writer,
                )?;
                borsh::BorshSerialize::serialize(
                    &args.new_graduation_target,
                    &mut writer,
                )?;
                borsh::BorshSerialize::serialize(&args.new_graduation_fee, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.new_damping_term, &mut writer)?;
                borsh::BorshSerialize::serialize(
                    &args.new_swap_fee_basis_points,
                    &mut writer,
                )?;
                borsh::BorshSerialize::serialize(
                    &args.new_token_for_stakers_basis_points,
                    &mut writer,
                )?;
                borsh::BorshSerialize::serialize(
                    &args.new_token_amount_for_raydium_liquidity,
                    &mut writer,
                )?;
                borsh::BorshSerialize::serialize(
                    &args.new_max_graduation_price_deviation_basis_points,
                    &mut writer,
                )?;
                borsh::BorshSerialize::serialize(
                    &args.new_max_swap_amount_for_pool_price_correction_basis_points,
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
pub const ADD_OPERATORS_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct AddOperatorsAccounts<'me, 'info> {
    pub config: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct AddOperatorsKeys {
    pub config: Pubkey,
    pub authority: Pubkey,
    pub system_program: Pubkey,
}
impl From<AddOperatorsAccounts<'_, '_>> for AddOperatorsKeys {
    fn from(accounts: AddOperatorsAccounts) -> Self {
        Self {
            config: *accounts.config.key,
            authority: *accounts.authority.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<AddOperatorsKeys> for [AccountMeta; ADD_OPERATORS_IX_ACCOUNTS_LEN] {
    fn from(keys: AddOperatorsKeys) -> Self {
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
impl From<[Pubkey; ADD_OPERATORS_IX_ACCOUNTS_LEN]> for AddOperatorsKeys {
    fn from(pubkeys: [Pubkey; ADD_OPERATORS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            config: pubkeys[0],
            authority: pubkeys[1],
            system_program: pubkeys[2],
        }
    }
}
impl<'info> From<AddOperatorsAccounts<'_, 'info>>
for [AccountInfo<'info>; ADD_OPERATORS_IX_ACCOUNTS_LEN] {
    fn from(accounts: AddOperatorsAccounts<'_, 'info>) -> Self {
        [
            accounts.config.clone(),
            accounts.authority.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; ADD_OPERATORS_IX_ACCOUNTS_LEN]>
for AddOperatorsAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; ADD_OPERATORS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            config: &arr[0],
            authority: &arr[1],
            system_program: &arr[2],
        }
    }
}
pub const ADD_OPERATORS_IX_DISCM: [u8; 8usize] = [165, 199, 62, 214, 81, 54, 4, 150];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AddOperatorsIxArgs {
    pub operators: Vec<Pubkey>,
}
#[derive(Clone, Debug, PartialEq)]
pub struct AddOperatorsIxData(pub AddOperatorsIxArgs);
impl From<AddOperatorsIxArgs> for AddOperatorsIxData {
    fn from(args: AddOperatorsIxArgs) -> Self {
        Self(args)
    }
}
impl AddOperatorsIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != ADD_OPERATORS_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let operators: Vec<Pubkey> = crate::borsh_de_or_default(&mut reader)?;
        Ok(Self(AddOperatorsIxArgs { operators }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&ADD_OPERATORS_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.operators, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn add_operators_ix_with_program_id(
    program_id: Pubkey,
    keys: AddOperatorsKeys,
    args: AddOperatorsIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; ADD_OPERATORS_IX_ACCOUNTS_LEN] = keys.into();
    let data: AddOperatorsIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn add_operators_ix(
    keys: AddOperatorsKeys,
    args: AddOperatorsIxArgs,
) -> std::io::Result<Instruction> {
    add_operators_ix_with_program_id(BOOP_PROGRAM_ID, keys, args)
}
pub fn add_operators_invoke_with_program_id(
    program_id: Pubkey,
    accounts: AddOperatorsAccounts<'_, '_>,
    args: AddOperatorsIxArgs,
) -> ProgramResult {
    let keys: AddOperatorsKeys = accounts.into();
    let ix = add_operators_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn add_operators_invoke(
    accounts: AddOperatorsAccounts<'_, '_>,
    args: AddOperatorsIxArgs,
) -> ProgramResult {
    add_operators_invoke_with_program_id(BOOP_PROGRAM_ID, accounts, args)
}
pub fn add_operators_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: AddOperatorsAccounts<'_, '_>,
    args: AddOperatorsIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: AddOperatorsKeys = accounts.into();
    let ix = add_operators_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn add_operators_invoke_signed(
    accounts: AddOperatorsAccounts<'_, '_>,
    args: AddOperatorsIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    add_operators_invoke_signed_with_program_id(BOOP_PROGRAM_ID, accounts, args, seeds)
}
pub fn add_operators_verify_account_keys(
    accounts: AddOperatorsAccounts<'_, '_>,
    keys: AddOperatorsKeys,
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
pub fn add_operators_verify_writable_privileges<'me, 'info>(
    accounts: AddOperatorsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.config, accounts.authority] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn add_operators_verify_signer_privileges<'me, 'info>(
    accounts: AddOperatorsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn add_operators_verify_account_privileges<'me, 'info>(
    accounts: AddOperatorsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    add_operators_verify_writable_privileges(accounts)?;
    add_operators_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const BUY_TOKEN_IX_ACCOUNTS_LEN: usize = 13;
#[derive(Copy, Clone, Debug)]
pub struct BuyTokenAccounts<'me, 'info> {
    pub mint: &'me AccountInfo<'info>,
    pub bonding_curve: &'me AccountInfo<'info>,
    pub trading_fees_vault: &'me AccountInfo<'info>,
    pub bonding_curve_vault: &'me AccountInfo<'info>,
    pub bonding_curve_sol_vault: &'me AccountInfo<'info>,
    pub recipient_token_account: &'me AccountInfo<'info>,
    pub buyer: &'me AccountInfo<'info>,
    pub config: &'me AccountInfo<'info>,
    pub vault_authority: &'me AccountInfo<'info>,
    pub wsol: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct BuyTokenKeys {
    pub mint: Pubkey,
    pub bonding_curve: Pubkey,
    pub trading_fees_vault: Pubkey,
    pub bonding_curve_vault: Pubkey,
    pub bonding_curve_sol_vault: Pubkey,
    pub recipient_token_account: Pubkey,
    pub buyer: Pubkey,
    pub config: Pubkey,
    pub vault_authority: Pubkey,
    pub wsol: Pubkey,
    pub system_program: Pubkey,
    pub token_program: Pubkey,
    pub associated_token_program: Pubkey,
}
impl From<BuyTokenAccounts<'_, '_>> for BuyTokenKeys {
    fn from(accounts: BuyTokenAccounts) -> Self {
        Self {
            mint: *accounts.mint.key,
            bonding_curve: *accounts.bonding_curve.key,
            trading_fees_vault: *accounts.trading_fees_vault.key,
            bonding_curve_vault: *accounts.bonding_curve_vault.key,
            bonding_curve_sol_vault: *accounts.bonding_curve_sol_vault.key,
            recipient_token_account: *accounts.recipient_token_account.key,
            buyer: *accounts.buyer.key,
            config: *accounts.config.key,
            vault_authority: *accounts.vault_authority.key,
            wsol: *accounts.wsol.key,
            system_program: *accounts.system_program.key,
            token_program: *accounts.token_program.key,
            associated_token_program: *accounts.associated_token_program.key,
        }
    }
}
impl From<BuyTokenKeys> for [AccountMeta; BUY_TOKEN_IX_ACCOUNTS_LEN] {
    fn from(keys: BuyTokenKeys) -> Self {
        [
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
                pubkey: keys.trading_fees_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.bonding_curve_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.bonding_curve_sol_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.recipient_token_account,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.buyer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.wsol,
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
impl From<[Pubkey; BUY_TOKEN_IX_ACCOUNTS_LEN]> for BuyTokenKeys {
    fn from(pubkeys: [Pubkey; BUY_TOKEN_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            mint: pubkeys[0],
            bonding_curve: pubkeys[1],
            trading_fees_vault: pubkeys[2],
            bonding_curve_vault: pubkeys[3],
            bonding_curve_sol_vault: pubkeys[4],
            recipient_token_account: pubkeys[5],
            buyer: pubkeys[6],
            config: pubkeys[7],
            vault_authority: pubkeys[8],
            wsol: pubkeys[9],
            system_program: pubkeys[10],
            token_program: pubkeys[11],
            associated_token_program: pubkeys[12],
        }
    }
}
impl<'info> From<BuyTokenAccounts<'_, 'info>>
for [AccountInfo<'info>; BUY_TOKEN_IX_ACCOUNTS_LEN] {
    fn from(accounts: BuyTokenAccounts<'_, 'info>) -> Self {
        [
            accounts.mint.clone(),
            accounts.bonding_curve.clone(),
            accounts.trading_fees_vault.clone(),
            accounts.bonding_curve_vault.clone(),
            accounts.bonding_curve_sol_vault.clone(),
            accounts.recipient_token_account.clone(),
            accounts.buyer.clone(),
            accounts.config.clone(),
            accounts.vault_authority.clone(),
            accounts.wsol.clone(),
            accounts.system_program.clone(),
            accounts.token_program.clone(),
            accounts.associated_token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; BUY_TOKEN_IX_ACCOUNTS_LEN]>
for BuyTokenAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; BUY_TOKEN_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            mint: &arr[0],
            bonding_curve: &arr[1],
            trading_fees_vault: &arr[2],
            bonding_curve_vault: &arr[3],
            bonding_curve_sol_vault: &arr[4],
            recipient_token_account: &arr[5],
            buyer: &arr[6],
            config: &arr[7],
            vault_authority: &arr[8],
            wsol: &arr[9],
            system_program: &arr[10],
            token_program: &arr[11],
            associated_token_program: &arr[12],
        }
    }
}
pub const BUY_TOKEN_IX_DISCM: [u8; 8usize] = [138, 127, 14, 91, 38, 87, 115, 105];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct BuyTokenIxArgs {
    pub buy_amount: u64,
    pub amount_out_min: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct BuyTokenIxData(pub BuyTokenIxArgs);
impl From<BuyTokenIxArgs> for BuyTokenIxData {
    fn from(args: BuyTokenIxArgs) -> Self {
        Self(args)
    }
}
impl BuyTokenIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != BUY_TOKEN_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let buy_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let amount_out_min: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(BuyTokenIxArgs {
                buy_amount,
                amount_out_min,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&BUY_TOKEN_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.buy_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.amount_out_min, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn buy_token_ix_with_program_id(
    program_id: Pubkey,
    keys: BuyTokenKeys,
    args: BuyTokenIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; BUY_TOKEN_IX_ACCOUNTS_LEN] = keys.into();
    let data: BuyTokenIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn buy_token_ix(
    keys: BuyTokenKeys,
    args: BuyTokenIxArgs,
) -> std::io::Result<Instruction> {
    buy_token_ix_with_program_id(BOOP_PROGRAM_ID, keys, args)
}
pub fn buy_token_invoke_with_program_id(
    program_id: Pubkey,
    accounts: BuyTokenAccounts<'_, '_>,
    args: BuyTokenIxArgs,
) -> ProgramResult {
    let keys: BuyTokenKeys = accounts.into();
    let ix = buy_token_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn buy_token_invoke(
    accounts: BuyTokenAccounts<'_, '_>,
    args: BuyTokenIxArgs,
) -> ProgramResult {
    buy_token_invoke_with_program_id(BOOP_PROGRAM_ID, accounts, args)
}
pub fn buy_token_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: BuyTokenAccounts<'_, '_>,
    args: BuyTokenIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: BuyTokenKeys = accounts.into();
    let ix = buy_token_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn buy_token_invoke_signed(
    accounts: BuyTokenAccounts<'_, '_>,
    args: BuyTokenIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    buy_token_invoke_signed_with_program_id(BOOP_PROGRAM_ID, accounts, args, seeds)
}
pub fn buy_token_verify_account_keys(
    accounts: BuyTokenAccounts<'_, '_>,
    keys: BuyTokenKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.mint.key, keys.mint),
        (*accounts.bonding_curve.key, keys.bonding_curve),
        (*accounts.trading_fees_vault.key, keys.trading_fees_vault),
        (*accounts.bonding_curve_vault.key, keys.bonding_curve_vault),
        (*accounts.bonding_curve_sol_vault.key, keys.bonding_curve_sol_vault),
        (*accounts.recipient_token_account.key, keys.recipient_token_account),
        (*accounts.buyer.key, keys.buyer),
        (*accounts.config.key, keys.config),
        (*accounts.vault_authority.key, keys.vault_authority),
        (*accounts.wsol.key, keys.wsol),
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
pub fn buy_token_verify_writable_privileges<'me, 'info>(
    accounts: BuyTokenAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.bonding_curve,
        accounts.trading_fees_vault,
        accounts.bonding_curve_vault,
        accounts.bonding_curve_sol_vault,
        accounts.buyer,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn buy_token_verify_signer_privileges<'me, 'info>(
    accounts: BuyTokenAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.buyer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn buy_token_verify_account_privileges<'me, 'info>(
    accounts: BuyTokenAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    buy_token_verify_writable_privileges(accounts)?;
    buy_token_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CANCEL_AUTHORITY_TRANSFER_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct CancelAuthorityTransferAccounts<'me, 'info> {
    pub authority: &'me AccountInfo<'info>,
    pub config: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CancelAuthorityTransferKeys {
    pub authority: Pubkey,
    pub config: Pubkey,
    pub system_program: Pubkey,
}
impl From<CancelAuthorityTransferAccounts<'_, '_>> for CancelAuthorityTransferKeys {
    fn from(accounts: CancelAuthorityTransferAccounts) -> Self {
        Self {
            authority: *accounts.authority.key,
            config: *accounts.config.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<CancelAuthorityTransferKeys>
for [AccountMeta; CANCEL_AUTHORITY_TRANSFER_IX_ACCOUNTS_LEN] {
    fn from(keys: CancelAuthorityTransferKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.authority,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.config,
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
impl From<[Pubkey; CANCEL_AUTHORITY_TRANSFER_IX_ACCOUNTS_LEN]>
for CancelAuthorityTransferKeys {
    fn from(pubkeys: [Pubkey; CANCEL_AUTHORITY_TRANSFER_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            authority: pubkeys[0],
            config: pubkeys[1],
            system_program: pubkeys[2],
        }
    }
}
impl<'info> From<CancelAuthorityTransferAccounts<'_, 'info>>
for [AccountInfo<'info>; CANCEL_AUTHORITY_TRANSFER_IX_ACCOUNTS_LEN] {
    fn from(accounts: CancelAuthorityTransferAccounts<'_, 'info>) -> Self {
        [
            accounts.authority.clone(),
            accounts.config.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; CANCEL_AUTHORITY_TRANSFER_IX_ACCOUNTS_LEN]>
for CancelAuthorityTransferAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; CANCEL_AUTHORITY_TRANSFER_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            authority: &arr[0],
            config: &arr[1],
            system_program: &arr[2],
        }
    }
}
pub const CANCEL_AUTHORITY_TRANSFER_IX_DISCM: [u8; 8usize] = [
    94, 131, 125, 184, 183, 24, 125, 229,
];
#[derive(Clone, Debug, PartialEq)]
pub struct CancelAuthorityTransferIxData;
impl CancelAuthorityTransferIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CANCEL_AUTHORITY_TRANSFER_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CANCEL_AUTHORITY_TRANSFER_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn cancel_authority_transfer_ix_with_program_id(
    program_id: Pubkey,
    keys: CancelAuthorityTransferKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CANCEL_AUTHORITY_TRANSFER_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: CancelAuthorityTransferIxData.try_to_vec()?,
    })
}
pub fn cancel_authority_transfer_ix(
    keys: CancelAuthorityTransferKeys,
) -> std::io::Result<Instruction> {
    cancel_authority_transfer_ix_with_program_id(BOOP_PROGRAM_ID, keys)
}
pub fn cancel_authority_transfer_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CancelAuthorityTransferAccounts<'_, '_>,
) -> ProgramResult {
    let keys: CancelAuthorityTransferKeys = accounts.into();
    let ix = cancel_authority_transfer_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn cancel_authority_transfer_invoke(
    accounts: CancelAuthorityTransferAccounts<'_, '_>,
) -> ProgramResult {
    cancel_authority_transfer_invoke_with_program_id(BOOP_PROGRAM_ID, accounts)
}
pub fn cancel_authority_transfer_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CancelAuthorityTransferAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CancelAuthorityTransferKeys = accounts.into();
    let ix = cancel_authority_transfer_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn cancel_authority_transfer_invoke_signed(
    accounts: CancelAuthorityTransferAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    cancel_authority_transfer_invoke_signed_with_program_id(
        BOOP_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn cancel_authority_transfer_verify_account_keys(
    accounts: CancelAuthorityTransferAccounts<'_, '_>,
    keys: CancelAuthorityTransferKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.authority.key, keys.authority),
        (*accounts.config.key, keys.config),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn cancel_authority_transfer_verify_writable_privileges<'me, 'info>(
    accounts: CancelAuthorityTransferAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.config] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn cancel_authority_transfer_verify_signer_privileges<'me, 'info>(
    accounts: CancelAuthorityTransferAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn cancel_authority_transfer_verify_account_privileges<'me, 'info>(
    accounts: CancelAuthorityTransferAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    cancel_authority_transfer_verify_writable_privileges(accounts)?;
    cancel_authority_transfer_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CLOSE_BONDING_CURVE_VAULT_IX_ACCOUNTS_LEN: usize = 12;
#[derive(Copy, Clone, Debug)]
pub struct CloseBondingCurveVaultAccounts<'me, 'info> {
    pub config: &'me AccountInfo<'info>,
    pub operator: &'me AccountInfo<'info>,
    pub vault_authority: &'me AccountInfo<'info>,
    pub bonding_curve: &'me AccountInfo<'info>,
    pub bonding_curve_vault: &'me AccountInfo<'info>,
    pub mint: &'me AccountInfo<'info>,
    pub recipient_token_account: &'me AccountInfo<'info>,
    pub recipient: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
    pub rent: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CloseBondingCurveVaultKeys {
    pub config: Pubkey,
    pub operator: Pubkey,
    pub vault_authority: Pubkey,
    pub bonding_curve: Pubkey,
    pub bonding_curve_vault: Pubkey,
    pub mint: Pubkey,
    pub recipient_token_account: Pubkey,
    pub recipient: Pubkey,
    pub token_program: Pubkey,
    pub system_program: Pubkey,
    pub associated_token_program: Pubkey,
    pub rent: Pubkey,
}
impl From<CloseBondingCurveVaultAccounts<'_, '_>> for CloseBondingCurveVaultKeys {
    fn from(accounts: CloseBondingCurveVaultAccounts) -> Self {
        Self {
            config: *accounts.config.key,
            operator: *accounts.operator.key,
            vault_authority: *accounts.vault_authority.key,
            bonding_curve: *accounts.bonding_curve.key,
            bonding_curve_vault: *accounts.bonding_curve_vault.key,
            mint: *accounts.mint.key,
            recipient_token_account: *accounts.recipient_token_account.key,
            recipient: *accounts.recipient.key,
            token_program: *accounts.token_program.key,
            system_program: *accounts.system_program.key,
            associated_token_program: *accounts.associated_token_program.key,
            rent: *accounts.rent.key,
        }
    }
}
impl From<CloseBondingCurveVaultKeys>
for [AccountMeta; CLOSE_BONDING_CURVE_VAULT_IX_ACCOUNTS_LEN] {
    fn from(keys: CloseBondingCurveVaultKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.operator,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vault_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.bonding_curve,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.bonding_curve_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.recipient_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.recipient,
                is_signer: false,
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
                pubkey: keys.rent,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; CLOSE_BONDING_CURVE_VAULT_IX_ACCOUNTS_LEN]>
for CloseBondingCurveVaultKeys {
    fn from(pubkeys: [Pubkey; CLOSE_BONDING_CURVE_VAULT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            config: pubkeys[0],
            operator: pubkeys[1],
            vault_authority: pubkeys[2],
            bonding_curve: pubkeys[3],
            bonding_curve_vault: pubkeys[4],
            mint: pubkeys[5],
            recipient_token_account: pubkeys[6],
            recipient: pubkeys[7],
            token_program: pubkeys[8],
            system_program: pubkeys[9],
            associated_token_program: pubkeys[10],
            rent: pubkeys[11],
        }
    }
}
impl<'info> From<CloseBondingCurveVaultAccounts<'_, 'info>>
for [AccountInfo<'info>; CLOSE_BONDING_CURVE_VAULT_IX_ACCOUNTS_LEN] {
    fn from(accounts: CloseBondingCurveVaultAccounts<'_, 'info>) -> Self {
        [
            accounts.config.clone(),
            accounts.operator.clone(),
            accounts.vault_authority.clone(),
            accounts.bonding_curve.clone(),
            accounts.bonding_curve_vault.clone(),
            accounts.mint.clone(),
            accounts.recipient_token_account.clone(),
            accounts.recipient.clone(),
            accounts.token_program.clone(),
            accounts.system_program.clone(),
            accounts.associated_token_program.clone(),
            accounts.rent.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; CLOSE_BONDING_CURVE_VAULT_IX_ACCOUNTS_LEN]>
for CloseBondingCurveVaultAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; CLOSE_BONDING_CURVE_VAULT_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            config: &arr[0],
            operator: &arr[1],
            vault_authority: &arr[2],
            bonding_curve: &arr[3],
            bonding_curve_vault: &arr[4],
            mint: &arr[5],
            recipient_token_account: &arr[6],
            recipient: &arr[7],
            token_program: &arr[8],
            system_program: &arr[9],
            associated_token_program: &arr[10],
            rent: &arr[11],
        }
    }
}
pub const CLOSE_BONDING_CURVE_VAULT_IX_DISCM: [u8; 8usize] = [
    189, 71, 189, 239, 113, 66, 59, 189,
];
#[derive(Clone, Debug, PartialEq)]
pub struct CloseBondingCurveVaultIxData;
impl CloseBondingCurveVaultIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CLOSE_BONDING_CURVE_VAULT_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CLOSE_BONDING_CURVE_VAULT_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn close_bonding_curve_vault_ix_with_program_id(
    program_id: Pubkey,
    keys: CloseBondingCurveVaultKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CLOSE_BONDING_CURVE_VAULT_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: CloseBondingCurveVaultIxData.try_to_vec()?,
    })
}
pub fn close_bonding_curve_vault_ix(
    keys: CloseBondingCurveVaultKeys,
) -> std::io::Result<Instruction> {
    close_bonding_curve_vault_ix_with_program_id(BOOP_PROGRAM_ID, keys)
}
pub fn close_bonding_curve_vault_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CloseBondingCurveVaultAccounts<'_, '_>,
) -> ProgramResult {
    let keys: CloseBondingCurveVaultKeys = accounts.into();
    let ix = close_bonding_curve_vault_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn close_bonding_curve_vault_invoke(
    accounts: CloseBondingCurveVaultAccounts<'_, '_>,
) -> ProgramResult {
    close_bonding_curve_vault_invoke_with_program_id(BOOP_PROGRAM_ID, accounts)
}
pub fn close_bonding_curve_vault_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CloseBondingCurveVaultAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CloseBondingCurveVaultKeys = accounts.into();
    let ix = close_bonding_curve_vault_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn close_bonding_curve_vault_invoke_signed(
    accounts: CloseBondingCurveVaultAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    close_bonding_curve_vault_invoke_signed_with_program_id(
        BOOP_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn close_bonding_curve_vault_verify_account_keys(
    accounts: CloseBondingCurveVaultAccounts<'_, '_>,
    keys: CloseBondingCurveVaultKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.config.key, keys.config),
        (*accounts.operator.key, keys.operator),
        (*accounts.vault_authority.key, keys.vault_authority),
        (*accounts.bonding_curve.key, keys.bonding_curve),
        (*accounts.bonding_curve_vault.key, keys.bonding_curve_vault),
        (*accounts.mint.key, keys.mint),
        (*accounts.recipient_token_account.key, keys.recipient_token_account),
        (*accounts.recipient.key, keys.recipient),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.associated_token_program.key, keys.associated_token_program),
        (*accounts.rent.key, keys.rent),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn close_bonding_curve_vault_verify_writable_privileges<'me, 'info>(
    accounts: CloseBondingCurveVaultAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.operator,
        accounts.bonding_curve,
        accounts.bonding_curve_vault,
        accounts.recipient_token_account,
        accounts.recipient,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn close_bonding_curve_vault_verify_signer_privileges<'me, 'info>(
    accounts: CloseBondingCurveVaultAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.operator] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn close_bonding_curve_vault_verify_account_privileges<'me, 'info>(
    accounts: CloseBondingCurveVaultAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    close_bonding_curve_vault_verify_writable_privileges(accounts)?;
    close_bonding_curve_vault_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const COLLECT_METEORA_TRADING_FEES_IX_ACCOUNTS_LEN: usize = 18;
#[derive(Copy, Clone, Debug)]
pub struct CollectMeteoraTradingFeesAccounts<'me, 'info> {
    pub operator: &'me AccountInfo<'info>,
    pub protocol_fee_recipient: &'me AccountInfo<'info>,
    pub config: &'me AccountInfo<'info>,
    pub pool_authority: &'me AccountInfo<'info>,
    pub pool: &'me AccountInfo<'info>,
    pub position: &'me AccountInfo<'info>,
    pub token_a_account: &'me AccountInfo<'info>,
    pub token_b_account: &'me AccountInfo<'info>,
    pub token_a_vault: &'me AccountInfo<'info>,
    pub token_b_vault: &'me AccountInfo<'info>,
    pub token_a_mint: &'me AccountInfo<'info>,
    pub token_b_mint: &'me AccountInfo<'info>,
    pub position_nft_account: &'me AccountInfo<'info>,
    pub vault_authority: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub cp_amm_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CollectMeteoraTradingFeesKeys {
    pub operator: Pubkey,
    pub protocol_fee_recipient: Pubkey,
    pub config: Pubkey,
    pub pool_authority: Pubkey,
    pub pool: Pubkey,
    pub position: Pubkey,
    pub token_a_account: Pubkey,
    pub token_b_account: Pubkey,
    pub token_a_vault: Pubkey,
    pub token_b_vault: Pubkey,
    pub token_a_mint: Pubkey,
    pub token_b_mint: Pubkey,
    pub position_nft_account: Pubkey,
    pub vault_authority: Pubkey,
    pub token_program: Pubkey,
    pub associated_token_program: Pubkey,
    pub event_authority: Pubkey,
    pub cp_amm_program: Pubkey,
}
impl From<CollectMeteoraTradingFeesAccounts<'_, '_>> for CollectMeteoraTradingFeesKeys {
    fn from(accounts: CollectMeteoraTradingFeesAccounts) -> Self {
        Self {
            operator: *accounts.operator.key,
            protocol_fee_recipient: *accounts.protocol_fee_recipient.key,
            config: *accounts.config.key,
            pool_authority: *accounts.pool_authority.key,
            pool: *accounts.pool.key,
            position: *accounts.position.key,
            token_a_account: *accounts.token_a_account.key,
            token_b_account: *accounts.token_b_account.key,
            token_a_vault: *accounts.token_a_vault.key,
            token_b_vault: *accounts.token_b_vault.key,
            token_a_mint: *accounts.token_a_mint.key,
            token_b_mint: *accounts.token_b_mint.key,
            position_nft_account: *accounts.position_nft_account.key,
            vault_authority: *accounts.vault_authority.key,
            token_program: *accounts.token_program.key,
            associated_token_program: *accounts.associated_token_program.key,
            event_authority: *accounts.event_authority.key,
            cp_amm_program: *accounts.cp_amm_program.key,
        }
    }
}
impl From<CollectMeteoraTradingFeesKeys>
for [AccountMeta; COLLECT_METEORA_TRADING_FEES_IX_ACCOUNTS_LEN] {
    fn from(keys: CollectMeteoraTradingFeesKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.operator,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.protocol_fee_recipient,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.config,
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
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.position,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_a_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_b_account,
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
                pubkey: keys.position_nft_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vault_authority,
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
                pubkey: keys.cp_amm_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; COLLECT_METEORA_TRADING_FEES_IX_ACCOUNTS_LEN]>
for CollectMeteoraTradingFeesKeys {
    fn from(pubkeys: [Pubkey; COLLECT_METEORA_TRADING_FEES_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            operator: pubkeys[0],
            protocol_fee_recipient: pubkeys[1],
            config: pubkeys[2],
            pool_authority: pubkeys[3],
            pool: pubkeys[4],
            position: pubkeys[5],
            token_a_account: pubkeys[6],
            token_b_account: pubkeys[7],
            token_a_vault: pubkeys[8],
            token_b_vault: pubkeys[9],
            token_a_mint: pubkeys[10],
            token_b_mint: pubkeys[11],
            position_nft_account: pubkeys[12],
            vault_authority: pubkeys[13],
            token_program: pubkeys[14],
            associated_token_program: pubkeys[15],
            event_authority: pubkeys[16],
            cp_amm_program: pubkeys[17],
        }
    }
}
impl<'info> From<CollectMeteoraTradingFeesAccounts<'_, 'info>>
for [AccountInfo<'info>; COLLECT_METEORA_TRADING_FEES_IX_ACCOUNTS_LEN] {
    fn from(accounts: CollectMeteoraTradingFeesAccounts<'_, 'info>) -> Self {
        [
            accounts.operator.clone(),
            accounts.protocol_fee_recipient.clone(),
            accounts.config.clone(),
            accounts.pool_authority.clone(),
            accounts.pool.clone(),
            accounts.position.clone(),
            accounts.token_a_account.clone(),
            accounts.token_b_account.clone(),
            accounts.token_a_vault.clone(),
            accounts.token_b_vault.clone(),
            accounts.token_a_mint.clone(),
            accounts.token_b_mint.clone(),
            accounts.position_nft_account.clone(),
            accounts.vault_authority.clone(),
            accounts.token_program.clone(),
            accounts.associated_token_program.clone(),
            accounts.event_authority.clone(),
            accounts.cp_amm_program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; COLLECT_METEORA_TRADING_FEES_IX_ACCOUNTS_LEN]>
for CollectMeteoraTradingFeesAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; COLLECT_METEORA_TRADING_FEES_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            operator: &arr[0],
            protocol_fee_recipient: &arr[1],
            config: &arr[2],
            pool_authority: &arr[3],
            pool: &arr[4],
            position: &arr[5],
            token_a_account: &arr[6],
            token_b_account: &arr[7],
            token_a_vault: &arr[8],
            token_b_vault: &arr[9],
            token_a_mint: &arr[10],
            token_b_mint: &arr[11],
            position_nft_account: &arr[12],
            vault_authority: &arr[13],
            token_program: &arr[14],
            associated_token_program: &arr[15],
            event_authority: &arr[16],
            cp_amm_program: &arr[17],
        }
    }
}
pub const COLLECT_METEORA_TRADING_FEES_IX_DISCM: [u8; 8usize] = [
    249, 95, 126, 91, 81, 162, 83, 250,
];
#[derive(Clone, Debug, PartialEq)]
pub struct CollectMeteoraTradingFeesIxData;
impl CollectMeteoraTradingFeesIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != COLLECT_METEORA_TRADING_FEES_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&COLLECT_METEORA_TRADING_FEES_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn collect_meteora_trading_fees_ix_with_program_id(
    program_id: Pubkey,
    keys: CollectMeteoraTradingFeesKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; COLLECT_METEORA_TRADING_FEES_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: CollectMeteoraTradingFeesIxData.try_to_vec()?,
    })
}
pub fn collect_meteora_trading_fees_ix(
    keys: CollectMeteoraTradingFeesKeys,
) -> std::io::Result<Instruction> {
    collect_meteora_trading_fees_ix_with_program_id(BOOP_PROGRAM_ID, keys)
}
pub fn collect_meteora_trading_fees_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CollectMeteoraTradingFeesAccounts<'_, '_>,
) -> ProgramResult {
    let keys: CollectMeteoraTradingFeesKeys = accounts.into();
    let ix = collect_meteora_trading_fees_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn collect_meteora_trading_fees_invoke(
    accounts: CollectMeteoraTradingFeesAccounts<'_, '_>,
) -> ProgramResult {
    collect_meteora_trading_fees_invoke_with_program_id(BOOP_PROGRAM_ID, accounts)
}
pub fn collect_meteora_trading_fees_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CollectMeteoraTradingFeesAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CollectMeteoraTradingFeesKeys = accounts.into();
    let ix = collect_meteora_trading_fees_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn collect_meteora_trading_fees_invoke_signed(
    accounts: CollectMeteoraTradingFeesAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    collect_meteora_trading_fees_invoke_signed_with_program_id(
        BOOP_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn collect_meteora_trading_fees_verify_account_keys(
    accounts: CollectMeteoraTradingFeesAccounts<'_, '_>,
    keys: CollectMeteoraTradingFeesKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.operator.key, keys.operator),
        (*accounts.protocol_fee_recipient.key, keys.protocol_fee_recipient),
        (*accounts.config.key, keys.config),
        (*accounts.pool_authority.key, keys.pool_authority),
        (*accounts.pool.key, keys.pool),
        (*accounts.position.key, keys.position),
        (*accounts.token_a_account.key, keys.token_a_account),
        (*accounts.token_b_account.key, keys.token_b_account),
        (*accounts.token_a_vault.key, keys.token_a_vault),
        (*accounts.token_b_vault.key, keys.token_b_vault),
        (*accounts.token_a_mint.key, keys.token_a_mint),
        (*accounts.token_b_mint.key, keys.token_b_mint),
        (*accounts.position_nft_account.key, keys.position_nft_account),
        (*accounts.vault_authority.key, keys.vault_authority),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.associated_token_program.key, keys.associated_token_program),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.cp_amm_program.key, keys.cp_amm_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn collect_meteora_trading_fees_verify_writable_privileges<'me, 'info>(
    accounts: CollectMeteoraTradingFeesAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.operator,
        accounts.position,
        accounts.token_a_account,
        accounts.token_b_account,
        accounts.token_a_vault,
        accounts.token_b_vault,
        accounts.position_nft_account,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn collect_meteora_trading_fees_verify_signer_privileges<'me, 'info>(
    accounts: CollectMeteoraTradingFeesAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.operator] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn collect_meteora_trading_fees_verify_account_privileges<'me, 'info>(
    accounts: CollectMeteoraTradingFeesAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    collect_meteora_trading_fees_verify_writable_privileges(accounts)?;
    collect_meteora_trading_fees_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const COLLECT_METEORA_TRADING_FEES_V2_IX_ACCOUNTS_LEN: usize = 19;
#[derive(Copy, Clone, Debug)]
pub struct CollectMeteoraTradingFeesV2Accounts<'me, 'info> {
    pub operator: &'me AccountInfo<'info>,
    pub protocol_fee_recipient: &'me AccountInfo<'info>,
    pub config: &'me AccountInfo<'info>,
    pub pool_authority: &'me AccountInfo<'info>,
    pub pool: &'me AccountInfo<'info>,
    pub position: &'me AccountInfo<'info>,
    pub token_a_account: &'me AccountInfo<'info>,
    pub token_b_account: &'me AccountInfo<'info>,
    pub token_a_vault: &'me AccountInfo<'info>,
    pub token_b_vault: &'me AccountInfo<'info>,
    pub token_a_mint: &'me AccountInfo<'info>,
    pub token_b_mint: &'me AccountInfo<'info>,
    pub position_nft_account: &'me AccountInfo<'info>,
    pub vault_authority: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub cp_amm_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CollectMeteoraTradingFeesV2Keys {
    pub operator: Pubkey,
    pub protocol_fee_recipient: Pubkey,
    pub config: Pubkey,
    pub pool_authority: Pubkey,
    pub pool: Pubkey,
    pub position: Pubkey,
    pub token_a_account: Pubkey,
    pub token_b_account: Pubkey,
    pub token_a_vault: Pubkey,
    pub token_b_vault: Pubkey,
    pub token_a_mint: Pubkey,
    pub token_b_mint: Pubkey,
    pub position_nft_account: Pubkey,
    pub vault_authority: Pubkey,
    pub token_program: Pubkey,
    pub associated_token_program: Pubkey,
    pub event_authority: Pubkey,
    pub cp_amm_program: Pubkey,
    pub system_program: Pubkey,
}
impl From<CollectMeteoraTradingFeesV2Accounts<'_, '_>>
for CollectMeteoraTradingFeesV2Keys {
    fn from(accounts: CollectMeteoraTradingFeesV2Accounts) -> Self {
        Self {
            operator: *accounts.operator.key,
            protocol_fee_recipient: *accounts.protocol_fee_recipient.key,
            config: *accounts.config.key,
            pool_authority: *accounts.pool_authority.key,
            pool: *accounts.pool.key,
            position: *accounts.position.key,
            token_a_account: *accounts.token_a_account.key,
            token_b_account: *accounts.token_b_account.key,
            token_a_vault: *accounts.token_a_vault.key,
            token_b_vault: *accounts.token_b_vault.key,
            token_a_mint: *accounts.token_a_mint.key,
            token_b_mint: *accounts.token_b_mint.key,
            position_nft_account: *accounts.position_nft_account.key,
            vault_authority: *accounts.vault_authority.key,
            token_program: *accounts.token_program.key,
            associated_token_program: *accounts.associated_token_program.key,
            event_authority: *accounts.event_authority.key,
            cp_amm_program: *accounts.cp_amm_program.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<CollectMeteoraTradingFeesV2Keys>
for [AccountMeta; COLLECT_METEORA_TRADING_FEES_V2_IX_ACCOUNTS_LEN] {
    fn from(keys: CollectMeteoraTradingFeesV2Keys) -> Self {
        [
            AccountMeta {
                pubkey: keys.operator,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.protocol_fee_recipient,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.config,
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
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.position,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_a_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_b_account,
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
                pubkey: keys.position_nft_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vault_authority,
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
                pubkey: keys.cp_amm_program,
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
impl From<[Pubkey; COLLECT_METEORA_TRADING_FEES_V2_IX_ACCOUNTS_LEN]>
for CollectMeteoraTradingFeesV2Keys {
    fn from(pubkeys: [Pubkey; COLLECT_METEORA_TRADING_FEES_V2_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            operator: pubkeys[0],
            protocol_fee_recipient: pubkeys[1],
            config: pubkeys[2],
            pool_authority: pubkeys[3],
            pool: pubkeys[4],
            position: pubkeys[5],
            token_a_account: pubkeys[6],
            token_b_account: pubkeys[7],
            token_a_vault: pubkeys[8],
            token_b_vault: pubkeys[9],
            token_a_mint: pubkeys[10],
            token_b_mint: pubkeys[11],
            position_nft_account: pubkeys[12],
            vault_authority: pubkeys[13],
            token_program: pubkeys[14],
            associated_token_program: pubkeys[15],
            event_authority: pubkeys[16],
            cp_amm_program: pubkeys[17],
            system_program: pubkeys[18],
        }
    }
}
impl<'info> From<CollectMeteoraTradingFeesV2Accounts<'_, 'info>>
for [AccountInfo<'info>; COLLECT_METEORA_TRADING_FEES_V2_IX_ACCOUNTS_LEN] {
    fn from(accounts: CollectMeteoraTradingFeesV2Accounts<'_, 'info>) -> Self {
        [
            accounts.operator.clone(),
            accounts.protocol_fee_recipient.clone(),
            accounts.config.clone(),
            accounts.pool_authority.clone(),
            accounts.pool.clone(),
            accounts.position.clone(),
            accounts.token_a_account.clone(),
            accounts.token_b_account.clone(),
            accounts.token_a_vault.clone(),
            accounts.token_b_vault.clone(),
            accounts.token_a_mint.clone(),
            accounts.token_b_mint.clone(),
            accounts.position_nft_account.clone(),
            accounts.vault_authority.clone(),
            accounts.token_program.clone(),
            accounts.associated_token_program.clone(),
            accounts.event_authority.clone(),
            accounts.cp_amm_program.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; COLLECT_METEORA_TRADING_FEES_V2_IX_ACCOUNTS_LEN]>
for CollectMeteoraTradingFeesV2Accounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; COLLECT_METEORA_TRADING_FEES_V2_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            operator: &arr[0],
            protocol_fee_recipient: &arr[1],
            config: &arr[2],
            pool_authority: &arr[3],
            pool: &arr[4],
            position: &arr[5],
            token_a_account: &arr[6],
            token_b_account: &arr[7],
            token_a_vault: &arr[8],
            token_b_vault: &arr[9],
            token_a_mint: &arr[10],
            token_b_mint: &arr[11],
            position_nft_account: &arr[12],
            vault_authority: &arr[13],
            token_program: &arr[14],
            associated_token_program: &arr[15],
            event_authority: &arr[16],
            cp_amm_program: &arr[17],
            system_program: &arr[18],
        }
    }
}
pub const COLLECT_METEORA_TRADING_FEES_V2_IX_DISCM: [u8; 8usize] = [
    96, 39, 109, 46, 5, 161, 15, 57,
];
#[derive(Clone, Debug, PartialEq)]
pub struct CollectMeteoraTradingFeesV2IxData;
impl CollectMeteoraTradingFeesV2IxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != COLLECT_METEORA_TRADING_FEES_V2_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&COLLECT_METEORA_TRADING_FEES_V2_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn collect_meteora_trading_fees_v2_ix_with_program_id(
    program_id: Pubkey,
    keys: CollectMeteoraTradingFeesV2Keys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; COLLECT_METEORA_TRADING_FEES_V2_IX_ACCOUNTS_LEN] = keys
        .into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: CollectMeteoraTradingFeesV2IxData.try_to_vec()?,
    })
}
pub fn collect_meteora_trading_fees_v2_ix(
    keys: CollectMeteoraTradingFeesV2Keys,
) -> std::io::Result<Instruction> {
    collect_meteora_trading_fees_v2_ix_with_program_id(BOOP_PROGRAM_ID, keys)
}
pub fn collect_meteora_trading_fees_v2_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CollectMeteoraTradingFeesV2Accounts<'_, '_>,
) -> ProgramResult {
    let keys: CollectMeteoraTradingFeesV2Keys = accounts.into();
    let ix = collect_meteora_trading_fees_v2_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn collect_meteora_trading_fees_v2_invoke(
    accounts: CollectMeteoraTradingFeesV2Accounts<'_, '_>,
) -> ProgramResult {
    collect_meteora_trading_fees_v2_invoke_with_program_id(BOOP_PROGRAM_ID, accounts)
}
pub fn collect_meteora_trading_fees_v2_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CollectMeteoraTradingFeesV2Accounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CollectMeteoraTradingFeesV2Keys = accounts.into();
    let ix = collect_meteora_trading_fees_v2_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn collect_meteora_trading_fees_v2_invoke_signed(
    accounts: CollectMeteoraTradingFeesV2Accounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    collect_meteora_trading_fees_v2_invoke_signed_with_program_id(
        BOOP_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn collect_meteora_trading_fees_v2_verify_account_keys(
    accounts: CollectMeteoraTradingFeesV2Accounts<'_, '_>,
    keys: CollectMeteoraTradingFeesV2Keys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.operator.key, keys.operator),
        (*accounts.protocol_fee_recipient.key, keys.protocol_fee_recipient),
        (*accounts.config.key, keys.config),
        (*accounts.pool_authority.key, keys.pool_authority),
        (*accounts.pool.key, keys.pool),
        (*accounts.position.key, keys.position),
        (*accounts.token_a_account.key, keys.token_a_account),
        (*accounts.token_b_account.key, keys.token_b_account),
        (*accounts.token_a_vault.key, keys.token_a_vault),
        (*accounts.token_b_vault.key, keys.token_b_vault),
        (*accounts.token_a_mint.key, keys.token_a_mint),
        (*accounts.token_b_mint.key, keys.token_b_mint),
        (*accounts.position_nft_account.key, keys.position_nft_account),
        (*accounts.vault_authority.key, keys.vault_authority),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.associated_token_program.key, keys.associated_token_program),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.cp_amm_program.key, keys.cp_amm_program),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn collect_meteora_trading_fees_v2_verify_writable_privileges<'me, 'info>(
    accounts: CollectMeteoraTradingFeesV2Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.operator,
        accounts.position,
        accounts.token_a_account,
        accounts.token_b_account,
        accounts.token_a_vault,
        accounts.token_b_vault,
        accounts.position_nft_account,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn collect_meteora_trading_fees_v2_verify_signer_privileges<'me, 'info>(
    accounts: CollectMeteoraTradingFeesV2Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.operator] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn collect_meteora_trading_fees_v2_verify_account_privileges<'me, 'info>(
    accounts: CollectMeteoraTradingFeesV2Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    collect_meteora_trading_fees_v2_verify_writable_privileges(accounts)?;
    collect_meteora_trading_fees_v2_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const COLLECT_TRADING_FEES_IX_ACCOUNTS_LEN: usize = 24;
#[derive(Copy, Clone, Debug)]
pub struct CollectTradingFeesAccounts<'me, 'info> {
    pub operator: &'me AccountInfo<'info>,
    pub protocol_fee_recipient: &'me AccountInfo<'info>,
    pub config: &'me AccountInfo<'info>,
    pub lock_program: &'me AccountInfo<'info>,
    pub vault_authority: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub fee_nft_account: &'me AccountInfo<'info>,
    pub locked_liquidity: &'me AccountInfo<'info>,
    pub cpmm_program: &'me AccountInfo<'info>,
    pub cp_authority: &'me AccountInfo<'info>,
    pub pool_state: &'me AccountInfo<'info>,
    pub lp_mint: &'me AccountInfo<'info>,
    pub recipient_token_0_account: &'me AccountInfo<'info>,
    pub recipient_token_1_account: &'me AccountInfo<'info>,
    pub token_0_vault: &'me AccountInfo<'info>,
    pub token_1_vault: &'me AccountInfo<'info>,
    pub vault_0_mint: &'me AccountInfo<'info>,
    pub vault_1_mint: &'me AccountInfo<'info>,
    pub locked_lp_vault: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub token_program_2022: &'me AccountInfo<'info>,
    pub memo_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CollectTradingFeesKeys {
    pub operator: Pubkey,
    pub protocol_fee_recipient: Pubkey,
    pub config: Pubkey,
    pub lock_program: Pubkey,
    pub vault_authority: Pubkey,
    pub authority: Pubkey,
    pub fee_nft_account: Pubkey,
    pub locked_liquidity: Pubkey,
    pub cpmm_program: Pubkey,
    pub cp_authority: Pubkey,
    pub pool_state: Pubkey,
    pub lp_mint: Pubkey,
    pub recipient_token_0_account: Pubkey,
    pub recipient_token_1_account: Pubkey,
    pub token_0_vault: Pubkey,
    pub token_1_vault: Pubkey,
    pub vault_0_mint: Pubkey,
    pub vault_1_mint: Pubkey,
    pub locked_lp_vault: Pubkey,
    pub system_program: Pubkey,
    pub associated_token_program: Pubkey,
    pub token_program: Pubkey,
    pub token_program_2022: Pubkey,
    pub memo_program: Pubkey,
}
impl From<CollectTradingFeesAccounts<'_, '_>> for CollectTradingFeesKeys {
    fn from(accounts: CollectTradingFeesAccounts) -> Self {
        Self {
            operator: *accounts.operator.key,
            protocol_fee_recipient: *accounts.protocol_fee_recipient.key,
            config: *accounts.config.key,
            lock_program: *accounts.lock_program.key,
            vault_authority: *accounts.vault_authority.key,
            authority: *accounts.authority.key,
            fee_nft_account: *accounts.fee_nft_account.key,
            locked_liquidity: *accounts.locked_liquidity.key,
            cpmm_program: *accounts.cpmm_program.key,
            cp_authority: *accounts.cp_authority.key,
            pool_state: *accounts.pool_state.key,
            lp_mint: *accounts.lp_mint.key,
            recipient_token_0_account: *accounts.recipient_token_0_account.key,
            recipient_token_1_account: *accounts.recipient_token_1_account.key,
            token_0_vault: *accounts.token_0_vault.key,
            token_1_vault: *accounts.token_1_vault.key,
            vault_0_mint: *accounts.vault_0_mint.key,
            vault_1_mint: *accounts.vault_1_mint.key,
            locked_lp_vault: *accounts.locked_lp_vault.key,
            system_program: *accounts.system_program.key,
            associated_token_program: *accounts.associated_token_program.key,
            token_program: *accounts.token_program.key,
            token_program_2022: *accounts.token_program_2022.key,
            memo_program: *accounts.memo_program.key,
        }
    }
}
impl From<CollectTradingFeesKeys>
for [AccountMeta; COLLECT_TRADING_FEES_IX_ACCOUNTS_LEN] {
    fn from(keys: CollectTradingFeesKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.operator,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.protocol_fee_recipient,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.lock_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.authority,
                is_signer: false,
                is_writable: false,
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
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault_1_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.locked_lp_vault,
                is_signer: false,
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
impl From<[Pubkey; COLLECT_TRADING_FEES_IX_ACCOUNTS_LEN]> for CollectTradingFeesKeys {
    fn from(pubkeys: [Pubkey; COLLECT_TRADING_FEES_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            operator: pubkeys[0],
            protocol_fee_recipient: pubkeys[1],
            config: pubkeys[2],
            lock_program: pubkeys[3],
            vault_authority: pubkeys[4],
            authority: pubkeys[5],
            fee_nft_account: pubkeys[6],
            locked_liquidity: pubkeys[7],
            cpmm_program: pubkeys[8],
            cp_authority: pubkeys[9],
            pool_state: pubkeys[10],
            lp_mint: pubkeys[11],
            recipient_token_0_account: pubkeys[12],
            recipient_token_1_account: pubkeys[13],
            token_0_vault: pubkeys[14],
            token_1_vault: pubkeys[15],
            vault_0_mint: pubkeys[16],
            vault_1_mint: pubkeys[17],
            locked_lp_vault: pubkeys[18],
            system_program: pubkeys[19],
            associated_token_program: pubkeys[20],
            token_program: pubkeys[21],
            token_program_2022: pubkeys[22],
            memo_program: pubkeys[23],
        }
    }
}
impl<'info> From<CollectTradingFeesAccounts<'_, 'info>>
for [AccountInfo<'info>; COLLECT_TRADING_FEES_IX_ACCOUNTS_LEN] {
    fn from(accounts: CollectTradingFeesAccounts<'_, 'info>) -> Self {
        [
            accounts.operator.clone(),
            accounts.protocol_fee_recipient.clone(),
            accounts.config.clone(),
            accounts.lock_program.clone(),
            accounts.vault_authority.clone(),
            accounts.authority.clone(),
            accounts.fee_nft_account.clone(),
            accounts.locked_liquidity.clone(),
            accounts.cpmm_program.clone(),
            accounts.cp_authority.clone(),
            accounts.pool_state.clone(),
            accounts.lp_mint.clone(),
            accounts.recipient_token_0_account.clone(),
            accounts.recipient_token_1_account.clone(),
            accounts.token_0_vault.clone(),
            accounts.token_1_vault.clone(),
            accounts.vault_0_mint.clone(),
            accounts.vault_1_mint.clone(),
            accounts.locked_lp_vault.clone(),
            accounts.system_program.clone(),
            accounts.associated_token_program.clone(),
            accounts.token_program.clone(),
            accounts.token_program_2022.clone(),
            accounts.memo_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; COLLECT_TRADING_FEES_IX_ACCOUNTS_LEN]>
for CollectTradingFeesAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; COLLECT_TRADING_FEES_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            operator: &arr[0],
            protocol_fee_recipient: &arr[1],
            config: &arr[2],
            lock_program: &arr[3],
            vault_authority: &arr[4],
            authority: &arr[5],
            fee_nft_account: &arr[6],
            locked_liquidity: &arr[7],
            cpmm_program: &arr[8],
            cp_authority: &arr[9],
            pool_state: &arr[10],
            lp_mint: &arr[11],
            recipient_token_0_account: &arr[12],
            recipient_token_1_account: &arr[13],
            token_0_vault: &arr[14],
            token_1_vault: &arr[15],
            vault_0_mint: &arr[16],
            vault_1_mint: &arr[17],
            locked_lp_vault: &arr[18],
            system_program: &arr[19],
            associated_token_program: &arr[20],
            token_program: &arr[21],
            token_program_2022: &arr[22],
            memo_program: &arr[23],
        }
    }
}
pub const COLLECT_TRADING_FEES_IX_DISCM: [u8; 8usize] = [
    189, 38, 205, 234, 81, 77, 25, 1,
];
#[derive(Clone, Debug, PartialEq)]
pub struct CollectTradingFeesIxData;
impl CollectTradingFeesIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != COLLECT_TRADING_FEES_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&COLLECT_TRADING_FEES_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn collect_trading_fees_ix_with_program_id(
    program_id: Pubkey,
    keys: CollectTradingFeesKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; COLLECT_TRADING_FEES_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: CollectTradingFeesIxData.try_to_vec()?,
    })
}
pub fn collect_trading_fees_ix(
    keys: CollectTradingFeesKeys,
) -> std::io::Result<Instruction> {
    collect_trading_fees_ix_with_program_id(BOOP_PROGRAM_ID, keys)
}
pub fn collect_trading_fees_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CollectTradingFeesAccounts<'_, '_>,
) -> ProgramResult {
    let keys: CollectTradingFeesKeys = accounts.into();
    let ix = collect_trading_fees_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn collect_trading_fees_invoke(
    accounts: CollectTradingFeesAccounts<'_, '_>,
) -> ProgramResult {
    collect_trading_fees_invoke_with_program_id(BOOP_PROGRAM_ID, accounts)
}
pub fn collect_trading_fees_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CollectTradingFeesAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CollectTradingFeesKeys = accounts.into();
    let ix = collect_trading_fees_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn collect_trading_fees_invoke_signed(
    accounts: CollectTradingFeesAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    collect_trading_fees_invoke_signed_with_program_id(BOOP_PROGRAM_ID, accounts, seeds)
}
pub fn collect_trading_fees_verify_account_keys(
    accounts: CollectTradingFeesAccounts<'_, '_>,
    keys: CollectTradingFeesKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.operator.key, keys.operator),
        (*accounts.protocol_fee_recipient.key, keys.protocol_fee_recipient),
        (*accounts.config.key, keys.config),
        (*accounts.lock_program.key, keys.lock_program),
        (*accounts.vault_authority.key, keys.vault_authority),
        (*accounts.authority.key, keys.authority),
        (*accounts.fee_nft_account.key, keys.fee_nft_account),
        (*accounts.locked_liquidity.key, keys.locked_liquidity),
        (*accounts.cpmm_program.key, keys.cpmm_program),
        (*accounts.cp_authority.key, keys.cp_authority),
        (*accounts.pool_state.key, keys.pool_state),
        (*accounts.lp_mint.key, keys.lp_mint),
        (*accounts.recipient_token_0_account.key, keys.recipient_token_0_account),
        (*accounts.recipient_token_1_account.key, keys.recipient_token_1_account),
        (*accounts.token_0_vault.key, keys.token_0_vault),
        (*accounts.token_1_vault.key, keys.token_1_vault),
        (*accounts.vault_0_mint.key, keys.vault_0_mint),
        (*accounts.vault_1_mint.key, keys.vault_1_mint),
        (*accounts.locked_lp_vault.key, keys.locked_lp_vault),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.associated_token_program.key, keys.associated_token_program),
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
pub fn collect_trading_fees_verify_writable_privileges<'me, 'info>(
    accounts: CollectTradingFeesAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.operator,
        accounts.locked_liquidity,
        accounts.pool_state,
        accounts.lp_mint,
        accounts.recipient_token_0_account,
        accounts.recipient_token_1_account,
        accounts.token_0_vault,
        accounts.token_1_vault,
        accounts.locked_lp_vault,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn collect_trading_fees_verify_signer_privileges<'me, 'info>(
    accounts: CollectTradingFeesAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.operator] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn collect_trading_fees_verify_account_privileges<'me, 'info>(
    accounts: CollectTradingFeesAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    collect_trading_fees_verify_writable_privileges(accounts)?;
    collect_trading_fees_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const COLLECT_TRADING_FEES_V2_IX_ACCOUNTS_LEN: usize = 24;
#[derive(Copy, Clone, Debug)]
pub struct CollectTradingFeesV2Accounts<'me, 'info> {
    pub operator: &'me AccountInfo<'info>,
    pub protocol_fee_recipient: &'me AccountInfo<'info>,
    pub config: &'me AccountInfo<'info>,
    pub lock_program: &'me AccountInfo<'info>,
    pub vault_authority: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub fee_nft_account: &'me AccountInfo<'info>,
    pub locked_liquidity: &'me AccountInfo<'info>,
    pub cpmm_program: &'me AccountInfo<'info>,
    pub cp_authority: &'me AccountInfo<'info>,
    pub pool_state: &'me AccountInfo<'info>,
    pub lp_mint: &'me AccountInfo<'info>,
    pub recipient_token_0_account: &'me AccountInfo<'info>,
    pub recipient_token_1_account: &'me AccountInfo<'info>,
    pub token_0_vault: &'me AccountInfo<'info>,
    pub token_1_vault: &'me AccountInfo<'info>,
    pub vault_0_mint: &'me AccountInfo<'info>,
    pub vault_1_mint: &'me AccountInfo<'info>,
    pub locked_lp_vault: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub token_program_2022: &'me AccountInfo<'info>,
    pub memo_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CollectTradingFeesV2Keys {
    pub operator: Pubkey,
    pub protocol_fee_recipient: Pubkey,
    pub config: Pubkey,
    pub lock_program: Pubkey,
    pub vault_authority: Pubkey,
    pub authority: Pubkey,
    pub fee_nft_account: Pubkey,
    pub locked_liquidity: Pubkey,
    pub cpmm_program: Pubkey,
    pub cp_authority: Pubkey,
    pub pool_state: Pubkey,
    pub lp_mint: Pubkey,
    pub recipient_token_0_account: Pubkey,
    pub recipient_token_1_account: Pubkey,
    pub token_0_vault: Pubkey,
    pub token_1_vault: Pubkey,
    pub vault_0_mint: Pubkey,
    pub vault_1_mint: Pubkey,
    pub locked_lp_vault: Pubkey,
    pub system_program: Pubkey,
    pub associated_token_program: Pubkey,
    pub token_program: Pubkey,
    pub token_program_2022: Pubkey,
    pub memo_program: Pubkey,
}
impl From<CollectTradingFeesV2Accounts<'_, '_>> for CollectTradingFeesV2Keys {
    fn from(accounts: CollectTradingFeesV2Accounts) -> Self {
        Self {
            operator: *accounts.operator.key,
            protocol_fee_recipient: *accounts.protocol_fee_recipient.key,
            config: *accounts.config.key,
            lock_program: *accounts.lock_program.key,
            vault_authority: *accounts.vault_authority.key,
            authority: *accounts.authority.key,
            fee_nft_account: *accounts.fee_nft_account.key,
            locked_liquidity: *accounts.locked_liquidity.key,
            cpmm_program: *accounts.cpmm_program.key,
            cp_authority: *accounts.cp_authority.key,
            pool_state: *accounts.pool_state.key,
            lp_mint: *accounts.lp_mint.key,
            recipient_token_0_account: *accounts.recipient_token_0_account.key,
            recipient_token_1_account: *accounts.recipient_token_1_account.key,
            token_0_vault: *accounts.token_0_vault.key,
            token_1_vault: *accounts.token_1_vault.key,
            vault_0_mint: *accounts.vault_0_mint.key,
            vault_1_mint: *accounts.vault_1_mint.key,
            locked_lp_vault: *accounts.locked_lp_vault.key,
            system_program: *accounts.system_program.key,
            associated_token_program: *accounts.associated_token_program.key,
            token_program: *accounts.token_program.key,
            token_program_2022: *accounts.token_program_2022.key,
            memo_program: *accounts.memo_program.key,
        }
    }
}
impl From<CollectTradingFeesV2Keys>
for [AccountMeta; COLLECT_TRADING_FEES_V2_IX_ACCOUNTS_LEN] {
    fn from(keys: CollectTradingFeesV2Keys) -> Self {
        [
            AccountMeta {
                pubkey: keys.operator,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.protocol_fee_recipient,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.lock_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.authority,
                is_signer: false,
                is_writable: false,
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
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault_1_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.locked_lp_vault,
                is_signer: false,
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
impl From<[Pubkey; COLLECT_TRADING_FEES_V2_IX_ACCOUNTS_LEN]>
for CollectTradingFeesV2Keys {
    fn from(pubkeys: [Pubkey; COLLECT_TRADING_FEES_V2_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            operator: pubkeys[0],
            protocol_fee_recipient: pubkeys[1],
            config: pubkeys[2],
            lock_program: pubkeys[3],
            vault_authority: pubkeys[4],
            authority: pubkeys[5],
            fee_nft_account: pubkeys[6],
            locked_liquidity: pubkeys[7],
            cpmm_program: pubkeys[8],
            cp_authority: pubkeys[9],
            pool_state: pubkeys[10],
            lp_mint: pubkeys[11],
            recipient_token_0_account: pubkeys[12],
            recipient_token_1_account: pubkeys[13],
            token_0_vault: pubkeys[14],
            token_1_vault: pubkeys[15],
            vault_0_mint: pubkeys[16],
            vault_1_mint: pubkeys[17],
            locked_lp_vault: pubkeys[18],
            system_program: pubkeys[19],
            associated_token_program: pubkeys[20],
            token_program: pubkeys[21],
            token_program_2022: pubkeys[22],
            memo_program: pubkeys[23],
        }
    }
}
impl<'info> From<CollectTradingFeesV2Accounts<'_, 'info>>
for [AccountInfo<'info>; COLLECT_TRADING_FEES_V2_IX_ACCOUNTS_LEN] {
    fn from(accounts: CollectTradingFeesV2Accounts<'_, 'info>) -> Self {
        [
            accounts.operator.clone(),
            accounts.protocol_fee_recipient.clone(),
            accounts.config.clone(),
            accounts.lock_program.clone(),
            accounts.vault_authority.clone(),
            accounts.authority.clone(),
            accounts.fee_nft_account.clone(),
            accounts.locked_liquidity.clone(),
            accounts.cpmm_program.clone(),
            accounts.cp_authority.clone(),
            accounts.pool_state.clone(),
            accounts.lp_mint.clone(),
            accounts.recipient_token_0_account.clone(),
            accounts.recipient_token_1_account.clone(),
            accounts.token_0_vault.clone(),
            accounts.token_1_vault.clone(),
            accounts.vault_0_mint.clone(),
            accounts.vault_1_mint.clone(),
            accounts.locked_lp_vault.clone(),
            accounts.system_program.clone(),
            accounts.associated_token_program.clone(),
            accounts.token_program.clone(),
            accounts.token_program_2022.clone(),
            accounts.memo_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; COLLECT_TRADING_FEES_V2_IX_ACCOUNTS_LEN]>
for CollectTradingFeesV2Accounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; COLLECT_TRADING_FEES_V2_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            operator: &arr[0],
            protocol_fee_recipient: &arr[1],
            config: &arr[2],
            lock_program: &arr[3],
            vault_authority: &arr[4],
            authority: &arr[5],
            fee_nft_account: &arr[6],
            locked_liquidity: &arr[7],
            cpmm_program: &arr[8],
            cp_authority: &arr[9],
            pool_state: &arr[10],
            lp_mint: &arr[11],
            recipient_token_0_account: &arr[12],
            recipient_token_1_account: &arr[13],
            token_0_vault: &arr[14],
            token_1_vault: &arr[15],
            vault_0_mint: &arr[16],
            vault_1_mint: &arr[17],
            locked_lp_vault: &arr[18],
            system_program: &arr[19],
            associated_token_program: &arr[20],
            token_program: &arr[21],
            token_program_2022: &arr[22],
            memo_program: &arr[23],
        }
    }
}
pub const COLLECT_TRADING_FEES_V2_IX_DISCM: [u8; 8usize] = [
    180, 138, 160, 155, 243, 88, 168, 7,
];
#[derive(Clone, Debug, PartialEq)]
pub struct CollectTradingFeesV2IxData;
impl CollectTradingFeesV2IxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != COLLECT_TRADING_FEES_V2_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&COLLECT_TRADING_FEES_V2_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn collect_trading_fees_v2_ix_with_program_id(
    program_id: Pubkey,
    keys: CollectTradingFeesV2Keys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; COLLECT_TRADING_FEES_V2_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: CollectTradingFeesV2IxData.try_to_vec()?,
    })
}
pub fn collect_trading_fees_v2_ix(
    keys: CollectTradingFeesV2Keys,
) -> std::io::Result<Instruction> {
    collect_trading_fees_v2_ix_with_program_id(BOOP_PROGRAM_ID, keys)
}
pub fn collect_trading_fees_v2_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CollectTradingFeesV2Accounts<'_, '_>,
) -> ProgramResult {
    let keys: CollectTradingFeesV2Keys = accounts.into();
    let ix = collect_trading_fees_v2_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn collect_trading_fees_v2_invoke(
    accounts: CollectTradingFeesV2Accounts<'_, '_>,
) -> ProgramResult {
    collect_trading_fees_v2_invoke_with_program_id(BOOP_PROGRAM_ID, accounts)
}
pub fn collect_trading_fees_v2_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CollectTradingFeesV2Accounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CollectTradingFeesV2Keys = accounts.into();
    let ix = collect_trading_fees_v2_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn collect_trading_fees_v2_invoke_signed(
    accounts: CollectTradingFeesV2Accounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    collect_trading_fees_v2_invoke_signed_with_program_id(
        BOOP_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn collect_trading_fees_v2_verify_account_keys(
    accounts: CollectTradingFeesV2Accounts<'_, '_>,
    keys: CollectTradingFeesV2Keys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.operator.key, keys.operator),
        (*accounts.protocol_fee_recipient.key, keys.protocol_fee_recipient),
        (*accounts.config.key, keys.config),
        (*accounts.lock_program.key, keys.lock_program),
        (*accounts.vault_authority.key, keys.vault_authority),
        (*accounts.authority.key, keys.authority),
        (*accounts.fee_nft_account.key, keys.fee_nft_account),
        (*accounts.locked_liquidity.key, keys.locked_liquidity),
        (*accounts.cpmm_program.key, keys.cpmm_program),
        (*accounts.cp_authority.key, keys.cp_authority),
        (*accounts.pool_state.key, keys.pool_state),
        (*accounts.lp_mint.key, keys.lp_mint),
        (*accounts.recipient_token_0_account.key, keys.recipient_token_0_account),
        (*accounts.recipient_token_1_account.key, keys.recipient_token_1_account),
        (*accounts.token_0_vault.key, keys.token_0_vault),
        (*accounts.token_1_vault.key, keys.token_1_vault),
        (*accounts.vault_0_mint.key, keys.vault_0_mint),
        (*accounts.vault_1_mint.key, keys.vault_1_mint),
        (*accounts.locked_lp_vault.key, keys.locked_lp_vault),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.associated_token_program.key, keys.associated_token_program),
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
pub fn collect_trading_fees_v2_verify_writable_privileges<'me, 'info>(
    accounts: CollectTradingFeesV2Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.operator,
        accounts.locked_liquidity,
        accounts.pool_state,
        accounts.lp_mint,
        accounts.recipient_token_0_account,
        accounts.recipient_token_1_account,
        accounts.token_0_vault,
        accounts.token_1_vault,
        accounts.locked_lp_vault,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn collect_trading_fees_v2_verify_signer_privileges<'me, 'info>(
    accounts: CollectTradingFeesV2Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.operator] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn collect_trading_fees_v2_verify_account_privileges<'me, 'info>(
    accounts: CollectTradingFeesV2Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    collect_trading_fees_v2_verify_writable_privileges(accounts)?;
    collect_trading_fees_v2_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const COMPLETE_AUTHORITY_TRANSFER_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct CompleteAuthorityTransferAccounts<'me, 'info> {
    pub pending_authority: &'me AccountInfo<'info>,
    pub config: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CompleteAuthorityTransferKeys {
    pub pending_authority: Pubkey,
    pub config: Pubkey,
    pub system_program: Pubkey,
}
impl From<CompleteAuthorityTransferAccounts<'_, '_>> for CompleteAuthorityTransferKeys {
    fn from(accounts: CompleteAuthorityTransferAccounts) -> Self {
        Self {
            pending_authority: *accounts.pending_authority.key,
            config: *accounts.config.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<CompleteAuthorityTransferKeys>
for [AccountMeta; COMPLETE_AUTHORITY_TRANSFER_IX_ACCOUNTS_LEN] {
    fn from(keys: CompleteAuthorityTransferKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.pending_authority,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.config,
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
impl From<[Pubkey; COMPLETE_AUTHORITY_TRANSFER_IX_ACCOUNTS_LEN]>
for CompleteAuthorityTransferKeys {
    fn from(pubkeys: [Pubkey; COMPLETE_AUTHORITY_TRANSFER_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            pending_authority: pubkeys[0],
            config: pubkeys[1],
            system_program: pubkeys[2],
        }
    }
}
impl<'info> From<CompleteAuthorityTransferAccounts<'_, 'info>>
for [AccountInfo<'info>; COMPLETE_AUTHORITY_TRANSFER_IX_ACCOUNTS_LEN] {
    fn from(accounts: CompleteAuthorityTransferAccounts<'_, 'info>) -> Self {
        [
            accounts.pending_authority.clone(),
            accounts.config.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; COMPLETE_AUTHORITY_TRANSFER_IX_ACCOUNTS_LEN]>
for CompleteAuthorityTransferAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; COMPLETE_AUTHORITY_TRANSFER_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            pending_authority: &arr[0],
            config: &arr[1],
            system_program: &arr[2],
        }
    }
}
pub const COMPLETE_AUTHORITY_TRANSFER_IX_DISCM: [u8; 8usize] = [
    81, 233, 91, 132, 175, 31, 151, 141,
];
#[derive(Clone, Debug, PartialEq)]
pub struct CompleteAuthorityTransferIxData;
impl CompleteAuthorityTransferIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != COMPLETE_AUTHORITY_TRANSFER_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&COMPLETE_AUTHORITY_TRANSFER_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn complete_authority_transfer_ix_with_program_id(
    program_id: Pubkey,
    keys: CompleteAuthorityTransferKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; COMPLETE_AUTHORITY_TRANSFER_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: CompleteAuthorityTransferIxData.try_to_vec()?,
    })
}
pub fn complete_authority_transfer_ix(
    keys: CompleteAuthorityTransferKeys,
) -> std::io::Result<Instruction> {
    complete_authority_transfer_ix_with_program_id(BOOP_PROGRAM_ID, keys)
}
pub fn complete_authority_transfer_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CompleteAuthorityTransferAccounts<'_, '_>,
) -> ProgramResult {
    let keys: CompleteAuthorityTransferKeys = accounts.into();
    let ix = complete_authority_transfer_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn complete_authority_transfer_invoke(
    accounts: CompleteAuthorityTransferAccounts<'_, '_>,
) -> ProgramResult {
    complete_authority_transfer_invoke_with_program_id(BOOP_PROGRAM_ID, accounts)
}
pub fn complete_authority_transfer_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CompleteAuthorityTransferAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CompleteAuthorityTransferKeys = accounts.into();
    let ix = complete_authority_transfer_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn complete_authority_transfer_invoke_signed(
    accounts: CompleteAuthorityTransferAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    complete_authority_transfer_invoke_signed_with_program_id(
        BOOP_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn complete_authority_transfer_verify_account_keys(
    accounts: CompleteAuthorityTransferAccounts<'_, '_>,
    keys: CompleteAuthorityTransferKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.pending_authority.key, keys.pending_authority),
        (*accounts.config.key, keys.config),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn complete_authority_transfer_verify_writable_privileges<'me, 'info>(
    accounts: CompleteAuthorityTransferAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.config] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn complete_authority_transfer_verify_signer_privileges<'me, 'info>(
    accounts: CompleteAuthorityTransferAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.pending_authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn complete_authority_transfer_verify_account_privileges<'me, 'info>(
    accounts: CompleteAuthorityTransferAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    complete_authority_transfer_verify_writable_privileges(accounts)?;
    complete_authority_transfer_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CREATE_METEORA_POOL_IX_ACCOUNTS_LEN: usize = 21;
#[derive(Copy, Clone, Debug)]
pub struct CreateMeteoraPoolAccounts<'me, 'info> {
    pub operator: &'me AccountInfo<'info>,
    pub config: &'me AccountInfo<'info>,
    pub vault_authority: &'me AccountInfo<'info>,
    pub cp_amm_config: &'me AccountInfo<'info>,
    pub pool_authority: &'me AccountInfo<'info>,
    pub pool: &'me AccountInfo<'info>,
    pub position: &'me AccountInfo<'info>,
    pub position_nft_mint: &'me AccountInfo<'info>,
    pub position_nft_account: &'me AccountInfo<'info>,
    pub token_a_mint: &'me AccountInfo<'info>,
    pub token_b_mint: &'me AccountInfo<'info>,
    pub token_a_vault: &'me AccountInfo<'info>,
    pub token_b_vault: &'me AccountInfo<'info>,
    pub bonding_curve: &'me AccountInfo<'info>,
    pub bonding_curve_vault: &'me AccountInfo<'info>,
    pub bonding_curve_wsol_vault: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub token_2022_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub cp_amm_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CreateMeteoraPoolKeys {
    pub operator: Pubkey,
    pub config: Pubkey,
    pub vault_authority: Pubkey,
    pub cp_amm_config: Pubkey,
    pub pool_authority: Pubkey,
    pub pool: Pubkey,
    pub position: Pubkey,
    pub position_nft_mint: Pubkey,
    pub position_nft_account: Pubkey,
    pub token_a_mint: Pubkey,
    pub token_b_mint: Pubkey,
    pub token_a_vault: Pubkey,
    pub token_b_vault: Pubkey,
    pub bonding_curve: Pubkey,
    pub bonding_curve_vault: Pubkey,
    pub bonding_curve_wsol_vault: Pubkey,
    pub token_program: Pubkey,
    pub token_2022_program: Pubkey,
    pub system_program: Pubkey,
    pub event_authority: Pubkey,
    pub cp_amm_program: Pubkey,
}
impl From<CreateMeteoraPoolAccounts<'_, '_>> for CreateMeteoraPoolKeys {
    fn from(accounts: CreateMeteoraPoolAccounts) -> Self {
        Self {
            operator: *accounts.operator.key,
            config: *accounts.config.key,
            vault_authority: *accounts.vault_authority.key,
            cp_amm_config: *accounts.cp_amm_config.key,
            pool_authority: *accounts.pool_authority.key,
            pool: *accounts.pool.key,
            position: *accounts.position.key,
            position_nft_mint: *accounts.position_nft_mint.key,
            position_nft_account: *accounts.position_nft_account.key,
            token_a_mint: *accounts.token_a_mint.key,
            token_b_mint: *accounts.token_b_mint.key,
            token_a_vault: *accounts.token_a_vault.key,
            token_b_vault: *accounts.token_b_vault.key,
            bonding_curve: *accounts.bonding_curve.key,
            bonding_curve_vault: *accounts.bonding_curve_vault.key,
            bonding_curve_wsol_vault: *accounts.bonding_curve_wsol_vault.key,
            token_program: *accounts.token_program.key,
            token_2022_program: *accounts.token_2022_program.key,
            system_program: *accounts.system_program.key,
            event_authority: *accounts.event_authority.key,
            cp_amm_program: *accounts.cp_amm_program.key,
        }
    }
}
impl From<CreateMeteoraPoolKeys> for [AccountMeta; CREATE_METEORA_POOL_IX_ACCOUNTS_LEN] {
    fn from(keys: CreateMeteoraPoolKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.operator,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault_authority,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.cp_amm_config,
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
                pubkey: keys.position,
                is_signer: false,
                is_writable: true,
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
                pubkey: keys.bonding_curve,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.bonding_curve_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.bonding_curve_wsol_vault,
                is_signer: false,
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
                pubkey: keys.cp_amm_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; CREATE_METEORA_POOL_IX_ACCOUNTS_LEN]> for CreateMeteoraPoolKeys {
    fn from(pubkeys: [Pubkey; CREATE_METEORA_POOL_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            operator: pubkeys[0],
            config: pubkeys[1],
            vault_authority: pubkeys[2],
            cp_amm_config: pubkeys[3],
            pool_authority: pubkeys[4],
            pool: pubkeys[5],
            position: pubkeys[6],
            position_nft_mint: pubkeys[7],
            position_nft_account: pubkeys[8],
            token_a_mint: pubkeys[9],
            token_b_mint: pubkeys[10],
            token_a_vault: pubkeys[11],
            token_b_vault: pubkeys[12],
            bonding_curve: pubkeys[13],
            bonding_curve_vault: pubkeys[14],
            bonding_curve_wsol_vault: pubkeys[15],
            token_program: pubkeys[16],
            token_2022_program: pubkeys[17],
            system_program: pubkeys[18],
            event_authority: pubkeys[19],
            cp_amm_program: pubkeys[20],
        }
    }
}
impl<'info> From<CreateMeteoraPoolAccounts<'_, 'info>>
for [AccountInfo<'info>; CREATE_METEORA_POOL_IX_ACCOUNTS_LEN] {
    fn from(accounts: CreateMeteoraPoolAccounts<'_, 'info>) -> Self {
        [
            accounts.operator.clone(),
            accounts.config.clone(),
            accounts.vault_authority.clone(),
            accounts.cp_amm_config.clone(),
            accounts.pool_authority.clone(),
            accounts.pool.clone(),
            accounts.position.clone(),
            accounts.position_nft_mint.clone(),
            accounts.position_nft_account.clone(),
            accounts.token_a_mint.clone(),
            accounts.token_b_mint.clone(),
            accounts.token_a_vault.clone(),
            accounts.token_b_vault.clone(),
            accounts.bonding_curve.clone(),
            accounts.bonding_curve_vault.clone(),
            accounts.bonding_curve_wsol_vault.clone(),
            accounts.token_program.clone(),
            accounts.token_2022_program.clone(),
            accounts.system_program.clone(),
            accounts.event_authority.clone(),
            accounts.cp_amm_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CREATE_METEORA_POOL_IX_ACCOUNTS_LEN]>
for CreateMeteoraPoolAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; CREATE_METEORA_POOL_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            operator: &arr[0],
            config: &arr[1],
            vault_authority: &arr[2],
            cp_amm_config: &arr[3],
            pool_authority: &arr[4],
            pool: &arr[5],
            position: &arr[6],
            position_nft_mint: &arr[7],
            position_nft_account: &arr[8],
            token_a_mint: &arr[9],
            token_b_mint: &arr[10],
            token_a_vault: &arr[11],
            token_b_vault: &arr[12],
            bonding_curve: &arr[13],
            bonding_curve_vault: &arr[14],
            bonding_curve_wsol_vault: &arr[15],
            token_program: &arr[16],
            token_2022_program: &arr[17],
            system_program: &arr[18],
            event_authority: &arr[19],
            cp_amm_program: &arr[20],
        }
    }
}
pub const CREATE_METEORA_POOL_IX_DISCM: [u8; 8usize] = [
    246, 254, 33, 37, 225, 176, 41, 232,
];
#[derive(Clone, Debug, PartialEq)]
pub struct CreateMeteoraPoolIxData;
impl CreateMeteoraPoolIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CREATE_METEORA_POOL_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CREATE_METEORA_POOL_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn create_meteora_pool_ix_with_program_id(
    program_id: Pubkey,
    keys: CreateMeteoraPoolKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CREATE_METEORA_POOL_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: CreateMeteoraPoolIxData.try_to_vec()?,
    })
}
pub fn create_meteora_pool_ix(
    keys: CreateMeteoraPoolKeys,
) -> std::io::Result<Instruction> {
    create_meteora_pool_ix_with_program_id(BOOP_PROGRAM_ID, keys)
}
pub fn create_meteora_pool_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CreateMeteoraPoolAccounts<'_, '_>,
) -> ProgramResult {
    let keys: CreateMeteoraPoolKeys = accounts.into();
    let ix = create_meteora_pool_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn create_meteora_pool_invoke(
    accounts: CreateMeteoraPoolAccounts<'_, '_>,
) -> ProgramResult {
    create_meteora_pool_invoke_with_program_id(BOOP_PROGRAM_ID, accounts)
}
pub fn create_meteora_pool_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CreateMeteoraPoolAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CreateMeteoraPoolKeys = accounts.into();
    let ix = create_meteora_pool_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn create_meteora_pool_invoke_signed(
    accounts: CreateMeteoraPoolAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    create_meteora_pool_invoke_signed_with_program_id(BOOP_PROGRAM_ID, accounts, seeds)
}
pub fn create_meteora_pool_verify_account_keys(
    accounts: CreateMeteoraPoolAccounts<'_, '_>,
    keys: CreateMeteoraPoolKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.operator.key, keys.operator),
        (*accounts.config.key, keys.config),
        (*accounts.vault_authority.key, keys.vault_authority),
        (*accounts.cp_amm_config.key, keys.cp_amm_config),
        (*accounts.pool_authority.key, keys.pool_authority),
        (*accounts.pool.key, keys.pool),
        (*accounts.position.key, keys.position),
        (*accounts.position_nft_mint.key, keys.position_nft_mint),
        (*accounts.position_nft_account.key, keys.position_nft_account),
        (*accounts.token_a_mint.key, keys.token_a_mint),
        (*accounts.token_b_mint.key, keys.token_b_mint),
        (*accounts.token_a_vault.key, keys.token_a_vault),
        (*accounts.token_b_vault.key, keys.token_b_vault),
        (*accounts.bonding_curve.key, keys.bonding_curve),
        (*accounts.bonding_curve_vault.key, keys.bonding_curve_vault),
        (*accounts.bonding_curve_wsol_vault.key, keys.bonding_curve_wsol_vault),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.token_2022_program.key, keys.token_2022_program),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.cp_amm_program.key, keys.cp_amm_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn create_meteora_pool_verify_writable_privileges<'me, 'info>(
    accounts: CreateMeteoraPoolAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.operator,
        accounts.vault_authority,
        accounts.pool,
        accounts.position,
        accounts.position_nft_mint,
        accounts.position_nft_account,
        accounts.token_a_vault,
        accounts.token_b_vault,
        accounts.bonding_curve,
        accounts.bonding_curve_vault,
        accounts.bonding_curve_wsol_vault,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn create_meteora_pool_verify_signer_privileges<'me, 'info>(
    accounts: CreateMeteoraPoolAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.operator, accounts.position_nft_mint] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn create_meteora_pool_verify_account_privileges<'me, 'info>(
    accounts: CreateMeteoraPoolAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    create_meteora_pool_verify_writable_privileges(accounts)?;
    create_meteora_pool_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CREATE_RAYDIUM_POOL_IX_ACCOUNTS_LEN: usize = 22;
#[derive(Copy, Clone, Debug)]
pub struct CreateRaydiumPoolAccounts<'me, 'info> {
    pub cpmm_program: &'me AccountInfo<'info>,
    pub amm_config: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub pool_state: &'me AccountInfo<'info>,
    pub token_0_mint: &'me AccountInfo<'info>,
    pub token_1_mint: &'me AccountInfo<'info>,
    pub lp_mint: &'me AccountInfo<'info>,
    pub vault_authority: &'me AccountInfo<'info>,
    pub bonding_curve: &'me AccountInfo<'info>,
    pub bonding_curve_vault: &'me AccountInfo<'info>,
    pub bonding_curve_wsol_vault: &'me AccountInfo<'info>,
    pub creator_lp_token: &'me AccountInfo<'info>,
    pub token_0_vault: &'me AccountInfo<'info>,
    pub token_1_vault: &'me AccountInfo<'info>,
    pub create_pool_fee: &'me AccountInfo<'info>,
    pub observation_state: &'me AccountInfo<'info>,
    pub operator: &'me AccountInfo<'info>,
    pub config: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub rent: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CreateRaydiumPoolKeys {
    pub cpmm_program: Pubkey,
    pub amm_config: Pubkey,
    pub authority: Pubkey,
    pub pool_state: Pubkey,
    pub token_0_mint: Pubkey,
    pub token_1_mint: Pubkey,
    pub lp_mint: Pubkey,
    pub vault_authority: Pubkey,
    pub bonding_curve: Pubkey,
    pub bonding_curve_vault: Pubkey,
    pub bonding_curve_wsol_vault: Pubkey,
    pub creator_lp_token: Pubkey,
    pub token_0_vault: Pubkey,
    pub token_1_vault: Pubkey,
    pub create_pool_fee: Pubkey,
    pub observation_state: Pubkey,
    pub operator: Pubkey,
    pub config: Pubkey,
    pub token_program: Pubkey,
    pub associated_token_program: Pubkey,
    pub system_program: Pubkey,
    pub rent: Pubkey,
}
impl From<CreateRaydiumPoolAccounts<'_, '_>> for CreateRaydiumPoolKeys {
    fn from(accounts: CreateRaydiumPoolAccounts) -> Self {
        Self {
            cpmm_program: *accounts.cpmm_program.key,
            amm_config: *accounts.amm_config.key,
            authority: *accounts.authority.key,
            pool_state: *accounts.pool_state.key,
            token_0_mint: *accounts.token_0_mint.key,
            token_1_mint: *accounts.token_1_mint.key,
            lp_mint: *accounts.lp_mint.key,
            vault_authority: *accounts.vault_authority.key,
            bonding_curve: *accounts.bonding_curve.key,
            bonding_curve_vault: *accounts.bonding_curve_vault.key,
            bonding_curve_wsol_vault: *accounts.bonding_curve_wsol_vault.key,
            creator_lp_token: *accounts.creator_lp_token.key,
            token_0_vault: *accounts.token_0_vault.key,
            token_1_vault: *accounts.token_1_vault.key,
            create_pool_fee: *accounts.create_pool_fee.key,
            observation_state: *accounts.observation_state.key,
            operator: *accounts.operator.key,
            config: *accounts.config.key,
            token_program: *accounts.token_program.key,
            associated_token_program: *accounts.associated_token_program.key,
            system_program: *accounts.system_program.key,
            rent: *accounts.rent.key,
        }
    }
}
impl From<CreateRaydiumPoolKeys> for [AccountMeta; CREATE_RAYDIUM_POOL_IX_ACCOUNTS_LEN] {
    fn from(keys: CreateRaydiumPoolKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.cpmm_program,
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
                is_writable: true,
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
                pubkey: keys.vault_authority,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.bonding_curve,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.bonding_curve_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.bonding_curve_wsol_vault,
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
                pubkey: keys.operator,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.config,
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
                pubkey: keys.rent,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; CREATE_RAYDIUM_POOL_IX_ACCOUNTS_LEN]> for CreateRaydiumPoolKeys {
    fn from(pubkeys: [Pubkey; CREATE_RAYDIUM_POOL_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            cpmm_program: pubkeys[0],
            amm_config: pubkeys[1],
            authority: pubkeys[2],
            pool_state: pubkeys[3],
            token_0_mint: pubkeys[4],
            token_1_mint: pubkeys[5],
            lp_mint: pubkeys[6],
            vault_authority: pubkeys[7],
            bonding_curve: pubkeys[8],
            bonding_curve_vault: pubkeys[9],
            bonding_curve_wsol_vault: pubkeys[10],
            creator_lp_token: pubkeys[11],
            token_0_vault: pubkeys[12],
            token_1_vault: pubkeys[13],
            create_pool_fee: pubkeys[14],
            observation_state: pubkeys[15],
            operator: pubkeys[16],
            config: pubkeys[17],
            token_program: pubkeys[18],
            associated_token_program: pubkeys[19],
            system_program: pubkeys[20],
            rent: pubkeys[21],
        }
    }
}
impl<'info> From<CreateRaydiumPoolAccounts<'_, 'info>>
for [AccountInfo<'info>; CREATE_RAYDIUM_POOL_IX_ACCOUNTS_LEN] {
    fn from(accounts: CreateRaydiumPoolAccounts<'_, 'info>) -> Self {
        [
            accounts.cpmm_program.clone(),
            accounts.amm_config.clone(),
            accounts.authority.clone(),
            accounts.pool_state.clone(),
            accounts.token_0_mint.clone(),
            accounts.token_1_mint.clone(),
            accounts.lp_mint.clone(),
            accounts.vault_authority.clone(),
            accounts.bonding_curve.clone(),
            accounts.bonding_curve_vault.clone(),
            accounts.bonding_curve_wsol_vault.clone(),
            accounts.creator_lp_token.clone(),
            accounts.token_0_vault.clone(),
            accounts.token_1_vault.clone(),
            accounts.create_pool_fee.clone(),
            accounts.observation_state.clone(),
            accounts.operator.clone(),
            accounts.config.clone(),
            accounts.token_program.clone(),
            accounts.associated_token_program.clone(),
            accounts.system_program.clone(),
            accounts.rent.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CREATE_RAYDIUM_POOL_IX_ACCOUNTS_LEN]>
for CreateRaydiumPoolAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; CREATE_RAYDIUM_POOL_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            cpmm_program: &arr[0],
            amm_config: &arr[1],
            authority: &arr[2],
            pool_state: &arr[3],
            token_0_mint: &arr[4],
            token_1_mint: &arr[5],
            lp_mint: &arr[6],
            vault_authority: &arr[7],
            bonding_curve: &arr[8],
            bonding_curve_vault: &arr[9],
            bonding_curve_wsol_vault: &arr[10],
            creator_lp_token: &arr[11],
            token_0_vault: &arr[12],
            token_1_vault: &arr[13],
            create_pool_fee: &arr[14],
            observation_state: &arr[15],
            operator: &arr[16],
            config: &arr[17],
            token_program: &arr[18],
            associated_token_program: &arr[19],
            system_program: &arr[20],
            rent: &arr[21],
        }
    }
}
pub const CREATE_RAYDIUM_POOL_IX_DISCM: [u8; 8usize] = [
    65, 45, 119, 77, 204, 178, 84, 2,
];
#[derive(Clone, Debug, PartialEq)]
pub struct CreateRaydiumPoolIxData;
impl CreateRaydiumPoolIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CREATE_RAYDIUM_POOL_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CREATE_RAYDIUM_POOL_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn create_raydium_pool_ix_with_program_id(
    program_id: Pubkey,
    keys: CreateRaydiumPoolKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CREATE_RAYDIUM_POOL_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: CreateRaydiumPoolIxData.try_to_vec()?,
    })
}
pub fn create_raydium_pool_ix(
    keys: CreateRaydiumPoolKeys,
) -> std::io::Result<Instruction> {
    create_raydium_pool_ix_with_program_id(BOOP_PROGRAM_ID, keys)
}
pub fn create_raydium_pool_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CreateRaydiumPoolAccounts<'_, '_>,
) -> ProgramResult {
    let keys: CreateRaydiumPoolKeys = accounts.into();
    let ix = create_raydium_pool_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn create_raydium_pool_invoke(
    accounts: CreateRaydiumPoolAccounts<'_, '_>,
) -> ProgramResult {
    create_raydium_pool_invoke_with_program_id(BOOP_PROGRAM_ID, accounts)
}
pub fn create_raydium_pool_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CreateRaydiumPoolAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CreateRaydiumPoolKeys = accounts.into();
    let ix = create_raydium_pool_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn create_raydium_pool_invoke_signed(
    accounts: CreateRaydiumPoolAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    create_raydium_pool_invoke_signed_with_program_id(BOOP_PROGRAM_ID, accounts, seeds)
}
pub fn create_raydium_pool_verify_account_keys(
    accounts: CreateRaydiumPoolAccounts<'_, '_>,
    keys: CreateRaydiumPoolKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.cpmm_program.key, keys.cpmm_program),
        (*accounts.amm_config.key, keys.amm_config),
        (*accounts.authority.key, keys.authority),
        (*accounts.pool_state.key, keys.pool_state),
        (*accounts.token_0_mint.key, keys.token_0_mint),
        (*accounts.token_1_mint.key, keys.token_1_mint),
        (*accounts.lp_mint.key, keys.lp_mint),
        (*accounts.vault_authority.key, keys.vault_authority),
        (*accounts.bonding_curve.key, keys.bonding_curve),
        (*accounts.bonding_curve_vault.key, keys.bonding_curve_vault),
        (*accounts.bonding_curve_wsol_vault.key, keys.bonding_curve_wsol_vault),
        (*accounts.creator_lp_token.key, keys.creator_lp_token),
        (*accounts.token_0_vault.key, keys.token_0_vault),
        (*accounts.token_1_vault.key, keys.token_1_vault),
        (*accounts.create_pool_fee.key, keys.create_pool_fee),
        (*accounts.observation_state.key, keys.observation_state),
        (*accounts.operator.key, keys.operator),
        (*accounts.config.key, keys.config),
        (*accounts.token_program.key, keys.token_program),
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
pub fn create_raydium_pool_verify_writable_privileges<'me, 'info>(
    accounts: CreateRaydiumPoolAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.pool_state,
        accounts.token_0_mint,
        accounts.lp_mint,
        accounts.vault_authority,
        accounts.bonding_curve,
        accounts.bonding_curve_vault,
        accounts.bonding_curve_wsol_vault,
        accounts.creator_lp_token,
        accounts.token_0_vault,
        accounts.token_1_vault,
        accounts.create_pool_fee,
        accounts.observation_state,
        accounts.operator,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn create_raydium_pool_verify_signer_privileges<'me, 'info>(
    accounts: CreateRaydiumPoolAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.operator] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn create_raydium_pool_verify_account_privileges<'me, 'info>(
    accounts: CreateRaydiumPoolAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    create_raydium_pool_verify_writable_privileges(accounts)?;
    create_raydium_pool_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CREATE_RAYDIUM_RANDOM_POOL_IX_ACCOUNTS_LEN: usize = 22;
#[derive(Copy, Clone, Debug)]
pub struct CreateRaydiumRandomPoolAccounts<'me, 'info> {
    pub cpmm_program: &'me AccountInfo<'info>,
    pub amm_config: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub pool_state: &'me AccountInfo<'info>,
    pub token_0_mint: &'me AccountInfo<'info>,
    pub token_1_mint: &'me AccountInfo<'info>,
    pub lp_mint: &'me AccountInfo<'info>,
    pub vault_authority: &'me AccountInfo<'info>,
    pub bonding_curve: &'me AccountInfo<'info>,
    pub bonding_curve_vault: &'me AccountInfo<'info>,
    pub bonding_curve_wsol_vault: &'me AccountInfo<'info>,
    pub creator_lp_token: &'me AccountInfo<'info>,
    pub token_0_vault: &'me AccountInfo<'info>,
    pub token_1_vault: &'me AccountInfo<'info>,
    pub create_pool_fee: &'me AccountInfo<'info>,
    pub observation_state: &'me AccountInfo<'info>,
    pub operator: &'me AccountInfo<'info>,
    pub config: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub rent: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CreateRaydiumRandomPoolKeys {
    pub cpmm_program: Pubkey,
    pub amm_config: Pubkey,
    pub authority: Pubkey,
    pub pool_state: Pubkey,
    pub token_0_mint: Pubkey,
    pub token_1_mint: Pubkey,
    pub lp_mint: Pubkey,
    pub vault_authority: Pubkey,
    pub bonding_curve: Pubkey,
    pub bonding_curve_vault: Pubkey,
    pub bonding_curve_wsol_vault: Pubkey,
    pub creator_lp_token: Pubkey,
    pub token_0_vault: Pubkey,
    pub token_1_vault: Pubkey,
    pub create_pool_fee: Pubkey,
    pub observation_state: Pubkey,
    pub operator: Pubkey,
    pub config: Pubkey,
    pub token_program: Pubkey,
    pub associated_token_program: Pubkey,
    pub system_program: Pubkey,
    pub rent: Pubkey,
}
impl From<CreateRaydiumRandomPoolAccounts<'_, '_>> for CreateRaydiumRandomPoolKeys {
    fn from(accounts: CreateRaydiumRandomPoolAccounts) -> Self {
        Self {
            cpmm_program: *accounts.cpmm_program.key,
            amm_config: *accounts.amm_config.key,
            authority: *accounts.authority.key,
            pool_state: *accounts.pool_state.key,
            token_0_mint: *accounts.token_0_mint.key,
            token_1_mint: *accounts.token_1_mint.key,
            lp_mint: *accounts.lp_mint.key,
            vault_authority: *accounts.vault_authority.key,
            bonding_curve: *accounts.bonding_curve.key,
            bonding_curve_vault: *accounts.bonding_curve_vault.key,
            bonding_curve_wsol_vault: *accounts.bonding_curve_wsol_vault.key,
            creator_lp_token: *accounts.creator_lp_token.key,
            token_0_vault: *accounts.token_0_vault.key,
            token_1_vault: *accounts.token_1_vault.key,
            create_pool_fee: *accounts.create_pool_fee.key,
            observation_state: *accounts.observation_state.key,
            operator: *accounts.operator.key,
            config: *accounts.config.key,
            token_program: *accounts.token_program.key,
            associated_token_program: *accounts.associated_token_program.key,
            system_program: *accounts.system_program.key,
            rent: *accounts.rent.key,
        }
    }
}
impl From<CreateRaydiumRandomPoolKeys>
for [AccountMeta; CREATE_RAYDIUM_RANDOM_POOL_IX_ACCOUNTS_LEN] {
    fn from(keys: CreateRaydiumRandomPoolKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.cpmm_program,
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
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_0_mint,
                is_signer: false,
                is_writable: true,
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
                pubkey: keys.vault_authority,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.bonding_curve,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.bonding_curve_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.bonding_curve_wsol_vault,
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
                pubkey: keys.operator,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.config,
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
                pubkey: keys.rent,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; CREATE_RAYDIUM_RANDOM_POOL_IX_ACCOUNTS_LEN]>
for CreateRaydiumRandomPoolKeys {
    fn from(pubkeys: [Pubkey; CREATE_RAYDIUM_RANDOM_POOL_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            cpmm_program: pubkeys[0],
            amm_config: pubkeys[1],
            authority: pubkeys[2],
            pool_state: pubkeys[3],
            token_0_mint: pubkeys[4],
            token_1_mint: pubkeys[5],
            lp_mint: pubkeys[6],
            vault_authority: pubkeys[7],
            bonding_curve: pubkeys[8],
            bonding_curve_vault: pubkeys[9],
            bonding_curve_wsol_vault: pubkeys[10],
            creator_lp_token: pubkeys[11],
            token_0_vault: pubkeys[12],
            token_1_vault: pubkeys[13],
            create_pool_fee: pubkeys[14],
            observation_state: pubkeys[15],
            operator: pubkeys[16],
            config: pubkeys[17],
            token_program: pubkeys[18],
            associated_token_program: pubkeys[19],
            system_program: pubkeys[20],
            rent: pubkeys[21],
        }
    }
}
impl<'info> From<CreateRaydiumRandomPoolAccounts<'_, 'info>>
for [AccountInfo<'info>; CREATE_RAYDIUM_RANDOM_POOL_IX_ACCOUNTS_LEN] {
    fn from(accounts: CreateRaydiumRandomPoolAccounts<'_, 'info>) -> Self {
        [
            accounts.cpmm_program.clone(),
            accounts.amm_config.clone(),
            accounts.authority.clone(),
            accounts.pool_state.clone(),
            accounts.token_0_mint.clone(),
            accounts.token_1_mint.clone(),
            accounts.lp_mint.clone(),
            accounts.vault_authority.clone(),
            accounts.bonding_curve.clone(),
            accounts.bonding_curve_vault.clone(),
            accounts.bonding_curve_wsol_vault.clone(),
            accounts.creator_lp_token.clone(),
            accounts.token_0_vault.clone(),
            accounts.token_1_vault.clone(),
            accounts.create_pool_fee.clone(),
            accounts.observation_state.clone(),
            accounts.operator.clone(),
            accounts.config.clone(),
            accounts.token_program.clone(),
            accounts.associated_token_program.clone(),
            accounts.system_program.clone(),
            accounts.rent.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; CREATE_RAYDIUM_RANDOM_POOL_IX_ACCOUNTS_LEN]>
for CreateRaydiumRandomPoolAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; CREATE_RAYDIUM_RANDOM_POOL_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            cpmm_program: &arr[0],
            amm_config: &arr[1],
            authority: &arr[2],
            pool_state: &arr[3],
            token_0_mint: &arr[4],
            token_1_mint: &arr[5],
            lp_mint: &arr[6],
            vault_authority: &arr[7],
            bonding_curve: &arr[8],
            bonding_curve_vault: &arr[9],
            bonding_curve_wsol_vault: &arr[10],
            creator_lp_token: &arr[11],
            token_0_vault: &arr[12],
            token_1_vault: &arr[13],
            create_pool_fee: &arr[14],
            observation_state: &arr[15],
            operator: &arr[16],
            config: &arr[17],
            token_program: &arr[18],
            associated_token_program: &arr[19],
            system_program: &arr[20],
            rent: &arr[21],
        }
    }
}
pub const CREATE_RAYDIUM_RANDOM_POOL_IX_DISCM: [u8; 8usize] = [
    78, 44, 173, 29, 132, 180, 4, 172,
];
#[derive(Clone, Debug, PartialEq)]
pub struct CreateRaydiumRandomPoolIxData;
impl CreateRaydiumRandomPoolIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CREATE_RAYDIUM_RANDOM_POOL_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CREATE_RAYDIUM_RANDOM_POOL_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn create_raydium_random_pool_ix_with_program_id(
    program_id: Pubkey,
    keys: CreateRaydiumRandomPoolKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CREATE_RAYDIUM_RANDOM_POOL_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: CreateRaydiumRandomPoolIxData.try_to_vec()?,
    })
}
pub fn create_raydium_random_pool_ix(
    keys: CreateRaydiumRandomPoolKeys,
) -> std::io::Result<Instruction> {
    create_raydium_random_pool_ix_with_program_id(BOOP_PROGRAM_ID, keys)
}
pub fn create_raydium_random_pool_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CreateRaydiumRandomPoolAccounts<'_, '_>,
) -> ProgramResult {
    let keys: CreateRaydiumRandomPoolKeys = accounts.into();
    let ix = create_raydium_random_pool_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn create_raydium_random_pool_invoke(
    accounts: CreateRaydiumRandomPoolAccounts<'_, '_>,
) -> ProgramResult {
    create_raydium_random_pool_invoke_with_program_id(BOOP_PROGRAM_ID, accounts)
}
pub fn create_raydium_random_pool_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CreateRaydiumRandomPoolAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CreateRaydiumRandomPoolKeys = accounts.into();
    let ix = create_raydium_random_pool_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn create_raydium_random_pool_invoke_signed(
    accounts: CreateRaydiumRandomPoolAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    create_raydium_random_pool_invoke_signed_with_program_id(
        BOOP_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn create_raydium_random_pool_verify_account_keys(
    accounts: CreateRaydiumRandomPoolAccounts<'_, '_>,
    keys: CreateRaydiumRandomPoolKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.cpmm_program.key, keys.cpmm_program),
        (*accounts.amm_config.key, keys.amm_config),
        (*accounts.authority.key, keys.authority),
        (*accounts.pool_state.key, keys.pool_state),
        (*accounts.token_0_mint.key, keys.token_0_mint),
        (*accounts.token_1_mint.key, keys.token_1_mint),
        (*accounts.lp_mint.key, keys.lp_mint),
        (*accounts.vault_authority.key, keys.vault_authority),
        (*accounts.bonding_curve.key, keys.bonding_curve),
        (*accounts.bonding_curve_vault.key, keys.bonding_curve_vault),
        (*accounts.bonding_curve_wsol_vault.key, keys.bonding_curve_wsol_vault),
        (*accounts.creator_lp_token.key, keys.creator_lp_token),
        (*accounts.token_0_vault.key, keys.token_0_vault),
        (*accounts.token_1_vault.key, keys.token_1_vault),
        (*accounts.create_pool_fee.key, keys.create_pool_fee),
        (*accounts.observation_state.key, keys.observation_state),
        (*accounts.operator.key, keys.operator),
        (*accounts.config.key, keys.config),
        (*accounts.token_program.key, keys.token_program),
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
pub fn create_raydium_random_pool_verify_writable_privileges<'me, 'info>(
    accounts: CreateRaydiumRandomPoolAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.pool_state,
        accounts.token_0_mint,
        accounts.lp_mint,
        accounts.vault_authority,
        accounts.bonding_curve,
        accounts.bonding_curve_vault,
        accounts.bonding_curve_wsol_vault,
        accounts.creator_lp_token,
        accounts.token_0_vault,
        accounts.token_1_vault,
        accounts.create_pool_fee,
        accounts.observation_state,
        accounts.operator,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn create_raydium_random_pool_verify_signer_privileges<'me, 'info>(
    accounts: CreateRaydiumRandomPoolAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.pool_state, accounts.operator] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn create_raydium_random_pool_verify_account_privileges<'me, 'info>(
    accounts: CreateRaydiumRandomPoolAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    create_raydium_random_pool_verify_writable_privileges(accounts)?;
    create_raydium_random_pool_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CREATE_TOKEN_IX_ACCOUNTS_LEN: usize = 8;
#[derive(Copy, Clone, Debug)]
pub struct CreateTokenAccounts<'me, 'info> {
    pub config: &'me AccountInfo<'info>,
    pub metadata: &'me AccountInfo<'info>,
    pub mint: &'me AccountInfo<'info>,
    pub payer: &'me AccountInfo<'info>,
    pub rent: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub token_metadata_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CreateTokenKeys {
    pub config: Pubkey,
    pub metadata: Pubkey,
    pub mint: Pubkey,
    pub payer: Pubkey,
    pub rent: Pubkey,
    pub system_program: Pubkey,
    pub token_program: Pubkey,
    pub token_metadata_program: Pubkey,
}
impl From<CreateTokenAccounts<'_, '_>> for CreateTokenKeys {
    fn from(accounts: CreateTokenAccounts) -> Self {
        Self {
            config: *accounts.config.key,
            metadata: *accounts.metadata.key,
            mint: *accounts.mint.key,
            payer: *accounts.payer.key,
            rent: *accounts.rent.key,
            system_program: *accounts.system_program.key,
            token_program: *accounts.token_program.key,
            token_metadata_program: *accounts.token_metadata_program.key,
        }
    }
}
impl From<CreateTokenKeys> for [AccountMeta; CREATE_TOKEN_IX_ACCOUNTS_LEN] {
    fn from(keys: CreateTokenKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.metadata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.payer,
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
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_metadata_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; CREATE_TOKEN_IX_ACCOUNTS_LEN]> for CreateTokenKeys {
    fn from(pubkeys: [Pubkey; CREATE_TOKEN_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            config: pubkeys[0],
            metadata: pubkeys[1],
            mint: pubkeys[2],
            payer: pubkeys[3],
            rent: pubkeys[4],
            system_program: pubkeys[5],
            token_program: pubkeys[6],
            token_metadata_program: pubkeys[7],
        }
    }
}
impl<'info> From<CreateTokenAccounts<'_, 'info>>
for [AccountInfo<'info>; CREATE_TOKEN_IX_ACCOUNTS_LEN] {
    fn from(accounts: CreateTokenAccounts<'_, 'info>) -> Self {
        [
            accounts.config.clone(),
            accounts.metadata.clone(),
            accounts.mint.clone(),
            accounts.payer.clone(),
            accounts.rent.clone(),
            accounts.system_program.clone(),
            accounts.token_program.clone(),
            accounts.token_metadata_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CREATE_TOKEN_IX_ACCOUNTS_LEN]>
for CreateTokenAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; CREATE_TOKEN_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            config: &arr[0],
            metadata: &arr[1],
            mint: &arr[2],
            payer: &arr[3],
            rent: &arr[4],
            system_program: &arr[5],
            token_program: &arr[6],
            token_metadata_program: &arr[7],
        }
    }
}
pub const CREATE_TOKEN_IX_DISCM: [u8; 8usize] = [84, 52, 204, 228, 24, 140, 234, 75];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CreateTokenIxArgs {
    pub salt: u64,
    pub name: String,
    pub symbol: String,
    pub uri: String,
}
#[derive(Clone, Debug, PartialEq)]
pub struct CreateTokenIxData(pub CreateTokenIxArgs);
impl From<CreateTokenIxArgs> for CreateTokenIxData {
    fn from(args: CreateTokenIxArgs) -> Self {
        Self(args)
    }
}
impl CreateTokenIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CREATE_TOKEN_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let salt: u64 = crate::borsh_de_or_default(&mut reader)?;
        let name: String = crate::borsh_de_or_default(&mut reader)?;
        let symbol: String = crate::borsh_de_or_default(&mut reader)?;
        let uri: String = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(CreateTokenIxArgs {
                salt,
                name,
                symbol,
                uri,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CREATE_TOKEN_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.salt, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.name, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.symbol, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.uri, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn create_token_ix_with_program_id(
    program_id: Pubkey,
    keys: CreateTokenKeys,
    args: CreateTokenIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CREATE_TOKEN_IX_ACCOUNTS_LEN] = keys.into();
    let data: CreateTokenIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn create_token_ix(
    keys: CreateTokenKeys,
    args: CreateTokenIxArgs,
) -> std::io::Result<Instruction> {
    create_token_ix_with_program_id(BOOP_PROGRAM_ID, keys, args)
}
pub fn create_token_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CreateTokenAccounts<'_, '_>,
    args: CreateTokenIxArgs,
) -> ProgramResult {
    let keys: CreateTokenKeys = accounts.into();
    let ix = create_token_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn create_token_invoke(
    accounts: CreateTokenAccounts<'_, '_>,
    args: CreateTokenIxArgs,
) -> ProgramResult {
    create_token_invoke_with_program_id(BOOP_PROGRAM_ID, accounts, args)
}
pub fn create_token_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CreateTokenAccounts<'_, '_>,
    args: CreateTokenIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CreateTokenKeys = accounts.into();
    let ix = create_token_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn create_token_invoke_signed(
    accounts: CreateTokenAccounts<'_, '_>,
    args: CreateTokenIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    create_token_invoke_signed_with_program_id(BOOP_PROGRAM_ID, accounts, args, seeds)
}
pub fn create_token_verify_account_keys(
    accounts: CreateTokenAccounts<'_, '_>,
    keys: CreateTokenKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.config.key, keys.config),
        (*accounts.metadata.key, keys.metadata),
        (*accounts.mint.key, keys.mint),
        (*accounts.payer.key, keys.payer),
        (*accounts.rent.key, keys.rent),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.token_metadata_program.key, keys.token_metadata_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn create_token_verify_writable_privileges<'me, 'info>(
    accounts: CreateTokenAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.metadata, accounts.mint, accounts.payer] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn create_token_verify_signer_privileges<'me, 'info>(
    accounts: CreateTokenAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.payer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn create_token_verify_account_privileges<'me, 'info>(
    accounts: CreateTokenAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    create_token_verify_writable_privileges(accounts)?;
    create_token_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CREATE_TOKEN_FALLBACK_IX_ACCOUNTS_LEN: usize = 8;
#[derive(Copy, Clone, Debug)]
pub struct CreateTokenFallbackAccounts<'me, 'info> {
    pub config: &'me AccountInfo<'info>,
    pub metadata: &'me AccountInfo<'info>,
    pub mint: &'me AccountInfo<'info>,
    pub payer: &'me AccountInfo<'info>,
    pub rent: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub token_metadata_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CreateTokenFallbackKeys {
    pub config: Pubkey,
    pub metadata: Pubkey,
    pub mint: Pubkey,
    pub payer: Pubkey,
    pub rent: Pubkey,
    pub system_program: Pubkey,
    pub token_program: Pubkey,
    pub token_metadata_program: Pubkey,
}
impl From<CreateTokenFallbackAccounts<'_, '_>> for CreateTokenFallbackKeys {
    fn from(accounts: CreateTokenFallbackAccounts) -> Self {
        Self {
            config: *accounts.config.key,
            metadata: *accounts.metadata.key,
            mint: *accounts.mint.key,
            payer: *accounts.payer.key,
            rent: *accounts.rent.key,
            system_program: *accounts.system_program.key,
            token_program: *accounts.token_program.key,
            token_metadata_program: *accounts.token_metadata_program.key,
        }
    }
}
impl From<CreateTokenFallbackKeys>
for [AccountMeta; CREATE_TOKEN_FALLBACK_IX_ACCOUNTS_LEN] {
    fn from(keys: CreateTokenFallbackKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.metadata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.payer,
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
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_metadata_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; CREATE_TOKEN_FALLBACK_IX_ACCOUNTS_LEN]> for CreateTokenFallbackKeys {
    fn from(pubkeys: [Pubkey; CREATE_TOKEN_FALLBACK_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            config: pubkeys[0],
            metadata: pubkeys[1],
            mint: pubkeys[2],
            payer: pubkeys[3],
            rent: pubkeys[4],
            system_program: pubkeys[5],
            token_program: pubkeys[6],
            token_metadata_program: pubkeys[7],
        }
    }
}
impl<'info> From<CreateTokenFallbackAccounts<'_, 'info>>
for [AccountInfo<'info>; CREATE_TOKEN_FALLBACK_IX_ACCOUNTS_LEN] {
    fn from(accounts: CreateTokenFallbackAccounts<'_, 'info>) -> Self {
        [
            accounts.config.clone(),
            accounts.metadata.clone(),
            accounts.mint.clone(),
            accounts.payer.clone(),
            accounts.rent.clone(),
            accounts.system_program.clone(),
            accounts.token_program.clone(),
            accounts.token_metadata_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CREATE_TOKEN_FALLBACK_IX_ACCOUNTS_LEN]>
for CreateTokenFallbackAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; CREATE_TOKEN_FALLBACK_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            config: &arr[0],
            metadata: &arr[1],
            mint: &arr[2],
            payer: &arr[3],
            rent: &arr[4],
            system_program: &arr[5],
            token_program: &arr[6],
            token_metadata_program: &arr[7],
        }
    }
}
pub const CREATE_TOKEN_FALLBACK_IX_DISCM: [u8; 8usize] = [
    253, 184, 126, 199, 235, 232, 172, 162,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CreateTokenFallbackIxArgs {
    pub salt: u64,
    pub name: String,
    pub symbol: String,
    pub uri: String,
}
#[derive(Clone, Debug, PartialEq)]
pub struct CreateTokenFallbackIxData(pub CreateTokenFallbackIxArgs);
impl From<CreateTokenFallbackIxArgs> for CreateTokenFallbackIxData {
    fn from(args: CreateTokenFallbackIxArgs) -> Self {
        Self(args)
    }
}
impl CreateTokenFallbackIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CREATE_TOKEN_FALLBACK_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let salt: u64 = crate::borsh_de_or_default(&mut reader)?;
        let name: String = crate::borsh_de_or_default(&mut reader)?;
        let symbol: String = crate::borsh_de_or_default(&mut reader)?;
        let uri: String = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(CreateTokenFallbackIxArgs {
                salt,
                name,
                symbol,
                uri,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CREATE_TOKEN_FALLBACK_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.salt, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.name, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.symbol, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.uri, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn create_token_fallback_ix_with_program_id(
    program_id: Pubkey,
    keys: CreateTokenFallbackKeys,
    args: CreateTokenFallbackIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CREATE_TOKEN_FALLBACK_IX_ACCOUNTS_LEN] = keys.into();
    let data: CreateTokenFallbackIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn create_token_fallback_ix(
    keys: CreateTokenFallbackKeys,
    args: CreateTokenFallbackIxArgs,
) -> std::io::Result<Instruction> {
    create_token_fallback_ix_with_program_id(BOOP_PROGRAM_ID, keys, args)
}
pub fn create_token_fallback_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CreateTokenFallbackAccounts<'_, '_>,
    args: CreateTokenFallbackIxArgs,
) -> ProgramResult {
    let keys: CreateTokenFallbackKeys = accounts.into();
    let ix = create_token_fallback_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn create_token_fallback_invoke(
    accounts: CreateTokenFallbackAccounts<'_, '_>,
    args: CreateTokenFallbackIxArgs,
) -> ProgramResult {
    create_token_fallback_invoke_with_program_id(BOOP_PROGRAM_ID, accounts, args)
}
pub fn create_token_fallback_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CreateTokenFallbackAccounts<'_, '_>,
    args: CreateTokenFallbackIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CreateTokenFallbackKeys = accounts.into();
    let ix = create_token_fallback_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn create_token_fallback_invoke_signed(
    accounts: CreateTokenFallbackAccounts<'_, '_>,
    args: CreateTokenFallbackIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    create_token_fallback_invoke_signed_with_program_id(
        BOOP_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn create_token_fallback_verify_account_keys(
    accounts: CreateTokenFallbackAccounts<'_, '_>,
    keys: CreateTokenFallbackKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.config.key, keys.config),
        (*accounts.metadata.key, keys.metadata),
        (*accounts.mint.key, keys.mint),
        (*accounts.payer.key, keys.payer),
        (*accounts.rent.key, keys.rent),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.token_metadata_program.key, keys.token_metadata_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn create_token_fallback_verify_writable_privileges<'me, 'info>(
    accounts: CreateTokenFallbackAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.metadata, accounts.mint, accounts.payer] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn create_token_fallback_verify_signer_privileges<'me, 'info>(
    accounts: CreateTokenFallbackAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.payer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn create_token_fallback_verify_account_privileges<'me, 'info>(
    accounts: CreateTokenFallbackAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    create_token_fallback_verify_writable_privileges(accounts)?;
    create_token_fallback_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const DEPLOY_BONDING_CURVE_IX_ACCOUNTS_LEN: usize = 10;
#[derive(Copy, Clone, Debug)]
pub struct DeployBondingCurveAccounts<'me, 'info> {
    pub mint: &'me AccountInfo<'info>,
    pub vault_authority: &'me AccountInfo<'info>,
    pub bonding_curve: &'me AccountInfo<'info>,
    pub bonding_curve_sol_vault: &'me AccountInfo<'info>,
    pub bonding_curve_vault: &'me AccountInfo<'info>,
    pub config: &'me AccountInfo<'info>,
    pub payer: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct DeployBondingCurveKeys {
    pub mint: Pubkey,
    pub vault_authority: Pubkey,
    pub bonding_curve: Pubkey,
    pub bonding_curve_sol_vault: Pubkey,
    pub bonding_curve_vault: Pubkey,
    pub config: Pubkey,
    pub payer: Pubkey,
    pub system_program: Pubkey,
    pub token_program: Pubkey,
    pub associated_token_program: Pubkey,
}
impl From<DeployBondingCurveAccounts<'_, '_>> for DeployBondingCurveKeys {
    fn from(accounts: DeployBondingCurveAccounts) -> Self {
        Self {
            mint: *accounts.mint.key,
            vault_authority: *accounts.vault_authority.key,
            bonding_curve: *accounts.bonding_curve.key,
            bonding_curve_sol_vault: *accounts.bonding_curve_sol_vault.key,
            bonding_curve_vault: *accounts.bonding_curve_vault.key,
            config: *accounts.config.key,
            payer: *accounts.payer.key,
            system_program: *accounts.system_program.key,
            token_program: *accounts.token_program.key,
            associated_token_program: *accounts.associated_token_program.key,
        }
    }
}
impl From<DeployBondingCurveKeys>
for [AccountMeta; DEPLOY_BONDING_CURVE_IX_ACCOUNTS_LEN] {
    fn from(keys: DeployBondingCurveKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vault_authority,
                is_signer: false,
                is_writable: false,
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
                pubkey: keys.bonding_curve_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.config,
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
impl From<[Pubkey; DEPLOY_BONDING_CURVE_IX_ACCOUNTS_LEN]> for DeployBondingCurveKeys {
    fn from(pubkeys: [Pubkey; DEPLOY_BONDING_CURVE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            mint: pubkeys[0],
            vault_authority: pubkeys[1],
            bonding_curve: pubkeys[2],
            bonding_curve_sol_vault: pubkeys[3],
            bonding_curve_vault: pubkeys[4],
            config: pubkeys[5],
            payer: pubkeys[6],
            system_program: pubkeys[7],
            token_program: pubkeys[8],
            associated_token_program: pubkeys[9],
        }
    }
}
impl<'info> From<DeployBondingCurveAccounts<'_, 'info>>
for [AccountInfo<'info>; DEPLOY_BONDING_CURVE_IX_ACCOUNTS_LEN] {
    fn from(accounts: DeployBondingCurveAccounts<'_, 'info>) -> Self {
        [
            accounts.mint.clone(),
            accounts.vault_authority.clone(),
            accounts.bonding_curve.clone(),
            accounts.bonding_curve_sol_vault.clone(),
            accounts.bonding_curve_vault.clone(),
            accounts.config.clone(),
            accounts.payer.clone(),
            accounts.system_program.clone(),
            accounts.token_program.clone(),
            accounts.associated_token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; DEPLOY_BONDING_CURVE_IX_ACCOUNTS_LEN]>
for DeployBondingCurveAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; DEPLOY_BONDING_CURVE_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            mint: &arr[0],
            vault_authority: &arr[1],
            bonding_curve: &arr[2],
            bonding_curve_sol_vault: &arr[3],
            bonding_curve_vault: &arr[4],
            config: &arr[5],
            payer: &arr[6],
            system_program: &arr[7],
            token_program: &arr[8],
            associated_token_program: &arr[9],
        }
    }
}
pub const DEPLOY_BONDING_CURVE_IX_DISCM: [u8; 8usize] = [
    180, 89, 199, 76, 168, 236, 217, 138,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DeployBondingCurveIxArgs {
    pub creator: Pubkey,
    pub salt: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct DeployBondingCurveIxData(pub DeployBondingCurveIxArgs);
impl From<DeployBondingCurveIxArgs> for DeployBondingCurveIxData {
    fn from(args: DeployBondingCurveIxArgs) -> Self {
        Self(args)
    }
}
impl DeployBondingCurveIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != DEPLOY_BONDING_CURVE_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let creator: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let salt: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(DeployBondingCurveIxArgs {
                creator,
                salt,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&DEPLOY_BONDING_CURVE_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.creator, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.salt, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn deploy_bonding_curve_ix_with_program_id(
    program_id: Pubkey,
    keys: DeployBondingCurveKeys,
    args: DeployBondingCurveIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; DEPLOY_BONDING_CURVE_IX_ACCOUNTS_LEN] = keys.into();
    let data: DeployBondingCurveIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn deploy_bonding_curve_ix(
    keys: DeployBondingCurveKeys,
    args: DeployBondingCurveIxArgs,
) -> std::io::Result<Instruction> {
    deploy_bonding_curve_ix_with_program_id(BOOP_PROGRAM_ID, keys, args)
}
pub fn deploy_bonding_curve_invoke_with_program_id(
    program_id: Pubkey,
    accounts: DeployBondingCurveAccounts<'_, '_>,
    args: DeployBondingCurveIxArgs,
) -> ProgramResult {
    let keys: DeployBondingCurveKeys = accounts.into();
    let ix = deploy_bonding_curve_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn deploy_bonding_curve_invoke(
    accounts: DeployBondingCurveAccounts<'_, '_>,
    args: DeployBondingCurveIxArgs,
) -> ProgramResult {
    deploy_bonding_curve_invoke_with_program_id(BOOP_PROGRAM_ID, accounts, args)
}
pub fn deploy_bonding_curve_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: DeployBondingCurveAccounts<'_, '_>,
    args: DeployBondingCurveIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: DeployBondingCurveKeys = accounts.into();
    let ix = deploy_bonding_curve_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn deploy_bonding_curve_invoke_signed(
    accounts: DeployBondingCurveAccounts<'_, '_>,
    args: DeployBondingCurveIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    deploy_bonding_curve_invoke_signed_with_program_id(
        BOOP_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn deploy_bonding_curve_verify_account_keys(
    accounts: DeployBondingCurveAccounts<'_, '_>,
    keys: DeployBondingCurveKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.mint.key, keys.mint),
        (*accounts.vault_authority.key, keys.vault_authority),
        (*accounts.bonding_curve.key, keys.bonding_curve),
        (*accounts.bonding_curve_sol_vault.key, keys.bonding_curve_sol_vault),
        (*accounts.bonding_curve_vault.key, keys.bonding_curve_vault),
        (*accounts.config.key, keys.config),
        (*accounts.payer.key, keys.payer),
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
pub fn deploy_bonding_curve_verify_writable_privileges<'me, 'info>(
    accounts: DeployBondingCurveAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.mint,
        accounts.bonding_curve,
        accounts.bonding_curve_sol_vault,
        accounts.bonding_curve_vault,
        accounts.payer,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn deploy_bonding_curve_verify_signer_privileges<'me, 'info>(
    accounts: DeployBondingCurveAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.payer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn deploy_bonding_curve_verify_account_privileges<'me, 'info>(
    accounts: DeployBondingCurveAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    deploy_bonding_curve_verify_writable_privileges(accounts)?;
    deploy_bonding_curve_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const DEPLOY_BONDING_CURVE_FALLBACK_IX_ACCOUNTS_LEN: usize = 10;
#[derive(Copy, Clone, Debug)]
pub struct DeployBondingCurveFallbackAccounts<'me, 'info> {
    pub mint: &'me AccountInfo<'info>,
    pub vault_authority: &'me AccountInfo<'info>,
    pub bonding_curve: &'me AccountInfo<'info>,
    pub bonding_curve_sol_vault: &'me AccountInfo<'info>,
    pub bonding_curve_vault: &'me AccountInfo<'info>,
    pub config: &'me AccountInfo<'info>,
    pub payer: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct DeployBondingCurveFallbackKeys {
    pub mint: Pubkey,
    pub vault_authority: Pubkey,
    pub bonding_curve: Pubkey,
    pub bonding_curve_sol_vault: Pubkey,
    pub bonding_curve_vault: Pubkey,
    pub config: Pubkey,
    pub payer: Pubkey,
    pub system_program: Pubkey,
    pub token_program: Pubkey,
    pub associated_token_program: Pubkey,
}
impl From<DeployBondingCurveFallbackAccounts<'_, '_>>
for DeployBondingCurveFallbackKeys {
    fn from(accounts: DeployBondingCurveFallbackAccounts) -> Self {
        Self {
            mint: *accounts.mint.key,
            vault_authority: *accounts.vault_authority.key,
            bonding_curve: *accounts.bonding_curve.key,
            bonding_curve_sol_vault: *accounts.bonding_curve_sol_vault.key,
            bonding_curve_vault: *accounts.bonding_curve_vault.key,
            config: *accounts.config.key,
            payer: *accounts.payer.key,
            system_program: *accounts.system_program.key,
            token_program: *accounts.token_program.key,
            associated_token_program: *accounts.associated_token_program.key,
        }
    }
}
impl From<DeployBondingCurveFallbackKeys>
for [AccountMeta; DEPLOY_BONDING_CURVE_FALLBACK_IX_ACCOUNTS_LEN] {
    fn from(keys: DeployBondingCurveFallbackKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vault_authority,
                is_signer: false,
                is_writable: false,
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
                pubkey: keys.bonding_curve_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.config,
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
impl From<[Pubkey; DEPLOY_BONDING_CURVE_FALLBACK_IX_ACCOUNTS_LEN]>
for DeployBondingCurveFallbackKeys {
    fn from(pubkeys: [Pubkey; DEPLOY_BONDING_CURVE_FALLBACK_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            mint: pubkeys[0],
            vault_authority: pubkeys[1],
            bonding_curve: pubkeys[2],
            bonding_curve_sol_vault: pubkeys[3],
            bonding_curve_vault: pubkeys[4],
            config: pubkeys[5],
            payer: pubkeys[6],
            system_program: pubkeys[7],
            token_program: pubkeys[8],
            associated_token_program: pubkeys[9],
        }
    }
}
impl<'info> From<DeployBondingCurveFallbackAccounts<'_, 'info>>
for [AccountInfo<'info>; DEPLOY_BONDING_CURVE_FALLBACK_IX_ACCOUNTS_LEN] {
    fn from(accounts: DeployBondingCurveFallbackAccounts<'_, 'info>) -> Self {
        [
            accounts.mint.clone(),
            accounts.vault_authority.clone(),
            accounts.bonding_curve.clone(),
            accounts.bonding_curve_sol_vault.clone(),
            accounts.bonding_curve_vault.clone(),
            accounts.config.clone(),
            accounts.payer.clone(),
            accounts.system_program.clone(),
            accounts.token_program.clone(),
            accounts.associated_token_program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; DEPLOY_BONDING_CURVE_FALLBACK_IX_ACCOUNTS_LEN]>
for DeployBondingCurveFallbackAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; DEPLOY_BONDING_CURVE_FALLBACK_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            mint: &arr[0],
            vault_authority: &arr[1],
            bonding_curve: &arr[2],
            bonding_curve_sol_vault: &arr[3],
            bonding_curve_vault: &arr[4],
            config: &arr[5],
            payer: &arr[6],
            system_program: &arr[7],
            token_program: &arr[8],
            associated_token_program: &arr[9],
        }
    }
}
pub const DEPLOY_BONDING_CURVE_FALLBACK_IX_DISCM: [u8; 8usize] = [
    53, 230, 172, 84, 77, 174, 22, 61,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DeployBondingCurveFallbackIxArgs {
    pub creator: Pubkey,
    pub salt: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct DeployBondingCurveFallbackIxData(pub DeployBondingCurveFallbackIxArgs);
impl From<DeployBondingCurveFallbackIxArgs> for DeployBondingCurveFallbackIxData {
    fn from(args: DeployBondingCurveFallbackIxArgs) -> Self {
        Self(args)
    }
}
impl DeployBondingCurveFallbackIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != DEPLOY_BONDING_CURVE_FALLBACK_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let creator: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let salt: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(DeployBondingCurveFallbackIxArgs {
                creator,
                salt,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&DEPLOY_BONDING_CURVE_FALLBACK_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.creator, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.salt, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn deploy_bonding_curve_fallback_ix_with_program_id(
    program_id: Pubkey,
    keys: DeployBondingCurveFallbackKeys,
    args: DeployBondingCurveFallbackIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; DEPLOY_BONDING_CURVE_FALLBACK_IX_ACCOUNTS_LEN] = keys
        .into();
    let data: DeployBondingCurveFallbackIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn deploy_bonding_curve_fallback_ix(
    keys: DeployBondingCurveFallbackKeys,
    args: DeployBondingCurveFallbackIxArgs,
) -> std::io::Result<Instruction> {
    deploy_bonding_curve_fallback_ix_with_program_id(BOOP_PROGRAM_ID, keys, args)
}
pub fn deploy_bonding_curve_fallback_invoke_with_program_id(
    program_id: Pubkey,
    accounts: DeployBondingCurveFallbackAccounts<'_, '_>,
    args: DeployBondingCurveFallbackIxArgs,
) -> ProgramResult {
    let keys: DeployBondingCurveFallbackKeys = accounts.into();
    let ix = deploy_bonding_curve_fallback_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn deploy_bonding_curve_fallback_invoke(
    accounts: DeployBondingCurveFallbackAccounts<'_, '_>,
    args: DeployBondingCurveFallbackIxArgs,
) -> ProgramResult {
    deploy_bonding_curve_fallback_invoke_with_program_id(BOOP_PROGRAM_ID, accounts, args)
}
pub fn deploy_bonding_curve_fallback_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: DeployBondingCurveFallbackAccounts<'_, '_>,
    args: DeployBondingCurveFallbackIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: DeployBondingCurveFallbackKeys = accounts.into();
    let ix = deploy_bonding_curve_fallback_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn deploy_bonding_curve_fallback_invoke_signed(
    accounts: DeployBondingCurveFallbackAccounts<'_, '_>,
    args: DeployBondingCurveFallbackIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    deploy_bonding_curve_fallback_invoke_signed_with_program_id(
        BOOP_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn deploy_bonding_curve_fallback_verify_account_keys(
    accounts: DeployBondingCurveFallbackAccounts<'_, '_>,
    keys: DeployBondingCurveFallbackKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.mint.key, keys.mint),
        (*accounts.vault_authority.key, keys.vault_authority),
        (*accounts.bonding_curve.key, keys.bonding_curve),
        (*accounts.bonding_curve_sol_vault.key, keys.bonding_curve_sol_vault),
        (*accounts.bonding_curve_vault.key, keys.bonding_curve_vault),
        (*accounts.config.key, keys.config),
        (*accounts.payer.key, keys.payer),
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
pub fn deploy_bonding_curve_fallback_verify_writable_privileges<'me, 'info>(
    accounts: DeployBondingCurveFallbackAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.mint,
        accounts.bonding_curve,
        accounts.bonding_curve_sol_vault,
        accounts.bonding_curve_vault,
        accounts.payer,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn deploy_bonding_curve_fallback_verify_signer_privileges<'me, 'info>(
    accounts: DeployBondingCurveFallbackAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.payer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn deploy_bonding_curve_fallback_verify_account_privileges<'me, 'info>(
    accounts: DeployBondingCurveFallbackAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    deploy_bonding_curve_fallback_verify_writable_privileges(accounts)?;
    deploy_bonding_curve_fallback_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const DEPOSIT_INTO_RAYDIUM_IX_ACCOUNTS_LEN: usize = 21;
#[derive(Copy, Clone, Debug)]
pub struct DepositIntoRaydiumAccounts<'me, 'info> {
    pub config: &'me AccountInfo<'info>,
    pub amm_config: &'me AccountInfo<'info>,
    pub operator: &'me AccountInfo<'info>,
    pub operator_wsol_account: &'me AccountInfo<'info>,
    pub vault_authority: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub pool_state: &'me AccountInfo<'info>,
    pub token_0_vault: &'me AccountInfo<'info>,
    pub token_1_vault: &'me AccountInfo<'info>,
    pub bonding_curve_vault: &'me AccountInfo<'info>,
    pub bonding_curve_wsol_vault: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub token_program_2022: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
    pub lp_mint: &'me AccountInfo<'info>,
    pub cpmm_program: &'me AccountInfo<'info>,
    pub owner_lp_token: &'me AccountInfo<'info>,
    pub bonding_curve: &'me AccountInfo<'info>,
    pub token_0_mint: &'me AccountInfo<'info>,
    pub token_1_mint: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct DepositIntoRaydiumKeys {
    pub config: Pubkey,
    pub amm_config: Pubkey,
    pub operator: Pubkey,
    pub operator_wsol_account: Pubkey,
    pub vault_authority: Pubkey,
    pub authority: Pubkey,
    pub pool_state: Pubkey,
    pub token_0_vault: Pubkey,
    pub token_1_vault: Pubkey,
    pub bonding_curve_vault: Pubkey,
    pub bonding_curve_wsol_vault: Pubkey,
    pub token_program: Pubkey,
    pub token_program_2022: Pubkey,
    pub system_program: Pubkey,
    pub associated_token_program: Pubkey,
    pub lp_mint: Pubkey,
    pub cpmm_program: Pubkey,
    pub owner_lp_token: Pubkey,
    pub bonding_curve: Pubkey,
    pub token_0_mint: Pubkey,
    pub token_1_mint: Pubkey,
}
impl From<DepositIntoRaydiumAccounts<'_, '_>> for DepositIntoRaydiumKeys {
    fn from(accounts: DepositIntoRaydiumAccounts) -> Self {
        Self {
            config: *accounts.config.key,
            amm_config: *accounts.amm_config.key,
            operator: *accounts.operator.key,
            operator_wsol_account: *accounts.operator_wsol_account.key,
            vault_authority: *accounts.vault_authority.key,
            authority: *accounts.authority.key,
            pool_state: *accounts.pool_state.key,
            token_0_vault: *accounts.token_0_vault.key,
            token_1_vault: *accounts.token_1_vault.key,
            bonding_curve_vault: *accounts.bonding_curve_vault.key,
            bonding_curve_wsol_vault: *accounts.bonding_curve_wsol_vault.key,
            token_program: *accounts.token_program.key,
            token_program_2022: *accounts.token_program_2022.key,
            system_program: *accounts.system_program.key,
            associated_token_program: *accounts.associated_token_program.key,
            lp_mint: *accounts.lp_mint.key,
            cpmm_program: *accounts.cpmm_program.key,
            owner_lp_token: *accounts.owner_lp_token.key,
            bonding_curve: *accounts.bonding_curve.key,
            token_0_mint: *accounts.token_0_mint.key,
            token_1_mint: *accounts.token_1_mint.key,
        }
    }
}
impl From<DepositIntoRaydiumKeys>
for [AccountMeta; DEPOSIT_INTO_RAYDIUM_IX_ACCOUNTS_LEN] {
    fn from(keys: DepositIntoRaydiumKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.amm_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.operator,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.operator_wsol_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vault_authority,
                is_signer: false,
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
                pubkey: keys.bonding_curve_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.bonding_curve_wsol_vault,
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
                pubkey: keys.lp_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.cpmm_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.owner_lp_token,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.bonding_curve,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_0_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_1_mint,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; DEPOSIT_INTO_RAYDIUM_IX_ACCOUNTS_LEN]> for DepositIntoRaydiumKeys {
    fn from(pubkeys: [Pubkey; DEPOSIT_INTO_RAYDIUM_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            config: pubkeys[0],
            amm_config: pubkeys[1],
            operator: pubkeys[2],
            operator_wsol_account: pubkeys[3],
            vault_authority: pubkeys[4],
            authority: pubkeys[5],
            pool_state: pubkeys[6],
            token_0_vault: pubkeys[7],
            token_1_vault: pubkeys[8],
            bonding_curve_vault: pubkeys[9],
            bonding_curve_wsol_vault: pubkeys[10],
            token_program: pubkeys[11],
            token_program_2022: pubkeys[12],
            system_program: pubkeys[13],
            associated_token_program: pubkeys[14],
            lp_mint: pubkeys[15],
            cpmm_program: pubkeys[16],
            owner_lp_token: pubkeys[17],
            bonding_curve: pubkeys[18],
            token_0_mint: pubkeys[19],
            token_1_mint: pubkeys[20],
        }
    }
}
impl<'info> From<DepositIntoRaydiumAccounts<'_, 'info>>
for [AccountInfo<'info>; DEPOSIT_INTO_RAYDIUM_IX_ACCOUNTS_LEN] {
    fn from(accounts: DepositIntoRaydiumAccounts<'_, 'info>) -> Self {
        [
            accounts.config.clone(),
            accounts.amm_config.clone(),
            accounts.operator.clone(),
            accounts.operator_wsol_account.clone(),
            accounts.vault_authority.clone(),
            accounts.authority.clone(),
            accounts.pool_state.clone(),
            accounts.token_0_vault.clone(),
            accounts.token_1_vault.clone(),
            accounts.bonding_curve_vault.clone(),
            accounts.bonding_curve_wsol_vault.clone(),
            accounts.token_program.clone(),
            accounts.token_program_2022.clone(),
            accounts.system_program.clone(),
            accounts.associated_token_program.clone(),
            accounts.lp_mint.clone(),
            accounts.cpmm_program.clone(),
            accounts.owner_lp_token.clone(),
            accounts.bonding_curve.clone(),
            accounts.token_0_mint.clone(),
            accounts.token_1_mint.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; DEPOSIT_INTO_RAYDIUM_IX_ACCOUNTS_LEN]>
for DepositIntoRaydiumAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; DEPOSIT_INTO_RAYDIUM_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            config: &arr[0],
            amm_config: &arr[1],
            operator: &arr[2],
            operator_wsol_account: &arr[3],
            vault_authority: &arr[4],
            authority: &arr[5],
            pool_state: &arr[6],
            token_0_vault: &arr[7],
            token_1_vault: &arr[8],
            bonding_curve_vault: &arr[9],
            bonding_curve_wsol_vault: &arr[10],
            token_program: &arr[11],
            token_program_2022: &arr[12],
            system_program: &arr[13],
            associated_token_program: &arr[14],
            lp_mint: &arr[15],
            cpmm_program: &arr[16],
            owner_lp_token: &arr[17],
            bonding_curve: &arr[18],
            token_0_mint: &arr[19],
            token_1_mint: &arr[20],
        }
    }
}
pub const DEPOSIT_INTO_RAYDIUM_IX_DISCM: [u8; 8usize] = [
    168, 89, 99, 30, 117, 49, 88, 224,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DepositIntoRaydiumIxArgs {
    pub lp_token_amount: u64,
    pub maximum_token_0_amount: u64,
    pub maximum_token_1_amount: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct DepositIntoRaydiumIxData(pub DepositIntoRaydiumIxArgs);
impl From<DepositIntoRaydiumIxArgs> for DepositIntoRaydiumIxData {
    fn from(args: DepositIntoRaydiumIxArgs) -> Self {
        Self(args)
    }
}
impl DepositIntoRaydiumIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != DEPOSIT_INTO_RAYDIUM_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let lp_token_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let maximum_token_0_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let maximum_token_1_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(DepositIntoRaydiumIxArgs {
                lp_token_amount,
                maximum_token_0_amount,
                maximum_token_1_amount,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&DEPOSIT_INTO_RAYDIUM_IX_DISCM)?;
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
pub fn deposit_into_raydium_ix_with_program_id(
    program_id: Pubkey,
    keys: DepositIntoRaydiumKeys,
    args: DepositIntoRaydiumIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; DEPOSIT_INTO_RAYDIUM_IX_ACCOUNTS_LEN] = keys.into();
    let data: DepositIntoRaydiumIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn deposit_into_raydium_ix(
    keys: DepositIntoRaydiumKeys,
    args: DepositIntoRaydiumIxArgs,
) -> std::io::Result<Instruction> {
    deposit_into_raydium_ix_with_program_id(BOOP_PROGRAM_ID, keys, args)
}
pub fn deposit_into_raydium_invoke_with_program_id(
    program_id: Pubkey,
    accounts: DepositIntoRaydiumAccounts<'_, '_>,
    args: DepositIntoRaydiumIxArgs,
) -> ProgramResult {
    let keys: DepositIntoRaydiumKeys = accounts.into();
    let ix = deposit_into_raydium_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn deposit_into_raydium_invoke(
    accounts: DepositIntoRaydiumAccounts<'_, '_>,
    args: DepositIntoRaydiumIxArgs,
) -> ProgramResult {
    deposit_into_raydium_invoke_with_program_id(BOOP_PROGRAM_ID, accounts, args)
}
pub fn deposit_into_raydium_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: DepositIntoRaydiumAccounts<'_, '_>,
    args: DepositIntoRaydiumIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: DepositIntoRaydiumKeys = accounts.into();
    let ix = deposit_into_raydium_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn deposit_into_raydium_invoke_signed(
    accounts: DepositIntoRaydiumAccounts<'_, '_>,
    args: DepositIntoRaydiumIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    deposit_into_raydium_invoke_signed_with_program_id(
        BOOP_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn deposit_into_raydium_verify_account_keys(
    accounts: DepositIntoRaydiumAccounts<'_, '_>,
    keys: DepositIntoRaydiumKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.config.key, keys.config),
        (*accounts.amm_config.key, keys.amm_config),
        (*accounts.operator.key, keys.operator),
        (*accounts.operator_wsol_account.key, keys.operator_wsol_account),
        (*accounts.vault_authority.key, keys.vault_authority),
        (*accounts.authority.key, keys.authority),
        (*accounts.pool_state.key, keys.pool_state),
        (*accounts.token_0_vault.key, keys.token_0_vault),
        (*accounts.token_1_vault.key, keys.token_1_vault),
        (*accounts.bonding_curve_vault.key, keys.bonding_curve_vault),
        (*accounts.bonding_curve_wsol_vault.key, keys.bonding_curve_wsol_vault),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.token_program_2022.key, keys.token_program_2022),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.associated_token_program.key, keys.associated_token_program),
        (*accounts.lp_mint.key, keys.lp_mint),
        (*accounts.cpmm_program.key, keys.cpmm_program),
        (*accounts.owner_lp_token.key, keys.owner_lp_token),
        (*accounts.bonding_curve.key, keys.bonding_curve),
        (*accounts.token_0_mint.key, keys.token_0_mint),
        (*accounts.token_1_mint.key, keys.token_1_mint),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn deposit_into_raydium_verify_writable_privileges<'me, 'info>(
    accounts: DepositIntoRaydiumAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.operator,
        accounts.operator_wsol_account,
        accounts.vault_authority,
        accounts.pool_state,
        accounts.token_0_vault,
        accounts.token_1_vault,
        accounts.bonding_curve_vault,
        accounts.bonding_curve_wsol_vault,
        accounts.lp_mint,
        accounts.owner_lp_token,
        accounts.bonding_curve,
        accounts.token_0_mint,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn deposit_into_raydium_verify_signer_privileges<'me, 'info>(
    accounts: DepositIntoRaydiumAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.operator] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn deposit_into_raydium_verify_account_privileges<'me, 'info>(
    accounts: DepositIntoRaydiumAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    deposit_into_raydium_verify_writable_privileges(accounts)?;
    deposit_into_raydium_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const GRADUATE_IX_ACCOUNTS_LEN: usize = 15;
#[derive(Copy, Clone, Debug)]
pub struct GraduateAccounts<'me, 'info> {
    pub mint: &'me AccountInfo<'info>,
    pub wsol: &'me AccountInfo<'info>,
    pub protocol_fee_recipient: &'me AccountInfo<'info>,
    pub token_distributor: &'me AccountInfo<'info>,
    pub token_distributor_token_account: &'me AccountInfo<'info>,
    pub vault_authority: &'me AccountInfo<'info>,
    pub bonding_curve_sol_vault: &'me AccountInfo<'info>,
    pub bonding_curve: &'me AccountInfo<'info>,
    pub bonding_curve_vault: &'me AccountInfo<'info>,
    pub bonding_curve_wsol_account: &'me AccountInfo<'info>,
    pub operator: &'me AccountInfo<'info>,
    pub config: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GraduateKeys {
    pub mint: Pubkey,
    pub wsol: Pubkey,
    pub protocol_fee_recipient: Pubkey,
    pub token_distributor: Pubkey,
    pub token_distributor_token_account: Pubkey,
    pub vault_authority: Pubkey,
    pub bonding_curve_sol_vault: Pubkey,
    pub bonding_curve: Pubkey,
    pub bonding_curve_vault: Pubkey,
    pub bonding_curve_wsol_account: Pubkey,
    pub operator: Pubkey,
    pub config: Pubkey,
    pub system_program: Pubkey,
    pub token_program: Pubkey,
    pub associated_token_program: Pubkey,
}
impl From<GraduateAccounts<'_, '_>> for GraduateKeys {
    fn from(accounts: GraduateAccounts) -> Self {
        Self {
            mint: *accounts.mint.key,
            wsol: *accounts.wsol.key,
            protocol_fee_recipient: *accounts.protocol_fee_recipient.key,
            token_distributor: *accounts.token_distributor.key,
            token_distributor_token_account: *accounts
                .token_distributor_token_account
                .key,
            vault_authority: *accounts.vault_authority.key,
            bonding_curve_sol_vault: *accounts.bonding_curve_sol_vault.key,
            bonding_curve: *accounts.bonding_curve.key,
            bonding_curve_vault: *accounts.bonding_curve_vault.key,
            bonding_curve_wsol_account: *accounts.bonding_curve_wsol_account.key,
            operator: *accounts.operator.key,
            config: *accounts.config.key,
            system_program: *accounts.system_program.key,
            token_program: *accounts.token_program.key,
            associated_token_program: *accounts.associated_token_program.key,
        }
    }
}
impl From<GraduateKeys> for [AccountMeta; GRADUATE_IX_ACCOUNTS_LEN] {
    fn from(keys: GraduateKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.wsol,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.protocol_fee_recipient,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_distributor,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_distributor_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vault_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.bonding_curve_sol_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.bonding_curve,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.bonding_curve_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.bonding_curve_wsol_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.operator,
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
impl From<[Pubkey; GRADUATE_IX_ACCOUNTS_LEN]> for GraduateKeys {
    fn from(pubkeys: [Pubkey; GRADUATE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            mint: pubkeys[0],
            wsol: pubkeys[1],
            protocol_fee_recipient: pubkeys[2],
            token_distributor: pubkeys[3],
            token_distributor_token_account: pubkeys[4],
            vault_authority: pubkeys[5],
            bonding_curve_sol_vault: pubkeys[6],
            bonding_curve: pubkeys[7],
            bonding_curve_vault: pubkeys[8],
            bonding_curve_wsol_account: pubkeys[9],
            operator: pubkeys[10],
            config: pubkeys[11],
            system_program: pubkeys[12],
            token_program: pubkeys[13],
            associated_token_program: pubkeys[14],
        }
    }
}
impl<'info> From<GraduateAccounts<'_, 'info>>
for [AccountInfo<'info>; GRADUATE_IX_ACCOUNTS_LEN] {
    fn from(accounts: GraduateAccounts<'_, 'info>) -> Self {
        [
            accounts.mint.clone(),
            accounts.wsol.clone(),
            accounts.protocol_fee_recipient.clone(),
            accounts.token_distributor.clone(),
            accounts.token_distributor_token_account.clone(),
            accounts.vault_authority.clone(),
            accounts.bonding_curve_sol_vault.clone(),
            accounts.bonding_curve.clone(),
            accounts.bonding_curve_vault.clone(),
            accounts.bonding_curve_wsol_account.clone(),
            accounts.operator.clone(),
            accounts.config.clone(),
            accounts.system_program.clone(),
            accounts.token_program.clone(),
            accounts.associated_token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; GRADUATE_IX_ACCOUNTS_LEN]>
for GraduateAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; GRADUATE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            mint: &arr[0],
            wsol: &arr[1],
            protocol_fee_recipient: &arr[2],
            token_distributor: &arr[3],
            token_distributor_token_account: &arr[4],
            vault_authority: &arr[5],
            bonding_curve_sol_vault: &arr[6],
            bonding_curve: &arr[7],
            bonding_curve_vault: &arr[8],
            bonding_curve_wsol_account: &arr[9],
            operator: &arr[10],
            config: &arr[11],
            system_program: &arr[12],
            token_program: &arr[13],
            associated_token_program: &arr[14],
        }
    }
}
pub const GRADUATE_IX_DISCM: [u8; 8usize] = [45, 235, 225, 181, 17, 218, 64, 130];
#[derive(Clone, Debug, PartialEq)]
pub struct GraduateIxData;
impl GraduateIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != GRADUATE_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&GRADUATE_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn graduate_ix_with_program_id(
    program_id: Pubkey,
    keys: GraduateKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; GRADUATE_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: GraduateIxData.try_to_vec()?,
    })
}
pub fn graduate_ix(keys: GraduateKeys) -> std::io::Result<Instruction> {
    graduate_ix_with_program_id(BOOP_PROGRAM_ID, keys)
}
pub fn graduate_invoke_with_program_id(
    program_id: Pubkey,
    accounts: GraduateAccounts<'_, '_>,
) -> ProgramResult {
    let keys: GraduateKeys = accounts.into();
    let ix = graduate_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn graduate_invoke(accounts: GraduateAccounts<'_, '_>) -> ProgramResult {
    graduate_invoke_with_program_id(BOOP_PROGRAM_ID, accounts)
}
pub fn graduate_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: GraduateAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: GraduateKeys = accounts.into();
    let ix = graduate_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn graduate_invoke_signed(
    accounts: GraduateAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    graduate_invoke_signed_with_program_id(BOOP_PROGRAM_ID, accounts, seeds)
}
pub fn graduate_verify_account_keys(
    accounts: GraduateAccounts<'_, '_>,
    keys: GraduateKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.mint.key, keys.mint),
        (*accounts.wsol.key, keys.wsol),
        (*accounts.protocol_fee_recipient.key, keys.protocol_fee_recipient),
        (*accounts.token_distributor.key, keys.token_distributor),
        (
            *accounts.token_distributor_token_account.key,
            keys.token_distributor_token_account,
        ),
        (*accounts.vault_authority.key, keys.vault_authority),
        (*accounts.bonding_curve_sol_vault.key, keys.bonding_curve_sol_vault),
        (*accounts.bonding_curve.key, keys.bonding_curve),
        (*accounts.bonding_curve_vault.key, keys.bonding_curve_vault),
        (*accounts.bonding_curve_wsol_account.key, keys.bonding_curve_wsol_account),
        (*accounts.operator.key, keys.operator),
        (*accounts.config.key, keys.config),
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
pub fn graduate_verify_writable_privileges<'me, 'info>(
    accounts: GraduateAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.protocol_fee_recipient,
        accounts.token_distributor_token_account,
        accounts.bonding_curve_sol_vault,
        accounts.bonding_curve,
        accounts.bonding_curve_vault,
        accounts.bonding_curve_wsol_account,
        accounts.operator,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn graduate_verify_signer_privileges<'me, 'info>(
    accounts: GraduateAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.operator] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn graduate_verify_account_privileges<'me, 'info>(
    accounts: GraduateAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    graduate_verify_writable_privileges(accounts)?;
    graduate_verify_signer_privileges(accounts)?;
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
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct InitializeIxArgs {
    pub protocol_fee_recipient: Pubkey,
    pub token_distributor: Pubkey,
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
        let protocol_fee_recipient: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let token_distributor: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(InitializeIxArgs {
                protocol_fee_recipient,
                token_distributor,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INITIALIZE_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.protocol_fee_recipient, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.token_distributor, &mut writer)?;
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
    initialize_ix_with_program_id(BOOP_PROGRAM_ID, keys, args)
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
    initialize_invoke_with_program_id(BOOP_PROGRAM_ID, accounts, args)
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
    initialize_invoke_signed_with_program_id(BOOP_PROGRAM_ID, accounts, args, seeds)
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
pub const INITIATE_AUTHORITY_TRANSFER_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct InitiateAuthorityTransferAccounts<'me, 'info> {
    pub authority: &'me AccountInfo<'info>,
    pub config: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct InitiateAuthorityTransferKeys {
    pub authority: Pubkey,
    pub config: Pubkey,
    pub system_program: Pubkey,
}
impl From<InitiateAuthorityTransferAccounts<'_, '_>> for InitiateAuthorityTransferKeys {
    fn from(accounts: InitiateAuthorityTransferAccounts) -> Self {
        Self {
            authority: *accounts.authority.key,
            config: *accounts.config.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<InitiateAuthorityTransferKeys>
for [AccountMeta; INITIATE_AUTHORITY_TRANSFER_IX_ACCOUNTS_LEN] {
    fn from(keys: InitiateAuthorityTransferKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.authority,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.config,
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
impl From<[Pubkey; INITIATE_AUTHORITY_TRANSFER_IX_ACCOUNTS_LEN]>
for InitiateAuthorityTransferKeys {
    fn from(pubkeys: [Pubkey; INITIATE_AUTHORITY_TRANSFER_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            authority: pubkeys[0],
            config: pubkeys[1],
            system_program: pubkeys[2],
        }
    }
}
impl<'info> From<InitiateAuthorityTransferAccounts<'_, 'info>>
for [AccountInfo<'info>; INITIATE_AUTHORITY_TRANSFER_IX_ACCOUNTS_LEN] {
    fn from(accounts: InitiateAuthorityTransferAccounts<'_, 'info>) -> Self {
        [
            accounts.authority.clone(),
            accounts.config.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; INITIATE_AUTHORITY_TRANSFER_IX_ACCOUNTS_LEN]>
for InitiateAuthorityTransferAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; INITIATE_AUTHORITY_TRANSFER_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            authority: &arr[0],
            config: &arr[1],
            system_program: &arr[2],
        }
    }
}
pub const INITIATE_AUTHORITY_TRANSFER_IX_DISCM: [u8; 8usize] = [
    210, 43, 101, 215, 119, 140, 106, 218,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct InitiateAuthorityTransferIxArgs {
    pub new_authority: Pubkey,
}
#[derive(Clone, Debug, PartialEq)]
pub struct InitiateAuthorityTransferIxData(pub InitiateAuthorityTransferIxArgs);
impl From<InitiateAuthorityTransferIxArgs> for InitiateAuthorityTransferIxData {
    fn from(args: InitiateAuthorityTransferIxArgs) -> Self {
        Self(args)
    }
}
impl InitiateAuthorityTransferIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INITIATE_AUTHORITY_TRANSFER_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let new_authority: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(InitiateAuthorityTransferIxArgs {
                new_authority,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INITIATE_AUTHORITY_TRANSFER_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.new_authority, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn initiate_authority_transfer_ix_with_program_id(
    program_id: Pubkey,
    keys: InitiateAuthorityTransferKeys,
    args: InitiateAuthorityTransferIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; INITIATE_AUTHORITY_TRANSFER_IX_ACCOUNTS_LEN] = keys.into();
    let data: InitiateAuthorityTransferIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn initiate_authority_transfer_ix(
    keys: InitiateAuthorityTransferKeys,
    args: InitiateAuthorityTransferIxArgs,
) -> std::io::Result<Instruction> {
    initiate_authority_transfer_ix_with_program_id(BOOP_PROGRAM_ID, keys, args)
}
pub fn initiate_authority_transfer_invoke_with_program_id(
    program_id: Pubkey,
    accounts: InitiateAuthorityTransferAccounts<'_, '_>,
    args: InitiateAuthorityTransferIxArgs,
) -> ProgramResult {
    let keys: InitiateAuthorityTransferKeys = accounts.into();
    let ix = initiate_authority_transfer_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn initiate_authority_transfer_invoke(
    accounts: InitiateAuthorityTransferAccounts<'_, '_>,
    args: InitiateAuthorityTransferIxArgs,
) -> ProgramResult {
    initiate_authority_transfer_invoke_with_program_id(BOOP_PROGRAM_ID, accounts, args)
}
pub fn initiate_authority_transfer_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: InitiateAuthorityTransferAccounts<'_, '_>,
    args: InitiateAuthorityTransferIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: InitiateAuthorityTransferKeys = accounts.into();
    let ix = initiate_authority_transfer_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn initiate_authority_transfer_invoke_signed(
    accounts: InitiateAuthorityTransferAccounts<'_, '_>,
    args: InitiateAuthorityTransferIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    initiate_authority_transfer_invoke_signed_with_program_id(
        BOOP_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn initiate_authority_transfer_verify_account_keys(
    accounts: InitiateAuthorityTransferAccounts<'_, '_>,
    keys: InitiateAuthorityTransferKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.authority.key, keys.authority),
        (*accounts.config.key, keys.config),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn initiate_authority_transfer_verify_writable_privileges<'me, 'info>(
    accounts: InitiateAuthorityTransferAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.config] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn initiate_authority_transfer_verify_signer_privileges<'me, 'info>(
    accounts: InitiateAuthorityTransferAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn initiate_authority_transfer_verify_account_privileges<'me, 'info>(
    accounts: InitiateAuthorityTransferAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    initiate_authority_transfer_verify_writable_privileges(accounts)?;
    initiate_authority_transfer_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const LOCK_RAYDIUM_LIQUIDITY_IX_ACCOUNTS_LEN: usize = 22;
#[derive(Copy, Clone, Debug)]
pub struct LockRaydiumLiquidityAccounts<'me, 'info> {
    pub lock_program: &'me AccountInfo<'info>,
    pub vault_authority: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub fee_nft_owner: &'me AccountInfo<'info>,
    pub fee_nft_mint: &'me AccountInfo<'info>,
    pub fee_nft_account: &'me AccountInfo<'info>,
    pub pool_state: &'me AccountInfo<'info>,
    pub locked_liquidity: &'me AccountInfo<'info>,
    pub lp_mint: &'me AccountInfo<'info>,
    pub liquidity_owner_lp: &'me AccountInfo<'info>,
    pub locked_lp_vault: &'me AccountInfo<'info>,
    pub token_0_vault: &'me AccountInfo<'info>,
    pub token_1_vault: &'me AccountInfo<'info>,
    pub operator: &'me AccountInfo<'info>,
    pub config: &'me AccountInfo<'info>,
    pub bonding_curve: &'me AccountInfo<'info>,
    pub metadata_account: &'me AccountInfo<'info>,
    pub rent: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
    pub metadata_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LockRaydiumLiquidityKeys {
    pub lock_program: Pubkey,
    pub vault_authority: Pubkey,
    pub authority: Pubkey,
    pub fee_nft_owner: Pubkey,
    pub fee_nft_mint: Pubkey,
    pub fee_nft_account: Pubkey,
    pub pool_state: Pubkey,
    pub locked_liquidity: Pubkey,
    pub lp_mint: Pubkey,
    pub liquidity_owner_lp: Pubkey,
    pub locked_lp_vault: Pubkey,
    pub token_0_vault: Pubkey,
    pub token_1_vault: Pubkey,
    pub operator: Pubkey,
    pub config: Pubkey,
    pub bonding_curve: Pubkey,
    pub metadata_account: Pubkey,
    pub rent: Pubkey,
    pub system_program: Pubkey,
    pub token_program: Pubkey,
    pub associated_token_program: Pubkey,
    pub metadata_program: Pubkey,
}
impl From<LockRaydiumLiquidityAccounts<'_, '_>> for LockRaydiumLiquidityKeys {
    fn from(accounts: LockRaydiumLiquidityAccounts) -> Self {
        Self {
            lock_program: *accounts.lock_program.key,
            vault_authority: *accounts.vault_authority.key,
            authority: *accounts.authority.key,
            fee_nft_owner: *accounts.fee_nft_owner.key,
            fee_nft_mint: *accounts.fee_nft_mint.key,
            fee_nft_account: *accounts.fee_nft_account.key,
            pool_state: *accounts.pool_state.key,
            locked_liquidity: *accounts.locked_liquidity.key,
            lp_mint: *accounts.lp_mint.key,
            liquidity_owner_lp: *accounts.liquidity_owner_lp.key,
            locked_lp_vault: *accounts.locked_lp_vault.key,
            token_0_vault: *accounts.token_0_vault.key,
            token_1_vault: *accounts.token_1_vault.key,
            operator: *accounts.operator.key,
            config: *accounts.config.key,
            bonding_curve: *accounts.bonding_curve.key,
            metadata_account: *accounts.metadata_account.key,
            rent: *accounts.rent.key,
            system_program: *accounts.system_program.key,
            token_program: *accounts.token_program.key,
            associated_token_program: *accounts.associated_token_program.key,
            metadata_program: *accounts.metadata_program.key,
        }
    }
}
impl From<LockRaydiumLiquidityKeys>
for [AccountMeta; LOCK_RAYDIUM_LIQUIDITY_IX_ACCOUNTS_LEN] {
    fn from(keys: LockRaydiumLiquidityKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.lock_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault_authority,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.authority,
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
                pubkey: keys.pool_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.locked_liquidity,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.lp_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.liquidity_owner_lp,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.locked_lp_vault,
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
                pubkey: keys.operator,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.bonding_curve,
                is_signer: false,
                is_writable: true,
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
                pubkey: keys.metadata_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; LOCK_RAYDIUM_LIQUIDITY_IX_ACCOUNTS_LEN]>
for LockRaydiumLiquidityKeys {
    fn from(pubkeys: [Pubkey; LOCK_RAYDIUM_LIQUIDITY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            lock_program: pubkeys[0],
            vault_authority: pubkeys[1],
            authority: pubkeys[2],
            fee_nft_owner: pubkeys[3],
            fee_nft_mint: pubkeys[4],
            fee_nft_account: pubkeys[5],
            pool_state: pubkeys[6],
            locked_liquidity: pubkeys[7],
            lp_mint: pubkeys[8],
            liquidity_owner_lp: pubkeys[9],
            locked_lp_vault: pubkeys[10],
            token_0_vault: pubkeys[11],
            token_1_vault: pubkeys[12],
            operator: pubkeys[13],
            config: pubkeys[14],
            bonding_curve: pubkeys[15],
            metadata_account: pubkeys[16],
            rent: pubkeys[17],
            system_program: pubkeys[18],
            token_program: pubkeys[19],
            associated_token_program: pubkeys[20],
            metadata_program: pubkeys[21],
        }
    }
}
impl<'info> From<LockRaydiumLiquidityAccounts<'_, 'info>>
for [AccountInfo<'info>; LOCK_RAYDIUM_LIQUIDITY_IX_ACCOUNTS_LEN] {
    fn from(accounts: LockRaydiumLiquidityAccounts<'_, 'info>) -> Self {
        [
            accounts.lock_program.clone(),
            accounts.vault_authority.clone(),
            accounts.authority.clone(),
            accounts.fee_nft_owner.clone(),
            accounts.fee_nft_mint.clone(),
            accounts.fee_nft_account.clone(),
            accounts.pool_state.clone(),
            accounts.locked_liquidity.clone(),
            accounts.lp_mint.clone(),
            accounts.liquidity_owner_lp.clone(),
            accounts.locked_lp_vault.clone(),
            accounts.token_0_vault.clone(),
            accounts.token_1_vault.clone(),
            accounts.operator.clone(),
            accounts.config.clone(),
            accounts.bonding_curve.clone(),
            accounts.metadata_account.clone(),
            accounts.rent.clone(),
            accounts.system_program.clone(),
            accounts.token_program.clone(),
            accounts.associated_token_program.clone(),
            accounts.metadata_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; LOCK_RAYDIUM_LIQUIDITY_IX_ACCOUNTS_LEN]>
for LockRaydiumLiquidityAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; LOCK_RAYDIUM_LIQUIDITY_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            lock_program: &arr[0],
            vault_authority: &arr[1],
            authority: &arr[2],
            fee_nft_owner: &arr[3],
            fee_nft_mint: &arr[4],
            fee_nft_account: &arr[5],
            pool_state: &arr[6],
            locked_liquidity: &arr[7],
            lp_mint: &arr[8],
            liquidity_owner_lp: &arr[9],
            locked_lp_vault: &arr[10],
            token_0_vault: &arr[11],
            token_1_vault: &arr[12],
            operator: &arr[13],
            config: &arr[14],
            bonding_curve: &arr[15],
            metadata_account: &arr[16],
            rent: &arr[17],
            system_program: &arr[18],
            token_program: &arr[19],
            associated_token_program: &arr[20],
            metadata_program: &arr[21],
        }
    }
}
pub const LOCK_RAYDIUM_LIQUIDITY_IX_DISCM: [u8; 8usize] = [
    173, 255, 148, 6, 122, 99, 140, 22,
];
#[derive(Clone, Debug, PartialEq)]
pub struct LockRaydiumLiquidityIxData;
impl LockRaydiumLiquidityIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != LOCK_RAYDIUM_LIQUIDITY_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&LOCK_RAYDIUM_LIQUIDITY_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn lock_raydium_liquidity_ix_with_program_id(
    program_id: Pubkey,
    keys: LockRaydiumLiquidityKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; LOCK_RAYDIUM_LIQUIDITY_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: LockRaydiumLiquidityIxData.try_to_vec()?,
    })
}
pub fn lock_raydium_liquidity_ix(
    keys: LockRaydiumLiquidityKeys,
) -> std::io::Result<Instruction> {
    lock_raydium_liquidity_ix_with_program_id(BOOP_PROGRAM_ID, keys)
}
pub fn lock_raydium_liquidity_invoke_with_program_id(
    program_id: Pubkey,
    accounts: LockRaydiumLiquidityAccounts<'_, '_>,
) -> ProgramResult {
    let keys: LockRaydiumLiquidityKeys = accounts.into();
    let ix = lock_raydium_liquidity_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn lock_raydium_liquidity_invoke(
    accounts: LockRaydiumLiquidityAccounts<'_, '_>,
) -> ProgramResult {
    lock_raydium_liquidity_invoke_with_program_id(BOOP_PROGRAM_ID, accounts)
}
pub fn lock_raydium_liquidity_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: LockRaydiumLiquidityAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: LockRaydiumLiquidityKeys = accounts.into();
    let ix = lock_raydium_liquidity_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn lock_raydium_liquidity_invoke_signed(
    accounts: LockRaydiumLiquidityAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    lock_raydium_liquidity_invoke_signed_with_program_id(
        BOOP_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn lock_raydium_liquidity_verify_account_keys(
    accounts: LockRaydiumLiquidityAccounts<'_, '_>,
    keys: LockRaydiumLiquidityKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.lock_program.key, keys.lock_program),
        (*accounts.vault_authority.key, keys.vault_authority),
        (*accounts.authority.key, keys.authority),
        (*accounts.fee_nft_owner.key, keys.fee_nft_owner),
        (*accounts.fee_nft_mint.key, keys.fee_nft_mint),
        (*accounts.fee_nft_account.key, keys.fee_nft_account),
        (*accounts.pool_state.key, keys.pool_state),
        (*accounts.locked_liquidity.key, keys.locked_liquidity),
        (*accounts.lp_mint.key, keys.lp_mint),
        (*accounts.liquidity_owner_lp.key, keys.liquidity_owner_lp),
        (*accounts.locked_lp_vault.key, keys.locked_lp_vault),
        (*accounts.token_0_vault.key, keys.token_0_vault),
        (*accounts.token_1_vault.key, keys.token_1_vault),
        (*accounts.operator.key, keys.operator),
        (*accounts.config.key, keys.config),
        (*accounts.bonding_curve.key, keys.bonding_curve),
        (*accounts.metadata_account.key, keys.metadata_account),
        (*accounts.rent.key, keys.rent),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.associated_token_program.key, keys.associated_token_program),
        (*accounts.metadata_program.key, keys.metadata_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn lock_raydium_liquidity_verify_writable_privileges<'me, 'info>(
    accounts: LockRaydiumLiquidityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.vault_authority,
        accounts.fee_nft_mint,
        accounts.fee_nft_account,
        accounts.pool_state,
        accounts.locked_liquidity,
        accounts.lp_mint,
        accounts.liquidity_owner_lp,
        accounts.locked_lp_vault,
        accounts.token_0_vault,
        accounts.token_1_vault,
        accounts.operator,
        accounts.bonding_curve,
        accounts.metadata_account,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn lock_raydium_liquidity_verify_signer_privileges<'me, 'info>(
    accounts: LockRaydiumLiquidityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.fee_nft_mint, accounts.operator] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn lock_raydium_liquidity_verify_account_privileges<'me, 'info>(
    accounts: LockRaydiumLiquidityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    lock_raydium_liquidity_verify_writable_privileges(accounts)?;
    lock_raydium_liquidity_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const REMOVE_OPERATORS_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct RemoveOperatorsAccounts<'me, 'info> {
    pub config: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct RemoveOperatorsKeys {
    pub config: Pubkey,
    pub authority: Pubkey,
    pub system_program: Pubkey,
}
impl From<RemoveOperatorsAccounts<'_, '_>> for RemoveOperatorsKeys {
    fn from(accounts: RemoveOperatorsAccounts) -> Self {
        Self {
            config: *accounts.config.key,
            authority: *accounts.authority.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<RemoveOperatorsKeys> for [AccountMeta; REMOVE_OPERATORS_IX_ACCOUNTS_LEN] {
    fn from(keys: RemoveOperatorsKeys) -> Self {
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
impl From<[Pubkey; REMOVE_OPERATORS_IX_ACCOUNTS_LEN]> for RemoveOperatorsKeys {
    fn from(pubkeys: [Pubkey; REMOVE_OPERATORS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            config: pubkeys[0],
            authority: pubkeys[1],
            system_program: pubkeys[2],
        }
    }
}
impl<'info> From<RemoveOperatorsAccounts<'_, 'info>>
for [AccountInfo<'info>; REMOVE_OPERATORS_IX_ACCOUNTS_LEN] {
    fn from(accounts: RemoveOperatorsAccounts<'_, 'info>) -> Self {
        [
            accounts.config.clone(),
            accounts.authority.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; REMOVE_OPERATORS_IX_ACCOUNTS_LEN]>
for RemoveOperatorsAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; REMOVE_OPERATORS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            config: &arr[0],
            authority: &arr[1],
            system_program: &arr[2],
        }
    }
}
pub const REMOVE_OPERATORS_IX_DISCM: [u8; 8usize] = [42, 20, 89, 83, 222, 37, 4, 109];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct RemoveOperatorsIxArgs {
    pub operators: Vec<Pubkey>,
}
#[derive(Clone, Debug, PartialEq)]
pub struct RemoveOperatorsIxData(pub RemoveOperatorsIxArgs);
impl From<RemoveOperatorsIxArgs> for RemoveOperatorsIxData {
    fn from(args: RemoveOperatorsIxArgs) -> Self {
        Self(args)
    }
}
impl RemoveOperatorsIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != REMOVE_OPERATORS_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let operators: Vec<Pubkey> = crate::borsh_de_or_default(&mut reader)?;
        Ok(Self(RemoveOperatorsIxArgs { operators }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&REMOVE_OPERATORS_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.operators, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn remove_operators_ix_with_program_id(
    program_id: Pubkey,
    keys: RemoveOperatorsKeys,
    args: RemoveOperatorsIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; REMOVE_OPERATORS_IX_ACCOUNTS_LEN] = keys.into();
    let data: RemoveOperatorsIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn remove_operators_ix(
    keys: RemoveOperatorsKeys,
    args: RemoveOperatorsIxArgs,
) -> std::io::Result<Instruction> {
    remove_operators_ix_with_program_id(BOOP_PROGRAM_ID, keys, args)
}
pub fn remove_operators_invoke_with_program_id(
    program_id: Pubkey,
    accounts: RemoveOperatorsAccounts<'_, '_>,
    args: RemoveOperatorsIxArgs,
) -> ProgramResult {
    let keys: RemoveOperatorsKeys = accounts.into();
    let ix = remove_operators_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn remove_operators_invoke(
    accounts: RemoveOperatorsAccounts<'_, '_>,
    args: RemoveOperatorsIxArgs,
) -> ProgramResult {
    remove_operators_invoke_with_program_id(BOOP_PROGRAM_ID, accounts, args)
}
pub fn remove_operators_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: RemoveOperatorsAccounts<'_, '_>,
    args: RemoveOperatorsIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: RemoveOperatorsKeys = accounts.into();
    let ix = remove_operators_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn remove_operators_invoke_signed(
    accounts: RemoveOperatorsAccounts<'_, '_>,
    args: RemoveOperatorsIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    remove_operators_invoke_signed_with_program_id(
        BOOP_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn remove_operators_verify_account_keys(
    accounts: RemoveOperatorsAccounts<'_, '_>,
    keys: RemoveOperatorsKeys,
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
pub fn remove_operators_verify_writable_privileges<'me, 'info>(
    accounts: RemoveOperatorsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.config, accounts.authority] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn remove_operators_verify_signer_privileges<'me, 'info>(
    accounts: RemoveOperatorsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn remove_operators_verify_account_privileges<'me, 'info>(
    accounts: RemoveOperatorsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    remove_operators_verify_writable_privileges(accounts)?;
    remove_operators_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SELL_TOKEN_IX_ACCOUNTS_LEN: usize = 12;
#[derive(Copy, Clone, Debug)]
pub struct SellTokenAccounts<'me, 'info> {
    pub mint: &'me AccountInfo<'info>,
    pub bonding_curve: &'me AccountInfo<'info>,
    pub trading_fees_vault: &'me AccountInfo<'info>,
    pub bonding_curve_vault: &'me AccountInfo<'info>,
    pub bonding_curve_sol_vault: &'me AccountInfo<'info>,
    pub seller_token_account: &'me AccountInfo<'info>,
    pub seller: &'me AccountInfo<'info>,
    pub recipient: &'me AccountInfo<'info>,
    pub config: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SellTokenKeys {
    pub mint: Pubkey,
    pub bonding_curve: Pubkey,
    pub trading_fees_vault: Pubkey,
    pub bonding_curve_vault: Pubkey,
    pub bonding_curve_sol_vault: Pubkey,
    pub seller_token_account: Pubkey,
    pub seller: Pubkey,
    pub recipient: Pubkey,
    pub config: Pubkey,
    pub system_program: Pubkey,
    pub token_program: Pubkey,
    pub associated_token_program: Pubkey,
}
impl From<SellTokenAccounts<'_, '_>> for SellTokenKeys {
    fn from(accounts: SellTokenAccounts) -> Self {
        Self {
            mint: *accounts.mint.key,
            bonding_curve: *accounts.bonding_curve.key,
            trading_fees_vault: *accounts.trading_fees_vault.key,
            bonding_curve_vault: *accounts.bonding_curve_vault.key,
            bonding_curve_sol_vault: *accounts.bonding_curve_sol_vault.key,
            seller_token_account: *accounts.seller_token_account.key,
            seller: *accounts.seller.key,
            recipient: *accounts.recipient.key,
            config: *accounts.config.key,
            system_program: *accounts.system_program.key,
            token_program: *accounts.token_program.key,
            associated_token_program: *accounts.associated_token_program.key,
        }
    }
}
impl From<SellTokenKeys> for [AccountMeta; SELL_TOKEN_IX_ACCOUNTS_LEN] {
    fn from(keys: SellTokenKeys) -> Self {
        [
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
                pubkey: keys.trading_fees_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.bonding_curve_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.bonding_curve_sol_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.seller_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.seller,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.recipient,
                is_signer: false,
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
impl From<[Pubkey; SELL_TOKEN_IX_ACCOUNTS_LEN]> for SellTokenKeys {
    fn from(pubkeys: [Pubkey; SELL_TOKEN_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            mint: pubkeys[0],
            bonding_curve: pubkeys[1],
            trading_fees_vault: pubkeys[2],
            bonding_curve_vault: pubkeys[3],
            bonding_curve_sol_vault: pubkeys[4],
            seller_token_account: pubkeys[5],
            seller: pubkeys[6],
            recipient: pubkeys[7],
            config: pubkeys[8],
            system_program: pubkeys[9],
            token_program: pubkeys[10],
            associated_token_program: pubkeys[11],
        }
    }
}
impl<'info> From<SellTokenAccounts<'_, 'info>>
for [AccountInfo<'info>; SELL_TOKEN_IX_ACCOUNTS_LEN] {
    fn from(accounts: SellTokenAccounts<'_, 'info>) -> Self {
        [
            accounts.mint.clone(),
            accounts.bonding_curve.clone(),
            accounts.trading_fees_vault.clone(),
            accounts.bonding_curve_vault.clone(),
            accounts.bonding_curve_sol_vault.clone(),
            accounts.seller_token_account.clone(),
            accounts.seller.clone(),
            accounts.recipient.clone(),
            accounts.config.clone(),
            accounts.system_program.clone(),
            accounts.token_program.clone(),
            accounts.associated_token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SELL_TOKEN_IX_ACCOUNTS_LEN]>
for SellTokenAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; SELL_TOKEN_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            mint: &arr[0],
            bonding_curve: &arr[1],
            trading_fees_vault: &arr[2],
            bonding_curve_vault: &arr[3],
            bonding_curve_sol_vault: &arr[4],
            seller_token_account: &arr[5],
            seller: &arr[6],
            recipient: &arr[7],
            config: &arr[8],
            system_program: &arr[9],
            token_program: &arr[10],
            associated_token_program: &arr[11],
        }
    }
}
pub const SELL_TOKEN_IX_DISCM: [u8; 8usize] = [109, 61, 40, 187, 230, 176, 135, 174];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SellTokenIxArgs {
    pub sell_amount: u64,
    pub amount_out_min: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct SellTokenIxData(pub SellTokenIxArgs);
impl From<SellTokenIxArgs> for SellTokenIxData {
    fn from(args: SellTokenIxArgs) -> Self {
        Self(args)
    }
}
impl SellTokenIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SELL_TOKEN_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let sell_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let amount_out_min: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(SellTokenIxArgs {
                sell_amount,
                amount_out_min,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SELL_TOKEN_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.sell_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.amount_out_min, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn sell_token_ix_with_program_id(
    program_id: Pubkey,
    keys: SellTokenKeys,
    args: SellTokenIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SELL_TOKEN_IX_ACCOUNTS_LEN] = keys.into();
    let data: SellTokenIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn sell_token_ix(
    keys: SellTokenKeys,
    args: SellTokenIxArgs,
) -> std::io::Result<Instruction> {
    sell_token_ix_with_program_id(BOOP_PROGRAM_ID, keys, args)
}
pub fn sell_token_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SellTokenAccounts<'_, '_>,
    args: SellTokenIxArgs,
) -> ProgramResult {
    let keys: SellTokenKeys = accounts.into();
    let ix = sell_token_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn sell_token_invoke(
    accounts: SellTokenAccounts<'_, '_>,
    args: SellTokenIxArgs,
) -> ProgramResult {
    sell_token_invoke_with_program_id(BOOP_PROGRAM_ID, accounts, args)
}
pub fn sell_token_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SellTokenAccounts<'_, '_>,
    args: SellTokenIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SellTokenKeys = accounts.into();
    let ix = sell_token_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn sell_token_invoke_signed(
    accounts: SellTokenAccounts<'_, '_>,
    args: SellTokenIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    sell_token_invoke_signed_with_program_id(BOOP_PROGRAM_ID, accounts, args, seeds)
}
pub fn sell_token_verify_account_keys(
    accounts: SellTokenAccounts<'_, '_>,
    keys: SellTokenKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.mint.key, keys.mint),
        (*accounts.bonding_curve.key, keys.bonding_curve),
        (*accounts.trading_fees_vault.key, keys.trading_fees_vault),
        (*accounts.bonding_curve_vault.key, keys.bonding_curve_vault),
        (*accounts.bonding_curve_sol_vault.key, keys.bonding_curve_sol_vault),
        (*accounts.seller_token_account.key, keys.seller_token_account),
        (*accounts.seller.key, keys.seller),
        (*accounts.recipient.key, keys.recipient),
        (*accounts.config.key, keys.config),
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
pub fn sell_token_verify_writable_privileges<'me, 'info>(
    accounts: SellTokenAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.bonding_curve,
        accounts.trading_fees_vault,
        accounts.bonding_curve_vault,
        accounts.bonding_curve_sol_vault,
        accounts.seller_token_account,
        accounts.seller,
        accounts.recipient,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn sell_token_verify_signer_privileges<'me, 'info>(
    accounts: SellTokenAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.seller] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn sell_token_verify_account_privileges<'me, 'info>(
    accounts: SellTokenAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    sell_token_verify_writable_privileges(accounts)?;
    sell_token_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SPLIT_POST_GRADUATION_TRADING_FEES_IX_ACCOUNTS_LEN: usize = 24;
#[derive(Copy, Clone, Debug)]
pub struct SplitPostGraduationTradingFeesAccounts<'me, 'info> {
    pub operator: &'me AccountInfo<'info>,
    pub mint: &'me AccountInfo<'info>,
    pub wsol: &'me AccountInfo<'info>,
    pub config: &'me AccountInfo<'info>,
    pub vault_authority: &'me AccountInfo<'info>,
    pub bonding_curve: &'me AccountInfo<'info>,
    pub trading_fees_vault: &'me AccountInfo<'info>,
    pub fee_splitter_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
    pub fee_splitter_config: &'me AccountInfo<'info>,
    pub fee_splitter_creator_vault: &'me AccountInfo<'info>,
    pub fee_splitter_vault_authority: &'me AccountInfo<'info>,
    pub fee_splitter_creator_vault_authority: &'me AccountInfo<'info>,
    pub fee_splitter_staking_mint: &'me AccountInfo<'info>,
    pub fee_splitter_wsol_vault: &'me AccountInfo<'info>,
    pub fee_splitter_creator_vault_authority_wsol_vault: &'me AccountInfo<'info>,
    pub fee_splitter_treasury_wsol_vault: &'me AccountInfo<'info>,
    pub fee_splitter_team_wsol_vault: &'me AccountInfo<'info>,
    pub fee_splitter_reward_pool: &'me AccountInfo<'info>,
    pub fee_splitter_reward_pool_staking_vault: &'me AccountInfo<'info>,
    pub fee_splitter_reward_pool_reward_vault: &'me AccountInfo<'info>,
    pub fee_splitter_reward_pool_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SplitPostGraduationTradingFeesKeys {
    pub operator: Pubkey,
    pub mint: Pubkey,
    pub wsol: Pubkey,
    pub config: Pubkey,
    pub vault_authority: Pubkey,
    pub bonding_curve: Pubkey,
    pub trading_fees_vault: Pubkey,
    pub fee_splitter_program: Pubkey,
    pub system_program: Pubkey,
    pub token_program: Pubkey,
    pub associated_token_program: Pubkey,
    pub fee_splitter_config: Pubkey,
    pub fee_splitter_creator_vault: Pubkey,
    pub fee_splitter_vault_authority: Pubkey,
    pub fee_splitter_creator_vault_authority: Pubkey,
    pub fee_splitter_staking_mint: Pubkey,
    pub fee_splitter_wsol_vault: Pubkey,
    pub fee_splitter_creator_vault_authority_wsol_vault: Pubkey,
    pub fee_splitter_treasury_wsol_vault: Pubkey,
    pub fee_splitter_team_wsol_vault: Pubkey,
    pub fee_splitter_reward_pool: Pubkey,
    pub fee_splitter_reward_pool_staking_vault: Pubkey,
    pub fee_splitter_reward_pool_reward_vault: Pubkey,
    pub fee_splitter_reward_pool_program: Pubkey,
}
impl From<SplitPostGraduationTradingFeesAccounts<'_, '_>>
for SplitPostGraduationTradingFeesKeys {
    fn from(accounts: SplitPostGraduationTradingFeesAccounts) -> Self {
        Self {
            operator: *accounts.operator.key,
            mint: *accounts.mint.key,
            wsol: *accounts.wsol.key,
            config: *accounts.config.key,
            vault_authority: *accounts.vault_authority.key,
            bonding_curve: *accounts.bonding_curve.key,
            trading_fees_vault: *accounts.trading_fees_vault.key,
            fee_splitter_program: *accounts.fee_splitter_program.key,
            system_program: *accounts.system_program.key,
            token_program: *accounts.token_program.key,
            associated_token_program: *accounts.associated_token_program.key,
            fee_splitter_config: *accounts.fee_splitter_config.key,
            fee_splitter_creator_vault: *accounts.fee_splitter_creator_vault.key,
            fee_splitter_vault_authority: *accounts.fee_splitter_vault_authority.key,
            fee_splitter_creator_vault_authority: *accounts
                .fee_splitter_creator_vault_authority
                .key,
            fee_splitter_staking_mint: *accounts.fee_splitter_staking_mint.key,
            fee_splitter_wsol_vault: *accounts.fee_splitter_wsol_vault.key,
            fee_splitter_creator_vault_authority_wsol_vault: *accounts
                .fee_splitter_creator_vault_authority_wsol_vault
                .key,
            fee_splitter_treasury_wsol_vault: *accounts
                .fee_splitter_treasury_wsol_vault
                .key,
            fee_splitter_team_wsol_vault: *accounts.fee_splitter_team_wsol_vault.key,
            fee_splitter_reward_pool: *accounts.fee_splitter_reward_pool.key,
            fee_splitter_reward_pool_staking_vault: *accounts
                .fee_splitter_reward_pool_staking_vault
                .key,
            fee_splitter_reward_pool_reward_vault: *accounts
                .fee_splitter_reward_pool_reward_vault
                .key,
            fee_splitter_reward_pool_program: *accounts
                .fee_splitter_reward_pool_program
                .key,
        }
    }
}
impl From<SplitPostGraduationTradingFeesKeys>
for [AccountMeta; SPLIT_POST_GRADUATION_TRADING_FEES_IX_ACCOUNTS_LEN] {
    fn from(keys: SplitPostGraduationTradingFeesKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.operator,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.wsol,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.bonding_curve,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.trading_fees_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.fee_splitter_program,
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
                pubkey: keys.fee_splitter_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.fee_splitter_creator_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.fee_splitter_vault_authority,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.fee_splitter_creator_vault_authority,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.fee_splitter_staking_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.fee_splitter_wsol_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.fee_splitter_creator_vault_authority_wsol_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.fee_splitter_treasury_wsol_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.fee_splitter_team_wsol_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.fee_splitter_reward_pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.fee_splitter_reward_pool_staking_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.fee_splitter_reward_pool_reward_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.fee_splitter_reward_pool_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; SPLIT_POST_GRADUATION_TRADING_FEES_IX_ACCOUNTS_LEN]>
for SplitPostGraduationTradingFeesKeys {
    fn from(
        pubkeys: [Pubkey; SPLIT_POST_GRADUATION_TRADING_FEES_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            operator: pubkeys[0],
            mint: pubkeys[1],
            wsol: pubkeys[2],
            config: pubkeys[3],
            vault_authority: pubkeys[4],
            bonding_curve: pubkeys[5],
            trading_fees_vault: pubkeys[6],
            fee_splitter_program: pubkeys[7],
            system_program: pubkeys[8],
            token_program: pubkeys[9],
            associated_token_program: pubkeys[10],
            fee_splitter_config: pubkeys[11],
            fee_splitter_creator_vault: pubkeys[12],
            fee_splitter_vault_authority: pubkeys[13],
            fee_splitter_creator_vault_authority: pubkeys[14],
            fee_splitter_staking_mint: pubkeys[15],
            fee_splitter_wsol_vault: pubkeys[16],
            fee_splitter_creator_vault_authority_wsol_vault: pubkeys[17],
            fee_splitter_treasury_wsol_vault: pubkeys[18],
            fee_splitter_team_wsol_vault: pubkeys[19],
            fee_splitter_reward_pool: pubkeys[20],
            fee_splitter_reward_pool_staking_vault: pubkeys[21],
            fee_splitter_reward_pool_reward_vault: pubkeys[22],
            fee_splitter_reward_pool_program: pubkeys[23],
        }
    }
}
impl<'info> From<SplitPostGraduationTradingFeesAccounts<'_, 'info>>
for [AccountInfo<'info>; SPLIT_POST_GRADUATION_TRADING_FEES_IX_ACCOUNTS_LEN] {
    fn from(accounts: SplitPostGraduationTradingFeesAccounts<'_, 'info>) -> Self {
        [
            accounts.operator.clone(),
            accounts.mint.clone(),
            accounts.wsol.clone(),
            accounts.config.clone(),
            accounts.vault_authority.clone(),
            accounts.bonding_curve.clone(),
            accounts.trading_fees_vault.clone(),
            accounts.fee_splitter_program.clone(),
            accounts.system_program.clone(),
            accounts.token_program.clone(),
            accounts.associated_token_program.clone(),
            accounts.fee_splitter_config.clone(),
            accounts.fee_splitter_creator_vault.clone(),
            accounts.fee_splitter_vault_authority.clone(),
            accounts.fee_splitter_creator_vault_authority.clone(),
            accounts.fee_splitter_staking_mint.clone(),
            accounts.fee_splitter_wsol_vault.clone(),
            accounts.fee_splitter_creator_vault_authority_wsol_vault.clone(),
            accounts.fee_splitter_treasury_wsol_vault.clone(),
            accounts.fee_splitter_team_wsol_vault.clone(),
            accounts.fee_splitter_reward_pool.clone(),
            accounts.fee_splitter_reward_pool_staking_vault.clone(),
            accounts.fee_splitter_reward_pool_reward_vault.clone(),
            accounts.fee_splitter_reward_pool_program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; SPLIT_POST_GRADUATION_TRADING_FEES_IX_ACCOUNTS_LEN]>
for SplitPostGraduationTradingFeesAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<
            'info,
        >; SPLIT_POST_GRADUATION_TRADING_FEES_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            operator: &arr[0],
            mint: &arr[1],
            wsol: &arr[2],
            config: &arr[3],
            vault_authority: &arr[4],
            bonding_curve: &arr[5],
            trading_fees_vault: &arr[6],
            fee_splitter_program: &arr[7],
            system_program: &arr[8],
            token_program: &arr[9],
            associated_token_program: &arr[10],
            fee_splitter_config: &arr[11],
            fee_splitter_creator_vault: &arr[12],
            fee_splitter_vault_authority: &arr[13],
            fee_splitter_creator_vault_authority: &arr[14],
            fee_splitter_staking_mint: &arr[15],
            fee_splitter_wsol_vault: &arr[16],
            fee_splitter_creator_vault_authority_wsol_vault: &arr[17],
            fee_splitter_treasury_wsol_vault: &arr[18],
            fee_splitter_team_wsol_vault: &arr[19],
            fee_splitter_reward_pool: &arr[20],
            fee_splitter_reward_pool_staking_vault: &arr[21],
            fee_splitter_reward_pool_reward_vault: &arr[22],
            fee_splitter_reward_pool_program: &arr[23],
        }
    }
}
pub const SPLIT_POST_GRADUATION_TRADING_FEES_IX_DISCM: [u8; 8usize] = [
    241, 178, 177, 69, 38, 187, 58, 176,
];
#[derive(Clone, Debug, PartialEq)]
pub struct SplitPostGraduationTradingFeesIxData;
impl SplitPostGraduationTradingFeesIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SPLIT_POST_GRADUATION_TRADING_FEES_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SPLIT_POST_GRADUATION_TRADING_FEES_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn split_post_graduation_trading_fees_ix_with_program_id(
    program_id: Pubkey,
    keys: SplitPostGraduationTradingFeesKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SPLIT_POST_GRADUATION_TRADING_FEES_IX_ACCOUNTS_LEN] = keys
        .into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: SplitPostGraduationTradingFeesIxData.try_to_vec()?,
    })
}
pub fn split_post_graduation_trading_fees_ix(
    keys: SplitPostGraduationTradingFeesKeys,
) -> std::io::Result<Instruction> {
    split_post_graduation_trading_fees_ix_with_program_id(BOOP_PROGRAM_ID, keys)
}
pub fn split_post_graduation_trading_fees_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SplitPostGraduationTradingFeesAccounts<'_, '_>,
) -> ProgramResult {
    let keys: SplitPostGraduationTradingFeesKeys = accounts.into();
    let ix = split_post_graduation_trading_fees_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn split_post_graduation_trading_fees_invoke(
    accounts: SplitPostGraduationTradingFeesAccounts<'_, '_>,
) -> ProgramResult {
    split_post_graduation_trading_fees_invoke_with_program_id(BOOP_PROGRAM_ID, accounts)
}
pub fn split_post_graduation_trading_fees_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SplitPostGraduationTradingFeesAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SplitPostGraduationTradingFeesKeys = accounts.into();
    let ix = split_post_graduation_trading_fees_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn split_post_graduation_trading_fees_invoke_signed(
    accounts: SplitPostGraduationTradingFeesAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    split_post_graduation_trading_fees_invoke_signed_with_program_id(
        BOOP_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn split_post_graduation_trading_fees_verify_account_keys(
    accounts: SplitPostGraduationTradingFeesAccounts<'_, '_>,
    keys: SplitPostGraduationTradingFeesKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.operator.key, keys.operator),
        (*accounts.mint.key, keys.mint),
        (*accounts.wsol.key, keys.wsol),
        (*accounts.config.key, keys.config),
        (*accounts.vault_authority.key, keys.vault_authority),
        (*accounts.bonding_curve.key, keys.bonding_curve),
        (*accounts.trading_fees_vault.key, keys.trading_fees_vault),
        (*accounts.fee_splitter_program.key, keys.fee_splitter_program),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.associated_token_program.key, keys.associated_token_program),
        (*accounts.fee_splitter_config.key, keys.fee_splitter_config),
        (*accounts.fee_splitter_creator_vault.key, keys.fee_splitter_creator_vault),
        (*accounts.fee_splitter_vault_authority.key, keys.fee_splitter_vault_authority),
        (
            *accounts.fee_splitter_creator_vault_authority.key,
            keys.fee_splitter_creator_vault_authority,
        ),
        (*accounts.fee_splitter_staking_mint.key, keys.fee_splitter_staking_mint),
        (*accounts.fee_splitter_wsol_vault.key, keys.fee_splitter_wsol_vault),
        (
            *accounts.fee_splitter_creator_vault_authority_wsol_vault.key,
            keys.fee_splitter_creator_vault_authority_wsol_vault,
        ),
        (
            *accounts.fee_splitter_treasury_wsol_vault.key,
            keys.fee_splitter_treasury_wsol_vault,
        ),
        (*accounts.fee_splitter_team_wsol_vault.key, keys.fee_splitter_team_wsol_vault),
        (*accounts.fee_splitter_reward_pool.key, keys.fee_splitter_reward_pool),
        (
            *accounts.fee_splitter_reward_pool_staking_vault.key,
            keys.fee_splitter_reward_pool_staking_vault,
        ),
        (
            *accounts.fee_splitter_reward_pool_reward_vault.key,
            keys.fee_splitter_reward_pool_reward_vault,
        ),
        (
            *accounts.fee_splitter_reward_pool_program.key,
            keys.fee_splitter_reward_pool_program,
        ),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn split_post_graduation_trading_fees_verify_writable_privileges<'me, 'info>(
    accounts: SplitPostGraduationTradingFeesAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.operator,
        accounts.bonding_curve,
        accounts.trading_fees_vault,
        accounts.fee_splitter_creator_vault,
        accounts.fee_splitter_vault_authority,
        accounts.fee_splitter_creator_vault_authority,
        accounts.fee_splitter_wsol_vault,
        accounts.fee_splitter_creator_vault_authority_wsol_vault,
        accounts.fee_splitter_treasury_wsol_vault,
        accounts.fee_splitter_team_wsol_vault,
        accounts.fee_splitter_reward_pool,
        accounts.fee_splitter_reward_pool_staking_vault,
        accounts.fee_splitter_reward_pool_reward_vault,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn split_post_graduation_trading_fees_verify_signer_privileges<'me, 'info>(
    accounts: SplitPostGraduationTradingFeesAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.operator] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn split_post_graduation_trading_fees_verify_account_privileges<'me, 'info>(
    accounts: SplitPostGraduationTradingFeesAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    split_post_graduation_trading_fees_verify_writable_privileges(accounts)?;
    split_post_graduation_trading_fees_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SPLIT_TRADING_FEES_IX_ACCOUNTS_LEN: usize = 24;
#[derive(Copy, Clone, Debug)]
pub struct SplitTradingFeesAccounts<'me, 'info> {
    pub operator: &'me AccountInfo<'info>,
    pub mint: &'me AccountInfo<'info>,
    pub wsol: &'me AccountInfo<'info>,
    pub config: &'me AccountInfo<'info>,
    pub vault_authority: &'me AccountInfo<'info>,
    pub bonding_curve: &'me AccountInfo<'info>,
    pub trading_fees_vault: &'me AccountInfo<'info>,
    pub fee_splitter_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
    pub fee_splitter_config: &'me AccountInfo<'info>,
    pub fee_splitter_creator_vault: &'me AccountInfo<'info>,
    pub fee_splitter_vault_authority: &'me AccountInfo<'info>,
    pub fee_splitter_creator_vault_authority: &'me AccountInfo<'info>,
    pub fee_splitter_staking_mint: &'me AccountInfo<'info>,
    pub fee_splitter_wsol_vault: &'me AccountInfo<'info>,
    pub fee_splitter_creator_vault_authority_wsol_vault: &'me AccountInfo<'info>,
    pub fee_splitter_treasury_wsol_vault: &'me AccountInfo<'info>,
    pub fee_splitter_team_wsol_vault: &'me AccountInfo<'info>,
    pub fee_splitter_reward_pool: &'me AccountInfo<'info>,
    pub fee_splitter_reward_pool_staking_vault: &'me AccountInfo<'info>,
    pub fee_splitter_reward_pool_reward_vault: &'me AccountInfo<'info>,
    pub fee_splitter_reward_pool_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SplitTradingFeesKeys {
    pub operator: Pubkey,
    pub mint: Pubkey,
    pub wsol: Pubkey,
    pub config: Pubkey,
    pub vault_authority: Pubkey,
    pub bonding_curve: Pubkey,
    pub trading_fees_vault: Pubkey,
    pub fee_splitter_program: Pubkey,
    pub system_program: Pubkey,
    pub token_program: Pubkey,
    pub associated_token_program: Pubkey,
    pub fee_splitter_config: Pubkey,
    pub fee_splitter_creator_vault: Pubkey,
    pub fee_splitter_vault_authority: Pubkey,
    pub fee_splitter_creator_vault_authority: Pubkey,
    pub fee_splitter_staking_mint: Pubkey,
    pub fee_splitter_wsol_vault: Pubkey,
    pub fee_splitter_creator_vault_authority_wsol_vault: Pubkey,
    pub fee_splitter_treasury_wsol_vault: Pubkey,
    pub fee_splitter_team_wsol_vault: Pubkey,
    pub fee_splitter_reward_pool: Pubkey,
    pub fee_splitter_reward_pool_staking_vault: Pubkey,
    pub fee_splitter_reward_pool_reward_vault: Pubkey,
    pub fee_splitter_reward_pool_program: Pubkey,
}
impl From<SplitTradingFeesAccounts<'_, '_>> for SplitTradingFeesKeys {
    fn from(accounts: SplitTradingFeesAccounts) -> Self {
        Self {
            operator: *accounts.operator.key,
            mint: *accounts.mint.key,
            wsol: *accounts.wsol.key,
            config: *accounts.config.key,
            vault_authority: *accounts.vault_authority.key,
            bonding_curve: *accounts.bonding_curve.key,
            trading_fees_vault: *accounts.trading_fees_vault.key,
            fee_splitter_program: *accounts.fee_splitter_program.key,
            system_program: *accounts.system_program.key,
            token_program: *accounts.token_program.key,
            associated_token_program: *accounts.associated_token_program.key,
            fee_splitter_config: *accounts.fee_splitter_config.key,
            fee_splitter_creator_vault: *accounts.fee_splitter_creator_vault.key,
            fee_splitter_vault_authority: *accounts.fee_splitter_vault_authority.key,
            fee_splitter_creator_vault_authority: *accounts
                .fee_splitter_creator_vault_authority
                .key,
            fee_splitter_staking_mint: *accounts.fee_splitter_staking_mint.key,
            fee_splitter_wsol_vault: *accounts.fee_splitter_wsol_vault.key,
            fee_splitter_creator_vault_authority_wsol_vault: *accounts
                .fee_splitter_creator_vault_authority_wsol_vault
                .key,
            fee_splitter_treasury_wsol_vault: *accounts
                .fee_splitter_treasury_wsol_vault
                .key,
            fee_splitter_team_wsol_vault: *accounts.fee_splitter_team_wsol_vault.key,
            fee_splitter_reward_pool: *accounts.fee_splitter_reward_pool.key,
            fee_splitter_reward_pool_staking_vault: *accounts
                .fee_splitter_reward_pool_staking_vault
                .key,
            fee_splitter_reward_pool_reward_vault: *accounts
                .fee_splitter_reward_pool_reward_vault
                .key,
            fee_splitter_reward_pool_program: *accounts
                .fee_splitter_reward_pool_program
                .key,
        }
    }
}
impl From<SplitTradingFeesKeys> for [AccountMeta; SPLIT_TRADING_FEES_IX_ACCOUNTS_LEN] {
    fn from(keys: SplitTradingFeesKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.operator,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.wsol,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.bonding_curve,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.trading_fees_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.fee_splitter_program,
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
                pubkey: keys.fee_splitter_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.fee_splitter_creator_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.fee_splitter_vault_authority,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.fee_splitter_creator_vault_authority,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.fee_splitter_staking_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.fee_splitter_wsol_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.fee_splitter_creator_vault_authority_wsol_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.fee_splitter_treasury_wsol_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.fee_splitter_team_wsol_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.fee_splitter_reward_pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.fee_splitter_reward_pool_staking_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.fee_splitter_reward_pool_reward_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.fee_splitter_reward_pool_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; SPLIT_TRADING_FEES_IX_ACCOUNTS_LEN]> for SplitTradingFeesKeys {
    fn from(pubkeys: [Pubkey; SPLIT_TRADING_FEES_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            operator: pubkeys[0],
            mint: pubkeys[1],
            wsol: pubkeys[2],
            config: pubkeys[3],
            vault_authority: pubkeys[4],
            bonding_curve: pubkeys[5],
            trading_fees_vault: pubkeys[6],
            fee_splitter_program: pubkeys[7],
            system_program: pubkeys[8],
            token_program: pubkeys[9],
            associated_token_program: pubkeys[10],
            fee_splitter_config: pubkeys[11],
            fee_splitter_creator_vault: pubkeys[12],
            fee_splitter_vault_authority: pubkeys[13],
            fee_splitter_creator_vault_authority: pubkeys[14],
            fee_splitter_staking_mint: pubkeys[15],
            fee_splitter_wsol_vault: pubkeys[16],
            fee_splitter_creator_vault_authority_wsol_vault: pubkeys[17],
            fee_splitter_treasury_wsol_vault: pubkeys[18],
            fee_splitter_team_wsol_vault: pubkeys[19],
            fee_splitter_reward_pool: pubkeys[20],
            fee_splitter_reward_pool_staking_vault: pubkeys[21],
            fee_splitter_reward_pool_reward_vault: pubkeys[22],
            fee_splitter_reward_pool_program: pubkeys[23],
        }
    }
}
impl<'info> From<SplitTradingFeesAccounts<'_, 'info>>
for [AccountInfo<'info>; SPLIT_TRADING_FEES_IX_ACCOUNTS_LEN] {
    fn from(accounts: SplitTradingFeesAccounts<'_, 'info>) -> Self {
        [
            accounts.operator.clone(),
            accounts.mint.clone(),
            accounts.wsol.clone(),
            accounts.config.clone(),
            accounts.vault_authority.clone(),
            accounts.bonding_curve.clone(),
            accounts.trading_fees_vault.clone(),
            accounts.fee_splitter_program.clone(),
            accounts.system_program.clone(),
            accounts.token_program.clone(),
            accounts.associated_token_program.clone(),
            accounts.fee_splitter_config.clone(),
            accounts.fee_splitter_creator_vault.clone(),
            accounts.fee_splitter_vault_authority.clone(),
            accounts.fee_splitter_creator_vault_authority.clone(),
            accounts.fee_splitter_staking_mint.clone(),
            accounts.fee_splitter_wsol_vault.clone(),
            accounts.fee_splitter_creator_vault_authority_wsol_vault.clone(),
            accounts.fee_splitter_treasury_wsol_vault.clone(),
            accounts.fee_splitter_team_wsol_vault.clone(),
            accounts.fee_splitter_reward_pool.clone(),
            accounts.fee_splitter_reward_pool_staking_vault.clone(),
            accounts.fee_splitter_reward_pool_reward_vault.clone(),
            accounts.fee_splitter_reward_pool_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SPLIT_TRADING_FEES_IX_ACCOUNTS_LEN]>
for SplitTradingFeesAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; SPLIT_TRADING_FEES_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            operator: &arr[0],
            mint: &arr[1],
            wsol: &arr[2],
            config: &arr[3],
            vault_authority: &arr[4],
            bonding_curve: &arr[5],
            trading_fees_vault: &arr[6],
            fee_splitter_program: &arr[7],
            system_program: &arr[8],
            token_program: &arr[9],
            associated_token_program: &arr[10],
            fee_splitter_config: &arr[11],
            fee_splitter_creator_vault: &arr[12],
            fee_splitter_vault_authority: &arr[13],
            fee_splitter_creator_vault_authority: &arr[14],
            fee_splitter_staking_mint: &arr[15],
            fee_splitter_wsol_vault: &arr[16],
            fee_splitter_creator_vault_authority_wsol_vault: &arr[17],
            fee_splitter_treasury_wsol_vault: &arr[18],
            fee_splitter_team_wsol_vault: &arr[19],
            fee_splitter_reward_pool: &arr[20],
            fee_splitter_reward_pool_staking_vault: &arr[21],
            fee_splitter_reward_pool_reward_vault: &arr[22],
            fee_splitter_reward_pool_program: &arr[23],
        }
    }
}
pub const SPLIT_TRADING_FEES_IX_DISCM: [u8; 8usize] = [
    96, 126, 225, 47, 185, 213, 50, 58,
];
#[derive(Clone, Debug, PartialEq)]
pub struct SplitTradingFeesIxData;
impl SplitTradingFeesIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SPLIT_TRADING_FEES_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SPLIT_TRADING_FEES_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn split_trading_fees_ix_with_program_id(
    program_id: Pubkey,
    keys: SplitTradingFeesKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SPLIT_TRADING_FEES_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: SplitTradingFeesIxData.try_to_vec()?,
    })
}
pub fn split_trading_fees_ix(
    keys: SplitTradingFeesKeys,
) -> std::io::Result<Instruction> {
    split_trading_fees_ix_with_program_id(BOOP_PROGRAM_ID, keys)
}
pub fn split_trading_fees_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SplitTradingFeesAccounts<'_, '_>,
) -> ProgramResult {
    let keys: SplitTradingFeesKeys = accounts.into();
    let ix = split_trading_fees_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn split_trading_fees_invoke(
    accounts: SplitTradingFeesAccounts<'_, '_>,
) -> ProgramResult {
    split_trading_fees_invoke_with_program_id(BOOP_PROGRAM_ID, accounts)
}
pub fn split_trading_fees_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SplitTradingFeesAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SplitTradingFeesKeys = accounts.into();
    let ix = split_trading_fees_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn split_trading_fees_invoke_signed(
    accounts: SplitTradingFeesAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    split_trading_fees_invoke_signed_with_program_id(BOOP_PROGRAM_ID, accounts, seeds)
}
pub fn split_trading_fees_verify_account_keys(
    accounts: SplitTradingFeesAccounts<'_, '_>,
    keys: SplitTradingFeesKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.operator.key, keys.operator),
        (*accounts.mint.key, keys.mint),
        (*accounts.wsol.key, keys.wsol),
        (*accounts.config.key, keys.config),
        (*accounts.vault_authority.key, keys.vault_authority),
        (*accounts.bonding_curve.key, keys.bonding_curve),
        (*accounts.trading_fees_vault.key, keys.trading_fees_vault),
        (*accounts.fee_splitter_program.key, keys.fee_splitter_program),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.associated_token_program.key, keys.associated_token_program),
        (*accounts.fee_splitter_config.key, keys.fee_splitter_config),
        (*accounts.fee_splitter_creator_vault.key, keys.fee_splitter_creator_vault),
        (*accounts.fee_splitter_vault_authority.key, keys.fee_splitter_vault_authority),
        (
            *accounts.fee_splitter_creator_vault_authority.key,
            keys.fee_splitter_creator_vault_authority,
        ),
        (*accounts.fee_splitter_staking_mint.key, keys.fee_splitter_staking_mint),
        (*accounts.fee_splitter_wsol_vault.key, keys.fee_splitter_wsol_vault),
        (
            *accounts.fee_splitter_creator_vault_authority_wsol_vault.key,
            keys.fee_splitter_creator_vault_authority_wsol_vault,
        ),
        (
            *accounts.fee_splitter_treasury_wsol_vault.key,
            keys.fee_splitter_treasury_wsol_vault,
        ),
        (*accounts.fee_splitter_team_wsol_vault.key, keys.fee_splitter_team_wsol_vault),
        (*accounts.fee_splitter_reward_pool.key, keys.fee_splitter_reward_pool),
        (
            *accounts.fee_splitter_reward_pool_staking_vault.key,
            keys.fee_splitter_reward_pool_staking_vault,
        ),
        (
            *accounts.fee_splitter_reward_pool_reward_vault.key,
            keys.fee_splitter_reward_pool_reward_vault,
        ),
        (
            *accounts.fee_splitter_reward_pool_program.key,
            keys.fee_splitter_reward_pool_program,
        ),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn split_trading_fees_verify_writable_privileges<'me, 'info>(
    accounts: SplitTradingFeesAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.operator,
        accounts.bonding_curve,
        accounts.trading_fees_vault,
        accounts.fee_splitter_creator_vault,
        accounts.fee_splitter_vault_authority,
        accounts.fee_splitter_creator_vault_authority,
        accounts.fee_splitter_wsol_vault,
        accounts.fee_splitter_creator_vault_authority_wsol_vault,
        accounts.fee_splitter_treasury_wsol_vault,
        accounts.fee_splitter_team_wsol_vault,
        accounts.fee_splitter_reward_pool,
        accounts.fee_splitter_reward_pool_staking_vault,
        accounts.fee_splitter_reward_pool_reward_vault,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn split_trading_fees_verify_signer_privileges<'me, 'info>(
    accounts: SplitTradingFeesAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.operator] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn split_trading_fees_verify_account_privileges<'me, 'info>(
    accounts: SplitTradingFeesAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    split_trading_fees_verify_writable_privileges(accounts)?;
    split_trading_fees_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SWAP_SOL_FOR_TOKENS_ON_RAYDIUM_IX_ACCOUNTS_LEN: usize = 16;
#[derive(Copy, Clone, Debug)]
pub struct SwapSolForTokensOnRaydiumAccounts<'me, 'info> {
    pub config: &'me AccountInfo<'info>,
    pub bonding_curve: &'me AccountInfo<'info>,
    pub amm_config: &'me AccountInfo<'info>,
    pub operator: &'me AccountInfo<'info>,
    pub vault_authority: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub pool_state: &'me AccountInfo<'info>,
    pub input_vault: &'me AccountInfo<'info>,
    pub output_vault: &'me AccountInfo<'info>,
    pub bonding_curve_vault: &'me AccountInfo<'info>,
    pub bonding_curve_wsol_vault: &'me AccountInfo<'info>,
    pub output_token_mint: &'me AccountInfo<'info>,
    pub input_token_mint: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub cp_swap_program: &'me AccountInfo<'info>,
    pub observation_state: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SwapSolForTokensOnRaydiumKeys {
    pub config: Pubkey,
    pub bonding_curve: Pubkey,
    pub amm_config: Pubkey,
    pub operator: Pubkey,
    pub vault_authority: Pubkey,
    pub authority: Pubkey,
    pub pool_state: Pubkey,
    pub input_vault: Pubkey,
    pub output_vault: Pubkey,
    pub bonding_curve_vault: Pubkey,
    pub bonding_curve_wsol_vault: Pubkey,
    pub output_token_mint: Pubkey,
    pub input_token_mint: Pubkey,
    pub token_program: Pubkey,
    pub cp_swap_program: Pubkey,
    pub observation_state: Pubkey,
}
impl From<SwapSolForTokensOnRaydiumAccounts<'_, '_>> for SwapSolForTokensOnRaydiumKeys {
    fn from(accounts: SwapSolForTokensOnRaydiumAccounts) -> Self {
        Self {
            config: *accounts.config.key,
            bonding_curve: *accounts.bonding_curve.key,
            amm_config: *accounts.amm_config.key,
            operator: *accounts.operator.key,
            vault_authority: *accounts.vault_authority.key,
            authority: *accounts.authority.key,
            pool_state: *accounts.pool_state.key,
            input_vault: *accounts.input_vault.key,
            output_vault: *accounts.output_vault.key,
            bonding_curve_vault: *accounts.bonding_curve_vault.key,
            bonding_curve_wsol_vault: *accounts.bonding_curve_wsol_vault.key,
            output_token_mint: *accounts.output_token_mint.key,
            input_token_mint: *accounts.input_token_mint.key,
            token_program: *accounts.token_program.key,
            cp_swap_program: *accounts.cp_swap_program.key,
            observation_state: *accounts.observation_state.key,
        }
    }
}
impl From<SwapSolForTokensOnRaydiumKeys>
for [AccountMeta; SWAP_SOL_FOR_TOKENS_ON_RAYDIUM_IX_ACCOUNTS_LEN] {
    fn from(keys: SwapSolForTokensOnRaydiumKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.bonding_curve,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.amm_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.operator,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vault_authority,
                is_signer: false,
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
                pubkey: keys.bonding_curve_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.bonding_curve_wsol_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.output_token_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.input_token_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.cp_swap_program,
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
impl From<[Pubkey; SWAP_SOL_FOR_TOKENS_ON_RAYDIUM_IX_ACCOUNTS_LEN]>
for SwapSolForTokensOnRaydiumKeys {
    fn from(pubkeys: [Pubkey; SWAP_SOL_FOR_TOKENS_ON_RAYDIUM_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            config: pubkeys[0],
            bonding_curve: pubkeys[1],
            amm_config: pubkeys[2],
            operator: pubkeys[3],
            vault_authority: pubkeys[4],
            authority: pubkeys[5],
            pool_state: pubkeys[6],
            input_vault: pubkeys[7],
            output_vault: pubkeys[8],
            bonding_curve_vault: pubkeys[9],
            bonding_curve_wsol_vault: pubkeys[10],
            output_token_mint: pubkeys[11],
            input_token_mint: pubkeys[12],
            token_program: pubkeys[13],
            cp_swap_program: pubkeys[14],
            observation_state: pubkeys[15],
        }
    }
}
impl<'info> From<SwapSolForTokensOnRaydiumAccounts<'_, 'info>>
for [AccountInfo<'info>; SWAP_SOL_FOR_TOKENS_ON_RAYDIUM_IX_ACCOUNTS_LEN] {
    fn from(accounts: SwapSolForTokensOnRaydiumAccounts<'_, 'info>) -> Self {
        [
            accounts.config.clone(),
            accounts.bonding_curve.clone(),
            accounts.amm_config.clone(),
            accounts.operator.clone(),
            accounts.vault_authority.clone(),
            accounts.authority.clone(),
            accounts.pool_state.clone(),
            accounts.input_vault.clone(),
            accounts.output_vault.clone(),
            accounts.bonding_curve_vault.clone(),
            accounts.bonding_curve_wsol_vault.clone(),
            accounts.output_token_mint.clone(),
            accounts.input_token_mint.clone(),
            accounts.token_program.clone(),
            accounts.cp_swap_program.clone(),
            accounts.observation_state.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; SWAP_SOL_FOR_TOKENS_ON_RAYDIUM_IX_ACCOUNTS_LEN]>
for SwapSolForTokensOnRaydiumAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; SWAP_SOL_FOR_TOKENS_ON_RAYDIUM_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            config: &arr[0],
            bonding_curve: &arr[1],
            amm_config: &arr[2],
            operator: &arr[3],
            vault_authority: &arr[4],
            authority: &arr[5],
            pool_state: &arr[6],
            input_vault: &arr[7],
            output_vault: &arr[8],
            bonding_curve_vault: &arr[9],
            bonding_curve_wsol_vault: &arr[10],
            output_token_mint: &arr[11],
            input_token_mint: &arr[12],
            token_program: &arr[13],
            cp_swap_program: &arr[14],
            observation_state: &arr[15],
        }
    }
}
pub const SWAP_SOL_FOR_TOKENS_ON_RAYDIUM_IX_DISCM: [u8; 8usize] = [
    107, 248, 131, 239, 152, 234, 54, 35,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SwapSolForTokensOnRaydiumIxArgs {
    pub amount_in: u64,
    pub minimum_amount_out: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct SwapSolForTokensOnRaydiumIxData(pub SwapSolForTokensOnRaydiumIxArgs);
impl From<SwapSolForTokensOnRaydiumIxArgs> for SwapSolForTokensOnRaydiumIxData {
    fn from(args: SwapSolForTokensOnRaydiumIxArgs) -> Self {
        Self(args)
    }
}
impl SwapSolForTokensOnRaydiumIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SWAP_SOL_FOR_TOKENS_ON_RAYDIUM_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let amount_in: u64 = crate::borsh_de_or_default(&mut reader)?;
        let minimum_amount_out: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(SwapSolForTokensOnRaydiumIxArgs {
                amount_in,
                minimum_amount_out,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SWAP_SOL_FOR_TOKENS_ON_RAYDIUM_IX_DISCM)?;
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
pub fn swap_sol_for_tokens_on_raydium_ix_with_program_id(
    program_id: Pubkey,
    keys: SwapSolForTokensOnRaydiumKeys,
    args: SwapSolForTokensOnRaydiumIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SWAP_SOL_FOR_TOKENS_ON_RAYDIUM_IX_ACCOUNTS_LEN] = keys
        .into();
    let data: SwapSolForTokensOnRaydiumIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn swap_sol_for_tokens_on_raydium_ix(
    keys: SwapSolForTokensOnRaydiumKeys,
    args: SwapSolForTokensOnRaydiumIxArgs,
) -> std::io::Result<Instruction> {
    swap_sol_for_tokens_on_raydium_ix_with_program_id(BOOP_PROGRAM_ID, keys, args)
}
pub fn swap_sol_for_tokens_on_raydium_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SwapSolForTokensOnRaydiumAccounts<'_, '_>,
    args: SwapSolForTokensOnRaydiumIxArgs,
) -> ProgramResult {
    let keys: SwapSolForTokensOnRaydiumKeys = accounts.into();
    let ix = swap_sol_for_tokens_on_raydium_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn swap_sol_for_tokens_on_raydium_invoke(
    accounts: SwapSolForTokensOnRaydiumAccounts<'_, '_>,
    args: SwapSolForTokensOnRaydiumIxArgs,
) -> ProgramResult {
    swap_sol_for_tokens_on_raydium_invoke_with_program_id(
        BOOP_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn swap_sol_for_tokens_on_raydium_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SwapSolForTokensOnRaydiumAccounts<'_, '_>,
    args: SwapSolForTokensOnRaydiumIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SwapSolForTokensOnRaydiumKeys = accounts.into();
    let ix = swap_sol_for_tokens_on_raydium_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn swap_sol_for_tokens_on_raydium_invoke_signed(
    accounts: SwapSolForTokensOnRaydiumAccounts<'_, '_>,
    args: SwapSolForTokensOnRaydiumIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    swap_sol_for_tokens_on_raydium_invoke_signed_with_program_id(
        BOOP_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn swap_sol_for_tokens_on_raydium_verify_account_keys(
    accounts: SwapSolForTokensOnRaydiumAccounts<'_, '_>,
    keys: SwapSolForTokensOnRaydiumKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.config.key, keys.config),
        (*accounts.bonding_curve.key, keys.bonding_curve),
        (*accounts.amm_config.key, keys.amm_config),
        (*accounts.operator.key, keys.operator),
        (*accounts.vault_authority.key, keys.vault_authority),
        (*accounts.authority.key, keys.authority),
        (*accounts.pool_state.key, keys.pool_state),
        (*accounts.input_vault.key, keys.input_vault),
        (*accounts.output_vault.key, keys.output_vault),
        (*accounts.bonding_curve_vault.key, keys.bonding_curve_vault),
        (*accounts.bonding_curve_wsol_vault.key, keys.bonding_curve_wsol_vault),
        (*accounts.output_token_mint.key, keys.output_token_mint),
        (*accounts.input_token_mint.key, keys.input_token_mint),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.cp_swap_program.key, keys.cp_swap_program),
        (*accounts.observation_state.key, keys.observation_state),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn swap_sol_for_tokens_on_raydium_verify_writable_privileges<'me, 'info>(
    accounts: SwapSolForTokensOnRaydiumAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.bonding_curve,
        accounts.operator,
        accounts.vault_authority,
        accounts.pool_state,
        accounts.input_vault,
        accounts.output_vault,
        accounts.bonding_curve_vault,
        accounts.bonding_curve_wsol_vault,
        accounts.observation_state,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn swap_sol_for_tokens_on_raydium_verify_signer_privileges<'me, 'info>(
    accounts: SwapSolForTokensOnRaydiumAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.operator] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn swap_sol_for_tokens_on_raydium_verify_account_privileges<'me, 'info>(
    accounts: SwapSolForTokensOnRaydiumAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    swap_sol_for_tokens_on_raydium_verify_writable_privileges(accounts)?;
    swap_sol_for_tokens_on_raydium_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SWAP_TOKENS_FOR_SOL_ON_RAYDIUM_IX_ACCOUNTS_LEN: usize = 16;
#[derive(Copy, Clone, Debug)]
pub struct SwapTokensForSolOnRaydiumAccounts<'me, 'info> {
    pub config: &'me AccountInfo<'info>,
    pub bonding_curve: &'me AccountInfo<'info>,
    pub amm_config: &'me AccountInfo<'info>,
    pub operator: &'me AccountInfo<'info>,
    pub vault_authority: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub pool_state: &'me AccountInfo<'info>,
    pub input_vault: &'me AccountInfo<'info>,
    pub output_vault: &'me AccountInfo<'info>,
    pub bonding_curve_vault: &'me AccountInfo<'info>,
    pub bonding_curve_wsol_vault: &'me AccountInfo<'info>,
    pub input_token_mint: &'me AccountInfo<'info>,
    pub output_token_mint: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub cp_swap_program: &'me AccountInfo<'info>,
    pub observation_state: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SwapTokensForSolOnRaydiumKeys {
    pub config: Pubkey,
    pub bonding_curve: Pubkey,
    pub amm_config: Pubkey,
    pub operator: Pubkey,
    pub vault_authority: Pubkey,
    pub authority: Pubkey,
    pub pool_state: Pubkey,
    pub input_vault: Pubkey,
    pub output_vault: Pubkey,
    pub bonding_curve_vault: Pubkey,
    pub bonding_curve_wsol_vault: Pubkey,
    pub input_token_mint: Pubkey,
    pub output_token_mint: Pubkey,
    pub token_program: Pubkey,
    pub cp_swap_program: Pubkey,
    pub observation_state: Pubkey,
}
impl From<SwapTokensForSolOnRaydiumAccounts<'_, '_>> for SwapTokensForSolOnRaydiumKeys {
    fn from(accounts: SwapTokensForSolOnRaydiumAccounts) -> Self {
        Self {
            config: *accounts.config.key,
            bonding_curve: *accounts.bonding_curve.key,
            amm_config: *accounts.amm_config.key,
            operator: *accounts.operator.key,
            vault_authority: *accounts.vault_authority.key,
            authority: *accounts.authority.key,
            pool_state: *accounts.pool_state.key,
            input_vault: *accounts.input_vault.key,
            output_vault: *accounts.output_vault.key,
            bonding_curve_vault: *accounts.bonding_curve_vault.key,
            bonding_curve_wsol_vault: *accounts.bonding_curve_wsol_vault.key,
            input_token_mint: *accounts.input_token_mint.key,
            output_token_mint: *accounts.output_token_mint.key,
            token_program: *accounts.token_program.key,
            cp_swap_program: *accounts.cp_swap_program.key,
            observation_state: *accounts.observation_state.key,
        }
    }
}
impl From<SwapTokensForSolOnRaydiumKeys>
for [AccountMeta; SWAP_TOKENS_FOR_SOL_ON_RAYDIUM_IX_ACCOUNTS_LEN] {
    fn from(keys: SwapTokensForSolOnRaydiumKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.bonding_curve,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.amm_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.operator,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vault_authority,
                is_signer: false,
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
                pubkey: keys.bonding_curve_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.bonding_curve_wsol_vault,
                is_signer: false,
                is_writable: true,
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
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.cp_swap_program,
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
impl From<[Pubkey; SWAP_TOKENS_FOR_SOL_ON_RAYDIUM_IX_ACCOUNTS_LEN]>
for SwapTokensForSolOnRaydiumKeys {
    fn from(pubkeys: [Pubkey; SWAP_TOKENS_FOR_SOL_ON_RAYDIUM_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            config: pubkeys[0],
            bonding_curve: pubkeys[1],
            amm_config: pubkeys[2],
            operator: pubkeys[3],
            vault_authority: pubkeys[4],
            authority: pubkeys[5],
            pool_state: pubkeys[6],
            input_vault: pubkeys[7],
            output_vault: pubkeys[8],
            bonding_curve_vault: pubkeys[9],
            bonding_curve_wsol_vault: pubkeys[10],
            input_token_mint: pubkeys[11],
            output_token_mint: pubkeys[12],
            token_program: pubkeys[13],
            cp_swap_program: pubkeys[14],
            observation_state: pubkeys[15],
        }
    }
}
impl<'info> From<SwapTokensForSolOnRaydiumAccounts<'_, 'info>>
for [AccountInfo<'info>; SWAP_TOKENS_FOR_SOL_ON_RAYDIUM_IX_ACCOUNTS_LEN] {
    fn from(accounts: SwapTokensForSolOnRaydiumAccounts<'_, 'info>) -> Self {
        [
            accounts.config.clone(),
            accounts.bonding_curve.clone(),
            accounts.amm_config.clone(),
            accounts.operator.clone(),
            accounts.vault_authority.clone(),
            accounts.authority.clone(),
            accounts.pool_state.clone(),
            accounts.input_vault.clone(),
            accounts.output_vault.clone(),
            accounts.bonding_curve_vault.clone(),
            accounts.bonding_curve_wsol_vault.clone(),
            accounts.input_token_mint.clone(),
            accounts.output_token_mint.clone(),
            accounts.token_program.clone(),
            accounts.cp_swap_program.clone(),
            accounts.observation_state.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; SWAP_TOKENS_FOR_SOL_ON_RAYDIUM_IX_ACCOUNTS_LEN]>
for SwapTokensForSolOnRaydiumAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; SWAP_TOKENS_FOR_SOL_ON_RAYDIUM_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            config: &arr[0],
            bonding_curve: &arr[1],
            amm_config: &arr[2],
            operator: &arr[3],
            vault_authority: &arr[4],
            authority: &arr[5],
            pool_state: &arr[6],
            input_vault: &arr[7],
            output_vault: &arr[8],
            bonding_curve_vault: &arr[9],
            bonding_curve_wsol_vault: &arr[10],
            input_token_mint: &arr[11],
            output_token_mint: &arr[12],
            token_program: &arr[13],
            cp_swap_program: &arr[14],
            observation_state: &arr[15],
        }
    }
}
pub const SWAP_TOKENS_FOR_SOL_ON_RAYDIUM_IX_DISCM: [u8; 8usize] = [
    216, 172, 130, 148, 34, 98, 215, 163,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SwapTokensForSolOnRaydiumIxArgs {
    pub amount_in: u64,
    pub minimum_amount_out: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct SwapTokensForSolOnRaydiumIxData(pub SwapTokensForSolOnRaydiumIxArgs);
impl From<SwapTokensForSolOnRaydiumIxArgs> for SwapTokensForSolOnRaydiumIxData {
    fn from(args: SwapTokensForSolOnRaydiumIxArgs) -> Self {
        Self(args)
    }
}
impl SwapTokensForSolOnRaydiumIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SWAP_TOKENS_FOR_SOL_ON_RAYDIUM_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let amount_in: u64 = crate::borsh_de_or_default(&mut reader)?;
        let minimum_amount_out: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(SwapTokensForSolOnRaydiumIxArgs {
                amount_in,
                minimum_amount_out,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SWAP_TOKENS_FOR_SOL_ON_RAYDIUM_IX_DISCM)?;
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
pub fn swap_tokens_for_sol_on_raydium_ix_with_program_id(
    program_id: Pubkey,
    keys: SwapTokensForSolOnRaydiumKeys,
    args: SwapTokensForSolOnRaydiumIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SWAP_TOKENS_FOR_SOL_ON_RAYDIUM_IX_ACCOUNTS_LEN] = keys
        .into();
    let data: SwapTokensForSolOnRaydiumIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn swap_tokens_for_sol_on_raydium_ix(
    keys: SwapTokensForSolOnRaydiumKeys,
    args: SwapTokensForSolOnRaydiumIxArgs,
) -> std::io::Result<Instruction> {
    swap_tokens_for_sol_on_raydium_ix_with_program_id(BOOP_PROGRAM_ID, keys, args)
}
pub fn swap_tokens_for_sol_on_raydium_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SwapTokensForSolOnRaydiumAccounts<'_, '_>,
    args: SwapTokensForSolOnRaydiumIxArgs,
) -> ProgramResult {
    let keys: SwapTokensForSolOnRaydiumKeys = accounts.into();
    let ix = swap_tokens_for_sol_on_raydium_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn swap_tokens_for_sol_on_raydium_invoke(
    accounts: SwapTokensForSolOnRaydiumAccounts<'_, '_>,
    args: SwapTokensForSolOnRaydiumIxArgs,
) -> ProgramResult {
    swap_tokens_for_sol_on_raydium_invoke_with_program_id(
        BOOP_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn swap_tokens_for_sol_on_raydium_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SwapTokensForSolOnRaydiumAccounts<'_, '_>,
    args: SwapTokensForSolOnRaydiumIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SwapTokensForSolOnRaydiumKeys = accounts.into();
    let ix = swap_tokens_for_sol_on_raydium_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn swap_tokens_for_sol_on_raydium_invoke_signed(
    accounts: SwapTokensForSolOnRaydiumAccounts<'_, '_>,
    args: SwapTokensForSolOnRaydiumIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    swap_tokens_for_sol_on_raydium_invoke_signed_with_program_id(
        BOOP_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn swap_tokens_for_sol_on_raydium_verify_account_keys(
    accounts: SwapTokensForSolOnRaydiumAccounts<'_, '_>,
    keys: SwapTokensForSolOnRaydiumKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.config.key, keys.config),
        (*accounts.bonding_curve.key, keys.bonding_curve),
        (*accounts.amm_config.key, keys.amm_config),
        (*accounts.operator.key, keys.operator),
        (*accounts.vault_authority.key, keys.vault_authority),
        (*accounts.authority.key, keys.authority),
        (*accounts.pool_state.key, keys.pool_state),
        (*accounts.input_vault.key, keys.input_vault),
        (*accounts.output_vault.key, keys.output_vault),
        (*accounts.bonding_curve_vault.key, keys.bonding_curve_vault),
        (*accounts.bonding_curve_wsol_vault.key, keys.bonding_curve_wsol_vault),
        (*accounts.input_token_mint.key, keys.input_token_mint),
        (*accounts.output_token_mint.key, keys.output_token_mint),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.cp_swap_program.key, keys.cp_swap_program),
        (*accounts.observation_state.key, keys.observation_state),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn swap_tokens_for_sol_on_raydium_verify_writable_privileges<'me, 'info>(
    accounts: SwapTokensForSolOnRaydiumAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.bonding_curve,
        accounts.operator,
        accounts.vault_authority,
        accounts.pool_state,
        accounts.input_vault,
        accounts.output_vault,
        accounts.bonding_curve_vault,
        accounts.bonding_curve_wsol_vault,
        accounts.observation_state,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn swap_tokens_for_sol_on_raydium_verify_signer_privileges<'me, 'info>(
    accounts: SwapTokensForSolOnRaydiumAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.operator] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn swap_tokens_for_sol_on_raydium_verify_account_privileges<'me, 'info>(
    accounts: SwapTokensForSolOnRaydiumAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    swap_tokens_for_sol_on_raydium_verify_writable_privileges(accounts)?;
    swap_tokens_for_sol_on_raydium_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const TOGGLE_PAUSED_IX_ACCOUNTS_LEN: usize = 2;
#[derive(Copy, Clone, Debug)]
pub struct TogglePausedAccounts<'me, 'info> {
    pub authority: &'me AccountInfo<'info>,
    pub config: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct TogglePausedKeys {
    pub authority: Pubkey,
    pub config: Pubkey,
}
impl From<TogglePausedAccounts<'_, '_>> for TogglePausedKeys {
    fn from(accounts: TogglePausedAccounts) -> Self {
        Self {
            authority: *accounts.authority.key,
            config: *accounts.config.key,
        }
    }
}
impl From<TogglePausedKeys> for [AccountMeta; TOGGLE_PAUSED_IX_ACCOUNTS_LEN] {
    fn from(keys: TogglePausedKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.authority,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.config,
                is_signer: false,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; TOGGLE_PAUSED_IX_ACCOUNTS_LEN]> for TogglePausedKeys {
    fn from(pubkeys: [Pubkey; TOGGLE_PAUSED_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            authority: pubkeys[0],
            config: pubkeys[1],
        }
    }
}
impl<'info> From<TogglePausedAccounts<'_, 'info>>
for [AccountInfo<'info>; TOGGLE_PAUSED_IX_ACCOUNTS_LEN] {
    fn from(accounts: TogglePausedAccounts<'_, 'info>) -> Self {
        [accounts.authority.clone(), accounts.config.clone()]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; TOGGLE_PAUSED_IX_ACCOUNTS_LEN]>
for TogglePausedAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; TOGGLE_PAUSED_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            authority: &arr[0],
            config: &arr[1],
        }
    }
}
pub const TOGGLE_PAUSED_IX_DISCM: [u8; 8usize] = [54, 83, 147, 198, 123, 97, 218, 72];
#[derive(Clone, Debug, PartialEq)]
pub struct TogglePausedIxData;
impl TogglePausedIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != TOGGLE_PAUSED_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&TOGGLE_PAUSED_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn toggle_paused_ix_with_program_id(
    program_id: Pubkey,
    keys: TogglePausedKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; TOGGLE_PAUSED_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: TogglePausedIxData.try_to_vec()?,
    })
}
pub fn toggle_paused_ix(keys: TogglePausedKeys) -> std::io::Result<Instruction> {
    toggle_paused_ix_with_program_id(BOOP_PROGRAM_ID, keys)
}
pub fn toggle_paused_invoke_with_program_id(
    program_id: Pubkey,
    accounts: TogglePausedAccounts<'_, '_>,
) -> ProgramResult {
    let keys: TogglePausedKeys = accounts.into();
    let ix = toggle_paused_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn toggle_paused_invoke(accounts: TogglePausedAccounts<'_, '_>) -> ProgramResult {
    toggle_paused_invoke_with_program_id(BOOP_PROGRAM_ID, accounts)
}
pub fn toggle_paused_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: TogglePausedAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: TogglePausedKeys = accounts.into();
    let ix = toggle_paused_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn toggle_paused_invoke_signed(
    accounts: TogglePausedAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    toggle_paused_invoke_signed_with_program_id(BOOP_PROGRAM_ID, accounts, seeds)
}
pub fn toggle_paused_verify_account_keys(
    accounts: TogglePausedAccounts<'_, '_>,
    keys: TogglePausedKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.authority.key, keys.authority),
        (*accounts.config.key, keys.config),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn toggle_paused_verify_writable_privileges<'me, 'info>(
    accounts: TogglePausedAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.authority, accounts.config] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn toggle_paused_verify_signer_privileges<'me, 'info>(
    accounts: TogglePausedAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn toggle_paused_verify_account_privileges<'me, 'info>(
    accounts: TogglePausedAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    toggle_paused_verify_writable_privileges(accounts)?;
    toggle_paused_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const UPDATE_CONFIG_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct UpdateConfigAccounts<'me, 'info> {
    pub config: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UpdateConfigKeys {
    pub config: Pubkey,
    pub authority: Pubkey,
    pub system_program: Pubkey,
}
impl From<UpdateConfigAccounts<'_, '_>> for UpdateConfigKeys {
    fn from(accounts: UpdateConfigAccounts) -> Self {
        Self {
            config: *accounts.config.key,
            authority: *accounts.authority.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<UpdateConfigKeys> for [AccountMeta; UPDATE_CONFIG_IX_ACCOUNTS_LEN] {
    fn from(keys: UpdateConfigKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.config,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.authority,
                is_signer: true,
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
impl From<[Pubkey; UPDATE_CONFIG_IX_ACCOUNTS_LEN]> for UpdateConfigKeys {
    fn from(pubkeys: [Pubkey; UPDATE_CONFIG_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            config: pubkeys[0],
            authority: pubkeys[1],
            system_program: pubkeys[2],
        }
    }
}
impl<'info> From<UpdateConfigAccounts<'_, 'info>>
for [AccountInfo<'info>; UPDATE_CONFIG_IX_ACCOUNTS_LEN] {
    fn from(accounts: UpdateConfigAccounts<'_, 'info>) -> Self {
        [
            accounts.config.clone(),
            accounts.authority.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; UPDATE_CONFIG_IX_ACCOUNTS_LEN]>
for UpdateConfigAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; UPDATE_CONFIG_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            config: &arr[0],
            authority: &arr[1],
            system_program: &arr[2],
        }
    }
}
pub const UPDATE_CONFIG_IX_DISCM: [u8; 8usize] = [29, 158, 252, 191, 10, 83, 219, 99];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UpdateConfigIxArgs {
    pub new_protocol_fee_recipient: Pubkey,
    pub new_virtual_sol_reserves: u64,
    pub new_virtual_token_reserves: u64,
    pub new_graduation_target: u64,
    pub new_graduation_fee: u64,
    pub new_damping_term: u8,
    pub new_swap_fee_basis_points: u8,
    pub new_token_for_stakers_basis_points: u16,
    pub new_token_amount_for_raydium_liquidity: u64,
    pub new_max_graduation_price_deviation_basis_points: u16,
    pub new_max_swap_amount_for_pool_price_correction_basis_points: u16,
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateConfigIxData(pub UpdateConfigIxArgs);
impl From<UpdateConfigIxArgs> for UpdateConfigIxData {
    fn from(args: UpdateConfigIxArgs) -> Self {
        Self(args)
    }
}
impl UpdateConfigIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_CONFIG_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let new_protocol_fee_recipient: Pubkey = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let new_virtual_sol_reserves: u64 = crate::borsh_de_or_default(&mut reader)?;
        let new_virtual_token_reserves: u64 = crate::borsh_de_or_default(&mut reader)?;
        let new_graduation_target: u64 = crate::borsh_de_or_default(&mut reader)?;
        let new_graduation_fee: u64 = crate::borsh_de_or_default(&mut reader)?;
        let new_damping_term: u8 = crate::borsh_de_or_default(&mut reader)?;
        let new_swap_fee_basis_points: u8 = crate::borsh_de_or_default(&mut reader)?;
        let new_token_for_stakers_basis_points: u16 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let new_token_amount_for_raydium_liquidity: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let new_max_graduation_price_deviation_basis_points: u16 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let new_max_swap_amount_for_pool_price_correction_basis_points: u16 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        Ok(
            Self(UpdateConfigIxArgs {
                new_protocol_fee_recipient,
                new_virtual_sol_reserves,
                new_virtual_token_reserves,
                new_graduation_target,
                new_graduation_fee,
                new_damping_term,
                new_swap_fee_basis_points,
                new_token_for_stakers_basis_points,
                new_token_amount_for_raydium_liquidity,
                new_max_graduation_price_deviation_basis_points,
                new_max_swap_amount_for_pool_price_correction_basis_points,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_CONFIG_IX_DISCM)?;
        borsh::BorshSerialize::serialize(
            &self.0.new_protocol_fee_recipient,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.0.new_virtual_sol_reserves, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.0.new_virtual_token_reserves,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.0.new_graduation_target, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.new_graduation_fee, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.new_damping_term, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.0.new_swap_fee_basis_points,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.0.new_token_for_stakers_basis_points,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.0.new_token_amount_for_raydium_liquidity,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.0.new_max_graduation_price_deviation_basis_points,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.0.new_max_swap_amount_for_pool_price_correction_basis_points,
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
pub fn update_config_ix_with_program_id(
    program_id: Pubkey,
    keys: UpdateConfigKeys,
    args: UpdateConfigIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; UPDATE_CONFIG_IX_ACCOUNTS_LEN] = keys.into();
    let data: UpdateConfigIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_config_ix(
    keys: UpdateConfigKeys,
    args: UpdateConfigIxArgs,
) -> std::io::Result<Instruction> {
    update_config_ix_with_program_id(BOOP_PROGRAM_ID, keys, args)
}
pub fn update_config_invoke_with_program_id(
    program_id: Pubkey,
    accounts: UpdateConfigAccounts<'_, '_>,
    args: UpdateConfigIxArgs,
) -> ProgramResult {
    let keys: UpdateConfigKeys = accounts.into();
    let ix = update_config_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn update_config_invoke(
    accounts: UpdateConfigAccounts<'_, '_>,
    args: UpdateConfigIxArgs,
) -> ProgramResult {
    update_config_invoke_with_program_id(BOOP_PROGRAM_ID, accounts, args)
}
pub fn update_config_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: UpdateConfigAccounts<'_, '_>,
    args: UpdateConfigIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: UpdateConfigKeys = accounts.into();
    let ix = update_config_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn update_config_invoke_signed(
    accounts: UpdateConfigAccounts<'_, '_>,
    args: UpdateConfigIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    update_config_invoke_signed_with_program_id(BOOP_PROGRAM_ID, accounts, args, seeds)
}
pub fn update_config_verify_account_keys(
    accounts: UpdateConfigAccounts<'_, '_>,
    keys: UpdateConfigKeys,
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
pub fn update_config_verify_writable_privileges<'me, 'info>(
    accounts: UpdateConfigAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.config] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn update_config_verify_signer_privileges<'me, 'info>(
    accounts: UpdateConfigAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn update_config_verify_account_privileges<'me, 'info>(
    accounts: UpdateConfigAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    update_config_verify_writable_privileges(accounts)?;
    update_config_verify_signer_privileges(accounts)?;
    Ok(())
}
