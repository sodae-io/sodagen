use solana_pubkey::Pubkey;
#[allow(unused_imports)]
use crate::*;
pub const ADMIN_ADDED_EVENT_EVENT_DISCM: [u8; 8] = [68, 183, 187, 200, 190, 214, 20, 77];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AdminAddedEvent {
    pub admin: Pubkey,
    pub boss: Pubkey,
}
impl AdminAddedEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let admin: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let boss: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { admin, boss })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.admin, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.boss, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct AdminAddedEventEvent(pub AdminAddedEvent);
impl AdminAddedEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != ADMIN_ADDED_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = AdminAddedEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&ADMIN_ADDED_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const ADMIN_REMOVED_EVENT_EVENT_DISCM: [u8; 8] = [226, 5, 12, 53, 69, 56, 172, 132];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AdminRemovedEvent {
    pub admin: Pubkey,
    pub boss: Pubkey,
}
impl AdminRemovedEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let admin: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let boss: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { admin, boss })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.admin, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.boss, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct AdminRemovedEventEvent(pub AdminRemovedEvent);
impl AdminRemovedEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != ADMIN_REMOVED_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = AdminRemovedEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&ADMIN_REMOVED_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const ADMINS_CLEARED_EVENT_EVENT_DISCM: [u8; 8] = [
    202, 81, 156, 32, 66, 208, 159, 153,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AdminsClearedEvent {
    pub boss: Pubkey,
}
impl AdminsClearedEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let boss: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { boss })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.boss, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct AdminsClearedEventEvent(pub AdminsClearedEvent);
impl AdminsClearedEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != ADMINS_CLEARED_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = AdminsClearedEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&ADMINS_CLEARED_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const ALL_OFFER_VECTORS_DELETED_EVENT_EVENT_DISCM: [u8; 8] = [
    13, 237, 37, 22, 175, 128, 178, 200,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AllOfferVectorsDeletedEvent {
    pub offer_pda: Pubkey,
    pub vectors_deleted_count: u8,
}
impl AllOfferVectorsDeletedEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let offer_pda: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let vectors_deleted_count: u8 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            offer_pda,
            vectors_deleted_count,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.offer_pda, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.vectors_deleted_count, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct AllOfferVectorsDeletedEventEvent(pub AllOfferVectorsDeletedEvent);
impl AllOfferVectorsDeletedEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != ALL_OFFER_VECTORS_DELETED_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = AllOfferVectorsDeletedEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&ALL_OFFER_VECTORS_DELETED_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const APPROVER_ADDED_EVENT_EVENT_DISCM: [u8; 8] = [
    130, 197, 173, 181, 53, 38, 162, 134,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ApproverAddedEvent {
    pub approver: Pubkey,
    pub boss: Pubkey,
}
impl ApproverAddedEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let approver: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let boss: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { approver, boss })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.approver, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.boss, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct ApproverAddedEventEvent(pub ApproverAddedEvent);
impl ApproverAddedEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != APPROVER_ADDED_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = ApproverAddedEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&APPROVER_ADDED_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const APPROVER_REMOVED_EVENT_EVENT_DISCM: [u8; 8] = [
    234, 1, 25, 206, 97, 119, 7, 23,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ApproverRemovedEvent {
    pub approver: Pubkey,
    pub boss: Pubkey,
}
impl ApproverRemovedEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let approver: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let boss: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { approver, boss })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.approver, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.boss, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct ApproverRemovedEventEvent(pub ApproverRemovedEvent);
impl ApproverRemovedEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != APPROVER_REMOVED_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = ApproverRemovedEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&APPROVER_REMOVED_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const BOSS_ACCEPTED_EVENT_EVENT_DISCM: [u8; 8] = [
    11, 133, 76, 152, 219, 5, 220, 103,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct BossAcceptedEvent {
    pub old_boss: Pubkey,
    pub new_boss: Pubkey,
}
impl BossAcceptedEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let old_boss: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let new_boss: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { old_boss, new_boss })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.old_boss, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.new_boss, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct BossAcceptedEventEvent(pub BossAcceptedEvent);
impl BossAcceptedEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != BOSS_ACCEPTED_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = BossAcceptedEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&BOSS_ACCEPTED_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const BOSS_PROPOSED_EVENT_EVENT_DISCM: [u8; 8] = [22, 117, 195, 6, 169, 57, 141, 17];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct BossProposedEvent {
    pub current_boss: Pubkey,
    pub proposed_boss: Pubkey,
}
impl BossProposedEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let current_boss: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let proposed_boss: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            current_boss,
            proposed_boss,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.current_boss, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.proposed_boss, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct BossProposedEventEvent(pub BossProposedEvent);
impl BossProposedEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != BOSS_PROPOSED_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = BossProposedEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&BOSS_PROPOSED_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const BUFFER_ACCRUED_EVENT_EVENT_DISCM: [u8; 8] = [
    142, 74, 130, 231, 194, 58, 5, 20,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct BufferAccruedEvent {
    pub token_in_mint: Pubkey,
    pub onyc_mint: Pubkey,
    pub seconds_elapsed: u64,
    pub apr_delta: u64,
    pub buffer_mint_amount: u64,
    pub reserve_mint_amount: u64,
    pub management_fee_mint_amount: u64,
    pub performance_fee_mint_amount: u64,
    pub old_previous_supply: u64,
    pub new_previous_supply: u64,
    pub old_previous_performance_fee_high_watermark: u64,
    pub new_performance_fee_high_watermark: u64,
    pub current_nav: u64,
    pub post_accrual_supply: u64,
    pub timestamp: i64,
}
impl BufferAccruedEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let token_in_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let onyc_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let seconds_elapsed: u64 = crate::borsh_de_or_default(&mut reader)?;
        let apr_delta: u64 = crate::borsh_de_or_default(&mut reader)?;
        let buffer_mint_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let reserve_mint_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let management_fee_mint_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let performance_fee_mint_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let old_previous_supply: u64 = crate::borsh_de_or_default(&mut reader)?;
        let new_previous_supply: u64 = crate::borsh_de_or_default(&mut reader)?;
        let old_previous_performance_fee_high_watermark: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let new_performance_fee_high_watermark: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let current_nav: u64 = crate::borsh_de_or_default(&mut reader)?;
        let post_accrual_supply: u64 = crate::borsh_de_or_default(&mut reader)?;
        let timestamp: i64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            token_in_mint,
            onyc_mint,
            seconds_elapsed,
            apr_delta,
            buffer_mint_amount,
            reserve_mint_amount,
            management_fee_mint_amount,
            performance_fee_mint_amount,
            old_previous_supply,
            new_previous_supply,
            old_previous_performance_fee_high_watermark,
            new_performance_fee_high_watermark,
            current_nav,
            post_accrual_supply,
            timestamp,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.token_in_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.onyc_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.seconds_elapsed, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.apr_delta, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.buffer_mint_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.reserve_mint_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.management_fee_mint_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.performance_fee_mint_amount,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.old_previous_supply, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.new_previous_supply, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.old_previous_performance_fee_high_watermark,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.new_performance_fee_high_watermark,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.current_nav, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.post_accrual_supply, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.timestamp, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct BufferAccruedEventEvent(pub BufferAccruedEvent);
impl BufferAccruedEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != BUFFER_ACCRUED_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = BufferAccruedEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&BUFFER_ACCRUED_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const BUFFER_BURNED_FOR_NAV_EVENT_EVENT_DISCM: [u8; 8] = [
    116, 208, 135, 209, 171, 149, 163, 99,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct BufferBurnedForNavEvent {
    pub burn_amount: u64,
    pub asset_adjustment_amount: u64,
    pub total_assets: u64,
    pub target_nav: u64,
}
impl BufferBurnedForNavEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let burn_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let asset_adjustment_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let total_assets: u64 = crate::borsh_de_or_default(&mut reader)?;
        let target_nav: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            burn_amount,
            asset_adjustment_amount,
            total_assets,
            target_nav,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.burn_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.asset_adjustment_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.total_assets, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.target_nav, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct BufferBurnedForNavEventEvent(pub BufferBurnedForNavEvent);
impl BufferBurnedForNavEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != BUFFER_BURNED_FOR_NAV_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = BufferBurnedForNavEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&BUFFER_BURNED_FOR_NAV_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const BUFFER_FEE_CONFIG_UPDATED_EVENT_EVENT_DISCM: [u8; 8] = [
    222, 252, 155, 30, 192, 243, 117, 240,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct BufferFeeConfigUpdatedEvent {
    pub old_management_fee_basis_points: u16,
    pub new_management_fee_basis_points: u16,
    pub old_performance_fee_basis_points: u16,
    pub new_performance_fee_basis_points: u16,
    pub old_performance_fee_high_watermark_enabled: bool,
    pub new_performance_fee_high_watermark_enabled: bool,
}
impl BufferFeeConfigUpdatedEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let old_management_fee_basis_points: u16 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let new_management_fee_basis_points: u16 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let old_performance_fee_basis_points: u16 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let new_performance_fee_basis_points: u16 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let old_performance_fee_high_watermark_enabled: bool = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let new_performance_fee_high_watermark_enabled: bool = crate::borsh_de_or_default(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self {
            old_management_fee_basis_points,
            new_management_fee_basis_points,
            old_performance_fee_basis_points,
            new_performance_fee_basis_points,
            old_performance_fee_high_watermark_enabled,
            new_performance_fee_high_watermark_enabled,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(
            &self.old_management_fee_basis_points,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.new_management_fee_basis_points,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.old_performance_fee_basis_points,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.new_performance_fee_basis_points,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.old_performance_fee_high_watermark_enabled,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.new_performance_fee_high_watermark_enabled,
            &mut writer,
        )?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct BufferFeeConfigUpdatedEventEvent(pub BufferFeeConfigUpdatedEvent);
impl BufferFeeConfigUpdatedEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != BUFFER_FEE_CONFIG_UPDATED_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = BufferFeeConfigUpdatedEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&BUFFER_FEE_CONFIG_UPDATED_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const BUFFER_GROSS_YIELD_UPDATED_EVENT_EVENT_DISCM: [u8; 8] = [
    180, 139, 51, 75, 136, 10, 63, 87,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct BufferGrossYieldUpdatedEvent {
    pub gross_yield: u64,
}
impl BufferGrossYieldUpdatedEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let gross_yield: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { gross_yield })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.gross_yield, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct BufferGrossYieldUpdatedEventEvent(pub BufferGrossYieldUpdatedEvent);
impl BufferGrossYieldUpdatedEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != BUFFER_GROSS_YIELD_UPDATED_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = BufferGrossYieldUpdatedEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&BUFFER_GROSS_YIELD_UPDATED_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const BUFFER_INITIALIZED_EVENT_EVENT_DISCM: [u8; 8] = [
    20, 3, 84, 4, 103, 231, 3, 246,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct BufferInitializedEvent {
    pub buffer_state: Pubkey,
    pub onyc_mint: Pubkey,
    pub main_offer: Pubkey,
    pub timestamp: i64,
}
impl BufferInitializedEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let buffer_state: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let onyc_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let main_offer: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let timestamp: i64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            buffer_state,
            onyc_mint,
            main_offer,
            timestamp,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.buffer_state, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.onyc_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.main_offer, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.timestamp, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct BufferInitializedEventEvent(pub BufferInitializedEvent);
impl BufferInitializedEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != BUFFER_INITIALIZED_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = BufferInitializedEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&BUFFER_INITIALIZED_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const CIRCULATING_SUPPLY_EXCLUDED_ACCOUNTS_SET_EVENT_EVENT_DISCM: [u8; 8] = [
    120, 218, 239, 105, 26, 15, 181, 28,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CirculatingSupplyExcludedAccountsSetEvent {
    pub owners: [Pubkey; 20],
    pub boss: Pubkey,
}
impl CirculatingSupplyExcludedAccountsSetEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let owners: [Pubkey; 20] = crate::borsh_de_or_default(&mut reader)?;
        let boss: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { owners, boss })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.owners, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.boss, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct CirculatingSupplyExcludedAccountsSetEventEvent(
    pub CirculatingSupplyExcludedAccountsSetEvent,
);
impl CirculatingSupplyExcludedAccountsSetEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CIRCULATING_SUPPLY_EXCLUDED_ACCOUNTS_SET_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = CirculatingSupplyExcludedAccountsSetEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CIRCULATING_SUPPLY_EXCLUDED_ACCOUNTS_SET_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const CIRCULATING_SUPPLY_EXCLUDED_BALANCE_UPDATED_EVENT_EVENT_DISCM: [u8; 8] = [
    247, 56, 105, 146, 129, 92, 134, 92,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CirculatingSupplyExcludedBalanceUpdatedEvent {
    pub amount: u64,
    pub updater: Pubkey,
    pub timestamp: i64,
    pub slot: u64,
}
impl CirculatingSupplyExcludedBalanceUpdatedEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let updater: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let timestamp: i64 = crate::borsh_de_or_default(&mut reader)?;
        let slot: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            amount,
            updater,
            timestamp,
            slot,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.updater, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.timestamp, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.slot, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct CirculatingSupplyExcludedBalanceUpdatedEventEvent(
    pub CirculatingSupplyExcludedBalanceUpdatedEvent,
);
impl CirculatingSupplyExcludedBalanceUpdatedEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CIRCULATING_SUPPLY_EXCLUDED_BALANCE_UPDATED_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = CirculatingSupplyExcludedBalanceUpdatedEvent::deserialize(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer
            .write_all(&CIRCULATING_SUPPLY_EXCLUDED_BALANCE_UPDATED_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const CONFIGURABLE_VAULT_DESTINATION_UPDATED_EVENT_EVENT_DISCM: [u8; 8] = [
    175, 21, 184, 75, 241, 98, 227, 202,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ConfigurableVaultDestinationUpdatedEvent {
    pub kind: u8,
    pub old_destination: Pubkey,
    pub new_destination: Pubkey,
}
impl ConfigurableVaultDestinationUpdatedEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let kind: u8 = crate::borsh_de_or_default(&mut reader)?;
        let old_destination: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let new_destination: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            kind,
            old_destination,
            new_destination,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.kind, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.old_destination, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.new_destination, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct ConfigurableVaultDestinationUpdatedEventEvent(
    pub ConfigurableVaultDestinationUpdatedEvent,
);
impl ConfigurableVaultDestinationUpdatedEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CONFIGURABLE_VAULT_DESTINATION_UPDATED_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = ConfigurableVaultDestinationUpdatedEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CONFIGURABLE_VAULT_DESTINATION_UPDATED_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const CONFIGURABLE_VAULT_WITHDRAWN_EVENT_EVENT_DISCM: [u8; 8] = [
    248, 90, 16, 109, 90, 155, 34, 132,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ConfigurableVaultWithdrawnEvent {
    pub kind: u8,
    pub mint: Pubkey,
    pub destination: Pubkey,
    pub amount: u64,
}
impl ConfigurableVaultWithdrawnEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let kind: u8 = crate::borsh_de_or_default(&mut reader)?;
        let mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let destination: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            kind,
            mint,
            destination,
            amount,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.kind, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.destination, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.amount, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct ConfigurableVaultWithdrawnEventEvent(pub ConfigurableVaultWithdrawnEvent);
impl ConfigurableVaultWithdrawnEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CONFIGURABLE_VAULT_WITHDRAWN_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = ConfigurableVaultWithdrawnEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CONFIGURABLE_VAULT_WITHDRAWN_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const GET_APY_EVENT_EVENT_DISCM: [u8; 8] = [235, 74, 195, 163, 16, 198, 159, 61];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetApyEvent {
    pub offer_pda: Pubkey,
    pub apy: u64,
    pub apr: u64,
    pub timestamp: u64,
}
impl GetApyEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let offer_pda: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let apy: u64 = crate::borsh_de_or_default(&mut reader)?;
        let apr: u64 = crate::borsh_de_or_default(&mut reader)?;
        let timestamp: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            offer_pda,
            apy,
            apr,
            timestamp,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.offer_pda, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.apy, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.apr, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.timestamp, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct GetApyEventEvent(pub GetApyEvent);
impl GetApyEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != GET_APY_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = GetApyEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&GET_APY_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const GET_CIRCULATING_SUPPLY_EVENT_EVENT_DISCM: [u8; 8] = [
    2, 255, 109, 150, 90, 242, 104, 206,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetCirculatingSupplyEvent {
    pub circulating_supply: u64,
    pub total_supply: u64,
    pub vault_amount: u64,
    pub timestamp: u64,
}
impl GetCirculatingSupplyEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let circulating_supply: u64 = crate::borsh_de_or_default(&mut reader)?;
        let total_supply: u64 = crate::borsh_de_or_default(&mut reader)?;
        let vault_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let timestamp: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            circulating_supply,
            total_supply,
            vault_amount,
            timestamp,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.circulating_supply, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.total_supply, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.vault_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.timestamp, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct GetCirculatingSupplyEventEvent(pub GetCirculatingSupplyEvent);
impl GetCirculatingSupplyEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != GET_CIRCULATING_SUPPLY_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = GetCirculatingSupplyEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&GET_CIRCULATING_SUPPLY_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const GET_CIRCULATING_SUPPLY_V2_EVENT_EVENT_DISCM: [u8; 8] = [
    3, 206, 54, 103, 158, 86, 35, 112,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetCirculatingSupplyV2Event {
    pub circulating_supply: u64,
    pub total_supply: u64,
    pub excluded_amount: u64,
    pub timestamp: u64,
}
impl GetCirculatingSupplyV2Event {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let circulating_supply: u64 = crate::borsh_de_or_default(&mut reader)?;
        let total_supply: u64 = crate::borsh_de_or_default(&mut reader)?;
        let excluded_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let timestamp: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            circulating_supply,
            total_supply,
            excluded_amount,
            timestamp,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.circulating_supply, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.total_supply, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.excluded_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.timestamp, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct GetCirculatingSupplyV2EventEvent(pub GetCirculatingSupplyV2Event);
impl GetCirculatingSupplyV2EventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != GET_CIRCULATING_SUPPLY_V2_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = GetCirculatingSupplyV2Event::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&GET_CIRCULATING_SUPPLY_V2_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const GET_NAV_EVENT_EVENT_DISCM: [u8; 8] = [112, 70, 141, 221, 181, 134, 99, 92];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetNavEvent {
    pub offer_pda: Pubkey,
    pub current_price: u64,
    pub timestamp: u64,
    pub next_price_change_timestamp: u64,
}
impl GetNavEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let offer_pda: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let current_price: u64 = crate::borsh_de_or_default(&mut reader)?;
        let timestamp: u64 = crate::borsh_de_or_default(&mut reader)?;
        let next_price_change_timestamp: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            offer_pda,
            current_price,
            timestamp,
            next_price_change_timestamp,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.offer_pda, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.current_price, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.timestamp, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.next_price_change_timestamp,
            &mut writer,
        )?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct GetNavEventEvent(pub GetNavEvent);
impl GetNavEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != GET_NAV_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = GetNavEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&GET_NAV_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const GET_NAV_ADJUSTMENT_EVENT_EVENT_DISCM: [u8; 8] = [
    22, 137, 159, 134, 238, 37, 111, 158,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetNavAdjustmentEvent {
    pub offer_pda: Pubkey,
    pub current_price: u64,
    pub previous_price: Option<u64>,
    pub adjustment: i64,
    pub timestamp: u64,
}
impl GetNavAdjustmentEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let offer_pda: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let current_price: u64 = crate::borsh_de_or_default(&mut reader)?;
        let previous_price: Option<u64> = crate::borsh_de_or_default(&mut reader)?;
        let adjustment: i64 = crate::borsh_de_or_default(&mut reader)?;
        let timestamp: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            offer_pda,
            current_price,
            previous_price,
            adjustment,
            timestamp,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.offer_pda, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.current_price, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.previous_price, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.adjustment, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.timestamp, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct GetNavAdjustmentEventEvent(pub GetNavAdjustmentEvent);
impl GetNavAdjustmentEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != GET_NAV_ADJUSTMENT_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = GetNavAdjustmentEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&GET_NAV_ADJUSTMENT_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const GET_TVL_EVENT_EVENT_DISCM: [u8; 8] = [12, 82, 39, 27, 40, 162, 216, 88];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetTvlEvent {
    pub offer_pda: Pubkey,
    pub tvl: u64,
    pub current_price: u64,
    pub token_supply: u64,
    pub timestamp: u64,
}
impl GetTvlEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let offer_pda: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let tvl: u64 = crate::borsh_de_or_default(&mut reader)?;
        let current_price: u64 = crate::borsh_de_or_default(&mut reader)?;
        let token_supply: u64 = crate::borsh_de_or_default(&mut reader)?;
        let timestamp: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            offer_pda,
            tvl,
            current_price,
            token_supply,
            timestamp,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.offer_pda, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.tvl, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.current_price, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_supply, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.timestamp, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct GetTvlEventEvent(pub GetTvlEvent);
impl GetTvlEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != GET_TVL_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = GetTvlEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&GET_TVL_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const KILL_SWITCH_TOGGLED_EVENT_EVENT_DISCM: [u8; 8] = [
    104, 2, 90, 20, 64, 132, 228, 122,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct KillSwitchToggledEvent {
    pub enabled: bool,
    pub signer: Pubkey,
}
impl KillSwitchToggledEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let enabled: bool = crate::borsh_de_or_default(&mut reader)?;
        let signer: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { enabled, signer })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.enabled, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.signer, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct KillSwitchToggledEventEvent(pub KillSwitchToggledEvent);
impl KillSwitchToggledEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != KILL_SWITCH_TOGGLED_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = KillSwitchToggledEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&KILL_SWITCH_TOGGLED_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const MAIN_OFFER_UPDATED_EVENT_EVENT_DISCM: [u8; 8] = [
    231, 184, 238, 159, 91, 90, 28, 198,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MainOfferUpdatedEvent {
    pub old_main_offer: Pubkey,
    pub new_main_offer: Pubkey,
}
impl MainOfferUpdatedEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let old_main_offer: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let new_main_offer: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            old_main_offer,
            new_main_offer,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.old_main_offer, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.new_main_offer, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct MainOfferUpdatedEventEvent(pub MainOfferUpdatedEvent);
impl MainOfferUpdatedEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != MAIN_OFFER_UPDATED_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = MainOfferUpdatedEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&MAIN_OFFER_UPDATED_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const MARKET_STATS_REFRESHED_EVENT_EVENT_DISCM: [u8; 8] = [
    125, 246, 174, 224, 0, 163, 111, 76,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MarketStatsRefreshedEvent {
    pub market_stats_pda: Pubkey,
    pub offer_pda: Pubkey,
    pub timestamp: i64,
    pub slot: u64,
}
impl MarketStatsRefreshedEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let market_stats_pda: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let offer_pda: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let timestamp: i64 = crate::borsh_de_or_default(&mut reader)?;
        let slot: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            market_stats_pda,
            offer_pda,
            timestamp,
            slot,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.market_stats_pda, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.offer_pda, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.timestamp, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.slot, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct MarketStatsRefreshedEventEvent(pub MarketStatsRefreshedEvent);
impl MarketStatsRefreshedEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != MARKET_STATS_REFRESHED_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = MarketStatsRefreshedEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&MARKET_STATS_REFRESHED_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const MAX_MINT_AMOUNT_CONFIGURED_EVENT_EVENT_DISCM: [u8; 8] = [
    148, 177, 167, 17, 6, 243, 15, 12,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MaxMintAmountConfiguredEvent {
    pub old_max_mint_amount: u64,
    pub new_max_mint_amount: u64,
}
impl MaxMintAmountConfiguredEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let old_max_mint_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let new_max_mint_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            old_max_mint_amount,
            new_max_mint_amount,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.old_max_mint_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.new_max_mint_amount, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct MaxMintAmountConfiguredEventEvent(pub MaxMintAmountConfiguredEvent);
impl MaxMintAmountConfiguredEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != MAX_MINT_AMOUNT_CONFIGURED_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = MaxMintAmountConfiguredEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&MAX_MINT_AMOUNT_CONFIGURED_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const MAX_SUPPLY_CONFIGURED_EVENT_EVENT_DISCM: [u8; 8] = [
    180, 54, 16, 115, 92, 70, 168, 123,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MaxSupplyConfiguredEvent {
    pub old_max_supply: u64,
    pub new_max_supply: u64,
}
impl MaxSupplyConfiguredEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let old_max_supply: u64 = crate::borsh_de_or_default(&mut reader)?;
        let new_max_supply: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            old_max_supply,
            new_max_supply,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.old_max_supply, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.new_max_supply, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct MaxSupplyConfiguredEventEvent(pub MaxSupplyConfiguredEvent);
impl MaxSupplyConfiguredEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != MAX_SUPPLY_CONFIGURED_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = MaxSupplyConfiguredEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&MAX_SUPPLY_CONFIGURED_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const MINT_AUTHORITY_TRANSFERRED_TO_BOSS_EVENT_EVENT_DISCM: [u8; 8] = [
    86, 223, 255, 189, 210, 62, 212, 151,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MintAuthorityTransferredToBossEvent {
    pub mint: Pubkey,
    pub old_authority: Pubkey,
    pub new_authority: Pubkey,
}
impl MintAuthorityTransferredToBossEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let old_authority: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let new_authority: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            mint,
            old_authority,
            new_authority,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.old_authority, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.new_authority, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct MintAuthorityTransferredToBossEventEvent(
    pub MintAuthorityTransferredToBossEvent,
);
impl MintAuthorityTransferredToBossEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != MINT_AUTHORITY_TRANSFERRED_TO_BOSS_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = MintAuthorityTransferredToBossEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&MINT_AUTHORITY_TRANSFERRED_TO_BOSS_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const MINT_AUTHORITY_TRANSFERRED_TO_PROGRAM_EVENT_EVENT_DISCM: [u8; 8] = [
    237, 15, 101, 27, 85, 70, 173, 232,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MintAuthorityTransferredToProgramEvent {
    pub mint: Pubkey,
    pub old_authority: Pubkey,
    pub new_authority: Pubkey,
}
impl MintAuthorityTransferredToProgramEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let old_authority: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let new_authority: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            mint,
            old_authority,
            new_authority,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.old_authority, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.new_authority, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct MintAuthorityTransferredToProgramEventEvent(
    pub MintAuthorityTransferredToProgramEvent,
);
impl MintAuthorityTransferredToProgramEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != MINT_AUTHORITY_TRANSFERRED_TO_PROGRAM_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = MintAuthorityTransferredToProgramEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&MINT_AUTHORITY_TRANSFERRED_TO_PROGRAM_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const O_NYC_MINT_UPDATED_EVENT_EVENT_DISCM: [u8; 8] = [
    221, 248, 176, 184, 134, 249, 29, 1,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ONycMintUpdatedEvent {
    pub old_onyc_mint: Pubkey,
    pub new_onyc_mint: Pubkey,
}
impl ONycMintUpdatedEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let old_onyc_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let new_onyc_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            old_onyc_mint,
            new_onyc_mint,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.old_onyc_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.new_onyc_mint, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct ONycMintUpdatedEventEvent(pub ONycMintUpdatedEvent);
impl ONycMintUpdatedEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != O_NYC_MINT_UPDATED_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = ONycMintUpdatedEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&O_NYC_MINT_UPDATED_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const OFFER_DISABLED_SET_EVENT_EVENT_DISCM: [u8; 8] = [
    241, 236, 239, 13, 242, 26, 158, 158,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct OfferDisabledSetEvent {
    pub offer_pda: Pubkey,
    pub disabled: bool,
    pub signer: Pubkey,
}
impl OfferDisabledSetEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let offer_pda: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let disabled: bool = crate::borsh_de_or_default(&mut reader)?;
        let signer: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            offer_pda,
            disabled,
            signer,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.offer_pda, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.disabled, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.signer, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct OfferDisabledSetEventEvent(pub OfferDisabledSetEvent);
impl OfferDisabledSetEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != OFFER_DISABLED_SET_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = OfferDisabledSetEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&OFFER_DISABLED_SET_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const OFFER_FEE_UPDATED_EVENT_EVENT_DISCM: [u8; 8] = [
    65, 77, 241, 6, 23, 133, 45, 180,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct OfferFeeUpdatedEvent {
    pub offer_pda: Pubkey,
    pub old_fee_basis_points: u16,
    pub new_fee_basis_points: u16,
    pub boss: Pubkey,
}
impl OfferFeeUpdatedEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let offer_pda: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let old_fee_basis_points: u16 = crate::borsh_de_or_default(&mut reader)?;
        let new_fee_basis_points: u16 = crate::borsh_de_or_default(&mut reader)?;
        let boss: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            offer_pda,
            old_fee_basis_points,
            new_fee_basis_points,
            boss,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.offer_pda, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.old_fee_basis_points, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.new_fee_basis_points, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.boss, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct OfferFeeUpdatedEventEvent(pub OfferFeeUpdatedEvent);
impl OfferFeeUpdatedEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != OFFER_FEE_UPDATED_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = OfferFeeUpdatedEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&OFFER_FEE_UPDATED_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const OFFER_MADE_EVENT_EVENT_DISCM: [u8; 8] = [206, 97, 61, 193, 90, 177, 83, 200];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct OfferMadeEvent {
    pub offer_pda: Pubkey,
    pub token_in_mint: Pubkey,
    pub token_out_mint: Pubkey,
    pub fee_basis_points: u16,
    pub boss: Pubkey,
    pub needs_approval: bool,
    pub allow_permissionless: bool,
}
impl OfferMadeEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let offer_pda: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let token_in_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let token_out_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let fee_basis_points: u16 = crate::borsh_de_or_default(&mut reader)?;
        let boss: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let needs_approval: bool = crate::borsh_de_or_default(&mut reader)?;
        let allow_permissionless: bool = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            offer_pda,
            token_in_mint,
            token_out_mint,
            fee_basis_points,
            boss,
            needs_approval,
            allow_permissionless,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.offer_pda, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_in_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_out_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fee_basis_points, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.boss, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.needs_approval, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.allow_permissionless, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct OfferMadeEventEvent(pub OfferMadeEvent);
impl OfferMadeEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != OFFER_MADE_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = OfferMadeEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&OFFER_MADE_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const OFFER_PERMISSIONLESS_FEE_UPDATED_EVENT_EVENT_DISCM: [u8; 8] = [
    140, 216, 176, 227, 74, 173, 69, 147,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct OfferPermissionlessFeeUpdatedEvent {
    pub offer_pda: Pubkey,
    pub old_fee_basis_points_permissionless: u16,
    pub new_fee_basis_points_permissionless: u16,
    pub boss: Pubkey,
}
impl OfferPermissionlessFeeUpdatedEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let offer_pda: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let old_fee_basis_points_permissionless: u16 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let new_fee_basis_points_permissionless: u16 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let boss: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            offer_pda,
            old_fee_basis_points_permissionless,
            new_fee_basis_points_permissionless,
            boss,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.offer_pda, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.old_fee_basis_points_permissionless,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.new_fee_basis_points_permissionless,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.boss, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct OfferPermissionlessFeeUpdatedEventEvent(
    pub OfferPermissionlessFeeUpdatedEvent,
);
impl OfferPermissionlessFeeUpdatedEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != OFFER_PERMISSIONLESS_FEE_UPDATED_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = OfferPermissionlessFeeUpdatedEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&OFFER_PERMISSIONLESS_FEE_UPDATED_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const OFFER_TAKEN_EVENT_EVENT_DISCM: [u8; 8] = [64, 121, 49, 21, 184, 132, 139, 54];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct OfferTakenEvent {
    pub offer_pda: Pubkey,
    pub token_in_amount: u64,
    pub token_out_amount: u64,
    pub fee_amount: u64,
    pub user: Pubkey,
}
impl OfferTakenEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let offer_pda: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let token_in_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let token_out_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let fee_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let user: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            offer_pda,
            token_in_amount,
            token_out_amount,
            fee_amount,
            user,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.offer_pda, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_in_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_out_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fee_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.user, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct OfferTakenEventEvent(pub OfferTakenEvent);
impl OfferTakenEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != OFFER_TAKEN_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = OfferTakenEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&OFFER_TAKEN_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const OFFER_TAKEN_PERMISSIONLESS_EVENT_EVENT_DISCM: [u8; 8] = [
    201, 45, 242, 200, 95, 48, 126, 143,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct OfferTakenPermissionlessEvent {
    pub offer_pda: Pubkey,
    pub token_in_amount: u64,
    pub token_out_amount: u64,
    pub fee_amount: u64,
    pub user: Pubkey,
}
impl OfferTakenPermissionlessEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let offer_pda: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let token_in_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let token_out_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let fee_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let user: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            offer_pda,
            token_in_amount,
            token_out_amount,
            fee_amount,
            user,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.offer_pda, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_in_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_out_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fee_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.user, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct OfferTakenPermissionlessEventEvent(pub OfferTakenPermissionlessEvent);
impl OfferTakenPermissionlessEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != OFFER_TAKEN_PERMISSIONLESS_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = OfferTakenPermissionlessEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&OFFER_TAKEN_PERMISSIONLESS_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const OFFER_VAULT_DEPOSIT_EVENT_EVENT_DISCM: [u8; 8] = [
    145, 212, 252, 95, 149, 4, 227, 27,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct OfferVaultDepositEvent {
    pub mint: Pubkey,
    pub amount: u64,
    pub depositor: Pubkey,
}
impl OfferVaultDepositEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let depositor: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { mint, amount, depositor })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.depositor, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct OfferVaultDepositEventEvent(pub OfferVaultDepositEvent);
impl OfferVaultDepositEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != OFFER_VAULT_DEPOSIT_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = OfferVaultDepositEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&OFFER_VAULT_DEPOSIT_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const OFFER_VAULT_WITHDRAW_EVENT_EVENT_DISCM: [u8; 8] = [
    29, 35, 115, 218, 32, 155, 211, 94,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct OfferVaultWithdrawEvent {
    pub mint: Pubkey,
    pub amount: u64,
    pub boss: Pubkey,
}
impl OfferVaultWithdrawEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let boss: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { mint, amount, boss })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.boss, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct OfferVaultWithdrawEventEvent(pub OfferVaultWithdrawEvent);
impl OfferVaultWithdrawEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != OFFER_VAULT_WITHDRAW_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = OfferVaultWithdrawEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&OFFER_VAULT_WITHDRAW_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const OFFER_VECTOR_ADDED_EVENT_EVENT_DISCM: [u8; 8] = [
    104, 34, 244, 250, 33, 201, 53, 103,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct OfferVectorAddedEvent {
    pub offer_pda: Pubkey,
    pub start_time: u64,
    pub base_time: u64,
    pub base_price: u64,
    pub apr: u64,
    pub price_fix_duration: u64,
}
impl OfferVectorAddedEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let offer_pda: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let start_time: u64 = crate::borsh_de_or_default(&mut reader)?;
        let base_time: u64 = crate::borsh_de_or_default(&mut reader)?;
        let base_price: u64 = crate::borsh_de_or_default(&mut reader)?;
        let apr: u64 = crate::borsh_de_or_default(&mut reader)?;
        let price_fix_duration: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            offer_pda,
            start_time,
            base_time,
            base_price,
            apr,
            price_fix_duration,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.offer_pda, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.start_time, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.base_time, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.base_price, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.apr, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.price_fix_duration, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct OfferVectorAddedEventEvent(pub OfferVectorAddedEvent);
impl OfferVectorAddedEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != OFFER_VECTOR_ADDED_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = OfferVectorAddedEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&OFFER_VECTOR_ADDED_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const OFFER_VECTOR_DELETED_EVENT_EVENT_DISCM: [u8; 8] = [
    11, 85, 222, 27, 101, 98, 160, 167,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct OfferVectorDeletedEvent {
    pub offer_pda: Pubkey,
    pub vector_start_time: u64,
}
impl OfferVectorDeletedEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let offer_pda: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let vector_start_time: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            offer_pda,
            vector_start_time,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.offer_pda, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.vector_start_time, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct OfferVectorDeletedEventEvent(pub OfferVectorDeletedEvent);
impl OfferVectorDeletedEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != OFFER_VECTOR_DELETED_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = OfferVectorDeletedEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&OFFER_VECTOR_DELETED_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const OFFER_VECTOR_EVICTED_EVENT_EVENT_DISCM: [u8; 8] = [
    52, 231, 183, 68, 181, 24, 100, 243,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct OfferVectorEvictedEvent {
    pub offer_token_in_mint: Pubkey,
    pub offer_token_out_mint: Pubkey,
    pub vector_start_time: u64,
}
impl OfferVectorEvictedEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let offer_token_in_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let offer_token_out_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let vector_start_time: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            offer_token_in_mint,
            offer_token_out_mint,
            vector_start_time,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.offer_token_in_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.offer_token_out_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.vector_start_time, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct OfferVectorEvictedEventEvent(pub OfferVectorEvictedEvent);
impl OfferVectorEvictedEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != OFFER_VECTOR_EVICTED_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = OfferVectorEvictedEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&OFFER_VECTOR_EVICTED_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const ONYC_TOKENS_MINTED_EVENT_EVENT_DISCM: [u8; 8] = [
    241, 171, 63, 134, 122, 8, 178, 120,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct OnycTokensMintedEvent {
    pub onyc_mint: Pubkey,
    pub boss: Pubkey,
    pub amount: u64,
}
impl OnycTokensMintedEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let onyc_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let boss: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { onyc_mint, boss, amount })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.onyc_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.boss, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.amount, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct OnycTokensMintedEventEvent(pub OnycTokensMintedEvent);
impl OnycTokensMintedEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != ONYC_TOKENS_MINTED_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = OnycTokensMintedEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&ONYC_TOKENS_MINTED_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const PROP_AMM_CONFIGURED_EVENT_EVENT_DISCM: [u8; 8] = [
    104, 110, 198, 241, 226, 200, 237, 41,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PropAmmConfiguredEvent {
    pub offer: Pubkey,
    pub asset_mint: Pubkey,
    pub onyc_mint: Pubkey,
    pub old_enabled: bool,
    pub new_enabled: bool,
    pub old_curve_peg_haircut_bps: u16,
    pub new_curve_peg_haircut_bps: u16,
    pub old_curve_exponent_scaled: u32,
    pub new_curve_exponent_scaled: u32,
    pub old_cadence_threshold: u32,
    pub new_cadence_threshold: u32,
    pub old_cadence_wave_scaled: u32,
    pub new_cadence_wave_scaled: u32,
    pub old_epoch_duration_seconds: i64,
    pub new_epoch_duration_seconds: i64,
    pub old_wall_sensitivity_scaled: u32,
    pub new_wall_sensitivity_scaled: u32,
    pub old_minimum_sell_haircut_onyc: u64,
    pub new_minimum_sell_haircut_onyc: u64,
}
impl PropAmmConfiguredEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let offer: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let asset_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let onyc_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let old_enabled: bool = crate::borsh_de_or_default(&mut reader)?;
        let new_enabled: bool = crate::borsh_de_or_default(&mut reader)?;
        let old_curve_peg_haircut_bps: u16 = crate::borsh_de_or_default(&mut reader)?;
        let new_curve_peg_haircut_bps: u16 = crate::borsh_de_or_default(&mut reader)?;
        let old_curve_exponent_scaled: u32 = crate::borsh_de_or_default(&mut reader)?;
        let new_curve_exponent_scaled: u32 = crate::borsh_de_or_default(&mut reader)?;
        let old_cadence_threshold: u32 = crate::borsh_de_or_default(&mut reader)?;
        let new_cadence_threshold: u32 = crate::borsh_de_or_default(&mut reader)?;
        let old_cadence_wave_scaled: u32 = crate::borsh_de_or_default(&mut reader)?;
        let new_cadence_wave_scaled: u32 = crate::borsh_de_or_default(&mut reader)?;
        let old_epoch_duration_seconds: i64 = crate::borsh_de_or_default(&mut reader)?;
        let new_epoch_duration_seconds: i64 = crate::borsh_de_or_default(&mut reader)?;
        let old_wall_sensitivity_scaled: u32 = crate::borsh_de_or_default(&mut reader)?;
        let new_wall_sensitivity_scaled: u32 = crate::borsh_de_or_default(&mut reader)?;
        let old_minimum_sell_haircut_onyc: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let new_minimum_sell_haircut_onyc: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self {
            offer,
            asset_mint,
            onyc_mint,
            old_enabled,
            new_enabled,
            old_curve_peg_haircut_bps,
            new_curve_peg_haircut_bps,
            old_curve_exponent_scaled,
            new_curve_exponent_scaled,
            old_cadence_threshold,
            new_cadence_threshold,
            old_cadence_wave_scaled,
            new_cadence_wave_scaled,
            old_epoch_duration_seconds,
            new_epoch_duration_seconds,
            old_wall_sensitivity_scaled,
            new_wall_sensitivity_scaled,
            old_minimum_sell_haircut_onyc,
            new_minimum_sell_haircut_onyc,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.offer, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.asset_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.onyc_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.old_enabled, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.new_enabled, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.old_curve_peg_haircut_bps, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.new_curve_peg_haircut_bps, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.old_curve_exponent_scaled, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.new_curve_exponent_scaled, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.old_cadence_threshold, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.new_cadence_threshold, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.old_cadence_wave_scaled, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.new_cadence_wave_scaled, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.old_epoch_duration_seconds, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.new_epoch_duration_seconds, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.old_wall_sensitivity_scaled,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.new_wall_sensitivity_scaled,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.old_minimum_sell_haircut_onyc,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.new_minimum_sell_haircut_onyc,
            &mut writer,
        )?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct PropAmmConfiguredEventEvent(pub PropAmmConfiguredEvent);
impl PropAmmConfiguredEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != PROP_AMM_CONFIGURED_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = PropAmmConfiguredEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&PROP_AMM_CONFIGURED_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const REDEMPTION_OFFER_CREATED_EVENT_EVENT_DISCM: [u8; 8] = [
    171, 25, 200, 106, 108, 123, 70, 65,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct RedemptionOfferCreatedEvent {
    pub redemption_offer_pda: Pubkey,
    pub offer: Pubkey,
    pub token_in_mint: Pubkey,
    pub token_out_mint: Pubkey,
    pub fee_basis_points: u16,
    pub fee_basis_points_prop_amm_sell: u16,
    pub vault_target_bps: u16,
}
impl RedemptionOfferCreatedEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let redemption_offer_pda: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let offer: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let token_in_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let token_out_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let fee_basis_points: u16 = crate::borsh_de_or_default(&mut reader)?;
        let fee_basis_points_prop_amm_sell: u16 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let vault_target_bps: u16 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            redemption_offer_pda,
            offer,
            token_in_mint,
            token_out_mint,
            fee_basis_points,
            fee_basis_points_prop_amm_sell,
            vault_target_bps,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.redemption_offer_pda, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.offer, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_in_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_out_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fee_basis_points, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.fee_basis_points_prop_amm_sell,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.vault_target_bps, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct RedemptionOfferCreatedEventEvent(pub RedemptionOfferCreatedEvent);
impl RedemptionOfferCreatedEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != REDEMPTION_OFFER_CREATED_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = RedemptionOfferCreatedEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&REDEMPTION_OFFER_CREATED_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const REDEMPTION_OFFER_DISABLED_SET_EVENT_EVENT_DISCM: [u8; 8] = [
    187, 148, 3, 190, 96, 151, 65, 45,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct RedemptionOfferDisabledSetEvent {
    pub redemption_offer_pda: Pubkey,
    pub disabled: bool,
    pub signer: Pubkey,
}
impl RedemptionOfferDisabledSetEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let redemption_offer_pda: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let disabled: bool = crate::borsh_de_or_default(&mut reader)?;
        let signer: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            redemption_offer_pda,
            disabled,
            signer,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.redemption_offer_pda, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.disabled, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.signer, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct RedemptionOfferDisabledSetEventEvent(pub RedemptionOfferDisabledSetEvent);
impl RedemptionOfferDisabledSetEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != REDEMPTION_OFFER_DISABLED_SET_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = RedemptionOfferDisabledSetEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&REDEMPTION_OFFER_DISABLED_SET_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const REDEMPTION_OFFER_FEE_UPDATED_EVENT_EVENT_DISCM: [u8; 8] = [
    221, 254, 77, 118, 205, 154, 166, 156,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct RedemptionOfferFeeUpdatedEvent {
    pub redemption_offer_pda: Pubkey,
    pub old_fee_basis_points: u16,
    pub new_fee_basis_points: u16,
    pub boss: Pubkey,
}
impl RedemptionOfferFeeUpdatedEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let redemption_offer_pda: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let old_fee_basis_points: u16 = crate::borsh_de_or_default(&mut reader)?;
        let new_fee_basis_points: u16 = crate::borsh_de_or_default(&mut reader)?;
        let boss: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            redemption_offer_pda,
            old_fee_basis_points,
            new_fee_basis_points,
            boss,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.redemption_offer_pda, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.old_fee_basis_points, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.new_fee_basis_points, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.boss, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct RedemptionOfferFeeUpdatedEventEvent(pub RedemptionOfferFeeUpdatedEvent);
impl RedemptionOfferFeeUpdatedEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != REDEMPTION_OFFER_FEE_UPDATED_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = RedemptionOfferFeeUpdatedEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&REDEMPTION_OFFER_FEE_UPDATED_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const REDEMPTION_OFFER_PROP_AMM_SELL_FEE_UPDATED_EVENT_EVENT_DISCM: [u8; 8] = [
    20, 70, 77, 133, 173, 84, 139, 47,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct RedemptionOfferPropAmmSellFeeUpdatedEvent {
    pub redemption_offer_pda: Pubkey,
    pub old_fee_basis_points_prop_amm_sell: u16,
    pub new_fee_basis_points_prop_amm_sell: u16,
    pub boss: Pubkey,
}
impl RedemptionOfferPropAmmSellFeeUpdatedEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let redemption_offer_pda: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let old_fee_basis_points_prop_amm_sell: u16 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let new_fee_basis_points_prop_amm_sell: u16 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let boss: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            redemption_offer_pda,
            old_fee_basis_points_prop_amm_sell,
            new_fee_basis_points_prop_amm_sell,
            boss,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.redemption_offer_pda, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.old_fee_basis_points_prop_amm_sell,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.new_fee_basis_points_prop_amm_sell,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.boss, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct RedemptionOfferPropAmmSellFeeUpdatedEventEvent(
    pub RedemptionOfferPropAmmSellFeeUpdatedEvent,
);
impl RedemptionOfferPropAmmSellFeeUpdatedEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != REDEMPTION_OFFER_PROP_AMM_SELL_FEE_UPDATED_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = RedemptionOfferPropAmmSellFeeUpdatedEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&REDEMPTION_OFFER_PROP_AMM_SELL_FEE_UPDATED_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const REDEMPTION_OFFER_VAULT_TARGET_UPDATED_EVENT_EVENT_DISCM: [u8; 8] = [
    199, 56, 132, 45, 20, 230, 171, 98,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct RedemptionOfferVaultTargetUpdatedEvent {
    pub redemption_offer_pda: Pubkey,
    pub old_vault_target_bps: u16,
    pub new_vault_target_bps: u16,
    pub boss: Pubkey,
}
impl RedemptionOfferVaultTargetUpdatedEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let redemption_offer_pda: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let old_vault_target_bps: u16 = crate::borsh_de_or_default(&mut reader)?;
        let new_vault_target_bps: u16 = crate::borsh_de_or_default(&mut reader)?;
        let boss: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            redemption_offer_pda,
            old_vault_target_bps,
            new_vault_target_bps,
            boss,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.redemption_offer_pda, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.old_vault_target_bps, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.new_vault_target_bps, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.boss, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct RedemptionOfferVaultTargetUpdatedEventEvent(
    pub RedemptionOfferVaultTargetUpdatedEvent,
);
impl RedemptionOfferVaultTargetUpdatedEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != REDEMPTION_OFFER_VAULT_TARGET_UPDATED_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = RedemptionOfferVaultTargetUpdatedEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&REDEMPTION_OFFER_VAULT_TARGET_UPDATED_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const REDEMPTION_REQUEST_CANCELLED_EVENT_EVENT_DISCM: [u8; 8] = [
    51, 146, 195, 92, 134, 230, 73, 134,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct RedemptionRequestCancelledEvent {
    pub redemption_request_pda: Pubkey,
    pub redemption_offer: Pubkey,
    pub redeemer: Pubkey,
    pub original_amount: u64,
    pub returned_amount: u64,
    pub cancelled_by: Pubkey,
}
impl RedemptionRequestCancelledEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let redemption_request_pda: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let redemption_offer: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let redeemer: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let original_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let returned_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let cancelled_by: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            redemption_request_pda,
            redemption_offer,
            redeemer,
            original_amount,
            returned_amount,
            cancelled_by,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.redemption_request_pda, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.redemption_offer, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.redeemer, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.original_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.returned_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.cancelled_by, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct RedemptionRequestCancelledEventEvent(pub RedemptionRequestCancelledEvent);
impl RedemptionRequestCancelledEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != REDEMPTION_REQUEST_CANCELLED_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = RedemptionRequestCancelledEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&REDEMPTION_REQUEST_CANCELLED_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const REDEMPTION_REQUEST_CREATED_EVENT_EVENT_DISCM: [u8; 8] = [
    30, 61, 76, 2, 36, 82, 84, 201,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct RedemptionRequestCreatedEvent {
    pub redemption_request_pda: Pubkey,
    pub redemption_offer_pda: Pubkey,
    pub redeemer: Pubkey,
    pub amount: u64,
    pub request_id: String,
}
impl RedemptionRequestCreatedEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let redemption_request_pda: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let redemption_offer_pda: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let redeemer: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let request_id: String = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            redemption_request_pda,
            redemption_offer_pda,
            redeemer,
            amount,
            request_id,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.redemption_request_pda, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.redemption_offer_pda, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.redeemer, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.request_id, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct RedemptionRequestCreatedEventEvent(pub RedemptionRequestCreatedEvent);
impl RedemptionRequestCreatedEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != REDEMPTION_REQUEST_CREATED_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = RedemptionRequestCreatedEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&REDEMPTION_REQUEST_CREATED_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const REDEMPTION_REQUEST_FULFILLED_EVENT_EVENT_DISCM: [u8; 8] = [
    154, 40, 115, 4, 42, 232, 47, 230,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct RedemptionRequestFulfilledEvent {
    pub redemption_request_pda: Pubkey,
    pub redemption_offer_pda: Pubkey,
    pub redeemer: Pubkey,
    pub token_in_net_amount: u64,
    pub token_in_fee_amount: u64,
    pub token_out_amount: u64,
    pub current_price: u64,
    pub fulfilled_amount: u64,
    pub total_fulfilled_amount: u64,
    pub is_fully_fulfilled: bool,
}
impl RedemptionRequestFulfilledEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let redemption_request_pda: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let redemption_offer_pda: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let redeemer: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let token_in_net_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let token_in_fee_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let token_out_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let current_price: u64 = crate::borsh_de_or_default(&mut reader)?;
        let fulfilled_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let total_fulfilled_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let is_fully_fulfilled: bool = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            redemption_request_pda,
            redemption_offer_pda,
            redeemer,
            token_in_net_amount,
            token_in_fee_amount,
            token_out_amount,
            current_price,
            fulfilled_amount,
            total_fulfilled_amount,
            is_fully_fulfilled,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.redemption_request_pda, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.redemption_offer_pda, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.redeemer, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_in_net_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_in_fee_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_out_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.current_price, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fulfilled_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.total_fulfilled_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.is_fully_fulfilled, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct RedemptionRequestFulfilledEventEvent(pub RedemptionRequestFulfilledEvent);
impl RedemptionRequestFulfilledEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != REDEMPTION_REQUEST_FULFILLED_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = RedemptionRequestFulfilledEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&REDEMPTION_REQUEST_FULFILLED_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const REDEMPTION_VAULT_DEPOSIT_EVENT_EVENT_DISCM: [u8; 8] = [
    229, 187, 130, 152, 236, 139, 239, 108,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct RedemptionVaultDepositEvent {
    pub mint: Pubkey,
    pub amount: u64,
    pub depositor: Pubkey,
}
impl RedemptionVaultDepositEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let depositor: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { mint, amount, depositor })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.depositor, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct RedemptionVaultDepositEventEvent(pub RedemptionVaultDepositEvent);
impl RedemptionVaultDepositEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != REDEMPTION_VAULT_DEPOSIT_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = RedemptionVaultDepositEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&REDEMPTION_VAULT_DEPOSIT_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const REDEMPTION_VAULT_WITHDRAW_EVENT_EVENT_DISCM: [u8; 8] = [
    255, 92, 199, 233, 33, 6, 21, 78,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct RedemptionVaultWithdrawEvent {
    pub mint: Pubkey,
    pub amount: u64,
    pub boss: Pubkey,
}
impl RedemptionVaultWithdrawEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let boss: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { mint, amount, boss })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.boss, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct RedemptionVaultWithdrawEventEvent(pub RedemptionVaultWithdrawEvent);
impl RedemptionVaultWithdrawEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != REDEMPTION_VAULT_WITHDRAW_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = RedemptionVaultWithdrawEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&REDEMPTION_VAULT_WITHDRAW_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const RESERVE_VAULT_DEPOSITED_EVENT_EVENT_DISCM: [u8; 8] = [
    82, 100, 155, 125, 96, 52, 235, 0,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ReserveVaultDepositedEvent {
    pub amount: u64,
    pub mint: Pubkey,
    pub depositor: Pubkey,
}
impl ReserveVaultDepositedEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let depositor: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { amount, mint, depositor })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.depositor, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct ReserveVaultDepositedEventEvent(pub ReserveVaultDepositedEvent);
impl ReserveVaultDepositedEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != RESERVE_VAULT_DEPOSITED_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = ReserveVaultDepositedEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&RESERVE_VAULT_DEPOSITED_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const RESERVE_VAULT_WITHDRAWN_EVENT_EVENT_DISCM: [u8; 8] = [
    145, 196, 225, 104, 251, 144, 66, 232,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ReserveVaultWithdrawnEvent {
    pub amount: u64,
    pub mint: Pubkey,
    pub boss: Pubkey,
}
impl ReserveVaultWithdrawnEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let boss: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { amount, mint, boss })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.boss, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct ReserveVaultWithdrawnEventEvent(pub ReserveVaultWithdrawnEvent);
impl ReserveVaultWithdrawnEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != RESERVE_VAULT_WITHDRAWN_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = ReserveVaultWithdrawnEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&RESERVE_VAULT_WITHDRAWN_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const STATE_CLOSED_EVENT_EVENT_DISCM: [u8; 8] = [
    205, 52, 85, 250, 177, 119, 155, 198,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct StateClosedEvent {
    pub state_pda: Pubkey,
    pub boss: Pubkey,
}
impl StateClosedEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let state_pda: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let boss: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { state_pda, boss })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.state_pda, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.boss, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct StateClosedEventEvent(pub StateClosedEvent);
impl StateClosedEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != STATE_CLOSED_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = StateClosedEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&STATE_CLOSED_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const WORKER_UPDATED_EVENT_EVENT_DISCM: [u8; 8] = [
    190, 3, 208, 81, 65, 111, 249, 100,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WorkerUpdatedEvent {
    pub old_worker: Pubkey,
    pub new_worker: Pubkey,
}
impl WorkerUpdatedEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let old_worker: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let new_worker: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { old_worker, new_worker })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.old_worker, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.new_worker, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct WorkerUpdatedEventEvent(pub WorkerUpdatedEvent);
impl WorkerUpdatedEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != WORKER_UPDATED_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = WorkerUpdatedEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&WORKER_UPDATED_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
