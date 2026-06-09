use solana_program_error::ProgramError;
use thiserror::Error;
#[derive(Clone, Copy, Debug, Eq, Error, num_derive::FromPrimitive, PartialEq)]
pub enum OpenbookV2Error {
    #[error("")]
    SomeError = 6000,
    #[error("Name lenght above limit")]
    InvalidInputNameLength = 6001,
    #[error("Market cannot be created as expired")]
    InvalidInputMarketExpired = 6002,
    #[error(
        "Taker fees should be positive and if maker fees are negative, greater or equal to their abs value"
    )]
    InvalidInputMarketFees = 6003,
    #[error("Lots cannot be negative")]
    InvalidInputLots = 6004,
    #[error("Lots size above market limits")]
    InvalidInputLotsSize = 6005,
    #[error("Input amounts above limits")]
    InvalidInputOrdersAmounts = 6006,
    #[error("Price lots should be greater than zero")]
    InvalidInputCancelSize = 6007,
    #[error("Expected cancel size should be greater than zero")]
    InvalidInputPriceLots = 6008,
    #[error("Peg limit should be greater than zero")]
    InvalidInputPegLimit = 6009,
    #[error(
        "The order type is invalid. A taker order must be Market or ImmediateOrCancel"
    )]
    InvalidInputOrderType = 6010,
    #[error("Order id cannot be zero")]
    InvalidInputOrderId = 6011,
    #[error("Slot above heap limit")]
    InvalidInputHeapSlots = 6012,
    #[error("Cannot combine two oracles of different providers")]
    InvalidOracleTypes = 6013,
    #[error("Cannot configure secondary oracle without primary")]
    InvalidSecondOracle = 6014,
    #[error(
        "This market does not have a `close_market_admin` and thus cannot be closed."
    )]
    NoCloseMarketAdmin = 6015,
    #[error("The signer of this transaction is not this market's `close_market_admin`.")]
    InvalidCloseMarketAdmin = 6016,
    #[error(
        "The `open_orders_admin` required by this market to sign all instructions that creates orders is missing or is not valid"
    )]
    InvalidOpenOrdersAdmin = 6017,
    #[error(
        "The `consume_events_admin` required by this market to sign all instructions that consume events is missing or is not valid"
    )]
    InvalidConsumeEventsAdmin = 6018,
    #[error("Provided `market_vault` is invalid")]
    InvalidMarketVault = 6019,
    #[error("Cannot be closed due to the existence of open orders accounts")]
    IndexerActiveOo = 6020,
    #[error("Cannot place a peg order due to invalid oracle state")]
    OraclePegInvalidOracleState = 6021,
    #[error("oracle type cannot be determined")]
    UnknownOracleType = 6022,
    #[error("an oracle does not reach the confidence threshold")]
    OracleConfidence = 6023,
    #[error("an oracle is stale")]
    OracleStale = 6024,
    #[error("Order id not found on the orderbook")]
    OrderIdNotFound = 6025,
    #[error("Event heap contains elements and market can't be closed")]
    EventHeapContainsElements = 6026,
    #[error("ImmediateOrCancel is not a PostOrderType")]
    InvalidOrderPostIoc = 6027,
    #[error("Market is not a PostOrderType")]
    InvalidOrderPostMarket = 6028,
    #[error("would self trade")]
    WouldSelfTrade = 6029,
    #[error("The Market has already expired.")]
    MarketHasExpired = 6030,
    #[error("Price lots should be greater than zero")]
    InvalidPriceLots = 6031,
    #[error("Oracle price above market limits")]
    InvalidOraclePrice = 6032,
    #[error("The Market has not expired yet.")]
    MarketHasNotExpired = 6033,
    #[error("No correct owner or delegate.")]
    NoOwnerOrDelegate = 6034,
    #[error("No correct owner")]
    NoOwner = 6035,
    #[error("No free order index in open orders account")]
    OpenOrdersFull = 6036,
    #[error("Book contains elements")]
    BookContainsElements = 6037,
    #[error("Could not find order in user account")]
    OpenOrdersOrderNotFound = 6038,
    #[error("Amount to post above book limits")]
    InvalidPostAmount = 6039,
    #[error("Oracle peg orders are not enabled for this market")]
    DisabledOraclePeg = 6040,
    #[error("Cannot close a non-empty market")]
    NonEmptyMarket = 6041,
    #[error("Cannot close a non-empty open orders account")]
    NonEmptyOpenOrdersPosition = 6042,
    #[error("Fill-Or-Kill order would generate a partial execution")]
    WouldExecutePartially = 6043,
}
impl From<OpenbookV2Error> for ProgramError {
    fn from(e: OpenbookV2Error) -> Self {
        ProgramError::Custom(e as u32)
    }
}
