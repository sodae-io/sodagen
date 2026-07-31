use borsh::{BorshDeserialize, BorshSerialize};
#[allow(unused_imports)]
use crate::*;
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
pub enum RoleType {
    #[default]
    MinterRoleUsDon,
    BurnerRoleUsDon,
    AdminRoleUsDon,
    AdminRoleUsDonManager,
    GuardianUsDon,
    DeployerRoleGmTokenFactory,
    PauserRoleGmTokenFactory,
    AdminRoleGmTokenFactory,
    MinterRoleGmToken,
    AdminRoleGmToken,
    PauserRoleGmTokenManager,
    PauserRoleGmToken,
    UnpauserRoleGmToken,
    AdminRoleGmTokenManager,
    IssuanceHoursRole,
    SetterRoleOndoSanityCheck,
    ConfigurerRoleOndoSanityCheck,
    AdminRoleOndoSanityCheck,
    AdminRoleWhitelist,
    UpdateMultiplierRole,
    UpdateMetadataRole,
}
