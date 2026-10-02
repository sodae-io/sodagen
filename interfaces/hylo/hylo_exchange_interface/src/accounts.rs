use borsh::{BorshDeserialize, BorshSerialize};
use solana_pubkey::Pubkey;
#[allow(unused_imports)]
use crate::*;
pub const ADDRESS_UPDATE_PROPOSAL_ACCOUNT_DISCM: [u8; 8] = [
    12, 31, 250, 50, 46, 210, 94, 182,
];
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
pub struct AddressUpdateProposal {
    pub address_field: AddressField,
    pub new_address: Pubkey,
    pub proposal_time: i64,
    pub ttl_secs: u64,
    pub approved: bool,
}
impl AddressUpdateProposal {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let address_field: AddressField = crate::borsh_de_or_default(&mut reader)?;
        let new_address: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let proposal_time: i64 = crate::borsh_de_or_default(&mut reader)?;
        let ttl_secs: u64 = crate::borsh_de_or_default(&mut reader)?;
        let approved: bool = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            address_field,
            new_address,
            proposal_time,
            ttl_secs,
            approved,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.address_field, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.new_address, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.proposal_time, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.ttl_secs, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.approved, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct AddressUpdateProposalAccount(pub AddressUpdateProposal);
impl AddressUpdateProposalAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != ADDRESS_UPDATE_PROPOSAL_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(AddressUpdateProposal::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&ADDRESS_UPDATE_PROPOSAL_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const EXO_PAIR_ACCOUNT_DISCM: [u8; 8] = [251, 244, 72, 181, 40, 119, 232, 48];
#[derive(
    Clone,
    Debug,
    BorshDeserialize,
    BorshSerialize,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
pub struct ExoPair {
    pub collateral_mint: Pubkey,
    pub levercoin_mint_bump: u8,
    pub levercoin_auth_bump: u8,
    pub vault_auth_bump: u8,
    pub fee_auth_bump: u8,
    pub oracle: Pubkey,
    pub oracle_feed_id: [u8; 32],
    pub oracle_interval_secs: u64,
    pub oracle_conf_tolerance: UFixValue64,
    pub stablecoin_mint_threshold: UFixValue64,
    pub virtual_stablecoin: VirtualStablecoin,
    pub borrow_rate_curve_config: BorrowRateCurveConfig,
    pub borrow_rate_harvest_cache: HarvestCache,
    pub levercoin_fees: LevercoinFees,
    pub sell_curve_config: RebalanceCurveConfig,
    pub buy_curve_config: RebalanceCurveConfig,
    pub borrow_rate_fee: UFixValue64,
    pub paused: bool,
    pub levercoin_market_cap_limit: UFixValue64,
    pub pool_drawdown: PoolDrawdown,
    pub virtual_stablecoin_supply_floor: UFixValue64,
    #[serde(with = "crate::big_array_serde")]
    pub reserved: [u8; 91],
}
impl ExoPair {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let collateral_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let levercoin_mint_bump: u8 = crate::borsh_de_or_default(&mut reader)?;
        let levercoin_auth_bump: u8 = crate::borsh_de_or_default(&mut reader)?;
        let vault_auth_bump: u8 = crate::borsh_de_or_default(&mut reader)?;
        let fee_auth_bump: u8 = crate::borsh_de_or_default(&mut reader)?;
        let oracle: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let oracle_feed_id: [u8; 32] = crate::borsh_de_or_default(&mut reader)?;
        let oracle_interval_secs: u64 = crate::borsh_de_or_default(&mut reader)?;
        let oracle_conf_tolerance = if reader.is_empty() {
            Default::default()
        } else {
            <UFixValue64>::deserialize(&mut reader)?
        };
        let stablecoin_mint_threshold = if reader.is_empty() {
            Default::default()
        } else {
            <UFixValue64>::deserialize(&mut reader)?
        };
        let virtual_stablecoin = if reader.is_empty() {
            Default::default()
        } else {
            <VirtualStablecoin>::deserialize(&mut reader)?
        };
        let borrow_rate_curve_config = if reader.is_empty() {
            Default::default()
        } else {
            <BorrowRateCurveConfig>::deserialize(&mut reader)?
        };
        let borrow_rate_harvest_cache = if reader.is_empty() {
            Default::default()
        } else {
            <HarvestCache>::deserialize(&mut reader)?
        };
        let levercoin_fees = if reader.is_empty() {
            Default::default()
        } else {
            <LevercoinFees>::deserialize(&mut reader)?
        };
        let sell_curve_config = if reader.is_empty() {
            Default::default()
        } else {
            <RebalanceCurveConfig>::deserialize(&mut reader)?
        };
        let buy_curve_config = if reader.is_empty() {
            Default::default()
        } else {
            <RebalanceCurveConfig>::deserialize(&mut reader)?
        };
        let borrow_rate_fee = if reader.is_empty() {
            Default::default()
        } else {
            <UFixValue64>::deserialize(&mut reader)?
        };
        let paused: bool = crate::borsh_de_or_default(&mut reader)?;
        let levercoin_market_cap_limit = if reader.is_empty() {
            Default::default()
        } else {
            <UFixValue64>::deserialize(&mut reader)?
        };
        let pool_drawdown = if reader.is_empty() {
            Default::default()
        } else {
            <PoolDrawdown>::deserialize(&mut reader)?
        };
        let virtual_stablecoin_supply_floor = if reader.is_empty() {
            Default::default()
        } else {
            <UFixValue64>::deserialize(&mut reader)?
        };
        let reserved = <[u8; 91] as borsh::BorshDeserialize>::deserialize_reader(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self {
            collateral_mint,
            levercoin_mint_bump,
            levercoin_auth_bump,
            vault_auth_bump,
            fee_auth_bump,
            oracle,
            oracle_feed_id,
            oracle_interval_secs,
            oracle_conf_tolerance,
            stablecoin_mint_threshold,
            virtual_stablecoin,
            borrow_rate_curve_config,
            borrow_rate_harvest_cache,
            levercoin_fees,
            sell_curve_config,
            buy_curve_config,
            borrow_rate_fee,
            paused,
            levercoin_market_cap_limit,
            pool_drawdown,
            virtual_stablecoin_supply_floor,
            reserved,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.collateral_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.levercoin_mint_bump, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.levercoin_auth_bump, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.vault_auth_bump, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fee_auth_bump, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.oracle, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.oracle_feed_id, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.oracle_interval_secs, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.oracle_conf_tolerance, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.stablecoin_mint_threshold, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.virtual_stablecoin, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.borrow_rate_curve_config, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.borrow_rate_harvest_cache, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.levercoin_fees, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.sell_curve_config, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.buy_curve_config, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.borrow_rate_fee, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.paused, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.levercoin_market_cap_limit, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.pool_drawdown, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.virtual_stablecoin_supply_floor,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.reserved, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct ExoPairAccount(pub ExoPair);
impl ExoPairAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != EXO_PAIR_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(ExoPair::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&EXO_PAIR_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const HYLO_ACCOUNT_DISCM: [u8; 8] = [114, 161, 169, 210, 204, 175, 149, 174];
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
pub struct Hylo {
    pub admin: Pubkey,
    pub treasury: Pubkey,
    pub lst_registry: Pubkey,
    pub stablecoin_mint: Pubkey,
    pub levercoin_mint: Pubkey,
    pub pause_authority: Pubkey,
    pub stablecoin_mint_bump: u8,
    pub stablecoin_auth_bump: u8,
    pub levercoin_mint_bump: u8,
    pub levercoin_auth_bump: u8,
    pub registry_auth_bump: u8,
    pub total_sol_cache_bump: u8,
    pub oracle_interval_secs: u64,
    pub stablecoin_fees: StablecoinFees,
    pub levercoin_fees: LevercoinFees,
    pub total_sol_cache: TotalSolCache,
    pub yield_harvest_cache: HarvestCache,
    pub yield_harvest_config: YieldHarvestConfig,
    pub stablecoin_mint_threshold: UFixValue64,
    pub unused_1: UFixValue64,
    pub oracle_conf_tolerance: UFixValue64,
    pub sol_usd_oracle: Pubkey,
    pub lst_swap_fee: UFixValue64,
    pub virtual_stablecoin: VirtualStablecoin,
    pub lst_buy_curve_config: RebalanceCurveConfig,
    pub lst_sell_curve_config: RebalanceCurveConfig,
    pub protocol_paused: bool,
    pub lst_pair_paused: bool,
    pub unused_2: UFixValue64,
    pub pool_drawdown: PoolDrawdown,
    pub reserved: [u8; 13],
}
impl Hylo {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let admin: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let treasury: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let lst_registry: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let stablecoin_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let levercoin_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let pause_authority: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let stablecoin_mint_bump: u8 = crate::borsh_de_or_default(&mut reader)?;
        let stablecoin_auth_bump: u8 = crate::borsh_de_or_default(&mut reader)?;
        let levercoin_mint_bump: u8 = crate::borsh_de_or_default(&mut reader)?;
        let levercoin_auth_bump: u8 = crate::borsh_de_or_default(&mut reader)?;
        let registry_auth_bump: u8 = crate::borsh_de_or_default(&mut reader)?;
        let total_sol_cache_bump: u8 = crate::borsh_de_or_default(&mut reader)?;
        let oracle_interval_secs: u64 = crate::borsh_de_or_default(&mut reader)?;
        let stablecoin_fees = if reader.is_empty() {
            Default::default()
        } else {
            <StablecoinFees>::deserialize(&mut reader)?
        };
        let levercoin_fees = if reader.is_empty() {
            Default::default()
        } else {
            <LevercoinFees>::deserialize(&mut reader)?
        };
        let total_sol_cache = if reader.is_empty() {
            Default::default()
        } else {
            <TotalSolCache>::deserialize(&mut reader)?
        };
        let yield_harvest_cache = if reader.is_empty() {
            Default::default()
        } else {
            <HarvestCache>::deserialize(&mut reader)?
        };
        let yield_harvest_config = if reader.is_empty() {
            Default::default()
        } else {
            <YieldHarvestConfig>::deserialize(&mut reader)?
        };
        let stablecoin_mint_threshold = if reader.is_empty() {
            Default::default()
        } else {
            <UFixValue64>::deserialize(&mut reader)?
        };
        let unused_1 = if reader.is_empty() {
            Default::default()
        } else {
            <UFixValue64>::deserialize(&mut reader)?
        };
        let oracle_conf_tolerance = if reader.is_empty() {
            Default::default()
        } else {
            <UFixValue64>::deserialize(&mut reader)?
        };
        let sol_usd_oracle: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let lst_swap_fee = if reader.is_empty() {
            Default::default()
        } else {
            <UFixValue64>::deserialize(&mut reader)?
        };
        let virtual_stablecoin = if reader.is_empty() {
            Default::default()
        } else {
            <VirtualStablecoin>::deserialize(&mut reader)?
        };
        let lst_buy_curve_config = if reader.is_empty() {
            Default::default()
        } else {
            <RebalanceCurveConfig>::deserialize(&mut reader)?
        };
        let lst_sell_curve_config = if reader.is_empty() {
            Default::default()
        } else {
            <RebalanceCurveConfig>::deserialize(&mut reader)?
        };
        let protocol_paused: bool = crate::borsh_de_or_default(&mut reader)?;
        let lst_pair_paused: bool = crate::borsh_de_or_default(&mut reader)?;
        let unused_2 = if reader.is_empty() {
            Default::default()
        } else {
            <UFixValue64>::deserialize(&mut reader)?
        };
        let pool_drawdown = if reader.is_empty() {
            Default::default()
        } else {
            <PoolDrawdown>::deserialize(&mut reader)?
        };
        let reserved: [u8; 13] = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            admin,
            treasury,
            lst_registry,
            stablecoin_mint,
            levercoin_mint,
            pause_authority,
            stablecoin_mint_bump,
            stablecoin_auth_bump,
            levercoin_mint_bump,
            levercoin_auth_bump,
            registry_auth_bump,
            total_sol_cache_bump,
            oracle_interval_secs,
            stablecoin_fees,
            levercoin_fees,
            total_sol_cache,
            yield_harvest_cache,
            yield_harvest_config,
            stablecoin_mint_threshold,
            unused_1,
            oracle_conf_tolerance,
            sol_usd_oracle,
            lst_swap_fee,
            virtual_stablecoin,
            lst_buy_curve_config,
            lst_sell_curve_config,
            protocol_paused,
            lst_pair_paused,
            unused_2,
            pool_drawdown,
            reserved,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.admin, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.treasury, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.lst_registry, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.stablecoin_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.levercoin_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.pause_authority, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.stablecoin_mint_bump, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.stablecoin_auth_bump, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.levercoin_mint_bump, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.levercoin_auth_bump, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.registry_auth_bump, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.total_sol_cache_bump, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.oracle_interval_secs, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.stablecoin_fees, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.levercoin_fees, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.total_sol_cache, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.yield_harvest_cache, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.yield_harvest_config, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.stablecoin_mint_threshold, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.unused_1, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.oracle_conf_tolerance, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.sol_usd_oracle, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.lst_swap_fee, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.virtual_stablecoin, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.lst_buy_curve_config, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.lst_sell_curve_config, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.protocol_paused, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.lst_pair_paused, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.unused_2, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.pool_drawdown, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.reserved, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct HyloAccount(pub Hylo);
impl HyloAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != HYLO_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(Hylo::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&HYLO_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const LST_HEADER_ACCOUNT_DISCM: [u8; 8] = [125, 135, 217, 151, 122, 202, 138, 59];
#[derive(
    Clone,
    Debug,
    BorshDeserialize,
    BorshSerialize,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
pub struct LstHeader {
    pub mint: Pubkey,
    pub vault: Pubkey,
    pub pool_state: Pubkey,
    pub stake_program: LstStakePoolProgram,
    pub prev_price_sol: LstSolPrice,
    pub price_sol: LstSolPrice,
    pub last_yield_harvest_epoch: u64,
    pub rebalance_fee: UFixValue64,
    #[serde(with = "crate::big_array_serde")]
    pub reserved: [u8; 55],
}
impl LstHeader {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let vault: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let pool_state: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let stake_program: LstStakePoolProgram = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let prev_price_sol = if reader.is_empty() {
            Default::default()
        } else {
            <LstSolPrice>::deserialize(&mut reader)?
        };
        let price_sol = if reader.is_empty() {
            Default::default()
        } else {
            <LstSolPrice>::deserialize(&mut reader)?
        };
        let last_yield_harvest_epoch: u64 = crate::borsh_de_or_default(&mut reader)?;
        let rebalance_fee = if reader.is_empty() {
            Default::default()
        } else {
            <UFixValue64>::deserialize(&mut reader)?
        };
        let reserved = <[u8; 55] as borsh::BorshDeserialize>::deserialize_reader(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self {
            mint,
            vault,
            pool_state,
            stake_program,
            prev_price_sol,
            price_sol,
            last_yield_harvest_epoch,
            rebalance_fee,
            reserved,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.vault, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.pool_state, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.stake_program, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.prev_price_sol, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.price_sol, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.last_yield_harvest_epoch, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.rebalance_fee, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.reserved, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct LstHeaderAccount(pub LstHeader);
impl LstHeaderAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != LST_HEADER_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(LstHeader::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&LST_HEADER_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const PRICE_UPDATE_V2_ACCOUNT_DISCM: [u8; 8] = [34, 241, 35, 99, 157, 126, 244, 205];
#[derive(
    Clone,
    Debug,
    BorshDeserialize,
    BorshSerialize,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
pub struct PriceUpdateV2 {
    pub write_authority: Pubkey,
    pub verification_level: VerificationLevel,
    pub price_message: PriceFeedMessage,
    pub posted_slot: u64,
}
impl PriceUpdateV2 {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let write_authority: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let verification_level = <VerificationLevel as borsh::BorshDeserialize>::deserialize_reader(
            &mut reader,
        )?;
        let price_message = if reader.is_empty() {
            Default::default()
        } else {
            <PriceFeedMessage>::deserialize(&mut reader)?
        };
        let posted_slot: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            write_authority,
            verification_level,
            price_message,
            posted_slot,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.write_authority, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.verification_level, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.price_message, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.posted_slot, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct PriceUpdateV2Account(pub PriceUpdateV2);
impl PriceUpdateV2Account {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != PRICE_UPDATE_V2_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(PriceUpdateV2::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&PRICE_UPDATE_V2_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const USDC_PAIR_ACCOUNT_DISCM: [u8; 8] = [130, 97, 194, 78, 22, 254, 137, 107];
#[derive(
    Clone,
    Debug,
    BorshDeserialize,
    BorshSerialize,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
pub struct UsdcPair {
    pub vault_auth_bump: u8,
    pub fee_auth_bump: u8,
    pub mint_fee: UFixValue64,
    pub oracle_interval_secs: u64,
    pub oracle_conf_tolerance: UFixValue64,
    pub virtual_stablecoin: VirtualStablecoin,
    pub paused: bool,
    pub par_tolerance: ParTolerance,
    pub redeem_fee: UFixValue64,
    #[serde(with = "crate::big_array_serde")]
    pub reserved: [u8; 109],
}
impl UsdcPair {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let vault_auth_bump: u8 = crate::borsh_de_or_default(&mut reader)?;
        let fee_auth_bump: u8 = crate::borsh_de_or_default(&mut reader)?;
        let mint_fee = if reader.is_empty() {
            Default::default()
        } else {
            <UFixValue64>::deserialize(&mut reader)?
        };
        let oracle_interval_secs: u64 = crate::borsh_de_or_default(&mut reader)?;
        let oracle_conf_tolerance = if reader.is_empty() {
            Default::default()
        } else {
            <UFixValue64>::deserialize(&mut reader)?
        };
        let virtual_stablecoin = if reader.is_empty() {
            Default::default()
        } else {
            <VirtualStablecoin>::deserialize(&mut reader)?
        };
        let paused: bool = crate::borsh_de_or_default(&mut reader)?;
        let par_tolerance = if reader.is_empty() {
            Default::default()
        } else {
            <ParTolerance>::deserialize(&mut reader)?
        };
        let redeem_fee = if reader.is_empty() {
            Default::default()
        } else {
            <UFixValue64>::deserialize(&mut reader)?
        };
        let reserved = <[u8; 109] as borsh::BorshDeserialize>::deserialize_reader(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self {
            vault_auth_bump,
            fee_auth_bump,
            mint_fee,
            oracle_interval_secs,
            oracle_conf_tolerance,
            virtual_stablecoin,
            paused,
            par_tolerance,
            redeem_fee,
            reserved,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.vault_auth_bump, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fee_auth_bump, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.mint_fee, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.oracle_interval_secs, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.oracle_conf_tolerance, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.virtual_stablecoin, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.paused, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.par_tolerance, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.redeem_fee, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.reserved, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct UsdcPairAccount(pub UsdcPair);
impl UsdcPairAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != USDC_PAIR_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(UsdcPair::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&USDC_PAIR_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
