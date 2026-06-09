use solana_program_error::ProgramError;
use thiserror::Error;
#[derive(Clone, Copy, Debug, Eq, Error, num_derive::FromPrimitive, PartialEq)]
pub enum DeltafiError {
    #[error("Swap account already in use")]
    AlreadyInUse = 6000,
    #[error("Address of the admin fee account is incorrect")]
    InvalidAdmin = 6001,
    #[error("Active admin transfer in progress")]
    ActiveTransfer = 6002,
    #[error("No active admin transfer in progress")]
    NoActiveTransfer = 6003,
    #[error("Admin transfer deadline exceeded")]
    AdminDeadlineExceeded = 6004,
    #[error("Account is not authorized to execute this instruction")]
    Unauthorized = 6005,
    #[error("Input account owner is not the program")]
    InvalidAccountOwner = 6006,
    #[error("Input account owner is not the program address")]
    InvalidOwner = 6007,
    #[error("Input account must be signer")]
    InvalidSigner = 6008,
    #[error("Output pool account owner cannot be the program address")]
    InvalidOutputOwner = 6009,
    #[error("Address of the provided swap token account is incorrect")]
    IncorrectSwapAccount = 6010,
    #[error("Invalid program address generated from nonce and key")]
    InvalidProgramAddress = 6011,
    #[error("Token account has a close authority")]
    InvalidCloseAuthority = 6012,
    #[error("Pool token mint has a freeze authority")]
    InvalidFreezeAuthority = 6013,
    #[error("Incorrect token program ID")]
    IncorrectTokenProgramId = 6014,
    #[error("Address of the provided token mint is incorrect")]
    IncorrectMint = 6015,
    #[error("Deserialized account is not an SPL Token mint")]
    UnexpectedMint = 6016,
    #[error("Swap input token accounts have the same mint")]
    RepeatedMint = 6017,
    #[error("Deserialized account is not an SPL Token account")]
    ExpectedAccount = 6018,
    #[error("Invalid instruction")]
    InvalidInstruction = 6019,
    #[error("Instruction unpack is failed")]
    InstructionUnpackError = 6020,
    #[error("Pool token supply is 0")]
    EmptyPool = 6021,
    #[error("Input token account empty")]
    EmptySupply = 6022,
    #[error("Pool token mint has a non-zero supply")]
    InvalidSupply = 6023,
    #[error("Token account has a delegate")]
    InvalidDelegate = 6024,
    #[error("InvalidInput")]
    InvalidInput = 6025,
    #[error("Swap pool is paused")]
    IsPaused = 6026,
    #[error("Lamport balance below rent-exempt threshold")]
    NotRentExempt = 6027,
    #[error("CalculationFailure")]
    CalculationFailure = 6028,
    #[error("Swap instruction exceeds desired slippage limit")]
    ExceededSlippage = 6029,
    #[error("Token mints must have same decimals")]
    MismatchedDecimals = 6030,
    #[error("Input pyth config is invalid")]
    InvalidPythConfig = 6031,
    #[error("Insufficient liquidity available")]
    InsufficientLiquidity = 6032,
    #[error("User has no liquidity position")]
    LiquidityPositionEmpty = 6033,
    #[error("Invalid position key")]
    InvalidPositionKey = 6034,
    #[error("Invalid claim timestamp")]
    InvalidClaimTime = 6035,
    #[error("Insufficient claim amount")]
    InsufficientClaimAmount = 6036,
    #[error("Insufficient funds")]
    InsufficientFunds = 6037,
    #[error("Withdraw not enough")]
    WithdrawNotEnough = 6038,
    #[error("Mint initialization failed")]
    TokenInitializeMintFailed = 6039,
    #[error("Invalid slope")]
    InvalidSlope = 6040,
    #[error("Invalid account")]
    InvalidAccount = 6041,
    #[error("Token transfer failed")]
    TokenTransferFailed = 6042,
    #[error("Token mint to failed")]
    TokenMintToFailed = 6043,
    #[error("Token burn failed")]
    TokenBurnFailed = 6044,
    #[error("Invalid pyth price")]
    InvalidPythPrice = 6045,
    #[error("Unstable pyth price")]
    UnstablePythPrice = 6046,
    #[error("Pyth confidence interval is too large")]
    InconfidentPythPrice = 6047,
    #[error("Index of out rage")]
    IndexOutOfRange = 6048,
    #[error("Input market config is invalid")]
    InvalidMarketConfig = 6049,
    #[error("Pyth program id is invalid")]
    InvalidPythProgramId = 6050,
    #[error("Potential Flash Loan Attack")]
    PotentialFlashLoanAttack = 6051,
    #[error("Incorrect swap type")]
    IncorrectSwapType = 6052,
    #[error("Incorrect stable price")]
    IncorrectStablePrice = 6053,
    #[error("Invalid token decimals")]
    InvalidTokenDecimals = 6054,
    #[error("Inconsistent pool state")]
    InconsistentPoolState = 6055,
    #[error("Invalid referrer address")]
    InvalidReferrer = 6056,
    #[error("Inconsistent initial pool token balance")]
    InconsistentInitialPoolTokenBalance = 6057,
    #[error("Swap out amount exceeds the limit")]
    ExceededSwapOutAmount = 6058,
    #[error("Already initialized")]
    AlreadyInitialized = 6059,
    #[error("Not initialized")]
    NotInitialized = 6060,
    #[error("Invalid swap config")]
    InvalidSwapConfig = 6061,
    #[error("Invalid farm config")]
    InvalidFarmConfig = 6062,
    #[error("Insufficient pool reserve")]
    InsufficientPoolReserve = 6063,
    #[error("stable swap price diff limit exceeded")]
    StableSwapPriceDiffLimitExceeded = 6064,
    #[error("Invalid timestamp")]
    InvalidTimestamp = 6065,
    #[error("InvalidSerumData")]
    InvalidSerumData = 6066,
    #[error("Invalid pyth price account")]
    InvalidPythPriceAccount = 6067,
    #[error("InvalidSerumMarketTokenRatio")]
    InvalidSerumMarketTokenRatio = 6068,
    #[error("DepeggedQuotePrice")]
    DepeggedQuotePrice = 6069,
    #[error("Invalid withdrawal amount")]
    InvalidWithdrawalAmount = 6070,
    #[error("Invalid staking amount")]
    InvalidStakingAmount = 6071,
    #[error("Rebate not enabled")]
    RebateNotEnabled = 6072,
}
impl From<DeltafiError> for ProgramError {
    fn from(e: DeltafiError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
