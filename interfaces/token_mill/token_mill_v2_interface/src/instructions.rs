use solana_pubkey::Pubkey;
use solana_cpi::{invoke, invoke_signed};
use solana_instruction::{AccountMeta, Instruction};
use solana_account_info::AccountInfo;
use solana_program_error::ProgramError;
use std::io::Read;
#[allow(unused_imports)]
use crate::*;
#[derive(Clone, Debug, PartialEq)]
pub enum TokenMillV2ProgramIx {
    AcceptConfigOwnership,
    ClaimCreatorFees,
    ClaimReferralFees,
    ClaimStakingRewards,
    CreateConfig(CreateConfigIxArgs),
    CreateMarket(CreateMarketIxArgs),
    CreateMarketWithSpl(CreateMarketWithSplIxArgs),
    CreateQuoteAssetBadge,
    CreateReferralAccount(CreateReferralAccountIxArgs),
    CreateStakePosition,
    CreateStaking,
    CreateVestingPlan(CreateVestingPlanIxArgs),
    Deposit(DepositIxArgs),
    Release,
    SetMarketPrices(SetMarketPricesIxArgs),
    Swap(SwapIxArgs),
    TransferConfigOwnership(TransferConfigOwnershipIxArgs),
    UpdateCreator(UpdateCreatorIxArgs),
    UpdateDefaultFeeShares(UpdateDefaultFeeSharesIxArgs),
    UpdateMarketFeeShares(UpdateMarketFeeSharesIxArgs),
    UpdateProtocolFeeRecipient(UpdateProtocolFeeRecipientIxArgs),
    UpdateQuoteAssetBadge(UpdateQuoteAssetBadgeIxArgs),
    Withdraw(WithdrawIxArgs),
}
impl TokenMillV2ProgramIx {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        if buf.starts_with(&ACCEPT_CONFIG_OWNERSHIP_IX_DISCM) {
            return Ok(Self::AcceptConfigOwnership);
        }
        if buf.starts_with(&CLAIM_CREATOR_FEES_IX_DISCM) {
            return Ok(Self::ClaimCreatorFees);
        }
        if buf.starts_with(&CLAIM_REFERRAL_FEES_IX_DISCM) {
            return Ok(Self::ClaimReferralFees);
        }
        if buf.starts_with(&CLAIM_STAKING_REWARDS_IX_DISCM) {
            return Ok(Self::ClaimStakingRewards);
        }
        if buf.starts_with(&CREATE_CONFIG_IX_DISCM) {
            let mut reader = &buf[CREATE_CONFIG_IX_DISCM.len()..];
            let authority: Pubkey = crate::borsh_de_or_default(&mut reader)?;
            let protocol_fee_recipient: Pubkey = crate::borsh_de_or_default(
                &mut reader,
            )?;
            let protocol_fee_share: u16 = crate::borsh_de_or_default(&mut reader)?;
            let referral_fee_share: u16 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::CreateConfig(CreateConfigIxArgs {
                    authority,
                    protocol_fee_recipient,
                    protocol_fee_share,
                    referral_fee_share,
                }),
            );
        }
        if buf.starts_with(&CREATE_MARKET_IX_DISCM) {
            let mut reader = &buf[CREATE_MARKET_IX_DISCM.len()..];
            let name: String = crate::borsh_de_or_default(&mut reader)?;
            let symbol: String = crate::borsh_de_or_default(&mut reader)?;
            let uri: String = crate::borsh_de_or_default(&mut reader)?;
            let total_supply: u64 = crate::borsh_de_or_default(&mut reader)?;
            let creator_fee_share: u16 = crate::borsh_de_or_default(&mut reader)?;
            let staking_fee_share: u16 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::CreateMarket(CreateMarketIxArgs {
                    name,
                    symbol,
                    uri,
                    total_supply,
                    creator_fee_share,
                    staking_fee_share,
                }),
            );
        }
        if buf.starts_with(&CREATE_MARKET_WITH_SPL_IX_DISCM) {
            let mut reader = &buf[CREATE_MARKET_WITH_SPL_IX_DISCM.len()..];
            let name: String = crate::borsh_de_or_default(&mut reader)?;
            let symbol: String = crate::borsh_de_or_default(&mut reader)?;
            let uri: String = crate::borsh_de_or_default(&mut reader)?;
            let total_supply: u64 = crate::borsh_de_or_default(&mut reader)?;
            let creator_fee_share: u16 = crate::borsh_de_or_default(&mut reader)?;
            let staking_fee_share: u16 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::CreateMarketWithSpl(CreateMarketWithSplIxArgs {
                    name,
                    symbol,
                    uri,
                    total_supply,
                    creator_fee_share,
                    staking_fee_share,
                }),
            );
        }
        if buf.starts_with(&CREATE_QUOTE_ASSET_BADGE_IX_DISCM) {
            return Ok(Self::CreateQuoteAssetBadge);
        }
        if buf.starts_with(&CREATE_REFERRAL_ACCOUNT_IX_DISCM) {
            let mut reader = &buf[CREATE_REFERRAL_ACCOUNT_IX_DISCM.len()..];
            let referrer: Pubkey = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::CreateReferralAccount(CreateReferralAccountIxArgs {
                    referrer,
                }),
            );
        }
        if buf.starts_with(&CREATE_STAKE_POSITION_IX_DISCM) {
            return Ok(Self::CreateStakePosition);
        }
        if buf.starts_with(&CREATE_STAKING_IX_DISCM) {
            return Ok(Self::CreateStaking);
        }
        if buf.starts_with(&CREATE_VESTING_PLAN_IX_DISCM) {
            let mut reader = &buf[CREATE_VESTING_PLAN_IX_DISCM.len()..];
            let start: i64 = crate::borsh_de_or_default(&mut reader)?;
            let vesting_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            let vesting_duration: i64 = crate::borsh_de_or_default(&mut reader)?;
            let cliff_duration: i64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::CreateVestingPlan(CreateVestingPlanIxArgs {
                    start,
                    vesting_amount,
                    vesting_duration,
                    cliff_duration,
                }),
            );
        }
        if buf.starts_with(&DEPOSIT_IX_DISCM) {
            let mut reader = &buf[DEPOSIT_IX_DISCM.len()..];
            let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(Self::Deposit(DepositIxArgs { amount }));
        }
        if buf.starts_with(&RELEASE_IX_DISCM) {
            return Ok(Self::Release);
        }
        if buf.starts_with(&SET_MARKET_PRICES_IX_DISCM) {
            let mut reader = &buf[SET_MARKET_PRICES_IX_DISCM.len()..];
            let bid_prices: [u64; 11] = crate::borsh_de_or_default(&mut reader)?;
            let ask_prices: [u64; 11] = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::SetMarketPrices(SetMarketPricesIxArgs {
                    bid_prices,
                    ask_prices,
                }),
            );
        }
        if buf.starts_with(&SWAP_IX_DISCM) {
            let mut reader = &buf[SWAP_IX_DISCM.len()..];
            let swap_type: SwapType = crate::borsh_de_or_default(&mut reader)?;
            let swap_amount_type: SwapAmountType = crate::borsh_de_or_default(
                &mut reader,
            )?;
            let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            let other_amount_threshold: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::Swap(SwapIxArgs {
                    swap_type,
                    swap_amount_type,
                    amount,
                    other_amount_threshold,
                }),
            );
        }
        if buf.starts_with(&TRANSFER_CONFIG_OWNERSHIP_IX_DISCM) {
            let mut reader = &buf[TRANSFER_CONFIG_OWNERSHIP_IX_DISCM.len()..];
            let pending_authority: Option<Pubkey> = crate::borsh_de_or_default(
                &mut reader,
            )?;
            return Ok(
                Self::TransferConfigOwnership(TransferConfigOwnershipIxArgs {
                    pending_authority,
                }),
            );
        }
        if buf.starts_with(&UPDATE_CREATOR_IX_DISCM) {
            let mut reader = &buf[UPDATE_CREATOR_IX_DISCM.len()..];
            let new_creator: Pubkey = crate::borsh_de_or_default(&mut reader)?;
            return Ok(Self::UpdateCreator(UpdateCreatorIxArgs { new_creator }));
        }
        if buf.starts_with(&UPDATE_DEFAULT_FEE_SHARES_IX_DISCM) {
            let mut reader = &buf[UPDATE_DEFAULT_FEE_SHARES_IX_DISCM.len()..];
            let new_default_protocol_fee_share: u16 = crate::borsh_de_or_default(
                &mut reader,
            )?;
            let new_referral_fee_share: u16 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::UpdateDefaultFeeShares(UpdateDefaultFeeSharesIxArgs {
                    new_default_protocol_fee_share,
                    new_referral_fee_share,
                }),
            );
        }
        if buf.starts_with(&UPDATE_MARKET_FEE_SHARES_IX_DISCM) {
            let mut reader = &buf[UPDATE_MARKET_FEE_SHARES_IX_DISCM.len()..];
            let new_creator_fee_share: u16 = crate::borsh_de_or_default(&mut reader)?;
            let new_staking_fee_share: u16 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::UpdateMarketFeeShares(UpdateMarketFeeSharesIxArgs {
                    new_creator_fee_share,
                    new_staking_fee_share,
                }),
            );
        }
        if buf.starts_with(&UPDATE_PROTOCOL_FEE_RECIPIENT_IX_DISCM) {
            let mut reader = &buf[UPDATE_PROTOCOL_FEE_RECIPIENT_IX_DISCM.len()..];
            let new_protocol_fee_recipient: Pubkey = crate::borsh_de_or_default(
                &mut reader,
            )?;
            return Ok(
                Self::UpdateProtocolFeeRecipient(UpdateProtocolFeeRecipientIxArgs {
                    new_protocol_fee_recipient,
                }),
            );
        }
        if buf.starts_with(&UPDATE_QUOTE_ASSET_BADGE_IX_DISCM) {
            let mut reader = &buf[UPDATE_QUOTE_ASSET_BADGE_IX_DISCM.len()..];
            let status: QuoteTokenBadgeStatus = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::UpdateQuoteAssetBadge(UpdateQuoteAssetBadgeIxArgs {
                    status,
                }),
            );
        }
        if buf.starts_with(&WITHDRAW_IX_DISCM) {
            let mut reader = &buf[WITHDRAW_IX_DISCM.len()..];
            let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(Self::Withdraw(WithdrawIxArgs { amount }));
        }
        Err(std::io::Error::from(std::io::ErrorKind::InvalidData))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        match self {
            Self::AcceptConfigOwnership => {
                writer.write_all(&ACCEPT_CONFIG_OWNERSHIP_IX_DISCM)
            }
            Self::ClaimCreatorFees => writer.write_all(&CLAIM_CREATOR_FEES_IX_DISCM),
            Self::ClaimReferralFees => writer.write_all(&CLAIM_REFERRAL_FEES_IX_DISCM),
            Self::ClaimStakingRewards => {
                writer.write_all(&CLAIM_STAKING_REWARDS_IX_DISCM)
            }
            Self::CreateConfig(args) => {
                writer.write_all(&CREATE_CONFIG_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.authority, &mut writer)?;
                borsh::BorshSerialize::serialize(
                    &args.protocol_fee_recipient,
                    &mut writer,
                )?;
                borsh::BorshSerialize::serialize(&args.protocol_fee_share, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.referral_fee_share, &mut writer)?;
                Ok(())
            }
            Self::CreateMarket(args) => {
                writer.write_all(&CREATE_MARKET_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.name, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.symbol, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.uri, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.total_supply, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.creator_fee_share, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.staking_fee_share, &mut writer)?;
                Ok(())
            }
            Self::CreateMarketWithSpl(args) => {
                writer.write_all(&CREATE_MARKET_WITH_SPL_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.name, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.symbol, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.uri, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.total_supply, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.creator_fee_share, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.staking_fee_share, &mut writer)?;
                Ok(())
            }
            Self::CreateQuoteAssetBadge => {
                writer.write_all(&CREATE_QUOTE_ASSET_BADGE_IX_DISCM)
            }
            Self::CreateReferralAccount(args) => {
                writer.write_all(&CREATE_REFERRAL_ACCOUNT_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.referrer, &mut writer)?;
                Ok(())
            }
            Self::CreateStakePosition => {
                writer.write_all(&CREATE_STAKE_POSITION_IX_DISCM)
            }
            Self::CreateStaking => writer.write_all(&CREATE_STAKING_IX_DISCM),
            Self::CreateVestingPlan(args) => {
                writer.write_all(&CREATE_VESTING_PLAN_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.start, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.vesting_amount, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.vesting_duration, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.cliff_duration, &mut writer)?;
                Ok(())
            }
            Self::Deposit(args) => {
                writer.write_all(&DEPOSIT_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.amount, &mut writer)?;
                Ok(())
            }
            Self::Release => writer.write_all(&RELEASE_IX_DISCM),
            Self::SetMarketPrices(args) => {
                writer.write_all(&SET_MARKET_PRICES_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.bid_prices, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.ask_prices, &mut writer)?;
                Ok(())
            }
            Self::Swap(args) => {
                writer.write_all(&SWAP_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.swap_type, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.swap_amount_type, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.amount, &mut writer)?;
                borsh::BorshSerialize::serialize(
                    &args.other_amount_threshold,
                    &mut writer,
                )?;
                Ok(())
            }
            Self::TransferConfigOwnership(args) => {
                writer.write_all(&TRANSFER_CONFIG_OWNERSHIP_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.pending_authority, &mut writer)?;
                Ok(())
            }
            Self::UpdateCreator(args) => {
                writer.write_all(&UPDATE_CREATOR_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.new_creator, &mut writer)?;
                Ok(())
            }
            Self::UpdateDefaultFeeShares(args) => {
                writer.write_all(&UPDATE_DEFAULT_FEE_SHARES_IX_DISCM)?;
                borsh::BorshSerialize::serialize(
                    &args.new_default_protocol_fee_share,
                    &mut writer,
                )?;
                borsh::BorshSerialize::serialize(
                    &args.new_referral_fee_share,
                    &mut writer,
                )?;
                Ok(())
            }
            Self::UpdateMarketFeeShares(args) => {
                writer.write_all(&UPDATE_MARKET_FEE_SHARES_IX_DISCM)?;
                borsh::BorshSerialize::serialize(
                    &args.new_creator_fee_share,
                    &mut writer,
                )?;
                borsh::BorshSerialize::serialize(
                    &args.new_staking_fee_share,
                    &mut writer,
                )?;
                Ok(())
            }
            Self::UpdateProtocolFeeRecipient(args) => {
                writer.write_all(&UPDATE_PROTOCOL_FEE_RECIPIENT_IX_DISCM)?;
                borsh::BorshSerialize::serialize(
                    &args.new_protocol_fee_recipient,
                    &mut writer,
                )?;
                Ok(())
            }
            Self::UpdateQuoteAssetBadge(args) => {
                writer.write_all(&UPDATE_QUOTE_ASSET_BADGE_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.status, &mut writer)?;
                Ok(())
            }
            Self::Withdraw(args) => {
                writer.write_all(&WITHDRAW_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.amount, &mut writer)?;
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
pub const ACCEPT_CONFIG_OWNERSHIP_IX_ACCOUNTS_LEN: usize = 4;
#[derive(Copy, Clone, Debug)]
pub struct AcceptConfigOwnershipAccounts<'me, 'info> {
    pub config: &'me AccountInfo<'info>,
    pub pending_authority: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct AcceptConfigOwnershipKeys {
    pub config: Pubkey,
    pub pending_authority: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<AcceptConfigOwnershipAccounts<'_, '_>> for AcceptConfigOwnershipKeys {
    fn from(accounts: AcceptConfigOwnershipAccounts) -> Self {
        Self {
            config: *accounts.config.key,
            pending_authority: *accounts.pending_authority.key,
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
                pubkey: keys.config,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pending_authority,
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
            config: pubkeys[0],
            pending_authority: pubkeys[1],
            event_authority: pubkeys[2],
            program: pubkeys[3],
        }
    }
}
impl<'info> From<AcceptConfigOwnershipAccounts<'_, 'info>>
for [AccountInfo<'info>; ACCEPT_CONFIG_OWNERSHIP_IX_ACCOUNTS_LEN] {
    fn from(accounts: AcceptConfigOwnershipAccounts<'_, 'info>) -> Self {
        [
            accounts.config.clone(),
            accounts.pending_authority.clone(),
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
            config: &arr[0],
            pending_authority: &arr[1],
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
    accept_config_ownership_ix_with_program_id(TOKEN_MILL_V2_PROGRAM_ID, keys)
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
    accept_config_ownership_invoke_with_program_id(TOKEN_MILL_V2_PROGRAM_ID, accounts)
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
        TOKEN_MILL_V2_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn accept_config_ownership_verify_account_keys(
    accounts: AcceptConfigOwnershipAccounts<'_, '_>,
    keys: AcceptConfigOwnershipKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.config.key, keys.config),
        (*accounts.pending_authority.key, keys.pending_authority),
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
    for should_be_writable in [accounts.config] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn accept_config_ownership_verify_signer_privileges<'me, 'info>(
    accounts: AcceptConfigOwnershipAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.pending_authority] {
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
pub const CLAIM_CREATOR_FEES_IX_ACCOUNTS_LEN: usize = 8;
#[derive(Copy, Clone, Debug)]
pub struct ClaimCreatorFeesAccounts<'me, 'info> {
    pub market: &'me AccountInfo<'info>,
    pub quote_token_mint: &'me AccountInfo<'info>,
    pub market_quote_token_ata: &'me AccountInfo<'info>,
    pub creator_quote_token_ata: &'me AccountInfo<'info>,
    pub creator: &'me AccountInfo<'info>,
    pub quote_token_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ClaimCreatorFeesKeys {
    pub market: Pubkey,
    pub quote_token_mint: Pubkey,
    pub market_quote_token_ata: Pubkey,
    pub creator_quote_token_ata: Pubkey,
    pub creator: Pubkey,
    pub quote_token_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<ClaimCreatorFeesAccounts<'_, '_>> for ClaimCreatorFeesKeys {
    fn from(accounts: ClaimCreatorFeesAccounts) -> Self {
        Self {
            market: *accounts.market.key,
            quote_token_mint: *accounts.quote_token_mint.key,
            market_quote_token_ata: *accounts.market_quote_token_ata.key,
            creator_quote_token_ata: *accounts.creator_quote_token_ata.key,
            creator: *accounts.creator.key,
            quote_token_program: *accounts.quote_token_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<ClaimCreatorFeesKeys> for [AccountMeta; CLAIM_CREATOR_FEES_IX_ACCOUNTS_LEN] {
    fn from(keys: ClaimCreatorFeesKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.market,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.quote_token_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.market_quote_token_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.creator_quote_token_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.creator,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.quote_token_program,
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
impl From<[Pubkey; CLAIM_CREATOR_FEES_IX_ACCOUNTS_LEN]> for ClaimCreatorFeesKeys {
    fn from(pubkeys: [Pubkey; CLAIM_CREATOR_FEES_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            market: pubkeys[0],
            quote_token_mint: pubkeys[1],
            market_quote_token_ata: pubkeys[2],
            creator_quote_token_ata: pubkeys[3],
            creator: pubkeys[4],
            quote_token_program: pubkeys[5],
            event_authority: pubkeys[6],
            program: pubkeys[7],
        }
    }
}
impl<'info> From<ClaimCreatorFeesAccounts<'_, 'info>>
for [AccountInfo<'info>; CLAIM_CREATOR_FEES_IX_ACCOUNTS_LEN] {
    fn from(accounts: ClaimCreatorFeesAccounts<'_, 'info>) -> Self {
        [
            accounts.market.clone(),
            accounts.quote_token_mint.clone(),
            accounts.market_quote_token_ata.clone(),
            accounts.creator_quote_token_ata.clone(),
            accounts.creator.clone(),
            accounts.quote_token_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CLAIM_CREATOR_FEES_IX_ACCOUNTS_LEN]>
for ClaimCreatorFeesAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; CLAIM_CREATOR_FEES_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            market: &arr[0],
            quote_token_mint: &arr[1],
            market_quote_token_ata: &arr[2],
            creator_quote_token_ata: &arr[3],
            creator: &arr[4],
            quote_token_program: &arr[5],
            event_authority: &arr[6],
            program: &arr[7],
        }
    }
}
pub const CLAIM_CREATOR_FEES_IX_DISCM: [u8; 8usize] = [
    0, 23, 125, 234, 156, 118, 134, 89,
];
#[derive(Clone, Debug, PartialEq)]
pub struct ClaimCreatorFeesIxData;
impl ClaimCreatorFeesIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CLAIM_CREATOR_FEES_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CLAIM_CREATOR_FEES_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn claim_creator_fees_ix_with_program_id(
    program_id: Pubkey,
    keys: ClaimCreatorFeesKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CLAIM_CREATOR_FEES_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: ClaimCreatorFeesIxData.try_to_vec()?,
    })
}
pub fn claim_creator_fees_ix(
    keys: ClaimCreatorFeesKeys,
) -> std::io::Result<Instruction> {
    claim_creator_fees_ix_with_program_id(TOKEN_MILL_V2_PROGRAM_ID, keys)
}
pub fn claim_creator_fees_invoke_with_program_id(
    program_id: Pubkey,
    accounts: ClaimCreatorFeesAccounts<'_, '_>,
) -> ProgramResult {
    let keys: ClaimCreatorFeesKeys = accounts.into();
    let ix = claim_creator_fees_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn claim_creator_fees_invoke(
    accounts: ClaimCreatorFeesAccounts<'_, '_>,
) -> ProgramResult {
    claim_creator_fees_invoke_with_program_id(TOKEN_MILL_V2_PROGRAM_ID, accounts)
}
pub fn claim_creator_fees_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: ClaimCreatorFeesAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: ClaimCreatorFeesKeys = accounts.into();
    let ix = claim_creator_fees_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn claim_creator_fees_invoke_signed(
    accounts: ClaimCreatorFeesAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    claim_creator_fees_invoke_signed_with_program_id(
        TOKEN_MILL_V2_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn claim_creator_fees_verify_account_keys(
    accounts: ClaimCreatorFeesAccounts<'_, '_>,
    keys: ClaimCreatorFeesKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.market.key, keys.market),
        (*accounts.quote_token_mint.key, keys.quote_token_mint),
        (*accounts.market_quote_token_ata.key, keys.market_quote_token_ata),
        (*accounts.creator_quote_token_ata.key, keys.creator_quote_token_ata),
        (*accounts.creator.key, keys.creator),
        (*accounts.quote_token_program.key, keys.quote_token_program),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn claim_creator_fees_verify_writable_privileges<'me, 'info>(
    accounts: ClaimCreatorFeesAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.market,
        accounts.market_quote_token_ata,
        accounts.creator_quote_token_ata,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn claim_creator_fees_verify_signer_privileges<'me, 'info>(
    accounts: ClaimCreatorFeesAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.creator] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn claim_creator_fees_verify_account_privileges<'me, 'info>(
    accounts: ClaimCreatorFeesAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    claim_creator_fees_verify_writable_privileges(accounts)?;
    claim_creator_fees_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CLAIM_REFERRAL_FEES_IX_ACCOUNTS_LEN: usize = 8;
#[derive(Copy, Clone, Debug)]
pub struct ClaimReferralFeesAccounts<'me, 'info> {
    pub referral_account: &'me AccountInfo<'info>,
    pub quote_token_mint: &'me AccountInfo<'info>,
    pub referral_account_quote_token_ata: &'me AccountInfo<'info>,
    pub referrer_quote_token_ata: &'me AccountInfo<'info>,
    pub referrer: &'me AccountInfo<'info>,
    pub quote_token_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ClaimReferralFeesKeys {
    pub referral_account: Pubkey,
    pub quote_token_mint: Pubkey,
    pub referral_account_quote_token_ata: Pubkey,
    pub referrer_quote_token_ata: Pubkey,
    pub referrer: Pubkey,
    pub quote_token_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<ClaimReferralFeesAccounts<'_, '_>> for ClaimReferralFeesKeys {
    fn from(accounts: ClaimReferralFeesAccounts) -> Self {
        Self {
            referral_account: *accounts.referral_account.key,
            quote_token_mint: *accounts.quote_token_mint.key,
            referral_account_quote_token_ata: *accounts
                .referral_account_quote_token_ata
                .key,
            referrer_quote_token_ata: *accounts.referrer_quote_token_ata.key,
            referrer: *accounts.referrer.key,
            quote_token_program: *accounts.quote_token_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<ClaimReferralFeesKeys> for [AccountMeta; CLAIM_REFERRAL_FEES_IX_ACCOUNTS_LEN] {
    fn from(keys: ClaimReferralFeesKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.referral_account,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.quote_token_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.referral_account_quote_token_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.referrer_quote_token_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.referrer,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.quote_token_program,
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
impl From<[Pubkey; CLAIM_REFERRAL_FEES_IX_ACCOUNTS_LEN]> for ClaimReferralFeesKeys {
    fn from(pubkeys: [Pubkey; CLAIM_REFERRAL_FEES_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            referral_account: pubkeys[0],
            quote_token_mint: pubkeys[1],
            referral_account_quote_token_ata: pubkeys[2],
            referrer_quote_token_ata: pubkeys[3],
            referrer: pubkeys[4],
            quote_token_program: pubkeys[5],
            event_authority: pubkeys[6],
            program: pubkeys[7],
        }
    }
}
impl<'info> From<ClaimReferralFeesAccounts<'_, 'info>>
for [AccountInfo<'info>; CLAIM_REFERRAL_FEES_IX_ACCOUNTS_LEN] {
    fn from(accounts: ClaimReferralFeesAccounts<'_, 'info>) -> Self {
        [
            accounts.referral_account.clone(),
            accounts.quote_token_mint.clone(),
            accounts.referral_account_quote_token_ata.clone(),
            accounts.referrer_quote_token_ata.clone(),
            accounts.referrer.clone(),
            accounts.quote_token_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CLAIM_REFERRAL_FEES_IX_ACCOUNTS_LEN]>
for ClaimReferralFeesAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; CLAIM_REFERRAL_FEES_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            referral_account: &arr[0],
            quote_token_mint: &arr[1],
            referral_account_quote_token_ata: &arr[2],
            referrer_quote_token_ata: &arr[3],
            referrer: &arr[4],
            quote_token_program: &arr[5],
            event_authority: &arr[6],
            program: &arr[7],
        }
    }
}
pub const CLAIM_REFERRAL_FEES_IX_DISCM: [u8; 8usize] = [
    208, 216, 137, 78, 36, 103, 162, 49,
];
#[derive(Clone, Debug, PartialEq)]
pub struct ClaimReferralFeesIxData;
impl ClaimReferralFeesIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CLAIM_REFERRAL_FEES_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CLAIM_REFERRAL_FEES_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn claim_referral_fees_ix_with_program_id(
    program_id: Pubkey,
    keys: ClaimReferralFeesKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CLAIM_REFERRAL_FEES_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: ClaimReferralFeesIxData.try_to_vec()?,
    })
}
pub fn claim_referral_fees_ix(
    keys: ClaimReferralFeesKeys,
) -> std::io::Result<Instruction> {
    claim_referral_fees_ix_with_program_id(TOKEN_MILL_V2_PROGRAM_ID, keys)
}
pub fn claim_referral_fees_invoke_with_program_id(
    program_id: Pubkey,
    accounts: ClaimReferralFeesAccounts<'_, '_>,
) -> ProgramResult {
    let keys: ClaimReferralFeesKeys = accounts.into();
    let ix = claim_referral_fees_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn claim_referral_fees_invoke(
    accounts: ClaimReferralFeesAccounts<'_, '_>,
) -> ProgramResult {
    claim_referral_fees_invoke_with_program_id(TOKEN_MILL_V2_PROGRAM_ID, accounts)
}
pub fn claim_referral_fees_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: ClaimReferralFeesAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: ClaimReferralFeesKeys = accounts.into();
    let ix = claim_referral_fees_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn claim_referral_fees_invoke_signed(
    accounts: ClaimReferralFeesAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    claim_referral_fees_invoke_signed_with_program_id(
        TOKEN_MILL_V2_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn claim_referral_fees_verify_account_keys(
    accounts: ClaimReferralFeesAccounts<'_, '_>,
    keys: ClaimReferralFeesKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.referral_account.key, keys.referral_account),
        (*accounts.quote_token_mint.key, keys.quote_token_mint),
        (
            *accounts.referral_account_quote_token_ata.key,
            keys.referral_account_quote_token_ata,
        ),
        (*accounts.referrer_quote_token_ata.key, keys.referrer_quote_token_ata),
        (*accounts.referrer.key, keys.referrer),
        (*accounts.quote_token_program.key, keys.quote_token_program),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn claim_referral_fees_verify_writable_privileges<'me, 'info>(
    accounts: ClaimReferralFeesAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.referral_account_quote_token_ata,
        accounts.referrer_quote_token_ata,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn claim_referral_fees_verify_signer_privileges<'me, 'info>(
    accounts: ClaimReferralFeesAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.referrer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn claim_referral_fees_verify_account_privileges<'me, 'info>(
    accounts: ClaimReferralFeesAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    claim_referral_fees_verify_writable_privileges(accounts)?;
    claim_referral_fees_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CLAIM_STAKING_REWARDS_IX_ACCOUNTS_LEN: usize = 10;
#[derive(Copy, Clone, Debug)]
pub struct ClaimStakingRewardsAccounts<'me, 'info> {
    pub market: &'me AccountInfo<'info>,
    pub staking: &'me AccountInfo<'info>,
    pub stake_position: &'me AccountInfo<'info>,
    pub quote_token_mint: &'me AccountInfo<'info>,
    pub market_quote_token_ata: &'me AccountInfo<'info>,
    pub user_quote_token_ata: &'me AccountInfo<'info>,
    pub user: &'me AccountInfo<'info>,
    pub quote_token_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ClaimStakingRewardsKeys {
    pub market: Pubkey,
    pub staking: Pubkey,
    pub stake_position: Pubkey,
    pub quote_token_mint: Pubkey,
    pub market_quote_token_ata: Pubkey,
    pub user_quote_token_ata: Pubkey,
    pub user: Pubkey,
    pub quote_token_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<ClaimStakingRewardsAccounts<'_, '_>> for ClaimStakingRewardsKeys {
    fn from(accounts: ClaimStakingRewardsAccounts) -> Self {
        Self {
            market: *accounts.market.key,
            staking: *accounts.staking.key,
            stake_position: *accounts.stake_position.key,
            quote_token_mint: *accounts.quote_token_mint.key,
            market_quote_token_ata: *accounts.market_quote_token_ata.key,
            user_quote_token_ata: *accounts.user_quote_token_ata.key,
            user: *accounts.user.key,
            quote_token_program: *accounts.quote_token_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<ClaimStakingRewardsKeys>
for [AccountMeta; CLAIM_STAKING_REWARDS_IX_ACCOUNTS_LEN] {
    fn from(keys: ClaimStakingRewardsKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.market,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.staking,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.stake_position,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.quote_token_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.market_quote_token_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_quote_token_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.quote_token_program,
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
impl From<[Pubkey; CLAIM_STAKING_REWARDS_IX_ACCOUNTS_LEN]> for ClaimStakingRewardsKeys {
    fn from(pubkeys: [Pubkey; CLAIM_STAKING_REWARDS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            market: pubkeys[0],
            staking: pubkeys[1],
            stake_position: pubkeys[2],
            quote_token_mint: pubkeys[3],
            market_quote_token_ata: pubkeys[4],
            user_quote_token_ata: pubkeys[5],
            user: pubkeys[6],
            quote_token_program: pubkeys[7],
            event_authority: pubkeys[8],
            program: pubkeys[9],
        }
    }
}
impl<'info> From<ClaimStakingRewardsAccounts<'_, 'info>>
for [AccountInfo<'info>; CLAIM_STAKING_REWARDS_IX_ACCOUNTS_LEN] {
    fn from(accounts: ClaimStakingRewardsAccounts<'_, 'info>) -> Self {
        [
            accounts.market.clone(),
            accounts.staking.clone(),
            accounts.stake_position.clone(),
            accounts.quote_token_mint.clone(),
            accounts.market_quote_token_ata.clone(),
            accounts.user_quote_token_ata.clone(),
            accounts.user.clone(),
            accounts.quote_token_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CLAIM_STAKING_REWARDS_IX_ACCOUNTS_LEN]>
for ClaimStakingRewardsAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; CLAIM_STAKING_REWARDS_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            market: &arr[0],
            staking: &arr[1],
            stake_position: &arr[2],
            quote_token_mint: &arr[3],
            market_quote_token_ata: &arr[4],
            user_quote_token_ata: &arr[5],
            user: &arr[6],
            quote_token_program: &arr[7],
            event_authority: &arr[8],
            program: &arr[9],
        }
    }
}
pub const CLAIM_STAKING_REWARDS_IX_DISCM: [u8; 8usize] = [
    229, 141, 170, 69, 111, 94, 6, 72,
];
#[derive(Clone, Debug, PartialEq)]
pub struct ClaimStakingRewardsIxData;
impl ClaimStakingRewardsIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CLAIM_STAKING_REWARDS_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CLAIM_STAKING_REWARDS_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn claim_staking_rewards_ix_with_program_id(
    program_id: Pubkey,
    keys: ClaimStakingRewardsKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CLAIM_STAKING_REWARDS_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: ClaimStakingRewardsIxData.try_to_vec()?,
    })
}
pub fn claim_staking_rewards_ix(
    keys: ClaimStakingRewardsKeys,
) -> std::io::Result<Instruction> {
    claim_staking_rewards_ix_with_program_id(TOKEN_MILL_V2_PROGRAM_ID, keys)
}
pub fn claim_staking_rewards_invoke_with_program_id(
    program_id: Pubkey,
    accounts: ClaimStakingRewardsAccounts<'_, '_>,
) -> ProgramResult {
    let keys: ClaimStakingRewardsKeys = accounts.into();
    let ix = claim_staking_rewards_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn claim_staking_rewards_invoke(
    accounts: ClaimStakingRewardsAccounts<'_, '_>,
) -> ProgramResult {
    claim_staking_rewards_invoke_with_program_id(TOKEN_MILL_V2_PROGRAM_ID, accounts)
}
pub fn claim_staking_rewards_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: ClaimStakingRewardsAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: ClaimStakingRewardsKeys = accounts.into();
    let ix = claim_staking_rewards_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn claim_staking_rewards_invoke_signed(
    accounts: ClaimStakingRewardsAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    claim_staking_rewards_invoke_signed_with_program_id(
        TOKEN_MILL_V2_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn claim_staking_rewards_verify_account_keys(
    accounts: ClaimStakingRewardsAccounts<'_, '_>,
    keys: ClaimStakingRewardsKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.market.key, keys.market),
        (*accounts.staking.key, keys.staking),
        (*accounts.stake_position.key, keys.stake_position),
        (*accounts.quote_token_mint.key, keys.quote_token_mint),
        (*accounts.market_quote_token_ata.key, keys.market_quote_token_ata),
        (*accounts.user_quote_token_ata.key, keys.user_quote_token_ata),
        (*accounts.user.key, keys.user),
        (*accounts.quote_token_program.key, keys.quote_token_program),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn claim_staking_rewards_verify_writable_privileges<'me, 'info>(
    accounts: ClaimStakingRewardsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.market,
        accounts.staking,
        accounts.stake_position,
        accounts.market_quote_token_ata,
        accounts.user_quote_token_ata,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn claim_staking_rewards_verify_signer_privileges<'me, 'info>(
    accounts: ClaimStakingRewardsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.user] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn claim_staking_rewards_verify_account_privileges<'me, 'info>(
    accounts: ClaimStakingRewardsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    claim_staking_rewards_verify_writable_privileges(accounts)?;
    claim_staking_rewards_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CREATE_CONFIG_IX_ACCOUNTS_LEN: usize = 5;
#[derive(Copy, Clone, Debug)]
pub struct CreateConfigAccounts<'me, 'info> {
    pub config: &'me AccountInfo<'info>,
    pub payer: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CreateConfigKeys {
    pub config: Pubkey,
    pub payer: Pubkey,
    pub system_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<CreateConfigAccounts<'_, '_>> for CreateConfigKeys {
    fn from(accounts: CreateConfigAccounts) -> Self {
        Self {
            config: *accounts.config.key,
            payer: *accounts.payer.key,
            system_program: *accounts.system_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<CreateConfigKeys> for [AccountMeta; CREATE_CONFIG_IX_ACCOUNTS_LEN] {
    fn from(keys: CreateConfigKeys) -> Self {
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
impl From<[Pubkey; CREATE_CONFIG_IX_ACCOUNTS_LEN]> for CreateConfigKeys {
    fn from(pubkeys: [Pubkey; CREATE_CONFIG_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            config: pubkeys[0],
            payer: pubkeys[1],
            system_program: pubkeys[2],
            event_authority: pubkeys[3],
            program: pubkeys[4],
        }
    }
}
impl<'info> From<CreateConfigAccounts<'_, 'info>>
for [AccountInfo<'info>; CREATE_CONFIG_IX_ACCOUNTS_LEN] {
    fn from(accounts: CreateConfigAccounts<'_, 'info>) -> Self {
        [
            accounts.config.clone(),
            accounts.payer.clone(),
            accounts.system_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CREATE_CONFIG_IX_ACCOUNTS_LEN]>
for CreateConfigAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; CREATE_CONFIG_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            config: &arr[0],
            payer: &arr[1],
            system_program: &arr[2],
            event_authority: &arr[3],
            program: &arr[4],
        }
    }
}
pub const CREATE_CONFIG_IX_DISCM: [u8; 8usize] = [201, 207, 243, 114, 75, 111, 47, 189];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CreateConfigIxArgs {
    pub authority: Pubkey,
    pub protocol_fee_recipient: Pubkey,
    pub protocol_fee_share: u16,
    pub referral_fee_share: u16,
}
#[derive(Clone, Debug, PartialEq)]
pub struct CreateConfigIxData(pub CreateConfigIxArgs);
impl From<CreateConfigIxArgs> for CreateConfigIxData {
    fn from(args: CreateConfigIxArgs) -> Self {
        Self(args)
    }
}
impl CreateConfigIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CREATE_CONFIG_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let authority: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let protocol_fee_recipient: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let protocol_fee_share: u16 = crate::borsh_de_or_default(&mut reader)?;
        let referral_fee_share: u16 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(CreateConfigIxArgs {
                authority,
                protocol_fee_recipient,
                protocol_fee_share,
                referral_fee_share,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CREATE_CONFIG_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.authority, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.protocol_fee_recipient, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.protocol_fee_share, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.referral_fee_share, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn create_config_ix_with_program_id(
    program_id: Pubkey,
    keys: CreateConfigKeys,
    args: CreateConfigIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CREATE_CONFIG_IX_ACCOUNTS_LEN] = keys.into();
    let data: CreateConfigIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn create_config_ix(
    keys: CreateConfigKeys,
    args: CreateConfigIxArgs,
) -> std::io::Result<Instruction> {
    create_config_ix_with_program_id(TOKEN_MILL_V2_PROGRAM_ID, keys, args)
}
pub fn create_config_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CreateConfigAccounts<'_, '_>,
    args: CreateConfigIxArgs,
) -> ProgramResult {
    let keys: CreateConfigKeys = accounts.into();
    let ix = create_config_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn create_config_invoke(
    accounts: CreateConfigAccounts<'_, '_>,
    args: CreateConfigIxArgs,
) -> ProgramResult {
    create_config_invoke_with_program_id(TOKEN_MILL_V2_PROGRAM_ID, accounts, args)
}
pub fn create_config_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CreateConfigAccounts<'_, '_>,
    args: CreateConfigIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CreateConfigKeys = accounts.into();
    let ix = create_config_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn create_config_invoke_signed(
    accounts: CreateConfigAccounts<'_, '_>,
    args: CreateConfigIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    create_config_invoke_signed_with_program_id(
        TOKEN_MILL_V2_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn create_config_verify_account_keys(
    accounts: CreateConfigAccounts<'_, '_>,
    keys: CreateConfigKeys,
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
pub fn create_config_verify_writable_privileges<'me, 'info>(
    accounts: CreateConfigAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.config, accounts.payer] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn create_config_verify_signer_privileges<'me, 'info>(
    accounts: CreateConfigAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.config, accounts.payer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn create_config_verify_account_privileges<'me, 'info>(
    accounts: CreateConfigAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    create_config_verify_writable_privileges(accounts)?;
    create_config_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CREATE_MARKET_IX_ACCOUNTS_LEN: usize = 12;
#[derive(Copy, Clone, Debug)]
pub struct CreateMarketAccounts<'me, 'info> {
    pub config: &'me AccountInfo<'info>,
    pub market: &'me AccountInfo<'info>,
    pub base_token_mint: &'me AccountInfo<'info>,
    pub market_base_token_ata: &'me AccountInfo<'info>,
    pub quote_token_badge: &'me AccountInfo<'info>,
    pub quote_token_mint: &'me AccountInfo<'info>,
    pub creator: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CreateMarketKeys {
    pub config: Pubkey,
    pub market: Pubkey,
    pub base_token_mint: Pubkey,
    pub market_base_token_ata: Pubkey,
    pub quote_token_badge: Pubkey,
    pub quote_token_mint: Pubkey,
    pub creator: Pubkey,
    pub system_program: Pubkey,
    pub token_program: Pubkey,
    pub associated_token_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<CreateMarketAccounts<'_, '_>> for CreateMarketKeys {
    fn from(accounts: CreateMarketAccounts) -> Self {
        Self {
            config: *accounts.config.key,
            market: *accounts.market.key,
            base_token_mint: *accounts.base_token_mint.key,
            market_base_token_ata: *accounts.market_base_token_ata.key,
            quote_token_badge: *accounts.quote_token_badge.key,
            quote_token_mint: *accounts.quote_token_mint.key,
            creator: *accounts.creator.key,
            system_program: *accounts.system_program.key,
            token_program: *accounts.token_program.key,
            associated_token_program: *accounts.associated_token_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<CreateMarketKeys> for [AccountMeta; CREATE_MARKET_IX_ACCOUNTS_LEN] {
    fn from(keys: CreateMarketKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.market,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.base_token_mint,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.market_base_token_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.quote_token_badge,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.quote_token_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.creator,
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
impl From<[Pubkey; CREATE_MARKET_IX_ACCOUNTS_LEN]> for CreateMarketKeys {
    fn from(pubkeys: [Pubkey; CREATE_MARKET_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            config: pubkeys[0],
            market: pubkeys[1],
            base_token_mint: pubkeys[2],
            market_base_token_ata: pubkeys[3],
            quote_token_badge: pubkeys[4],
            quote_token_mint: pubkeys[5],
            creator: pubkeys[6],
            system_program: pubkeys[7],
            token_program: pubkeys[8],
            associated_token_program: pubkeys[9],
            event_authority: pubkeys[10],
            program: pubkeys[11],
        }
    }
}
impl<'info> From<CreateMarketAccounts<'_, 'info>>
for [AccountInfo<'info>; CREATE_MARKET_IX_ACCOUNTS_LEN] {
    fn from(accounts: CreateMarketAccounts<'_, 'info>) -> Self {
        [
            accounts.config.clone(),
            accounts.market.clone(),
            accounts.base_token_mint.clone(),
            accounts.market_base_token_ata.clone(),
            accounts.quote_token_badge.clone(),
            accounts.quote_token_mint.clone(),
            accounts.creator.clone(),
            accounts.system_program.clone(),
            accounts.token_program.clone(),
            accounts.associated_token_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CREATE_MARKET_IX_ACCOUNTS_LEN]>
for CreateMarketAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; CREATE_MARKET_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            config: &arr[0],
            market: &arr[1],
            base_token_mint: &arr[2],
            market_base_token_ata: &arr[3],
            quote_token_badge: &arr[4],
            quote_token_mint: &arr[5],
            creator: &arr[6],
            system_program: &arr[7],
            token_program: &arr[8],
            associated_token_program: &arr[9],
            event_authority: &arr[10],
            program: &arr[11],
        }
    }
}
pub const CREATE_MARKET_IX_DISCM: [u8; 8usize] = [103, 226, 97, 235, 200, 188, 251, 254];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CreateMarketIxArgs {
    pub name: String,
    pub symbol: String,
    pub uri: String,
    pub total_supply: u64,
    pub creator_fee_share: u16,
    pub staking_fee_share: u16,
}
#[derive(Clone, Debug, PartialEq)]
pub struct CreateMarketIxData(pub CreateMarketIxArgs);
impl From<CreateMarketIxArgs> for CreateMarketIxData {
    fn from(args: CreateMarketIxArgs) -> Self {
        Self(args)
    }
}
impl CreateMarketIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CREATE_MARKET_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let name: String = crate::borsh_de_or_default(&mut reader)?;
        let symbol: String = crate::borsh_de_or_default(&mut reader)?;
        let uri: String = crate::borsh_de_or_default(&mut reader)?;
        let total_supply: u64 = crate::borsh_de_or_default(&mut reader)?;
        let creator_fee_share: u16 = crate::borsh_de_or_default(&mut reader)?;
        let staking_fee_share: u16 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(CreateMarketIxArgs {
                name,
                symbol,
                uri,
                total_supply,
                creator_fee_share,
                staking_fee_share,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CREATE_MARKET_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.name, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.symbol, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.uri, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.total_supply, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.creator_fee_share, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.staking_fee_share, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn create_market_ix_with_program_id(
    program_id: Pubkey,
    keys: CreateMarketKeys,
    args: CreateMarketIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CREATE_MARKET_IX_ACCOUNTS_LEN] = keys.into();
    let data: CreateMarketIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn create_market_ix(
    keys: CreateMarketKeys,
    args: CreateMarketIxArgs,
) -> std::io::Result<Instruction> {
    create_market_ix_with_program_id(TOKEN_MILL_V2_PROGRAM_ID, keys, args)
}
pub fn create_market_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CreateMarketAccounts<'_, '_>,
    args: CreateMarketIxArgs,
) -> ProgramResult {
    let keys: CreateMarketKeys = accounts.into();
    let ix = create_market_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn create_market_invoke(
    accounts: CreateMarketAccounts<'_, '_>,
    args: CreateMarketIxArgs,
) -> ProgramResult {
    create_market_invoke_with_program_id(TOKEN_MILL_V2_PROGRAM_ID, accounts, args)
}
pub fn create_market_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CreateMarketAccounts<'_, '_>,
    args: CreateMarketIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CreateMarketKeys = accounts.into();
    let ix = create_market_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn create_market_invoke_signed(
    accounts: CreateMarketAccounts<'_, '_>,
    args: CreateMarketIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    create_market_invoke_signed_with_program_id(
        TOKEN_MILL_V2_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn create_market_verify_account_keys(
    accounts: CreateMarketAccounts<'_, '_>,
    keys: CreateMarketKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.config.key, keys.config),
        (*accounts.market.key, keys.market),
        (*accounts.base_token_mint.key, keys.base_token_mint),
        (*accounts.market_base_token_ata.key, keys.market_base_token_ata),
        (*accounts.quote_token_badge.key, keys.quote_token_badge),
        (*accounts.quote_token_mint.key, keys.quote_token_mint),
        (*accounts.creator.key, keys.creator),
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
pub fn create_market_verify_writable_privileges<'me, 'info>(
    accounts: CreateMarketAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.market,
        accounts.base_token_mint,
        accounts.market_base_token_ata,
        accounts.creator,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn create_market_verify_signer_privileges<'me, 'info>(
    accounts: CreateMarketAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.base_token_mint, accounts.creator] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn create_market_verify_account_privileges<'me, 'info>(
    accounts: CreateMarketAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    create_market_verify_writable_privileges(accounts)?;
    create_market_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CREATE_MARKET_WITH_SPL_IX_ACCOUNTS_LEN: usize = 14;
#[derive(Copy, Clone, Debug)]
pub struct CreateMarketWithSplAccounts<'me, 'info> {
    pub config: &'me AccountInfo<'info>,
    pub market: &'me AccountInfo<'info>,
    pub base_token_mint: &'me AccountInfo<'info>,
    pub base_token_metadata: &'me AccountInfo<'info>,
    pub market_base_token_ata: &'me AccountInfo<'info>,
    pub quote_token_badge: &'me AccountInfo<'info>,
    pub quote_token_mint: &'me AccountInfo<'info>,
    pub creator: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub token_metadata_program: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CreateMarketWithSplKeys {
    pub config: Pubkey,
    pub market: Pubkey,
    pub base_token_mint: Pubkey,
    pub base_token_metadata: Pubkey,
    pub market_base_token_ata: Pubkey,
    pub quote_token_badge: Pubkey,
    pub quote_token_mint: Pubkey,
    pub creator: Pubkey,
    pub system_program: Pubkey,
    pub token_program: Pubkey,
    pub token_metadata_program: Pubkey,
    pub associated_token_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<CreateMarketWithSplAccounts<'_, '_>> for CreateMarketWithSplKeys {
    fn from(accounts: CreateMarketWithSplAccounts) -> Self {
        Self {
            config: *accounts.config.key,
            market: *accounts.market.key,
            base_token_mint: *accounts.base_token_mint.key,
            base_token_metadata: *accounts.base_token_metadata.key,
            market_base_token_ata: *accounts.market_base_token_ata.key,
            quote_token_badge: *accounts.quote_token_badge.key,
            quote_token_mint: *accounts.quote_token_mint.key,
            creator: *accounts.creator.key,
            system_program: *accounts.system_program.key,
            token_program: *accounts.token_program.key,
            token_metadata_program: *accounts.token_metadata_program.key,
            associated_token_program: *accounts.associated_token_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<CreateMarketWithSplKeys>
for [AccountMeta; CREATE_MARKET_WITH_SPL_IX_ACCOUNTS_LEN] {
    fn from(keys: CreateMarketWithSplKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.market,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.base_token_mint,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.base_token_metadata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.market_base_token_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.quote_token_badge,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.quote_token_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.creator,
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
impl From<[Pubkey; CREATE_MARKET_WITH_SPL_IX_ACCOUNTS_LEN]> for CreateMarketWithSplKeys {
    fn from(pubkeys: [Pubkey; CREATE_MARKET_WITH_SPL_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            config: pubkeys[0],
            market: pubkeys[1],
            base_token_mint: pubkeys[2],
            base_token_metadata: pubkeys[3],
            market_base_token_ata: pubkeys[4],
            quote_token_badge: pubkeys[5],
            quote_token_mint: pubkeys[6],
            creator: pubkeys[7],
            system_program: pubkeys[8],
            token_program: pubkeys[9],
            token_metadata_program: pubkeys[10],
            associated_token_program: pubkeys[11],
            event_authority: pubkeys[12],
            program: pubkeys[13],
        }
    }
}
impl<'info> From<CreateMarketWithSplAccounts<'_, 'info>>
for [AccountInfo<'info>; CREATE_MARKET_WITH_SPL_IX_ACCOUNTS_LEN] {
    fn from(accounts: CreateMarketWithSplAccounts<'_, 'info>) -> Self {
        [
            accounts.config.clone(),
            accounts.market.clone(),
            accounts.base_token_mint.clone(),
            accounts.base_token_metadata.clone(),
            accounts.market_base_token_ata.clone(),
            accounts.quote_token_badge.clone(),
            accounts.quote_token_mint.clone(),
            accounts.creator.clone(),
            accounts.system_program.clone(),
            accounts.token_program.clone(),
            accounts.token_metadata_program.clone(),
            accounts.associated_token_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CREATE_MARKET_WITH_SPL_IX_ACCOUNTS_LEN]>
for CreateMarketWithSplAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; CREATE_MARKET_WITH_SPL_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            config: &arr[0],
            market: &arr[1],
            base_token_mint: &arr[2],
            base_token_metadata: &arr[3],
            market_base_token_ata: &arr[4],
            quote_token_badge: &arr[5],
            quote_token_mint: &arr[6],
            creator: &arr[7],
            system_program: &arr[8],
            token_program: &arr[9],
            token_metadata_program: &arr[10],
            associated_token_program: &arr[11],
            event_authority: &arr[12],
            program: &arr[13],
        }
    }
}
pub const CREATE_MARKET_WITH_SPL_IX_DISCM: [u8; 8usize] = [
    75, 117, 88, 13, 142, 106, 70, 82,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CreateMarketWithSplIxArgs {
    pub name: String,
    pub symbol: String,
    pub uri: String,
    pub total_supply: u64,
    pub creator_fee_share: u16,
    pub staking_fee_share: u16,
}
#[derive(Clone, Debug, PartialEq)]
pub struct CreateMarketWithSplIxData(pub CreateMarketWithSplIxArgs);
impl From<CreateMarketWithSplIxArgs> for CreateMarketWithSplIxData {
    fn from(args: CreateMarketWithSplIxArgs) -> Self {
        Self(args)
    }
}
impl CreateMarketWithSplIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CREATE_MARKET_WITH_SPL_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let name: String = crate::borsh_de_or_default(&mut reader)?;
        let symbol: String = crate::borsh_de_or_default(&mut reader)?;
        let uri: String = crate::borsh_de_or_default(&mut reader)?;
        let total_supply: u64 = crate::borsh_de_or_default(&mut reader)?;
        let creator_fee_share: u16 = crate::borsh_de_or_default(&mut reader)?;
        let staking_fee_share: u16 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(CreateMarketWithSplIxArgs {
                name,
                symbol,
                uri,
                total_supply,
                creator_fee_share,
                staking_fee_share,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CREATE_MARKET_WITH_SPL_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.name, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.symbol, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.uri, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.total_supply, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.creator_fee_share, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.staking_fee_share, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn create_market_with_spl_ix_with_program_id(
    program_id: Pubkey,
    keys: CreateMarketWithSplKeys,
    args: CreateMarketWithSplIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CREATE_MARKET_WITH_SPL_IX_ACCOUNTS_LEN] = keys.into();
    let data: CreateMarketWithSplIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn create_market_with_spl_ix(
    keys: CreateMarketWithSplKeys,
    args: CreateMarketWithSplIxArgs,
) -> std::io::Result<Instruction> {
    create_market_with_spl_ix_with_program_id(TOKEN_MILL_V2_PROGRAM_ID, keys, args)
}
pub fn create_market_with_spl_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CreateMarketWithSplAccounts<'_, '_>,
    args: CreateMarketWithSplIxArgs,
) -> ProgramResult {
    let keys: CreateMarketWithSplKeys = accounts.into();
    let ix = create_market_with_spl_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn create_market_with_spl_invoke(
    accounts: CreateMarketWithSplAccounts<'_, '_>,
    args: CreateMarketWithSplIxArgs,
) -> ProgramResult {
    create_market_with_spl_invoke_with_program_id(
        TOKEN_MILL_V2_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn create_market_with_spl_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CreateMarketWithSplAccounts<'_, '_>,
    args: CreateMarketWithSplIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CreateMarketWithSplKeys = accounts.into();
    let ix = create_market_with_spl_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn create_market_with_spl_invoke_signed(
    accounts: CreateMarketWithSplAccounts<'_, '_>,
    args: CreateMarketWithSplIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    create_market_with_spl_invoke_signed_with_program_id(
        TOKEN_MILL_V2_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn create_market_with_spl_verify_account_keys(
    accounts: CreateMarketWithSplAccounts<'_, '_>,
    keys: CreateMarketWithSplKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.config.key, keys.config),
        (*accounts.market.key, keys.market),
        (*accounts.base_token_mint.key, keys.base_token_mint),
        (*accounts.base_token_metadata.key, keys.base_token_metadata),
        (*accounts.market_base_token_ata.key, keys.market_base_token_ata),
        (*accounts.quote_token_badge.key, keys.quote_token_badge),
        (*accounts.quote_token_mint.key, keys.quote_token_mint),
        (*accounts.creator.key, keys.creator),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.token_metadata_program.key, keys.token_metadata_program),
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
pub fn create_market_with_spl_verify_writable_privileges<'me, 'info>(
    accounts: CreateMarketWithSplAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.market,
        accounts.base_token_mint,
        accounts.base_token_metadata,
        accounts.market_base_token_ata,
        accounts.creator,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn create_market_with_spl_verify_signer_privileges<'me, 'info>(
    accounts: CreateMarketWithSplAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.base_token_mint, accounts.creator] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn create_market_with_spl_verify_account_privileges<'me, 'info>(
    accounts: CreateMarketWithSplAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    create_market_with_spl_verify_writable_privileges(accounts)?;
    create_market_with_spl_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CREATE_QUOTE_ASSET_BADGE_IX_ACCOUNTS_LEN: usize = 7;
#[derive(Copy, Clone, Debug)]
pub struct CreateQuoteAssetBadgeAccounts<'me, 'info> {
    pub config: &'me AccountInfo<'info>,
    pub quote_asset_badge: &'me AccountInfo<'info>,
    pub token_mint: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CreateQuoteAssetBadgeKeys {
    pub config: Pubkey,
    pub quote_asset_badge: Pubkey,
    pub token_mint: Pubkey,
    pub authority: Pubkey,
    pub system_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<CreateQuoteAssetBadgeAccounts<'_, '_>> for CreateQuoteAssetBadgeKeys {
    fn from(accounts: CreateQuoteAssetBadgeAccounts) -> Self {
        Self {
            config: *accounts.config.key,
            quote_asset_badge: *accounts.quote_asset_badge.key,
            token_mint: *accounts.token_mint.key,
            authority: *accounts.authority.key,
            system_program: *accounts.system_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<CreateQuoteAssetBadgeKeys>
for [AccountMeta; CREATE_QUOTE_ASSET_BADGE_IX_ACCOUNTS_LEN] {
    fn from(keys: CreateQuoteAssetBadgeKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.config,
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
                pubkey: keys.authority,
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
impl From<[Pubkey; CREATE_QUOTE_ASSET_BADGE_IX_ACCOUNTS_LEN]>
for CreateQuoteAssetBadgeKeys {
    fn from(pubkeys: [Pubkey; CREATE_QUOTE_ASSET_BADGE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            config: pubkeys[0],
            quote_asset_badge: pubkeys[1],
            token_mint: pubkeys[2],
            authority: pubkeys[3],
            system_program: pubkeys[4],
            event_authority: pubkeys[5],
            program: pubkeys[6],
        }
    }
}
impl<'info> From<CreateQuoteAssetBadgeAccounts<'_, 'info>>
for [AccountInfo<'info>; CREATE_QUOTE_ASSET_BADGE_IX_ACCOUNTS_LEN] {
    fn from(accounts: CreateQuoteAssetBadgeAccounts<'_, 'info>) -> Self {
        [
            accounts.config.clone(),
            accounts.quote_asset_badge.clone(),
            accounts.token_mint.clone(),
            accounts.authority.clone(),
            accounts.system_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; CREATE_QUOTE_ASSET_BADGE_IX_ACCOUNTS_LEN]>
for CreateQuoteAssetBadgeAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; CREATE_QUOTE_ASSET_BADGE_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            config: &arr[0],
            quote_asset_badge: &arr[1],
            token_mint: &arr[2],
            authority: &arr[3],
            system_program: &arr[4],
            event_authority: &arr[5],
            program: &arr[6],
        }
    }
}
pub const CREATE_QUOTE_ASSET_BADGE_IX_DISCM: [u8; 8usize] = [
    224, 76, 142, 221, 109, 134, 164, 74,
];
#[derive(Clone, Debug, PartialEq)]
pub struct CreateQuoteAssetBadgeIxData;
impl CreateQuoteAssetBadgeIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CREATE_QUOTE_ASSET_BADGE_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CREATE_QUOTE_ASSET_BADGE_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn create_quote_asset_badge_ix_with_program_id(
    program_id: Pubkey,
    keys: CreateQuoteAssetBadgeKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CREATE_QUOTE_ASSET_BADGE_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: CreateQuoteAssetBadgeIxData.try_to_vec()?,
    })
}
pub fn create_quote_asset_badge_ix(
    keys: CreateQuoteAssetBadgeKeys,
) -> std::io::Result<Instruction> {
    create_quote_asset_badge_ix_with_program_id(TOKEN_MILL_V2_PROGRAM_ID, keys)
}
pub fn create_quote_asset_badge_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CreateQuoteAssetBadgeAccounts<'_, '_>,
) -> ProgramResult {
    let keys: CreateQuoteAssetBadgeKeys = accounts.into();
    let ix = create_quote_asset_badge_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn create_quote_asset_badge_invoke(
    accounts: CreateQuoteAssetBadgeAccounts<'_, '_>,
) -> ProgramResult {
    create_quote_asset_badge_invoke_with_program_id(TOKEN_MILL_V2_PROGRAM_ID, accounts)
}
pub fn create_quote_asset_badge_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CreateQuoteAssetBadgeAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CreateQuoteAssetBadgeKeys = accounts.into();
    let ix = create_quote_asset_badge_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn create_quote_asset_badge_invoke_signed(
    accounts: CreateQuoteAssetBadgeAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    create_quote_asset_badge_invoke_signed_with_program_id(
        TOKEN_MILL_V2_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn create_quote_asset_badge_verify_account_keys(
    accounts: CreateQuoteAssetBadgeAccounts<'_, '_>,
    keys: CreateQuoteAssetBadgeKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.config.key, keys.config),
        (*accounts.quote_asset_badge.key, keys.quote_asset_badge),
        (*accounts.token_mint.key, keys.token_mint),
        (*accounts.authority.key, keys.authority),
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
pub fn create_quote_asset_badge_verify_writable_privileges<'me, 'info>(
    accounts: CreateQuoteAssetBadgeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.quote_asset_badge, accounts.authority] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn create_quote_asset_badge_verify_signer_privileges<'me, 'info>(
    accounts: CreateQuoteAssetBadgeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn create_quote_asset_badge_verify_account_privileges<'me, 'info>(
    accounts: CreateQuoteAssetBadgeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    create_quote_asset_badge_verify_writable_privileges(accounts)?;
    create_quote_asset_badge_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CREATE_REFERRAL_ACCOUNT_IX_ACCOUNTS_LEN: usize = 4;
#[derive(Copy, Clone, Debug)]
pub struct CreateReferralAccountAccounts<'me, 'info> {
    pub config: &'me AccountInfo<'info>,
    pub referral_account: &'me AccountInfo<'info>,
    pub user: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CreateReferralAccountKeys {
    pub config: Pubkey,
    pub referral_account: Pubkey,
    pub user: Pubkey,
    pub system_program: Pubkey,
}
impl From<CreateReferralAccountAccounts<'_, '_>> for CreateReferralAccountKeys {
    fn from(accounts: CreateReferralAccountAccounts) -> Self {
        Self {
            config: *accounts.config.key,
            referral_account: *accounts.referral_account.key,
            user: *accounts.user.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<CreateReferralAccountKeys>
for [AccountMeta; CREATE_REFERRAL_ACCOUNT_IX_ACCOUNTS_LEN] {
    fn from(keys: CreateReferralAccountKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.referral_account,
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
        ]
    }
}
impl From<[Pubkey; CREATE_REFERRAL_ACCOUNT_IX_ACCOUNTS_LEN]>
for CreateReferralAccountKeys {
    fn from(pubkeys: [Pubkey; CREATE_REFERRAL_ACCOUNT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            config: pubkeys[0],
            referral_account: pubkeys[1],
            user: pubkeys[2],
            system_program: pubkeys[3],
        }
    }
}
impl<'info> From<CreateReferralAccountAccounts<'_, 'info>>
for [AccountInfo<'info>; CREATE_REFERRAL_ACCOUNT_IX_ACCOUNTS_LEN] {
    fn from(accounts: CreateReferralAccountAccounts<'_, 'info>) -> Self {
        [
            accounts.config.clone(),
            accounts.referral_account.clone(),
            accounts.user.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CREATE_REFERRAL_ACCOUNT_IX_ACCOUNTS_LEN]>
for CreateReferralAccountAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; CREATE_REFERRAL_ACCOUNT_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            config: &arr[0],
            referral_account: &arr[1],
            user: &arr[2],
            system_program: &arr[3],
        }
    }
}
pub const CREATE_REFERRAL_ACCOUNT_IX_DISCM: [u8; 8usize] = [
    235, 55, 82, 230, 52, 35, 56, 210,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CreateReferralAccountIxArgs {
    pub referrer: Pubkey,
}
#[derive(Clone, Debug, PartialEq)]
pub struct CreateReferralAccountIxData(pub CreateReferralAccountIxArgs);
impl From<CreateReferralAccountIxArgs> for CreateReferralAccountIxData {
    fn from(args: CreateReferralAccountIxArgs) -> Self {
        Self(args)
    }
}
impl CreateReferralAccountIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CREATE_REFERRAL_ACCOUNT_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let referrer: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(CreateReferralAccountIxArgs {
                referrer,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CREATE_REFERRAL_ACCOUNT_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.referrer, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn create_referral_account_ix_with_program_id(
    program_id: Pubkey,
    keys: CreateReferralAccountKeys,
    args: CreateReferralAccountIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CREATE_REFERRAL_ACCOUNT_IX_ACCOUNTS_LEN] = keys.into();
    let data: CreateReferralAccountIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn create_referral_account_ix(
    keys: CreateReferralAccountKeys,
    args: CreateReferralAccountIxArgs,
) -> std::io::Result<Instruction> {
    create_referral_account_ix_with_program_id(TOKEN_MILL_V2_PROGRAM_ID, keys, args)
}
pub fn create_referral_account_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CreateReferralAccountAccounts<'_, '_>,
    args: CreateReferralAccountIxArgs,
) -> ProgramResult {
    let keys: CreateReferralAccountKeys = accounts.into();
    let ix = create_referral_account_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn create_referral_account_invoke(
    accounts: CreateReferralAccountAccounts<'_, '_>,
    args: CreateReferralAccountIxArgs,
) -> ProgramResult {
    create_referral_account_invoke_with_program_id(
        TOKEN_MILL_V2_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn create_referral_account_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CreateReferralAccountAccounts<'_, '_>,
    args: CreateReferralAccountIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CreateReferralAccountKeys = accounts.into();
    let ix = create_referral_account_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn create_referral_account_invoke_signed(
    accounts: CreateReferralAccountAccounts<'_, '_>,
    args: CreateReferralAccountIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    create_referral_account_invoke_signed_with_program_id(
        TOKEN_MILL_V2_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn create_referral_account_verify_account_keys(
    accounts: CreateReferralAccountAccounts<'_, '_>,
    keys: CreateReferralAccountKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.config.key, keys.config),
        (*accounts.referral_account.key, keys.referral_account),
        (*accounts.user.key, keys.user),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn create_referral_account_verify_writable_privileges<'me, 'info>(
    accounts: CreateReferralAccountAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.referral_account, accounts.user] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn create_referral_account_verify_signer_privileges<'me, 'info>(
    accounts: CreateReferralAccountAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.user] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn create_referral_account_verify_account_privileges<'me, 'info>(
    accounts: CreateReferralAccountAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    create_referral_account_verify_writable_privileges(accounts)?;
    create_referral_account_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CREATE_STAKE_POSITION_IX_ACCOUNTS_LEN: usize = 4;
#[derive(Copy, Clone, Debug)]
pub struct CreateStakePositionAccounts<'me, 'info> {
    pub market: &'me AccountInfo<'info>,
    pub stake_position: &'me AccountInfo<'info>,
    pub user: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CreateStakePositionKeys {
    pub market: Pubkey,
    pub stake_position: Pubkey,
    pub user: Pubkey,
    pub system_program: Pubkey,
}
impl From<CreateStakePositionAccounts<'_, '_>> for CreateStakePositionKeys {
    fn from(accounts: CreateStakePositionAccounts) -> Self {
        Self {
            market: *accounts.market.key,
            stake_position: *accounts.stake_position.key,
            user: *accounts.user.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<CreateStakePositionKeys>
for [AccountMeta; CREATE_STAKE_POSITION_IX_ACCOUNTS_LEN] {
    fn from(keys: CreateStakePositionKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.market,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.stake_position,
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
        ]
    }
}
impl From<[Pubkey; CREATE_STAKE_POSITION_IX_ACCOUNTS_LEN]> for CreateStakePositionKeys {
    fn from(pubkeys: [Pubkey; CREATE_STAKE_POSITION_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            market: pubkeys[0],
            stake_position: pubkeys[1],
            user: pubkeys[2],
            system_program: pubkeys[3],
        }
    }
}
impl<'info> From<CreateStakePositionAccounts<'_, 'info>>
for [AccountInfo<'info>; CREATE_STAKE_POSITION_IX_ACCOUNTS_LEN] {
    fn from(accounts: CreateStakePositionAccounts<'_, 'info>) -> Self {
        [
            accounts.market.clone(),
            accounts.stake_position.clone(),
            accounts.user.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CREATE_STAKE_POSITION_IX_ACCOUNTS_LEN]>
for CreateStakePositionAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; CREATE_STAKE_POSITION_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            market: &arr[0],
            stake_position: &arr[1],
            user: &arr[2],
            system_program: &arr[3],
        }
    }
}
pub const CREATE_STAKE_POSITION_IX_DISCM: [u8; 8usize] = [
    92, 168, 96, 133, 102, 121, 86, 138,
];
#[derive(Clone, Debug, PartialEq)]
pub struct CreateStakePositionIxData;
impl CreateStakePositionIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CREATE_STAKE_POSITION_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CREATE_STAKE_POSITION_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn create_stake_position_ix_with_program_id(
    program_id: Pubkey,
    keys: CreateStakePositionKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CREATE_STAKE_POSITION_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: CreateStakePositionIxData.try_to_vec()?,
    })
}
pub fn create_stake_position_ix(
    keys: CreateStakePositionKeys,
) -> std::io::Result<Instruction> {
    create_stake_position_ix_with_program_id(TOKEN_MILL_V2_PROGRAM_ID, keys)
}
pub fn create_stake_position_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CreateStakePositionAccounts<'_, '_>,
) -> ProgramResult {
    let keys: CreateStakePositionKeys = accounts.into();
    let ix = create_stake_position_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn create_stake_position_invoke(
    accounts: CreateStakePositionAccounts<'_, '_>,
) -> ProgramResult {
    create_stake_position_invoke_with_program_id(TOKEN_MILL_V2_PROGRAM_ID, accounts)
}
pub fn create_stake_position_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CreateStakePositionAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CreateStakePositionKeys = accounts.into();
    let ix = create_stake_position_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn create_stake_position_invoke_signed(
    accounts: CreateStakePositionAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    create_stake_position_invoke_signed_with_program_id(
        TOKEN_MILL_V2_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn create_stake_position_verify_account_keys(
    accounts: CreateStakePositionAccounts<'_, '_>,
    keys: CreateStakePositionKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.market.key, keys.market),
        (*accounts.stake_position.key, keys.stake_position),
        (*accounts.user.key, keys.user),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn create_stake_position_verify_writable_privileges<'me, 'info>(
    accounts: CreateStakePositionAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.stake_position, accounts.user] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn create_stake_position_verify_signer_privileges<'me, 'info>(
    accounts: CreateStakePositionAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.user] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn create_stake_position_verify_account_privileges<'me, 'info>(
    accounts: CreateStakePositionAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    create_stake_position_verify_writable_privileges(accounts)?;
    create_stake_position_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CREATE_STAKING_IX_ACCOUNTS_LEN: usize = 4;
#[derive(Copy, Clone, Debug)]
pub struct CreateStakingAccounts<'me, 'info> {
    pub market: &'me AccountInfo<'info>,
    pub staking: &'me AccountInfo<'info>,
    pub payer: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CreateStakingKeys {
    pub market: Pubkey,
    pub staking: Pubkey,
    pub payer: Pubkey,
    pub system_program: Pubkey,
}
impl From<CreateStakingAccounts<'_, '_>> for CreateStakingKeys {
    fn from(accounts: CreateStakingAccounts) -> Self {
        Self {
            market: *accounts.market.key,
            staking: *accounts.staking.key,
            payer: *accounts.payer.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<CreateStakingKeys> for [AccountMeta; CREATE_STAKING_IX_ACCOUNTS_LEN] {
    fn from(keys: CreateStakingKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.market,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.staking,
                is_signer: false,
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
        ]
    }
}
impl From<[Pubkey; CREATE_STAKING_IX_ACCOUNTS_LEN]> for CreateStakingKeys {
    fn from(pubkeys: [Pubkey; CREATE_STAKING_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            market: pubkeys[0],
            staking: pubkeys[1],
            payer: pubkeys[2],
            system_program: pubkeys[3],
        }
    }
}
impl<'info> From<CreateStakingAccounts<'_, 'info>>
for [AccountInfo<'info>; CREATE_STAKING_IX_ACCOUNTS_LEN] {
    fn from(accounts: CreateStakingAccounts<'_, 'info>) -> Self {
        [
            accounts.market.clone(),
            accounts.staking.clone(),
            accounts.payer.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CREATE_STAKING_IX_ACCOUNTS_LEN]>
for CreateStakingAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; CREATE_STAKING_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            market: &arr[0],
            staking: &arr[1],
            payer: &arr[2],
            system_program: &arr[3],
        }
    }
}
pub const CREATE_STAKING_IX_DISCM: [u8; 8usize] = [184, 219, 61, 66, 140, 212, 112, 133];
#[derive(Clone, Debug, PartialEq)]
pub struct CreateStakingIxData;
impl CreateStakingIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CREATE_STAKING_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CREATE_STAKING_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn create_staking_ix_with_program_id(
    program_id: Pubkey,
    keys: CreateStakingKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CREATE_STAKING_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: CreateStakingIxData.try_to_vec()?,
    })
}
pub fn create_staking_ix(keys: CreateStakingKeys) -> std::io::Result<Instruction> {
    create_staking_ix_with_program_id(TOKEN_MILL_V2_PROGRAM_ID, keys)
}
pub fn create_staking_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CreateStakingAccounts<'_, '_>,
) -> ProgramResult {
    let keys: CreateStakingKeys = accounts.into();
    let ix = create_staking_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn create_staking_invoke(accounts: CreateStakingAccounts<'_, '_>) -> ProgramResult {
    create_staking_invoke_with_program_id(TOKEN_MILL_V2_PROGRAM_ID, accounts)
}
pub fn create_staking_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CreateStakingAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CreateStakingKeys = accounts.into();
    let ix = create_staking_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn create_staking_invoke_signed(
    accounts: CreateStakingAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    create_staking_invoke_signed_with_program_id(
        TOKEN_MILL_V2_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn create_staking_verify_account_keys(
    accounts: CreateStakingAccounts<'_, '_>,
    keys: CreateStakingKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.market.key, keys.market),
        (*accounts.staking.key, keys.staking),
        (*accounts.payer.key, keys.payer),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn create_staking_verify_writable_privileges<'me, 'info>(
    accounts: CreateStakingAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.staking, accounts.payer] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn create_staking_verify_signer_privileges<'me, 'info>(
    accounts: CreateStakingAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.payer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn create_staking_verify_account_privileges<'me, 'info>(
    accounts: CreateStakingAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    create_staking_verify_writable_privileges(accounts)?;
    create_staking_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CREATE_VESTING_PLAN_IX_ACCOUNTS_LEN: usize = 12;
#[derive(Copy, Clone, Debug)]
pub struct CreateVestingPlanAccounts<'me, 'info> {
    pub market: &'me AccountInfo<'info>,
    pub staking: &'me AccountInfo<'info>,
    pub stake_position: &'me AccountInfo<'info>,
    pub vesting_plan: &'me AccountInfo<'info>,
    pub base_token_mint: &'me AccountInfo<'info>,
    pub market_base_token_ata: &'me AccountInfo<'info>,
    pub user_base_token_ata: &'me AccountInfo<'info>,
    pub user: &'me AccountInfo<'info>,
    pub base_token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CreateVestingPlanKeys {
    pub market: Pubkey,
    pub staking: Pubkey,
    pub stake_position: Pubkey,
    pub vesting_plan: Pubkey,
    pub base_token_mint: Pubkey,
    pub market_base_token_ata: Pubkey,
    pub user_base_token_ata: Pubkey,
    pub user: Pubkey,
    pub base_token_program: Pubkey,
    pub system_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<CreateVestingPlanAccounts<'_, '_>> for CreateVestingPlanKeys {
    fn from(accounts: CreateVestingPlanAccounts) -> Self {
        Self {
            market: *accounts.market.key,
            staking: *accounts.staking.key,
            stake_position: *accounts.stake_position.key,
            vesting_plan: *accounts.vesting_plan.key,
            base_token_mint: *accounts.base_token_mint.key,
            market_base_token_ata: *accounts.market_base_token_ata.key,
            user_base_token_ata: *accounts.user_base_token_ata.key,
            user: *accounts.user.key,
            base_token_program: *accounts.base_token_program.key,
            system_program: *accounts.system_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<CreateVestingPlanKeys> for [AccountMeta; CREATE_VESTING_PLAN_IX_ACCOUNTS_LEN] {
    fn from(keys: CreateVestingPlanKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.market,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.staking,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.stake_position,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vesting_plan,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.base_token_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.market_base_token_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_base_token_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.base_token_program,
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
impl From<[Pubkey; CREATE_VESTING_PLAN_IX_ACCOUNTS_LEN]> for CreateVestingPlanKeys {
    fn from(pubkeys: [Pubkey; CREATE_VESTING_PLAN_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            market: pubkeys[0],
            staking: pubkeys[1],
            stake_position: pubkeys[2],
            vesting_plan: pubkeys[3],
            base_token_mint: pubkeys[4],
            market_base_token_ata: pubkeys[5],
            user_base_token_ata: pubkeys[6],
            user: pubkeys[7],
            base_token_program: pubkeys[8],
            system_program: pubkeys[9],
            event_authority: pubkeys[10],
            program: pubkeys[11],
        }
    }
}
impl<'info> From<CreateVestingPlanAccounts<'_, 'info>>
for [AccountInfo<'info>; CREATE_VESTING_PLAN_IX_ACCOUNTS_LEN] {
    fn from(accounts: CreateVestingPlanAccounts<'_, 'info>) -> Self {
        [
            accounts.market.clone(),
            accounts.staking.clone(),
            accounts.stake_position.clone(),
            accounts.vesting_plan.clone(),
            accounts.base_token_mint.clone(),
            accounts.market_base_token_ata.clone(),
            accounts.user_base_token_ata.clone(),
            accounts.user.clone(),
            accounts.base_token_program.clone(),
            accounts.system_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CREATE_VESTING_PLAN_IX_ACCOUNTS_LEN]>
for CreateVestingPlanAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; CREATE_VESTING_PLAN_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            market: &arr[0],
            staking: &arr[1],
            stake_position: &arr[2],
            vesting_plan: &arr[3],
            base_token_mint: &arr[4],
            market_base_token_ata: &arr[5],
            user_base_token_ata: &arr[6],
            user: &arr[7],
            base_token_program: &arr[8],
            system_program: &arr[9],
            event_authority: &arr[10],
            program: &arr[11],
        }
    }
}
pub const CREATE_VESTING_PLAN_IX_DISCM: [u8; 8usize] = [
    243, 11, 234, 132, 14, 178, 152, 158,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CreateVestingPlanIxArgs {
    pub start: i64,
    pub vesting_amount: u64,
    pub vesting_duration: i64,
    pub cliff_duration: i64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct CreateVestingPlanIxData(pub CreateVestingPlanIxArgs);
impl From<CreateVestingPlanIxArgs> for CreateVestingPlanIxData {
    fn from(args: CreateVestingPlanIxArgs) -> Self {
        Self(args)
    }
}
impl CreateVestingPlanIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CREATE_VESTING_PLAN_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let start: i64 = crate::borsh_de_or_default(&mut reader)?;
        let vesting_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let vesting_duration: i64 = crate::borsh_de_or_default(&mut reader)?;
        let cliff_duration: i64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(CreateVestingPlanIxArgs {
                start,
                vesting_amount,
                vesting_duration,
                cliff_duration,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CREATE_VESTING_PLAN_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.start, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.vesting_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.vesting_duration, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.cliff_duration, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn create_vesting_plan_ix_with_program_id(
    program_id: Pubkey,
    keys: CreateVestingPlanKeys,
    args: CreateVestingPlanIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CREATE_VESTING_PLAN_IX_ACCOUNTS_LEN] = keys.into();
    let data: CreateVestingPlanIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn create_vesting_plan_ix(
    keys: CreateVestingPlanKeys,
    args: CreateVestingPlanIxArgs,
) -> std::io::Result<Instruction> {
    create_vesting_plan_ix_with_program_id(TOKEN_MILL_V2_PROGRAM_ID, keys, args)
}
pub fn create_vesting_plan_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CreateVestingPlanAccounts<'_, '_>,
    args: CreateVestingPlanIxArgs,
) -> ProgramResult {
    let keys: CreateVestingPlanKeys = accounts.into();
    let ix = create_vesting_plan_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn create_vesting_plan_invoke(
    accounts: CreateVestingPlanAccounts<'_, '_>,
    args: CreateVestingPlanIxArgs,
) -> ProgramResult {
    create_vesting_plan_invoke_with_program_id(TOKEN_MILL_V2_PROGRAM_ID, accounts, args)
}
pub fn create_vesting_plan_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CreateVestingPlanAccounts<'_, '_>,
    args: CreateVestingPlanIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CreateVestingPlanKeys = accounts.into();
    let ix = create_vesting_plan_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn create_vesting_plan_invoke_signed(
    accounts: CreateVestingPlanAccounts<'_, '_>,
    args: CreateVestingPlanIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    create_vesting_plan_invoke_signed_with_program_id(
        TOKEN_MILL_V2_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn create_vesting_plan_verify_account_keys(
    accounts: CreateVestingPlanAccounts<'_, '_>,
    keys: CreateVestingPlanKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.market.key, keys.market),
        (*accounts.staking.key, keys.staking),
        (*accounts.stake_position.key, keys.stake_position),
        (*accounts.vesting_plan.key, keys.vesting_plan),
        (*accounts.base_token_mint.key, keys.base_token_mint),
        (*accounts.market_base_token_ata.key, keys.market_base_token_ata),
        (*accounts.user_base_token_ata.key, keys.user_base_token_ata),
        (*accounts.user.key, keys.user),
        (*accounts.base_token_program.key, keys.base_token_program),
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
pub fn create_vesting_plan_verify_writable_privileges<'me, 'info>(
    accounts: CreateVestingPlanAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.market,
        accounts.staking,
        accounts.stake_position,
        accounts.vesting_plan,
        accounts.market_base_token_ata,
        accounts.user_base_token_ata,
        accounts.user,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn create_vesting_plan_verify_signer_privileges<'me, 'info>(
    accounts: CreateVestingPlanAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.vesting_plan, accounts.user] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn create_vesting_plan_verify_account_privileges<'me, 'info>(
    accounts: CreateVestingPlanAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    create_vesting_plan_verify_writable_privileges(accounts)?;
    create_vesting_plan_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const DEPOSIT_IX_ACCOUNTS_LEN: usize = 10;
#[derive(Copy, Clone, Debug)]
pub struct DepositAccounts<'me, 'info> {
    pub market: &'me AccountInfo<'info>,
    pub staking: &'me AccountInfo<'info>,
    pub stake_position: &'me AccountInfo<'info>,
    pub base_token_mint: &'me AccountInfo<'info>,
    pub market_base_token_ata: &'me AccountInfo<'info>,
    pub user_base_token_ata: &'me AccountInfo<'info>,
    pub user: &'me AccountInfo<'info>,
    pub base_token_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct DepositKeys {
    pub market: Pubkey,
    pub staking: Pubkey,
    pub stake_position: Pubkey,
    pub base_token_mint: Pubkey,
    pub market_base_token_ata: Pubkey,
    pub user_base_token_ata: Pubkey,
    pub user: Pubkey,
    pub base_token_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<DepositAccounts<'_, '_>> for DepositKeys {
    fn from(accounts: DepositAccounts) -> Self {
        Self {
            market: *accounts.market.key,
            staking: *accounts.staking.key,
            stake_position: *accounts.stake_position.key,
            base_token_mint: *accounts.base_token_mint.key,
            market_base_token_ata: *accounts.market_base_token_ata.key,
            user_base_token_ata: *accounts.user_base_token_ata.key,
            user: *accounts.user.key,
            base_token_program: *accounts.base_token_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<DepositKeys> for [AccountMeta; DEPOSIT_IX_ACCOUNTS_LEN] {
    fn from(keys: DepositKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.market,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.staking,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.stake_position,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.base_token_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.market_base_token_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_base_token_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.base_token_program,
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
impl From<[Pubkey; DEPOSIT_IX_ACCOUNTS_LEN]> for DepositKeys {
    fn from(pubkeys: [Pubkey; DEPOSIT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            market: pubkeys[0],
            staking: pubkeys[1],
            stake_position: pubkeys[2],
            base_token_mint: pubkeys[3],
            market_base_token_ata: pubkeys[4],
            user_base_token_ata: pubkeys[5],
            user: pubkeys[6],
            base_token_program: pubkeys[7],
            event_authority: pubkeys[8],
            program: pubkeys[9],
        }
    }
}
impl<'info> From<DepositAccounts<'_, 'info>>
for [AccountInfo<'info>; DEPOSIT_IX_ACCOUNTS_LEN] {
    fn from(accounts: DepositAccounts<'_, 'info>) -> Self {
        [
            accounts.market.clone(),
            accounts.staking.clone(),
            accounts.stake_position.clone(),
            accounts.base_token_mint.clone(),
            accounts.market_base_token_ata.clone(),
            accounts.user_base_token_ata.clone(),
            accounts.user.clone(),
            accounts.base_token_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; DEPOSIT_IX_ACCOUNTS_LEN]>
for DepositAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; DEPOSIT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            market: &arr[0],
            staking: &arr[1],
            stake_position: &arr[2],
            base_token_mint: &arr[3],
            market_base_token_ata: &arr[4],
            user_base_token_ata: &arr[5],
            user: &arr[6],
            base_token_program: &arr[7],
            event_authority: &arr[8],
            program: &arr[9],
        }
    }
}
pub const DEPOSIT_IX_DISCM: [u8; 8usize] = [242, 35, 198, 137, 82, 225, 242, 182];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DepositIxArgs {
    pub amount: u64,
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
        let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(Self(DepositIxArgs { amount }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&DEPOSIT_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.amount, &mut writer)?;
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
    deposit_ix_with_program_id(TOKEN_MILL_V2_PROGRAM_ID, keys, args)
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
    deposit_invoke_with_program_id(TOKEN_MILL_V2_PROGRAM_ID, accounts, args)
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
        TOKEN_MILL_V2_PROGRAM_ID,
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
        (*accounts.market.key, keys.market),
        (*accounts.staking.key, keys.staking),
        (*accounts.stake_position.key, keys.stake_position),
        (*accounts.base_token_mint.key, keys.base_token_mint),
        (*accounts.market_base_token_ata.key, keys.market_base_token_ata),
        (*accounts.user_base_token_ata.key, keys.user_base_token_ata),
        (*accounts.user.key, keys.user),
        (*accounts.base_token_program.key, keys.base_token_program),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
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
        accounts.market,
        accounts.staking,
        accounts.stake_position,
        accounts.market_base_token_ata,
        accounts.user_base_token_ata,
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
    for should_be_signer in [accounts.user] {
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
pub const RELEASE_IX_ACCOUNTS_LEN: usize = 11;
#[derive(Copy, Clone, Debug)]
pub struct ReleaseAccounts<'me, 'info> {
    pub market: &'me AccountInfo<'info>,
    pub staking: &'me AccountInfo<'info>,
    pub stake_position: &'me AccountInfo<'info>,
    pub vesting_plan: &'me AccountInfo<'info>,
    pub base_token_mint: &'me AccountInfo<'info>,
    pub market_base_token_ata: &'me AccountInfo<'info>,
    pub user_base_token_ata: &'me AccountInfo<'info>,
    pub user: &'me AccountInfo<'info>,
    pub base_token_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ReleaseKeys {
    pub market: Pubkey,
    pub staking: Pubkey,
    pub stake_position: Pubkey,
    pub vesting_plan: Pubkey,
    pub base_token_mint: Pubkey,
    pub market_base_token_ata: Pubkey,
    pub user_base_token_ata: Pubkey,
    pub user: Pubkey,
    pub base_token_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<ReleaseAccounts<'_, '_>> for ReleaseKeys {
    fn from(accounts: ReleaseAccounts) -> Self {
        Self {
            market: *accounts.market.key,
            staking: *accounts.staking.key,
            stake_position: *accounts.stake_position.key,
            vesting_plan: *accounts.vesting_plan.key,
            base_token_mint: *accounts.base_token_mint.key,
            market_base_token_ata: *accounts.market_base_token_ata.key,
            user_base_token_ata: *accounts.user_base_token_ata.key,
            user: *accounts.user.key,
            base_token_program: *accounts.base_token_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<ReleaseKeys> for [AccountMeta; RELEASE_IX_ACCOUNTS_LEN] {
    fn from(keys: ReleaseKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.market,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.staking,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.stake_position,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vesting_plan,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.base_token_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.market_base_token_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_base_token_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.base_token_program,
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
impl From<[Pubkey; RELEASE_IX_ACCOUNTS_LEN]> for ReleaseKeys {
    fn from(pubkeys: [Pubkey; RELEASE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            market: pubkeys[0],
            staking: pubkeys[1],
            stake_position: pubkeys[2],
            vesting_plan: pubkeys[3],
            base_token_mint: pubkeys[4],
            market_base_token_ata: pubkeys[5],
            user_base_token_ata: pubkeys[6],
            user: pubkeys[7],
            base_token_program: pubkeys[8],
            event_authority: pubkeys[9],
            program: pubkeys[10],
        }
    }
}
impl<'info> From<ReleaseAccounts<'_, 'info>>
for [AccountInfo<'info>; RELEASE_IX_ACCOUNTS_LEN] {
    fn from(accounts: ReleaseAccounts<'_, 'info>) -> Self {
        [
            accounts.market.clone(),
            accounts.staking.clone(),
            accounts.stake_position.clone(),
            accounts.vesting_plan.clone(),
            accounts.base_token_mint.clone(),
            accounts.market_base_token_ata.clone(),
            accounts.user_base_token_ata.clone(),
            accounts.user.clone(),
            accounts.base_token_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; RELEASE_IX_ACCOUNTS_LEN]>
for ReleaseAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; RELEASE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            market: &arr[0],
            staking: &arr[1],
            stake_position: &arr[2],
            vesting_plan: &arr[3],
            base_token_mint: &arr[4],
            market_base_token_ata: &arr[5],
            user_base_token_ata: &arr[6],
            user: &arr[7],
            base_token_program: &arr[8],
            event_authority: &arr[9],
            program: &arr[10],
        }
    }
}
pub const RELEASE_IX_DISCM: [u8; 8usize] = [253, 249, 15, 206, 28, 127, 193, 241];
#[derive(Clone, Debug, PartialEq)]
pub struct ReleaseIxData;
impl ReleaseIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != RELEASE_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&RELEASE_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn release_ix_with_program_id(
    program_id: Pubkey,
    keys: ReleaseKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; RELEASE_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: ReleaseIxData.try_to_vec()?,
    })
}
pub fn release_ix(keys: ReleaseKeys) -> std::io::Result<Instruction> {
    release_ix_with_program_id(TOKEN_MILL_V2_PROGRAM_ID, keys)
}
pub fn release_invoke_with_program_id(
    program_id: Pubkey,
    accounts: ReleaseAccounts<'_, '_>,
) -> ProgramResult {
    let keys: ReleaseKeys = accounts.into();
    let ix = release_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn release_invoke(accounts: ReleaseAccounts<'_, '_>) -> ProgramResult {
    release_invoke_with_program_id(TOKEN_MILL_V2_PROGRAM_ID, accounts)
}
pub fn release_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: ReleaseAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: ReleaseKeys = accounts.into();
    let ix = release_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn release_invoke_signed(
    accounts: ReleaseAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    release_invoke_signed_with_program_id(TOKEN_MILL_V2_PROGRAM_ID, accounts, seeds)
}
pub fn release_verify_account_keys(
    accounts: ReleaseAccounts<'_, '_>,
    keys: ReleaseKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.market.key, keys.market),
        (*accounts.staking.key, keys.staking),
        (*accounts.stake_position.key, keys.stake_position),
        (*accounts.vesting_plan.key, keys.vesting_plan),
        (*accounts.base_token_mint.key, keys.base_token_mint),
        (*accounts.market_base_token_ata.key, keys.market_base_token_ata),
        (*accounts.user_base_token_ata.key, keys.user_base_token_ata),
        (*accounts.user.key, keys.user),
        (*accounts.base_token_program.key, keys.base_token_program),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn release_verify_writable_privileges<'me, 'info>(
    accounts: ReleaseAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.market,
        accounts.staking,
        accounts.stake_position,
        accounts.vesting_plan,
        accounts.market_base_token_ata,
        accounts.user_base_token_ata,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn release_verify_signer_privileges<'me, 'info>(
    accounts: ReleaseAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.user] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn release_verify_account_privileges<'me, 'info>(
    accounts: ReleaseAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    release_verify_writable_privileges(accounts)?;
    release_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SET_MARKET_PRICES_IX_ACCOUNTS_LEN: usize = 4;
#[derive(Copy, Clone, Debug)]
pub struct SetMarketPricesAccounts<'me, 'info> {
    pub market: &'me AccountInfo<'info>,
    pub creator: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SetMarketPricesKeys {
    pub market: Pubkey,
    pub creator: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<SetMarketPricesAccounts<'_, '_>> for SetMarketPricesKeys {
    fn from(accounts: SetMarketPricesAccounts) -> Self {
        Self {
            market: *accounts.market.key,
            creator: *accounts.creator.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<SetMarketPricesKeys> for [AccountMeta; SET_MARKET_PRICES_IX_ACCOUNTS_LEN] {
    fn from(keys: SetMarketPricesKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.market,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.creator,
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
impl From<[Pubkey; SET_MARKET_PRICES_IX_ACCOUNTS_LEN]> for SetMarketPricesKeys {
    fn from(pubkeys: [Pubkey; SET_MARKET_PRICES_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            market: pubkeys[0],
            creator: pubkeys[1],
            event_authority: pubkeys[2],
            program: pubkeys[3],
        }
    }
}
impl<'info> From<SetMarketPricesAccounts<'_, 'info>>
for [AccountInfo<'info>; SET_MARKET_PRICES_IX_ACCOUNTS_LEN] {
    fn from(accounts: SetMarketPricesAccounts<'_, 'info>) -> Self {
        [
            accounts.market.clone(),
            accounts.creator.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SET_MARKET_PRICES_IX_ACCOUNTS_LEN]>
for SetMarketPricesAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; SET_MARKET_PRICES_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            market: &arr[0],
            creator: &arr[1],
            event_authority: &arr[2],
            program: &arr[3],
        }
    }
}
pub const SET_MARKET_PRICES_IX_DISCM: [u8; 8usize] = [
    39, 123, 107, 117, 49, 29, 21, 159,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SetMarketPricesIxArgs {
    pub bid_prices: [u64; 11],
    pub ask_prices: [u64; 11],
}
#[derive(Clone, Debug, PartialEq)]
pub struct SetMarketPricesIxData(pub SetMarketPricesIxArgs);
impl From<SetMarketPricesIxArgs> for SetMarketPricesIxData {
    fn from(args: SetMarketPricesIxArgs) -> Self {
        Self(args)
    }
}
impl SetMarketPricesIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SET_MARKET_PRICES_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let bid_prices: [u64; 11] = crate::borsh_de_or_default(&mut reader)?;
        let ask_prices: [u64; 11] = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(SetMarketPricesIxArgs {
                bid_prices,
                ask_prices,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SET_MARKET_PRICES_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.bid_prices, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.ask_prices, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn set_market_prices_ix_with_program_id(
    program_id: Pubkey,
    keys: SetMarketPricesKeys,
    args: SetMarketPricesIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SET_MARKET_PRICES_IX_ACCOUNTS_LEN] = keys.into();
    let data: SetMarketPricesIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn set_market_prices_ix(
    keys: SetMarketPricesKeys,
    args: SetMarketPricesIxArgs,
) -> std::io::Result<Instruction> {
    set_market_prices_ix_with_program_id(TOKEN_MILL_V2_PROGRAM_ID, keys, args)
}
pub fn set_market_prices_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SetMarketPricesAccounts<'_, '_>,
    args: SetMarketPricesIxArgs,
) -> ProgramResult {
    let keys: SetMarketPricesKeys = accounts.into();
    let ix = set_market_prices_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn set_market_prices_invoke(
    accounts: SetMarketPricesAccounts<'_, '_>,
    args: SetMarketPricesIxArgs,
) -> ProgramResult {
    set_market_prices_invoke_with_program_id(TOKEN_MILL_V2_PROGRAM_ID, accounts, args)
}
pub fn set_market_prices_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SetMarketPricesAccounts<'_, '_>,
    args: SetMarketPricesIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SetMarketPricesKeys = accounts.into();
    let ix = set_market_prices_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn set_market_prices_invoke_signed(
    accounts: SetMarketPricesAccounts<'_, '_>,
    args: SetMarketPricesIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    set_market_prices_invoke_signed_with_program_id(
        TOKEN_MILL_V2_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn set_market_prices_verify_account_keys(
    accounts: SetMarketPricesAccounts<'_, '_>,
    keys: SetMarketPricesKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.market.key, keys.market),
        (*accounts.creator.key, keys.creator),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn set_market_prices_verify_writable_privileges<'me, 'info>(
    accounts: SetMarketPricesAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.market] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn set_market_prices_verify_signer_privileges<'me, 'info>(
    accounts: SetMarketPricesAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.creator] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn set_market_prices_verify_account_privileges<'me, 'info>(
    accounts: SetMarketPricesAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    set_market_prices_verify_writable_privileges(accounts)?;
    set_market_prices_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SWAP_IX_ACCOUNTS_LEN: usize = 15;
#[derive(Copy, Clone, Debug)]
pub struct SwapAccounts<'me, 'info> {
    pub config: &'me AccountInfo<'info>,
    pub market: &'me AccountInfo<'info>,
    pub base_token_mint: &'me AccountInfo<'info>,
    pub quote_token_mint: &'me AccountInfo<'info>,
    pub market_base_token_ata: &'me AccountInfo<'info>,
    pub market_quote_token_ata: &'me AccountInfo<'info>,
    pub user_base_token_account: &'me AccountInfo<'info>,
    pub user_quote_token_account: &'me AccountInfo<'info>,
    pub protocol_quote_token_ata: &'me AccountInfo<'info>,
    pub referral_token_account: &'me AccountInfo<'info>,
    pub user: &'me AccountInfo<'info>,
    pub base_token_program: &'me AccountInfo<'info>,
    pub quote_token_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SwapKeys {
    pub config: Pubkey,
    pub market: Pubkey,
    pub base_token_mint: Pubkey,
    pub quote_token_mint: Pubkey,
    pub market_base_token_ata: Pubkey,
    pub market_quote_token_ata: Pubkey,
    pub user_base_token_account: Pubkey,
    pub user_quote_token_account: Pubkey,
    pub protocol_quote_token_ata: Pubkey,
    pub referral_token_account: Pubkey,
    pub user: Pubkey,
    pub base_token_program: Pubkey,
    pub quote_token_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<SwapAccounts<'_, '_>> for SwapKeys {
    fn from(accounts: SwapAccounts) -> Self {
        Self {
            config: *accounts.config.key,
            market: *accounts.market.key,
            base_token_mint: *accounts.base_token_mint.key,
            quote_token_mint: *accounts.quote_token_mint.key,
            market_base_token_ata: *accounts.market_base_token_ata.key,
            market_quote_token_ata: *accounts.market_quote_token_ata.key,
            user_base_token_account: *accounts.user_base_token_account.key,
            user_quote_token_account: *accounts.user_quote_token_account.key,
            protocol_quote_token_ata: *accounts.protocol_quote_token_ata.key,
            referral_token_account: *accounts.referral_token_account.key,
            user: *accounts.user.key,
            base_token_program: *accounts.base_token_program.key,
            quote_token_program: *accounts.quote_token_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<SwapKeys> for [AccountMeta; SWAP_IX_ACCOUNTS_LEN] {
    fn from(keys: SwapKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.market,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.base_token_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.quote_token_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.market_base_token_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.market_quote_token_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_base_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_quote_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.protocol_quote_token_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.referral_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.base_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.quote_token_program,
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
            config: pubkeys[0],
            market: pubkeys[1],
            base_token_mint: pubkeys[2],
            quote_token_mint: pubkeys[3],
            market_base_token_ata: pubkeys[4],
            market_quote_token_ata: pubkeys[5],
            user_base_token_account: pubkeys[6],
            user_quote_token_account: pubkeys[7],
            protocol_quote_token_ata: pubkeys[8],
            referral_token_account: pubkeys[9],
            user: pubkeys[10],
            base_token_program: pubkeys[11],
            quote_token_program: pubkeys[12],
            event_authority: pubkeys[13],
            program: pubkeys[14],
        }
    }
}
impl<'info> From<SwapAccounts<'_, 'info>>
for [AccountInfo<'info>; SWAP_IX_ACCOUNTS_LEN] {
    fn from(accounts: SwapAccounts<'_, 'info>) -> Self {
        [
            accounts.config.clone(),
            accounts.market.clone(),
            accounts.base_token_mint.clone(),
            accounts.quote_token_mint.clone(),
            accounts.market_base_token_ata.clone(),
            accounts.market_quote_token_ata.clone(),
            accounts.user_base_token_account.clone(),
            accounts.user_quote_token_account.clone(),
            accounts.protocol_quote_token_ata.clone(),
            accounts.referral_token_account.clone(),
            accounts.user.clone(),
            accounts.base_token_program.clone(),
            accounts.quote_token_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SWAP_IX_ACCOUNTS_LEN]>
for SwapAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; SWAP_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            config: &arr[0],
            market: &arr[1],
            base_token_mint: &arr[2],
            quote_token_mint: &arr[3],
            market_base_token_ata: &arr[4],
            market_quote_token_ata: &arr[5],
            user_base_token_account: &arr[6],
            user_quote_token_account: &arr[7],
            protocol_quote_token_ata: &arr[8],
            referral_token_account: &arr[9],
            user: &arr[10],
            base_token_program: &arr[11],
            quote_token_program: &arr[12],
            event_authority: &arr[13],
            program: &arr[14],
        }
    }
}
pub const SWAP_IX_DISCM: [u8; 8usize] = [248, 198, 158, 145, 225, 117, 135, 200];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SwapIxArgs {
    pub swap_type: SwapType,
    pub swap_amount_type: SwapAmountType,
    pub amount: u64,
    pub other_amount_threshold: u64,
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
        let swap_type: SwapType = crate::borsh_de_or_default(&mut reader)?;
        let swap_amount_type: SwapAmountType = crate::borsh_de_or_default(&mut reader)?;
        let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let other_amount_threshold: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(SwapIxArgs {
                swap_type,
                swap_amount_type,
                amount,
                other_amount_threshold,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SWAP_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.swap_type, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.swap_amount_type, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.other_amount_threshold, &mut writer)?;
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
    swap_ix_with_program_id(TOKEN_MILL_V2_PROGRAM_ID, keys, args)
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
    swap_invoke_with_program_id(TOKEN_MILL_V2_PROGRAM_ID, accounts, args)
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
    swap_invoke_signed_with_program_id(TOKEN_MILL_V2_PROGRAM_ID, accounts, args, seeds)
}
pub fn swap_verify_account_keys(
    accounts: SwapAccounts<'_, '_>,
    keys: SwapKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.config.key, keys.config),
        (*accounts.market.key, keys.market),
        (*accounts.base_token_mint.key, keys.base_token_mint),
        (*accounts.quote_token_mint.key, keys.quote_token_mint),
        (*accounts.market_base_token_ata.key, keys.market_base_token_ata),
        (*accounts.market_quote_token_ata.key, keys.market_quote_token_ata),
        (*accounts.user_base_token_account.key, keys.user_base_token_account),
        (*accounts.user_quote_token_account.key, keys.user_quote_token_account),
        (*accounts.protocol_quote_token_ata.key, keys.protocol_quote_token_ata),
        (*accounts.referral_token_account.key, keys.referral_token_account),
        (*accounts.user.key, keys.user),
        (*accounts.base_token_program.key, keys.base_token_program),
        (*accounts.quote_token_program.key, keys.quote_token_program),
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
        accounts.market,
        accounts.market_base_token_ata,
        accounts.market_quote_token_ata,
        accounts.user_base_token_account,
        accounts.user_quote_token_account,
        accounts.protocol_quote_token_ata,
        accounts.referral_token_account,
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
    pub config: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct TransferConfigOwnershipKeys {
    pub config: Pubkey,
    pub authority: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<TransferConfigOwnershipAccounts<'_, '_>> for TransferConfigOwnershipKeys {
    fn from(accounts: TransferConfigOwnershipAccounts) -> Self {
        Self {
            config: *accounts.config.key,
            authority: *accounts.authority.key,
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
            config: pubkeys[0],
            authority: pubkeys[1],
            event_authority: pubkeys[2],
            program: pubkeys[3],
        }
    }
}
impl<'info> From<TransferConfigOwnershipAccounts<'_, 'info>>
for [AccountInfo<'info>; TRANSFER_CONFIG_OWNERSHIP_IX_ACCOUNTS_LEN] {
    fn from(accounts: TransferConfigOwnershipAccounts<'_, 'info>) -> Self {
        [
            accounts.config.clone(),
            accounts.authority.clone(),
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
            config: &arr[0],
            authority: &arr[1],
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
    pub pending_authority: Option<Pubkey>,
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
        let pending_authority: Option<Pubkey> = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(TransferConfigOwnershipIxArgs {
                pending_authority,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&TRANSFER_CONFIG_OWNERSHIP_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.pending_authority, &mut writer)?;
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
    transfer_config_ownership_ix_with_program_id(TOKEN_MILL_V2_PROGRAM_ID, keys, args)
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
        TOKEN_MILL_V2_PROGRAM_ID,
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
        TOKEN_MILL_V2_PROGRAM_ID,
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
        (*accounts.config.key, keys.config),
        (*accounts.authority.key, keys.authority),
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
    for should_be_writable in [accounts.config] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn transfer_config_ownership_verify_signer_privileges<'me, 'info>(
    accounts: TransferConfigOwnershipAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority] {
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
pub const UPDATE_CREATOR_IX_ACCOUNTS_LEN: usize = 4;
#[derive(Copy, Clone, Debug)]
pub struct UpdateCreatorAccounts<'me, 'info> {
    pub market: &'me AccountInfo<'info>,
    pub creator: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UpdateCreatorKeys {
    pub market: Pubkey,
    pub creator: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<UpdateCreatorAccounts<'_, '_>> for UpdateCreatorKeys {
    fn from(accounts: UpdateCreatorAccounts) -> Self {
        Self {
            market: *accounts.market.key,
            creator: *accounts.creator.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<UpdateCreatorKeys> for [AccountMeta; UPDATE_CREATOR_IX_ACCOUNTS_LEN] {
    fn from(keys: UpdateCreatorKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.market,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.creator,
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
impl From<[Pubkey; UPDATE_CREATOR_IX_ACCOUNTS_LEN]> for UpdateCreatorKeys {
    fn from(pubkeys: [Pubkey; UPDATE_CREATOR_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            market: pubkeys[0],
            creator: pubkeys[1],
            event_authority: pubkeys[2],
            program: pubkeys[3],
        }
    }
}
impl<'info> From<UpdateCreatorAccounts<'_, 'info>>
for [AccountInfo<'info>; UPDATE_CREATOR_IX_ACCOUNTS_LEN] {
    fn from(accounts: UpdateCreatorAccounts<'_, 'info>) -> Self {
        [
            accounts.market.clone(),
            accounts.creator.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; UPDATE_CREATOR_IX_ACCOUNTS_LEN]>
for UpdateCreatorAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; UPDATE_CREATOR_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            market: &arr[0],
            creator: &arr[1],
            event_authority: &arr[2],
            program: &arr[3],
        }
    }
}
pub const UPDATE_CREATOR_IX_DISCM: [u8; 8usize] = [39, 221, 251, 213, 194, 161, 31, 207];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UpdateCreatorIxArgs {
    pub new_creator: Pubkey,
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateCreatorIxData(pub UpdateCreatorIxArgs);
impl From<UpdateCreatorIxArgs> for UpdateCreatorIxData {
    fn from(args: UpdateCreatorIxArgs) -> Self {
        Self(args)
    }
}
impl UpdateCreatorIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_CREATOR_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let new_creator: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        Ok(Self(UpdateCreatorIxArgs { new_creator }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_CREATOR_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.new_creator, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn update_creator_ix_with_program_id(
    program_id: Pubkey,
    keys: UpdateCreatorKeys,
    args: UpdateCreatorIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; UPDATE_CREATOR_IX_ACCOUNTS_LEN] = keys.into();
    let data: UpdateCreatorIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_creator_ix(
    keys: UpdateCreatorKeys,
    args: UpdateCreatorIxArgs,
) -> std::io::Result<Instruction> {
    update_creator_ix_with_program_id(TOKEN_MILL_V2_PROGRAM_ID, keys, args)
}
pub fn update_creator_invoke_with_program_id(
    program_id: Pubkey,
    accounts: UpdateCreatorAccounts<'_, '_>,
    args: UpdateCreatorIxArgs,
) -> ProgramResult {
    let keys: UpdateCreatorKeys = accounts.into();
    let ix = update_creator_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn update_creator_invoke(
    accounts: UpdateCreatorAccounts<'_, '_>,
    args: UpdateCreatorIxArgs,
) -> ProgramResult {
    update_creator_invoke_with_program_id(TOKEN_MILL_V2_PROGRAM_ID, accounts, args)
}
pub fn update_creator_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: UpdateCreatorAccounts<'_, '_>,
    args: UpdateCreatorIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: UpdateCreatorKeys = accounts.into();
    let ix = update_creator_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn update_creator_invoke_signed(
    accounts: UpdateCreatorAccounts<'_, '_>,
    args: UpdateCreatorIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    update_creator_invoke_signed_with_program_id(
        TOKEN_MILL_V2_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn update_creator_verify_account_keys(
    accounts: UpdateCreatorAccounts<'_, '_>,
    keys: UpdateCreatorKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.market.key, keys.market),
        (*accounts.creator.key, keys.creator),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn update_creator_verify_writable_privileges<'me, 'info>(
    accounts: UpdateCreatorAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.market] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn update_creator_verify_signer_privileges<'me, 'info>(
    accounts: UpdateCreatorAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.creator] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn update_creator_verify_account_privileges<'me, 'info>(
    accounts: UpdateCreatorAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    update_creator_verify_writable_privileges(accounts)?;
    update_creator_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const UPDATE_DEFAULT_FEE_SHARES_IX_ACCOUNTS_LEN: usize = 4;
#[derive(Copy, Clone, Debug)]
pub struct UpdateDefaultFeeSharesAccounts<'me, 'info> {
    pub config: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UpdateDefaultFeeSharesKeys {
    pub config: Pubkey,
    pub authority: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<UpdateDefaultFeeSharesAccounts<'_, '_>> for UpdateDefaultFeeSharesKeys {
    fn from(accounts: UpdateDefaultFeeSharesAccounts) -> Self {
        Self {
            config: *accounts.config.key,
            authority: *accounts.authority.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<UpdateDefaultFeeSharesKeys>
for [AccountMeta; UPDATE_DEFAULT_FEE_SHARES_IX_ACCOUNTS_LEN] {
    fn from(keys: UpdateDefaultFeeSharesKeys) -> Self {
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
impl From<[Pubkey; UPDATE_DEFAULT_FEE_SHARES_IX_ACCOUNTS_LEN]>
for UpdateDefaultFeeSharesKeys {
    fn from(pubkeys: [Pubkey; UPDATE_DEFAULT_FEE_SHARES_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            config: pubkeys[0],
            authority: pubkeys[1],
            event_authority: pubkeys[2],
            program: pubkeys[3],
        }
    }
}
impl<'info> From<UpdateDefaultFeeSharesAccounts<'_, 'info>>
for [AccountInfo<'info>; UPDATE_DEFAULT_FEE_SHARES_IX_ACCOUNTS_LEN] {
    fn from(accounts: UpdateDefaultFeeSharesAccounts<'_, 'info>) -> Self {
        [
            accounts.config.clone(),
            accounts.authority.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; UPDATE_DEFAULT_FEE_SHARES_IX_ACCOUNTS_LEN]>
for UpdateDefaultFeeSharesAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; UPDATE_DEFAULT_FEE_SHARES_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            config: &arr[0],
            authority: &arr[1],
            event_authority: &arr[2],
            program: &arr[3],
        }
    }
}
pub const UPDATE_DEFAULT_FEE_SHARES_IX_DISCM: [u8; 8usize] = [
    115, 93, 80, 199, 54, 219, 32, 85,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UpdateDefaultFeeSharesIxArgs {
    pub new_default_protocol_fee_share: u16,
    pub new_referral_fee_share: u16,
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateDefaultFeeSharesIxData(pub UpdateDefaultFeeSharesIxArgs);
impl From<UpdateDefaultFeeSharesIxArgs> for UpdateDefaultFeeSharesIxData {
    fn from(args: UpdateDefaultFeeSharesIxArgs) -> Self {
        Self(args)
    }
}
impl UpdateDefaultFeeSharesIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_DEFAULT_FEE_SHARES_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let new_default_protocol_fee_share: u16 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let new_referral_fee_share: u16 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(UpdateDefaultFeeSharesIxArgs {
                new_default_protocol_fee_share,
                new_referral_fee_share,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_DEFAULT_FEE_SHARES_IX_DISCM)?;
        borsh::BorshSerialize::serialize(
            &self.0.new_default_protocol_fee_share,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.0.new_referral_fee_share, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn update_default_fee_shares_ix_with_program_id(
    program_id: Pubkey,
    keys: UpdateDefaultFeeSharesKeys,
    args: UpdateDefaultFeeSharesIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; UPDATE_DEFAULT_FEE_SHARES_IX_ACCOUNTS_LEN] = keys.into();
    let data: UpdateDefaultFeeSharesIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_default_fee_shares_ix(
    keys: UpdateDefaultFeeSharesKeys,
    args: UpdateDefaultFeeSharesIxArgs,
) -> std::io::Result<Instruction> {
    update_default_fee_shares_ix_with_program_id(TOKEN_MILL_V2_PROGRAM_ID, keys, args)
}
pub fn update_default_fee_shares_invoke_with_program_id(
    program_id: Pubkey,
    accounts: UpdateDefaultFeeSharesAccounts<'_, '_>,
    args: UpdateDefaultFeeSharesIxArgs,
) -> ProgramResult {
    let keys: UpdateDefaultFeeSharesKeys = accounts.into();
    let ix = update_default_fee_shares_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn update_default_fee_shares_invoke(
    accounts: UpdateDefaultFeeSharesAccounts<'_, '_>,
    args: UpdateDefaultFeeSharesIxArgs,
) -> ProgramResult {
    update_default_fee_shares_invoke_with_program_id(
        TOKEN_MILL_V2_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn update_default_fee_shares_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: UpdateDefaultFeeSharesAccounts<'_, '_>,
    args: UpdateDefaultFeeSharesIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: UpdateDefaultFeeSharesKeys = accounts.into();
    let ix = update_default_fee_shares_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn update_default_fee_shares_invoke_signed(
    accounts: UpdateDefaultFeeSharesAccounts<'_, '_>,
    args: UpdateDefaultFeeSharesIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    update_default_fee_shares_invoke_signed_with_program_id(
        TOKEN_MILL_V2_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn update_default_fee_shares_verify_account_keys(
    accounts: UpdateDefaultFeeSharesAccounts<'_, '_>,
    keys: UpdateDefaultFeeSharesKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.config.key, keys.config),
        (*accounts.authority.key, keys.authority),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn update_default_fee_shares_verify_writable_privileges<'me, 'info>(
    accounts: UpdateDefaultFeeSharesAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.config] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn update_default_fee_shares_verify_signer_privileges<'me, 'info>(
    accounts: UpdateDefaultFeeSharesAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn update_default_fee_shares_verify_account_privileges<'me, 'info>(
    accounts: UpdateDefaultFeeSharesAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    update_default_fee_shares_verify_writable_privileges(accounts)?;
    update_default_fee_shares_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const UPDATE_MARKET_FEE_SHARES_IX_ACCOUNTS_LEN: usize = 4;
#[derive(Copy, Clone, Debug)]
pub struct UpdateMarketFeeSharesAccounts<'me, 'info> {
    pub market: &'me AccountInfo<'info>,
    pub creator: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UpdateMarketFeeSharesKeys {
    pub market: Pubkey,
    pub creator: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<UpdateMarketFeeSharesAccounts<'_, '_>> for UpdateMarketFeeSharesKeys {
    fn from(accounts: UpdateMarketFeeSharesAccounts) -> Self {
        Self {
            market: *accounts.market.key,
            creator: *accounts.creator.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<UpdateMarketFeeSharesKeys>
for [AccountMeta; UPDATE_MARKET_FEE_SHARES_IX_ACCOUNTS_LEN] {
    fn from(keys: UpdateMarketFeeSharesKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.market,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.creator,
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
impl From<[Pubkey; UPDATE_MARKET_FEE_SHARES_IX_ACCOUNTS_LEN]>
for UpdateMarketFeeSharesKeys {
    fn from(pubkeys: [Pubkey; UPDATE_MARKET_FEE_SHARES_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            market: pubkeys[0],
            creator: pubkeys[1],
            event_authority: pubkeys[2],
            program: pubkeys[3],
        }
    }
}
impl<'info> From<UpdateMarketFeeSharesAccounts<'_, 'info>>
for [AccountInfo<'info>; UPDATE_MARKET_FEE_SHARES_IX_ACCOUNTS_LEN] {
    fn from(accounts: UpdateMarketFeeSharesAccounts<'_, 'info>) -> Self {
        [
            accounts.market.clone(),
            accounts.creator.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; UPDATE_MARKET_FEE_SHARES_IX_ACCOUNTS_LEN]>
for UpdateMarketFeeSharesAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; UPDATE_MARKET_FEE_SHARES_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            market: &arr[0],
            creator: &arr[1],
            event_authority: &arr[2],
            program: &arr[3],
        }
    }
}
pub const UPDATE_MARKET_FEE_SHARES_IX_DISCM: [u8; 8usize] = [
    233, 190, 64, 95, 167, 94, 190, 251,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UpdateMarketFeeSharesIxArgs {
    pub new_creator_fee_share: u16,
    pub new_staking_fee_share: u16,
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateMarketFeeSharesIxData(pub UpdateMarketFeeSharesIxArgs);
impl From<UpdateMarketFeeSharesIxArgs> for UpdateMarketFeeSharesIxData {
    fn from(args: UpdateMarketFeeSharesIxArgs) -> Self {
        Self(args)
    }
}
impl UpdateMarketFeeSharesIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_MARKET_FEE_SHARES_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let new_creator_fee_share: u16 = crate::borsh_de_or_default(&mut reader)?;
        let new_staking_fee_share: u16 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(UpdateMarketFeeSharesIxArgs {
                new_creator_fee_share,
                new_staking_fee_share,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_MARKET_FEE_SHARES_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.new_creator_fee_share, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.new_staking_fee_share, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn update_market_fee_shares_ix_with_program_id(
    program_id: Pubkey,
    keys: UpdateMarketFeeSharesKeys,
    args: UpdateMarketFeeSharesIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; UPDATE_MARKET_FEE_SHARES_IX_ACCOUNTS_LEN] = keys.into();
    let data: UpdateMarketFeeSharesIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_market_fee_shares_ix(
    keys: UpdateMarketFeeSharesKeys,
    args: UpdateMarketFeeSharesIxArgs,
) -> std::io::Result<Instruction> {
    update_market_fee_shares_ix_with_program_id(TOKEN_MILL_V2_PROGRAM_ID, keys, args)
}
pub fn update_market_fee_shares_invoke_with_program_id(
    program_id: Pubkey,
    accounts: UpdateMarketFeeSharesAccounts<'_, '_>,
    args: UpdateMarketFeeSharesIxArgs,
) -> ProgramResult {
    let keys: UpdateMarketFeeSharesKeys = accounts.into();
    let ix = update_market_fee_shares_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn update_market_fee_shares_invoke(
    accounts: UpdateMarketFeeSharesAccounts<'_, '_>,
    args: UpdateMarketFeeSharesIxArgs,
) -> ProgramResult {
    update_market_fee_shares_invoke_with_program_id(
        TOKEN_MILL_V2_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn update_market_fee_shares_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: UpdateMarketFeeSharesAccounts<'_, '_>,
    args: UpdateMarketFeeSharesIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: UpdateMarketFeeSharesKeys = accounts.into();
    let ix = update_market_fee_shares_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn update_market_fee_shares_invoke_signed(
    accounts: UpdateMarketFeeSharesAccounts<'_, '_>,
    args: UpdateMarketFeeSharesIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    update_market_fee_shares_invoke_signed_with_program_id(
        TOKEN_MILL_V2_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn update_market_fee_shares_verify_account_keys(
    accounts: UpdateMarketFeeSharesAccounts<'_, '_>,
    keys: UpdateMarketFeeSharesKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.market.key, keys.market),
        (*accounts.creator.key, keys.creator),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn update_market_fee_shares_verify_writable_privileges<'me, 'info>(
    accounts: UpdateMarketFeeSharesAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.market] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn update_market_fee_shares_verify_signer_privileges<'me, 'info>(
    accounts: UpdateMarketFeeSharesAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.creator] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn update_market_fee_shares_verify_account_privileges<'me, 'info>(
    accounts: UpdateMarketFeeSharesAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    update_market_fee_shares_verify_writable_privileges(accounts)?;
    update_market_fee_shares_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const UPDATE_PROTOCOL_FEE_RECIPIENT_IX_ACCOUNTS_LEN: usize = 4;
#[derive(Copy, Clone, Debug)]
pub struct UpdateProtocolFeeRecipientAccounts<'me, 'info> {
    pub config: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UpdateProtocolFeeRecipientKeys {
    pub config: Pubkey,
    pub authority: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<UpdateProtocolFeeRecipientAccounts<'_, '_>>
for UpdateProtocolFeeRecipientKeys {
    fn from(accounts: UpdateProtocolFeeRecipientAccounts) -> Self {
        Self {
            config: *accounts.config.key,
            authority: *accounts.authority.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<UpdateProtocolFeeRecipientKeys>
for [AccountMeta; UPDATE_PROTOCOL_FEE_RECIPIENT_IX_ACCOUNTS_LEN] {
    fn from(keys: UpdateProtocolFeeRecipientKeys) -> Self {
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
impl From<[Pubkey; UPDATE_PROTOCOL_FEE_RECIPIENT_IX_ACCOUNTS_LEN]>
for UpdateProtocolFeeRecipientKeys {
    fn from(pubkeys: [Pubkey; UPDATE_PROTOCOL_FEE_RECIPIENT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            config: pubkeys[0],
            authority: pubkeys[1],
            event_authority: pubkeys[2],
            program: pubkeys[3],
        }
    }
}
impl<'info> From<UpdateProtocolFeeRecipientAccounts<'_, 'info>>
for [AccountInfo<'info>; UPDATE_PROTOCOL_FEE_RECIPIENT_IX_ACCOUNTS_LEN] {
    fn from(accounts: UpdateProtocolFeeRecipientAccounts<'_, 'info>) -> Self {
        [
            accounts.config.clone(),
            accounts.authority.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; UPDATE_PROTOCOL_FEE_RECIPIENT_IX_ACCOUNTS_LEN]>
for UpdateProtocolFeeRecipientAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; UPDATE_PROTOCOL_FEE_RECIPIENT_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            config: &arr[0],
            authority: &arr[1],
            event_authority: &arr[2],
            program: &arr[3],
        }
    }
}
pub const UPDATE_PROTOCOL_FEE_RECIPIENT_IX_DISCM: [u8; 8usize] = [
    213, 60, 21, 106, 42, 67, 60, 162,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UpdateProtocolFeeRecipientIxArgs {
    pub new_protocol_fee_recipient: Pubkey,
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateProtocolFeeRecipientIxData(pub UpdateProtocolFeeRecipientIxArgs);
impl From<UpdateProtocolFeeRecipientIxArgs> for UpdateProtocolFeeRecipientIxData {
    fn from(args: UpdateProtocolFeeRecipientIxArgs) -> Self {
        Self(args)
    }
}
impl UpdateProtocolFeeRecipientIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_PROTOCOL_FEE_RECIPIENT_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let new_protocol_fee_recipient: Pubkey = crate::borsh_de_or_default(
            &mut reader,
        )?;
        Ok(
            Self(UpdateProtocolFeeRecipientIxArgs {
                new_protocol_fee_recipient,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_PROTOCOL_FEE_RECIPIENT_IX_DISCM)?;
        borsh::BorshSerialize::serialize(
            &self.0.new_protocol_fee_recipient,
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
pub fn update_protocol_fee_recipient_ix_with_program_id(
    program_id: Pubkey,
    keys: UpdateProtocolFeeRecipientKeys,
    args: UpdateProtocolFeeRecipientIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; UPDATE_PROTOCOL_FEE_RECIPIENT_IX_ACCOUNTS_LEN] = keys
        .into();
    let data: UpdateProtocolFeeRecipientIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_protocol_fee_recipient_ix(
    keys: UpdateProtocolFeeRecipientKeys,
    args: UpdateProtocolFeeRecipientIxArgs,
) -> std::io::Result<Instruction> {
    update_protocol_fee_recipient_ix_with_program_id(
        TOKEN_MILL_V2_PROGRAM_ID,
        keys,
        args,
    )
}
pub fn update_protocol_fee_recipient_invoke_with_program_id(
    program_id: Pubkey,
    accounts: UpdateProtocolFeeRecipientAccounts<'_, '_>,
    args: UpdateProtocolFeeRecipientIxArgs,
) -> ProgramResult {
    let keys: UpdateProtocolFeeRecipientKeys = accounts.into();
    let ix = update_protocol_fee_recipient_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn update_protocol_fee_recipient_invoke(
    accounts: UpdateProtocolFeeRecipientAccounts<'_, '_>,
    args: UpdateProtocolFeeRecipientIxArgs,
) -> ProgramResult {
    update_protocol_fee_recipient_invoke_with_program_id(
        TOKEN_MILL_V2_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn update_protocol_fee_recipient_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: UpdateProtocolFeeRecipientAccounts<'_, '_>,
    args: UpdateProtocolFeeRecipientIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: UpdateProtocolFeeRecipientKeys = accounts.into();
    let ix = update_protocol_fee_recipient_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn update_protocol_fee_recipient_invoke_signed(
    accounts: UpdateProtocolFeeRecipientAccounts<'_, '_>,
    args: UpdateProtocolFeeRecipientIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    update_protocol_fee_recipient_invoke_signed_with_program_id(
        TOKEN_MILL_V2_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn update_protocol_fee_recipient_verify_account_keys(
    accounts: UpdateProtocolFeeRecipientAccounts<'_, '_>,
    keys: UpdateProtocolFeeRecipientKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.config.key, keys.config),
        (*accounts.authority.key, keys.authority),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn update_protocol_fee_recipient_verify_writable_privileges<'me, 'info>(
    accounts: UpdateProtocolFeeRecipientAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.config] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn update_protocol_fee_recipient_verify_signer_privileges<'me, 'info>(
    accounts: UpdateProtocolFeeRecipientAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn update_protocol_fee_recipient_verify_account_privileges<'me, 'info>(
    accounts: UpdateProtocolFeeRecipientAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    update_protocol_fee_recipient_verify_writable_privileges(accounts)?;
    update_protocol_fee_recipient_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const UPDATE_QUOTE_ASSET_BADGE_IX_ACCOUNTS_LEN: usize = 6;
#[derive(Copy, Clone, Debug)]
pub struct UpdateQuoteAssetBadgeAccounts<'me, 'info> {
    pub config: &'me AccountInfo<'info>,
    pub quote_asset_badge: &'me AccountInfo<'info>,
    pub token_mint: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UpdateQuoteAssetBadgeKeys {
    pub config: Pubkey,
    pub quote_asset_badge: Pubkey,
    pub token_mint: Pubkey,
    pub authority: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<UpdateQuoteAssetBadgeAccounts<'_, '_>> for UpdateQuoteAssetBadgeKeys {
    fn from(accounts: UpdateQuoteAssetBadgeAccounts) -> Self {
        Self {
            config: *accounts.config.key,
            quote_asset_badge: *accounts.quote_asset_badge.key,
            token_mint: *accounts.token_mint.key,
            authority: *accounts.authority.key,
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
                pubkey: keys.config,
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
                pubkey: keys.authority,
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
            config: pubkeys[0],
            quote_asset_badge: pubkeys[1],
            token_mint: pubkeys[2],
            authority: pubkeys[3],
            event_authority: pubkeys[4],
            program: pubkeys[5],
        }
    }
}
impl<'info> From<UpdateQuoteAssetBadgeAccounts<'_, 'info>>
for [AccountInfo<'info>; UPDATE_QUOTE_ASSET_BADGE_IX_ACCOUNTS_LEN] {
    fn from(accounts: UpdateQuoteAssetBadgeAccounts<'_, 'info>) -> Self {
        [
            accounts.config.clone(),
            accounts.quote_asset_badge.clone(),
            accounts.token_mint.clone(),
            accounts.authority.clone(),
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
            config: &arr[0],
            quote_asset_badge: &arr[1],
            token_mint: &arr[2],
            authority: &arr[3],
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
    pub status: QuoteTokenBadgeStatus,
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
        let status: QuoteTokenBadgeStatus = crate::borsh_de_or_default(&mut reader)?;
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
    update_quote_asset_badge_ix_with_program_id(TOKEN_MILL_V2_PROGRAM_ID, keys, args)
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
        TOKEN_MILL_V2_PROGRAM_ID,
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
        TOKEN_MILL_V2_PROGRAM_ID,
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
        (*accounts.config.key, keys.config),
        (*accounts.quote_asset_badge.key, keys.quote_asset_badge),
        (*accounts.token_mint.key, keys.token_mint),
        (*accounts.authority.key, keys.authority),
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
    for should_be_signer in [accounts.authority] {
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
pub const WITHDRAW_IX_ACCOUNTS_LEN: usize = 10;
#[derive(Copy, Clone, Debug)]
pub struct WithdrawAccounts<'me, 'info> {
    pub market: &'me AccountInfo<'info>,
    pub staking: &'me AccountInfo<'info>,
    pub stake_position: &'me AccountInfo<'info>,
    pub base_token_mint: &'me AccountInfo<'info>,
    pub market_base_token_ata: &'me AccountInfo<'info>,
    pub user_base_token_ata: &'me AccountInfo<'info>,
    pub user: &'me AccountInfo<'info>,
    pub base_token_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct WithdrawKeys {
    pub market: Pubkey,
    pub staking: Pubkey,
    pub stake_position: Pubkey,
    pub base_token_mint: Pubkey,
    pub market_base_token_ata: Pubkey,
    pub user_base_token_ata: Pubkey,
    pub user: Pubkey,
    pub base_token_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<WithdrawAccounts<'_, '_>> for WithdrawKeys {
    fn from(accounts: WithdrawAccounts) -> Self {
        Self {
            market: *accounts.market.key,
            staking: *accounts.staking.key,
            stake_position: *accounts.stake_position.key,
            base_token_mint: *accounts.base_token_mint.key,
            market_base_token_ata: *accounts.market_base_token_ata.key,
            user_base_token_ata: *accounts.user_base_token_ata.key,
            user: *accounts.user.key,
            base_token_program: *accounts.base_token_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<WithdrawKeys> for [AccountMeta; WITHDRAW_IX_ACCOUNTS_LEN] {
    fn from(keys: WithdrawKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.market,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.staking,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.stake_position,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.base_token_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.market_base_token_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_base_token_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.base_token_program,
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
impl From<[Pubkey; WITHDRAW_IX_ACCOUNTS_LEN]> for WithdrawKeys {
    fn from(pubkeys: [Pubkey; WITHDRAW_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            market: pubkeys[0],
            staking: pubkeys[1],
            stake_position: pubkeys[2],
            base_token_mint: pubkeys[3],
            market_base_token_ata: pubkeys[4],
            user_base_token_ata: pubkeys[5],
            user: pubkeys[6],
            base_token_program: pubkeys[7],
            event_authority: pubkeys[8],
            program: pubkeys[9],
        }
    }
}
impl<'info> From<WithdrawAccounts<'_, 'info>>
for [AccountInfo<'info>; WITHDRAW_IX_ACCOUNTS_LEN] {
    fn from(accounts: WithdrawAccounts<'_, 'info>) -> Self {
        [
            accounts.market.clone(),
            accounts.staking.clone(),
            accounts.stake_position.clone(),
            accounts.base_token_mint.clone(),
            accounts.market_base_token_ata.clone(),
            accounts.user_base_token_ata.clone(),
            accounts.user.clone(),
            accounts.base_token_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; WITHDRAW_IX_ACCOUNTS_LEN]>
for WithdrawAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; WITHDRAW_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            market: &arr[0],
            staking: &arr[1],
            stake_position: &arr[2],
            base_token_mint: &arr[3],
            market_base_token_ata: &arr[4],
            user_base_token_ata: &arr[5],
            user: &arr[6],
            base_token_program: &arr[7],
            event_authority: &arr[8],
            program: &arr[9],
        }
    }
}
pub const WITHDRAW_IX_DISCM: [u8; 8usize] = [183, 18, 70, 156, 148, 109, 161, 34];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WithdrawIxArgs {
    pub amount: u64,
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
        let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(Self(WithdrawIxArgs { amount }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&WITHDRAW_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.amount, &mut writer)?;
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
    withdraw_ix_with_program_id(TOKEN_MILL_V2_PROGRAM_ID, keys, args)
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
    withdraw_invoke_with_program_id(TOKEN_MILL_V2_PROGRAM_ID, accounts, args)
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
        TOKEN_MILL_V2_PROGRAM_ID,
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
        (*accounts.market.key, keys.market),
        (*accounts.staking.key, keys.staking),
        (*accounts.stake_position.key, keys.stake_position),
        (*accounts.base_token_mint.key, keys.base_token_mint),
        (*accounts.market_base_token_ata.key, keys.market_base_token_ata),
        (*accounts.user_base_token_ata.key, keys.user_base_token_ata),
        (*accounts.user.key, keys.user),
        (*accounts.base_token_program.key, keys.base_token_program),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
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
        accounts.market,
        accounts.staking,
        accounts.stake_position,
        accounts.market_base_token_ata,
        accounts.user_base_token_ata,
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
    for should_be_signer in [accounts.user] {
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
