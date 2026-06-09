use borsh::{BorshDeserialize, BorshSerialize};
use solana_pubkey::Pubkey;
#[allow(unused_imports)]
use crate::*;
pub const FUND_STATE_ACCOUNT_DISCM: [u8; 8] = [3, 254, 145, 43, 146, 96, 162, 104];
#[derive(
    Clone,
    Debug,
    BorshDeserialize,
    BorshSerialize,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
pub struct FundState {
    pub version: u64,
    pub manager: Pubkey,
    pub fund_token: Pubkey,
    pub manager_fee: u64,
    pub supply_outsanding: u64,
    pub actively_managed: u64,
    pub active_buy_states: u64,
    pub sell_state: u64,
    pub rebalance_sell_state: u64,
    pub host_pubkey: Pubkey,
    pub host_fee: u64,
    pub num_of_tokens: u64,
    pub current_comp_token: [u64; 20],
    pub current_comp_amount: [u64; 20],
    pub last_rebalance_time: [u64; 20],
    pub target_weight: [u64; 20],
    pub weight_sum: u64,
    pub current_weight: [u64; 20],
    pub fund_worth: u64,
    pub last_update_time: u64,
    pub refilter_interval: u64,
    pub reweight_interval: u64,
    pub rebalance_interval: u64,
    pub rebalance_threshold: u64,
    pub rebalance_slippage: u64,
    pub lp_offset_threshold: u64,
    pub last_refilter_time: u64,
    pub last_reweight_time: u64,
    pub rules_ready: u64,
    #[serde(with = "crate::big_array_serde")]
    pub asset_pool: [u64; 200],
    pub num_of_rules: u64,
    pub rules: [Rule; 20],
    pub num_rule_tokens: u64,
    pub rule_tokens: [u64; 20],
    pub rule_token_weights: [u64; 20],
    pub message_digest_five: [u8; 16],
    pub disable_rebalance: u64,
    pub disable_lp: u64,
    pub allow_multi_asset_contribution: u8,
    pub symbol_length: u8,
    pub symbol: [u8; 10],
    pub name_length: u8,
    #[serde(with = "crate::big_array_serde")]
    pub name: [u8; 60],
    pub uri_length: u8,
    #[serde(with = "crate::big_array_serde")]
    pub uri: [u8; 300],
    #[serde(with = "crate::big_array_serde")]
    pub extra_bytes: [u8; 394],
}
impl FundState {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let version: u64 = crate::borsh_de_or_default(&mut reader)?;
        let manager: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let fund_token: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let manager_fee: u64 = crate::borsh_de_or_default(&mut reader)?;
        let supply_outsanding: u64 = crate::borsh_de_or_default(&mut reader)?;
        let actively_managed: u64 = crate::borsh_de_or_default(&mut reader)?;
        let active_buy_states: u64 = crate::borsh_de_or_default(&mut reader)?;
        let sell_state: u64 = crate::borsh_de_or_default(&mut reader)?;
        let rebalance_sell_state: u64 = crate::borsh_de_or_default(&mut reader)?;
        let host_pubkey: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let host_fee: u64 = crate::borsh_de_or_default(&mut reader)?;
        let num_of_tokens: u64 = crate::borsh_de_or_default(&mut reader)?;
        let current_comp_token: [u64; 20] = crate::borsh_de_or_default(&mut reader)?;
        let current_comp_amount: [u64; 20] = crate::borsh_de_or_default(&mut reader)?;
        let last_rebalance_time: [u64; 20] = crate::borsh_de_or_default(&mut reader)?;
        let target_weight: [u64; 20] = crate::borsh_de_or_default(&mut reader)?;
        let weight_sum: u64 = crate::borsh_de_or_default(&mut reader)?;
        let current_weight: [u64; 20] = crate::borsh_de_or_default(&mut reader)?;
        let fund_worth: u64 = crate::borsh_de_or_default(&mut reader)?;
        let last_update_time: u64 = crate::borsh_de_or_default(&mut reader)?;
        let refilter_interval: u64 = crate::borsh_de_or_default(&mut reader)?;
        let reweight_interval: u64 = crate::borsh_de_or_default(&mut reader)?;
        let rebalance_interval: u64 = crate::borsh_de_or_default(&mut reader)?;
        let rebalance_threshold: u64 = crate::borsh_de_or_default(&mut reader)?;
        let rebalance_slippage: u64 = crate::borsh_de_or_default(&mut reader)?;
        let lp_offset_threshold: u64 = crate::borsh_de_or_default(&mut reader)?;
        let last_refilter_time: u64 = crate::borsh_de_or_default(&mut reader)?;
        let last_reweight_time: u64 = crate::borsh_de_or_default(&mut reader)?;
        let rules_ready: u64 = crate::borsh_de_or_default(&mut reader)?;
        let asset_pool = <[u64; 200] as borsh::BorshDeserialize>::deserialize_reader(
            &mut reader,
        )?;
        let num_of_rules: u64 = crate::borsh_de_or_default(&mut reader)?;
        let rules: [Rule; 20] = crate::borsh_de_or_default(&mut reader)?;
        let num_rule_tokens: u64 = crate::borsh_de_or_default(&mut reader)?;
        let rule_tokens: [u64; 20] = crate::borsh_de_or_default(&mut reader)?;
        let rule_token_weights: [u64; 20] = crate::borsh_de_or_default(&mut reader)?;
        let message_digest_five: [u8; 16] = crate::borsh_de_or_default(&mut reader)?;
        let disable_rebalance: u64 = crate::borsh_de_or_default(&mut reader)?;
        let disable_lp: u64 = crate::borsh_de_or_default(&mut reader)?;
        let allow_multi_asset_contribution: u8 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let symbol_length: u8 = crate::borsh_de_or_default(&mut reader)?;
        let symbol: [u8; 10] = crate::borsh_de_or_default(&mut reader)?;
        let name_length: u8 = crate::borsh_de_or_default(&mut reader)?;
        let name = <[u8; 60] as borsh::BorshDeserialize>::deserialize_reader(
            &mut reader,
        )?;
        let uri_length: u8 = crate::borsh_de_or_default(&mut reader)?;
        let uri = <[u8; 300] as borsh::BorshDeserialize>::deserialize_reader(
            &mut reader,
        )?;
        let extra_bytes = <[u8; 394] as borsh::BorshDeserialize>::deserialize_reader(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self {
            version,
            manager,
            fund_token,
            manager_fee,
            supply_outsanding,
            actively_managed,
            active_buy_states,
            sell_state,
            rebalance_sell_state,
            host_pubkey,
            host_fee,
            num_of_tokens,
            current_comp_token,
            current_comp_amount,
            last_rebalance_time,
            target_weight,
            weight_sum,
            current_weight,
            fund_worth,
            last_update_time,
            refilter_interval,
            reweight_interval,
            rebalance_interval,
            rebalance_threshold,
            rebalance_slippage,
            lp_offset_threshold,
            last_refilter_time,
            last_reweight_time,
            rules_ready,
            asset_pool,
            num_of_rules,
            rules,
            num_rule_tokens,
            rule_tokens,
            rule_token_weights,
            message_digest_five,
            disable_rebalance,
            disable_lp,
            allow_multi_asset_contribution,
            symbol_length,
            symbol,
            name_length,
            name,
            uri_length,
            uri,
            extra_bytes,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.version, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.manager, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fund_token, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.manager_fee, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.supply_outsanding, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.actively_managed, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.active_buy_states, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.sell_state, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.rebalance_sell_state, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.host_pubkey, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.host_fee, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.num_of_tokens, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.current_comp_token, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.current_comp_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.last_rebalance_time, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.target_weight, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.weight_sum, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.current_weight, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fund_worth, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.last_update_time, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.refilter_interval, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.reweight_interval, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.rebalance_interval, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.rebalance_threshold, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.rebalance_slippage, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.lp_offset_threshold, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.last_refilter_time, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.last_reweight_time, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.rules_ready, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.asset_pool, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.num_of_rules, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.rules, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.num_rule_tokens, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.rule_tokens, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.rule_token_weights, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.message_digest_five, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.disable_rebalance, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.disable_lp, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.allow_multi_asset_contribution,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.symbol_length, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.symbol, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.name_length, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.name, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.uri_length, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.uri, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.extra_bytes, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct FundStateAccount(pub FundState);
impl FundStateAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != FUND_STATE_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(FundState::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&FUND_STATE_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const BUY_STATE_ACCOUNT_DISCM: [u8; 8] = [37, 16, 214, 100, 229, 251, 20, 36];
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
pub struct BuyState {
    pub fund: Pubkey,
    pub buyer: Pubkey,
    pub fund_manager: Pubkey,
    pub host_platform: Pubkey,
    pub buyer_fund_token_account: Pubkey,
    pub usdc_contributed: u64,
    pub usdc_left: u64,
    pub token: [u64; 20],
    pub amount_to_spend: [u64; 20],
    pub amount_bought: [u64; 20],
    pub creation_timestamp: u64,
    pub contributed_value: u64,
}
impl BuyState {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let fund: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let buyer: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let fund_manager: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let host_platform: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let buyer_fund_token_account: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let usdc_contributed: u64 = crate::borsh_de_or_default(&mut reader)?;
        let usdc_left: u64 = crate::borsh_de_or_default(&mut reader)?;
        let token: [u64; 20] = crate::borsh_de_or_default(&mut reader)?;
        let amount_to_spend: [u64; 20] = crate::borsh_de_or_default(&mut reader)?;
        let amount_bought: [u64; 20] = crate::borsh_de_or_default(&mut reader)?;
        let creation_timestamp: u64 = crate::borsh_de_or_default(&mut reader)?;
        let contributed_value: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            fund,
            buyer,
            fund_manager,
            host_platform,
            buyer_fund_token_account,
            usdc_contributed,
            usdc_left,
            token,
            amount_to_spend,
            amount_bought,
            creation_timestamp,
            contributed_value,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.fund, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.buyer, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fund_manager, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.host_platform, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.buyer_fund_token_account, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.usdc_contributed, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.usdc_left, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.amount_to_spend, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.amount_bought, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.creation_timestamp, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.contributed_value, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct BuyStateAccount(pub BuyState);
impl BuyStateAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != BUY_STATE_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(BuyState::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&BUY_STATE_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const TOKEN_INFO_ACCOUNT_DISCM: [u8; 8] = [109, 162, 52, 125, 77, 166, 37, 202];
#[derive(
    Clone,
    Debug,
    BorshDeserialize,
    BorshSerialize,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
pub struct TokenInfo {
    pub num_tokens: u64,
    #[serde(with = "crate::big_array_serde")]
    pub token_mint: [Pubkey; 200],
    #[serde(with = "crate::big_array_serde")]
    pub pda_token_account: [Pubkey; 200],
    #[serde(with = "crate::big_array_serde")]
    pub coingecko_ids: [[u8; 30]; 200],
    #[serde(with = "crate::big_array_serde")]
    pub pyth: [Pubkey; 200],
    #[serde(with = "crate::big_array_serde")]
    pub decimals: [u8; 200],
}
impl TokenInfo {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let num_tokens: u64 = crate::borsh_de_or_default(&mut reader)?;
        let token_mint = <[Pubkey; 200] as borsh::BorshDeserialize>::deserialize_reader(
            &mut reader,
        )?;
        let pda_token_account = <[Pubkey; 200] as borsh::BorshDeserialize>::deserialize_reader(
            &mut reader,
        )?;
        let coingecko_ids = <[[u8; 30]; 200] as borsh::BorshDeserialize>::deserialize_reader(
            &mut reader,
        )?;
        let pyth = <[Pubkey; 200] as borsh::BorshDeserialize>::deserialize_reader(
            &mut reader,
        )?;
        let decimals = <[u8; 200] as borsh::BorshDeserialize>::deserialize_reader(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self {
            num_tokens,
            token_mint,
            pda_token_account,
            coingecko_ids,
            pyth,
            decimals,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.num_tokens, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.pda_token_account, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.coingecko_ids, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.pyth, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.decimals, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct TokenInfoAccount(pub TokenInfo);
impl TokenInfoAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != TOKEN_INFO_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(TokenInfo::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&TOKEN_INFO_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const TOKEN_LIST_ACCOUNT_DISCM: [u8; 8] = [145, 167, 153, 173, 5, 187, 157, 150];
#[derive(
    Clone,
    Debug,
    BorshDeserialize,
    BorshSerialize,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
pub struct TokenList {
    pub num_tokens: u64,
    #[serde(with = "crate::big_array_serde")]
    pub list: [TokenSettings; 200],
}
impl TokenList {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let num_tokens: u64 = crate::borsh_de_or_default(&mut reader)?;
        let list = <[TokenSettings; 200] as borsh::BorshDeserialize>::deserialize_reader(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self { num_tokens, list })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.num_tokens, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.list, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct TokenListAccount(pub TokenList);
impl TokenListAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != TOKEN_LIST_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(TokenList::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&TOKEN_LIST_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const DATABASE_ACCOUNT_DISCM: [u8; 8] = [235, 53, 109, 35, 184, 30, 81, 213];
#[derive(
    Clone,
    Debug,
    BorshDeserialize,
    BorshSerialize,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
pub struct Database {
    #[serde(with = "crate::big_array_serde")]
    pub data: [TokenData; 200],
    pub number_of_tokens: u64,
}
impl Database {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let data = <[TokenData; 200] as borsh::BorshDeserialize>::deserialize_reader(
            &mut reader,
        )?;
        let number_of_tokens: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { data, number_of_tokens })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.data, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.number_of_tokens, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct DatabaseAccount(pub Database);
impl DatabaseAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != DATABASE_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(Database::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&DATABASE_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const TOKEN_STATS_ACCOUNT_DISCM: [u8; 8] = [7, 126, 25, 232, 73, 79, 202, 236];
#[derive(
    Clone,
    Debug,
    BorshDeserialize,
    BorshSerialize,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
pub struct TokenStats {
    #[serde(with = "crate::big_array_serde")]
    pub stats: [[Stats; 6]; 200],
}
impl TokenStats {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let stats = <[[Stats; 6]; 200] as borsh::BorshDeserialize>::deserialize_reader(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self { stats })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.stats, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct TokenStatsAccount(pub TokenStats);
impl TokenStatsAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != TOKEN_STATS_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(TokenStats::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&TOKEN_STATS_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const PRISM_DATA_ACCOUNT_DISCM: [u8; 8] = [126, 79, 160, 183, 77, 115, 246, 38];
#[derive(
    Clone,
    Debug,
    BorshDeserialize,
    BorshSerialize,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
pub struct PrismData {
    #[serde(with = "crate::big_array_serde")]
    pub buy: [TokenPriceData; 200],
    #[serde(with = "crate::big_array_serde")]
    pub sell: [TokenPriceData; 200],
}
impl PrismData {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let buy = <[TokenPriceData; 200] as borsh::BorshDeserialize>::deserialize_reader(
            &mut reader,
        )?;
        let sell = <[TokenPriceData; 200] as borsh::BorshDeserialize>::deserialize_reader(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self { buy, sell })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.buy, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.sell, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct PrismDataAccount(pub PrismData);
impl PrismDataAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != PRISM_DATA_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(PrismData::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&PRISM_DATA_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
