use borsh::{BorshDeserialize, BorshSerialize};
use solana_pubkey::Pubkey;
#[allow(unused_imports)]
use crate::*;
pub const FARM_ACCOUNT_DISCM: [u8; 8] = [161, 156, 211, 253, 250, 64, 53, 250];
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
pub struct Farm {
    pub pool: Pubkey,
    pub tokens: [Pubkey; 3],
    pub token_accounts: [Pubkey; 3],
    pub supply: [Token; 3],
    pub supply_left: [Token; 3],
    pub accumulated_seconds_per_share: FixedPoint,
    pub offset_seconds_per_share: FixedPoint,
    pub start_time: u64,
    pub end_time: u64,
    pub last_update: u64,
    pub bump: u8,
    pub farm_type: FarmType,
}
impl Farm {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let pool: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let tokens: [Pubkey; 3] = crate::borsh_de_or_default(&mut reader)?;
        let token_accounts: [Pubkey; 3] = crate::borsh_de_or_default(&mut reader)?;
        let supply: [Token; 3] = crate::borsh_de_or_default(&mut reader)?;
        let supply_left: [Token; 3] = crate::borsh_de_or_default(&mut reader)?;
        let accumulated_seconds_per_share = if reader.is_empty() {
            Default::default()
        } else {
            <FixedPoint>::deserialize(&mut reader)?
        };
        let offset_seconds_per_share = if reader.is_empty() {
            Default::default()
        } else {
            <FixedPoint>::deserialize(&mut reader)?
        };
        let start_time: u64 = crate::borsh_de_or_default(&mut reader)?;
        let end_time: u64 = crate::borsh_de_or_default(&mut reader)?;
        let last_update: u64 = crate::borsh_de_or_default(&mut reader)?;
        let bump: u8 = crate::borsh_de_or_default(&mut reader)?;
        let farm_type: FarmType = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            pool,
            tokens,
            token_accounts,
            supply,
            supply_left,
            accumulated_seconds_per_share,
            offset_seconds_per_share,
            start_time,
            end_time,
            last_update,
            bump,
            farm_type,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.pool, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.tokens, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_accounts, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.supply, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.supply_left, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.accumulated_seconds_per_share,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.offset_seconds_per_share, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.start_time, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.end_time, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.last_update, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.bump, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.farm_type, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct FarmAccount(pub Farm);
impl FarmAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != FARM_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(Farm::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&FARM_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const POOL_V2_ACCOUNT_DISCM: [u8; 8] = [91, 12, 214, 87, 7, 185, 167, 55];
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
pub struct PoolV2 {
    pub token_x: Pubkey,
    pub token_y: Pubkey,
    pub pool_x_account: Pubkey,
    pub pool_y_account: Pubkey,
    pub admin: Pubkey,
    pub project_owner: Pubkey,
    pub token_x_reserve: Token,
    pub token_y_reserve: Token,
    pub self_shares: Token,
    pub all_shares: Token,
    pub buyback_amount_x: Token,
    pub buyback_amount_y: Token,
    pub project_amount_x: Token,
    pub project_amount_y: Token,
    pub mercanti_amount_x: Token,
    pub mercanti_amount_y: Token,
    pub lp_accumulator_x: FixedPoint,
    pub lp_accumulator_y: FixedPoint,
    pub const_k: Product,
    pub price: FixedPoint,
    pub lp_fee: FixedPoint,
    pub buyback_fee: FixedPoint,
    pub project_fee: FixedPoint,
    pub mercanti_fee: FixedPoint,
    pub farm_count: u64,
    pub pool_bump: u8,
    pub lp_token: Pubkey,
    pub lp_token_mint_bump: u8,
    pub padding: [u64; 8],
}
impl PoolV2 {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let token_x: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let token_y: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let pool_x_account: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let pool_y_account: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let admin: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let project_owner: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let token_x_reserve = if reader.is_empty() {
            Default::default()
        } else {
            <Token>::deserialize(&mut reader)?
        };
        let token_y_reserve = if reader.is_empty() {
            Default::default()
        } else {
            <Token>::deserialize(&mut reader)?
        };
        let self_shares = if reader.is_empty() {
            Default::default()
        } else {
            <Token>::deserialize(&mut reader)?
        };
        let all_shares = if reader.is_empty() {
            Default::default()
        } else {
            <Token>::deserialize(&mut reader)?
        };
        let buyback_amount_x = if reader.is_empty() {
            Default::default()
        } else {
            <Token>::deserialize(&mut reader)?
        };
        let buyback_amount_y = if reader.is_empty() {
            Default::default()
        } else {
            <Token>::deserialize(&mut reader)?
        };
        let project_amount_x = if reader.is_empty() {
            Default::default()
        } else {
            <Token>::deserialize(&mut reader)?
        };
        let project_amount_y = if reader.is_empty() {
            Default::default()
        } else {
            <Token>::deserialize(&mut reader)?
        };
        let mercanti_amount_x = if reader.is_empty() {
            Default::default()
        } else {
            <Token>::deserialize(&mut reader)?
        };
        let mercanti_amount_y = if reader.is_empty() {
            Default::default()
        } else {
            <Token>::deserialize(&mut reader)?
        };
        let lp_accumulator_x = if reader.is_empty() {
            Default::default()
        } else {
            <FixedPoint>::deserialize(&mut reader)?
        };
        let lp_accumulator_y = if reader.is_empty() {
            Default::default()
        } else {
            <FixedPoint>::deserialize(&mut reader)?
        };
        let const_k = if reader.is_empty() {
            Default::default()
        } else {
            <Product>::deserialize(&mut reader)?
        };
        let price = if reader.is_empty() {
            Default::default()
        } else {
            <FixedPoint>::deserialize(&mut reader)?
        };
        let lp_fee = if reader.is_empty() {
            Default::default()
        } else {
            <FixedPoint>::deserialize(&mut reader)?
        };
        let buyback_fee = if reader.is_empty() {
            Default::default()
        } else {
            <FixedPoint>::deserialize(&mut reader)?
        };
        let project_fee = if reader.is_empty() {
            Default::default()
        } else {
            <FixedPoint>::deserialize(&mut reader)?
        };
        let mercanti_fee = if reader.is_empty() {
            Default::default()
        } else {
            <FixedPoint>::deserialize(&mut reader)?
        };
        let farm_count: u64 = crate::borsh_de_or_default(&mut reader)?;
        let pool_bump: u8 = crate::borsh_de_or_default(&mut reader)?;
        let lp_token: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let lp_token_mint_bump: u8 = crate::borsh_de_or_default(&mut reader)?;
        let padding: [u64; 8] = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            token_x,
            token_y,
            pool_x_account,
            pool_y_account,
            admin,
            project_owner,
            token_x_reserve,
            token_y_reserve,
            self_shares,
            all_shares,
            buyback_amount_x,
            buyback_amount_y,
            project_amount_x,
            project_amount_y,
            mercanti_amount_x,
            mercanti_amount_y,
            lp_accumulator_x,
            lp_accumulator_y,
            const_k,
            price,
            lp_fee,
            buyback_fee,
            project_fee,
            mercanti_fee,
            farm_count,
            pool_bump,
            lp_token,
            lp_token_mint_bump,
            padding,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.token_x, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_y, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.pool_x_account, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.pool_y_account, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.admin, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.project_owner, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_x_reserve, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_y_reserve, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.self_shares, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.all_shares, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.buyback_amount_x, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.buyback_amount_y, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.project_amount_x, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.project_amount_y, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.mercanti_amount_x, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.mercanti_amount_y, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.lp_accumulator_x, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.lp_accumulator_y, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.const_k, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.price, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.lp_fee, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.buyback_fee, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.project_fee, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.mercanti_fee, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.farm_count, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.pool_bump, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.lp_token, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.lp_token_mint_bump, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.padding, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct PoolV2Account(pub PoolV2);
impl PoolV2Account {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != POOL_V2_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(PoolV2::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&POOL_V2_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const POOL_ACCOUNT_DISCM: [u8; 8] = [241, 154, 109, 4, 17, 177, 109, 188];
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
pub struct Pool {
    pub token_x: Pubkey,
    pub token_y: Pubkey,
    pub pool_x_account: Pubkey,
    pub pool_y_account: Pubkey,
    pub admin: Pubkey,
    pub project_owner: Pubkey,
    pub token_x_reserve: Token,
    pub token_y_reserve: Token,
    pub self_shares: Token,
    pub all_shares: Token,
    pub buyback_amount_x: Token,
    pub buyback_amount_y: Token,
    pub project_amount_x: Token,
    pub project_amount_y: Token,
    pub mercanti_amount_x: Token,
    pub mercanti_amount_y: Token,
    pub lp_accumulator_x: FixedPoint,
    pub lp_accumulator_y: FixedPoint,
    pub const_k: Product,
    pub price: FixedPoint,
    pub lp_fee: FixedPoint,
    pub buyback_fee: FixedPoint,
    pub project_fee: FixedPoint,
    pub mercanti_fee: FixedPoint,
    pub farm_count: u64,
    pub bump: u8,
}
impl Pool {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let token_x: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let token_y: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let pool_x_account: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let pool_y_account: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let admin: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let project_owner: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let token_x_reserve = if reader.is_empty() {
            Default::default()
        } else {
            <Token>::deserialize(&mut reader)?
        };
        let token_y_reserve = if reader.is_empty() {
            Default::default()
        } else {
            <Token>::deserialize(&mut reader)?
        };
        let self_shares = if reader.is_empty() {
            Default::default()
        } else {
            <Token>::deserialize(&mut reader)?
        };
        let all_shares = if reader.is_empty() {
            Default::default()
        } else {
            <Token>::deserialize(&mut reader)?
        };
        let buyback_amount_x = if reader.is_empty() {
            Default::default()
        } else {
            <Token>::deserialize(&mut reader)?
        };
        let buyback_amount_y = if reader.is_empty() {
            Default::default()
        } else {
            <Token>::deserialize(&mut reader)?
        };
        let project_amount_x = if reader.is_empty() {
            Default::default()
        } else {
            <Token>::deserialize(&mut reader)?
        };
        let project_amount_y = if reader.is_empty() {
            Default::default()
        } else {
            <Token>::deserialize(&mut reader)?
        };
        let mercanti_amount_x = if reader.is_empty() {
            Default::default()
        } else {
            <Token>::deserialize(&mut reader)?
        };
        let mercanti_amount_y = if reader.is_empty() {
            Default::default()
        } else {
            <Token>::deserialize(&mut reader)?
        };
        let lp_accumulator_x = if reader.is_empty() {
            Default::default()
        } else {
            <FixedPoint>::deserialize(&mut reader)?
        };
        let lp_accumulator_y = if reader.is_empty() {
            Default::default()
        } else {
            <FixedPoint>::deserialize(&mut reader)?
        };
        let const_k = if reader.is_empty() {
            Default::default()
        } else {
            <Product>::deserialize(&mut reader)?
        };
        let price = if reader.is_empty() {
            Default::default()
        } else {
            <FixedPoint>::deserialize(&mut reader)?
        };
        let lp_fee = if reader.is_empty() {
            Default::default()
        } else {
            <FixedPoint>::deserialize(&mut reader)?
        };
        let buyback_fee = if reader.is_empty() {
            Default::default()
        } else {
            <FixedPoint>::deserialize(&mut reader)?
        };
        let project_fee = if reader.is_empty() {
            Default::default()
        } else {
            <FixedPoint>::deserialize(&mut reader)?
        };
        let mercanti_fee = if reader.is_empty() {
            Default::default()
        } else {
            <FixedPoint>::deserialize(&mut reader)?
        };
        let farm_count: u64 = crate::borsh_de_or_default(&mut reader)?;
        let bump: u8 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            token_x,
            token_y,
            pool_x_account,
            pool_y_account,
            admin,
            project_owner,
            token_x_reserve,
            token_y_reserve,
            self_shares,
            all_shares,
            buyback_amount_x,
            buyback_amount_y,
            project_amount_x,
            project_amount_y,
            mercanti_amount_x,
            mercanti_amount_y,
            lp_accumulator_x,
            lp_accumulator_y,
            const_k,
            price,
            lp_fee,
            buyback_fee,
            project_fee,
            mercanti_fee,
            farm_count,
            bump,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.token_x, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_y, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.pool_x_account, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.pool_y_account, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.admin, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.project_owner, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_x_reserve, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_y_reserve, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.self_shares, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.all_shares, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.buyback_amount_x, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.buyback_amount_y, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.project_amount_x, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.project_amount_y, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.mercanti_amount_x, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.mercanti_amount_y, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.lp_accumulator_x, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.lp_accumulator_y, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.const_k, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.price, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.lp_fee, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.buyback_fee, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.project_fee, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.mercanti_fee, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.farm_count, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.bump, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct PoolAccount(pub Pool);
impl PoolAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != POOL_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(Pool::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&POOL_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const PROVIDER_ACCOUNT_DISCM: [u8; 8] = [164, 180, 71, 17, 75, 216, 80, 195];
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
pub struct Provider {
    pub token_x: Pubkey,
    pub token_y: Pubkey,
    pub owner: Pubkey,
    pub shares: Token,
    pub last_fee_accumulator_x: FixedPoint,
    pub last_fee_accumulator_y: FixedPoint,
    pub last_seconds_per_share: FixedPoint,
    pub last_withdraw_time: u64,
    pub tokens_owed_x: Token,
    pub tokens_owed_y: Token,
    pub current_farm_count: u64,
    pub bump: u8,
}
impl Provider {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let token_x: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let token_y: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let owner: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let shares = if reader.is_empty() {
            Default::default()
        } else {
            <Token>::deserialize(&mut reader)?
        };
        let last_fee_accumulator_x = if reader.is_empty() {
            Default::default()
        } else {
            <FixedPoint>::deserialize(&mut reader)?
        };
        let last_fee_accumulator_y = if reader.is_empty() {
            Default::default()
        } else {
            <FixedPoint>::deserialize(&mut reader)?
        };
        let last_seconds_per_share = if reader.is_empty() {
            Default::default()
        } else {
            <FixedPoint>::deserialize(&mut reader)?
        };
        let last_withdraw_time: u64 = crate::borsh_de_or_default(&mut reader)?;
        let tokens_owed_x = if reader.is_empty() {
            Default::default()
        } else {
            <Token>::deserialize(&mut reader)?
        };
        let tokens_owed_y = if reader.is_empty() {
            Default::default()
        } else {
            <Token>::deserialize(&mut reader)?
        };
        let current_farm_count: u64 = crate::borsh_de_or_default(&mut reader)?;
        let bump: u8 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            token_x,
            token_y,
            owner,
            shares,
            last_fee_accumulator_x,
            last_fee_accumulator_y,
            last_seconds_per_share,
            last_withdraw_time,
            tokens_owed_x,
            tokens_owed_y,
            current_farm_count,
            bump,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.token_x, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_y, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.owner, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.shares, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.last_fee_accumulator_x, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.last_fee_accumulator_y, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.last_seconds_per_share, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.last_withdraw_time, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.tokens_owed_x, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.tokens_owed_y, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.current_farm_count, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.bump, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct ProviderAccount(pub Provider);
impl ProviderAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != PROVIDER_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(Provider::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&PROVIDER_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const STATE_ACCOUNT_DISCM: [u8; 8] = [216, 146, 107, 94, 104, 75, 182, 177];
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
pub struct State {
    pub admin: Pubkey,
    pub program_authority: Pubkey,
    pub bump: u8,
    pub nonce: u8,
}
impl State {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let admin: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let program_authority: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let bump: u8 = crate::borsh_de_or_default(&mut reader)?;
        let nonce: u8 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            admin,
            program_authority,
            bump,
            nonce,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.admin, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.program_authority, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.bump, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.nonce, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct StateAccount(pub State);
impl StateAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != STATE_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(State::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&STATE_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
