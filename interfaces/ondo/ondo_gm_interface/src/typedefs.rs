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
impl TryFrom<u8> for RoleType {
    type Error = std::io::Error;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0u8 => Ok(Self::MinterRoleUsDon),
            1u8 => Ok(Self::BurnerRoleUsDon),
            2u8 => Ok(Self::AdminRoleUsDon),
            3u8 => Ok(Self::AdminRoleUsDonManager),
            4u8 => Ok(Self::GuardianUsDon),
            5u8 => Ok(Self::DeployerRoleGmTokenFactory),
            6u8 => Ok(Self::PauserRoleGmTokenFactory),
            7u8 => Ok(Self::AdminRoleGmTokenFactory),
            8u8 => Ok(Self::MinterRoleGmToken),
            9u8 => Ok(Self::AdminRoleGmToken),
            10u8 => Ok(Self::PauserRoleGmTokenManager),
            11u8 => Ok(Self::PauserRoleGmToken),
            12u8 => Ok(Self::UnpauserRoleGmToken),
            13u8 => Ok(Self::AdminRoleGmTokenManager),
            14u8 => Ok(Self::IssuanceHoursRole),
            15u8 => Ok(Self::SetterRoleOndoSanityCheck),
            16u8 => Ok(Self::ConfigurerRoleOndoSanityCheck),
            17u8 => Ok(Self::AdminRoleOndoSanityCheck),
            18u8 => Ok(Self::AdminRoleWhitelist),
            19u8 => Ok(Self::UpdateMultiplierRole),
            20u8 => Ok(Self::UpdateMetadataRole),
            _ => Err(std::io::Error::from(std::io::ErrorKind::InvalidData)),
        }
    }
}
