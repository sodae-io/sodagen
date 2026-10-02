use solana_pubkey::Pubkey;
#[allow(unused_imports)]
use crate::*;
pub const ACCEPT_ADDRESS_UPDATE_EVENT_EVENT_DISCM: [u8; 8] = [
    63, 130, 1, 235, 255, 47, 229, 210,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AcceptAddressUpdateEvent {
    pub address_field: AddressField,
    pub old_address: Pubkey,
    pub new_address: Pubkey,
}
impl AcceptAddressUpdateEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let address_field: AddressField = crate::borsh_de_or_default(&mut reader)?;
        let old_address: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let new_address: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            address_field,
            old_address,
            new_address,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.address_field, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.old_address, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.new_address, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct AcceptAddressUpdateEventEvent(pub AcceptAddressUpdateEvent);
impl AcceptAddressUpdateEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != ACCEPT_ADDRESS_UPDATE_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = AcceptAddressUpdateEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&ACCEPT_ADDRESS_UPDATE_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const APPROVE_ADDRESS_UPDATE_EVENT_EVENT_DISCM: [u8; 8] = [
    98, 91, 95, 138, 125, 114, 11, 245,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ApproveAddressUpdateEvent {
    pub address_field: AddressField,
    pub new_address: Pubkey,
}
impl ApproveAddressUpdateEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let address_field: AddressField = crate::borsh_de_or_default(&mut reader)?;
        let new_address: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { address_field, new_address })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.address_field, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.new_address, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct ApproveAddressUpdateEventEvent(pub ApproveAddressUpdateEvent);
impl ApproveAddressUpdateEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != APPROVE_ADDRESS_UPDATE_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = ApproveAddressUpdateEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&APPROVE_ADDRESS_UPDATE_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const CANCEL_ADDRESS_UPDATE_EVENT_EVENT_DISCM: [u8; 8] = [
    240, 141, 179, 211, 245, 181, 23, 212,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CancelAddressUpdateEvent {
    pub address_field: AddressField,
    pub new_address: Pubkey,
}
impl CancelAddressUpdateEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let address_field: AddressField = crate::borsh_de_or_default(&mut reader)?;
        let new_address: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { address_field, new_address })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.address_field, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.new_address, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct CancelAddressUpdateEventEvent(pub CancelAddressUpdateEvent);
impl CancelAddressUpdateEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CANCEL_ADDRESS_UPDATE_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = CancelAddressUpdateEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CANCEL_ADDRESS_UPDATE_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const CONVERT_LEVER_TO_STABLE_EXO_EVENT_EVENT_DISCM: [u8; 8] = [
    54, 111, 46, 48, 123, 42, 111, 119,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ConvertLeverToStableExoEvent {
    pub collateral_mint: Pubkey,
    pub levercoin_burned: UFixValue64,
    pub levercoin_nav: UFixValue64,
    pub stablecoin_minted_user: UFixValue64,
    pub stablecoin_minted_fees: UFixValue64,
    pub stablecoin_nav: UFixValue64,
    pub collateral_usd_price: OraclePriceEvent,
    pub virtual_stablecoin_supply: UFixValue64,
}
impl ConvertLeverToStableExoEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let collateral_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let levercoin_burned = if reader.is_empty() {
            Default::default()
        } else {
            <UFixValue64>::deserialize(&mut reader)?
        };
        let levercoin_nav = if reader.is_empty() {
            Default::default()
        } else {
            <UFixValue64>::deserialize(&mut reader)?
        };
        let stablecoin_minted_user = if reader.is_empty() {
            Default::default()
        } else {
            <UFixValue64>::deserialize(&mut reader)?
        };
        let stablecoin_minted_fees = if reader.is_empty() {
            Default::default()
        } else {
            <UFixValue64>::deserialize(&mut reader)?
        };
        let stablecoin_nav = if reader.is_empty() {
            Default::default()
        } else {
            <UFixValue64>::deserialize(&mut reader)?
        };
        let collateral_usd_price = if reader.is_empty() {
            Default::default()
        } else {
            <OraclePriceEvent>::deserialize(&mut reader)?
        };
        let virtual_stablecoin_supply = if reader.is_empty() {
            Default::default()
        } else {
            <UFixValue64>::deserialize(&mut reader)?
        };
        *__buf = reader;
        Ok(Self {
            collateral_mint,
            levercoin_burned,
            levercoin_nav,
            stablecoin_minted_user,
            stablecoin_minted_fees,
            stablecoin_nav,
            collateral_usd_price,
            virtual_stablecoin_supply,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.collateral_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.levercoin_burned, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.levercoin_nav, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.stablecoin_minted_user, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.stablecoin_minted_fees, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.stablecoin_nav, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.collateral_usd_price, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.virtual_stablecoin_supply, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct ConvertLeverToStableExoEventEvent(pub ConvertLeverToStableExoEvent);
impl ConvertLeverToStableExoEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CONVERT_LEVER_TO_STABLE_EXO_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = ConvertLeverToStableExoEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CONVERT_LEVER_TO_STABLE_EXO_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const CONVERT_LEVER_TO_STABLE_LST_EVENT_EVENT_DISCM: [u8; 8] = [
    134, 150, 107, 160, 239, 63, 162, 243,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ConvertLeverToStableLstEvent {
    pub levercoin_burned: UFixValue64,
    pub levercoin_nav: UFixValue64,
    pub stablecoin_minted_user: UFixValue64,
    pub stablecoin_minted_fees: UFixValue64,
    pub stablecoin_nav: UFixValue64,
}
impl ConvertLeverToStableLstEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let levercoin_burned = if reader.is_empty() {
            Default::default()
        } else {
            <UFixValue64>::deserialize(&mut reader)?
        };
        let levercoin_nav = if reader.is_empty() {
            Default::default()
        } else {
            <UFixValue64>::deserialize(&mut reader)?
        };
        let stablecoin_minted_user = if reader.is_empty() {
            Default::default()
        } else {
            <UFixValue64>::deserialize(&mut reader)?
        };
        let stablecoin_minted_fees = if reader.is_empty() {
            Default::default()
        } else {
            <UFixValue64>::deserialize(&mut reader)?
        };
        let stablecoin_nav = if reader.is_empty() {
            Default::default()
        } else {
            <UFixValue64>::deserialize(&mut reader)?
        };
        *__buf = reader;
        Ok(Self {
            levercoin_burned,
            levercoin_nav,
            stablecoin_minted_user,
            stablecoin_minted_fees,
            stablecoin_nav,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.levercoin_burned, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.levercoin_nav, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.stablecoin_minted_user, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.stablecoin_minted_fees, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.stablecoin_nav, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct ConvertLeverToStableLstEventEvent(pub ConvertLeverToStableLstEvent);
impl ConvertLeverToStableLstEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CONVERT_LEVER_TO_STABLE_LST_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = ConvertLeverToStableLstEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CONVERT_LEVER_TO_STABLE_LST_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const CONVERT_STABLE_TO_LEVER_EXO_EVENT_EVENT_DISCM: [u8; 8] = [
    254, 238, 62, 13, 38, 223, 93, 62,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ConvertStableToLeverExoEvent {
    pub collateral_mint: Pubkey,
    pub stablecoin_burned: UFixValue64,
    pub stablecoin_fees: UFixValue64,
    pub stablecoin_nav: UFixValue64,
    pub levercoin_minted: UFixValue64,
    pub levercoin_nav: UFixValue64,
    pub collateral_usd_price: OraclePriceEvent,
    pub virtual_stablecoin_supply: UFixValue64,
}
impl ConvertStableToLeverExoEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let collateral_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let stablecoin_burned = if reader.is_empty() {
            Default::default()
        } else {
            <UFixValue64>::deserialize(&mut reader)?
        };
        let stablecoin_fees = if reader.is_empty() {
            Default::default()
        } else {
            <UFixValue64>::deserialize(&mut reader)?
        };
        let stablecoin_nav = if reader.is_empty() {
            Default::default()
        } else {
            <UFixValue64>::deserialize(&mut reader)?
        };
        let levercoin_minted = if reader.is_empty() {
            Default::default()
        } else {
            <UFixValue64>::deserialize(&mut reader)?
        };
        let levercoin_nav = if reader.is_empty() {
            Default::default()
        } else {
            <UFixValue64>::deserialize(&mut reader)?
        };
        let collateral_usd_price = if reader.is_empty() {
            Default::default()
        } else {
            <OraclePriceEvent>::deserialize(&mut reader)?
        };
        let virtual_stablecoin_supply = if reader.is_empty() {
            Default::default()
        } else {
            <UFixValue64>::deserialize(&mut reader)?
        };
        *__buf = reader;
        Ok(Self {
            collateral_mint,
            stablecoin_burned,
            stablecoin_fees,
            stablecoin_nav,
            levercoin_minted,
            levercoin_nav,
            collateral_usd_price,
            virtual_stablecoin_supply,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.collateral_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.stablecoin_burned, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.stablecoin_fees, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.stablecoin_nav, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.levercoin_minted, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.levercoin_nav, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.collateral_usd_price, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.virtual_stablecoin_supply, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct ConvertStableToLeverExoEventEvent(pub ConvertStableToLeverExoEvent);
impl ConvertStableToLeverExoEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CONVERT_STABLE_TO_LEVER_EXO_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = ConvertStableToLeverExoEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CONVERT_STABLE_TO_LEVER_EXO_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const CONVERT_STABLE_TO_LEVER_LST_EVENT_EVENT_DISCM: [u8; 8] = [
    152, 64, 205, 195, 44, 206, 26, 35,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ConvertStableToLeverLstEvent {
    pub stablecoin_burned: UFixValue64,
    pub stablecoin_fees: UFixValue64,
    pub stablecoin_nav: UFixValue64,
    pub levercoin_minted: UFixValue64,
    pub levercoin_nav: UFixValue64,
}
impl ConvertStableToLeverLstEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let stablecoin_burned = if reader.is_empty() {
            Default::default()
        } else {
            <UFixValue64>::deserialize(&mut reader)?
        };
        let stablecoin_fees = if reader.is_empty() {
            Default::default()
        } else {
            <UFixValue64>::deserialize(&mut reader)?
        };
        let stablecoin_nav = if reader.is_empty() {
            Default::default()
        } else {
            <UFixValue64>::deserialize(&mut reader)?
        };
        let levercoin_minted = if reader.is_empty() {
            Default::default()
        } else {
            <UFixValue64>::deserialize(&mut reader)?
        };
        let levercoin_nav = if reader.is_empty() {
            Default::default()
        } else {
            <UFixValue64>::deserialize(&mut reader)?
        };
        *__buf = reader;
        Ok(Self {
            stablecoin_burned,
            stablecoin_fees,
            stablecoin_nav,
            levercoin_minted,
            levercoin_nav,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.stablecoin_burned, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.stablecoin_fees, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.stablecoin_nav, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.levercoin_minted, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.levercoin_nav, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct ConvertStableToLeverLstEventEvent(pub ConvertStableToLeverLstEvent);
impl ConvertStableToLeverLstEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CONVERT_STABLE_TO_LEVER_LST_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = ConvertStableToLeverLstEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CONVERT_STABLE_TO_LEVER_LST_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const GENESIS_MINT_EXO_EVENT_EVENT_DISCM: [u8; 8] = [
    13, 215, 102, 188, 148, 156, 165, 145,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GenesisMintExoEvent {
    pub exo_pair: Pubkey,
    pub collateral_mint: Pubkey,
    pub collateral_deposited: UFixValue64,
    pub levercoin_minted: UFixValue64,
    pub stablecoin_minted: UFixValue64,
    pub collateral_ratio: UFixValue64,
    pub collateral_usd_price: OraclePriceEvent,
}
impl GenesisMintExoEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let exo_pair: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let collateral_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let collateral_deposited = if reader.is_empty() {
            Default::default()
        } else {
            <UFixValue64>::deserialize(&mut reader)?
        };
        let levercoin_minted = if reader.is_empty() {
            Default::default()
        } else {
            <UFixValue64>::deserialize(&mut reader)?
        };
        let stablecoin_minted = if reader.is_empty() {
            Default::default()
        } else {
            <UFixValue64>::deserialize(&mut reader)?
        };
        let collateral_ratio = if reader.is_empty() {
            Default::default()
        } else {
            <UFixValue64>::deserialize(&mut reader)?
        };
        let collateral_usd_price = if reader.is_empty() {
            Default::default()
        } else {
            <OraclePriceEvent>::deserialize(&mut reader)?
        };
        *__buf = reader;
        Ok(Self {
            exo_pair,
            collateral_mint,
            collateral_deposited,
            levercoin_minted,
            stablecoin_minted,
            collateral_ratio,
            collateral_usd_price,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.exo_pair, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.collateral_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.collateral_deposited, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.levercoin_minted, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.stablecoin_minted, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.collateral_ratio, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.collateral_usd_price, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct GenesisMintExoEventEvent(pub GenesisMintExoEvent);
impl GenesisMintExoEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != GENESIS_MINT_EXO_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = GenesisMintExoEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&GENESIS_MINT_EXO_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const HARVEST_BORROW_RATE_EVENT_EVENT_DISCM: [u8; 8] = [
    93, 52, 145, 219, 106, 174, 45, 135,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct HarvestBorrowRateEvent {
    pub collateral_mint: Pubkey,
    pub levercoin_market_cap: UFixValue64,
    pub total_stablecoin_harvested: UFixValue64,
    pub fees_extracted: UFixValue64,
    pub stablecoin_to_pool: UFixValue64,
    pub pool_drawdown_repaid: UFixValue64,
    pub collateral_usd_price: OraclePriceEvent,
}
impl HarvestBorrowRateEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let collateral_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let levercoin_market_cap = if reader.is_empty() {
            Default::default()
        } else {
            <UFixValue64>::deserialize(&mut reader)?
        };
        let total_stablecoin_harvested = if reader.is_empty() {
            Default::default()
        } else {
            <UFixValue64>::deserialize(&mut reader)?
        };
        let fees_extracted = if reader.is_empty() {
            Default::default()
        } else {
            <UFixValue64>::deserialize(&mut reader)?
        };
        let stablecoin_to_pool = if reader.is_empty() {
            Default::default()
        } else {
            <UFixValue64>::deserialize(&mut reader)?
        };
        let pool_drawdown_repaid = if reader.is_empty() {
            Default::default()
        } else {
            <UFixValue64>::deserialize(&mut reader)?
        };
        let collateral_usd_price = if reader.is_empty() {
            Default::default()
        } else {
            <OraclePriceEvent>::deserialize(&mut reader)?
        };
        *__buf = reader;
        Ok(Self {
            collateral_mint,
            levercoin_market_cap,
            total_stablecoin_harvested,
            fees_extracted,
            stablecoin_to_pool,
            pool_drawdown_repaid,
            collateral_usd_price,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.collateral_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.levercoin_market_cap, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.total_stablecoin_harvested, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fees_extracted, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.stablecoin_to_pool, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.pool_drawdown_repaid, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.collateral_usd_price, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct HarvestBorrowRateEventEvent(pub HarvestBorrowRateEvent);
impl HarvestBorrowRateEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != HARVEST_BORROW_RATE_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = HarvestBorrowRateEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&HARVEST_BORROW_RATE_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const HARVEST_YIELD_EVENT_EVENT_DISCM: [u8; 8] = [
    164, 43, 118, 208, 1, 71, 104, 111,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct HarvestYieldEvent {
    pub total_sol_harvested: UFixValue64,
    pub fees_extracted: UFixValue64,
    pub token_to_pool: UFixValue64,
    pub pool_drawdown_repaid: UFixValue64,
    pub sol_usd_price: OraclePriceEvent,
}
impl HarvestYieldEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let total_sol_harvested = if reader.is_empty() {
            Default::default()
        } else {
            <UFixValue64>::deserialize(&mut reader)?
        };
        let fees_extracted = if reader.is_empty() {
            Default::default()
        } else {
            <UFixValue64>::deserialize(&mut reader)?
        };
        let token_to_pool = if reader.is_empty() {
            Default::default()
        } else {
            <UFixValue64>::deserialize(&mut reader)?
        };
        let pool_drawdown_repaid = if reader.is_empty() {
            Default::default()
        } else {
            <UFixValue64>::deserialize(&mut reader)?
        };
        let sol_usd_price = if reader.is_empty() {
            Default::default()
        } else {
            <OraclePriceEvent>::deserialize(&mut reader)?
        };
        *__buf = reader;
        Ok(Self {
            total_sol_harvested,
            fees_extracted,
            token_to_pool,
            pool_drawdown_repaid,
            sol_usd_price,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.total_sol_harvested, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fees_extracted, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_to_pool, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.pool_drawdown_repaid, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.sol_usd_price, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct HarvestYieldEventEvent(pub HarvestYieldEvent);
impl HarvestYieldEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != HARVEST_YIELD_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = HarvestYieldEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&HARVEST_YIELD_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const INITIALIZE_LST_VIRTUAL_STABLECOIN_EVENT_EVENT_DISCM: [u8; 8] = [
    246, 85, 19, 34, 235, 75, 42, 42,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct InitializeLstVirtualStablecoinEvent {
    pub stablecoin_amount: UFixValue64,
}
impl InitializeLstVirtualStablecoinEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let stablecoin_amount = if reader.is_empty() {
            Default::default()
        } else {
            <UFixValue64>::deserialize(&mut reader)?
        };
        *__buf = reader;
        Ok(Self { stablecoin_amount })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.stablecoin_amount, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct InitializeLstVirtualStablecoinEventEvent(
    pub InitializeLstVirtualStablecoinEvent,
);
impl InitializeLstVirtualStablecoinEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INITIALIZE_LST_VIRTUAL_STABLECOIN_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = InitializeLstVirtualStablecoinEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INITIALIZE_LST_VIRTUAL_STABLECOIN_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const INITIALIZE_USDC_EVENT_EVENT_DISCM: [u8; 8] = [
    118, 215, 0, 196, 205, 141, 64, 145,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct InitializeUsdcEvent {
    pub vault_auth_bump: u8,
    pub fee_auth_bump: u8,
    pub mint_fee: UFixValue64,
    pub redeem_fee: UFixValue64,
    pub oracle_interval_secs: u64,
    pub oracle_conf_tolerance: UFixValue64,
    pub par_tolerance: UFixValue64,
}
impl InitializeUsdcEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let vault_auth_bump: u8 = crate::borsh_de_or_default(&mut reader)?;
        let fee_auth_bump: u8 = crate::borsh_de_or_default(&mut reader)?;
        let mint_fee = if reader.is_empty() {
            Default::default()
        } else {
            <UFixValue64>::deserialize(&mut reader)?
        };
        let redeem_fee = if reader.is_empty() {
            Default::default()
        } else {
            <UFixValue64>::deserialize(&mut reader)?
        };
        let oracle_interval_secs: u64 = crate::borsh_de_or_default(&mut reader)?;
        let oracle_conf_tolerance = if reader.is_empty() {
            Default::default()
        } else {
            <UFixValue64>::deserialize(&mut reader)?
        };
        let par_tolerance = if reader.is_empty() {
            Default::default()
        } else {
            <UFixValue64>::deserialize(&mut reader)?
        };
        *__buf = reader;
        Ok(Self {
            vault_auth_bump,
            fee_auth_bump,
            mint_fee,
            redeem_fee,
            oracle_interval_secs,
            oracle_conf_tolerance,
            par_tolerance,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.vault_auth_bump, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fee_auth_bump, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.mint_fee, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.redeem_fee, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.oracle_interval_secs, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.oracle_conf_tolerance, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.par_tolerance, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct InitializeUsdcEventEvent(pub InitializeUsdcEvent);
impl InitializeUsdcEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INITIALIZE_USDC_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = InitializeUsdcEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INITIALIZE_USDC_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const MINT_LEVERCOIN_EXO_EVENT_EVENT_DISCM: [u8; 8] = [
    232, 106, 176, 56, 229, 14, 200, 9,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MintLevercoinExoEvent {
    pub collateral_mint: Pubkey,
    pub minted: UFixValue64,
    pub nav: UFixValue64,
    pub oracle: Pubkey,
    pub collateral_usd_price: OraclePriceEvent,
    pub collateral_deposited: UFixValue64,
    pub fees_deposited: UFixValue64,
}
impl MintLevercoinExoEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let collateral_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let minted = if reader.is_empty() {
            Default::default()
        } else {
            <UFixValue64>::deserialize(&mut reader)?
        };
        let nav = if reader.is_empty() {
            Default::default()
        } else {
            <UFixValue64>::deserialize(&mut reader)?
        };
        let oracle: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let collateral_usd_price = if reader.is_empty() {
            Default::default()
        } else {
            <OraclePriceEvent>::deserialize(&mut reader)?
        };
        let collateral_deposited = if reader.is_empty() {
            Default::default()
        } else {
            <UFixValue64>::deserialize(&mut reader)?
        };
        let fees_deposited = if reader.is_empty() {
            Default::default()
        } else {
            <UFixValue64>::deserialize(&mut reader)?
        };
        *__buf = reader;
        Ok(Self {
            collateral_mint,
            minted,
            nav,
            oracle,
            collateral_usd_price,
            collateral_deposited,
            fees_deposited,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.collateral_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.minted, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.nav, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.oracle, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.collateral_usd_price, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.collateral_deposited, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fees_deposited, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct MintLevercoinExoEventEvent(pub MintLevercoinExoEvent);
impl MintLevercoinExoEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != MINT_LEVERCOIN_EXO_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = MintLevercoinExoEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&MINT_LEVERCOIN_EXO_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const MINT_LEVERCOIN_LST_EVENT_EVENT_DISCM: [u8; 8] = [
    118, 107, 196, 75, 46, 250, 152, 173,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MintLevercoinLstEvent {
    pub minted: UFixValue64,
    pub nav: UFixValue64,
    pub sol_usd_price: OraclePriceEvent,
    pub lst_mint: Pubkey,
    pub lst_sol_price: UFixValue64,
    pub collateral_deposited: UFixValue64,
    pub fees_deposited: UFixValue64,
}
impl MintLevercoinLstEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let minted = if reader.is_empty() {
            Default::default()
        } else {
            <UFixValue64>::deserialize(&mut reader)?
        };
        let nav = if reader.is_empty() {
            Default::default()
        } else {
            <UFixValue64>::deserialize(&mut reader)?
        };
        let sol_usd_price = if reader.is_empty() {
            Default::default()
        } else {
            <OraclePriceEvent>::deserialize(&mut reader)?
        };
        let lst_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let lst_sol_price = if reader.is_empty() {
            Default::default()
        } else {
            <UFixValue64>::deserialize(&mut reader)?
        };
        let collateral_deposited = if reader.is_empty() {
            Default::default()
        } else {
            <UFixValue64>::deserialize(&mut reader)?
        };
        let fees_deposited = if reader.is_empty() {
            Default::default()
        } else {
            <UFixValue64>::deserialize(&mut reader)?
        };
        *__buf = reader;
        Ok(Self {
            minted,
            nav,
            sol_usd_price,
            lst_mint,
            lst_sol_price,
            collateral_deposited,
            fees_deposited,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.minted, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.nav, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.sol_usd_price, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.lst_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.lst_sol_price, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.collateral_deposited, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fees_deposited, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct MintLevercoinLstEventEvent(pub MintLevercoinLstEvent);
impl MintLevercoinLstEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != MINT_LEVERCOIN_LST_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = MintLevercoinLstEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&MINT_LEVERCOIN_LST_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const MINT_STABLECOIN_EXO_EVENT_EVENT_DISCM: [u8; 8] = [
    171, 225, 49, 219, 112, 252, 51, 16,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MintStablecoinExoEvent {
    pub collateral_mint: Pubkey,
    pub minted: UFixValue64,
    pub nav: UFixValue64,
    pub collateral_usd_price: OraclePriceEvent,
    pub collateral_deposited: UFixValue64,
    pub fees_deposited: UFixValue64,
    pub virtual_stablecoin_supply: UFixValue64,
    pub stablecoin_supply: UFixValue64,
}
impl MintStablecoinExoEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let collateral_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let minted = if reader.is_empty() {
            Default::default()
        } else {
            <UFixValue64>::deserialize(&mut reader)?
        };
        let nav = if reader.is_empty() {
            Default::default()
        } else {
            <UFixValue64>::deserialize(&mut reader)?
        };
        let collateral_usd_price = if reader.is_empty() {
            Default::default()
        } else {
            <OraclePriceEvent>::deserialize(&mut reader)?
        };
        let collateral_deposited = if reader.is_empty() {
            Default::default()
        } else {
            <UFixValue64>::deserialize(&mut reader)?
        };
        let fees_deposited = if reader.is_empty() {
            Default::default()
        } else {
            <UFixValue64>::deserialize(&mut reader)?
        };
        let virtual_stablecoin_supply = if reader.is_empty() {
            Default::default()
        } else {
            <UFixValue64>::deserialize(&mut reader)?
        };
        let stablecoin_supply = if reader.is_empty() {
            Default::default()
        } else {
            <UFixValue64>::deserialize(&mut reader)?
        };
        *__buf = reader;
        Ok(Self {
            collateral_mint,
            minted,
            nav,
            collateral_usd_price,
            collateral_deposited,
            fees_deposited,
            virtual_stablecoin_supply,
            stablecoin_supply,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.collateral_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.minted, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.nav, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.collateral_usd_price, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.collateral_deposited, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fees_deposited, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.virtual_stablecoin_supply, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.stablecoin_supply, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct MintStablecoinExoEventEvent(pub MintStablecoinExoEvent);
impl MintStablecoinExoEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != MINT_STABLECOIN_EXO_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = MintStablecoinExoEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&MINT_STABLECOIN_EXO_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const MINT_STABLECOIN_LST_EVENT_EVENT_DISCM: [u8; 8] = [
    232, 212, 134, 22, 1, 96, 15, 68,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MintStablecoinLstEvent {
    pub minted: UFixValue64,
    pub nav: UFixValue64,
    pub sol_usd_price: OraclePriceEvent,
    pub lst_mint: Pubkey,
    pub lst_sol_price: UFixValue64,
    pub collateral_deposited: UFixValue64,
    pub fees_deposited: UFixValue64,
}
impl MintStablecoinLstEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let minted = if reader.is_empty() {
            Default::default()
        } else {
            <UFixValue64>::deserialize(&mut reader)?
        };
        let nav = if reader.is_empty() {
            Default::default()
        } else {
            <UFixValue64>::deserialize(&mut reader)?
        };
        let sol_usd_price = if reader.is_empty() {
            Default::default()
        } else {
            <OraclePriceEvent>::deserialize(&mut reader)?
        };
        let lst_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let lst_sol_price = if reader.is_empty() {
            Default::default()
        } else {
            <UFixValue64>::deserialize(&mut reader)?
        };
        let collateral_deposited = if reader.is_empty() {
            Default::default()
        } else {
            <UFixValue64>::deserialize(&mut reader)?
        };
        let fees_deposited = if reader.is_empty() {
            Default::default()
        } else {
            <UFixValue64>::deserialize(&mut reader)?
        };
        *__buf = reader;
        Ok(Self {
            minted,
            nav,
            sol_usd_price,
            lst_mint,
            lst_sol_price,
            collateral_deposited,
            fees_deposited,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.minted, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.nav, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.sol_usd_price, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.lst_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.lst_sol_price, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.collateral_deposited, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fees_deposited, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct MintStablecoinLstEventEvent(pub MintStablecoinLstEvent);
impl MintStablecoinLstEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != MINT_STABLECOIN_LST_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = MintStablecoinLstEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&MINT_STABLECOIN_LST_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const MINT_STABLECOIN_USDC_EVENT_EVENT_DISCM: [u8; 8] = [
    21, 192, 43, 134, 103, 237, 140, 189,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MintStablecoinUsdcEvent {
    pub usdc_deposited: UFixValue64,
    pub usdc_fees: UFixValue64,
    pub stablecoin_minted: UFixValue64,
    pub virtual_stablecoin_supply: UFixValue64,
}
impl MintStablecoinUsdcEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let usdc_deposited = if reader.is_empty() {
            Default::default()
        } else {
            <UFixValue64>::deserialize(&mut reader)?
        };
        let usdc_fees = if reader.is_empty() {
            Default::default()
        } else {
            <UFixValue64>::deserialize(&mut reader)?
        };
        let stablecoin_minted = if reader.is_empty() {
            Default::default()
        } else {
            <UFixValue64>::deserialize(&mut reader)?
        };
        let virtual_stablecoin_supply = if reader.is_empty() {
            Default::default()
        } else {
            <UFixValue64>::deserialize(&mut reader)?
        };
        *__buf = reader;
        Ok(Self {
            usdc_deposited,
            usdc_fees,
            stablecoin_minted,
            virtual_stablecoin_supply,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.usdc_deposited, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.usdc_fees, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.stablecoin_minted, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.virtual_stablecoin_supply, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct MintStablecoinUsdcEventEvent(pub MintStablecoinUsdcEvent);
impl MintStablecoinUsdcEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != MINT_STABLECOIN_USDC_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = MintStablecoinUsdcEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&MINT_STABLECOIN_USDC_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const PAUSE_EVENT_EVENT_DISCM: [u8; 8] = [32, 51, 61, 169, 156, 104, 130, 43];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PauseEvent {}
impl PauseEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        *__buf = reader;
        Ok(Self {})
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct PauseEventEvent(pub PauseEvent);
impl PauseEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != PAUSE_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = PauseEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&PAUSE_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const PROPOSE_ADDRESS_UPDATE_EVENT_EVENT_DISCM: [u8; 8] = [
    46, 255, 33, 127, 122, 204, 96, 63,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ProposeAddressUpdateEvent {
    pub address_field: AddressField,
    pub current_address: Pubkey,
    pub new_address: Pubkey,
    pub proposal_time: i64,
    pub ttl_secs: u64,
}
impl ProposeAddressUpdateEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let address_field: AddressField = crate::borsh_de_or_default(&mut reader)?;
        let current_address: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let new_address: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let proposal_time: i64 = crate::borsh_de_or_default(&mut reader)?;
        let ttl_secs: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            address_field,
            current_address,
            new_address,
            proposal_time,
            ttl_secs,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.address_field, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.current_address, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.new_address, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.proposal_time, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.ttl_secs, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct ProposeAddressUpdateEventEvent(pub ProposeAddressUpdateEvent);
impl ProposeAddressUpdateEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != PROPOSE_ADDRESS_UPDATE_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = ProposeAddressUpdateEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&PROPOSE_ADDRESS_UPDATE_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const REDEEM_LEVERCOIN_EXO_EVENT_EVENT_DISCM: [u8; 8] = [
    211, 197, 187, 4, 5, 123, 13, 57,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct RedeemLevercoinExoEvent {
    pub collateral_mint: Pubkey,
    pub redeemed: UFixValue64,
    pub nav: UFixValue64,
    pub collateral_usd_price: OraclePriceEvent,
    pub collateral_withdrawn: UFixValue64,
    pub fees_deposited: UFixValue64,
}
impl RedeemLevercoinExoEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let collateral_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let redeemed = if reader.is_empty() {
            Default::default()
        } else {
            <UFixValue64>::deserialize(&mut reader)?
        };
        let nav = if reader.is_empty() {
            Default::default()
        } else {
            <UFixValue64>::deserialize(&mut reader)?
        };
        let collateral_usd_price = if reader.is_empty() {
            Default::default()
        } else {
            <OraclePriceEvent>::deserialize(&mut reader)?
        };
        let collateral_withdrawn = if reader.is_empty() {
            Default::default()
        } else {
            <UFixValue64>::deserialize(&mut reader)?
        };
        let fees_deposited = if reader.is_empty() {
            Default::default()
        } else {
            <UFixValue64>::deserialize(&mut reader)?
        };
        *__buf = reader;
        Ok(Self {
            collateral_mint,
            redeemed,
            nav,
            collateral_usd_price,
            collateral_withdrawn,
            fees_deposited,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.collateral_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.redeemed, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.nav, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.collateral_usd_price, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.collateral_withdrawn, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fees_deposited, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct RedeemLevercoinExoEventEvent(pub RedeemLevercoinExoEvent);
impl RedeemLevercoinExoEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != REDEEM_LEVERCOIN_EXO_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = RedeemLevercoinExoEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&REDEEM_LEVERCOIN_EXO_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const REDEEM_LEVERCOIN_LST_EVENT_EVENT_DISCM: [u8; 8] = [
    109, 2, 214, 42, 98, 240, 234, 53,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct RedeemLevercoinLstEvent {
    pub redeemed: UFixValue64,
    pub nav: UFixValue64,
    pub sol_usd_price: OraclePriceEvent,
    pub lst_mint: Pubkey,
    pub lst_sol_price: UFixValue64,
    pub collateral_withdrawn: UFixValue64,
    pub fees_deposited: UFixValue64,
}
impl RedeemLevercoinLstEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let redeemed = if reader.is_empty() {
            Default::default()
        } else {
            <UFixValue64>::deserialize(&mut reader)?
        };
        let nav = if reader.is_empty() {
            Default::default()
        } else {
            <UFixValue64>::deserialize(&mut reader)?
        };
        let sol_usd_price = if reader.is_empty() {
            Default::default()
        } else {
            <OraclePriceEvent>::deserialize(&mut reader)?
        };
        let lst_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let lst_sol_price = if reader.is_empty() {
            Default::default()
        } else {
            <UFixValue64>::deserialize(&mut reader)?
        };
        let collateral_withdrawn = if reader.is_empty() {
            Default::default()
        } else {
            <UFixValue64>::deserialize(&mut reader)?
        };
        let fees_deposited = if reader.is_empty() {
            Default::default()
        } else {
            <UFixValue64>::deserialize(&mut reader)?
        };
        *__buf = reader;
        Ok(Self {
            redeemed,
            nav,
            sol_usd_price,
            lst_mint,
            lst_sol_price,
            collateral_withdrawn,
            fees_deposited,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.redeemed, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.nav, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.sol_usd_price, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.lst_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.lst_sol_price, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.collateral_withdrawn, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fees_deposited, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct RedeemLevercoinLstEventEvent(pub RedeemLevercoinLstEvent);
impl RedeemLevercoinLstEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != REDEEM_LEVERCOIN_LST_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = RedeemLevercoinLstEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&REDEEM_LEVERCOIN_LST_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const REDEEM_STABLECOIN_EXO_EVENT_EVENT_DISCM: [u8; 8] = [
    245, 26, 38, 7, 4, 107, 191, 196,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct RedeemStablecoinExoEvent {
    pub collateral_mint: Pubkey,
    pub redeemed: UFixValue64,
    pub nav: UFixValue64,
    pub collateral_usd_price: OraclePriceEvent,
    pub collateral_withdrawn: UFixValue64,
    pub fees_deposited: UFixValue64,
    pub virtual_stablecoin_supply: UFixValue64,
    pub stablecoin_supply: UFixValue64,
}
impl RedeemStablecoinExoEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let collateral_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let redeemed = if reader.is_empty() {
            Default::default()
        } else {
            <UFixValue64>::deserialize(&mut reader)?
        };
        let nav = if reader.is_empty() {
            Default::default()
        } else {
            <UFixValue64>::deserialize(&mut reader)?
        };
        let collateral_usd_price = if reader.is_empty() {
            Default::default()
        } else {
            <OraclePriceEvent>::deserialize(&mut reader)?
        };
        let collateral_withdrawn = if reader.is_empty() {
            Default::default()
        } else {
            <UFixValue64>::deserialize(&mut reader)?
        };
        let fees_deposited = if reader.is_empty() {
            Default::default()
        } else {
            <UFixValue64>::deserialize(&mut reader)?
        };
        let virtual_stablecoin_supply = if reader.is_empty() {
            Default::default()
        } else {
            <UFixValue64>::deserialize(&mut reader)?
        };
        let stablecoin_supply = if reader.is_empty() {
            Default::default()
        } else {
            <UFixValue64>::deserialize(&mut reader)?
        };
        *__buf = reader;
        Ok(Self {
            collateral_mint,
            redeemed,
            nav,
            collateral_usd_price,
            collateral_withdrawn,
            fees_deposited,
            virtual_stablecoin_supply,
            stablecoin_supply,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.collateral_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.redeemed, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.nav, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.collateral_usd_price, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.collateral_withdrawn, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fees_deposited, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.virtual_stablecoin_supply, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.stablecoin_supply, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct RedeemStablecoinExoEventEvent(pub RedeemStablecoinExoEvent);
impl RedeemStablecoinExoEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != REDEEM_STABLECOIN_EXO_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = RedeemStablecoinExoEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&REDEEM_STABLECOIN_EXO_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const REDEEM_STABLECOIN_LST_EVENT_EVENT_DISCM: [u8; 8] = [
    106, 30, 230, 38, 104, 152, 239, 238,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct RedeemStablecoinLstEvent {
    pub redeemed: UFixValue64,
    pub nav: UFixValue64,
    pub sol_usd_price: OraclePriceEvent,
    pub lst_mint: Pubkey,
    pub lst_sol_price: UFixValue64,
    pub collateral_withdrawn: UFixValue64,
    pub fees_deposited: UFixValue64,
}
impl RedeemStablecoinLstEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let redeemed = if reader.is_empty() {
            Default::default()
        } else {
            <UFixValue64>::deserialize(&mut reader)?
        };
        let nav = if reader.is_empty() {
            Default::default()
        } else {
            <UFixValue64>::deserialize(&mut reader)?
        };
        let sol_usd_price = if reader.is_empty() {
            Default::default()
        } else {
            <OraclePriceEvent>::deserialize(&mut reader)?
        };
        let lst_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let lst_sol_price = if reader.is_empty() {
            Default::default()
        } else {
            <UFixValue64>::deserialize(&mut reader)?
        };
        let collateral_withdrawn = if reader.is_empty() {
            Default::default()
        } else {
            <UFixValue64>::deserialize(&mut reader)?
        };
        let fees_deposited = if reader.is_empty() {
            Default::default()
        } else {
            <UFixValue64>::deserialize(&mut reader)?
        };
        *__buf = reader;
        Ok(Self {
            redeemed,
            nav,
            sol_usd_price,
            lst_mint,
            lst_sol_price,
            collateral_withdrawn,
            fees_deposited,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.redeemed, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.nav, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.sol_usd_price, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.lst_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.lst_sol_price, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.collateral_withdrawn, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fees_deposited, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct RedeemStablecoinLstEventEvent(pub RedeemStablecoinLstEvent);
impl RedeemStablecoinLstEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != REDEEM_STABLECOIN_LST_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = RedeemStablecoinLstEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&REDEEM_STABLECOIN_LST_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const REDEEM_STABLECOIN_USDC_EVENT_EVENT_DISCM: [u8; 8] = [
    107, 128, 104, 31, 190, 98, 172, 133,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct RedeemStablecoinUsdcEvent {
    pub stablecoin_burned: UFixValue64,
    pub stablecoin_fees: UFixValue64,
    pub usdc_withdrawn: UFixValue64,
    pub virtual_stablecoin_supply: UFixValue64,
}
impl RedeemStablecoinUsdcEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let stablecoin_burned = if reader.is_empty() {
            Default::default()
        } else {
            <UFixValue64>::deserialize(&mut reader)?
        };
        let stablecoin_fees = if reader.is_empty() {
            Default::default()
        } else {
            <UFixValue64>::deserialize(&mut reader)?
        };
        let usdc_withdrawn = if reader.is_empty() {
            Default::default()
        } else {
            <UFixValue64>::deserialize(&mut reader)?
        };
        let virtual_stablecoin_supply = if reader.is_empty() {
            Default::default()
        } else {
            <UFixValue64>::deserialize(&mut reader)?
        };
        *__buf = reader;
        Ok(Self {
            stablecoin_burned,
            stablecoin_fees,
            usdc_withdrawn,
            virtual_stablecoin_supply,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.stablecoin_burned, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.stablecoin_fees, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.usdc_withdrawn, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.virtual_stablecoin_supply, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct RedeemStablecoinUsdcEventEvent(pub RedeemStablecoinUsdcEvent);
impl RedeemStablecoinUsdcEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != REDEEM_STABLECOIN_USDC_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = RedeemStablecoinUsdcEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&REDEEM_STABLECOIN_USDC_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const REGISTER_EXO_EVENT_EVENT_DISCM: [u8; 8] = [19, 251, 201, 170, 74, 201, 0, 249];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct RegisterExoEvent {
    pub exo_pair: Pubkey,
    pub collateral_mint: Pubkey,
    pub levercoin_mint: Pubkey,
    pub collateral_vault: Pubkey,
    pub fee_vault: Pubkey,
    pub oracle: Pubkey,
    pub oracle_interval_secs: u64,
    pub oracle_conf_tolerance: UFixValue64,
    pub borrow_rate_curve_config: BorrowRateCurveConfig,
    pub borrow_rate_fee: UFixValue64,
}
impl RegisterExoEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let exo_pair: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let collateral_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let levercoin_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let collateral_vault: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let fee_vault: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let oracle: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let oracle_interval_secs: u64 = crate::borsh_de_or_default(&mut reader)?;
        let oracle_conf_tolerance = if reader.is_empty() {
            Default::default()
        } else {
            <UFixValue64>::deserialize(&mut reader)?
        };
        let borrow_rate_curve_config = if reader.is_empty() {
            Default::default()
        } else {
            <BorrowRateCurveConfig>::deserialize(&mut reader)?
        };
        let borrow_rate_fee = if reader.is_empty() {
            Default::default()
        } else {
            <UFixValue64>::deserialize(&mut reader)?
        };
        *__buf = reader;
        Ok(Self {
            exo_pair,
            collateral_mint,
            levercoin_mint,
            collateral_vault,
            fee_vault,
            oracle,
            oracle_interval_secs,
            oracle_conf_tolerance,
            borrow_rate_curve_config,
            borrow_rate_fee,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.exo_pair, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.collateral_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.levercoin_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.collateral_vault, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fee_vault, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.oracle, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.oracle_interval_secs, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.oracle_conf_tolerance, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.borrow_rate_curve_config, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.borrow_rate_fee, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct RegisterExoEventEvent(pub RegisterExoEvent);
impl RegisterExoEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != REGISTER_EXO_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = RegisterExoEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&REGISTER_EXO_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const REGISTER_LST_EVENT_EVENT_DISCM: [u8; 8] = [67, 129, 63, 97, 144, 52, 195, 33];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct RegisterLstEvent {
    pub header: Pubkey,
    pub mint: Pubkey,
    pub vault: Pubkey,
    pub pool_state: Pubkey,
}
impl RegisterLstEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let header: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let vault: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let pool_state: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            header,
            mint,
            vault,
            pool_state,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.header, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.vault, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.pool_state, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct RegisterLstEventEvent(pub RegisterLstEvent);
impl RegisterLstEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != REGISTER_LST_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = RegisterLstEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&REGISTER_LST_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const SETTLE_REBALANCE_PNL_EXO_EVENT_EVENT_DISCM: [u8; 8] = [
    194, 118, 20, 98, 233, 76, 198, 71,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SettleRebalancePnlExoEvent {
    pub collateral_mint: Pubkey,
    pub pnl: RebalancePnlValue,
    pub stablecoin_burned: UFixValue64,
    pub stablecoin_minted: UFixValue64,
}
impl SettleRebalancePnlExoEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let collateral_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let pnl = <RebalancePnlValue as borsh::BorshDeserialize>::deserialize_reader(
            &mut reader,
        )?;
        let stablecoin_burned = if reader.is_empty() {
            Default::default()
        } else {
            <UFixValue64>::deserialize(&mut reader)?
        };
        let stablecoin_minted = if reader.is_empty() {
            Default::default()
        } else {
            <UFixValue64>::deserialize(&mut reader)?
        };
        *__buf = reader;
        Ok(Self {
            collateral_mint,
            pnl,
            stablecoin_burned,
            stablecoin_minted,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.collateral_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.pnl, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.stablecoin_burned, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.stablecoin_minted, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct SettleRebalancePnlExoEventEvent(pub SettleRebalancePnlExoEvent);
impl SettleRebalancePnlExoEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SETTLE_REBALANCE_PNL_EXO_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = SettleRebalancePnlExoEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SETTLE_REBALANCE_PNL_EXO_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const SETTLE_REBALANCE_PNL_LST_EVENT_EVENT_DISCM: [u8; 8] = [
    126, 98, 197, 1, 27, 143, 31, 81,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SettleRebalancePnlLstEvent {
    pub pnl: RebalancePnlValue,
    pub stablecoin_burned: UFixValue64,
    pub stablecoin_minted: UFixValue64,
}
impl SettleRebalancePnlLstEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let pnl = <RebalancePnlValue as borsh::BorshDeserialize>::deserialize_reader(
            &mut reader,
        )?;
        let stablecoin_burned = if reader.is_empty() {
            Default::default()
        } else {
            <UFixValue64>::deserialize(&mut reader)?
        };
        let stablecoin_minted = if reader.is_empty() {
            Default::default()
        } else {
            <UFixValue64>::deserialize(&mut reader)?
        };
        *__buf = reader;
        Ok(Self {
            pnl,
            stablecoin_burned,
            stablecoin_minted,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.pnl, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.stablecoin_burned, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.stablecoin_minted, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct SettleRebalancePnlLstEventEvent(pub SettleRebalancePnlLstEvent);
impl SettleRebalancePnlLstEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SETTLE_REBALANCE_PNL_LST_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = SettleRebalancePnlLstEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SETTLE_REBALANCE_PNL_LST_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const SETTLE_VIRTUAL_STABLECOIN_EXO_EVENT_EVENT_DISCM: [u8; 8] = [
    154, 111, 203, 192, 57, 52, 51, 136,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SettleVirtualStablecoinExoEvent {
    pub collateral_mint: Pubkey,
    pub stablecoin_burned: UFixValue64,
    pub stablecoin_minted: UFixValue64,
    pub virtual_stablecoin_supply: UFixValue64,
    pub pool_drawdown_outstanding: UFixValue64,
    pub pool_balance: UFixValue64,
}
impl SettleVirtualStablecoinExoEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let collateral_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let stablecoin_burned = if reader.is_empty() {
            Default::default()
        } else {
            <UFixValue64>::deserialize(&mut reader)?
        };
        let stablecoin_minted = if reader.is_empty() {
            Default::default()
        } else {
            <UFixValue64>::deserialize(&mut reader)?
        };
        let virtual_stablecoin_supply = if reader.is_empty() {
            Default::default()
        } else {
            <UFixValue64>::deserialize(&mut reader)?
        };
        let pool_drawdown_outstanding = if reader.is_empty() {
            Default::default()
        } else {
            <UFixValue64>::deserialize(&mut reader)?
        };
        let pool_balance = if reader.is_empty() {
            Default::default()
        } else {
            <UFixValue64>::deserialize(&mut reader)?
        };
        *__buf = reader;
        Ok(Self {
            collateral_mint,
            stablecoin_burned,
            stablecoin_minted,
            virtual_stablecoin_supply,
            pool_drawdown_outstanding,
            pool_balance,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.collateral_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.stablecoin_burned, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.stablecoin_minted, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.virtual_stablecoin_supply, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.pool_drawdown_outstanding, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.pool_balance, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct SettleVirtualStablecoinExoEventEvent(pub SettleVirtualStablecoinExoEvent);
impl SettleVirtualStablecoinExoEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SETTLE_VIRTUAL_STABLECOIN_EXO_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = SettleVirtualStablecoinExoEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SETTLE_VIRTUAL_STABLECOIN_EXO_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const SETTLE_VIRTUAL_STABLECOIN_LST_EVENT_EVENT_DISCM: [u8; 8] = [
    79, 143, 203, 204, 148, 191, 4, 14,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SettleVirtualStablecoinLstEvent {
    pub stablecoin_burned: UFixValue64,
    pub stablecoin_minted: UFixValue64,
    pub virtual_stablecoin_supply: UFixValue64,
    pub pool_drawdown_outstanding: UFixValue64,
    pub pool_balance: UFixValue64,
}
impl SettleVirtualStablecoinLstEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let stablecoin_burned = if reader.is_empty() {
            Default::default()
        } else {
            <UFixValue64>::deserialize(&mut reader)?
        };
        let stablecoin_minted = if reader.is_empty() {
            Default::default()
        } else {
            <UFixValue64>::deserialize(&mut reader)?
        };
        let virtual_stablecoin_supply = if reader.is_empty() {
            Default::default()
        } else {
            <UFixValue64>::deserialize(&mut reader)?
        };
        let pool_drawdown_outstanding = if reader.is_empty() {
            Default::default()
        } else {
            <UFixValue64>::deserialize(&mut reader)?
        };
        let pool_balance = if reader.is_empty() {
            Default::default()
        } else {
            <UFixValue64>::deserialize(&mut reader)?
        };
        *__buf = reader;
        Ok(Self {
            stablecoin_burned,
            stablecoin_minted,
            virtual_stablecoin_supply,
            pool_drawdown_outstanding,
            pool_balance,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.stablecoin_burned, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.stablecoin_minted, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.virtual_stablecoin_supply, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.pool_drawdown_outstanding, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.pool_balance, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct SettleVirtualStablecoinLstEventEvent(pub SettleVirtualStablecoinLstEvent);
impl SettleVirtualStablecoinLstEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SETTLE_VIRTUAL_STABLECOIN_LST_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = SettleVirtualStablecoinLstEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SETTLE_VIRTUAL_STABLECOIN_LST_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const SETTLE_VIRTUAL_STABLECOIN_USDC_EVENT_EVENT_DISCM: [u8; 8] = [
    123, 57, 94, 153, 184, 185, 19, 158,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SettleVirtualStablecoinUsdcEvent {
    pub stablecoin_minted: UFixValue64,
    pub virtual_stablecoin_supply: UFixValue64,
    pub pool_balance: UFixValue64,
}
impl SettleVirtualStablecoinUsdcEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let stablecoin_minted = if reader.is_empty() {
            Default::default()
        } else {
            <UFixValue64>::deserialize(&mut reader)?
        };
        let virtual_stablecoin_supply = if reader.is_empty() {
            Default::default()
        } else {
            <UFixValue64>::deserialize(&mut reader)?
        };
        let pool_balance = if reader.is_empty() {
            Default::default()
        } else {
            <UFixValue64>::deserialize(&mut reader)?
        };
        *__buf = reader;
        Ok(Self {
            stablecoin_minted,
            virtual_stablecoin_supply,
            pool_balance,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.stablecoin_minted, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.virtual_stablecoin_supply, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.pool_balance, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct SettleVirtualStablecoinUsdcEventEvent(pub SettleVirtualStablecoinUsdcEvent);
impl SettleVirtualStablecoinUsdcEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SETTLE_VIRTUAL_STABLECOIN_USDC_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = SettleVirtualStablecoinUsdcEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SETTLE_VIRTUAL_STABLECOIN_USDC_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const SWAP_EXO_TO_USDC_EVENT_EVENT_DISCM: [u8; 8] = [
    192, 253, 46, 74, 155, 62, 202, 95,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SwapExoToUsdcEvent {
    pub collateral_mint: Pubkey,
    pub collateral_deposited: UFixValue64,
    pub collateral_usd_price: UFixValue64,
    pub usdc_withdrawn: UFixValue64,
}
impl SwapExoToUsdcEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let collateral_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let collateral_deposited = if reader.is_empty() {
            Default::default()
        } else {
            <UFixValue64>::deserialize(&mut reader)?
        };
        let collateral_usd_price = if reader.is_empty() {
            Default::default()
        } else {
            <UFixValue64>::deserialize(&mut reader)?
        };
        let usdc_withdrawn = if reader.is_empty() {
            Default::default()
        } else {
            <UFixValue64>::deserialize(&mut reader)?
        };
        *__buf = reader;
        Ok(Self {
            collateral_mint,
            collateral_deposited,
            collateral_usd_price,
            usdc_withdrawn,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.collateral_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.collateral_deposited, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.collateral_usd_price, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.usdc_withdrawn, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct SwapExoToUsdcEventEvent(pub SwapExoToUsdcEvent);
impl SwapExoToUsdcEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SWAP_EXO_TO_USDC_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = SwapExoToUsdcEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SWAP_EXO_TO_USDC_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const SWAP_LST_TO_LST_EVENT_EVENT_DISCM: [u8; 8] = [
    132, 61, 108, 235, 150, 214, 171, 212,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SwapLstToLstEvent {
    pub lst_a_mint: Pubkey,
    pub lst_a_in: UFixValue64,
    pub lst_a_fees_extracted: UFixValue64,
    pub lst_b_mint: Pubkey,
    pub lst_b_out: UFixValue64,
}
impl SwapLstToLstEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let lst_a_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let lst_a_in = if reader.is_empty() {
            Default::default()
        } else {
            <UFixValue64>::deserialize(&mut reader)?
        };
        let lst_a_fees_extracted = if reader.is_empty() {
            Default::default()
        } else {
            <UFixValue64>::deserialize(&mut reader)?
        };
        let lst_b_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let lst_b_out = if reader.is_empty() {
            Default::default()
        } else {
            <UFixValue64>::deserialize(&mut reader)?
        };
        *__buf = reader;
        Ok(Self {
            lst_a_mint,
            lst_a_in,
            lst_a_fees_extracted,
            lst_b_mint,
            lst_b_out,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.lst_a_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.lst_a_in, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.lst_a_fees_extracted, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.lst_b_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.lst_b_out, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct SwapLstToLstEventEvent(pub SwapLstToLstEvent);
impl SwapLstToLstEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SWAP_LST_TO_LST_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = SwapLstToLstEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SWAP_LST_TO_LST_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const SWAP_LST_TO_USDC_EVENT_EVENT_DISCM: [u8; 8] = [
    8, 54, 75, 98, 162, 102, 253, 37,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SwapLstToUsdcEvent {
    pub lst_mint: Pubkey,
    pub lst_deposited: UFixValue64,
    pub sol_rebalance_usd_price: UFixValue64,
    pub usdc_withdrawn: UFixValue64,
}
impl SwapLstToUsdcEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let lst_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let lst_deposited = if reader.is_empty() {
            Default::default()
        } else {
            <UFixValue64>::deserialize(&mut reader)?
        };
        let sol_rebalance_usd_price = if reader.is_empty() {
            Default::default()
        } else {
            <UFixValue64>::deserialize(&mut reader)?
        };
        let usdc_withdrawn = if reader.is_empty() {
            Default::default()
        } else {
            <UFixValue64>::deserialize(&mut reader)?
        };
        *__buf = reader;
        Ok(Self {
            lst_mint,
            lst_deposited,
            sol_rebalance_usd_price,
            usdc_withdrawn,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.lst_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.lst_deposited, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.sol_rebalance_usd_price, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.usdc_withdrawn, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct SwapLstToUsdcEventEvent(pub SwapLstToUsdcEvent);
impl SwapLstToUsdcEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SWAP_LST_TO_USDC_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = SwapLstToUsdcEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SWAP_LST_TO_USDC_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const SWAP_USDC_TO_EXO_EVENT_EVENT_DISCM: [u8; 8] = [
    153, 230, 128, 51, 128, 167, 160, 4,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SwapUsdcToExoEvent {
    pub collateral_mint: Pubkey,
    pub usdc_deposited: UFixValue64,
    pub collateral_withdrawn: UFixValue64,
    pub collateral_usd_price: UFixValue64,
}
impl SwapUsdcToExoEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let collateral_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let usdc_deposited = if reader.is_empty() {
            Default::default()
        } else {
            <UFixValue64>::deserialize(&mut reader)?
        };
        let collateral_withdrawn = if reader.is_empty() {
            Default::default()
        } else {
            <UFixValue64>::deserialize(&mut reader)?
        };
        let collateral_usd_price = if reader.is_empty() {
            Default::default()
        } else {
            <UFixValue64>::deserialize(&mut reader)?
        };
        *__buf = reader;
        Ok(Self {
            collateral_mint,
            usdc_deposited,
            collateral_withdrawn,
            collateral_usd_price,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.collateral_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.usdc_deposited, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.collateral_withdrawn, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.collateral_usd_price, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct SwapUsdcToExoEventEvent(pub SwapUsdcToExoEvent);
impl SwapUsdcToExoEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SWAP_USDC_TO_EXO_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = SwapUsdcToExoEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SWAP_USDC_TO_EXO_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const SWAP_USDC_TO_LST_EVENT_EVENT_DISCM: [u8; 8] = [
    51, 183, 182, 250, 39, 255, 201, 80,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SwapUsdcToLstEvent {
    pub lst_mint: Pubkey,
    pub usdc_deposited: UFixValue64,
    pub lst_withdrawn: UFixValue64,
    pub sol_rebalance_usd_price: UFixValue64,
}
impl SwapUsdcToLstEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let lst_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let usdc_deposited = if reader.is_empty() {
            Default::default()
        } else {
            <UFixValue64>::deserialize(&mut reader)?
        };
        let lst_withdrawn = if reader.is_empty() {
            Default::default()
        } else {
            <UFixValue64>::deserialize(&mut reader)?
        };
        let sol_rebalance_usd_price = if reader.is_empty() {
            Default::default()
        } else {
            <UFixValue64>::deserialize(&mut reader)?
        };
        *__buf = reader;
        Ok(Self {
            lst_mint,
            usdc_deposited,
            lst_withdrawn,
            sol_rebalance_usd_price,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.lst_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.usdc_deposited, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.lst_withdrawn, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.sol_rebalance_usd_price, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct SwapUsdcToLstEventEvent(pub SwapUsdcToLstEvent);
impl SwapUsdcToLstEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SWAP_USDC_TO_LST_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = SwapUsdcToLstEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SWAP_USDC_TO_LST_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const UNPAUSE_EVENT_EVENT_DISCM: [u8; 8] = [134, 156, 8, 215, 185, 128, 192, 217];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UnpauseEvent {}
impl UnpauseEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        *__buf = reader;
        Ok(Self {})
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct UnpauseEventEvent(pub UnpauseEvent);
impl UnpauseEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UNPAUSE_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = UnpauseEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UNPAUSE_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const UPDATE_BORROW_RATE_CURVE_CONFIG_EVENT_EVENT_DISCM: [u8; 8] = [
    202, 107, 241, 157, 215, 231, 0, 135,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UpdateBorrowRateCurveConfigEvent {
    pub old_curve_config: BorrowRateCurveConfig,
    pub new_curve_config: BorrowRateCurveConfig,
}
impl UpdateBorrowRateCurveConfigEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let old_curve_config = if reader.is_empty() {
            Default::default()
        } else {
            <BorrowRateCurveConfig>::deserialize(&mut reader)?
        };
        let new_curve_config = if reader.is_empty() {
            Default::default()
        } else {
            <BorrowRateCurveConfig>::deserialize(&mut reader)?
        };
        *__buf = reader;
        Ok(Self {
            old_curve_config,
            new_curve_config,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.old_curve_config, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.new_curve_config, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateBorrowRateCurveConfigEventEvent(pub UpdateBorrowRateCurveConfigEvent);
impl UpdateBorrowRateCurveConfigEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_BORROW_RATE_CURVE_CONFIG_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = UpdateBorrowRateCurveConfigEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_BORROW_RATE_CURVE_CONFIG_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const UPDATE_FEE_EVENT_EVENT_DISCM: [u8; 8] = [79, 79, 188, 14, 247, 41, 59, 187];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UpdateFeeEvent {
    pub old_fee: UFixValue64,
    pub new_fee: UFixValue64,
}
impl UpdateFeeEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let old_fee = if reader.is_empty() {
            Default::default()
        } else {
            <UFixValue64>::deserialize(&mut reader)?
        };
        let new_fee = if reader.is_empty() {
            Default::default()
        } else {
            <UFixValue64>::deserialize(&mut reader)?
        };
        *__buf = reader;
        Ok(Self { old_fee, new_fee })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.old_fee, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.new_fee, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateFeeEventEvent(pub UpdateFeeEvent);
impl UpdateFeeEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_FEE_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = UpdateFeeEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_FEE_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const UPDATE_LEVERCOIN_FEES_EVENT_EVENT_DISCM: [u8; 8] = [
    84, 176, 60, 195, 204, 115, 242, 169,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UpdateLevercoinFeesEvent {
    pub old_levercoin_fees: LevercoinFees,
    pub new_levercoin_fees: LevercoinFees,
}
impl UpdateLevercoinFeesEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let old_levercoin_fees = if reader.is_empty() {
            Default::default()
        } else {
            <LevercoinFees>::deserialize(&mut reader)?
        };
        let new_levercoin_fees = if reader.is_empty() {
            Default::default()
        } else {
            <LevercoinFees>::deserialize(&mut reader)?
        };
        *__buf = reader;
        Ok(Self {
            old_levercoin_fees,
            new_levercoin_fees,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.old_levercoin_fees, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.new_levercoin_fees, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateLevercoinFeesEventEvent(pub UpdateLevercoinFeesEvent);
impl UpdateLevercoinFeesEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_LEVERCOIN_FEES_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = UpdateLevercoinFeesEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_LEVERCOIN_FEES_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const UPDATE_LEVERCOIN_MARKET_CAP_LIMIT_EVENT_EVENT_DISCM: [u8; 8] = [
    8, 97, 206, 242, 245, 149, 100, 108,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UpdateLevercoinMarketCapLimitEvent {
    pub old_levercoin_market_cap_limit: UFixValue64,
    pub new_levercoin_market_cap_limit: UFixValue64,
}
impl UpdateLevercoinMarketCapLimitEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let old_levercoin_market_cap_limit = if reader.is_empty() {
            Default::default()
        } else {
            <UFixValue64>::deserialize(&mut reader)?
        };
        let new_levercoin_market_cap_limit = if reader.is_empty() {
            Default::default()
        } else {
            <UFixValue64>::deserialize(&mut reader)?
        };
        *__buf = reader;
        Ok(Self {
            old_levercoin_market_cap_limit,
            new_levercoin_market_cap_limit,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(
            &self.old_levercoin_market_cap_limit,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.new_levercoin_market_cap_limit,
            &mut writer,
        )?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateLevercoinMarketCapLimitEventEvent(
    pub UpdateLevercoinMarketCapLimitEvent,
);
impl UpdateLevercoinMarketCapLimitEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_LEVERCOIN_MARKET_CAP_LIMIT_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = UpdateLevercoinMarketCapLimitEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_LEVERCOIN_MARKET_CAP_LIMIT_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const UPDATE_LST_PRICES_EVENT_EVENT_DISCM: [u8; 8] = [
    104, 0, 189, 118, 195, 155, 77, 151,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UpdateLstPricesEvent {
    pub updated_mints: Vec<Pubkey>,
    pub new_total_sol: UFixValue64,
}
impl UpdateLstPricesEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let updated_mints: Vec<Pubkey> = crate::borsh_de_or_default(&mut reader)?;
        let new_total_sol = if reader.is_empty() {
            Default::default()
        } else {
            <UFixValue64>::deserialize(&mut reader)?
        };
        *__buf = reader;
        Ok(Self {
            updated_mints,
            new_total_sol,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.updated_mints, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.new_total_sol, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateLstPricesEventEvent(pub UpdateLstPricesEvent);
impl UpdateLstPricesEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_LST_PRICES_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = UpdateLstPricesEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_LST_PRICES_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const UPDATE_LST_REBALANCE_FEE_EVENT_EVENT_DISCM: [u8; 8] = [
    17, 84, 12, 240, 183, 23, 159, 70,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UpdateLstRebalanceFeeEvent {
    pub lst_mint: Pubkey,
    pub old_rebalance_fee: UFixValue64,
    pub new_rebalance_fee: UFixValue64,
}
impl UpdateLstRebalanceFeeEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let lst_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let old_rebalance_fee = if reader.is_empty() {
            Default::default()
        } else {
            <UFixValue64>::deserialize(&mut reader)?
        };
        let new_rebalance_fee = if reader.is_empty() {
            Default::default()
        } else {
            <UFixValue64>::deserialize(&mut reader)?
        };
        *__buf = reader;
        Ok(Self {
            lst_mint,
            old_rebalance_fee,
            new_rebalance_fee,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.lst_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.old_rebalance_fee, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.new_rebalance_fee, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateLstRebalanceFeeEventEvent(pub UpdateLstRebalanceFeeEvent);
impl UpdateLstRebalanceFeeEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_LST_REBALANCE_FEE_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = UpdateLstRebalanceFeeEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_LST_REBALANCE_FEE_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const UPDATE_ORACLE_ADDRESS_EVENT_EVENT_DISCM: [u8; 8] = [
    28, 247, 140, 128, 156, 178, 148, 53,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UpdateOracleAddressEvent {
    pub old_oracle: Pubkey,
    pub new_oracle: Pubkey,
}
impl UpdateOracleAddressEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let old_oracle: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let new_oracle: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { old_oracle, new_oracle })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.old_oracle, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.new_oracle, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateOracleAddressEventEvent(pub UpdateOracleAddressEvent);
impl UpdateOracleAddressEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_ORACLE_ADDRESS_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = UpdateOracleAddressEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_ORACLE_ADDRESS_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const UPDATE_ORACLE_CONF_EVENT_EVENT_DISCM: [u8; 8] = [
    190, 89, 82, 192, 231, 31, 206, 91,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UpdateOracleConfEvent {
    pub old_oracle_conf_tolerance: UFixValue64,
    pub new_oracle_conf_tolerance: UFixValue64,
}
impl UpdateOracleConfEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let old_oracle_conf_tolerance = if reader.is_empty() {
            Default::default()
        } else {
            <UFixValue64>::deserialize(&mut reader)?
        };
        let new_oracle_conf_tolerance = if reader.is_empty() {
            Default::default()
        } else {
            <UFixValue64>::deserialize(&mut reader)?
        };
        *__buf = reader;
        Ok(Self {
            old_oracle_conf_tolerance,
            new_oracle_conf_tolerance,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.old_oracle_conf_tolerance, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.new_oracle_conf_tolerance, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateOracleConfEventEvent(pub UpdateOracleConfEvent);
impl UpdateOracleConfEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_ORACLE_CONF_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = UpdateOracleConfEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_ORACLE_CONF_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const UPDATE_ORACLE_INTERVAL_EVENT_EVENT_DISCM: [u8; 8] = [
    190, 132, 209, 41, 79, 137, 195, 226,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UpdateOracleIntervalEvent {
    pub old_oracle_interval_secs: u64,
    pub new_oracle_interval_secs: u64,
}
impl UpdateOracleIntervalEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let old_oracle_interval_secs: u64 = crate::borsh_de_or_default(&mut reader)?;
        let new_oracle_interval_secs: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            old_oracle_interval_secs,
            new_oracle_interval_secs,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.old_oracle_interval_secs, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.new_oracle_interval_secs, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateOracleIntervalEventEvent(pub UpdateOracleIntervalEvent);
impl UpdateOracleIntervalEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_ORACLE_INTERVAL_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = UpdateOracleIntervalEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_ORACLE_INTERVAL_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const UPDATE_PAR_TOLERANCE_EVENT_EVENT_DISCM: [u8; 8] = [
    13, 28, 13, 209, 214, 57, 197, 161,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UpdateParToleranceEvent {
    pub old_par_tolerance: UFixValue64,
    pub new_par_tolerance: UFixValue64,
}
impl UpdateParToleranceEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let old_par_tolerance = if reader.is_empty() {
            Default::default()
        } else {
            <UFixValue64>::deserialize(&mut reader)?
        };
        let new_par_tolerance = if reader.is_empty() {
            Default::default()
        } else {
            <UFixValue64>::deserialize(&mut reader)?
        };
        *__buf = reader;
        Ok(Self {
            old_par_tolerance,
            new_par_tolerance,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.old_par_tolerance, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.new_par_tolerance, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateParToleranceEventEvent(pub UpdateParToleranceEvent);
impl UpdateParToleranceEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_PAR_TOLERANCE_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = UpdateParToleranceEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_PAR_TOLERANCE_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const UPDATE_REBALANCE_CURVE_CONFIG_EVENT_EVENT_DISCM: [u8; 8] = [
    91, 73, 254, 1, 250, 240, 86, 48,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UpdateRebalanceCurveConfigEvent {
    pub old_curve_config: RebalanceCurveConfig,
    pub new_curve_config: RebalanceCurveConfig,
}
impl UpdateRebalanceCurveConfigEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let old_curve_config = if reader.is_empty() {
            Default::default()
        } else {
            <RebalanceCurveConfig>::deserialize(&mut reader)?
        };
        let new_curve_config = if reader.is_empty() {
            Default::default()
        } else {
            <RebalanceCurveConfig>::deserialize(&mut reader)?
        };
        *__buf = reader;
        Ok(Self {
            old_curve_config,
            new_curve_config,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.old_curve_config, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.new_curve_config, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateRebalanceCurveConfigEventEvent(pub UpdateRebalanceCurveConfigEvent);
impl UpdateRebalanceCurveConfigEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_REBALANCE_CURVE_CONFIG_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = UpdateRebalanceCurveConfigEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_REBALANCE_CURVE_CONFIG_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const UPDATE_STABLECOIN_MINT_THRESHOLD_EVENT_EVENT_DISCM: [u8; 8] = [
    81, 104, 35, 118, 8, 9, 211, 62,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UpdateStablecoinMintThresholdEvent {
    pub old_stablecoin_mint_threshold: UFixValue64,
    pub new_stablecoin_mint_threshold: UFixValue64,
}
impl UpdateStablecoinMintThresholdEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let old_stablecoin_mint_threshold = if reader.is_empty() {
            Default::default()
        } else {
            <UFixValue64>::deserialize(&mut reader)?
        };
        let new_stablecoin_mint_threshold = if reader.is_empty() {
            Default::default()
        } else {
            <UFixValue64>::deserialize(&mut reader)?
        };
        *__buf = reader;
        Ok(Self {
            old_stablecoin_mint_threshold,
            new_stablecoin_mint_threshold,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(
            &self.old_stablecoin_mint_threshold,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.new_stablecoin_mint_threshold,
            &mut writer,
        )?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateStablecoinMintThresholdEventEvent(
    pub UpdateStablecoinMintThresholdEvent,
);
impl UpdateStablecoinMintThresholdEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_STABLECOIN_MINT_THRESHOLD_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = UpdateStablecoinMintThresholdEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_STABLECOIN_MINT_THRESHOLD_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const UPDATE_YIELD_HARVEST_CONFIG_EVENT_EVENT_DISCM: [u8; 8] = [
    67, 168, 96, 20, 78, 117, 245, 206,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UpdateYieldHarvestConfigEvent {
    pub old_yield_harvest_config: YieldHarvestConfig,
    pub new_yield_harvest_config: YieldHarvestConfig,
}
impl UpdateYieldHarvestConfigEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let old_yield_harvest_config = if reader.is_empty() {
            Default::default()
        } else {
            <YieldHarvestConfig>::deserialize(&mut reader)?
        };
        let new_yield_harvest_config = if reader.is_empty() {
            Default::default()
        } else {
            <YieldHarvestConfig>::deserialize(&mut reader)?
        };
        *__buf = reader;
        Ok(Self {
            old_yield_harvest_config,
            new_yield_harvest_config,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.old_yield_harvest_config, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.new_yield_harvest_config, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateYieldHarvestConfigEventEvent(pub UpdateYieldHarvestConfigEvent);
impl UpdateYieldHarvestConfigEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_YIELD_HARVEST_CONFIG_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = UpdateYieldHarvestConfigEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_YIELD_HARVEST_CONFIG_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const WITHDRAW_FEES_EVENT_EVENT_DISCM: [u8; 8] = [
    236, 118, 138, 90, 139, 173, 177, 89,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WithdrawFeesEvent {
    pub mint: Pubkey,
    pub vault: Pubkey,
    pub treasury_ata: Pubkey,
    pub amount: u64,
}
impl WithdrawFeesEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let vault: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let treasury_ata: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            mint,
            vault,
            treasury_ata,
            amount,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.vault, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.treasury_ata, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.amount, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct WithdrawFeesEventEvent(pub WithdrawFeesEvent);
impl WithdrawFeesEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != WITHDRAW_FEES_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = WithdrawFeesEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&WITHDRAW_FEES_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
