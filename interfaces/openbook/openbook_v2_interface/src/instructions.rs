use solana_pubkey::Pubkey;
use solana_cpi::{invoke, invoke_signed};
use solana_instruction::{AccountMeta, Instruction};
use solana_account_info::AccountInfo;
use solana_program_error::ProgramError;
use std::io::Read;
#[allow(unused_imports)]
use crate::*;
#[derive(Clone, Debug, PartialEq)]
pub enum OpenbookV2ProgramIx {
    CreateMarket(CreateMarketIxArgs),
    CloseMarket,
    CreateOpenOrdersIndexer,
    CloseOpenOrdersIndexer,
    CreateOpenOrdersAccount(CreateOpenOrdersAccountIxArgs),
    CloseOpenOrdersAccount,
    PlaceOrder(PlaceOrderIxArgs),
    EditOrder(EditOrderIxArgs),
    EditOrderPegged(EditOrderPeggedIxArgs),
    PlaceOrders(PlaceOrdersIxArgs),
    CancelAllAndPlaceOrders(CancelAllAndPlaceOrdersIxArgs),
    PlaceOrderPegged(PlaceOrderPeggedIxArgs),
    PlaceTakeOrder(PlaceTakeOrderIxArgs),
    ConsumeEvents(ConsumeEventsIxArgs),
    ConsumeGivenEvents(ConsumeGivenEventsIxArgs),
    CancelOrder(CancelOrderIxArgs),
    CancelOrderByClientOrderId(CancelOrderByClientOrderIdIxArgs),
    CancelAllOrders(CancelAllOrdersIxArgs),
    Deposit(DepositIxArgs),
    Refill(RefillIxArgs),
    SettleFunds,
    SettleFundsExpired,
    SweepFees,
    SetDelegate,
    SetMarketExpired,
    PruneOrders(PruneOrdersIxArgs),
    StubOracleCreate(StubOracleCreateIxArgs),
    StubOracleClose,
    StubOracleSet(StubOracleSetIxArgs),
}
impl OpenbookV2ProgramIx {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        if buf.starts_with(&CREATE_MARKET_IX_DISCM) {
            let mut reader = &buf[CREATE_MARKET_IX_DISCM.len()..];
            let name: String = crate::borsh_de_or_default(&mut reader)?;
            let oracle_config = if reader.is_empty() {
                Default::default()
            } else {
                <OracleConfigParams>::deserialize(&mut reader)?
            };
            let quote_lot_size: i64 = crate::borsh_de_or_default(&mut reader)?;
            let base_lot_size: i64 = crate::borsh_de_or_default(&mut reader)?;
            let maker_fee: i64 = crate::borsh_de_or_default(&mut reader)?;
            let taker_fee: i64 = crate::borsh_de_or_default(&mut reader)?;
            let time_expiry: i64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::CreateMarket(CreateMarketIxArgs {
                    name,
                    oracle_config,
                    quote_lot_size,
                    base_lot_size,
                    maker_fee,
                    taker_fee,
                    time_expiry,
                }),
            );
        }
        if buf.starts_with(&CLOSE_MARKET_IX_DISCM) {
            return Ok(Self::CloseMarket);
        }
        if buf.starts_with(&CREATE_OPEN_ORDERS_INDEXER_IX_DISCM) {
            return Ok(Self::CreateOpenOrdersIndexer);
        }
        if buf.starts_with(&CLOSE_OPEN_ORDERS_INDEXER_IX_DISCM) {
            return Ok(Self::CloseOpenOrdersIndexer);
        }
        if buf.starts_with(&CREATE_OPEN_ORDERS_ACCOUNT_IX_DISCM) {
            let mut reader = &buf[CREATE_OPEN_ORDERS_ACCOUNT_IX_DISCM.len()..];
            let name: String = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::CreateOpenOrdersAccount(CreateOpenOrdersAccountIxArgs {
                    name,
                }),
            );
        }
        if buf.starts_with(&CLOSE_OPEN_ORDERS_ACCOUNT_IX_DISCM) {
            return Ok(Self::CloseOpenOrdersAccount);
        }
        if buf.starts_with(&PLACE_ORDER_IX_DISCM) {
            let mut reader = &buf[PLACE_ORDER_IX_DISCM.len()..];
            let args = if reader.is_empty() {
                Default::default()
            } else {
                <PlaceOrderArgs>::deserialize(&mut reader)?
            };
            return Ok(Self::PlaceOrder(PlaceOrderIxArgs { args }));
        }
        if buf.starts_with(&EDIT_ORDER_IX_DISCM) {
            let mut reader = &buf[EDIT_ORDER_IX_DISCM.len()..];
            let client_order_id: u64 = crate::borsh_de_or_default(&mut reader)?;
            let expected_cancel_size: i64 = crate::borsh_de_or_default(&mut reader)?;
            let place_order = if reader.is_empty() {
                Default::default()
            } else {
                <PlaceOrderArgs>::deserialize(&mut reader)?
            };
            return Ok(
                Self::EditOrder(EditOrderIxArgs {
                    client_order_id,
                    expected_cancel_size,
                    place_order,
                }),
            );
        }
        if buf.starts_with(&EDIT_ORDER_PEGGED_IX_DISCM) {
            let mut reader = &buf[EDIT_ORDER_PEGGED_IX_DISCM.len()..];
            let client_order_id: u64 = crate::borsh_de_or_default(&mut reader)?;
            let expected_cancel_size: i64 = crate::borsh_de_or_default(&mut reader)?;
            let place_order = if reader.is_empty() {
                Default::default()
            } else {
                <PlaceOrderPeggedArgs>::deserialize(&mut reader)?
            };
            return Ok(
                Self::EditOrderPegged(EditOrderPeggedIxArgs {
                    client_order_id,
                    expected_cancel_size,
                    place_order,
                }),
            );
        }
        if buf.starts_with(&PLACE_ORDERS_IX_DISCM) {
            let mut reader = &buf[PLACE_ORDERS_IX_DISCM.len()..];
            let orders_type: PlaceOrderType = crate::borsh_de_or_default(&mut reader)?;
            let bids: Vec<PlaceMultipleOrdersArgs> = crate::borsh_de_or_default(
                &mut reader,
            )?;
            let asks: Vec<PlaceMultipleOrdersArgs> = crate::borsh_de_or_default(
                &mut reader,
            )?;
            let limit: u8 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::PlaceOrders(PlaceOrdersIxArgs {
                    orders_type,
                    bids,
                    asks,
                    limit,
                }),
            );
        }
        if buf.starts_with(&CANCEL_ALL_AND_PLACE_ORDERS_IX_DISCM) {
            let mut reader = &buf[CANCEL_ALL_AND_PLACE_ORDERS_IX_DISCM.len()..];
            let orders_type: PlaceOrderType = crate::borsh_de_or_default(&mut reader)?;
            let bids: Vec<PlaceMultipleOrdersArgs> = crate::borsh_de_or_default(
                &mut reader,
            )?;
            let asks: Vec<PlaceMultipleOrdersArgs> = crate::borsh_de_or_default(
                &mut reader,
            )?;
            let limit: u8 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::CancelAllAndPlaceOrders(CancelAllAndPlaceOrdersIxArgs {
                    orders_type,
                    bids,
                    asks,
                    limit,
                }),
            );
        }
        if buf.starts_with(&PLACE_ORDER_PEGGED_IX_DISCM) {
            let mut reader = &buf[PLACE_ORDER_PEGGED_IX_DISCM.len()..];
            let args = if reader.is_empty() {
                Default::default()
            } else {
                <PlaceOrderPeggedArgs>::deserialize(&mut reader)?
            };
            return Ok(Self::PlaceOrderPegged(PlaceOrderPeggedIxArgs { args }));
        }
        if buf.starts_with(&PLACE_TAKE_ORDER_IX_DISCM) {
            let mut reader = &buf[PLACE_TAKE_ORDER_IX_DISCM.len()..];
            let args = if reader.is_empty() {
                Default::default()
            } else {
                <PlaceTakeOrderArgs>::deserialize(&mut reader)?
            };
            return Ok(Self::PlaceTakeOrder(PlaceTakeOrderIxArgs { args }));
        }
        if buf.starts_with(&CONSUME_EVENTS_IX_DISCM) {
            let mut reader = &buf[CONSUME_EVENTS_IX_DISCM.len()..];
            let limit: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(Self::ConsumeEvents(ConsumeEventsIxArgs { limit }));
        }
        if buf.starts_with(&CONSUME_GIVEN_EVENTS_IX_DISCM) {
            let mut reader = &buf[CONSUME_GIVEN_EVENTS_IX_DISCM.len()..];
            let slots: Vec<u64> = crate::borsh_de_or_default(&mut reader)?;
            return Ok(Self::ConsumeGivenEvents(ConsumeGivenEventsIxArgs { slots }));
        }
        if buf.starts_with(&CANCEL_ORDER_IX_DISCM) {
            let mut reader = &buf[CANCEL_ORDER_IX_DISCM.len()..];
            let order_id: u128 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(Self::CancelOrder(CancelOrderIxArgs { order_id }));
        }
        if buf.starts_with(&CANCEL_ORDER_BY_CLIENT_ORDER_ID_IX_DISCM) {
            let mut reader = &buf[CANCEL_ORDER_BY_CLIENT_ORDER_ID_IX_DISCM.len()..];
            let client_order_id: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::CancelOrderByClientOrderId(CancelOrderByClientOrderIdIxArgs {
                    client_order_id,
                }),
            );
        }
        if buf.starts_with(&CANCEL_ALL_ORDERS_IX_DISCM) {
            let mut reader = &buf[CANCEL_ALL_ORDERS_IX_DISCM.len()..];
            let side_option: Option<Side> = crate::borsh_de_or_default(&mut reader)?;
            let limit: u8 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::CancelAllOrders(CancelAllOrdersIxArgs {
                    side_option,
                    limit,
                }),
            );
        }
        if buf.starts_with(&DEPOSIT_IX_DISCM) {
            let mut reader = &buf[DEPOSIT_IX_DISCM.len()..];
            let base_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            let quote_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::Deposit(DepositIxArgs {
                    base_amount,
                    quote_amount,
                }),
            );
        }
        if buf.starts_with(&REFILL_IX_DISCM) {
            let mut reader = &buf[REFILL_IX_DISCM.len()..];
            let base_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            let quote_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::Refill(RefillIxArgs {
                    base_amount,
                    quote_amount,
                }),
            );
        }
        if buf.starts_with(&SETTLE_FUNDS_IX_DISCM) {
            return Ok(Self::SettleFunds);
        }
        if buf.starts_with(&SETTLE_FUNDS_EXPIRED_IX_DISCM) {
            return Ok(Self::SettleFundsExpired);
        }
        if buf.starts_with(&SWEEP_FEES_IX_DISCM) {
            return Ok(Self::SweepFees);
        }
        if buf.starts_with(&SET_DELEGATE_IX_DISCM) {
            return Ok(Self::SetDelegate);
        }
        if buf.starts_with(&SET_MARKET_EXPIRED_IX_DISCM) {
            return Ok(Self::SetMarketExpired);
        }
        if buf.starts_with(&PRUNE_ORDERS_IX_DISCM) {
            let mut reader = &buf[PRUNE_ORDERS_IX_DISCM.len()..];
            let limit: u8 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(Self::PruneOrders(PruneOrdersIxArgs { limit }));
        }
        if buf.starts_with(&STUB_ORACLE_CREATE_IX_DISCM) {
            let mut reader = &buf[STUB_ORACLE_CREATE_IX_DISCM.len()..];
            let price: f64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(Self::StubOracleCreate(StubOracleCreateIxArgs { price }));
        }
        if buf.starts_with(&STUB_ORACLE_CLOSE_IX_DISCM) {
            return Ok(Self::StubOracleClose);
        }
        if buf.starts_with(&STUB_ORACLE_SET_IX_DISCM) {
            let mut reader = &buf[STUB_ORACLE_SET_IX_DISCM.len()..];
            let price: f64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(Self::StubOracleSet(StubOracleSetIxArgs { price }));
        }
        Err(std::io::Error::from(std::io::ErrorKind::InvalidData))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        match self {
            Self::CreateMarket(args) => {
                writer.write_all(&CREATE_MARKET_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.name, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.oracle_config, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.quote_lot_size, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.base_lot_size, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.maker_fee, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.taker_fee, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.time_expiry, &mut writer)?;
                Ok(())
            }
            Self::CloseMarket => writer.write_all(&CLOSE_MARKET_IX_DISCM),
            Self::CreateOpenOrdersIndexer => {
                writer.write_all(&CREATE_OPEN_ORDERS_INDEXER_IX_DISCM)
            }
            Self::CloseOpenOrdersIndexer => {
                writer.write_all(&CLOSE_OPEN_ORDERS_INDEXER_IX_DISCM)
            }
            Self::CreateOpenOrdersAccount(args) => {
                writer.write_all(&CREATE_OPEN_ORDERS_ACCOUNT_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.name, &mut writer)?;
                Ok(())
            }
            Self::CloseOpenOrdersAccount => {
                writer.write_all(&CLOSE_OPEN_ORDERS_ACCOUNT_IX_DISCM)
            }
            Self::PlaceOrder(args) => {
                writer.write_all(&PLACE_ORDER_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.args, &mut writer)?;
                Ok(())
            }
            Self::EditOrder(args) => {
                writer.write_all(&EDIT_ORDER_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.client_order_id, &mut writer)?;
                borsh::BorshSerialize::serialize(
                    &args.expected_cancel_size,
                    &mut writer,
                )?;
                borsh::BorshSerialize::serialize(&args.place_order, &mut writer)?;
                Ok(())
            }
            Self::EditOrderPegged(args) => {
                writer.write_all(&EDIT_ORDER_PEGGED_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.client_order_id, &mut writer)?;
                borsh::BorshSerialize::serialize(
                    &args.expected_cancel_size,
                    &mut writer,
                )?;
                borsh::BorshSerialize::serialize(&args.place_order, &mut writer)?;
                Ok(())
            }
            Self::PlaceOrders(args) => {
                writer.write_all(&PLACE_ORDERS_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.orders_type, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.bids, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.asks, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.limit, &mut writer)?;
                Ok(())
            }
            Self::CancelAllAndPlaceOrders(args) => {
                writer.write_all(&CANCEL_ALL_AND_PLACE_ORDERS_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.orders_type, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.bids, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.asks, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.limit, &mut writer)?;
                Ok(())
            }
            Self::PlaceOrderPegged(args) => {
                writer.write_all(&PLACE_ORDER_PEGGED_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.args, &mut writer)?;
                Ok(())
            }
            Self::PlaceTakeOrder(args) => {
                writer.write_all(&PLACE_TAKE_ORDER_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.args, &mut writer)?;
                Ok(())
            }
            Self::ConsumeEvents(args) => {
                writer.write_all(&CONSUME_EVENTS_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.limit, &mut writer)?;
                Ok(())
            }
            Self::ConsumeGivenEvents(args) => {
                writer.write_all(&CONSUME_GIVEN_EVENTS_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.slots, &mut writer)?;
                Ok(())
            }
            Self::CancelOrder(args) => {
                writer.write_all(&CANCEL_ORDER_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.order_id, &mut writer)?;
                Ok(())
            }
            Self::CancelOrderByClientOrderId(args) => {
                writer.write_all(&CANCEL_ORDER_BY_CLIENT_ORDER_ID_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.client_order_id, &mut writer)?;
                Ok(())
            }
            Self::CancelAllOrders(args) => {
                writer.write_all(&CANCEL_ALL_ORDERS_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.side_option, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.limit, &mut writer)?;
                Ok(())
            }
            Self::Deposit(args) => {
                writer.write_all(&DEPOSIT_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.base_amount, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.quote_amount, &mut writer)?;
                Ok(())
            }
            Self::Refill(args) => {
                writer.write_all(&REFILL_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.base_amount, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.quote_amount, &mut writer)?;
                Ok(())
            }
            Self::SettleFunds => writer.write_all(&SETTLE_FUNDS_IX_DISCM),
            Self::SettleFundsExpired => writer.write_all(&SETTLE_FUNDS_EXPIRED_IX_DISCM),
            Self::SweepFees => writer.write_all(&SWEEP_FEES_IX_DISCM),
            Self::SetDelegate => writer.write_all(&SET_DELEGATE_IX_DISCM),
            Self::SetMarketExpired => writer.write_all(&SET_MARKET_EXPIRED_IX_DISCM),
            Self::PruneOrders(args) => {
                writer.write_all(&PRUNE_ORDERS_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.limit, &mut writer)?;
                Ok(())
            }
            Self::StubOracleCreate(args) => {
                writer.write_all(&STUB_ORACLE_CREATE_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.price, &mut writer)?;
                Ok(())
            }
            Self::StubOracleClose => writer.write_all(&STUB_ORACLE_CLOSE_IX_DISCM),
            Self::StubOracleSet(args) => {
                writer.write_all(&STUB_ORACLE_SET_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.price, &mut writer)?;
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
pub const CREATE_MARKET_IX_ACCOUNTS_LEN: usize = 21;
#[derive(Copy, Clone, Debug)]
pub struct CreateMarketAccounts<'me, 'info> {
    pub market: &'me AccountInfo<'info>,
    pub market_authority: &'me AccountInfo<'info>,
    pub bids: &'me AccountInfo<'info>,
    pub asks: &'me AccountInfo<'info>,
    pub event_heap: &'me AccountInfo<'info>,
    pub payer: &'me AccountInfo<'info>,
    pub market_base_vault: &'me AccountInfo<'info>,
    pub market_quote_vault: &'me AccountInfo<'info>,
    pub base_mint: &'me AccountInfo<'info>,
    pub quote_mint: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
    pub oracle_a: &'me AccountInfo<'info>,
    pub oracle_b: &'me AccountInfo<'info>,
    pub collect_fee_admin: &'me AccountInfo<'info>,
    pub open_orders_admin: &'me AccountInfo<'info>,
    pub consume_events_admin: &'me AccountInfo<'info>,
    pub close_market_admin: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CreateMarketKeys {
    pub market: Pubkey,
    pub market_authority: Pubkey,
    pub bids: Pubkey,
    pub asks: Pubkey,
    pub event_heap: Pubkey,
    pub payer: Pubkey,
    pub market_base_vault: Pubkey,
    pub market_quote_vault: Pubkey,
    pub base_mint: Pubkey,
    pub quote_mint: Pubkey,
    pub system_program: Pubkey,
    pub token_program: Pubkey,
    pub associated_token_program: Pubkey,
    pub oracle_a: Pubkey,
    pub oracle_b: Pubkey,
    pub collect_fee_admin: Pubkey,
    pub open_orders_admin: Pubkey,
    pub consume_events_admin: Pubkey,
    pub close_market_admin: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<CreateMarketAccounts<'_, '_>> for CreateMarketKeys {
    fn from(accounts: CreateMarketAccounts) -> Self {
        Self {
            market: *accounts.market.key,
            market_authority: *accounts.market_authority.key,
            bids: *accounts.bids.key,
            asks: *accounts.asks.key,
            event_heap: *accounts.event_heap.key,
            payer: *accounts.payer.key,
            market_base_vault: *accounts.market_base_vault.key,
            market_quote_vault: *accounts.market_quote_vault.key,
            base_mint: *accounts.base_mint.key,
            quote_mint: *accounts.quote_mint.key,
            system_program: *accounts.system_program.key,
            token_program: *accounts.token_program.key,
            associated_token_program: *accounts.associated_token_program.key,
            oracle_a: *accounts.oracle_a.key,
            oracle_b: *accounts.oracle_b.key,
            collect_fee_admin: *accounts.collect_fee_admin.key,
            open_orders_admin: *accounts.open_orders_admin.key,
            consume_events_admin: *accounts.consume_events_admin.key,
            close_market_admin: *accounts.close_market_admin.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<CreateMarketKeys> for [AccountMeta; CREATE_MARKET_IX_ACCOUNTS_LEN] {
    fn from(keys: CreateMarketKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.market,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.market_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.bids,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.asks,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.event_heap,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.payer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.market_base_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.market_quote_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.base_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.quote_mint,
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
                pubkey: keys.oracle_a,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.oracle_b,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.collect_fee_admin,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.open_orders_admin,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.consume_events_admin,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.close_market_admin,
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
            market: pubkeys[0],
            market_authority: pubkeys[1],
            bids: pubkeys[2],
            asks: pubkeys[3],
            event_heap: pubkeys[4],
            payer: pubkeys[5],
            market_base_vault: pubkeys[6],
            market_quote_vault: pubkeys[7],
            base_mint: pubkeys[8],
            quote_mint: pubkeys[9],
            system_program: pubkeys[10],
            token_program: pubkeys[11],
            associated_token_program: pubkeys[12],
            oracle_a: pubkeys[13],
            oracle_b: pubkeys[14],
            collect_fee_admin: pubkeys[15],
            open_orders_admin: pubkeys[16],
            consume_events_admin: pubkeys[17],
            close_market_admin: pubkeys[18],
            event_authority: pubkeys[19],
            program: pubkeys[20],
        }
    }
}
impl<'info> From<CreateMarketAccounts<'_, 'info>>
for [AccountInfo<'info>; CREATE_MARKET_IX_ACCOUNTS_LEN] {
    fn from(accounts: CreateMarketAccounts<'_, 'info>) -> Self {
        [
            accounts.market.clone(),
            accounts.market_authority.clone(),
            accounts.bids.clone(),
            accounts.asks.clone(),
            accounts.event_heap.clone(),
            accounts.payer.clone(),
            accounts.market_base_vault.clone(),
            accounts.market_quote_vault.clone(),
            accounts.base_mint.clone(),
            accounts.quote_mint.clone(),
            accounts.system_program.clone(),
            accounts.token_program.clone(),
            accounts.associated_token_program.clone(),
            accounts.oracle_a.clone(),
            accounts.oracle_b.clone(),
            accounts.collect_fee_admin.clone(),
            accounts.open_orders_admin.clone(),
            accounts.consume_events_admin.clone(),
            accounts.close_market_admin.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CREATE_MARKET_IX_ACCOUNTS_LEN]>
for CreateMarketAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; CREATE_MARKET_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            market: &arr[0],
            market_authority: &arr[1],
            bids: &arr[2],
            asks: &arr[3],
            event_heap: &arr[4],
            payer: &arr[5],
            market_base_vault: &arr[6],
            market_quote_vault: &arr[7],
            base_mint: &arr[8],
            quote_mint: &arr[9],
            system_program: &arr[10],
            token_program: &arr[11],
            associated_token_program: &arr[12],
            oracle_a: &arr[13],
            oracle_b: &arr[14],
            collect_fee_admin: &arr[15],
            open_orders_admin: &arr[16],
            consume_events_admin: &arr[17],
            close_market_admin: &arr[18],
            event_authority: &arr[19],
            program: &arr[20],
        }
    }
}
pub const CREATE_MARKET_IX_DISCM: [u8; 8usize] = [103, 226, 97, 235, 200, 188, 251, 254];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CreateMarketIxArgs {
    pub name: String,
    pub oracle_config: OracleConfigParams,
    pub quote_lot_size: i64,
    pub base_lot_size: i64,
    pub maker_fee: i64,
    pub taker_fee: i64,
    pub time_expiry: i64,
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
        let oracle_config = if reader.is_empty() {
            Default::default()
        } else {
            <OracleConfigParams>::deserialize(&mut reader)?
        };
        let quote_lot_size: i64 = crate::borsh_de_or_default(&mut reader)?;
        let base_lot_size: i64 = crate::borsh_de_or_default(&mut reader)?;
        let maker_fee: i64 = crate::borsh_de_or_default(&mut reader)?;
        let taker_fee: i64 = crate::borsh_de_or_default(&mut reader)?;
        let time_expiry: i64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(CreateMarketIxArgs {
                name,
                oracle_config,
                quote_lot_size,
                base_lot_size,
                maker_fee,
                taker_fee,
                time_expiry,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CREATE_MARKET_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.name, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.oracle_config, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.quote_lot_size, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.base_lot_size, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.maker_fee, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.taker_fee, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.time_expiry, &mut writer)?;
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
    create_market_ix_with_program_id(OPENBOOK_V2_PROGRAM_ID, keys, args)
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
    create_market_invoke_with_program_id(OPENBOOK_V2_PROGRAM_ID, accounts, args)
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
        OPENBOOK_V2_PROGRAM_ID,
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
        (*accounts.market.key, keys.market),
        (*accounts.market_authority.key, keys.market_authority),
        (*accounts.bids.key, keys.bids),
        (*accounts.asks.key, keys.asks),
        (*accounts.event_heap.key, keys.event_heap),
        (*accounts.payer.key, keys.payer),
        (*accounts.market_base_vault.key, keys.market_base_vault),
        (*accounts.market_quote_vault.key, keys.market_quote_vault),
        (*accounts.base_mint.key, keys.base_mint),
        (*accounts.quote_mint.key, keys.quote_mint),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.associated_token_program.key, keys.associated_token_program),
        (*accounts.oracle_a.key, keys.oracle_a),
        (*accounts.oracle_b.key, keys.oracle_b),
        (*accounts.collect_fee_admin.key, keys.collect_fee_admin),
        (*accounts.open_orders_admin.key, keys.open_orders_admin),
        (*accounts.consume_events_admin.key, keys.consume_events_admin),
        (*accounts.close_market_admin.key, keys.close_market_admin),
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
        accounts.bids,
        accounts.asks,
        accounts.event_heap,
        accounts.payer,
        accounts.market_base_vault,
        accounts.market_quote_vault,
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
    for should_be_signer in [accounts.market, accounts.payer] {
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
pub const CLOSE_MARKET_IX_ACCOUNTS_LEN: usize = 7;
#[derive(Copy, Clone, Debug)]
pub struct CloseMarketAccounts<'me, 'info> {
    pub close_market_admin: &'me AccountInfo<'info>,
    pub market: &'me AccountInfo<'info>,
    pub bids: &'me AccountInfo<'info>,
    pub asks: &'me AccountInfo<'info>,
    pub event_heap: &'me AccountInfo<'info>,
    pub sol_destination: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CloseMarketKeys {
    pub close_market_admin: Pubkey,
    pub market: Pubkey,
    pub bids: Pubkey,
    pub asks: Pubkey,
    pub event_heap: Pubkey,
    pub sol_destination: Pubkey,
    pub token_program: Pubkey,
}
impl From<CloseMarketAccounts<'_, '_>> for CloseMarketKeys {
    fn from(accounts: CloseMarketAccounts) -> Self {
        Self {
            close_market_admin: *accounts.close_market_admin.key,
            market: *accounts.market.key,
            bids: *accounts.bids.key,
            asks: *accounts.asks.key,
            event_heap: *accounts.event_heap.key,
            sol_destination: *accounts.sol_destination.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<CloseMarketKeys> for [AccountMeta; CLOSE_MARKET_IX_ACCOUNTS_LEN] {
    fn from(keys: CloseMarketKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.close_market_admin,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.market,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.bids,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.asks,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.event_heap,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.sol_destination,
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
impl From<[Pubkey; CLOSE_MARKET_IX_ACCOUNTS_LEN]> for CloseMarketKeys {
    fn from(pubkeys: [Pubkey; CLOSE_MARKET_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            close_market_admin: pubkeys[0],
            market: pubkeys[1],
            bids: pubkeys[2],
            asks: pubkeys[3],
            event_heap: pubkeys[4],
            sol_destination: pubkeys[5],
            token_program: pubkeys[6],
        }
    }
}
impl<'info> From<CloseMarketAccounts<'_, 'info>>
for [AccountInfo<'info>; CLOSE_MARKET_IX_ACCOUNTS_LEN] {
    fn from(accounts: CloseMarketAccounts<'_, 'info>) -> Self {
        [
            accounts.close_market_admin.clone(),
            accounts.market.clone(),
            accounts.bids.clone(),
            accounts.asks.clone(),
            accounts.event_heap.clone(),
            accounts.sol_destination.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CLOSE_MARKET_IX_ACCOUNTS_LEN]>
for CloseMarketAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; CLOSE_MARKET_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            close_market_admin: &arr[0],
            market: &arr[1],
            bids: &arr[2],
            asks: &arr[3],
            event_heap: &arr[4],
            sol_destination: &arr[5],
            token_program: &arr[6],
        }
    }
}
pub const CLOSE_MARKET_IX_DISCM: [u8; 8usize] = [88, 154, 248, 186, 48, 14, 123, 244];
#[derive(Clone, Debug, PartialEq)]
pub struct CloseMarketIxData;
impl CloseMarketIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CLOSE_MARKET_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CLOSE_MARKET_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn close_market_ix_with_program_id(
    program_id: Pubkey,
    keys: CloseMarketKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CLOSE_MARKET_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: CloseMarketIxData.try_to_vec()?,
    })
}
pub fn close_market_ix(keys: CloseMarketKeys) -> std::io::Result<Instruction> {
    close_market_ix_with_program_id(OPENBOOK_V2_PROGRAM_ID, keys)
}
pub fn close_market_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CloseMarketAccounts<'_, '_>,
) -> ProgramResult {
    let keys: CloseMarketKeys = accounts.into();
    let ix = close_market_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn close_market_invoke(accounts: CloseMarketAccounts<'_, '_>) -> ProgramResult {
    close_market_invoke_with_program_id(OPENBOOK_V2_PROGRAM_ID, accounts)
}
pub fn close_market_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CloseMarketAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CloseMarketKeys = accounts.into();
    let ix = close_market_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn close_market_invoke_signed(
    accounts: CloseMarketAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    close_market_invoke_signed_with_program_id(OPENBOOK_V2_PROGRAM_ID, accounts, seeds)
}
pub fn close_market_verify_account_keys(
    accounts: CloseMarketAccounts<'_, '_>,
    keys: CloseMarketKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.close_market_admin.key, keys.close_market_admin),
        (*accounts.market.key, keys.market),
        (*accounts.bids.key, keys.bids),
        (*accounts.asks.key, keys.asks),
        (*accounts.event_heap.key, keys.event_heap),
        (*accounts.sol_destination.key, keys.sol_destination),
        (*accounts.token_program.key, keys.token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn close_market_verify_writable_privileges<'me, 'info>(
    accounts: CloseMarketAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.market,
        accounts.bids,
        accounts.asks,
        accounts.event_heap,
        accounts.sol_destination,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn close_market_verify_signer_privileges<'me, 'info>(
    accounts: CloseMarketAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.close_market_admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn close_market_verify_account_privileges<'me, 'info>(
    accounts: CloseMarketAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    close_market_verify_writable_privileges(accounts)?;
    close_market_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CREATE_OPEN_ORDERS_INDEXER_IX_ACCOUNTS_LEN: usize = 4;
#[derive(Copy, Clone, Debug)]
pub struct CreateOpenOrdersIndexerAccounts<'me, 'info> {
    pub payer: &'me AccountInfo<'info>,
    pub owner: &'me AccountInfo<'info>,
    pub open_orders_indexer: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CreateOpenOrdersIndexerKeys {
    pub payer: Pubkey,
    pub owner: Pubkey,
    pub open_orders_indexer: Pubkey,
    pub system_program: Pubkey,
}
impl From<CreateOpenOrdersIndexerAccounts<'_, '_>> for CreateOpenOrdersIndexerKeys {
    fn from(accounts: CreateOpenOrdersIndexerAccounts) -> Self {
        Self {
            payer: *accounts.payer.key,
            owner: *accounts.owner.key,
            open_orders_indexer: *accounts.open_orders_indexer.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<CreateOpenOrdersIndexerKeys>
for [AccountMeta; CREATE_OPEN_ORDERS_INDEXER_IX_ACCOUNTS_LEN] {
    fn from(keys: CreateOpenOrdersIndexerKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.payer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.owner,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.open_orders_indexer,
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
impl From<[Pubkey; CREATE_OPEN_ORDERS_INDEXER_IX_ACCOUNTS_LEN]>
for CreateOpenOrdersIndexerKeys {
    fn from(pubkeys: [Pubkey; CREATE_OPEN_ORDERS_INDEXER_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            payer: pubkeys[0],
            owner: pubkeys[1],
            open_orders_indexer: pubkeys[2],
            system_program: pubkeys[3],
        }
    }
}
impl<'info> From<CreateOpenOrdersIndexerAccounts<'_, 'info>>
for [AccountInfo<'info>; CREATE_OPEN_ORDERS_INDEXER_IX_ACCOUNTS_LEN] {
    fn from(accounts: CreateOpenOrdersIndexerAccounts<'_, 'info>) -> Self {
        [
            accounts.payer.clone(),
            accounts.owner.clone(),
            accounts.open_orders_indexer.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; CREATE_OPEN_ORDERS_INDEXER_IX_ACCOUNTS_LEN]>
for CreateOpenOrdersIndexerAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; CREATE_OPEN_ORDERS_INDEXER_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            payer: &arr[0],
            owner: &arr[1],
            open_orders_indexer: &arr[2],
            system_program: &arr[3],
        }
    }
}
pub const CREATE_OPEN_ORDERS_INDEXER_IX_DISCM: [u8; 8usize] = [
    64, 64, 153, 255, 217, 71, 249, 133,
];
#[derive(Clone, Debug, PartialEq)]
pub struct CreateOpenOrdersIndexerIxData;
impl CreateOpenOrdersIndexerIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CREATE_OPEN_ORDERS_INDEXER_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CREATE_OPEN_ORDERS_INDEXER_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn create_open_orders_indexer_ix_with_program_id(
    program_id: Pubkey,
    keys: CreateOpenOrdersIndexerKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CREATE_OPEN_ORDERS_INDEXER_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: CreateOpenOrdersIndexerIxData.try_to_vec()?,
    })
}
pub fn create_open_orders_indexer_ix(
    keys: CreateOpenOrdersIndexerKeys,
) -> std::io::Result<Instruction> {
    create_open_orders_indexer_ix_with_program_id(OPENBOOK_V2_PROGRAM_ID, keys)
}
pub fn create_open_orders_indexer_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CreateOpenOrdersIndexerAccounts<'_, '_>,
) -> ProgramResult {
    let keys: CreateOpenOrdersIndexerKeys = accounts.into();
    let ix = create_open_orders_indexer_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn create_open_orders_indexer_invoke(
    accounts: CreateOpenOrdersIndexerAccounts<'_, '_>,
) -> ProgramResult {
    create_open_orders_indexer_invoke_with_program_id(OPENBOOK_V2_PROGRAM_ID, accounts)
}
pub fn create_open_orders_indexer_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CreateOpenOrdersIndexerAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CreateOpenOrdersIndexerKeys = accounts.into();
    let ix = create_open_orders_indexer_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn create_open_orders_indexer_invoke_signed(
    accounts: CreateOpenOrdersIndexerAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    create_open_orders_indexer_invoke_signed_with_program_id(
        OPENBOOK_V2_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn create_open_orders_indexer_verify_account_keys(
    accounts: CreateOpenOrdersIndexerAccounts<'_, '_>,
    keys: CreateOpenOrdersIndexerKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.payer.key, keys.payer),
        (*accounts.owner.key, keys.owner),
        (*accounts.open_orders_indexer.key, keys.open_orders_indexer),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn create_open_orders_indexer_verify_writable_privileges<'me, 'info>(
    accounts: CreateOpenOrdersIndexerAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.payer, accounts.open_orders_indexer] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn create_open_orders_indexer_verify_signer_privileges<'me, 'info>(
    accounts: CreateOpenOrdersIndexerAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.payer, accounts.owner] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn create_open_orders_indexer_verify_account_privileges<'me, 'info>(
    accounts: CreateOpenOrdersIndexerAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    create_open_orders_indexer_verify_writable_privileges(accounts)?;
    create_open_orders_indexer_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CLOSE_OPEN_ORDERS_INDEXER_IX_ACCOUNTS_LEN: usize = 4;
#[derive(Copy, Clone, Debug)]
pub struct CloseOpenOrdersIndexerAccounts<'me, 'info> {
    pub owner: &'me AccountInfo<'info>,
    pub open_orders_indexer: &'me AccountInfo<'info>,
    pub sol_destination: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CloseOpenOrdersIndexerKeys {
    pub owner: Pubkey,
    pub open_orders_indexer: Pubkey,
    pub sol_destination: Pubkey,
    pub token_program: Pubkey,
}
impl From<CloseOpenOrdersIndexerAccounts<'_, '_>> for CloseOpenOrdersIndexerKeys {
    fn from(accounts: CloseOpenOrdersIndexerAccounts) -> Self {
        Self {
            owner: *accounts.owner.key,
            open_orders_indexer: *accounts.open_orders_indexer.key,
            sol_destination: *accounts.sol_destination.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<CloseOpenOrdersIndexerKeys>
for [AccountMeta; CLOSE_OPEN_ORDERS_INDEXER_IX_ACCOUNTS_LEN] {
    fn from(keys: CloseOpenOrdersIndexerKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.owner,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.open_orders_indexer,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.sol_destination,
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
impl From<[Pubkey; CLOSE_OPEN_ORDERS_INDEXER_IX_ACCOUNTS_LEN]>
for CloseOpenOrdersIndexerKeys {
    fn from(pubkeys: [Pubkey; CLOSE_OPEN_ORDERS_INDEXER_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            owner: pubkeys[0],
            open_orders_indexer: pubkeys[1],
            sol_destination: pubkeys[2],
            token_program: pubkeys[3],
        }
    }
}
impl<'info> From<CloseOpenOrdersIndexerAccounts<'_, 'info>>
for [AccountInfo<'info>; CLOSE_OPEN_ORDERS_INDEXER_IX_ACCOUNTS_LEN] {
    fn from(accounts: CloseOpenOrdersIndexerAccounts<'_, 'info>) -> Self {
        [
            accounts.owner.clone(),
            accounts.open_orders_indexer.clone(),
            accounts.sol_destination.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; CLOSE_OPEN_ORDERS_INDEXER_IX_ACCOUNTS_LEN]>
for CloseOpenOrdersIndexerAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; CLOSE_OPEN_ORDERS_INDEXER_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            owner: &arr[0],
            open_orders_indexer: &arr[1],
            sol_destination: &arr[2],
            token_program: &arr[3],
        }
    }
}
pub const CLOSE_OPEN_ORDERS_INDEXER_IX_DISCM: [u8; 8usize] = [
    103, 249, 229, 231, 247, 253, 197, 136,
];
#[derive(Clone, Debug, PartialEq)]
pub struct CloseOpenOrdersIndexerIxData;
impl CloseOpenOrdersIndexerIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CLOSE_OPEN_ORDERS_INDEXER_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CLOSE_OPEN_ORDERS_INDEXER_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn close_open_orders_indexer_ix_with_program_id(
    program_id: Pubkey,
    keys: CloseOpenOrdersIndexerKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CLOSE_OPEN_ORDERS_INDEXER_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: CloseOpenOrdersIndexerIxData.try_to_vec()?,
    })
}
pub fn close_open_orders_indexer_ix(
    keys: CloseOpenOrdersIndexerKeys,
) -> std::io::Result<Instruction> {
    close_open_orders_indexer_ix_with_program_id(OPENBOOK_V2_PROGRAM_ID, keys)
}
pub fn close_open_orders_indexer_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CloseOpenOrdersIndexerAccounts<'_, '_>,
) -> ProgramResult {
    let keys: CloseOpenOrdersIndexerKeys = accounts.into();
    let ix = close_open_orders_indexer_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn close_open_orders_indexer_invoke(
    accounts: CloseOpenOrdersIndexerAccounts<'_, '_>,
) -> ProgramResult {
    close_open_orders_indexer_invoke_with_program_id(OPENBOOK_V2_PROGRAM_ID, accounts)
}
pub fn close_open_orders_indexer_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CloseOpenOrdersIndexerAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CloseOpenOrdersIndexerKeys = accounts.into();
    let ix = close_open_orders_indexer_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn close_open_orders_indexer_invoke_signed(
    accounts: CloseOpenOrdersIndexerAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    close_open_orders_indexer_invoke_signed_with_program_id(
        OPENBOOK_V2_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn close_open_orders_indexer_verify_account_keys(
    accounts: CloseOpenOrdersIndexerAccounts<'_, '_>,
    keys: CloseOpenOrdersIndexerKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.owner.key, keys.owner),
        (*accounts.open_orders_indexer.key, keys.open_orders_indexer),
        (*accounts.sol_destination.key, keys.sol_destination),
        (*accounts.token_program.key, keys.token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn close_open_orders_indexer_verify_writable_privileges<'me, 'info>(
    accounts: CloseOpenOrdersIndexerAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.open_orders_indexer, accounts.sol_destination] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn close_open_orders_indexer_verify_signer_privileges<'me, 'info>(
    accounts: CloseOpenOrdersIndexerAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.owner] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn close_open_orders_indexer_verify_account_privileges<'me, 'info>(
    accounts: CloseOpenOrdersIndexerAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    close_open_orders_indexer_verify_writable_privileges(accounts)?;
    close_open_orders_indexer_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CREATE_OPEN_ORDERS_ACCOUNT_IX_ACCOUNTS_LEN: usize = 7;
#[derive(Copy, Clone, Debug)]
pub struct CreateOpenOrdersAccountAccounts<'me, 'info> {
    pub payer: &'me AccountInfo<'info>,
    pub owner: &'me AccountInfo<'info>,
    pub delegate_account: &'me AccountInfo<'info>,
    pub open_orders_indexer: &'me AccountInfo<'info>,
    pub open_orders_account: &'me AccountInfo<'info>,
    pub market: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CreateOpenOrdersAccountKeys {
    pub payer: Pubkey,
    pub owner: Pubkey,
    pub delegate_account: Pubkey,
    pub open_orders_indexer: Pubkey,
    pub open_orders_account: Pubkey,
    pub market: Pubkey,
    pub system_program: Pubkey,
}
impl From<CreateOpenOrdersAccountAccounts<'_, '_>> for CreateOpenOrdersAccountKeys {
    fn from(accounts: CreateOpenOrdersAccountAccounts) -> Self {
        Self {
            payer: *accounts.payer.key,
            owner: *accounts.owner.key,
            delegate_account: *accounts.delegate_account.key,
            open_orders_indexer: *accounts.open_orders_indexer.key,
            open_orders_account: *accounts.open_orders_account.key,
            market: *accounts.market.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<CreateOpenOrdersAccountKeys>
for [AccountMeta; CREATE_OPEN_ORDERS_ACCOUNT_IX_ACCOUNTS_LEN] {
    fn from(keys: CreateOpenOrdersAccountKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.payer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.owner,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.delegate_account,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.open_orders_indexer,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.open_orders_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.market,
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
impl From<[Pubkey; CREATE_OPEN_ORDERS_ACCOUNT_IX_ACCOUNTS_LEN]>
for CreateOpenOrdersAccountKeys {
    fn from(pubkeys: [Pubkey; CREATE_OPEN_ORDERS_ACCOUNT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            payer: pubkeys[0],
            owner: pubkeys[1],
            delegate_account: pubkeys[2],
            open_orders_indexer: pubkeys[3],
            open_orders_account: pubkeys[4],
            market: pubkeys[5],
            system_program: pubkeys[6],
        }
    }
}
impl<'info> From<CreateOpenOrdersAccountAccounts<'_, 'info>>
for [AccountInfo<'info>; CREATE_OPEN_ORDERS_ACCOUNT_IX_ACCOUNTS_LEN] {
    fn from(accounts: CreateOpenOrdersAccountAccounts<'_, 'info>) -> Self {
        [
            accounts.payer.clone(),
            accounts.owner.clone(),
            accounts.delegate_account.clone(),
            accounts.open_orders_indexer.clone(),
            accounts.open_orders_account.clone(),
            accounts.market.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; CREATE_OPEN_ORDERS_ACCOUNT_IX_ACCOUNTS_LEN]>
for CreateOpenOrdersAccountAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; CREATE_OPEN_ORDERS_ACCOUNT_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            payer: &arr[0],
            owner: &arr[1],
            delegate_account: &arr[2],
            open_orders_indexer: &arr[3],
            open_orders_account: &arr[4],
            market: &arr[5],
            system_program: &arr[6],
        }
    }
}
pub const CREATE_OPEN_ORDERS_ACCOUNT_IX_DISCM: [u8; 8usize] = [
    204, 181, 175, 222, 40, 125, 188, 71,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CreateOpenOrdersAccountIxArgs {
    pub name: String,
}
#[derive(Clone, Debug, PartialEq)]
pub struct CreateOpenOrdersAccountIxData(pub CreateOpenOrdersAccountIxArgs);
impl From<CreateOpenOrdersAccountIxArgs> for CreateOpenOrdersAccountIxData {
    fn from(args: CreateOpenOrdersAccountIxArgs) -> Self {
        Self(args)
    }
}
impl CreateOpenOrdersAccountIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CREATE_OPEN_ORDERS_ACCOUNT_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let name: String = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(CreateOpenOrdersAccountIxArgs {
                name,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CREATE_OPEN_ORDERS_ACCOUNT_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.name, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn create_open_orders_account_ix_with_program_id(
    program_id: Pubkey,
    keys: CreateOpenOrdersAccountKeys,
    args: CreateOpenOrdersAccountIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CREATE_OPEN_ORDERS_ACCOUNT_IX_ACCOUNTS_LEN] = keys.into();
    let data: CreateOpenOrdersAccountIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn create_open_orders_account_ix(
    keys: CreateOpenOrdersAccountKeys,
    args: CreateOpenOrdersAccountIxArgs,
) -> std::io::Result<Instruction> {
    create_open_orders_account_ix_with_program_id(OPENBOOK_V2_PROGRAM_ID, keys, args)
}
pub fn create_open_orders_account_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CreateOpenOrdersAccountAccounts<'_, '_>,
    args: CreateOpenOrdersAccountIxArgs,
) -> ProgramResult {
    let keys: CreateOpenOrdersAccountKeys = accounts.into();
    let ix = create_open_orders_account_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn create_open_orders_account_invoke(
    accounts: CreateOpenOrdersAccountAccounts<'_, '_>,
    args: CreateOpenOrdersAccountIxArgs,
) -> ProgramResult {
    create_open_orders_account_invoke_with_program_id(
        OPENBOOK_V2_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn create_open_orders_account_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CreateOpenOrdersAccountAccounts<'_, '_>,
    args: CreateOpenOrdersAccountIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CreateOpenOrdersAccountKeys = accounts.into();
    let ix = create_open_orders_account_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn create_open_orders_account_invoke_signed(
    accounts: CreateOpenOrdersAccountAccounts<'_, '_>,
    args: CreateOpenOrdersAccountIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    create_open_orders_account_invoke_signed_with_program_id(
        OPENBOOK_V2_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn create_open_orders_account_verify_account_keys(
    accounts: CreateOpenOrdersAccountAccounts<'_, '_>,
    keys: CreateOpenOrdersAccountKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.payer.key, keys.payer),
        (*accounts.owner.key, keys.owner),
        (*accounts.delegate_account.key, keys.delegate_account),
        (*accounts.open_orders_indexer.key, keys.open_orders_indexer),
        (*accounts.open_orders_account.key, keys.open_orders_account),
        (*accounts.market.key, keys.market),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn create_open_orders_account_verify_writable_privileges<'me, 'info>(
    accounts: CreateOpenOrdersAccountAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.payer,
        accounts.open_orders_indexer,
        accounts.open_orders_account,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn create_open_orders_account_verify_signer_privileges<'me, 'info>(
    accounts: CreateOpenOrdersAccountAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.payer, accounts.owner] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn create_open_orders_account_verify_account_privileges<'me, 'info>(
    accounts: CreateOpenOrdersAccountAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    create_open_orders_account_verify_writable_privileges(accounts)?;
    create_open_orders_account_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CLOSE_OPEN_ORDERS_ACCOUNT_IX_ACCOUNTS_LEN: usize = 5;
#[derive(Copy, Clone, Debug)]
pub struct CloseOpenOrdersAccountAccounts<'me, 'info> {
    pub owner: &'me AccountInfo<'info>,
    pub open_orders_indexer: &'me AccountInfo<'info>,
    pub open_orders_account: &'me AccountInfo<'info>,
    pub sol_destination: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CloseOpenOrdersAccountKeys {
    pub owner: Pubkey,
    pub open_orders_indexer: Pubkey,
    pub open_orders_account: Pubkey,
    pub sol_destination: Pubkey,
    pub system_program: Pubkey,
}
impl From<CloseOpenOrdersAccountAccounts<'_, '_>> for CloseOpenOrdersAccountKeys {
    fn from(accounts: CloseOpenOrdersAccountAccounts) -> Self {
        Self {
            owner: *accounts.owner.key,
            open_orders_indexer: *accounts.open_orders_indexer.key,
            open_orders_account: *accounts.open_orders_account.key,
            sol_destination: *accounts.sol_destination.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<CloseOpenOrdersAccountKeys>
for [AccountMeta; CLOSE_OPEN_ORDERS_ACCOUNT_IX_ACCOUNTS_LEN] {
    fn from(keys: CloseOpenOrdersAccountKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.owner,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.open_orders_indexer,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.open_orders_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.sol_destination,
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
impl From<[Pubkey; CLOSE_OPEN_ORDERS_ACCOUNT_IX_ACCOUNTS_LEN]>
for CloseOpenOrdersAccountKeys {
    fn from(pubkeys: [Pubkey; CLOSE_OPEN_ORDERS_ACCOUNT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            owner: pubkeys[0],
            open_orders_indexer: pubkeys[1],
            open_orders_account: pubkeys[2],
            sol_destination: pubkeys[3],
            system_program: pubkeys[4],
        }
    }
}
impl<'info> From<CloseOpenOrdersAccountAccounts<'_, 'info>>
for [AccountInfo<'info>; CLOSE_OPEN_ORDERS_ACCOUNT_IX_ACCOUNTS_LEN] {
    fn from(accounts: CloseOpenOrdersAccountAccounts<'_, 'info>) -> Self {
        [
            accounts.owner.clone(),
            accounts.open_orders_indexer.clone(),
            accounts.open_orders_account.clone(),
            accounts.sol_destination.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; CLOSE_OPEN_ORDERS_ACCOUNT_IX_ACCOUNTS_LEN]>
for CloseOpenOrdersAccountAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; CLOSE_OPEN_ORDERS_ACCOUNT_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            owner: &arr[0],
            open_orders_indexer: &arr[1],
            open_orders_account: &arr[2],
            sol_destination: &arr[3],
            system_program: &arr[4],
        }
    }
}
pub const CLOSE_OPEN_ORDERS_ACCOUNT_IX_DISCM: [u8; 8usize] = [
    176, 74, 115, 210, 54, 179, 91, 103,
];
#[derive(Clone, Debug, PartialEq)]
pub struct CloseOpenOrdersAccountIxData;
impl CloseOpenOrdersAccountIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CLOSE_OPEN_ORDERS_ACCOUNT_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CLOSE_OPEN_ORDERS_ACCOUNT_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn close_open_orders_account_ix_with_program_id(
    program_id: Pubkey,
    keys: CloseOpenOrdersAccountKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CLOSE_OPEN_ORDERS_ACCOUNT_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: CloseOpenOrdersAccountIxData.try_to_vec()?,
    })
}
pub fn close_open_orders_account_ix(
    keys: CloseOpenOrdersAccountKeys,
) -> std::io::Result<Instruction> {
    close_open_orders_account_ix_with_program_id(OPENBOOK_V2_PROGRAM_ID, keys)
}
pub fn close_open_orders_account_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CloseOpenOrdersAccountAccounts<'_, '_>,
) -> ProgramResult {
    let keys: CloseOpenOrdersAccountKeys = accounts.into();
    let ix = close_open_orders_account_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn close_open_orders_account_invoke(
    accounts: CloseOpenOrdersAccountAccounts<'_, '_>,
) -> ProgramResult {
    close_open_orders_account_invoke_with_program_id(OPENBOOK_V2_PROGRAM_ID, accounts)
}
pub fn close_open_orders_account_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CloseOpenOrdersAccountAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CloseOpenOrdersAccountKeys = accounts.into();
    let ix = close_open_orders_account_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn close_open_orders_account_invoke_signed(
    accounts: CloseOpenOrdersAccountAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    close_open_orders_account_invoke_signed_with_program_id(
        OPENBOOK_V2_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn close_open_orders_account_verify_account_keys(
    accounts: CloseOpenOrdersAccountAccounts<'_, '_>,
    keys: CloseOpenOrdersAccountKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.owner.key, keys.owner),
        (*accounts.open_orders_indexer.key, keys.open_orders_indexer),
        (*accounts.open_orders_account.key, keys.open_orders_account),
        (*accounts.sol_destination.key, keys.sol_destination),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn close_open_orders_account_verify_writable_privileges<'me, 'info>(
    accounts: CloseOpenOrdersAccountAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.open_orders_indexer,
        accounts.open_orders_account,
        accounts.sol_destination,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn close_open_orders_account_verify_signer_privileges<'me, 'info>(
    accounts: CloseOpenOrdersAccountAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.owner] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn close_open_orders_account_verify_account_privileges<'me, 'info>(
    accounts: CloseOpenOrdersAccountAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    close_open_orders_account_verify_writable_privileges(accounts)?;
    close_open_orders_account_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const PLACE_ORDER_IX_ACCOUNTS_LEN: usize = 12;
#[derive(Copy, Clone, Debug)]
pub struct PlaceOrderAccounts<'me, 'info> {
    pub signer: &'me AccountInfo<'info>,
    pub open_orders_account: &'me AccountInfo<'info>,
    pub open_orders_admin: &'me AccountInfo<'info>,
    pub user_token_account: &'me AccountInfo<'info>,
    pub market: &'me AccountInfo<'info>,
    pub bids: &'me AccountInfo<'info>,
    pub asks: &'me AccountInfo<'info>,
    pub event_heap: &'me AccountInfo<'info>,
    pub market_vault: &'me AccountInfo<'info>,
    pub oracle_a: &'me AccountInfo<'info>,
    pub oracle_b: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct PlaceOrderKeys {
    pub signer: Pubkey,
    pub open_orders_account: Pubkey,
    pub open_orders_admin: Pubkey,
    pub user_token_account: Pubkey,
    pub market: Pubkey,
    pub bids: Pubkey,
    pub asks: Pubkey,
    pub event_heap: Pubkey,
    pub market_vault: Pubkey,
    pub oracle_a: Pubkey,
    pub oracle_b: Pubkey,
    pub token_program: Pubkey,
}
impl From<PlaceOrderAccounts<'_, '_>> for PlaceOrderKeys {
    fn from(accounts: PlaceOrderAccounts) -> Self {
        Self {
            signer: *accounts.signer.key,
            open_orders_account: *accounts.open_orders_account.key,
            open_orders_admin: *accounts.open_orders_admin.key,
            user_token_account: *accounts.user_token_account.key,
            market: *accounts.market.key,
            bids: *accounts.bids.key,
            asks: *accounts.asks.key,
            event_heap: *accounts.event_heap.key,
            market_vault: *accounts.market_vault.key,
            oracle_a: *accounts.oracle_a.key,
            oracle_b: *accounts.oracle_b.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<PlaceOrderKeys> for [AccountMeta; PLACE_ORDER_IX_ACCOUNTS_LEN] {
    fn from(keys: PlaceOrderKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.signer,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.open_orders_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.open_orders_admin,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.user_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.market,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.bids,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.asks,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.event_heap,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.market_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.oracle_a,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.oracle_b,
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
impl From<[Pubkey; PLACE_ORDER_IX_ACCOUNTS_LEN]> for PlaceOrderKeys {
    fn from(pubkeys: [Pubkey; PLACE_ORDER_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            signer: pubkeys[0],
            open_orders_account: pubkeys[1],
            open_orders_admin: pubkeys[2],
            user_token_account: pubkeys[3],
            market: pubkeys[4],
            bids: pubkeys[5],
            asks: pubkeys[6],
            event_heap: pubkeys[7],
            market_vault: pubkeys[8],
            oracle_a: pubkeys[9],
            oracle_b: pubkeys[10],
            token_program: pubkeys[11],
        }
    }
}
impl<'info> From<PlaceOrderAccounts<'_, 'info>>
for [AccountInfo<'info>; PLACE_ORDER_IX_ACCOUNTS_LEN] {
    fn from(accounts: PlaceOrderAccounts<'_, 'info>) -> Self {
        [
            accounts.signer.clone(),
            accounts.open_orders_account.clone(),
            accounts.open_orders_admin.clone(),
            accounts.user_token_account.clone(),
            accounts.market.clone(),
            accounts.bids.clone(),
            accounts.asks.clone(),
            accounts.event_heap.clone(),
            accounts.market_vault.clone(),
            accounts.oracle_a.clone(),
            accounts.oracle_b.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; PLACE_ORDER_IX_ACCOUNTS_LEN]>
for PlaceOrderAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; PLACE_ORDER_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            signer: &arr[0],
            open_orders_account: &arr[1],
            open_orders_admin: &arr[2],
            user_token_account: &arr[3],
            market: &arr[4],
            bids: &arr[5],
            asks: &arr[6],
            event_heap: &arr[7],
            market_vault: &arr[8],
            oracle_a: &arr[9],
            oracle_b: &arr[10],
            token_program: &arr[11],
        }
    }
}
pub const PLACE_ORDER_IX_DISCM: [u8; 8usize] = [51, 194, 155, 175, 109, 130, 96, 106];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PlaceOrderIxArgs {
    pub args: PlaceOrderArgs,
}
#[derive(Clone, Debug, PartialEq)]
pub struct PlaceOrderIxData(pub PlaceOrderIxArgs);
impl From<PlaceOrderIxArgs> for PlaceOrderIxData {
    fn from(args: PlaceOrderIxArgs) -> Self {
        Self(args)
    }
}
impl PlaceOrderIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != PLACE_ORDER_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let args = if reader.is_empty() {
            Default::default()
        } else {
            <PlaceOrderArgs>::deserialize(&mut reader)?
        };
        Ok(Self(PlaceOrderIxArgs { args }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&PLACE_ORDER_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.args, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn place_order_ix_with_program_id(
    program_id: Pubkey,
    keys: PlaceOrderKeys,
    args: PlaceOrderIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; PLACE_ORDER_IX_ACCOUNTS_LEN] = keys.into();
    let data: PlaceOrderIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn place_order_ix(
    keys: PlaceOrderKeys,
    args: PlaceOrderIxArgs,
) -> std::io::Result<Instruction> {
    place_order_ix_with_program_id(OPENBOOK_V2_PROGRAM_ID, keys, args)
}
pub fn place_order_invoke_with_program_id(
    program_id: Pubkey,
    accounts: PlaceOrderAccounts<'_, '_>,
    args: PlaceOrderIxArgs,
) -> ProgramResult {
    let keys: PlaceOrderKeys = accounts.into();
    let ix = place_order_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn place_order_invoke(
    accounts: PlaceOrderAccounts<'_, '_>,
    args: PlaceOrderIxArgs,
) -> ProgramResult {
    place_order_invoke_with_program_id(OPENBOOK_V2_PROGRAM_ID, accounts, args)
}
pub fn place_order_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: PlaceOrderAccounts<'_, '_>,
    args: PlaceOrderIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: PlaceOrderKeys = accounts.into();
    let ix = place_order_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn place_order_invoke_signed(
    accounts: PlaceOrderAccounts<'_, '_>,
    args: PlaceOrderIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    place_order_invoke_signed_with_program_id(
        OPENBOOK_V2_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn place_order_verify_account_keys(
    accounts: PlaceOrderAccounts<'_, '_>,
    keys: PlaceOrderKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.signer.key, keys.signer),
        (*accounts.open_orders_account.key, keys.open_orders_account),
        (*accounts.open_orders_admin.key, keys.open_orders_admin),
        (*accounts.user_token_account.key, keys.user_token_account),
        (*accounts.market.key, keys.market),
        (*accounts.bids.key, keys.bids),
        (*accounts.asks.key, keys.asks),
        (*accounts.event_heap.key, keys.event_heap),
        (*accounts.market_vault.key, keys.market_vault),
        (*accounts.oracle_a.key, keys.oracle_a),
        (*accounts.oracle_b.key, keys.oracle_b),
        (*accounts.token_program.key, keys.token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn place_order_verify_writable_privileges<'me, 'info>(
    accounts: PlaceOrderAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.open_orders_account,
        accounts.user_token_account,
        accounts.market,
        accounts.bids,
        accounts.asks,
        accounts.event_heap,
        accounts.market_vault,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn place_order_verify_signer_privileges<'me, 'info>(
    accounts: PlaceOrderAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.signer, accounts.open_orders_admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn place_order_verify_account_privileges<'me, 'info>(
    accounts: PlaceOrderAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    place_order_verify_writable_privileges(accounts)?;
    place_order_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const EDIT_ORDER_IX_ACCOUNTS_LEN: usize = 12;
#[derive(Copy, Clone, Debug)]
pub struct EditOrderAccounts<'me, 'info> {
    pub signer: &'me AccountInfo<'info>,
    pub open_orders_account: &'me AccountInfo<'info>,
    pub open_orders_admin: &'me AccountInfo<'info>,
    pub user_token_account: &'me AccountInfo<'info>,
    pub market: &'me AccountInfo<'info>,
    pub bids: &'me AccountInfo<'info>,
    pub asks: &'me AccountInfo<'info>,
    pub event_heap: &'me AccountInfo<'info>,
    pub market_vault: &'me AccountInfo<'info>,
    pub oracle_a: &'me AccountInfo<'info>,
    pub oracle_b: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct EditOrderKeys {
    pub signer: Pubkey,
    pub open_orders_account: Pubkey,
    pub open_orders_admin: Pubkey,
    pub user_token_account: Pubkey,
    pub market: Pubkey,
    pub bids: Pubkey,
    pub asks: Pubkey,
    pub event_heap: Pubkey,
    pub market_vault: Pubkey,
    pub oracle_a: Pubkey,
    pub oracle_b: Pubkey,
    pub token_program: Pubkey,
}
impl From<EditOrderAccounts<'_, '_>> for EditOrderKeys {
    fn from(accounts: EditOrderAccounts) -> Self {
        Self {
            signer: *accounts.signer.key,
            open_orders_account: *accounts.open_orders_account.key,
            open_orders_admin: *accounts.open_orders_admin.key,
            user_token_account: *accounts.user_token_account.key,
            market: *accounts.market.key,
            bids: *accounts.bids.key,
            asks: *accounts.asks.key,
            event_heap: *accounts.event_heap.key,
            market_vault: *accounts.market_vault.key,
            oracle_a: *accounts.oracle_a.key,
            oracle_b: *accounts.oracle_b.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<EditOrderKeys> for [AccountMeta; EDIT_ORDER_IX_ACCOUNTS_LEN] {
    fn from(keys: EditOrderKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.signer,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.open_orders_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.open_orders_admin,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.user_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.market,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.bids,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.asks,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.event_heap,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.market_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.oracle_a,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.oracle_b,
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
impl From<[Pubkey; EDIT_ORDER_IX_ACCOUNTS_LEN]> for EditOrderKeys {
    fn from(pubkeys: [Pubkey; EDIT_ORDER_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            signer: pubkeys[0],
            open_orders_account: pubkeys[1],
            open_orders_admin: pubkeys[2],
            user_token_account: pubkeys[3],
            market: pubkeys[4],
            bids: pubkeys[5],
            asks: pubkeys[6],
            event_heap: pubkeys[7],
            market_vault: pubkeys[8],
            oracle_a: pubkeys[9],
            oracle_b: pubkeys[10],
            token_program: pubkeys[11],
        }
    }
}
impl<'info> From<EditOrderAccounts<'_, 'info>>
for [AccountInfo<'info>; EDIT_ORDER_IX_ACCOUNTS_LEN] {
    fn from(accounts: EditOrderAccounts<'_, 'info>) -> Self {
        [
            accounts.signer.clone(),
            accounts.open_orders_account.clone(),
            accounts.open_orders_admin.clone(),
            accounts.user_token_account.clone(),
            accounts.market.clone(),
            accounts.bids.clone(),
            accounts.asks.clone(),
            accounts.event_heap.clone(),
            accounts.market_vault.clone(),
            accounts.oracle_a.clone(),
            accounts.oracle_b.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; EDIT_ORDER_IX_ACCOUNTS_LEN]>
for EditOrderAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; EDIT_ORDER_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            signer: &arr[0],
            open_orders_account: &arr[1],
            open_orders_admin: &arr[2],
            user_token_account: &arr[3],
            market: &arr[4],
            bids: &arr[5],
            asks: &arr[6],
            event_heap: &arr[7],
            market_vault: &arr[8],
            oracle_a: &arr[9],
            oracle_b: &arr[10],
            token_program: &arr[11],
        }
    }
}
pub const EDIT_ORDER_IX_DISCM: [u8; 8usize] = [254, 208, 118, 29, 173, 248, 200, 70];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct EditOrderIxArgs {
    pub client_order_id: u64,
    pub expected_cancel_size: i64,
    pub place_order: PlaceOrderArgs,
}
#[derive(Clone, Debug, PartialEq)]
pub struct EditOrderIxData(pub EditOrderIxArgs);
impl From<EditOrderIxArgs> for EditOrderIxData {
    fn from(args: EditOrderIxArgs) -> Self {
        Self(args)
    }
}
impl EditOrderIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != EDIT_ORDER_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let client_order_id: u64 = crate::borsh_de_or_default(&mut reader)?;
        let expected_cancel_size: i64 = crate::borsh_de_or_default(&mut reader)?;
        let place_order = if reader.is_empty() {
            Default::default()
        } else {
            <PlaceOrderArgs>::deserialize(&mut reader)?
        };
        Ok(
            Self(EditOrderIxArgs {
                client_order_id,
                expected_cancel_size,
                place_order,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&EDIT_ORDER_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.client_order_id, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.expected_cancel_size, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.place_order, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn edit_order_ix_with_program_id(
    program_id: Pubkey,
    keys: EditOrderKeys,
    args: EditOrderIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; EDIT_ORDER_IX_ACCOUNTS_LEN] = keys.into();
    let data: EditOrderIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn edit_order_ix(
    keys: EditOrderKeys,
    args: EditOrderIxArgs,
) -> std::io::Result<Instruction> {
    edit_order_ix_with_program_id(OPENBOOK_V2_PROGRAM_ID, keys, args)
}
pub fn edit_order_invoke_with_program_id(
    program_id: Pubkey,
    accounts: EditOrderAccounts<'_, '_>,
    args: EditOrderIxArgs,
) -> ProgramResult {
    let keys: EditOrderKeys = accounts.into();
    let ix = edit_order_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn edit_order_invoke(
    accounts: EditOrderAccounts<'_, '_>,
    args: EditOrderIxArgs,
) -> ProgramResult {
    edit_order_invoke_with_program_id(OPENBOOK_V2_PROGRAM_ID, accounts, args)
}
pub fn edit_order_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: EditOrderAccounts<'_, '_>,
    args: EditOrderIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: EditOrderKeys = accounts.into();
    let ix = edit_order_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn edit_order_invoke_signed(
    accounts: EditOrderAccounts<'_, '_>,
    args: EditOrderIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    edit_order_invoke_signed_with_program_id(
        OPENBOOK_V2_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn edit_order_verify_account_keys(
    accounts: EditOrderAccounts<'_, '_>,
    keys: EditOrderKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.signer.key, keys.signer),
        (*accounts.open_orders_account.key, keys.open_orders_account),
        (*accounts.open_orders_admin.key, keys.open_orders_admin),
        (*accounts.user_token_account.key, keys.user_token_account),
        (*accounts.market.key, keys.market),
        (*accounts.bids.key, keys.bids),
        (*accounts.asks.key, keys.asks),
        (*accounts.event_heap.key, keys.event_heap),
        (*accounts.market_vault.key, keys.market_vault),
        (*accounts.oracle_a.key, keys.oracle_a),
        (*accounts.oracle_b.key, keys.oracle_b),
        (*accounts.token_program.key, keys.token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn edit_order_verify_writable_privileges<'me, 'info>(
    accounts: EditOrderAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.open_orders_account,
        accounts.user_token_account,
        accounts.market,
        accounts.bids,
        accounts.asks,
        accounts.event_heap,
        accounts.market_vault,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn edit_order_verify_signer_privileges<'me, 'info>(
    accounts: EditOrderAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.signer, accounts.open_orders_admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn edit_order_verify_account_privileges<'me, 'info>(
    accounts: EditOrderAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    edit_order_verify_writable_privileges(accounts)?;
    edit_order_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const EDIT_ORDER_PEGGED_IX_ACCOUNTS_LEN: usize = 12;
#[derive(Copy, Clone, Debug)]
pub struct EditOrderPeggedAccounts<'me, 'info> {
    pub signer: &'me AccountInfo<'info>,
    pub open_orders_account: &'me AccountInfo<'info>,
    pub open_orders_admin: &'me AccountInfo<'info>,
    pub user_token_account: &'me AccountInfo<'info>,
    pub market: &'me AccountInfo<'info>,
    pub bids: &'me AccountInfo<'info>,
    pub asks: &'me AccountInfo<'info>,
    pub event_heap: &'me AccountInfo<'info>,
    pub market_vault: &'me AccountInfo<'info>,
    pub oracle_a: &'me AccountInfo<'info>,
    pub oracle_b: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct EditOrderPeggedKeys {
    pub signer: Pubkey,
    pub open_orders_account: Pubkey,
    pub open_orders_admin: Pubkey,
    pub user_token_account: Pubkey,
    pub market: Pubkey,
    pub bids: Pubkey,
    pub asks: Pubkey,
    pub event_heap: Pubkey,
    pub market_vault: Pubkey,
    pub oracle_a: Pubkey,
    pub oracle_b: Pubkey,
    pub token_program: Pubkey,
}
impl From<EditOrderPeggedAccounts<'_, '_>> for EditOrderPeggedKeys {
    fn from(accounts: EditOrderPeggedAccounts) -> Self {
        Self {
            signer: *accounts.signer.key,
            open_orders_account: *accounts.open_orders_account.key,
            open_orders_admin: *accounts.open_orders_admin.key,
            user_token_account: *accounts.user_token_account.key,
            market: *accounts.market.key,
            bids: *accounts.bids.key,
            asks: *accounts.asks.key,
            event_heap: *accounts.event_heap.key,
            market_vault: *accounts.market_vault.key,
            oracle_a: *accounts.oracle_a.key,
            oracle_b: *accounts.oracle_b.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<EditOrderPeggedKeys> for [AccountMeta; EDIT_ORDER_PEGGED_IX_ACCOUNTS_LEN] {
    fn from(keys: EditOrderPeggedKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.signer,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.open_orders_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.open_orders_admin,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.user_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.market,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.bids,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.asks,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.event_heap,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.market_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.oracle_a,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.oracle_b,
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
impl From<[Pubkey; EDIT_ORDER_PEGGED_IX_ACCOUNTS_LEN]> for EditOrderPeggedKeys {
    fn from(pubkeys: [Pubkey; EDIT_ORDER_PEGGED_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            signer: pubkeys[0],
            open_orders_account: pubkeys[1],
            open_orders_admin: pubkeys[2],
            user_token_account: pubkeys[3],
            market: pubkeys[4],
            bids: pubkeys[5],
            asks: pubkeys[6],
            event_heap: pubkeys[7],
            market_vault: pubkeys[8],
            oracle_a: pubkeys[9],
            oracle_b: pubkeys[10],
            token_program: pubkeys[11],
        }
    }
}
impl<'info> From<EditOrderPeggedAccounts<'_, 'info>>
for [AccountInfo<'info>; EDIT_ORDER_PEGGED_IX_ACCOUNTS_LEN] {
    fn from(accounts: EditOrderPeggedAccounts<'_, 'info>) -> Self {
        [
            accounts.signer.clone(),
            accounts.open_orders_account.clone(),
            accounts.open_orders_admin.clone(),
            accounts.user_token_account.clone(),
            accounts.market.clone(),
            accounts.bids.clone(),
            accounts.asks.clone(),
            accounts.event_heap.clone(),
            accounts.market_vault.clone(),
            accounts.oracle_a.clone(),
            accounts.oracle_b.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; EDIT_ORDER_PEGGED_IX_ACCOUNTS_LEN]>
for EditOrderPeggedAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; EDIT_ORDER_PEGGED_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            signer: &arr[0],
            open_orders_account: &arr[1],
            open_orders_admin: &arr[2],
            user_token_account: &arr[3],
            market: &arr[4],
            bids: &arr[5],
            asks: &arr[6],
            event_heap: &arr[7],
            market_vault: &arr[8],
            oracle_a: &arr[9],
            oracle_b: &arr[10],
            token_program: &arr[11],
        }
    }
}
pub const EDIT_ORDER_PEGGED_IX_DISCM: [u8; 8usize] = [
    62, 187, 125, 69, 26, 221, 157, 133,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct EditOrderPeggedIxArgs {
    pub client_order_id: u64,
    pub expected_cancel_size: i64,
    pub place_order: PlaceOrderPeggedArgs,
}
#[derive(Clone, Debug, PartialEq)]
pub struct EditOrderPeggedIxData(pub EditOrderPeggedIxArgs);
impl From<EditOrderPeggedIxArgs> for EditOrderPeggedIxData {
    fn from(args: EditOrderPeggedIxArgs) -> Self {
        Self(args)
    }
}
impl EditOrderPeggedIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != EDIT_ORDER_PEGGED_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let client_order_id: u64 = crate::borsh_de_or_default(&mut reader)?;
        let expected_cancel_size: i64 = crate::borsh_de_or_default(&mut reader)?;
        let place_order = if reader.is_empty() {
            Default::default()
        } else {
            <PlaceOrderPeggedArgs>::deserialize(&mut reader)?
        };
        Ok(
            Self(EditOrderPeggedIxArgs {
                client_order_id,
                expected_cancel_size,
                place_order,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&EDIT_ORDER_PEGGED_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.client_order_id, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.expected_cancel_size, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.place_order, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn edit_order_pegged_ix_with_program_id(
    program_id: Pubkey,
    keys: EditOrderPeggedKeys,
    args: EditOrderPeggedIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; EDIT_ORDER_PEGGED_IX_ACCOUNTS_LEN] = keys.into();
    let data: EditOrderPeggedIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn edit_order_pegged_ix(
    keys: EditOrderPeggedKeys,
    args: EditOrderPeggedIxArgs,
) -> std::io::Result<Instruction> {
    edit_order_pegged_ix_with_program_id(OPENBOOK_V2_PROGRAM_ID, keys, args)
}
pub fn edit_order_pegged_invoke_with_program_id(
    program_id: Pubkey,
    accounts: EditOrderPeggedAccounts<'_, '_>,
    args: EditOrderPeggedIxArgs,
) -> ProgramResult {
    let keys: EditOrderPeggedKeys = accounts.into();
    let ix = edit_order_pegged_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn edit_order_pegged_invoke(
    accounts: EditOrderPeggedAccounts<'_, '_>,
    args: EditOrderPeggedIxArgs,
) -> ProgramResult {
    edit_order_pegged_invoke_with_program_id(OPENBOOK_V2_PROGRAM_ID, accounts, args)
}
pub fn edit_order_pegged_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: EditOrderPeggedAccounts<'_, '_>,
    args: EditOrderPeggedIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: EditOrderPeggedKeys = accounts.into();
    let ix = edit_order_pegged_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn edit_order_pegged_invoke_signed(
    accounts: EditOrderPeggedAccounts<'_, '_>,
    args: EditOrderPeggedIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    edit_order_pegged_invoke_signed_with_program_id(
        OPENBOOK_V2_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn edit_order_pegged_verify_account_keys(
    accounts: EditOrderPeggedAccounts<'_, '_>,
    keys: EditOrderPeggedKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.signer.key, keys.signer),
        (*accounts.open_orders_account.key, keys.open_orders_account),
        (*accounts.open_orders_admin.key, keys.open_orders_admin),
        (*accounts.user_token_account.key, keys.user_token_account),
        (*accounts.market.key, keys.market),
        (*accounts.bids.key, keys.bids),
        (*accounts.asks.key, keys.asks),
        (*accounts.event_heap.key, keys.event_heap),
        (*accounts.market_vault.key, keys.market_vault),
        (*accounts.oracle_a.key, keys.oracle_a),
        (*accounts.oracle_b.key, keys.oracle_b),
        (*accounts.token_program.key, keys.token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn edit_order_pegged_verify_writable_privileges<'me, 'info>(
    accounts: EditOrderPeggedAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.open_orders_account,
        accounts.user_token_account,
        accounts.market,
        accounts.bids,
        accounts.asks,
        accounts.event_heap,
        accounts.market_vault,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn edit_order_pegged_verify_signer_privileges<'me, 'info>(
    accounts: EditOrderPeggedAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.signer, accounts.open_orders_admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn edit_order_pegged_verify_account_privileges<'me, 'info>(
    accounts: EditOrderPeggedAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    edit_order_pegged_verify_writable_privileges(accounts)?;
    edit_order_pegged_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const PLACE_ORDERS_IX_ACCOUNTS_LEN: usize = 14;
#[derive(Copy, Clone, Debug)]
pub struct PlaceOrdersAccounts<'me, 'info> {
    pub signer: &'me AccountInfo<'info>,
    pub open_orders_account: &'me AccountInfo<'info>,
    pub open_orders_admin: &'me AccountInfo<'info>,
    pub user_quote_account: &'me AccountInfo<'info>,
    pub user_base_account: &'me AccountInfo<'info>,
    pub market: &'me AccountInfo<'info>,
    pub bids: &'me AccountInfo<'info>,
    pub asks: &'me AccountInfo<'info>,
    pub event_heap: &'me AccountInfo<'info>,
    pub market_quote_vault: &'me AccountInfo<'info>,
    pub market_base_vault: &'me AccountInfo<'info>,
    pub oracle_a: &'me AccountInfo<'info>,
    pub oracle_b: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct PlaceOrdersKeys {
    pub signer: Pubkey,
    pub open_orders_account: Pubkey,
    pub open_orders_admin: Pubkey,
    pub user_quote_account: Pubkey,
    pub user_base_account: Pubkey,
    pub market: Pubkey,
    pub bids: Pubkey,
    pub asks: Pubkey,
    pub event_heap: Pubkey,
    pub market_quote_vault: Pubkey,
    pub market_base_vault: Pubkey,
    pub oracle_a: Pubkey,
    pub oracle_b: Pubkey,
    pub token_program: Pubkey,
}
impl From<PlaceOrdersAccounts<'_, '_>> for PlaceOrdersKeys {
    fn from(accounts: PlaceOrdersAccounts) -> Self {
        Self {
            signer: *accounts.signer.key,
            open_orders_account: *accounts.open_orders_account.key,
            open_orders_admin: *accounts.open_orders_admin.key,
            user_quote_account: *accounts.user_quote_account.key,
            user_base_account: *accounts.user_base_account.key,
            market: *accounts.market.key,
            bids: *accounts.bids.key,
            asks: *accounts.asks.key,
            event_heap: *accounts.event_heap.key,
            market_quote_vault: *accounts.market_quote_vault.key,
            market_base_vault: *accounts.market_base_vault.key,
            oracle_a: *accounts.oracle_a.key,
            oracle_b: *accounts.oracle_b.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<PlaceOrdersKeys> for [AccountMeta; PLACE_ORDERS_IX_ACCOUNTS_LEN] {
    fn from(keys: PlaceOrdersKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.signer,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.open_orders_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.open_orders_admin,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.user_quote_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_base_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.market,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.bids,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.asks,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.event_heap,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.market_quote_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.market_base_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.oracle_a,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.oracle_b,
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
impl From<[Pubkey; PLACE_ORDERS_IX_ACCOUNTS_LEN]> for PlaceOrdersKeys {
    fn from(pubkeys: [Pubkey; PLACE_ORDERS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            signer: pubkeys[0],
            open_orders_account: pubkeys[1],
            open_orders_admin: pubkeys[2],
            user_quote_account: pubkeys[3],
            user_base_account: pubkeys[4],
            market: pubkeys[5],
            bids: pubkeys[6],
            asks: pubkeys[7],
            event_heap: pubkeys[8],
            market_quote_vault: pubkeys[9],
            market_base_vault: pubkeys[10],
            oracle_a: pubkeys[11],
            oracle_b: pubkeys[12],
            token_program: pubkeys[13],
        }
    }
}
impl<'info> From<PlaceOrdersAccounts<'_, 'info>>
for [AccountInfo<'info>; PLACE_ORDERS_IX_ACCOUNTS_LEN] {
    fn from(accounts: PlaceOrdersAccounts<'_, 'info>) -> Self {
        [
            accounts.signer.clone(),
            accounts.open_orders_account.clone(),
            accounts.open_orders_admin.clone(),
            accounts.user_quote_account.clone(),
            accounts.user_base_account.clone(),
            accounts.market.clone(),
            accounts.bids.clone(),
            accounts.asks.clone(),
            accounts.event_heap.clone(),
            accounts.market_quote_vault.clone(),
            accounts.market_base_vault.clone(),
            accounts.oracle_a.clone(),
            accounts.oracle_b.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; PLACE_ORDERS_IX_ACCOUNTS_LEN]>
for PlaceOrdersAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; PLACE_ORDERS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            signer: &arr[0],
            open_orders_account: &arr[1],
            open_orders_admin: &arr[2],
            user_quote_account: &arr[3],
            user_base_account: &arr[4],
            market: &arr[5],
            bids: &arr[6],
            asks: &arr[7],
            event_heap: &arr[8],
            market_quote_vault: &arr[9],
            market_base_vault: &arr[10],
            oracle_a: &arr[11],
            oracle_b: &arr[12],
            token_program: &arr[13],
        }
    }
}
pub const PLACE_ORDERS_IX_DISCM: [u8; 8usize] = [60, 63, 50, 123, 12, 197, 60, 190];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PlaceOrdersIxArgs {
    pub orders_type: PlaceOrderType,
    pub bids: Vec<PlaceMultipleOrdersArgs>,
    pub asks: Vec<PlaceMultipleOrdersArgs>,
    pub limit: u8,
}
#[derive(Clone, Debug, PartialEq)]
pub struct PlaceOrdersIxData(pub PlaceOrdersIxArgs);
impl From<PlaceOrdersIxArgs> for PlaceOrdersIxData {
    fn from(args: PlaceOrdersIxArgs) -> Self {
        Self(args)
    }
}
impl PlaceOrdersIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != PLACE_ORDERS_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let orders_type: PlaceOrderType = crate::borsh_de_or_default(&mut reader)?;
        let bids: Vec<PlaceMultipleOrdersArgs> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let asks: Vec<PlaceMultipleOrdersArgs> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let limit: u8 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(PlaceOrdersIxArgs {
                orders_type,
                bids,
                asks,
                limit,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&PLACE_ORDERS_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.orders_type, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.bids, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.asks, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.limit, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn place_orders_ix_with_program_id(
    program_id: Pubkey,
    keys: PlaceOrdersKeys,
    args: PlaceOrdersIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; PLACE_ORDERS_IX_ACCOUNTS_LEN] = keys.into();
    let data: PlaceOrdersIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn place_orders_ix(
    keys: PlaceOrdersKeys,
    args: PlaceOrdersIxArgs,
) -> std::io::Result<Instruction> {
    place_orders_ix_with_program_id(OPENBOOK_V2_PROGRAM_ID, keys, args)
}
pub fn place_orders_invoke_with_program_id(
    program_id: Pubkey,
    accounts: PlaceOrdersAccounts<'_, '_>,
    args: PlaceOrdersIxArgs,
) -> ProgramResult {
    let keys: PlaceOrdersKeys = accounts.into();
    let ix = place_orders_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn place_orders_invoke(
    accounts: PlaceOrdersAccounts<'_, '_>,
    args: PlaceOrdersIxArgs,
) -> ProgramResult {
    place_orders_invoke_with_program_id(OPENBOOK_V2_PROGRAM_ID, accounts, args)
}
pub fn place_orders_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: PlaceOrdersAccounts<'_, '_>,
    args: PlaceOrdersIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: PlaceOrdersKeys = accounts.into();
    let ix = place_orders_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn place_orders_invoke_signed(
    accounts: PlaceOrdersAccounts<'_, '_>,
    args: PlaceOrdersIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    place_orders_invoke_signed_with_program_id(
        OPENBOOK_V2_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn place_orders_verify_account_keys(
    accounts: PlaceOrdersAccounts<'_, '_>,
    keys: PlaceOrdersKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.signer.key, keys.signer),
        (*accounts.open_orders_account.key, keys.open_orders_account),
        (*accounts.open_orders_admin.key, keys.open_orders_admin),
        (*accounts.user_quote_account.key, keys.user_quote_account),
        (*accounts.user_base_account.key, keys.user_base_account),
        (*accounts.market.key, keys.market),
        (*accounts.bids.key, keys.bids),
        (*accounts.asks.key, keys.asks),
        (*accounts.event_heap.key, keys.event_heap),
        (*accounts.market_quote_vault.key, keys.market_quote_vault),
        (*accounts.market_base_vault.key, keys.market_base_vault),
        (*accounts.oracle_a.key, keys.oracle_a),
        (*accounts.oracle_b.key, keys.oracle_b),
        (*accounts.token_program.key, keys.token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn place_orders_verify_writable_privileges<'me, 'info>(
    accounts: PlaceOrdersAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.open_orders_account,
        accounts.user_quote_account,
        accounts.user_base_account,
        accounts.market,
        accounts.bids,
        accounts.asks,
        accounts.event_heap,
        accounts.market_quote_vault,
        accounts.market_base_vault,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn place_orders_verify_signer_privileges<'me, 'info>(
    accounts: PlaceOrdersAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.signer, accounts.open_orders_admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn place_orders_verify_account_privileges<'me, 'info>(
    accounts: PlaceOrdersAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    place_orders_verify_writable_privileges(accounts)?;
    place_orders_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CANCEL_ALL_AND_PLACE_ORDERS_IX_ACCOUNTS_LEN: usize = 14;
#[derive(Copy, Clone, Debug)]
pub struct CancelAllAndPlaceOrdersAccounts<'me, 'info> {
    pub signer: &'me AccountInfo<'info>,
    pub open_orders_account: &'me AccountInfo<'info>,
    pub open_orders_admin: &'me AccountInfo<'info>,
    pub user_quote_account: &'me AccountInfo<'info>,
    pub user_base_account: &'me AccountInfo<'info>,
    pub market: &'me AccountInfo<'info>,
    pub bids: &'me AccountInfo<'info>,
    pub asks: &'me AccountInfo<'info>,
    pub event_heap: &'me AccountInfo<'info>,
    pub market_quote_vault: &'me AccountInfo<'info>,
    pub market_base_vault: &'me AccountInfo<'info>,
    pub oracle_a: &'me AccountInfo<'info>,
    pub oracle_b: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CancelAllAndPlaceOrdersKeys {
    pub signer: Pubkey,
    pub open_orders_account: Pubkey,
    pub open_orders_admin: Pubkey,
    pub user_quote_account: Pubkey,
    pub user_base_account: Pubkey,
    pub market: Pubkey,
    pub bids: Pubkey,
    pub asks: Pubkey,
    pub event_heap: Pubkey,
    pub market_quote_vault: Pubkey,
    pub market_base_vault: Pubkey,
    pub oracle_a: Pubkey,
    pub oracle_b: Pubkey,
    pub token_program: Pubkey,
}
impl From<CancelAllAndPlaceOrdersAccounts<'_, '_>> for CancelAllAndPlaceOrdersKeys {
    fn from(accounts: CancelAllAndPlaceOrdersAccounts) -> Self {
        Self {
            signer: *accounts.signer.key,
            open_orders_account: *accounts.open_orders_account.key,
            open_orders_admin: *accounts.open_orders_admin.key,
            user_quote_account: *accounts.user_quote_account.key,
            user_base_account: *accounts.user_base_account.key,
            market: *accounts.market.key,
            bids: *accounts.bids.key,
            asks: *accounts.asks.key,
            event_heap: *accounts.event_heap.key,
            market_quote_vault: *accounts.market_quote_vault.key,
            market_base_vault: *accounts.market_base_vault.key,
            oracle_a: *accounts.oracle_a.key,
            oracle_b: *accounts.oracle_b.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<CancelAllAndPlaceOrdersKeys>
for [AccountMeta; CANCEL_ALL_AND_PLACE_ORDERS_IX_ACCOUNTS_LEN] {
    fn from(keys: CancelAllAndPlaceOrdersKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.signer,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.open_orders_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.open_orders_admin,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.user_quote_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_base_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.market,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.bids,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.asks,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.event_heap,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.market_quote_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.market_base_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.oracle_a,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.oracle_b,
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
impl From<[Pubkey; CANCEL_ALL_AND_PLACE_ORDERS_IX_ACCOUNTS_LEN]>
for CancelAllAndPlaceOrdersKeys {
    fn from(pubkeys: [Pubkey; CANCEL_ALL_AND_PLACE_ORDERS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            signer: pubkeys[0],
            open_orders_account: pubkeys[1],
            open_orders_admin: pubkeys[2],
            user_quote_account: pubkeys[3],
            user_base_account: pubkeys[4],
            market: pubkeys[5],
            bids: pubkeys[6],
            asks: pubkeys[7],
            event_heap: pubkeys[8],
            market_quote_vault: pubkeys[9],
            market_base_vault: pubkeys[10],
            oracle_a: pubkeys[11],
            oracle_b: pubkeys[12],
            token_program: pubkeys[13],
        }
    }
}
impl<'info> From<CancelAllAndPlaceOrdersAccounts<'_, 'info>>
for [AccountInfo<'info>; CANCEL_ALL_AND_PLACE_ORDERS_IX_ACCOUNTS_LEN] {
    fn from(accounts: CancelAllAndPlaceOrdersAccounts<'_, 'info>) -> Self {
        [
            accounts.signer.clone(),
            accounts.open_orders_account.clone(),
            accounts.open_orders_admin.clone(),
            accounts.user_quote_account.clone(),
            accounts.user_base_account.clone(),
            accounts.market.clone(),
            accounts.bids.clone(),
            accounts.asks.clone(),
            accounts.event_heap.clone(),
            accounts.market_quote_vault.clone(),
            accounts.market_base_vault.clone(),
            accounts.oracle_a.clone(),
            accounts.oracle_b.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; CANCEL_ALL_AND_PLACE_ORDERS_IX_ACCOUNTS_LEN]>
for CancelAllAndPlaceOrdersAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; CANCEL_ALL_AND_PLACE_ORDERS_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            signer: &arr[0],
            open_orders_account: &arr[1],
            open_orders_admin: &arr[2],
            user_quote_account: &arr[3],
            user_base_account: &arr[4],
            market: &arr[5],
            bids: &arr[6],
            asks: &arr[7],
            event_heap: &arr[8],
            market_quote_vault: &arr[9],
            market_base_vault: &arr[10],
            oracle_a: &arr[11],
            oracle_b: &arr[12],
            token_program: &arr[13],
        }
    }
}
pub const CANCEL_ALL_AND_PLACE_ORDERS_IX_DISCM: [u8; 8usize] = [
    128, 155, 222, 60, 186, 40, 225, 50,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CancelAllAndPlaceOrdersIxArgs {
    pub orders_type: PlaceOrderType,
    pub bids: Vec<PlaceMultipleOrdersArgs>,
    pub asks: Vec<PlaceMultipleOrdersArgs>,
    pub limit: u8,
}
#[derive(Clone, Debug, PartialEq)]
pub struct CancelAllAndPlaceOrdersIxData(pub CancelAllAndPlaceOrdersIxArgs);
impl From<CancelAllAndPlaceOrdersIxArgs> for CancelAllAndPlaceOrdersIxData {
    fn from(args: CancelAllAndPlaceOrdersIxArgs) -> Self {
        Self(args)
    }
}
impl CancelAllAndPlaceOrdersIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CANCEL_ALL_AND_PLACE_ORDERS_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let orders_type: PlaceOrderType = crate::borsh_de_or_default(&mut reader)?;
        let bids: Vec<PlaceMultipleOrdersArgs> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let asks: Vec<PlaceMultipleOrdersArgs> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let limit: u8 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(CancelAllAndPlaceOrdersIxArgs {
                orders_type,
                bids,
                asks,
                limit,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CANCEL_ALL_AND_PLACE_ORDERS_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.orders_type, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.bids, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.asks, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.limit, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn cancel_all_and_place_orders_ix_with_program_id(
    program_id: Pubkey,
    keys: CancelAllAndPlaceOrdersKeys,
    args: CancelAllAndPlaceOrdersIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CANCEL_ALL_AND_PLACE_ORDERS_IX_ACCOUNTS_LEN] = keys.into();
    let data: CancelAllAndPlaceOrdersIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn cancel_all_and_place_orders_ix(
    keys: CancelAllAndPlaceOrdersKeys,
    args: CancelAllAndPlaceOrdersIxArgs,
) -> std::io::Result<Instruction> {
    cancel_all_and_place_orders_ix_with_program_id(OPENBOOK_V2_PROGRAM_ID, keys, args)
}
pub fn cancel_all_and_place_orders_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CancelAllAndPlaceOrdersAccounts<'_, '_>,
    args: CancelAllAndPlaceOrdersIxArgs,
) -> ProgramResult {
    let keys: CancelAllAndPlaceOrdersKeys = accounts.into();
    let ix = cancel_all_and_place_orders_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn cancel_all_and_place_orders_invoke(
    accounts: CancelAllAndPlaceOrdersAccounts<'_, '_>,
    args: CancelAllAndPlaceOrdersIxArgs,
) -> ProgramResult {
    cancel_all_and_place_orders_invoke_with_program_id(
        OPENBOOK_V2_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn cancel_all_and_place_orders_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CancelAllAndPlaceOrdersAccounts<'_, '_>,
    args: CancelAllAndPlaceOrdersIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CancelAllAndPlaceOrdersKeys = accounts.into();
    let ix = cancel_all_and_place_orders_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn cancel_all_and_place_orders_invoke_signed(
    accounts: CancelAllAndPlaceOrdersAccounts<'_, '_>,
    args: CancelAllAndPlaceOrdersIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    cancel_all_and_place_orders_invoke_signed_with_program_id(
        OPENBOOK_V2_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn cancel_all_and_place_orders_verify_account_keys(
    accounts: CancelAllAndPlaceOrdersAccounts<'_, '_>,
    keys: CancelAllAndPlaceOrdersKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.signer.key, keys.signer),
        (*accounts.open_orders_account.key, keys.open_orders_account),
        (*accounts.open_orders_admin.key, keys.open_orders_admin),
        (*accounts.user_quote_account.key, keys.user_quote_account),
        (*accounts.user_base_account.key, keys.user_base_account),
        (*accounts.market.key, keys.market),
        (*accounts.bids.key, keys.bids),
        (*accounts.asks.key, keys.asks),
        (*accounts.event_heap.key, keys.event_heap),
        (*accounts.market_quote_vault.key, keys.market_quote_vault),
        (*accounts.market_base_vault.key, keys.market_base_vault),
        (*accounts.oracle_a.key, keys.oracle_a),
        (*accounts.oracle_b.key, keys.oracle_b),
        (*accounts.token_program.key, keys.token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn cancel_all_and_place_orders_verify_writable_privileges<'me, 'info>(
    accounts: CancelAllAndPlaceOrdersAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.open_orders_account,
        accounts.user_quote_account,
        accounts.user_base_account,
        accounts.market,
        accounts.bids,
        accounts.asks,
        accounts.event_heap,
        accounts.market_quote_vault,
        accounts.market_base_vault,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn cancel_all_and_place_orders_verify_signer_privileges<'me, 'info>(
    accounts: CancelAllAndPlaceOrdersAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.signer, accounts.open_orders_admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn cancel_all_and_place_orders_verify_account_privileges<'me, 'info>(
    accounts: CancelAllAndPlaceOrdersAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    cancel_all_and_place_orders_verify_writable_privileges(accounts)?;
    cancel_all_and_place_orders_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const PLACE_ORDER_PEGGED_IX_ACCOUNTS_LEN: usize = 12;
#[derive(Copy, Clone, Debug)]
pub struct PlaceOrderPeggedAccounts<'me, 'info> {
    pub signer: &'me AccountInfo<'info>,
    pub open_orders_account: &'me AccountInfo<'info>,
    pub open_orders_admin: &'me AccountInfo<'info>,
    pub user_token_account: &'me AccountInfo<'info>,
    pub market: &'me AccountInfo<'info>,
    pub bids: &'me AccountInfo<'info>,
    pub asks: &'me AccountInfo<'info>,
    pub event_heap: &'me AccountInfo<'info>,
    pub market_vault: &'me AccountInfo<'info>,
    pub oracle_a: &'me AccountInfo<'info>,
    pub oracle_b: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct PlaceOrderPeggedKeys {
    pub signer: Pubkey,
    pub open_orders_account: Pubkey,
    pub open_orders_admin: Pubkey,
    pub user_token_account: Pubkey,
    pub market: Pubkey,
    pub bids: Pubkey,
    pub asks: Pubkey,
    pub event_heap: Pubkey,
    pub market_vault: Pubkey,
    pub oracle_a: Pubkey,
    pub oracle_b: Pubkey,
    pub token_program: Pubkey,
}
impl From<PlaceOrderPeggedAccounts<'_, '_>> for PlaceOrderPeggedKeys {
    fn from(accounts: PlaceOrderPeggedAccounts) -> Self {
        Self {
            signer: *accounts.signer.key,
            open_orders_account: *accounts.open_orders_account.key,
            open_orders_admin: *accounts.open_orders_admin.key,
            user_token_account: *accounts.user_token_account.key,
            market: *accounts.market.key,
            bids: *accounts.bids.key,
            asks: *accounts.asks.key,
            event_heap: *accounts.event_heap.key,
            market_vault: *accounts.market_vault.key,
            oracle_a: *accounts.oracle_a.key,
            oracle_b: *accounts.oracle_b.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<PlaceOrderPeggedKeys> for [AccountMeta; PLACE_ORDER_PEGGED_IX_ACCOUNTS_LEN] {
    fn from(keys: PlaceOrderPeggedKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.signer,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.open_orders_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.open_orders_admin,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.user_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.market,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.bids,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.asks,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.event_heap,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.market_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.oracle_a,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.oracle_b,
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
impl From<[Pubkey; PLACE_ORDER_PEGGED_IX_ACCOUNTS_LEN]> for PlaceOrderPeggedKeys {
    fn from(pubkeys: [Pubkey; PLACE_ORDER_PEGGED_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            signer: pubkeys[0],
            open_orders_account: pubkeys[1],
            open_orders_admin: pubkeys[2],
            user_token_account: pubkeys[3],
            market: pubkeys[4],
            bids: pubkeys[5],
            asks: pubkeys[6],
            event_heap: pubkeys[7],
            market_vault: pubkeys[8],
            oracle_a: pubkeys[9],
            oracle_b: pubkeys[10],
            token_program: pubkeys[11],
        }
    }
}
impl<'info> From<PlaceOrderPeggedAccounts<'_, 'info>>
for [AccountInfo<'info>; PLACE_ORDER_PEGGED_IX_ACCOUNTS_LEN] {
    fn from(accounts: PlaceOrderPeggedAccounts<'_, 'info>) -> Self {
        [
            accounts.signer.clone(),
            accounts.open_orders_account.clone(),
            accounts.open_orders_admin.clone(),
            accounts.user_token_account.clone(),
            accounts.market.clone(),
            accounts.bids.clone(),
            accounts.asks.clone(),
            accounts.event_heap.clone(),
            accounts.market_vault.clone(),
            accounts.oracle_a.clone(),
            accounts.oracle_b.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; PLACE_ORDER_PEGGED_IX_ACCOUNTS_LEN]>
for PlaceOrderPeggedAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; PLACE_ORDER_PEGGED_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            signer: &arr[0],
            open_orders_account: &arr[1],
            open_orders_admin: &arr[2],
            user_token_account: &arr[3],
            market: &arr[4],
            bids: &arr[5],
            asks: &arr[6],
            event_heap: &arr[7],
            market_vault: &arr[8],
            oracle_a: &arr[9],
            oracle_b: &arr[10],
            token_program: &arr[11],
        }
    }
}
pub const PLACE_ORDER_PEGGED_IX_DISCM: [u8; 8usize] = [
    141, 185, 251, 63, 74, 85, 210, 145,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PlaceOrderPeggedIxArgs {
    pub args: PlaceOrderPeggedArgs,
}
#[derive(Clone, Debug, PartialEq)]
pub struct PlaceOrderPeggedIxData(pub PlaceOrderPeggedIxArgs);
impl From<PlaceOrderPeggedIxArgs> for PlaceOrderPeggedIxData {
    fn from(args: PlaceOrderPeggedIxArgs) -> Self {
        Self(args)
    }
}
impl PlaceOrderPeggedIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != PLACE_ORDER_PEGGED_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let args = if reader.is_empty() {
            Default::default()
        } else {
            <PlaceOrderPeggedArgs>::deserialize(&mut reader)?
        };
        Ok(Self(PlaceOrderPeggedIxArgs { args }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&PLACE_ORDER_PEGGED_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.args, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn place_order_pegged_ix_with_program_id(
    program_id: Pubkey,
    keys: PlaceOrderPeggedKeys,
    args: PlaceOrderPeggedIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; PLACE_ORDER_PEGGED_IX_ACCOUNTS_LEN] = keys.into();
    let data: PlaceOrderPeggedIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn place_order_pegged_ix(
    keys: PlaceOrderPeggedKeys,
    args: PlaceOrderPeggedIxArgs,
) -> std::io::Result<Instruction> {
    place_order_pegged_ix_with_program_id(OPENBOOK_V2_PROGRAM_ID, keys, args)
}
pub fn place_order_pegged_invoke_with_program_id(
    program_id: Pubkey,
    accounts: PlaceOrderPeggedAccounts<'_, '_>,
    args: PlaceOrderPeggedIxArgs,
) -> ProgramResult {
    let keys: PlaceOrderPeggedKeys = accounts.into();
    let ix = place_order_pegged_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn place_order_pegged_invoke(
    accounts: PlaceOrderPeggedAccounts<'_, '_>,
    args: PlaceOrderPeggedIxArgs,
) -> ProgramResult {
    place_order_pegged_invoke_with_program_id(OPENBOOK_V2_PROGRAM_ID, accounts, args)
}
pub fn place_order_pegged_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: PlaceOrderPeggedAccounts<'_, '_>,
    args: PlaceOrderPeggedIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: PlaceOrderPeggedKeys = accounts.into();
    let ix = place_order_pegged_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn place_order_pegged_invoke_signed(
    accounts: PlaceOrderPeggedAccounts<'_, '_>,
    args: PlaceOrderPeggedIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    place_order_pegged_invoke_signed_with_program_id(
        OPENBOOK_V2_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn place_order_pegged_verify_account_keys(
    accounts: PlaceOrderPeggedAccounts<'_, '_>,
    keys: PlaceOrderPeggedKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.signer.key, keys.signer),
        (*accounts.open_orders_account.key, keys.open_orders_account),
        (*accounts.open_orders_admin.key, keys.open_orders_admin),
        (*accounts.user_token_account.key, keys.user_token_account),
        (*accounts.market.key, keys.market),
        (*accounts.bids.key, keys.bids),
        (*accounts.asks.key, keys.asks),
        (*accounts.event_heap.key, keys.event_heap),
        (*accounts.market_vault.key, keys.market_vault),
        (*accounts.oracle_a.key, keys.oracle_a),
        (*accounts.oracle_b.key, keys.oracle_b),
        (*accounts.token_program.key, keys.token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn place_order_pegged_verify_writable_privileges<'me, 'info>(
    accounts: PlaceOrderPeggedAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.open_orders_account,
        accounts.user_token_account,
        accounts.market,
        accounts.bids,
        accounts.asks,
        accounts.event_heap,
        accounts.market_vault,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn place_order_pegged_verify_signer_privileges<'me, 'info>(
    accounts: PlaceOrderPeggedAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.signer, accounts.open_orders_admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn place_order_pegged_verify_account_privileges<'me, 'info>(
    accounts: PlaceOrderPeggedAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    place_order_pegged_verify_writable_privileges(accounts)?;
    place_order_pegged_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const PLACE_TAKE_ORDER_IX_ACCOUNTS_LEN: usize = 16;
#[derive(Copy, Clone, Debug)]
pub struct PlaceTakeOrderAccounts<'me, 'info> {
    pub signer: &'me AccountInfo<'info>,
    pub penalty_payer: &'me AccountInfo<'info>,
    pub market: &'me AccountInfo<'info>,
    pub market_authority: &'me AccountInfo<'info>,
    pub bids: &'me AccountInfo<'info>,
    pub asks: &'me AccountInfo<'info>,
    pub market_base_vault: &'me AccountInfo<'info>,
    pub market_quote_vault: &'me AccountInfo<'info>,
    pub event_heap: &'me AccountInfo<'info>,
    pub user_base_account: &'me AccountInfo<'info>,
    pub user_quote_account: &'me AccountInfo<'info>,
    pub oracle_a: &'me AccountInfo<'info>,
    pub oracle_b: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub open_orders_admin: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct PlaceTakeOrderKeys {
    pub signer: Pubkey,
    pub penalty_payer: Pubkey,
    pub market: Pubkey,
    pub market_authority: Pubkey,
    pub bids: Pubkey,
    pub asks: Pubkey,
    pub market_base_vault: Pubkey,
    pub market_quote_vault: Pubkey,
    pub event_heap: Pubkey,
    pub user_base_account: Pubkey,
    pub user_quote_account: Pubkey,
    pub oracle_a: Pubkey,
    pub oracle_b: Pubkey,
    pub token_program: Pubkey,
    pub system_program: Pubkey,
    pub open_orders_admin: Pubkey,
}
impl From<PlaceTakeOrderAccounts<'_, '_>> for PlaceTakeOrderKeys {
    fn from(accounts: PlaceTakeOrderAccounts) -> Self {
        Self {
            signer: *accounts.signer.key,
            penalty_payer: *accounts.penalty_payer.key,
            market: *accounts.market.key,
            market_authority: *accounts.market_authority.key,
            bids: *accounts.bids.key,
            asks: *accounts.asks.key,
            market_base_vault: *accounts.market_base_vault.key,
            market_quote_vault: *accounts.market_quote_vault.key,
            event_heap: *accounts.event_heap.key,
            user_base_account: *accounts.user_base_account.key,
            user_quote_account: *accounts.user_quote_account.key,
            oracle_a: *accounts.oracle_a.key,
            oracle_b: *accounts.oracle_b.key,
            token_program: *accounts.token_program.key,
            system_program: *accounts.system_program.key,
            open_orders_admin: *accounts.open_orders_admin.key,
        }
    }
}
impl From<PlaceTakeOrderKeys> for [AccountMeta; PLACE_TAKE_ORDER_IX_ACCOUNTS_LEN] {
    fn from(keys: PlaceTakeOrderKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.signer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.penalty_payer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.market,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.market_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.bids,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.asks,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.market_base_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.market_quote_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.event_heap,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_base_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_quote_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.oracle_a,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.oracle_b,
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
                pubkey: keys.open_orders_admin,
                is_signer: true,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; PLACE_TAKE_ORDER_IX_ACCOUNTS_LEN]> for PlaceTakeOrderKeys {
    fn from(pubkeys: [Pubkey; PLACE_TAKE_ORDER_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            signer: pubkeys[0],
            penalty_payer: pubkeys[1],
            market: pubkeys[2],
            market_authority: pubkeys[3],
            bids: pubkeys[4],
            asks: pubkeys[5],
            market_base_vault: pubkeys[6],
            market_quote_vault: pubkeys[7],
            event_heap: pubkeys[8],
            user_base_account: pubkeys[9],
            user_quote_account: pubkeys[10],
            oracle_a: pubkeys[11],
            oracle_b: pubkeys[12],
            token_program: pubkeys[13],
            system_program: pubkeys[14],
            open_orders_admin: pubkeys[15],
        }
    }
}
impl<'info> From<PlaceTakeOrderAccounts<'_, 'info>>
for [AccountInfo<'info>; PLACE_TAKE_ORDER_IX_ACCOUNTS_LEN] {
    fn from(accounts: PlaceTakeOrderAccounts<'_, 'info>) -> Self {
        [
            accounts.signer.clone(),
            accounts.penalty_payer.clone(),
            accounts.market.clone(),
            accounts.market_authority.clone(),
            accounts.bids.clone(),
            accounts.asks.clone(),
            accounts.market_base_vault.clone(),
            accounts.market_quote_vault.clone(),
            accounts.event_heap.clone(),
            accounts.user_base_account.clone(),
            accounts.user_quote_account.clone(),
            accounts.oracle_a.clone(),
            accounts.oracle_b.clone(),
            accounts.token_program.clone(),
            accounts.system_program.clone(),
            accounts.open_orders_admin.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; PLACE_TAKE_ORDER_IX_ACCOUNTS_LEN]>
for PlaceTakeOrderAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; PLACE_TAKE_ORDER_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            signer: &arr[0],
            penalty_payer: &arr[1],
            market: &arr[2],
            market_authority: &arr[3],
            bids: &arr[4],
            asks: &arr[5],
            market_base_vault: &arr[6],
            market_quote_vault: &arr[7],
            event_heap: &arr[8],
            user_base_account: &arr[9],
            user_quote_account: &arr[10],
            oracle_a: &arr[11],
            oracle_b: &arr[12],
            token_program: &arr[13],
            system_program: &arr[14],
            open_orders_admin: &arr[15],
        }
    }
}
pub const PLACE_TAKE_ORDER_IX_DISCM: [u8; 8usize] = [3, 44, 71, 3, 26, 199, 203, 85];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PlaceTakeOrderIxArgs {
    pub args: PlaceTakeOrderArgs,
}
#[derive(Clone, Debug, PartialEq)]
pub struct PlaceTakeOrderIxData(pub PlaceTakeOrderIxArgs);
impl From<PlaceTakeOrderIxArgs> for PlaceTakeOrderIxData {
    fn from(args: PlaceTakeOrderIxArgs) -> Self {
        Self(args)
    }
}
impl PlaceTakeOrderIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != PLACE_TAKE_ORDER_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let args = if reader.is_empty() {
            Default::default()
        } else {
            <PlaceTakeOrderArgs>::deserialize(&mut reader)?
        };
        Ok(Self(PlaceTakeOrderIxArgs { args }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&PLACE_TAKE_ORDER_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.args, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn place_take_order_ix_with_program_id(
    program_id: Pubkey,
    keys: PlaceTakeOrderKeys,
    args: PlaceTakeOrderIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; PLACE_TAKE_ORDER_IX_ACCOUNTS_LEN] = keys.into();
    let data: PlaceTakeOrderIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn place_take_order_ix(
    keys: PlaceTakeOrderKeys,
    args: PlaceTakeOrderIxArgs,
) -> std::io::Result<Instruction> {
    place_take_order_ix_with_program_id(OPENBOOK_V2_PROGRAM_ID, keys, args)
}
pub fn place_take_order_invoke_with_program_id(
    program_id: Pubkey,
    accounts: PlaceTakeOrderAccounts<'_, '_>,
    args: PlaceTakeOrderIxArgs,
) -> ProgramResult {
    let keys: PlaceTakeOrderKeys = accounts.into();
    let ix = place_take_order_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn place_take_order_invoke(
    accounts: PlaceTakeOrderAccounts<'_, '_>,
    args: PlaceTakeOrderIxArgs,
) -> ProgramResult {
    place_take_order_invoke_with_program_id(OPENBOOK_V2_PROGRAM_ID, accounts, args)
}
pub fn place_take_order_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: PlaceTakeOrderAccounts<'_, '_>,
    args: PlaceTakeOrderIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: PlaceTakeOrderKeys = accounts.into();
    let ix = place_take_order_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn place_take_order_invoke_signed(
    accounts: PlaceTakeOrderAccounts<'_, '_>,
    args: PlaceTakeOrderIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    place_take_order_invoke_signed_with_program_id(
        OPENBOOK_V2_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn place_take_order_verify_account_keys(
    accounts: PlaceTakeOrderAccounts<'_, '_>,
    keys: PlaceTakeOrderKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.signer.key, keys.signer),
        (*accounts.penalty_payer.key, keys.penalty_payer),
        (*accounts.market.key, keys.market),
        (*accounts.market_authority.key, keys.market_authority),
        (*accounts.bids.key, keys.bids),
        (*accounts.asks.key, keys.asks),
        (*accounts.market_base_vault.key, keys.market_base_vault),
        (*accounts.market_quote_vault.key, keys.market_quote_vault),
        (*accounts.event_heap.key, keys.event_heap),
        (*accounts.user_base_account.key, keys.user_base_account),
        (*accounts.user_quote_account.key, keys.user_quote_account),
        (*accounts.oracle_a.key, keys.oracle_a),
        (*accounts.oracle_b.key, keys.oracle_b),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.open_orders_admin.key, keys.open_orders_admin),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn place_take_order_verify_writable_privileges<'me, 'info>(
    accounts: PlaceTakeOrderAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.signer,
        accounts.penalty_payer,
        accounts.market,
        accounts.bids,
        accounts.asks,
        accounts.market_base_vault,
        accounts.market_quote_vault,
        accounts.event_heap,
        accounts.user_base_account,
        accounts.user_quote_account,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn place_take_order_verify_signer_privileges<'me, 'info>(
    accounts: PlaceTakeOrderAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [
        accounts.signer,
        accounts.penalty_payer,
        accounts.open_orders_admin,
    ] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn place_take_order_verify_account_privileges<'me, 'info>(
    accounts: PlaceTakeOrderAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    place_take_order_verify_writable_privileges(accounts)?;
    place_take_order_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CONSUME_EVENTS_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct ConsumeEventsAccounts<'me, 'info> {
    pub consume_events_admin: &'me AccountInfo<'info>,
    pub market: &'me AccountInfo<'info>,
    pub event_heap: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ConsumeEventsKeys {
    pub consume_events_admin: Pubkey,
    pub market: Pubkey,
    pub event_heap: Pubkey,
}
impl From<ConsumeEventsAccounts<'_, '_>> for ConsumeEventsKeys {
    fn from(accounts: ConsumeEventsAccounts) -> Self {
        Self {
            consume_events_admin: *accounts.consume_events_admin.key,
            market: *accounts.market.key,
            event_heap: *accounts.event_heap.key,
        }
    }
}
impl From<ConsumeEventsKeys> for [AccountMeta; CONSUME_EVENTS_IX_ACCOUNTS_LEN] {
    fn from(keys: ConsumeEventsKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.consume_events_admin,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.market,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.event_heap,
                is_signer: false,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; CONSUME_EVENTS_IX_ACCOUNTS_LEN]> for ConsumeEventsKeys {
    fn from(pubkeys: [Pubkey; CONSUME_EVENTS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            consume_events_admin: pubkeys[0],
            market: pubkeys[1],
            event_heap: pubkeys[2],
        }
    }
}
impl<'info> From<ConsumeEventsAccounts<'_, 'info>>
for [AccountInfo<'info>; CONSUME_EVENTS_IX_ACCOUNTS_LEN] {
    fn from(accounts: ConsumeEventsAccounts<'_, 'info>) -> Self {
        [
            accounts.consume_events_admin.clone(),
            accounts.market.clone(),
            accounts.event_heap.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CONSUME_EVENTS_IX_ACCOUNTS_LEN]>
for ConsumeEventsAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; CONSUME_EVENTS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            consume_events_admin: &arr[0],
            market: &arr[1],
            event_heap: &arr[2],
        }
    }
}
pub const CONSUME_EVENTS_IX_DISCM: [u8; 8usize] = [221, 145, 177, 52, 31, 47, 63, 201];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ConsumeEventsIxArgs {
    pub limit: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct ConsumeEventsIxData(pub ConsumeEventsIxArgs);
impl From<ConsumeEventsIxArgs> for ConsumeEventsIxData {
    fn from(args: ConsumeEventsIxArgs) -> Self {
        Self(args)
    }
}
impl ConsumeEventsIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CONSUME_EVENTS_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let limit: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(Self(ConsumeEventsIxArgs { limit }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CONSUME_EVENTS_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.limit, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn consume_events_ix_with_program_id(
    program_id: Pubkey,
    keys: ConsumeEventsKeys,
    args: ConsumeEventsIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CONSUME_EVENTS_IX_ACCOUNTS_LEN] = keys.into();
    let data: ConsumeEventsIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn consume_events_ix(
    keys: ConsumeEventsKeys,
    args: ConsumeEventsIxArgs,
) -> std::io::Result<Instruction> {
    consume_events_ix_with_program_id(OPENBOOK_V2_PROGRAM_ID, keys, args)
}
pub fn consume_events_invoke_with_program_id(
    program_id: Pubkey,
    accounts: ConsumeEventsAccounts<'_, '_>,
    args: ConsumeEventsIxArgs,
) -> ProgramResult {
    let keys: ConsumeEventsKeys = accounts.into();
    let ix = consume_events_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn consume_events_invoke(
    accounts: ConsumeEventsAccounts<'_, '_>,
    args: ConsumeEventsIxArgs,
) -> ProgramResult {
    consume_events_invoke_with_program_id(OPENBOOK_V2_PROGRAM_ID, accounts, args)
}
pub fn consume_events_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: ConsumeEventsAccounts<'_, '_>,
    args: ConsumeEventsIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: ConsumeEventsKeys = accounts.into();
    let ix = consume_events_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn consume_events_invoke_signed(
    accounts: ConsumeEventsAccounts<'_, '_>,
    args: ConsumeEventsIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    consume_events_invoke_signed_with_program_id(
        OPENBOOK_V2_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn consume_events_verify_account_keys(
    accounts: ConsumeEventsAccounts<'_, '_>,
    keys: ConsumeEventsKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.consume_events_admin.key, keys.consume_events_admin),
        (*accounts.market.key, keys.market),
        (*accounts.event_heap.key, keys.event_heap),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn consume_events_verify_writable_privileges<'me, 'info>(
    accounts: ConsumeEventsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.market, accounts.event_heap] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn consume_events_verify_signer_privileges<'me, 'info>(
    accounts: ConsumeEventsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.consume_events_admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn consume_events_verify_account_privileges<'me, 'info>(
    accounts: ConsumeEventsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    consume_events_verify_writable_privileges(accounts)?;
    consume_events_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CONSUME_GIVEN_EVENTS_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct ConsumeGivenEventsAccounts<'me, 'info> {
    pub consume_events_admin: &'me AccountInfo<'info>,
    pub market: &'me AccountInfo<'info>,
    pub event_heap: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ConsumeGivenEventsKeys {
    pub consume_events_admin: Pubkey,
    pub market: Pubkey,
    pub event_heap: Pubkey,
}
impl From<ConsumeGivenEventsAccounts<'_, '_>> for ConsumeGivenEventsKeys {
    fn from(accounts: ConsumeGivenEventsAccounts) -> Self {
        Self {
            consume_events_admin: *accounts.consume_events_admin.key,
            market: *accounts.market.key,
            event_heap: *accounts.event_heap.key,
        }
    }
}
impl From<ConsumeGivenEventsKeys>
for [AccountMeta; CONSUME_GIVEN_EVENTS_IX_ACCOUNTS_LEN] {
    fn from(keys: ConsumeGivenEventsKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.consume_events_admin,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.market,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.event_heap,
                is_signer: false,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; CONSUME_GIVEN_EVENTS_IX_ACCOUNTS_LEN]> for ConsumeGivenEventsKeys {
    fn from(pubkeys: [Pubkey; CONSUME_GIVEN_EVENTS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            consume_events_admin: pubkeys[0],
            market: pubkeys[1],
            event_heap: pubkeys[2],
        }
    }
}
impl<'info> From<ConsumeGivenEventsAccounts<'_, 'info>>
for [AccountInfo<'info>; CONSUME_GIVEN_EVENTS_IX_ACCOUNTS_LEN] {
    fn from(accounts: ConsumeGivenEventsAccounts<'_, 'info>) -> Self {
        [
            accounts.consume_events_admin.clone(),
            accounts.market.clone(),
            accounts.event_heap.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CONSUME_GIVEN_EVENTS_IX_ACCOUNTS_LEN]>
for ConsumeGivenEventsAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; CONSUME_GIVEN_EVENTS_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            consume_events_admin: &arr[0],
            market: &arr[1],
            event_heap: &arr[2],
        }
    }
}
pub const CONSUME_GIVEN_EVENTS_IX_DISCM: [u8; 8usize] = [
    209, 227, 54, 4, 109, 172, 41, 71,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ConsumeGivenEventsIxArgs {
    pub slots: Vec<u64>,
}
#[derive(Clone, Debug, PartialEq)]
pub struct ConsumeGivenEventsIxData(pub ConsumeGivenEventsIxArgs);
impl From<ConsumeGivenEventsIxArgs> for ConsumeGivenEventsIxData {
    fn from(args: ConsumeGivenEventsIxArgs) -> Self {
        Self(args)
    }
}
impl ConsumeGivenEventsIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CONSUME_GIVEN_EVENTS_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let slots: Vec<u64> = crate::borsh_de_or_default(&mut reader)?;
        Ok(Self(ConsumeGivenEventsIxArgs { slots }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CONSUME_GIVEN_EVENTS_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.slots, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn consume_given_events_ix_with_program_id(
    program_id: Pubkey,
    keys: ConsumeGivenEventsKeys,
    args: ConsumeGivenEventsIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CONSUME_GIVEN_EVENTS_IX_ACCOUNTS_LEN] = keys.into();
    let data: ConsumeGivenEventsIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn consume_given_events_ix(
    keys: ConsumeGivenEventsKeys,
    args: ConsumeGivenEventsIxArgs,
) -> std::io::Result<Instruction> {
    consume_given_events_ix_with_program_id(OPENBOOK_V2_PROGRAM_ID, keys, args)
}
pub fn consume_given_events_invoke_with_program_id(
    program_id: Pubkey,
    accounts: ConsumeGivenEventsAccounts<'_, '_>,
    args: ConsumeGivenEventsIxArgs,
) -> ProgramResult {
    let keys: ConsumeGivenEventsKeys = accounts.into();
    let ix = consume_given_events_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn consume_given_events_invoke(
    accounts: ConsumeGivenEventsAccounts<'_, '_>,
    args: ConsumeGivenEventsIxArgs,
) -> ProgramResult {
    consume_given_events_invoke_with_program_id(OPENBOOK_V2_PROGRAM_ID, accounts, args)
}
pub fn consume_given_events_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: ConsumeGivenEventsAccounts<'_, '_>,
    args: ConsumeGivenEventsIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: ConsumeGivenEventsKeys = accounts.into();
    let ix = consume_given_events_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn consume_given_events_invoke_signed(
    accounts: ConsumeGivenEventsAccounts<'_, '_>,
    args: ConsumeGivenEventsIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    consume_given_events_invoke_signed_with_program_id(
        OPENBOOK_V2_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn consume_given_events_verify_account_keys(
    accounts: ConsumeGivenEventsAccounts<'_, '_>,
    keys: ConsumeGivenEventsKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.consume_events_admin.key, keys.consume_events_admin),
        (*accounts.market.key, keys.market),
        (*accounts.event_heap.key, keys.event_heap),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn consume_given_events_verify_writable_privileges<'me, 'info>(
    accounts: ConsumeGivenEventsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.market, accounts.event_heap] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn consume_given_events_verify_signer_privileges<'me, 'info>(
    accounts: ConsumeGivenEventsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.consume_events_admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn consume_given_events_verify_account_privileges<'me, 'info>(
    accounts: ConsumeGivenEventsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    consume_given_events_verify_writable_privileges(accounts)?;
    consume_given_events_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CANCEL_ORDER_IX_ACCOUNTS_LEN: usize = 5;
#[derive(Copy, Clone, Debug)]
pub struct CancelOrderAccounts<'me, 'info> {
    pub signer: &'me AccountInfo<'info>,
    pub open_orders_account: &'me AccountInfo<'info>,
    pub market: &'me AccountInfo<'info>,
    pub bids: &'me AccountInfo<'info>,
    pub asks: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CancelOrderKeys {
    pub signer: Pubkey,
    pub open_orders_account: Pubkey,
    pub market: Pubkey,
    pub bids: Pubkey,
    pub asks: Pubkey,
}
impl From<CancelOrderAccounts<'_, '_>> for CancelOrderKeys {
    fn from(accounts: CancelOrderAccounts) -> Self {
        Self {
            signer: *accounts.signer.key,
            open_orders_account: *accounts.open_orders_account.key,
            market: *accounts.market.key,
            bids: *accounts.bids.key,
            asks: *accounts.asks.key,
        }
    }
}
impl From<CancelOrderKeys> for [AccountMeta; CANCEL_ORDER_IX_ACCOUNTS_LEN] {
    fn from(keys: CancelOrderKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.signer,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.open_orders_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.market,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.bids,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.asks,
                is_signer: false,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; CANCEL_ORDER_IX_ACCOUNTS_LEN]> for CancelOrderKeys {
    fn from(pubkeys: [Pubkey; CANCEL_ORDER_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            signer: pubkeys[0],
            open_orders_account: pubkeys[1],
            market: pubkeys[2],
            bids: pubkeys[3],
            asks: pubkeys[4],
        }
    }
}
impl<'info> From<CancelOrderAccounts<'_, 'info>>
for [AccountInfo<'info>; CANCEL_ORDER_IX_ACCOUNTS_LEN] {
    fn from(accounts: CancelOrderAccounts<'_, 'info>) -> Self {
        [
            accounts.signer.clone(),
            accounts.open_orders_account.clone(),
            accounts.market.clone(),
            accounts.bids.clone(),
            accounts.asks.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CANCEL_ORDER_IX_ACCOUNTS_LEN]>
for CancelOrderAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; CANCEL_ORDER_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            signer: &arr[0],
            open_orders_account: &arr[1],
            market: &arr[2],
            bids: &arr[3],
            asks: &arr[4],
        }
    }
}
pub const CANCEL_ORDER_IX_DISCM: [u8; 8usize] = [95, 129, 237, 240, 8, 49, 223, 132];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CancelOrderIxArgs {
    pub order_id: u128,
}
#[derive(Clone, Debug, PartialEq)]
pub struct CancelOrderIxData(pub CancelOrderIxArgs);
impl From<CancelOrderIxArgs> for CancelOrderIxData {
    fn from(args: CancelOrderIxArgs) -> Self {
        Self(args)
    }
}
impl CancelOrderIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CANCEL_ORDER_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let order_id: u128 = crate::borsh_de_or_default(&mut reader)?;
        Ok(Self(CancelOrderIxArgs { order_id }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CANCEL_ORDER_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.order_id, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn cancel_order_ix_with_program_id(
    program_id: Pubkey,
    keys: CancelOrderKeys,
    args: CancelOrderIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CANCEL_ORDER_IX_ACCOUNTS_LEN] = keys.into();
    let data: CancelOrderIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn cancel_order_ix(
    keys: CancelOrderKeys,
    args: CancelOrderIxArgs,
) -> std::io::Result<Instruction> {
    cancel_order_ix_with_program_id(OPENBOOK_V2_PROGRAM_ID, keys, args)
}
pub fn cancel_order_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CancelOrderAccounts<'_, '_>,
    args: CancelOrderIxArgs,
) -> ProgramResult {
    let keys: CancelOrderKeys = accounts.into();
    let ix = cancel_order_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn cancel_order_invoke(
    accounts: CancelOrderAccounts<'_, '_>,
    args: CancelOrderIxArgs,
) -> ProgramResult {
    cancel_order_invoke_with_program_id(OPENBOOK_V2_PROGRAM_ID, accounts, args)
}
pub fn cancel_order_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CancelOrderAccounts<'_, '_>,
    args: CancelOrderIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CancelOrderKeys = accounts.into();
    let ix = cancel_order_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn cancel_order_invoke_signed(
    accounts: CancelOrderAccounts<'_, '_>,
    args: CancelOrderIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    cancel_order_invoke_signed_with_program_id(
        OPENBOOK_V2_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn cancel_order_verify_account_keys(
    accounts: CancelOrderAccounts<'_, '_>,
    keys: CancelOrderKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.signer.key, keys.signer),
        (*accounts.open_orders_account.key, keys.open_orders_account),
        (*accounts.market.key, keys.market),
        (*accounts.bids.key, keys.bids),
        (*accounts.asks.key, keys.asks),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn cancel_order_verify_writable_privileges<'me, 'info>(
    accounts: CancelOrderAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.open_orders_account,
        accounts.bids,
        accounts.asks,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn cancel_order_verify_signer_privileges<'me, 'info>(
    accounts: CancelOrderAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.signer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn cancel_order_verify_account_privileges<'me, 'info>(
    accounts: CancelOrderAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    cancel_order_verify_writable_privileges(accounts)?;
    cancel_order_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CANCEL_ORDER_BY_CLIENT_ORDER_ID_IX_ACCOUNTS_LEN: usize = 5;
#[derive(Copy, Clone, Debug)]
pub struct CancelOrderByClientOrderIdAccounts<'me, 'info> {
    pub signer: &'me AccountInfo<'info>,
    pub open_orders_account: &'me AccountInfo<'info>,
    pub market: &'me AccountInfo<'info>,
    pub bids: &'me AccountInfo<'info>,
    pub asks: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CancelOrderByClientOrderIdKeys {
    pub signer: Pubkey,
    pub open_orders_account: Pubkey,
    pub market: Pubkey,
    pub bids: Pubkey,
    pub asks: Pubkey,
}
impl From<CancelOrderByClientOrderIdAccounts<'_, '_>>
for CancelOrderByClientOrderIdKeys {
    fn from(accounts: CancelOrderByClientOrderIdAccounts) -> Self {
        Self {
            signer: *accounts.signer.key,
            open_orders_account: *accounts.open_orders_account.key,
            market: *accounts.market.key,
            bids: *accounts.bids.key,
            asks: *accounts.asks.key,
        }
    }
}
impl From<CancelOrderByClientOrderIdKeys>
for [AccountMeta; CANCEL_ORDER_BY_CLIENT_ORDER_ID_IX_ACCOUNTS_LEN] {
    fn from(keys: CancelOrderByClientOrderIdKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.signer,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.open_orders_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.market,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.bids,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.asks,
                is_signer: false,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; CANCEL_ORDER_BY_CLIENT_ORDER_ID_IX_ACCOUNTS_LEN]>
for CancelOrderByClientOrderIdKeys {
    fn from(pubkeys: [Pubkey; CANCEL_ORDER_BY_CLIENT_ORDER_ID_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            signer: pubkeys[0],
            open_orders_account: pubkeys[1],
            market: pubkeys[2],
            bids: pubkeys[3],
            asks: pubkeys[4],
        }
    }
}
impl<'info> From<CancelOrderByClientOrderIdAccounts<'_, 'info>>
for [AccountInfo<'info>; CANCEL_ORDER_BY_CLIENT_ORDER_ID_IX_ACCOUNTS_LEN] {
    fn from(accounts: CancelOrderByClientOrderIdAccounts<'_, 'info>) -> Self {
        [
            accounts.signer.clone(),
            accounts.open_orders_account.clone(),
            accounts.market.clone(),
            accounts.bids.clone(),
            accounts.asks.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; CANCEL_ORDER_BY_CLIENT_ORDER_ID_IX_ACCOUNTS_LEN]>
for CancelOrderByClientOrderIdAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; CANCEL_ORDER_BY_CLIENT_ORDER_ID_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            signer: &arr[0],
            open_orders_account: &arr[1],
            market: &arr[2],
            bids: &arr[3],
            asks: &arr[4],
        }
    }
}
pub const CANCEL_ORDER_BY_CLIENT_ORDER_ID_IX_DISCM: [u8; 8usize] = [
    115, 178, 201, 8, 175, 183, 123, 119,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CancelOrderByClientOrderIdIxArgs {
    pub client_order_id: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct CancelOrderByClientOrderIdIxData(pub CancelOrderByClientOrderIdIxArgs);
impl From<CancelOrderByClientOrderIdIxArgs> for CancelOrderByClientOrderIdIxData {
    fn from(args: CancelOrderByClientOrderIdIxArgs) -> Self {
        Self(args)
    }
}
impl CancelOrderByClientOrderIdIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CANCEL_ORDER_BY_CLIENT_ORDER_ID_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let client_order_id: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(CancelOrderByClientOrderIdIxArgs {
                client_order_id,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CANCEL_ORDER_BY_CLIENT_ORDER_ID_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.client_order_id, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn cancel_order_by_client_order_id_ix_with_program_id(
    program_id: Pubkey,
    keys: CancelOrderByClientOrderIdKeys,
    args: CancelOrderByClientOrderIdIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CANCEL_ORDER_BY_CLIENT_ORDER_ID_IX_ACCOUNTS_LEN] = keys
        .into();
    let data: CancelOrderByClientOrderIdIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn cancel_order_by_client_order_id_ix(
    keys: CancelOrderByClientOrderIdKeys,
    args: CancelOrderByClientOrderIdIxArgs,
) -> std::io::Result<Instruction> {
    cancel_order_by_client_order_id_ix_with_program_id(
        OPENBOOK_V2_PROGRAM_ID,
        keys,
        args,
    )
}
pub fn cancel_order_by_client_order_id_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CancelOrderByClientOrderIdAccounts<'_, '_>,
    args: CancelOrderByClientOrderIdIxArgs,
) -> ProgramResult {
    let keys: CancelOrderByClientOrderIdKeys = accounts.into();
    let ix = cancel_order_by_client_order_id_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn cancel_order_by_client_order_id_invoke(
    accounts: CancelOrderByClientOrderIdAccounts<'_, '_>,
    args: CancelOrderByClientOrderIdIxArgs,
) -> ProgramResult {
    cancel_order_by_client_order_id_invoke_with_program_id(
        OPENBOOK_V2_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn cancel_order_by_client_order_id_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CancelOrderByClientOrderIdAccounts<'_, '_>,
    args: CancelOrderByClientOrderIdIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CancelOrderByClientOrderIdKeys = accounts.into();
    let ix = cancel_order_by_client_order_id_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn cancel_order_by_client_order_id_invoke_signed(
    accounts: CancelOrderByClientOrderIdAccounts<'_, '_>,
    args: CancelOrderByClientOrderIdIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    cancel_order_by_client_order_id_invoke_signed_with_program_id(
        OPENBOOK_V2_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn cancel_order_by_client_order_id_verify_account_keys(
    accounts: CancelOrderByClientOrderIdAccounts<'_, '_>,
    keys: CancelOrderByClientOrderIdKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.signer.key, keys.signer),
        (*accounts.open_orders_account.key, keys.open_orders_account),
        (*accounts.market.key, keys.market),
        (*accounts.bids.key, keys.bids),
        (*accounts.asks.key, keys.asks),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn cancel_order_by_client_order_id_verify_writable_privileges<'me, 'info>(
    accounts: CancelOrderByClientOrderIdAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.open_orders_account,
        accounts.bids,
        accounts.asks,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn cancel_order_by_client_order_id_verify_signer_privileges<'me, 'info>(
    accounts: CancelOrderByClientOrderIdAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.signer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn cancel_order_by_client_order_id_verify_account_privileges<'me, 'info>(
    accounts: CancelOrderByClientOrderIdAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    cancel_order_by_client_order_id_verify_writable_privileges(accounts)?;
    cancel_order_by_client_order_id_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CANCEL_ALL_ORDERS_IX_ACCOUNTS_LEN: usize = 5;
#[derive(Copy, Clone, Debug)]
pub struct CancelAllOrdersAccounts<'me, 'info> {
    pub signer: &'me AccountInfo<'info>,
    pub open_orders_account: &'me AccountInfo<'info>,
    pub market: &'me AccountInfo<'info>,
    pub bids: &'me AccountInfo<'info>,
    pub asks: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CancelAllOrdersKeys {
    pub signer: Pubkey,
    pub open_orders_account: Pubkey,
    pub market: Pubkey,
    pub bids: Pubkey,
    pub asks: Pubkey,
}
impl From<CancelAllOrdersAccounts<'_, '_>> for CancelAllOrdersKeys {
    fn from(accounts: CancelAllOrdersAccounts) -> Self {
        Self {
            signer: *accounts.signer.key,
            open_orders_account: *accounts.open_orders_account.key,
            market: *accounts.market.key,
            bids: *accounts.bids.key,
            asks: *accounts.asks.key,
        }
    }
}
impl From<CancelAllOrdersKeys> for [AccountMeta; CANCEL_ALL_ORDERS_IX_ACCOUNTS_LEN] {
    fn from(keys: CancelAllOrdersKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.signer,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.open_orders_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.market,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.bids,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.asks,
                is_signer: false,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; CANCEL_ALL_ORDERS_IX_ACCOUNTS_LEN]> for CancelAllOrdersKeys {
    fn from(pubkeys: [Pubkey; CANCEL_ALL_ORDERS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            signer: pubkeys[0],
            open_orders_account: pubkeys[1],
            market: pubkeys[2],
            bids: pubkeys[3],
            asks: pubkeys[4],
        }
    }
}
impl<'info> From<CancelAllOrdersAccounts<'_, 'info>>
for [AccountInfo<'info>; CANCEL_ALL_ORDERS_IX_ACCOUNTS_LEN] {
    fn from(accounts: CancelAllOrdersAccounts<'_, 'info>) -> Self {
        [
            accounts.signer.clone(),
            accounts.open_orders_account.clone(),
            accounts.market.clone(),
            accounts.bids.clone(),
            accounts.asks.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CANCEL_ALL_ORDERS_IX_ACCOUNTS_LEN]>
for CancelAllOrdersAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; CANCEL_ALL_ORDERS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            signer: &arr[0],
            open_orders_account: &arr[1],
            market: &arr[2],
            bids: &arr[3],
            asks: &arr[4],
        }
    }
}
pub const CANCEL_ALL_ORDERS_IX_DISCM: [u8; 8usize] = [
    196, 83, 243, 171, 17, 100, 160, 143,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CancelAllOrdersIxArgs {
    pub side_option: Option<Side>,
    pub limit: u8,
}
#[derive(Clone, Debug, PartialEq)]
pub struct CancelAllOrdersIxData(pub CancelAllOrdersIxArgs);
impl From<CancelAllOrdersIxArgs> for CancelAllOrdersIxData {
    fn from(args: CancelAllOrdersIxArgs) -> Self {
        Self(args)
    }
}
impl CancelAllOrdersIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CANCEL_ALL_ORDERS_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let side_option: Option<Side> = crate::borsh_de_or_default(&mut reader)?;
        let limit: u8 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(CancelAllOrdersIxArgs {
                side_option,
                limit,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CANCEL_ALL_ORDERS_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.side_option, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.limit, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn cancel_all_orders_ix_with_program_id(
    program_id: Pubkey,
    keys: CancelAllOrdersKeys,
    args: CancelAllOrdersIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CANCEL_ALL_ORDERS_IX_ACCOUNTS_LEN] = keys.into();
    let data: CancelAllOrdersIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn cancel_all_orders_ix(
    keys: CancelAllOrdersKeys,
    args: CancelAllOrdersIxArgs,
) -> std::io::Result<Instruction> {
    cancel_all_orders_ix_with_program_id(OPENBOOK_V2_PROGRAM_ID, keys, args)
}
pub fn cancel_all_orders_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CancelAllOrdersAccounts<'_, '_>,
    args: CancelAllOrdersIxArgs,
) -> ProgramResult {
    let keys: CancelAllOrdersKeys = accounts.into();
    let ix = cancel_all_orders_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn cancel_all_orders_invoke(
    accounts: CancelAllOrdersAccounts<'_, '_>,
    args: CancelAllOrdersIxArgs,
) -> ProgramResult {
    cancel_all_orders_invoke_with_program_id(OPENBOOK_V2_PROGRAM_ID, accounts, args)
}
pub fn cancel_all_orders_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CancelAllOrdersAccounts<'_, '_>,
    args: CancelAllOrdersIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CancelAllOrdersKeys = accounts.into();
    let ix = cancel_all_orders_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn cancel_all_orders_invoke_signed(
    accounts: CancelAllOrdersAccounts<'_, '_>,
    args: CancelAllOrdersIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    cancel_all_orders_invoke_signed_with_program_id(
        OPENBOOK_V2_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn cancel_all_orders_verify_account_keys(
    accounts: CancelAllOrdersAccounts<'_, '_>,
    keys: CancelAllOrdersKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.signer.key, keys.signer),
        (*accounts.open_orders_account.key, keys.open_orders_account),
        (*accounts.market.key, keys.market),
        (*accounts.bids.key, keys.bids),
        (*accounts.asks.key, keys.asks),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn cancel_all_orders_verify_writable_privileges<'me, 'info>(
    accounts: CancelAllOrdersAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.open_orders_account,
        accounts.bids,
        accounts.asks,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn cancel_all_orders_verify_signer_privileges<'me, 'info>(
    accounts: CancelAllOrdersAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.signer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn cancel_all_orders_verify_account_privileges<'me, 'info>(
    accounts: CancelAllOrdersAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    cancel_all_orders_verify_writable_privileges(accounts)?;
    cancel_all_orders_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const DEPOSIT_IX_ACCOUNTS_LEN: usize = 8;
#[derive(Copy, Clone, Debug)]
pub struct DepositAccounts<'me, 'info> {
    pub owner: &'me AccountInfo<'info>,
    pub user_base_account: &'me AccountInfo<'info>,
    pub user_quote_account: &'me AccountInfo<'info>,
    pub open_orders_account: &'me AccountInfo<'info>,
    pub market: &'me AccountInfo<'info>,
    pub market_base_vault: &'me AccountInfo<'info>,
    pub market_quote_vault: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct DepositKeys {
    pub owner: Pubkey,
    pub user_base_account: Pubkey,
    pub user_quote_account: Pubkey,
    pub open_orders_account: Pubkey,
    pub market: Pubkey,
    pub market_base_vault: Pubkey,
    pub market_quote_vault: Pubkey,
    pub token_program: Pubkey,
}
impl From<DepositAccounts<'_, '_>> for DepositKeys {
    fn from(accounts: DepositAccounts) -> Self {
        Self {
            owner: *accounts.owner.key,
            user_base_account: *accounts.user_base_account.key,
            user_quote_account: *accounts.user_quote_account.key,
            open_orders_account: *accounts.open_orders_account.key,
            market: *accounts.market.key,
            market_base_vault: *accounts.market_base_vault.key,
            market_quote_vault: *accounts.market_quote_vault.key,
            token_program: *accounts.token_program.key,
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
                pubkey: keys.user_base_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_quote_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.open_orders_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.market,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.market_base_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.market_quote_vault,
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
impl From<[Pubkey; DEPOSIT_IX_ACCOUNTS_LEN]> for DepositKeys {
    fn from(pubkeys: [Pubkey; DEPOSIT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            owner: pubkeys[0],
            user_base_account: pubkeys[1],
            user_quote_account: pubkeys[2],
            open_orders_account: pubkeys[3],
            market: pubkeys[4],
            market_base_vault: pubkeys[5],
            market_quote_vault: pubkeys[6],
            token_program: pubkeys[7],
        }
    }
}
impl<'info> From<DepositAccounts<'_, 'info>>
for [AccountInfo<'info>; DEPOSIT_IX_ACCOUNTS_LEN] {
    fn from(accounts: DepositAccounts<'_, 'info>) -> Self {
        [
            accounts.owner.clone(),
            accounts.user_base_account.clone(),
            accounts.user_quote_account.clone(),
            accounts.open_orders_account.clone(),
            accounts.market.clone(),
            accounts.market_base_vault.clone(),
            accounts.market_quote_vault.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; DEPOSIT_IX_ACCOUNTS_LEN]>
for DepositAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; DEPOSIT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            owner: &arr[0],
            user_base_account: &arr[1],
            user_quote_account: &arr[2],
            open_orders_account: &arr[3],
            market: &arr[4],
            market_base_vault: &arr[5],
            market_quote_vault: &arr[6],
            token_program: &arr[7],
        }
    }
}
pub const DEPOSIT_IX_DISCM: [u8; 8usize] = [242, 35, 198, 137, 82, 225, 242, 182];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DepositIxArgs {
    pub base_amount: u64,
    pub quote_amount: u64,
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
        let base_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let quote_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(DepositIxArgs {
                base_amount,
                quote_amount,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&DEPOSIT_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.base_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.quote_amount, &mut writer)?;
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
    deposit_ix_with_program_id(OPENBOOK_V2_PROGRAM_ID, keys, args)
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
    deposit_invoke_with_program_id(OPENBOOK_V2_PROGRAM_ID, accounts, args)
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
    deposit_invoke_signed_with_program_id(OPENBOOK_V2_PROGRAM_ID, accounts, args, seeds)
}
pub fn deposit_verify_account_keys(
    accounts: DepositAccounts<'_, '_>,
    keys: DepositKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.owner.key, keys.owner),
        (*accounts.user_base_account.key, keys.user_base_account),
        (*accounts.user_quote_account.key, keys.user_quote_account),
        (*accounts.open_orders_account.key, keys.open_orders_account),
        (*accounts.market.key, keys.market),
        (*accounts.market_base_vault.key, keys.market_base_vault),
        (*accounts.market_quote_vault.key, keys.market_quote_vault),
        (*accounts.token_program.key, keys.token_program),
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
        accounts.user_base_account,
        accounts.user_quote_account,
        accounts.open_orders_account,
        accounts.market,
        accounts.market_base_vault,
        accounts.market_quote_vault,
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
pub const REFILL_IX_ACCOUNTS_LEN: usize = 8;
#[derive(Copy, Clone, Debug)]
pub struct RefillAccounts<'me, 'info> {
    pub owner: &'me AccountInfo<'info>,
    pub user_base_account: &'me AccountInfo<'info>,
    pub user_quote_account: &'me AccountInfo<'info>,
    pub open_orders_account: &'me AccountInfo<'info>,
    pub market: &'me AccountInfo<'info>,
    pub market_base_vault: &'me AccountInfo<'info>,
    pub market_quote_vault: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct RefillKeys {
    pub owner: Pubkey,
    pub user_base_account: Pubkey,
    pub user_quote_account: Pubkey,
    pub open_orders_account: Pubkey,
    pub market: Pubkey,
    pub market_base_vault: Pubkey,
    pub market_quote_vault: Pubkey,
    pub token_program: Pubkey,
}
impl From<RefillAccounts<'_, '_>> for RefillKeys {
    fn from(accounts: RefillAccounts) -> Self {
        Self {
            owner: *accounts.owner.key,
            user_base_account: *accounts.user_base_account.key,
            user_quote_account: *accounts.user_quote_account.key,
            open_orders_account: *accounts.open_orders_account.key,
            market: *accounts.market.key,
            market_base_vault: *accounts.market_base_vault.key,
            market_quote_vault: *accounts.market_quote_vault.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<RefillKeys> for [AccountMeta; REFILL_IX_ACCOUNTS_LEN] {
    fn from(keys: RefillKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.owner,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.user_base_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_quote_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.open_orders_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.market,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.market_base_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.market_quote_vault,
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
impl From<[Pubkey; REFILL_IX_ACCOUNTS_LEN]> for RefillKeys {
    fn from(pubkeys: [Pubkey; REFILL_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            owner: pubkeys[0],
            user_base_account: pubkeys[1],
            user_quote_account: pubkeys[2],
            open_orders_account: pubkeys[3],
            market: pubkeys[4],
            market_base_vault: pubkeys[5],
            market_quote_vault: pubkeys[6],
            token_program: pubkeys[7],
        }
    }
}
impl<'info> From<RefillAccounts<'_, 'info>>
for [AccountInfo<'info>; REFILL_IX_ACCOUNTS_LEN] {
    fn from(accounts: RefillAccounts<'_, 'info>) -> Self {
        [
            accounts.owner.clone(),
            accounts.user_base_account.clone(),
            accounts.user_quote_account.clone(),
            accounts.open_orders_account.clone(),
            accounts.market.clone(),
            accounts.market_base_vault.clone(),
            accounts.market_quote_vault.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; REFILL_IX_ACCOUNTS_LEN]>
for RefillAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; REFILL_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            owner: &arr[0],
            user_base_account: &arr[1],
            user_quote_account: &arr[2],
            open_orders_account: &arr[3],
            market: &arr[4],
            market_base_vault: &arr[5],
            market_quote_vault: &arr[6],
            token_program: &arr[7],
        }
    }
}
pub const REFILL_IX_DISCM: [u8; 8usize] = [128, 207, 142, 11, 54, 232, 38, 201];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct RefillIxArgs {
    pub base_amount: u64,
    pub quote_amount: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct RefillIxData(pub RefillIxArgs);
impl From<RefillIxArgs> for RefillIxData {
    fn from(args: RefillIxArgs) -> Self {
        Self(args)
    }
}
impl RefillIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != REFILL_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let base_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let quote_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(RefillIxArgs {
                base_amount,
                quote_amount,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&REFILL_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.base_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.quote_amount, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn refill_ix_with_program_id(
    program_id: Pubkey,
    keys: RefillKeys,
    args: RefillIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; REFILL_IX_ACCOUNTS_LEN] = keys.into();
    let data: RefillIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn refill_ix(keys: RefillKeys, args: RefillIxArgs) -> std::io::Result<Instruction> {
    refill_ix_with_program_id(OPENBOOK_V2_PROGRAM_ID, keys, args)
}
pub fn refill_invoke_with_program_id(
    program_id: Pubkey,
    accounts: RefillAccounts<'_, '_>,
    args: RefillIxArgs,
) -> ProgramResult {
    let keys: RefillKeys = accounts.into();
    let ix = refill_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn refill_invoke(
    accounts: RefillAccounts<'_, '_>,
    args: RefillIxArgs,
) -> ProgramResult {
    refill_invoke_with_program_id(OPENBOOK_V2_PROGRAM_ID, accounts, args)
}
pub fn refill_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: RefillAccounts<'_, '_>,
    args: RefillIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: RefillKeys = accounts.into();
    let ix = refill_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn refill_invoke_signed(
    accounts: RefillAccounts<'_, '_>,
    args: RefillIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    refill_invoke_signed_with_program_id(OPENBOOK_V2_PROGRAM_ID, accounts, args, seeds)
}
pub fn refill_verify_account_keys(
    accounts: RefillAccounts<'_, '_>,
    keys: RefillKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.owner.key, keys.owner),
        (*accounts.user_base_account.key, keys.user_base_account),
        (*accounts.user_quote_account.key, keys.user_quote_account),
        (*accounts.open_orders_account.key, keys.open_orders_account),
        (*accounts.market.key, keys.market),
        (*accounts.market_base_vault.key, keys.market_base_vault),
        (*accounts.market_quote_vault.key, keys.market_quote_vault),
        (*accounts.token_program.key, keys.token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn refill_verify_writable_privileges<'me, 'info>(
    accounts: RefillAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.user_base_account,
        accounts.user_quote_account,
        accounts.open_orders_account,
        accounts.market,
        accounts.market_base_vault,
        accounts.market_quote_vault,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn refill_verify_signer_privileges<'me, 'info>(
    accounts: RefillAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.owner] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn refill_verify_account_privileges<'me, 'info>(
    accounts: RefillAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    refill_verify_writable_privileges(accounts)?;
    refill_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SETTLE_FUNDS_IX_ACCOUNTS_LEN: usize = 12;
#[derive(Copy, Clone, Debug)]
pub struct SettleFundsAccounts<'me, 'info> {
    pub owner: &'me AccountInfo<'info>,
    pub penalty_payer: &'me AccountInfo<'info>,
    pub open_orders_account: &'me AccountInfo<'info>,
    pub market: &'me AccountInfo<'info>,
    pub market_authority: &'me AccountInfo<'info>,
    pub market_base_vault: &'me AccountInfo<'info>,
    pub market_quote_vault: &'me AccountInfo<'info>,
    pub user_base_account: &'me AccountInfo<'info>,
    pub user_quote_account: &'me AccountInfo<'info>,
    pub referrer_account: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SettleFundsKeys {
    pub owner: Pubkey,
    pub penalty_payer: Pubkey,
    pub open_orders_account: Pubkey,
    pub market: Pubkey,
    pub market_authority: Pubkey,
    pub market_base_vault: Pubkey,
    pub market_quote_vault: Pubkey,
    pub user_base_account: Pubkey,
    pub user_quote_account: Pubkey,
    pub referrer_account: Pubkey,
    pub token_program: Pubkey,
    pub system_program: Pubkey,
}
impl From<SettleFundsAccounts<'_, '_>> for SettleFundsKeys {
    fn from(accounts: SettleFundsAccounts) -> Self {
        Self {
            owner: *accounts.owner.key,
            penalty_payer: *accounts.penalty_payer.key,
            open_orders_account: *accounts.open_orders_account.key,
            market: *accounts.market.key,
            market_authority: *accounts.market_authority.key,
            market_base_vault: *accounts.market_base_vault.key,
            market_quote_vault: *accounts.market_quote_vault.key,
            user_base_account: *accounts.user_base_account.key,
            user_quote_account: *accounts.user_quote_account.key,
            referrer_account: *accounts.referrer_account.key,
            token_program: *accounts.token_program.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<SettleFundsKeys> for [AccountMeta; SETTLE_FUNDS_IX_ACCOUNTS_LEN] {
    fn from(keys: SettleFundsKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.owner,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.penalty_payer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.open_orders_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.market,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.market_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.market_base_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.market_quote_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_base_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_quote_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.referrer_account,
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
        ]
    }
}
impl From<[Pubkey; SETTLE_FUNDS_IX_ACCOUNTS_LEN]> for SettleFundsKeys {
    fn from(pubkeys: [Pubkey; SETTLE_FUNDS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            owner: pubkeys[0],
            penalty_payer: pubkeys[1],
            open_orders_account: pubkeys[2],
            market: pubkeys[3],
            market_authority: pubkeys[4],
            market_base_vault: pubkeys[5],
            market_quote_vault: pubkeys[6],
            user_base_account: pubkeys[7],
            user_quote_account: pubkeys[8],
            referrer_account: pubkeys[9],
            token_program: pubkeys[10],
            system_program: pubkeys[11],
        }
    }
}
impl<'info> From<SettleFundsAccounts<'_, 'info>>
for [AccountInfo<'info>; SETTLE_FUNDS_IX_ACCOUNTS_LEN] {
    fn from(accounts: SettleFundsAccounts<'_, 'info>) -> Self {
        [
            accounts.owner.clone(),
            accounts.penalty_payer.clone(),
            accounts.open_orders_account.clone(),
            accounts.market.clone(),
            accounts.market_authority.clone(),
            accounts.market_base_vault.clone(),
            accounts.market_quote_vault.clone(),
            accounts.user_base_account.clone(),
            accounts.user_quote_account.clone(),
            accounts.referrer_account.clone(),
            accounts.token_program.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SETTLE_FUNDS_IX_ACCOUNTS_LEN]>
for SettleFundsAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; SETTLE_FUNDS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            owner: &arr[0],
            penalty_payer: &arr[1],
            open_orders_account: &arr[2],
            market: &arr[3],
            market_authority: &arr[4],
            market_base_vault: &arr[5],
            market_quote_vault: &arr[6],
            user_base_account: &arr[7],
            user_quote_account: &arr[8],
            referrer_account: &arr[9],
            token_program: &arr[10],
            system_program: &arr[11],
        }
    }
}
pub const SETTLE_FUNDS_IX_DISCM: [u8; 8usize] = [238, 64, 163, 96, 75, 171, 16, 33];
#[derive(Clone, Debug, PartialEq)]
pub struct SettleFundsIxData;
impl SettleFundsIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SETTLE_FUNDS_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SETTLE_FUNDS_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn settle_funds_ix_with_program_id(
    program_id: Pubkey,
    keys: SettleFundsKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SETTLE_FUNDS_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: SettleFundsIxData.try_to_vec()?,
    })
}
pub fn settle_funds_ix(keys: SettleFundsKeys) -> std::io::Result<Instruction> {
    settle_funds_ix_with_program_id(OPENBOOK_V2_PROGRAM_ID, keys)
}
pub fn settle_funds_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SettleFundsAccounts<'_, '_>,
) -> ProgramResult {
    let keys: SettleFundsKeys = accounts.into();
    let ix = settle_funds_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn settle_funds_invoke(accounts: SettleFundsAccounts<'_, '_>) -> ProgramResult {
    settle_funds_invoke_with_program_id(OPENBOOK_V2_PROGRAM_ID, accounts)
}
pub fn settle_funds_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SettleFundsAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SettleFundsKeys = accounts.into();
    let ix = settle_funds_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn settle_funds_invoke_signed(
    accounts: SettleFundsAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    settle_funds_invoke_signed_with_program_id(OPENBOOK_V2_PROGRAM_ID, accounts, seeds)
}
pub fn settle_funds_verify_account_keys(
    accounts: SettleFundsAccounts<'_, '_>,
    keys: SettleFundsKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.owner.key, keys.owner),
        (*accounts.penalty_payer.key, keys.penalty_payer),
        (*accounts.open_orders_account.key, keys.open_orders_account),
        (*accounts.market.key, keys.market),
        (*accounts.market_authority.key, keys.market_authority),
        (*accounts.market_base_vault.key, keys.market_base_vault),
        (*accounts.market_quote_vault.key, keys.market_quote_vault),
        (*accounts.user_base_account.key, keys.user_base_account),
        (*accounts.user_quote_account.key, keys.user_quote_account),
        (*accounts.referrer_account.key, keys.referrer_account),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn settle_funds_verify_writable_privileges<'me, 'info>(
    accounts: SettleFundsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.owner,
        accounts.penalty_payer,
        accounts.open_orders_account,
        accounts.market,
        accounts.market_base_vault,
        accounts.market_quote_vault,
        accounts.user_base_account,
        accounts.user_quote_account,
        accounts.referrer_account,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn settle_funds_verify_signer_privileges<'me, 'info>(
    accounts: SettleFundsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.owner, accounts.penalty_payer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn settle_funds_verify_account_privileges<'me, 'info>(
    accounts: SettleFundsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    settle_funds_verify_writable_privileges(accounts)?;
    settle_funds_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SETTLE_FUNDS_EXPIRED_IX_ACCOUNTS_LEN: usize = 13;
#[derive(Copy, Clone, Debug)]
pub struct SettleFundsExpiredAccounts<'me, 'info> {
    pub close_market_admin: &'me AccountInfo<'info>,
    pub owner: &'me AccountInfo<'info>,
    pub penalty_payer: &'me AccountInfo<'info>,
    pub open_orders_account: &'me AccountInfo<'info>,
    pub market: &'me AccountInfo<'info>,
    pub market_authority: &'me AccountInfo<'info>,
    pub market_base_vault: &'me AccountInfo<'info>,
    pub market_quote_vault: &'me AccountInfo<'info>,
    pub user_base_account: &'me AccountInfo<'info>,
    pub user_quote_account: &'me AccountInfo<'info>,
    pub referrer_account: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SettleFundsExpiredKeys {
    pub close_market_admin: Pubkey,
    pub owner: Pubkey,
    pub penalty_payer: Pubkey,
    pub open_orders_account: Pubkey,
    pub market: Pubkey,
    pub market_authority: Pubkey,
    pub market_base_vault: Pubkey,
    pub market_quote_vault: Pubkey,
    pub user_base_account: Pubkey,
    pub user_quote_account: Pubkey,
    pub referrer_account: Pubkey,
    pub token_program: Pubkey,
    pub system_program: Pubkey,
}
impl From<SettleFundsExpiredAccounts<'_, '_>> for SettleFundsExpiredKeys {
    fn from(accounts: SettleFundsExpiredAccounts) -> Self {
        Self {
            close_market_admin: *accounts.close_market_admin.key,
            owner: *accounts.owner.key,
            penalty_payer: *accounts.penalty_payer.key,
            open_orders_account: *accounts.open_orders_account.key,
            market: *accounts.market.key,
            market_authority: *accounts.market_authority.key,
            market_base_vault: *accounts.market_base_vault.key,
            market_quote_vault: *accounts.market_quote_vault.key,
            user_base_account: *accounts.user_base_account.key,
            user_quote_account: *accounts.user_quote_account.key,
            referrer_account: *accounts.referrer_account.key,
            token_program: *accounts.token_program.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<SettleFundsExpiredKeys>
for [AccountMeta; SETTLE_FUNDS_EXPIRED_IX_ACCOUNTS_LEN] {
    fn from(keys: SettleFundsExpiredKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.close_market_admin,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.owner,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.penalty_payer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.open_orders_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.market,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.market_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.market_base_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.market_quote_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_base_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_quote_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.referrer_account,
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
        ]
    }
}
impl From<[Pubkey; SETTLE_FUNDS_EXPIRED_IX_ACCOUNTS_LEN]> for SettleFundsExpiredKeys {
    fn from(pubkeys: [Pubkey; SETTLE_FUNDS_EXPIRED_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            close_market_admin: pubkeys[0],
            owner: pubkeys[1],
            penalty_payer: pubkeys[2],
            open_orders_account: pubkeys[3],
            market: pubkeys[4],
            market_authority: pubkeys[5],
            market_base_vault: pubkeys[6],
            market_quote_vault: pubkeys[7],
            user_base_account: pubkeys[8],
            user_quote_account: pubkeys[9],
            referrer_account: pubkeys[10],
            token_program: pubkeys[11],
            system_program: pubkeys[12],
        }
    }
}
impl<'info> From<SettleFundsExpiredAccounts<'_, 'info>>
for [AccountInfo<'info>; SETTLE_FUNDS_EXPIRED_IX_ACCOUNTS_LEN] {
    fn from(accounts: SettleFundsExpiredAccounts<'_, 'info>) -> Self {
        [
            accounts.close_market_admin.clone(),
            accounts.owner.clone(),
            accounts.penalty_payer.clone(),
            accounts.open_orders_account.clone(),
            accounts.market.clone(),
            accounts.market_authority.clone(),
            accounts.market_base_vault.clone(),
            accounts.market_quote_vault.clone(),
            accounts.user_base_account.clone(),
            accounts.user_quote_account.clone(),
            accounts.referrer_account.clone(),
            accounts.token_program.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SETTLE_FUNDS_EXPIRED_IX_ACCOUNTS_LEN]>
for SettleFundsExpiredAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; SETTLE_FUNDS_EXPIRED_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            close_market_admin: &arr[0],
            owner: &arr[1],
            penalty_payer: &arr[2],
            open_orders_account: &arr[3],
            market: &arr[4],
            market_authority: &arr[5],
            market_base_vault: &arr[6],
            market_quote_vault: &arr[7],
            user_base_account: &arr[8],
            user_quote_account: &arr[9],
            referrer_account: &arr[10],
            token_program: &arr[11],
            system_program: &arr[12],
        }
    }
}
pub const SETTLE_FUNDS_EXPIRED_IX_DISCM: [u8; 8usize] = [
    107, 18, 56, 69, 228, 56, 55, 164,
];
#[derive(Clone, Debug, PartialEq)]
pub struct SettleFundsExpiredIxData;
impl SettleFundsExpiredIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SETTLE_FUNDS_EXPIRED_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SETTLE_FUNDS_EXPIRED_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn settle_funds_expired_ix_with_program_id(
    program_id: Pubkey,
    keys: SettleFundsExpiredKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SETTLE_FUNDS_EXPIRED_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: SettleFundsExpiredIxData.try_to_vec()?,
    })
}
pub fn settle_funds_expired_ix(
    keys: SettleFundsExpiredKeys,
) -> std::io::Result<Instruction> {
    settle_funds_expired_ix_with_program_id(OPENBOOK_V2_PROGRAM_ID, keys)
}
pub fn settle_funds_expired_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SettleFundsExpiredAccounts<'_, '_>,
) -> ProgramResult {
    let keys: SettleFundsExpiredKeys = accounts.into();
    let ix = settle_funds_expired_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn settle_funds_expired_invoke(
    accounts: SettleFundsExpiredAccounts<'_, '_>,
) -> ProgramResult {
    settle_funds_expired_invoke_with_program_id(OPENBOOK_V2_PROGRAM_ID, accounts)
}
pub fn settle_funds_expired_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SettleFundsExpiredAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SettleFundsExpiredKeys = accounts.into();
    let ix = settle_funds_expired_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn settle_funds_expired_invoke_signed(
    accounts: SettleFundsExpiredAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    settle_funds_expired_invoke_signed_with_program_id(
        OPENBOOK_V2_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn settle_funds_expired_verify_account_keys(
    accounts: SettleFundsExpiredAccounts<'_, '_>,
    keys: SettleFundsExpiredKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.close_market_admin.key, keys.close_market_admin),
        (*accounts.owner.key, keys.owner),
        (*accounts.penalty_payer.key, keys.penalty_payer),
        (*accounts.open_orders_account.key, keys.open_orders_account),
        (*accounts.market.key, keys.market),
        (*accounts.market_authority.key, keys.market_authority),
        (*accounts.market_base_vault.key, keys.market_base_vault),
        (*accounts.market_quote_vault.key, keys.market_quote_vault),
        (*accounts.user_base_account.key, keys.user_base_account),
        (*accounts.user_quote_account.key, keys.user_quote_account),
        (*accounts.referrer_account.key, keys.referrer_account),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn settle_funds_expired_verify_writable_privileges<'me, 'info>(
    accounts: SettleFundsExpiredAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.owner,
        accounts.penalty_payer,
        accounts.open_orders_account,
        accounts.market,
        accounts.market_base_vault,
        accounts.market_quote_vault,
        accounts.user_base_account,
        accounts.user_quote_account,
        accounts.referrer_account,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn settle_funds_expired_verify_signer_privileges<'me, 'info>(
    accounts: SettleFundsExpiredAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [
        accounts.close_market_admin,
        accounts.owner,
        accounts.penalty_payer,
    ] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn settle_funds_expired_verify_account_privileges<'me, 'info>(
    accounts: SettleFundsExpiredAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    settle_funds_expired_verify_writable_privileges(accounts)?;
    settle_funds_expired_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SWEEP_FEES_IX_ACCOUNTS_LEN: usize = 6;
#[derive(Copy, Clone, Debug)]
pub struct SweepFeesAccounts<'me, 'info> {
    pub collect_fee_admin: &'me AccountInfo<'info>,
    pub market: &'me AccountInfo<'info>,
    pub market_authority: &'me AccountInfo<'info>,
    pub market_quote_vault: &'me AccountInfo<'info>,
    pub token_receiver_account: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SweepFeesKeys {
    pub collect_fee_admin: Pubkey,
    pub market: Pubkey,
    pub market_authority: Pubkey,
    pub market_quote_vault: Pubkey,
    pub token_receiver_account: Pubkey,
    pub token_program: Pubkey,
}
impl From<SweepFeesAccounts<'_, '_>> for SweepFeesKeys {
    fn from(accounts: SweepFeesAccounts) -> Self {
        Self {
            collect_fee_admin: *accounts.collect_fee_admin.key,
            market: *accounts.market.key,
            market_authority: *accounts.market_authority.key,
            market_quote_vault: *accounts.market_quote_vault.key,
            token_receiver_account: *accounts.token_receiver_account.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<SweepFeesKeys> for [AccountMeta; SWEEP_FEES_IX_ACCOUNTS_LEN] {
    fn from(keys: SweepFeesKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.collect_fee_admin,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.market,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.market_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.market_quote_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_receiver_account,
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
impl From<[Pubkey; SWEEP_FEES_IX_ACCOUNTS_LEN]> for SweepFeesKeys {
    fn from(pubkeys: [Pubkey; SWEEP_FEES_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            collect_fee_admin: pubkeys[0],
            market: pubkeys[1],
            market_authority: pubkeys[2],
            market_quote_vault: pubkeys[3],
            token_receiver_account: pubkeys[4],
            token_program: pubkeys[5],
        }
    }
}
impl<'info> From<SweepFeesAccounts<'_, 'info>>
for [AccountInfo<'info>; SWEEP_FEES_IX_ACCOUNTS_LEN] {
    fn from(accounts: SweepFeesAccounts<'_, 'info>) -> Self {
        [
            accounts.collect_fee_admin.clone(),
            accounts.market.clone(),
            accounts.market_authority.clone(),
            accounts.market_quote_vault.clone(),
            accounts.token_receiver_account.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SWEEP_FEES_IX_ACCOUNTS_LEN]>
for SweepFeesAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; SWEEP_FEES_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            collect_fee_admin: &arr[0],
            market: &arr[1],
            market_authority: &arr[2],
            market_quote_vault: &arr[3],
            token_receiver_account: &arr[4],
            token_program: &arr[5],
        }
    }
}
pub const SWEEP_FEES_IX_DISCM: [u8; 8usize] = [175, 225, 98, 71, 118, 66, 34, 148];
#[derive(Clone, Debug, PartialEq)]
pub struct SweepFeesIxData;
impl SweepFeesIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SWEEP_FEES_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SWEEP_FEES_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn sweep_fees_ix_with_program_id(
    program_id: Pubkey,
    keys: SweepFeesKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SWEEP_FEES_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: SweepFeesIxData.try_to_vec()?,
    })
}
pub fn sweep_fees_ix(keys: SweepFeesKeys) -> std::io::Result<Instruction> {
    sweep_fees_ix_with_program_id(OPENBOOK_V2_PROGRAM_ID, keys)
}
pub fn sweep_fees_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SweepFeesAccounts<'_, '_>,
) -> ProgramResult {
    let keys: SweepFeesKeys = accounts.into();
    let ix = sweep_fees_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn sweep_fees_invoke(accounts: SweepFeesAccounts<'_, '_>) -> ProgramResult {
    sweep_fees_invoke_with_program_id(OPENBOOK_V2_PROGRAM_ID, accounts)
}
pub fn sweep_fees_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SweepFeesAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SweepFeesKeys = accounts.into();
    let ix = sweep_fees_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn sweep_fees_invoke_signed(
    accounts: SweepFeesAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    sweep_fees_invoke_signed_with_program_id(OPENBOOK_V2_PROGRAM_ID, accounts, seeds)
}
pub fn sweep_fees_verify_account_keys(
    accounts: SweepFeesAccounts<'_, '_>,
    keys: SweepFeesKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.collect_fee_admin.key, keys.collect_fee_admin),
        (*accounts.market.key, keys.market),
        (*accounts.market_authority.key, keys.market_authority),
        (*accounts.market_quote_vault.key, keys.market_quote_vault),
        (*accounts.token_receiver_account.key, keys.token_receiver_account),
        (*accounts.token_program.key, keys.token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn sweep_fees_verify_writable_privileges<'me, 'info>(
    accounts: SweepFeesAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.market,
        accounts.market_quote_vault,
        accounts.token_receiver_account,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn sweep_fees_verify_signer_privileges<'me, 'info>(
    accounts: SweepFeesAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.collect_fee_admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn sweep_fees_verify_account_privileges<'me, 'info>(
    accounts: SweepFeesAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    sweep_fees_verify_writable_privileges(accounts)?;
    sweep_fees_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SET_DELEGATE_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct SetDelegateAccounts<'me, 'info> {
    pub owner: &'me AccountInfo<'info>,
    pub open_orders_account: &'me AccountInfo<'info>,
    pub delegate_account: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SetDelegateKeys {
    pub owner: Pubkey,
    pub open_orders_account: Pubkey,
    pub delegate_account: Pubkey,
}
impl From<SetDelegateAccounts<'_, '_>> for SetDelegateKeys {
    fn from(accounts: SetDelegateAccounts) -> Self {
        Self {
            owner: *accounts.owner.key,
            open_orders_account: *accounts.open_orders_account.key,
            delegate_account: *accounts.delegate_account.key,
        }
    }
}
impl From<SetDelegateKeys> for [AccountMeta; SET_DELEGATE_IX_ACCOUNTS_LEN] {
    fn from(keys: SetDelegateKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.owner,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.open_orders_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.delegate_account,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; SET_DELEGATE_IX_ACCOUNTS_LEN]> for SetDelegateKeys {
    fn from(pubkeys: [Pubkey; SET_DELEGATE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            owner: pubkeys[0],
            open_orders_account: pubkeys[1],
            delegate_account: pubkeys[2],
        }
    }
}
impl<'info> From<SetDelegateAccounts<'_, 'info>>
for [AccountInfo<'info>; SET_DELEGATE_IX_ACCOUNTS_LEN] {
    fn from(accounts: SetDelegateAccounts<'_, 'info>) -> Self {
        [
            accounts.owner.clone(),
            accounts.open_orders_account.clone(),
            accounts.delegate_account.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SET_DELEGATE_IX_ACCOUNTS_LEN]>
for SetDelegateAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; SET_DELEGATE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            owner: &arr[0],
            open_orders_account: &arr[1],
            delegate_account: &arr[2],
        }
    }
}
pub const SET_DELEGATE_IX_DISCM: [u8; 8usize] = [242, 30, 46, 76, 108, 235, 128, 181];
#[derive(Clone, Debug, PartialEq)]
pub struct SetDelegateIxData;
impl SetDelegateIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SET_DELEGATE_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SET_DELEGATE_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn set_delegate_ix_with_program_id(
    program_id: Pubkey,
    keys: SetDelegateKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SET_DELEGATE_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: SetDelegateIxData.try_to_vec()?,
    })
}
pub fn set_delegate_ix(keys: SetDelegateKeys) -> std::io::Result<Instruction> {
    set_delegate_ix_with_program_id(OPENBOOK_V2_PROGRAM_ID, keys)
}
pub fn set_delegate_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SetDelegateAccounts<'_, '_>,
) -> ProgramResult {
    let keys: SetDelegateKeys = accounts.into();
    let ix = set_delegate_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn set_delegate_invoke(accounts: SetDelegateAccounts<'_, '_>) -> ProgramResult {
    set_delegate_invoke_with_program_id(OPENBOOK_V2_PROGRAM_ID, accounts)
}
pub fn set_delegate_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SetDelegateAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SetDelegateKeys = accounts.into();
    let ix = set_delegate_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn set_delegate_invoke_signed(
    accounts: SetDelegateAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    set_delegate_invoke_signed_with_program_id(OPENBOOK_V2_PROGRAM_ID, accounts, seeds)
}
pub fn set_delegate_verify_account_keys(
    accounts: SetDelegateAccounts<'_, '_>,
    keys: SetDelegateKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.owner.key, keys.owner),
        (*accounts.open_orders_account.key, keys.open_orders_account),
        (*accounts.delegate_account.key, keys.delegate_account),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn set_delegate_verify_writable_privileges<'me, 'info>(
    accounts: SetDelegateAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.owner, accounts.open_orders_account] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn set_delegate_verify_signer_privileges<'me, 'info>(
    accounts: SetDelegateAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.owner] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn set_delegate_verify_account_privileges<'me, 'info>(
    accounts: SetDelegateAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    set_delegate_verify_writable_privileges(accounts)?;
    set_delegate_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SET_MARKET_EXPIRED_IX_ACCOUNTS_LEN: usize = 2;
#[derive(Copy, Clone, Debug)]
pub struct SetMarketExpiredAccounts<'me, 'info> {
    pub close_market_admin: &'me AccountInfo<'info>,
    pub market: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SetMarketExpiredKeys {
    pub close_market_admin: Pubkey,
    pub market: Pubkey,
}
impl From<SetMarketExpiredAccounts<'_, '_>> for SetMarketExpiredKeys {
    fn from(accounts: SetMarketExpiredAccounts) -> Self {
        Self {
            close_market_admin: *accounts.close_market_admin.key,
            market: *accounts.market.key,
        }
    }
}
impl From<SetMarketExpiredKeys> for [AccountMeta; SET_MARKET_EXPIRED_IX_ACCOUNTS_LEN] {
    fn from(keys: SetMarketExpiredKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.close_market_admin,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.market,
                is_signer: false,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; SET_MARKET_EXPIRED_IX_ACCOUNTS_LEN]> for SetMarketExpiredKeys {
    fn from(pubkeys: [Pubkey; SET_MARKET_EXPIRED_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            close_market_admin: pubkeys[0],
            market: pubkeys[1],
        }
    }
}
impl<'info> From<SetMarketExpiredAccounts<'_, 'info>>
for [AccountInfo<'info>; SET_MARKET_EXPIRED_IX_ACCOUNTS_LEN] {
    fn from(accounts: SetMarketExpiredAccounts<'_, 'info>) -> Self {
        [accounts.close_market_admin.clone(), accounts.market.clone()]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SET_MARKET_EXPIRED_IX_ACCOUNTS_LEN]>
for SetMarketExpiredAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; SET_MARKET_EXPIRED_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            close_market_admin: &arr[0],
            market: &arr[1],
        }
    }
}
pub const SET_MARKET_EXPIRED_IX_DISCM: [u8; 8usize] = [
    219, 82, 219, 236, 60, 115, 197, 64,
];
#[derive(Clone, Debug, PartialEq)]
pub struct SetMarketExpiredIxData;
impl SetMarketExpiredIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SET_MARKET_EXPIRED_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SET_MARKET_EXPIRED_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn set_market_expired_ix_with_program_id(
    program_id: Pubkey,
    keys: SetMarketExpiredKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SET_MARKET_EXPIRED_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: SetMarketExpiredIxData.try_to_vec()?,
    })
}
pub fn set_market_expired_ix(
    keys: SetMarketExpiredKeys,
) -> std::io::Result<Instruction> {
    set_market_expired_ix_with_program_id(OPENBOOK_V2_PROGRAM_ID, keys)
}
pub fn set_market_expired_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SetMarketExpiredAccounts<'_, '_>,
) -> ProgramResult {
    let keys: SetMarketExpiredKeys = accounts.into();
    let ix = set_market_expired_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn set_market_expired_invoke(
    accounts: SetMarketExpiredAccounts<'_, '_>,
) -> ProgramResult {
    set_market_expired_invoke_with_program_id(OPENBOOK_V2_PROGRAM_ID, accounts)
}
pub fn set_market_expired_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SetMarketExpiredAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SetMarketExpiredKeys = accounts.into();
    let ix = set_market_expired_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn set_market_expired_invoke_signed(
    accounts: SetMarketExpiredAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    set_market_expired_invoke_signed_with_program_id(
        OPENBOOK_V2_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn set_market_expired_verify_account_keys(
    accounts: SetMarketExpiredAccounts<'_, '_>,
    keys: SetMarketExpiredKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.close_market_admin.key, keys.close_market_admin),
        (*accounts.market.key, keys.market),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn set_market_expired_verify_writable_privileges<'me, 'info>(
    accounts: SetMarketExpiredAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.market] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn set_market_expired_verify_signer_privileges<'me, 'info>(
    accounts: SetMarketExpiredAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.close_market_admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn set_market_expired_verify_account_privileges<'me, 'info>(
    accounts: SetMarketExpiredAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    set_market_expired_verify_writable_privileges(accounts)?;
    set_market_expired_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const PRUNE_ORDERS_IX_ACCOUNTS_LEN: usize = 5;
#[derive(Copy, Clone, Debug)]
pub struct PruneOrdersAccounts<'me, 'info> {
    pub close_market_admin: &'me AccountInfo<'info>,
    pub open_orders_account: &'me AccountInfo<'info>,
    pub market: &'me AccountInfo<'info>,
    pub bids: &'me AccountInfo<'info>,
    pub asks: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct PruneOrdersKeys {
    pub close_market_admin: Pubkey,
    pub open_orders_account: Pubkey,
    pub market: Pubkey,
    pub bids: Pubkey,
    pub asks: Pubkey,
}
impl From<PruneOrdersAccounts<'_, '_>> for PruneOrdersKeys {
    fn from(accounts: PruneOrdersAccounts) -> Self {
        Self {
            close_market_admin: *accounts.close_market_admin.key,
            open_orders_account: *accounts.open_orders_account.key,
            market: *accounts.market.key,
            bids: *accounts.bids.key,
            asks: *accounts.asks.key,
        }
    }
}
impl From<PruneOrdersKeys> for [AccountMeta; PRUNE_ORDERS_IX_ACCOUNTS_LEN] {
    fn from(keys: PruneOrdersKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.close_market_admin,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.open_orders_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.market,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.bids,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.asks,
                is_signer: false,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; PRUNE_ORDERS_IX_ACCOUNTS_LEN]> for PruneOrdersKeys {
    fn from(pubkeys: [Pubkey; PRUNE_ORDERS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            close_market_admin: pubkeys[0],
            open_orders_account: pubkeys[1],
            market: pubkeys[2],
            bids: pubkeys[3],
            asks: pubkeys[4],
        }
    }
}
impl<'info> From<PruneOrdersAccounts<'_, 'info>>
for [AccountInfo<'info>; PRUNE_ORDERS_IX_ACCOUNTS_LEN] {
    fn from(accounts: PruneOrdersAccounts<'_, 'info>) -> Self {
        [
            accounts.close_market_admin.clone(),
            accounts.open_orders_account.clone(),
            accounts.market.clone(),
            accounts.bids.clone(),
            accounts.asks.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; PRUNE_ORDERS_IX_ACCOUNTS_LEN]>
for PruneOrdersAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; PRUNE_ORDERS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            close_market_admin: &arr[0],
            open_orders_account: &arr[1],
            market: &arr[2],
            bids: &arr[3],
            asks: &arr[4],
        }
    }
}
pub const PRUNE_ORDERS_IX_DISCM: [u8; 8usize] = [27, 213, 159, 191, 12, 116, 112, 121];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PruneOrdersIxArgs {
    pub limit: u8,
}
#[derive(Clone, Debug, PartialEq)]
pub struct PruneOrdersIxData(pub PruneOrdersIxArgs);
impl From<PruneOrdersIxArgs> for PruneOrdersIxData {
    fn from(args: PruneOrdersIxArgs) -> Self {
        Self(args)
    }
}
impl PruneOrdersIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != PRUNE_ORDERS_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let limit: u8 = crate::borsh_de_or_default(&mut reader)?;
        Ok(Self(PruneOrdersIxArgs { limit }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&PRUNE_ORDERS_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.limit, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn prune_orders_ix_with_program_id(
    program_id: Pubkey,
    keys: PruneOrdersKeys,
    args: PruneOrdersIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; PRUNE_ORDERS_IX_ACCOUNTS_LEN] = keys.into();
    let data: PruneOrdersIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn prune_orders_ix(
    keys: PruneOrdersKeys,
    args: PruneOrdersIxArgs,
) -> std::io::Result<Instruction> {
    prune_orders_ix_with_program_id(OPENBOOK_V2_PROGRAM_ID, keys, args)
}
pub fn prune_orders_invoke_with_program_id(
    program_id: Pubkey,
    accounts: PruneOrdersAccounts<'_, '_>,
    args: PruneOrdersIxArgs,
) -> ProgramResult {
    let keys: PruneOrdersKeys = accounts.into();
    let ix = prune_orders_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn prune_orders_invoke(
    accounts: PruneOrdersAccounts<'_, '_>,
    args: PruneOrdersIxArgs,
) -> ProgramResult {
    prune_orders_invoke_with_program_id(OPENBOOK_V2_PROGRAM_ID, accounts, args)
}
pub fn prune_orders_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: PruneOrdersAccounts<'_, '_>,
    args: PruneOrdersIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: PruneOrdersKeys = accounts.into();
    let ix = prune_orders_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn prune_orders_invoke_signed(
    accounts: PruneOrdersAccounts<'_, '_>,
    args: PruneOrdersIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    prune_orders_invoke_signed_with_program_id(
        OPENBOOK_V2_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn prune_orders_verify_account_keys(
    accounts: PruneOrdersAccounts<'_, '_>,
    keys: PruneOrdersKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.close_market_admin.key, keys.close_market_admin),
        (*accounts.open_orders_account.key, keys.open_orders_account),
        (*accounts.market.key, keys.market),
        (*accounts.bids.key, keys.bids),
        (*accounts.asks.key, keys.asks),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn prune_orders_verify_writable_privileges<'me, 'info>(
    accounts: PruneOrdersAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.open_orders_account,
        accounts.bids,
        accounts.asks,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn prune_orders_verify_signer_privileges<'me, 'info>(
    accounts: PruneOrdersAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.close_market_admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn prune_orders_verify_account_privileges<'me, 'info>(
    accounts: PruneOrdersAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    prune_orders_verify_writable_privileges(accounts)?;
    prune_orders_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const STUB_ORACLE_CREATE_IX_ACCOUNTS_LEN: usize = 5;
#[derive(Copy, Clone, Debug)]
pub struct StubOracleCreateAccounts<'me, 'info> {
    pub payer: &'me AccountInfo<'info>,
    pub owner: &'me AccountInfo<'info>,
    pub oracle: &'me AccountInfo<'info>,
    pub mint: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct StubOracleCreateKeys {
    pub payer: Pubkey,
    pub owner: Pubkey,
    pub oracle: Pubkey,
    pub mint: Pubkey,
    pub system_program: Pubkey,
}
impl From<StubOracleCreateAccounts<'_, '_>> for StubOracleCreateKeys {
    fn from(accounts: StubOracleCreateAccounts) -> Self {
        Self {
            payer: *accounts.payer.key,
            owner: *accounts.owner.key,
            oracle: *accounts.oracle.key,
            mint: *accounts.mint.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<StubOracleCreateKeys> for [AccountMeta; STUB_ORACLE_CREATE_IX_ACCOUNTS_LEN] {
    fn from(keys: StubOracleCreateKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.payer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.owner,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.oracle,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.mint,
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
impl From<[Pubkey; STUB_ORACLE_CREATE_IX_ACCOUNTS_LEN]> for StubOracleCreateKeys {
    fn from(pubkeys: [Pubkey; STUB_ORACLE_CREATE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            payer: pubkeys[0],
            owner: pubkeys[1],
            oracle: pubkeys[2],
            mint: pubkeys[3],
            system_program: pubkeys[4],
        }
    }
}
impl<'info> From<StubOracleCreateAccounts<'_, 'info>>
for [AccountInfo<'info>; STUB_ORACLE_CREATE_IX_ACCOUNTS_LEN] {
    fn from(accounts: StubOracleCreateAccounts<'_, 'info>) -> Self {
        [
            accounts.payer.clone(),
            accounts.owner.clone(),
            accounts.oracle.clone(),
            accounts.mint.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; STUB_ORACLE_CREATE_IX_ACCOUNTS_LEN]>
for StubOracleCreateAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; STUB_ORACLE_CREATE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            payer: &arr[0],
            owner: &arr[1],
            oracle: &arr[2],
            mint: &arr[3],
            system_program: &arr[4],
        }
    }
}
pub const STUB_ORACLE_CREATE_IX_DISCM: [u8; 8usize] = [
    172, 63, 101, 83, 141, 76, 199, 216,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct StubOracleCreateIxArgs {
    pub price: f64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct StubOracleCreateIxData(pub StubOracleCreateIxArgs);
impl From<StubOracleCreateIxArgs> for StubOracleCreateIxData {
    fn from(args: StubOracleCreateIxArgs) -> Self {
        Self(args)
    }
}
impl StubOracleCreateIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != STUB_ORACLE_CREATE_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let price: f64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(Self(StubOracleCreateIxArgs { price }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&STUB_ORACLE_CREATE_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.price, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn stub_oracle_create_ix_with_program_id(
    program_id: Pubkey,
    keys: StubOracleCreateKeys,
    args: StubOracleCreateIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; STUB_ORACLE_CREATE_IX_ACCOUNTS_LEN] = keys.into();
    let data: StubOracleCreateIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn stub_oracle_create_ix(
    keys: StubOracleCreateKeys,
    args: StubOracleCreateIxArgs,
) -> std::io::Result<Instruction> {
    stub_oracle_create_ix_with_program_id(OPENBOOK_V2_PROGRAM_ID, keys, args)
}
pub fn stub_oracle_create_invoke_with_program_id(
    program_id: Pubkey,
    accounts: StubOracleCreateAccounts<'_, '_>,
    args: StubOracleCreateIxArgs,
) -> ProgramResult {
    let keys: StubOracleCreateKeys = accounts.into();
    let ix = stub_oracle_create_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn stub_oracle_create_invoke(
    accounts: StubOracleCreateAccounts<'_, '_>,
    args: StubOracleCreateIxArgs,
) -> ProgramResult {
    stub_oracle_create_invoke_with_program_id(OPENBOOK_V2_PROGRAM_ID, accounts, args)
}
pub fn stub_oracle_create_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: StubOracleCreateAccounts<'_, '_>,
    args: StubOracleCreateIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: StubOracleCreateKeys = accounts.into();
    let ix = stub_oracle_create_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn stub_oracle_create_invoke_signed(
    accounts: StubOracleCreateAccounts<'_, '_>,
    args: StubOracleCreateIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    stub_oracle_create_invoke_signed_with_program_id(
        OPENBOOK_V2_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn stub_oracle_create_verify_account_keys(
    accounts: StubOracleCreateAccounts<'_, '_>,
    keys: StubOracleCreateKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.payer.key, keys.payer),
        (*accounts.owner.key, keys.owner),
        (*accounts.oracle.key, keys.oracle),
        (*accounts.mint.key, keys.mint),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn stub_oracle_create_verify_writable_privileges<'me, 'info>(
    accounts: StubOracleCreateAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.payer, accounts.oracle] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn stub_oracle_create_verify_signer_privileges<'me, 'info>(
    accounts: StubOracleCreateAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.payer, accounts.owner] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn stub_oracle_create_verify_account_privileges<'me, 'info>(
    accounts: StubOracleCreateAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    stub_oracle_create_verify_writable_privileges(accounts)?;
    stub_oracle_create_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const STUB_ORACLE_CLOSE_IX_ACCOUNTS_LEN: usize = 4;
#[derive(Copy, Clone, Debug)]
pub struct StubOracleCloseAccounts<'me, 'info> {
    pub owner: &'me AccountInfo<'info>,
    pub oracle: &'me AccountInfo<'info>,
    pub sol_destination: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct StubOracleCloseKeys {
    pub owner: Pubkey,
    pub oracle: Pubkey,
    pub sol_destination: Pubkey,
    pub token_program: Pubkey,
}
impl From<StubOracleCloseAccounts<'_, '_>> for StubOracleCloseKeys {
    fn from(accounts: StubOracleCloseAccounts) -> Self {
        Self {
            owner: *accounts.owner.key,
            oracle: *accounts.oracle.key,
            sol_destination: *accounts.sol_destination.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<StubOracleCloseKeys> for [AccountMeta; STUB_ORACLE_CLOSE_IX_ACCOUNTS_LEN] {
    fn from(keys: StubOracleCloseKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.owner,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.oracle,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.sol_destination,
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
impl From<[Pubkey; STUB_ORACLE_CLOSE_IX_ACCOUNTS_LEN]> for StubOracleCloseKeys {
    fn from(pubkeys: [Pubkey; STUB_ORACLE_CLOSE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            owner: pubkeys[0],
            oracle: pubkeys[1],
            sol_destination: pubkeys[2],
            token_program: pubkeys[3],
        }
    }
}
impl<'info> From<StubOracleCloseAccounts<'_, 'info>>
for [AccountInfo<'info>; STUB_ORACLE_CLOSE_IX_ACCOUNTS_LEN] {
    fn from(accounts: StubOracleCloseAccounts<'_, 'info>) -> Self {
        [
            accounts.owner.clone(),
            accounts.oracle.clone(),
            accounts.sol_destination.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; STUB_ORACLE_CLOSE_IX_ACCOUNTS_LEN]>
for StubOracleCloseAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; STUB_ORACLE_CLOSE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            owner: &arr[0],
            oracle: &arr[1],
            sol_destination: &arr[2],
            token_program: &arr[3],
        }
    }
}
pub const STUB_ORACLE_CLOSE_IX_DISCM: [u8; 8usize] = [92, 137, 45, 3, 45, 60, 117, 224];
#[derive(Clone, Debug, PartialEq)]
pub struct StubOracleCloseIxData;
impl StubOracleCloseIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != STUB_ORACLE_CLOSE_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&STUB_ORACLE_CLOSE_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn stub_oracle_close_ix_with_program_id(
    program_id: Pubkey,
    keys: StubOracleCloseKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; STUB_ORACLE_CLOSE_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: StubOracleCloseIxData.try_to_vec()?,
    })
}
pub fn stub_oracle_close_ix(keys: StubOracleCloseKeys) -> std::io::Result<Instruction> {
    stub_oracle_close_ix_with_program_id(OPENBOOK_V2_PROGRAM_ID, keys)
}
pub fn stub_oracle_close_invoke_with_program_id(
    program_id: Pubkey,
    accounts: StubOracleCloseAccounts<'_, '_>,
) -> ProgramResult {
    let keys: StubOracleCloseKeys = accounts.into();
    let ix = stub_oracle_close_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn stub_oracle_close_invoke(
    accounts: StubOracleCloseAccounts<'_, '_>,
) -> ProgramResult {
    stub_oracle_close_invoke_with_program_id(OPENBOOK_V2_PROGRAM_ID, accounts)
}
pub fn stub_oracle_close_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: StubOracleCloseAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: StubOracleCloseKeys = accounts.into();
    let ix = stub_oracle_close_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn stub_oracle_close_invoke_signed(
    accounts: StubOracleCloseAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    stub_oracle_close_invoke_signed_with_program_id(
        OPENBOOK_V2_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn stub_oracle_close_verify_account_keys(
    accounts: StubOracleCloseAccounts<'_, '_>,
    keys: StubOracleCloseKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.owner.key, keys.owner),
        (*accounts.oracle.key, keys.oracle),
        (*accounts.sol_destination.key, keys.sol_destination),
        (*accounts.token_program.key, keys.token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn stub_oracle_close_verify_writable_privileges<'me, 'info>(
    accounts: StubOracleCloseAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.oracle, accounts.sol_destination] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn stub_oracle_close_verify_signer_privileges<'me, 'info>(
    accounts: StubOracleCloseAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.owner] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn stub_oracle_close_verify_account_privileges<'me, 'info>(
    accounts: StubOracleCloseAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    stub_oracle_close_verify_writable_privileges(accounts)?;
    stub_oracle_close_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const STUB_ORACLE_SET_IX_ACCOUNTS_LEN: usize = 2;
#[derive(Copy, Clone, Debug)]
pub struct StubOracleSetAccounts<'me, 'info> {
    pub owner: &'me AccountInfo<'info>,
    pub oracle: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct StubOracleSetKeys {
    pub owner: Pubkey,
    pub oracle: Pubkey,
}
impl From<StubOracleSetAccounts<'_, '_>> for StubOracleSetKeys {
    fn from(accounts: StubOracleSetAccounts) -> Self {
        Self {
            owner: *accounts.owner.key,
            oracle: *accounts.oracle.key,
        }
    }
}
impl From<StubOracleSetKeys> for [AccountMeta; STUB_ORACLE_SET_IX_ACCOUNTS_LEN] {
    fn from(keys: StubOracleSetKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.owner,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.oracle,
                is_signer: false,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; STUB_ORACLE_SET_IX_ACCOUNTS_LEN]> for StubOracleSetKeys {
    fn from(pubkeys: [Pubkey; STUB_ORACLE_SET_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            owner: pubkeys[0],
            oracle: pubkeys[1],
        }
    }
}
impl<'info> From<StubOracleSetAccounts<'_, 'info>>
for [AccountInfo<'info>; STUB_ORACLE_SET_IX_ACCOUNTS_LEN] {
    fn from(accounts: StubOracleSetAccounts<'_, 'info>) -> Self {
        [accounts.owner.clone(), accounts.oracle.clone()]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; STUB_ORACLE_SET_IX_ACCOUNTS_LEN]>
for StubOracleSetAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; STUB_ORACLE_SET_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            owner: &arr[0],
            oracle: &arr[1],
        }
    }
}
pub const STUB_ORACLE_SET_IX_DISCM: [u8; 8usize] = [
    109, 198, 79, 121, 65, 202, 161, 142,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct StubOracleSetIxArgs {
    pub price: f64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct StubOracleSetIxData(pub StubOracleSetIxArgs);
impl From<StubOracleSetIxArgs> for StubOracleSetIxData {
    fn from(args: StubOracleSetIxArgs) -> Self {
        Self(args)
    }
}
impl StubOracleSetIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != STUB_ORACLE_SET_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let price: f64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(Self(StubOracleSetIxArgs { price }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&STUB_ORACLE_SET_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.price, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn stub_oracle_set_ix_with_program_id(
    program_id: Pubkey,
    keys: StubOracleSetKeys,
    args: StubOracleSetIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; STUB_ORACLE_SET_IX_ACCOUNTS_LEN] = keys.into();
    let data: StubOracleSetIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn stub_oracle_set_ix(
    keys: StubOracleSetKeys,
    args: StubOracleSetIxArgs,
) -> std::io::Result<Instruction> {
    stub_oracle_set_ix_with_program_id(OPENBOOK_V2_PROGRAM_ID, keys, args)
}
pub fn stub_oracle_set_invoke_with_program_id(
    program_id: Pubkey,
    accounts: StubOracleSetAccounts<'_, '_>,
    args: StubOracleSetIxArgs,
) -> ProgramResult {
    let keys: StubOracleSetKeys = accounts.into();
    let ix = stub_oracle_set_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn stub_oracle_set_invoke(
    accounts: StubOracleSetAccounts<'_, '_>,
    args: StubOracleSetIxArgs,
) -> ProgramResult {
    stub_oracle_set_invoke_with_program_id(OPENBOOK_V2_PROGRAM_ID, accounts, args)
}
pub fn stub_oracle_set_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: StubOracleSetAccounts<'_, '_>,
    args: StubOracleSetIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: StubOracleSetKeys = accounts.into();
    let ix = stub_oracle_set_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn stub_oracle_set_invoke_signed(
    accounts: StubOracleSetAccounts<'_, '_>,
    args: StubOracleSetIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    stub_oracle_set_invoke_signed_with_program_id(
        OPENBOOK_V2_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn stub_oracle_set_verify_account_keys(
    accounts: StubOracleSetAccounts<'_, '_>,
    keys: StubOracleSetKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.owner.key, keys.owner),
        (*accounts.oracle.key, keys.oracle),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn stub_oracle_set_verify_writable_privileges<'me, 'info>(
    accounts: StubOracleSetAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.oracle] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn stub_oracle_set_verify_signer_privileges<'me, 'info>(
    accounts: StubOracleSetAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.owner] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn stub_oracle_set_verify_account_privileges<'me, 'info>(
    accounts: StubOracleSetAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    stub_oracle_set_verify_writable_privileges(accounts)?;
    stub_oracle_set_verify_signer_privileges(accounts)?;
    Ok(())
}
