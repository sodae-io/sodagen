use solana_program_error::ProgramError;
use thiserror::Error;
#[derive(Clone, Copy, Debug, Eq, Error, num_derive::FromPrimitive, PartialEq)]
pub enum YvaultsError {
    #[error("Integer overflow")]
    IntegerOverflow = 6000,
    #[error("Operation Forbidden")]
    OperationForbidden = 6001,
    #[error("[DEPRECATED] Zero amount")]
    ZeroAmount = 6002,
    #[error("Unable to deserialize account")]
    UnableToDeserializeAccount = 6003,
    #[error("[DEPRECATED] Vault balance does not match for token A")]
    VaultBalanceDoesNotMatchTokenA = 6004,
    #[error("[DEPRECATED] Vault balance does not match for token B")]
    VaultBalanceDoesNotMatchTokenB = 6005,
    #[error("[DEPRECATED] Shares issued amount does not match")]
    SharesIssuedAmountDoesNotMatch = 6006,
    #[error("Key is not present in global config")]
    GlobalConfigKeyError = 6007,
    #[error("[DEPRECATED] System is in emergency mode")]
    SystemInEmergencyMode = 6008,
    #[error("Global deposit is currently blocked")]
    GlobalDepositBlocked = 6009,
    #[error("Global withdraw is currently blocked")]
    GlobalWithdrawBlocked = 6010,
    #[error("Global invest is currently blocked")]
    GlobalInvestBlocked = 6011,
    #[error("Out of range integral conversion attempted")]
    OutOfRangeIntegralConversion = 6012,
    #[error("[DEPRECATED] Mathematical operation with overflow")]
    MathOverflow = 6013,
    #[error("Unable to withdraw more liquidity than available in position")]
    TooMuchLiquidityToWithdraw = 6014,
    #[error("Deposit amounts must be greater than zero")]
    DepositAmountsZero = 6015,
    #[error("Number of shares to withdraw must be greater than zero")]
    SharesZero = 6016,
    #[error("Strategy not active")]
    StrategyNotActive = 6017,
    #[error("There are unharvested gains")]
    UnharvestedAmounts = 6018,
    #[error("Reward mapping incorrect")]
    InvalidRewardMapping = 6019,
    #[error("Reward index incorrect")]
    InvalidRewardIndex = 6020,
    #[error("Cannot use uninitialized reward vault")]
    OwnRewardUninitialized = 6021,
    #[error("Price is not valid")]
    PriceNotValid = 6022,
    #[error("Must provide almost equal amounts of tokens")]
    SwapRewardImbalanced = 6023,
    #[error("Swap reward is zero or less than requested")]
    SwapRewardTooSmall = 6024,
    #[error("Swap reward is less than what user requested as minimum")]
    SwapRewardLessThanRequested = 6025,
    #[error("Swap reward is less than minimum acceptable")]
    SwapRewardLessThanMinimum = 6026,
    #[error("Wrong discriminator")]
    WrongDiscriminator = 6027,
    #[error("Wrong mint")]
    WrongMint = 6028,
    #[error("Wrong vault")]
    WrongVault = 6029,
    #[error("Swap amounts must be greater than zero")]
    SwapAmountsZero = 6030,
    #[error("Price too old")]
    PriceTooOld = 6031,
    #[error("Cannot invest zero amount")]
    CannotInvestZeroAmount = 6032,
    #[error("Cannot have zero investable amount")]
    MaxInvestableZero = 6033,
    #[error("Collect fees is blocked")]
    CollectFeesBlocked = 6034,
    #[error("Collect rewards is blocked")]
    CollectRewardsBlocked = 6035,
    #[error("Swap rewards is blocked")]
    SwapRewardsBlocked = 6036,
    #[error("Reward collateral ID is incorrect for strategy")]
    WrongRewardCollateralId = 6037,
    #[error("Position account doesn't match internal records")]
    InvalidPositionAccount = 6038,
    #[error("Scope account could not be deserialized")]
    CouldNotDeserializeScope = 6039,
    #[error("[DEPRECATED] Collateral ID invalid for strategy")]
    WrongCollateralId = 6040,
    #[error("Collaterals exceed deposit cap")]
    CollateralTokensExceedDepositCap = 6041,
    #[error("Swap uneven vaults is blocked")]
    SwapUnevenVaultsBlocked = 6042,
    #[error("Cannot swap as vaults are already balanced")]
    VaultsAreAlreadyBalanced = 6043,
    #[error("Cannot swap uneven vaults when position is out of range")]
    CannotSwapUnevenOutOfRange = 6044,
    #[error("Cannot divide by zero")]
    DivideByZero = 6045,
    #[error("[DEPRECATED] Delta A too large")]
    DeltaATooLarge = 6046,
    #[error("[DEPRECATED] Delta B too large")]
    DeltaBTooLarge = 6047,
    #[error("[DEPRECATED] Cannot executive withdraw zero amount")]
    CannotExecutiveWithdrawZeroAmount = 6048,
    #[error("Cannot withdraw zero amount")]
    CannotWithdrawZeroAmount = 6049,
    #[error("[DEPRECATED] Cannot collect fees on zero liquidity position")]
    CannotCollectFeesOnZeroLiquidityPosition = 6050,
    #[error("Cannot deposit inactive position")]
    StrategyNotActiveWhenDepositing = 6051,
    #[error("Cannot open position with existing opened position")]
    CannotOpenPositionWithExistingPosition = 6052,
    #[error("Collaterals exceed deposit ixn cap")]
    CollateralTokensExceedDepositCapPerIxn = 6053,
    #[error("Cannot deposit when strategy out of range")]
    CannotDepositOutOfRange = 6054,
    #[error("Cannot invest when strategy out of range")]
    CannotInvestOutOfRange = 6055,
    #[error("Withdrawal cap is reached")]
    WithdrawalCapReached = 6056,
    #[error("Timestamp decrease")]
    TimestampDecrease = 6057,
    #[error("CPI not allowed")]
    CpiNotAllowed = 6058,
    #[error("Cannot use orca price as it is too different from scope price")]
    OrcaPriceTooDifferentFromScope = 6059,
    #[error("Lower tick larger than upper tick")]
    LowerTickLargerThanUpperTick = 6060,
    #[error("Lower tick is lower than the minimal supported low tick")]
    LowerTickTooLow = 6061,
    #[error("Upper tick is larger than the maximum supported tick")]
    UpperTickTooLarge = 6062,
    #[error("Lower tick is not a multiple of tick spacing")]
    LowerTickNotMultipleOfTickSpacing = 6063,
    #[error("Upper tick is not a multiple of tick spacing")]
    UpperTickNotMultipleOfTickSpacing = 6064,
    #[error("Cannot change admin authority")]
    CannotChangeAdminAuthority = 6065,
    #[error("Cannot resize with smaller new size")]
    CannotResizeAccount = 6066,
    #[error("Scope chain update failed")]
    ScopeChainUpdateFailed = 6067,
    #[error("Price too divergent from twap")]
    PriceTooDivergentFromTwap = 6068,
    #[error("[DEPRECATED] Can not override the existing reward")]
    ExistingRewardOverride = 6069,
    #[error("Kamino reward id exceeds the available slots")]
    WrongKaminoRewardId = 6070,
    #[error("Kamino reward is not initialized")]
    KaminoRewardNotExist = 6071,
    #[error("Kamino reward is already initialized")]
    KaminoRewardAlreadyExists = 6072,
    #[error("Kamino collateral is not valid")]
    KaminoCollateralNotValid = 6073,
    #[error(
        "[DEPRECATED] Expected kamino reward is bigger then the available amount within the vault"
    )]
    KaminoRewardExceedsAvailableAmount = 6074,
    #[error("Swap uneven vaults result in the opposite unbalance of the vaults")]
    SwapUnevenVaultsOvershoot = 6075,
    #[error("Bps parameter passed to instruction is not in range")]
    BpsNotInRange = 6076,
    #[error("Emergency Swap is blocked")]
    EmergencySwapBlocked = 6077,
    #[error("Strategy is expected to be frozen for this operation")]
    StrategyNotFrozen = 6078,
    #[error("Token left in vault post swap are lower than expected")]
    UnexpectedTokenAmountsPostSwap = 6079,
    #[error("Account doesn't belong to the DEX")]
    AccountNotBelongToDex = 6080,
    #[error("Wrong DEX program ID")]
    WrongDexProgramId = 6081,
    #[error("Cannot use uninitialized orca reward vault")]
    OrcaRewardUninitialized = 6082,
    #[error("Invalid admin authority")]
    InvalidAdminAuthority = 6083,
    #[error("Token price is bigger than heuristic")]
    PriceIsBiggerThanHeuristic = 6084,
    #[error("Token price is lower than heuristic")]
    PriceIsLowerThanHeuristic = 6085,
    #[error("Account different than expected")]
    AccountDifferentThanExpected = 6086,
    #[error("Swap amount below the minimum value")]
    SwapAmountsTooSmall = 6087,
    #[error("Invalid dex program id")]
    InvalidDexProgramId = 6088,
    #[error("Strategy deposit is currently blocked")]
    StrategyDepositBlocked = 6089,
    #[error("Strategy invest is currently blocked")]
    StrategyInvestBlocked = 6090,
    #[error("Strategy withdraw is currently blocked")]
    StrategyWithdrawBlocked = 6091,
    #[error("Vault swap can't be performed in the required direction")]
    WrongSwapVaultDirection = 6092,
    #[error("Provided amount for vault swap is over the limit")]
    SwapVaultsTooBig = 6093,
    #[error("Token out for cash based vault swap is below minimum expected")]
    SwapVaultsCashOutputBelowMinimum = 6094,
    #[error("Flash ixs initiated without the closing ix in the transaction")]
    FlashIxsNotEnded = 6095,
    #[error(
        "Some unexpected instructions are present in the tx. Either before or after the flash ixs, or some ix target the same program between"
    )]
    FlashTxWithUnexpectedIxs = 6096,
    #[error("Some accounts differ between the two flash ixs")]
    FlashIxsAccountMismatch = 6097,
    #[error("A scope ix is present in a flash tx")]
    FlashIxsIncludeScope = 6098,
    #[error("Flash vault swap is blocked on this strategy")]
    FlashVaultSwapBlocked = 6099,
    #[error(
        "Unexpected amount of tokens in ata prior flash vault swap (wrong amount_to_leave_to_user)"
    )]
    FlashVaultSwapWrongAmountToLeave = 6100,
    #[error("Deposit amount less than minimal allowed")]
    DepositLessThanMinimum = 6101,
    #[error("Cannot deposit without invest")]
    DepositWithoutInvestDisallowed = 6102,
    #[error("Invalid Scope Chain")]
    InvalidScopeChain = 6103,
    #[error("Invalid Scope TWAP Chain")]
    InvalidScopeTwapChain = 6104,
    #[error("Existent position has liquidity, new position creation is forbidden")]
    PositionHasRemainingLiquidity = 6105,
    #[error("Deposit is not allowed as pool is rebalancing")]
    PoolRebalancing = 6106,
    #[error("Permissionless rebalancing is disabled")]
    PermissionlessRebalancingDisabled = 6107,
    #[error("Only the owner of the strategy can manually rebalance it")]
    ManualRebalanceInvalidOwner = 6108,
    #[error("Invalid rebalance type for the strategy")]
    InvalidRebalanceType = 6109,
    #[error("No rebalance necessary based on current rebalance type/parameters")]
    NoRebalanceNecessary = 6110,
    #[error("The given tick arrays do not match the rebalance result")]
    TickArraysDoNotMatchRebalance = 6111,
    #[error("Expected strategy position to be initialized")]
    StrategyPositionNotValid = 6112,
    #[error("Rebalance state could not be deserialized")]
    CouldNotDeserializeRebalanceState = 6113,
    #[error("Rebalance state could not be serialized")]
    CouldNotSerializeRebalanceState = 6114,
    #[error("Rebalance params could not be deserialized")]
    CouldNotDeserializeRebalanceParams = 6115,
    #[error(
        "Deposit is not allowed as token amounts are not enough to match our holdings ratio"
    )]
    NotEnoughTokensForRatio = 6116,
    #[error("The provided amounts are too small")]
    AmountsRepresentZeroShares = 6117,
    #[error("Rouding errors exceed the maximal loss tolerance")]
    MaxLossExceeded = 6118,
    #[error("Reward does not match strategy token")]
    RewardNotStrategyToken = 6119,
    #[error("Decimal to u64 conversion failed")]
    DecimalToU64ConversionFailed = 6120,
    #[error("Decimal operation failed")]
    DecimalOperationFailed = 6121,
    #[error("Deposit is not allowed as the strategy is not fully invested in the pool ")]
    VaultBalancesCausesWrongSharesIssuance = 6122,
    #[error("Token cannot be used in strategy creation")]
    TokenDisabled = 6123,
    #[error("Invalid reference price type")]
    InvalidReferencePriceType = 6124,
    #[error("Token amount to be swapped is not enough")]
    TokenToSwapNotEnough = 6125,
    #[error("Token amount in ata is different than the expected amount")]
    TokenAccountBalanceMismatch = 6126,
    #[error("Unexpected programID for prerequisite ix")]
    UnexpectedProgramIdForPrerequisiteIx = 6127,
    #[error(
        "Got an error from the dex specific function while computing the fees/rewards update"
    )]
    ComputeFeesAndRewardsUpdateError = 6128,
    #[error("There must be no shares issued when closing a strategy")]
    SharesNotZero = 6129,
    #[error("Invalid Scope staking rate Chain")]
    InvalidScopeStakingRateChain = 6130,
    #[error("Staking rate (provided by Scope) is not valid")]
    StakingRateNotValid = 6131,
    #[error("Decimal to u128 conversion failed")]
    DecimalToU128ConversionFailed = 6132,
    #[error("Decimal sqrt on negative number")]
    DecimalNegativeSqrtRoot = 6133,
    #[error("Drifting strategy is moving in the opposite direction")]
    DriftingOppositeDirection = 6134,
    #[error("Wrong reward collateral_id")]
    WrongRewardCollateralId2 = 6135,
    #[error("Collateral info already exists for given index")]
    CollateralInfoAlreadyExists = 6136,
    #[error("Invest is too early after the position was opened")]
    InvestTooEarly = 6137,
    #[error("Swap uneven is too early after the position was opened")]
    SwapUnevenTooEarly = 6138,
    #[error("Flash swap is too early after the position was opened")]
    FlashSwapTooEarly = 6139,
    #[error(
        "Rebalance caps reached, no rebalances are allowed until the end of the current interval"
    )]
    RebalancesCapReached = 6140,
    #[error(
        "Cannot swap uneven because authority is set and the given signer does not correspond"
    )]
    SwapUnevenInvalidAuthority = 6141,
    #[error("Invalid tick requested")]
    InvalidTick = 6142,
    #[error("Meteora math overflowed")]
    MeteoraMathOverflow = 6143,
    #[error("Expected strategy tick arrays to be initialized")]
    StrategyTickArrayNotValid = 6144,
    #[error("Wrong event authority")]
    WrongEventAuthority = 6145,
    #[error("Strategy field update is not allowed")]
    StrategyFieldUpdateNotAllowed = 6146,
    #[error("DEX is not supported for this operation")]
    UnsupportedDex = 6147,
    #[error("Invalid BPS value provided")]
    InvalidBpsValue = 6148,
    #[error("Reward vault override not allowed")]
    RewardVaultOverrideNotAllowed = 6149,
    #[error(
        "Got invalid reward from the dex specific function while computing the fees/rewards update"
    )]
    ComputeFeesAndRewardsInvalidReward = 6150,
    #[error("No tokens to withdraw from treasury fee vault")]
    EmptyTreasury = 6151,
    #[error("New pool reward mint does not match the old pool reward mint")]
    ChangingPoolRewardMintMismatch = 6152,
    #[error("The provided reward vault does not match the strategy state")]
    ProvidedRewardVaultMismatch = 6153,
    #[error("The provided reward vault does not match the strategy state")]
    RepeatedMint = 6154,
    #[error("The token extension is not supported by the program")]
    UnsupportedTokenExtension = 6155,
    #[error("Cannot initialize strategy with this dex while having a mint with token22")]
    UnsupportedDexForToken22 = 6156,
    #[error("Scope price index is not present in global config")]
    GlobalConfigInvalidScopePriceIndex = 6157,
    #[error("Scope prices ids array in global config is full")]
    GlobalConfigScopePricesIdsFull = 6158,
    #[error("Scope prices ids array already contains given scope price id")]
    GlobalConfigAlreadyContainsScopePriceId = 6159,
    #[error(
        "Expected scope prices account being passed in when adding or updating a scope prices feed"
    )]
    GlobalConfigExpectedScopePricesAccount = 6160,
    #[error(
        "A reward scope price account should have been passed in, but it is missing"
    )]
    RewardScopePriceAccountNotPresent = 6161,
    #[error("Incorrect scope prices account id passed in")]
    IncorrectScopePricesAccountId = 6162,
    #[error("Could not calculate get_price_usd_unchecked()")]
    CouldNotCalculatePriceTwap = 6163,
    #[error("Reference price is too far from the current pool price")]
    ReferencePriceTooFarFromPoolPrice = 6164,
    #[error("Invest cooldown has not elapsed")]
    InvestCooldownNotElapsed = 6165,
    #[error("Uninvested value is below the minimum invest trigger")]
    InvestAmountBelowMinimum = 6166,
    #[error("Deposit and invest is disabled on mainnet")]
    DepositAndInvestDisabled = 6167,
}
impl From<YvaultsError> for ProgramError {
    fn from(e: YvaultsError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
