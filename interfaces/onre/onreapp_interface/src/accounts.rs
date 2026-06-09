use borsh::{BorshDeserialize, BorshSerialize};
use solana_pubkey::Pubkey;
#[allow(unused_imports)]
use crate::*;
pub const OFFER_ACCOUNT_DISCM: [u8; 8] = [215, 88, 60, 71, 170, 162, 73, 229];
#[derive(
    Clone,
    Debug,
    BorshDeserialize,
    BorshSerialize,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
pub struct Offer {
    pub token_in_mint: Pubkey,
    pub token_out_mint: Pubkey,
    pub vectors: [OfferVector; 10],
    pub fee_basis_points: u16,
    pub bump: u8,
    pub needs_approval: u8,
    pub allow_permissionless: u8,
    #[serde(with = "crate::big_array_serde")]
    pub reserved: [u8; 131],
}
impl Offer {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let token_in_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let token_out_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let vectors: [OfferVector; 10] = crate::borsh_de_or_default(&mut reader)?;
        let fee_basis_points: u16 = crate::borsh_de_or_default(&mut reader)?;
        let bump: u8 = crate::borsh_de_or_default(&mut reader)?;
        let needs_approval: u8 = crate::borsh_de_or_default(&mut reader)?;
        let allow_permissionless: u8 = crate::borsh_de_or_default(&mut reader)?;
        let reserved = <[u8; 131] as borsh::BorshDeserialize>::deserialize_reader(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self {
            token_in_mint,
            token_out_mint,
            vectors,
            fee_basis_points,
            bump,
            needs_approval,
            allow_permissionless,
            reserved,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.token_in_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_out_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.vectors, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fee_basis_points, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.bump, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.needs_approval, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.allow_permissionless, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.reserved, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct OfferAccount(pub Offer);
impl OfferAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != OFFER_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(Offer::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&OFFER_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const PERMISSIONLESS_AUTHORITY_ACCOUNT_DISCM: [u8; 8] = [
    241, 34, 5, 97, 43, 102, 149, 52,
];
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
pub struct PermissionlessAuthority {
    pub name: String,
}
impl PermissionlessAuthority {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let name: String = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { name })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.name, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct PermissionlessAuthorityAccount(pub PermissionlessAuthority);
impl PermissionlessAuthorityAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != PERMISSIONLESS_AUTHORITY_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(PermissionlessAuthority::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&PERMISSIONLESS_AUTHORITY_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const REDEMPTION_OFFER_ACCOUNT_DISCM: [u8; 8] = [
    170, 229, 178, 15, 184, 107, 140, 41,
];
#[derive(
    Clone,
    Debug,
    BorshDeserialize,
    BorshSerialize,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
pub struct RedemptionOffer {
    pub offer: Pubkey,
    pub token_in_mint: Pubkey,
    pub token_out_mint: Pubkey,
    pub executed_redemptions: u128,
    pub requested_redemptions: u128,
    pub fee_basis_points: u16,
    pub request_counter: u64,
    pub bump: u8,
    #[serde(with = "crate::big_array_serde")]
    pub reserved: [u8; 109],
}
impl RedemptionOffer {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let offer: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let token_in_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let token_out_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let executed_redemptions: u128 = crate::borsh_de_or_default(&mut reader)?;
        let requested_redemptions: u128 = crate::borsh_de_or_default(&mut reader)?;
        let fee_basis_points: u16 = crate::borsh_de_or_default(&mut reader)?;
        let request_counter: u64 = crate::borsh_de_or_default(&mut reader)?;
        let bump: u8 = crate::borsh_de_or_default(&mut reader)?;
        let reserved = <[u8; 109] as borsh::BorshDeserialize>::deserialize_reader(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self {
            offer,
            token_in_mint,
            token_out_mint,
            executed_redemptions,
            requested_redemptions,
            fee_basis_points,
            request_counter,
            bump,
            reserved,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.offer, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_in_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_out_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.executed_redemptions, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.requested_redemptions, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fee_basis_points, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.request_counter, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.bump, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.reserved, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct RedemptionOfferAccount(pub RedemptionOffer);
impl RedemptionOfferAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != REDEMPTION_OFFER_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(RedemptionOffer::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&REDEMPTION_OFFER_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const REDEMPTION_REQUEST_ACCOUNT_DISCM: [u8; 8] = [
    117, 157, 214, 214, 64, 160, 31, 58,
];
#[derive(
    Clone,
    Debug,
    BorshDeserialize,
    BorshSerialize,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
pub struct RedemptionRequest {
    pub offer: Pubkey,
    pub request_id: u64,
    pub redeemer: Pubkey,
    pub amount: u64,
    pub bump: u8,
    #[serde(with = "crate::big_array_serde")]
    pub reserved: [u8; 127],
}
impl RedemptionRequest {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let offer: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let request_id: u64 = crate::borsh_de_or_default(&mut reader)?;
        let redeemer: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let bump: u8 = crate::borsh_de_or_default(&mut reader)?;
        let reserved = <[u8; 127] as borsh::BorshDeserialize>::deserialize_reader(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self {
            offer,
            request_id,
            redeemer,
            amount,
            bump,
            reserved,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.offer, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.request_id, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.redeemer, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.bump, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.reserved, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct RedemptionRequestAccount(pub RedemptionRequest);
impl RedemptionRequestAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != REDEMPTION_REQUEST_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(RedemptionRequest::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&REDEMPTION_REQUEST_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const STATE_ACCOUNT_DISCM: [u8; 8] = [216, 146, 107, 94, 104, 75, 182, 177];
#[derive(
    Clone,
    Debug,
    BorshDeserialize,
    BorshSerialize,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
pub struct State {
    pub boss: Pubkey,
    pub proposed_boss: Pubkey,
    pub is_killed: bool,
    pub onyc_mint: Pubkey,
    pub admins: [Pubkey; 20],
    pub approver1: Pubkey,
    pub approver2: Pubkey,
    pub bump: u8,
    pub max_supply: u64,
    pub redemption_admin: Pubkey,
    #[serde(with = "crate::big_array_serde")]
    pub reserved: [u8; 96],
}
impl State {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let boss: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let proposed_boss: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let is_killed: bool = crate::borsh_de_or_default(&mut reader)?;
        let onyc_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let admins: [Pubkey; 20] = crate::borsh_de_or_default(&mut reader)?;
        let approver1: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let approver2: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let bump: u8 = crate::borsh_de_or_default(&mut reader)?;
        let max_supply: u64 = crate::borsh_de_or_default(&mut reader)?;
        let redemption_admin: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let reserved = <[u8; 96] as borsh::BorshDeserialize>::deserialize_reader(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self {
            boss,
            proposed_boss,
            is_killed,
            onyc_mint,
            admins,
            approver1,
            approver2,
            bump,
            max_supply,
            redemption_admin,
            reserved,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.boss, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.proposed_boss, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.is_killed, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.onyc_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.admins, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.approver1, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.approver2, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.bump, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.max_supply, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.redemption_admin, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.reserved, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct StateAccount(pub State);
impl StateAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != STATE_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(State::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&STATE_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
