use solana_program_error::ProgramError;
use thiserror::Error;
#[derive(Clone, Copy, Debug, Eq, Error, num_derive::FromPrimitive, PartialEq)]
pub enum DcaError {
    #[error("Invalid deposit amount")]
    InvalidAmount = 6000,
    #[error("Invalid deposit amount")]
    InvalidCycleAmount = 6001,
    #[error("Invalid pair")]
    InvalidPair = 6002,
    #[error("Too frequent DCA cycle")]
    TooFrequent = 6003,
    #[error("Minimum price constraint must be greater than 0")]
    InvalidMinPrice = 6004,
    #[error("Maximum price constraint must be greater than 0")]
    InvalidMaxPrice = 6005,
    #[error("In amount needs to be more than in amount per cycle")]
    InAmountInsufficient = 6006,
    #[error("Wrong user")]
    Unauthorized = 6007,
    #[error("inAta not passed in")]
    NoInAta = 6008,
    #[error("userInAta not passed in")]
    NoUserInAta = 6009,
    #[error("outAta not passed in")]
    NoOutAta = 6010,
    #[error("userOutAta not passed in")]
    NoUserOutAta = 6011,
    #[error("Trying to withdraw more than available")]
    InsufficientBalanceInProgram = 6012,
    #[error("Deposit should be more than 0")]
    InvalidDepositAmount = 6013,
    #[error("User has insufficient balance")]
    UserInsufficientBalance = 6014,
    #[error("Unauthorized Keeper")]
    UnauthorizedKeeper = 6015,
    #[error("Unrecognized Program")]
    UnrecognizedProgram = 6016,
    #[error("Calculation errors")]
    MathErrors = 6017,
    #[error("Not time to fill")]
    KeeperNotTimeToFill = 6018,
    #[error("Order amount wrong")]
    OrderFillAmountWrong = 6019,
    #[error("Out amount below expectations")]
    SwapOutAmountBelowMinimum = 6020,
    #[error("Wrong admin")]
    WrongAdmin = 6021,
    #[error("Overflow in arithmetic operation")]
    MathOverflow = 6022,
    #[error("Address Mismatch")]
    AddressMismatch = 6023,
    #[error("Program Mismatch")]
    ProgramMismatch = 6024,
    #[error("Incorrect Repayment Amount")]
    IncorrectRepaymentAmount = 6025,
    #[error("Cannot Borrow Before Repay")]
    CannotBorrowBeforeRepay = 6026,
    #[error("No Repayment Found")]
    NoRepaymentInstructionFound = 6027,
    #[error("Missing Swap Instruction")]
    MissingSwapInstructions = 6028,
    #[error("Expected Instruction to use Jupiter Swap Program")]
    UnexpectedSwapProgram = 6029,
    #[error("Invalid Swap Mint")]
    InvalidSwapMint = 6030,
    #[error("Unknown Instruction")]
    UnknownInstruction = 6031,
    #[error("Missing Repay Instruction")]
    MissingRepayInstructions = 6032,
    #[error("Keeper Shortchanged")]
    KeeperShortchanged = 6033,
    #[error("Jup Swap to Wrong Out Account")]
    WrongSwapOutAccount = 6034,
    #[error("Transfer amount should be exactly account balance")]
    WrongTransferAmount = 6035,
    #[error("Insufficient balance for rent")]
    InsufficientBalanceForRent = 6036,
    #[error("Unexpected SOL amount in intermediate account")]
    UnexpectedSolBalance = 6037,
    #[error("Too little WSOL to perform transfer")]
    InsufficientWsolForTransfer = 6038,
    #[error("Did not call initiate_flash_fill")]
    MissedInstruction = 6039,
    #[error("Did not call this program's initiate_flash_fill")]
    WrongProgram = 6040,
    #[error("Can't close account with balance")]
    BalanceNotZero = 6041,
    #[error("Should not have WSOL leftover in DCA out-token account")]
    UnexpectedWsolLeftover = 6042,
    #[error("Should pass in a WSOL intermediate account when transferring SOL")]
    IntermediateAccountNotSet = 6043,
    #[error("Did not call jup swap")]
    UnexpectedSwapInstruction = 6044,
    #[error("Expect more from swap")]
    SwapOutLessThanUserMinimum = 6045,
    #[error("Expect less from swap")]
    SwapOutMoreThanUserMaximum = 6046,
}
impl From<DcaError> for ProgramError {
    fn from(e: DcaError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
