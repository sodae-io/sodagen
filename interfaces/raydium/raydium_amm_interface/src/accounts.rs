use borsh::{BorshDeserialize, BorshSerialize};
use solana_pubkey::Pubkey;
#[allow(unused_imports)]
use crate::*;
pub const TARGET_ORDERS_ACCOUNT_DISCM: [u8; 8] = [113, 225, 140, 255, 65, 144, 239, 231];
#[derive(
    Clone,
    Debug,
    BorshDeserialize,
    BorshSerialize,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
pub struct TargetOrders {
    pub owner: [u64; 4],
    #[serde(with = "crate::big_array_serde")]
    pub buy_orders: [TargetOrder; 50],
    pub padding1: [u64; 8],
    pub target_x: u128,
    pub target_y: u128,
    pub plan_x_buy: u128,
    pub plan_y_buy: u128,
    pub plan_x_sell: u128,
    pub plan_y_sell: u128,
    pub placed_x: u128,
    pub placed_y: u128,
    pub calc_pnl_x: u128,
    pub calc_pnl_y: u128,
    #[serde(with = "crate::big_array_serde")]
    pub sell_orders: [TargetOrder; 50],
    pub padding2: [u64; 6],
    pub replace_buy_client_id: [u64; 10],
    pub replace_sell_client_id: [u64; 10],
    pub last_order_numerator: u64,
    pub last_order_denominator: u64,
    pub plan_orders_cur: u64,
    pub place_orders_cur: u64,
    pub valid_buy_order_num: u64,
    pub valid_sell_order_num: u64,
    pub padding3: [u64; 10],
    pub free_slot_bits: u128,
}
impl TargetOrders {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let owner: [u64; 4] = crate::borsh_de_or_default(&mut reader)?;
        let buy_orders = <[TargetOrder; 50] as borsh::BorshDeserialize>::deserialize_reader(
            &mut reader,
        )?;
        let padding1: [u64; 8] = crate::borsh_de_or_default(&mut reader)?;
        let target_x: u128 = crate::borsh_de_or_default(&mut reader)?;
        let target_y: u128 = crate::borsh_de_or_default(&mut reader)?;
        let plan_x_buy: u128 = crate::borsh_de_or_default(&mut reader)?;
        let plan_y_buy: u128 = crate::borsh_de_or_default(&mut reader)?;
        let plan_x_sell: u128 = crate::borsh_de_or_default(&mut reader)?;
        let plan_y_sell: u128 = crate::borsh_de_or_default(&mut reader)?;
        let placed_x: u128 = crate::borsh_de_or_default(&mut reader)?;
        let placed_y: u128 = crate::borsh_de_or_default(&mut reader)?;
        let calc_pnl_x: u128 = crate::borsh_de_or_default(&mut reader)?;
        let calc_pnl_y: u128 = crate::borsh_de_or_default(&mut reader)?;
        let sell_orders = <[TargetOrder; 50] as borsh::BorshDeserialize>::deserialize_reader(
            &mut reader,
        )?;
        let padding2: [u64; 6] = crate::borsh_de_or_default(&mut reader)?;
        let replace_buy_client_id: [u64; 10] = crate::borsh_de_or_default(&mut reader)?;
        let replace_sell_client_id: [u64; 10] = crate::borsh_de_or_default(&mut reader)?;
        let last_order_numerator: u64 = crate::borsh_de_or_default(&mut reader)?;
        let last_order_denominator: u64 = crate::borsh_de_or_default(&mut reader)?;
        let plan_orders_cur: u64 = crate::borsh_de_or_default(&mut reader)?;
        let place_orders_cur: u64 = crate::borsh_de_or_default(&mut reader)?;
        let valid_buy_order_num: u64 = crate::borsh_de_or_default(&mut reader)?;
        let valid_sell_order_num: u64 = crate::borsh_de_or_default(&mut reader)?;
        let padding3: [u64; 10] = crate::borsh_de_or_default(&mut reader)?;
        let free_slot_bits: u128 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            owner,
            buy_orders,
            padding1,
            target_x,
            target_y,
            plan_x_buy,
            plan_y_buy,
            plan_x_sell,
            plan_y_sell,
            placed_x,
            placed_y,
            calc_pnl_x,
            calc_pnl_y,
            sell_orders,
            padding2,
            replace_buy_client_id,
            replace_sell_client_id,
            last_order_numerator,
            last_order_denominator,
            plan_orders_cur,
            place_orders_cur,
            valid_buy_order_num,
            valid_sell_order_num,
            padding3,
            free_slot_bits,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.owner, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.buy_orders, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.padding1, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.target_x, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.target_y, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.plan_x_buy, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.plan_y_buy, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.plan_x_sell, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.plan_y_sell, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.placed_x, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.placed_y, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.calc_pnl_x, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.calc_pnl_y, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.sell_orders, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.padding2, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.replace_buy_client_id, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.replace_sell_client_id, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.last_order_numerator, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.last_order_denominator, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.plan_orders_cur, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.place_orders_cur, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.valid_buy_order_num, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.valid_sell_order_num, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.padding3, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.free_slot_bits, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct TargetOrdersAccount(pub TargetOrders);
impl TargetOrdersAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != TARGET_ORDERS_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(TargetOrders::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&TARGET_ORDERS_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const FEES_ACCOUNT_DISCM: [u8; 8] = [151, 157, 50, 115, 130, 72, 179, 36];
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
pub struct Fees {
    pub min_separate_numerator: u64,
    pub min_separate_denominator: u64,
    pub trade_fee_numerator: u64,
    pub trade_fee_denominator: u64,
    pub pnl_numerator: u64,
    pub pnl_denominator: u64,
    pub swap_fee_numerator: u64,
    pub swap_fee_denominator: u64,
}
impl Fees {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let min_separate_numerator: u64 = crate::borsh_de_or_default(&mut reader)?;
        let min_separate_denominator: u64 = crate::borsh_de_or_default(&mut reader)?;
        let trade_fee_numerator: u64 = crate::borsh_de_or_default(&mut reader)?;
        let trade_fee_denominator: u64 = crate::borsh_de_or_default(&mut reader)?;
        let pnl_numerator: u64 = crate::borsh_de_or_default(&mut reader)?;
        let pnl_denominator: u64 = crate::borsh_de_or_default(&mut reader)?;
        let swap_fee_numerator: u64 = crate::borsh_de_or_default(&mut reader)?;
        let swap_fee_denominator: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            min_separate_numerator,
            min_separate_denominator,
            trade_fee_numerator,
            trade_fee_denominator,
            pnl_numerator,
            pnl_denominator,
            swap_fee_numerator,
            swap_fee_denominator,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.min_separate_numerator, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.min_separate_denominator, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.trade_fee_numerator, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.trade_fee_denominator, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.pnl_numerator, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.pnl_denominator, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.swap_fee_numerator, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.swap_fee_denominator, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct FeesAccount(pub Fees);
impl FeesAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != FEES_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(Fees::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&FEES_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const AMM_INFO_ACCOUNT_DISCM: [u8; 8] = [33, 217, 2, 203, 184, 83, 235, 91];
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
pub struct AmmInfo {
    pub status: u64,
    pub nonce: u64,
    pub order_num: u64,
    pub depth: u64,
    pub coin_decimals: u64,
    pub pc_decimals: u64,
    pub state: u64,
    pub reset_flag: u64,
    pub min_size: u64,
    pub vol_max_cut_ratio: u64,
    pub amount_wave: u64,
    pub coin_lot_size: u64,
    pub pc_lot_size: u64,
    pub min_price_multiplier: u64,
    pub max_price_multiplier: u64,
    pub sys_decimal_value: u64,
    pub fees: Fees,
    pub out_put: OutPutData,
    pub token_coin: Pubkey,
    pub token_pc: Pubkey,
    pub coin_mint: Pubkey,
    pub pc_mint: Pubkey,
    pub lp_mint: Pubkey,
    pub open_orders: Pubkey,
    pub market: Pubkey,
    pub serum_dex: Pubkey,
    pub target_orders: Pubkey,
    pub withdraw_queue: Pubkey,
    pub token_temp_lp: Pubkey,
    pub amm_owner: Pubkey,
    pub lp_amount: u64,
    pub client_order_id: u64,
    pub padding: [u64; 2],
}
impl AmmInfo {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let status: u64 = crate::borsh_de_or_default(&mut reader)?;
        let nonce: u64 = crate::borsh_de_or_default(&mut reader)?;
        let order_num: u64 = crate::borsh_de_or_default(&mut reader)?;
        let depth: u64 = crate::borsh_de_or_default(&mut reader)?;
        let coin_decimals: u64 = crate::borsh_de_or_default(&mut reader)?;
        let pc_decimals: u64 = crate::borsh_de_or_default(&mut reader)?;
        let state: u64 = crate::borsh_de_or_default(&mut reader)?;
        let reset_flag: u64 = crate::borsh_de_or_default(&mut reader)?;
        let min_size: u64 = crate::borsh_de_or_default(&mut reader)?;
        let vol_max_cut_ratio: u64 = crate::borsh_de_or_default(&mut reader)?;
        let amount_wave: u64 = crate::borsh_de_or_default(&mut reader)?;
        let coin_lot_size: u64 = crate::borsh_de_or_default(&mut reader)?;
        let pc_lot_size: u64 = crate::borsh_de_or_default(&mut reader)?;
        let min_price_multiplier: u64 = crate::borsh_de_or_default(&mut reader)?;
        let max_price_multiplier: u64 = crate::borsh_de_or_default(&mut reader)?;
        let sys_decimal_value: u64 = crate::borsh_de_or_default(&mut reader)?;
        let fees = if reader.is_empty() {
            Default::default()
        } else {
            <Fees>::deserialize(&mut reader)?
        };
        let out_put = if reader.is_empty() {
            Default::default()
        } else {
            <OutPutData>::deserialize(&mut reader)?
        };
        let token_coin: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let token_pc: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let coin_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let pc_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let lp_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let open_orders: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let market: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let serum_dex: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let target_orders: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let withdraw_queue: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let token_temp_lp: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let amm_owner: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let lp_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let client_order_id: u64 = crate::borsh_de_or_default(&mut reader)?;
        let padding: [u64; 2] = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            status,
            nonce,
            order_num,
            depth,
            coin_decimals,
            pc_decimals,
            state,
            reset_flag,
            min_size,
            vol_max_cut_ratio,
            amount_wave,
            coin_lot_size,
            pc_lot_size,
            min_price_multiplier,
            max_price_multiplier,
            sys_decimal_value,
            fees,
            out_put,
            token_coin,
            token_pc,
            coin_mint,
            pc_mint,
            lp_mint,
            open_orders,
            market,
            serum_dex,
            target_orders,
            withdraw_queue,
            token_temp_lp,
            amm_owner,
            lp_amount,
            client_order_id,
            padding,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.status, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.nonce, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.order_num, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.depth, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.coin_decimals, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.pc_decimals, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.state, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.reset_flag, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.min_size, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.vol_max_cut_ratio, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.amount_wave, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.coin_lot_size, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.pc_lot_size, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.min_price_multiplier, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.max_price_multiplier, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.sys_decimal_value, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fees, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.out_put, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_coin, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_pc, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.coin_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.pc_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.lp_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.open_orders, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.market, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.serum_dex, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.target_orders, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.withdraw_queue, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_temp_lp, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.amm_owner, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.lp_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.client_order_id, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.padding, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct AmmInfoAccount(pub AmmInfo);
impl AmmInfoAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != AMM_INFO_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(AmmInfo::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&AMM_INFO_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
