use borsh::{BorshDeserialize, BorshSerialize};
use solana_pubkey::Pubkey;
#[allow(unused_imports)]
use crate::*;
pub const CLONE_ACCOUNT_DISCM: [u8; 8] = [20, 243, 87, 121, 2, 202, 130, 130];
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
pub struct Clone {
    pub admin: Pubkey,
    pub auth: [Pubkey; 10],
    pub bump: u8,
    pub collateral: Collateral,
    pub comet_collateral_ild_liquidator_fee_bps: u16,
    pub comet_onasset_ild_liquidator_fee_bps: u16,
    pub borrow_liquidator_fee_bps: u16,
    pub treasury_address: Pubkey,
    pub event_counter: u64,
    pub non_auth_liquidations_enabled: bool,
}
impl Clone {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let admin: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let auth: [Pubkey; 10] = crate::borsh_de_or_default(&mut reader)?;
        let bump: u8 = crate::borsh_de_or_default(&mut reader)?;
        let collateral = if reader.is_empty() {
            Default::default()
        } else {
            <Collateral>::deserialize(&mut reader)?
        };
        let comet_collateral_ild_liquidator_fee_bps: u16 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let comet_onasset_ild_liquidator_fee_bps: u16 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let borrow_liquidator_fee_bps: u16 = crate::borsh_de_or_default(&mut reader)?;
        let treasury_address: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let event_counter: u64 = crate::borsh_de_or_default(&mut reader)?;
        let non_auth_liquidations_enabled: bool = crate::borsh_de_or_default(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self {
            admin,
            auth,
            bump,
            collateral,
            comet_collateral_ild_liquidator_fee_bps,
            comet_onasset_ild_liquidator_fee_bps,
            borrow_liquidator_fee_bps,
            treasury_address,
            event_counter,
            non_auth_liquidations_enabled,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.admin, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.auth, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.bump, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.collateral, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.comet_collateral_ild_liquidator_fee_bps,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.comet_onasset_ild_liquidator_fee_bps,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.borrow_liquidator_fee_bps, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.treasury_address, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.event_counter, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.non_auth_liquidations_enabled,
            &mut writer,
        )?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct CloneAccount(pub Clone);
impl CloneAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CLONE_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(Clone::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CLONE_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const POOLS_ACCOUNT_DISCM: [u8; 8] = [107, 216, 188, 161, 30, 47, 151, 9];
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
pub struct Pools {
    pub pools: Vec<Pool>,
}
impl Pools {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let pools: Vec<Pool> = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { pools })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.pools, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct PoolsAccount(pub Pools);
impl PoolsAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != POOLS_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(Pools::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&POOLS_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const ORACLES_ACCOUNT_DISCM: [u8; 8] = [118, 5, 196, 234, 105, 220, 8, 8];
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
pub struct Oracles {
    pub oracles: Vec<OracleInfo>,
}
impl Oracles {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let oracles: Vec<OracleInfo> = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { oracles })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.oracles, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct OraclesAccount(pub Oracles);
impl OraclesAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != ORACLES_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(Oracles::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&ORACLES_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const USER_ACCOUNT_DISCM: [u8; 8] = [159, 117, 95, 227, 239, 151, 58, 236];
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
pub struct User {
    pub borrows: Vec<Borrow>,
    pub comet: Comet,
}
impl User {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let borrows: Vec<Borrow> = crate::borsh_de_or_default(&mut reader)?;
        let comet = if reader.is_empty() {
            Default::default()
        } else {
            <Comet>::deserialize(&mut reader)?
        };
        *__buf = reader;
        Ok(Self { borrows, comet })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.borrows, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.comet, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct UserAccount(pub User);
impl UserAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != USER_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(User::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&USER_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
