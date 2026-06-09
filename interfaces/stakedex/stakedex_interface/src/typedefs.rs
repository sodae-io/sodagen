use borsh::{BorshDeserialize, BorshSerialize};
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
pub struct SwapViaStakeArgs {
    pub amount: u64,
    pub bridge_stake_seed: u32,
}
impl SwapViaStakeArgs {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let bridge_stake_seed: u32 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { amount, bridge_stake_seed })
    }
}
