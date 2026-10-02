use solana_pubkey::Pubkey;
use solana_cpi::{invoke, invoke_signed};
use solana_instruction::{AccountMeta, Instruction};
use solana_account_info::AccountInfo;
use solana_program_error::ProgramError;
use std::io::Read;
#[allow(unused_imports)]
use crate::*;
#[derive(Clone, Debug, PartialEq)]
pub enum GammaProgramIx {
    AddOrUpdateActor(AddOrUpdateActorIxArgs),
    AddPartner,
    CalculateRewards,
    ClaimPartnerFees,
    ClaimRewards,
    CollectFundFee(CollectFundFeeIxArgs),
    CollectProtocolFee(CollectProtocolFeeIxArgs),
    CreateAmmConfig(CreateAmmConfigIxArgs),
    CreateRewards(CreateRewardsIxArgs),
    CreateSwapReferral(CreateSwapReferralIxArgs),
    Deposit(DepositIxArgs),
    InitUserPoolLiquidity(InitUserPoolLiquidityIxArgs),
    Initialize(InitializeIxArgs),
    InitializePartner(InitializePartnerIxArgs),
    InitializePoolPartners,
    OracleBasedSwapBaseInput(OracleBasedSwapBaseInputIxArgs),
    PoolConfigUpdate(PoolConfigUpdateIxArgs),
    PoolConfigUpdateV2(PoolConfigUpdateV2IxArgs),
    PoolConfigUpdateV3(PoolConfigUpdateV3IxArgs),
    RebalanceKamino,
    SwapBaseInput(SwapBaseInputIxArgs),
    SwapBaseOutput(SwapBaseOutputIxArgs),
    UpdateAmmConfig(UpdateAmmConfigIxArgs),
    UpdatePartner(UpdatePartnerIxArgs),
    UpdatePartnerFees,
    UpdatePool(UpdatePoolIxArgs),
    Withdraw(WithdrawIxArgs),
}
impl GammaProgramIx {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        if buf.starts_with(&ADD_OR_UPDATE_ACTOR_IX_DISCM) {
            let mut reader = &buf[ADD_OR_UPDATE_ACTOR_IX_DISCM.len()..];
            let is_admin: bool = crate::borsh_de_or_default(&mut reader)?;
            return Ok(Self::AddOrUpdateActor(AddOrUpdateActorIxArgs { is_admin }));
        }
        if buf.starts_with(&ADD_PARTNER_IX_DISCM) {
            return Ok(Self::AddPartner);
        }
        if buf.starts_with(&CALCULATE_REWARDS_IX_DISCM) {
            return Ok(Self::CalculateRewards);
        }
        if buf.starts_with(&CLAIM_PARTNER_FEES_IX_DISCM) {
            return Ok(Self::ClaimPartnerFees);
        }
        if buf.starts_with(&CLAIM_REWARDS_IX_DISCM) {
            return Ok(Self::ClaimRewards);
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
            let max_open_time: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::CreateAmmConfig(CreateAmmConfigIxArgs {
                    index,
                    trade_fee_rate,
                    protocol_fee_rate,
                    fund_fee_rate,
                    create_pool_fee,
                    max_open_time,
                }),
            );
        }
        if buf.starts_with(&CREATE_REWARDS_IX_DISCM) {
            let mut reader = &buf[CREATE_REWARDS_IX_DISCM.len()..];
            let start_time: u64 = crate::borsh_de_or_default(&mut reader)?;
            let end_time: u64 = crate::borsh_de_or_default(&mut reader)?;
            let reward_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::CreateRewards(CreateRewardsIxArgs {
                    start_time,
                    end_time,
                    reward_amount,
                }),
            );
        }
        if buf.starts_with(&CREATE_SWAP_REFERRAL_IX_DISCM) {
            let mut reader = &buf[CREATE_SWAP_REFERRAL_IX_DISCM.len()..];
            let name: String = crate::borsh_de_or_default(&mut reader)?;
            let default_share_bps: u16 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::CreateSwapReferral(CreateSwapReferralIxArgs {
                    name,
                    default_share_bps,
                }),
            );
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
        if buf.starts_with(&INIT_USER_POOL_LIQUIDITY_IX_DISCM) {
            let mut reader = &buf[INIT_USER_POOL_LIQUIDITY_IX_DISCM.len()..];
            let partner: Option<Pubkey> = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::InitUserPoolLiquidity(InitUserPoolLiquidityIxArgs {
                    partner,
                }),
            );
        }
        if buf.starts_with(&INITIALIZE_IX_DISCM) {
            let mut reader = &buf[INITIALIZE_IX_DISCM.len()..];
            let init_amount_0: u64 = crate::borsh_de_or_default(&mut reader)?;
            let init_amount_1: u64 = crate::borsh_de_or_default(&mut reader)?;
            let open_time: u64 = crate::borsh_de_or_default(&mut reader)?;
            let max_trade_fee_rate: u64 = crate::borsh_de_or_default(&mut reader)?;
            let volatility_factor: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::Initialize(InitializeIxArgs {
                    init_amount_0,
                    init_amount_1,
                    open_time,
                    max_trade_fee_rate,
                    volatility_factor,
                }),
            );
        }
        if buf.starts_with(&INITIALIZE_PARTNER_IX_DISCM) {
            let mut reader = &buf[INITIALIZE_PARTNER_IX_DISCM.len()..];
            let name: String = crate::borsh_de_or_default(&mut reader)?;
            let token_0_token_account: Pubkey = crate::borsh_de_or_default(&mut reader)?;
            let token_1_token_account: Pubkey = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::InitializePartner(InitializePartnerIxArgs {
                    name,
                    token_0_token_account,
                    token_1_token_account,
                }),
            );
        }
        if buf.starts_with(&INITIALIZE_POOL_PARTNERS_IX_DISCM) {
            return Ok(Self::InitializePoolPartners);
        }
        if buf.starts_with(&ORACLE_BASED_SWAP_BASE_INPUT_IX_DISCM) {
            let mut reader = &buf[ORACLE_BASED_SWAP_BASE_INPUT_IX_DISCM.len()..];
            let amount_in: u64 = crate::borsh_de_or_default(&mut reader)?;
            let minimum_amount_out: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::OracleBasedSwapBaseInput(OracleBasedSwapBaseInputIxArgs {
                    amount_in,
                    minimum_amount_out,
                }),
            );
        }
        if buf.starts_with(&POOL_CONFIG_UPDATE_IX_DISCM) {
            let mut reader = &buf[POOL_CONFIG_UPDATE_IX_DISCM.len()..];
            let h1: u128 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(Self::PoolConfigUpdate(PoolConfigUpdateIxArgs { h1 }));
        }
        if buf.starts_with(&POOL_CONFIG_UPDATE_V2_IX_DISCM) {
            let mut reader = &buf[POOL_CONFIG_UPDATE_V2_IX_DISCM.len()..];
            let h1: u128 = crate::borsh_de_or_default(&mut reader)?;
            let price_fetched_at: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::PoolConfigUpdateV2(PoolConfigUpdateV2IxArgs {
                    h1,
                    price_fetched_at,
                }),
            );
        }
        if buf.starts_with(&POOL_CONFIG_UPDATE_V3_IX_DISCM) {
            let mut reader = &buf[POOL_CONFIG_UPDATE_V3_IX_DISCM.len()..];
            let h1: u128 = crate::borsh_de_or_default(&mut reader)?;
            let h2: u128 = crate::borsh_de_or_default(&mut reader)?;
            let price_fetched_at: u64 = crate::borsh_de_or_default(&mut reader)?;
            let min_trade_rate_at_oracle_price: Option<u32> = crate::borsh_de_or_default(
                &mut reader,
            )?;
            let oracle_price_delay_fee_rate_per_second: Option<u32> = crate::borsh_de_or_default(
                &mut reader,
            )?;
            let max_oracle_price_delay_fee: Option<u32> = crate::borsh_de_or_default(
                &mut reader,
            )?;
            let drift_factor: Option<u32> = crate::borsh_de_or_default(&mut reader)?;
            let max_drift_factor: Option<u32> = crate::borsh_de_or_default(&mut reader)?;
            let volatility_factor: Option<u64> = crate::borsh_de_or_default(
                &mut reader,
            )?;
            return Ok(
                Self::PoolConfigUpdateV3(PoolConfigUpdateV3IxArgs {
                    h1,
                    h2,
                    price_fetched_at,
                    min_trade_rate_at_oracle_price,
                    oracle_price_delay_fee_rate_per_second,
                    max_oracle_price_delay_fee,
                    drift_factor,
                    max_drift_factor,
                    volatility_factor,
                }),
            );
        }
        if buf.starts_with(&REBALANCE_KAMINO_IX_DISCM) {
            return Ok(Self::RebalanceKamino);
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
            let param: u16 = crate::borsh_de_or_default(&mut reader)?;
            let value: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::UpdateAmmConfig(UpdateAmmConfigIxArgs {
                    param,
                    value,
                }),
            );
        }
        if buf.starts_with(&UPDATE_PARTNER_IX_DISCM) {
            let mut reader = &buf[UPDATE_PARTNER_IX_DISCM.len()..];
            let token_account_0: Option<Pubkey> = crate::borsh_de_or_default(
                &mut reader,
            )?;
            let token_account_1: Option<Pubkey> = crate::borsh_de_or_default(
                &mut reader,
            )?;
            return Ok(
                Self::UpdatePartner(UpdatePartnerIxArgs {
                    token_account_0,
                    token_account_1,
                }),
            );
        }
        if buf.starts_with(&UPDATE_PARTNER_FEES_IX_DISCM) {
            return Ok(Self::UpdatePartnerFees);
        }
        if buf.starts_with(&UPDATE_POOL_IX_DISCM) {
            let mut reader = &buf[UPDATE_POOL_IX_DISCM.len()..];
            let param: u32 = crate::borsh_de_or_default(&mut reader)?;
            let value: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(Self::UpdatePool(UpdatePoolIxArgs { param, value }));
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
            Self::AddOrUpdateActor(args) => {
                writer.write_all(&ADD_OR_UPDATE_ACTOR_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.is_admin, &mut writer)?;
                Ok(())
            }
            Self::AddPartner => writer.write_all(&ADD_PARTNER_IX_DISCM),
            Self::CalculateRewards => writer.write_all(&CALCULATE_REWARDS_IX_DISCM),
            Self::ClaimPartnerFees => writer.write_all(&CLAIM_PARTNER_FEES_IX_DISCM),
            Self::ClaimRewards => writer.write_all(&CLAIM_REWARDS_IX_DISCM),
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
                borsh::BorshSerialize::serialize(&args.max_open_time, &mut writer)?;
                Ok(())
            }
            Self::CreateRewards(args) => {
                writer.write_all(&CREATE_REWARDS_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.start_time, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.end_time, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.reward_amount, &mut writer)?;
                Ok(())
            }
            Self::CreateSwapReferral(args) => {
                writer.write_all(&CREATE_SWAP_REFERRAL_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.name, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.default_share_bps, &mut writer)?;
                Ok(())
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
            Self::InitUserPoolLiquidity(args) => {
                writer.write_all(&INIT_USER_POOL_LIQUIDITY_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.partner, &mut writer)?;
                Ok(())
            }
            Self::Initialize(args) => {
                writer.write_all(&INITIALIZE_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.init_amount_0, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.init_amount_1, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.open_time, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.max_trade_fee_rate, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.volatility_factor, &mut writer)?;
                Ok(())
            }
            Self::InitializePartner(args) => {
                writer.write_all(&INITIALIZE_PARTNER_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.name, &mut writer)?;
                borsh::BorshSerialize::serialize(
                    &args.token_0_token_account,
                    &mut writer,
                )?;
                borsh::BorshSerialize::serialize(
                    &args.token_1_token_account,
                    &mut writer,
                )?;
                Ok(())
            }
            Self::InitializePoolPartners => {
                writer.write_all(&INITIALIZE_POOL_PARTNERS_IX_DISCM)
            }
            Self::OracleBasedSwapBaseInput(args) => {
                writer.write_all(&ORACLE_BASED_SWAP_BASE_INPUT_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.amount_in, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.minimum_amount_out, &mut writer)?;
                Ok(())
            }
            Self::PoolConfigUpdate(args) => {
                writer.write_all(&POOL_CONFIG_UPDATE_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.h1, &mut writer)?;
                Ok(())
            }
            Self::PoolConfigUpdateV2(args) => {
                writer.write_all(&POOL_CONFIG_UPDATE_V2_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.h1, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.price_fetched_at, &mut writer)?;
                Ok(())
            }
            Self::PoolConfigUpdateV3(args) => {
                writer.write_all(&POOL_CONFIG_UPDATE_V3_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.h1, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.h2, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.price_fetched_at, &mut writer)?;
                borsh::BorshSerialize::serialize(
                    &args.min_trade_rate_at_oracle_price,
                    &mut writer,
                )?;
                borsh::BorshSerialize::serialize(
                    &args.oracle_price_delay_fee_rate_per_second,
                    &mut writer,
                )?;
                borsh::BorshSerialize::serialize(
                    &args.max_oracle_price_delay_fee,
                    &mut writer,
                )?;
                borsh::BorshSerialize::serialize(&args.drift_factor, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.max_drift_factor, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.volatility_factor, &mut writer)?;
                Ok(())
            }
            Self::RebalanceKamino => writer.write_all(&REBALANCE_KAMINO_IX_DISCM),
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
            Self::UpdatePartner(args) => {
                writer.write_all(&UPDATE_PARTNER_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.token_account_0, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.token_account_1, &mut writer)?;
                Ok(())
            }
            Self::UpdatePartnerFees => writer.write_all(&UPDATE_PARTNER_FEES_IX_DISCM),
            Self::UpdatePool(args) => {
                writer.write_all(&UPDATE_POOL_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.param, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.value, &mut writer)?;
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
pub const ADD_OR_UPDATE_ACTOR_IX_ACCOUNTS_LEN: usize = 4;
#[derive(Copy, Clone, Debug)]
pub struct AddOrUpdateActorAccounts<'me, 'info> {
    pub owner: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub actor: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct AddOrUpdateActorKeys {
    pub owner: Pubkey,
    pub system_program: Pubkey,
    pub actor: Pubkey,
    pub authority: Pubkey,
}
impl From<AddOrUpdateActorAccounts<'_, '_>> for AddOrUpdateActorKeys {
    fn from(accounts: AddOrUpdateActorAccounts) -> Self {
        Self {
            owner: *accounts.owner.key,
            system_program: *accounts.system_program.key,
            actor: *accounts.actor.key,
            authority: *accounts.authority.key,
        }
    }
}
impl From<AddOrUpdateActorKeys> for [AccountMeta; ADD_OR_UPDATE_ACTOR_IX_ACCOUNTS_LEN] {
    fn from(keys: AddOrUpdateActorKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.owner,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.system_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.actor,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.authority,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; ADD_OR_UPDATE_ACTOR_IX_ACCOUNTS_LEN]> for AddOrUpdateActorKeys {
    fn from(pubkeys: [Pubkey; ADD_OR_UPDATE_ACTOR_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            owner: pubkeys[0],
            system_program: pubkeys[1],
            actor: pubkeys[2],
            authority: pubkeys[3],
        }
    }
}
impl<'info> From<AddOrUpdateActorAccounts<'_, 'info>>
for [AccountInfo<'info>; ADD_OR_UPDATE_ACTOR_IX_ACCOUNTS_LEN] {
    fn from(accounts: AddOrUpdateActorAccounts<'_, 'info>) -> Self {
        [
            accounts.owner.clone(),
            accounts.system_program.clone(),
            accounts.actor.clone(),
            accounts.authority.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; ADD_OR_UPDATE_ACTOR_IX_ACCOUNTS_LEN]>
for AddOrUpdateActorAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; ADD_OR_UPDATE_ACTOR_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            owner: &arr[0],
            system_program: &arr[1],
            actor: &arr[2],
            authority: &arr[3],
        }
    }
}
pub const ADD_OR_UPDATE_ACTOR_IX_DISCM: [u8; 8usize] = [
    113, 136, 228, 230, 107, 185, 128, 227,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AddOrUpdateActorIxArgs {
    pub is_admin: bool,
}
#[derive(Clone, Debug, PartialEq)]
pub struct AddOrUpdateActorIxData(pub AddOrUpdateActorIxArgs);
impl From<AddOrUpdateActorIxArgs> for AddOrUpdateActorIxData {
    fn from(args: AddOrUpdateActorIxArgs) -> Self {
        Self(args)
    }
}
impl AddOrUpdateActorIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != ADD_OR_UPDATE_ACTOR_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let is_admin: bool = crate::borsh_de_or_default(&mut reader)?;
        Ok(Self(AddOrUpdateActorIxArgs { is_admin }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&ADD_OR_UPDATE_ACTOR_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.is_admin, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn add_or_update_actor_ix_with_program_id(
    program_id: Pubkey,
    keys: AddOrUpdateActorKeys,
    args: AddOrUpdateActorIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; ADD_OR_UPDATE_ACTOR_IX_ACCOUNTS_LEN] = keys.into();
    let data: AddOrUpdateActorIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn add_or_update_actor_ix(
    keys: AddOrUpdateActorKeys,
    args: AddOrUpdateActorIxArgs,
) -> std::io::Result<Instruction> {
    add_or_update_actor_ix_with_program_id(GAMMA_PROGRAM_ID, keys, args)
}
pub fn add_or_update_actor_invoke_with_program_id(
    program_id: Pubkey,
    accounts: AddOrUpdateActorAccounts<'_, '_>,
    args: AddOrUpdateActorIxArgs,
) -> ProgramResult {
    let keys: AddOrUpdateActorKeys = accounts.into();
    let ix = add_or_update_actor_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn add_or_update_actor_invoke(
    accounts: AddOrUpdateActorAccounts<'_, '_>,
    args: AddOrUpdateActorIxArgs,
) -> ProgramResult {
    add_or_update_actor_invoke_with_program_id(GAMMA_PROGRAM_ID, accounts, args)
}
pub fn add_or_update_actor_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: AddOrUpdateActorAccounts<'_, '_>,
    args: AddOrUpdateActorIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: AddOrUpdateActorKeys = accounts.into();
    let ix = add_or_update_actor_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn add_or_update_actor_invoke_signed(
    accounts: AddOrUpdateActorAccounts<'_, '_>,
    args: AddOrUpdateActorIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    add_or_update_actor_invoke_signed_with_program_id(
        GAMMA_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn add_or_update_actor_verify_account_keys(
    accounts: AddOrUpdateActorAccounts<'_, '_>,
    keys: AddOrUpdateActorKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.owner.key, keys.owner),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.actor.key, keys.actor),
        (*accounts.authority.key, keys.authority),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn add_or_update_actor_verify_writable_privileges<'me, 'info>(
    accounts: AddOrUpdateActorAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.owner, accounts.actor] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn add_or_update_actor_verify_signer_privileges<'me, 'info>(
    accounts: AddOrUpdateActorAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.owner] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn add_or_update_actor_verify_account_privileges<'me, 'info>(
    accounts: AddOrUpdateActorAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    add_or_update_actor_verify_writable_privileges(accounts)?;
    add_or_update_actor_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const ADD_PARTNER_IX_ACCOUNTS_LEN: usize = 5;
#[derive(Copy, Clone, Debug)]
pub struct AddPartnerAccounts<'me, 'info> {
    pub authority: &'me AccountInfo<'info>,
    pub amm_config: &'me AccountInfo<'info>,
    pub pool_state: &'me AccountInfo<'info>,
    pub pool_partners: &'me AccountInfo<'info>,
    pub partner: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct AddPartnerKeys {
    pub authority: Pubkey,
    pub amm_config: Pubkey,
    pub pool_state: Pubkey,
    pub pool_partners: Pubkey,
    pub partner: Pubkey,
}
impl From<AddPartnerAccounts<'_, '_>> for AddPartnerKeys {
    fn from(accounts: AddPartnerAccounts) -> Self {
        Self {
            authority: *accounts.authority.key,
            amm_config: *accounts.amm_config.key,
            pool_state: *accounts.pool_state.key,
            pool_partners: *accounts.pool_partners.key,
            partner: *accounts.partner.key,
        }
    }
}
impl From<AddPartnerKeys> for [AccountMeta; ADD_PARTNER_IX_ACCOUNTS_LEN] {
    fn from(keys: AddPartnerKeys) -> Self {
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
                pubkey: keys.pool_partners,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.partner,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; ADD_PARTNER_IX_ACCOUNTS_LEN]> for AddPartnerKeys {
    fn from(pubkeys: [Pubkey; ADD_PARTNER_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            authority: pubkeys[0],
            amm_config: pubkeys[1],
            pool_state: pubkeys[2],
            pool_partners: pubkeys[3],
            partner: pubkeys[4],
        }
    }
}
impl<'info> From<AddPartnerAccounts<'_, 'info>>
for [AccountInfo<'info>; ADD_PARTNER_IX_ACCOUNTS_LEN] {
    fn from(accounts: AddPartnerAccounts<'_, 'info>) -> Self {
        [
            accounts.authority.clone(),
            accounts.amm_config.clone(),
            accounts.pool_state.clone(),
            accounts.pool_partners.clone(),
            accounts.partner.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; ADD_PARTNER_IX_ACCOUNTS_LEN]>
for AddPartnerAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; ADD_PARTNER_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            authority: &arr[0],
            amm_config: &arr[1],
            pool_state: &arr[2],
            pool_partners: &arr[3],
            partner: &arr[4],
        }
    }
}
pub const ADD_PARTNER_IX_DISCM: [u8; 8usize] = [180, 111, 45, 157, 241, 187, 234, 88];
#[derive(Clone, Debug, PartialEq)]
pub struct AddPartnerIxData;
impl AddPartnerIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != ADD_PARTNER_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&ADD_PARTNER_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn add_partner_ix_with_program_id(
    program_id: Pubkey,
    keys: AddPartnerKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; ADD_PARTNER_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: AddPartnerIxData.try_to_vec()?,
    })
}
pub fn add_partner_ix(keys: AddPartnerKeys) -> std::io::Result<Instruction> {
    add_partner_ix_with_program_id(GAMMA_PROGRAM_ID, keys)
}
pub fn add_partner_invoke_with_program_id(
    program_id: Pubkey,
    accounts: AddPartnerAccounts<'_, '_>,
) -> ProgramResult {
    let keys: AddPartnerKeys = accounts.into();
    let ix = add_partner_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn add_partner_invoke(accounts: AddPartnerAccounts<'_, '_>) -> ProgramResult {
    add_partner_invoke_with_program_id(GAMMA_PROGRAM_ID, accounts)
}
pub fn add_partner_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: AddPartnerAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: AddPartnerKeys = accounts.into();
    let ix = add_partner_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn add_partner_invoke_signed(
    accounts: AddPartnerAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    add_partner_invoke_signed_with_program_id(GAMMA_PROGRAM_ID, accounts, seeds)
}
pub fn add_partner_verify_account_keys(
    accounts: AddPartnerAccounts<'_, '_>,
    keys: AddPartnerKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.authority.key, keys.authority),
        (*accounts.amm_config.key, keys.amm_config),
        (*accounts.pool_state.key, keys.pool_state),
        (*accounts.pool_partners.key, keys.pool_partners),
        (*accounts.partner.key, keys.partner),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn add_partner_verify_writable_privileges<'me, 'info>(
    accounts: AddPartnerAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.pool_state, accounts.pool_partners] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn add_partner_verify_signer_privileges<'me, 'info>(
    accounts: AddPartnerAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn add_partner_verify_account_privileges<'me, 'info>(
    accounts: AddPartnerAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    add_partner_verify_writable_privileges(accounts)?;
    add_partner_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CALCULATE_REWARDS_IX_ACCOUNTS_LEN: usize = 7;
#[derive(Copy, Clone, Debug)]
pub struct CalculateRewardsAccounts<'me, 'info> {
    pub signer: &'me AccountInfo<'info>,
    pub user: &'me AccountInfo<'info>,
    pub pool_state: &'me AccountInfo<'info>,
    pub reward_info: &'me AccountInfo<'info>,
    pub user_reward_info: &'me AccountInfo<'info>,
    pub user_pool_liquidity: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CalculateRewardsKeys {
    pub signer: Pubkey,
    pub user: Pubkey,
    pub pool_state: Pubkey,
    pub reward_info: Pubkey,
    pub user_reward_info: Pubkey,
    pub user_pool_liquidity: Pubkey,
    pub system_program: Pubkey,
}
impl From<CalculateRewardsAccounts<'_, '_>> for CalculateRewardsKeys {
    fn from(accounts: CalculateRewardsAccounts) -> Self {
        Self {
            signer: *accounts.signer.key,
            user: *accounts.user.key,
            pool_state: *accounts.pool_state.key,
            reward_info: *accounts.reward_info.key,
            user_reward_info: *accounts.user_reward_info.key,
            user_pool_liquidity: *accounts.user_pool_liquidity.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<CalculateRewardsKeys> for [AccountMeta; CALCULATE_REWARDS_IX_ACCOUNTS_LEN] {
    fn from(keys: CalculateRewardsKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.signer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_state,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.reward_info,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_reward_info,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_pool_liquidity,
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
impl From<[Pubkey; CALCULATE_REWARDS_IX_ACCOUNTS_LEN]> for CalculateRewardsKeys {
    fn from(pubkeys: [Pubkey; CALCULATE_REWARDS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            signer: pubkeys[0],
            user: pubkeys[1],
            pool_state: pubkeys[2],
            reward_info: pubkeys[3],
            user_reward_info: pubkeys[4],
            user_pool_liquidity: pubkeys[5],
            system_program: pubkeys[6],
        }
    }
}
impl<'info> From<CalculateRewardsAccounts<'_, 'info>>
for [AccountInfo<'info>; CALCULATE_REWARDS_IX_ACCOUNTS_LEN] {
    fn from(accounts: CalculateRewardsAccounts<'_, 'info>) -> Self {
        [
            accounts.signer.clone(),
            accounts.user.clone(),
            accounts.pool_state.clone(),
            accounts.reward_info.clone(),
            accounts.user_reward_info.clone(),
            accounts.user_pool_liquidity.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CALCULATE_REWARDS_IX_ACCOUNTS_LEN]>
for CalculateRewardsAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; CALCULATE_REWARDS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            signer: &arr[0],
            user: &arr[1],
            pool_state: &arr[2],
            reward_info: &arr[3],
            user_reward_info: &arr[4],
            user_pool_liquidity: &arr[5],
            system_program: &arr[6],
        }
    }
}
pub const CALCULATE_REWARDS_IX_DISCM: [u8; 8usize] = [
    199, 115, 201, 124, 71, 81, 143, 252,
];
#[derive(Clone, Debug, PartialEq)]
pub struct CalculateRewardsIxData;
impl CalculateRewardsIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CALCULATE_REWARDS_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CALCULATE_REWARDS_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn calculate_rewards_ix_with_program_id(
    program_id: Pubkey,
    keys: CalculateRewardsKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CALCULATE_REWARDS_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: CalculateRewardsIxData.try_to_vec()?,
    })
}
pub fn calculate_rewards_ix(keys: CalculateRewardsKeys) -> std::io::Result<Instruction> {
    calculate_rewards_ix_with_program_id(GAMMA_PROGRAM_ID, keys)
}
pub fn calculate_rewards_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CalculateRewardsAccounts<'_, '_>,
) -> ProgramResult {
    let keys: CalculateRewardsKeys = accounts.into();
    let ix = calculate_rewards_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn calculate_rewards_invoke(
    accounts: CalculateRewardsAccounts<'_, '_>,
) -> ProgramResult {
    calculate_rewards_invoke_with_program_id(GAMMA_PROGRAM_ID, accounts)
}
pub fn calculate_rewards_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CalculateRewardsAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CalculateRewardsKeys = accounts.into();
    let ix = calculate_rewards_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn calculate_rewards_invoke_signed(
    accounts: CalculateRewardsAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    calculate_rewards_invoke_signed_with_program_id(GAMMA_PROGRAM_ID, accounts, seeds)
}
pub fn calculate_rewards_verify_account_keys(
    accounts: CalculateRewardsAccounts<'_, '_>,
    keys: CalculateRewardsKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.signer.key, keys.signer),
        (*accounts.user.key, keys.user),
        (*accounts.pool_state.key, keys.pool_state),
        (*accounts.reward_info.key, keys.reward_info),
        (*accounts.user_reward_info.key, keys.user_reward_info),
        (*accounts.user_pool_liquidity.key, keys.user_pool_liquidity),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn calculate_rewards_verify_writable_privileges<'me, 'info>(
    accounts: CalculateRewardsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.signer,
        accounts.reward_info,
        accounts.user_reward_info,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn calculate_rewards_verify_signer_privileges<'me, 'info>(
    accounts: CalculateRewardsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.signer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn calculate_rewards_verify_account_privileges<'me, 'info>(
    accounts: CalculateRewardsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    calculate_rewards_verify_writable_privileges(accounts)?;
    calculate_rewards_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CLAIM_PARTNER_FEES_IX_ACCOUNTS_LEN: usize = 12;
#[derive(Copy, Clone, Debug)]
pub struct ClaimPartnerFeesAccounts<'me, 'info> {
    pub partner: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub pool_state: &'me AccountInfo<'info>,
    pub token_0_vault: &'me AccountInfo<'info>,
    pub token_1_vault: &'me AccountInfo<'info>,
    pub vault_0_mint: &'me AccountInfo<'info>,
    pub vault_1_mint: &'me AccountInfo<'info>,
    pub pool_partners: &'me AccountInfo<'info>,
    pub token_0_token_account: &'me AccountInfo<'info>,
    pub token_1_token_account: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub token_program_2022: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ClaimPartnerFeesKeys {
    pub partner: Pubkey,
    pub authority: Pubkey,
    pub pool_state: Pubkey,
    pub token_0_vault: Pubkey,
    pub token_1_vault: Pubkey,
    pub vault_0_mint: Pubkey,
    pub vault_1_mint: Pubkey,
    pub pool_partners: Pubkey,
    pub token_0_token_account: Pubkey,
    pub token_1_token_account: Pubkey,
    pub token_program: Pubkey,
    pub token_program_2022: Pubkey,
}
impl From<ClaimPartnerFeesAccounts<'_, '_>> for ClaimPartnerFeesKeys {
    fn from(accounts: ClaimPartnerFeesAccounts) -> Self {
        Self {
            partner: *accounts.partner.key,
            authority: *accounts.authority.key,
            pool_state: *accounts.pool_state.key,
            token_0_vault: *accounts.token_0_vault.key,
            token_1_vault: *accounts.token_1_vault.key,
            vault_0_mint: *accounts.vault_0_mint.key,
            vault_1_mint: *accounts.vault_1_mint.key,
            pool_partners: *accounts.pool_partners.key,
            token_0_token_account: *accounts.token_0_token_account.key,
            token_1_token_account: *accounts.token_1_token_account.key,
            token_program: *accounts.token_program.key,
            token_program_2022: *accounts.token_program_2022.key,
        }
    }
}
impl From<ClaimPartnerFeesKeys> for [AccountMeta; CLAIM_PARTNER_FEES_IX_ACCOUNTS_LEN] {
    fn from(keys: ClaimPartnerFeesKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.partner,
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
                pubkey: keys.pool_partners,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_0_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_1_token_account,
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
impl From<[Pubkey; CLAIM_PARTNER_FEES_IX_ACCOUNTS_LEN]> for ClaimPartnerFeesKeys {
    fn from(pubkeys: [Pubkey; CLAIM_PARTNER_FEES_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            partner: pubkeys[0],
            authority: pubkeys[1],
            pool_state: pubkeys[2],
            token_0_vault: pubkeys[3],
            token_1_vault: pubkeys[4],
            vault_0_mint: pubkeys[5],
            vault_1_mint: pubkeys[6],
            pool_partners: pubkeys[7],
            token_0_token_account: pubkeys[8],
            token_1_token_account: pubkeys[9],
            token_program: pubkeys[10],
            token_program_2022: pubkeys[11],
        }
    }
}
impl<'info> From<ClaimPartnerFeesAccounts<'_, 'info>>
for [AccountInfo<'info>; CLAIM_PARTNER_FEES_IX_ACCOUNTS_LEN] {
    fn from(accounts: ClaimPartnerFeesAccounts<'_, 'info>) -> Self {
        [
            accounts.partner.clone(),
            accounts.authority.clone(),
            accounts.pool_state.clone(),
            accounts.token_0_vault.clone(),
            accounts.token_1_vault.clone(),
            accounts.vault_0_mint.clone(),
            accounts.vault_1_mint.clone(),
            accounts.pool_partners.clone(),
            accounts.token_0_token_account.clone(),
            accounts.token_1_token_account.clone(),
            accounts.token_program.clone(),
            accounts.token_program_2022.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CLAIM_PARTNER_FEES_IX_ACCOUNTS_LEN]>
for ClaimPartnerFeesAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; CLAIM_PARTNER_FEES_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            partner: &arr[0],
            authority: &arr[1],
            pool_state: &arr[2],
            token_0_vault: &arr[3],
            token_1_vault: &arr[4],
            vault_0_mint: &arr[5],
            vault_1_mint: &arr[6],
            pool_partners: &arr[7],
            token_0_token_account: &arr[8],
            token_1_token_account: &arr[9],
            token_program: &arr[10],
            token_program_2022: &arr[11],
        }
    }
}
pub const CLAIM_PARTNER_FEES_IX_DISCM: [u8; 8usize] = [
    114, 71, 103, 57, 160, 205, 242, 185,
];
#[derive(Clone, Debug, PartialEq)]
pub struct ClaimPartnerFeesIxData;
impl ClaimPartnerFeesIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CLAIM_PARTNER_FEES_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CLAIM_PARTNER_FEES_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn claim_partner_fees_ix_with_program_id(
    program_id: Pubkey,
    keys: ClaimPartnerFeesKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CLAIM_PARTNER_FEES_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: ClaimPartnerFeesIxData.try_to_vec()?,
    })
}
pub fn claim_partner_fees_ix(
    keys: ClaimPartnerFeesKeys,
) -> std::io::Result<Instruction> {
    claim_partner_fees_ix_with_program_id(GAMMA_PROGRAM_ID, keys)
}
pub fn claim_partner_fees_invoke_with_program_id(
    program_id: Pubkey,
    accounts: ClaimPartnerFeesAccounts<'_, '_>,
) -> ProgramResult {
    let keys: ClaimPartnerFeesKeys = accounts.into();
    let ix = claim_partner_fees_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn claim_partner_fees_invoke(
    accounts: ClaimPartnerFeesAccounts<'_, '_>,
) -> ProgramResult {
    claim_partner_fees_invoke_with_program_id(GAMMA_PROGRAM_ID, accounts)
}
pub fn claim_partner_fees_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: ClaimPartnerFeesAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: ClaimPartnerFeesKeys = accounts.into();
    let ix = claim_partner_fees_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn claim_partner_fees_invoke_signed(
    accounts: ClaimPartnerFeesAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    claim_partner_fees_invoke_signed_with_program_id(GAMMA_PROGRAM_ID, accounts, seeds)
}
pub fn claim_partner_fees_verify_account_keys(
    accounts: ClaimPartnerFeesAccounts<'_, '_>,
    keys: ClaimPartnerFeesKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.partner.key, keys.partner),
        (*accounts.authority.key, keys.authority),
        (*accounts.pool_state.key, keys.pool_state),
        (*accounts.token_0_vault.key, keys.token_0_vault),
        (*accounts.token_1_vault.key, keys.token_1_vault),
        (*accounts.vault_0_mint.key, keys.vault_0_mint),
        (*accounts.vault_1_mint.key, keys.vault_1_mint),
        (*accounts.pool_partners.key, keys.pool_partners),
        (*accounts.token_0_token_account.key, keys.token_0_token_account),
        (*accounts.token_1_token_account.key, keys.token_1_token_account),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.token_program_2022.key, keys.token_program_2022),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn claim_partner_fees_verify_writable_privileges<'me, 'info>(
    accounts: ClaimPartnerFeesAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.token_0_vault,
        accounts.token_1_vault,
        accounts.pool_partners,
        accounts.token_0_token_account,
        accounts.token_1_token_account,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn claim_partner_fees_verify_account_privileges<'me, 'info>(
    accounts: ClaimPartnerFeesAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    claim_partner_fees_verify_writable_privileges(accounts)?;
    Ok(())
}
pub const CLAIM_REWARDS_IX_ACCOUNTS_LEN: usize = 11;
#[derive(Copy, Clone, Debug)]
pub struct ClaimRewardsAccounts<'me, 'info> {
    pub user: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub pool_state: &'me AccountInfo<'info>,
    pub reward_info: &'me AccountInfo<'info>,
    pub reward_vault: &'me AccountInfo<'info>,
    pub user_token_account: &'me AccountInfo<'info>,
    pub user_reward_info: &'me AccountInfo<'info>,
    pub reward_mint: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub token_program_2022: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ClaimRewardsKeys {
    pub user: Pubkey,
    pub authority: Pubkey,
    pub pool_state: Pubkey,
    pub reward_info: Pubkey,
    pub reward_vault: Pubkey,
    pub user_token_account: Pubkey,
    pub user_reward_info: Pubkey,
    pub reward_mint: Pubkey,
    pub token_program: Pubkey,
    pub token_program_2022: Pubkey,
    pub system_program: Pubkey,
}
impl From<ClaimRewardsAccounts<'_, '_>> for ClaimRewardsKeys {
    fn from(accounts: ClaimRewardsAccounts) -> Self {
        Self {
            user: *accounts.user.key,
            authority: *accounts.authority.key,
            pool_state: *accounts.pool_state.key,
            reward_info: *accounts.reward_info.key,
            reward_vault: *accounts.reward_vault.key,
            user_token_account: *accounts.user_token_account.key,
            user_reward_info: *accounts.user_reward_info.key,
            reward_mint: *accounts.reward_mint.key,
            token_program: *accounts.token_program.key,
            token_program_2022: *accounts.token_program_2022.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<ClaimRewardsKeys> for [AccountMeta; CLAIM_REWARDS_IX_ACCOUNTS_LEN] {
    fn from(keys: ClaimRewardsKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.user,
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
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.reward_info,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.reward_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_reward_info,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.reward_mint,
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
        ]
    }
}
impl From<[Pubkey; CLAIM_REWARDS_IX_ACCOUNTS_LEN]> for ClaimRewardsKeys {
    fn from(pubkeys: [Pubkey; CLAIM_REWARDS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            user: pubkeys[0],
            authority: pubkeys[1],
            pool_state: pubkeys[2],
            reward_info: pubkeys[3],
            reward_vault: pubkeys[4],
            user_token_account: pubkeys[5],
            user_reward_info: pubkeys[6],
            reward_mint: pubkeys[7],
            token_program: pubkeys[8],
            token_program_2022: pubkeys[9],
            system_program: pubkeys[10],
        }
    }
}
impl<'info> From<ClaimRewardsAccounts<'_, 'info>>
for [AccountInfo<'info>; CLAIM_REWARDS_IX_ACCOUNTS_LEN] {
    fn from(accounts: ClaimRewardsAccounts<'_, 'info>) -> Self {
        [
            accounts.user.clone(),
            accounts.authority.clone(),
            accounts.pool_state.clone(),
            accounts.reward_info.clone(),
            accounts.reward_vault.clone(),
            accounts.user_token_account.clone(),
            accounts.user_reward_info.clone(),
            accounts.reward_mint.clone(),
            accounts.token_program.clone(),
            accounts.token_program_2022.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CLAIM_REWARDS_IX_ACCOUNTS_LEN]>
for ClaimRewardsAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; CLAIM_REWARDS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            user: &arr[0],
            authority: &arr[1],
            pool_state: &arr[2],
            reward_info: &arr[3],
            reward_vault: &arr[4],
            user_token_account: &arr[5],
            user_reward_info: &arr[6],
            reward_mint: &arr[7],
            token_program: &arr[8],
            token_program_2022: &arr[9],
            system_program: &arr[10],
        }
    }
}
pub const CLAIM_REWARDS_IX_DISCM: [u8; 8usize] = [4, 144, 132, 71, 116, 23, 151, 80];
#[derive(Clone, Debug, PartialEq)]
pub struct ClaimRewardsIxData;
impl ClaimRewardsIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CLAIM_REWARDS_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CLAIM_REWARDS_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn claim_rewards_ix_with_program_id(
    program_id: Pubkey,
    keys: ClaimRewardsKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CLAIM_REWARDS_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: ClaimRewardsIxData.try_to_vec()?,
    })
}
pub fn claim_rewards_ix(keys: ClaimRewardsKeys) -> std::io::Result<Instruction> {
    claim_rewards_ix_with_program_id(GAMMA_PROGRAM_ID, keys)
}
pub fn claim_rewards_invoke_with_program_id(
    program_id: Pubkey,
    accounts: ClaimRewardsAccounts<'_, '_>,
) -> ProgramResult {
    let keys: ClaimRewardsKeys = accounts.into();
    let ix = claim_rewards_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn claim_rewards_invoke(accounts: ClaimRewardsAccounts<'_, '_>) -> ProgramResult {
    claim_rewards_invoke_with_program_id(GAMMA_PROGRAM_ID, accounts)
}
pub fn claim_rewards_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: ClaimRewardsAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: ClaimRewardsKeys = accounts.into();
    let ix = claim_rewards_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn claim_rewards_invoke_signed(
    accounts: ClaimRewardsAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    claim_rewards_invoke_signed_with_program_id(GAMMA_PROGRAM_ID, accounts, seeds)
}
pub fn claim_rewards_verify_account_keys(
    accounts: ClaimRewardsAccounts<'_, '_>,
    keys: ClaimRewardsKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.user.key, keys.user),
        (*accounts.authority.key, keys.authority),
        (*accounts.pool_state.key, keys.pool_state),
        (*accounts.reward_info.key, keys.reward_info),
        (*accounts.reward_vault.key, keys.reward_vault),
        (*accounts.user_token_account.key, keys.user_token_account),
        (*accounts.user_reward_info.key, keys.user_reward_info),
        (*accounts.reward_mint.key, keys.reward_mint),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.token_program_2022.key, keys.token_program_2022),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn claim_rewards_verify_writable_privileges<'me, 'info>(
    accounts: ClaimRewardsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.user,
        accounts.reward_vault,
        accounts.user_token_account,
        accounts.user_reward_info,
        accounts.reward_mint,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn claim_rewards_verify_signer_privileges<'me, 'info>(
    accounts: ClaimRewardsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.user] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn claim_rewards_verify_account_privileges<'me, 'info>(
    accounts: ClaimRewardsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    claim_rewards_verify_writable_privileges(accounts)?;
    claim_rewards_verify_signer_privileges(accounts)?;
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
    collect_fund_fee_ix_with_program_id(GAMMA_PROGRAM_ID, keys, args)
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
    collect_fund_fee_invoke_with_program_id(GAMMA_PROGRAM_ID, accounts, args)
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
        GAMMA_PROGRAM_ID,
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
    collect_protocol_fee_ix_with_program_id(GAMMA_PROGRAM_ID, keys, args)
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
    collect_protocol_fee_invoke_with_program_id(GAMMA_PROGRAM_ID, accounts, args)
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
        GAMMA_PROGRAM_ID,
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
    pub max_open_time: u64,
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
        let max_open_time: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(CreateAmmConfigIxArgs {
                index,
                trade_fee_rate,
                protocol_fee_rate,
                fund_fee_rate,
                create_pool_fee,
                max_open_time,
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
        borsh::BorshSerialize::serialize(&self.0.max_open_time, &mut writer)?;
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
    create_amm_config_ix_with_program_id(GAMMA_PROGRAM_ID, keys, args)
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
    create_amm_config_invoke_with_program_id(GAMMA_PROGRAM_ID, accounts, args)
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
        GAMMA_PROGRAM_ID,
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
pub const CREATE_REWARDS_IX_ACCOUNTS_LEN: usize = 10;
#[derive(Copy, Clone, Debug)]
pub struct CreateRewardsAccounts<'me, 'info> {
    pub reward_provider: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub pool_state: &'me AccountInfo<'info>,
    pub reward_info: &'me AccountInfo<'info>,
    pub reward_providers_token_account: &'me AccountInfo<'info>,
    pub reward_vault: &'me AccountInfo<'info>,
    pub reward_mint: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub token_program_2022: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CreateRewardsKeys {
    pub reward_provider: Pubkey,
    pub authority: Pubkey,
    pub pool_state: Pubkey,
    pub reward_info: Pubkey,
    pub reward_providers_token_account: Pubkey,
    pub reward_vault: Pubkey,
    pub reward_mint: Pubkey,
    pub token_program: Pubkey,
    pub token_program_2022: Pubkey,
    pub system_program: Pubkey,
}
impl From<CreateRewardsAccounts<'_, '_>> for CreateRewardsKeys {
    fn from(accounts: CreateRewardsAccounts) -> Self {
        Self {
            reward_provider: *accounts.reward_provider.key,
            authority: *accounts.authority.key,
            pool_state: *accounts.pool_state.key,
            reward_info: *accounts.reward_info.key,
            reward_providers_token_account: *accounts.reward_providers_token_account.key,
            reward_vault: *accounts.reward_vault.key,
            reward_mint: *accounts.reward_mint.key,
            token_program: *accounts.token_program.key,
            token_program_2022: *accounts.token_program_2022.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<CreateRewardsKeys> for [AccountMeta; CREATE_REWARDS_IX_ACCOUNTS_LEN] {
    fn from(keys: CreateRewardsKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.reward_provider,
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
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.reward_info,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.reward_providers_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.reward_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.reward_mint,
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
        ]
    }
}
impl From<[Pubkey; CREATE_REWARDS_IX_ACCOUNTS_LEN]> for CreateRewardsKeys {
    fn from(pubkeys: [Pubkey; CREATE_REWARDS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            reward_provider: pubkeys[0],
            authority: pubkeys[1],
            pool_state: pubkeys[2],
            reward_info: pubkeys[3],
            reward_providers_token_account: pubkeys[4],
            reward_vault: pubkeys[5],
            reward_mint: pubkeys[6],
            token_program: pubkeys[7],
            token_program_2022: pubkeys[8],
            system_program: pubkeys[9],
        }
    }
}
impl<'info> From<CreateRewardsAccounts<'_, 'info>>
for [AccountInfo<'info>; CREATE_REWARDS_IX_ACCOUNTS_LEN] {
    fn from(accounts: CreateRewardsAccounts<'_, 'info>) -> Self {
        [
            accounts.reward_provider.clone(),
            accounts.authority.clone(),
            accounts.pool_state.clone(),
            accounts.reward_info.clone(),
            accounts.reward_providers_token_account.clone(),
            accounts.reward_vault.clone(),
            accounts.reward_mint.clone(),
            accounts.token_program.clone(),
            accounts.token_program_2022.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CREATE_REWARDS_IX_ACCOUNTS_LEN]>
for CreateRewardsAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; CREATE_REWARDS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            reward_provider: &arr[0],
            authority: &arr[1],
            pool_state: &arr[2],
            reward_info: &arr[3],
            reward_providers_token_account: &arr[4],
            reward_vault: &arr[5],
            reward_mint: &arr[6],
            token_program: &arr[7],
            token_program_2022: &arr[8],
            system_program: &arr[9],
        }
    }
}
pub const CREATE_REWARDS_IX_DISCM: [u8; 8usize] = [124, 251, 145, 232, 5, 173, 159, 223];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CreateRewardsIxArgs {
    pub start_time: u64,
    pub end_time: u64,
    pub reward_amount: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct CreateRewardsIxData(pub CreateRewardsIxArgs);
impl From<CreateRewardsIxArgs> for CreateRewardsIxData {
    fn from(args: CreateRewardsIxArgs) -> Self {
        Self(args)
    }
}
impl CreateRewardsIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CREATE_REWARDS_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let start_time: u64 = crate::borsh_de_or_default(&mut reader)?;
        let end_time: u64 = crate::borsh_de_or_default(&mut reader)?;
        let reward_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(CreateRewardsIxArgs {
                start_time,
                end_time,
                reward_amount,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CREATE_REWARDS_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.start_time, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.end_time, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.reward_amount, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn create_rewards_ix_with_program_id(
    program_id: Pubkey,
    keys: CreateRewardsKeys,
    args: CreateRewardsIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CREATE_REWARDS_IX_ACCOUNTS_LEN] = keys.into();
    let data: CreateRewardsIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn create_rewards_ix(
    keys: CreateRewardsKeys,
    args: CreateRewardsIxArgs,
) -> std::io::Result<Instruction> {
    create_rewards_ix_with_program_id(GAMMA_PROGRAM_ID, keys, args)
}
pub fn create_rewards_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CreateRewardsAccounts<'_, '_>,
    args: CreateRewardsIxArgs,
) -> ProgramResult {
    let keys: CreateRewardsKeys = accounts.into();
    let ix = create_rewards_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn create_rewards_invoke(
    accounts: CreateRewardsAccounts<'_, '_>,
    args: CreateRewardsIxArgs,
) -> ProgramResult {
    create_rewards_invoke_with_program_id(GAMMA_PROGRAM_ID, accounts, args)
}
pub fn create_rewards_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CreateRewardsAccounts<'_, '_>,
    args: CreateRewardsIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CreateRewardsKeys = accounts.into();
    let ix = create_rewards_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn create_rewards_invoke_signed(
    accounts: CreateRewardsAccounts<'_, '_>,
    args: CreateRewardsIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    create_rewards_invoke_signed_with_program_id(GAMMA_PROGRAM_ID, accounts, args, seeds)
}
pub fn create_rewards_verify_account_keys(
    accounts: CreateRewardsAccounts<'_, '_>,
    keys: CreateRewardsKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.reward_provider.key, keys.reward_provider),
        (*accounts.authority.key, keys.authority),
        (*accounts.pool_state.key, keys.pool_state),
        (*accounts.reward_info.key, keys.reward_info),
        (
            *accounts.reward_providers_token_account.key,
            keys.reward_providers_token_account,
        ),
        (*accounts.reward_vault.key, keys.reward_vault),
        (*accounts.reward_mint.key, keys.reward_mint),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.token_program_2022.key, keys.token_program_2022),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn create_rewards_verify_writable_privileges<'me, 'info>(
    accounts: CreateRewardsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.reward_provider,
        accounts.reward_info,
        accounts.reward_providers_token_account,
        accounts.reward_vault,
        accounts.reward_mint,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn create_rewards_verify_signer_privileges<'me, 'info>(
    accounts: CreateRewardsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.reward_provider] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn create_rewards_verify_account_privileges<'me, 'info>(
    accounts: CreateRewardsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    create_rewards_verify_writable_privileges(accounts)?;
    create_rewards_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CREATE_SWAP_REFERRAL_IX_ACCOUNTS_LEN: usize = 7;
#[derive(Copy, Clone, Debug)]
pub struct CreateSwapReferralAccounts<'me, 'info> {
    pub admin: &'me AccountInfo<'info>,
    pub owner: &'me AccountInfo<'info>,
    pub payer: &'me AccountInfo<'info>,
    pub amm_config: &'me AccountInfo<'info>,
    pub project: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub referral_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CreateSwapReferralKeys {
    pub admin: Pubkey,
    pub owner: Pubkey,
    pub payer: Pubkey,
    pub amm_config: Pubkey,
    pub project: Pubkey,
    pub system_program: Pubkey,
    pub referral_program: Pubkey,
}
impl From<CreateSwapReferralAccounts<'_, '_>> for CreateSwapReferralKeys {
    fn from(accounts: CreateSwapReferralAccounts) -> Self {
        Self {
            admin: *accounts.admin.key,
            owner: *accounts.owner.key,
            payer: *accounts.payer.key,
            amm_config: *accounts.amm_config.key,
            project: *accounts.project.key,
            system_program: *accounts.system_program.key,
            referral_program: *accounts.referral_program.key,
        }
    }
}
impl From<CreateSwapReferralKeys>
for [AccountMeta; CREATE_SWAP_REFERRAL_IX_ACCOUNTS_LEN] {
    fn from(keys: CreateSwapReferralKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.admin,
                is_signer: true,
                is_writable: false,
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
                pubkey: keys.amm_config,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.project,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.system_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.referral_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; CREATE_SWAP_REFERRAL_IX_ACCOUNTS_LEN]> for CreateSwapReferralKeys {
    fn from(pubkeys: [Pubkey; CREATE_SWAP_REFERRAL_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin: pubkeys[0],
            owner: pubkeys[1],
            payer: pubkeys[2],
            amm_config: pubkeys[3],
            project: pubkeys[4],
            system_program: pubkeys[5],
            referral_program: pubkeys[6],
        }
    }
}
impl<'info> From<CreateSwapReferralAccounts<'_, 'info>>
for [AccountInfo<'info>; CREATE_SWAP_REFERRAL_IX_ACCOUNTS_LEN] {
    fn from(accounts: CreateSwapReferralAccounts<'_, 'info>) -> Self {
        [
            accounts.admin.clone(),
            accounts.owner.clone(),
            accounts.payer.clone(),
            accounts.amm_config.clone(),
            accounts.project.clone(),
            accounts.system_program.clone(),
            accounts.referral_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CREATE_SWAP_REFERRAL_IX_ACCOUNTS_LEN]>
for CreateSwapReferralAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; CREATE_SWAP_REFERRAL_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            admin: &arr[0],
            owner: &arr[1],
            payer: &arr[2],
            amm_config: &arr[3],
            project: &arr[4],
            system_program: &arr[5],
            referral_program: &arr[6],
        }
    }
}
pub const CREATE_SWAP_REFERRAL_IX_DISCM: [u8; 8usize] = [
    67, 131, 93, 236, 56, 6, 40, 77,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CreateSwapReferralIxArgs {
    pub name: String,
    pub default_share_bps: u16,
}
#[derive(Clone, Debug, PartialEq)]
pub struct CreateSwapReferralIxData(pub CreateSwapReferralIxArgs);
impl From<CreateSwapReferralIxArgs> for CreateSwapReferralIxData {
    fn from(args: CreateSwapReferralIxArgs) -> Self {
        Self(args)
    }
}
impl CreateSwapReferralIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CREATE_SWAP_REFERRAL_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let name: String = crate::borsh_de_or_default(&mut reader)?;
        let default_share_bps: u16 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(CreateSwapReferralIxArgs {
                name,
                default_share_bps,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CREATE_SWAP_REFERRAL_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.name, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.default_share_bps, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn create_swap_referral_ix_with_program_id(
    program_id: Pubkey,
    keys: CreateSwapReferralKeys,
    args: CreateSwapReferralIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CREATE_SWAP_REFERRAL_IX_ACCOUNTS_LEN] = keys.into();
    let data: CreateSwapReferralIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn create_swap_referral_ix(
    keys: CreateSwapReferralKeys,
    args: CreateSwapReferralIxArgs,
) -> std::io::Result<Instruction> {
    create_swap_referral_ix_with_program_id(GAMMA_PROGRAM_ID, keys, args)
}
pub fn create_swap_referral_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CreateSwapReferralAccounts<'_, '_>,
    args: CreateSwapReferralIxArgs,
) -> ProgramResult {
    let keys: CreateSwapReferralKeys = accounts.into();
    let ix = create_swap_referral_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn create_swap_referral_invoke(
    accounts: CreateSwapReferralAccounts<'_, '_>,
    args: CreateSwapReferralIxArgs,
) -> ProgramResult {
    create_swap_referral_invoke_with_program_id(GAMMA_PROGRAM_ID, accounts, args)
}
pub fn create_swap_referral_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CreateSwapReferralAccounts<'_, '_>,
    args: CreateSwapReferralIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CreateSwapReferralKeys = accounts.into();
    let ix = create_swap_referral_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn create_swap_referral_invoke_signed(
    accounts: CreateSwapReferralAccounts<'_, '_>,
    args: CreateSwapReferralIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    create_swap_referral_invoke_signed_with_program_id(
        GAMMA_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn create_swap_referral_verify_account_keys(
    accounts: CreateSwapReferralAccounts<'_, '_>,
    keys: CreateSwapReferralKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.admin.key, keys.admin),
        (*accounts.owner.key, keys.owner),
        (*accounts.payer.key, keys.payer),
        (*accounts.amm_config.key, keys.amm_config),
        (*accounts.project.key, keys.project),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.referral_program.key, keys.referral_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn create_swap_referral_verify_writable_privileges<'me, 'info>(
    accounts: CreateSwapReferralAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.payer, accounts.amm_config, accounts.project] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn create_swap_referral_verify_signer_privileges<'me, 'info>(
    accounts: CreateSwapReferralAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin, accounts.payer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn create_swap_referral_verify_account_privileges<'me, 'info>(
    accounts: CreateSwapReferralAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    create_swap_referral_verify_writable_privileges(accounts)?;
    create_swap_referral_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const DEPOSIT_IX_ACCOUNTS_LEN: usize = 13;
#[derive(Copy, Clone, Debug)]
pub struct DepositAccounts<'me, 'info> {
    pub owner: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub pool_state: &'me AccountInfo<'info>,
    pub user_pool_liquidity: &'me AccountInfo<'info>,
    pub token_0_account: &'me AccountInfo<'info>,
    pub token_1_account: &'me AccountInfo<'info>,
    pub token_0_vault: &'me AccountInfo<'info>,
    pub token_1_vault: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub token_program_2022: &'me AccountInfo<'info>,
    pub vault_0_mint: &'me AccountInfo<'info>,
    pub vault_1_mint: &'me AccountInfo<'info>,
    pub pool_partners: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct DepositKeys {
    pub owner: Pubkey,
    pub authority: Pubkey,
    pub pool_state: Pubkey,
    pub user_pool_liquidity: Pubkey,
    pub token_0_account: Pubkey,
    pub token_1_account: Pubkey,
    pub token_0_vault: Pubkey,
    pub token_1_vault: Pubkey,
    pub token_program: Pubkey,
    pub token_program_2022: Pubkey,
    pub vault_0_mint: Pubkey,
    pub vault_1_mint: Pubkey,
    pub pool_partners: Pubkey,
}
impl From<DepositAccounts<'_, '_>> for DepositKeys {
    fn from(accounts: DepositAccounts) -> Self {
        Self {
            owner: *accounts.owner.key,
            authority: *accounts.authority.key,
            pool_state: *accounts.pool_state.key,
            user_pool_liquidity: *accounts.user_pool_liquidity.key,
            token_0_account: *accounts.token_0_account.key,
            token_1_account: *accounts.token_1_account.key,
            token_0_vault: *accounts.token_0_vault.key,
            token_1_vault: *accounts.token_1_vault.key,
            token_program: *accounts.token_program.key,
            token_program_2022: *accounts.token_program_2022.key,
            vault_0_mint: *accounts.vault_0_mint.key,
            vault_1_mint: *accounts.vault_1_mint.key,
            pool_partners: *accounts.pool_partners.key,
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
                pubkey: keys.user_pool_liquidity,
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
                pubkey: keys.pool_partners,
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
            user_pool_liquidity: pubkeys[3],
            token_0_account: pubkeys[4],
            token_1_account: pubkeys[5],
            token_0_vault: pubkeys[6],
            token_1_vault: pubkeys[7],
            token_program: pubkeys[8],
            token_program_2022: pubkeys[9],
            vault_0_mint: pubkeys[10],
            vault_1_mint: pubkeys[11],
            pool_partners: pubkeys[12],
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
            accounts.user_pool_liquidity.clone(),
            accounts.token_0_account.clone(),
            accounts.token_1_account.clone(),
            accounts.token_0_vault.clone(),
            accounts.token_1_vault.clone(),
            accounts.token_program.clone(),
            accounts.token_program_2022.clone(),
            accounts.vault_0_mint.clone(),
            accounts.vault_1_mint.clone(),
            accounts.pool_partners.clone(),
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
            user_pool_liquidity: &arr[3],
            token_0_account: &arr[4],
            token_1_account: &arr[5],
            token_0_vault: &arr[6],
            token_1_vault: &arr[7],
            token_program: &arr[8],
            token_program_2022: &arr[9],
            vault_0_mint: &arr[10],
            vault_1_mint: &arr[11],
            pool_partners: &arr[12],
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
    deposit_ix_with_program_id(GAMMA_PROGRAM_ID, keys, args)
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
    deposit_invoke_with_program_id(GAMMA_PROGRAM_ID, accounts, args)
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
    deposit_invoke_signed_with_program_id(GAMMA_PROGRAM_ID, accounts, args, seeds)
}
pub fn deposit_verify_account_keys(
    accounts: DepositAccounts<'_, '_>,
    keys: DepositKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.owner.key, keys.owner),
        (*accounts.authority.key, keys.authority),
        (*accounts.pool_state.key, keys.pool_state),
        (*accounts.user_pool_liquidity.key, keys.user_pool_liquidity),
        (*accounts.token_0_account.key, keys.token_0_account),
        (*accounts.token_1_account.key, keys.token_1_account),
        (*accounts.token_0_vault.key, keys.token_0_vault),
        (*accounts.token_1_vault.key, keys.token_1_vault),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.token_program_2022.key, keys.token_program_2022),
        (*accounts.vault_0_mint.key, keys.vault_0_mint),
        (*accounts.vault_1_mint.key, keys.vault_1_mint),
        (*accounts.pool_partners.key, keys.pool_partners),
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
        accounts.user_pool_liquidity,
        accounts.token_0_account,
        accounts.token_1_account,
        accounts.token_0_vault,
        accounts.token_1_vault,
        accounts.pool_partners,
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
pub const INIT_USER_POOL_LIQUIDITY_IX_ACCOUNTS_LEN: usize = 5;
#[derive(Copy, Clone, Debug)]
pub struct InitUserPoolLiquidityAccounts<'me, 'info> {
    pub user: &'me AccountInfo<'info>,
    pub pool_state: &'me AccountInfo<'info>,
    pub user_pool_liquidity: &'me AccountInfo<'info>,
    pub pool_partners: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct InitUserPoolLiquidityKeys {
    pub user: Pubkey,
    pub pool_state: Pubkey,
    pub user_pool_liquidity: Pubkey,
    pub pool_partners: Pubkey,
    pub system_program: Pubkey,
}
impl From<InitUserPoolLiquidityAccounts<'_, '_>> for InitUserPoolLiquidityKeys {
    fn from(accounts: InitUserPoolLiquidityAccounts) -> Self {
        Self {
            user: *accounts.user.key,
            pool_state: *accounts.pool_state.key,
            user_pool_liquidity: *accounts.user_pool_liquidity.key,
            pool_partners: *accounts.pool_partners.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<InitUserPoolLiquidityKeys>
for [AccountMeta; INIT_USER_POOL_LIQUIDITY_IX_ACCOUNTS_LEN] {
    fn from(keys: InitUserPoolLiquidityKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.user,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pool_state,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.user_pool_liquidity,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pool_partners,
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
impl From<[Pubkey; INIT_USER_POOL_LIQUIDITY_IX_ACCOUNTS_LEN]>
for InitUserPoolLiquidityKeys {
    fn from(pubkeys: [Pubkey; INIT_USER_POOL_LIQUIDITY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            user: pubkeys[0],
            pool_state: pubkeys[1],
            user_pool_liquidity: pubkeys[2],
            pool_partners: pubkeys[3],
            system_program: pubkeys[4],
        }
    }
}
impl<'info> From<InitUserPoolLiquidityAccounts<'_, 'info>>
for [AccountInfo<'info>; INIT_USER_POOL_LIQUIDITY_IX_ACCOUNTS_LEN] {
    fn from(accounts: InitUserPoolLiquidityAccounts<'_, 'info>) -> Self {
        [
            accounts.user.clone(),
            accounts.pool_state.clone(),
            accounts.user_pool_liquidity.clone(),
            accounts.pool_partners.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; INIT_USER_POOL_LIQUIDITY_IX_ACCOUNTS_LEN]>
for InitUserPoolLiquidityAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; INIT_USER_POOL_LIQUIDITY_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            user: &arr[0],
            pool_state: &arr[1],
            user_pool_liquidity: &arr[2],
            pool_partners: &arr[3],
            system_program: &arr[4],
        }
    }
}
pub const INIT_USER_POOL_LIQUIDITY_IX_DISCM: [u8; 8usize] = [
    227, 221, 200, 212, 36, 107, 149, 36,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct InitUserPoolLiquidityIxArgs {
    pub partner: Option<Pubkey>,
}
#[derive(Clone, Debug, PartialEq)]
pub struct InitUserPoolLiquidityIxData(pub InitUserPoolLiquidityIxArgs);
impl From<InitUserPoolLiquidityIxArgs> for InitUserPoolLiquidityIxData {
    fn from(args: InitUserPoolLiquidityIxArgs) -> Self {
        Self(args)
    }
}
impl InitUserPoolLiquidityIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INIT_USER_POOL_LIQUIDITY_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let partner: Option<Pubkey> = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(InitUserPoolLiquidityIxArgs {
                partner,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INIT_USER_POOL_LIQUIDITY_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.partner, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn init_user_pool_liquidity_ix_with_program_id(
    program_id: Pubkey,
    keys: InitUserPoolLiquidityKeys,
    args: InitUserPoolLiquidityIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; INIT_USER_POOL_LIQUIDITY_IX_ACCOUNTS_LEN] = keys.into();
    let data: InitUserPoolLiquidityIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn init_user_pool_liquidity_ix(
    keys: InitUserPoolLiquidityKeys,
    args: InitUserPoolLiquidityIxArgs,
) -> std::io::Result<Instruction> {
    init_user_pool_liquidity_ix_with_program_id(GAMMA_PROGRAM_ID, keys, args)
}
pub fn init_user_pool_liquidity_invoke_with_program_id(
    program_id: Pubkey,
    accounts: InitUserPoolLiquidityAccounts<'_, '_>,
    args: InitUserPoolLiquidityIxArgs,
) -> ProgramResult {
    let keys: InitUserPoolLiquidityKeys = accounts.into();
    let ix = init_user_pool_liquidity_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn init_user_pool_liquidity_invoke(
    accounts: InitUserPoolLiquidityAccounts<'_, '_>,
    args: InitUserPoolLiquidityIxArgs,
) -> ProgramResult {
    init_user_pool_liquidity_invoke_with_program_id(GAMMA_PROGRAM_ID, accounts, args)
}
pub fn init_user_pool_liquidity_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: InitUserPoolLiquidityAccounts<'_, '_>,
    args: InitUserPoolLiquidityIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: InitUserPoolLiquidityKeys = accounts.into();
    let ix = init_user_pool_liquidity_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn init_user_pool_liquidity_invoke_signed(
    accounts: InitUserPoolLiquidityAccounts<'_, '_>,
    args: InitUserPoolLiquidityIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    init_user_pool_liquidity_invoke_signed_with_program_id(
        GAMMA_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn init_user_pool_liquidity_verify_account_keys(
    accounts: InitUserPoolLiquidityAccounts<'_, '_>,
    keys: InitUserPoolLiquidityKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.user.key, keys.user),
        (*accounts.pool_state.key, keys.pool_state),
        (*accounts.user_pool_liquidity.key, keys.user_pool_liquidity),
        (*accounts.pool_partners.key, keys.pool_partners),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn init_user_pool_liquidity_verify_writable_privileges<'me, 'info>(
    accounts: InitUserPoolLiquidityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.user,
        accounts.user_pool_liquidity,
        accounts.pool_partners,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn init_user_pool_liquidity_verify_signer_privileges<'me, 'info>(
    accounts: InitUserPoolLiquidityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.user] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn init_user_pool_liquidity_verify_account_privileges<'me, 'info>(
    accounts: InitUserPoolLiquidityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    init_user_pool_liquidity_verify_writable_privileges(accounts)?;
    init_user_pool_liquidity_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const INITIALIZE_IX_ACCOUNTS_LEN: usize = 19;
#[derive(Copy, Clone, Debug)]
pub struct InitializeAccounts<'me, 'info> {
    pub creator: &'me AccountInfo<'info>,
    pub amm_config: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub pool_state: &'me AccountInfo<'info>,
    pub user_pool_liquidity: &'me AccountInfo<'info>,
    pub token_0_mint: &'me AccountInfo<'info>,
    pub token_1_mint: &'me AccountInfo<'info>,
    pub creator_token_0: &'me AccountInfo<'info>,
    pub creator_token_1: &'me AccountInfo<'info>,
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
    pub user_pool_liquidity: Pubkey,
    pub token_0_mint: Pubkey,
    pub token_1_mint: Pubkey,
    pub creator_token_0: Pubkey,
    pub creator_token_1: Pubkey,
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
            user_pool_liquidity: *accounts.user_pool_liquidity.key,
            token_0_mint: *accounts.token_0_mint.key,
            token_1_mint: *accounts.token_1_mint.key,
            creator_token_0: *accounts.creator_token_0.key,
            creator_token_1: *accounts.creator_token_1.key,
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
                pubkey: keys.user_pool_liquidity,
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
            user_pool_liquidity: pubkeys[4],
            token_0_mint: pubkeys[5],
            token_1_mint: pubkeys[6],
            creator_token_0: pubkeys[7],
            creator_token_1: pubkeys[8],
            token_0_vault: pubkeys[9],
            token_1_vault: pubkeys[10],
            create_pool_fee: pubkeys[11],
            observation_state: pubkeys[12],
            token_program: pubkeys[13],
            token_0_program: pubkeys[14],
            token_1_program: pubkeys[15],
            associated_token_program: pubkeys[16],
            system_program: pubkeys[17],
            rent: pubkeys[18],
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
            accounts.user_pool_liquidity.clone(),
            accounts.token_0_mint.clone(),
            accounts.token_1_mint.clone(),
            accounts.creator_token_0.clone(),
            accounts.creator_token_1.clone(),
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
            user_pool_liquidity: &arr[4],
            token_0_mint: &arr[5],
            token_1_mint: &arr[6],
            creator_token_0: &arr[7],
            creator_token_1: &arr[8],
            token_0_vault: &arr[9],
            token_1_vault: &arr[10],
            create_pool_fee: &arr[11],
            observation_state: &arr[12],
            token_program: &arr[13],
            token_0_program: &arr[14],
            token_1_program: &arr[15],
            associated_token_program: &arr[16],
            system_program: &arr[17],
            rent: &arr[18],
        }
    }
}
pub const INITIALIZE_IX_DISCM: [u8; 8usize] = [175, 175, 109, 31, 13, 152, 155, 237];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct InitializeIxArgs {
    pub init_amount_0: u64,
    pub init_amount_1: u64,
    pub open_time: u64,
    pub max_trade_fee_rate: u64,
    pub volatility_factor: u64,
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
        let max_trade_fee_rate: u64 = crate::borsh_de_or_default(&mut reader)?;
        let volatility_factor: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(InitializeIxArgs {
                init_amount_0,
                init_amount_1,
                open_time,
                max_trade_fee_rate,
                volatility_factor,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INITIALIZE_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.init_amount_0, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.init_amount_1, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.open_time, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.max_trade_fee_rate, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.volatility_factor, &mut writer)?;
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
    initialize_ix_with_program_id(GAMMA_PROGRAM_ID, keys, args)
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
    initialize_invoke_with_program_id(GAMMA_PROGRAM_ID, accounts, args)
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
    initialize_invoke_signed_with_program_id(GAMMA_PROGRAM_ID, accounts, args, seeds)
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
        (*accounts.user_pool_liquidity.key, keys.user_pool_liquidity),
        (*accounts.token_0_mint.key, keys.token_0_mint),
        (*accounts.token_1_mint.key, keys.token_1_mint),
        (*accounts.creator_token_0.key, keys.creator_token_0),
        (*accounts.creator_token_1.key, keys.creator_token_1),
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
        accounts.user_pool_liquidity,
        accounts.creator_token_0,
        accounts.creator_token_1,
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
pub const INITIALIZE_PARTNER_IX_ACCOUNTS_LEN: usize = 6;
#[derive(Copy, Clone, Debug)]
pub struct InitializePartnerAccounts<'me, 'info> {
    pub payer: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub pool_state: &'me AccountInfo<'info>,
    pub pool_partners: &'me AccountInfo<'info>,
    pub partner: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct InitializePartnerKeys {
    pub payer: Pubkey,
    pub authority: Pubkey,
    pub pool_state: Pubkey,
    pub pool_partners: Pubkey,
    pub partner: Pubkey,
    pub system_program: Pubkey,
}
impl From<InitializePartnerAccounts<'_, '_>> for InitializePartnerKeys {
    fn from(accounts: InitializePartnerAccounts) -> Self {
        Self {
            payer: *accounts.payer.key,
            authority: *accounts.authority.key,
            pool_state: *accounts.pool_state.key,
            pool_partners: *accounts.pool_partners.key,
            partner: *accounts.partner.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<InitializePartnerKeys> for [AccountMeta; INITIALIZE_PARTNER_IX_ACCOUNTS_LEN] {
    fn from(keys: InitializePartnerKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.payer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.authority,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_state,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_partners,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.partner,
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
impl From<[Pubkey; INITIALIZE_PARTNER_IX_ACCOUNTS_LEN]> for InitializePartnerKeys {
    fn from(pubkeys: [Pubkey; INITIALIZE_PARTNER_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            payer: pubkeys[0],
            authority: pubkeys[1],
            pool_state: pubkeys[2],
            pool_partners: pubkeys[3],
            partner: pubkeys[4],
            system_program: pubkeys[5],
        }
    }
}
impl<'info> From<InitializePartnerAccounts<'_, 'info>>
for [AccountInfo<'info>; INITIALIZE_PARTNER_IX_ACCOUNTS_LEN] {
    fn from(accounts: InitializePartnerAccounts<'_, 'info>) -> Self {
        [
            accounts.payer.clone(),
            accounts.authority.clone(),
            accounts.pool_state.clone(),
            accounts.pool_partners.clone(),
            accounts.partner.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; INITIALIZE_PARTNER_IX_ACCOUNTS_LEN]>
for InitializePartnerAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; INITIALIZE_PARTNER_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            payer: &arr[0],
            authority: &arr[1],
            pool_state: &arr[2],
            pool_partners: &arr[3],
            partner: &arr[4],
            system_program: &arr[5],
        }
    }
}
pub const INITIALIZE_PARTNER_IX_DISCM: [u8; 8usize] = [
    165, 62, 179, 112, 129, 50, 173, 144,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct InitializePartnerIxArgs {
    pub name: String,
    pub token_0_token_account: Pubkey,
    pub token_1_token_account: Pubkey,
}
#[derive(Clone, Debug, PartialEq)]
pub struct InitializePartnerIxData(pub InitializePartnerIxArgs);
impl From<InitializePartnerIxArgs> for InitializePartnerIxData {
    fn from(args: InitializePartnerIxArgs) -> Self {
        Self(args)
    }
}
impl InitializePartnerIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INITIALIZE_PARTNER_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let name: String = crate::borsh_de_or_default(&mut reader)?;
        let token_0_token_account: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let token_1_token_account: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(InitializePartnerIxArgs {
                name,
                token_0_token_account,
                token_1_token_account,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INITIALIZE_PARTNER_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.name, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.token_0_token_account, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.token_1_token_account, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn initialize_partner_ix_with_program_id(
    program_id: Pubkey,
    keys: InitializePartnerKeys,
    args: InitializePartnerIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; INITIALIZE_PARTNER_IX_ACCOUNTS_LEN] = keys.into();
    let data: InitializePartnerIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn initialize_partner_ix(
    keys: InitializePartnerKeys,
    args: InitializePartnerIxArgs,
) -> std::io::Result<Instruction> {
    initialize_partner_ix_with_program_id(GAMMA_PROGRAM_ID, keys, args)
}
pub fn initialize_partner_invoke_with_program_id(
    program_id: Pubkey,
    accounts: InitializePartnerAccounts<'_, '_>,
    args: InitializePartnerIxArgs,
) -> ProgramResult {
    let keys: InitializePartnerKeys = accounts.into();
    let ix = initialize_partner_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn initialize_partner_invoke(
    accounts: InitializePartnerAccounts<'_, '_>,
    args: InitializePartnerIxArgs,
) -> ProgramResult {
    initialize_partner_invoke_with_program_id(GAMMA_PROGRAM_ID, accounts, args)
}
pub fn initialize_partner_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: InitializePartnerAccounts<'_, '_>,
    args: InitializePartnerIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: InitializePartnerKeys = accounts.into();
    let ix = initialize_partner_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn initialize_partner_invoke_signed(
    accounts: InitializePartnerAccounts<'_, '_>,
    args: InitializePartnerIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    initialize_partner_invoke_signed_with_program_id(
        GAMMA_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn initialize_partner_verify_account_keys(
    accounts: InitializePartnerAccounts<'_, '_>,
    keys: InitializePartnerKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.payer.key, keys.payer),
        (*accounts.authority.key, keys.authority),
        (*accounts.pool_state.key, keys.pool_state),
        (*accounts.pool_partners.key, keys.pool_partners),
        (*accounts.partner.key, keys.partner),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn initialize_partner_verify_writable_privileges<'me, 'info>(
    accounts: InitializePartnerAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.payer,
        accounts.pool_partners,
        accounts.partner,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn initialize_partner_verify_signer_privileges<'me, 'info>(
    accounts: InitializePartnerAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.payer, accounts.authority, accounts.partner] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn initialize_partner_verify_account_privileges<'me, 'info>(
    accounts: InitializePartnerAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    initialize_partner_verify_writable_privileges(accounts)?;
    initialize_partner_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const INITIALIZE_POOL_PARTNERS_IX_ACCOUNTS_LEN: usize = 4;
#[derive(Copy, Clone, Debug)]
pub struct InitializePoolPartnersAccounts<'me, 'info> {
    pub payer: &'me AccountInfo<'info>,
    pub pool_state: &'me AccountInfo<'info>,
    pub pool_partners: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct InitializePoolPartnersKeys {
    pub payer: Pubkey,
    pub pool_state: Pubkey,
    pub pool_partners: Pubkey,
    pub system_program: Pubkey,
}
impl From<InitializePoolPartnersAccounts<'_, '_>> for InitializePoolPartnersKeys {
    fn from(accounts: InitializePoolPartnersAccounts) -> Self {
        Self {
            payer: *accounts.payer.key,
            pool_state: *accounts.pool_state.key,
            pool_partners: *accounts.pool_partners.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<InitializePoolPartnersKeys>
for [AccountMeta; INITIALIZE_POOL_PARTNERS_IX_ACCOUNTS_LEN] {
    fn from(keys: InitializePoolPartnersKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.payer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pool_state,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_partners,
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
impl From<[Pubkey; INITIALIZE_POOL_PARTNERS_IX_ACCOUNTS_LEN]>
for InitializePoolPartnersKeys {
    fn from(pubkeys: [Pubkey; INITIALIZE_POOL_PARTNERS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            payer: pubkeys[0],
            pool_state: pubkeys[1],
            pool_partners: pubkeys[2],
            system_program: pubkeys[3],
        }
    }
}
impl<'info> From<InitializePoolPartnersAccounts<'_, 'info>>
for [AccountInfo<'info>; INITIALIZE_POOL_PARTNERS_IX_ACCOUNTS_LEN] {
    fn from(accounts: InitializePoolPartnersAccounts<'_, 'info>) -> Self {
        [
            accounts.payer.clone(),
            accounts.pool_state.clone(),
            accounts.pool_partners.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; INITIALIZE_POOL_PARTNERS_IX_ACCOUNTS_LEN]>
for InitializePoolPartnersAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; INITIALIZE_POOL_PARTNERS_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            payer: &arr[0],
            pool_state: &arr[1],
            pool_partners: &arr[2],
            system_program: &arr[3],
        }
    }
}
pub const INITIALIZE_POOL_PARTNERS_IX_DISCM: [u8; 8usize] = [
    99, 11, 108, 186, 1, 126, 209, 251,
];
#[derive(Clone, Debug, PartialEq)]
pub struct InitializePoolPartnersIxData;
impl InitializePoolPartnersIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INITIALIZE_POOL_PARTNERS_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INITIALIZE_POOL_PARTNERS_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn initialize_pool_partners_ix_with_program_id(
    program_id: Pubkey,
    keys: InitializePoolPartnersKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; INITIALIZE_POOL_PARTNERS_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: InitializePoolPartnersIxData.try_to_vec()?,
    })
}
pub fn initialize_pool_partners_ix(
    keys: InitializePoolPartnersKeys,
) -> std::io::Result<Instruction> {
    initialize_pool_partners_ix_with_program_id(GAMMA_PROGRAM_ID, keys)
}
pub fn initialize_pool_partners_invoke_with_program_id(
    program_id: Pubkey,
    accounts: InitializePoolPartnersAccounts<'_, '_>,
) -> ProgramResult {
    let keys: InitializePoolPartnersKeys = accounts.into();
    let ix = initialize_pool_partners_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn initialize_pool_partners_invoke(
    accounts: InitializePoolPartnersAccounts<'_, '_>,
) -> ProgramResult {
    initialize_pool_partners_invoke_with_program_id(GAMMA_PROGRAM_ID, accounts)
}
pub fn initialize_pool_partners_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: InitializePoolPartnersAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: InitializePoolPartnersKeys = accounts.into();
    let ix = initialize_pool_partners_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn initialize_pool_partners_invoke_signed(
    accounts: InitializePoolPartnersAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    initialize_pool_partners_invoke_signed_with_program_id(
        GAMMA_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn initialize_pool_partners_verify_account_keys(
    accounts: InitializePoolPartnersAccounts<'_, '_>,
    keys: InitializePoolPartnersKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.payer.key, keys.payer),
        (*accounts.pool_state.key, keys.pool_state),
        (*accounts.pool_partners.key, keys.pool_partners),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn initialize_pool_partners_verify_writable_privileges<'me, 'info>(
    accounts: InitializePoolPartnersAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.payer, accounts.pool_partners] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn initialize_pool_partners_verify_signer_privileges<'me, 'info>(
    accounts: InitializePoolPartnersAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.payer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn initialize_pool_partners_verify_account_privileges<'me, 'info>(
    accounts: InitializePoolPartnersAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    initialize_pool_partners_verify_writable_privileges(accounts)?;
    initialize_pool_partners_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const ORACLE_BASED_SWAP_BASE_INPUT_IX_ACCOUNTS_LEN: usize = 13;
#[derive(Copy, Clone, Debug)]
pub struct OracleBasedSwapBaseInputAccounts<'me, 'info> {
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
pub struct OracleBasedSwapBaseInputKeys {
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
impl From<OracleBasedSwapBaseInputAccounts<'_, '_>> for OracleBasedSwapBaseInputKeys {
    fn from(accounts: OracleBasedSwapBaseInputAccounts) -> Self {
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
impl From<OracleBasedSwapBaseInputKeys>
for [AccountMeta; ORACLE_BASED_SWAP_BASE_INPUT_IX_ACCOUNTS_LEN] {
    fn from(keys: OracleBasedSwapBaseInputKeys) -> Self {
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
impl From<[Pubkey; ORACLE_BASED_SWAP_BASE_INPUT_IX_ACCOUNTS_LEN]>
for OracleBasedSwapBaseInputKeys {
    fn from(pubkeys: [Pubkey; ORACLE_BASED_SWAP_BASE_INPUT_IX_ACCOUNTS_LEN]) -> Self {
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
impl<'info> From<OracleBasedSwapBaseInputAccounts<'_, 'info>>
for [AccountInfo<'info>; ORACLE_BASED_SWAP_BASE_INPUT_IX_ACCOUNTS_LEN] {
    fn from(accounts: OracleBasedSwapBaseInputAccounts<'_, 'info>) -> Self {
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
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; ORACLE_BASED_SWAP_BASE_INPUT_IX_ACCOUNTS_LEN]>
for OracleBasedSwapBaseInputAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; ORACLE_BASED_SWAP_BASE_INPUT_IX_ACCOUNTS_LEN],
    ) -> Self {
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
pub const ORACLE_BASED_SWAP_BASE_INPUT_IX_DISCM: [u8; 8usize] = [
    239, 82, 192, 187, 160, 26, 223, 223,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct OracleBasedSwapBaseInputIxArgs {
    pub amount_in: u64,
    pub minimum_amount_out: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct OracleBasedSwapBaseInputIxData(pub OracleBasedSwapBaseInputIxArgs);
impl From<OracleBasedSwapBaseInputIxArgs> for OracleBasedSwapBaseInputIxData {
    fn from(args: OracleBasedSwapBaseInputIxArgs) -> Self {
        Self(args)
    }
}
impl OracleBasedSwapBaseInputIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != ORACLE_BASED_SWAP_BASE_INPUT_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let amount_in: u64 = crate::borsh_de_or_default(&mut reader)?;
        let minimum_amount_out: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(OracleBasedSwapBaseInputIxArgs {
                amount_in,
                minimum_amount_out,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&ORACLE_BASED_SWAP_BASE_INPUT_IX_DISCM)?;
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
pub fn oracle_based_swap_base_input_ix_with_program_id(
    program_id: Pubkey,
    keys: OracleBasedSwapBaseInputKeys,
    args: OracleBasedSwapBaseInputIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; ORACLE_BASED_SWAP_BASE_INPUT_IX_ACCOUNTS_LEN] = keys.into();
    let data: OracleBasedSwapBaseInputIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn oracle_based_swap_base_input_ix(
    keys: OracleBasedSwapBaseInputKeys,
    args: OracleBasedSwapBaseInputIxArgs,
) -> std::io::Result<Instruction> {
    oracle_based_swap_base_input_ix_with_program_id(GAMMA_PROGRAM_ID, keys, args)
}
pub fn oracle_based_swap_base_input_invoke_with_program_id(
    program_id: Pubkey,
    accounts: OracleBasedSwapBaseInputAccounts<'_, '_>,
    args: OracleBasedSwapBaseInputIxArgs,
) -> ProgramResult {
    let keys: OracleBasedSwapBaseInputKeys = accounts.into();
    let ix = oracle_based_swap_base_input_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn oracle_based_swap_base_input_invoke(
    accounts: OracleBasedSwapBaseInputAccounts<'_, '_>,
    args: OracleBasedSwapBaseInputIxArgs,
) -> ProgramResult {
    oracle_based_swap_base_input_invoke_with_program_id(GAMMA_PROGRAM_ID, accounts, args)
}
pub fn oracle_based_swap_base_input_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: OracleBasedSwapBaseInputAccounts<'_, '_>,
    args: OracleBasedSwapBaseInputIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: OracleBasedSwapBaseInputKeys = accounts.into();
    let ix = oracle_based_swap_base_input_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn oracle_based_swap_base_input_invoke_signed(
    accounts: OracleBasedSwapBaseInputAccounts<'_, '_>,
    args: OracleBasedSwapBaseInputIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    oracle_based_swap_base_input_invoke_signed_with_program_id(
        GAMMA_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn oracle_based_swap_base_input_verify_account_keys(
    accounts: OracleBasedSwapBaseInputAccounts<'_, '_>,
    keys: OracleBasedSwapBaseInputKeys,
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
pub fn oracle_based_swap_base_input_verify_writable_privileges<'me, 'info>(
    accounts: OracleBasedSwapBaseInputAccounts<'me, 'info>,
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
pub fn oracle_based_swap_base_input_verify_signer_privileges<'me, 'info>(
    accounts: OracleBasedSwapBaseInputAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.payer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn oracle_based_swap_base_input_verify_account_privileges<'me, 'info>(
    accounts: OracleBasedSwapBaseInputAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    oracle_based_swap_base_input_verify_writable_privileges(accounts)?;
    oracle_based_swap_base_input_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const POOL_CONFIG_UPDATE_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct PoolConfigUpdateAccounts<'me, 'info> {
    pub authority: &'me AccountInfo<'info>,
    pub pool_state: &'me AccountInfo<'info>,
    pub amm_config: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct PoolConfigUpdateKeys {
    pub authority: Pubkey,
    pub pool_state: Pubkey,
    pub amm_config: Pubkey,
}
impl From<PoolConfigUpdateAccounts<'_, '_>> for PoolConfigUpdateKeys {
    fn from(accounts: PoolConfigUpdateAccounts) -> Self {
        Self {
            authority: *accounts.authority.key,
            pool_state: *accounts.pool_state.key,
            amm_config: *accounts.amm_config.key,
        }
    }
}
impl From<PoolConfigUpdateKeys> for [AccountMeta; POOL_CONFIG_UPDATE_IX_ACCOUNTS_LEN] {
    fn from(keys: PoolConfigUpdateKeys) -> Self {
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
            AccountMeta {
                pubkey: keys.amm_config,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; POOL_CONFIG_UPDATE_IX_ACCOUNTS_LEN]> for PoolConfigUpdateKeys {
    fn from(pubkeys: [Pubkey; POOL_CONFIG_UPDATE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            authority: pubkeys[0],
            pool_state: pubkeys[1],
            amm_config: pubkeys[2],
        }
    }
}
impl<'info> From<PoolConfigUpdateAccounts<'_, 'info>>
for [AccountInfo<'info>; POOL_CONFIG_UPDATE_IX_ACCOUNTS_LEN] {
    fn from(accounts: PoolConfigUpdateAccounts<'_, 'info>) -> Self {
        [
            accounts.authority.clone(),
            accounts.pool_state.clone(),
            accounts.amm_config.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; POOL_CONFIG_UPDATE_IX_ACCOUNTS_LEN]>
for PoolConfigUpdateAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; POOL_CONFIG_UPDATE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            authority: &arr[0],
            pool_state: &arr[1],
            amm_config: &arr[2],
        }
    }
}
pub const POOL_CONFIG_UPDATE_IX_DISCM: [u8; 8usize] = [
    196, 212, 99, 63, 59, 117, 248, 223,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PoolConfigUpdateIxArgs {
    pub h1: u128,
}
#[derive(Clone, Debug, PartialEq)]
pub struct PoolConfigUpdateIxData(pub PoolConfigUpdateIxArgs);
impl From<PoolConfigUpdateIxArgs> for PoolConfigUpdateIxData {
    fn from(args: PoolConfigUpdateIxArgs) -> Self {
        Self(args)
    }
}
impl PoolConfigUpdateIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != POOL_CONFIG_UPDATE_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let h1: u128 = crate::borsh_de_or_default(&mut reader)?;
        Ok(Self(PoolConfigUpdateIxArgs { h1 }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&POOL_CONFIG_UPDATE_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.h1, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn pool_config_update_ix_with_program_id(
    program_id: Pubkey,
    keys: PoolConfigUpdateKeys,
    args: PoolConfigUpdateIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; POOL_CONFIG_UPDATE_IX_ACCOUNTS_LEN] = keys.into();
    let data: PoolConfigUpdateIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn pool_config_update_ix(
    keys: PoolConfigUpdateKeys,
    args: PoolConfigUpdateIxArgs,
) -> std::io::Result<Instruction> {
    pool_config_update_ix_with_program_id(GAMMA_PROGRAM_ID, keys, args)
}
pub fn pool_config_update_invoke_with_program_id(
    program_id: Pubkey,
    accounts: PoolConfigUpdateAccounts<'_, '_>,
    args: PoolConfigUpdateIxArgs,
) -> ProgramResult {
    let keys: PoolConfigUpdateKeys = accounts.into();
    let ix = pool_config_update_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn pool_config_update_invoke(
    accounts: PoolConfigUpdateAccounts<'_, '_>,
    args: PoolConfigUpdateIxArgs,
) -> ProgramResult {
    pool_config_update_invoke_with_program_id(GAMMA_PROGRAM_ID, accounts, args)
}
pub fn pool_config_update_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: PoolConfigUpdateAccounts<'_, '_>,
    args: PoolConfigUpdateIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: PoolConfigUpdateKeys = accounts.into();
    let ix = pool_config_update_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn pool_config_update_invoke_signed(
    accounts: PoolConfigUpdateAccounts<'_, '_>,
    args: PoolConfigUpdateIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    pool_config_update_invoke_signed_with_program_id(
        GAMMA_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn pool_config_update_verify_account_keys(
    accounts: PoolConfigUpdateAccounts<'_, '_>,
    keys: PoolConfigUpdateKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.authority.key, keys.authority),
        (*accounts.pool_state.key, keys.pool_state),
        (*accounts.amm_config.key, keys.amm_config),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn pool_config_update_verify_writable_privileges<'me, 'info>(
    accounts: PoolConfigUpdateAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.pool_state] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn pool_config_update_verify_signer_privileges<'me, 'info>(
    accounts: PoolConfigUpdateAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn pool_config_update_verify_account_privileges<'me, 'info>(
    accounts: PoolConfigUpdateAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    pool_config_update_verify_writable_privileges(accounts)?;
    pool_config_update_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const POOL_CONFIG_UPDATE_V2_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct PoolConfigUpdateV2Accounts<'me, 'info> {
    pub authority: &'me AccountInfo<'info>,
    pub pool_state: &'me AccountInfo<'info>,
    pub amm_config: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct PoolConfigUpdateV2Keys {
    pub authority: Pubkey,
    pub pool_state: Pubkey,
    pub amm_config: Pubkey,
}
impl From<PoolConfigUpdateV2Accounts<'_, '_>> for PoolConfigUpdateV2Keys {
    fn from(accounts: PoolConfigUpdateV2Accounts) -> Self {
        Self {
            authority: *accounts.authority.key,
            pool_state: *accounts.pool_state.key,
            amm_config: *accounts.amm_config.key,
        }
    }
}
impl From<PoolConfigUpdateV2Keys>
for [AccountMeta; POOL_CONFIG_UPDATE_V2_IX_ACCOUNTS_LEN] {
    fn from(keys: PoolConfigUpdateV2Keys) -> Self {
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
            AccountMeta {
                pubkey: keys.amm_config,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; POOL_CONFIG_UPDATE_V2_IX_ACCOUNTS_LEN]> for PoolConfigUpdateV2Keys {
    fn from(pubkeys: [Pubkey; POOL_CONFIG_UPDATE_V2_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            authority: pubkeys[0],
            pool_state: pubkeys[1],
            amm_config: pubkeys[2],
        }
    }
}
impl<'info> From<PoolConfigUpdateV2Accounts<'_, 'info>>
for [AccountInfo<'info>; POOL_CONFIG_UPDATE_V2_IX_ACCOUNTS_LEN] {
    fn from(accounts: PoolConfigUpdateV2Accounts<'_, 'info>) -> Self {
        [
            accounts.authority.clone(),
            accounts.pool_state.clone(),
            accounts.amm_config.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; POOL_CONFIG_UPDATE_V2_IX_ACCOUNTS_LEN]>
for PoolConfigUpdateV2Accounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; POOL_CONFIG_UPDATE_V2_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            authority: &arr[0],
            pool_state: &arr[1],
            amm_config: &arr[2],
        }
    }
}
pub const POOL_CONFIG_UPDATE_V2_IX_DISCM: [u8; 8usize] = [
    228, 161, 199, 222, 53, 172, 121, 238,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PoolConfigUpdateV2IxArgs {
    pub h1: u128,
    pub price_fetched_at: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct PoolConfigUpdateV2IxData(pub PoolConfigUpdateV2IxArgs);
impl From<PoolConfigUpdateV2IxArgs> for PoolConfigUpdateV2IxData {
    fn from(args: PoolConfigUpdateV2IxArgs) -> Self {
        Self(args)
    }
}
impl PoolConfigUpdateV2IxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != POOL_CONFIG_UPDATE_V2_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let h1: u128 = crate::borsh_de_or_default(&mut reader)?;
        let price_fetched_at: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(PoolConfigUpdateV2IxArgs {
                h1,
                price_fetched_at,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&POOL_CONFIG_UPDATE_V2_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.h1, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.price_fetched_at, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn pool_config_update_v2_ix_with_program_id(
    program_id: Pubkey,
    keys: PoolConfigUpdateV2Keys,
    args: PoolConfigUpdateV2IxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; POOL_CONFIG_UPDATE_V2_IX_ACCOUNTS_LEN] = keys.into();
    let data: PoolConfigUpdateV2IxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn pool_config_update_v2_ix(
    keys: PoolConfigUpdateV2Keys,
    args: PoolConfigUpdateV2IxArgs,
) -> std::io::Result<Instruction> {
    pool_config_update_v2_ix_with_program_id(GAMMA_PROGRAM_ID, keys, args)
}
pub fn pool_config_update_v2_invoke_with_program_id(
    program_id: Pubkey,
    accounts: PoolConfigUpdateV2Accounts<'_, '_>,
    args: PoolConfigUpdateV2IxArgs,
) -> ProgramResult {
    let keys: PoolConfigUpdateV2Keys = accounts.into();
    let ix = pool_config_update_v2_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn pool_config_update_v2_invoke(
    accounts: PoolConfigUpdateV2Accounts<'_, '_>,
    args: PoolConfigUpdateV2IxArgs,
) -> ProgramResult {
    pool_config_update_v2_invoke_with_program_id(GAMMA_PROGRAM_ID, accounts, args)
}
pub fn pool_config_update_v2_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: PoolConfigUpdateV2Accounts<'_, '_>,
    args: PoolConfigUpdateV2IxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: PoolConfigUpdateV2Keys = accounts.into();
    let ix = pool_config_update_v2_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn pool_config_update_v2_invoke_signed(
    accounts: PoolConfigUpdateV2Accounts<'_, '_>,
    args: PoolConfigUpdateV2IxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    pool_config_update_v2_invoke_signed_with_program_id(
        GAMMA_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn pool_config_update_v2_verify_account_keys(
    accounts: PoolConfigUpdateV2Accounts<'_, '_>,
    keys: PoolConfigUpdateV2Keys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.authority.key, keys.authority),
        (*accounts.pool_state.key, keys.pool_state),
        (*accounts.amm_config.key, keys.amm_config),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn pool_config_update_v2_verify_writable_privileges<'me, 'info>(
    accounts: PoolConfigUpdateV2Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.pool_state] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn pool_config_update_v2_verify_signer_privileges<'me, 'info>(
    accounts: PoolConfigUpdateV2Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn pool_config_update_v2_verify_account_privileges<'me, 'info>(
    accounts: PoolConfigUpdateV2Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    pool_config_update_v2_verify_writable_privileges(accounts)?;
    pool_config_update_v2_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const POOL_CONFIG_UPDATE_V3_IX_ACCOUNTS_LEN: usize = 5;
#[derive(Copy, Clone, Debug)]
pub struct PoolConfigUpdateV3Accounts<'me, 'info> {
    pub authority: &'me AccountInfo<'info>,
    pub pool_state: &'me AccountInfo<'info>,
    pub amm_config: &'me AccountInfo<'info>,
    pub observation_state: &'me AccountInfo<'info>,
    pub actor: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct PoolConfigUpdateV3Keys {
    pub authority: Pubkey,
    pub pool_state: Pubkey,
    pub amm_config: Pubkey,
    pub observation_state: Pubkey,
    pub actor: Pubkey,
}
impl From<PoolConfigUpdateV3Accounts<'_, '_>> for PoolConfigUpdateV3Keys {
    fn from(accounts: PoolConfigUpdateV3Accounts) -> Self {
        Self {
            authority: *accounts.authority.key,
            pool_state: *accounts.pool_state.key,
            amm_config: *accounts.amm_config.key,
            observation_state: *accounts.observation_state.key,
            actor: *accounts.actor.key,
        }
    }
}
impl From<PoolConfigUpdateV3Keys>
for [AccountMeta; POOL_CONFIG_UPDATE_V3_IX_ACCOUNTS_LEN] {
    fn from(keys: PoolConfigUpdateV3Keys) -> Self {
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
            AccountMeta {
                pubkey: keys.amm_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.observation_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.actor,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; POOL_CONFIG_UPDATE_V3_IX_ACCOUNTS_LEN]> for PoolConfigUpdateV3Keys {
    fn from(pubkeys: [Pubkey; POOL_CONFIG_UPDATE_V3_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            authority: pubkeys[0],
            pool_state: pubkeys[1],
            amm_config: pubkeys[2],
            observation_state: pubkeys[3],
            actor: pubkeys[4],
        }
    }
}
impl<'info> From<PoolConfigUpdateV3Accounts<'_, 'info>>
for [AccountInfo<'info>; POOL_CONFIG_UPDATE_V3_IX_ACCOUNTS_LEN] {
    fn from(accounts: PoolConfigUpdateV3Accounts<'_, 'info>) -> Self {
        [
            accounts.authority.clone(),
            accounts.pool_state.clone(),
            accounts.amm_config.clone(),
            accounts.observation_state.clone(),
            accounts.actor.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; POOL_CONFIG_UPDATE_V3_IX_ACCOUNTS_LEN]>
for PoolConfigUpdateV3Accounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; POOL_CONFIG_UPDATE_V3_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            authority: &arr[0],
            pool_state: &arr[1],
            amm_config: &arr[2],
            observation_state: &arr[3],
            actor: &arr[4],
        }
    }
}
pub const POOL_CONFIG_UPDATE_V3_IX_DISCM: [u8; 8usize] = [
    123, 135, 23, 15, 174, 100, 120, 152,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PoolConfigUpdateV3IxArgs {
    pub h1: u128,
    pub h2: u128,
    pub price_fetched_at: u64,
    pub min_trade_rate_at_oracle_price: Option<u32>,
    pub oracle_price_delay_fee_rate_per_second: Option<u32>,
    pub max_oracle_price_delay_fee: Option<u32>,
    pub drift_factor: Option<u32>,
    pub max_drift_factor: Option<u32>,
    pub volatility_factor: Option<u64>,
}
#[derive(Clone, Debug, PartialEq)]
pub struct PoolConfigUpdateV3IxData(pub PoolConfigUpdateV3IxArgs);
impl From<PoolConfigUpdateV3IxArgs> for PoolConfigUpdateV3IxData {
    fn from(args: PoolConfigUpdateV3IxArgs) -> Self {
        Self(args)
    }
}
impl PoolConfigUpdateV3IxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != POOL_CONFIG_UPDATE_V3_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let h1: u128 = crate::borsh_de_or_default(&mut reader)?;
        let h2: u128 = crate::borsh_de_or_default(&mut reader)?;
        let price_fetched_at: u64 = crate::borsh_de_or_default(&mut reader)?;
        let min_trade_rate_at_oracle_price: Option<u32> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let oracle_price_delay_fee_rate_per_second: Option<u32> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let max_oracle_price_delay_fee: Option<u32> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let drift_factor: Option<u32> = crate::borsh_de_or_default(&mut reader)?;
        let max_drift_factor: Option<u32> = crate::borsh_de_or_default(&mut reader)?;
        let volatility_factor: Option<u64> = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(PoolConfigUpdateV3IxArgs {
                h1,
                h2,
                price_fetched_at,
                min_trade_rate_at_oracle_price,
                oracle_price_delay_fee_rate_per_second,
                max_oracle_price_delay_fee,
                drift_factor,
                max_drift_factor,
                volatility_factor,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&POOL_CONFIG_UPDATE_V3_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.h1, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.h2, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.price_fetched_at, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.0.min_trade_rate_at_oracle_price,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.0.oracle_price_delay_fee_rate_per_second,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.0.max_oracle_price_delay_fee,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.0.drift_factor, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.max_drift_factor, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.volatility_factor, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn pool_config_update_v3_ix_with_program_id(
    program_id: Pubkey,
    keys: PoolConfigUpdateV3Keys,
    args: PoolConfigUpdateV3IxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; POOL_CONFIG_UPDATE_V3_IX_ACCOUNTS_LEN] = keys.into();
    let data: PoolConfigUpdateV3IxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn pool_config_update_v3_ix(
    keys: PoolConfigUpdateV3Keys,
    args: PoolConfigUpdateV3IxArgs,
) -> std::io::Result<Instruction> {
    pool_config_update_v3_ix_with_program_id(GAMMA_PROGRAM_ID, keys, args)
}
pub fn pool_config_update_v3_invoke_with_program_id(
    program_id: Pubkey,
    accounts: PoolConfigUpdateV3Accounts<'_, '_>,
    args: PoolConfigUpdateV3IxArgs,
) -> ProgramResult {
    let keys: PoolConfigUpdateV3Keys = accounts.into();
    let ix = pool_config_update_v3_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn pool_config_update_v3_invoke(
    accounts: PoolConfigUpdateV3Accounts<'_, '_>,
    args: PoolConfigUpdateV3IxArgs,
) -> ProgramResult {
    pool_config_update_v3_invoke_with_program_id(GAMMA_PROGRAM_ID, accounts, args)
}
pub fn pool_config_update_v3_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: PoolConfigUpdateV3Accounts<'_, '_>,
    args: PoolConfigUpdateV3IxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: PoolConfigUpdateV3Keys = accounts.into();
    let ix = pool_config_update_v3_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn pool_config_update_v3_invoke_signed(
    accounts: PoolConfigUpdateV3Accounts<'_, '_>,
    args: PoolConfigUpdateV3IxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    pool_config_update_v3_invoke_signed_with_program_id(
        GAMMA_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn pool_config_update_v3_verify_account_keys(
    accounts: PoolConfigUpdateV3Accounts<'_, '_>,
    keys: PoolConfigUpdateV3Keys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.authority.key, keys.authority),
        (*accounts.pool_state.key, keys.pool_state),
        (*accounts.amm_config.key, keys.amm_config),
        (*accounts.observation_state.key, keys.observation_state),
        (*accounts.actor.key, keys.actor),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn pool_config_update_v3_verify_writable_privileges<'me, 'info>(
    accounts: PoolConfigUpdateV3Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.pool_state, accounts.observation_state] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn pool_config_update_v3_verify_signer_privileges<'me, 'info>(
    accounts: PoolConfigUpdateV3Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn pool_config_update_v3_verify_account_privileges<'me, 'info>(
    accounts: PoolConfigUpdateV3Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    pool_config_update_v3_verify_writable_privileges(accounts)?;
    pool_config_update_v3_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const REBALANCE_KAMINO_IX_ACCOUNTS_LEN: usize = 18;
#[derive(Copy, Clone, Debug)]
pub struct RebalanceKaminoAccounts<'me, 'info> {
    pub signer: &'me AccountInfo<'info>,
    pub gamma_authority: &'me AccountInfo<'info>,
    pub pool_state: &'me AccountInfo<'info>,
    pub token_vault: &'me AccountInfo<'info>,
    pub token_mint: &'me AccountInfo<'info>,
    pub kamino_reserve: &'me AccountInfo<'info>,
    pub kamino_lending_market: &'me AccountInfo<'info>,
    pub lending_market_authority: &'me AccountInfo<'info>,
    pub reserve_liquidity_supply: &'me AccountInfo<'info>,
    pub reserve_collateral_mint: &'me AccountInfo<'info>,
    pub gamma_pool_destination_collateral: &'me AccountInfo<'info>,
    pub instruction_sysvar_account: &'me AccountInfo<'info>,
    pub liquidity_token_program: &'me AccountInfo<'info>,
    pub collateral_token_program: &'me AccountInfo<'info>,
    pub kamino_program: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub token_program_2022: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct RebalanceKaminoKeys {
    pub signer: Pubkey,
    pub gamma_authority: Pubkey,
    pub pool_state: Pubkey,
    pub token_vault: Pubkey,
    pub token_mint: Pubkey,
    pub kamino_reserve: Pubkey,
    pub kamino_lending_market: Pubkey,
    pub lending_market_authority: Pubkey,
    pub reserve_liquidity_supply: Pubkey,
    pub reserve_collateral_mint: Pubkey,
    pub gamma_pool_destination_collateral: Pubkey,
    pub instruction_sysvar_account: Pubkey,
    pub liquidity_token_program: Pubkey,
    pub collateral_token_program: Pubkey,
    pub kamino_program: Pubkey,
    pub token_program: Pubkey,
    pub token_program_2022: Pubkey,
    pub system_program: Pubkey,
}
impl From<RebalanceKaminoAccounts<'_, '_>> for RebalanceKaminoKeys {
    fn from(accounts: RebalanceKaminoAccounts) -> Self {
        Self {
            signer: *accounts.signer.key,
            gamma_authority: *accounts.gamma_authority.key,
            pool_state: *accounts.pool_state.key,
            token_vault: *accounts.token_vault.key,
            token_mint: *accounts.token_mint.key,
            kamino_reserve: *accounts.kamino_reserve.key,
            kamino_lending_market: *accounts.kamino_lending_market.key,
            lending_market_authority: *accounts.lending_market_authority.key,
            reserve_liquidity_supply: *accounts.reserve_liquidity_supply.key,
            reserve_collateral_mint: *accounts.reserve_collateral_mint.key,
            gamma_pool_destination_collateral: *accounts
                .gamma_pool_destination_collateral
                .key,
            instruction_sysvar_account: *accounts.instruction_sysvar_account.key,
            liquidity_token_program: *accounts.liquidity_token_program.key,
            collateral_token_program: *accounts.collateral_token_program.key,
            kamino_program: *accounts.kamino_program.key,
            token_program: *accounts.token_program.key,
            token_program_2022: *accounts.token_program_2022.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<RebalanceKaminoKeys> for [AccountMeta; REBALANCE_KAMINO_IX_ACCOUNTS_LEN] {
    fn from(keys: RebalanceKaminoKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.signer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.gamma_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_state,
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
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.kamino_reserve,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.kamino_lending_market,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.lending_market_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.reserve_liquidity_supply,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.reserve_collateral_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.gamma_pool_destination_collateral,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.instruction_sysvar_account,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.liquidity_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.collateral_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.kamino_program,
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
                pubkey: keys.system_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; REBALANCE_KAMINO_IX_ACCOUNTS_LEN]> for RebalanceKaminoKeys {
    fn from(pubkeys: [Pubkey; REBALANCE_KAMINO_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            signer: pubkeys[0],
            gamma_authority: pubkeys[1],
            pool_state: pubkeys[2],
            token_vault: pubkeys[3],
            token_mint: pubkeys[4],
            kamino_reserve: pubkeys[5],
            kamino_lending_market: pubkeys[6],
            lending_market_authority: pubkeys[7],
            reserve_liquidity_supply: pubkeys[8],
            reserve_collateral_mint: pubkeys[9],
            gamma_pool_destination_collateral: pubkeys[10],
            instruction_sysvar_account: pubkeys[11],
            liquidity_token_program: pubkeys[12],
            collateral_token_program: pubkeys[13],
            kamino_program: pubkeys[14],
            token_program: pubkeys[15],
            token_program_2022: pubkeys[16],
            system_program: pubkeys[17],
        }
    }
}
impl<'info> From<RebalanceKaminoAccounts<'_, 'info>>
for [AccountInfo<'info>; REBALANCE_KAMINO_IX_ACCOUNTS_LEN] {
    fn from(accounts: RebalanceKaminoAccounts<'_, 'info>) -> Self {
        [
            accounts.signer.clone(),
            accounts.gamma_authority.clone(),
            accounts.pool_state.clone(),
            accounts.token_vault.clone(),
            accounts.token_mint.clone(),
            accounts.kamino_reserve.clone(),
            accounts.kamino_lending_market.clone(),
            accounts.lending_market_authority.clone(),
            accounts.reserve_liquidity_supply.clone(),
            accounts.reserve_collateral_mint.clone(),
            accounts.gamma_pool_destination_collateral.clone(),
            accounts.instruction_sysvar_account.clone(),
            accounts.liquidity_token_program.clone(),
            accounts.collateral_token_program.clone(),
            accounts.kamino_program.clone(),
            accounts.token_program.clone(),
            accounts.token_program_2022.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; REBALANCE_KAMINO_IX_ACCOUNTS_LEN]>
for RebalanceKaminoAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; REBALANCE_KAMINO_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            signer: &arr[0],
            gamma_authority: &arr[1],
            pool_state: &arr[2],
            token_vault: &arr[3],
            token_mint: &arr[4],
            kamino_reserve: &arr[5],
            kamino_lending_market: &arr[6],
            lending_market_authority: &arr[7],
            reserve_liquidity_supply: &arr[8],
            reserve_collateral_mint: &arr[9],
            gamma_pool_destination_collateral: &arr[10],
            instruction_sysvar_account: &arr[11],
            liquidity_token_program: &arr[12],
            collateral_token_program: &arr[13],
            kamino_program: &arr[14],
            token_program: &arr[15],
            token_program_2022: &arr[16],
            system_program: &arr[17],
        }
    }
}
pub const REBALANCE_KAMINO_IX_DISCM: [u8; 8usize] = [153, 94, 34, 16, 92, 181, 147, 215];
#[derive(Clone, Debug, PartialEq)]
pub struct RebalanceKaminoIxData;
impl RebalanceKaminoIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != REBALANCE_KAMINO_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&REBALANCE_KAMINO_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn rebalance_kamino_ix_with_program_id(
    program_id: Pubkey,
    keys: RebalanceKaminoKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; REBALANCE_KAMINO_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: RebalanceKaminoIxData.try_to_vec()?,
    })
}
pub fn rebalance_kamino_ix(keys: RebalanceKaminoKeys) -> std::io::Result<Instruction> {
    rebalance_kamino_ix_with_program_id(GAMMA_PROGRAM_ID, keys)
}
pub fn rebalance_kamino_invoke_with_program_id(
    program_id: Pubkey,
    accounts: RebalanceKaminoAccounts<'_, '_>,
) -> ProgramResult {
    let keys: RebalanceKaminoKeys = accounts.into();
    let ix = rebalance_kamino_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn rebalance_kamino_invoke(
    accounts: RebalanceKaminoAccounts<'_, '_>,
) -> ProgramResult {
    rebalance_kamino_invoke_with_program_id(GAMMA_PROGRAM_ID, accounts)
}
pub fn rebalance_kamino_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: RebalanceKaminoAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: RebalanceKaminoKeys = accounts.into();
    let ix = rebalance_kamino_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn rebalance_kamino_invoke_signed(
    accounts: RebalanceKaminoAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    rebalance_kamino_invoke_signed_with_program_id(GAMMA_PROGRAM_ID, accounts, seeds)
}
pub fn rebalance_kamino_verify_account_keys(
    accounts: RebalanceKaminoAccounts<'_, '_>,
    keys: RebalanceKaminoKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.signer.key, keys.signer),
        (*accounts.gamma_authority.key, keys.gamma_authority),
        (*accounts.pool_state.key, keys.pool_state),
        (*accounts.token_vault.key, keys.token_vault),
        (*accounts.token_mint.key, keys.token_mint),
        (*accounts.kamino_reserve.key, keys.kamino_reserve),
        (*accounts.kamino_lending_market.key, keys.kamino_lending_market),
        (*accounts.lending_market_authority.key, keys.lending_market_authority),
        (*accounts.reserve_liquidity_supply.key, keys.reserve_liquidity_supply),
        (*accounts.reserve_collateral_mint.key, keys.reserve_collateral_mint),
        (
            *accounts.gamma_pool_destination_collateral.key,
            keys.gamma_pool_destination_collateral,
        ),
        (*accounts.instruction_sysvar_account.key, keys.instruction_sysvar_account),
        (*accounts.liquidity_token_program.key, keys.liquidity_token_program),
        (*accounts.collateral_token_program.key, keys.collateral_token_program),
        (*accounts.kamino_program.key, keys.kamino_program),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.token_program_2022.key, keys.token_program_2022),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn rebalance_kamino_verify_writable_privileges<'me, 'info>(
    accounts: RebalanceKaminoAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.signer,
        accounts.pool_state,
        accounts.token_vault,
        accounts.token_mint,
        accounts.kamino_reserve,
        accounts.kamino_lending_market,
        accounts.reserve_liquidity_supply,
        accounts.reserve_collateral_mint,
        accounts.gamma_pool_destination_collateral,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn rebalance_kamino_verify_signer_privileges<'me, 'info>(
    accounts: RebalanceKaminoAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.signer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn rebalance_kamino_verify_account_privileges<'me, 'info>(
    accounts: RebalanceKaminoAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    rebalance_kamino_verify_writable_privileges(accounts)?;
    rebalance_kamino_verify_signer_privileges(accounts)?;
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
    swap_base_input_ix_with_program_id(GAMMA_PROGRAM_ID, keys, args)
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
    swap_base_input_invoke_with_program_id(GAMMA_PROGRAM_ID, accounts, args)
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
        GAMMA_PROGRAM_ID,
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
    swap_base_output_ix_with_program_id(GAMMA_PROGRAM_ID, keys, args)
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
    swap_base_output_invoke_with_program_id(GAMMA_PROGRAM_ID, accounts, args)
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
        GAMMA_PROGRAM_ID,
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
    pub param: u16,
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
        let param: u16 = crate::borsh_de_or_default(&mut reader)?;
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
    update_amm_config_ix_with_program_id(GAMMA_PROGRAM_ID, keys, args)
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
    update_amm_config_invoke_with_program_id(GAMMA_PROGRAM_ID, accounts, args)
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
        GAMMA_PROGRAM_ID,
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
pub const UPDATE_PARTNER_IX_ACCOUNTS_LEN: usize = 2;
#[derive(Copy, Clone, Debug)]
pub struct UpdatePartnerAccounts<'me, 'info> {
    pub authority: &'me AccountInfo<'info>,
    pub partner: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UpdatePartnerKeys {
    pub authority: Pubkey,
    pub partner: Pubkey,
}
impl From<UpdatePartnerAccounts<'_, '_>> for UpdatePartnerKeys {
    fn from(accounts: UpdatePartnerAccounts) -> Self {
        Self {
            authority: *accounts.authority.key,
            partner: *accounts.partner.key,
        }
    }
}
impl From<UpdatePartnerKeys> for [AccountMeta; UPDATE_PARTNER_IX_ACCOUNTS_LEN] {
    fn from(keys: UpdatePartnerKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.authority,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.partner,
                is_signer: false,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; UPDATE_PARTNER_IX_ACCOUNTS_LEN]> for UpdatePartnerKeys {
    fn from(pubkeys: [Pubkey; UPDATE_PARTNER_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            authority: pubkeys[0],
            partner: pubkeys[1],
        }
    }
}
impl<'info> From<UpdatePartnerAccounts<'_, 'info>>
for [AccountInfo<'info>; UPDATE_PARTNER_IX_ACCOUNTS_LEN] {
    fn from(accounts: UpdatePartnerAccounts<'_, 'info>) -> Self {
        [accounts.authority.clone(), accounts.partner.clone()]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; UPDATE_PARTNER_IX_ACCOUNTS_LEN]>
for UpdatePartnerAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; UPDATE_PARTNER_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            authority: &arr[0],
            partner: &arr[1],
        }
    }
}
pub const UPDATE_PARTNER_IX_DISCM: [u8; 8usize] = [19, 112, 236, 81, 127, 55, 21, 196];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UpdatePartnerIxArgs {
    pub token_account_0: Option<Pubkey>,
    pub token_account_1: Option<Pubkey>,
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdatePartnerIxData(pub UpdatePartnerIxArgs);
impl From<UpdatePartnerIxArgs> for UpdatePartnerIxData {
    fn from(args: UpdatePartnerIxArgs) -> Self {
        Self(args)
    }
}
impl UpdatePartnerIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_PARTNER_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let token_account_0: Option<Pubkey> = crate::borsh_de_or_default(&mut reader)?;
        let token_account_1: Option<Pubkey> = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(UpdatePartnerIxArgs {
                token_account_0,
                token_account_1,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_PARTNER_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.token_account_0, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.token_account_1, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn update_partner_ix_with_program_id(
    program_id: Pubkey,
    keys: UpdatePartnerKeys,
    args: UpdatePartnerIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; UPDATE_PARTNER_IX_ACCOUNTS_LEN] = keys.into();
    let data: UpdatePartnerIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_partner_ix(
    keys: UpdatePartnerKeys,
    args: UpdatePartnerIxArgs,
) -> std::io::Result<Instruction> {
    update_partner_ix_with_program_id(GAMMA_PROGRAM_ID, keys, args)
}
pub fn update_partner_invoke_with_program_id(
    program_id: Pubkey,
    accounts: UpdatePartnerAccounts<'_, '_>,
    args: UpdatePartnerIxArgs,
) -> ProgramResult {
    let keys: UpdatePartnerKeys = accounts.into();
    let ix = update_partner_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn update_partner_invoke(
    accounts: UpdatePartnerAccounts<'_, '_>,
    args: UpdatePartnerIxArgs,
) -> ProgramResult {
    update_partner_invoke_with_program_id(GAMMA_PROGRAM_ID, accounts, args)
}
pub fn update_partner_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: UpdatePartnerAccounts<'_, '_>,
    args: UpdatePartnerIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: UpdatePartnerKeys = accounts.into();
    let ix = update_partner_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn update_partner_invoke_signed(
    accounts: UpdatePartnerAccounts<'_, '_>,
    args: UpdatePartnerIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    update_partner_invoke_signed_with_program_id(GAMMA_PROGRAM_ID, accounts, args, seeds)
}
pub fn update_partner_verify_account_keys(
    accounts: UpdatePartnerAccounts<'_, '_>,
    keys: UpdatePartnerKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.authority.key, keys.authority),
        (*accounts.partner.key, keys.partner),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn update_partner_verify_writable_privileges<'me, 'info>(
    accounts: UpdatePartnerAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.partner] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn update_partner_verify_signer_privileges<'me, 'info>(
    accounts: UpdatePartnerAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn update_partner_verify_account_privileges<'me, 'info>(
    accounts: UpdatePartnerAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    update_partner_verify_writable_privileges(accounts)?;
    update_partner_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const UPDATE_PARTNER_FEES_IX_ACCOUNTS_LEN: usize = 2;
#[derive(Copy, Clone, Debug)]
pub struct UpdatePartnerFeesAccounts<'me, 'info> {
    pub pool_state: &'me AccountInfo<'info>,
    pub pool_partners: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UpdatePartnerFeesKeys {
    pub pool_state: Pubkey,
    pub pool_partners: Pubkey,
}
impl From<UpdatePartnerFeesAccounts<'_, '_>> for UpdatePartnerFeesKeys {
    fn from(accounts: UpdatePartnerFeesAccounts) -> Self {
        Self {
            pool_state: *accounts.pool_state.key,
            pool_partners: *accounts.pool_partners.key,
        }
    }
}
impl From<UpdatePartnerFeesKeys> for [AccountMeta; UPDATE_PARTNER_FEES_IX_ACCOUNTS_LEN] {
    fn from(keys: UpdatePartnerFeesKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.pool_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pool_partners,
                is_signer: false,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; UPDATE_PARTNER_FEES_IX_ACCOUNTS_LEN]> for UpdatePartnerFeesKeys {
    fn from(pubkeys: [Pubkey; UPDATE_PARTNER_FEES_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            pool_state: pubkeys[0],
            pool_partners: pubkeys[1],
        }
    }
}
impl<'info> From<UpdatePartnerFeesAccounts<'_, 'info>>
for [AccountInfo<'info>; UPDATE_PARTNER_FEES_IX_ACCOUNTS_LEN] {
    fn from(accounts: UpdatePartnerFeesAccounts<'_, 'info>) -> Self {
        [accounts.pool_state.clone(), accounts.pool_partners.clone()]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; UPDATE_PARTNER_FEES_IX_ACCOUNTS_LEN]>
for UpdatePartnerFeesAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; UPDATE_PARTNER_FEES_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            pool_state: &arr[0],
            pool_partners: &arr[1],
        }
    }
}
pub const UPDATE_PARTNER_FEES_IX_DISCM: [u8; 8usize] = [
    154, 142, 122, 49, 239, 161, 232, 29,
];
#[derive(Clone, Debug, PartialEq)]
pub struct UpdatePartnerFeesIxData;
impl UpdatePartnerFeesIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_PARTNER_FEES_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_PARTNER_FEES_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn update_partner_fees_ix_with_program_id(
    program_id: Pubkey,
    keys: UpdatePartnerFeesKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; UPDATE_PARTNER_FEES_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: UpdatePartnerFeesIxData.try_to_vec()?,
    })
}
pub fn update_partner_fees_ix(
    keys: UpdatePartnerFeesKeys,
) -> std::io::Result<Instruction> {
    update_partner_fees_ix_with_program_id(GAMMA_PROGRAM_ID, keys)
}
pub fn update_partner_fees_invoke_with_program_id(
    program_id: Pubkey,
    accounts: UpdatePartnerFeesAccounts<'_, '_>,
) -> ProgramResult {
    let keys: UpdatePartnerFeesKeys = accounts.into();
    let ix = update_partner_fees_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn update_partner_fees_invoke(
    accounts: UpdatePartnerFeesAccounts<'_, '_>,
) -> ProgramResult {
    update_partner_fees_invoke_with_program_id(GAMMA_PROGRAM_ID, accounts)
}
pub fn update_partner_fees_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: UpdatePartnerFeesAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: UpdatePartnerFeesKeys = accounts.into();
    let ix = update_partner_fees_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn update_partner_fees_invoke_signed(
    accounts: UpdatePartnerFeesAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    update_partner_fees_invoke_signed_with_program_id(GAMMA_PROGRAM_ID, accounts, seeds)
}
pub fn update_partner_fees_verify_account_keys(
    accounts: UpdatePartnerFeesAccounts<'_, '_>,
    keys: UpdatePartnerFeesKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.pool_state.key, keys.pool_state),
        (*accounts.pool_partners.key, keys.pool_partners),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn update_partner_fees_verify_writable_privileges<'me, 'info>(
    accounts: UpdatePartnerFeesAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.pool_state, accounts.pool_partners] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn update_partner_fees_verify_account_privileges<'me, 'info>(
    accounts: UpdatePartnerFeesAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    update_partner_fees_verify_writable_privileges(accounts)?;
    Ok(())
}
pub const UPDATE_POOL_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct UpdatePoolAccounts<'me, 'info> {
    pub authority: &'me AccountInfo<'info>,
    pub pool_state: &'me AccountInfo<'info>,
    pub actor: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UpdatePoolKeys {
    pub authority: Pubkey,
    pub pool_state: Pubkey,
    pub actor: Pubkey,
}
impl From<UpdatePoolAccounts<'_, '_>> for UpdatePoolKeys {
    fn from(accounts: UpdatePoolAccounts) -> Self {
        Self {
            authority: *accounts.authority.key,
            pool_state: *accounts.pool_state.key,
            actor: *accounts.actor.key,
        }
    }
}
impl From<UpdatePoolKeys> for [AccountMeta; UPDATE_POOL_IX_ACCOUNTS_LEN] {
    fn from(keys: UpdatePoolKeys) -> Self {
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
            AccountMeta {
                pubkey: keys.actor,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; UPDATE_POOL_IX_ACCOUNTS_LEN]> for UpdatePoolKeys {
    fn from(pubkeys: [Pubkey; UPDATE_POOL_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            authority: pubkeys[0],
            pool_state: pubkeys[1],
            actor: pubkeys[2],
        }
    }
}
impl<'info> From<UpdatePoolAccounts<'_, 'info>>
for [AccountInfo<'info>; UPDATE_POOL_IX_ACCOUNTS_LEN] {
    fn from(accounts: UpdatePoolAccounts<'_, 'info>) -> Self {
        [accounts.authority.clone(), accounts.pool_state.clone(), accounts.actor.clone()]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; UPDATE_POOL_IX_ACCOUNTS_LEN]>
for UpdatePoolAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; UPDATE_POOL_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            authority: &arr[0],
            pool_state: &arr[1],
            actor: &arr[2],
        }
    }
}
pub const UPDATE_POOL_IX_DISCM: [u8; 8usize] = [239, 214, 170, 78, 36, 35, 30, 34];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UpdatePoolIxArgs {
    pub param: u32,
    pub value: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdatePoolIxData(pub UpdatePoolIxArgs);
impl From<UpdatePoolIxArgs> for UpdatePoolIxData {
    fn from(args: UpdatePoolIxArgs) -> Self {
        Self(args)
    }
}
impl UpdatePoolIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_POOL_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let param: u32 = crate::borsh_de_or_default(&mut reader)?;
        let value: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(Self(UpdatePoolIxArgs { param, value }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_POOL_IX_DISCM)?;
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
pub fn update_pool_ix_with_program_id(
    program_id: Pubkey,
    keys: UpdatePoolKeys,
    args: UpdatePoolIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; UPDATE_POOL_IX_ACCOUNTS_LEN] = keys.into();
    let data: UpdatePoolIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_pool_ix(
    keys: UpdatePoolKeys,
    args: UpdatePoolIxArgs,
) -> std::io::Result<Instruction> {
    update_pool_ix_with_program_id(GAMMA_PROGRAM_ID, keys, args)
}
pub fn update_pool_invoke_with_program_id(
    program_id: Pubkey,
    accounts: UpdatePoolAccounts<'_, '_>,
    args: UpdatePoolIxArgs,
) -> ProgramResult {
    let keys: UpdatePoolKeys = accounts.into();
    let ix = update_pool_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn update_pool_invoke(
    accounts: UpdatePoolAccounts<'_, '_>,
    args: UpdatePoolIxArgs,
) -> ProgramResult {
    update_pool_invoke_with_program_id(GAMMA_PROGRAM_ID, accounts, args)
}
pub fn update_pool_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: UpdatePoolAccounts<'_, '_>,
    args: UpdatePoolIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: UpdatePoolKeys = accounts.into();
    let ix = update_pool_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn update_pool_invoke_signed(
    accounts: UpdatePoolAccounts<'_, '_>,
    args: UpdatePoolIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    update_pool_invoke_signed_with_program_id(GAMMA_PROGRAM_ID, accounts, args, seeds)
}
pub fn update_pool_verify_account_keys(
    accounts: UpdatePoolAccounts<'_, '_>,
    keys: UpdatePoolKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.authority.key, keys.authority),
        (*accounts.pool_state.key, keys.pool_state),
        (*accounts.actor.key, keys.actor),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn update_pool_verify_writable_privileges<'me, 'info>(
    accounts: UpdatePoolAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.pool_state] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn update_pool_verify_signer_privileges<'me, 'info>(
    accounts: UpdatePoolAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn update_pool_verify_account_privileges<'me, 'info>(
    accounts: UpdatePoolAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    update_pool_verify_writable_privileges(accounts)?;
    update_pool_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const WITHDRAW_IX_ACCOUNTS_LEN: usize = 16;
#[derive(Copy, Clone, Debug)]
pub struct WithdrawAccounts<'me, 'info> {
    pub owner: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub pool_state: &'me AccountInfo<'info>,
    pub user_pool_liquidity: &'me AccountInfo<'info>,
    pub token_0_account: &'me AccountInfo<'info>,
    pub token_1_account: &'me AccountInfo<'info>,
    pub token_0_vault: &'me AccountInfo<'info>,
    pub token_1_vault: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub token_program_2022: &'me AccountInfo<'info>,
    pub vault_0_mint: &'me AccountInfo<'info>,
    pub vault_1_mint: &'me AccountInfo<'info>,
    pub pool_partners: &'me AccountInfo<'info>,
    pub memo_program: &'me AccountInfo<'info>,
    pub kamino_program: &'me AccountInfo<'info>,
    pub instruction_sysvar_account: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct WithdrawKeys {
    pub owner: Pubkey,
    pub authority: Pubkey,
    pub pool_state: Pubkey,
    pub user_pool_liquidity: Pubkey,
    pub token_0_account: Pubkey,
    pub token_1_account: Pubkey,
    pub token_0_vault: Pubkey,
    pub token_1_vault: Pubkey,
    pub token_program: Pubkey,
    pub token_program_2022: Pubkey,
    pub vault_0_mint: Pubkey,
    pub vault_1_mint: Pubkey,
    pub pool_partners: Pubkey,
    pub memo_program: Pubkey,
    pub kamino_program: Pubkey,
    pub instruction_sysvar_account: Pubkey,
}
impl From<WithdrawAccounts<'_, '_>> for WithdrawKeys {
    fn from(accounts: WithdrawAccounts) -> Self {
        Self {
            owner: *accounts.owner.key,
            authority: *accounts.authority.key,
            pool_state: *accounts.pool_state.key,
            user_pool_liquidity: *accounts.user_pool_liquidity.key,
            token_0_account: *accounts.token_0_account.key,
            token_1_account: *accounts.token_1_account.key,
            token_0_vault: *accounts.token_0_vault.key,
            token_1_vault: *accounts.token_1_vault.key,
            token_program: *accounts.token_program.key,
            token_program_2022: *accounts.token_program_2022.key,
            vault_0_mint: *accounts.vault_0_mint.key,
            vault_1_mint: *accounts.vault_1_mint.key,
            pool_partners: *accounts.pool_partners.key,
            memo_program: *accounts.memo_program.key,
            kamino_program: *accounts.kamino_program.key,
            instruction_sysvar_account: *accounts.instruction_sysvar_account.key,
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
                pubkey: keys.user_pool_liquidity,
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
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vault_1_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pool_partners,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.memo_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.kamino_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.instruction_sysvar_account,
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
            user_pool_liquidity: pubkeys[3],
            token_0_account: pubkeys[4],
            token_1_account: pubkeys[5],
            token_0_vault: pubkeys[6],
            token_1_vault: pubkeys[7],
            token_program: pubkeys[8],
            token_program_2022: pubkeys[9],
            vault_0_mint: pubkeys[10],
            vault_1_mint: pubkeys[11],
            pool_partners: pubkeys[12],
            memo_program: pubkeys[13],
            kamino_program: pubkeys[14],
            instruction_sysvar_account: pubkeys[15],
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
            accounts.user_pool_liquidity.clone(),
            accounts.token_0_account.clone(),
            accounts.token_1_account.clone(),
            accounts.token_0_vault.clone(),
            accounts.token_1_vault.clone(),
            accounts.token_program.clone(),
            accounts.token_program_2022.clone(),
            accounts.vault_0_mint.clone(),
            accounts.vault_1_mint.clone(),
            accounts.pool_partners.clone(),
            accounts.memo_program.clone(),
            accounts.kamino_program.clone(),
            accounts.instruction_sysvar_account.clone(),
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
            user_pool_liquidity: &arr[3],
            token_0_account: &arr[4],
            token_1_account: &arr[5],
            token_0_vault: &arr[6],
            token_1_vault: &arr[7],
            token_program: &arr[8],
            token_program_2022: &arr[9],
            vault_0_mint: &arr[10],
            vault_1_mint: &arr[11],
            pool_partners: &arr[12],
            memo_program: &arr[13],
            kamino_program: &arr[14],
            instruction_sysvar_account: &arr[15],
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
    withdraw_ix_with_program_id(GAMMA_PROGRAM_ID, keys, args)
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
    withdraw_invoke_with_program_id(GAMMA_PROGRAM_ID, accounts, args)
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
    withdraw_invoke_signed_with_program_id(GAMMA_PROGRAM_ID, accounts, args, seeds)
}
pub fn withdraw_verify_account_keys(
    accounts: WithdrawAccounts<'_, '_>,
    keys: WithdrawKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.owner.key, keys.owner),
        (*accounts.authority.key, keys.authority),
        (*accounts.pool_state.key, keys.pool_state),
        (*accounts.user_pool_liquidity.key, keys.user_pool_liquidity),
        (*accounts.token_0_account.key, keys.token_0_account),
        (*accounts.token_1_account.key, keys.token_1_account),
        (*accounts.token_0_vault.key, keys.token_0_vault),
        (*accounts.token_1_vault.key, keys.token_1_vault),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.token_program_2022.key, keys.token_program_2022),
        (*accounts.vault_0_mint.key, keys.vault_0_mint),
        (*accounts.vault_1_mint.key, keys.vault_1_mint),
        (*accounts.pool_partners.key, keys.pool_partners),
        (*accounts.memo_program.key, keys.memo_program),
        (*accounts.kamino_program.key, keys.kamino_program),
        (*accounts.instruction_sysvar_account.key, keys.instruction_sysvar_account),
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
        accounts.user_pool_liquidity,
        accounts.token_0_account,
        accounts.token_1_account,
        accounts.token_0_vault,
        accounts.token_1_vault,
        accounts.vault_0_mint,
        accounts.vault_1_mint,
        accounts.pool_partners,
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
