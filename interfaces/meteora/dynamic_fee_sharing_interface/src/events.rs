use solana_pubkey::Pubkey;
#[allow(unused_imports)]
use crate::*;
pub const EVT_CLAIM_FEE_EVENT_DISCM: [u8; 8] = [6, 36, 88, 232, 53, 193, 253, 98];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct EvtClaimFee {
    pub fee_vault: Pubkey,
    pub user: Pubkey,
    pub index: u8,
    pub claimed_fee: u64,
}
impl EvtClaimFee {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let fee_vault: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let user: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let index: u8 = crate::borsh_de_or_default(&mut reader)?;
        let claimed_fee: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            fee_vault,
            user,
            index,
            claimed_fee,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.fee_vault, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.user, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.index, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.claimed_fee, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct EvtClaimFeeEvent(pub EvtClaimFee);
impl EvtClaimFeeEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != EVT_CLAIM_FEE_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = EvtClaimFee::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&EVT_CLAIM_FEE_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const EVT_FUND_FEE_EVENT_DISCM: [u8; 8] = [15, 14, 233, 140, 19, 195, 163, 7];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct EvtFundFee {
    pub source_program: Pubkey,
    pub fee_vault: Pubkey,
    pub funded_amount: u64,
    pub fee_per_share: u128,
    pub payload: Vec<u8>,
}
impl EvtFundFee {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let source_program: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let fee_vault: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let funded_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let fee_per_share: u128 = crate::borsh_de_or_default(&mut reader)?;
        let payload: Vec<u8> = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            source_program,
            fee_vault,
            funded_amount,
            fee_per_share,
            payload,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.source_program, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fee_vault, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.funded_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fee_per_share, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.payload, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct EvtFundFeeEvent(pub EvtFundFee);
impl EvtFundFeeEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != EVT_FUND_FEE_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = EvtFundFee::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&EVT_FUND_FEE_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const EVT_INITIALIZE_FEE_VAULT_EVENT_DISCM: [u8; 8] = [
    42, 203, 38, 10, 38, 178, 238, 77,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct EvtInitializeFeeVault {
    pub fee_vault: Pubkey,
    pub token_mint: Pubkey,
    pub owner: Pubkey,
    pub base: Pubkey,
    pub params: InitializeFeeVaultParameters,
}
impl EvtInitializeFeeVault {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let fee_vault: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let token_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let owner: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let base: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let params = if reader.is_empty() {
            Default::default()
        } else {
            <InitializeFeeVaultParameters>::deserialize(&mut reader)?
        };
        *__buf = reader;
        Ok(Self {
            fee_vault,
            token_mint,
            owner,
            base,
            params,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.fee_vault, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.owner, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.base, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.params, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct EvtInitializeFeeVaultEvent(pub EvtInitializeFeeVault);
impl EvtInitializeFeeVaultEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != EVT_INITIALIZE_FEE_VAULT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = EvtInitializeFeeVault::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&EVT_INITIALIZE_FEE_VAULT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
