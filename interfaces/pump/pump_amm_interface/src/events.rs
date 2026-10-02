use solana_pubkey::Pubkey;
#[allow(unused_imports)]
use crate::*;
pub const ADMIN_SET_COIN_CREATOR_EVENT_EVENT_DISCM: [u8; 8] = [
    45, 220, 93, 24, 25, 97, 172, 104,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AdminSetCoinCreatorEvent {
    pub timestamp: i64,
    pub admin_set_coin_creator_authority: Pubkey,
    pub base_mint: Pubkey,
    pub pool: Pubkey,
    pub old_coin_creator: Pubkey,
    pub new_coin_creator: Pubkey,
}
impl AdminSetCoinCreatorEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let timestamp: i64 = crate::borsh_de_or_default(&mut reader)?;
        let admin_set_coin_creator_authority: Pubkey = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let base_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let pool: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let old_coin_creator: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let new_coin_creator: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            timestamp,
            admin_set_coin_creator_authority,
            base_mint,
            pool,
            old_coin_creator,
            new_coin_creator,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.timestamp, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.admin_set_coin_creator_authority,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.base_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.pool, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.old_coin_creator, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.new_coin_creator, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct AdminSetCoinCreatorEventEvent(pub AdminSetCoinCreatorEvent);
impl AdminSetCoinCreatorEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != ADMIN_SET_COIN_CREATOR_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = AdminSetCoinCreatorEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&ADMIN_SET_COIN_CREATOR_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const ADMIN_SET_COIN_CREATOR_FEE_EDITABLE_EVENT_EVENT_DISCM: [u8; 8] = [
    191, 198, 127, 188, 109, 174, 9, 105,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AdminSetCoinCreatorFeeEditableEvent {
    pub timestamp: i64,
    pub admin_set_coin_creator_authority: Pubkey,
    pub base_mint: Pubkey,
    pub pool: Pubkey,
}
impl AdminSetCoinCreatorFeeEditableEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let timestamp: i64 = crate::borsh_de_or_default(&mut reader)?;
        let admin_set_coin_creator_authority: Pubkey = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let base_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let pool: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            timestamp,
            admin_set_coin_creator_authority,
            base_mint,
            pool,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.timestamp, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.admin_set_coin_creator_authority,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.base_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.pool, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct AdminSetCoinCreatorFeeEditableEventEvent(
    pub AdminSetCoinCreatorFeeEditableEvent,
);
impl AdminSetCoinCreatorFeeEditableEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != ADMIN_SET_COIN_CREATOR_FEE_EDITABLE_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = AdminSetCoinCreatorFeeEditableEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&ADMIN_SET_COIN_CREATOR_FEE_EDITABLE_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const ADMIN_UPDATE_TOKEN_INCENTIVES_EVENT_EVENT_DISCM: [u8; 8] = [
    147, 250, 108, 120, 247, 29, 67, 222,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AdminUpdateTokenIncentivesEvent {
    pub start_time: i64,
    pub end_time: i64,
    pub day_number: u64,
    pub token_supply_per_day: u64,
    pub mint: Pubkey,
    pub seconds_in_a_day: i64,
    pub timestamp: i64,
}
impl AdminUpdateTokenIncentivesEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let start_time: i64 = crate::borsh_de_or_default(&mut reader)?;
        let end_time: i64 = crate::borsh_de_or_default(&mut reader)?;
        let day_number: u64 = crate::borsh_de_or_default(&mut reader)?;
        let token_supply_per_day: u64 = crate::borsh_de_or_default(&mut reader)?;
        let mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let seconds_in_a_day: i64 = crate::borsh_de_or_default(&mut reader)?;
        let timestamp: i64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            start_time,
            end_time,
            day_number,
            token_supply_per_day,
            mint,
            seconds_in_a_day,
            timestamp,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.start_time, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.end_time, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.day_number, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_supply_per_day, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.seconds_in_a_day, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.timestamp, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct AdminUpdateTokenIncentivesEventEvent(pub AdminUpdateTokenIncentivesEvent);
impl AdminUpdateTokenIncentivesEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != ADMIN_UPDATE_TOKEN_INCENTIVES_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = AdminUpdateTokenIncentivesEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&ADMIN_UPDATE_TOKEN_INCENTIVES_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const BOOST_BUY_AND_BURN_EVENT_EVENT_DISCM: [u8; 8] = [
    63, 69, 28, 22, 48, 92, 194, 185,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct BoostBuyAndBurnEvent {
    pub timestamp: i64,
    pub mint: Pubkey,
    pub bonding_curve: Pubkey,
    pub pool: Pubkey,
    pub authority: Pubkey,
    pub quote_amount_in_requested: u64,
    pub quote_amount_in_used: u64,
    pub base_amount_burned: u64,
    pub virtual_quote_reserves: i128,
    pub real_quote_reserves_after: u64,
    pub base_reserves_after: u64,
    pub boost_vault_remaining: u64,
}
impl BoostBuyAndBurnEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let timestamp: i64 = crate::borsh_de_or_default(&mut reader)?;
        let mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let bonding_curve: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let pool: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let authority: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let quote_amount_in_requested: u64 = crate::borsh_de_or_default(&mut reader)?;
        let quote_amount_in_used: u64 = crate::borsh_de_or_default(&mut reader)?;
        let base_amount_burned: u64 = crate::borsh_de_or_default(&mut reader)?;
        let virtual_quote_reserves: i128 = crate::borsh_de_or_default(&mut reader)?;
        let real_quote_reserves_after: u64 = crate::borsh_de_or_default(&mut reader)?;
        let base_reserves_after: u64 = crate::borsh_de_or_default(&mut reader)?;
        let boost_vault_remaining: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            timestamp,
            mint,
            bonding_curve,
            pool,
            authority,
            quote_amount_in_requested,
            quote_amount_in_used,
            base_amount_burned,
            virtual_quote_reserves,
            real_quote_reserves_after,
            base_reserves_after,
            boost_vault_remaining,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.timestamp, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.bonding_curve, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.pool, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.authority, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.quote_amount_in_requested, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.quote_amount_in_used, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.base_amount_burned, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.virtual_quote_reserves, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.real_quote_reserves_after, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.base_reserves_after, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.boost_vault_remaining, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct BoostBuyAndBurnEventEvent(pub BoostBuyAndBurnEvent);
impl BoostBuyAndBurnEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != BOOST_BUY_AND_BURN_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = BoostBuyAndBurnEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&BOOST_BUY_AND_BURN_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const BUY_EVENT_EVENT_DISCM: [u8; 8] = [103, 244, 82, 31, 44, 245, 119, 119];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct BuyEvent {
    pub timestamp: i64,
    pub base_amount_out: u64,
    pub max_quote_amount_in: u64,
    pub user_base_token_reserves: u64,
    pub user_quote_token_reserves: u64,
    pub pool_base_token_reserves: u64,
    pub pool_quote_token_reserves: u64,
    pub quote_amount_in: u64,
    pub lp_fee_basis_points: u64,
    pub lp_fee: u64,
    pub protocol_fee_basis_points: u64,
    pub protocol_fee: u64,
    pub quote_amount_in_with_lp_fee: u64,
    pub user_quote_amount_in: u64,
    pub pool: Pubkey,
    pub user: Pubkey,
    pub user_base_token_account: Pubkey,
    pub user_quote_token_account: Pubkey,
    pub protocol_fee_recipient: Pubkey,
    pub protocol_fee_recipient_token_account: Pubkey,
    pub coin_creator: Pubkey,
    pub coin_creator_fee_basis_points: u64,
    pub coin_creator_fee: u64,
    pub track_volume: bool,
    pub total_unclaimed_tokens: u64,
    pub total_claimed_tokens: u64,
    pub current_sol_volume: u64,
    pub last_update_timestamp: i64,
    pub min_base_amount_out: u64,
    pub ix_name: String,
    pub cashback_fee_basis_points: u64,
    pub cashback: u64,
    pub buyback_fee_basis_points: u64,
    pub buyback_fee: u64,
    pub virtual_quote_reserves: i128,
    pub can_boost: bool,
    pub base_supply: u64,
}
impl BuyEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let timestamp: i64 = crate::borsh_de_or_default(&mut reader)?;
        let base_amount_out: u64 = crate::borsh_de_or_default(&mut reader)?;
        let max_quote_amount_in: u64 = crate::borsh_de_or_default(&mut reader)?;
        let user_base_token_reserves: u64 = crate::borsh_de_or_default(&mut reader)?;
        let user_quote_token_reserves: u64 = crate::borsh_de_or_default(&mut reader)?;
        let pool_base_token_reserves: u64 = crate::borsh_de_or_default(&mut reader)?;
        let pool_quote_token_reserves: u64 = crate::borsh_de_or_default(&mut reader)?;
        let quote_amount_in: u64 = crate::borsh_de_or_default(&mut reader)?;
        let lp_fee_basis_points: u64 = crate::borsh_de_or_default(&mut reader)?;
        let lp_fee: u64 = crate::borsh_de_or_default(&mut reader)?;
        let protocol_fee_basis_points: u64 = crate::borsh_de_or_default(&mut reader)?;
        let protocol_fee: u64 = crate::borsh_de_or_default(&mut reader)?;
        let quote_amount_in_with_lp_fee: u64 = crate::borsh_de_or_default(&mut reader)?;
        let user_quote_amount_in: u64 = crate::borsh_de_or_default(&mut reader)?;
        let pool: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let user: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let user_base_token_account: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let user_quote_token_account: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let protocol_fee_recipient: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let protocol_fee_recipient_token_account: Pubkey = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let coin_creator: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let coin_creator_fee_basis_points: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let coin_creator_fee: u64 = crate::borsh_de_or_default(&mut reader)?;
        let track_volume: bool = crate::borsh_de_or_default(&mut reader)?;
        let total_unclaimed_tokens: u64 = crate::borsh_de_or_default(&mut reader)?;
        let total_claimed_tokens: u64 = crate::borsh_de_or_default(&mut reader)?;
        let current_sol_volume: u64 = crate::borsh_de_or_default(&mut reader)?;
        let last_update_timestamp: i64 = crate::borsh_de_or_default(&mut reader)?;
        let min_base_amount_out: u64 = crate::borsh_de_or_default(&mut reader)?;
        let ix_name: String = crate::borsh_de_or_default(&mut reader)?;
        let cashback_fee_basis_points: u64 = crate::borsh_de_or_default(&mut reader)?;
        let cashback: u64 = crate::borsh_de_or_default(&mut reader)?;
        let buyback_fee_basis_points: u64 = crate::borsh_de_or_default(&mut reader)?;
        let buyback_fee: u64 = crate::borsh_de_or_default(&mut reader)?;
        let virtual_quote_reserves: i128 = crate::borsh_de_or_default(&mut reader)?;
        let can_boost: bool = crate::borsh_de_or_default(&mut reader)?;
        let base_supply: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            timestamp,
            base_amount_out,
            max_quote_amount_in,
            user_base_token_reserves,
            user_quote_token_reserves,
            pool_base_token_reserves,
            pool_quote_token_reserves,
            quote_amount_in,
            lp_fee_basis_points,
            lp_fee,
            protocol_fee_basis_points,
            protocol_fee,
            quote_amount_in_with_lp_fee,
            user_quote_amount_in,
            pool,
            user,
            user_base_token_account,
            user_quote_token_account,
            protocol_fee_recipient,
            protocol_fee_recipient_token_account,
            coin_creator,
            coin_creator_fee_basis_points,
            coin_creator_fee,
            track_volume,
            total_unclaimed_tokens,
            total_claimed_tokens,
            current_sol_volume,
            last_update_timestamp,
            min_base_amount_out,
            ix_name,
            cashback_fee_basis_points,
            cashback,
            buyback_fee_basis_points,
            buyback_fee,
            virtual_quote_reserves,
            can_boost,
            base_supply,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.timestamp, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.base_amount_out, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.max_quote_amount_in, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.user_base_token_reserves, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.user_quote_token_reserves, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.pool_base_token_reserves, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.pool_quote_token_reserves, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.quote_amount_in, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.lp_fee_basis_points, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.lp_fee, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.protocol_fee_basis_points, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.protocol_fee, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.quote_amount_in_with_lp_fee,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.user_quote_amount_in, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.pool, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.user, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.user_base_token_account, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.user_quote_token_account, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.protocol_fee_recipient, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.protocol_fee_recipient_token_account,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.coin_creator, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.coin_creator_fee_basis_points,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.coin_creator_fee, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.track_volume, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.total_unclaimed_tokens, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.total_claimed_tokens, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.current_sol_volume, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.last_update_timestamp, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.min_base_amount_out, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.ix_name, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.cashback_fee_basis_points, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.cashback, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.buyback_fee_basis_points, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.buyback_fee, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.virtual_quote_reserves, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.can_boost, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.base_supply, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct BuyEventEvent(pub BuyEvent);
impl BuyEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != BUY_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = BuyEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&BUY_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const CLAIM_CASHBACK_EVENT_EVENT_DISCM: [u8; 8] = [
    226, 214, 246, 33, 7, 242, 147, 229,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ClaimCashbackEvent {
    pub user: Pubkey,
    pub amount: u64,
    pub timestamp: i64,
    pub total_claimed: u64,
    pub total_cashback_earned: u64,
}
impl ClaimCashbackEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let user: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let timestamp: i64 = crate::borsh_de_or_default(&mut reader)?;
        let total_claimed: u64 = crate::borsh_de_or_default(&mut reader)?;
        let total_cashback_earned: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            user,
            amount,
            timestamp,
            total_claimed,
            total_cashback_earned,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.user, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.timestamp, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.total_claimed, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.total_cashback_earned, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct ClaimCashbackEventEvent(pub ClaimCashbackEvent);
impl ClaimCashbackEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CLAIM_CASHBACK_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = ClaimCashbackEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CLAIM_CASHBACK_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const CLAIM_TOKEN_INCENTIVES_EVENT_EVENT_DISCM: [u8; 8] = [
    79, 172, 246, 49, 205, 91, 206, 232,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ClaimTokenIncentivesEvent {
    pub user: Pubkey,
    pub mint: Pubkey,
    pub amount: u64,
    pub timestamp: i64,
    pub total_claimed_tokens: u64,
    pub current_sol_volume: u64,
}
impl ClaimTokenIncentivesEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let user: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let timestamp: i64 = crate::borsh_de_or_default(&mut reader)?;
        let total_claimed_tokens: u64 = crate::borsh_de_or_default(&mut reader)?;
        let current_sol_volume: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            user,
            mint,
            amount,
            timestamp,
            total_claimed_tokens,
            current_sol_volume,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.user, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.timestamp, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.total_claimed_tokens, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.current_sol_volume, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct ClaimTokenIncentivesEventEvent(pub ClaimTokenIncentivesEvent);
impl ClaimTokenIncentivesEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CLAIM_TOKEN_INCENTIVES_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = ClaimTokenIncentivesEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CLAIM_TOKEN_INCENTIVES_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const CLOSE_USER_VOLUME_ACCUMULATOR_EVENT_EVENT_DISCM: [u8; 8] = [
    146, 159, 189, 172, 146, 88, 56, 244,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CloseUserVolumeAccumulatorEvent {
    pub user: Pubkey,
    pub timestamp: i64,
    pub total_unclaimed_tokens: u64,
    pub total_claimed_tokens: u64,
    pub current_sol_volume: u64,
    pub last_update_timestamp: i64,
}
impl CloseUserVolumeAccumulatorEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let user: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let timestamp: i64 = crate::borsh_de_or_default(&mut reader)?;
        let total_unclaimed_tokens: u64 = crate::borsh_de_or_default(&mut reader)?;
        let total_claimed_tokens: u64 = crate::borsh_de_or_default(&mut reader)?;
        let current_sol_volume: u64 = crate::borsh_de_or_default(&mut reader)?;
        let last_update_timestamp: i64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            user,
            timestamp,
            total_unclaimed_tokens,
            total_claimed_tokens,
            current_sol_volume,
            last_update_timestamp,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.user, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.timestamp, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.total_unclaimed_tokens, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.total_claimed_tokens, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.current_sol_volume, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.last_update_timestamp, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct CloseUserVolumeAccumulatorEventEvent(pub CloseUserVolumeAccumulatorEvent);
impl CloseUserVolumeAccumulatorEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CLOSE_USER_VOLUME_ACCUMULATOR_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = CloseUserVolumeAccumulatorEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CLOSE_USER_VOLUME_ACCUMULATOR_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const COLLECT_COIN_CREATOR_FEE_EVENT_EVENT_DISCM: [u8; 8] = [
    232, 245, 194, 238, 234, 218, 58, 89,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CollectCoinCreatorFeeEvent {
    pub timestamp: i64,
    pub coin_creator: Pubkey,
    pub coin_creator_fee: u64,
    pub coin_creator_vault_ata: Pubkey,
    pub coin_creator_token_account: Pubkey,
}
impl CollectCoinCreatorFeeEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let timestamp: i64 = crate::borsh_de_or_default(&mut reader)?;
        let coin_creator: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let coin_creator_fee: u64 = crate::borsh_de_or_default(&mut reader)?;
        let coin_creator_vault_ata: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let coin_creator_token_account: Pubkey = crate::borsh_de_or_default(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self {
            timestamp,
            coin_creator,
            coin_creator_fee,
            coin_creator_vault_ata,
            coin_creator_token_account,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.timestamp, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.coin_creator, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.coin_creator_fee, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.coin_creator_vault_ata, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.coin_creator_token_account, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct CollectCoinCreatorFeeEventEvent(pub CollectCoinCreatorFeeEvent);
impl CollectCoinCreatorFeeEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != COLLECT_COIN_CREATOR_FEE_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = CollectCoinCreatorFeeEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&COLLECT_COIN_CREATOR_FEE_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const CREATE_CONFIG_EVENT_EVENT_DISCM: [u8; 8] = [107, 52, 89, 129, 55, 226, 81, 22];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CreateConfigEvent {
    pub timestamp: i64,
    pub admin: Pubkey,
    pub lp_fee_basis_points: u64,
    pub protocol_fee_basis_points: u64,
    pub protocol_fee_recipients: [Pubkey; 8],
    pub coin_creator_fee_basis_points: u64,
    pub admin_set_coin_creator_authority: Pubkey,
}
impl CreateConfigEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let timestamp: i64 = crate::borsh_de_or_default(&mut reader)?;
        let admin: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let lp_fee_basis_points: u64 = crate::borsh_de_or_default(&mut reader)?;
        let protocol_fee_basis_points: u64 = crate::borsh_de_or_default(&mut reader)?;
        let protocol_fee_recipients: [Pubkey; 8] = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let coin_creator_fee_basis_points: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let admin_set_coin_creator_authority: Pubkey = crate::borsh_de_or_default(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self {
            timestamp,
            admin,
            lp_fee_basis_points,
            protocol_fee_basis_points,
            protocol_fee_recipients,
            coin_creator_fee_basis_points,
            admin_set_coin_creator_authority,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.timestamp, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.admin, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.lp_fee_basis_points, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.protocol_fee_basis_points, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.protocol_fee_recipients, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.coin_creator_fee_basis_points,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.admin_set_coin_creator_authority,
            &mut writer,
        )?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct CreateConfigEventEvent(pub CreateConfigEvent);
impl CreateConfigEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CREATE_CONFIG_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = CreateConfigEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CREATE_CONFIG_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const CREATE_POOL_EVENT_EVENT_DISCM: [u8; 8] = [
    177, 49, 12, 210, 160, 118, 167, 116,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CreatePoolEvent {
    pub timestamp: i64,
    pub index: u16,
    pub creator: Pubkey,
    pub base_mint: Pubkey,
    pub quote_mint: Pubkey,
    pub base_mint_decimals: u8,
    pub quote_mint_decimals: u8,
    pub base_amount_in: u64,
    pub quote_amount_in: u64,
    pub pool_base_amount: u64,
    pub pool_quote_amount: u64,
    pub minimum_liquidity: u64,
    pub initial_liquidity: u64,
    pub lp_token_amount_out: u64,
    pub pool_bump: u8,
    pub pool: Pubkey,
    pub lp_mint: Pubkey,
    pub user_base_token_account: Pubkey,
    pub user_quote_token_account: Pubkey,
    pub coin_creator: Pubkey,
    pub is_mayhem_mode: bool,
    pub creator_fee_bps: u64,
    pub can_edit_creator_fee: bool,
}
impl CreatePoolEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let timestamp: i64 = crate::borsh_de_or_default(&mut reader)?;
        let index: u16 = crate::borsh_de_or_default(&mut reader)?;
        let creator: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let base_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let quote_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let base_mint_decimals: u8 = crate::borsh_de_or_default(&mut reader)?;
        let quote_mint_decimals: u8 = crate::borsh_de_or_default(&mut reader)?;
        let base_amount_in: u64 = crate::borsh_de_or_default(&mut reader)?;
        let quote_amount_in: u64 = crate::borsh_de_or_default(&mut reader)?;
        let pool_base_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let pool_quote_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let minimum_liquidity: u64 = crate::borsh_de_or_default(&mut reader)?;
        let initial_liquidity: u64 = crate::borsh_de_or_default(&mut reader)?;
        let lp_token_amount_out: u64 = crate::borsh_de_or_default(&mut reader)?;
        let pool_bump: u8 = crate::borsh_de_or_default(&mut reader)?;
        let pool: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let lp_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let user_base_token_account: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let user_quote_token_account: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let coin_creator: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let is_mayhem_mode: bool = crate::borsh_de_or_default(&mut reader)?;
        let creator_fee_bps: u64 = crate::borsh_de_or_default(&mut reader)?;
        let can_edit_creator_fee: bool = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            timestamp,
            index,
            creator,
            base_mint,
            quote_mint,
            base_mint_decimals,
            quote_mint_decimals,
            base_amount_in,
            quote_amount_in,
            pool_base_amount,
            pool_quote_amount,
            minimum_liquidity,
            initial_liquidity,
            lp_token_amount_out,
            pool_bump,
            pool,
            lp_mint,
            user_base_token_account,
            user_quote_token_account,
            coin_creator,
            is_mayhem_mode,
            creator_fee_bps,
            can_edit_creator_fee,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.timestamp, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.index, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.creator, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.base_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.quote_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.base_mint_decimals, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.quote_mint_decimals, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.base_amount_in, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.quote_amount_in, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.pool_base_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.pool_quote_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.minimum_liquidity, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.initial_liquidity, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.lp_token_amount_out, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.pool_bump, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.pool, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.lp_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.user_base_token_account, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.user_quote_token_account, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.coin_creator, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.is_mayhem_mode, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.creator_fee_bps, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.can_edit_creator_fee, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct CreatePoolEventEvent(pub CreatePoolEvent);
impl CreatePoolEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CREATE_POOL_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = CreatePoolEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CREATE_POOL_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const DEPOSIT_EVENT_EVENT_DISCM: [u8; 8] = [120, 248, 61, 83, 31, 142, 107, 144];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DepositEvent {
    pub timestamp: i64,
    pub lp_token_amount_out: u64,
    pub max_base_amount_in: u64,
    pub max_quote_amount_in: u64,
    pub user_base_token_reserves: u64,
    pub user_quote_token_reserves: u64,
    pub pool_base_token_reserves: u64,
    pub pool_quote_token_reserves: u64,
    pub base_amount_in: u64,
    pub quote_amount_in: u64,
    pub lp_mint_supply: u64,
    pub pool: Pubkey,
    pub user: Pubkey,
    pub user_base_token_account: Pubkey,
    pub user_quote_token_account: Pubkey,
    pub user_pool_token_account: Pubkey,
}
impl DepositEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let timestamp: i64 = crate::borsh_de_or_default(&mut reader)?;
        let lp_token_amount_out: u64 = crate::borsh_de_or_default(&mut reader)?;
        let max_base_amount_in: u64 = crate::borsh_de_or_default(&mut reader)?;
        let max_quote_amount_in: u64 = crate::borsh_de_or_default(&mut reader)?;
        let user_base_token_reserves: u64 = crate::borsh_de_or_default(&mut reader)?;
        let user_quote_token_reserves: u64 = crate::borsh_de_or_default(&mut reader)?;
        let pool_base_token_reserves: u64 = crate::borsh_de_or_default(&mut reader)?;
        let pool_quote_token_reserves: u64 = crate::borsh_de_or_default(&mut reader)?;
        let base_amount_in: u64 = crate::borsh_de_or_default(&mut reader)?;
        let quote_amount_in: u64 = crate::borsh_de_or_default(&mut reader)?;
        let lp_mint_supply: u64 = crate::borsh_de_or_default(&mut reader)?;
        let pool: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let user: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let user_base_token_account: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let user_quote_token_account: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let user_pool_token_account: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            timestamp,
            lp_token_amount_out,
            max_base_amount_in,
            max_quote_amount_in,
            user_base_token_reserves,
            user_quote_token_reserves,
            pool_base_token_reserves,
            pool_quote_token_reserves,
            base_amount_in,
            quote_amount_in,
            lp_mint_supply,
            pool,
            user,
            user_base_token_account,
            user_quote_token_account,
            user_pool_token_account,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.timestamp, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.lp_token_amount_out, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.max_base_amount_in, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.max_quote_amount_in, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.user_base_token_reserves, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.user_quote_token_reserves, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.pool_base_token_reserves, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.pool_quote_token_reserves, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.base_amount_in, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.quote_amount_in, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.lp_mint_supply, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.pool, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.user, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.user_base_token_account, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.user_quote_token_account, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.user_pool_token_account, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct DepositEventEvent(pub DepositEvent);
impl DepositEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != DEPOSIT_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = DepositEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&DEPOSIT_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const DISABLE_EVENT_EVENT_DISCM: [u8; 8] = [107, 253, 193, 76, 228, 202, 27, 104];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DisableEvent {
    pub timestamp: i64,
    pub admin: Pubkey,
    pub disable_create_pool: bool,
    pub disable_deposit: bool,
    pub disable_withdraw: bool,
    pub disable_buy: bool,
    pub disable_sell: bool,
}
impl DisableEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let timestamp: i64 = crate::borsh_de_or_default(&mut reader)?;
        let admin: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let disable_create_pool: bool = crate::borsh_de_or_default(&mut reader)?;
        let disable_deposit: bool = crate::borsh_de_or_default(&mut reader)?;
        let disable_withdraw: bool = crate::borsh_de_or_default(&mut reader)?;
        let disable_buy: bool = crate::borsh_de_or_default(&mut reader)?;
        let disable_sell: bool = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            timestamp,
            admin,
            disable_create_pool,
            disable_deposit,
            disable_withdraw,
            disable_buy,
            disable_sell,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.timestamp, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.admin, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.disable_create_pool, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.disable_deposit, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.disable_withdraw, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.disable_buy, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.disable_sell, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct DisableEventEvent(pub DisableEvent);
impl DisableEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != DISABLE_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = DisableEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&DISABLE_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const EXTEND_ACCOUNT_EVENT_EVENT_DISCM: [u8; 8] = [
    97, 97, 215, 144, 93, 146, 22, 124,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ExtendAccountEvent {
    pub timestamp: i64,
    pub account: Pubkey,
    pub user: Pubkey,
    pub current_size: u64,
    pub new_size: u64,
}
impl ExtendAccountEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let timestamp: i64 = crate::borsh_de_or_default(&mut reader)?;
        let account: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let user: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let current_size: u64 = crate::borsh_de_or_default(&mut reader)?;
        let new_size: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            timestamp,
            account,
            user,
            current_size,
            new_size,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.timestamp, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.account, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.user, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.current_size, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.new_size, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct ExtendAccountEventEvent(pub ExtendAccountEvent);
impl ExtendAccountEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != EXTEND_ACCOUNT_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = ExtendAccountEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&EXTEND_ACCOUNT_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const INIT_BOOST_EVENT_EVENT_DISCM: [u8; 8] = [174, 124, 74, 249, 4, 81, 246, 17];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct InitBoostEvent {
    pub timestamp: i64,
    pub mint: Pubkey,
    pub bonding_curve: Pubkey,
    pub pool: Pubkey,
    pub virtual_quote_reserves: i128,
    pub real_quote_reserves_after: u64,
}
impl InitBoostEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let timestamp: i64 = crate::borsh_de_or_default(&mut reader)?;
        let mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let bonding_curve: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let pool: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let virtual_quote_reserves: i128 = crate::borsh_de_or_default(&mut reader)?;
        let real_quote_reserves_after: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            timestamp,
            mint,
            bonding_curve,
            pool,
            virtual_quote_reserves,
            real_quote_reserves_after,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.timestamp, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.bonding_curve, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.pool, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.virtual_quote_reserves, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.real_quote_reserves_after, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct InitBoostEventEvent(pub InitBoostEvent);
impl InitBoostEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INIT_BOOST_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = InitBoostEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INIT_BOOST_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const INIT_USER_VOLUME_ACCUMULATOR_EVENT_EVENT_DISCM: [u8; 8] = [
    134, 36, 13, 72, 232, 101, 130, 216,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct InitUserVolumeAccumulatorEvent {
    pub payer: Pubkey,
    pub user: Pubkey,
    pub timestamp: i64,
}
impl InitUserVolumeAccumulatorEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let payer: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let user: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let timestamp: i64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { payer, user, timestamp })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.payer, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.user, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.timestamp, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct InitUserVolumeAccumulatorEventEvent(pub InitUserVolumeAccumulatorEvent);
impl InitUserVolumeAccumulatorEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INIT_USER_VOLUME_ACCUMULATOR_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = InitUserVolumeAccumulatorEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INIT_USER_VOLUME_ACCUMULATOR_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const MIGRATE_POOL_COIN_CREATOR_EVENT_EVENT_DISCM: [u8; 8] = [
    170, 221, 82, 199, 147, 165, 247, 46,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MigratePoolCoinCreatorEvent {
    pub timestamp: i64,
    pub base_mint: Pubkey,
    pub pool: Pubkey,
    pub sharing_config: Pubkey,
    pub old_coin_creator: Pubkey,
    pub new_coin_creator: Pubkey,
}
impl MigratePoolCoinCreatorEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let timestamp: i64 = crate::borsh_de_or_default(&mut reader)?;
        let base_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let pool: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let sharing_config: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let old_coin_creator: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let new_coin_creator: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            timestamp,
            base_mint,
            pool,
            sharing_config,
            old_coin_creator,
            new_coin_creator,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.timestamp, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.base_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.pool, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.sharing_config, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.old_coin_creator, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.new_coin_creator, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct MigratePoolCoinCreatorEventEvent(pub MigratePoolCoinCreatorEvent);
impl MigratePoolCoinCreatorEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != MIGRATE_POOL_COIN_CREATOR_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = MigratePoolCoinCreatorEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&MIGRATE_POOL_COIN_CREATOR_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const RESERVED_FEE_RECIPIENTS_EVENT_EVENT_DISCM: [u8; 8] = [
    43, 188, 250, 18, 221, 75, 187, 95,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ReservedFeeRecipientsEvent {
    pub timestamp: i64,
    pub reserved_fee_recipient: Pubkey,
    pub reserved_fee_recipients: [Pubkey; 7],
}
impl ReservedFeeRecipientsEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let timestamp: i64 = crate::borsh_de_or_default(&mut reader)?;
        let reserved_fee_recipient: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let reserved_fee_recipients: [Pubkey; 7] = crate::borsh_de_or_default(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self {
            timestamp,
            reserved_fee_recipient,
            reserved_fee_recipients,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.timestamp, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.reserved_fee_recipient, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.reserved_fee_recipients, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct ReservedFeeRecipientsEventEvent(pub ReservedFeeRecipientsEvent);
impl ReservedFeeRecipientsEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != RESERVED_FEE_RECIPIENTS_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = ReservedFeeRecipientsEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&RESERVED_FEE_RECIPIENTS_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const SELL_EVENT_EVENT_DISCM: [u8; 8] = [62, 47, 55, 10, 165, 3, 220, 42];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SellEvent {
    pub timestamp: i64,
    pub base_amount_in: u64,
    pub min_quote_amount_out: u64,
    pub user_base_token_reserves: u64,
    pub user_quote_token_reserves: u64,
    pub pool_base_token_reserves: u64,
    pub pool_quote_token_reserves: u64,
    pub quote_amount_out: u64,
    pub lp_fee_basis_points: u64,
    pub lp_fee: u64,
    pub protocol_fee_basis_points: u64,
    pub protocol_fee: u64,
    pub quote_amount_out_without_lp_fee: u64,
    pub user_quote_amount_out: u64,
    pub pool: Pubkey,
    pub user: Pubkey,
    pub user_base_token_account: Pubkey,
    pub user_quote_token_account: Pubkey,
    pub protocol_fee_recipient: Pubkey,
    pub protocol_fee_recipient_token_account: Pubkey,
    pub coin_creator: Pubkey,
    pub coin_creator_fee_basis_points: u64,
    pub coin_creator_fee: u64,
    pub cashback_fee_basis_points: u64,
    pub cashback: u64,
    pub buyback_fee_basis_points: u64,
    pub buyback_fee: u64,
    pub virtual_quote_reserves: i128,
    pub can_boost: bool,
    pub base_supply: u64,
}
impl SellEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let timestamp: i64 = crate::borsh_de_or_default(&mut reader)?;
        let base_amount_in: u64 = crate::borsh_de_or_default(&mut reader)?;
        let min_quote_amount_out: u64 = crate::borsh_de_or_default(&mut reader)?;
        let user_base_token_reserves: u64 = crate::borsh_de_or_default(&mut reader)?;
        let user_quote_token_reserves: u64 = crate::borsh_de_or_default(&mut reader)?;
        let pool_base_token_reserves: u64 = crate::borsh_de_or_default(&mut reader)?;
        let pool_quote_token_reserves: u64 = crate::borsh_de_or_default(&mut reader)?;
        let quote_amount_out: u64 = crate::borsh_de_or_default(&mut reader)?;
        let lp_fee_basis_points: u64 = crate::borsh_de_or_default(&mut reader)?;
        let lp_fee: u64 = crate::borsh_de_or_default(&mut reader)?;
        let protocol_fee_basis_points: u64 = crate::borsh_de_or_default(&mut reader)?;
        let protocol_fee: u64 = crate::borsh_de_or_default(&mut reader)?;
        let quote_amount_out_without_lp_fee: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let user_quote_amount_out: u64 = crate::borsh_de_or_default(&mut reader)?;
        let pool: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let user: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let user_base_token_account: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let user_quote_token_account: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let protocol_fee_recipient: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let protocol_fee_recipient_token_account: Pubkey = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let coin_creator: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let coin_creator_fee_basis_points: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let coin_creator_fee: u64 = crate::borsh_de_or_default(&mut reader)?;
        let cashback_fee_basis_points: u64 = crate::borsh_de_or_default(&mut reader)?;
        let cashback: u64 = crate::borsh_de_or_default(&mut reader)?;
        let buyback_fee_basis_points: u64 = crate::borsh_de_or_default(&mut reader)?;
        let buyback_fee: u64 = crate::borsh_de_or_default(&mut reader)?;
        let virtual_quote_reserves: i128 = crate::borsh_de_or_default(&mut reader)?;
        let can_boost: bool = crate::borsh_de_or_default(&mut reader)?;
        let base_supply: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            timestamp,
            base_amount_in,
            min_quote_amount_out,
            user_base_token_reserves,
            user_quote_token_reserves,
            pool_base_token_reserves,
            pool_quote_token_reserves,
            quote_amount_out,
            lp_fee_basis_points,
            lp_fee,
            protocol_fee_basis_points,
            protocol_fee,
            quote_amount_out_without_lp_fee,
            user_quote_amount_out,
            pool,
            user,
            user_base_token_account,
            user_quote_token_account,
            protocol_fee_recipient,
            protocol_fee_recipient_token_account,
            coin_creator,
            coin_creator_fee_basis_points,
            coin_creator_fee,
            cashback_fee_basis_points,
            cashback,
            buyback_fee_basis_points,
            buyback_fee,
            virtual_quote_reserves,
            can_boost,
            base_supply,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.timestamp, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.base_amount_in, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.min_quote_amount_out, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.user_base_token_reserves, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.user_quote_token_reserves, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.pool_base_token_reserves, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.pool_quote_token_reserves, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.quote_amount_out, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.lp_fee_basis_points, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.lp_fee, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.protocol_fee_basis_points, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.protocol_fee, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.quote_amount_out_without_lp_fee,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.user_quote_amount_out, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.pool, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.user, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.user_base_token_account, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.user_quote_token_account, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.protocol_fee_recipient, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.protocol_fee_recipient_token_account,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.coin_creator, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.coin_creator_fee_basis_points,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.coin_creator_fee, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.cashback_fee_basis_points, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.cashback, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.buyback_fee_basis_points, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.buyback_fee, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.virtual_quote_reserves, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.can_boost, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.base_supply, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct SellEventEvent(pub SellEvent);
impl SellEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SELL_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = SellEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SELL_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const SET_BONDING_CURVE_COIN_CREATOR_EVENT_EVENT_DISCM: [u8; 8] = [
    242, 231, 235, 102, 65, 99, 189, 211,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SetBondingCurveCoinCreatorEvent {
    pub timestamp: i64,
    pub base_mint: Pubkey,
    pub pool: Pubkey,
    pub bonding_curve: Pubkey,
    pub coin_creator: Pubkey,
}
impl SetBondingCurveCoinCreatorEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let timestamp: i64 = crate::borsh_de_or_default(&mut reader)?;
        let base_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let pool: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let bonding_curve: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let coin_creator: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            timestamp,
            base_mint,
            pool,
            bonding_curve,
            coin_creator,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.timestamp, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.base_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.pool, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.bonding_curve, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.coin_creator, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct SetBondingCurveCoinCreatorEventEvent(pub SetBondingCurveCoinCreatorEvent);
impl SetBondingCurveCoinCreatorEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SET_BONDING_CURVE_COIN_CREATOR_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = SetBondingCurveCoinCreatorEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SET_BONDING_CURVE_COIN_CREATOR_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const SET_BOOST_AUTHORITY_EVENT_EVENT_DISCM: [u8; 8] = [
    89, 128, 240, 141, 91, 202, 71, 105,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SetBoostAuthorityEvent {
    pub timestamp: i64,
    pub admin: Pubkey,
    pub old_boost_authority: Pubkey,
    pub new_boost_authority: Pubkey,
}
impl SetBoostAuthorityEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let timestamp: i64 = crate::borsh_de_or_default(&mut reader)?;
        let admin: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let old_boost_authority: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let new_boost_authority: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            timestamp,
            admin,
            old_boost_authority,
            new_boost_authority,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.timestamp, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.admin, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.old_boost_authority, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.new_boost_authority, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct SetBoostAuthorityEventEvent(pub SetBoostAuthorityEvent);
impl SetBoostAuthorityEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SET_BOOST_AUTHORITY_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = SetBoostAuthorityEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SET_BOOST_AUTHORITY_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const SET_COIN_CREATOR_FEE_BPS_EVENT_EVENT_DISCM: [u8; 8] = [
    230, 138, 241, 158, 28, 232, 228, 211,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SetCoinCreatorFeeBpsEvent {
    pub timestamp: i64,
    pub authority: Pubkey,
    pub base_mint: Pubkey,
    pub pool: Pubkey,
    pub old_creator_fee_bps: u64,
    pub new_creator_fee_bps: u64,
}
impl SetCoinCreatorFeeBpsEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let timestamp: i64 = crate::borsh_de_or_default(&mut reader)?;
        let authority: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let base_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let pool: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let old_creator_fee_bps: u64 = crate::borsh_de_or_default(&mut reader)?;
        let new_creator_fee_bps: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            timestamp,
            authority,
            base_mint,
            pool,
            old_creator_fee_bps,
            new_creator_fee_bps,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.timestamp, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.authority, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.base_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.pool, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.old_creator_fee_bps, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.new_creator_fee_bps, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct SetCoinCreatorFeeBpsEventEvent(pub SetCoinCreatorFeeBpsEvent);
impl SetCoinCreatorFeeBpsEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SET_COIN_CREATOR_FEE_BPS_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = SetCoinCreatorFeeBpsEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SET_COIN_CREATOR_FEE_BPS_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const SET_METAPLEX_COIN_CREATOR_EVENT_EVENT_DISCM: [u8; 8] = [
    150, 107, 199, 123, 124, 207, 102, 228,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SetMetaplexCoinCreatorEvent {
    pub timestamp: i64,
    pub base_mint: Pubkey,
    pub pool: Pubkey,
    pub metadata: Pubkey,
    pub coin_creator: Pubkey,
}
impl SetMetaplexCoinCreatorEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let timestamp: i64 = crate::borsh_de_or_default(&mut reader)?;
        let base_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let pool: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let metadata: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let coin_creator: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            timestamp,
            base_mint,
            pool,
            metadata,
            coin_creator,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.timestamp, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.base_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.pool, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.metadata, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.coin_creator, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct SetMetaplexCoinCreatorEventEvent(pub SetMetaplexCoinCreatorEvent);
impl SetMetaplexCoinCreatorEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SET_METAPLEX_COIN_CREATOR_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = SetMetaplexCoinCreatorEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SET_METAPLEX_COIN_CREATOR_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const SYNC_USER_VOLUME_ACCUMULATOR_EVENT_EVENT_DISCM: [u8; 8] = [
    197, 122, 167, 124, 116, 81, 91, 255,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SyncUserVolumeAccumulatorEvent {
    pub user: Pubkey,
    pub total_claimed_tokens_before: u64,
    pub total_claimed_tokens_after: u64,
    pub timestamp: i64,
}
impl SyncUserVolumeAccumulatorEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let user: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let total_claimed_tokens_before: u64 = crate::borsh_de_or_default(&mut reader)?;
        let total_claimed_tokens_after: u64 = crate::borsh_de_or_default(&mut reader)?;
        let timestamp: i64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            user,
            total_claimed_tokens_before,
            total_claimed_tokens_after,
            timestamp,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.user, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.total_claimed_tokens_before,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.total_claimed_tokens_after, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.timestamp, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct SyncUserVolumeAccumulatorEventEvent(pub SyncUserVolumeAccumulatorEvent);
impl SyncUserVolumeAccumulatorEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SYNC_USER_VOLUME_ACCUMULATOR_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = SyncUserVolumeAccumulatorEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SYNC_USER_VOLUME_ACCUMULATOR_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const UPDATE_ADMIN_EVENT_EVENT_DISCM: [u8; 8] = [
    225, 152, 171, 87, 246, 63, 66, 234,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UpdateAdminEvent {
    pub timestamp: i64,
    pub admin: Pubkey,
    pub new_admin: Pubkey,
}
impl UpdateAdminEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let timestamp: i64 = crate::borsh_de_or_default(&mut reader)?;
        let admin: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let new_admin: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            timestamp,
            admin,
            new_admin,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.timestamp, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.admin, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.new_admin, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateAdminEventEvent(pub UpdateAdminEvent);
impl UpdateAdminEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_ADMIN_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = UpdateAdminEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_ADMIN_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const UPDATE_CREATOR_FEE_CONFIG_EVENT_EVENT_DISCM: [u8; 8] = [
    152, 198, 124, 124, 106, 246, 127, 191,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UpdateCreatorFeeConfigEvent {
    pub timestamp: i64,
    pub admin: Pubkey,
    pub creator_fee_configurable: bool,
    pub max_configurable_creator_fee_bps: u64,
}
impl UpdateCreatorFeeConfigEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let timestamp: i64 = crate::borsh_de_or_default(&mut reader)?;
        let admin: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let creator_fee_configurable: bool = crate::borsh_de_or_default(&mut reader)?;
        let max_configurable_creator_fee_bps: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self {
            timestamp,
            admin,
            creator_fee_configurable,
            max_configurable_creator_fee_bps,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.timestamp, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.admin, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.creator_fee_configurable, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.max_configurable_creator_fee_bps,
            &mut writer,
        )?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateCreatorFeeConfigEventEvent(pub UpdateCreatorFeeConfigEvent);
impl UpdateCreatorFeeConfigEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_CREATOR_FEE_CONFIG_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = UpdateCreatorFeeConfigEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_CREATOR_FEE_CONFIG_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const UPDATE_FEE_CONFIG_EVENT_EVENT_DISCM: [u8; 8] = [
    90, 23, 65, 35, 62, 244, 188, 208,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UpdateFeeConfigEvent {
    pub timestamp: i64,
    pub admin: Pubkey,
    pub lp_fee_basis_points: u64,
    pub protocol_fee_basis_points: u64,
    pub protocol_fee_recipients: [Pubkey; 8],
    pub coin_creator_fee_basis_points: u64,
    pub admin_set_coin_creator_authority: Pubkey,
}
impl UpdateFeeConfigEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let timestamp: i64 = crate::borsh_de_or_default(&mut reader)?;
        let admin: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let lp_fee_basis_points: u64 = crate::borsh_de_or_default(&mut reader)?;
        let protocol_fee_basis_points: u64 = crate::borsh_de_or_default(&mut reader)?;
        let protocol_fee_recipients: [Pubkey; 8] = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let coin_creator_fee_basis_points: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let admin_set_coin_creator_authority: Pubkey = crate::borsh_de_or_default(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self {
            timestamp,
            admin,
            lp_fee_basis_points,
            protocol_fee_basis_points,
            protocol_fee_recipients,
            coin_creator_fee_basis_points,
            admin_set_coin_creator_authority,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.timestamp, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.admin, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.lp_fee_basis_points, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.protocol_fee_basis_points, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.protocol_fee_recipients, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.coin_creator_fee_basis_points,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.admin_set_coin_creator_authority,
            &mut writer,
        )?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateFeeConfigEventEvent(pub UpdateFeeConfigEvent);
impl UpdateFeeConfigEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_FEE_CONFIG_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = UpdateFeeConfigEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_FEE_CONFIG_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const WITHDRAW_EVENT_EVENT_DISCM: [u8; 8] = [22, 9, 133, 26, 160, 44, 71, 192];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WithdrawEvent {
    pub timestamp: i64,
    pub lp_token_amount_in: u64,
    pub min_base_amount_out: u64,
    pub min_quote_amount_out: u64,
    pub user_base_token_reserves: u64,
    pub user_quote_token_reserves: u64,
    pub pool_base_token_reserves: u64,
    pub pool_quote_token_reserves: u64,
    pub base_amount_out: u64,
    pub quote_amount_out: u64,
    pub lp_mint_supply: u64,
    pub pool: Pubkey,
    pub user: Pubkey,
    pub user_base_token_account: Pubkey,
    pub user_quote_token_account: Pubkey,
    pub user_pool_token_account: Pubkey,
}
impl WithdrawEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let timestamp: i64 = crate::borsh_de_or_default(&mut reader)?;
        let lp_token_amount_in: u64 = crate::borsh_de_or_default(&mut reader)?;
        let min_base_amount_out: u64 = crate::borsh_de_or_default(&mut reader)?;
        let min_quote_amount_out: u64 = crate::borsh_de_or_default(&mut reader)?;
        let user_base_token_reserves: u64 = crate::borsh_de_or_default(&mut reader)?;
        let user_quote_token_reserves: u64 = crate::borsh_de_or_default(&mut reader)?;
        let pool_base_token_reserves: u64 = crate::borsh_de_or_default(&mut reader)?;
        let pool_quote_token_reserves: u64 = crate::borsh_de_or_default(&mut reader)?;
        let base_amount_out: u64 = crate::borsh_de_or_default(&mut reader)?;
        let quote_amount_out: u64 = crate::borsh_de_or_default(&mut reader)?;
        let lp_mint_supply: u64 = crate::borsh_de_or_default(&mut reader)?;
        let pool: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let user: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let user_base_token_account: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let user_quote_token_account: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let user_pool_token_account: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            timestamp,
            lp_token_amount_in,
            min_base_amount_out,
            min_quote_amount_out,
            user_base_token_reserves,
            user_quote_token_reserves,
            pool_base_token_reserves,
            pool_quote_token_reserves,
            base_amount_out,
            quote_amount_out,
            lp_mint_supply,
            pool,
            user,
            user_base_token_account,
            user_quote_token_account,
            user_pool_token_account,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.timestamp, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.lp_token_amount_in, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.min_base_amount_out, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.min_quote_amount_out, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.user_base_token_reserves, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.user_quote_token_reserves, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.pool_base_token_reserves, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.pool_quote_token_reserves, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.base_amount_out, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.quote_amount_out, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.lp_mint_supply, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.pool, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.user, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.user_base_token_account, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.user_quote_token_account, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.user_pool_token_account, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct WithdrawEventEvent(pub WithdrawEvent);
impl WithdrawEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != WITHDRAW_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = WithdrawEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&WITHDRAW_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
