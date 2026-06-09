use solana_pubkey::Pubkey;
use solana_cpi::{invoke, invoke_signed};
use solana_instruction::{AccountMeta, Instruction};
use solana_account_info::AccountInfo;
use solana_program_error::ProgramError;
use std::io::Read;
#[allow(unused_imports)]
use crate::*;
#[derive(Clone, Debug, PartialEq)]
pub enum AldrinProgramIx {
    InitializeMarket(InitializeMarketIxArgs),
    NewOrder(NewOrderIxArgs),
    MatchOrders(MatchOrdersIxArgs),
    ConsumeEvents(ConsumeEventsIxArgs),
    CancelOrder(CancelOrderIxArgs),
    SettleFunds,
    CancelOrderByClientId(CancelOrderByClientIdIxArgs),
    DisableMarket,
    SweepFees,
    NewOrderV2(NewOrderV2IxArgs),
    NewOrderV3(NewOrderV3IxArgs),
    CancelOrderV2(CancelOrderV2IxArgs),
    CancelOrderByClientIdV2(CancelOrderByClientIdV2IxArgs),
    SendTake(SendTakeIxArgs),
    CloseOpenOrders,
    InitOpenOrders,
    Prune(PruneIxArgs),
    ConsumeEventsPermissioned(ConsumeEventsPermissionedIxArgs),
}
impl AldrinProgramIx {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        if buf.starts_with(&INITIALIZE_MARKET_IX_DISCM) {
            let mut reader = &buf[INITIALIZE_MARKET_IX_DISCM.len()..];
            let args = if reader.is_empty() {
                Default::default()
            } else {
                <InitializeMarketInstruction>::deserialize(&mut reader)?
            };
            return Ok(Self::InitializeMarket(InitializeMarketIxArgs { args }));
        }
        if buf.starts_with(&NEW_ORDER_IX_DISCM) {
            let mut reader = &buf[NEW_ORDER_IX_DISCM.len()..];
            let args = if reader.is_empty() {
                Default::default()
            } else {
                <NewOrderInstructionV1>::deserialize(&mut reader)?
            };
            return Ok(Self::NewOrder(NewOrderIxArgs { args }));
        }
        if buf.starts_with(&MATCH_ORDERS_IX_DISCM) {
            let mut reader = &buf[MATCH_ORDERS_IX_DISCM.len()..];
            let limit: u16 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(Self::MatchOrders(MatchOrdersIxArgs { limit }));
        }
        if buf.starts_with(&CONSUME_EVENTS_IX_DISCM) {
            let mut reader = &buf[CONSUME_EVENTS_IX_DISCM.len()..];
            let limit: u16 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(Self::ConsumeEvents(ConsumeEventsIxArgs { limit }));
        }
        if buf.starts_with(&CANCEL_ORDER_IX_DISCM) {
            let mut reader = &buf[CANCEL_ORDER_IX_DISCM.len()..];
            let args = if reader.is_empty() {
                Default::default()
            } else {
                <CancelOrderInstructionV2>::deserialize(&mut reader)?
            };
            return Ok(Self::CancelOrder(CancelOrderIxArgs { args }));
        }
        if buf.starts_with(&SETTLE_FUNDS_IX_DISCM) {
            return Ok(Self::SettleFunds);
        }
        if buf.starts_with(&CANCEL_ORDER_BY_CLIENT_ID_IX_DISCM) {
            let mut reader = &buf[CANCEL_ORDER_BY_CLIENT_ID_IX_DISCM.len()..];
            let client_order_id: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::CancelOrderByClientId(CancelOrderByClientIdIxArgs {
                    client_order_id,
                }),
            );
        }
        if buf.starts_with(&DISABLE_MARKET_IX_DISCM) {
            return Ok(Self::DisableMarket);
        }
        if buf.starts_with(&SWEEP_FEES_IX_DISCM) {
            return Ok(Self::SweepFees);
        }
        if buf.starts_with(&NEW_ORDER_V2_IX_DISCM) {
            let mut reader = &buf[NEW_ORDER_V2_IX_DISCM.len()..];
            let args = if reader.is_empty() {
                Default::default()
            } else {
                <NewOrderInstructionV2>::deserialize(&mut reader)?
            };
            return Ok(Self::NewOrderV2(NewOrderV2IxArgs { args }));
        }
        if buf.starts_with(&NEW_ORDER_V3_IX_DISCM) {
            let mut reader = &buf[NEW_ORDER_V3_IX_DISCM.len()..];
            let args = if reader.is_empty() {
                Default::default()
            } else {
                <NewOrderInstructionV3>::deserialize(&mut reader)?
            };
            return Ok(Self::NewOrderV3(NewOrderV3IxArgs { args }));
        }
        if buf.starts_with(&CANCEL_ORDER_V2_IX_DISCM) {
            let mut reader = &buf[CANCEL_ORDER_V2_IX_DISCM.len()..];
            let args = if reader.is_empty() {
                Default::default()
            } else {
                <CancelOrderInstructionV2>::deserialize(&mut reader)?
            };
            return Ok(Self::CancelOrderV2(CancelOrderV2IxArgs { args }));
        }
        if buf.starts_with(&CANCEL_ORDER_BY_CLIENT_ID_V2_IX_DISCM) {
            let mut reader = &buf[CANCEL_ORDER_BY_CLIENT_ID_V2_IX_DISCM.len()..];
            let client_order_id: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::CancelOrderByClientIdV2(CancelOrderByClientIdV2IxArgs {
                    client_order_id,
                }),
            );
        }
        if buf.starts_with(&SEND_TAKE_IX_DISCM) {
            let mut reader = &buf[SEND_TAKE_IX_DISCM.len()..];
            let args = if reader.is_empty() {
                Default::default()
            } else {
                <SendTakeInstruction>::deserialize(&mut reader)?
            };
            return Ok(Self::SendTake(SendTakeIxArgs { args }));
        }
        if buf.starts_with(&CLOSE_OPEN_ORDERS_IX_DISCM) {
            return Ok(Self::CloseOpenOrders);
        }
        if buf.starts_with(&INIT_OPEN_ORDERS_IX_DISCM) {
            return Ok(Self::InitOpenOrders);
        }
        if buf.starts_with(&PRUNE_IX_DISCM) {
            let mut reader = &buf[PRUNE_IX_DISCM.len()..];
            let limit: u16 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(Self::Prune(PruneIxArgs { limit }));
        }
        if buf.starts_with(&CONSUME_EVENTS_PERMISSIONED_IX_DISCM) {
            let mut reader = &buf[CONSUME_EVENTS_PERMISSIONED_IX_DISCM.len()..];
            let limit: u16 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::ConsumeEventsPermissioned(ConsumeEventsPermissionedIxArgs {
                    limit,
                }),
            );
        }
        Err(std::io::Error::from(std::io::ErrorKind::InvalidData))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        match self {
            Self::InitializeMarket(args) => {
                writer.write_all(&INITIALIZE_MARKET_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.args, &mut writer)?;
                Ok(())
            }
            Self::NewOrder(args) => {
                writer.write_all(&NEW_ORDER_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.args, &mut writer)?;
                Ok(())
            }
            Self::MatchOrders(args) => {
                writer.write_all(&MATCH_ORDERS_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.limit, &mut writer)?;
                Ok(())
            }
            Self::ConsumeEvents(args) => {
                writer.write_all(&CONSUME_EVENTS_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.limit, &mut writer)?;
                Ok(())
            }
            Self::CancelOrder(args) => {
                writer.write_all(&CANCEL_ORDER_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.args, &mut writer)?;
                Ok(())
            }
            Self::SettleFunds => writer.write_all(&SETTLE_FUNDS_IX_DISCM),
            Self::CancelOrderByClientId(args) => {
                writer.write_all(&CANCEL_ORDER_BY_CLIENT_ID_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.client_order_id, &mut writer)?;
                Ok(())
            }
            Self::DisableMarket => writer.write_all(&DISABLE_MARKET_IX_DISCM),
            Self::SweepFees => writer.write_all(&SWEEP_FEES_IX_DISCM),
            Self::NewOrderV2(args) => {
                writer.write_all(&NEW_ORDER_V2_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.args, &mut writer)?;
                Ok(())
            }
            Self::NewOrderV3(args) => {
                writer.write_all(&NEW_ORDER_V3_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.args, &mut writer)?;
                Ok(())
            }
            Self::CancelOrderV2(args) => {
                writer.write_all(&CANCEL_ORDER_V2_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.args, &mut writer)?;
                Ok(())
            }
            Self::CancelOrderByClientIdV2(args) => {
                writer.write_all(&CANCEL_ORDER_BY_CLIENT_ID_V2_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.client_order_id, &mut writer)?;
                Ok(())
            }
            Self::SendTake(args) => {
                writer.write_all(&SEND_TAKE_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.args, &mut writer)?;
                Ok(())
            }
            Self::CloseOpenOrders => writer.write_all(&CLOSE_OPEN_ORDERS_IX_DISCM),
            Self::InitOpenOrders => writer.write_all(&INIT_OPEN_ORDERS_IX_DISCM),
            Self::Prune(args) => {
                writer.write_all(&PRUNE_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.limit, &mut writer)?;
                Ok(())
            }
            Self::ConsumeEventsPermissioned(args) => {
                writer.write_all(&CONSUME_EVENTS_PERMISSIONED_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.limit, &mut writer)?;
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
pub const INITIALIZE_MARKET_IX_ACCOUNTS_LEN: usize = 13;
#[derive(Copy, Clone, Debug)]
pub struct InitializeMarketAccounts<'me, 'info> {
    pub market_to_initialize: &'me AccountInfo<'info>,
    pub request_queue: &'me AccountInfo<'info>,
    pub event_queue: &'me AccountInfo<'info>,
    pub bids: &'me AccountInfo<'info>,
    pub asks: &'me AccountInfo<'info>,
    pub spl_token_account_coin: &'me AccountInfo<'info>,
    pub spl_token_account_price: &'me AccountInfo<'info>,
    pub coin_currency_mint: &'me AccountInfo<'info>,
    pub price_currency_mint: &'me AccountInfo<'info>,
    pub rent_sysvar: &'me AccountInfo<'info>,
    pub open_orders_market_authority: &'me AccountInfo<'info>,
    pub prune_authority: &'me AccountInfo<'info>,
    pub crank_authority: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct InitializeMarketKeys {
    pub market_to_initialize: Pubkey,
    pub request_queue: Pubkey,
    pub event_queue: Pubkey,
    pub bids: Pubkey,
    pub asks: Pubkey,
    pub spl_token_account_coin: Pubkey,
    pub spl_token_account_price: Pubkey,
    pub coin_currency_mint: Pubkey,
    pub price_currency_mint: Pubkey,
    pub rent_sysvar: Pubkey,
    pub open_orders_market_authority: Pubkey,
    pub prune_authority: Pubkey,
    pub crank_authority: Pubkey,
}
impl From<InitializeMarketAccounts<'_, '_>> for InitializeMarketKeys {
    fn from(accounts: InitializeMarketAccounts) -> Self {
        Self {
            market_to_initialize: *accounts.market_to_initialize.key,
            request_queue: *accounts.request_queue.key,
            event_queue: *accounts.event_queue.key,
            bids: *accounts.bids.key,
            asks: *accounts.asks.key,
            spl_token_account_coin: *accounts.spl_token_account_coin.key,
            spl_token_account_price: *accounts.spl_token_account_price.key,
            coin_currency_mint: *accounts.coin_currency_mint.key,
            price_currency_mint: *accounts.price_currency_mint.key,
            rent_sysvar: *accounts.rent_sysvar.key,
            open_orders_market_authority: *accounts.open_orders_market_authority.key,
            prune_authority: *accounts.prune_authority.key,
            crank_authority: *accounts.crank_authority.key,
        }
    }
}
impl From<InitializeMarketKeys> for [AccountMeta; INITIALIZE_MARKET_IX_ACCOUNTS_LEN] {
    fn from(keys: InitializeMarketKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.market_to_initialize,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.request_queue,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.event_queue,
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
                pubkey: keys.spl_token_account_coin,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.spl_token_account_price,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.coin_currency_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.price_currency_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.rent_sysvar,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.open_orders_market_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.prune_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.crank_authority,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; INITIALIZE_MARKET_IX_ACCOUNTS_LEN]> for InitializeMarketKeys {
    fn from(pubkeys: [Pubkey; INITIALIZE_MARKET_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            market_to_initialize: pubkeys[0],
            request_queue: pubkeys[1],
            event_queue: pubkeys[2],
            bids: pubkeys[3],
            asks: pubkeys[4],
            spl_token_account_coin: pubkeys[5],
            spl_token_account_price: pubkeys[6],
            coin_currency_mint: pubkeys[7],
            price_currency_mint: pubkeys[8],
            rent_sysvar: pubkeys[9],
            open_orders_market_authority: pubkeys[10],
            prune_authority: pubkeys[11],
            crank_authority: pubkeys[12],
        }
    }
}
impl<'info> From<InitializeMarketAccounts<'_, 'info>>
for [AccountInfo<'info>; INITIALIZE_MARKET_IX_ACCOUNTS_LEN] {
    fn from(accounts: InitializeMarketAccounts<'_, 'info>) -> Self {
        [
            accounts.market_to_initialize.clone(),
            accounts.request_queue.clone(),
            accounts.event_queue.clone(),
            accounts.bids.clone(),
            accounts.asks.clone(),
            accounts.spl_token_account_coin.clone(),
            accounts.spl_token_account_price.clone(),
            accounts.coin_currency_mint.clone(),
            accounts.price_currency_mint.clone(),
            accounts.rent_sysvar.clone(),
            accounts.open_orders_market_authority.clone(),
            accounts.prune_authority.clone(),
            accounts.crank_authority.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; INITIALIZE_MARKET_IX_ACCOUNTS_LEN]>
for InitializeMarketAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; INITIALIZE_MARKET_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            market_to_initialize: &arr[0],
            request_queue: &arr[1],
            event_queue: &arr[2],
            bids: &arr[3],
            asks: &arr[4],
            spl_token_account_coin: &arr[5],
            spl_token_account_price: &arr[6],
            coin_currency_mint: &arr[7],
            price_currency_mint: &arr[8],
            rent_sysvar: &arr[9],
            open_orders_market_authority: &arr[10],
            prune_authority: &arr[11],
            crank_authority: &arr[12],
        }
    }
}
pub const INITIALIZE_MARKET_IX_DISCM: [u8; 4usize] = [0, 0, 0, 0];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct InitializeMarketIxArgs {
    pub args: InitializeMarketInstruction,
}
#[derive(Clone, Debug, PartialEq)]
pub struct InitializeMarketIxData(pub InitializeMarketIxArgs);
impl From<InitializeMarketIxArgs> for InitializeMarketIxData {
    fn from(args: InitializeMarketIxArgs) -> Self {
        Self(args)
    }
}
impl InitializeMarketIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 4usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INITIALIZE_MARKET_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let args = if reader.is_empty() {
            Default::default()
        } else {
            <InitializeMarketInstruction>::deserialize(&mut reader)?
        };
        Ok(Self(InitializeMarketIxArgs { args }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INITIALIZE_MARKET_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.args, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn initialize_market_ix_with_program_id(
    program_id: Pubkey,
    keys: InitializeMarketKeys,
    args: InitializeMarketIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; INITIALIZE_MARKET_IX_ACCOUNTS_LEN] = keys.into();
    let data: InitializeMarketIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn initialize_market_ix(
    keys: InitializeMarketKeys,
    args: InitializeMarketIxArgs,
) -> std::io::Result<Instruction> {
    initialize_market_ix_with_program_id(ALDRIN_PROGRAM_ID, keys, args)
}
pub fn initialize_market_invoke_with_program_id(
    program_id: Pubkey,
    accounts: InitializeMarketAccounts<'_, '_>,
    args: InitializeMarketIxArgs,
) -> ProgramResult {
    let keys: InitializeMarketKeys = accounts.into();
    let ix = initialize_market_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn initialize_market_invoke(
    accounts: InitializeMarketAccounts<'_, '_>,
    args: InitializeMarketIxArgs,
) -> ProgramResult {
    initialize_market_invoke_with_program_id(ALDRIN_PROGRAM_ID, accounts, args)
}
pub fn initialize_market_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: InitializeMarketAccounts<'_, '_>,
    args: InitializeMarketIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: InitializeMarketKeys = accounts.into();
    let ix = initialize_market_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn initialize_market_invoke_signed(
    accounts: InitializeMarketAccounts<'_, '_>,
    args: InitializeMarketIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    initialize_market_invoke_signed_with_program_id(
        ALDRIN_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn initialize_market_verify_account_keys(
    accounts: InitializeMarketAccounts<'_, '_>,
    keys: InitializeMarketKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.market_to_initialize.key, keys.market_to_initialize),
        (*accounts.request_queue.key, keys.request_queue),
        (*accounts.event_queue.key, keys.event_queue),
        (*accounts.bids.key, keys.bids),
        (*accounts.asks.key, keys.asks),
        (*accounts.spl_token_account_coin.key, keys.spl_token_account_coin),
        (*accounts.spl_token_account_price.key, keys.spl_token_account_price),
        (*accounts.coin_currency_mint.key, keys.coin_currency_mint),
        (*accounts.price_currency_mint.key, keys.price_currency_mint),
        (*accounts.rent_sysvar.key, keys.rent_sysvar),
        (*accounts.open_orders_market_authority.key, keys.open_orders_market_authority),
        (*accounts.prune_authority.key, keys.prune_authority),
        (*accounts.crank_authority.key, keys.crank_authority),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn initialize_market_verify_writable_privileges<'me, 'info>(
    accounts: InitializeMarketAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.market_to_initialize,
        accounts.request_queue,
        accounts.event_queue,
        accounts.bids,
        accounts.asks,
        accounts.spl_token_account_coin,
        accounts.spl_token_account_price,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn initialize_market_verify_account_privileges<'me, 'info>(
    accounts: InitializeMarketAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    initialize_market_verify_writable_privileges(accounts)?;
    Ok(())
}
pub const NEW_ORDER_IX_ACCOUNTS_LEN: usize = 10;
#[derive(Copy, Clone, Debug)]
pub struct NewOrderAccounts<'me, 'info> {
    pub market: &'me AccountInfo<'info>,
    pub open_orders: &'me AccountInfo<'info>,
    pub request_queue: &'me AccountInfo<'info>,
    pub order_payer: &'me AccountInfo<'info>,
    pub owner: &'me AccountInfo<'info>,
    pub coin_vault: &'me AccountInfo<'info>,
    pub pc_vault: &'me AccountInfo<'info>,
    pub spl_token_program: &'me AccountInfo<'info>,
    pub rent_sysvar: &'me AccountInfo<'info>,
    pub fee_discounts: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct NewOrderKeys {
    pub market: Pubkey,
    pub open_orders: Pubkey,
    pub request_queue: Pubkey,
    pub order_payer: Pubkey,
    pub owner: Pubkey,
    pub coin_vault: Pubkey,
    pub pc_vault: Pubkey,
    pub spl_token_program: Pubkey,
    pub rent_sysvar: Pubkey,
    pub fee_discounts: Pubkey,
}
impl From<NewOrderAccounts<'_, '_>> for NewOrderKeys {
    fn from(accounts: NewOrderAccounts) -> Self {
        Self {
            market: *accounts.market.key,
            open_orders: *accounts.open_orders.key,
            request_queue: *accounts.request_queue.key,
            order_payer: *accounts.order_payer.key,
            owner: *accounts.owner.key,
            coin_vault: *accounts.coin_vault.key,
            pc_vault: *accounts.pc_vault.key,
            spl_token_program: *accounts.spl_token_program.key,
            rent_sysvar: *accounts.rent_sysvar.key,
            fee_discounts: *accounts.fee_discounts.key,
        }
    }
}
impl From<NewOrderKeys> for [AccountMeta; NEW_ORDER_IX_ACCOUNTS_LEN] {
    fn from(keys: NewOrderKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.market,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.open_orders,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.request_queue,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.order_payer,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.owner,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.coin_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pc_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.spl_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.rent_sysvar,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.fee_discounts,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; NEW_ORDER_IX_ACCOUNTS_LEN]> for NewOrderKeys {
    fn from(pubkeys: [Pubkey; NEW_ORDER_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            market: pubkeys[0],
            open_orders: pubkeys[1],
            request_queue: pubkeys[2],
            order_payer: pubkeys[3],
            owner: pubkeys[4],
            coin_vault: pubkeys[5],
            pc_vault: pubkeys[6],
            spl_token_program: pubkeys[7],
            rent_sysvar: pubkeys[8],
            fee_discounts: pubkeys[9],
        }
    }
}
impl<'info> From<NewOrderAccounts<'_, 'info>>
for [AccountInfo<'info>; NEW_ORDER_IX_ACCOUNTS_LEN] {
    fn from(accounts: NewOrderAccounts<'_, 'info>) -> Self {
        [
            accounts.market.clone(),
            accounts.open_orders.clone(),
            accounts.request_queue.clone(),
            accounts.order_payer.clone(),
            accounts.owner.clone(),
            accounts.coin_vault.clone(),
            accounts.pc_vault.clone(),
            accounts.spl_token_program.clone(),
            accounts.rent_sysvar.clone(),
            accounts.fee_discounts.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; NEW_ORDER_IX_ACCOUNTS_LEN]>
for NewOrderAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; NEW_ORDER_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            market: &arr[0],
            open_orders: &arr[1],
            request_queue: &arr[2],
            order_payer: &arr[3],
            owner: &arr[4],
            coin_vault: &arr[5],
            pc_vault: &arr[6],
            spl_token_program: &arr[7],
            rent_sysvar: &arr[8],
            fee_discounts: &arr[9],
        }
    }
}
pub const NEW_ORDER_IX_DISCM: [u8; 4usize] = [1, 0, 0, 0];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct NewOrderIxArgs {
    pub args: NewOrderInstructionV1,
}
#[derive(Clone, Debug, PartialEq)]
pub struct NewOrderIxData(pub NewOrderIxArgs);
impl From<NewOrderIxArgs> for NewOrderIxData {
    fn from(args: NewOrderIxArgs) -> Self {
        Self(args)
    }
}
impl NewOrderIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 4usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != NEW_ORDER_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let args = if reader.is_empty() {
            Default::default()
        } else {
            <NewOrderInstructionV1>::deserialize(&mut reader)?
        };
        Ok(Self(NewOrderIxArgs { args }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&NEW_ORDER_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.args, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn new_order_ix_with_program_id(
    program_id: Pubkey,
    keys: NewOrderKeys,
    args: NewOrderIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; NEW_ORDER_IX_ACCOUNTS_LEN] = keys.into();
    let data: NewOrderIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn new_order_ix(
    keys: NewOrderKeys,
    args: NewOrderIxArgs,
) -> std::io::Result<Instruction> {
    new_order_ix_with_program_id(ALDRIN_PROGRAM_ID, keys, args)
}
pub fn new_order_invoke_with_program_id(
    program_id: Pubkey,
    accounts: NewOrderAccounts<'_, '_>,
    args: NewOrderIxArgs,
) -> ProgramResult {
    let keys: NewOrderKeys = accounts.into();
    let ix = new_order_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn new_order_invoke(
    accounts: NewOrderAccounts<'_, '_>,
    args: NewOrderIxArgs,
) -> ProgramResult {
    new_order_invoke_with_program_id(ALDRIN_PROGRAM_ID, accounts, args)
}
pub fn new_order_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: NewOrderAccounts<'_, '_>,
    args: NewOrderIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: NewOrderKeys = accounts.into();
    let ix = new_order_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn new_order_invoke_signed(
    accounts: NewOrderAccounts<'_, '_>,
    args: NewOrderIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    new_order_invoke_signed_with_program_id(ALDRIN_PROGRAM_ID, accounts, args, seeds)
}
pub fn new_order_verify_account_keys(
    accounts: NewOrderAccounts<'_, '_>,
    keys: NewOrderKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.market.key, keys.market),
        (*accounts.open_orders.key, keys.open_orders),
        (*accounts.request_queue.key, keys.request_queue),
        (*accounts.order_payer.key, keys.order_payer),
        (*accounts.owner.key, keys.owner),
        (*accounts.coin_vault.key, keys.coin_vault),
        (*accounts.pc_vault.key, keys.pc_vault),
        (*accounts.spl_token_program.key, keys.spl_token_program),
        (*accounts.rent_sysvar.key, keys.rent_sysvar),
        (*accounts.fee_discounts.key, keys.fee_discounts),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn new_order_verify_writable_privileges<'me, 'info>(
    accounts: NewOrderAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.market,
        accounts.open_orders,
        accounts.request_queue,
        accounts.order_payer,
        accounts.coin_vault,
        accounts.pc_vault,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn new_order_verify_signer_privileges<'me, 'info>(
    accounts: NewOrderAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.owner] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn new_order_verify_account_privileges<'me, 'info>(
    accounts: NewOrderAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    new_order_verify_writable_privileges(accounts)?;
    new_order_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const MATCH_ORDERS_IX_ACCOUNTS_LEN: usize = 5;
#[derive(Copy, Clone, Debug)]
pub struct MatchOrdersAccounts<'me, 'info> {
    pub market: &'me AccountInfo<'info>,
    pub request_queue: &'me AccountInfo<'info>,
    pub event_queue: &'me AccountInfo<'info>,
    pub bids: &'me AccountInfo<'info>,
    pub asks: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MatchOrdersKeys {
    pub market: Pubkey,
    pub request_queue: Pubkey,
    pub event_queue: Pubkey,
    pub bids: Pubkey,
    pub asks: Pubkey,
}
impl From<MatchOrdersAccounts<'_, '_>> for MatchOrdersKeys {
    fn from(accounts: MatchOrdersAccounts) -> Self {
        Self {
            market: *accounts.market.key,
            request_queue: *accounts.request_queue.key,
            event_queue: *accounts.event_queue.key,
            bids: *accounts.bids.key,
            asks: *accounts.asks.key,
        }
    }
}
impl From<MatchOrdersKeys> for [AccountMeta; MATCH_ORDERS_IX_ACCOUNTS_LEN] {
    fn from(keys: MatchOrdersKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.market,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.request_queue,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.event_queue,
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
        ]
    }
}
impl From<[Pubkey; MATCH_ORDERS_IX_ACCOUNTS_LEN]> for MatchOrdersKeys {
    fn from(pubkeys: [Pubkey; MATCH_ORDERS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            market: pubkeys[0],
            request_queue: pubkeys[1],
            event_queue: pubkeys[2],
            bids: pubkeys[3],
            asks: pubkeys[4],
        }
    }
}
impl<'info> From<MatchOrdersAccounts<'_, 'info>>
for [AccountInfo<'info>; MATCH_ORDERS_IX_ACCOUNTS_LEN] {
    fn from(accounts: MatchOrdersAccounts<'_, 'info>) -> Self {
        [
            accounts.market.clone(),
            accounts.request_queue.clone(),
            accounts.event_queue.clone(),
            accounts.bids.clone(),
            accounts.asks.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; MATCH_ORDERS_IX_ACCOUNTS_LEN]>
for MatchOrdersAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; MATCH_ORDERS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            market: &arr[0],
            request_queue: &arr[1],
            event_queue: &arr[2],
            bids: &arr[3],
            asks: &arr[4],
        }
    }
}
pub const MATCH_ORDERS_IX_DISCM: [u8; 4usize] = [2, 0, 0, 0];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MatchOrdersIxArgs {
    pub limit: u16,
}
#[derive(Clone, Debug, PartialEq)]
pub struct MatchOrdersIxData(pub MatchOrdersIxArgs);
impl From<MatchOrdersIxArgs> for MatchOrdersIxData {
    fn from(args: MatchOrdersIxArgs) -> Self {
        Self(args)
    }
}
impl MatchOrdersIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 4usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != MATCH_ORDERS_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let limit: u16 = crate::borsh_de_or_default(&mut reader)?;
        Ok(Self(MatchOrdersIxArgs { limit }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&MATCH_ORDERS_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.limit, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn match_orders_ix_with_program_id(
    program_id: Pubkey,
    keys: MatchOrdersKeys,
    args: MatchOrdersIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; MATCH_ORDERS_IX_ACCOUNTS_LEN] = keys.into();
    let data: MatchOrdersIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn match_orders_ix(
    keys: MatchOrdersKeys,
    args: MatchOrdersIxArgs,
) -> std::io::Result<Instruction> {
    match_orders_ix_with_program_id(ALDRIN_PROGRAM_ID, keys, args)
}
pub fn match_orders_invoke_with_program_id(
    program_id: Pubkey,
    accounts: MatchOrdersAccounts<'_, '_>,
    args: MatchOrdersIxArgs,
) -> ProgramResult {
    let keys: MatchOrdersKeys = accounts.into();
    let ix = match_orders_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn match_orders_invoke(
    accounts: MatchOrdersAccounts<'_, '_>,
    args: MatchOrdersIxArgs,
) -> ProgramResult {
    match_orders_invoke_with_program_id(ALDRIN_PROGRAM_ID, accounts, args)
}
pub fn match_orders_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: MatchOrdersAccounts<'_, '_>,
    args: MatchOrdersIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: MatchOrdersKeys = accounts.into();
    let ix = match_orders_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn match_orders_invoke_signed(
    accounts: MatchOrdersAccounts<'_, '_>,
    args: MatchOrdersIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    match_orders_invoke_signed_with_program_id(ALDRIN_PROGRAM_ID, accounts, args, seeds)
}
pub fn match_orders_verify_account_keys(
    accounts: MatchOrdersAccounts<'_, '_>,
    keys: MatchOrdersKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.market.key, keys.market),
        (*accounts.request_queue.key, keys.request_queue),
        (*accounts.event_queue.key, keys.event_queue),
        (*accounts.bids.key, keys.bids),
        (*accounts.asks.key, keys.asks),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn match_orders_verify_writable_privileges<'me, 'info>(
    accounts: MatchOrdersAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.market,
        accounts.request_queue,
        accounts.event_queue,
        accounts.bids,
        accounts.asks,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn match_orders_verify_account_privileges<'me, 'info>(
    accounts: MatchOrdersAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    match_orders_verify_writable_privileges(accounts)?;
    Ok(())
}
pub const CONSUME_EVENTS_IX_ACCOUNTS_LEN: usize = 5;
#[derive(Copy, Clone, Debug)]
pub struct ConsumeEventsAccounts<'me, 'info> {
    pub open_orders: &'me AccountInfo<'info>,
    pub market: &'me AccountInfo<'info>,
    pub event_queue: &'me AccountInfo<'info>,
    pub coin_fee_receivable: &'me AccountInfo<'info>,
    pub pc_fee_receivable: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ConsumeEventsKeys {
    pub open_orders: Pubkey,
    pub market: Pubkey,
    pub event_queue: Pubkey,
    pub coin_fee_receivable: Pubkey,
    pub pc_fee_receivable: Pubkey,
}
impl From<ConsumeEventsAccounts<'_, '_>> for ConsumeEventsKeys {
    fn from(accounts: ConsumeEventsAccounts) -> Self {
        Self {
            open_orders: *accounts.open_orders.key,
            market: *accounts.market.key,
            event_queue: *accounts.event_queue.key,
            coin_fee_receivable: *accounts.coin_fee_receivable.key,
            pc_fee_receivable: *accounts.pc_fee_receivable.key,
        }
    }
}
impl From<ConsumeEventsKeys> for [AccountMeta; CONSUME_EVENTS_IX_ACCOUNTS_LEN] {
    fn from(keys: ConsumeEventsKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.open_orders,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.market,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.event_queue,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.coin_fee_receivable,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pc_fee_receivable,
                is_signer: false,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; CONSUME_EVENTS_IX_ACCOUNTS_LEN]> for ConsumeEventsKeys {
    fn from(pubkeys: [Pubkey; CONSUME_EVENTS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            open_orders: pubkeys[0],
            market: pubkeys[1],
            event_queue: pubkeys[2],
            coin_fee_receivable: pubkeys[3],
            pc_fee_receivable: pubkeys[4],
        }
    }
}
impl<'info> From<ConsumeEventsAccounts<'_, 'info>>
for [AccountInfo<'info>; CONSUME_EVENTS_IX_ACCOUNTS_LEN] {
    fn from(accounts: ConsumeEventsAccounts<'_, 'info>) -> Self {
        [
            accounts.open_orders.clone(),
            accounts.market.clone(),
            accounts.event_queue.clone(),
            accounts.coin_fee_receivable.clone(),
            accounts.pc_fee_receivable.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CONSUME_EVENTS_IX_ACCOUNTS_LEN]>
for ConsumeEventsAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; CONSUME_EVENTS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            open_orders: &arr[0],
            market: &arr[1],
            event_queue: &arr[2],
            coin_fee_receivable: &arr[3],
            pc_fee_receivable: &arr[4],
        }
    }
}
pub const CONSUME_EVENTS_IX_DISCM: [u8; 4usize] = [3, 0, 0, 0];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ConsumeEventsIxArgs {
    pub limit: u16,
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
        let mut maybe_discm = [0u8; 4usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CONSUME_EVENTS_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let limit: u16 = crate::borsh_de_or_default(&mut reader)?;
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
    consume_events_ix_with_program_id(ALDRIN_PROGRAM_ID, keys, args)
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
    consume_events_invoke_with_program_id(ALDRIN_PROGRAM_ID, accounts, args)
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
        ALDRIN_PROGRAM_ID,
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
        (*accounts.open_orders.key, keys.open_orders),
        (*accounts.market.key, keys.market),
        (*accounts.event_queue.key, keys.event_queue),
        (*accounts.coin_fee_receivable.key, keys.coin_fee_receivable),
        (*accounts.pc_fee_receivable.key, keys.pc_fee_receivable),
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
    for should_be_writable in [
        accounts.open_orders,
        accounts.market,
        accounts.event_queue,
        accounts.coin_fee_receivable,
        accounts.pc_fee_receivable,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn consume_events_verify_account_privileges<'me, 'info>(
    accounts: ConsumeEventsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    consume_events_verify_writable_privileges(accounts)?;
    Ok(())
}
pub const CANCEL_ORDER_IX_ACCOUNTS_LEN: usize = 4;
#[derive(Copy, Clone, Debug)]
pub struct CancelOrderAccounts<'me, 'info> {
    pub market: &'me AccountInfo<'info>,
    pub open_orders: &'me AccountInfo<'info>,
    pub request_queue: &'me AccountInfo<'info>,
    pub owner: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CancelOrderKeys {
    pub market: Pubkey,
    pub open_orders: Pubkey,
    pub request_queue: Pubkey,
    pub owner: Pubkey,
}
impl From<CancelOrderAccounts<'_, '_>> for CancelOrderKeys {
    fn from(accounts: CancelOrderAccounts) -> Self {
        Self {
            market: *accounts.market.key,
            open_orders: *accounts.open_orders.key,
            request_queue: *accounts.request_queue.key,
            owner: *accounts.owner.key,
        }
    }
}
impl From<CancelOrderKeys> for [AccountMeta; CANCEL_ORDER_IX_ACCOUNTS_LEN] {
    fn from(keys: CancelOrderKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.market,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.open_orders,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.request_queue,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.owner,
                is_signer: true,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; CANCEL_ORDER_IX_ACCOUNTS_LEN]> for CancelOrderKeys {
    fn from(pubkeys: [Pubkey; CANCEL_ORDER_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            market: pubkeys[0],
            open_orders: pubkeys[1],
            request_queue: pubkeys[2],
            owner: pubkeys[3],
        }
    }
}
impl<'info> From<CancelOrderAccounts<'_, 'info>>
for [AccountInfo<'info>; CANCEL_ORDER_IX_ACCOUNTS_LEN] {
    fn from(accounts: CancelOrderAccounts<'_, 'info>) -> Self {
        [
            accounts.market.clone(),
            accounts.open_orders.clone(),
            accounts.request_queue.clone(),
            accounts.owner.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CANCEL_ORDER_IX_ACCOUNTS_LEN]>
for CancelOrderAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; CANCEL_ORDER_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            market: &arr[0],
            open_orders: &arr[1],
            request_queue: &arr[2],
            owner: &arr[3],
        }
    }
}
pub const CANCEL_ORDER_IX_DISCM: [u8; 4usize] = [4, 0, 0, 0];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CancelOrderIxArgs {
    pub args: CancelOrderInstructionV2,
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
        let mut maybe_discm = [0u8; 4usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CANCEL_ORDER_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let args = if reader.is_empty() {
            Default::default()
        } else {
            <CancelOrderInstructionV2>::deserialize(&mut reader)?
        };
        Ok(Self(CancelOrderIxArgs { args }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CANCEL_ORDER_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.args, &mut writer)?;
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
    cancel_order_ix_with_program_id(ALDRIN_PROGRAM_ID, keys, args)
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
    cancel_order_invoke_with_program_id(ALDRIN_PROGRAM_ID, accounts, args)
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
    cancel_order_invoke_signed_with_program_id(ALDRIN_PROGRAM_ID, accounts, args, seeds)
}
pub fn cancel_order_verify_account_keys(
    accounts: CancelOrderAccounts<'_, '_>,
    keys: CancelOrderKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.market.key, keys.market),
        (*accounts.open_orders.key, keys.open_orders),
        (*accounts.request_queue.key, keys.request_queue),
        (*accounts.owner.key, keys.owner),
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
    for should_be_writable in [accounts.open_orders, accounts.request_queue] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn cancel_order_verify_signer_privileges<'me, 'info>(
    accounts: CancelOrderAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.owner] {
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
pub const SETTLE_FUNDS_IX_ACCOUNTS_LEN: usize = 10;
#[derive(Copy, Clone, Debug)]
pub struct SettleFundsAccounts<'me, 'info> {
    pub market: &'me AccountInfo<'info>,
    pub open_orders: &'me AccountInfo<'info>,
    pub owner: &'me AccountInfo<'info>,
    pub coin_vault: &'me AccountInfo<'info>,
    pub pc_vault: &'me AccountInfo<'info>,
    pub coin_wallet: &'me AccountInfo<'info>,
    pub pc_wallet: &'me AccountInfo<'info>,
    pub vault_signer: &'me AccountInfo<'info>,
    pub spl_token_program: &'me AccountInfo<'info>,
    pub referrer_pc_wallet: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SettleFundsKeys {
    pub market: Pubkey,
    pub open_orders: Pubkey,
    pub owner: Pubkey,
    pub coin_vault: Pubkey,
    pub pc_vault: Pubkey,
    pub coin_wallet: Pubkey,
    pub pc_wallet: Pubkey,
    pub vault_signer: Pubkey,
    pub spl_token_program: Pubkey,
    pub referrer_pc_wallet: Pubkey,
}
impl From<SettleFundsAccounts<'_, '_>> for SettleFundsKeys {
    fn from(accounts: SettleFundsAccounts) -> Self {
        Self {
            market: *accounts.market.key,
            open_orders: *accounts.open_orders.key,
            owner: *accounts.owner.key,
            coin_vault: *accounts.coin_vault.key,
            pc_vault: *accounts.pc_vault.key,
            coin_wallet: *accounts.coin_wallet.key,
            pc_wallet: *accounts.pc_wallet.key,
            vault_signer: *accounts.vault_signer.key,
            spl_token_program: *accounts.spl_token_program.key,
            referrer_pc_wallet: *accounts.referrer_pc_wallet.key,
        }
    }
}
impl From<SettleFundsKeys> for [AccountMeta; SETTLE_FUNDS_IX_ACCOUNTS_LEN] {
    fn from(keys: SettleFundsKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.market,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.open_orders,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.owner,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.coin_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pc_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.coin_wallet,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pc_wallet,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vault_signer,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.spl_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.referrer_pc_wallet,
                is_signer: false,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; SETTLE_FUNDS_IX_ACCOUNTS_LEN]> for SettleFundsKeys {
    fn from(pubkeys: [Pubkey; SETTLE_FUNDS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            market: pubkeys[0],
            open_orders: pubkeys[1],
            owner: pubkeys[2],
            coin_vault: pubkeys[3],
            pc_vault: pubkeys[4],
            coin_wallet: pubkeys[5],
            pc_wallet: pubkeys[6],
            vault_signer: pubkeys[7],
            spl_token_program: pubkeys[8],
            referrer_pc_wallet: pubkeys[9],
        }
    }
}
impl<'info> From<SettleFundsAccounts<'_, 'info>>
for [AccountInfo<'info>; SETTLE_FUNDS_IX_ACCOUNTS_LEN] {
    fn from(accounts: SettleFundsAccounts<'_, 'info>) -> Self {
        [
            accounts.market.clone(),
            accounts.open_orders.clone(),
            accounts.owner.clone(),
            accounts.coin_vault.clone(),
            accounts.pc_vault.clone(),
            accounts.coin_wallet.clone(),
            accounts.pc_wallet.clone(),
            accounts.vault_signer.clone(),
            accounts.spl_token_program.clone(),
            accounts.referrer_pc_wallet.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SETTLE_FUNDS_IX_ACCOUNTS_LEN]>
for SettleFundsAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; SETTLE_FUNDS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            market: &arr[0],
            open_orders: &arr[1],
            owner: &arr[2],
            coin_vault: &arr[3],
            pc_vault: &arr[4],
            coin_wallet: &arr[5],
            pc_wallet: &arr[6],
            vault_signer: &arr[7],
            spl_token_program: &arr[8],
            referrer_pc_wallet: &arr[9],
        }
    }
}
pub const SETTLE_FUNDS_IX_DISCM: [u8; 4usize] = [5, 0, 0, 0];
#[derive(Clone, Debug, PartialEq)]
pub struct SettleFundsIxData;
impl SettleFundsIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 4usize];
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
    settle_funds_ix_with_program_id(ALDRIN_PROGRAM_ID, keys)
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
    settle_funds_invoke_with_program_id(ALDRIN_PROGRAM_ID, accounts)
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
    settle_funds_invoke_signed_with_program_id(ALDRIN_PROGRAM_ID, accounts, seeds)
}
pub fn settle_funds_verify_account_keys(
    accounts: SettleFundsAccounts<'_, '_>,
    keys: SettleFundsKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.market.key, keys.market),
        (*accounts.open_orders.key, keys.open_orders),
        (*accounts.owner.key, keys.owner),
        (*accounts.coin_vault.key, keys.coin_vault),
        (*accounts.pc_vault.key, keys.pc_vault),
        (*accounts.coin_wallet.key, keys.coin_wallet),
        (*accounts.pc_wallet.key, keys.pc_wallet),
        (*accounts.vault_signer.key, keys.vault_signer),
        (*accounts.spl_token_program.key, keys.spl_token_program),
        (*accounts.referrer_pc_wallet.key, keys.referrer_pc_wallet),
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
        accounts.market,
        accounts.open_orders,
        accounts.coin_vault,
        accounts.pc_vault,
        accounts.coin_wallet,
        accounts.pc_wallet,
        accounts.referrer_pc_wallet,
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
    for should_be_signer in [accounts.owner] {
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
pub const CANCEL_ORDER_BY_CLIENT_ID_IX_ACCOUNTS_LEN: usize = 4;
#[derive(Copy, Clone, Debug)]
pub struct CancelOrderByClientIdAccounts<'me, 'info> {
    pub market: &'me AccountInfo<'info>,
    pub open_orders: &'me AccountInfo<'info>,
    pub request_queue: &'me AccountInfo<'info>,
    pub owner: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CancelOrderByClientIdKeys {
    pub market: Pubkey,
    pub open_orders: Pubkey,
    pub request_queue: Pubkey,
    pub owner: Pubkey,
}
impl From<CancelOrderByClientIdAccounts<'_, '_>> for CancelOrderByClientIdKeys {
    fn from(accounts: CancelOrderByClientIdAccounts) -> Self {
        Self {
            market: *accounts.market.key,
            open_orders: *accounts.open_orders.key,
            request_queue: *accounts.request_queue.key,
            owner: *accounts.owner.key,
        }
    }
}
impl From<CancelOrderByClientIdKeys>
for [AccountMeta; CANCEL_ORDER_BY_CLIENT_ID_IX_ACCOUNTS_LEN] {
    fn from(keys: CancelOrderByClientIdKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.market,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.open_orders,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.request_queue,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.owner,
                is_signer: true,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; CANCEL_ORDER_BY_CLIENT_ID_IX_ACCOUNTS_LEN]>
for CancelOrderByClientIdKeys {
    fn from(pubkeys: [Pubkey; CANCEL_ORDER_BY_CLIENT_ID_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            market: pubkeys[0],
            open_orders: pubkeys[1],
            request_queue: pubkeys[2],
            owner: pubkeys[3],
        }
    }
}
impl<'info> From<CancelOrderByClientIdAccounts<'_, 'info>>
for [AccountInfo<'info>; CANCEL_ORDER_BY_CLIENT_ID_IX_ACCOUNTS_LEN] {
    fn from(accounts: CancelOrderByClientIdAccounts<'_, 'info>) -> Self {
        [
            accounts.market.clone(),
            accounts.open_orders.clone(),
            accounts.request_queue.clone(),
            accounts.owner.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; CANCEL_ORDER_BY_CLIENT_ID_IX_ACCOUNTS_LEN]>
for CancelOrderByClientIdAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; CANCEL_ORDER_BY_CLIENT_ID_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            market: &arr[0],
            open_orders: &arr[1],
            request_queue: &arr[2],
            owner: &arr[3],
        }
    }
}
pub const CANCEL_ORDER_BY_CLIENT_ID_IX_DISCM: [u8; 4usize] = [6, 0, 0, 0];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CancelOrderByClientIdIxArgs {
    pub client_order_id: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct CancelOrderByClientIdIxData(pub CancelOrderByClientIdIxArgs);
impl From<CancelOrderByClientIdIxArgs> for CancelOrderByClientIdIxData {
    fn from(args: CancelOrderByClientIdIxArgs) -> Self {
        Self(args)
    }
}
impl CancelOrderByClientIdIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 4usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CANCEL_ORDER_BY_CLIENT_ID_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let client_order_id: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(CancelOrderByClientIdIxArgs {
                client_order_id,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CANCEL_ORDER_BY_CLIENT_ID_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.client_order_id, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn cancel_order_by_client_id_ix_with_program_id(
    program_id: Pubkey,
    keys: CancelOrderByClientIdKeys,
    args: CancelOrderByClientIdIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CANCEL_ORDER_BY_CLIENT_ID_IX_ACCOUNTS_LEN] = keys.into();
    let data: CancelOrderByClientIdIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn cancel_order_by_client_id_ix(
    keys: CancelOrderByClientIdKeys,
    args: CancelOrderByClientIdIxArgs,
) -> std::io::Result<Instruction> {
    cancel_order_by_client_id_ix_with_program_id(ALDRIN_PROGRAM_ID, keys, args)
}
pub fn cancel_order_by_client_id_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CancelOrderByClientIdAccounts<'_, '_>,
    args: CancelOrderByClientIdIxArgs,
) -> ProgramResult {
    let keys: CancelOrderByClientIdKeys = accounts.into();
    let ix = cancel_order_by_client_id_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn cancel_order_by_client_id_invoke(
    accounts: CancelOrderByClientIdAccounts<'_, '_>,
    args: CancelOrderByClientIdIxArgs,
) -> ProgramResult {
    cancel_order_by_client_id_invoke_with_program_id(ALDRIN_PROGRAM_ID, accounts, args)
}
pub fn cancel_order_by_client_id_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CancelOrderByClientIdAccounts<'_, '_>,
    args: CancelOrderByClientIdIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CancelOrderByClientIdKeys = accounts.into();
    let ix = cancel_order_by_client_id_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn cancel_order_by_client_id_invoke_signed(
    accounts: CancelOrderByClientIdAccounts<'_, '_>,
    args: CancelOrderByClientIdIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    cancel_order_by_client_id_invoke_signed_with_program_id(
        ALDRIN_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn cancel_order_by_client_id_verify_account_keys(
    accounts: CancelOrderByClientIdAccounts<'_, '_>,
    keys: CancelOrderByClientIdKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.market.key, keys.market),
        (*accounts.open_orders.key, keys.open_orders),
        (*accounts.request_queue.key, keys.request_queue),
        (*accounts.owner.key, keys.owner),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn cancel_order_by_client_id_verify_writable_privileges<'me, 'info>(
    accounts: CancelOrderByClientIdAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.open_orders, accounts.request_queue] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn cancel_order_by_client_id_verify_signer_privileges<'me, 'info>(
    accounts: CancelOrderByClientIdAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.owner] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn cancel_order_by_client_id_verify_account_privileges<'me, 'info>(
    accounts: CancelOrderByClientIdAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    cancel_order_by_client_id_verify_writable_privileges(accounts)?;
    cancel_order_by_client_id_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const DISABLE_MARKET_IX_ACCOUNTS_LEN: usize = 2;
#[derive(Copy, Clone, Debug)]
pub struct DisableMarketAccounts<'me, 'info> {
    pub market: &'me AccountInfo<'info>,
    pub disable_authority: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct DisableMarketKeys {
    pub market: Pubkey,
    pub disable_authority: Pubkey,
}
impl From<DisableMarketAccounts<'_, '_>> for DisableMarketKeys {
    fn from(accounts: DisableMarketAccounts) -> Self {
        Self {
            market: *accounts.market.key,
            disable_authority: *accounts.disable_authority.key,
        }
    }
}
impl From<DisableMarketKeys> for [AccountMeta; DISABLE_MARKET_IX_ACCOUNTS_LEN] {
    fn from(keys: DisableMarketKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.market,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.disable_authority,
                is_signer: true,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; DISABLE_MARKET_IX_ACCOUNTS_LEN]> for DisableMarketKeys {
    fn from(pubkeys: [Pubkey; DISABLE_MARKET_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            market: pubkeys[0],
            disable_authority: pubkeys[1],
        }
    }
}
impl<'info> From<DisableMarketAccounts<'_, 'info>>
for [AccountInfo<'info>; DISABLE_MARKET_IX_ACCOUNTS_LEN] {
    fn from(accounts: DisableMarketAccounts<'_, 'info>) -> Self {
        [accounts.market.clone(), accounts.disable_authority.clone()]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; DISABLE_MARKET_IX_ACCOUNTS_LEN]>
for DisableMarketAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; DISABLE_MARKET_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            market: &arr[0],
            disable_authority: &arr[1],
        }
    }
}
pub const DISABLE_MARKET_IX_DISCM: [u8; 4usize] = [7, 0, 0, 0];
#[derive(Clone, Debug, PartialEq)]
pub struct DisableMarketIxData;
impl DisableMarketIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 4usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != DISABLE_MARKET_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&DISABLE_MARKET_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn disable_market_ix_with_program_id(
    program_id: Pubkey,
    keys: DisableMarketKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; DISABLE_MARKET_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: DisableMarketIxData.try_to_vec()?,
    })
}
pub fn disable_market_ix(keys: DisableMarketKeys) -> std::io::Result<Instruction> {
    disable_market_ix_with_program_id(ALDRIN_PROGRAM_ID, keys)
}
pub fn disable_market_invoke_with_program_id(
    program_id: Pubkey,
    accounts: DisableMarketAccounts<'_, '_>,
) -> ProgramResult {
    let keys: DisableMarketKeys = accounts.into();
    let ix = disable_market_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn disable_market_invoke(accounts: DisableMarketAccounts<'_, '_>) -> ProgramResult {
    disable_market_invoke_with_program_id(ALDRIN_PROGRAM_ID, accounts)
}
pub fn disable_market_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: DisableMarketAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: DisableMarketKeys = accounts.into();
    let ix = disable_market_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn disable_market_invoke_signed(
    accounts: DisableMarketAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    disable_market_invoke_signed_with_program_id(ALDRIN_PROGRAM_ID, accounts, seeds)
}
pub fn disable_market_verify_account_keys(
    accounts: DisableMarketAccounts<'_, '_>,
    keys: DisableMarketKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.market.key, keys.market),
        (*accounts.disable_authority.key, keys.disable_authority),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn disable_market_verify_writable_privileges<'me, 'info>(
    accounts: DisableMarketAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.market] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn disable_market_verify_signer_privileges<'me, 'info>(
    accounts: DisableMarketAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.disable_authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn disable_market_verify_account_privileges<'me, 'info>(
    accounts: DisableMarketAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    disable_market_verify_writable_privileges(accounts)?;
    disable_market_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SWEEP_FEES_IX_ACCOUNTS_LEN: usize = 6;
#[derive(Copy, Clone, Debug)]
pub struct SweepFeesAccounts<'me, 'info> {
    pub market: &'me AccountInfo<'info>,
    pub pc_vault: &'me AccountInfo<'info>,
    pub fee_sweeping_authority: &'me AccountInfo<'info>,
    pub fee_receivable: &'me AccountInfo<'info>,
    pub vault_signer: &'me AccountInfo<'info>,
    pub spl_token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SweepFeesKeys {
    pub market: Pubkey,
    pub pc_vault: Pubkey,
    pub fee_sweeping_authority: Pubkey,
    pub fee_receivable: Pubkey,
    pub vault_signer: Pubkey,
    pub spl_token_program: Pubkey,
}
impl From<SweepFeesAccounts<'_, '_>> for SweepFeesKeys {
    fn from(accounts: SweepFeesAccounts) -> Self {
        Self {
            market: *accounts.market.key,
            pc_vault: *accounts.pc_vault.key,
            fee_sweeping_authority: *accounts.fee_sweeping_authority.key,
            fee_receivable: *accounts.fee_receivable.key,
            vault_signer: *accounts.vault_signer.key,
            spl_token_program: *accounts.spl_token_program.key,
        }
    }
}
impl From<SweepFeesKeys> for [AccountMeta; SWEEP_FEES_IX_ACCOUNTS_LEN] {
    fn from(keys: SweepFeesKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.market,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pc_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.fee_sweeping_authority,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.fee_receivable,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vault_signer,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.spl_token_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; SWEEP_FEES_IX_ACCOUNTS_LEN]> for SweepFeesKeys {
    fn from(pubkeys: [Pubkey; SWEEP_FEES_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            market: pubkeys[0],
            pc_vault: pubkeys[1],
            fee_sweeping_authority: pubkeys[2],
            fee_receivable: pubkeys[3],
            vault_signer: pubkeys[4],
            spl_token_program: pubkeys[5],
        }
    }
}
impl<'info> From<SweepFeesAccounts<'_, 'info>>
for [AccountInfo<'info>; SWEEP_FEES_IX_ACCOUNTS_LEN] {
    fn from(accounts: SweepFeesAccounts<'_, 'info>) -> Self {
        [
            accounts.market.clone(),
            accounts.pc_vault.clone(),
            accounts.fee_sweeping_authority.clone(),
            accounts.fee_receivable.clone(),
            accounts.vault_signer.clone(),
            accounts.spl_token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SWEEP_FEES_IX_ACCOUNTS_LEN]>
for SweepFeesAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; SWEEP_FEES_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            market: &arr[0],
            pc_vault: &arr[1],
            fee_sweeping_authority: &arr[2],
            fee_receivable: &arr[3],
            vault_signer: &arr[4],
            spl_token_program: &arr[5],
        }
    }
}
pub const SWEEP_FEES_IX_DISCM: [u8; 4usize] = [8, 0, 0, 0];
#[derive(Clone, Debug, PartialEq)]
pub struct SweepFeesIxData;
impl SweepFeesIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 4usize];
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
    sweep_fees_ix_with_program_id(ALDRIN_PROGRAM_ID, keys)
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
    sweep_fees_invoke_with_program_id(ALDRIN_PROGRAM_ID, accounts)
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
    sweep_fees_invoke_signed_with_program_id(ALDRIN_PROGRAM_ID, accounts, seeds)
}
pub fn sweep_fees_verify_account_keys(
    accounts: SweepFeesAccounts<'_, '_>,
    keys: SweepFeesKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.market.key, keys.market),
        (*accounts.pc_vault.key, keys.pc_vault),
        (*accounts.fee_sweeping_authority.key, keys.fee_sweeping_authority),
        (*accounts.fee_receivable.key, keys.fee_receivable),
        (*accounts.vault_signer.key, keys.vault_signer),
        (*accounts.spl_token_program.key, keys.spl_token_program),
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
        accounts.pc_vault,
        accounts.fee_receivable,
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
    for should_be_signer in [accounts.fee_sweeping_authority] {
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
pub const NEW_ORDER_V2_IX_ACCOUNTS_LEN: usize = 10;
#[derive(Copy, Clone, Debug)]
pub struct NewOrderV2Accounts<'me, 'info> {
    pub market: &'me AccountInfo<'info>,
    pub open_orders: &'me AccountInfo<'info>,
    pub request_queue: &'me AccountInfo<'info>,
    pub order_payer: &'me AccountInfo<'info>,
    pub owner: &'me AccountInfo<'info>,
    pub coin_vault: &'me AccountInfo<'info>,
    pub pc_vault: &'me AccountInfo<'info>,
    pub spl_token_program: &'me AccountInfo<'info>,
    pub rent_sysvar: &'me AccountInfo<'info>,
    pub fee_discounts: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct NewOrderV2Keys {
    pub market: Pubkey,
    pub open_orders: Pubkey,
    pub request_queue: Pubkey,
    pub order_payer: Pubkey,
    pub owner: Pubkey,
    pub coin_vault: Pubkey,
    pub pc_vault: Pubkey,
    pub spl_token_program: Pubkey,
    pub rent_sysvar: Pubkey,
    pub fee_discounts: Pubkey,
}
impl From<NewOrderV2Accounts<'_, '_>> for NewOrderV2Keys {
    fn from(accounts: NewOrderV2Accounts) -> Self {
        Self {
            market: *accounts.market.key,
            open_orders: *accounts.open_orders.key,
            request_queue: *accounts.request_queue.key,
            order_payer: *accounts.order_payer.key,
            owner: *accounts.owner.key,
            coin_vault: *accounts.coin_vault.key,
            pc_vault: *accounts.pc_vault.key,
            spl_token_program: *accounts.spl_token_program.key,
            rent_sysvar: *accounts.rent_sysvar.key,
            fee_discounts: *accounts.fee_discounts.key,
        }
    }
}
impl From<NewOrderV2Keys> for [AccountMeta; NEW_ORDER_V2_IX_ACCOUNTS_LEN] {
    fn from(keys: NewOrderV2Keys) -> Self {
        [
            AccountMeta {
                pubkey: keys.market,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.open_orders,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.request_queue,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.order_payer,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.owner,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.coin_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pc_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.spl_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.rent_sysvar,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.fee_discounts,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; NEW_ORDER_V2_IX_ACCOUNTS_LEN]> for NewOrderV2Keys {
    fn from(pubkeys: [Pubkey; NEW_ORDER_V2_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            market: pubkeys[0],
            open_orders: pubkeys[1],
            request_queue: pubkeys[2],
            order_payer: pubkeys[3],
            owner: pubkeys[4],
            coin_vault: pubkeys[5],
            pc_vault: pubkeys[6],
            spl_token_program: pubkeys[7],
            rent_sysvar: pubkeys[8],
            fee_discounts: pubkeys[9],
        }
    }
}
impl<'info> From<NewOrderV2Accounts<'_, 'info>>
for [AccountInfo<'info>; NEW_ORDER_V2_IX_ACCOUNTS_LEN] {
    fn from(accounts: NewOrderV2Accounts<'_, 'info>) -> Self {
        [
            accounts.market.clone(),
            accounts.open_orders.clone(),
            accounts.request_queue.clone(),
            accounts.order_payer.clone(),
            accounts.owner.clone(),
            accounts.coin_vault.clone(),
            accounts.pc_vault.clone(),
            accounts.spl_token_program.clone(),
            accounts.rent_sysvar.clone(),
            accounts.fee_discounts.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; NEW_ORDER_V2_IX_ACCOUNTS_LEN]>
for NewOrderV2Accounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; NEW_ORDER_V2_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            market: &arr[0],
            open_orders: &arr[1],
            request_queue: &arr[2],
            order_payer: &arr[3],
            owner: &arr[4],
            coin_vault: &arr[5],
            pc_vault: &arr[6],
            spl_token_program: &arr[7],
            rent_sysvar: &arr[8],
            fee_discounts: &arr[9],
        }
    }
}
pub const NEW_ORDER_V2_IX_DISCM: [u8; 4usize] = [9, 0, 0, 0];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct NewOrderV2IxArgs {
    pub args: NewOrderInstructionV2,
}
#[derive(Clone, Debug, PartialEq)]
pub struct NewOrderV2IxData(pub NewOrderV2IxArgs);
impl From<NewOrderV2IxArgs> for NewOrderV2IxData {
    fn from(args: NewOrderV2IxArgs) -> Self {
        Self(args)
    }
}
impl NewOrderV2IxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 4usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != NEW_ORDER_V2_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let args = if reader.is_empty() {
            Default::default()
        } else {
            <NewOrderInstructionV2>::deserialize(&mut reader)?
        };
        Ok(Self(NewOrderV2IxArgs { args }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&NEW_ORDER_V2_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.args, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn new_order_v2_ix_with_program_id(
    program_id: Pubkey,
    keys: NewOrderV2Keys,
    args: NewOrderV2IxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; NEW_ORDER_V2_IX_ACCOUNTS_LEN] = keys.into();
    let data: NewOrderV2IxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn new_order_v2_ix(
    keys: NewOrderV2Keys,
    args: NewOrderV2IxArgs,
) -> std::io::Result<Instruction> {
    new_order_v2_ix_with_program_id(ALDRIN_PROGRAM_ID, keys, args)
}
pub fn new_order_v2_invoke_with_program_id(
    program_id: Pubkey,
    accounts: NewOrderV2Accounts<'_, '_>,
    args: NewOrderV2IxArgs,
) -> ProgramResult {
    let keys: NewOrderV2Keys = accounts.into();
    let ix = new_order_v2_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn new_order_v2_invoke(
    accounts: NewOrderV2Accounts<'_, '_>,
    args: NewOrderV2IxArgs,
) -> ProgramResult {
    new_order_v2_invoke_with_program_id(ALDRIN_PROGRAM_ID, accounts, args)
}
pub fn new_order_v2_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: NewOrderV2Accounts<'_, '_>,
    args: NewOrderV2IxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: NewOrderV2Keys = accounts.into();
    let ix = new_order_v2_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn new_order_v2_invoke_signed(
    accounts: NewOrderV2Accounts<'_, '_>,
    args: NewOrderV2IxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    new_order_v2_invoke_signed_with_program_id(ALDRIN_PROGRAM_ID, accounts, args, seeds)
}
pub fn new_order_v2_verify_account_keys(
    accounts: NewOrderV2Accounts<'_, '_>,
    keys: NewOrderV2Keys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.market.key, keys.market),
        (*accounts.open_orders.key, keys.open_orders),
        (*accounts.request_queue.key, keys.request_queue),
        (*accounts.order_payer.key, keys.order_payer),
        (*accounts.owner.key, keys.owner),
        (*accounts.coin_vault.key, keys.coin_vault),
        (*accounts.pc_vault.key, keys.pc_vault),
        (*accounts.spl_token_program.key, keys.spl_token_program),
        (*accounts.rent_sysvar.key, keys.rent_sysvar),
        (*accounts.fee_discounts.key, keys.fee_discounts),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn new_order_v2_verify_writable_privileges<'me, 'info>(
    accounts: NewOrderV2Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.market,
        accounts.open_orders,
        accounts.request_queue,
        accounts.order_payer,
        accounts.coin_vault,
        accounts.pc_vault,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn new_order_v2_verify_signer_privileges<'me, 'info>(
    accounts: NewOrderV2Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.owner] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn new_order_v2_verify_account_privileges<'me, 'info>(
    accounts: NewOrderV2Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    new_order_v2_verify_writable_privileges(accounts)?;
    new_order_v2_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const NEW_ORDER_V3_IX_ACCOUNTS_LEN: usize = 13;
#[derive(Copy, Clone, Debug)]
pub struct NewOrderV3Accounts<'me, 'info> {
    pub market: &'me AccountInfo<'info>,
    pub open_orders: &'me AccountInfo<'info>,
    pub request_queue: &'me AccountInfo<'info>,
    pub event_queue: &'me AccountInfo<'info>,
    pub bids: &'me AccountInfo<'info>,
    pub asks: &'me AccountInfo<'info>,
    pub order_payer: &'me AccountInfo<'info>,
    pub owner: &'me AccountInfo<'info>,
    pub coin_vault: &'me AccountInfo<'info>,
    pub pc_vault: &'me AccountInfo<'info>,
    pub spl_token_program: &'me AccountInfo<'info>,
    pub rent_sysvar: &'me AccountInfo<'info>,
    pub fee_discounts: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct NewOrderV3Keys {
    pub market: Pubkey,
    pub open_orders: Pubkey,
    pub request_queue: Pubkey,
    pub event_queue: Pubkey,
    pub bids: Pubkey,
    pub asks: Pubkey,
    pub order_payer: Pubkey,
    pub owner: Pubkey,
    pub coin_vault: Pubkey,
    pub pc_vault: Pubkey,
    pub spl_token_program: Pubkey,
    pub rent_sysvar: Pubkey,
    pub fee_discounts: Pubkey,
}
impl From<NewOrderV3Accounts<'_, '_>> for NewOrderV3Keys {
    fn from(accounts: NewOrderV3Accounts) -> Self {
        Self {
            market: *accounts.market.key,
            open_orders: *accounts.open_orders.key,
            request_queue: *accounts.request_queue.key,
            event_queue: *accounts.event_queue.key,
            bids: *accounts.bids.key,
            asks: *accounts.asks.key,
            order_payer: *accounts.order_payer.key,
            owner: *accounts.owner.key,
            coin_vault: *accounts.coin_vault.key,
            pc_vault: *accounts.pc_vault.key,
            spl_token_program: *accounts.spl_token_program.key,
            rent_sysvar: *accounts.rent_sysvar.key,
            fee_discounts: *accounts.fee_discounts.key,
        }
    }
}
impl From<NewOrderV3Keys> for [AccountMeta; NEW_ORDER_V3_IX_ACCOUNTS_LEN] {
    fn from(keys: NewOrderV3Keys) -> Self {
        [
            AccountMeta {
                pubkey: keys.market,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.open_orders,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.request_queue,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.event_queue,
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
                pubkey: keys.order_payer,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.owner,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.coin_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pc_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.spl_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.rent_sysvar,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.fee_discounts,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; NEW_ORDER_V3_IX_ACCOUNTS_LEN]> for NewOrderV3Keys {
    fn from(pubkeys: [Pubkey; NEW_ORDER_V3_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            market: pubkeys[0],
            open_orders: pubkeys[1],
            request_queue: pubkeys[2],
            event_queue: pubkeys[3],
            bids: pubkeys[4],
            asks: pubkeys[5],
            order_payer: pubkeys[6],
            owner: pubkeys[7],
            coin_vault: pubkeys[8],
            pc_vault: pubkeys[9],
            spl_token_program: pubkeys[10],
            rent_sysvar: pubkeys[11],
            fee_discounts: pubkeys[12],
        }
    }
}
impl<'info> From<NewOrderV3Accounts<'_, 'info>>
for [AccountInfo<'info>; NEW_ORDER_V3_IX_ACCOUNTS_LEN] {
    fn from(accounts: NewOrderV3Accounts<'_, 'info>) -> Self {
        [
            accounts.market.clone(),
            accounts.open_orders.clone(),
            accounts.request_queue.clone(),
            accounts.event_queue.clone(),
            accounts.bids.clone(),
            accounts.asks.clone(),
            accounts.order_payer.clone(),
            accounts.owner.clone(),
            accounts.coin_vault.clone(),
            accounts.pc_vault.clone(),
            accounts.spl_token_program.clone(),
            accounts.rent_sysvar.clone(),
            accounts.fee_discounts.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; NEW_ORDER_V3_IX_ACCOUNTS_LEN]>
for NewOrderV3Accounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; NEW_ORDER_V3_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            market: &arr[0],
            open_orders: &arr[1],
            request_queue: &arr[2],
            event_queue: &arr[3],
            bids: &arr[4],
            asks: &arr[5],
            order_payer: &arr[6],
            owner: &arr[7],
            coin_vault: &arr[8],
            pc_vault: &arr[9],
            spl_token_program: &arr[10],
            rent_sysvar: &arr[11],
            fee_discounts: &arr[12],
        }
    }
}
pub const NEW_ORDER_V3_IX_DISCM: [u8; 4usize] = [10, 0, 0, 0];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct NewOrderV3IxArgs {
    pub args: NewOrderInstructionV3,
}
#[derive(Clone, Debug, PartialEq)]
pub struct NewOrderV3IxData(pub NewOrderV3IxArgs);
impl From<NewOrderV3IxArgs> for NewOrderV3IxData {
    fn from(args: NewOrderV3IxArgs) -> Self {
        Self(args)
    }
}
impl NewOrderV3IxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 4usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != NEW_ORDER_V3_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let args = if reader.is_empty() {
            Default::default()
        } else {
            <NewOrderInstructionV3>::deserialize(&mut reader)?
        };
        Ok(Self(NewOrderV3IxArgs { args }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&NEW_ORDER_V3_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.args, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn new_order_v3_ix_with_program_id(
    program_id: Pubkey,
    keys: NewOrderV3Keys,
    args: NewOrderV3IxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; NEW_ORDER_V3_IX_ACCOUNTS_LEN] = keys.into();
    let data: NewOrderV3IxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn new_order_v3_ix(
    keys: NewOrderV3Keys,
    args: NewOrderV3IxArgs,
) -> std::io::Result<Instruction> {
    new_order_v3_ix_with_program_id(ALDRIN_PROGRAM_ID, keys, args)
}
pub fn new_order_v3_invoke_with_program_id(
    program_id: Pubkey,
    accounts: NewOrderV3Accounts<'_, '_>,
    args: NewOrderV3IxArgs,
) -> ProgramResult {
    let keys: NewOrderV3Keys = accounts.into();
    let ix = new_order_v3_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn new_order_v3_invoke(
    accounts: NewOrderV3Accounts<'_, '_>,
    args: NewOrderV3IxArgs,
) -> ProgramResult {
    new_order_v3_invoke_with_program_id(ALDRIN_PROGRAM_ID, accounts, args)
}
pub fn new_order_v3_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: NewOrderV3Accounts<'_, '_>,
    args: NewOrderV3IxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: NewOrderV3Keys = accounts.into();
    let ix = new_order_v3_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn new_order_v3_invoke_signed(
    accounts: NewOrderV3Accounts<'_, '_>,
    args: NewOrderV3IxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    new_order_v3_invoke_signed_with_program_id(ALDRIN_PROGRAM_ID, accounts, args, seeds)
}
pub fn new_order_v3_verify_account_keys(
    accounts: NewOrderV3Accounts<'_, '_>,
    keys: NewOrderV3Keys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.market.key, keys.market),
        (*accounts.open_orders.key, keys.open_orders),
        (*accounts.request_queue.key, keys.request_queue),
        (*accounts.event_queue.key, keys.event_queue),
        (*accounts.bids.key, keys.bids),
        (*accounts.asks.key, keys.asks),
        (*accounts.order_payer.key, keys.order_payer),
        (*accounts.owner.key, keys.owner),
        (*accounts.coin_vault.key, keys.coin_vault),
        (*accounts.pc_vault.key, keys.pc_vault),
        (*accounts.spl_token_program.key, keys.spl_token_program),
        (*accounts.rent_sysvar.key, keys.rent_sysvar),
        (*accounts.fee_discounts.key, keys.fee_discounts),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn new_order_v3_verify_writable_privileges<'me, 'info>(
    accounts: NewOrderV3Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.market,
        accounts.open_orders,
        accounts.request_queue,
        accounts.event_queue,
        accounts.bids,
        accounts.asks,
        accounts.order_payer,
        accounts.coin_vault,
        accounts.pc_vault,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn new_order_v3_verify_signer_privileges<'me, 'info>(
    accounts: NewOrderV3Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.owner] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn new_order_v3_verify_account_privileges<'me, 'info>(
    accounts: NewOrderV3Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    new_order_v3_verify_writable_privileges(accounts)?;
    new_order_v3_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CANCEL_ORDER_V2_IX_ACCOUNTS_LEN: usize = 6;
#[derive(Copy, Clone, Debug)]
pub struct CancelOrderV2Accounts<'me, 'info> {
    pub market: &'me AccountInfo<'info>,
    pub bids: &'me AccountInfo<'info>,
    pub asks: &'me AccountInfo<'info>,
    pub open_orders: &'me AccountInfo<'info>,
    pub owner: &'me AccountInfo<'info>,
    pub event_queue: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CancelOrderV2Keys {
    pub market: Pubkey,
    pub bids: Pubkey,
    pub asks: Pubkey,
    pub open_orders: Pubkey,
    pub owner: Pubkey,
    pub event_queue: Pubkey,
}
impl From<CancelOrderV2Accounts<'_, '_>> for CancelOrderV2Keys {
    fn from(accounts: CancelOrderV2Accounts) -> Self {
        Self {
            market: *accounts.market.key,
            bids: *accounts.bids.key,
            asks: *accounts.asks.key,
            open_orders: *accounts.open_orders.key,
            owner: *accounts.owner.key,
            event_queue: *accounts.event_queue.key,
        }
    }
}
impl From<CancelOrderV2Keys> for [AccountMeta; CANCEL_ORDER_V2_IX_ACCOUNTS_LEN] {
    fn from(keys: CancelOrderV2Keys) -> Self {
        [
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
                pubkey: keys.open_orders,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.owner,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.event_queue,
                is_signer: false,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; CANCEL_ORDER_V2_IX_ACCOUNTS_LEN]> for CancelOrderV2Keys {
    fn from(pubkeys: [Pubkey; CANCEL_ORDER_V2_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            market: pubkeys[0],
            bids: pubkeys[1],
            asks: pubkeys[2],
            open_orders: pubkeys[3],
            owner: pubkeys[4],
            event_queue: pubkeys[5],
        }
    }
}
impl<'info> From<CancelOrderV2Accounts<'_, 'info>>
for [AccountInfo<'info>; CANCEL_ORDER_V2_IX_ACCOUNTS_LEN] {
    fn from(accounts: CancelOrderV2Accounts<'_, 'info>) -> Self {
        [
            accounts.market.clone(),
            accounts.bids.clone(),
            accounts.asks.clone(),
            accounts.open_orders.clone(),
            accounts.owner.clone(),
            accounts.event_queue.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CANCEL_ORDER_V2_IX_ACCOUNTS_LEN]>
for CancelOrderV2Accounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; CANCEL_ORDER_V2_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            market: &arr[0],
            bids: &arr[1],
            asks: &arr[2],
            open_orders: &arr[3],
            owner: &arr[4],
            event_queue: &arr[5],
        }
    }
}
pub const CANCEL_ORDER_V2_IX_DISCM: [u8; 4usize] = [11, 0, 0, 0];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CancelOrderV2IxArgs {
    pub args: CancelOrderInstructionV2,
}
#[derive(Clone, Debug, PartialEq)]
pub struct CancelOrderV2IxData(pub CancelOrderV2IxArgs);
impl From<CancelOrderV2IxArgs> for CancelOrderV2IxData {
    fn from(args: CancelOrderV2IxArgs) -> Self {
        Self(args)
    }
}
impl CancelOrderV2IxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 4usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CANCEL_ORDER_V2_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let args = if reader.is_empty() {
            Default::default()
        } else {
            <CancelOrderInstructionV2>::deserialize(&mut reader)?
        };
        Ok(Self(CancelOrderV2IxArgs { args }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CANCEL_ORDER_V2_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.args, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn cancel_order_v2_ix_with_program_id(
    program_id: Pubkey,
    keys: CancelOrderV2Keys,
    args: CancelOrderV2IxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CANCEL_ORDER_V2_IX_ACCOUNTS_LEN] = keys.into();
    let data: CancelOrderV2IxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn cancel_order_v2_ix(
    keys: CancelOrderV2Keys,
    args: CancelOrderV2IxArgs,
) -> std::io::Result<Instruction> {
    cancel_order_v2_ix_with_program_id(ALDRIN_PROGRAM_ID, keys, args)
}
pub fn cancel_order_v2_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CancelOrderV2Accounts<'_, '_>,
    args: CancelOrderV2IxArgs,
) -> ProgramResult {
    let keys: CancelOrderV2Keys = accounts.into();
    let ix = cancel_order_v2_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn cancel_order_v2_invoke(
    accounts: CancelOrderV2Accounts<'_, '_>,
    args: CancelOrderV2IxArgs,
) -> ProgramResult {
    cancel_order_v2_invoke_with_program_id(ALDRIN_PROGRAM_ID, accounts, args)
}
pub fn cancel_order_v2_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CancelOrderV2Accounts<'_, '_>,
    args: CancelOrderV2IxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CancelOrderV2Keys = accounts.into();
    let ix = cancel_order_v2_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn cancel_order_v2_invoke_signed(
    accounts: CancelOrderV2Accounts<'_, '_>,
    args: CancelOrderV2IxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    cancel_order_v2_invoke_signed_with_program_id(
        ALDRIN_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn cancel_order_v2_verify_account_keys(
    accounts: CancelOrderV2Accounts<'_, '_>,
    keys: CancelOrderV2Keys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.market.key, keys.market),
        (*accounts.bids.key, keys.bids),
        (*accounts.asks.key, keys.asks),
        (*accounts.open_orders.key, keys.open_orders),
        (*accounts.owner.key, keys.owner),
        (*accounts.event_queue.key, keys.event_queue),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn cancel_order_v2_verify_writable_privileges<'me, 'info>(
    accounts: CancelOrderV2Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.market,
        accounts.bids,
        accounts.asks,
        accounts.open_orders,
        accounts.event_queue,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn cancel_order_v2_verify_signer_privileges<'me, 'info>(
    accounts: CancelOrderV2Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.owner] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn cancel_order_v2_verify_account_privileges<'me, 'info>(
    accounts: CancelOrderV2Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    cancel_order_v2_verify_writable_privileges(accounts)?;
    cancel_order_v2_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CANCEL_ORDER_BY_CLIENT_ID_V2_IX_ACCOUNTS_LEN: usize = 6;
#[derive(Copy, Clone, Debug)]
pub struct CancelOrderByClientIdV2Accounts<'me, 'info> {
    pub market: &'me AccountInfo<'info>,
    pub bids: &'me AccountInfo<'info>,
    pub asks: &'me AccountInfo<'info>,
    pub open_orders: &'me AccountInfo<'info>,
    pub owner: &'me AccountInfo<'info>,
    pub event_queue: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CancelOrderByClientIdV2Keys {
    pub market: Pubkey,
    pub bids: Pubkey,
    pub asks: Pubkey,
    pub open_orders: Pubkey,
    pub owner: Pubkey,
    pub event_queue: Pubkey,
}
impl From<CancelOrderByClientIdV2Accounts<'_, '_>> for CancelOrderByClientIdV2Keys {
    fn from(accounts: CancelOrderByClientIdV2Accounts) -> Self {
        Self {
            market: *accounts.market.key,
            bids: *accounts.bids.key,
            asks: *accounts.asks.key,
            open_orders: *accounts.open_orders.key,
            owner: *accounts.owner.key,
            event_queue: *accounts.event_queue.key,
        }
    }
}
impl From<CancelOrderByClientIdV2Keys>
for [AccountMeta; CANCEL_ORDER_BY_CLIENT_ID_V2_IX_ACCOUNTS_LEN] {
    fn from(keys: CancelOrderByClientIdV2Keys) -> Self {
        [
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
                pubkey: keys.open_orders,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.owner,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.event_queue,
                is_signer: false,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; CANCEL_ORDER_BY_CLIENT_ID_V2_IX_ACCOUNTS_LEN]>
for CancelOrderByClientIdV2Keys {
    fn from(pubkeys: [Pubkey; CANCEL_ORDER_BY_CLIENT_ID_V2_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            market: pubkeys[0],
            bids: pubkeys[1],
            asks: pubkeys[2],
            open_orders: pubkeys[3],
            owner: pubkeys[4],
            event_queue: pubkeys[5],
        }
    }
}
impl<'info> From<CancelOrderByClientIdV2Accounts<'_, 'info>>
for [AccountInfo<'info>; CANCEL_ORDER_BY_CLIENT_ID_V2_IX_ACCOUNTS_LEN] {
    fn from(accounts: CancelOrderByClientIdV2Accounts<'_, 'info>) -> Self {
        [
            accounts.market.clone(),
            accounts.bids.clone(),
            accounts.asks.clone(),
            accounts.open_orders.clone(),
            accounts.owner.clone(),
            accounts.event_queue.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; CANCEL_ORDER_BY_CLIENT_ID_V2_IX_ACCOUNTS_LEN]>
for CancelOrderByClientIdV2Accounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; CANCEL_ORDER_BY_CLIENT_ID_V2_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            market: &arr[0],
            bids: &arr[1],
            asks: &arr[2],
            open_orders: &arr[3],
            owner: &arr[4],
            event_queue: &arr[5],
        }
    }
}
pub const CANCEL_ORDER_BY_CLIENT_ID_V2_IX_DISCM: [u8; 4usize] = [12, 0, 0, 0];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CancelOrderByClientIdV2IxArgs {
    pub client_order_id: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct CancelOrderByClientIdV2IxData(pub CancelOrderByClientIdV2IxArgs);
impl From<CancelOrderByClientIdV2IxArgs> for CancelOrderByClientIdV2IxData {
    fn from(args: CancelOrderByClientIdV2IxArgs) -> Self {
        Self(args)
    }
}
impl CancelOrderByClientIdV2IxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 4usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CANCEL_ORDER_BY_CLIENT_ID_V2_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let client_order_id: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(CancelOrderByClientIdV2IxArgs {
                client_order_id,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CANCEL_ORDER_BY_CLIENT_ID_V2_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.client_order_id, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn cancel_order_by_client_id_v2_ix_with_program_id(
    program_id: Pubkey,
    keys: CancelOrderByClientIdV2Keys,
    args: CancelOrderByClientIdV2IxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CANCEL_ORDER_BY_CLIENT_ID_V2_IX_ACCOUNTS_LEN] = keys.into();
    let data: CancelOrderByClientIdV2IxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn cancel_order_by_client_id_v2_ix(
    keys: CancelOrderByClientIdV2Keys,
    args: CancelOrderByClientIdV2IxArgs,
) -> std::io::Result<Instruction> {
    cancel_order_by_client_id_v2_ix_with_program_id(ALDRIN_PROGRAM_ID, keys, args)
}
pub fn cancel_order_by_client_id_v2_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CancelOrderByClientIdV2Accounts<'_, '_>,
    args: CancelOrderByClientIdV2IxArgs,
) -> ProgramResult {
    let keys: CancelOrderByClientIdV2Keys = accounts.into();
    let ix = cancel_order_by_client_id_v2_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn cancel_order_by_client_id_v2_invoke(
    accounts: CancelOrderByClientIdV2Accounts<'_, '_>,
    args: CancelOrderByClientIdV2IxArgs,
) -> ProgramResult {
    cancel_order_by_client_id_v2_invoke_with_program_id(
        ALDRIN_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn cancel_order_by_client_id_v2_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CancelOrderByClientIdV2Accounts<'_, '_>,
    args: CancelOrderByClientIdV2IxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CancelOrderByClientIdV2Keys = accounts.into();
    let ix = cancel_order_by_client_id_v2_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn cancel_order_by_client_id_v2_invoke_signed(
    accounts: CancelOrderByClientIdV2Accounts<'_, '_>,
    args: CancelOrderByClientIdV2IxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    cancel_order_by_client_id_v2_invoke_signed_with_program_id(
        ALDRIN_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn cancel_order_by_client_id_v2_verify_account_keys(
    accounts: CancelOrderByClientIdV2Accounts<'_, '_>,
    keys: CancelOrderByClientIdV2Keys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.market.key, keys.market),
        (*accounts.bids.key, keys.bids),
        (*accounts.asks.key, keys.asks),
        (*accounts.open_orders.key, keys.open_orders),
        (*accounts.owner.key, keys.owner),
        (*accounts.event_queue.key, keys.event_queue),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn cancel_order_by_client_id_v2_verify_writable_privileges<'me, 'info>(
    accounts: CancelOrderByClientIdV2Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.market,
        accounts.bids,
        accounts.asks,
        accounts.open_orders,
        accounts.event_queue,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn cancel_order_by_client_id_v2_verify_signer_privileges<'me, 'info>(
    accounts: CancelOrderByClientIdV2Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.owner] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn cancel_order_by_client_id_v2_verify_account_privileges<'me, 'info>(
    accounts: CancelOrderByClientIdV2Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    cancel_order_by_client_id_v2_verify_writable_privileges(accounts)?;
    cancel_order_by_client_id_v2_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SEND_TAKE_IX_ACCOUNTS_LEN: usize = 13;
#[derive(Copy, Clone, Debug)]
pub struct SendTakeAccounts<'me, 'info> {
    pub market: &'me AccountInfo<'info>,
    pub request_queue: &'me AccountInfo<'info>,
    pub event_queue: &'me AccountInfo<'info>,
    pub bids: &'me AccountInfo<'info>,
    pub asks: &'me AccountInfo<'info>,
    pub coin_currency_wallet: &'me AccountInfo<'info>,
    pub price_currency_wallet: &'me AccountInfo<'info>,
    pub signer: &'me AccountInfo<'info>,
    pub coin_vault: &'me AccountInfo<'info>,
    pub pc_vault: &'me AccountInfo<'info>,
    pub spl_token_program: &'me AccountInfo<'info>,
    pub vault_signer: &'me AccountInfo<'info>,
    pub fee_discounts: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SendTakeKeys {
    pub market: Pubkey,
    pub request_queue: Pubkey,
    pub event_queue: Pubkey,
    pub bids: Pubkey,
    pub asks: Pubkey,
    pub coin_currency_wallet: Pubkey,
    pub price_currency_wallet: Pubkey,
    pub signer: Pubkey,
    pub coin_vault: Pubkey,
    pub pc_vault: Pubkey,
    pub spl_token_program: Pubkey,
    pub vault_signer: Pubkey,
    pub fee_discounts: Pubkey,
}
impl From<SendTakeAccounts<'_, '_>> for SendTakeKeys {
    fn from(accounts: SendTakeAccounts) -> Self {
        Self {
            market: *accounts.market.key,
            request_queue: *accounts.request_queue.key,
            event_queue: *accounts.event_queue.key,
            bids: *accounts.bids.key,
            asks: *accounts.asks.key,
            coin_currency_wallet: *accounts.coin_currency_wallet.key,
            price_currency_wallet: *accounts.price_currency_wallet.key,
            signer: *accounts.signer.key,
            coin_vault: *accounts.coin_vault.key,
            pc_vault: *accounts.pc_vault.key,
            spl_token_program: *accounts.spl_token_program.key,
            vault_signer: *accounts.vault_signer.key,
            fee_discounts: *accounts.fee_discounts.key,
        }
    }
}
impl From<SendTakeKeys> for [AccountMeta; SEND_TAKE_IX_ACCOUNTS_LEN] {
    fn from(keys: SendTakeKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.market,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.request_queue,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.event_queue,
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
                pubkey: keys.coin_currency_wallet,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.price_currency_wallet,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.signer,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.coin_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pc_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.spl_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault_signer,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.fee_discounts,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; SEND_TAKE_IX_ACCOUNTS_LEN]> for SendTakeKeys {
    fn from(pubkeys: [Pubkey; SEND_TAKE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            market: pubkeys[0],
            request_queue: pubkeys[1],
            event_queue: pubkeys[2],
            bids: pubkeys[3],
            asks: pubkeys[4],
            coin_currency_wallet: pubkeys[5],
            price_currency_wallet: pubkeys[6],
            signer: pubkeys[7],
            coin_vault: pubkeys[8],
            pc_vault: pubkeys[9],
            spl_token_program: pubkeys[10],
            vault_signer: pubkeys[11],
            fee_discounts: pubkeys[12],
        }
    }
}
impl<'info> From<SendTakeAccounts<'_, 'info>>
for [AccountInfo<'info>; SEND_TAKE_IX_ACCOUNTS_LEN] {
    fn from(accounts: SendTakeAccounts<'_, 'info>) -> Self {
        [
            accounts.market.clone(),
            accounts.request_queue.clone(),
            accounts.event_queue.clone(),
            accounts.bids.clone(),
            accounts.asks.clone(),
            accounts.coin_currency_wallet.clone(),
            accounts.price_currency_wallet.clone(),
            accounts.signer.clone(),
            accounts.coin_vault.clone(),
            accounts.pc_vault.clone(),
            accounts.spl_token_program.clone(),
            accounts.vault_signer.clone(),
            accounts.fee_discounts.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SEND_TAKE_IX_ACCOUNTS_LEN]>
for SendTakeAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; SEND_TAKE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            market: &arr[0],
            request_queue: &arr[1],
            event_queue: &arr[2],
            bids: &arr[3],
            asks: &arr[4],
            coin_currency_wallet: &arr[5],
            price_currency_wallet: &arr[6],
            signer: &arr[7],
            coin_vault: &arr[8],
            pc_vault: &arr[9],
            spl_token_program: &arr[10],
            vault_signer: &arr[11],
            fee_discounts: &arr[12],
        }
    }
}
pub const SEND_TAKE_IX_DISCM: [u8; 4usize] = [13, 0, 0, 0];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SendTakeIxArgs {
    pub args: SendTakeInstruction,
}
#[derive(Clone, Debug, PartialEq)]
pub struct SendTakeIxData(pub SendTakeIxArgs);
impl From<SendTakeIxArgs> for SendTakeIxData {
    fn from(args: SendTakeIxArgs) -> Self {
        Self(args)
    }
}
impl SendTakeIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 4usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SEND_TAKE_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let args = if reader.is_empty() {
            Default::default()
        } else {
            <SendTakeInstruction>::deserialize(&mut reader)?
        };
        Ok(Self(SendTakeIxArgs { args }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SEND_TAKE_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.args, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn send_take_ix_with_program_id(
    program_id: Pubkey,
    keys: SendTakeKeys,
    args: SendTakeIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SEND_TAKE_IX_ACCOUNTS_LEN] = keys.into();
    let data: SendTakeIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn send_take_ix(
    keys: SendTakeKeys,
    args: SendTakeIxArgs,
) -> std::io::Result<Instruction> {
    send_take_ix_with_program_id(ALDRIN_PROGRAM_ID, keys, args)
}
pub fn send_take_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SendTakeAccounts<'_, '_>,
    args: SendTakeIxArgs,
) -> ProgramResult {
    let keys: SendTakeKeys = accounts.into();
    let ix = send_take_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn send_take_invoke(
    accounts: SendTakeAccounts<'_, '_>,
    args: SendTakeIxArgs,
) -> ProgramResult {
    send_take_invoke_with_program_id(ALDRIN_PROGRAM_ID, accounts, args)
}
pub fn send_take_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SendTakeAccounts<'_, '_>,
    args: SendTakeIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SendTakeKeys = accounts.into();
    let ix = send_take_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn send_take_invoke_signed(
    accounts: SendTakeAccounts<'_, '_>,
    args: SendTakeIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    send_take_invoke_signed_with_program_id(ALDRIN_PROGRAM_ID, accounts, args, seeds)
}
pub fn send_take_verify_account_keys(
    accounts: SendTakeAccounts<'_, '_>,
    keys: SendTakeKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.market.key, keys.market),
        (*accounts.request_queue.key, keys.request_queue),
        (*accounts.event_queue.key, keys.event_queue),
        (*accounts.bids.key, keys.bids),
        (*accounts.asks.key, keys.asks),
        (*accounts.coin_currency_wallet.key, keys.coin_currency_wallet),
        (*accounts.price_currency_wallet.key, keys.price_currency_wallet),
        (*accounts.signer.key, keys.signer),
        (*accounts.coin_vault.key, keys.coin_vault),
        (*accounts.pc_vault.key, keys.pc_vault),
        (*accounts.spl_token_program.key, keys.spl_token_program),
        (*accounts.vault_signer.key, keys.vault_signer),
        (*accounts.fee_discounts.key, keys.fee_discounts),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn send_take_verify_writable_privileges<'me, 'info>(
    accounts: SendTakeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.market,
        accounts.request_queue,
        accounts.event_queue,
        accounts.bids,
        accounts.asks,
        accounts.coin_currency_wallet,
        accounts.price_currency_wallet,
        accounts.coin_vault,
        accounts.pc_vault,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn send_take_verify_account_privileges<'me, 'info>(
    accounts: SendTakeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    send_take_verify_writable_privileges(accounts)?;
    Ok(())
}
pub const CLOSE_OPEN_ORDERS_IX_ACCOUNTS_LEN: usize = 4;
#[derive(Copy, Clone, Debug)]
pub struct CloseOpenOrdersAccounts<'me, 'info> {
    pub open_orders: &'me AccountInfo<'info>,
    pub owner: &'me AccountInfo<'info>,
    pub destination: &'me AccountInfo<'info>,
    pub market: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CloseOpenOrdersKeys {
    pub open_orders: Pubkey,
    pub owner: Pubkey,
    pub destination: Pubkey,
    pub market: Pubkey,
}
impl From<CloseOpenOrdersAccounts<'_, '_>> for CloseOpenOrdersKeys {
    fn from(accounts: CloseOpenOrdersAccounts) -> Self {
        Self {
            open_orders: *accounts.open_orders.key,
            owner: *accounts.owner.key,
            destination: *accounts.destination.key,
            market: *accounts.market.key,
        }
    }
}
impl From<CloseOpenOrdersKeys> for [AccountMeta; CLOSE_OPEN_ORDERS_IX_ACCOUNTS_LEN] {
    fn from(keys: CloseOpenOrdersKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.open_orders,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.owner,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.destination,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.market,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; CLOSE_OPEN_ORDERS_IX_ACCOUNTS_LEN]> for CloseOpenOrdersKeys {
    fn from(pubkeys: [Pubkey; CLOSE_OPEN_ORDERS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            open_orders: pubkeys[0],
            owner: pubkeys[1],
            destination: pubkeys[2],
            market: pubkeys[3],
        }
    }
}
impl<'info> From<CloseOpenOrdersAccounts<'_, 'info>>
for [AccountInfo<'info>; CLOSE_OPEN_ORDERS_IX_ACCOUNTS_LEN] {
    fn from(accounts: CloseOpenOrdersAccounts<'_, 'info>) -> Self {
        [
            accounts.open_orders.clone(),
            accounts.owner.clone(),
            accounts.destination.clone(),
            accounts.market.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CLOSE_OPEN_ORDERS_IX_ACCOUNTS_LEN]>
for CloseOpenOrdersAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; CLOSE_OPEN_ORDERS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            open_orders: &arr[0],
            owner: &arr[1],
            destination: &arr[2],
            market: &arr[3],
        }
    }
}
pub const CLOSE_OPEN_ORDERS_IX_DISCM: [u8; 4usize] = [14, 0, 0, 0];
#[derive(Clone, Debug, PartialEq)]
pub struct CloseOpenOrdersIxData;
impl CloseOpenOrdersIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 4usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CLOSE_OPEN_ORDERS_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CLOSE_OPEN_ORDERS_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn close_open_orders_ix_with_program_id(
    program_id: Pubkey,
    keys: CloseOpenOrdersKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CLOSE_OPEN_ORDERS_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: CloseOpenOrdersIxData.try_to_vec()?,
    })
}
pub fn close_open_orders_ix(keys: CloseOpenOrdersKeys) -> std::io::Result<Instruction> {
    close_open_orders_ix_with_program_id(ALDRIN_PROGRAM_ID, keys)
}
pub fn close_open_orders_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CloseOpenOrdersAccounts<'_, '_>,
) -> ProgramResult {
    let keys: CloseOpenOrdersKeys = accounts.into();
    let ix = close_open_orders_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn close_open_orders_invoke(
    accounts: CloseOpenOrdersAccounts<'_, '_>,
) -> ProgramResult {
    close_open_orders_invoke_with_program_id(ALDRIN_PROGRAM_ID, accounts)
}
pub fn close_open_orders_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CloseOpenOrdersAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CloseOpenOrdersKeys = accounts.into();
    let ix = close_open_orders_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn close_open_orders_invoke_signed(
    accounts: CloseOpenOrdersAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    close_open_orders_invoke_signed_with_program_id(ALDRIN_PROGRAM_ID, accounts, seeds)
}
pub fn close_open_orders_verify_account_keys(
    accounts: CloseOpenOrdersAccounts<'_, '_>,
    keys: CloseOpenOrdersKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.open_orders.key, keys.open_orders),
        (*accounts.owner.key, keys.owner),
        (*accounts.destination.key, keys.destination),
        (*accounts.market.key, keys.market),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn close_open_orders_verify_writable_privileges<'me, 'info>(
    accounts: CloseOpenOrdersAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.open_orders, accounts.destination] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn close_open_orders_verify_signer_privileges<'me, 'info>(
    accounts: CloseOpenOrdersAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.owner] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn close_open_orders_verify_account_privileges<'me, 'info>(
    accounts: CloseOpenOrdersAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    close_open_orders_verify_writable_privileges(accounts)?;
    close_open_orders_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const INIT_OPEN_ORDERS_IX_ACCOUNTS_LEN: usize = 5;
#[derive(Copy, Clone, Debug)]
pub struct InitOpenOrdersAccounts<'me, 'info> {
    pub open_orders: &'me AccountInfo<'info>,
    pub owner: &'me AccountInfo<'info>,
    pub market: &'me AccountInfo<'info>,
    pub rent: &'me AccountInfo<'info>,
    pub market_authority: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct InitOpenOrdersKeys {
    pub open_orders: Pubkey,
    pub owner: Pubkey,
    pub market: Pubkey,
    pub rent: Pubkey,
    pub market_authority: Pubkey,
}
impl From<InitOpenOrdersAccounts<'_, '_>> for InitOpenOrdersKeys {
    fn from(accounts: InitOpenOrdersAccounts) -> Self {
        Self {
            open_orders: *accounts.open_orders.key,
            owner: *accounts.owner.key,
            market: *accounts.market.key,
            rent: *accounts.rent.key,
            market_authority: *accounts.market_authority.key,
        }
    }
}
impl From<InitOpenOrdersKeys> for [AccountMeta; INIT_OPEN_ORDERS_IX_ACCOUNTS_LEN] {
    fn from(keys: InitOpenOrdersKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.open_orders,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.owner,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.market,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.rent,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.market_authority,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; INIT_OPEN_ORDERS_IX_ACCOUNTS_LEN]> for InitOpenOrdersKeys {
    fn from(pubkeys: [Pubkey; INIT_OPEN_ORDERS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            open_orders: pubkeys[0],
            owner: pubkeys[1],
            market: pubkeys[2],
            rent: pubkeys[3],
            market_authority: pubkeys[4],
        }
    }
}
impl<'info> From<InitOpenOrdersAccounts<'_, 'info>>
for [AccountInfo<'info>; INIT_OPEN_ORDERS_IX_ACCOUNTS_LEN] {
    fn from(accounts: InitOpenOrdersAccounts<'_, 'info>) -> Self {
        [
            accounts.open_orders.clone(),
            accounts.owner.clone(),
            accounts.market.clone(),
            accounts.rent.clone(),
            accounts.market_authority.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; INIT_OPEN_ORDERS_IX_ACCOUNTS_LEN]>
for InitOpenOrdersAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; INIT_OPEN_ORDERS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            open_orders: &arr[0],
            owner: &arr[1],
            market: &arr[2],
            rent: &arr[3],
            market_authority: &arr[4],
        }
    }
}
pub const INIT_OPEN_ORDERS_IX_DISCM: [u8; 4usize] = [15, 0, 0, 0];
#[derive(Clone, Debug, PartialEq)]
pub struct InitOpenOrdersIxData;
impl InitOpenOrdersIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 4usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INIT_OPEN_ORDERS_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INIT_OPEN_ORDERS_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn init_open_orders_ix_with_program_id(
    program_id: Pubkey,
    keys: InitOpenOrdersKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; INIT_OPEN_ORDERS_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: InitOpenOrdersIxData.try_to_vec()?,
    })
}
pub fn init_open_orders_ix(keys: InitOpenOrdersKeys) -> std::io::Result<Instruction> {
    init_open_orders_ix_with_program_id(ALDRIN_PROGRAM_ID, keys)
}
pub fn init_open_orders_invoke_with_program_id(
    program_id: Pubkey,
    accounts: InitOpenOrdersAccounts<'_, '_>,
) -> ProgramResult {
    let keys: InitOpenOrdersKeys = accounts.into();
    let ix = init_open_orders_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn init_open_orders_invoke(
    accounts: InitOpenOrdersAccounts<'_, '_>,
) -> ProgramResult {
    init_open_orders_invoke_with_program_id(ALDRIN_PROGRAM_ID, accounts)
}
pub fn init_open_orders_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: InitOpenOrdersAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: InitOpenOrdersKeys = accounts.into();
    let ix = init_open_orders_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn init_open_orders_invoke_signed(
    accounts: InitOpenOrdersAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    init_open_orders_invoke_signed_with_program_id(ALDRIN_PROGRAM_ID, accounts, seeds)
}
pub fn init_open_orders_verify_account_keys(
    accounts: InitOpenOrdersAccounts<'_, '_>,
    keys: InitOpenOrdersKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.open_orders.key, keys.open_orders),
        (*accounts.owner.key, keys.owner),
        (*accounts.market.key, keys.market),
        (*accounts.rent.key, keys.rent),
        (*accounts.market_authority.key, keys.market_authority),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn init_open_orders_verify_writable_privileges<'me, 'info>(
    accounts: InitOpenOrdersAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.open_orders] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn init_open_orders_verify_signer_privileges<'me, 'info>(
    accounts: InitOpenOrdersAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.owner] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn init_open_orders_verify_account_privileges<'me, 'info>(
    accounts: InitOpenOrdersAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    init_open_orders_verify_writable_privileges(accounts)?;
    init_open_orders_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const PRUNE_IX_ACCOUNTS_LEN: usize = 7;
#[derive(Copy, Clone, Debug)]
pub struct PruneAccounts<'me, 'info> {
    pub market: &'me AccountInfo<'info>,
    pub bids: &'me AccountInfo<'info>,
    pub asks: &'me AccountInfo<'info>,
    pub prune_authority: &'me AccountInfo<'info>,
    pub open_orders: &'me AccountInfo<'info>,
    pub owner: &'me AccountInfo<'info>,
    pub event_queue: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct PruneKeys {
    pub market: Pubkey,
    pub bids: Pubkey,
    pub asks: Pubkey,
    pub prune_authority: Pubkey,
    pub open_orders: Pubkey,
    pub owner: Pubkey,
    pub event_queue: Pubkey,
}
impl From<PruneAccounts<'_, '_>> for PruneKeys {
    fn from(accounts: PruneAccounts) -> Self {
        Self {
            market: *accounts.market.key,
            bids: *accounts.bids.key,
            asks: *accounts.asks.key,
            prune_authority: *accounts.prune_authority.key,
            open_orders: *accounts.open_orders.key,
            owner: *accounts.owner.key,
            event_queue: *accounts.event_queue.key,
        }
    }
}
impl From<PruneKeys> for [AccountMeta; PRUNE_IX_ACCOUNTS_LEN] {
    fn from(keys: PruneKeys) -> Self {
        [
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
                pubkey: keys.prune_authority,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.open_orders,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.owner,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.event_queue,
                is_signer: false,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; PRUNE_IX_ACCOUNTS_LEN]> for PruneKeys {
    fn from(pubkeys: [Pubkey; PRUNE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            market: pubkeys[0],
            bids: pubkeys[1],
            asks: pubkeys[2],
            prune_authority: pubkeys[3],
            open_orders: pubkeys[4],
            owner: pubkeys[5],
            event_queue: pubkeys[6],
        }
    }
}
impl<'info> From<PruneAccounts<'_, 'info>>
for [AccountInfo<'info>; PRUNE_IX_ACCOUNTS_LEN] {
    fn from(accounts: PruneAccounts<'_, 'info>) -> Self {
        [
            accounts.market.clone(),
            accounts.bids.clone(),
            accounts.asks.clone(),
            accounts.prune_authority.clone(),
            accounts.open_orders.clone(),
            accounts.owner.clone(),
            accounts.event_queue.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; PRUNE_IX_ACCOUNTS_LEN]>
for PruneAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; PRUNE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            market: &arr[0],
            bids: &arr[1],
            asks: &arr[2],
            prune_authority: &arr[3],
            open_orders: &arr[4],
            owner: &arr[5],
            event_queue: &arr[6],
        }
    }
}
pub const PRUNE_IX_DISCM: [u8; 4usize] = [16, 0, 0, 0];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PruneIxArgs {
    pub limit: u16,
}
#[derive(Clone, Debug, PartialEq)]
pub struct PruneIxData(pub PruneIxArgs);
impl From<PruneIxArgs> for PruneIxData {
    fn from(args: PruneIxArgs) -> Self {
        Self(args)
    }
}
impl PruneIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 4usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != PRUNE_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let limit: u16 = crate::borsh_de_or_default(&mut reader)?;
        Ok(Self(PruneIxArgs { limit }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&PRUNE_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.limit, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn prune_ix_with_program_id(
    program_id: Pubkey,
    keys: PruneKeys,
    args: PruneIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; PRUNE_IX_ACCOUNTS_LEN] = keys.into();
    let data: PruneIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn prune_ix(keys: PruneKeys, args: PruneIxArgs) -> std::io::Result<Instruction> {
    prune_ix_with_program_id(ALDRIN_PROGRAM_ID, keys, args)
}
pub fn prune_invoke_with_program_id(
    program_id: Pubkey,
    accounts: PruneAccounts<'_, '_>,
    args: PruneIxArgs,
) -> ProgramResult {
    let keys: PruneKeys = accounts.into();
    let ix = prune_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn prune_invoke(
    accounts: PruneAccounts<'_, '_>,
    args: PruneIxArgs,
) -> ProgramResult {
    prune_invoke_with_program_id(ALDRIN_PROGRAM_ID, accounts, args)
}
pub fn prune_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: PruneAccounts<'_, '_>,
    args: PruneIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: PruneKeys = accounts.into();
    let ix = prune_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn prune_invoke_signed(
    accounts: PruneAccounts<'_, '_>,
    args: PruneIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    prune_invoke_signed_with_program_id(ALDRIN_PROGRAM_ID, accounts, args, seeds)
}
pub fn prune_verify_account_keys(
    accounts: PruneAccounts<'_, '_>,
    keys: PruneKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.market.key, keys.market),
        (*accounts.bids.key, keys.bids),
        (*accounts.asks.key, keys.asks),
        (*accounts.prune_authority.key, keys.prune_authority),
        (*accounts.open_orders.key, keys.open_orders),
        (*accounts.owner.key, keys.owner),
        (*accounts.event_queue.key, keys.event_queue),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn prune_verify_writable_privileges<'me, 'info>(
    accounts: PruneAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.market,
        accounts.bids,
        accounts.asks,
        accounts.open_orders,
        accounts.event_queue,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn prune_verify_signer_privileges<'me, 'info>(
    accounts: PruneAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.prune_authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn prune_verify_account_privileges<'me, 'info>(
    accounts: PruneAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    prune_verify_writable_privileges(accounts)?;
    prune_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CONSUME_EVENTS_PERMISSIONED_IX_ACCOUNTS_LEN: usize = 4;
#[derive(Copy, Clone, Debug)]
pub struct ConsumeEventsPermissionedAccounts<'me, 'info> {
    pub open_orders: &'me AccountInfo<'info>,
    pub market: &'me AccountInfo<'info>,
    pub event_queue: &'me AccountInfo<'info>,
    pub crank_authority: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ConsumeEventsPermissionedKeys {
    pub open_orders: Pubkey,
    pub market: Pubkey,
    pub event_queue: Pubkey,
    pub crank_authority: Pubkey,
}
impl From<ConsumeEventsPermissionedAccounts<'_, '_>> for ConsumeEventsPermissionedKeys {
    fn from(accounts: ConsumeEventsPermissionedAccounts) -> Self {
        Self {
            open_orders: *accounts.open_orders.key,
            market: *accounts.market.key,
            event_queue: *accounts.event_queue.key,
            crank_authority: *accounts.crank_authority.key,
        }
    }
}
impl From<ConsumeEventsPermissionedKeys>
for [AccountMeta; CONSUME_EVENTS_PERMISSIONED_IX_ACCOUNTS_LEN] {
    fn from(keys: ConsumeEventsPermissionedKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.open_orders,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.market,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.event_queue,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.crank_authority,
                is_signer: true,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; CONSUME_EVENTS_PERMISSIONED_IX_ACCOUNTS_LEN]>
for ConsumeEventsPermissionedKeys {
    fn from(pubkeys: [Pubkey; CONSUME_EVENTS_PERMISSIONED_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            open_orders: pubkeys[0],
            market: pubkeys[1],
            event_queue: pubkeys[2],
            crank_authority: pubkeys[3],
        }
    }
}
impl<'info> From<ConsumeEventsPermissionedAccounts<'_, 'info>>
for [AccountInfo<'info>; CONSUME_EVENTS_PERMISSIONED_IX_ACCOUNTS_LEN] {
    fn from(accounts: ConsumeEventsPermissionedAccounts<'_, 'info>) -> Self {
        [
            accounts.open_orders.clone(),
            accounts.market.clone(),
            accounts.event_queue.clone(),
            accounts.crank_authority.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; CONSUME_EVENTS_PERMISSIONED_IX_ACCOUNTS_LEN]>
for ConsumeEventsPermissionedAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; CONSUME_EVENTS_PERMISSIONED_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            open_orders: &arr[0],
            market: &arr[1],
            event_queue: &arr[2],
            crank_authority: &arr[3],
        }
    }
}
pub const CONSUME_EVENTS_PERMISSIONED_IX_DISCM: [u8; 4usize] = [17, 0, 0, 0];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ConsumeEventsPermissionedIxArgs {
    pub limit: u16,
}
#[derive(Clone, Debug, PartialEq)]
pub struct ConsumeEventsPermissionedIxData(pub ConsumeEventsPermissionedIxArgs);
impl From<ConsumeEventsPermissionedIxArgs> for ConsumeEventsPermissionedIxData {
    fn from(args: ConsumeEventsPermissionedIxArgs) -> Self {
        Self(args)
    }
}
impl ConsumeEventsPermissionedIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 4usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CONSUME_EVENTS_PERMISSIONED_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let limit: u16 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(ConsumeEventsPermissionedIxArgs {
                limit,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CONSUME_EVENTS_PERMISSIONED_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.limit, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn consume_events_permissioned_ix_with_program_id(
    program_id: Pubkey,
    keys: ConsumeEventsPermissionedKeys,
    args: ConsumeEventsPermissionedIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CONSUME_EVENTS_PERMISSIONED_IX_ACCOUNTS_LEN] = keys.into();
    let data: ConsumeEventsPermissionedIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn consume_events_permissioned_ix(
    keys: ConsumeEventsPermissionedKeys,
    args: ConsumeEventsPermissionedIxArgs,
) -> std::io::Result<Instruction> {
    consume_events_permissioned_ix_with_program_id(ALDRIN_PROGRAM_ID, keys, args)
}
pub fn consume_events_permissioned_invoke_with_program_id(
    program_id: Pubkey,
    accounts: ConsumeEventsPermissionedAccounts<'_, '_>,
    args: ConsumeEventsPermissionedIxArgs,
) -> ProgramResult {
    let keys: ConsumeEventsPermissionedKeys = accounts.into();
    let ix = consume_events_permissioned_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn consume_events_permissioned_invoke(
    accounts: ConsumeEventsPermissionedAccounts<'_, '_>,
    args: ConsumeEventsPermissionedIxArgs,
) -> ProgramResult {
    consume_events_permissioned_invoke_with_program_id(ALDRIN_PROGRAM_ID, accounts, args)
}
pub fn consume_events_permissioned_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: ConsumeEventsPermissionedAccounts<'_, '_>,
    args: ConsumeEventsPermissionedIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: ConsumeEventsPermissionedKeys = accounts.into();
    let ix = consume_events_permissioned_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn consume_events_permissioned_invoke_signed(
    accounts: ConsumeEventsPermissionedAccounts<'_, '_>,
    args: ConsumeEventsPermissionedIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    consume_events_permissioned_invoke_signed_with_program_id(
        ALDRIN_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn consume_events_permissioned_verify_account_keys(
    accounts: ConsumeEventsPermissionedAccounts<'_, '_>,
    keys: ConsumeEventsPermissionedKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.open_orders.key, keys.open_orders),
        (*accounts.market.key, keys.market),
        (*accounts.event_queue.key, keys.event_queue),
        (*accounts.crank_authority.key, keys.crank_authority),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn consume_events_permissioned_verify_writable_privileges<'me, 'info>(
    accounts: ConsumeEventsPermissionedAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.open_orders,
        accounts.market,
        accounts.event_queue,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn consume_events_permissioned_verify_signer_privileges<'me, 'info>(
    accounts: ConsumeEventsPermissionedAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.crank_authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn consume_events_permissioned_verify_account_privileges<'me, 'info>(
    accounts: ConsumeEventsPermissionedAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    consume_events_permissioned_verify_writable_privileges(accounts)?;
    consume_events_permissioned_verify_signer_privileges(accounts)?;
    Ok(())
}
