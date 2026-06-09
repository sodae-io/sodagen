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
pub enum OrderStatus {
    #[default]
    Active,
    Filled,
    Cancelled,
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
pub enum OrderType {
    #[default]
    Vanilla,
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
pub enum UpdateGlobalConfigMode {
    #[default]
    UpdateEmergencyMode,
    UpdateFlashTakeOrderBlocked,
    UpdateBlockNewOrders,
    UpdateBlockOrderTaking,
    UpdateHostFeeBps,
    UpdateAdminAuthorityCached,
    UpdateOrderTakingPermissionless,
    UpdateOrderCloseDelaySeconds,
    UpdateTxnFeeCost,
    UpdateAtaCreationCost,
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
pub enum UpdateGlobalConfigValue {
    Bool(bool),
    U16(u16),
    U64(u64),
    Pubkey(Pubkey),
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
pub enum UpdateOrderMode {
    #[default]
    UpdatePermissionless,
    UpdateCounterparty,
}
