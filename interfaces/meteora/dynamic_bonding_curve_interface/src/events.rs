use solana_pubkey::Pubkey;
#[allow(unused_imports)]
use crate::*;
pub const EVT_CLAIM_CREATOR_TRADING_FEE_EVENT_DISCM: [u8; 8] = [
    154, 228, 215, 202, 133, 155, 214, 138,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct EvtClaimCreatorTradingFee {
    pub pool: Pubkey,
    pub token_base_amount: u64,
    pub token_quote_amount: u64,
}
impl EvtClaimCreatorTradingFee {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let pool: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let token_base_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let token_quote_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            pool,
            token_base_amount,
            token_quote_amount,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.pool, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_base_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_quote_amount, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct EvtClaimCreatorTradingFeeEvent(pub EvtClaimCreatorTradingFee);
impl EvtClaimCreatorTradingFeeEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != EVT_CLAIM_CREATOR_TRADING_FEE_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = EvtClaimCreatorTradingFee::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&EVT_CLAIM_CREATOR_TRADING_FEE_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const EVT_CLAIM_POOL_CREATION_FEE_EVENT_DISCM: [u8; 8] = [
    149, 111, 149, 44, 136, 64, 175, 62,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct EvtClaimPoolCreationFee {
    pub pool: Pubkey,
    pub receiver: Pubkey,
    pub creation_fee: u64,
}
impl EvtClaimPoolCreationFee {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let pool: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let receiver: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let creation_fee: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            pool,
            receiver,
            creation_fee,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.pool, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.receiver, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.creation_fee, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct EvtClaimPoolCreationFeeEvent(pub EvtClaimPoolCreationFee);
impl EvtClaimPoolCreationFeeEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != EVT_CLAIM_POOL_CREATION_FEE_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = EvtClaimPoolCreationFee::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&EVT_CLAIM_POOL_CREATION_FEE_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const EVT_CLAIM_PROTOCOL_FEE2_EVENT_DISCM: [u8; 8] = [
    187, 133, 66, 9, 205, 161, 84, 13,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct EvtClaimProtocolFee2 {
    pub pool: Pubkey,
    pub receiver_token_account: Pubkey,
    pub token_mint: Pubkey,
    pub amount: u64,
}
impl EvtClaimProtocolFee2 {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let pool: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let receiver_token_account: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let token_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            pool,
            receiver_token_account,
            token_mint,
            amount,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.pool, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.receiver_token_account, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.amount, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct EvtClaimProtocolFee2Event(pub EvtClaimProtocolFee2);
impl EvtClaimProtocolFee2Event {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != EVT_CLAIM_PROTOCOL_FEE2_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = EvtClaimProtocolFee2::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&EVT_CLAIM_PROTOCOL_FEE2_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const EVT_CLAIM_TRADING_FEE_EVENT_DISCM: [u8; 8] = [
    26, 83, 117, 240, 92, 202, 112, 254,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct EvtClaimTradingFee {
    pub pool: Pubkey,
    pub token_base_amount: u64,
    pub token_quote_amount: u64,
}
impl EvtClaimTradingFee {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let pool: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let token_base_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let token_quote_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            pool,
            token_base_amount,
            token_quote_amount,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.pool, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_base_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_quote_amount, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct EvtClaimTradingFeeEvent(pub EvtClaimTradingFee);
impl EvtClaimTradingFeeEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != EVT_CLAIM_TRADING_FEE_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = EvtClaimTradingFee::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&EVT_CLAIM_TRADING_FEE_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const EVT_CLOSE_CLAIM_FEE_OPERATOR_EVENT_DISCM: [u8; 8] = [
    111, 39, 37, 55, 110, 216, 194, 23,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct EvtCloseClaimFeeOperator {
    pub claim_fee_operator: Pubkey,
    pub operator: Pubkey,
}
impl EvtCloseClaimFeeOperator {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let claim_fee_operator: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let operator: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            claim_fee_operator,
            operator,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.claim_fee_operator, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.operator, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct EvtCloseClaimFeeOperatorEvent(pub EvtCloseClaimFeeOperator);
impl EvtCloseClaimFeeOperatorEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != EVT_CLOSE_CLAIM_FEE_OPERATOR_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = EvtCloseClaimFeeOperator::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&EVT_CLOSE_CLAIM_FEE_OPERATOR_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const EVT_CLOSE_TOKEN_BADGE_EVENT_DISCM: [u8; 8] = [
    60, 217, 34, 82, 183, 10, 89, 188,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct EvtCloseTokenBadge {
    pub token_mint: Pubkey,
}
impl EvtCloseTokenBadge {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let token_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { token_mint })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.token_mint, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct EvtCloseTokenBadgeEvent(pub EvtCloseTokenBadge);
impl EvtCloseTokenBadgeEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != EVT_CLOSE_TOKEN_BADGE_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = EvtCloseTokenBadge::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&EVT_CLOSE_TOKEN_BADGE_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const EVT_CREATE_CLAIM_FEE_OPERATOR_EVENT_DISCM: [u8; 8] = [
    21, 6, 153, 120, 68, 116, 28, 177,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct EvtCreateClaimFeeOperator {
    pub operator: Pubkey,
}
impl EvtCreateClaimFeeOperator {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let operator: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { operator })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.operator, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct EvtCreateClaimFeeOperatorEvent(pub EvtCreateClaimFeeOperator);
impl EvtCreateClaimFeeOperatorEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != EVT_CREATE_CLAIM_FEE_OPERATOR_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = EvtCreateClaimFeeOperator::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&EVT_CREATE_CLAIM_FEE_OPERATOR_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const EVT_CREATE_CONFIG_EVENT_DISCM: [u8; 8] = [
    131, 207, 180, 174, 180, 73, 165, 54,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct EvtCreateConfig {
    pub config: Pubkey,
    pub quote_mint: Pubkey,
    pub fee_claimer: Pubkey,
    pub owner: Pubkey,
    pub pool_fees: PoolFeeParameters,
    pub collect_fee_mode: u8,
    pub migration_option: u8,
    pub activation_type: u8,
    pub token_decimal: u8,
    pub token_type: u8,
    pub partner_permanent_locked_liquidity_percentage: u8,
    pub partner_liquidity_percentage: u8,
    pub creator_permanent_locked_liquidity_percentage: u8,
    pub creator_liquidity_percentage: u8,
    pub swap_base_amount: u64,
    pub migration_quote_threshold: u64,
    pub migration_base_amount: u64,
    pub sqrt_start_price: u128,
    pub locked_vesting: LockedVestingParams,
    pub migration_fee_option: u8,
    pub fixed_token_supply_flag: u8,
    pub pre_migration_token_supply: u64,
    pub post_migration_token_supply: u64,
    pub curve: Vec<LiquidityDistributionParameters>,
}
impl EvtCreateConfig {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let config: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let quote_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let fee_claimer: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let owner: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let pool_fees = if reader.is_empty() {
            Default::default()
        } else {
            <PoolFeeParameters>::deserialize(&mut reader)?
        };
        let collect_fee_mode: u8 = crate::borsh_de_or_default(&mut reader)?;
        let migration_option: u8 = crate::borsh_de_or_default(&mut reader)?;
        let activation_type: u8 = crate::borsh_de_or_default(&mut reader)?;
        let token_decimal: u8 = crate::borsh_de_or_default(&mut reader)?;
        let token_type: u8 = crate::borsh_de_or_default(&mut reader)?;
        let partner_permanent_locked_liquidity_percentage: u8 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let partner_liquidity_percentage: u8 = crate::borsh_de_or_default(&mut reader)?;
        let creator_permanent_locked_liquidity_percentage: u8 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let creator_liquidity_percentage: u8 = crate::borsh_de_or_default(&mut reader)?;
        let swap_base_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let migration_quote_threshold: u64 = crate::borsh_de_or_default(&mut reader)?;
        let migration_base_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let sqrt_start_price: u128 = crate::borsh_de_or_default(&mut reader)?;
        let locked_vesting = if reader.is_empty() {
            Default::default()
        } else {
            <LockedVestingParams>::deserialize(&mut reader)?
        };
        let migration_fee_option: u8 = crate::borsh_de_or_default(&mut reader)?;
        let fixed_token_supply_flag: u8 = crate::borsh_de_or_default(&mut reader)?;
        let pre_migration_token_supply: u64 = crate::borsh_de_or_default(&mut reader)?;
        let post_migration_token_supply: u64 = crate::borsh_de_or_default(&mut reader)?;
        let curve: Vec<LiquidityDistributionParameters> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self {
            config,
            quote_mint,
            fee_claimer,
            owner,
            pool_fees,
            collect_fee_mode,
            migration_option,
            activation_type,
            token_decimal,
            token_type,
            partner_permanent_locked_liquidity_percentage,
            partner_liquidity_percentage,
            creator_permanent_locked_liquidity_percentage,
            creator_liquidity_percentage,
            swap_base_amount,
            migration_quote_threshold,
            migration_base_amount,
            sqrt_start_price,
            locked_vesting,
            migration_fee_option,
            fixed_token_supply_flag,
            pre_migration_token_supply,
            post_migration_token_supply,
            curve,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.config, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.quote_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fee_claimer, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.owner, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.pool_fees, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.collect_fee_mode, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.migration_option, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.activation_type, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_decimal, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_type, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.partner_permanent_locked_liquidity_percentage,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.partner_liquidity_percentage,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.creator_permanent_locked_liquidity_percentage,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.creator_liquidity_percentage,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.swap_base_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.migration_quote_threshold, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.migration_base_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.sqrt_start_price, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.locked_vesting, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.migration_fee_option, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fixed_token_supply_flag, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.pre_migration_token_supply, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.post_migration_token_supply,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.curve, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct EvtCreateConfigEvent(pub EvtCreateConfig);
impl EvtCreateConfigEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != EVT_CREATE_CONFIG_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = EvtCreateConfig::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&EVT_CREATE_CONFIG_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const EVT_CREATE_CONFIG_V2_EVENT_DISCM: [u8; 8] = [
    163, 74, 66, 187, 119, 195, 26, 144,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct EvtCreateConfigV2 {
    pub config: Pubkey,
    pub quote_mint: Pubkey,
    pub fee_claimer: Pubkey,
    pub leftover_receiver: Pubkey,
    pub config_parameters: ConfigParameters,
}
impl EvtCreateConfigV2 {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let config: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let quote_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let fee_claimer: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let leftover_receiver: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let config_parameters = if reader.is_empty() {
            Default::default()
        } else {
            <ConfigParameters>::deserialize(&mut reader)?
        };
        *__buf = reader;
        Ok(Self {
            config,
            quote_mint,
            fee_claimer,
            leftover_receiver,
            config_parameters,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.config, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.quote_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fee_claimer, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.leftover_receiver, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.config_parameters, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct EvtCreateConfigV2Event(pub EvtCreateConfigV2);
impl EvtCreateConfigV2Event {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != EVT_CREATE_CONFIG_V2_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = EvtCreateConfigV2::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&EVT_CREATE_CONFIG_V2_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const EVT_CREATE_CONFIG_V2_WITH_TRANSFER_HOOK_EVENT_DISCM: [u8; 8] = [
    182, 81, 135, 4, 39, 46, 132, 253,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct EvtCreateConfigV2WithTransferHook {
    pub config: Pubkey,
    pub quote_mint: Pubkey,
    pub fee_claimer: Pubkey,
    pub leftover_receiver: Pubkey,
    pub config_parameters: ConfigParameters,
    pub transfer_hook_program: Pubkey,
}
impl EvtCreateConfigV2WithTransferHook {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let config: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let quote_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let fee_claimer: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let leftover_receiver: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let config_parameters = if reader.is_empty() {
            Default::default()
        } else {
            <ConfigParameters>::deserialize(&mut reader)?
        };
        let transfer_hook_program: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            config,
            quote_mint,
            fee_claimer,
            leftover_receiver,
            config_parameters,
            transfer_hook_program,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.config, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.quote_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fee_claimer, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.leftover_receiver, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.config_parameters, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.transfer_hook_program, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct EvtCreateConfigV2WithTransferHookEvent(pub EvtCreateConfigV2WithTransferHook);
impl EvtCreateConfigV2WithTransferHookEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != EVT_CREATE_CONFIG_V2_WITH_TRANSFER_HOOK_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = EvtCreateConfigV2WithTransferHook::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&EVT_CREATE_CONFIG_V2_WITH_TRANSFER_HOOK_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const EVT_CREATE_METEORA_MIGRATION_METADATA_EVENT_DISCM: [u8; 8] = [
    99, 167, 133, 63, 214, 143, 175, 139,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct EvtCreateMeteoraMigrationMetadata {
    pub virtual_pool: Pubkey,
}
impl EvtCreateMeteoraMigrationMetadata {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let virtual_pool: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { virtual_pool })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.virtual_pool, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct EvtCreateMeteoraMigrationMetadataEvent(pub EvtCreateMeteoraMigrationMetadata);
impl EvtCreateMeteoraMigrationMetadataEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != EVT_CREATE_METEORA_MIGRATION_METADATA_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = EvtCreateMeteoraMigrationMetadata::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&EVT_CREATE_METEORA_MIGRATION_METADATA_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const EVT_CREATE_TOKEN_BADGE_EVENT_DISCM: [u8; 8] = [
    141, 120, 134, 116, 34, 28, 114, 160,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct EvtCreateTokenBadge {
    pub token_mint: Pubkey,
}
impl EvtCreateTokenBadge {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let token_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { token_mint })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.token_mint, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct EvtCreateTokenBadgeEvent(pub EvtCreateTokenBadge);
impl EvtCreateTokenBadgeEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != EVT_CREATE_TOKEN_BADGE_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = EvtCreateTokenBadge::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&EVT_CREATE_TOKEN_BADGE_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const EVT_CREATOR_WITHDRAW_SURPLUS_EVENT_DISCM: [u8; 8] = [
    152, 73, 21, 15, 66, 87, 53, 157,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct EvtCreatorWithdrawSurplus {
    pub pool: Pubkey,
    pub surplus_amount: u64,
}
impl EvtCreatorWithdrawSurplus {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let pool: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let surplus_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { pool, surplus_amount })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.pool, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.surplus_amount, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct EvtCreatorWithdrawSurplusEvent(pub EvtCreatorWithdrawSurplus);
impl EvtCreatorWithdrawSurplusEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != EVT_CREATOR_WITHDRAW_SURPLUS_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = EvtCreatorWithdrawSurplus::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&EVT_CREATOR_WITHDRAW_SURPLUS_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const EVT_CURVE_COMPLETE_EVENT_DISCM: [u8; 8] = [229, 231, 86, 84, 156, 134, 75, 24];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct EvtCurveComplete {
    pub pool: Pubkey,
    pub config: Pubkey,
    pub base_reserve: u64,
    pub quote_reserve: u64,
}
impl EvtCurveComplete {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let pool: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let config: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let base_reserve: u64 = crate::borsh_de_or_default(&mut reader)?;
        let quote_reserve: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            pool,
            config,
            base_reserve,
            quote_reserve,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.pool, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.config, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.base_reserve, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.quote_reserve, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct EvtCurveCompleteEvent(pub EvtCurveComplete);
impl EvtCurveCompleteEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != EVT_CURVE_COMPLETE_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = EvtCurveComplete::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&EVT_CURVE_COMPLETE_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const EVT_CURVE_COMPLETE_WITH_TRANSFER_HOOK_EVENT_DISCM: [u8; 8] = [
    59, 47, 109, 205, 13, 31, 44, 159,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct EvtCurveCompleteWithTransferHook {
    pub pool: Pubkey,
    pub config: Pubkey,
    pub base_reserve: u64,
    pub quote_reserve: u64,
}
impl EvtCurveCompleteWithTransferHook {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let pool: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let config: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let base_reserve: u64 = crate::borsh_de_or_default(&mut reader)?;
        let quote_reserve: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            pool,
            config,
            base_reserve,
            quote_reserve,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.pool, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.config, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.base_reserve, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.quote_reserve, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct EvtCurveCompleteWithTransferHookEvent(pub EvtCurveCompleteWithTransferHook);
impl EvtCurveCompleteWithTransferHookEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != EVT_CURVE_COMPLETE_WITH_TRANSFER_HOOK_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = EvtCurveCompleteWithTransferHook::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&EVT_CURVE_COMPLETE_WITH_TRANSFER_HOOK_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const EVT_INITIALIZE_POOL_EVENT_DISCM: [u8; 8] = [
    228, 50, 246, 85, 203, 66, 134, 37,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct EvtInitializePool {
    pub pool: Pubkey,
    pub config: Pubkey,
    pub creator: Pubkey,
    pub base_mint: Pubkey,
    pub pool_type: u8,
    pub activation_point: u64,
}
impl EvtInitializePool {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let pool: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let config: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let creator: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let base_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let pool_type: u8 = crate::borsh_de_or_default(&mut reader)?;
        let activation_point: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            pool,
            config,
            creator,
            base_mint,
            pool_type,
            activation_point,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.pool, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.config, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.creator, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.base_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.pool_type, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.activation_point, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct EvtInitializePoolEvent(pub EvtInitializePool);
impl EvtInitializePoolEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != EVT_INITIALIZE_POOL_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = EvtInitializePool::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&EVT_INITIALIZE_POOL_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const EVT_INITIALIZE_POOL_WITH_TRANSFER_HOOK_EVENT_DISCM: [u8; 8] = [
    213, 137, 164, 53, 193, 74, 15, 110,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct EvtInitializePoolWithTransferHook {
    pub pool: Pubkey,
    pub config: Pubkey,
    pub creator: Pubkey,
    pub base_mint: Pubkey,
    pub pool_type: u8,
    pub activation_point: u64,
}
impl EvtInitializePoolWithTransferHook {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let pool: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let config: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let creator: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let base_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let pool_type: u8 = crate::borsh_de_or_default(&mut reader)?;
        let activation_point: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            pool,
            config,
            creator,
            base_mint,
            pool_type,
            activation_point,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.pool, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.config, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.creator, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.base_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.pool_type, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.activation_point, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct EvtInitializePoolWithTransferHookEvent(pub EvtInitializePoolWithTransferHook);
impl EvtInitializePoolWithTransferHookEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != EVT_INITIALIZE_POOL_WITH_TRANSFER_HOOK_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = EvtInitializePoolWithTransferHook::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&EVT_INITIALIZE_POOL_WITH_TRANSFER_HOOK_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const EVT_PARTNER_CLAIM_POOL_CREATION_FEE_EVENT_DISCM: [u8; 8] = [
    174, 223, 44, 150, 145, 98, 89, 195,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct EvtPartnerClaimPoolCreationFee {
    pub pool: Pubkey,
    pub partner: Pubkey,
    pub creation_fee: u64,
    pub fee_receiver: Pubkey,
}
impl EvtPartnerClaimPoolCreationFee {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let pool: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let partner: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let creation_fee: u64 = crate::borsh_de_or_default(&mut reader)?;
        let fee_receiver: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            pool,
            partner,
            creation_fee,
            fee_receiver,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.pool, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.partner, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.creation_fee, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fee_receiver, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct EvtPartnerClaimPoolCreationFeeEvent(pub EvtPartnerClaimPoolCreationFee);
impl EvtPartnerClaimPoolCreationFeeEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != EVT_PARTNER_CLAIM_POOL_CREATION_FEE_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = EvtPartnerClaimPoolCreationFee::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&EVT_PARTNER_CLAIM_POOL_CREATION_FEE_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const EVT_PARTNER_METADATA_EVENT_DISCM: [u8; 8] = [200, 127, 6, 55, 13, 32, 8, 150];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct EvtPartnerMetadata {
    pub partner_metadata: Pubkey,
    pub fee_claimer: Pubkey,
}
impl EvtPartnerMetadata {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let partner_metadata: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let fee_claimer: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            partner_metadata,
            fee_claimer,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.partner_metadata, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fee_claimer, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct EvtPartnerMetadataEvent(pub EvtPartnerMetadata);
impl EvtPartnerMetadataEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != EVT_PARTNER_METADATA_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = EvtPartnerMetadata::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&EVT_PARTNER_METADATA_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const EVT_PARTNER_WITHDRAW_SURPLUS_EVENT_DISCM: [u8; 8] = [
    195, 56, 152, 9, 232, 72, 35, 22,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct EvtPartnerWithdrawSurplus {
    pub pool: Pubkey,
    pub surplus_amount: u64,
}
impl EvtPartnerWithdrawSurplus {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let pool: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let surplus_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { pool, surplus_amount })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.pool, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.surplus_amount, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct EvtPartnerWithdrawSurplusEvent(pub EvtPartnerWithdrawSurplus);
impl EvtPartnerWithdrawSurplusEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != EVT_PARTNER_WITHDRAW_SURPLUS_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = EvtPartnerWithdrawSurplus::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&EVT_PARTNER_WITHDRAW_SURPLUS_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const EVT_SWAP_EVENT_DISCM: [u8; 8] = [27, 60, 21, 213, 138, 170, 187, 147];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct EvtSwap {
    pub pool: Pubkey,
    pub config: Pubkey,
    pub trade_direction: u8,
    pub has_referral: bool,
    pub params: SwapParameters,
    pub swap_result: SwapResult,
    pub amount_in: u64,
    pub current_timestamp: u64,
}
impl EvtSwap {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let pool: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let config: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let trade_direction: u8 = crate::borsh_de_or_default(&mut reader)?;
        let has_referral: bool = crate::borsh_de_or_default(&mut reader)?;
        let params = if reader.is_empty() {
            Default::default()
        } else {
            <SwapParameters>::deserialize(&mut reader)?
        };
        let swap_result = if reader.is_empty() {
            Default::default()
        } else {
            <SwapResult>::deserialize(&mut reader)?
        };
        let amount_in: u64 = crate::borsh_de_or_default(&mut reader)?;
        let current_timestamp: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            pool,
            config,
            trade_direction,
            has_referral,
            params,
            swap_result,
            amount_in,
            current_timestamp,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.pool, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.config, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.trade_direction, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.has_referral, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.params, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.swap_result, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.amount_in, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.current_timestamp, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct EvtSwapEvent(pub EvtSwap);
impl EvtSwapEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != EVT_SWAP_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = EvtSwap::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&EVT_SWAP_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const EVT_SWAP2_EVENT_DISCM: [u8; 8] = [189, 66, 51, 168, 38, 80, 117, 153];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct EvtSwap2 {
    pub pool: Pubkey,
    pub config: Pubkey,
    pub trade_direction: u8,
    pub has_referral: bool,
    pub swap_parameters: SwapParameters2,
    pub swap_result: SwapResult2,
    pub quote_reserve_amount: u64,
    pub migration_threshold: u64,
    pub current_timestamp: u64,
}
impl EvtSwap2 {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let pool: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let config: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let trade_direction: u8 = crate::borsh_de_or_default(&mut reader)?;
        let has_referral: bool = crate::borsh_de_or_default(&mut reader)?;
        let swap_parameters = if reader.is_empty() {
            Default::default()
        } else {
            <SwapParameters2>::deserialize(&mut reader)?
        };
        let swap_result = if reader.is_empty() {
            Default::default()
        } else {
            <SwapResult2>::deserialize(&mut reader)?
        };
        let quote_reserve_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let migration_threshold: u64 = crate::borsh_de_or_default(&mut reader)?;
        let current_timestamp: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            pool,
            config,
            trade_direction,
            has_referral,
            swap_parameters,
            swap_result,
            quote_reserve_amount,
            migration_threshold,
            current_timestamp,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.pool, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.config, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.trade_direction, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.has_referral, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.swap_parameters, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.swap_result, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.quote_reserve_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.migration_threshold, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.current_timestamp, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct EvtSwap2Event(pub EvtSwap2);
impl EvtSwap2Event {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != EVT_SWAP2_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = EvtSwap2::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&EVT_SWAP2_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const EVT_SWAP2_WITH_TRANSFER_HOOK_EVENT_DISCM: [u8; 8] = [
    134, 59, 168, 120, 94, 51, 114, 231,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct EvtSwap2WithTransferHook {
    pub pool: Pubkey,
    pub config: Pubkey,
    pub trade_direction: u8,
    pub has_referral: bool,
    pub swap_parameters: SwapParameters2,
    pub swap_result: SwapResult2,
    pub quote_reserve_amount: u64,
    pub migration_threshold: u64,
    pub current_timestamp: u64,
}
impl EvtSwap2WithTransferHook {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let pool: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let config: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let trade_direction: u8 = crate::borsh_de_or_default(&mut reader)?;
        let has_referral: bool = crate::borsh_de_or_default(&mut reader)?;
        let swap_parameters = if reader.is_empty() {
            Default::default()
        } else {
            <SwapParameters2>::deserialize(&mut reader)?
        };
        let swap_result = if reader.is_empty() {
            Default::default()
        } else {
            <SwapResult2>::deserialize(&mut reader)?
        };
        let quote_reserve_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let migration_threshold: u64 = crate::borsh_de_or_default(&mut reader)?;
        let current_timestamp: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            pool,
            config,
            trade_direction,
            has_referral,
            swap_parameters,
            swap_result,
            quote_reserve_amount,
            migration_threshold,
            current_timestamp,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.pool, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.config, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.trade_direction, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.has_referral, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.swap_parameters, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.swap_result, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.quote_reserve_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.migration_threshold, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.current_timestamp, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct EvtSwap2WithTransferHookEvent(pub EvtSwap2WithTransferHook);
impl EvtSwap2WithTransferHookEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != EVT_SWAP2_WITH_TRANSFER_HOOK_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = EvtSwap2WithTransferHook::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&EVT_SWAP2_WITH_TRANSFER_HOOK_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const EVT_UPDATE_POOL_CREATOR_EVENT_DISCM: [u8; 8] = [
    107, 225, 165, 237, 91, 158, 213, 220,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct EvtUpdatePoolCreator {
    pub pool: Pubkey,
    pub creator: Pubkey,
    pub new_creator: Pubkey,
}
impl EvtUpdatePoolCreator {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let pool: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let creator: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let new_creator: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { pool, creator, new_creator })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.pool, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.creator, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.new_creator, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct EvtUpdatePoolCreatorEvent(pub EvtUpdatePoolCreator);
impl EvtUpdatePoolCreatorEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != EVT_UPDATE_POOL_CREATOR_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = EvtUpdatePoolCreator::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&EVT_UPDATE_POOL_CREATOR_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const EVT_VIRTUAL_POOL_METADATA_EVENT_DISCM: [u8; 8] = [
    188, 18, 72, 76, 195, 91, 38, 74,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct EvtVirtualPoolMetadata {
    pub virtual_pool_metadata: Pubkey,
    pub virtual_pool: Pubkey,
}
impl EvtVirtualPoolMetadata {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let virtual_pool_metadata: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let virtual_pool: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            virtual_pool_metadata,
            virtual_pool,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.virtual_pool_metadata, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.virtual_pool, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct EvtVirtualPoolMetadataEvent(pub EvtVirtualPoolMetadata);
impl EvtVirtualPoolMetadataEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != EVT_VIRTUAL_POOL_METADATA_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = EvtVirtualPoolMetadata::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&EVT_VIRTUAL_POOL_METADATA_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const EVT_WITHDRAW_LEFTOVER_EVENT_DISCM: [u8; 8] = [
    191, 189, 104, 143, 111, 156, 94, 229,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct EvtWithdrawLeftover {
    pub pool: Pubkey,
    pub leftover_receiver: Pubkey,
    pub leftover_amount: u64,
}
impl EvtWithdrawLeftover {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let pool: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let leftover_receiver: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let leftover_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            pool,
            leftover_receiver,
            leftover_amount,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.pool, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.leftover_receiver, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.leftover_amount, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct EvtWithdrawLeftoverEvent(pub EvtWithdrawLeftover);
impl EvtWithdrawLeftoverEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != EVT_WITHDRAW_LEFTOVER_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = EvtWithdrawLeftover::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&EVT_WITHDRAW_LEFTOVER_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const EVT_WITHDRAW_MIGRATION_FEE_EVENT_DISCM: [u8; 8] = [
    26, 203, 84, 85, 161, 23, 100, 214,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct EvtWithdrawMigrationFee {
    pub pool: Pubkey,
    pub fee: u64,
    pub flag: u8,
}
impl EvtWithdrawMigrationFee {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let pool: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let fee: u64 = crate::borsh_de_or_default(&mut reader)?;
        let flag: u8 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { pool, fee, flag })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.pool, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fee, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.flag, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct EvtWithdrawMigrationFeeEvent(pub EvtWithdrawMigrationFee);
impl EvtWithdrawMigrationFeeEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != EVT_WITHDRAW_MIGRATION_FEE_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = EvtWithdrawMigrationFee::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&EVT_WITHDRAW_MIGRATION_FEE_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
