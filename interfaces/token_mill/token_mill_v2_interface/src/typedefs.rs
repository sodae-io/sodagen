use borsh::{BorshDeserialize, BorshSerialize};
#[allow(unused_imports)]
use crate::*;
use solana_pubkey::Pubkey;
#[derive(
    Clone,
    Debug,
    Default,
    BorshDeserialize,
    BorshSerialize,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
pub struct MarketFees {
    pub staking_fee_share: u16,
    pub creator_fee_share: u16,
    pub _space: u32,
    pub pending_staking_fees: u64,
    pub pending_creator_fees: u64,
}
impl MarketFees {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let staking_fee_share: u16 = crate::borsh_de_or_default(&mut reader)?;
        let creator_fee_share: u16 = crate::borsh_de_or_default(&mut reader)?;
        let _space: u32 = crate::borsh_de_or_default(&mut reader)?;
        let pending_staking_fees: u64 = crate::borsh_de_or_default(&mut reader)?;
        let pending_creator_fees: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            staking_fee_share,
            creator_fee_share,
            _space,
            pending_staking_fees,
            pending_creator_fees,
        })
    }
}
#[derive(
    Clone,
    Debug,
    Default,
    BorshDeserialize,
    BorshSerialize,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
pub enum QuoteTokenBadgeStatus {
    #[default]
    Disabled,
    Enabled,
}
#[derive(
    Clone,
    Debug,
    Default,
    BorshDeserialize,
    BorshSerialize,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
pub enum SwapAmountType {
    #[default]
    ExactInput,
    ExactOutput,
}
#[derive(
    Clone,
    Debug,
    Default,
    BorshDeserialize,
    BorshSerialize,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
pub enum SwapType {
    #[default]
    Buy,
    Sell,
}
#[derive(
    Clone,
    Debug,
    Default,
    BorshDeserialize,
    BorshSerialize,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
pub struct TokenMillConfigCreationEvent {
    pub config: Pubkey,
    pub authority: Pubkey,
    pub default_protocol_fee_share: u16,
    pub referral_fee_share: u16,
}
impl TokenMillConfigCreationEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let config: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let authority: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let default_protocol_fee_share: u16 = crate::borsh_de_or_default(&mut reader)?;
        let referral_fee_share: u16 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            config,
            authority,
            default_protocol_fee_share,
            referral_fee_share,
        })
    }
}
#[derive(
    Clone,
    Debug,
    Default,
    BorshDeserialize,
    BorshSerialize,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
pub struct TokenMillConfigOwnershipTransferEvent {
    pub config: Pubkey,
    pub new_authority: Pubkey,
}
impl TokenMillConfigOwnershipTransferEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let config: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let new_authority: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { config, new_authority })
    }
}
#[derive(
    Clone,
    Debug,
    Default,
    BorshDeserialize,
    BorshSerialize,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
pub struct TokenMillCreatorFeeClaimEvent {
    pub market: Pubkey,
    pub creator: Pubkey,
    pub fees_distributed: u64,
}
impl TokenMillCreatorFeeClaimEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let market: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let creator: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let fees_distributed: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            market,
            creator,
            fees_distributed,
        })
    }
}
#[derive(
    Clone,
    Debug,
    Default,
    BorshDeserialize,
    BorshSerialize,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
pub struct TokenMillCreatorUpdateEvent {
    pub market: Pubkey,
    pub new_creator: Pubkey,
}
impl TokenMillCreatorUpdateEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let market: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let new_creator: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { market, new_creator })
    }
}
#[derive(
    Clone,
    Debug,
    Default,
    BorshDeserialize,
    BorshSerialize,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
pub struct TokenMillDefaultFeeSharesUpdateEvent {
    pub config: Pubkey,
    pub new_default_protocol_fee_share: u16,
    pub new_referral_fee_share: u16,
}
impl TokenMillDefaultFeeSharesUpdateEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let config: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let new_default_protocol_fee_share: u16 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let new_referral_fee_share: u16 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            config,
            new_default_protocol_fee_share,
            new_referral_fee_share,
        })
    }
}
#[derive(
    Clone,
    Debug,
    Default,
    BorshDeserialize,
    BorshSerialize,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
pub struct TokenMillMarketCreationEvent {
    pub config: Pubkey,
    pub market: Pubkey,
    pub creator: Pubkey,
    pub base_token_mint: Pubkey,
    pub quote_token_mint: Pubkey,
    pub total_supply: u64,
    pub protocol_fee_share: u16,
    pub referral_fee_share: u16,
    pub creator_fee_share: u16,
    pub staking_fee_share: u16,
}
impl TokenMillMarketCreationEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let config: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let market: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let creator: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let base_token_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let quote_token_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let total_supply: u64 = crate::borsh_de_or_default(&mut reader)?;
        let protocol_fee_share: u16 = crate::borsh_de_or_default(&mut reader)?;
        let referral_fee_share: u16 = crate::borsh_de_or_default(&mut reader)?;
        let creator_fee_share: u16 = crate::borsh_de_or_default(&mut reader)?;
        let staking_fee_share: u16 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            config,
            market,
            creator,
            base_token_mint,
            quote_token_mint,
            total_supply,
            protocol_fee_share,
            referral_fee_share,
            creator_fee_share,
            staking_fee_share,
        })
    }
}
#[derive(
    Clone,
    Debug,
    Default,
    BorshDeserialize,
    BorshSerialize,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
pub struct TokenMillMarketFeeSharesUpdateEvent {
    pub market: Pubkey,
    pub new_creator_fee_share: u16,
    pub new_staking_fee_share: u16,
}
impl TokenMillMarketFeeSharesUpdateEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let market: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let new_creator_fee_share: u16 = crate::borsh_de_or_default(&mut reader)?;
        let new_staking_fee_share: u16 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            market,
            new_creator_fee_share,
            new_staking_fee_share,
        })
    }
}
#[derive(
    Clone,
    Debug,
    Default,
    BorshDeserialize,
    BorshSerialize,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
pub struct TokenMillMarketPriceSetEvent {
    pub market: Pubkey,
    pub bid_prices: [u64; 11],
    pub ask_prices: [u64; 11],
}
impl TokenMillMarketPriceSetEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let market: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let bid_prices: [u64; 11] = crate::borsh_de_or_default(&mut reader)?;
        let ask_prices: [u64; 11] = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            market,
            bid_prices,
            ask_prices,
        })
    }
}
#[derive(
    Clone,
    Debug,
    Default,
    BorshDeserialize,
    BorshSerialize,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
pub struct TokenMillProtocolFeeRecipientUpdateEvent {
    pub config: Pubkey,
    pub new_protocol_fee_recipient: Pubkey,
}
impl TokenMillProtocolFeeRecipientUpdateEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let config: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let new_protocol_fee_recipient: Pubkey = crate::borsh_de_or_default(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self {
            config,
            new_protocol_fee_recipient,
        })
    }
}
#[derive(
    Clone,
    Debug,
    Default,
    BorshDeserialize,
    BorshSerialize,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
pub struct TokenMillQuoteTokenBadgeEvent {
    pub config: Pubkey,
    pub quote_token_mint: Pubkey,
    pub quote_asset_badge_status: QuoteTokenBadgeStatus,
}
impl TokenMillQuoteTokenBadgeEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let config: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let quote_token_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let quote_asset_badge_status: QuoteTokenBadgeStatus = crate::borsh_de_or_default(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self {
            config,
            quote_token_mint,
            quote_asset_badge_status,
        })
    }
}
#[derive(
    Clone,
    Debug,
    Default,
    BorshDeserialize,
    BorshSerialize,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
pub struct TokenMillReferralFeeClaimEvent {
    pub referrer: Pubkey,
    pub quote_token_mint: Pubkey,
    pub fees_distributed: u64,
}
impl TokenMillReferralFeeClaimEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let referrer: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let quote_token_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let fees_distributed: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            referrer,
            quote_token_mint,
            fees_distributed,
        })
    }
}
#[derive(
    Clone,
    Debug,
    Default,
    BorshDeserialize,
    BorshSerialize,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
pub struct TokenMillStakingDepositEvent {
    pub market: Pubkey,
    pub user: Pubkey,
    pub amount: u64,
}
impl TokenMillStakingDepositEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let market: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let user: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { market, user, amount })
    }
}
#[derive(
    Clone,
    Debug,
    Default,
    BorshDeserialize,
    BorshSerialize,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
pub struct TokenMillStakingRewardsClaimEvent {
    pub market: Pubkey,
    pub user: Pubkey,
    pub amount_distributed: u64,
}
impl TokenMillStakingRewardsClaimEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let market: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let user: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let amount_distributed: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            market,
            user,
            amount_distributed,
        })
    }
}
#[derive(
    Clone,
    Debug,
    Default,
    BorshDeserialize,
    BorshSerialize,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
pub struct TokenMillStakingWithdrawalEvent {
    pub market: Pubkey,
    pub user: Pubkey,
    pub amount: u64,
}
impl TokenMillStakingWithdrawalEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let market: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let user: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { market, user, amount })
    }
}
#[derive(
    Clone,
    Debug,
    Default,
    BorshDeserialize,
    BorshSerialize,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
pub struct TokenMillSwapEvent {
    pub user: Pubkey,
    pub market: Pubkey,
    pub swap_type: SwapType,
    pub base_amount: u64,
    pub quote_amount: u64,
    pub referral_token_account: Option<Pubkey>,
    pub creator_fee: u64,
    pub staking_fee: u64,
    pub protocol_fee: u64,
    pub referral_fee: u64,
}
impl TokenMillSwapEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let user: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let market: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let swap_type: SwapType = crate::borsh_de_or_default(&mut reader)?;
        let base_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let quote_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let referral_token_account: Option<Pubkey> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let creator_fee: u64 = crate::borsh_de_or_default(&mut reader)?;
        let staking_fee: u64 = crate::borsh_de_or_default(&mut reader)?;
        let protocol_fee: u64 = crate::borsh_de_or_default(&mut reader)?;
        let referral_fee: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            user,
            market,
            swap_type,
            base_amount,
            quote_amount,
            referral_token_account,
            creator_fee,
            staking_fee,
            protocol_fee,
            referral_fee,
        })
    }
}
#[derive(
    Clone,
    Debug,
    Default,
    BorshDeserialize,
    BorshSerialize,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
pub struct TokenMillVestingPlanCreationEvent {
    pub market: Pubkey,
    pub user: Pubkey,
    pub vesting_plan: Pubkey,
    pub vesting_amount: u64,
    pub start: i64,
    pub vesting_duration: i64,
    pub cliff_duration: i64,
}
impl TokenMillVestingPlanCreationEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let market: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let user: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let vesting_plan: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let vesting_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let start: i64 = crate::borsh_de_or_default(&mut reader)?;
        let vesting_duration: i64 = crate::borsh_de_or_default(&mut reader)?;
        let cliff_duration: i64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            market,
            user,
            vesting_plan,
            vesting_amount,
            start,
            vesting_duration,
            cliff_duration,
        })
    }
}
#[derive(
    Clone,
    Debug,
    Default,
    BorshDeserialize,
    BorshSerialize,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
pub struct TokenMillVestingPlanReleaseEvent {
    pub vesting_plan: Pubkey,
    pub amount_released: u64,
}
impl TokenMillVestingPlanReleaseEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let vesting_plan: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let amount_released: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            vesting_plan,
            amount_released,
        })
    }
}
