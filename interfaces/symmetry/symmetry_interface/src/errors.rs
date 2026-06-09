use solana_program_error::ProgramError;
use thiserror::Error;
#[derive(Clone, Copy, Debug, Eq, Error, num_derive::FromPrimitive, PartialEq)]
pub enum SymmetryError {
    #[error("Rules are already set")]
    AlreadySet = 6000,
    #[error("Incorrect pda_usdc_account")]
    IncorrectPdaUsdcAccount = 6001,
    #[error("Incorrect Oracle account provided")]
    IncorrectOracleAccount = 6002,
    #[error("Fund_state is provided instead of sell_state")]
    IncorrectSellState = 6003,
    #[error("Incorrect pda_token_account")]
    IncorrectPdaTokenAccount = 6004,
    #[error("Current weights need to be updated")]
    UpdateCurrentWeights = 6005,
    #[error("Enough time hasn't passed yet")]
    TimeHasntPassed = 6006,
    #[error("weight is in rebalance threshold")]
    InThreshold = 6007,
    #[error("Number of Assets must be less or equal to 20")]
    MoreThan20Assets = 6008,
    #[error("Constraint error")]
    ConstraintError = 6009,
    #[error("Fund isn't actively managed")]
    NotActivelyManaged = 6010,
    #[error("Incorrect smf_fee_account")]
    IncorrectSmfFeeAccount = 6011,
    #[error("Expo must be in [0;1] range")]
    ExpoRangeError = 6012,
    #[error("Refilter or Reweight shouldn't be called for sell_state")]
    NoRefilterAndReweightForSellState = 6013,
    #[error("Incorrect rebalance_fee_account")]
    IncorrectRebalanceFeeAccount = 6014,
    #[error("Rebalance function already bought this token")]
    TokenIsAlreadyBought = 6015,
    #[error("Fund state must be updated")]
    FundStateMustBeUpdated = 6016,
    #[error("Rule weight is more than 1000")]
    RuleWeightLimitError = 6017,
    #[error("Refilter, Reweight or Rebalance interval limits are incorrect")]
    IntervalLimitsAreIncorrect = 6018,
    #[error("Swap Exceeded FundState Rebalance Slippage")]
    SlippageError = 6019,
    #[error("Passed token is not present in buy_state")]
    TokenIsntPresentInState = 6020,
    #[error("There are less free tokens Tokens in fund_state than buyer wants")]
    LessTokenInFund = 6021,
    #[error("USDC worth is less than token worth")]
    UsdcIsntEnough = 6022,
    #[error("Token weight after swap exceeds target weight")]
    ExceedsTargetWeight = 6023,
    #[error("Fund worth fill decrease after swap")]
    FundWorthDecreasing = 6024,
    #[error("Swap slippage exceeded")]
    SlippageExceeded = 6025,
    #[error("manager_usdc_account doesnt belong to manager")]
    IncorrectManagerAccount = 6026,
    #[error("Sell state rebalance should be executed by manager")]
    WrongSigner = 6027,
    #[error("Only ClaimTokens function is available on this sellState")]
    ClaimTokens = 6028,
    #[error("filter_by and weight_by should be in [0;3] range")]
    FilterOrWeightByError = 6029,
    #[error("filter_days and weight_days should be in [0;5] range")]
    FilterOrWeightDaysError = 6030,
    #[error("sort_by must be 0 or 1")]
    SortByError = 6031,
    #[error("Incorrect refferal USDC account")]
    IncorrectRefferalFeeAccount = 6032,
    #[error("Incorrect Buyer Token account")]
    IncorrectTokenAccount = 6033,
    #[error("To call refilter/reweight function token stats must be updated")]
    TokenStatsShouldBeUpdated = 6034,
    #[error("Fund is a SellState")]
    SellState = 6035,
    #[error("Program is freezed. Contact developer support.")]
    ProgramFreezed = 6036,
    #[error("TVL Limit reached as symmetry funds are in beta mode.")]
    TvlLimitReached = 6037,
    #[error("Max allowed contribution is limited to 5000 USDC")]
    BuyLimit = 6038,
    #[error("Pyth invalid slot")]
    PythValidSlot = 6039,
    #[error("Pyth status should be Trading")]
    PythStatus = 6040,
    #[error("Pyth low confidence")]
    PythConfidence = 6041,
    #[error("Pyth price can not be negative")]
    PythNegativePrice = 6042,
    #[error("Asset Pool shouldn't contain repeating tokens and should contain USDC")]
    AssetPool = 6043,
    #[error("Invalid instruction data was provided")]
    InvalidInstructionData = 6044,
    #[error("Fee account is not associated")]
    NotAssociatedTokenAccount = 6045,
    #[error("Could not swap enough amount")]
    CouldNotSwap = 6046,
    #[error("Incorrect token Id")]
    IncorrectTokenId = 6047,
    #[error("Fixed rule should contain only 1 asset")]
    FixedRule = 6048,
    #[error("Fund is still active")]
    FundIsActive = 6049,
    #[error("Buy state is being claimed")]
    BuyStateIsBeingClaimed = 6050,
    #[error("Asset pool contains token with offline oracle status")]
    AssetPoolContainsOfflineOracleToken = 6051,
    #[error("Liquidity Provision is disabled for one of the tokens")]
    LpOf = 6052,
    #[error("Svb Oracle not updated in last X seconds")]
    SvbOracleTimestamp = 6053,
    #[error("Auto Rebalance has been disabled by fund manager")]
    RebalanceDisabled = 6054,
    #[error("This is a private fund. Only manager can contribute")]
    PrivateFund = 6055,
    #[error("Account not owned by the program")]
    IncorrectOwner = 6056,
    #[error("Change not permitted")]
    NoPermission = 6057,
    #[error("Symbol should contain 3-10 letters")]
    WrongSymbolFormat = 6058,
    #[error("Name should contain 5-60 characters")]
    WrongNameFormat = 6059,
    #[error("Metadata URI should contain 0-300 characters")]
    WrongUriFormat = 6060,
}
impl From<SymmetryError> for ProgramError {
    fn from(e: SymmetryError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
