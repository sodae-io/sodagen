use solana_pubkey::Pubkey;
#[allow(unused_imports)]
use crate::*;
pub const ACCEPT_PARTNER_CLAIM_AUTHORITY_EVENT_EVENT_DISCM: [u8; 8] = [
    224, 217, 211, 179, 135, 101, 211, 134,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AcceptPartnerClaimAuthorityEvent {
    pub new_authority: Pubkey,
    pub partner: Pubkey,
    pub old_authority: Pubkey,
}
impl AcceptPartnerClaimAuthorityEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let new_authority: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let partner: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let old_authority: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            new_authority,
            partner,
            old_authority,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.new_authority, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.partner, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.old_authority, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct AcceptPartnerClaimAuthorityEventEvent(pub AcceptPartnerClaimAuthorityEvent);
impl AcceptPartnerClaimAuthorityEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != ACCEPT_PARTNER_CLAIM_AUTHORITY_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = AcceptPartnerClaimAuthorityEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&ACCEPT_PARTNER_CLAIM_AUTHORITY_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const ACCEPT_PROTOCOL_AUTHORITY_EVENT_EVENT_DISCM: [u8; 8] = [
    127, 2, 205, 112, 109, 145, 155, 226,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AcceptProtocolAuthorityEvent {
    pub new_authority: Pubkey,
    pub config: Pubkey,
    pub old_authority: Pubkey,
}
impl AcceptProtocolAuthorityEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let new_authority: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let config: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let old_authority: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            new_authority,
            config,
            old_authority,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.new_authority, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.config, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.old_authority, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct AcceptProtocolAuthorityEventEvent(pub AcceptProtocolAuthorityEvent);
impl AcceptProtocolAuthorityEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != ACCEPT_PROTOCOL_AUTHORITY_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = AcceptProtocolAuthorityEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&ACCEPT_PROTOCOL_AUTHORITY_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const COLLECT_FEE_EVENT_EVENT_DISCM: [u8; 8] = [2, 91, 59, 165, 111, 4, 58, 254];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CollectFeeEvent {
    pub pool: Pubkey,
    pub owner: Pubkey,
    pub position_nft_mint: Pubkey,
    pub amount_a: u64,
    pub amount_b: u64,
}
impl CollectFeeEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let pool: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let owner: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let position_nft_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let amount_a: u64 = crate::borsh_de_or_default(&mut reader)?;
        let amount_b: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            pool,
            owner,
            position_nft_mint,
            amount_a,
            amount_b,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.pool, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.owner, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.position_nft_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.amount_a, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.amount_b, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct CollectFeeEventEvent(pub CollectFeeEvent);
impl CollectFeeEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != COLLECT_FEE_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = CollectFeeEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&COLLECT_FEE_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const COLLECT_PARTNER_FEE_EVENT_EVENT_DISCM: [u8; 8] = [
    78, 0, 247, 158, 203, 180, 139, 40,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CollectPartnerFeeEvent {
    pub claim_authority: Pubkey,
    pub partner: Pubkey,
    pub pool: Pubkey,
    pub amount_a: u64,
    pub amount_b: u64,
}
impl CollectPartnerFeeEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let claim_authority: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let partner: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let pool: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let amount_a: u64 = crate::borsh_de_or_default(&mut reader)?;
        let amount_b: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            claim_authority,
            partner,
            pool,
            amount_a,
            amount_b,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.claim_authority, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.partner, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.pool, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.amount_a, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.amount_b, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct CollectPartnerFeeEventEvent(pub CollectPartnerFeeEvent);
impl CollectPartnerFeeEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != COLLECT_PARTNER_FEE_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = CollectPartnerFeeEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&COLLECT_PARTNER_FEE_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const COLLECT_PROTOCOL_FEE_EVENT_EVENT_DISCM: [u8; 8] = [
    206, 87, 17, 79, 45, 41, 213, 61,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CollectProtocolFeeEvent {
    pub claim_authority: Pubkey,
    pub config: Pubkey,
    pub pool: Pubkey,
    pub amount_a: u64,
    pub amount_b: u64,
}
impl CollectProtocolFeeEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let claim_authority: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let config: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let pool: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let amount_a: u64 = crate::borsh_de_or_default(&mut reader)?;
        let amount_b: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            claim_authority,
            config,
            pool,
            amount_a,
            amount_b,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.claim_authority, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.config, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.pool, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.amount_a, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.amount_b, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct CollectProtocolFeeEventEvent(pub CollectProtocolFeeEvent);
impl CollectProtocolFeeEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != COLLECT_PROTOCOL_FEE_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = CollectProtocolFeeEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&COLLECT_PROTOCOL_FEE_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const COLLECT_REWARDER_EVENT_EVENT_DISCM: [u8; 8] = [
    73, 73, 46, 155, 111, 184, 138, 115,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CollectRewarderEvent {
    pub pool: Pubkey,
    pub owner: Pubkey,
    pub position_nft_mint: Pubkey,
    pub amount: u64,
    pub rewarder_mint: Pubkey,
}
impl CollectRewarderEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let pool: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let owner: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let position_nft_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let rewarder_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            pool,
            owner,
            position_nft_mint,
            amount,
            rewarder_mint,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.pool, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.owner, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.position_nft_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.rewarder_mint, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct CollectRewarderEventEvent(pub CollectRewarderEvent);
impl CollectRewarderEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != COLLECT_REWARDER_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = CollectRewarderEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&COLLECT_REWARDER_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const CREATE_CLMM_POOL_EVENT_EVENT_DISCM: [u8; 8] = [
    150, 186, 51, 20, 78, 38, 146, 112,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CreateClmmPoolEvent {
    pub payer: Pubkey,
    pub config: Pubkey,
    pub fee_tier: Pubkey,
    pub pool: Pubkey,
    pub token_a: Pubkey,
    pub token_b: Pubkey,
}
impl CreateClmmPoolEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let payer: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let config: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let fee_tier: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let pool: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let token_a: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let token_b: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            payer,
            config,
            fee_tier,
            pool,
            token_a,
            token_b,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.payer, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.config, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fee_tier, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.pool, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_a, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_b, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct CreateClmmPoolEventEvent(pub CreateClmmPoolEvent);
impl CreateClmmPoolEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CREATE_CLMM_POOL_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = CreateClmmPoolEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CREATE_CLMM_POOL_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const CREATE_CLMMPOOL_METADATA_EVENT_EVENT_DISCM: [u8; 8] = [
    28, 85, 177, 6, 27, 33, 6, 100,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CreateClmmpoolMetadataEvent {
    pub clmmpool: Pubkey,
    pub clmmpool_metadata: Pubkey,
}
impl CreateClmmpoolMetadataEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let clmmpool: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let clmmpool_metadata: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            clmmpool,
            clmmpool_metadata,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.clmmpool, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.clmmpool_metadata, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct CreateClmmpoolMetadataEventEvent(pub CreateClmmpoolMetadataEvent);
impl CreateClmmpoolMetadataEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CREATE_CLMMPOOL_METADATA_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = CreateClmmpoolMetadataEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CREATE_CLMMPOOL_METADATA_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const CREATE_FEE_TIER_EVENT_EVENT_DISCM: [u8; 8] = [
    127, 22, 80, 66, 233, 23, 107, 219,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CreateFeeTierEvent {
    pub payer: Pubkey,
    pub config: Pubkey,
    pub fee_tier: Pubkey,
}
impl CreateFeeTierEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let payer: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let config: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let fee_tier: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { payer, config, fee_tier })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.payer, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.config, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fee_tier, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct CreateFeeTierEventEvent(pub CreateFeeTierEvent);
impl CreateFeeTierEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CREATE_FEE_TIER_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = CreateFeeTierEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CREATE_FEE_TIER_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const CREATE_PARTNER_EVENT_EVENT_DISCM: [u8; 8] = [
    56, 176, 81, 26, 181, 197, 23, 69,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CreatePartnerEvent {
    pub config: Pubkey,
    pub protocol_authority: Pubkey,
    pub partner: Pubkey,
}
impl CreatePartnerEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let config: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let protocol_authority: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let partner: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            config,
            protocol_authority,
            partner,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.config, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.protocol_authority, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.partner, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct CreatePartnerEventEvent(pub CreatePartnerEvent);
impl CreatePartnerEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CREATE_PARTNER_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = CreatePartnerEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CREATE_PARTNER_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const CREATE_TICK_ARRAY_EVENT_EVENT_DISCM: [u8; 8] = [
    115, 167, 47, 9, 214, 148, 117, 237,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CreateTickArrayEvent {
    pub payer: Pubkey,
    pub pool: Pubkey,
    pub tick_array: Pubkey,
    pub array_index: u16,
}
impl CreateTickArrayEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let payer: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let pool: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let tick_array: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let array_index: u16 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            payer,
            pool,
            tick_array,
            array_index,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.payer, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.pool, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.tick_array, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.array_index, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct CreateTickArrayEventEvent(pub CreateTickArrayEvent);
impl CreateTickArrayEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CREATE_TICK_ARRAY_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = CreateTickArrayEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CREATE_TICK_ARRAY_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const CREATE_TICK_ARRAY_MAP_EVENT_EVENT_DISCM: [u8; 8] = [
    108, 173, 106, 173, 108, 161, 205, 118,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CreateTickArrayMapEvent {
    pub payer: Pubkey,
    pub pool: Pubkey,
    pub tick_array_map: Pubkey,
}
impl CreateTickArrayMapEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let payer: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let pool: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let tick_array_map: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            payer,
            pool,
            tick_array_map,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.payer, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.pool, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.tick_array_map, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct CreateTickArrayMapEventEvent(pub CreateTickArrayMapEvent);
impl CreateTickArrayMapEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CREATE_TICK_ARRAY_MAP_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = CreateTickArrayMapEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CREATE_TICK_ARRAY_MAP_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const DECREASE_LIQUIDITY_EVENT_EVENT_DISCM: [u8; 8] = [
    58, 222, 86, 58, 68, 50, 85, 56,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DecreaseLiquidityEvent {
    pub pool: Pubkey,
    pub owner: Pubkey,
    pub position_nft_mint: Pubkey,
    pub delta_liquidity: u128,
    pub amount_a: u64,
    pub amount_b: u64,
}
impl DecreaseLiquidityEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let pool: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let owner: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let position_nft_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let delta_liquidity: u128 = crate::borsh_de_or_default(&mut reader)?;
        let amount_a: u64 = crate::borsh_de_or_default(&mut reader)?;
        let amount_b: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            pool,
            owner,
            position_nft_mint,
            delta_liquidity,
            amount_a,
            amount_b,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.pool, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.owner, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.position_nft_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.delta_liquidity, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.amount_a, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.amount_b, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct DecreaseLiquidityEventEvent(pub DecreaseLiquidityEvent);
impl DecreaseLiquidityEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != DECREASE_LIQUIDITY_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = DecreaseLiquidityEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&DECREASE_LIQUIDITY_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const INCREASE_LIQUIDITY_EVENT_EVENT_DISCM: [u8; 8] = [
    49, 79, 105, 212, 32, 34, 30, 84,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct IncreaseLiquidityEvent {
    pub pool: Pubkey,
    pub owner: Pubkey,
    pub position_nft_mint: Pubkey,
    pub delta_liquidity: u128,
    pub amount_a: u64,
    pub amount_b: u64,
}
impl IncreaseLiquidityEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let pool: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let owner: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let position_nft_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let delta_liquidity: u128 = crate::borsh_de_or_default(&mut reader)?;
        let amount_a: u64 = crate::borsh_de_or_default(&mut reader)?;
        let amount_b: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            pool,
            owner,
            position_nft_mint,
            delta_liquidity,
            amount_a,
            amount_b,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.pool, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.owner, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.position_nft_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.delta_liquidity, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.amount_a, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.amount_b, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct IncreaseLiquidityEventEvent(pub IncreaseLiquidityEvent);
impl IncreaseLiquidityEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INCREASE_LIQUIDITY_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = IncreaseLiquidityEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INCREASE_LIQUIDITY_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const INCREASE_LIQUIDITY_WITH_FIXED_TOKEN_EVENT_EVENT_DISCM: [u8; 8] = [
    66, 159, 66, 27, 243, 55, 13, 176,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct IncreaseLiquidityWithFixedTokenEvent {
    pub pool: Pubkey,
    pub owner: Pubkey,
    pub position_nft_mint: Pubkey,
    pub delta_liquidity: u128,
    pub amount_a: u64,
    pub amount_b: u64,
}
impl IncreaseLiquidityWithFixedTokenEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let pool: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let owner: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let position_nft_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let delta_liquidity: u128 = crate::borsh_de_or_default(&mut reader)?;
        let amount_a: u64 = crate::borsh_de_or_default(&mut reader)?;
        let amount_b: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            pool,
            owner,
            position_nft_mint,
            delta_liquidity,
            amount_a,
            amount_b,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.pool, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.owner, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.position_nft_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.delta_liquidity, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.amount_a, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.amount_b, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct IncreaseLiquidityWithFixedTokenEventEvent(
    pub IncreaseLiquidityWithFixedTokenEvent,
);
impl IncreaseLiquidityWithFixedTokenEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INCREASE_LIQUIDITY_WITH_FIXED_TOKEN_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = IncreaseLiquidityWithFixedTokenEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INCREASE_LIQUIDITY_WITH_FIXED_TOKEN_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const INITIALIZE_CLMM_CONFIG_EVENT_EVENT_DISCM: [u8; 8] = [
    155, 202, 135, 177, 23, 27, 26, 203,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct InitializeClmmConfigEvent {
    pub config: Pubkey,
    pub fee_authority: Pubkey,
    pub claim_authority: Pubkey,
    pub create_pool_authority: Pubkey,
    pub fee_rate: u16,
}
impl InitializeClmmConfigEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let config: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let fee_authority: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let claim_authority: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let create_pool_authority: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let fee_rate: u16 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            config,
            fee_authority,
            claim_authority,
            create_pool_authority,
            fee_rate,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.config, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fee_authority, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.claim_authority, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.create_pool_authority, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fee_rate, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct InitializeClmmConfigEventEvent(pub InitializeClmmConfigEvent);
impl InitializeClmmConfigEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INITIALIZE_CLMM_CONFIG_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = InitializeClmmConfigEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INITIALIZE_CLMM_CONFIG_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const OPEN_POSITION_EVENT_EVENT_DISCM: [u8; 8] = [
    83, 43, 164, 147, 169, 87, 81, 172,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct OpenPositionEvent {
    pub pool: Pubkey,
    pub owner: Pubkey,
    pub position_nft_mint: Pubkey,
    pub position: Pubkey,
    pub tick_lower_index: i32,
    pub tick_upper_index: i32,
}
impl OpenPositionEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let pool: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let owner: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let position_nft_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let position: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let tick_lower_index: i32 = crate::borsh_de_or_default(&mut reader)?;
        let tick_upper_index: i32 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            pool,
            owner,
            position_nft_mint,
            position,
            tick_lower_index,
            tick_upper_index,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.pool, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.owner, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.position_nft_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.position, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.tick_lower_index, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.tick_upper_index, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct OpenPositionEventEvent(pub OpenPositionEvent);
impl OpenPositionEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != OPEN_POSITION_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = OpenPositionEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&OPEN_POSITION_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const REMOVE_POSITION_EVENT_EVENT_DISCM: [u8; 8] = [
    187, 160, 184, 228, 105, 43, 143, 65,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct RemovePositionEvent {
    pub pool: Pubkey,
    pub owner: Pubkey,
    pub position_nft_mint: Pubkey,
    pub position: Pubkey,
}
impl RemovePositionEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let pool: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let owner: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let position_nft_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let position: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            pool,
            owner,
            position_nft_mint,
            position,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.pool, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.owner, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.position_nft_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.position, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct RemovePositionEventEvent(pub RemovePositionEvent);
impl RemovePositionEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != REMOVE_POSITION_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = RemovePositionEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&REMOVE_POSITION_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const SWAP_EVENT_EVENT_DISCM: [u8; 8] = [64, 198, 205, 232, 38, 8, 113, 226];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SwapEvent {
    pub pool: Pubkey,
    pub owner: Pubkey,
    pub partner: Pubkey,
    pub a_to_b: bool,
    pub amount_in: u64,
    pub amount_out: u64,
    pub ref_amount: u64,
    pub fee_amount: u64,
    pub protocol_amount: u64,
    pub vault_a_amount: u64,
    pub vault_b_amount: u64,
}
impl SwapEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let pool: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let owner: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let partner: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let a_to_b: bool = crate::borsh_de_or_default(&mut reader)?;
        let amount_in: u64 = crate::borsh_de_or_default(&mut reader)?;
        let amount_out: u64 = crate::borsh_de_or_default(&mut reader)?;
        let ref_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let fee_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let protocol_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let vault_a_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let vault_b_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            pool,
            owner,
            partner,
            a_to_b,
            amount_in,
            amount_out,
            ref_amount,
            fee_amount,
            protocol_amount,
            vault_a_amount,
            vault_b_amount,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.pool, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.owner, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.partner, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.a_to_b, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.amount_in, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.amount_out, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.ref_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fee_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.protocol_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.vault_a_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.vault_b_amount, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct SwapEventEvent(pub SwapEvent);
impl SwapEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SWAP_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = SwapEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SWAP_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const SWAP_WITH_PARTNER_EVENT_EVENT_DISCM: [u8; 8] = [
    90, 20, 5, 252, 145, 37, 38, 150,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SwapWithPartnerEvent {
    pub pool: Pubkey,
    pub owner: Pubkey,
    pub partner: Pubkey,
    pub a_to_b: bool,
    pub amount_in: u64,
    pub amount_out: u64,
    pub ref_amount: u64,
    pub fee_amount: u64,
    pub protocol_amount: u64,
    pub vault_a_amount: u64,
    pub vault_b_amount: u64,
}
impl SwapWithPartnerEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let pool: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let owner: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let partner: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let a_to_b: bool = crate::borsh_de_or_default(&mut reader)?;
        let amount_in: u64 = crate::borsh_de_or_default(&mut reader)?;
        let amount_out: u64 = crate::borsh_de_or_default(&mut reader)?;
        let ref_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let fee_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let protocol_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let vault_a_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let vault_b_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            pool,
            owner,
            partner,
            a_to_b,
            amount_in,
            amount_out,
            ref_amount,
            fee_amount,
            protocol_amount,
            vault_a_amount,
            vault_b_amount,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.pool, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.owner, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.partner, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.a_to_b, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.amount_in, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.amount_out, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.ref_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fee_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.protocol_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.vault_a_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.vault_b_amount, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct SwapWithPartnerEventEvent(pub SwapWithPartnerEvent);
impl SwapWithPartnerEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SWAP_WITH_PARTNER_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = SwapWithPartnerEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SWAP_WITH_PARTNER_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const TRANSFER_PARTNER_CLAIM_AUTHORITY_EVENT_EVENT_DISCM: [u8; 8] = [
    6, 92, 137, 243, 158, 74, 130, 144,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TransferPartnerClaimAuthorityEvent {
    pub new_authority: Pubkey,
    pub partner: Pubkey,
    pub old_authority: Pubkey,
}
impl TransferPartnerClaimAuthorityEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let new_authority: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let partner: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let old_authority: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            new_authority,
            partner,
            old_authority,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.new_authority, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.partner, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.old_authority, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct TransferPartnerClaimAuthorityEventEvent(
    pub TransferPartnerClaimAuthorityEvent,
);
impl TransferPartnerClaimAuthorityEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != TRANSFER_PARTNER_CLAIM_AUTHORITY_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = TransferPartnerClaimAuthorityEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&TRANSFER_PARTNER_CLAIM_AUTHORITY_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const TRANSFER_PROTOCOL_AUTHORITY_EVENT_EVENT_DISCM: [u8; 8] = [
    154, 204, 141, 219, 71, 14, 188, 51,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TransferProtocolAuthorityEvent {
    pub new_authority: Pubkey,
    pub config: Pubkey,
    pub old_authority: Pubkey,
}
impl TransferProtocolAuthorityEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let new_authority: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let config: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let old_authority: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            new_authority,
            config,
            old_authority,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.new_authority, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.config, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.old_authority, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct TransferProtocolAuthorityEventEvent(pub TransferProtocolAuthorityEvent);
impl TransferProtocolAuthorityEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != TRANSFER_PROTOCOL_AUTHORITY_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = TransferProtocolAuthorityEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&TRANSFER_PROTOCOL_AUTHORITY_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const UPDATE_CONFIG_EVENT_EVENT_DISCM: [u8; 8] = [
    96, 112, 253, 102, 59, 78, 75, 134,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UpdateConfigEvent {
    pub config: Pubkey,
    pub new_protocol_fee_rate: Option<u16>,
    pub create_pool_authority: Option<Pubkey>,
    pub claim_authority: Option<Pubkey>,
}
impl UpdateConfigEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let config: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let new_protocol_fee_rate: Option<u16> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let create_pool_authority: Option<Pubkey> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let claim_authority: Option<Pubkey> = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            config,
            new_protocol_fee_rate,
            create_pool_authority,
            claim_authority,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.config, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.new_protocol_fee_rate, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.create_pool_authority, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.claim_authority, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateConfigEventEvent(pub UpdateConfigEvent);
impl UpdateConfigEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_CONFIG_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = UpdateConfigEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_CONFIG_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const UPDATE_FEE_RATE_EVENT_EVENT_DISCM: [u8; 8] = [
    39, 59, 218, 11, 10, 142, 179, 252,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UpdateFeeRateEvent {
    pub pool: Pubkey,
    pub config: Pubkey,
}
impl UpdateFeeRateEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let pool: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let config: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { pool, config })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.pool, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.config, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateFeeRateEventEvent(pub UpdateFeeRateEvent);
impl UpdateFeeRateEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_FEE_RATE_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = UpdateFeeRateEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_FEE_RATE_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const UPDATE_PARTNER_EVENT_EVENT_DISCM: [u8; 8] = [
    101, 134, 118, 84, 55, 38, 23, 138,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UpdatePartnerEvent {
    pub authority: Pubkey,
    pub partner: Pubkey,
    pub new_fee_rate: Option<u16>,
    pub new_claim_authority: Option<Pubkey>,
}
impl UpdatePartnerEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let authority: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let partner: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let new_fee_rate: Option<u16> = crate::borsh_de_or_default(&mut reader)?;
        let new_claim_authority: Option<Pubkey> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self {
            authority,
            partner,
            new_fee_rate,
            new_claim_authority,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.authority, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.partner, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.new_fee_rate, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.new_claim_authority, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdatePartnerEventEvent(pub UpdatePartnerEvent);
impl UpdatePartnerEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_PARTNER_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = UpdatePartnerEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_PARTNER_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
