solana_pubkey::declare_id!("whirLbMiicVdio4qvUfM5KAg6Ct8VwpYzGff3uctyCc");
pub const WHIRLPOOL_PROGRAM_ID: solana_pubkey::Pubkey = ID;
pub type ProgramResult = Result<(), solana_program_error::ProgramError>;
pub(crate) mod big_array_serde;
/// Tolerant field read: returns `T::default()` when the slice is already
/// exhausted, so payloads encoded before a trailing field was added still
/// decode. Only defaults at a true field boundary (slice empty), so a
/// field present-but-truncated still errors via the inner read.
#[doc(hidden)]
#[allow(dead_code)]
#[inline]
pub(crate) fn borsh_de_or_default<T: borsh::BorshDeserialize + Default>(
    reader: &mut &[u8],
) -> std::io::Result<T> {
    if reader.is_empty() {
        Ok(T::default())
    } else {
        borsh::BorshDeserialize::deserialize_reader(reader)
    }
}
pub mod accounts;
pub use accounts::*;
pub mod typedefs;
pub use typedefs::*;
pub mod instructions;
pub use instructions::*;
pub mod errors;
pub use errors::*;
pub mod events;
pub use events::*;
