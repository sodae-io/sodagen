use borsh::{BorshDeserialize, BorshSerialize};
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
pub struct CreateMetadataAccountArgsV3 {
    pub data: DataV2,
    pub is_mutable: bool,
    pub collection_details: Option<u8>,
}
impl CreateMetadataAccountArgsV3 {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let data = if reader.is_empty() {
            Default::default()
        } else {
            <DataV2>::deserialize(&mut reader)?
        };
        let is_mutable: bool = crate::borsh_de_or_default(&mut reader)?;
        let collection_details: Option<u8> = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            data,
            is_mutable,
            collection_details,
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
pub struct UpdateMetadataAccountArgsV2 {
    pub data: Option<DataV2>,
    pub update_authority: Option<Pubkey>,
    pub primary_sale_happened: Option<bool>,
    pub is_mutable: Option<bool>,
}
impl UpdateMetadataAccountArgsV2 {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let data: Option<DataV2> = crate::borsh_de_or_default(&mut reader)?;
        let update_authority: Option<Pubkey> = crate::borsh_de_or_default(&mut reader)?;
        let primary_sale_happened: Option<bool> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let is_mutable: Option<bool> = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            data,
            update_authority,
            primary_sale_happened,
            is_mutable,
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
pub struct DataV2 {
    pub name: String,
    pub symbol: String,
    pub uri: String,
    pub seller_fee_basis_points: u16,
    pub creators: Option<u8>,
    pub collection: Option<u8>,
    pub uses: Option<u8>,
}
impl DataV2 {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let name: String = crate::borsh_de_or_default(&mut reader)?;
        let symbol: String = crate::borsh_de_or_default(&mut reader)?;
        let uri: String = crate::borsh_de_or_default(&mut reader)?;
        let seller_fee_basis_points: u16 = crate::borsh_de_or_default(&mut reader)?;
        let creators: Option<u8> = crate::borsh_de_or_default(&mut reader)?;
        let collection: Option<u8> = crate::borsh_de_or_default(&mut reader)?;
        let uses: Option<u8> = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            name,
            symbol,
            uri,
            seller_fee_basis_points,
            creators,
            collection,
            uses,
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
pub enum ConfigAdminRole {
    #[default]
    FeeAdmin,
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
pub enum VaultAdminRole {
    #[default]
    DelegationAdmin,
    OperatorAdmin,
    NcnAdmin,
    SlasherAdmin,
    CapacityAdmin,
    FeeWallet,
    MintBurnAdmin,
    DelegateAssetAdmin,
    FeeAdmin,
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
pub enum WithdrawalAllocationMethod {
    #[default]
    Greedy,
}
#[derive(
    Clone,
    Debug,
    BorshDeserialize,
    BorshSerialize,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
pub struct DelegationState {
    pub staked_amount: PodU64,
    pub enqueued_for_cooldown_amount: PodU64,
    pub cooling_down_amount: PodU64,
    #[serde(with = "crate::big_array_serde")]
    pub reserved: [u8; 256],
}
impl DelegationState {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let staked_amount = if reader.is_empty() {
            Default::default()
        } else {
            <PodU64>::deserialize(&mut reader)?
        };
        let enqueued_for_cooldown_amount = if reader.is_empty() {
            Default::default()
        } else {
            <PodU64>::deserialize(&mut reader)?
        };
        let cooling_down_amount = if reader.is_empty() {
            Default::default()
        } else {
            <PodU64>::deserialize(&mut reader)?
        };
        let reserved = <[u8; 256] as borsh::BorshDeserialize>::deserialize_reader(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self {
            staked_amount,
            enqueued_for_cooldown_amount,
            cooling_down_amount,
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
pub struct PodBool {
    pub inner: u8,
}
impl PodBool {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let inner: u8 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { inner })
    }
}
