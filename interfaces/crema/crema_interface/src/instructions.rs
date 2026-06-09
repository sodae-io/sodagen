use solana_pubkey::Pubkey;
use solana_cpi::{invoke, invoke_signed};
use solana_instruction::{AccountMeta, Instruction};
use solana_account_info::AccountInfo;
use solana_program_error::ProgramError;
use std::io::Read;
#[allow(unused_imports)]
use crate::*;
#[derive(Clone, Debug, PartialEq)]
pub enum CremaProgramIx {
    InitializeClmmConfig(InitializeClmmConfigIxArgs),
    CreateFeeTier(CreateFeeTierIxArgs),
    UpdateConfig(UpdateConfigIxArgs),
    UpdateFeeRate(UpdateFeeRateIxArgs),
    TransferProtocolAuthority,
    AcceptProtocolAuthority,
    CreateClmmpool(CreateClmmpoolIxArgs),
    CreateTickArray(CreateTickArrayIxArgs),
    CreateTickArrayMap,
    OpenPosition(OpenPositionIxArgs),
    RemovePosition,
    IncreaseLiquidity(IncreaseLiquidityIxArgs),
    IncreaseLiquidityWithFixedToken(IncreaseLiquidityWithFixedTokenIxArgs),
    DecreaseLiquidity(DecreaseLiquidityIxArgs),
    Swap(SwapIxArgs),
    CollectFee,
    CollectProtocolFee,
    CreatePartner(CreatePartnerIxArgs),
    UpdatePartner(UpdatePartnerIxArgs),
    CollectPartnerFee,
    SwapWithPartner(SwapWithPartnerIxArgs),
    InitializeRewarder(InitializeRewarderIxArgs),
    UpdateRewarderEmission(UpdateRewarderEmissionIxArgs),
    CollectRewarder(CollectRewarderIxArgs),
    TransferPartnerClaimAuthority,
    AcceptPartnerClaimAuthority,
    PauseClmmpool,
    UnpauseClmmpool,
    CreateClmmpoolMetadata(CreateClmmpoolMetadataIxArgs),
}
impl CremaProgramIx {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        if buf.starts_with(&INITIALIZE_CLMM_CONFIG_IX_DISCM) {
            let mut reader = &buf[INITIALIZE_CLMM_CONFIG_IX_DISCM.len()..];
            let protocol_authority: Pubkey = crate::borsh_de_or_default(&mut reader)?;
            let protocol_fee_claim_authority: Pubkey = crate::borsh_de_or_default(
                &mut reader,
            )?;
            let create_pool_authority: Pubkey = crate::borsh_de_or_default(&mut reader)?;
            let protocol_fee_rate: u16 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::InitializeClmmConfig(InitializeClmmConfigIxArgs {
                    protocol_authority,
                    protocol_fee_claim_authority,
                    create_pool_authority,
                    protocol_fee_rate,
                }),
            );
        }
        if buf.starts_with(&CREATE_FEE_TIER_IX_DISCM) {
            let mut reader = &buf[CREATE_FEE_TIER_IX_DISCM.len()..];
            let tick_spacing: u16 = crate::borsh_de_or_default(&mut reader)?;
            let fee_rate: u16 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::CreateFeeTier(CreateFeeTierIxArgs {
                    tick_spacing,
                    fee_rate,
                }),
            );
        }
        if buf.starts_with(&UPDATE_CONFIG_IX_DISCM) {
            let mut reader = &buf[UPDATE_CONFIG_IX_DISCM.len()..];
            let new_protocol_fee_rate: Option<u16> = crate::borsh_de_or_default(
                &mut reader,
            )?;
            let create_pool_authority: Option<Pubkey> = crate::borsh_de_or_default(
                &mut reader,
            )?;
            let claim_authority: Option<Pubkey> = crate::borsh_de_or_default(
                &mut reader,
            )?;
            return Ok(
                Self::UpdateConfig(UpdateConfigIxArgs {
                    new_protocol_fee_rate,
                    create_pool_authority,
                    claim_authority,
                }),
            );
        }
        if buf.starts_with(&UPDATE_FEE_RATE_IX_DISCM) {
            let mut reader = &buf[UPDATE_FEE_RATE_IX_DISCM.len()..];
            let new_fee_rate: u16 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::UpdateFeeRate(UpdateFeeRateIxArgs {
                    new_fee_rate,
                }),
            );
        }
        if buf.starts_with(&TRANSFER_PROTOCOL_AUTHORITY_IX_DISCM) {
            return Ok(Self::TransferProtocolAuthority);
        }
        if buf.starts_with(&ACCEPT_PROTOCOL_AUTHORITY_IX_DISCM) {
            return Ok(Self::AcceptProtocolAuthority);
        }
        if buf.starts_with(&CREATE_CLMMPOOL_IX_DISCM) {
            let mut reader = &buf[CREATE_CLMMPOOL_IX_DISCM.len()..];
            let init_sqrt_price: u128 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::CreateClmmpool(CreateClmmpoolIxArgs {
                    init_sqrt_price,
                }),
            );
        }
        if buf.starts_with(&CREATE_TICK_ARRAY_IX_DISCM) {
            let mut reader = &buf[CREATE_TICK_ARRAY_IX_DISCM.len()..];
            let array_index: u16 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::CreateTickArray(CreateTickArrayIxArgs {
                    array_index,
                }),
            );
        }
        if buf.starts_with(&CREATE_TICK_ARRAY_MAP_IX_DISCM) {
            return Ok(Self::CreateTickArrayMap);
        }
        if buf.starts_with(&OPEN_POSITION_IX_DISCM) {
            let mut reader = &buf[OPEN_POSITION_IX_DISCM.len()..];
            let tick_lower_index: i32 = crate::borsh_de_or_default(&mut reader)?;
            let tick_upper_index: i32 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::OpenPosition(OpenPositionIxArgs {
                    tick_lower_index,
                    tick_upper_index,
                }),
            );
        }
        if buf.starts_with(&REMOVE_POSITION_IX_DISCM) {
            return Ok(Self::RemovePosition);
        }
        if buf.starts_with(&INCREASE_LIQUIDITY_IX_DISCM) {
            let mut reader = &buf[INCREASE_LIQUIDITY_IX_DISCM.len()..];
            let delta_liquidity: u128 = crate::borsh_de_or_default(&mut reader)?;
            let token_a_max: u64 = crate::borsh_de_or_default(&mut reader)?;
            let token_b_max: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::IncreaseLiquidity(IncreaseLiquidityIxArgs {
                    delta_liquidity,
                    token_a_max,
                    token_b_max,
                }),
            );
        }
        if buf.starts_with(&INCREASE_LIQUIDITY_WITH_FIXED_TOKEN_IX_DISCM) {
            let mut reader = &buf[INCREASE_LIQUIDITY_WITH_FIXED_TOKEN_IX_DISCM.len()..];
            let token_a: u64 = crate::borsh_de_or_default(&mut reader)?;
            let token_b: u64 = crate::borsh_de_or_default(&mut reader)?;
            let is_a_fixed: bool = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::IncreaseLiquidityWithFixedToken(IncreaseLiquidityWithFixedTokenIxArgs {
                    token_a,
                    token_b,
                    is_a_fixed,
                }),
            );
        }
        if buf.starts_with(&DECREASE_LIQUIDITY_IX_DISCM) {
            let mut reader = &buf[DECREASE_LIQUIDITY_IX_DISCM.len()..];
            let delta_liquidity: u128 = crate::borsh_de_or_default(&mut reader)?;
            let token_a_min: u64 = crate::borsh_de_or_default(&mut reader)?;
            let token_b_min: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::DecreaseLiquidity(DecreaseLiquidityIxArgs {
                    delta_liquidity,
                    token_a_min,
                    token_b_min,
                }),
            );
        }
        if buf.starts_with(&SWAP_IX_DISCM) {
            let mut reader = &buf[SWAP_IX_DISCM.len()..];
            let a_to_b: bool = crate::borsh_de_or_default(&mut reader)?;
            let by_amount_in: bool = crate::borsh_de_or_default(&mut reader)?;
            let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            let amount_limit: u64 = crate::borsh_de_or_default(&mut reader)?;
            let sqrt_price_limit: u128 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::Swap(SwapIxArgs {
                    a_to_b,
                    by_amount_in,
                    amount,
                    amount_limit,
                    sqrt_price_limit,
                }),
            );
        }
        if buf.starts_with(&COLLECT_FEE_IX_DISCM) {
            return Ok(Self::CollectFee);
        }
        if buf.starts_with(&COLLECT_PROTOCOL_FEE_IX_DISCM) {
            return Ok(Self::CollectProtocolFee);
        }
        if buf.starts_with(&CREATE_PARTNER_IX_DISCM) {
            let mut reader = &buf[CREATE_PARTNER_IX_DISCM.len()..];
            let partner_fee_claim_authority: Pubkey = crate::borsh_de_or_default(
                &mut reader,
            )?;
            let fee_rate: u16 = crate::borsh_de_or_default(&mut reader)?;
            let start_time: u64 = crate::borsh_de_or_default(&mut reader)?;
            let end_time: u64 = crate::borsh_de_or_default(&mut reader)?;
            let name: String = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::CreatePartner(CreatePartnerIxArgs {
                    partner_fee_claim_authority,
                    fee_rate,
                    start_time,
                    end_time,
                    name,
                }),
            );
        }
        if buf.starts_with(&UPDATE_PARTNER_IX_DISCM) {
            let mut reader = &buf[UPDATE_PARTNER_IX_DISCM.len()..];
            let new_fee_rate: Option<u16> = crate::borsh_de_or_default(&mut reader)?;
            let new_claim_authority: Option<Pubkey> = crate::borsh_de_or_default(
                &mut reader,
            )?;
            let start_time: Option<u64> = crate::borsh_de_or_default(&mut reader)?;
            let end_time: Option<u64> = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::UpdatePartner(UpdatePartnerIxArgs {
                    new_fee_rate,
                    new_claim_authority,
                    start_time,
                    end_time,
                }),
            );
        }
        if buf.starts_with(&COLLECT_PARTNER_FEE_IX_DISCM) {
            return Ok(Self::CollectPartnerFee);
        }
        if buf.starts_with(&SWAP_WITH_PARTNER_IX_DISCM) {
            let mut reader = &buf[SWAP_WITH_PARTNER_IX_DISCM.len()..];
            let a_to_b: bool = crate::borsh_de_or_default(&mut reader)?;
            let by_amount_in: bool = crate::borsh_de_or_default(&mut reader)?;
            let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            let amount_limit: u64 = crate::borsh_de_or_default(&mut reader)?;
            let sqrt_price_limit: u128 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::SwapWithPartner(SwapWithPartnerIxArgs {
                    a_to_b,
                    by_amount_in,
                    amount,
                    amount_limit,
                    sqrt_price_limit,
                }),
            );
        }
        if buf.starts_with(&INITIALIZE_REWARDER_IX_DISCM) {
            let mut reader = &buf[INITIALIZE_REWARDER_IX_DISCM.len()..];
            let rewarder_index: u8 = crate::borsh_de_or_default(&mut reader)?;
            let mint_wrapper: Pubkey = crate::borsh_de_or_default(&mut reader)?;
            let minter: Pubkey = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::InitializeRewarder(InitializeRewarderIxArgs {
                    rewarder_index,
                    mint_wrapper,
                    minter,
                }),
            );
        }
        if buf.starts_with(&UPDATE_REWARDER_EMISSION_IX_DISCM) {
            let mut reader = &buf[UPDATE_REWARDER_EMISSION_IX_DISCM.len()..];
            let rewarder_index: u8 = crate::borsh_de_or_default(&mut reader)?;
            let emissions_per_second: u128 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::UpdateRewarderEmission(UpdateRewarderEmissionIxArgs {
                    rewarder_index,
                    emissions_per_second,
                }),
            );
        }
        if buf.starts_with(&COLLECT_REWARDER_IX_DISCM) {
            let mut reader = &buf[COLLECT_REWARDER_IX_DISCM.len()..];
            let rewarder_index: u8 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::CollectRewarder(CollectRewarderIxArgs {
                    rewarder_index,
                }),
            );
        }
        if buf.starts_with(&TRANSFER_PARTNER_CLAIM_AUTHORITY_IX_DISCM) {
            return Ok(Self::TransferPartnerClaimAuthority);
        }
        if buf.starts_with(&ACCEPT_PARTNER_CLAIM_AUTHORITY_IX_DISCM) {
            return Ok(Self::AcceptPartnerClaimAuthority);
        }
        if buf.starts_with(&PAUSE_CLMMPOOL_IX_DISCM) {
            return Ok(Self::PauseClmmpool);
        }
        if buf.starts_with(&UNPAUSE_CLMMPOOL_IX_DISCM) {
            return Ok(Self::UnpauseClmmpool);
        }
        if buf.starts_with(&CREATE_CLMMPOOL_METADATA_IX_DISCM) {
            let mut reader = &buf[CREATE_CLMMPOOL_METADATA_IX_DISCM.len()..];
            let name: String = crate::borsh_de_or_default(&mut reader)?;
            let uri: String = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::CreateClmmpoolMetadata(CreateClmmpoolMetadataIxArgs {
                    name,
                    uri,
                }),
            );
        }
        Err(std::io::Error::from(std::io::ErrorKind::InvalidData))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        match self {
            Self::InitializeClmmConfig(args) => {
                writer.write_all(&INITIALIZE_CLMM_CONFIG_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.protocol_authority, &mut writer)?;
                borsh::BorshSerialize::serialize(
                    &args.protocol_fee_claim_authority,
                    &mut writer,
                )?;
                borsh::BorshSerialize::serialize(
                    &args.create_pool_authority,
                    &mut writer,
                )?;
                borsh::BorshSerialize::serialize(&args.protocol_fee_rate, &mut writer)?;
                Ok(())
            }
            Self::CreateFeeTier(args) => {
                writer.write_all(&CREATE_FEE_TIER_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.tick_spacing, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.fee_rate, &mut writer)?;
                Ok(())
            }
            Self::UpdateConfig(args) => {
                writer.write_all(&UPDATE_CONFIG_IX_DISCM)?;
                borsh::BorshSerialize::serialize(
                    &args.new_protocol_fee_rate,
                    &mut writer,
                )?;
                borsh::BorshSerialize::serialize(
                    &args.create_pool_authority,
                    &mut writer,
                )?;
                borsh::BorshSerialize::serialize(&args.claim_authority, &mut writer)?;
                Ok(())
            }
            Self::UpdateFeeRate(args) => {
                writer.write_all(&UPDATE_FEE_RATE_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.new_fee_rate, &mut writer)?;
                Ok(())
            }
            Self::TransferProtocolAuthority => {
                writer.write_all(&TRANSFER_PROTOCOL_AUTHORITY_IX_DISCM)
            }
            Self::AcceptProtocolAuthority => {
                writer.write_all(&ACCEPT_PROTOCOL_AUTHORITY_IX_DISCM)
            }
            Self::CreateClmmpool(args) => {
                writer.write_all(&CREATE_CLMMPOOL_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.init_sqrt_price, &mut writer)?;
                Ok(())
            }
            Self::CreateTickArray(args) => {
                writer.write_all(&CREATE_TICK_ARRAY_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.array_index, &mut writer)?;
                Ok(())
            }
            Self::CreateTickArrayMap => writer.write_all(&CREATE_TICK_ARRAY_MAP_IX_DISCM),
            Self::OpenPosition(args) => {
                writer.write_all(&OPEN_POSITION_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.tick_lower_index, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.tick_upper_index, &mut writer)?;
                Ok(())
            }
            Self::RemovePosition => writer.write_all(&REMOVE_POSITION_IX_DISCM),
            Self::IncreaseLiquidity(args) => {
                writer.write_all(&INCREASE_LIQUIDITY_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.delta_liquidity, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.token_a_max, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.token_b_max, &mut writer)?;
                Ok(())
            }
            Self::IncreaseLiquidityWithFixedToken(args) => {
                writer.write_all(&INCREASE_LIQUIDITY_WITH_FIXED_TOKEN_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.token_a, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.token_b, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.is_a_fixed, &mut writer)?;
                Ok(())
            }
            Self::DecreaseLiquidity(args) => {
                writer.write_all(&DECREASE_LIQUIDITY_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.delta_liquidity, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.token_a_min, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.token_b_min, &mut writer)?;
                Ok(())
            }
            Self::Swap(args) => {
                writer.write_all(&SWAP_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.a_to_b, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.by_amount_in, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.amount, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.amount_limit, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.sqrt_price_limit, &mut writer)?;
                Ok(())
            }
            Self::CollectFee => writer.write_all(&COLLECT_FEE_IX_DISCM),
            Self::CollectProtocolFee => writer.write_all(&COLLECT_PROTOCOL_FEE_IX_DISCM),
            Self::CreatePartner(args) => {
                writer.write_all(&CREATE_PARTNER_IX_DISCM)?;
                borsh::BorshSerialize::serialize(
                    &args.partner_fee_claim_authority,
                    &mut writer,
                )?;
                borsh::BorshSerialize::serialize(&args.fee_rate, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.start_time, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.end_time, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.name, &mut writer)?;
                Ok(())
            }
            Self::UpdatePartner(args) => {
                writer.write_all(&UPDATE_PARTNER_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.new_fee_rate, &mut writer)?;
                borsh::BorshSerialize::serialize(
                    &args.new_claim_authority,
                    &mut writer,
                )?;
                borsh::BorshSerialize::serialize(&args.start_time, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.end_time, &mut writer)?;
                Ok(())
            }
            Self::CollectPartnerFee => writer.write_all(&COLLECT_PARTNER_FEE_IX_DISCM),
            Self::SwapWithPartner(args) => {
                writer.write_all(&SWAP_WITH_PARTNER_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.a_to_b, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.by_amount_in, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.amount, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.amount_limit, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.sqrt_price_limit, &mut writer)?;
                Ok(())
            }
            Self::InitializeRewarder(args) => {
                writer.write_all(&INITIALIZE_REWARDER_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.rewarder_index, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.mint_wrapper, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.minter, &mut writer)?;
                Ok(())
            }
            Self::UpdateRewarderEmission(args) => {
                writer.write_all(&UPDATE_REWARDER_EMISSION_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.rewarder_index, &mut writer)?;
                borsh::BorshSerialize::serialize(
                    &args.emissions_per_second,
                    &mut writer,
                )?;
                Ok(())
            }
            Self::CollectRewarder(args) => {
                writer.write_all(&COLLECT_REWARDER_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.rewarder_index, &mut writer)?;
                Ok(())
            }
            Self::TransferPartnerClaimAuthority => {
                writer.write_all(&TRANSFER_PARTNER_CLAIM_AUTHORITY_IX_DISCM)
            }
            Self::AcceptPartnerClaimAuthority => {
                writer.write_all(&ACCEPT_PARTNER_CLAIM_AUTHORITY_IX_DISCM)
            }
            Self::PauseClmmpool => writer.write_all(&PAUSE_CLMMPOOL_IX_DISCM),
            Self::UnpauseClmmpool => writer.write_all(&UNPAUSE_CLMMPOOL_IX_DISCM),
            Self::CreateClmmpoolMetadata(args) => {
                writer.write_all(&CREATE_CLMMPOOL_METADATA_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.name, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.uri, &mut writer)?;
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
pub const INITIALIZE_CLMM_CONFIG_IX_ACCOUNTS_LEN: usize = 4;
#[derive(Copy, Clone, Debug)]
pub struct InitializeClmmConfigAccounts<'me, 'info> {
    pub payer: &'me AccountInfo<'info>,
    pub clmm_config: &'me AccountInfo<'info>,
    pub rent: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct InitializeClmmConfigKeys {
    pub payer: Pubkey,
    pub clmm_config: Pubkey,
    pub rent: Pubkey,
    pub system_program: Pubkey,
}
impl From<InitializeClmmConfigAccounts<'_, '_>> for InitializeClmmConfigKeys {
    fn from(accounts: InitializeClmmConfigAccounts) -> Self {
        Self {
            payer: *accounts.payer.key,
            clmm_config: *accounts.clmm_config.key,
            rent: *accounts.rent.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<InitializeClmmConfigKeys>
for [AccountMeta; INITIALIZE_CLMM_CONFIG_IX_ACCOUNTS_LEN] {
    fn from(keys: InitializeClmmConfigKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.payer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.clmm_config,
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
        ]
    }
}
impl From<[Pubkey; INITIALIZE_CLMM_CONFIG_IX_ACCOUNTS_LEN]>
for InitializeClmmConfigKeys {
    fn from(pubkeys: [Pubkey; INITIALIZE_CLMM_CONFIG_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            payer: pubkeys[0],
            clmm_config: pubkeys[1],
            rent: pubkeys[2],
            system_program: pubkeys[3],
        }
    }
}
impl<'info> From<InitializeClmmConfigAccounts<'_, 'info>>
for [AccountInfo<'info>; INITIALIZE_CLMM_CONFIG_IX_ACCOUNTS_LEN] {
    fn from(accounts: InitializeClmmConfigAccounts<'_, 'info>) -> Self {
        [
            accounts.payer.clone(),
            accounts.clmm_config.clone(),
            accounts.rent.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; INITIALIZE_CLMM_CONFIG_IX_ACCOUNTS_LEN]>
for InitializeClmmConfigAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; INITIALIZE_CLMM_CONFIG_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            payer: &arr[0],
            clmm_config: &arr[1],
            rent: &arr[2],
            system_program: &arr[3],
        }
    }
}
pub const INITIALIZE_CLMM_CONFIG_IX_DISCM: [u8; 8usize] = [
    180, 32, 62, 76, 60, 4, 156, 171,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct InitializeClmmConfigIxArgs {
    pub protocol_authority: Pubkey,
    pub protocol_fee_claim_authority: Pubkey,
    pub create_pool_authority: Pubkey,
    pub protocol_fee_rate: u16,
}
#[derive(Clone, Debug, PartialEq)]
pub struct InitializeClmmConfigIxData(pub InitializeClmmConfigIxArgs);
impl From<InitializeClmmConfigIxArgs> for InitializeClmmConfigIxData {
    fn from(args: InitializeClmmConfigIxArgs) -> Self {
        Self(args)
    }
}
impl InitializeClmmConfigIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INITIALIZE_CLMM_CONFIG_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let protocol_authority: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let protocol_fee_claim_authority: Pubkey = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let create_pool_authority: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let protocol_fee_rate: u16 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(InitializeClmmConfigIxArgs {
                protocol_authority,
                protocol_fee_claim_authority,
                create_pool_authority,
                protocol_fee_rate,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INITIALIZE_CLMM_CONFIG_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.protocol_authority, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.0.protocol_fee_claim_authority,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.0.create_pool_authority, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.protocol_fee_rate, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn initialize_clmm_config_ix_with_program_id(
    program_id: Pubkey,
    keys: InitializeClmmConfigKeys,
    args: InitializeClmmConfigIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; INITIALIZE_CLMM_CONFIG_IX_ACCOUNTS_LEN] = keys.into();
    let data: InitializeClmmConfigIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn initialize_clmm_config_ix(
    keys: InitializeClmmConfigKeys,
    args: InitializeClmmConfigIxArgs,
) -> std::io::Result<Instruction> {
    initialize_clmm_config_ix_with_program_id(CREMA_PROGRAM_ID, keys, args)
}
pub fn initialize_clmm_config_invoke_with_program_id(
    program_id: Pubkey,
    accounts: InitializeClmmConfigAccounts<'_, '_>,
    args: InitializeClmmConfigIxArgs,
) -> ProgramResult {
    let keys: InitializeClmmConfigKeys = accounts.into();
    let ix = initialize_clmm_config_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn initialize_clmm_config_invoke(
    accounts: InitializeClmmConfigAccounts<'_, '_>,
    args: InitializeClmmConfigIxArgs,
) -> ProgramResult {
    initialize_clmm_config_invoke_with_program_id(CREMA_PROGRAM_ID, accounts, args)
}
pub fn initialize_clmm_config_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: InitializeClmmConfigAccounts<'_, '_>,
    args: InitializeClmmConfigIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: InitializeClmmConfigKeys = accounts.into();
    let ix = initialize_clmm_config_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn initialize_clmm_config_invoke_signed(
    accounts: InitializeClmmConfigAccounts<'_, '_>,
    args: InitializeClmmConfigIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    initialize_clmm_config_invoke_signed_with_program_id(
        CREMA_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn initialize_clmm_config_verify_account_keys(
    accounts: InitializeClmmConfigAccounts<'_, '_>,
    keys: InitializeClmmConfigKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.payer.key, keys.payer),
        (*accounts.clmm_config.key, keys.clmm_config),
        (*accounts.rent.key, keys.rent),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn initialize_clmm_config_verify_writable_privileges<'me, 'info>(
    accounts: InitializeClmmConfigAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.payer, accounts.clmm_config] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn initialize_clmm_config_verify_signer_privileges<'me, 'info>(
    accounts: InitializeClmmConfigAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.payer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn initialize_clmm_config_verify_account_privileges<'me, 'info>(
    accounts: InitializeClmmConfigAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    initialize_clmm_config_verify_writable_privileges(accounts)?;
    initialize_clmm_config_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CREATE_FEE_TIER_IX_ACCOUNTS_LEN: usize = 6;
#[derive(Copy, Clone, Debug)]
pub struct CreateFeeTierAccounts<'me, 'info> {
    pub payer: &'me AccountInfo<'info>,
    pub clmm_config: &'me AccountInfo<'info>,
    pub fee_tier: &'me AccountInfo<'info>,
    pub protocol_authority: &'me AccountInfo<'info>,
    pub rent: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CreateFeeTierKeys {
    pub payer: Pubkey,
    pub clmm_config: Pubkey,
    pub fee_tier: Pubkey,
    pub protocol_authority: Pubkey,
    pub rent: Pubkey,
    pub system_program: Pubkey,
}
impl From<CreateFeeTierAccounts<'_, '_>> for CreateFeeTierKeys {
    fn from(accounts: CreateFeeTierAccounts) -> Self {
        Self {
            payer: *accounts.payer.key,
            clmm_config: *accounts.clmm_config.key,
            fee_tier: *accounts.fee_tier.key,
            protocol_authority: *accounts.protocol_authority.key,
            rent: *accounts.rent.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<CreateFeeTierKeys> for [AccountMeta; CREATE_FEE_TIER_IX_ACCOUNTS_LEN] {
    fn from(keys: CreateFeeTierKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.payer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.clmm_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.fee_tier,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.protocol_authority,
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
impl From<[Pubkey; CREATE_FEE_TIER_IX_ACCOUNTS_LEN]> for CreateFeeTierKeys {
    fn from(pubkeys: [Pubkey; CREATE_FEE_TIER_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            payer: pubkeys[0],
            clmm_config: pubkeys[1],
            fee_tier: pubkeys[2],
            protocol_authority: pubkeys[3],
            rent: pubkeys[4],
            system_program: pubkeys[5],
        }
    }
}
impl<'info> From<CreateFeeTierAccounts<'_, 'info>>
for [AccountInfo<'info>; CREATE_FEE_TIER_IX_ACCOUNTS_LEN] {
    fn from(accounts: CreateFeeTierAccounts<'_, 'info>) -> Self {
        [
            accounts.payer.clone(),
            accounts.clmm_config.clone(),
            accounts.fee_tier.clone(),
            accounts.protocol_authority.clone(),
            accounts.rent.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CREATE_FEE_TIER_IX_ACCOUNTS_LEN]>
for CreateFeeTierAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; CREATE_FEE_TIER_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            payer: &arr[0],
            clmm_config: &arr[1],
            fee_tier: &arr[2],
            protocol_authority: &arr[3],
            rent: &arr[4],
            system_program: &arr[5],
        }
    }
}
pub const CREATE_FEE_TIER_IX_DISCM: [u8; 8usize] = [150, 158, 85, 114, 219, 75, 212, 91];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CreateFeeTierIxArgs {
    pub tick_spacing: u16,
    pub fee_rate: u16,
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
        let tick_spacing: u16 = crate::borsh_de_or_default(&mut reader)?;
        let fee_rate: u16 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(CreateFeeTierIxArgs {
                tick_spacing,
                fee_rate,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CREATE_FEE_TIER_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.tick_spacing, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.fee_rate, &mut writer)?;
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
    create_fee_tier_ix_with_program_id(CREMA_PROGRAM_ID, keys, args)
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
    create_fee_tier_invoke_with_program_id(CREMA_PROGRAM_ID, accounts, args)
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
        CREMA_PROGRAM_ID,
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
        (*accounts.payer.key, keys.payer),
        (*accounts.clmm_config.key, keys.clmm_config),
        (*accounts.fee_tier.key, keys.fee_tier),
        (*accounts.protocol_authority.key, keys.protocol_authority),
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
    for should_be_writable in [accounts.payer, accounts.fee_tier] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn create_fee_tier_verify_signer_privileges<'me, 'info>(
    accounts: CreateFeeTierAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.payer, accounts.protocol_authority] {
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
pub const UPDATE_CONFIG_IX_ACCOUNTS_LEN: usize = 2;
#[derive(Copy, Clone, Debug)]
pub struct UpdateConfigAccounts<'me, 'info> {
    pub clmm_config: &'me AccountInfo<'info>,
    pub protocol_authority: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UpdateConfigKeys {
    pub clmm_config: Pubkey,
    pub protocol_authority: Pubkey,
}
impl From<UpdateConfigAccounts<'_, '_>> for UpdateConfigKeys {
    fn from(accounts: UpdateConfigAccounts) -> Self {
        Self {
            clmm_config: *accounts.clmm_config.key,
            protocol_authority: *accounts.protocol_authority.key,
        }
    }
}
impl From<UpdateConfigKeys> for [AccountMeta; UPDATE_CONFIG_IX_ACCOUNTS_LEN] {
    fn from(keys: UpdateConfigKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.clmm_config,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.protocol_authority,
                is_signer: true,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; UPDATE_CONFIG_IX_ACCOUNTS_LEN]> for UpdateConfigKeys {
    fn from(pubkeys: [Pubkey; UPDATE_CONFIG_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            clmm_config: pubkeys[0],
            protocol_authority: pubkeys[1],
        }
    }
}
impl<'info> From<UpdateConfigAccounts<'_, 'info>>
for [AccountInfo<'info>; UPDATE_CONFIG_IX_ACCOUNTS_LEN] {
    fn from(accounts: UpdateConfigAccounts<'_, 'info>) -> Self {
        [accounts.clmm_config.clone(), accounts.protocol_authority.clone()]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; UPDATE_CONFIG_IX_ACCOUNTS_LEN]>
for UpdateConfigAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; UPDATE_CONFIG_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            clmm_config: &arr[0],
            protocol_authority: &arr[1],
        }
    }
}
pub const UPDATE_CONFIG_IX_DISCM: [u8; 8usize] = [29, 158, 252, 191, 10, 83, 219, 99];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UpdateConfigIxArgs {
    pub new_protocol_fee_rate: Option<u16>,
    pub create_pool_authority: Option<Pubkey>,
    pub claim_authority: Option<Pubkey>,
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
        let new_protocol_fee_rate: Option<u16> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let create_pool_authority: Option<Pubkey> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let claim_authority: Option<Pubkey> = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(UpdateConfigIxArgs {
                new_protocol_fee_rate,
                create_pool_authority,
                claim_authority,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_CONFIG_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.new_protocol_fee_rate, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.create_pool_authority, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.claim_authority, &mut writer)?;
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
    update_config_ix_with_program_id(CREMA_PROGRAM_ID, keys, args)
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
    update_config_invoke_with_program_id(CREMA_PROGRAM_ID, accounts, args)
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
    update_config_invoke_signed_with_program_id(CREMA_PROGRAM_ID, accounts, args, seeds)
}
pub fn update_config_verify_account_keys(
    accounts: UpdateConfigAccounts<'_, '_>,
    keys: UpdateConfigKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.clmm_config.key, keys.clmm_config),
        (*accounts.protocol_authority.key, keys.protocol_authority),
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
    for should_be_writable in [accounts.clmm_config] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn update_config_verify_signer_privileges<'me, 'info>(
    accounts: UpdateConfigAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.protocol_authority] {
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
pub const UPDATE_FEE_RATE_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct UpdateFeeRateAccounts<'me, 'info> {
    pub protocol_authority: &'me AccountInfo<'info>,
    pub clmm_config: &'me AccountInfo<'info>,
    pub clmmpool: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UpdateFeeRateKeys {
    pub protocol_authority: Pubkey,
    pub clmm_config: Pubkey,
    pub clmmpool: Pubkey,
}
impl From<UpdateFeeRateAccounts<'_, '_>> for UpdateFeeRateKeys {
    fn from(accounts: UpdateFeeRateAccounts) -> Self {
        Self {
            protocol_authority: *accounts.protocol_authority.key,
            clmm_config: *accounts.clmm_config.key,
            clmmpool: *accounts.clmmpool.key,
        }
    }
}
impl From<UpdateFeeRateKeys> for [AccountMeta; UPDATE_FEE_RATE_IX_ACCOUNTS_LEN] {
    fn from(keys: UpdateFeeRateKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.protocol_authority,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.clmm_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.clmmpool,
                is_signer: false,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; UPDATE_FEE_RATE_IX_ACCOUNTS_LEN]> for UpdateFeeRateKeys {
    fn from(pubkeys: [Pubkey; UPDATE_FEE_RATE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            protocol_authority: pubkeys[0],
            clmm_config: pubkeys[1],
            clmmpool: pubkeys[2],
        }
    }
}
impl<'info> From<UpdateFeeRateAccounts<'_, 'info>>
for [AccountInfo<'info>; UPDATE_FEE_RATE_IX_ACCOUNTS_LEN] {
    fn from(accounts: UpdateFeeRateAccounts<'_, 'info>) -> Self {
        [
            accounts.protocol_authority.clone(),
            accounts.clmm_config.clone(),
            accounts.clmmpool.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; UPDATE_FEE_RATE_IX_ACCOUNTS_LEN]>
for UpdateFeeRateAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; UPDATE_FEE_RATE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            protocol_authority: &arr[0],
            clmm_config: &arr[1],
            clmmpool: &arr[2],
        }
    }
}
pub const UPDATE_FEE_RATE_IX_DISCM: [u8; 8usize] = [195, 241, 226, 216, 102, 1, 5, 122];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UpdateFeeRateIxArgs {
    pub new_fee_rate: u16,
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateFeeRateIxData(pub UpdateFeeRateIxArgs);
impl From<UpdateFeeRateIxArgs> for UpdateFeeRateIxData {
    fn from(args: UpdateFeeRateIxArgs) -> Self {
        Self(args)
    }
}
impl UpdateFeeRateIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_FEE_RATE_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let new_fee_rate: u16 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(UpdateFeeRateIxArgs {
                new_fee_rate,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_FEE_RATE_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.new_fee_rate, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn update_fee_rate_ix_with_program_id(
    program_id: Pubkey,
    keys: UpdateFeeRateKeys,
    args: UpdateFeeRateIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; UPDATE_FEE_RATE_IX_ACCOUNTS_LEN] = keys.into();
    let data: UpdateFeeRateIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_fee_rate_ix(
    keys: UpdateFeeRateKeys,
    args: UpdateFeeRateIxArgs,
) -> std::io::Result<Instruction> {
    update_fee_rate_ix_with_program_id(CREMA_PROGRAM_ID, keys, args)
}
pub fn update_fee_rate_invoke_with_program_id(
    program_id: Pubkey,
    accounts: UpdateFeeRateAccounts<'_, '_>,
    args: UpdateFeeRateIxArgs,
) -> ProgramResult {
    let keys: UpdateFeeRateKeys = accounts.into();
    let ix = update_fee_rate_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn update_fee_rate_invoke(
    accounts: UpdateFeeRateAccounts<'_, '_>,
    args: UpdateFeeRateIxArgs,
) -> ProgramResult {
    update_fee_rate_invoke_with_program_id(CREMA_PROGRAM_ID, accounts, args)
}
pub fn update_fee_rate_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: UpdateFeeRateAccounts<'_, '_>,
    args: UpdateFeeRateIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: UpdateFeeRateKeys = accounts.into();
    let ix = update_fee_rate_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn update_fee_rate_invoke_signed(
    accounts: UpdateFeeRateAccounts<'_, '_>,
    args: UpdateFeeRateIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    update_fee_rate_invoke_signed_with_program_id(
        CREMA_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn update_fee_rate_verify_account_keys(
    accounts: UpdateFeeRateAccounts<'_, '_>,
    keys: UpdateFeeRateKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.protocol_authority.key, keys.protocol_authority),
        (*accounts.clmm_config.key, keys.clmm_config),
        (*accounts.clmmpool.key, keys.clmmpool),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn update_fee_rate_verify_writable_privileges<'me, 'info>(
    accounts: UpdateFeeRateAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.clmmpool] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn update_fee_rate_verify_signer_privileges<'me, 'info>(
    accounts: UpdateFeeRateAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.protocol_authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn update_fee_rate_verify_account_privileges<'me, 'info>(
    accounts: UpdateFeeRateAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    update_fee_rate_verify_writable_privileges(accounts)?;
    update_fee_rate_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const TRANSFER_PROTOCOL_AUTHORITY_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct TransferProtocolAuthorityAccounts<'me, 'info> {
    pub protocol_authority: &'me AccountInfo<'info>,
    pub clmm_config: &'me AccountInfo<'info>,
    pub new_authority: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct TransferProtocolAuthorityKeys {
    pub protocol_authority: Pubkey,
    pub clmm_config: Pubkey,
    pub new_authority: Pubkey,
}
impl From<TransferProtocolAuthorityAccounts<'_, '_>> for TransferProtocolAuthorityKeys {
    fn from(accounts: TransferProtocolAuthorityAccounts) -> Self {
        Self {
            protocol_authority: *accounts.protocol_authority.key,
            clmm_config: *accounts.clmm_config.key,
            new_authority: *accounts.new_authority.key,
        }
    }
}
impl From<TransferProtocolAuthorityKeys>
for [AccountMeta; TRANSFER_PROTOCOL_AUTHORITY_IX_ACCOUNTS_LEN] {
    fn from(keys: TransferProtocolAuthorityKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.protocol_authority,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.clmm_config,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.new_authority,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; TRANSFER_PROTOCOL_AUTHORITY_IX_ACCOUNTS_LEN]>
for TransferProtocolAuthorityKeys {
    fn from(pubkeys: [Pubkey; TRANSFER_PROTOCOL_AUTHORITY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            protocol_authority: pubkeys[0],
            clmm_config: pubkeys[1],
            new_authority: pubkeys[2],
        }
    }
}
impl<'info> From<TransferProtocolAuthorityAccounts<'_, 'info>>
for [AccountInfo<'info>; TRANSFER_PROTOCOL_AUTHORITY_IX_ACCOUNTS_LEN] {
    fn from(accounts: TransferProtocolAuthorityAccounts<'_, 'info>) -> Self {
        [
            accounts.protocol_authority.clone(),
            accounts.clmm_config.clone(),
            accounts.new_authority.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; TRANSFER_PROTOCOL_AUTHORITY_IX_ACCOUNTS_LEN]>
for TransferProtocolAuthorityAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; TRANSFER_PROTOCOL_AUTHORITY_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            protocol_authority: &arr[0],
            clmm_config: &arr[1],
            new_authority: &arr[2],
        }
    }
}
pub const TRANSFER_PROTOCOL_AUTHORITY_IX_DISCM: [u8; 8usize] = [
    35, 76, 36, 77, 136, 112, 158, 222,
];
#[derive(Clone, Debug, PartialEq)]
pub struct TransferProtocolAuthorityIxData;
impl TransferProtocolAuthorityIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != TRANSFER_PROTOCOL_AUTHORITY_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&TRANSFER_PROTOCOL_AUTHORITY_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn transfer_protocol_authority_ix_with_program_id(
    program_id: Pubkey,
    keys: TransferProtocolAuthorityKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; TRANSFER_PROTOCOL_AUTHORITY_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: TransferProtocolAuthorityIxData.try_to_vec()?,
    })
}
pub fn transfer_protocol_authority_ix(
    keys: TransferProtocolAuthorityKeys,
) -> std::io::Result<Instruction> {
    transfer_protocol_authority_ix_with_program_id(CREMA_PROGRAM_ID, keys)
}
pub fn transfer_protocol_authority_invoke_with_program_id(
    program_id: Pubkey,
    accounts: TransferProtocolAuthorityAccounts<'_, '_>,
) -> ProgramResult {
    let keys: TransferProtocolAuthorityKeys = accounts.into();
    let ix = transfer_protocol_authority_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn transfer_protocol_authority_invoke(
    accounts: TransferProtocolAuthorityAccounts<'_, '_>,
) -> ProgramResult {
    transfer_protocol_authority_invoke_with_program_id(CREMA_PROGRAM_ID, accounts)
}
pub fn transfer_protocol_authority_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: TransferProtocolAuthorityAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: TransferProtocolAuthorityKeys = accounts.into();
    let ix = transfer_protocol_authority_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn transfer_protocol_authority_invoke_signed(
    accounts: TransferProtocolAuthorityAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    transfer_protocol_authority_invoke_signed_with_program_id(
        CREMA_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn transfer_protocol_authority_verify_account_keys(
    accounts: TransferProtocolAuthorityAccounts<'_, '_>,
    keys: TransferProtocolAuthorityKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.protocol_authority.key, keys.protocol_authority),
        (*accounts.clmm_config.key, keys.clmm_config),
        (*accounts.new_authority.key, keys.new_authority),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn transfer_protocol_authority_verify_writable_privileges<'me, 'info>(
    accounts: TransferProtocolAuthorityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.clmm_config] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn transfer_protocol_authority_verify_signer_privileges<'me, 'info>(
    accounts: TransferProtocolAuthorityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.protocol_authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn transfer_protocol_authority_verify_account_privileges<'me, 'info>(
    accounts: TransferProtocolAuthorityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    transfer_protocol_authority_verify_writable_privileges(accounts)?;
    transfer_protocol_authority_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const ACCEPT_PROTOCOL_AUTHORITY_IX_ACCOUNTS_LEN: usize = 2;
#[derive(Copy, Clone, Debug)]
pub struct AcceptProtocolAuthorityAccounts<'me, 'info> {
    pub new_authority: &'me AccountInfo<'info>,
    pub clmm_config: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct AcceptProtocolAuthorityKeys {
    pub new_authority: Pubkey,
    pub clmm_config: Pubkey,
}
impl From<AcceptProtocolAuthorityAccounts<'_, '_>> for AcceptProtocolAuthorityKeys {
    fn from(accounts: AcceptProtocolAuthorityAccounts) -> Self {
        Self {
            new_authority: *accounts.new_authority.key,
            clmm_config: *accounts.clmm_config.key,
        }
    }
}
impl From<AcceptProtocolAuthorityKeys>
for [AccountMeta; ACCEPT_PROTOCOL_AUTHORITY_IX_ACCOUNTS_LEN] {
    fn from(keys: AcceptProtocolAuthorityKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.new_authority,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.clmm_config,
                is_signer: false,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; ACCEPT_PROTOCOL_AUTHORITY_IX_ACCOUNTS_LEN]>
for AcceptProtocolAuthorityKeys {
    fn from(pubkeys: [Pubkey; ACCEPT_PROTOCOL_AUTHORITY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            new_authority: pubkeys[0],
            clmm_config: pubkeys[1],
        }
    }
}
impl<'info> From<AcceptProtocolAuthorityAccounts<'_, 'info>>
for [AccountInfo<'info>; ACCEPT_PROTOCOL_AUTHORITY_IX_ACCOUNTS_LEN] {
    fn from(accounts: AcceptProtocolAuthorityAccounts<'_, 'info>) -> Self {
        [accounts.new_authority.clone(), accounts.clmm_config.clone()]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; ACCEPT_PROTOCOL_AUTHORITY_IX_ACCOUNTS_LEN]>
for AcceptProtocolAuthorityAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; ACCEPT_PROTOCOL_AUTHORITY_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            new_authority: &arr[0],
            clmm_config: &arr[1],
        }
    }
}
pub const ACCEPT_PROTOCOL_AUTHORITY_IX_DISCM: [u8; 8usize] = [
    237, 122, 6, 39, 53, 202, 141, 113,
];
#[derive(Clone, Debug, PartialEq)]
pub struct AcceptProtocolAuthorityIxData;
impl AcceptProtocolAuthorityIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != ACCEPT_PROTOCOL_AUTHORITY_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&ACCEPT_PROTOCOL_AUTHORITY_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn accept_protocol_authority_ix_with_program_id(
    program_id: Pubkey,
    keys: AcceptProtocolAuthorityKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; ACCEPT_PROTOCOL_AUTHORITY_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: AcceptProtocolAuthorityIxData.try_to_vec()?,
    })
}
pub fn accept_protocol_authority_ix(
    keys: AcceptProtocolAuthorityKeys,
) -> std::io::Result<Instruction> {
    accept_protocol_authority_ix_with_program_id(CREMA_PROGRAM_ID, keys)
}
pub fn accept_protocol_authority_invoke_with_program_id(
    program_id: Pubkey,
    accounts: AcceptProtocolAuthorityAccounts<'_, '_>,
) -> ProgramResult {
    let keys: AcceptProtocolAuthorityKeys = accounts.into();
    let ix = accept_protocol_authority_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn accept_protocol_authority_invoke(
    accounts: AcceptProtocolAuthorityAccounts<'_, '_>,
) -> ProgramResult {
    accept_protocol_authority_invoke_with_program_id(CREMA_PROGRAM_ID, accounts)
}
pub fn accept_protocol_authority_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: AcceptProtocolAuthorityAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: AcceptProtocolAuthorityKeys = accounts.into();
    let ix = accept_protocol_authority_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn accept_protocol_authority_invoke_signed(
    accounts: AcceptProtocolAuthorityAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    accept_protocol_authority_invoke_signed_with_program_id(
        CREMA_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn accept_protocol_authority_verify_account_keys(
    accounts: AcceptProtocolAuthorityAccounts<'_, '_>,
    keys: AcceptProtocolAuthorityKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.new_authority.key, keys.new_authority),
        (*accounts.clmm_config.key, keys.clmm_config),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn accept_protocol_authority_verify_writable_privileges<'me, 'info>(
    accounts: AcceptProtocolAuthorityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.clmm_config] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn accept_protocol_authority_verify_signer_privileges<'me, 'info>(
    accounts: AcceptProtocolAuthorityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.new_authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn accept_protocol_authority_verify_account_privileges<'me, 'info>(
    accounts: AcceptProtocolAuthorityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    accept_protocol_authority_verify_writable_privileges(accounts)?;
    accept_protocol_authority_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CREATE_CLMMPOOL_IX_ACCOUNTS_LEN: usize = 12;
#[derive(Copy, Clone, Debug)]
pub struct CreateClmmpoolAccounts<'me, 'info> {
    pub payer: &'me AccountInfo<'info>,
    pub clmm_config: &'me AccountInfo<'info>,
    pub fee_tier: &'me AccountInfo<'info>,
    pub clmmpool: &'me AccountInfo<'info>,
    pub token_a: &'me AccountInfo<'info>,
    pub token_b: &'me AccountInfo<'info>,
    pub token_a_vault: &'me AccountInfo<'info>,
    pub token_b_vault: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub rent: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CreateClmmpoolKeys {
    pub payer: Pubkey,
    pub clmm_config: Pubkey,
    pub fee_tier: Pubkey,
    pub clmmpool: Pubkey,
    pub token_a: Pubkey,
    pub token_b: Pubkey,
    pub token_a_vault: Pubkey,
    pub token_b_vault: Pubkey,
    pub token_program: Pubkey,
    pub associated_token_program: Pubkey,
    pub system_program: Pubkey,
    pub rent: Pubkey,
}
impl From<CreateClmmpoolAccounts<'_, '_>> for CreateClmmpoolKeys {
    fn from(accounts: CreateClmmpoolAccounts) -> Self {
        Self {
            payer: *accounts.payer.key,
            clmm_config: *accounts.clmm_config.key,
            fee_tier: *accounts.fee_tier.key,
            clmmpool: *accounts.clmmpool.key,
            token_a: *accounts.token_a.key,
            token_b: *accounts.token_b.key,
            token_a_vault: *accounts.token_a_vault.key,
            token_b_vault: *accounts.token_b_vault.key,
            token_program: *accounts.token_program.key,
            associated_token_program: *accounts.associated_token_program.key,
            system_program: *accounts.system_program.key,
            rent: *accounts.rent.key,
        }
    }
}
impl From<CreateClmmpoolKeys> for [AccountMeta; CREATE_CLMMPOOL_IX_ACCOUNTS_LEN] {
    fn from(keys: CreateClmmpoolKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.payer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.clmm_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.fee_tier,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.clmmpool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_a,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_b,
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
impl From<[Pubkey; CREATE_CLMMPOOL_IX_ACCOUNTS_LEN]> for CreateClmmpoolKeys {
    fn from(pubkeys: [Pubkey; CREATE_CLMMPOOL_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            payer: pubkeys[0],
            clmm_config: pubkeys[1],
            fee_tier: pubkeys[2],
            clmmpool: pubkeys[3],
            token_a: pubkeys[4],
            token_b: pubkeys[5],
            token_a_vault: pubkeys[6],
            token_b_vault: pubkeys[7],
            token_program: pubkeys[8],
            associated_token_program: pubkeys[9],
            system_program: pubkeys[10],
            rent: pubkeys[11],
        }
    }
}
impl<'info> From<CreateClmmpoolAccounts<'_, 'info>>
for [AccountInfo<'info>; CREATE_CLMMPOOL_IX_ACCOUNTS_LEN] {
    fn from(accounts: CreateClmmpoolAccounts<'_, 'info>) -> Self {
        [
            accounts.payer.clone(),
            accounts.clmm_config.clone(),
            accounts.fee_tier.clone(),
            accounts.clmmpool.clone(),
            accounts.token_a.clone(),
            accounts.token_b.clone(),
            accounts.token_a_vault.clone(),
            accounts.token_b_vault.clone(),
            accounts.token_program.clone(),
            accounts.associated_token_program.clone(),
            accounts.system_program.clone(),
            accounts.rent.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CREATE_CLMMPOOL_IX_ACCOUNTS_LEN]>
for CreateClmmpoolAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; CREATE_CLMMPOOL_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            payer: &arr[0],
            clmm_config: &arr[1],
            fee_tier: &arr[2],
            clmmpool: &arr[3],
            token_a: &arr[4],
            token_b: &arr[5],
            token_a_vault: &arr[6],
            token_b_vault: &arr[7],
            token_program: &arr[8],
            associated_token_program: &arr[9],
            system_program: &arr[10],
            rent: &arr[11],
        }
    }
}
pub const CREATE_CLMMPOOL_IX_DISCM: [u8; 8usize] = [
    95, 241, 101, 176, 11, 238, 210, 237,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CreateClmmpoolIxArgs {
    pub init_sqrt_price: u128,
}
#[derive(Clone, Debug, PartialEq)]
pub struct CreateClmmpoolIxData(pub CreateClmmpoolIxArgs);
impl From<CreateClmmpoolIxArgs> for CreateClmmpoolIxData {
    fn from(args: CreateClmmpoolIxArgs) -> Self {
        Self(args)
    }
}
impl CreateClmmpoolIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CREATE_CLMMPOOL_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let init_sqrt_price: u128 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(CreateClmmpoolIxArgs {
                init_sqrt_price,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CREATE_CLMMPOOL_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.init_sqrt_price, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn create_clmmpool_ix_with_program_id(
    program_id: Pubkey,
    keys: CreateClmmpoolKeys,
    args: CreateClmmpoolIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CREATE_CLMMPOOL_IX_ACCOUNTS_LEN] = keys.into();
    let data: CreateClmmpoolIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn create_clmmpool_ix(
    keys: CreateClmmpoolKeys,
    args: CreateClmmpoolIxArgs,
) -> std::io::Result<Instruction> {
    create_clmmpool_ix_with_program_id(CREMA_PROGRAM_ID, keys, args)
}
pub fn create_clmmpool_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CreateClmmpoolAccounts<'_, '_>,
    args: CreateClmmpoolIxArgs,
) -> ProgramResult {
    let keys: CreateClmmpoolKeys = accounts.into();
    let ix = create_clmmpool_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn create_clmmpool_invoke(
    accounts: CreateClmmpoolAccounts<'_, '_>,
    args: CreateClmmpoolIxArgs,
) -> ProgramResult {
    create_clmmpool_invoke_with_program_id(CREMA_PROGRAM_ID, accounts, args)
}
pub fn create_clmmpool_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CreateClmmpoolAccounts<'_, '_>,
    args: CreateClmmpoolIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CreateClmmpoolKeys = accounts.into();
    let ix = create_clmmpool_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn create_clmmpool_invoke_signed(
    accounts: CreateClmmpoolAccounts<'_, '_>,
    args: CreateClmmpoolIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    create_clmmpool_invoke_signed_with_program_id(
        CREMA_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn create_clmmpool_verify_account_keys(
    accounts: CreateClmmpoolAccounts<'_, '_>,
    keys: CreateClmmpoolKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.payer.key, keys.payer),
        (*accounts.clmm_config.key, keys.clmm_config),
        (*accounts.fee_tier.key, keys.fee_tier),
        (*accounts.clmmpool.key, keys.clmmpool),
        (*accounts.token_a.key, keys.token_a),
        (*accounts.token_b.key, keys.token_b),
        (*accounts.token_a_vault.key, keys.token_a_vault),
        (*accounts.token_b_vault.key, keys.token_b_vault),
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
pub fn create_clmmpool_verify_writable_privileges<'me, 'info>(
    accounts: CreateClmmpoolAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.payer,
        accounts.clmmpool,
        accounts.token_a_vault,
        accounts.token_b_vault,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn create_clmmpool_verify_signer_privileges<'me, 'info>(
    accounts: CreateClmmpoolAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.payer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn create_clmmpool_verify_account_privileges<'me, 'info>(
    accounts: CreateClmmpoolAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    create_clmmpool_verify_writable_privileges(accounts)?;
    create_clmmpool_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CREATE_TICK_ARRAY_IX_ACCOUNTS_LEN: usize = 5;
#[derive(Copy, Clone, Debug)]
pub struct CreateTickArrayAccounts<'me, 'info> {
    pub payer: &'me AccountInfo<'info>,
    pub clmmpool: &'me AccountInfo<'info>,
    pub tick_array: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub rent: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CreateTickArrayKeys {
    pub payer: Pubkey,
    pub clmmpool: Pubkey,
    pub tick_array: Pubkey,
    pub system_program: Pubkey,
    pub rent: Pubkey,
}
impl From<CreateTickArrayAccounts<'_, '_>> for CreateTickArrayKeys {
    fn from(accounts: CreateTickArrayAccounts) -> Self {
        Self {
            payer: *accounts.payer.key,
            clmmpool: *accounts.clmmpool.key,
            tick_array: *accounts.tick_array.key,
            system_program: *accounts.system_program.key,
            rent: *accounts.rent.key,
        }
    }
}
impl From<CreateTickArrayKeys> for [AccountMeta; CREATE_TICK_ARRAY_IX_ACCOUNTS_LEN] {
    fn from(keys: CreateTickArrayKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.payer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.clmmpool,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.tick_array,
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
impl From<[Pubkey; CREATE_TICK_ARRAY_IX_ACCOUNTS_LEN]> for CreateTickArrayKeys {
    fn from(pubkeys: [Pubkey; CREATE_TICK_ARRAY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            payer: pubkeys[0],
            clmmpool: pubkeys[1],
            tick_array: pubkeys[2],
            system_program: pubkeys[3],
            rent: pubkeys[4],
        }
    }
}
impl<'info> From<CreateTickArrayAccounts<'_, 'info>>
for [AccountInfo<'info>; CREATE_TICK_ARRAY_IX_ACCOUNTS_LEN] {
    fn from(accounts: CreateTickArrayAccounts<'_, 'info>) -> Self {
        [
            accounts.payer.clone(),
            accounts.clmmpool.clone(),
            accounts.tick_array.clone(),
            accounts.system_program.clone(),
            accounts.rent.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CREATE_TICK_ARRAY_IX_ACCOUNTS_LEN]>
for CreateTickArrayAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; CREATE_TICK_ARRAY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            payer: &arr[0],
            clmmpool: &arr[1],
            tick_array: &arr[2],
            system_program: &arr[3],
            rent: &arr[4],
        }
    }
}
pub const CREATE_TICK_ARRAY_IX_DISCM: [u8; 8usize] = [253, 248, 163, 171, 253, 8, 8, 81];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CreateTickArrayIxArgs {
    pub array_index: u16,
}
#[derive(Clone, Debug, PartialEq)]
pub struct CreateTickArrayIxData(pub CreateTickArrayIxArgs);
impl From<CreateTickArrayIxArgs> for CreateTickArrayIxData {
    fn from(args: CreateTickArrayIxArgs) -> Self {
        Self(args)
    }
}
impl CreateTickArrayIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CREATE_TICK_ARRAY_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let array_index: u16 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(CreateTickArrayIxArgs {
                array_index,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CREATE_TICK_ARRAY_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.array_index, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn create_tick_array_ix_with_program_id(
    program_id: Pubkey,
    keys: CreateTickArrayKeys,
    args: CreateTickArrayIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CREATE_TICK_ARRAY_IX_ACCOUNTS_LEN] = keys.into();
    let data: CreateTickArrayIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn create_tick_array_ix(
    keys: CreateTickArrayKeys,
    args: CreateTickArrayIxArgs,
) -> std::io::Result<Instruction> {
    create_tick_array_ix_with_program_id(CREMA_PROGRAM_ID, keys, args)
}
pub fn create_tick_array_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CreateTickArrayAccounts<'_, '_>,
    args: CreateTickArrayIxArgs,
) -> ProgramResult {
    let keys: CreateTickArrayKeys = accounts.into();
    let ix = create_tick_array_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn create_tick_array_invoke(
    accounts: CreateTickArrayAccounts<'_, '_>,
    args: CreateTickArrayIxArgs,
) -> ProgramResult {
    create_tick_array_invoke_with_program_id(CREMA_PROGRAM_ID, accounts, args)
}
pub fn create_tick_array_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CreateTickArrayAccounts<'_, '_>,
    args: CreateTickArrayIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CreateTickArrayKeys = accounts.into();
    let ix = create_tick_array_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn create_tick_array_invoke_signed(
    accounts: CreateTickArrayAccounts<'_, '_>,
    args: CreateTickArrayIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    create_tick_array_invoke_signed_with_program_id(
        CREMA_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn create_tick_array_verify_account_keys(
    accounts: CreateTickArrayAccounts<'_, '_>,
    keys: CreateTickArrayKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.payer.key, keys.payer),
        (*accounts.clmmpool.key, keys.clmmpool),
        (*accounts.tick_array.key, keys.tick_array),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.rent.key, keys.rent),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn create_tick_array_verify_writable_privileges<'me, 'info>(
    accounts: CreateTickArrayAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.payer, accounts.tick_array] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn create_tick_array_verify_signer_privileges<'me, 'info>(
    accounts: CreateTickArrayAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.payer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn create_tick_array_verify_account_privileges<'me, 'info>(
    accounts: CreateTickArrayAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    create_tick_array_verify_writable_privileges(accounts)?;
    create_tick_array_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CREATE_TICK_ARRAY_MAP_IX_ACCOUNTS_LEN: usize = 5;
#[derive(Copy, Clone, Debug)]
pub struct CreateTickArrayMapAccounts<'me, 'info> {
    pub payer: &'me AccountInfo<'info>,
    pub clmmpool: &'me AccountInfo<'info>,
    pub tick_array_map: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub rent: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CreateTickArrayMapKeys {
    pub payer: Pubkey,
    pub clmmpool: Pubkey,
    pub tick_array_map: Pubkey,
    pub system_program: Pubkey,
    pub rent: Pubkey,
}
impl From<CreateTickArrayMapAccounts<'_, '_>> for CreateTickArrayMapKeys {
    fn from(accounts: CreateTickArrayMapAccounts) -> Self {
        Self {
            payer: *accounts.payer.key,
            clmmpool: *accounts.clmmpool.key,
            tick_array_map: *accounts.tick_array_map.key,
            system_program: *accounts.system_program.key,
            rent: *accounts.rent.key,
        }
    }
}
impl From<CreateTickArrayMapKeys>
for [AccountMeta; CREATE_TICK_ARRAY_MAP_IX_ACCOUNTS_LEN] {
    fn from(keys: CreateTickArrayMapKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.payer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.clmmpool,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.tick_array_map,
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
impl From<[Pubkey; CREATE_TICK_ARRAY_MAP_IX_ACCOUNTS_LEN]> for CreateTickArrayMapKeys {
    fn from(pubkeys: [Pubkey; CREATE_TICK_ARRAY_MAP_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            payer: pubkeys[0],
            clmmpool: pubkeys[1],
            tick_array_map: pubkeys[2],
            system_program: pubkeys[3],
            rent: pubkeys[4],
        }
    }
}
impl<'info> From<CreateTickArrayMapAccounts<'_, 'info>>
for [AccountInfo<'info>; CREATE_TICK_ARRAY_MAP_IX_ACCOUNTS_LEN] {
    fn from(accounts: CreateTickArrayMapAccounts<'_, 'info>) -> Self {
        [
            accounts.payer.clone(),
            accounts.clmmpool.clone(),
            accounts.tick_array_map.clone(),
            accounts.system_program.clone(),
            accounts.rent.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CREATE_TICK_ARRAY_MAP_IX_ACCOUNTS_LEN]>
for CreateTickArrayMapAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; CREATE_TICK_ARRAY_MAP_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            payer: &arr[0],
            clmmpool: &arr[1],
            tick_array_map: &arr[2],
            system_program: &arr[3],
            rent: &arr[4],
        }
    }
}
pub const CREATE_TICK_ARRAY_MAP_IX_DISCM: [u8; 8usize] = [
    224, 106, 163, 173, 9, 151, 198, 70,
];
#[derive(Clone, Debug, PartialEq)]
pub struct CreateTickArrayMapIxData;
impl CreateTickArrayMapIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CREATE_TICK_ARRAY_MAP_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CREATE_TICK_ARRAY_MAP_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn create_tick_array_map_ix_with_program_id(
    program_id: Pubkey,
    keys: CreateTickArrayMapKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CREATE_TICK_ARRAY_MAP_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: CreateTickArrayMapIxData.try_to_vec()?,
    })
}
pub fn create_tick_array_map_ix(
    keys: CreateTickArrayMapKeys,
) -> std::io::Result<Instruction> {
    create_tick_array_map_ix_with_program_id(CREMA_PROGRAM_ID, keys)
}
pub fn create_tick_array_map_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CreateTickArrayMapAccounts<'_, '_>,
) -> ProgramResult {
    let keys: CreateTickArrayMapKeys = accounts.into();
    let ix = create_tick_array_map_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn create_tick_array_map_invoke(
    accounts: CreateTickArrayMapAccounts<'_, '_>,
) -> ProgramResult {
    create_tick_array_map_invoke_with_program_id(CREMA_PROGRAM_ID, accounts)
}
pub fn create_tick_array_map_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CreateTickArrayMapAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CreateTickArrayMapKeys = accounts.into();
    let ix = create_tick_array_map_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn create_tick_array_map_invoke_signed(
    accounts: CreateTickArrayMapAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    create_tick_array_map_invoke_signed_with_program_id(
        CREMA_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn create_tick_array_map_verify_account_keys(
    accounts: CreateTickArrayMapAccounts<'_, '_>,
    keys: CreateTickArrayMapKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.payer.key, keys.payer),
        (*accounts.clmmpool.key, keys.clmmpool),
        (*accounts.tick_array_map.key, keys.tick_array_map),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.rent.key, keys.rent),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn create_tick_array_map_verify_writable_privileges<'me, 'info>(
    accounts: CreateTickArrayMapAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.payer, accounts.tick_array_map] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn create_tick_array_map_verify_signer_privileges<'me, 'info>(
    accounts: CreateTickArrayMapAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.payer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn create_tick_array_map_verify_account_privileges<'me, 'info>(
    accounts: CreateTickArrayMapAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    create_tick_array_map_verify_writable_privileges(accounts)?;
    create_tick_array_map_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const OPEN_POSITION_IX_ACCOUNTS_LEN: usize = 12;
#[derive(Copy, Clone, Debug)]
pub struct OpenPositionAccounts<'me, 'info> {
    pub owner: &'me AccountInfo<'info>,
    pub clmmpool: &'me AccountInfo<'info>,
    pub position: &'me AccountInfo<'info>,
    pub position_nft_mint: &'me AccountInfo<'info>,
    pub position_metadata_account: &'me AccountInfo<'info>,
    pub position_edition: &'me AccountInfo<'info>,
    pub position_ata: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
    pub metadata_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub rent: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct OpenPositionKeys {
    pub owner: Pubkey,
    pub clmmpool: Pubkey,
    pub position: Pubkey,
    pub position_nft_mint: Pubkey,
    pub position_metadata_account: Pubkey,
    pub position_edition: Pubkey,
    pub position_ata: Pubkey,
    pub token_program: Pubkey,
    pub associated_token_program: Pubkey,
    pub metadata_program: Pubkey,
    pub system_program: Pubkey,
    pub rent: Pubkey,
}
impl From<OpenPositionAccounts<'_, '_>> for OpenPositionKeys {
    fn from(accounts: OpenPositionAccounts) -> Self {
        Self {
            owner: *accounts.owner.key,
            clmmpool: *accounts.clmmpool.key,
            position: *accounts.position.key,
            position_nft_mint: *accounts.position_nft_mint.key,
            position_metadata_account: *accounts.position_metadata_account.key,
            position_edition: *accounts.position_edition.key,
            position_ata: *accounts.position_ata.key,
            token_program: *accounts.token_program.key,
            associated_token_program: *accounts.associated_token_program.key,
            metadata_program: *accounts.metadata_program.key,
            system_program: *accounts.system_program.key,
            rent: *accounts.rent.key,
        }
    }
}
impl From<OpenPositionKeys> for [AccountMeta; OPEN_POSITION_IX_ACCOUNTS_LEN] {
    fn from(keys: OpenPositionKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.owner,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.clmmpool,
                is_signer: false,
                is_writable: false,
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
                pubkey: keys.position_metadata_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.position_edition,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.position_ata,
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
                pubkey: keys.metadata_program,
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
impl From<[Pubkey; OPEN_POSITION_IX_ACCOUNTS_LEN]> for OpenPositionKeys {
    fn from(pubkeys: [Pubkey; OPEN_POSITION_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            owner: pubkeys[0],
            clmmpool: pubkeys[1],
            position: pubkeys[2],
            position_nft_mint: pubkeys[3],
            position_metadata_account: pubkeys[4],
            position_edition: pubkeys[5],
            position_ata: pubkeys[6],
            token_program: pubkeys[7],
            associated_token_program: pubkeys[8],
            metadata_program: pubkeys[9],
            system_program: pubkeys[10],
            rent: pubkeys[11],
        }
    }
}
impl<'info> From<OpenPositionAccounts<'_, 'info>>
for [AccountInfo<'info>; OPEN_POSITION_IX_ACCOUNTS_LEN] {
    fn from(accounts: OpenPositionAccounts<'_, 'info>) -> Self {
        [
            accounts.owner.clone(),
            accounts.clmmpool.clone(),
            accounts.position.clone(),
            accounts.position_nft_mint.clone(),
            accounts.position_metadata_account.clone(),
            accounts.position_edition.clone(),
            accounts.position_ata.clone(),
            accounts.token_program.clone(),
            accounts.associated_token_program.clone(),
            accounts.metadata_program.clone(),
            accounts.system_program.clone(),
            accounts.rent.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; OPEN_POSITION_IX_ACCOUNTS_LEN]>
for OpenPositionAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; OPEN_POSITION_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            owner: &arr[0],
            clmmpool: &arr[1],
            position: &arr[2],
            position_nft_mint: &arr[3],
            position_metadata_account: &arr[4],
            position_edition: &arr[5],
            position_ata: &arr[6],
            token_program: &arr[7],
            associated_token_program: &arr[8],
            metadata_program: &arr[9],
            system_program: &arr[10],
            rent: &arr[11],
        }
    }
}
pub const OPEN_POSITION_IX_DISCM: [u8; 8usize] = [135, 128, 47, 77, 15, 152, 240, 49];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct OpenPositionIxArgs {
    pub tick_lower_index: i32,
    pub tick_upper_index: i32,
}
#[derive(Clone, Debug, PartialEq)]
pub struct OpenPositionIxData(pub OpenPositionIxArgs);
impl From<OpenPositionIxArgs> for OpenPositionIxData {
    fn from(args: OpenPositionIxArgs) -> Self {
        Self(args)
    }
}
impl OpenPositionIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != OPEN_POSITION_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let tick_lower_index: i32 = crate::borsh_de_or_default(&mut reader)?;
        let tick_upper_index: i32 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(OpenPositionIxArgs {
                tick_lower_index,
                tick_upper_index,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&OPEN_POSITION_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.tick_lower_index, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.tick_upper_index, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn open_position_ix_with_program_id(
    program_id: Pubkey,
    keys: OpenPositionKeys,
    args: OpenPositionIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; OPEN_POSITION_IX_ACCOUNTS_LEN] = keys.into();
    let data: OpenPositionIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn open_position_ix(
    keys: OpenPositionKeys,
    args: OpenPositionIxArgs,
) -> std::io::Result<Instruction> {
    open_position_ix_with_program_id(CREMA_PROGRAM_ID, keys, args)
}
pub fn open_position_invoke_with_program_id(
    program_id: Pubkey,
    accounts: OpenPositionAccounts<'_, '_>,
    args: OpenPositionIxArgs,
) -> ProgramResult {
    let keys: OpenPositionKeys = accounts.into();
    let ix = open_position_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn open_position_invoke(
    accounts: OpenPositionAccounts<'_, '_>,
    args: OpenPositionIxArgs,
) -> ProgramResult {
    open_position_invoke_with_program_id(CREMA_PROGRAM_ID, accounts, args)
}
pub fn open_position_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: OpenPositionAccounts<'_, '_>,
    args: OpenPositionIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: OpenPositionKeys = accounts.into();
    let ix = open_position_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn open_position_invoke_signed(
    accounts: OpenPositionAccounts<'_, '_>,
    args: OpenPositionIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    open_position_invoke_signed_with_program_id(CREMA_PROGRAM_ID, accounts, args, seeds)
}
pub fn open_position_verify_account_keys(
    accounts: OpenPositionAccounts<'_, '_>,
    keys: OpenPositionKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.owner.key, keys.owner),
        (*accounts.clmmpool.key, keys.clmmpool),
        (*accounts.position.key, keys.position),
        (*accounts.position_nft_mint.key, keys.position_nft_mint),
        (*accounts.position_metadata_account.key, keys.position_metadata_account),
        (*accounts.position_edition.key, keys.position_edition),
        (*accounts.position_ata.key, keys.position_ata),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.associated_token_program.key, keys.associated_token_program),
        (*accounts.metadata_program.key, keys.metadata_program),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.rent.key, keys.rent),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn open_position_verify_writable_privileges<'me, 'info>(
    accounts: OpenPositionAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.owner,
        accounts.position,
        accounts.position_nft_mint,
        accounts.position_metadata_account,
        accounts.position_edition,
        accounts.position_ata,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn open_position_verify_signer_privileges<'me, 'info>(
    accounts: OpenPositionAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.owner, accounts.position_nft_mint] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn open_position_verify_account_privileges<'me, 'info>(
    accounts: OpenPositionAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    open_position_verify_writable_privileges(accounts)?;
    open_position_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const REMOVE_POSITION_IX_ACCOUNTS_LEN: usize = 8;
#[derive(Copy, Clone, Debug)]
pub struct RemovePositionAccounts<'me, 'info> {
    pub owner: &'me AccountInfo<'info>,
    pub position: &'me AccountInfo<'info>,
    pub position_nft_mint: &'me AccountInfo<'info>,
    pub position_ata: &'me AccountInfo<'info>,
    pub position_metadata_account: &'me AccountInfo<'info>,
    pub position_edition: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub metadata_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct RemovePositionKeys {
    pub owner: Pubkey,
    pub position: Pubkey,
    pub position_nft_mint: Pubkey,
    pub position_ata: Pubkey,
    pub position_metadata_account: Pubkey,
    pub position_edition: Pubkey,
    pub token_program: Pubkey,
    pub metadata_program: Pubkey,
}
impl From<RemovePositionAccounts<'_, '_>> for RemovePositionKeys {
    fn from(accounts: RemovePositionAccounts) -> Self {
        Self {
            owner: *accounts.owner.key,
            position: *accounts.position.key,
            position_nft_mint: *accounts.position_nft_mint.key,
            position_ata: *accounts.position_ata.key,
            position_metadata_account: *accounts.position_metadata_account.key,
            position_edition: *accounts.position_edition.key,
            token_program: *accounts.token_program.key,
            metadata_program: *accounts.metadata_program.key,
        }
    }
}
impl From<RemovePositionKeys> for [AccountMeta; REMOVE_POSITION_IX_ACCOUNTS_LEN] {
    fn from(keys: RemovePositionKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.owner,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.position,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.position_nft_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.position_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.position_metadata_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.position_edition,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_program,
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
impl From<[Pubkey; REMOVE_POSITION_IX_ACCOUNTS_LEN]> for RemovePositionKeys {
    fn from(pubkeys: [Pubkey; REMOVE_POSITION_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            owner: pubkeys[0],
            position: pubkeys[1],
            position_nft_mint: pubkeys[2],
            position_ata: pubkeys[3],
            position_metadata_account: pubkeys[4],
            position_edition: pubkeys[5],
            token_program: pubkeys[6],
            metadata_program: pubkeys[7],
        }
    }
}
impl<'info> From<RemovePositionAccounts<'_, 'info>>
for [AccountInfo<'info>; REMOVE_POSITION_IX_ACCOUNTS_LEN] {
    fn from(accounts: RemovePositionAccounts<'_, 'info>) -> Self {
        [
            accounts.owner.clone(),
            accounts.position.clone(),
            accounts.position_nft_mint.clone(),
            accounts.position_ata.clone(),
            accounts.position_metadata_account.clone(),
            accounts.position_edition.clone(),
            accounts.token_program.clone(),
            accounts.metadata_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; REMOVE_POSITION_IX_ACCOUNTS_LEN]>
for RemovePositionAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; REMOVE_POSITION_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            owner: &arr[0],
            position: &arr[1],
            position_nft_mint: &arr[2],
            position_ata: &arr[3],
            position_metadata_account: &arr[4],
            position_edition: &arr[5],
            token_program: &arr[6],
            metadata_program: &arr[7],
        }
    }
}
pub const REMOVE_POSITION_IX_DISCM: [u8; 8usize] = [219, 24, 236, 110, 138, 80, 129, 6];
#[derive(Clone, Debug, PartialEq)]
pub struct RemovePositionIxData;
impl RemovePositionIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != REMOVE_POSITION_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&REMOVE_POSITION_IX_DISCM)
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
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; REMOVE_POSITION_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: RemovePositionIxData.try_to_vec()?,
    })
}
pub fn remove_position_ix(keys: RemovePositionKeys) -> std::io::Result<Instruction> {
    remove_position_ix_with_program_id(CREMA_PROGRAM_ID, keys)
}
pub fn remove_position_invoke_with_program_id(
    program_id: Pubkey,
    accounts: RemovePositionAccounts<'_, '_>,
) -> ProgramResult {
    let keys: RemovePositionKeys = accounts.into();
    let ix = remove_position_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn remove_position_invoke(
    accounts: RemovePositionAccounts<'_, '_>,
) -> ProgramResult {
    remove_position_invoke_with_program_id(CREMA_PROGRAM_ID, accounts)
}
pub fn remove_position_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: RemovePositionAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: RemovePositionKeys = accounts.into();
    let ix = remove_position_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn remove_position_invoke_signed(
    accounts: RemovePositionAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    remove_position_invoke_signed_with_program_id(CREMA_PROGRAM_ID, accounts, seeds)
}
pub fn remove_position_verify_account_keys(
    accounts: RemovePositionAccounts<'_, '_>,
    keys: RemovePositionKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.owner.key, keys.owner),
        (*accounts.position.key, keys.position),
        (*accounts.position_nft_mint.key, keys.position_nft_mint),
        (*accounts.position_ata.key, keys.position_ata),
        (*accounts.position_metadata_account.key, keys.position_metadata_account),
        (*accounts.position_edition.key, keys.position_edition),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.metadata_program.key, keys.metadata_program),
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
        accounts.owner,
        accounts.position,
        accounts.position_nft_mint,
        accounts.position_ata,
        accounts.position_metadata_account,
        accounts.position_edition,
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
pub const INCREASE_LIQUIDITY_IX_ACCOUNTS_LEN: usize = 12;
#[derive(Copy, Clone, Debug)]
pub struct IncreaseLiquidityAccounts<'me, 'info> {
    pub owner: &'me AccountInfo<'info>,
    pub clmmpool: &'me AccountInfo<'info>,
    pub position: &'me AccountInfo<'info>,
    pub position_ata: &'me AccountInfo<'info>,
    pub token_a_ata: &'me AccountInfo<'info>,
    pub token_b_ata: &'me AccountInfo<'info>,
    pub token_a_vault: &'me AccountInfo<'info>,
    pub token_b_vault: &'me AccountInfo<'info>,
    pub tick_array_lower: &'me AccountInfo<'info>,
    pub tick_array_upper: &'me AccountInfo<'info>,
    pub tick_array_map: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct IncreaseLiquidityKeys {
    pub owner: Pubkey,
    pub clmmpool: Pubkey,
    pub position: Pubkey,
    pub position_ata: Pubkey,
    pub token_a_ata: Pubkey,
    pub token_b_ata: Pubkey,
    pub token_a_vault: Pubkey,
    pub token_b_vault: Pubkey,
    pub tick_array_lower: Pubkey,
    pub tick_array_upper: Pubkey,
    pub tick_array_map: Pubkey,
    pub token_program: Pubkey,
}
impl From<IncreaseLiquidityAccounts<'_, '_>> for IncreaseLiquidityKeys {
    fn from(accounts: IncreaseLiquidityAccounts) -> Self {
        Self {
            owner: *accounts.owner.key,
            clmmpool: *accounts.clmmpool.key,
            position: *accounts.position.key,
            position_ata: *accounts.position_ata.key,
            token_a_ata: *accounts.token_a_ata.key,
            token_b_ata: *accounts.token_b_ata.key,
            token_a_vault: *accounts.token_a_vault.key,
            token_b_vault: *accounts.token_b_vault.key,
            tick_array_lower: *accounts.tick_array_lower.key,
            tick_array_upper: *accounts.tick_array_upper.key,
            tick_array_map: *accounts.tick_array_map.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<IncreaseLiquidityKeys> for [AccountMeta; INCREASE_LIQUIDITY_IX_ACCOUNTS_LEN] {
    fn from(keys: IncreaseLiquidityKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.owner,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.clmmpool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.position,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.position_ata,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_a_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_b_ata,
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
                pubkey: keys.tick_array_map,
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
impl From<[Pubkey; INCREASE_LIQUIDITY_IX_ACCOUNTS_LEN]> for IncreaseLiquidityKeys {
    fn from(pubkeys: [Pubkey; INCREASE_LIQUIDITY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            owner: pubkeys[0],
            clmmpool: pubkeys[1],
            position: pubkeys[2],
            position_ata: pubkeys[3],
            token_a_ata: pubkeys[4],
            token_b_ata: pubkeys[5],
            token_a_vault: pubkeys[6],
            token_b_vault: pubkeys[7],
            tick_array_lower: pubkeys[8],
            tick_array_upper: pubkeys[9],
            tick_array_map: pubkeys[10],
            token_program: pubkeys[11],
        }
    }
}
impl<'info> From<IncreaseLiquidityAccounts<'_, 'info>>
for [AccountInfo<'info>; INCREASE_LIQUIDITY_IX_ACCOUNTS_LEN] {
    fn from(accounts: IncreaseLiquidityAccounts<'_, 'info>) -> Self {
        [
            accounts.owner.clone(),
            accounts.clmmpool.clone(),
            accounts.position.clone(),
            accounts.position_ata.clone(),
            accounts.token_a_ata.clone(),
            accounts.token_b_ata.clone(),
            accounts.token_a_vault.clone(),
            accounts.token_b_vault.clone(),
            accounts.tick_array_lower.clone(),
            accounts.tick_array_upper.clone(),
            accounts.tick_array_map.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; INCREASE_LIQUIDITY_IX_ACCOUNTS_LEN]>
for IncreaseLiquidityAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; INCREASE_LIQUIDITY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            owner: &arr[0],
            clmmpool: &arr[1],
            position: &arr[2],
            position_ata: &arr[3],
            token_a_ata: &arr[4],
            token_b_ata: &arr[5],
            token_a_vault: &arr[6],
            token_b_vault: &arr[7],
            tick_array_lower: &arr[8],
            tick_array_upper: &arr[9],
            tick_array_map: &arr[10],
            token_program: &arr[11],
        }
    }
}
pub const INCREASE_LIQUIDITY_IX_DISCM: [u8; 8usize] = [
    46, 156, 243, 118, 13, 205, 251, 178,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct IncreaseLiquidityIxArgs {
    pub delta_liquidity: u128,
    pub token_a_max: u64,
    pub token_b_max: u64,
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
        let delta_liquidity: u128 = crate::borsh_de_or_default(&mut reader)?;
        let token_a_max: u64 = crate::borsh_de_or_default(&mut reader)?;
        let token_b_max: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(IncreaseLiquidityIxArgs {
                delta_liquidity,
                token_a_max,
                token_b_max,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INCREASE_LIQUIDITY_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.delta_liquidity, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.token_a_max, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.token_b_max, &mut writer)?;
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
    increase_liquidity_ix_with_program_id(CREMA_PROGRAM_ID, keys, args)
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
    increase_liquidity_invoke_with_program_id(CREMA_PROGRAM_ID, accounts, args)
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
        CREMA_PROGRAM_ID,
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
        (*accounts.owner.key, keys.owner),
        (*accounts.clmmpool.key, keys.clmmpool),
        (*accounts.position.key, keys.position),
        (*accounts.position_ata.key, keys.position_ata),
        (*accounts.token_a_ata.key, keys.token_a_ata),
        (*accounts.token_b_ata.key, keys.token_b_ata),
        (*accounts.token_a_vault.key, keys.token_a_vault),
        (*accounts.token_b_vault.key, keys.token_b_vault),
        (*accounts.tick_array_lower.key, keys.tick_array_lower),
        (*accounts.tick_array_upper.key, keys.tick_array_upper),
        (*accounts.tick_array_map.key, keys.tick_array_map),
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
        accounts.owner,
        accounts.clmmpool,
        accounts.position,
        accounts.token_a_ata,
        accounts.token_b_ata,
        accounts.token_a_vault,
        accounts.token_b_vault,
        accounts.tick_array_lower,
        accounts.tick_array_upper,
        accounts.tick_array_map,
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
    for should_be_signer in [accounts.owner] {
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
pub const INCREASE_LIQUIDITY_WITH_FIXED_TOKEN_IX_ACCOUNTS_LEN: usize = 12;
#[derive(Copy, Clone, Debug)]
pub struct IncreaseLiquidityWithFixedTokenAccounts<'me, 'info> {
    pub owner: &'me AccountInfo<'info>,
    pub clmmpool: &'me AccountInfo<'info>,
    pub position: &'me AccountInfo<'info>,
    pub position_ata: &'me AccountInfo<'info>,
    pub token_a_ata: &'me AccountInfo<'info>,
    pub token_b_ata: &'me AccountInfo<'info>,
    pub token_a_vault: &'me AccountInfo<'info>,
    pub token_b_vault: &'me AccountInfo<'info>,
    pub tick_array_lower: &'me AccountInfo<'info>,
    pub tick_array_upper: &'me AccountInfo<'info>,
    pub tick_array_map: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct IncreaseLiquidityWithFixedTokenKeys {
    pub owner: Pubkey,
    pub clmmpool: Pubkey,
    pub position: Pubkey,
    pub position_ata: Pubkey,
    pub token_a_ata: Pubkey,
    pub token_b_ata: Pubkey,
    pub token_a_vault: Pubkey,
    pub token_b_vault: Pubkey,
    pub tick_array_lower: Pubkey,
    pub tick_array_upper: Pubkey,
    pub tick_array_map: Pubkey,
    pub token_program: Pubkey,
}
impl From<IncreaseLiquidityWithFixedTokenAccounts<'_, '_>>
for IncreaseLiquidityWithFixedTokenKeys {
    fn from(accounts: IncreaseLiquidityWithFixedTokenAccounts) -> Self {
        Self {
            owner: *accounts.owner.key,
            clmmpool: *accounts.clmmpool.key,
            position: *accounts.position.key,
            position_ata: *accounts.position_ata.key,
            token_a_ata: *accounts.token_a_ata.key,
            token_b_ata: *accounts.token_b_ata.key,
            token_a_vault: *accounts.token_a_vault.key,
            token_b_vault: *accounts.token_b_vault.key,
            tick_array_lower: *accounts.tick_array_lower.key,
            tick_array_upper: *accounts.tick_array_upper.key,
            tick_array_map: *accounts.tick_array_map.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<IncreaseLiquidityWithFixedTokenKeys>
for [AccountMeta; INCREASE_LIQUIDITY_WITH_FIXED_TOKEN_IX_ACCOUNTS_LEN] {
    fn from(keys: IncreaseLiquidityWithFixedTokenKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.owner,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.clmmpool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.position,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.position_ata,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_a_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_b_ata,
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
                pubkey: keys.tick_array_map,
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
impl From<[Pubkey; INCREASE_LIQUIDITY_WITH_FIXED_TOKEN_IX_ACCOUNTS_LEN]>
for IncreaseLiquidityWithFixedTokenKeys {
    fn from(
        pubkeys: [Pubkey; INCREASE_LIQUIDITY_WITH_FIXED_TOKEN_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            owner: pubkeys[0],
            clmmpool: pubkeys[1],
            position: pubkeys[2],
            position_ata: pubkeys[3],
            token_a_ata: pubkeys[4],
            token_b_ata: pubkeys[5],
            token_a_vault: pubkeys[6],
            token_b_vault: pubkeys[7],
            tick_array_lower: pubkeys[8],
            tick_array_upper: pubkeys[9],
            tick_array_map: pubkeys[10],
            token_program: pubkeys[11],
        }
    }
}
impl<'info> From<IncreaseLiquidityWithFixedTokenAccounts<'_, 'info>>
for [AccountInfo<'info>; INCREASE_LIQUIDITY_WITH_FIXED_TOKEN_IX_ACCOUNTS_LEN] {
    fn from(accounts: IncreaseLiquidityWithFixedTokenAccounts<'_, 'info>) -> Self {
        [
            accounts.owner.clone(),
            accounts.clmmpool.clone(),
            accounts.position.clone(),
            accounts.position_ata.clone(),
            accounts.token_a_ata.clone(),
            accounts.token_b_ata.clone(),
            accounts.token_a_vault.clone(),
            accounts.token_b_vault.clone(),
            accounts.tick_array_lower.clone(),
            accounts.tick_array_upper.clone(),
            accounts.tick_array_map.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; INCREASE_LIQUIDITY_WITH_FIXED_TOKEN_IX_ACCOUNTS_LEN]>
for IncreaseLiquidityWithFixedTokenAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<
            'info,
        >; INCREASE_LIQUIDITY_WITH_FIXED_TOKEN_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            owner: &arr[0],
            clmmpool: &arr[1],
            position: &arr[2],
            position_ata: &arr[3],
            token_a_ata: &arr[4],
            token_b_ata: &arr[5],
            token_a_vault: &arr[6],
            token_b_vault: &arr[7],
            tick_array_lower: &arr[8],
            tick_array_upper: &arr[9],
            tick_array_map: &arr[10],
            token_program: &arr[11],
        }
    }
}
pub const INCREASE_LIQUIDITY_WITH_FIXED_TOKEN_IX_DISCM: [u8; 8usize] = [
    174, 15, 121, 101, 108, 2, 174, 159,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct IncreaseLiquidityWithFixedTokenIxArgs {
    pub token_a: u64,
    pub token_b: u64,
    pub is_a_fixed: bool,
}
#[derive(Clone, Debug, PartialEq)]
pub struct IncreaseLiquidityWithFixedTokenIxData(
    pub IncreaseLiquidityWithFixedTokenIxArgs,
);
impl From<IncreaseLiquidityWithFixedTokenIxArgs>
for IncreaseLiquidityWithFixedTokenIxData {
    fn from(args: IncreaseLiquidityWithFixedTokenIxArgs) -> Self {
        Self(args)
    }
}
impl IncreaseLiquidityWithFixedTokenIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INCREASE_LIQUIDITY_WITH_FIXED_TOKEN_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let token_a: u64 = crate::borsh_de_or_default(&mut reader)?;
        let token_b: u64 = crate::borsh_de_or_default(&mut reader)?;
        let is_a_fixed: bool = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(IncreaseLiquidityWithFixedTokenIxArgs {
                token_a,
                token_b,
                is_a_fixed,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INCREASE_LIQUIDITY_WITH_FIXED_TOKEN_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.token_a, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.token_b, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.is_a_fixed, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn increase_liquidity_with_fixed_token_ix_with_program_id(
    program_id: Pubkey,
    keys: IncreaseLiquidityWithFixedTokenKeys,
    args: IncreaseLiquidityWithFixedTokenIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; INCREASE_LIQUIDITY_WITH_FIXED_TOKEN_IX_ACCOUNTS_LEN] = keys
        .into();
    let data: IncreaseLiquidityWithFixedTokenIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn increase_liquidity_with_fixed_token_ix(
    keys: IncreaseLiquidityWithFixedTokenKeys,
    args: IncreaseLiquidityWithFixedTokenIxArgs,
) -> std::io::Result<Instruction> {
    increase_liquidity_with_fixed_token_ix_with_program_id(CREMA_PROGRAM_ID, keys, args)
}
pub fn increase_liquidity_with_fixed_token_invoke_with_program_id(
    program_id: Pubkey,
    accounts: IncreaseLiquidityWithFixedTokenAccounts<'_, '_>,
    args: IncreaseLiquidityWithFixedTokenIxArgs,
) -> ProgramResult {
    let keys: IncreaseLiquidityWithFixedTokenKeys = accounts.into();
    let ix = increase_liquidity_with_fixed_token_ix_with_program_id(
        program_id,
        keys,
        args,
    )?;
    invoke_instruction(&ix, accounts)
}
pub fn increase_liquidity_with_fixed_token_invoke(
    accounts: IncreaseLiquidityWithFixedTokenAccounts<'_, '_>,
    args: IncreaseLiquidityWithFixedTokenIxArgs,
) -> ProgramResult {
    increase_liquidity_with_fixed_token_invoke_with_program_id(
        CREMA_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn increase_liquidity_with_fixed_token_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: IncreaseLiquidityWithFixedTokenAccounts<'_, '_>,
    args: IncreaseLiquidityWithFixedTokenIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: IncreaseLiquidityWithFixedTokenKeys = accounts.into();
    let ix = increase_liquidity_with_fixed_token_ix_with_program_id(
        program_id,
        keys,
        args,
    )?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn increase_liquidity_with_fixed_token_invoke_signed(
    accounts: IncreaseLiquidityWithFixedTokenAccounts<'_, '_>,
    args: IncreaseLiquidityWithFixedTokenIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    increase_liquidity_with_fixed_token_invoke_signed_with_program_id(
        CREMA_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn increase_liquidity_with_fixed_token_verify_account_keys(
    accounts: IncreaseLiquidityWithFixedTokenAccounts<'_, '_>,
    keys: IncreaseLiquidityWithFixedTokenKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.owner.key, keys.owner),
        (*accounts.clmmpool.key, keys.clmmpool),
        (*accounts.position.key, keys.position),
        (*accounts.position_ata.key, keys.position_ata),
        (*accounts.token_a_ata.key, keys.token_a_ata),
        (*accounts.token_b_ata.key, keys.token_b_ata),
        (*accounts.token_a_vault.key, keys.token_a_vault),
        (*accounts.token_b_vault.key, keys.token_b_vault),
        (*accounts.tick_array_lower.key, keys.tick_array_lower),
        (*accounts.tick_array_upper.key, keys.tick_array_upper),
        (*accounts.tick_array_map.key, keys.tick_array_map),
        (*accounts.token_program.key, keys.token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn increase_liquidity_with_fixed_token_verify_writable_privileges<'me, 'info>(
    accounts: IncreaseLiquidityWithFixedTokenAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.owner,
        accounts.clmmpool,
        accounts.position,
        accounts.token_a_ata,
        accounts.token_b_ata,
        accounts.token_a_vault,
        accounts.token_b_vault,
        accounts.tick_array_lower,
        accounts.tick_array_upper,
        accounts.tick_array_map,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn increase_liquidity_with_fixed_token_verify_signer_privileges<'me, 'info>(
    accounts: IncreaseLiquidityWithFixedTokenAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.owner] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn increase_liquidity_with_fixed_token_verify_account_privileges<'me, 'info>(
    accounts: IncreaseLiquidityWithFixedTokenAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    increase_liquidity_with_fixed_token_verify_writable_privileges(accounts)?;
    increase_liquidity_with_fixed_token_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const DECREASE_LIQUIDITY_IX_ACCOUNTS_LEN: usize = 12;
#[derive(Copy, Clone, Debug)]
pub struct DecreaseLiquidityAccounts<'me, 'info> {
    pub owner: &'me AccountInfo<'info>,
    pub clmmpool: &'me AccountInfo<'info>,
    pub position: &'me AccountInfo<'info>,
    pub position_ata: &'me AccountInfo<'info>,
    pub token_a_ata: &'me AccountInfo<'info>,
    pub token_b_ata: &'me AccountInfo<'info>,
    pub token_a_vault: &'me AccountInfo<'info>,
    pub token_b_vault: &'me AccountInfo<'info>,
    pub tick_array_lower: &'me AccountInfo<'info>,
    pub tick_array_upper: &'me AccountInfo<'info>,
    pub tick_array_map: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct DecreaseLiquidityKeys {
    pub owner: Pubkey,
    pub clmmpool: Pubkey,
    pub position: Pubkey,
    pub position_ata: Pubkey,
    pub token_a_ata: Pubkey,
    pub token_b_ata: Pubkey,
    pub token_a_vault: Pubkey,
    pub token_b_vault: Pubkey,
    pub tick_array_lower: Pubkey,
    pub tick_array_upper: Pubkey,
    pub tick_array_map: Pubkey,
    pub token_program: Pubkey,
}
impl From<DecreaseLiquidityAccounts<'_, '_>> for DecreaseLiquidityKeys {
    fn from(accounts: DecreaseLiquidityAccounts) -> Self {
        Self {
            owner: *accounts.owner.key,
            clmmpool: *accounts.clmmpool.key,
            position: *accounts.position.key,
            position_ata: *accounts.position_ata.key,
            token_a_ata: *accounts.token_a_ata.key,
            token_b_ata: *accounts.token_b_ata.key,
            token_a_vault: *accounts.token_a_vault.key,
            token_b_vault: *accounts.token_b_vault.key,
            tick_array_lower: *accounts.tick_array_lower.key,
            tick_array_upper: *accounts.tick_array_upper.key,
            tick_array_map: *accounts.tick_array_map.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<DecreaseLiquidityKeys> for [AccountMeta; DECREASE_LIQUIDITY_IX_ACCOUNTS_LEN] {
    fn from(keys: DecreaseLiquidityKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.owner,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.clmmpool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.position,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.position_ata,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_a_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_b_ata,
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
                pubkey: keys.tick_array_map,
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
impl From<[Pubkey; DECREASE_LIQUIDITY_IX_ACCOUNTS_LEN]> for DecreaseLiquidityKeys {
    fn from(pubkeys: [Pubkey; DECREASE_LIQUIDITY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            owner: pubkeys[0],
            clmmpool: pubkeys[1],
            position: pubkeys[2],
            position_ata: pubkeys[3],
            token_a_ata: pubkeys[4],
            token_b_ata: pubkeys[5],
            token_a_vault: pubkeys[6],
            token_b_vault: pubkeys[7],
            tick_array_lower: pubkeys[8],
            tick_array_upper: pubkeys[9],
            tick_array_map: pubkeys[10],
            token_program: pubkeys[11],
        }
    }
}
impl<'info> From<DecreaseLiquidityAccounts<'_, 'info>>
for [AccountInfo<'info>; DECREASE_LIQUIDITY_IX_ACCOUNTS_LEN] {
    fn from(accounts: DecreaseLiquidityAccounts<'_, 'info>) -> Self {
        [
            accounts.owner.clone(),
            accounts.clmmpool.clone(),
            accounts.position.clone(),
            accounts.position_ata.clone(),
            accounts.token_a_ata.clone(),
            accounts.token_b_ata.clone(),
            accounts.token_a_vault.clone(),
            accounts.token_b_vault.clone(),
            accounts.tick_array_lower.clone(),
            accounts.tick_array_upper.clone(),
            accounts.tick_array_map.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; DECREASE_LIQUIDITY_IX_ACCOUNTS_LEN]>
for DecreaseLiquidityAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; DECREASE_LIQUIDITY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            owner: &arr[0],
            clmmpool: &arr[1],
            position: &arr[2],
            position_ata: &arr[3],
            token_a_ata: &arr[4],
            token_b_ata: &arr[5],
            token_a_vault: &arr[6],
            token_b_vault: &arr[7],
            tick_array_lower: &arr[8],
            tick_array_upper: &arr[9],
            tick_array_map: &arr[10],
            token_program: &arr[11],
        }
    }
}
pub const DECREASE_LIQUIDITY_IX_DISCM: [u8; 8usize] = [
    160, 38, 208, 111, 104, 91, 44, 1,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DecreaseLiquidityIxArgs {
    pub delta_liquidity: u128,
    pub token_a_min: u64,
    pub token_b_min: u64,
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
        let delta_liquidity: u128 = crate::borsh_de_or_default(&mut reader)?;
        let token_a_min: u64 = crate::borsh_de_or_default(&mut reader)?;
        let token_b_min: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(DecreaseLiquidityIxArgs {
                delta_liquidity,
                token_a_min,
                token_b_min,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&DECREASE_LIQUIDITY_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.delta_liquidity, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.token_a_min, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.token_b_min, &mut writer)?;
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
    decrease_liquidity_ix_with_program_id(CREMA_PROGRAM_ID, keys, args)
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
    decrease_liquidity_invoke_with_program_id(CREMA_PROGRAM_ID, accounts, args)
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
        CREMA_PROGRAM_ID,
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
        (*accounts.owner.key, keys.owner),
        (*accounts.clmmpool.key, keys.clmmpool),
        (*accounts.position.key, keys.position),
        (*accounts.position_ata.key, keys.position_ata),
        (*accounts.token_a_ata.key, keys.token_a_ata),
        (*accounts.token_b_ata.key, keys.token_b_ata),
        (*accounts.token_a_vault.key, keys.token_a_vault),
        (*accounts.token_b_vault.key, keys.token_b_vault),
        (*accounts.tick_array_lower.key, keys.tick_array_lower),
        (*accounts.tick_array_upper.key, keys.tick_array_upper),
        (*accounts.tick_array_map.key, keys.tick_array_map),
        (*accounts.token_program.key, keys.token_program),
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
        accounts.clmmpool,
        accounts.position,
        accounts.token_a_ata,
        accounts.token_b_ata,
        accounts.token_a_vault,
        accounts.token_b_vault,
        accounts.tick_array_lower,
        accounts.tick_array_upper,
        accounts.tick_array_map,
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
    for should_be_signer in [accounts.owner] {
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
pub const SWAP_IX_ACCOUNTS_LEN: usize = 11;
#[derive(Copy, Clone, Debug)]
pub struct SwapAccounts<'me, 'info> {
    pub clmm_config: &'me AccountInfo<'info>,
    pub clmmpool: &'me AccountInfo<'info>,
    pub token_a: &'me AccountInfo<'info>,
    pub token_b: &'me AccountInfo<'info>,
    pub account_a: &'me AccountInfo<'info>,
    pub account_b: &'me AccountInfo<'info>,
    pub token_a_vault: &'me AccountInfo<'info>,
    pub token_b_vault: &'me AccountInfo<'info>,
    pub tick_array_map: &'me AccountInfo<'info>,
    pub owner: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SwapKeys {
    pub clmm_config: Pubkey,
    pub clmmpool: Pubkey,
    pub token_a: Pubkey,
    pub token_b: Pubkey,
    pub account_a: Pubkey,
    pub account_b: Pubkey,
    pub token_a_vault: Pubkey,
    pub token_b_vault: Pubkey,
    pub tick_array_map: Pubkey,
    pub owner: Pubkey,
    pub token_program: Pubkey,
}
impl From<SwapAccounts<'_, '_>> for SwapKeys {
    fn from(accounts: SwapAccounts) -> Self {
        Self {
            clmm_config: *accounts.clmm_config.key,
            clmmpool: *accounts.clmmpool.key,
            token_a: *accounts.token_a.key,
            token_b: *accounts.token_b.key,
            account_a: *accounts.account_a.key,
            account_b: *accounts.account_b.key,
            token_a_vault: *accounts.token_a_vault.key,
            token_b_vault: *accounts.token_b_vault.key,
            tick_array_map: *accounts.tick_array_map.key,
            owner: *accounts.owner.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<SwapKeys> for [AccountMeta; SWAP_IX_ACCOUNTS_LEN] {
    fn from(keys: SwapKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.clmm_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.clmmpool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_a,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_b,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.account_a,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.account_b,
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
                pubkey: keys.tick_array_map,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.owner,
                is_signer: true,
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
            clmm_config: pubkeys[0],
            clmmpool: pubkeys[1],
            token_a: pubkeys[2],
            token_b: pubkeys[3],
            account_a: pubkeys[4],
            account_b: pubkeys[5],
            token_a_vault: pubkeys[6],
            token_b_vault: pubkeys[7],
            tick_array_map: pubkeys[8],
            owner: pubkeys[9],
            token_program: pubkeys[10],
        }
    }
}
impl<'info> From<SwapAccounts<'_, 'info>>
for [AccountInfo<'info>; SWAP_IX_ACCOUNTS_LEN] {
    fn from(accounts: SwapAccounts<'_, 'info>) -> Self {
        [
            accounts.clmm_config.clone(),
            accounts.clmmpool.clone(),
            accounts.token_a.clone(),
            accounts.token_b.clone(),
            accounts.account_a.clone(),
            accounts.account_b.clone(),
            accounts.token_a_vault.clone(),
            accounts.token_b_vault.clone(),
            accounts.tick_array_map.clone(),
            accounts.owner.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SWAP_IX_ACCOUNTS_LEN]>
for SwapAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; SWAP_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            clmm_config: &arr[0],
            clmmpool: &arr[1],
            token_a: &arr[2],
            token_b: &arr[3],
            account_a: &arr[4],
            account_b: &arr[5],
            token_a_vault: &arr[6],
            token_b_vault: &arr[7],
            tick_array_map: &arr[8],
            owner: &arr[9],
            token_program: &arr[10],
        }
    }
}
pub const SWAP_IX_DISCM: [u8; 8usize] = [248, 198, 158, 145, 225, 117, 135, 200];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SwapIxArgs {
    pub a_to_b: bool,
    pub by_amount_in: bool,
    pub amount: u64,
    pub amount_limit: u64,
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
        let a_to_b: bool = crate::borsh_de_or_default(&mut reader)?;
        let by_amount_in: bool = crate::borsh_de_or_default(&mut reader)?;
        let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let amount_limit: u64 = crate::borsh_de_or_default(&mut reader)?;
        let sqrt_price_limit: u128 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(SwapIxArgs {
                a_to_b,
                by_amount_in,
                amount,
                amount_limit,
                sqrt_price_limit,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SWAP_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.a_to_b, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.by_amount_in, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.amount_limit, &mut writer)?;
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
    swap_ix_with_program_id(CREMA_PROGRAM_ID, keys, args)
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
    swap_invoke_with_program_id(CREMA_PROGRAM_ID, accounts, args)
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
    swap_invoke_signed_with_program_id(CREMA_PROGRAM_ID, accounts, args, seeds)
}
pub fn swap_verify_account_keys(
    accounts: SwapAccounts<'_, '_>,
    keys: SwapKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.clmm_config.key, keys.clmm_config),
        (*accounts.clmmpool.key, keys.clmmpool),
        (*accounts.token_a.key, keys.token_a),
        (*accounts.token_b.key, keys.token_b),
        (*accounts.account_a.key, keys.account_a),
        (*accounts.account_b.key, keys.account_b),
        (*accounts.token_a_vault.key, keys.token_a_vault),
        (*accounts.token_b_vault.key, keys.token_b_vault),
        (*accounts.tick_array_map.key, keys.tick_array_map),
        (*accounts.owner.key, keys.owner),
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
        accounts.clmmpool,
        accounts.account_a,
        accounts.account_b,
        accounts.token_a_vault,
        accounts.token_b_vault,
        accounts.tick_array_map,
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
pub const COLLECT_FEE_IX_ACCOUNTS_LEN: usize = 11;
#[derive(Copy, Clone, Debug)]
pub struct CollectFeeAccounts<'me, 'info> {
    pub owner: &'me AccountInfo<'info>,
    pub clmmpool: &'me AccountInfo<'info>,
    pub position: &'me AccountInfo<'info>,
    pub position_ata: &'me AccountInfo<'info>,
    pub token_a_ata: &'me AccountInfo<'info>,
    pub token_b_ata: &'me AccountInfo<'info>,
    pub token_a_vault: &'me AccountInfo<'info>,
    pub token_b_vault: &'me AccountInfo<'info>,
    pub tick_array_lower: &'me AccountInfo<'info>,
    pub tick_array_upper: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CollectFeeKeys {
    pub owner: Pubkey,
    pub clmmpool: Pubkey,
    pub position: Pubkey,
    pub position_ata: Pubkey,
    pub token_a_ata: Pubkey,
    pub token_b_ata: Pubkey,
    pub token_a_vault: Pubkey,
    pub token_b_vault: Pubkey,
    pub tick_array_lower: Pubkey,
    pub tick_array_upper: Pubkey,
    pub token_program: Pubkey,
}
impl From<CollectFeeAccounts<'_, '_>> for CollectFeeKeys {
    fn from(accounts: CollectFeeAccounts) -> Self {
        Self {
            owner: *accounts.owner.key,
            clmmpool: *accounts.clmmpool.key,
            position: *accounts.position.key,
            position_ata: *accounts.position_ata.key,
            token_a_ata: *accounts.token_a_ata.key,
            token_b_ata: *accounts.token_b_ata.key,
            token_a_vault: *accounts.token_a_vault.key,
            token_b_vault: *accounts.token_b_vault.key,
            tick_array_lower: *accounts.tick_array_lower.key,
            tick_array_upper: *accounts.tick_array_upper.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<CollectFeeKeys> for [AccountMeta; COLLECT_FEE_IX_ACCOUNTS_LEN] {
    fn from(keys: CollectFeeKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.owner,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.clmmpool,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.position,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.position_ata,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_a_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_b_ata,
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
                pubkey: keys.tick_array_lower,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.tick_array_upper,
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
impl From<[Pubkey; COLLECT_FEE_IX_ACCOUNTS_LEN]> for CollectFeeKeys {
    fn from(pubkeys: [Pubkey; COLLECT_FEE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            owner: pubkeys[0],
            clmmpool: pubkeys[1],
            position: pubkeys[2],
            position_ata: pubkeys[3],
            token_a_ata: pubkeys[4],
            token_b_ata: pubkeys[5],
            token_a_vault: pubkeys[6],
            token_b_vault: pubkeys[7],
            tick_array_lower: pubkeys[8],
            tick_array_upper: pubkeys[9],
            token_program: pubkeys[10],
        }
    }
}
impl<'info> From<CollectFeeAccounts<'_, 'info>>
for [AccountInfo<'info>; COLLECT_FEE_IX_ACCOUNTS_LEN] {
    fn from(accounts: CollectFeeAccounts<'_, 'info>) -> Self {
        [
            accounts.owner.clone(),
            accounts.clmmpool.clone(),
            accounts.position.clone(),
            accounts.position_ata.clone(),
            accounts.token_a_ata.clone(),
            accounts.token_b_ata.clone(),
            accounts.token_a_vault.clone(),
            accounts.token_b_vault.clone(),
            accounts.tick_array_lower.clone(),
            accounts.tick_array_upper.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; COLLECT_FEE_IX_ACCOUNTS_LEN]>
for CollectFeeAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; COLLECT_FEE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            owner: &arr[0],
            clmmpool: &arr[1],
            position: &arr[2],
            position_ata: &arr[3],
            token_a_ata: &arr[4],
            token_b_ata: &arr[5],
            token_a_vault: &arr[6],
            token_b_vault: &arr[7],
            tick_array_lower: &arr[8],
            tick_array_upper: &arr[9],
            token_program: &arr[10],
        }
    }
}
pub const COLLECT_FEE_IX_DISCM: [u8; 8usize] = [60, 173, 247, 103, 4, 93, 130, 48];
#[derive(Clone, Debug, PartialEq)]
pub struct CollectFeeIxData;
impl CollectFeeIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != COLLECT_FEE_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&COLLECT_FEE_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn collect_fee_ix_with_program_id(
    program_id: Pubkey,
    keys: CollectFeeKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; COLLECT_FEE_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: CollectFeeIxData.try_to_vec()?,
    })
}
pub fn collect_fee_ix(keys: CollectFeeKeys) -> std::io::Result<Instruction> {
    collect_fee_ix_with_program_id(CREMA_PROGRAM_ID, keys)
}
pub fn collect_fee_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CollectFeeAccounts<'_, '_>,
) -> ProgramResult {
    let keys: CollectFeeKeys = accounts.into();
    let ix = collect_fee_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn collect_fee_invoke(accounts: CollectFeeAccounts<'_, '_>) -> ProgramResult {
    collect_fee_invoke_with_program_id(CREMA_PROGRAM_ID, accounts)
}
pub fn collect_fee_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CollectFeeAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CollectFeeKeys = accounts.into();
    let ix = collect_fee_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn collect_fee_invoke_signed(
    accounts: CollectFeeAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    collect_fee_invoke_signed_with_program_id(CREMA_PROGRAM_ID, accounts, seeds)
}
pub fn collect_fee_verify_account_keys(
    accounts: CollectFeeAccounts<'_, '_>,
    keys: CollectFeeKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.owner.key, keys.owner),
        (*accounts.clmmpool.key, keys.clmmpool),
        (*accounts.position.key, keys.position),
        (*accounts.position_ata.key, keys.position_ata),
        (*accounts.token_a_ata.key, keys.token_a_ata),
        (*accounts.token_b_ata.key, keys.token_b_ata),
        (*accounts.token_a_vault.key, keys.token_a_vault),
        (*accounts.token_b_vault.key, keys.token_b_vault),
        (*accounts.tick_array_lower.key, keys.tick_array_lower),
        (*accounts.tick_array_upper.key, keys.tick_array_upper),
        (*accounts.token_program.key, keys.token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn collect_fee_verify_writable_privileges<'me, 'info>(
    accounts: CollectFeeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.position,
        accounts.token_a_ata,
        accounts.token_b_ata,
        accounts.token_a_vault,
        accounts.token_b_vault,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn collect_fee_verify_signer_privileges<'me, 'info>(
    accounts: CollectFeeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.owner] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn collect_fee_verify_account_privileges<'me, 'info>(
    accounts: CollectFeeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    collect_fee_verify_writable_privileges(accounts)?;
    collect_fee_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const COLLECT_PROTOCOL_FEE_IX_ACCOUNTS_LEN: usize = 8;
#[derive(Copy, Clone, Debug)]
pub struct CollectProtocolFeeAccounts<'me, 'info> {
    pub protocol_fee_claim_authority: &'me AccountInfo<'info>,
    pub clmm_config: &'me AccountInfo<'info>,
    pub clmmpool: &'me AccountInfo<'info>,
    pub token_a_ata: &'me AccountInfo<'info>,
    pub token_b_ata: &'me AccountInfo<'info>,
    pub token_a_vault: &'me AccountInfo<'info>,
    pub token_b_vault: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CollectProtocolFeeKeys {
    pub protocol_fee_claim_authority: Pubkey,
    pub clmm_config: Pubkey,
    pub clmmpool: Pubkey,
    pub token_a_ata: Pubkey,
    pub token_b_ata: Pubkey,
    pub token_a_vault: Pubkey,
    pub token_b_vault: Pubkey,
    pub token_program: Pubkey,
}
impl From<CollectProtocolFeeAccounts<'_, '_>> for CollectProtocolFeeKeys {
    fn from(accounts: CollectProtocolFeeAccounts) -> Self {
        Self {
            protocol_fee_claim_authority: *accounts.protocol_fee_claim_authority.key,
            clmm_config: *accounts.clmm_config.key,
            clmmpool: *accounts.clmmpool.key,
            token_a_ata: *accounts.token_a_ata.key,
            token_b_ata: *accounts.token_b_ata.key,
            token_a_vault: *accounts.token_a_vault.key,
            token_b_vault: *accounts.token_b_vault.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<CollectProtocolFeeKeys>
for [AccountMeta; COLLECT_PROTOCOL_FEE_IX_ACCOUNTS_LEN] {
    fn from(keys: CollectProtocolFeeKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.protocol_fee_claim_authority,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.clmm_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.clmmpool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_a_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_b_ata,
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
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; COLLECT_PROTOCOL_FEE_IX_ACCOUNTS_LEN]> for CollectProtocolFeeKeys {
    fn from(pubkeys: [Pubkey; COLLECT_PROTOCOL_FEE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            protocol_fee_claim_authority: pubkeys[0],
            clmm_config: pubkeys[1],
            clmmpool: pubkeys[2],
            token_a_ata: pubkeys[3],
            token_b_ata: pubkeys[4],
            token_a_vault: pubkeys[5],
            token_b_vault: pubkeys[6],
            token_program: pubkeys[7],
        }
    }
}
impl<'info> From<CollectProtocolFeeAccounts<'_, 'info>>
for [AccountInfo<'info>; COLLECT_PROTOCOL_FEE_IX_ACCOUNTS_LEN] {
    fn from(accounts: CollectProtocolFeeAccounts<'_, 'info>) -> Self {
        [
            accounts.protocol_fee_claim_authority.clone(),
            accounts.clmm_config.clone(),
            accounts.clmmpool.clone(),
            accounts.token_a_ata.clone(),
            accounts.token_b_ata.clone(),
            accounts.token_a_vault.clone(),
            accounts.token_b_vault.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; COLLECT_PROTOCOL_FEE_IX_ACCOUNTS_LEN]>
for CollectProtocolFeeAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; COLLECT_PROTOCOL_FEE_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            protocol_fee_claim_authority: &arr[0],
            clmm_config: &arr[1],
            clmmpool: &arr[2],
            token_a_ata: &arr[3],
            token_b_ata: &arr[4],
            token_a_vault: &arr[5],
            token_b_vault: &arr[6],
            token_program: &arr[7],
        }
    }
}
pub const COLLECT_PROTOCOL_FEE_IX_DISCM: [u8; 8usize] = [
    136, 136, 252, 221, 194, 66, 126, 89,
];
#[derive(Clone, Debug, PartialEq)]
pub struct CollectProtocolFeeIxData;
impl CollectProtocolFeeIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != COLLECT_PROTOCOL_FEE_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&COLLECT_PROTOCOL_FEE_IX_DISCM)
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
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; COLLECT_PROTOCOL_FEE_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: CollectProtocolFeeIxData.try_to_vec()?,
    })
}
pub fn collect_protocol_fee_ix(
    keys: CollectProtocolFeeKeys,
) -> std::io::Result<Instruction> {
    collect_protocol_fee_ix_with_program_id(CREMA_PROGRAM_ID, keys)
}
pub fn collect_protocol_fee_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CollectProtocolFeeAccounts<'_, '_>,
) -> ProgramResult {
    let keys: CollectProtocolFeeKeys = accounts.into();
    let ix = collect_protocol_fee_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn collect_protocol_fee_invoke(
    accounts: CollectProtocolFeeAccounts<'_, '_>,
) -> ProgramResult {
    collect_protocol_fee_invoke_with_program_id(CREMA_PROGRAM_ID, accounts)
}
pub fn collect_protocol_fee_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CollectProtocolFeeAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CollectProtocolFeeKeys = accounts.into();
    let ix = collect_protocol_fee_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn collect_protocol_fee_invoke_signed(
    accounts: CollectProtocolFeeAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    collect_protocol_fee_invoke_signed_with_program_id(CREMA_PROGRAM_ID, accounts, seeds)
}
pub fn collect_protocol_fee_verify_account_keys(
    accounts: CollectProtocolFeeAccounts<'_, '_>,
    keys: CollectProtocolFeeKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.protocol_fee_claim_authority.key, keys.protocol_fee_claim_authority),
        (*accounts.clmm_config.key, keys.clmm_config),
        (*accounts.clmmpool.key, keys.clmmpool),
        (*accounts.token_a_ata.key, keys.token_a_ata),
        (*accounts.token_b_ata.key, keys.token_b_ata),
        (*accounts.token_a_vault.key, keys.token_a_vault),
        (*accounts.token_b_vault.key, keys.token_b_vault),
        (*accounts.token_program.key, keys.token_program),
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
        accounts.clmmpool,
        accounts.token_a_ata,
        accounts.token_b_ata,
        accounts.token_a_vault,
        accounts.token_b_vault,
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
    for should_be_signer in [accounts.protocol_fee_claim_authority] {
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
pub const CREATE_PARTNER_IX_ACCOUNTS_LEN: usize = 7;
#[derive(Copy, Clone, Debug)]
pub struct CreatePartnerAccounts<'me, 'info> {
    pub payer: &'me AccountInfo<'info>,
    pub clmm_config: &'me AccountInfo<'info>,
    pub protocol_authority: &'me AccountInfo<'info>,
    pub base: &'me AccountInfo<'info>,
    pub partner: &'me AccountInfo<'info>,
    pub rent: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CreatePartnerKeys {
    pub payer: Pubkey,
    pub clmm_config: Pubkey,
    pub protocol_authority: Pubkey,
    pub base: Pubkey,
    pub partner: Pubkey,
    pub rent: Pubkey,
    pub system_program: Pubkey,
}
impl From<CreatePartnerAccounts<'_, '_>> for CreatePartnerKeys {
    fn from(accounts: CreatePartnerAccounts) -> Self {
        Self {
            payer: *accounts.payer.key,
            clmm_config: *accounts.clmm_config.key,
            protocol_authority: *accounts.protocol_authority.key,
            base: *accounts.base.key,
            partner: *accounts.partner.key,
            rent: *accounts.rent.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<CreatePartnerKeys> for [AccountMeta; CREATE_PARTNER_IX_ACCOUNTS_LEN] {
    fn from(keys: CreatePartnerKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.payer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.clmm_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.protocol_authority,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.base,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.partner,
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
        ]
    }
}
impl From<[Pubkey; CREATE_PARTNER_IX_ACCOUNTS_LEN]> for CreatePartnerKeys {
    fn from(pubkeys: [Pubkey; CREATE_PARTNER_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            payer: pubkeys[0],
            clmm_config: pubkeys[1],
            protocol_authority: pubkeys[2],
            base: pubkeys[3],
            partner: pubkeys[4],
            rent: pubkeys[5],
            system_program: pubkeys[6],
        }
    }
}
impl<'info> From<CreatePartnerAccounts<'_, 'info>>
for [AccountInfo<'info>; CREATE_PARTNER_IX_ACCOUNTS_LEN] {
    fn from(accounts: CreatePartnerAccounts<'_, 'info>) -> Self {
        [
            accounts.payer.clone(),
            accounts.clmm_config.clone(),
            accounts.protocol_authority.clone(),
            accounts.base.clone(),
            accounts.partner.clone(),
            accounts.rent.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CREATE_PARTNER_IX_ACCOUNTS_LEN]>
for CreatePartnerAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; CREATE_PARTNER_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            payer: &arr[0],
            clmm_config: &arr[1],
            protocol_authority: &arr[2],
            base: &arr[3],
            partner: &arr[4],
            rent: &arr[5],
            system_program: &arr[6],
        }
    }
}
pub const CREATE_PARTNER_IX_DISCM: [u8; 8usize] = [220, 20, 67, 171, 205, 106, 128, 56];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CreatePartnerIxArgs {
    pub partner_fee_claim_authority: Pubkey,
    pub fee_rate: u16,
    pub start_time: u64,
    pub end_time: u64,
    pub name: String,
}
#[derive(Clone, Debug, PartialEq)]
pub struct CreatePartnerIxData(pub CreatePartnerIxArgs);
impl From<CreatePartnerIxArgs> for CreatePartnerIxData {
    fn from(args: CreatePartnerIxArgs) -> Self {
        Self(args)
    }
}
impl CreatePartnerIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CREATE_PARTNER_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let partner_fee_claim_authority: Pubkey = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let fee_rate: u16 = crate::borsh_de_or_default(&mut reader)?;
        let start_time: u64 = crate::borsh_de_or_default(&mut reader)?;
        let end_time: u64 = crate::borsh_de_or_default(&mut reader)?;
        let name: String = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(CreatePartnerIxArgs {
                partner_fee_claim_authority,
                fee_rate,
                start_time,
                end_time,
                name,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CREATE_PARTNER_IX_DISCM)?;
        borsh::BorshSerialize::serialize(
            &self.0.partner_fee_claim_authority,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.0.fee_rate, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.start_time, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.end_time, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.name, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn create_partner_ix_with_program_id(
    program_id: Pubkey,
    keys: CreatePartnerKeys,
    args: CreatePartnerIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CREATE_PARTNER_IX_ACCOUNTS_LEN] = keys.into();
    let data: CreatePartnerIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn create_partner_ix(
    keys: CreatePartnerKeys,
    args: CreatePartnerIxArgs,
) -> std::io::Result<Instruction> {
    create_partner_ix_with_program_id(CREMA_PROGRAM_ID, keys, args)
}
pub fn create_partner_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CreatePartnerAccounts<'_, '_>,
    args: CreatePartnerIxArgs,
) -> ProgramResult {
    let keys: CreatePartnerKeys = accounts.into();
    let ix = create_partner_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn create_partner_invoke(
    accounts: CreatePartnerAccounts<'_, '_>,
    args: CreatePartnerIxArgs,
) -> ProgramResult {
    create_partner_invoke_with_program_id(CREMA_PROGRAM_ID, accounts, args)
}
pub fn create_partner_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CreatePartnerAccounts<'_, '_>,
    args: CreatePartnerIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CreatePartnerKeys = accounts.into();
    let ix = create_partner_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn create_partner_invoke_signed(
    accounts: CreatePartnerAccounts<'_, '_>,
    args: CreatePartnerIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    create_partner_invoke_signed_with_program_id(CREMA_PROGRAM_ID, accounts, args, seeds)
}
pub fn create_partner_verify_account_keys(
    accounts: CreatePartnerAccounts<'_, '_>,
    keys: CreatePartnerKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.payer.key, keys.payer),
        (*accounts.clmm_config.key, keys.clmm_config),
        (*accounts.protocol_authority.key, keys.protocol_authority),
        (*accounts.base.key, keys.base),
        (*accounts.partner.key, keys.partner),
        (*accounts.rent.key, keys.rent),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn create_partner_verify_writable_privileges<'me, 'info>(
    accounts: CreatePartnerAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.payer, accounts.partner] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn create_partner_verify_signer_privileges<'me, 'info>(
    accounts: CreatePartnerAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [
        accounts.payer,
        accounts.protocol_authority,
        accounts.base,
    ] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn create_partner_verify_account_privileges<'me, 'info>(
    accounts: CreatePartnerAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    create_partner_verify_writable_privileges(accounts)?;
    create_partner_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const UPDATE_PARTNER_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct UpdatePartnerAccounts<'me, 'info> {
    pub clmm_config: &'me AccountInfo<'info>,
    pub partner: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UpdatePartnerKeys {
    pub clmm_config: Pubkey,
    pub partner: Pubkey,
    pub authority: Pubkey,
}
impl From<UpdatePartnerAccounts<'_, '_>> for UpdatePartnerKeys {
    fn from(accounts: UpdatePartnerAccounts) -> Self {
        Self {
            clmm_config: *accounts.clmm_config.key,
            partner: *accounts.partner.key,
            authority: *accounts.authority.key,
        }
    }
}
impl From<UpdatePartnerKeys> for [AccountMeta; UPDATE_PARTNER_IX_ACCOUNTS_LEN] {
    fn from(keys: UpdatePartnerKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.clmm_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.partner,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.authority,
                is_signer: true,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; UPDATE_PARTNER_IX_ACCOUNTS_LEN]> for UpdatePartnerKeys {
    fn from(pubkeys: [Pubkey; UPDATE_PARTNER_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            clmm_config: pubkeys[0],
            partner: pubkeys[1],
            authority: pubkeys[2],
        }
    }
}
impl<'info> From<UpdatePartnerAccounts<'_, 'info>>
for [AccountInfo<'info>; UPDATE_PARTNER_IX_ACCOUNTS_LEN] {
    fn from(accounts: UpdatePartnerAccounts<'_, 'info>) -> Self {
        [
            accounts.clmm_config.clone(),
            accounts.partner.clone(),
            accounts.authority.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; UPDATE_PARTNER_IX_ACCOUNTS_LEN]>
for UpdatePartnerAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; UPDATE_PARTNER_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            clmm_config: &arr[0],
            partner: &arr[1],
            authority: &arr[2],
        }
    }
}
pub const UPDATE_PARTNER_IX_DISCM: [u8; 8usize] = [19, 112, 236, 81, 127, 55, 21, 196];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UpdatePartnerIxArgs {
    pub new_fee_rate: Option<u16>,
    pub new_claim_authority: Option<Pubkey>,
    pub start_time: Option<u64>,
    pub end_time: Option<u64>,
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
        let new_fee_rate: Option<u16> = crate::borsh_de_or_default(&mut reader)?;
        let new_claim_authority: Option<Pubkey> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let start_time: Option<u64> = crate::borsh_de_or_default(&mut reader)?;
        let end_time: Option<u64> = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(UpdatePartnerIxArgs {
                new_fee_rate,
                new_claim_authority,
                start_time,
                end_time,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_PARTNER_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.new_fee_rate, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.new_claim_authority, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.start_time, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.end_time, &mut writer)?;
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
    update_partner_ix_with_program_id(CREMA_PROGRAM_ID, keys, args)
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
    update_partner_invoke_with_program_id(CREMA_PROGRAM_ID, accounts, args)
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
    update_partner_invoke_signed_with_program_id(CREMA_PROGRAM_ID, accounts, args, seeds)
}
pub fn update_partner_verify_account_keys(
    accounts: UpdatePartnerAccounts<'_, '_>,
    keys: UpdatePartnerKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.clmm_config.key, keys.clmm_config),
        (*accounts.partner.key, keys.partner),
        (*accounts.authority.key, keys.authority),
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
pub const COLLECT_PARTNER_FEE_IX_ACCOUNTS_LEN: usize = 8;
#[derive(Copy, Clone, Debug)]
pub struct CollectPartnerFeeAccounts<'me, 'info> {
    pub partner_fee_claim_authority: &'me AccountInfo<'info>,
    pub partner: &'me AccountInfo<'info>,
    pub clmmpool: &'me AccountInfo<'info>,
    pub token_a_ata: &'me AccountInfo<'info>,
    pub token_b_ata: &'me AccountInfo<'info>,
    pub token_a_partner_fee_vault: &'me AccountInfo<'info>,
    pub token_b_partner_fee_vault: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CollectPartnerFeeKeys {
    pub partner_fee_claim_authority: Pubkey,
    pub partner: Pubkey,
    pub clmmpool: Pubkey,
    pub token_a_ata: Pubkey,
    pub token_b_ata: Pubkey,
    pub token_a_partner_fee_vault: Pubkey,
    pub token_b_partner_fee_vault: Pubkey,
    pub token_program: Pubkey,
}
impl From<CollectPartnerFeeAccounts<'_, '_>> for CollectPartnerFeeKeys {
    fn from(accounts: CollectPartnerFeeAccounts) -> Self {
        Self {
            partner_fee_claim_authority: *accounts.partner_fee_claim_authority.key,
            partner: *accounts.partner.key,
            clmmpool: *accounts.clmmpool.key,
            token_a_ata: *accounts.token_a_ata.key,
            token_b_ata: *accounts.token_b_ata.key,
            token_a_partner_fee_vault: *accounts.token_a_partner_fee_vault.key,
            token_b_partner_fee_vault: *accounts.token_b_partner_fee_vault.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<CollectPartnerFeeKeys> for [AccountMeta; COLLECT_PARTNER_FEE_IX_ACCOUNTS_LEN] {
    fn from(keys: CollectPartnerFeeKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.partner_fee_claim_authority,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.partner,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.clmmpool,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_a_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_b_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_a_partner_fee_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_b_partner_fee_vault,
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
impl From<[Pubkey; COLLECT_PARTNER_FEE_IX_ACCOUNTS_LEN]> for CollectPartnerFeeKeys {
    fn from(pubkeys: [Pubkey; COLLECT_PARTNER_FEE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            partner_fee_claim_authority: pubkeys[0],
            partner: pubkeys[1],
            clmmpool: pubkeys[2],
            token_a_ata: pubkeys[3],
            token_b_ata: pubkeys[4],
            token_a_partner_fee_vault: pubkeys[5],
            token_b_partner_fee_vault: pubkeys[6],
            token_program: pubkeys[7],
        }
    }
}
impl<'info> From<CollectPartnerFeeAccounts<'_, 'info>>
for [AccountInfo<'info>; COLLECT_PARTNER_FEE_IX_ACCOUNTS_LEN] {
    fn from(accounts: CollectPartnerFeeAccounts<'_, 'info>) -> Self {
        [
            accounts.partner_fee_claim_authority.clone(),
            accounts.partner.clone(),
            accounts.clmmpool.clone(),
            accounts.token_a_ata.clone(),
            accounts.token_b_ata.clone(),
            accounts.token_a_partner_fee_vault.clone(),
            accounts.token_b_partner_fee_vault.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; COLLECT_PARTNER_FEE_IX_ACCOUNTS_LEN]>
for CollectPartnerFeeAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; COLLECT_PARTNER_FEE_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            partner_fee_claim_authority: &arr[0],
            partner: &arr[1],
            clmmpool: &arr[2],
            token_a_ata: &arr[3],
            token_b_ata: &arr[4],
            token_a_partner_fee_vault: &arr[5],
            token_b_partner_fee_vault: &arr[6],
            token_program: &arr[7],
        }
    }
}
pub const COLLECT_PARTNER_FEE_IX_DISCM: [u8; 8usize] = [
    143, 25, 124, 254, 164, 204, 231, 51,
];
#[derive(Clone, Debug, PartialEq)]
pub struct CollectPartnerFeeIxData;
impl CollectPartnerFeeIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != COLLECT_PARTNER_FEE_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&COLLECT_PARTNER_FEE_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn collect_partner_fee_ix_with_program_id(
    program_id: Pubkey,
    keys: CollectPartnerFeeKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; COLLECT_PARTNER_FEE_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: CollectPartnerFeeIxData.try_to_vec()?,
    })
}
pub fn collect_partner_fee_ix(
    keys: CollectPartnerFeeKeys,
) -> std::io::Result<Instruction> {
    collect_partner_fee_ix_with_program_id(CREMA_PROGRAM_ID, keys)
}
pub fn collect_partner_fee_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CollectPartnerFeeAccounts<'_, '_>,
) -> ProgramResult {
    let keys: CollectPartnerFeeKeys = accounts.into();
    let ix = collect_partner_fee_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn collect_partner_fee_invoke(
    accounts: CollectPartnerFeeAccounts<'_, '_>,
) -> ProgramResult {
    collect_partner_fee_invoke_with_program_id(CREMA_PROGRAM_ID, accounts)
}
pub fn collect_partner_fee_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CollectPartnerFeeAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CollectPartnerFeeKeys = accounts.into();
    let ix = collect_partner_fee_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn collect_partner_fee_invoke_signed(
    accounts: CollectPartnerFeeAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    collect_partner_fee_invoke_signed_with_program_id(CREMA_PROGRAM_ID, accounts, seeds)
}
pub fn collect_partner_fee_verify_account_keys(
    accounts: CollectPartnerFeeAccounts<'_, '_>,
    keys: CollectPartnerFeeKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.partner_fee_claim_authority.key, keys.partner_fee_claim_authority),
        (*accounts.partner.key, keys.partner),
        (*accounts.clmmpool.key, keys.clmmpool),
        (*accounts.token_a_ata.key, keys.token_a_ata),
        (*accounts.token_b_ata.key, keys.token_b_ata),
        (*accounts.token_a_partner_fee_vault.key, keys.token_a_partner_fee_vault),
        (*accounts.token_b_partner_fee_vault.key, keys.token_b_partner_fee_vault),
        (*accounts.token_program.key, keys.token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn collect_partner_fee_verify_writable_privileges<'me, 'info>(
    accounts: CollectPartnerFeeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.token_a_ata,
        accounts.token_b_ata,
        accounts.token_a_partner_fee_vault,
        accounts.token_b_partner_fee_vault,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn collect_partner_fee_verify_signer_privileges<'me, 'info>(
    accounts: CollectPartnerFeeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.partner_fee_claim_authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn collect_partner_fee_verify_account_privileges<'me, 'info>(
    accounts: CollectPartnerFeeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    collect_partner_fee_verify_writable_privileges(accounts)?;
    collect_partner_fee_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SWAP_WITH_PARTNER_IX_ACCOUNTS_LEN: usize = 14;
#[derive(Copy, Clone, Debug)]
pub struct SwapWithPartnerAccounts<'me, 'info> {
    pub clmm_config: &'me AccountInfo<'info>,
    pub clmmpool: &'me AccountInfo<'info>,
    pub token_a: &'me AccountInfo<'info>,
    pub token_b: &'me AccountInfo<'info>,
    pub account_a: &'me AccountInfo<'info>,
    pub account_b: &'me AccountInfo<'info>,
    pub token_a_vault: &'me AccountInfo<'info>,
    pub token_b_vault: &'me AccountInfo<'info>,
    pub tick_array_map: &'me AccountInfo<'info>,
    pub owner: &'me AccountInfo<'info>,
    pub partner: &'me AccountInfo<'info>,
    pub partner_ata_a: &'me AccountInfo<'info>,
    pub partner_ata_b: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SwapWithPartnerKeys {
    pub clmm_config: Pubkey,
    pub clmmpool: Pubkey,
    pub token_a: Pubkey,
    pub token_b: Pubkey,
    pub account_a: Pubkey,
    pub account_b: Pubkey,
    pub token_a_vault: Pubkey,
    pub token_b_vault: Pubkey,
    pub tick_array_map: Pubkey,
    pub owner: Pubkey,
    pub partner: Pubkey,
    pub partner_ata_a: Pubkey,
    pub partner_ata_b: Pubkey,
    pub token_program: Pubkey,
}
impl From<SwapWithPartnerAccounts<'_, '_>> for SwapWithPartnerKeys {
    fn from(accounts: SwapWithPartnerAccounts) -> Self {
        Self {
            clmm_config: *accounts.clmm_config.key,
            clmmpool: *accounts.clmmpool.key,
            token_a: *accounts.token_a.key,
            token_b: *accounts.token_b.key,
            account_a: *accounts.account_a.key,
            account_b: *accounts.account_b.key,
            token_a_vault: *accounts.token_a_vault.key,
            token_b_vault: *accounts.token_b_vault.key,
            tick_array_map: *accounts.tick_array_map.key,
            owner: *accounts.owner.key,
            partner: *accounts.partner.key,
            partner_ata_a: *accounts.partner_ata_a.key,
            partner_ata_b: *accounts.partner_ata_b.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<SwapWithPartnerKeys> for [AccountMeta; SWAP_WITH_PARTNER_IX_ACCOUNTS_LEN] {
    fn from(keys: SwapWithPartnerKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.clmm_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.clmmpool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_a,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_b,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.account_a,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.account_b,
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
                pubkey: keys.tick_array_map,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.owner,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.partner,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.partner_ata_a,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.partner_ata_b,
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
impl From<[Pubkey; SWAP_WITH_PARTNER_IX_ACCOUNTS_LEN]> for SwapWithPartnerKeys {
    fn from(pubkeys: [Pubkey; SWAP_WITH_PARTNER_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            clmm_config: pubkeys[0],
            clmmpool: pubkeys[1],
            token_a: pubkeys[2],
            token_b: pubkeys[3],
            account_a: pubkeys[4],
            account_b: pubkeys[5],
            token_a_vault: pubkeys[6],
            token_b_vault: pubkeys[7],
            tick_array_map: pubkeys[8],
            owner: pubkeys[9],
            partner: pubkeys[10],
            partner_ata_a: pubkeys[11],
            partner_ata_b: pubkeys[12],
            token_program: pubkeys[13],
        }
    }
}
impl<'info> From<SwapWithPartnerAccounts<'_, 'info>>
for [AccountInfo<'info>; SWAP_WITH_PARTNER_IX_ACCOUNTS_LEN] {
    fn from(accounts: SwapWithPartnerAccounts<'_, 'info>) -> Self {
        [
            accounts.clmm_config.clone(),
            accounts.clmmpool.clone(),
            accounts.token_a.clone(),
            accounts.token_b.clone(),
            accounts.account_a.clone(),
            accounts.account_b.clone(),
            accounts.token_a_vault.clone(),
            accounts.token_b_vault.clone(),
            accounts.tick_array_map.clone(),
            accounts.owner.clone(),
            accounts.partner.clone(),
            accounts.partner_ata_a.clone(),
            accounts.partner_ata_b.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SWAP_WITH_PARTNER_IX_ACCOUNTS_LEN]>
for SwapWithPartnerAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; SWAP_WITH_PARTNER_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            clmm_config: &arr[0],
            clmmpool: &arr[1],
            token_a: &arr[2],
            token_b: &arr[3],
            account_a: &arr[4],
            account_b: &arr[5],
            token_a_vault: &arr[6],
            token_b_vault: &arr[7],
            tick_array_map: &arr[8],
            owner: &arr[9],
            partner: &arr[10],
            partner_ata_a: &arr[11],
            partner_ata_b: &arr[12],
            token_program: &arr[13],
        }
    }
}
pub const SWAP_WITH_PARTNER_IX_DISCM: [u8; 8usize] = [
    133, 215, 191, 214, 102, 243, 55, 25,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SwapWithPartnerIxArgs {
    pub a_to_b: bool,
    pub by_amount_in: bool,
    pub amount: u64,
    pub amount_limit: u64,
    pub sqrt_price_limit: u128,
}
#[derive(Clone, Debug, PartialEq)]
pub struct SwapWithPartnerIxData(pub SwapWithPartnerIxArgs);
impl From<SwapWithPartnerIxArgs> for SwapWithPartnerIxData {
    fn from(args: SwapWithPartnerIxArgs) -> Self {
        Self(args)
    }
}
impl SwapWithPartnerIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SWAP_WITH_PARTNER_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let a_to_b: bool = crate::borsh_de_or_default(&mut reader)?;
        let by_amount_in: bool = crate::borsh_de_or_default(&mut reader)?;
        let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let amount_limit: u64 = crate::borsh_de_or_default(&mut reader)?;
        let sqrt_price_limit: u128 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(SwapWithPartnerIxArgs {
                a_to_b,
                by_amount_in,
                amount,
                amount_limit,
                sqrt_price_limit,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SWAP_WITH_PARTNER_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.a_to_b, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.by_amount_in, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.amount_limit, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.sqrt_price_limit, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn swap_with_partner_ix_with_program_id(
    program_id: Pubkey,
    keys: SwapWithPartnerKeys,
    args: SwapWithPartnerIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SWAP_WITH_PARTNER_IX_ACCOUNTS_LEN] = keys.into();
    let data: SwapWithPartnerIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn swap_with_partner_ix(
    keys: SwapWithPartnerKeys,
    args: SwapWithPartnerIxArgs,
) -> std::io::Result<Instruction> {
    swap_with_partner_ix_with_program_id(CREMA_PROGRAM_ID, keys, args)
}
pub fn swap_with_partner_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SwapWithPartnerAccounts<'_, '_>,
    args: SwapWithPartnerIxArgs,
) -> ProgramResult {
    let keys: SwapWithPartnerKeys = accounts.into();
    let ix = swap_with_partner_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn swap_with_partner_invoke(
    accounts: SwapWithPartnerAccounts<'_, '_>,
    args: SwapWithPartnerIxArgs,
) -> ProgramResult {
    swap_with_partner_invoke_with_program_id(CREMA_PROGRAM_ID, accounts, args)
}
pub fn swap_with_partner_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SwapWithPartnerAccounts<'_, '_>,
    args: SwapWithPartnerIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SwapWithPartnerKeys = accounts.into();
    let ix = swap_with_partner_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn swap_with_partner_invoke_signed(
    accounts: SwapWithPartnerAccounts<'_, '_>,
    args: SwapWithPartnerIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    swap_with_partner_invoke_signed_with_program_id(
        CREMA_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn swap_with_partner_verify_account_keys(
    accounts: SwapWithPartnerAccounts<'_, '_>,
    keys: SwapWithPartnerKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.clmm_config.key, keys.clmm_config),
        (*accounts.clmmpool.key, keys.clmmpool),
        (*accounts.token_a.key, keys.token_a),
        (*accounts.token_b.key, keys.token_b),
        (*accounts.account_a.key, keys.account_a),
        (*accounts.account_b.key, keys.account_b),
        (*accounts.token_a_vault.key, keys.token_a_vault),
        (*accounts.token_b_vault.key, keys.token_b_vault),
        (*accounts.tick_array_map.key, keys.tick_array_map),
        (*accounts.owner.key, keys.owner),
        (*accounts.partner.key, keys.partner),
        (*accounts.partner_ata_a.key, keys.partner_ata_a),
        (*accounts.partner_ata_b.key, keys.partner_ata_b),
        (*accounts.token_program.key, keys.token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn swap_with_partner_verify_writable_privileges<'me, 'info>(
    accounts: SwapWithPartnerAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.clmmpool,
        accounts.account_a,
        accounts.account_b,
        accounts.token_a_vault,
        accounts.token_b_vault,
        accounts.tick_array_map,
        accounts.partner_ata_a,
        accounts.partner_ata_b,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn swap_with_partner_verify_signer_privileges<'me, 'info>(
    accounts: SwapWithPartnerAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.owner] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn swap_with_partner_verify_account_privileges<'me, 'info>(
    accounts: SwapWithPartnerAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    swap_with_partner_verify_writable_privileges(accounts)?;
    swap_with_partner_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const INITIALIZE_REWARDER_IX_ACCOUNTS_LEN: usize = 8;
#[derive(Copy, Clone, Debug)]
pub struct InitializeRewarderAccounts<'me, 'info> {
    pub payer: &'me AccountInfo<'info>,
    pub clmm_config: &'me AccountInfo<'info>,
    pub clmmpool: &'me AccountInfo<'info>,
    pub rewarder_authority: &'me AccountInfo<'info>,
    pub rewarder_token_mint: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub rent: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct InitializeRewarderKeys {
    pub payer: Pubkey,
    pub clmm_config: Pubkey,
    pub clmmpool: Pubkey,
    pub rewarder_authority: Pubkey,
    pub rewarder_token_mint: Pubkey,
    pub token_program: Pubkey,
    pub system_program: Pubkey,
    pub rent: Pubkey,
}
impl From<InitializeRewarderAccounts<'_, '_>> for InitializeRewarderKeys {
    fn from(accounts: InitializeRewarderAccounts) -> Self {
        Self {
            payer: *accounts.payer.key,
            clmm_config: *accounts.clmm_config.key,
            clmmpool: *accounts.clmmpool.key,
            rewarder_authority: *accounts.rewarder_authority.key,
            rewarder_token_mint: *accounts.rewarder_token_mint.key,
            token_program: *accounts.token_program.key,
            system_program: *accounts.system_program.key,
            rent: *accounts.rent.key,
        }
    }
}
impl From<InitializeRewarderKeys>
for [AccountMeta; INITIALIZE_REWARDER_IX_ACCOUNTS_LEN] {
    fn from(keys: InitializeRewarderKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.payer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.clmm_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.clmmpool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.rewarder_authority,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.rewarder_token_mint,
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
            AccountMeta {
                pubkey: keys.rent,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; INITIALIZE_REWARDER_IX_ACCOUNTS_LEN]> for InitializeRewarderKeys {
    fn from(pubkeys: [Pubkey; INITIALIZE_REWARDER_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            payer: pubkeys[0],
            clmm_config: pubkeys[1],
            clmmpool: pubkeys[2],
            rewarder_authority: pubkeys[3],
            rewarder_token_mint: pubkeys[4],
            token_program: pubkeys[5],
            system_program: pubkeys[6],
            rent: pubkeys[7],
        }
    }
}
impl<'info> From<InitializeRewarderAccounts<'_, 'info>>
for [AccountInfo<'info>; INITIALIZE_REWARDER_IX_ACCOUNTS_LEN] {
    fn from(accounts: InitializeRewarderAccounts<'_, 'info>) -> Self {
        [
            accounts.payer.clone(),
            accounts.clmm_config.clone(),
            accounts.clmmpool.clone(),
            accounts.rewarder_authority.clone(),
            accounts.rewarder_token_mint.clone(),
            accounts.token_program.clone(),
            accounts.system_program.clone(),
            accounts.rent.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; INITIALIZE_REWARDER_IX_ACCOUNTS_LEN]>
for InitializeRewarderAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; INITIALIZE_REWARDER_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            payer: &arr[0],
            clmm_config: &arr[1],
            clmmpool: &arr[2],
            rewarder_authority: &arr[3],
            rewarder_token_mint: &arr[4],
            token_program: &arr[5],
            system_program: &arr[6],
            rent: &arr[7],
        }
    }
}
pub const INITIALIZE_REWARDER_IX_DISCM: [u8; 8usize] = [
    98, 76, 144, 187, 240, 243, 35, 250,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct InitializeRewarderIxArgs {
    pub rewarder_index: u8,
    pub mint_wrapper: Pubkey,
    pub minter: Pubkey,
}
#[derive(Clone, Debug, PartialEq)]
pub struct InitializeRewarderIxData(pub InitializeRewarderIxArgs);
impl From<InitializeRewarderIxArgs> for InitializeRewarderIxData {
    fn from(args: InitializeRewarderIxArgs) -> Self {
        Self(args)
    }
}
impl InitializeRewarderIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INITIALIZE_REWARDER_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let rewarder_index: u8 = crate::borsh_de_or_default(&mut reader)?;
        let mint_wrapper: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let minter: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(InitializeRewarderIxArgs {
                rewarder_index,
                mint_wrapper,
                minter,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INITIALIZE_REWARDER_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.rewarder_index, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.mint_wrapper, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.minter, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn initialize_rewarder_ix_with_program_id(
    program_id: Pubkey,
    keys: InitializeRewarderKeys,
    args: InitializeRewarderIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; INITIALIZE_REWARDER_IX_ACCOUNTS_LEN] = keys.into();
    let data: InitializeRewarderIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn initialize_rewarder_ix(
    keys: InitializeRewarderKeys,
    args: InitializeRewarderIxArgs,
) -> std::io::Result<Instruction> {
    initialize_rewarder_ix_with_program_id(CREMA_PROGRAM_ID, keys, args)
}
pub fn initialize_rewarder_invoke_with_program_id(
    program_id: Pubkey,
    accounts: InitializeRewarderAccounts<'_, '_>,
    args: InitializeRewarderIxArgs,
) -> ProgramResult {
    let keys: InitializeRewarderKeys = accounts.into();
    let ix = initialize_rewarder_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn initialize_rewarder_invoke(
    accounts: InitializeRewarderAccounts<'_, '_>,
    args: InitializeRewarderIxArgs,
) -> ProgramResult {
    initialize_rewarder_invoke_with_program_id(CREMA_PROGRAM_ID, accounts, args)
}
pub fn initialize_rewarder_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: InitializeRewarderAccounts<'_, '_>,
    args: InitializeRewarderIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: InitializeRewarderKeys = accounts.into();
    let ix = initialize_rewarder_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn initialize_rewarder_invoke_signed(
    accounts: InitializeRewarderAccounts<'_, '_>,
    args: InitializeRewarderIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    initialize_rewarder_invoke_signed_with_program_id(
        CREMA_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn initialize_rewarder_verify_account_keys(
    accounts: InitializeRewarderAccounts<'_, '_>,
    keys: InitializeRewarderKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.payer.key, keys.payer),
        (*accounts.clmm_config.key, keys.clmm_config),
        (*accounts.clmmpool.key, keys.clmmpool),
        (*accounts.rewarder_authority.key, keys.rewarder_authority),
        (*accounts.rewarder_token_mint.key, keys.rewarder_token_mint),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.rent.key, keys.rent),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn initialize_rewarder_verify_writable_privileges<'me, 'info>(
    accounts: InitializeRewarderAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.payer, accounts.clmmpool] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn initialize_rewarder_verify_signer_privileges<'me, 'info>(
    accounts: InitializeRewarderAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.payer, accounts.rewarder_authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn initialize_rewarder_verify_account_privileges<'me, 'info>(
    accounts: InitializeRewarderAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    initialize_rewarder_verify_writable_privileges(accounts)?;
    initialize_rewarder_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const UPDATE_REWARDER_EMISSION_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct UpdateRewarderEmissionAccounts<'me, 'info> {
    pub rewarder_authority: &'me AccountInfo<'info>,
    pub clmm_config: &'me AccountInfo<'info>,
    pub clmmpool: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UpdateRewarderEmissionKeys {
    pub rewarder_authority: Pubkey,
    pub clmm_config: Pubkey,
    pub clmmpool: Pubkey,
}
impl From<UpdateRewarderEmissionAccounts<'_, '_>> for UpdateRewarderEmissionKeys {
    fn from(accounts: UpdateRewarderEmissionAccounts) -> Self {
        Self {
            rewarder_authority: *accounts.rewarder_authority.key,
            clmm_config: *accounts.clmm_config.key,
            clmmpool: *accounts.clmmpool.key,
        }
    }
}
impl From<UpdateRewarderEmissionKeys>
for [AccountMeta; UPDATE_REWARDER_EMISSION_IX_ACCOUNTS_LEN] {
    fn from(keys: UpdateRewarderEmissionKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.rewarder_authority,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.clmm_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.clmmpool,
                is_signer: false,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; UPDATE_REWARDER_EMISSION_IX_ACCOUNTS_LEN]>
for UpdateRewarderEmissionKeys {
    fn from(pubkeys: [Pubkey; UPDATE_REWARDER_EMISSION_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            rewarder_authority: pubkeys[0],
            clmm_config: pubkeys[1],
            clmmpool: pubkeys[2],
        }
    }
}
impl<'info> From<UpdateRewarderEmissionAccounts<'_, 'info>>
for [AccountInfo<'info>; UPDATE_REWARDER_EMISSION_IX_ACCOUNTS_LEN] {
    fn from(accounts: UpdateRewarderEmissionAccounts<'_, 'info>) -> Self {
        [
            accounts.rewarder_authority.clone(),
            accounts.clmm_config.clone(),
            accounts.clmmpool.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; UPDATE_REWARDER_EMISSION_IX_ACCOUNTS_LEN]>
for UpdateRewarderEmissionAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; UPDATE_REWARDER_EMISSION_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            rewarder_authority: &arr[0],
            clmm_config: &arr[1],
            clmmpool: &arr[2],
        }
    }
}
pub const UPDATE_REWARDER_EMISSION_IX_DISCM: [u8; 8usize] = [
    3, 175, 121, 96, 176, 184, 204, 7,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UpdateRewarderEmissionIxArgs {
    pub rewarder_index: u8,
    pub emissions_per_second: u128,
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateRewarderEmissionIxData(pub UpdateRewarderEmissionIxArgs);
impl From<UpdateRewarderEmissionIxArgs> for UpdateRewarderEmissionIxData {
    fn from(args: UpdateRewarderEmissionIxArgs) -> Self {
        Self(args)
    }
}
impl UpdateRewarderEmissionIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_REWARDER_EMISSION_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let rewarder_index: u8 = crate::borsh_de_or_default(&mut reader)?;
        let emissions_per_second: u128 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(UpdateRewarderEmissionIxArgs {
                rewarder_index,
                emissions_per_second,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_REWARDER_EMISSION_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.rewarder_index, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.emissions_per_second, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn update_rewarder_emission_ix_with_program_id(
    program_id: Pubkey,
    keys: UpdateRewarderEmissionKeys,
    args: UpdateRewarderEmissionIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; UPDATE_REWARDER_EMISSION_IX_ACCOUNTS_LEN] = keys.into();
    let data: UpdateRewarderEmissionIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_rewarder_emission_ix(
    keys: UpdateRewarderEmissionKeys,
    args: UpdateRewarderEmissionIxArgs,
) -> std::io::Result<Instruction> {
    update_rewarder_emission_ix_with_program_id(CREMA_PROGRAM_ID, keys, args)
}
pub fn update_rewarder_emission_invoke_with_program_id(
    program_id: Pubkey,
    accounts: UpdateRewarderEmissionAccounts<'_, '_>,
    args: UpdateRewarderEmissionIxArgs,
) -> ProgramResult {
    let keys: UpdateRewarderEmissionKeys = accounts.into();
    let ix = update_rewarder_emission_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn update_rewarder_emission_invoke(
    accounts: UpdateRewarderEmissionAccounts<'_, '_>,
    args: UpdateRewarderEmissionIxArgs,
) -> ProgramResult {
    update_rewarder_emission_invoke_with_program_id(CREMA_PROGRAM_ID, accounts, args)
}
pub fn update_rewarder_emission_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: UpdateRewarderEmissionAccounts<'_, '_>,
    args: UpdateRewarderEmissionIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: UpdateRewarderEmissionKeys = accounts.into();
    let ix = update_rewarder_emission_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn update_rewarder_emission_invoke_signed(
    accounts: UpdateRewarderEmissionAccounts<'_, '_>,
    args: UpdateRewarderEmissionIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    update_rewarder_emission_invoke_signed_with_program_id(
        CREMA_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn update_rewarder_emission_verify_account_keys(
    accounts: UpdateRewarderEmissionAccounts<'_, '_>,
    keys: UpdateRewarderEmissionKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.rewarder_authority.key, keys.rewarder_authority),
        (*accounts.clmm_config.key, keys.clmm_config),
        (*accounts.clmmpool.key, keys.clmmpool),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn update_rewarder_emission_verify_writable_privileges<'me, 'info>(
    accounts: UpdateRewarderEmissionAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.clmmpool] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn update_rewarder_emission_verify_signer_privileges<'me, 'info>(
    accounts: UpdateRewarderEmissionAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.rewarder_authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn update_rewarder_emission_verify_account_privileges<'me, 'info>(
    accounts: UpdateRewarderEmissionAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    update_rewarder_emission_verify_writable_privileges(accounts)?;
    update_rewarder_emission_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const COLLECT_REWARDER_IX_ACCOUNTS_LEN: usize = 12;
#[derive(Copy, Clone, Debug)]
pub struct CollectRewarderAccounts<'me, 'info> {
    pub owner: &'me AccountInfo<'info>,
    pub clmmpool: &'me AccountInfo<'info>,
    pub position: &'me AccountInfo<'info>,
    pub position_ata: &'me AccountInfo<'info>,
    pub rewarder_ata: &'me AccountInfo<'info>,
    pub mint_wrapper: &'me AccountInfo<'info>,
    pub minter: &'me AccountInfo<'info>,
    pub mint_wrapper_program: &'me AccountInfo<'info>,
    pub rewards_token_mint: &'me AccountInfo<'info>,
    pub tick_array_lower: &'me AccountInfo<'info>,
    pub tick_array_upper: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CollectRewarderKeys {
    pub owner: Pubkey,
    pub clmmpool: Pubkey,
    pub position: Pubkey,
    pub position_ata: Pubkey,
    pub rewarder_ata: Pubkey,
    pub mint_wrapper: Pubkey,
    pub minter: Pubkey,
    pub mint_wrapper_program: Pubkey,
    pub rewards_token_mint: Pubkey,
    pub tick_array_lower: Pubkey,
    pub tick_array_upper: Pubkey,
    pub token_program: Pubkey,
}
impl From<CollectRewarderAccounts<'_, '_>> for CollectRewarderKeys {
    fn from(accounts: CollectRewarderAccounts) -> Self {
        Self {
            owner: *accounts.owner.key,
            clmmpool: *accounts.clmmpool.key,
            position: *accounts.position.key,
            position_ata: *accounts.position_ata.key,
            rewarder_ata: *accounts.rewarder_ata.key,
            mint_wrapper: *accounts.mint_wrapper.key,
            minter: *accounts.minter.key,
            mint_wrapper_program: *accounts.mint_wrapper_program.key,
            rewards_token_mint: *accounts.rewards_token_mint.key,
            tick_array_lower: *accounts.tick_array_lower.key,
            tick_array_upper: *accounts.tick_array_upper.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<CollectRewarderKeys> for [AccountMeta; COLLECT_REWARDER_IX_ACCOUNTS_LEN] {
    fn from(keys: CollectRewarderKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.owner,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.clmmpool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.position,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.position_ata,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.rewarder_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.mint_wrapper,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.minter,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.mint_wrapper_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.rewards_token_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.tick_array_lower,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.tick_array_upper,
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
impl From<[Pubkey; COLLECT_REWARDER_IX_ACCOUNTS_LEN]> for CollectRewarderKeys {
    fn from(pubkeys: [Pubkey; COLLECT_REWARDER_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            owner: pubkeys[0],
            clmmpool: pubkeys[1],
            position: pubkeys[2],
            position_ata: pubkeys[3],
            rewarder_ata: pubkeys[4],
            mint_wrapper: pubkeys[5],
            minter: pubkeys[6],
            mint_wrapper_program: pubkeys[7],
            rewards_token_mint: pubkeys[8],
            tick_array_lower: pubkeys[9],
            tick_array_upper: pubkeys[10],
            token_program: pubkeys[11],
        }
    }
}
impl<'info> From<CollectRewarderAccounts<'_, 'info>>
for [AccountInfo<'info>; COLLECT_REWARDER_IX_ACCOUNTS_LEN] {
    fn from(accounts: CollectRewarderAccounts<'_, 'info>) -> Self {
        [
            accounts.owner.clone(),
            accounts.clmmpool.clone(),
            accounts.position.clone(),
            accounts.position_ata.clone(),
            accounts.rewarder_ata.clone(),
            accounts.mint_wrapper.clone(),
            accounts.minter.clone(),
            accounts.mint_wrapper_program.clone(),
            accounts.rewards_token_mint.clone(),
            accounts.tick_array_lower.clone(),
            accounts.tick_array_upper.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; COLLECT_REWARDER_IX_ACCOUNTS_LEN]>
for CollectRewarderAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; COLLECT_REWARDER_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            owner: &arr[0],
            clmmpool: &arr[1],
            position: &arr[2],
            position_ata: &arr[3],
            rewarder_ata: &arr[4],
            mint_wrapper: &arr[5],
            minter: &arr[6],
            mint_wrapper_program: &arr[7],
            rewards_token_mint: &arr[8],
            tick_array_lower: &arr[9],
            tick_array_upper: &arr[10],
            token_program: &arr[11],
        }
    }
}
pub const COLLECT_REWARDER_IX_DISCM: [u8; 8usize] = [255, 139, 69, 35, 166, 41, 94, 90];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CollectRewarderIxArgs {
    pub rewarder_index: u8,
}
#[derive(Clone, Debug, PartialEq)]
pub struct CollectRewarderIxData(pub CollectRewarderIxArgs);
impl From<CollectRewarderIxArgs> for CollectRewarderIxData {
    fn from(args: CollectRewarderIxArgs) -> Self {
        Self(args)
    }
}
impl CollectRewarderIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != COLLECT_REWARDER_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let rewarder_index: u8 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(CollectRewarderIxArgs {
                rewarder_index,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&COLLECT_REWARDER_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.rewarder_index, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn collect_rewarder_ix_with_program_id(
    program_id: Pubkey,
    keys: CollectRewarderKeys,
    args: CollectRewarderIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; COLLECT_REWARDER_IX_ACCOUNTS_LEN] = keys.into();
    let data: CollectRewarderIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn collect_rewarder_ix(
    keys: CollectRewarderKeys,
    args: CollectRewarderIxArgs,
) -> std::io::Result<Instruction> {
    collect_rewarder_ix_with_program_id(CREMA_PROGRAM_ID, keys, args)
}
pub fn collect_rewarder_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CollectRewarderAccounts<'_, '_>,
    args: CollectRewarderIxArgs,
) -> ProgramResult {
    let keys: CollectRewarderKeys = accounts.into();
    let ix = collect_rewarder_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn collect_rewarder_invoke(
    accounts: CollectRewarderAccounts<'_, '_>,
    args: CollectRewarderIxArgs,
) -> ProgramResult {
    collect_rewarder_invoke_with_program_id(CREMA_PROGRAM_ID, accounts, args)
}
pub fn collect_rewarder_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CollectRewarderAccounts<'_, '_>,
    args: CollectRewarderIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CollectRewarderKeys = accounts.into();
    let ix = collect_rewarder_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn collect_rewarder_invoke_signed(
    accounts: CollectRewarderAccounts<'_, '_>,
    args: CollectRewarderIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    collect_rewarder_invoke_signed_with_program_id(
        CREMA_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn collect_rewarder_verify_account_keys(
    accounts: CollectRewarderAccounts<'_, '_>,
    keys: CollectRewarderKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.owner.key, keys.owner),
        (*accounts.clmmpool.key, keys.clmmpool),
        (*accounts.position.key, keys.position),
        (*accounts.position_ata.key, keys.position_ata),
        (*accounts.rewarder_ata.key, keys.rewarder_ata),
        (*accounts.mint_wrapper.key, keys.mint_wrapper),
        (*accounts.minter.key, keys.minter),
        (*accounts.mint_wrapper_program.key, keys.mint_wrapper_program),
        (*accounts.rewards_token_mint.key, keys.rewards_token_mint),
        (*accounts.tick_array_lower.key, keys.tick_array_lower),
        (*accounts.tick_array_upper.key, keys.tick_array_upper),
        (*accounts.token_program.key, keys.token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn collect_rewarder_verify_writable_privileges<'me, 'info>(
    accounts: CollectRewarderAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.clmmpool,
        accounts.position,
        accounts.rewarder_ata,
        accounts.mint_wrapper,
        accounts.minter,
        accounts.rewards_token_mint,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn collect_rewarder_verify_signer_privileges<'me, 'info>(
    accounts: CollectRewarderAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.owner] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn collect_rewarder_verify_account_privileges<'me, 'info>(
    accounts: CollectRewarderAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    collect_rewarder_verify_writable_privileges(accounts)?;
    collect_rewarder_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const TRANSFER_PARTNER_CLAIM_AUTHORITY_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct TransferPartnerClaimAuthorityAccounts<'me, 'info> {
    pub partner_claim_authority: &'me AccountInfo<'info>,
    pub partner: &'me AccountInfo<'info>,
    pub new_authority: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct TransferPartnerClaimAuthorityKeys {
    pub partner_claim_authority: Pubkey,
    pub partner: Pubkey,
    pub new_authority: Pubkey,
}
impl From<TransferPartnerClaimAuthorityAccounts<'_, '_>>
for TransferPartnerClaimAuthorityKeys {
    fn from(accounts: TransferPartnerClaimAuthorityAccounts) -> Self {
        Self {
            partner_claim_authority: *accounts.partner_claim_authority.key,
            partner: *accounts.partner.key,
            new_authority: *accounts.new_authority.key,
        }
    }
}
impl From<TransferPartnerClaimAuthorityKeys>
for [AccountMeta; TRANSFER_PARTNER_CLAIM_AUTHORITY_IX_ACCOUNTS_LEN] {
    fn from(keys: TransferPartnerClaimAuthorityKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.partner_claim_authority,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.partner,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.new_authority,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; TRANSFER_PARTNER_CLAIM_AUTHORITY_IX_ACCOUNTS_LEN]>
for TransferPartnerClaimAuthorityKeys {
    fn from(
        pubkeys: [Pubkey; TRANSFER_PARTNER_CLAIM_AUTHORITY_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            partner_claim_authority: pubkeys[0],
            partner: pubkeys[1],
            new_authority: pubkeys[2],
        }
    }
}
impl<'info> From<TransferPartnerClaimAuthorityAccounts<'_, 'info>>
for [AccountInfo<'info>; TRANSFER_PARTNER_CLAIM_AUTHORITY_IX_ACCOUNTS_LEN] {
    fn from(accounts: TransferPartnerClaimAuthorityAccounts<'_, 'info>) -> Self {
        [
            accounts.partner_claim_authority.clone(),
            accounts.partner.clone(),
            accounts.new_authority.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; TRANSFER_PARTNER_CLAIM_AUTHORITY_IX_ACCOUNTS_LEN]>
for TransferPartnerClaimAuthorityAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; TRANSFER_PARTNER_CLAIM_AUTHORITY_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            partner_claim_authority: &arr[0],
            partner: &arr[1],
            new_authority: &arr[2],
        }
    }
}
pub const TRANSFER_PARTNER_CLAIM_AUTHORITY_IX_DISCM: [u8; 8usize] = [
    223, 194, 192, 161, 152, 161, 84, 15,
];
#[derive(Clone, Debug, PartialEq)]
pub struct TransferPartnerClaimAuthorityIxData;
impl TransferPartnerClaimAuthorityIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != TRANSFER_PARTNER_CLAIM_AUTHORITY_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&TRANSFER_PARTNER_CLAIM_AUTHORITY_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn transfer_partner_claim_authority_ix_with_program_id(
    program_id: Pubkey,
    keys: TransferPartnerClaimAuthorityKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; TRANSFER_PARTNER_CLAIM_AUTHORITY_IX_ACCOUNTS_LEN] = keys
        .into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: TransferPartnerClaimAuthorityIxData.try_to_vec()?,
    })
}
pub fn transfer_partner_claim_authority_ix(
    keys: TransferPartnerClaimAuthorityKeys,
) -> std::io::Result<Instruction> {
    transfer_partner_claim_authority_ix_with_program_id(CREMA_PROGRAM_ID, keys)
}
pub fn transfer_partner_claim_authority_invoke_with_program_id(
    program_id: Pubkey,
    accounts: TransferPartnerClaimAuthorityAccounts<'_, '_>,
) -> ProgramResult {
    let keys: TransferPartnerClaimAuthorityKeys = accounts.into();
    let ix = transfer_partner_claim_authority_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn transfer_partner_claim_authority_invoke(
    accounts: TransferPartnerClaimAuthorityAccounts<'_, '_>,
) -> ProgramResult {
    transfer_partner_claim_authority_invoke_with_program_id(CREMA_PROGRAM_ID, accounts)
}
pub fn transfer_partner_claim_authority_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: TransferPartnerClaimAuthorityAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: TransferPartnerClaimAuthorityKeys = accounts.into();
    let ix = transfer_partner_claim_authority_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn transfer_partner_claim_authority_invoke_signed(
    accounts: TransferPartnerClaimAuthorityAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    transfer_partner_claim_authority_invoke_signed_with_program_id(
        CREMA_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn transfer_partner_claim_authority_verify_account_keys(
    accounts: TransferPartnerClaimAuthorityAccounts<'_, '_>,
    keys: TransferPartnerClaimAuthorityKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.partner_claim_authority.key, keys.partner_claim_authority),
        (*accounts.partner.key, keys.partner),
        (*accounts.new_authority.key, keys.new_authority),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn transfer_partner_claim_authority_verify_writable_privileges<'me, 'info>(
    accounts: TransferPartnerClaimAuthorityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.partner] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn transfer_partner_claim_authority_verify_signer_privileges<'me, 'info>(
    accounts: TransferPartnerClaimAuthorityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.partner_claim_authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn transfer_partner_claim_authority_verify_account_privileges<'me, 'info>(
    accounts: TransferPartnerClaimAuthorityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    transfer_partner_claim_authority_verify_writable_privileges(accounts)?;
    transfer_partner_claim_authority_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const ACCEPT_PARTNER_CLAIM_AUTHORITY_IX_ACCOUNTS_LEN: usize = 2;
#[derive(Copy, Clone, Debug)]
pub struct AcceptPartnerClaimAuthorityAccounts<'me, 'info> {
    pub new_authority: &'me AccountInfo<'info>,
    pub partner: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct AcceptPartnerClaimAuthorityKeys {
    pub new_authority: Pubkey,
    pub partner: Pubkey,
}
impl From<AcceptPartnerClaimAuthorityAccounts<'_, '_>>
for AcceptPartnerClaimAuthorityKeys {
    fn from(accounts: AcceptPartnerClaimAuthorityAccounts) -> Self {
        Self {
            new_authority: *accounts.new_authority.key,
            partner: *accounts.partner.key,
        }
    }
}
impl From<AcceptPartnerClaimAuthorityKeys>
for [AccountMeta; ACCEPT_PARTNER_CLAIM_AUTHORITY_IX_ACCOUNTS_LEN] {
    fn from(keys: AcceptPartnerClaimAuthorityKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.new_authority,
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
impl From<[Pubkey; ACCEPT_PARTNER_CLAIM_AUTHORITY_IX_ACCOUNTS_LEN]>
for AcceptPartnerClaimAuthorityKeys {
    fn from(pubkeys: [Pubkey; ACCEPT_PARTNER_CLAIM_AUTHORITY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            new_authority: pubkeys[0],
            partner: pubkeys[1],
        }
    }
}
impl<'info> From<AcceptPartnerClaimAuthorityAccounts<'_, 'info>>
for [AccountInfo<'info>; ACCEPT_PARTNER_CLAIM_AUTHORITY_IX_ACCOUNTS_LEN] {
    fn from(accounts: AcceptPartnerClaimAuthorityAccounts<'_, 'info>) -> Self {
        [accounts.new_authority.clone(), accounts.partner.clone()]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; ACCEPT_PARTNER_CLAIM_AUTHORITY_IX_ACCOUNTS_LEN]>
for AcceptPartnerClaimAuthorityAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; ACCEPT_PARTNER_CLAIM_AUTHORITY_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            new_authority: &arr[0],
            partner: &arr[1],
        }
    }
}
pub const ACCEPT_PARTNER_CLAIM_AUTHORITY_IX_DISCM: [u8; 8usize] = [
    208, 241, 224, 132, 101, 56, 35, 3,
];
#[derive(Clone, Debug, PartialEq)]
pub struct AcceptPartnerClaimAuthorityIxData;
impl AcceptPartnerClaimAuthorityIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != ACCEPT_PARTNER_CLAIM_AUTHORITY_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&ACCEPT_PARTNER_CLAIM_AUTHORITY_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn accept_partner_claim_authority_ix_with_program_id(
    program_id: Pubkey,
    keys: AcceptPartnerClaimAuthorityKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; ACCEPT_PARTNER_CLAIM_AUTHORITY_IX_ACCOUNTS_LEN] = keys
        .into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: AcceptPartnerClaimAuthorityIxData.try_to_vec()?,
    })
}
pub fn accept_partner_claim_authority_ix(
    keys: AcceptPartnerClaimAuthorityKeys,
) -> std::io::Result<Instruction> {
    accept_partner_claim_authority_ix_with_program_id(CREMA_PROGRAM_ID, keys)
}
pub fn accept_partner_claim_authority_invoke_with_program_id(
    program_id: Pubkey,
    accounts: AcceptPartnerClaimAuthorityAccounts<'_, '_>,
) -> ProgramResult {
    let keys: AcceptPartnerClaimAuthorityKeys = accounts.into();
    let ix = accept_partner_claim_authority_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn accept_partner_claim_authority_invoke(
    accounts: AcceptPartnerClaimAuthorityAccounts<'_, '_>,
) -> ProgramResult {
    accept_partner_claim_authority_invoke_with_program_id(CREMA_PROGRAM_ID, accounts)
}
pub fn accept_partner_claim_authority_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: AcceptPartnerClaimAuthorityAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: AcceptPartnerClaimAuthorityKeys = accounts.into();
    let ix = accept_partner_claim_authority_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn accept_partner_claim_authority_invoke_signed(
    accounts: AcceptPartnerClaimAuthorityAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    accept_partner_claim_authority_invoke_signed_with_program_id(
        CREMA_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn accept_partner_claim_authority_verify_account_keys(
    accounts: AcceptPartnerClaimAuthorityAccounts<'_, '_>,
    keys: AcceptPartnerClaimAuthorityKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.new_authority.key, keys.new_authority),
        (*accounts.partner.key, keys.partner),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn accept_partner_claim_authority_verify_writable_privileges<'me, 'info>(
    accounts: AcceptPartnerClaimAuthorityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.partner] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn accept_partner_claim_authority_verify_signer_privileges<'me, 'info>(
    accounts: AcceptPartnerClaimAuthorityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.new_authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn accept_partner_claim_authority_verify_account_privileges<'me, 'info>(
    accounts: AcceptPartnerClaimAuthorityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    accept_partner_claim_authority_verify_writable_privileges(accounts)?;
    accept_partner_claim_authority_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const PAUSE_CLMMPOOL_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct PauseClmmpoolAccounts<'me, 'info> {
    pub clmm_config: &'me AccountInfo<'info>,
    pub protocol_authority: &'me AccountInfo<'info>,
    pub clmmpool: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct PauseClmmpoolKeys {
    pub clmm_config: Pubkey,
    pub protocol_authority: Pubkey,
    pub clmmpool: Pubkey,
}
impl From<PauseClmmpoolAccounts<'_, '_>> for PauseClmmpoolKeys {
    fn from(accounts: PauseClmmpoolAccounts) -> Self {
        Self {
            clmm_config: *accounts.clmm_config.key,
            protocol_authority: *accounts.protocol_authority.key,
            clmmpool: *accounts.clmmpool.key,
        }
    }
}
impl From<PauseClmmpoolKeys> for [AccountMeta; PAUSE_CLMMPOOL_IX_ACCOUNTS_LEN] {
    fn from(keys: PauseClmmpoolKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.clmm_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.protocol_authority,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.clmmpool,
                is_signer: false,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; PAUSE_CLMMPOOL_IX_ACCOUNTS_LEN]> for PauseClmmpoolKeys {
    fn from(pubkeys: [Pubkey; PAUSE_CLMMPOOL_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            clmm_config: pubkeys[0],
            protocol_authority: pubkeys[1],
            clmmpool: pubkeys[2],
        }
    }
}
impl<'info> From<PauseClmmpoolAccounts<'_, 'info>>
for [AccountInfo<'info>; PAUSE_CLMMPOOL_IX_ACCOUNTS_LEN] {
    fn from(accounts: PauseClmmpoolAccounts<'_, 'info>) -> Self {
        [
            accounts.clmm_config.clone(),
            accounts.protocol_authority.clone(),
            accounts.clmmpool.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; PAUSE_CLMMPOOL_IX_ACCOUNTS_LEN]>
for PauseClmmpoolAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; PAUSE_CLMMPOOL_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            clmm_config: &arr[0],
            protocol_authority: &arr[1],
            clmmpool: &arr[2],
        }
    }
}
pub const PAUSE_CLMMPOOL_IX_DISCM: [u8; 8usize] = [82, 251, 68, 4, 104, 122, 119, 234];
#[derive(Clone, Debug, PartialEq)]
pub struct PauseClmmpoolIxData;
impl PauseClmmpoolIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != PAUSE_CLMMPOOL_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&PAUSE_CLMMPOOL_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn pause_clmmpool_ix_with_program_id(
    program_id: Pubkey,
    keys: PauseClmmpoolKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; PAUSE_CLMMPOOL_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: PauseClmmpoolIxData.try_to_vec()?,
    })
}
pub fn pause_clmmpool_ix(keys: PauseClmmpoolKeys) -> std::io::Result<Instruction> {
    pause_clmmpool_ix_with_program_id(CREMA_PROGRAM_ID, keys)
}
pub fn pause_clmmpool_invoke_with_program_id(
    program_id: Pubkey,
    accounts: PauseClmmpoolAccounts<'_, '_>,
) -> ProgramResult {
    let keys: PauseClmmpoolKeys = accounts.into();
    let ix = pause_clmmpool_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn pause_clmmpool_invoke(accounts: PauseClmmpoolAccounts<'_, '_>) -> ProgramResult {
    pause_clmmpool_invoke_with_program_id(CREMA_PROGRAM_ID, accounts)
}
pub fn pause_clmmpool_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: PauseClmmpoolAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: PauseClmmpoolKeys = accounts.into();
    let ix = pause_clmmpool_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn pause_clmmpool_invoke_signed(
    accounts: PauseClmmpoolAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    pause_clmmpool_invoke_signed_with_program_id(CREMA_PROGRAM_ID, accounts, seeds)
}
pub fn pause_clmmpool_verify_account_keys(
    accounts: PauseClmmpoolAccounts<'_, '_>,
    keys: PauseClmmpoolKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.clmm_config.key, keys.clmm_config),
        (*accounts.protocol_authority.key, keys.protocol_authority),
        (*accounts.clmmpool.key, keys.clmmpool),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn pause_clmmpool_verify_writable_privileges<'me, 'info>(
    accounts: PauseClmmpoolAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.clmmpool] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn pause_clmmpool_verify_signer_privileges<'me, 'info>(
    accounts: PauseClmmpoolAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.protocol_authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn pause_clmmpool_verify_account_privileges<'me, 'info>(
    accounts: PauseClmmpoolAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    pause_clmmpool_verify_writable_privileges(accounts)?;
    pause_clmmpool_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const UNPAUSE_CLMMPOOL_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct UnpauseClmmpoolAccounts<'me, 'info> {
    pub clmm_config: &'me AccountInfo<'info>,
    pub protocol_authority: &'me AccountInfo<'info>,
    pub clmmpool: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UnpauseClmmpoolKeys {
    pub clmm_config: Pubkey,
    pub protocol_authority: Pubkey,
    pub clmmpool: Pubkey,
}
impl From<UnpauseClmmpoolAccounts<'_, '_>> for UnpauseClmmpoolKeys {
    fn from(accounts: UnpauseClmmpoolAccounts) -> Self {
        Self {
            clmm_config: *accounts.clmm_config.key,
            protocol_authority: *accounts.protocol_authority.key,
            clmmpool: *accounts.clmmpool.key,
        }
    }
}
impl From<UnpauseClmmpoolKeys> for [AccountMeta; UNPAUSE_CLMMPOOL_IX_ACCOUNTS_LEN] {
    fn from(keys: UnpauseClmmpoolKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.clmm_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.protocol_authority,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.clmmpool,
                is_signer: false,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; UNPAUSE_CLMMPOOL_IX_ACCOUNTS_LEN]> for UnpauseClmmpoolKeys {
    fn from(pubkeys: [Pubkey; UNPAUSE_CLMMPOOL_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            clmm_config: pubkeys[0],
            protocol_authority: pubkeys[1],
            clmmpool: pubkeys[2],
        }
    }
}
impl<'info> From<UnpauseClmmpoolAccounts<'_, 'info>>
for [AccountInfo<'info>; UNPAUSE_CLMMPOOL_IX_ACCOUNTS_LEN] {
    fn from(accounts: UnpauseClmmpoolAccounts<'_, 'info>) -> Self {
        [
            accounts.clmm_config.clone(),
            accounts.protocol_authority.clone(),
            accounts.clmmpool.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; UNPAUSE_CLMMPOOL_IX_ACCOUNTS_LEN]>
for UnpauseClmmpoolAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; UNPAUSE_CLMMPOOL_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            clmm_config: &arr[0],
            protocol_authority: &arr[1],
            clmmpool: &arr[2],
        }
    }
}
pub const UNPAUSE_CLMMPOOL_IX_DISCM: [u8; 8usize] = [153, 20, 95, 128, 159, 250, 244, 2];
#[derive(Clone, Debug, PartialEq)]
pub struct UnpauseClmmpoolIxData;
impl UnpauseClmmpoolIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UNPAUSE_CLMMPOOL_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UNPAUSE_CLMMPOOL_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn unpause_clmmpool_ix_with_program_id(
    program_id: Pubkey,
    keys: UnpauseClmmpoolKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; UNPAUSE_CLMMPOOL_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: UnpauseClmmpoolIxData.try_to_vec()?,
    })
}
pub fn unpause_clmmpool_ix(keys: UnpauseClmmpoolKeys) -> std::io::Result<Instruction> {
    unpause_clmmpool_ix_with_program_id(CREMA_PROGRAM_ID, keys)
}
pub fn unpause_clmmpool_invoke_with_program_id(
    program_id: Pubkey,
    accounts: UnpauseClmmpoolAccounts<'_, '_>,
) -> ProgramResult {
    let keys: UnpauseClmmpoolKeys = accounts.into();
    let ix = unpause_clmmpool_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn unpause_clmmpool_invoke(
    accounts: UnpauseClmmpoolAccounts<'_, '_>,
) -> ProgramResult {
    unpause_clmmpool_invoke_with_program_id(CREMA_PROGRAM_ID, accounts)
}
pub fn unpause_clmmpool_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: UnpauseClmmpoolAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: UnpauseClmmpoolKeys = accounts.into();
    let ix = unpause_clmmpool_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn unpause_clmmpool_invoke_signed(
    accounts: UnpauseClmmpoolAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    unpause_clmmpool_invoke_signed_with_program_id(CREMA_PROGRAM_ID, accounts, seeds)
}
pub fn unpause_clmmpool_verify_account_keys(
    accounts: UnpauseClmmpoolAccounts<'_, '_>,
    keys: UnpauseClmmpoolKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.clmm_config.key, keys.clmm_config),
        (*accounts.protocol_authority.key, keys.protocol_authority),
        (*accounts.clmmpool.key, keys.clmmpool),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn unpause_clmmpool_verify_writable_privileges<'me, 'info>(
    accounts: UnpauseClmmpoolAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.clmmpool] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn unpause_clmmpool_verify_signer_privileges<'me, 'info>(
    accounts: UnpauseClmmpoolAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.protocol_authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn unpause_clmmpool_verify_account_privileges<'me, 'info>(
    accounts: UnpauseClmmpoolAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    unpause_clmmpool_verify_writable_privileges(accounts)?;
    unpause_clmmpool_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CREATE_CLMMPOOL_METADATA_IX_ACCOUNTS_LEN: usize = 7;
#[derive(Copy, Clone, Debug)]
pub struct CreateClmmpoolMetadataAccounts<'me, 'info> {
    pub payer: &'me AccountInfo<'info>,
    pub clmm_config: &'me AccountInfo<'info>,
    pub clmmpool: &'me AccountInfo<'info>,
    pub clmmpool_metadata: &'me AccountInfo<'info>,
    pub protocol_authority: &'me AccountInfo<'info>,
    pub rent: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CreateClmmpoolMetadataKeys {
    pub payer: Pubkey,
    pub clmm_config: Pubkey,
    pub clmmpool: Pubkey,
    pub clmmpool_metadata: Pubkey,
    pub protocol_authority: Pubkey,
    pub rent: Pubkey,
    pub system_program: Pubkey,
}
impl From<CreateClmmpoolMetadataAccounts<'_, '_>> for CreateClmmpoolMetadataKeys {
    fn from(accounts: CreateClmmpoolMetadataAccounts) -> Self {
        Self {
            payer: *accounts.payer.key,
            clmm_config: *accounts.clmm_config.key,
            clmmpool: *accounts.clmmpool.key,
            clmmpool_metadata: *accounts.clmmpool_metadata.key,
            protocol_authority: *accounts.protocol_authority.key,
            rent: *accounts.rent.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<CreateClmmpoolMetadataKeys>
for [AccountMeta; CREATE_CLMMPOOL_METADATA_IX_ACCOUNTS_LEN] {
    fn from(keys: CreateClmmpoolMetadataKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.payer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.clmm_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.clmmpool,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.clmmpool_metadata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.protocol_authority,
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
impl From<[Pubkey; CREATE_CLMMPOOL_METADATA_IX_ACCOUNTS_LEN]>
for CreateClmmpoolMetadataKeys {
    fn from(pubkeys: [Pubkey; CREATE_CLMMPOOL_METADATA_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            payer: pubkeys[0],
            clmm_config: pubkeys[1],
            clmmpool: pubkeys[2],
            clmmpool_metadata: pubkeys[3],
            protocol_authority: pubkeys[4],
            rent: pubkeys[5],
            system_program: pubkeys[6],
        }
    }
}
impl<'info> From<CreateClmmpoolMetadataAccounts<'_, 'info>>
for [AccountInfo<'info>; CREATE_CLMMPOOL_METADATA_IX_ACCOUNTS_LEN] {
    fn from(accounts: CreateClmmpoolMetadataAccounts<'_, 'info>) -> Self {
        [
            accounts.payer.clone(),
            accounts.clmm_config.clone(),
            accounts.clmmpool.clone(),
            accounts.clmmpool_metadata.clone(),
            accounts.protocol_authority.clone(),
            accounts.rent.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; CREATE_CLMMPOOL_METADATA_IX_ACCOUNTS_LEN]>
for CreateClmmpoolMetadataAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; CREATE_CLMMPOOL_METADATA_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            payer: &arr[0],
            clmm_config: &arr[1],
            clmmpool: &arr[2],
            clmmpool_metadata: &arr[3],
            protocol_authority: &arr[4],
            rent: &arr[5],
            system_program: &arr[6],
        }
    }
}
pub const CREATE_CLMMPOOL_METADATA_IX_DISCM: [u8; 8usize] = [
    245, 213, 182, 15, 65, 104, 8, 244,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CreateClmmpoolMetadataIxArgs {
    pub name: String,
    pub uri: String,
}
#[derive(Clone, Debug, PartialEq)]
pub struct CreateClmmpoolMetadataIxData(pub CreateClmmpoolMetadataIxArgs);
impl From<CreateClmmpoolMetadataIxArgs> for CreateClmmpoolMetadataIxData {
    fn from(args: CreateClmmpoolMetadataIxArgs) -> Self {
        Self(args)
    }
}
impl CreateClmmpoolMetadataIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CREATE_CLMMPOOL_METADATA_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let name: String = crate::borsh_de_or_default(&mut reader)?;
        let uri: String = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(CreateClmmpoolMetadataIxArgs {
                name,
                uri,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CREATE_CLMMPOOL_METADATA_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.name, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.uri, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn create_clmmpool_metadata_ix_with_program_id(
    program_id: Pubkey,
    keys: CreateClmmpoolMetadataKeys,
    args: CreateClmmpoolMetadataIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CREATE_CLMMPOOL_METADATA_IX_ACCOUNTS_LEN] = keys.into();
    let data: CreateClmmpoolMetadataIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn create_clmmpool_metadata_ix(
    keys: CreateClmmpoolMetadataKeys,
    args: CreateClmmpoolMetadataIxArgs,
) -> std::io::Result<Instruction> {
    create_clmmpool_metadata_ix_with_program_id(CREMA_PROGRAM_ID, keys, args)
}
pub fn create_clmmpool_metadata_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CreateClmmpoolMetadataAccounts<'_, '_>,
    args: CreateClmmpoolMetadataIxArgs,
) -> ProgramResult {
    let keys: CreateClmmpoolMetadataKeys = accounts.into();
    let ix = create_clmmpool_metadata_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn create_clmmpool_metadata_invoke(
    accounts: CreateClmmpoolMetadataAccounts<'_, '_>,
    args: CreateClmmpoolMetadataIxArgs,
) -> ProgramResult {
    create_clmmpool_metadata_invoke_with_program_id(CREMA_PROGRAM_ID, accounts, args)
}
pub fn create_clmmpool_metadata_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CreateClmmpoolMetadataAccounts<'_, '_>,
    args: CreateClmmpoolMetadataIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CreateClmmpoolMetadataKeys = accounts.into();
    let ix = create_clmmpool_metadata_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn create_clmmpool_metadata_invoke_signed(
    accounts: CreateClmmpoolMetadataAccounts<'_, '_>,
    args: CreateClmmpoolMetadataIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    create_clmmpool_metadata_invoke_signed_with_program_id(
        CREMA_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn create_clmmpool_metadata_verify_account_keys(
    accounts: CreateClmmpoolMetadataAccounts<'_, '_>,
    keys: CreateClmmpoolMetadataKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.payer.key, keys.payer),
        (*accounts.clmm_config.key, keys.clmm_config),
        (*accounts.clmmpool.key, keys.clmmpool),
        (*accounts.clmmpool_metadata.key, keys.clmmpool_metadata),
        (*accounts.protocol_authority.key, keys.protocol_authority),
        (*accounts.rent.key, keys.rent),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn create_clmmpool_metadata_verify_writable_privileges<'me, 'info>(
    accounts: CreateClmmpoolMetadataAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.payer, accounts.clmmpool_metadata] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn create_clmmpool_metadata_verify_signer_privileges<'me, 'info>(
    accounts: CreateClmmpoolMetadataAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.payer, accounts.protocol_authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn create_clmmpool_metadata_verify_account_privileges<'me, 'info>(
    accounts: CreateClmmpoolMetadataAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    create_clmmpool_metadata_verify_writable_privileges(accounts)?;
    create_clmmpool_metadata_verify_signer_privileges(accounts)?;
    Ok(())
}
