use solana_pubkey::Pubkey;
#[allow(unused_imports)]
use crate::*;
pub const CONFIG_CREATION_EVENT_DISCM: [u8; 8] = [22, 226, 190, 234, 176, 24, 250, 11];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ConfigCreation {
    pub admin: Pubkey,
    pub config: Pubkey,
    pub quote_token_mint: Pubkey,
    pub protocol_fee_share: u32,
    pub protocol_fee_reserve: Pubkey,
    pub creator_fee_pool: Pubkey,
    pub fee_recipient_change_cooldown: u32,
    pub market_settings: MarketSettingsInput,
}
impl ConfigCreation {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let admin: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let config: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let quote_token_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let protocol_fee_share: u32 = crate::borsh_de_or_default(&mut reader)?;
        let protocol_fee_reserve: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let creator_fee_pool: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let fee_recipient_change_cooldown: u32 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let market_settings = if reader.is_empty() {
            Default::default()
        } else {
            <MarketSettingsInput>::deserialize(&mut reader)?
        };
        *__buf = reader;
        Ok(Self {
            admin,
            config,
            quote_token_mint,
            protocol_fee_share,
            protocol_fee_reserve,
            creator_fee_pool,
            fee_recipient_change_cooldown,
            market_settings,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.admin, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.config, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.quote_token_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.protocol_fee_share, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.protocol_fee_reserve, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.creator_fee_pool, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.fee_recipient_change_cooldown,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.market_settings, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct ConfigCreationEvent(pub ConfigCreation);
impl ConfigCreationEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CONFIG_CREATION_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = ConfigCreation::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CONFIG_CREATION_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const CONFIG_DEFAULT_MARKET_SETTINGS_UPDATE_EVENT_DISCM: [u8; 8] = [
    86, 122, 72, 223, 93, 152, 224, 97,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ConfigDefaultMarketSettingsUpdate {
    pub config: Pubkey,
    pub new_market_settings: MarketSettingsInput,
}
impl ConfigDefaultMarketSettingsUpdate {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let config: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let new_market_settings = if reader.is_empty() {
            Default::default()
        } else {
            <MarketSettingsInput>::deserialize(&mut reader)?
        };
        *__buf = reader;
        Ok(Self {
            config,
            new_market_settings,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.config, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.new_market_settings, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct ConfigDefaultMarketSettingsUpdateEvent(pub ConfigDefaultMarketSettingsUpdate);
impl ConfigDefaultMarketSettingsUpdateEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CONFIG_DEFAULT_MARKET_SETTINGS_UPDATE_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = ConfigDefaultMarketSettingsUpdate::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CONFIG_DEFAULT_MARKET_SETTINGS_UPDATE_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const CONFIG_FEE_SETTINGS_UPDATE_EVENT_DISCM: [u8; 8] = [
    137, 142, 221, 54, 111, 184, 135, 2,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ConfigFeeSettingsUpdate {
    pub config: Pubkey,
    pub new_protocol_fee_reserve: Pubkey,
    pub new_protocol_fee_share: u32,
    pub new_creator_fee_pool: Pubkey,
    pub new_fee_recipient_change_cooldown: u32,
}
impl ConfigFeeSettingsUpdate {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let config: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let new_protocol_fee_reserve: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let new_protocol_fee_share: u32 = crate::borsh_de_or_default(&mut reader)?;
        let new_creator_fee_pool: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let new_fee_recipient_change_cooldown: u32 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self {
            config,
            new_protocol_fee_reserve,
            new_protocol_fee_share,
            new_creator_fee_pool,
            new_fee_recipient_change_cooldown,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.config, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.new_protocol_fee_reserve, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.new_protocol_fee_share, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.new_creator_fee_pool, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.new_fee_recipient_change_cooldown,
            &mut writer,
        )?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct ConfigFeeSettingsUpdateEvent(pub ConfigFeeSettingsUpdate);
impl ConfigFeeSettingsUpdateEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CONFIG_FEE_SETTINGS_UPDATE_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = ConfigFeeSettingsUpdate::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CONFIG_FEE_SETTINGS_UPDATE_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const CONFIG_OWNERSHIP_TRANSFER_EVENT_DISCM: [u8; 8] = [
    244, 36, 50, 39, 145, 158, 100, 124,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ConfigOwnershipTransfer {
    pub config: Pubkey,
    pub new_admin: Pubkey,
}
impl ConfigOwnershipTransfer {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let config: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let new_admin: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { config, new_admin })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.config, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.new_admin, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct ConfigOwnershipTransferEvent(pub ConfigOwnershipTransfer);
impl ConfigOwnershipTransferEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CONFIG_OWNERSHIP_TRANSFER_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = ConfigOwnershipTransfer::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CONFIG_OWNERSHIP_TRANSFER_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const FEE_RESERVE_UPDATE_EVENT_DISCM: [u8; 8] = [
    114, 245, 136, 81, 246, 204, 133, 12,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct FeeReserveUpdate {
    pub config: Pubkey,
    pub market: Pubkey,
    pub new_fee_reserve: Option<Pubkey>,
}
impl FeeReserveUpdate {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let config: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let market: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let new_fee_reserve: Option<Pubkey> = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            config,
            market,
            new_fee_reserve,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.config, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.market, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.new_fee_reserve, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct FeeReserveUpdateEvent(pub FeeReserveUpdate);
impl FeeReserveUpdateEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != FEE_RESERVE_UPDATE_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = FeeReserveUpdate::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&FEE_RESERVE_UPDATE_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const MARKET_CREATION_EVENT_DISCM: [u8; 8] = [77, 174, 149, 37, 77, 226, 133, 219];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MarketCreation {
    pub config: Pubkey,
    pub market: Pubkey,
    pub creator: Pubkey,
    pub token_mint_0: Pubkey,
    pub swap_authority: Option<Pubkey>,
}
impl MarketCreation {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let config: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let market: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let creator: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let token_mint_0: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let swap_authority: Option<Pubkey> = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            config,
            market,
            creator,
            token_mint_0,
            swap_authority,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.config, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.market, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.creator, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_mint_0, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.swap_authority, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct MarketCreationEvent(pub MarketCreation);
impl MarketCreationEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != MARKET_CREATION_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = MarketCreation::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&MARKET_CREATION_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const MARKET_SWAP_AUTHORITY_REMOVED_EVENT_DISCM: [u8; 8] = [
    61, 11, 41, 211, 46, 134, 227, 249,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MarketSwapAuthorityRemoved {
    pub config: Pubkey,
    pub market: Pubkey,
}
impl MarketSwapAuthorityRemoved {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let config: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let market: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { config, market })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.config, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.market, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct MarketSwapAuthorityRemovedEvent(pub MarketSwapAuthorityRemoved);
impl MarketSwapAuthorityRemovedEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != MARKET_SWAP_AUTHORITY_REMOVED_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = MarketSwapAuthorityRemoved::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&MARKET_SWAP_AUTHORITY_REMOVED_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const SWAP_EVENT_DISCM: [u8; 8] = [81, 108, 227, 190, 205, 208, 10, 196];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Swap {
    pub config: Pubkey,
    pub user: Pubkey,
    pub market: Pubkey,
    pub zero_for_one: bool,
    pub swap_result: SwapResult,
}
impl Swap {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let config: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let user: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let market: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let zero_for_one: bool = crate::borsh_de_or_default(&mut reader)?;
        let swap_result = if reader.is_empty() {
            Default::default()
        } else {
            <SwapResult>::deserialize(&mut reader)?
        };
        *__buf = reader;
        Ok(Self {
            config,
            user,
            market,
            zero_for_one,
            swap_result,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.config, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.user, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.market, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.zero_for_one, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.swap_result, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct SwapEvent(pub Swap);
impl SwapEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SWAP_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = Swap::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SWAP_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
