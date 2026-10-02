use solana_program_error::ProgramError;
use thiserror::Error;
#[derive(Clone, Copy, Debug, Eq, Error, num_derive::FromPrimitive, PartialEq)]
pub enum HyloExchangeError {
    #[error("Cannot redeem levercoin due to Depeg. NAV would be 0 or lower.")]
    LevercoinRedeemDisabled = 6000,
    #[error("Levercoin to stablecoin swap disabled due to rebalance mode.")]
    LeverToStableDisabled = 6001,
    #[error("Stablecoin to levercoin swap disabled due to rebalance mode.")]
    StableToLeverDisabled = 6002,
    #[error("Error during CPI to Sanctum LST/SOL calculator.")]
    SanctumCpi = 6003,
    #[error("LST registry cannot be initialized twice.")]
    LstRegistryAlreadyInitialized = 6004,
    #[error("LST registry calculators were already added to lookup table.")]
    LstRegistryCalculatorsAlreadyInitialized = 6005,
    #[error("LST specific accounts found empty when attempting to load registry.")]
    LstRegistryEmpty = 6006,
    #[error("Sanctum calculator contexts in LST registry preamble are malformed.")]
    LstRegistryPreamble = 6007,
    #[error("Failed to deserialize registry lookup table.")]
    LstRegistryLookupTableDeser = 6008,
    #[error("Contents of LST registry did not match remaining_accounts.")]
    LstRegistryLookupTableInvalid = 6009,
    #[error("Mint/vault/pool accounts in registry block do not match header.")]
    LstBlockInvalid = 6010,
    #[error("Attempted to register an LST with invalid Sanctum context accounts.")]
    LstContextInvalid = 6011,
    #[error("Addition overflow while computing total SOL in LST registry.")]
    LstAdditionOverflow = 6012,
    #[error("Cached LST price not from current epoch. Run pricing crank to update.")]
    LstPriceOutdated = 6013,
    #[error("Failed to compute delta between current and previous LST prices.")]
    LstPriceDelta = 6014,
    #[error("Found current epoch less than previous in LST header.")]
    LstPriceEpochsInvalid = 6015,
    #[error("Overflow while computing LST SOL appreciation.")]
    LstSolAppreciation = 6016,
    #[error("Stablecoin mint disabled. Collateral ratio is below minting threshold.")]
    StablecoinMintDisabled = 6017,
    #[error("Levercoin mint disabled Due to depeg.")]
    LevercoinMintDisabled = 6018,
    #[error("Yield harvest configuration percentages failed validation.")]
    YieldHarvestConfigValidation = 6019,
    #[error("Yield harvest has already occurred during this epoch.")]
    YieldHarvestAlreadyRun = 6020,
    #[error("Arithmetic error while computing yield harvest allocation.")]
    YieldHarvestAllocation = 6021,
    #[error("Yield harvest already occurred during this epoch.")]
    YieldHarvestEpoch = 6022,
    #[error("Cannot swap from an asset to itself.")]
    IdentitySwap = 6023,
    #[error("Incorrect decimals assumption for given collateral mint.")]
    ExoAmountDecimals = 6024,
    #[error("Unable to upconvert amount to desired exponent.")]
    ExoAmountUpConversion = 6025,
    #[error("Oracle for exo collateral does not match given feed_id.")]
    ExoOracleInvalid = 6026,
    #[error("Borrow rate harvest has already occurred during this epoch.")]
    BorrowRateHarvestAlreadyRun = 6027,
    #[error("Underflow while computing elapsed epochs for borrow rate harvest.")]
    BorrowRateHarvestEpochUnderflow = 6028,
    #[error("Virtual stablecoin already initialized for LSTs.")]
    LstVirtualStablecoinAlreadyInitialized = 6029,
    #[error("Cannot update configuration with identical value.")]
    AdminNoop = 6030,
    #[error("Sell-side rebalancing is inactive at current collateral ratio.")]
    RebalanceSellInactive = 6031,
    #[error("Buy-side rebalancing is inactive at current collateral ratio.")]
    RebalanceBuyInactive = 6032,
    #[error("Mint is not in the exogenous collateral allowlist.")]
    ExoMintNotInAllowlist = 6033,
    #[error("Address change proposal has expired.")]
    AddressChangeExpired = 6034,
    #[error("Address change proposal's TTL not in configured range.")]
    AddressChangeTtlInvalid = 6035,
    #[error("Address change proposal has not been approved.")]
    AddressChangeNotApproved = 6036,
    #[error("Address change proposal has already been approved.")]
    AddressChangeAlreadyApproved = 6037,
    #[error("Address change approver must be upgrade authority.")]
    AddressChangeUpgradeAuthority = 6038,
    #[error("Failed while converting precision for a token amount.")]
    TokenAmountPrecisionError = 6039,
    #[error("Underflow while computing virtual stablecoin delta.")]
    SettleVirtualStablecoinUnderflow = 6040,
    #[error("Underflow while converting TVL to stablecoin.")]
    SettleVirtualStablecoinConversion = 6041,
    #[error("Virtual stablecoin settlement noop: nothing to drawdown or repay.")]
    SettleVirtualStablecoinNoop = 6042,
    #[error("Rebalance PnL settlement disabled in Depeg mode.")]
    SettleRebalancePnlDisabled = 6043,
    #[error("LST stake pool is not supported.")]
    LstStakePoolNotSupported = 6044,
    #[error("Exo pair genesis mint constraints not met.")]
    ExoGenesisConstraints = 6045,
    #[error("Error or constraint not met for collateral ratio for genesis.")]
    ExoGenesisCollateralRatio = 6046,
    #[error("Cannot unpause exo pair due to zero virtual stablecoin.")]
    ExoPairZeroVirtualStablecoin = 6047,
    #[error("Virtual stablecoin supply exceeds USDC vault balance.")]
    VirtualStablecoinExceedsVault = 6048,
}
impl From<HyloExchangeError> for ProgramError {
    fn from(e: HyloExchangeError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
