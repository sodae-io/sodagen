use borsh::{BorshDeserialize, BorshSerialize};
#[allow(unused_imports)]
use crate::*;
use solana_pubkey::Pubkey;
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
pub struct WhitelistedExtension {
    pub program_id: Pubkey,
    pub mint: Pubkey,
    pub token_program: Pubkey,
}
impl WhitelistedExtension {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let program_id: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let token_program: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            program_id,
            mint,
            token_program,
        })
    }
}
