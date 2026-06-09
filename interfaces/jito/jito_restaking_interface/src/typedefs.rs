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
pub struct SlotToggle {
    pub slot_added: PodU64,
    pub slot_removed: PodU64,
    pub reserved: [u8; 32],
}
impl SlotToggle {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let slot_added = if reader.is_empty() {
            Default::default()
        } else {
            <PodU64>::deserialize(&mut reader)?
        };
        let slot_removed = if reader.is_empty() {
            Default::default()
        } else {
            <PodU64>::deserialize(&mut reader)?
        };
        let reserved: [u8; 32] = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            slot_added,
            slot_removed,
            reserved,
        })
    }
}
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
pub enum NcnAdminRole {
    #[default]
    OperatorAdmin,
    VaultAdmin,
    SlasherAdmin,
    DelegateAdmin,
    MetadataAdmin,
    WeightTableAdmin,
    NcnProgramAdmin,
}
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
pub enum OperatorAdminRole {
    #[default]
    NcnAdmin,
    VaultAdmin,
    VoterAdmin,
    DelegateAdmin,
    MetadataAdmin,
}
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
pub struct PodU64 {
    pub inner: u64,
}
impl PodU64 {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let inner: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { inner })
    }
}
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
pub struct PodU16 {
    pub inner: u16,
}
impl PodU16 {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let inner: u16 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { inner })
    }
}
