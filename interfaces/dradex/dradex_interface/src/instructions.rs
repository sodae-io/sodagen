use solana_pubkey::Pubkey;
use solana_cpi::{invoke, invoke_signed};
use solana_instruction::{AccountMeta, Instruction};
use solana_account_info::AccountInfo;
use solana_program_error::ProgramError;
use std::io::Read;
#[allow(unused_imports)]
use crate::*;
#[derive(Clone, Debug, PartialEq)]
pub enum DradexProgramIx {
    SetDexConfig(SetDexConfigIxArgs),
    SetDexUserFeeTier(SetDexUserFeeTierIxArgs),
    CreateDexUser(CreateDexUserIxArgs),
    CreateMarketUser,
    CreateMarket(CreateMarketIxArgs),
    AddLiquidity(AddLiquidityIxArgs),
    RemoveLiquidity(RemoveLiquidityIxArgs),
    CreateOrder(CreateOrderIxArgs),
    SettleFunds,
    CancelOrder(CancelOrderIxArgs),
    DaoSetFundManager,
    DaoClaimRevenue(DaoClaimRevenueIxArgs),
    ConsumeEvents(ConsumeEventsIxArgs),
}
impl DradexProgramIx {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        if buf.starts_with(&SET_DEX_CONFIG_IX_DISCM) {
            let mut reader = &buf[SET_DEX_CONFIG_IX_DISCM.len()..];
            let input = if reader.is_empty() {
                Default::default()
            } else {
                <DexConfigInput>::deserialize(&mut reader)?
            };
            return Ok(Self::SetDexConfig(SetDexConfigIxArgs { input }));
        }
        if buf.starts_with(&SET_DEX_USER_FEE_TIER_IX_DISCM) {
            let mut reader = &buf[SET_DEX_USER_FEE_TIER_IX_DISCM.len()..];
            let input = if reader.is_empty() {
                Default::default()
            } else {
                <DexUserFeeTierInput>::deserialize(&mut reader)?
            };
            return Ok(Self::SetDexUserFeeTier(SetDexUserFeeTierIxArgs { input }));
        }
        if buf.starts_with(&CREATE_DEX_USER_IX_DISCM) {
            let mut reader = &buf[CREATE_DEX_USER_IX_DISCM.len()..];
            let referrer: Option<[u8; 32]> = crate::borsh_de_or_default(&mut reader)?;
            return Ok(Self::CreateDexUser(CreateDexUserIxArgs { referrer }));
        }
        if buf.starts_with(&CREATE_MARKET_USER_IX_DISCM) {
            return Ok(Self::CreateMarketUser);
        }
        if buf.starts_with(&CREATE_MARKET_IX_DISCM) {
            let mut reader = &buf[CREATE_MARKET_IX_DISCM.len()..];
            let input = if reader.is_empty() {
                Default::default()
            } else {
                <MarketInput>::deserialize(&mut reader)?
            };
            return Ok(Self::CreateMarket(CreateMarketIxArgs { input }));
        }
        if buf.starts_with(&ADD_LIQUIDITY_IX_DISCM) {
            let mut reader = &buf[ADD_LIQUIDITY_IX_DISCM.len()..];
            let t0_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            let t1_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::AddLiquidity(AddLiquidityIxArgs {
                    t0_amount,
                    t1_amount,
                }),
            );
        }
        if buf.starts_with(&REMOVE_LIQUIDITY_IX_DISCM) {
            let mut reader = &buf[REMOVE_LIQUIDITY_IX_DISCM.len()..];
            let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(Self::RemoveLiquidity(RemoveLiquidityIxArgs { amount }));
        }
        if buf.starts_with(&CREATE_ORDER_IX_DISCM) {
            let mut reader = &buf[CREATE_ORDER_IX_DISCM.len()..];
            let input = if reader.is_empty() {
                Default::default()
            } else {
                <OrderInput>::deserialize(&mut reader)?
            };
            return Ok(Self::CreateOrder(CreateOrderIxArgs { input }));
        }
        if buf.starts_with(&SETTLE_FUNDS_IX_DISCM) {
            return Ok(Self::SettleFunds);
        }
        if buf.starts_with(&CANCEL_ORDER_IX_DISCM) {
            let mut reader = &buf[CANCEL_ORDER_IX_DISCM.len()..];
            let input = if reader.is_empty() {
                Default::default()
            } else {
                <CancelOrderInput>::deserialize(&mut reader)?
            };
            return Ok(Self::CancelOrder(CancelOrderIxArgs { input }));
        }
        if buf.starts_with(&DAO_SET_FUND_MANAGER_IX_DISCM) {
            return Ok(Self::DaoSetFundManager);
        }
        if buf.starts_with(&DAO_CLAIM_REVENUE_IX_DISCM) {
            let mut reader = &buf[DAO_CLAIM_REVENUE_IX_DISCM.len()..];
            let token_index: u8 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::DaoClaimRevenue(DaoClaimRevenueIxArgs {
                    token_index,
                }),
            );
        }
        if buf.starts_with(&CONSUME_EVENTS_IX_DISCM) {
            let mut reader = &buf[CONSUME_EVENTS_IX_DISCM.len()..];
            let input = if reader.is_empty() {
                Default::default()
            } else {
                <ConsumeEventsInput>::deserialize(&mut reader)?
            };
            return Ok(Self::ConsumeEvents(ConsumeEventsIxArgs { input }));
        }
        Err(std::io::Error::from(std::io::ErrorKind::InvalidData))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        match self {
            Self::SetDexConfig(args) => {
                writer.write_all(&SET_DEX_CONFIG_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.input, &mut writer)?;
                Ok(())
            }
            Self::SetDexUserFeeTier(args) => {
                writer.write_all(&SET_DEX_USER_FEE_TIER_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.input, &mut writer)?;
                Ok(())
            }
            Self::CreateDexUser(args) => {
                writer.write_all(&CREATE_DEX_USER_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.referrer, &mut writer)?;
                Ok(())
            }
            Self::CreateMarketUser => writer.write_all(&CREATE_MARKET_USER_IX_DISCM),
            Self::CreateMarket(args) => {
                writer.write_all(&CREATE_MARKET_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.input, &mut writer)?;
                Ok(())
            }
            Self::AddLiquidity(args) => {
                writer.write_all(&ADD_LIQUIDITY_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.t0_amount, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.t1_amount, &mut writer)?;
                Ok(())
            }
            Self::RemoveLiquidity(args) => {
                writer.write_all(&REMOVE_LIQUIDITY_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.amount, &mut writer)?;
                Ok(())
            }
            Self::CreateOrder(args) => {
                writer.write_all(&CREATE_ORDER_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.input, &mut writer)?;
                Ok(())
            }
            Self::SettleFunds => writer.write_all(&SETTLE_FUNDS_IX_DISCM),
            Self::CancelOrder(args) => {
                writer.write_all(&CANCEL_ORDER_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.input, &mut writer)?;
                Ok(())
            }
            Self::DaoSetFundManager => writer.write_all(&DAO_SET_FUND_MANAGER_IX_DISCM),
            Self::DaoClaimRevenue(args) => {
                writer.write_all(&DAO_CLAIM_REVENUE_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.token_index, &mut writer)?;
                Ok(())
            }
            Self::ConsumeEvents(args) => {
                writer.write_all(&CONSUME_EVENTS_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.input, &mut writer)?;
                Ok(())
            }
        }
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
fn invoke_instruction<'info, A: Into<[AccountInfo<'info>; N]>, const N: usize>(
    ix: &Instruction,
    accounts: A,
) -> ProgramResult {
    let account_info: [AccountInfo<'info>; N] = accounts.into();
    invoke(ix, &account_info)
}
fn invoke_instruction_signed<'info, A: Into<[AccountInfo<'info>; N]>, const N: usize>(
    ix: &Instruction,
    accounts: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let account_info: [AccountInfo<'info>; N] = accounts.into();
    invoke_signed(ix, &account_info, seeds)
}
pub const SET_DEX_CONFIG_IX_ACCOUNTS_LEN: usize = 4;
#[derive(Copy, Clone, Debug)]
pub struct SetDexConfigAccounts<'me, 'info> {
    pub master: &'me AccountInfo<'info>,
    pub signer: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SetDexConfigKeys {
    pub master: Pubkey,
    pub signer: Pubkey,
    pub authority: Pubkey,
    pub system_program: Pubkey,
}
impl From<SetDexConfigAccounts<'_, '_>> for SetDexConfigKeys {
    fn from(accounts: SetDexConfigAccounts) -> Self {
        Self {
            master: *accounts.master.key,
            signer: *accounts.signer.key,
            authority: *accounts.authority.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<SetDexConfigKeys> for [AccountMeta; SET_DEX_CONFIG_IX_ACCOUNTS_LEN] {
    fn from(keys: SetDexConfigKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.master,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.signer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.authority,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.system_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; SET_DEX_CONFIG_IX_ACCOUNTS_LEN]> for SetDexConfigKeys {
    fn from(pubkeys: [Pubkey; SET_DEX_CONFIG_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            master: pubkeys[0],
            signer: pubkeys[1],
            authority: pubkeys[2],
            system_program: pubkeys[3],
        }
    }
}
impl<'info> From<SetDexConfigAccounts<'_, 'info>>
for [AccountInfo<'info>; SET_DEX_CONFIG_IX_ACCOUNTS_LEN] {
    fn from(accounts: SetDexConfigAccounts<'_, 'info>) -> Self {
        [
            accounts.master.clone(),
            accounts.signer.clone(),
            accounts.authority.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SET_DEX_CONFIG_IX_ACCOUNTS_LEN]>
for SetDexConfigAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; SET_DEX_CONFIG_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            master: &arr[0],
            signer: &arr[1],
            authority: &arr[2],
            system_program: &arr[3],
        }
    }
}
pub const SET_DEX_CONFIG_IX_DISCM: [u8; 8usize] = [29, 135, 205, 194, 87, 80, 59, 140];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SetDexConfigIxArgs {
    pub input: DexConfigInput,
}
#[derive(Clone, Debug, PartialEq)]
pub struct SetDexConfigIxData(pub SetDexConfigIxArgs);
impl From<SetDexConfigIxArgs> for SetDexConfigIxData {
    fn from(args: SetDexConfigIxArgs) -> Self {
        Self(args)
    }
}
impl SetDexConfigIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SET_DEX_CONFIG_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let input = if reader.is_empty() {
            Default::default()
        } else {
            <DexConfigInput>::deserialize(&mut reader)?
        };
        Ok(Self(SetDexConfigIxArgs { input }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SET_DEX_CONFIG_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.input, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn set_dex_config_ix_with_program_id(
    program_id: Pubkey,
    keys: SetDexConfigKeys,
    args: SetDexConfigIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SET_DEX_CONFIG_IX_ACCOUNTS_LEN] = keys.into();
    let data: SetDexConfigIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn set_dex_config_ix(
    keys: SetDexConfigKeys,
    args: SetDexConfigIxArgs,
) -> std::io::Result<Instruction> {
    set_dex_config_ix_with_program_id(DRADEX_PROGRAM_ID, keys, args)
}
pub fn set_dex_config_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SetDexConfigAccounts<'_, '_>,
    args: SetDexConfigIxArgs,
) -> ProgramResult {
    let keys: SetDexConfigKeys = accounts.into();
    let ix = set_dex_config_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn set_dex_config_invoke(
    accounts: SetDexConfigAccounts<'_, '_>,
    args: SetDexConfigIxArgs,
) -> ProgramResult {
    set_dex_config_invoke_with_program_id(DRADEX_PROGRAM_ID, accounts, args)
}
pub fn set_dex_config_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SetDexConfigAccounts<'_, '_>,
    args: SetDexConfigIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SetDexConfigKeys = accounts.into();
    let ix = set_dex_config_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn set_dex_config_invoke_signed(
    accounts: SetDexConfigAccounts<'_, '_>,
    args: SetDexConfigIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    set_dex_config_invoke_signed_with_program_id(
        DRADEX_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn set_dex_config_verify_account_keys(
    accounts: SetDexConfigAccounts<'_, '_>,
    keys: SetDexConfigKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.master.key, keys.master),
        (*accounts.signer.key, keys.signer),
        (*accounts.authority.key, keys.authority),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn set_dex_config_verify_writable_privileges<'me, 'info>(
    accounts: SetDexConfigAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.master, accounts.signer] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn set_dex_config_verify_signer_privileges<'me, 'info>(
    accounts: SetDexConfigAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.signer, accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn set_dex_config_verify_account_privileges<'me, 'info>(
    accounts: SetDexConfigAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    set_dex_config_verify_writable_privileges(accounts)?;
    set_dex_config_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SET_DEX_USER_FEE_TIER_IX_ACCOUNTS_LEN: usize = 5;
#[derive(Copy, Clone, Debug)]
pub struct SetDexUserFeeTierAccounts<'me, 'info> {
    pub dex_user: &'me AccountInfo<'info>,
    pub user: &'me AccountInfo<'info>,
    pub payer: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SetDexUserFeeTierKeys {
    pub dex_user: Pubkey,
    pub user: Pubkey,
    pub payer: Pubkey,
    pub authority: Pubkey,
    pub system_program: Pubkey,
}
impl From<SetDexUserFeeTierAccounts<'_, '_>> for SetDexUserFeeTierKeys {
    fn from(accounts: SetDexUserFeeTierAccounts) -> Self {
        Self {
            dex_user: *accounts.dex_user.key,
            user: *accounts.user.key,
            payer: *accounts.payer.key,
            authority: *accounts.authority.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<SetDexUserFeeTierKeys>
for [AccountMeta; SET_DEX_USER_FEE_TIER_IX_ACCOUNTS_LEN] {
    fn from(keys: SetDexUserFeeTierKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.dex_user,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.payer,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.authority,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.system_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; SET_DEX_USER_FEE_TIER_IX_ACCOUNTS_LEN]> for SetDexUserFeeTierKeys {
    fn from(pubkeys: [Pubkey; SET_DEX_USER_FEE_TIER_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            dex_user: pubkeys[0],
            user: pubkeys[1],
            payer: pubkeys[2],
            authority: pubkeys[3],
            system_program: pubkeys[4],
        }
    }
}
impl<'info> From<SetDexUserFeeTierAccounts<'_, 'info>>
for [AccountInfo<'info>; SET_DEX_USER_FEE_TIER_IX_ACCOUNTS_LEN] {
    fn from(accounts: SetDexUserFeeTierAccounts<'_, 'info>) -> Self {
        [
            accounts.dex_user.clone(),
            accounts.user.clone(),
            accounts.payer.clone(),
            accounts.authority.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SET_DEX_USER_FEE_TIER_IX_ACCOUNTS_LEN]>
for SetDexUserFeeTierAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; SET_DEX_USER_FEE_TIER_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            dex_user: &arr[0],
            user: &arr[1],
            payer: &arr[2],
            authority: &arr[3],
            system_program: &arr[4],
        }
    }
}
pub const SET_DEX_USER_FEE_TIER_IX_DISCM: [u8; 8usize] = [
    75, 213, 151, 11, 33, 16, 224, 75,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SetDexUserFeeTierIxArgs {
    pub input: DexUserFeeTierInput,
}
#[derive(Clone, Debug, PartialEq)]
pub struct SetDexUserFeeTierIxData(pub SetDexUserFeeTierIxArgs);
impl From<SetDexUserFeeTierIxArgs> for SetDexUserFeeTierIxData {
    fn from(args: SetDexUserFeeTierIxArgs) -> Self {
        Self(args)
    }
}
impl SetDexUserFeeTierIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SET_DEX_USER_FEE_TIER_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let input = if reader.is_empty() {
            Default::default()
        } else {
            <DexUserFeeTierInput>::deserialize(&mut reader)?
        };
        Ok(Self(SetDexUserFeeTierIxArgs { input }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SET_DEX_USER_FEE_TIER_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.input, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn set_dex_user_fee_tier_ix_with_program_id(
    program_id: Pubkey,
    keys: SetDexUserFeeTierKeys,
    args: SetDexUserFeeTierIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SET_DEX_USER_FEE_TIER_IX_ACCOUNTS_LEN] = keys.into();
    let data: SetDexUserFeeTierIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn set_dex_user_fee_tier_ix(
    keys: SetDexUserFeeTierKeys,
    args: SetDexUserFeeTierIxArgs,
) -> std::io::Result<Instruction> {
    set_dex_user_fee_tier_ix_with_program_id(DRADEX_PROGRAM_ID, keys, args)
}
pub fn set_dex_user_fee_tier_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SetDexUserFeeTierAccounts<'_, '_>,
    args: SetDexUserFeeTierIxArgs,
) -> ProgramResult {
    let keys: SetDexUserFeeTierKeys = accounts.into();
    let ix = set_dex_user_fee_tier_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn set_dex_user_fee_tier_invoke(
    accounts: SetDexUserFeeTierAccounts<'_, '_>,
    args: SetDexUserFeeTierIxArgs,
) -> ProgramResult {
    set_dex_user_fee_tier_invoke_with_program_id(DRADEX_PROGRAM_ID, accounts, args)
}
pub fn set_dex_user_fee_tier_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SetDexUserFeeTierAccounts<'_, '_>,
    args: SetDexUserFeeTierIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SetDexUserFeeTierKeys = accounts.into();
    let ix = set_dex_user_fee_tier_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn set_dex_user_fee_tier_invoke_signed(
    accounts: SetDexUserFeeTierAccounts<'_, '_>,
    args: SetDexUserFeeTierIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    set_dex_user_fee_tier_invoke_signed_with_program_id(
        DRADEX_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn set_dex_user_fee_tier_verify_account_keys(
    accounts: SetDexUserFeeTierAccounts<'_, '_>,
    keys: SetDexUserFeeTierKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.dex_user.key, keys.dex_user),
        (*accounts.user.key, keys.user),
        (*accounts.payer.key, keys.payer),
        (*accounts.authority.key, keys.authority),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn set_dex_user_fee_tier_verify_writable_privileges<'me, 'info>(
    accounts: SetDexUserFeeTierAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.dex_user] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn set_dex_user_fee_tier_verify_signer_privileges<'me, 'info>(
    accounts: SetDexUserFeeTierAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.payer, accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn set_dex_user_fee_tier_verify_account_privileges<'me, 'info>(
    accounts: SetDexUserFeeTierAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    set_dex_user_fee_tier_verify_writable_privileges(accounts)?;
    set_dex_user_fee_tier_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CREATE_DEX_USER_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct CreateDexUserAccounts<'me, 'info> {
    pub dex_user: &'me AccountInfo<'info>,
    pub signer: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CreateDexUserKeys {
    pub dex_user: Pubkey,
    pub signer: Pubkey,
    pub system_program: Pubkey,
}
impl From<CreateDexUserAccounts<'_, '_>> for CreateDexUserKeys {
    fn from(accounts: CreateDexUserAccounts) -> Self {
        Self {
            dex_user: *accounts.dex_user.key,
            signer: *accounts.signer.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<CreateDexUserKeys> for [AccountMeta; CREATE_DEX_USER_IX_ACCOUNTS_LEN] {
    fn from(keys: CreateDexUserKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.dex_user,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.signer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.system_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; CREATE_DEX_USER_IX_ACCOUNTS_LEN]> for CreateDexUserKeys {
    fn from(pubkeys: [Pubkey; CREATE_DEX_USER_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            dex_user: pubkeys[0],
            signer: pubkeys[1],
            system_program: pubkeys[2],
        }
    }
}
impl<'info> From<CreateDexUserAccounts<'_, 'info>>
for [AccountInfo<'info>; CREATE_DEX_USER_IX_ACCOUNTS_LEN] {
    fn from(accounts: CreateDexUserAccounts<'_, 'info>) -> Self {
        [
            accounts.dex_user.clone(),
            accounts.signer.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CREATE_DEX_USER_IX_ACCOUNTS_LEN]>
for CreateDexUserAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; CREATE_DEX_USER_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            dex_user: &arr[0],
            signer: &arr[1],
            system_program: &arr[2],
        }
    }
}
pub const CREATE_DEX_USER_IX_DISCM: [u8; 8usize] = [4, 117, 37, 192, 40, 124, 9, 10];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CreateDexUserIxArgs {
    pub referrer: Option<[u8; 32]>,
}
#[derive(Clone, Debug, PartialEq)]
pub struct CreateDexUserIxData(pub CreateDexUserIxArgs);
impl From<CreateDexUserIxArgs> for CreateDexUserIxData {
    fn from(args: CreateDexUserIxArgs) -> Self {
        Self(args)
    }
}
impl CreateDexUserIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CREATE_DEX_USER_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let referrer: Option<[u8; 32]> = crate::borsh_de_or_default(&mut reader)?;
        Ok(Self(CreateDexUserIxArgs { referrer }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CREATE_DEX_USER_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.referrer, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn create_dex_user_ix_with_program_id(
    program_id: Pubkey,
    keys: CreateDexUserKeys,
    args: CreateDexUserIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CREATE_DEX_USER_IX_ACCOUNTS_LEN] = keys.into();
    let data: CreateDexUserIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn create_dex_user_ix(
    keys: CreateDexUserKeys,
    args: CreateDexUserIxArgs,
) -> std::io::Result<Instruction> {
    create_dex_user_ix_with_program_id(DRADEX_PROGRAM_ID, keys, args)
}
pub fn create_dex_user_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CreateDexUserAccounts<'_, '_>,
    args: CreateDexUserIxArgs,
) -> ProgramResult {
    let keys: CreateDexUserKeys = accounts.into();
    let ix = create_dex_user_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn create_dex_user_invoke(
    accounts: CreateDexUserAccounts<'_, '_>,
    args: CreateDexUserIxArgs,
) -> ProgramResult {
    create_dex_user_invoke_with_program_id(DRADEX_PROGRAM_ID, accounts, args)
}
pub fn create_dex_user_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CreateDexUserAccounts<'_, '_>,
    args: CreateDexUserIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CreateDexUserKeys = accounts.into();
    let ix = create_dex_user_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn create_dex_user_invoke_signed(
    accounts: CreateDexUserAccounts<'_, '_>,
    args: CreateDexUserIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    create_dex_user_invoke_signed_with_program_id(
        DRADEX_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn create_dex_user_verify_account_keys(
    accounts: CreateDexUserAccounts<'_, '_>,
    keys: CreateDexUserKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.dex_user.key, keys.dex_user),
        (*accounts.signer.key, keys.signer),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn create_dex_user_verify_writable_privileges<'me, 'info>(
    accounts: CreateDexUserAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.dex_user, accounts.signer] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn create_dex_user_verify_signer_privileges<'me, 'info>(
    accounts: CreateDexUserAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.signer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn create_dex_user_verify_account_privileges<'me, 'info>(
    accounts: CreateDexUserAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    create_dex_user_verify_writable_privileges(accounts)?;
    create_dex_user_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CREATE_MARKET_USER_IX_ACCOUNTS_LEN: usize = 5;
#[derive(Copy, Clone, Debug)]
pub struct CreateMarketUserAccounts<'me, 'info> {
    pub pair: &'me AccountInfo<'info>,
    pub market: &'me AccountInfo<'info>,
    pub market_user: &'me AccountInfo<'info>,
    pub signer: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CreateMarketUserKeys {
    pub pair: Pubkey,
    pub market: Pubkey,
    pub market_user: Pubkey,
    pub signer: Pubkey,
    pub system_program: Pubkey,
}
impl From<CreateMarketUserAccounts<'_, '_>> for CreateMarketUserKeys {
    fn from(accounts: CreateMarketUserAccounts) -> Self {
        Self {
            pair: *accounts.pair.key,
            market: *accounts.market.key,
            market_user: *accounts.market_user.key,
            signer: *accounts.signer.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<CreateMarketUserKeys> for [AccountMeta; CREATE_MARKET_USER_IX_ACCOUNTS_LEN] {
    fn from(keys: CreateMarketUserKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.pair,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.market,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.market_user,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.signer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.system_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; CREATE_MARKET_USER_IX_ACCOUNTS_LEN]> for CreateMarketUserKeys {
    fn from(pubkeys: [Pubkey; CREATE_MARKET_USER_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            pair: pubkeys[0],
            market: pubkeys[1],
            market_user: pubkeys[2],
            signer: pubkeys[3],
            system_program: pubkeys[4],
        }
    }
}
impl<'info> From<CreateMarketUserAccounts<'_, 'info>>
for [AccountInfo<'info>; CREATE_MARKET_USER_IX_ACCOUNTS_LEN] {
    fn from(accounts: CreateMarketUserAccounts<'_, 'info>) -> Self {
        [
            accounts.pair.clone(),
            accounts.market.clone(),
            accounts.market_user.clone(),
            accounts.signer.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CREATE_MARKET_USER_IX_ACCOUNTS_LEN]>
for CreateMarketUserAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; CREATE_MARKET_USER_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            pair: &arr[0],
            market: &arr[1],
            market_user: &arr[2],
            signer: &arr[3],
            system_program: &arr[4],
        }
    }
}
pub const CREATE_MARKET_USER_IX_DISCM: [u8; 8usize] = [
    131, 190, 94, 92, 253, 248, 25, 105,
];
#[derive(Clone, Debug, PartialEq)]
pub struct CreateMarketUserIxData;
impl CreateMarketUserIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CREATE_MARKET_USER_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CREATE_MARKET_USER_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn create_market_user_ix_with_program_id(
    program_id: Pubkey,
    keys: CreateMarketUserKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CREATE_MARKET_USER_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: CreateMarketUserIxData.try_to_vec()?,
    })
}
pub fn create_market_user_ix(
    keys: CreateMarketUserKeys,
) -> std::io::Result<Instruction> {
    create_market_user_ix_with_program_id(DRADEX_PROGRAM_ID, keys)
}
pub fn create_market_user_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CreateMarketUserAccounts<'_, '_>,
) -> ProgramResult {
    let keys: CreateMarketUserKeys = accounts.into();
    let ix = create_market_user_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn create_market_user_invoke(
    accounts: CreateMarketUserAccounts<'_, '_>,
) -> ProgramResult {
    create_market_user_invoke_with_program_id(DRADEX_PROGRAM_ID, accounts)
}
pub fn create_market_user_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CreateMarketUserAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CreateMarketUserKeys = accounts.into();
    let ix = create_market_user_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn create_market_user_invoke_signed(
    accounts: CreateMarketUserAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    create_market_user_invoke_signed_with_program_id(DRADEX_PROGRAM_ID, accounts, seeds)
}
pub fn create_market_user_verify_account_keys(
    accounts: CreateMarketUserAccounts<'_, '_>,
    keys: CreateMarketUserKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.pair.key, keys.pair),
        (*accounts.market.key, keys.market),
        (*accounts.market_user.key, keys.market_user),
        (*accounts.signer.key, keys.signer),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn create_market_user_verify_writable_privileges<'me, 'info>(
    accounts: CreateMarketUserAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.market, accounts.market_user, accounts.signer] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn create_market_user_verify_signer_privileges<'me, 'info>(
    accounts: CreateMarketUserAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.signer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn create_market_user_verify_account_privileges<'me, 'info>(
    accounts: CreateMarketUserAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    create_market_user_verify_writable_privileges(accounts)?;
    create_market_user_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CREATE_MARKET_IX_ACCOUNTS_LEN: usize = 15;
#[derive(Copy, Clone, Debug)]
pub struct CreateMarketAccounts<'me, 'info> {
    pub pair: &'me AccountInfo<'info>,
    pub market: &'me AccountInfo<'info>,
    pub event_queue: &'me AccountInfo<'info>,
    pub t0_mint: &'me AccountInfo<'info>,
    pub t1_mint: &'me AccountInfo<'info>,
    pub lp_token_mint: &'me AccountInfo<'info>,
    pub t0_vault: &'me AccountInfo<'info>,
    pub t1_vault: &'me AccountInfo<'info>,
    pub bids: &'me AccountInfo<'info>,
    pub asks: &'me AccountInfo<'info>,
    pub master: &'me AccountInfo<'info>,
    pub signer: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub rent: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CreateMarketKeys {
    pub pair: Pubkey,
    pub market: Pubkey,
    pub event_queue: Pubkey,
    pub t0_mint: Pubkey,
    pub t1_mint: Pubkey,
    pub lp_token_mint: Pubkey,
    pub t0_vault: Pubkey,
    pub t1_vault: Pubkey,
    pub bids: Pubkey,
    pub asks: Pubkey,
    pub master: Pubkey,
    pub signer: Pubkey,
    pub system_program: Pubkey,
    pub rent: Pubkey,
    pub token_program: Pubkey,
}
impl From<CreateMarketAccounts<'_, '_>> for CreateMarketKeys {
    fn from(accounts: CreateMarketAccounts) -> Self {
        Self {
            pair: *accounts.pair.key,
            market: *accounts.market.key,
            event_queue: *accounts.event_queue.key,
            t0_mint: *accounts.t0_mint.key,
            t1_mint: *accounts.t1_mint.key,
            lp_token_mint: *accounts.lp_token_mint.key,
            t0_vault: *accounts.t0_vault.key,
            t1_vault: *accounts.t1_vault.key,
            bids: *accounts.bids.key,
            asks: *accounts.asks.key,
            master: *accounts.master.key,
            signer: *accounts.signer.key,
            system_program: *accounts.system_program.key,
            rent: *accounts.rent.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<CreateMarketKeys> for [AccountMeta; CREATE_MARKET_IX_ACCOUNTS_LEN] {
    fn from(keys: CreateMarketKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.pair,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.market,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.event_queue,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.t0_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.t1_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.lp_token_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.t0_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.t1_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.bids,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.asks,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.master,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.signer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.system_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.rent,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; CREATE_MARKET_IX_ACCOUNTS_LEN]> for CreateMarketKeys {
    fn from(pubkeys: [Pubkey; CREATE_MARKET_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            pair: pubkeys[0],
            market: pubkeys[1],
            event_queue: pubkeys[2],
            t0_mint: pubkeys[3],
            t1_mint: pubkeys[4],
            lp_token_mint: pubkeys[5],
            t0_vault: pubkeys[6],
            t1_vault: pubkeys[7],
            bids: pubkeys[8],
            asks: pubkeys[9],
            master: pubkeys[10],
            signer: pubkeys[11],
            system_program: pubkeys[12],
            rent: pubkeys[13],
            token_program: pubkeys[14],
        }
    }
}
impl<'info> From<CreateMarketAccounts<'_, 'info>>
for [AccountInfo<'info>; CREATE_MARKET_IX_ACCOUNTS_LEN] {
    fn from(accounts: CreateMarketAccounts<'_, 'info>) -> Self {
        [
            accounts.pair.clone(),
            accounts.market.clone(),
            accounts.event_queue.clone(),
            accounts.t0_mint.clone(),
            accounts.t1_mint.clone(),
            accounts.lp_token_mint.clone(),
            accounts.t0_vault.clone(),
            accounts.t1_vault.clone(),
            accounts.bids.clone(),
            accounts.asks.clone(),
            accounts.master.clone(),
            accounts.signer.clone(),
            accounts.system_program.clone(),
            accounts.rent.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CREATE_MARKET_IX_ACCOUNTS_LEN]>
for CreateMarketAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; CREATE_MARKET_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            pair: &arr[0],
            market: &arr[1],
            event_queue: &arr[2],
            t0_mint: &arr[3],
            t1_mint: &arr[4],
            lp_token_mint: &arr[5],
            t0_vault: &arr[6],
            t1_vault: &arr[7],
            bids: &arr[8],
            asks: &arr[9],
            master: &arr[10],
            signer: &arr[11],
            system_program: &arr[12],
            rent: &arr[13],
            token_program: &arr[14],
        }
    }
}
pub const CREATE_MARKET_IX_DISCM: [u8; 8usize] = [103, 226, 97, 235, 200, 188, 251, 254];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CreateMarketIxArgs {
    pub input: MarketInput,
}
#[derive(Clone, Debug, PartialEq)]
pub struct CreateMarketIxData(pub CreateMarketIxArgs);
impl From<CreateMarketIxArgs> for CreateMarketIxData {
    fn from(args: CreateMarketIxArgs) -> Self {
        Self(args)
    }
}
impl CreateMarketIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CREATE_MARKET_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let input = if reader.is_empty() {
            Default::default()
        } else {
            <MarketInput>::deserialize(&mut reader)?
        };
        Ok(Self(CreateMarketIxArgs { input }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CREATE_MARKET_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.input, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn create_market_ix_with_program_id(
    program_id: Pubkey,
    keys: CreateMarketKeys,
    args: CreateMarketIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CREATE_MARKET_IX_ACCOUNTS_LEN] = keys.into();
    let data: CreateMarketIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn create_market_ix(
    keys: CreateMarketKeys,
    args: CreateMarketIxArgs,
) -> std::io::Result<Instruction> {
    create_market_ix_with_program_id(DRADEX_PROGRAM_ID, keys, args)
}
pub fn create_market_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CreateMarketAccounts<'_, '_>,
    args: CreateMarketIxArgs,
) -> ProgramResult {
    let keys: CreateMarketKeys = accounts.into();
    let ix = create_market_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn create_market_invoke(
    accounts: CreateMarketAccounts<'_, '_>,
    args: CreateMarketIxArgs,
) -> ProgramResult {
    create_market_invoke_with_program_id(DRADEX_PROGRAM_ID, accounts, args)
}
pub fn create_market_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CreateMarketAccounts<'_, '_>,
    args: CreateMarketIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CreateMarketKeys = accounts.into();
    let ix = create_market_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn create_market_invoke_signed(
    accounts: CreateMarketAccounts<'_, '_>,
    args: CreateMarketIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    create_market_invoke_signed_with_program_id(DRADEX_PROGRAM_ID, accounts, args, seeds)
}
pub fn create_market_verify_account_keys(
    accounts: CreateMarketAccounts<'_, '_>,
    keys: CreateMarketKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.pair.key, keys.pair),
        (*accounts.market.key, keys.market),
        (*accounts.event_queue.key, keys.event_queue),
        (*accounts.t0_mint.key, keys.t0_mint),
        (*accounts.t1_mint.key, keys.t1_mint),
        (*accounts.lp_token_mint.key, keys.lp_token_mint),
        (*accounts.t0_vault.key, keys.t0_vault),
        (*accounts.t1_vault.key, keys.t1_vault),
        (*accounts.bids.key, keys.bids),
        (*accounts.asks.key, keys.asks),
        (*accounts.master.key, keys.master),
        (*accounts.signer.key, keys.signer),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.rent.key, keys.rent),
        (*accounts.token_program.key, keys.token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn create_market_verify_writable_privileges<'me, 'info>(
    accounts: CreateMarketAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.pair,
        accounts.market,
        accounts.event_queue,
        accounts.lp_token_mint,
        accounts.t0_vault,
        accounts.t1_vault,
        accounts.bids,
        accounts.asks,
        accounts.master,
        accounts.signer,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn create_market_verify_signer_privileges<'me, 'info>(
    accounts: CreateMarketAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.pair, accounts.signer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn create_market_verify_account_privileges<'me, 'info>(
    accounts: CreateMarketAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    create_market_verify_writable_privileges(accounts)?;
    create_market_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const ADD_LIQUIDITY_IX_ACCOUNTS_LEN: usize = 17;
#[derive(Copy, Clone, Debug)]
pub struct AddLiquidityAccounts<'me, 'info> {
    pub pair: &'me AccountInfo<'info>,
    pub market: &'me AccountInfo<'info>,
    pub t0_mint: &'me AccountInfo<'info>,
    pub t1_mint: &'me AccountInfo<'info>,
    pub t0_vault: &'me AccountInfo<'info>,
    pub t0_user: &'me AccountInfo<'info>,
    pub t1_vault: &'me AccountInfo<'info>,
    pub t1_user: &'me AccountInfo<'info>,
    pub lp_token_mint: &'me AccountInfo<'info>,
    pub lp_token_user: &'me AccountInfo<'info>,
    pub lp_token_vault: &'me AccountInfo<'info>,
    pub master: &'me AccountInfo<'info>,
    pub signer: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub rent: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub logger: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct AddLiquidityKeys {
    pub pair: Pubkey,
    pub market: Pubkey,
    pub t0_mint: Pubkey,
    pub t1_mint: Pubkey,
    pub t0_vault: Pubkey,
    pub t0_user: Pubkey,
    pub t1_vault: Pubkey,
    pub t1_user: Pubkey,
    pub lp_token_mint: Pubkey,
    pub lp_token_user: Pubkey,
    pub lp_token_vault: Pubkey,
    pub master: Pubkey,
    pub signer: Pubkey,
    pub system_program: Pubkey,
    pub rent: Pubkey,
    pub token_program: Pubkey,
    pub logger: Pubkey,
}
impl From<AddLiquidityAccounts<'_, '_>> for AddLiquidityKeys {
    fn from(accounts: AddLiquidityAccounts) -> Self {
        Self {
            pair: *accounts.pair.key,
            market: *accounts.market.key,
            t0_mint: *accounts.t0_mint.key,
            t1_mint: *accounts.t1_mint.key,
            t0_vault: *accounts.t0_vault.key,
            t0_user: *accounts.t0_user.key,
            t1_vault: *accounts.t1_vault.key,
            t1_user: *accounts.t1_user.key,
            lp_token_mint: *accounts.lp_token_mint.key,
            lp_token_user: *accounts.lp_token_user.key,
            lp_token_vault: *accounts.lp_token_vault.key,
            master: *accounts.master.key,
            signer: *accounts.signer.key,
            system_program: *accounts.system_program.key,
            rent: *accounts.rent.key,
            token_program: *accounts.token_program.key,
            logger: *accounts.logger.key,
        }
    }
}
impl From<AddLiquidityKeys> for [AccountMeta; ADD_LIQUIDITY_IX_ACCOUNTS_LEN] {
    fn from(keys: AddLiquidityKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.pair,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.market,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.t0_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.t1_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.t0_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.t0_user,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.t1_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.t1_user,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.lp_token_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.lp_token_user,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.lp_token_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.master,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.signer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.system_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.rent,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.logger,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; ADD_LIQUIDITY_IX_ACCOUNTS_LEN]> for AddLiquidityKeys {
    fn from(pubkeys: [Pubkey; ADD_LIQUIDITY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            pair: pubkeys[0],
            market: pubkeys[1],
            t0_mint: pubkeys[2],
            t1_mint: pubkeys[3],
            t0_vault: pubkeys[4],
            t0_user: pubkeys[5],
            t1_vault: pubkeys[6],
            t1_user: pubkeys[7],
            lp_token_mint: pubkeys[8],
            lp_token_user: pubkeys[9],
            lp_token_vault: pubkeys[10],
            master: pubkeys[11],
            signer: pubkeys[12],
            system_program: pubkeys[13],
            rent: pubkeys[14],
            token_program: pubkeys[15],
            logger: pubkeys[16],
        }
    }
}
impl<'info> From<AddLiquidityAccounts<'_, 'info>>
for [AccountInfo<'info>; ADD_LIQUIDITY_IX_ACCOUNTS_LEN] {
    fn from(accounts: AddLiquidityAccounts<'_, 'info>) -> Self {
        [
            accounts.pair.clone(),
            accounts.market.clone(),
            accounts.t0_mint.clone(),
            accounts.t1_mint.clone(),
            accounts.t0_vault.clone(),
            accounts.t0_user.clone(),
            accounts.t1_vault.clone(),
            accounts.t1_user.clone(),
            accounts.lp_token_mint.clone(),
            accounts.lp_token_user.clone(),
            accounts.lp_token_vault.clone(),
            accounts.master.clone(),
            accounts.signer.clone(),
            accounts.system_program.clone(),
            accounts.rent.clone(),
            accounts.token_program.clone(),
            accounts.logger.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; ADD_LIQUIDITY_IX_ACCOUNTS_LEN]>
for AddLiquidityAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; ADD_LIQUIDITY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            pair: &arr[0],
            market: &arr[1],
            t0_mint: &arr[2],
            t1_mint: &arr[3],
            t0_vault: &arr[4],
            t0_user: &arr[5],
            t1_vault: &arr[6],
            t1_user: &arr[7],
            lp_token_mint: &arr[8],
            lp_token_user: &arr[9],
            lp_token_vault: &arr[10],
            master: &arr[11],
            signer: &arr[12],
            system_program: &arr[13],
            rent: &arr[14],
            token_program: &arr[15],
            logger: &arr[16],
        }
    }
}
pub const ADD_LIQUIDITY_IX_DISCM: [u8; 8usize] = [181, 157, 89, 67, 143, 182, 52, 72];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AddLiquidityIxArgs {
    pub t0_amount: u64,
    pub t1_amount: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct AddLiquidityIxData(pub AddLiquidityIxArgs);
impl From<AddLiquidityIxArgs> for AddLiquidityIxData {
    fn from(args: AddLiquidityIxArgs) -> Self {
        Self(args)
    }
}
impl AddLiquidityIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != ADD_LIQUIDITY_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let t0_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let t1_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(AddLiquidityIxArgs {
                t0_amount,
                t1_amount,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&ADD_LIQUIDITY_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.t0_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.t1_amount, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn add_liquidity_ix_with_program_id(
    program_id: Pubkey,
    keys: AddLiquidityKeys,
    args: AddLiquidityIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; ADD_LIQUIDITY_IX_ACCOUNTS_LEN] = keys.into();
    let data: AddLiquidityIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn add_liquidity_ix(
    keys: AddLiquidityKeys,
    args: AddLiquidityIxArgs,
) -> std::io::Result<Instruction> {
    add_liquidity_ix_with_program_id(DRADEX_PROGRAM_ID, keys, args)
}
pub fn add_liquidity_invoke_with_program_id(
    program_id: Pubkey,
    accounts: AddLiquidityAccounts<'_, '_>,
    args: AddLiquidityIxArgs,
) -> ProgramResult {
    let keys: AddLiquidityKeys = accounts.into();
    let ix = add_liquidity_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn add_liquidity_invoke(
    accounts: AddLiquidityAccounts<'_, '_>,
    args: AddLiquidityIxArgs,
) -> ProgramResult {
    add_liquidity_invoke_with_program_id(DRADEX_PROGRAM_ID, accounts, args)
}
pub fn add_liquidity_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: AddLiquidityAccounts<'_, '_>,
    args: AddLiquidityIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: AddLiquidityKeys = accounts.into();
    let ix = add_liquidity_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn add_liquidity_invoke_signed(
    accounts: AddLiquidityAccounts<'_, '_>,
    args: AddLiquidityIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    add_liquidity_invoke_signed_with_program_id(DRADEX_PROGRAM_ID, accounts, args, seeds)
}
pub fn add_liquidity_verify_account_keys(
    accounts: AddLiquidityAccounts<'_, '_>,
    keys: AddLiquidityKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.pair.key, keys.pair),
        (*accounts.market.key, keys.market),
        (*accounts.t0_mint.key, keys.t0_mint),
        (*accounts.t1_mint.key, keys.t1_mint),
        (*accounts.t0_vault.key, keys.t0_vault),
        (*accounts.t0_user.key, keys.t0_user),
        (*accounts.t1_vault.key, keys.t1_vault),
        (*accounts.t1_user.key, keys.t1_user),
        (*accounts.lp_token_mint.key, keys.lp_token_mint),
        (*accounts.lp_token_user.key, keys.lp_token_user),
        (*accounts.lp_token_vault.key, keys.lp_token_vault),
        (*accounts.master.key, keys.master),
        (*accounts.signer.key, keys.signer),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.rent.key, keys.rent),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.logger.key, keys.logger),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn add_liquidity_verify_writable_privileges<'me, 'info>(
    accounts: AddLiquidityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.market,
        accounts.t0_mint,
        accounts.t1_mint,
        accounts.t0_vault,
        accounts.t0_user,
        accounts.t1_vault,
        accounts.t1_user,
        accounts.lp_token_mint,
        accounts.lp_token_user,
        accounts.lp_token_vault,
        accounts.master,
        accounts.signer,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn add_liquidity_verify_signer_privileges<'me, 'info>(
    accounts: AddLiquidityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.signer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn add_liquidity_verify_account_privileges<'me, 'info>(
    accounts: AddLiquidityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    add_liquidity_verify_writable_privileges(accounts)?;
    add_liquidity_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const REMOVE_LIQUIDITY_IX_ACCOUNTS_LEN: usize = 17;
#[derive(Copy, Clone, Debug)]
pub struct RemoveLiquidityAccounts<'me, 'info> {
    pub pair: &'me AccountInfo<'info>,
    pub market: &'me AccountInfo<'info>,
    pub t0_mint: &'me AccountInfo<'info>,
    pub t1_mint: &'me AccountInfo<'info>,
    pub t0_vault: &'me AccountInfo<'info>,
    pub t0_user: &'me AccountInfo<'info>,
    pub t1_vault: &'me AccountInfo<'info>,
    pub t1_user: &'me AccountInfo<'info>,
    pub lp_token_mint: &'me AccountInfo<'info>,
    pub lp_token_user: &'me AccountInfo<'info>,
    pub lp_token_vault: &'me AccountInfo<'info>,
    pub master: &'me AccountInfo<'info>,
    pub signer: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub rent: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub logger: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct RemoveLiquidityKeys {
    pub pair: Pubkey,
    pub market: Pubkey,
    pub t0_mint: Pubkey,
    pub t1_mint: Pubkey,
    pub t0_vault: Pubkey,
    pub t0_user: Pubkey,
    pub t1_vault: Pubkey,
    pub t1_user: Pubkey,
    pub lp_token_mint: Pubkey,
    pub lp_token_user: Pubkey,
    pub lp_token_vault: Pubkey,
    pub master: Pubkey,
    pub signer: Pubkey,
    pub system_program: Pubkey,
    pub rent: Pubkey,
    pub token_program: Pubkey,
    pub logger: Pubkey,
}
impl From<RemoveLiquidityAccounts<'_, '_>> for RemoveLiquidityKeys {
    fn from(accounts: RemoveLiquidityAccounts) -> Self {
        Self {
            pair: *accounts.pair.key,
            market: *accounts.market.key,
            t0_mint: *accounts.t0_mint.key,
            t1_mint: *accounts.t1_mint.key,
            t0_vault: *accounts.t0_vault.key,
            t0_user: *accounts.t0_user.key,
            t1_vault: *accounts.t1_vault.key,
            t1_user: *accounts.t1_user.key,
            lp_token_mint: *accounts.lp_token_mint.key,
            lp_token_user: *accounts.lp_token_user.key,
            lp_token_vault: *accounts.lp_token_vault.key,
            master: *accounts.master.key,
            signer: *accounts.signer.key,
            system_program: *accounts.system_program.key,
            rent: *accounts.rent.key,
            token_program: *accounts.token_program.key,
            logger: *accounts.logger.key,
        }
    }
}
impl From<RemoveLiquidityKeys> for [AccountMeta; REMOVE_LIQUIDITY_IX_ACCOUNTS_LEN] {
    fn from(keys: RemoveLiquidityKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.pair,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.market,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.t0_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.t1_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.t0_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.t0_user,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.t1_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.t1_user,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.lp_token_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.lp_token_user,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.lp_token_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.master,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.signer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.system_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.rent,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.logger,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; REMOVE_LIQUIDITY_IX_ACCOUNTS_LEN]> for RemoveLiquidityKeys {
    fn from(pubkeys: [Pubkey; REMOVE_LIQUIDITY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            pair: pubkeys[0],
            market: pubkeys[1],
            t0_mint: pubkeys[2],
            t1_mint: pubkeys[3],
            t0_vault: pubkeys[4],
            t0_user: pubkeys[5],
            t1_vault: pubkeys[6],
            t1_user: pubkeys[7],
            lp_token_mint: pubkeys[8],
            lp_token_user: pubkeys[9],
            lp_token_vault: pubkeys[10],
            master: pubkeys[11],
            signer: pubkeys[12],
            system_program: pubkeys[13],
            rent: pubkeys[14],
            token_program: pubkeys[15],
            logger: pubkeys[16],
        }
    }
}
impl<'info> From<RemoveLiquidityAccounts<'_, 'info>>
for [AccountInfo<'info>; REMOVE_LIQUIDITY_IX_ACCOUNTS_LEN] {
    fn from(accounts: RemoveLiquidityAccounts<'_, 'info>) -> Self {
        [
            accounts.pair.clone(),
            accounts.market.clone(),
            accounts.t0_mint.clone(),
            accounts.t1_mint.clone(),
            accounts.t0_vault.clone(),
            accounts.t0_user.clone(),
            accounts.t1_vault.clone(),
            accounts.t1_user.clone(),
            accounts.lp_token_mint.clone(),
            accounts.lp_token_user.clone(),
            accounts.lp_token_vault.clone(),
            accounts.master.clone(),
            accounts.signer.clone(),
            accounts.system_program.clone(),
            accounts.rent.clone(),
            accounts.token_program.clone(),
            accounts.logger.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; REMOVE_LIQUIDITY_IX_ACCOUNTS_LEN]>
for RemoveLiquidityAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; REMOVE_LIQUIDITY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            pair: &arr[0],
            market: &arr[1],
            t0_mint: &arr[2],
            t1_mint: &arr[3],
            t0_vault: &arr[4],
            t0_user: &arr[5],
            t1_vault: &arr[6],
            t1_user: &arr[7],
            lp_token_mint: &arr[8],
            lp_token_user: &arr[9],
            lp_token_vault: &arr[10],
            master: &arr[11],
            signer: &arr[12],
            system_program: &arr[13],
            rent: &arr[14],
            token_program: &arr[15],
            logger: &arr[16],
        }
    }
}
pub const REMOVE_LIQUIDITY_IX_DISCM: [u8; 8usize] = [80, 85, 209, 72, 24, 206, 177, 108];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct RemoveLiquidityIxArgs {
    pub amount: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct RemoveLiquidityIxData(pub RemoveLiquidityIxArgs);
impl From<RemoveLiquidityIxArgs> for RemoveLiquidityIxData {
    fn from(args: RemoveLiquidityIxArgs) -> Self {
        Self(args)
    }
}
impl RemoveLiquidityIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != REMOVE_LIQUIDITY_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(Self(RemoveLiquidityIxArgs { amount }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&REMOVE_LIQUIDITY_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.amount, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn remove_liquidity_ix_with_program_id(
    program_id: Pubkey,
    keys: RemoveLiquidityKeys,
    args: RemoveLiquidityIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; REMOVE_LIQUIDITY_IX_ACCOUNTS_LEN] = keys.into();
    let data: RemoveLiquidityIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn remove_liquidity_ix(
    keys: RemoveLiquidityKeys,
    args: RemoveLiquidityIxArgs,
) -> std::io::Result<Instruction> {
    remove_liquidity_ix_with_program_id(DRADEX_PROGRAM_ID, keys, args)
}
pub fn remove_liquidity_invoke_with_program_id(
    program_id: Pubkey,
    accounts: RemoveLiquidityAccounts<'_, '_>,
    args: RemoveLiquidityIxArgs,
) -> ProgramResult {
    let keys: RemoveLiquidityKeys = accounts.into();
    let ix = remove_liquidity_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn remove_liquidity_invoke(
    accounts: RemoveLiquidityAccounts<'_, '_>,
    args: RemoveLiquidityIxArgs,
) -> ProgramResult {
    remove_liquidity_invoke_with_program_id(DRADEX_PROGRAM_ID, accounts, args)
}
pub fn remove_liquidity_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: RemoveLiquidityAccounts<'_, '_>,
    args: RemoveLiquidityIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: RemoveLiquidityKeys = accounts.into();
    let ix = remove_liquidity_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn remove_liquidity_invoke_signed(
    accounts: RemoveLiquidityAccounts<'_, '_>,
    args: RemoveLiquidityIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    remove_liquidity_invoke_signed_with_program_id(
        DRADEX_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn remove_liquidity_verify_account_keys(
    accounts: RemoveLiquidityAccounts<'_, '_>,
    keys: RemoveLiquidityKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.pair.key, keys.pair),
        (*accounts.market.key, keys.market),
        (*accounts.t0_mint.key, keys.t0_mint),
        (*accounts.t1_mint.key, keys.t1_mint),
        (*accounts.t0_vault.key, keys.t0_vault),
        (*accounts.t0_user.key, keys.t0_user),
        (*accounts.t1_vault.key, keys.t1_vault),
        (*accounts.t1_user.key, keys.t1_user),
        (*accounts.lp_token_mint.key, keys.lp_token_mint),
        (*accounts.lp_token_user.key, keys.lp_token_user),
        (*accounts.lp_token_vault.key, keys.lp_token_vault),
        (*accounts.master.key, keys.master),
        (*accounts.signer.key, keys.signer),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.rent.key, keys.rent),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.logger.key, keys.logger),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn remove_liquidity_verify_writable_privileges<'me, 'info>(
    accounts: RemoveLiquidityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.market,
        accounts.t0_mint,
        accounts.t1_mint,
        accounts.t0_vault,
        accounts.t0_user,
        accounts.t1_vault,
        accounts.t1_user,
        accounts.lp_token_mint,
        accounts.lp_token_user,
        accounts.lp_token_vault,
        accounts.master,
        accounts.signer,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn remove_liquidity_verify_signer_privileges<'me, 'info>(
    accounts: RemoveLiquidityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.signer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn remove_liquidity_verify_account_privileges<'me, 'info>(
    accounts: RemoveLiquidityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    remove_liquidity_verify_writable_privileges(accounts)?;
    remove_liquidity_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CREATE_ORDER_IX_ACCOUNTS_LEN: usize = 16;
#[derive(Copy, Clone, Debug)]
pub struct CreateOrderAccounts<'me, 'info> {
    pub pair: &'me AccountInfo<'info>,
    pub market: &'me AccountInfo<'info>,
    pub event_queue: &'me AccountInfo<'info>,
    pub dex_user: &'me AccountInfo<'info>,
    pub market_user: &'me AccountInfo<'info>,
    pub bids: &'me AccountInfo<'info>,
    pub asks: &'me AccountInfo<'info>,
    pub t0_vault: &'me AccountInfo<'info>,
    pub t1_vault: &'me AccountInfo<'info>,
    pub t0_user: &'me AccountInfo<'info>,
    pub t1_user: &'me AccountInfo<'info>,
    pub master: &'me AccountInfo<'info>,
    pub signer: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub logger: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CreateOrderKeys {
    pub pair: Pubkey,
    pub market: Pubkey,
    pub event_queue: Pubkey,
    pub dex_user: Pubkey,
    pub market_user: Pubkey,
    pub bids: Pubkey,
    pub asks: Pubkey,
    pub t0_vault: Pubkey,
    pub t1_vault: Pubkey,
    pub t0_user: Pubkey,
    pub t1_user: Pubkey,
    pub master: Pubkey,
    pub signer: Pubkey,
    pub system_program: Pubkey,
    pub token_program: Pubkey,
    pub logger: Pubkey,
}
impl From<CreateOrderAccounts<'_, '_>> for CreateOrderKeys {
    fn from(accounts: CreateOrderAccounts) -> Self {
        Self {
            pair: *accounts.pair.key,
            market: *accounts.market.key,
            event_queue: *accounts.event_queue.key,
            dex_user: *accounts.dex_user.key,
            market_user: *accounts.market_user.key,
            bids: *accounts.bids.key,
            asks: *accounts.asks.key,
            t0_vault: *accounts.t0_vault.key,
            t1_vault: *accounts.t1_vault.key,
            t0_user: *accounts.t0_user.key,
            t1_user: *accounts.t1_user.key,
            master: *accounts.master.key,
            signer: *accounts.signer.key,
            system_program: *accounts.system_program.key,
            token_program: *accounts.token_program.key,
            logger: *accounts.logger.key,
        }
    }
}
impl From<CreateOrderKeys> for [AccountMeta; CREATE_ORDER_IX_ACCOUNTS_LEN] {
    fn from(keys: CreateOrderKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.pair,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.market,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.event_queue,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.dex_user,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.market_user,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.bids,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.asks,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.t0_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.t1_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.t0_user,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.t1_user,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.master,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.signer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.system_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.logger,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; CREATE_ORDER_IX_ACCOUNTS_LEN]> for CreateOrderKeys {
    fn from(pubkeys: [Pubkey; CREATE_ORDER_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            pair: pubkeys[0],
            market: pubkeys[1],
            event_queue: pubkeys[2],
            dex_user: pubkeys[3],
            market_user: pubkeys[4],
            bids: pubkeys[5],
            asks: pubkeys[6],
            t0_vault: pubkeys[7],
            t1_vault: pubkeys[8],
            t0_user: pubkeys[9],
            t1_user: pubkeys[10],
            master: pubkeys[11],
            signer: pubkeys[12],
            system_program: pubkeys[13],
            token_program: pubkeys[14],
            logger: pubkeys[15],
        }
    }
}
impl<'info> From<CreateOrderAccounts<'_, 'info>>
for [AccountInfo<'info>; CREATE_ORDER_IX_ACCOUNTS_LEN] {
    fn from(accounts: CreateOrderAccounts<'_, 'info>) -> Self {
        [
            accounts.pair.clone(),
            accounts.market.clone(),
            accounts.event_queue.clone(),
            accounts.dex_user.clone(),
            accounts.market_user.clone(),
            accounts.bids.clone(),
            accounts.asks.clone(),
            accounts.t0_vault.clone(),
            accounts.t1_vault.clone(),
            accounts.t0_user.clone(),
            accounts.t1_user.clone(),
            accounts.master.clone(),
            accounts.signer.clone(),
            accounts.system_program.clone(),
            accounts.token_program.clone(),
            accounts.logger.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CREATE_ORDER_IX_ACCOUNTS_LEN]>
for CreateOrderAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; CREATE_ORDER_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            pair: &arr[0],
            market: &arr[1],
            event_queue: &arr[2],
            dex_user: &arr[3],
            market_user: &arr[4],
            bids: &arr[5],
            asks: &arr[6],
            t0_vault: &arr[7],
            t1_vault: &arr[8],
            t0_user: &arr[9],
            t1_user: &arr[10],
            master: &arr[11],
            signer: &arr[12],
            system_program: &arr[13],
            token_program: &arr[14],
            logger: &arr[15],
        }
    }
}
pub const CREATE_ORDER_IX_DISCM: [u8; 8usize] = [141, 54, 37, 207, 237, 210, 250, 215];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CreateOrderIxArgs {
    pub input: OrderInput,
}
#[derive(Clone, Debug, PartialEq)]
pub struct CreateOrderIxData(pub CreateOrderIxArgs);
impl From<CreateOrderIxArgs> for CreateOrderIxData {
    fn from(args: CreateOrderIxArgs) -> Self {
        Self(args)
    }
}
impl CreateOrderIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CREATE_ORDER_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let input = if reader.is_empty() {
            Default::default()
        } else {
            <OrderInput>::deserialize(&mut reader)?
        };
        Ok(Self(CreateOrderIxArgs { input }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CREATE_ORDER_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.input, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn create_order_ix_with_program_id(
    program_id: Pubkey,
    keys: CreateOrderKeys,
    args: CreateOrderIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CREATE_ORDER_IX_ACCOUNTS_LEN] = keys.into();
    let data: CreateOrderIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn create_order_ix(
    keys: CreateOrderKeys,
    args: CreateOrderIxArgs,
) -> std::io::Result<Instruction> {
    create_order_ix_with_program_id(DRADEX_PROGRAM_ID, keys, args)
}
pub fn create_order_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CreateOrderAccounts<'_, '_>,
    args: CreateOrderIxArgs,
) -> ProgramResult {
    let keys: CreateOrderKeys = accounts.into();
    let ix = create_order_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn create_order_invoke(
    accounts: CreateOrderAccounts<'_, '_>,
    args: CreateOrderIxArgs,
) -> ProgramResult {
    create_order_invoke_with_program_id(DRADEX_PROGRAM_ID, accounts, args)
}
pub fn create_order_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CreateOrderAccounts<'_, '_>,
    args: CreateOrderIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CreateOrderKeys = accounts.into();
    let ix = create_order_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn create_order_invoke_signed(
    accounts: CreateOrderAccounts<'_, '_>,
    args: CreateOrderIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    create_order_invoke_signed_with_program_id(DRADEX_PROGRAM_ID, accounts, args, seeds)
}
pub fn create_order_verify_account_keys(
    accounts: CreateOrderAccounts<'_, '_>,
    keys: CreateOrderKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.pair.key, keys.pair),
        (*accounts.market.key, keys.market),
        (*accounts.event_queue.key, keys.event_queue),
        (*accounts.dex_user.key, keys.dex_user),
        (*accounts.market_user.key, keys.market_user),
        (*accounts.bids.key, keys.bids),
        (*accounts.asks.key, keys.asks),
        (*accounts.t0_vault.key, keys.t0_vault),
        (*accounts.t1_vault.key, keys.t1_vault),
        (*accounts.t0_user.key, keys.t0_user),
        (*accounts.t1_user.key, keys.t1_user),
        (*accounts.master.key, keys.master),
        (*accounts.signer.key, keys.signer),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.logger.key, keys.logger),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn create_order_verify_writable_privileges<'me, 'info>(
    accounts: CreateOrderAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.pair,
        accounts.market,
        accounts.event_queue,
        accounts.market_user,
        accounts.bids,
        accounts.asks,
        accounts.t0_vault,
        accounts.t1_vault,
        accounts.t0_user,
        accounts.t1_user,
        accounts.signer,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn create_order_verify_signer_privileges<'me, 'info>(
    accounts: CreateOrderAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.signer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn create_order_verify_account_privileges<'me, 'info>(
    accounts: CreateOrderAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    create_order_verify_writable_privileges(accounts)?;
    create_order_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SETTLE_FUNDS_IX_ACCOUNTS_LEN: usize = 18;
#[derive(Copy, Clone, Debug)]
pub struct SettleFundsAccounts<'me, 'info> {
    pub pair: &'me AccountInfo<'info>,
    pub market: &'me AccountInfo<'info>,
    pub event_queue: &'me AccountInfo<'info>,
    pub dex_user: &'me AccountInfo<'info>,
    pub market_user: &'me AccountInfo<'info>,
    pub t0_mint: &'me AccountInfo<'info>,
    pub t1_mint: &'me AccountInfo<'info>,
    pub bids: &'me AccountInfo<'info>,
    pub asks: &'me AccountInfo<'info>,
    pub t0_vault: &'me AccountInfo<'info>,
    pub t1_vault: &'me AccountInfo<'info>,
    pub t0_user: &'me AccountInfo<'info>,
    pub t1_user: &'me AccountInfo<'info>,
    pub master: &'me AccountInfo<'info>,
    pub signer: &'me AccountInfo<'info>,
    pub rent: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub logger: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SettleFundsKeys {
    pub pair: Pubkey,
    pub market: Pubkey,
    pub event_queue: Pubkey,
    pub dex_user: Pubkey,
    pub market_user: Pubkey,
    pub t0_mint: Pubkey,
    pub t1_mint: Pubkey,
    pub bids: Pubkey,
    pub asks: Pubkey,
    pub t0_vault: Pubkey,
    pub t1_vault: Pubkey,
    pub t0_user: Pubkey,
    pub t1_user: Pubkey,
    pub master: Pubkey,
    pub signer: Pubkey,
    pub rent: Pubkey,
    pub token_program: Pubkey,
    pub logger: Pubkey,
}
impl From<SettleFundsAccounts<'_, '_>> for SettleFundsKeys {
    fn from(accounts: SettleFundsAccounts) -> Self {
        Self {
            pair: *accounts.pair.key,
            market: *accounts.market.key,
            event_queue: *accounts.event_queue.key,
            dex_user: *accounts.dex_user.key,
            market_user: *accounts.market_user.key,
            t0_mint: *accounts.t0_mint.key,
            t1_mint: *accounts.t1_mint.key,
            bids: *accounts.bids.key,
            asks: *accounts.asks.key,
            t0_vault: *accounts.t0_vault.key,
            t1_vault: *accounts.t1_vault.key,
            t0_user: *accounts.t0_user.key,
            t1_user: *accounts.t1_user.key,
            master: *accounts.master.key,
            signer: *accounts.signer.key,
            rent: *accounts.rent.key,
            token_program: *accounts.token_program.key,
            logger: *accounts.logger.key,
        }
    }
}
impl From<SettleFundsKeys> for [AccountMeta; SETTLE_FUNDS_IX_ACCOUNTS_LEN] {
    fn from(keys: SettleFundsKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.pair,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.market,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.event_queue,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.dex_user,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.market_user,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.t0_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.t1_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.bids,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.asks,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.t0_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.t1_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.t0_user,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.t1_user,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.master,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.signer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.rent,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.logger,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; SETTLE_FUNDS_IX_ACCOUNTS_LEN]> for SettleFundsKeys {
    fn from(pubkeys: [Pubkey; SETTLE_FUNDS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            pair: pubkeys[0],
            market: pubkeys[1],
            event_queue: pubkeys[2],
            dex_user: pubkeys[3],
            market_user: pubkeys[4],
            t0_mint: pubkeys[5],
            t1_mint: pubkeys[6],
            bids: pubkeys[7],
            asks: pubkeys[8],
            t0_vault: pubkeys[9],
            t1_vault: pubkeys[10],
            t0_user: pubkeys[11],
            t1_user: pubkeys[12],
            master: pubkeys[13],
            signer: pubkeys[14],
            rent: pubkeys[15],
            token_program: pubkeys[16],
            logger: pubkeys[17],
        }
    }
}
impl<'info> From<SettleFundsAccounts<'_, 'info>>
for [AccountInfo<'info>; SETTLE_FUNDS_IX_ACCOUNTS_LEN] {
    fn from(accounts: SettleFundsAccounts<'_, 'info>) -> Self {
        [
            accounts.pair.clone(),
            accounts.market.clone(),
            accounts.event_queue.clone(),
            accounts.dex_user.clone(),
            accounts.market_user.clone(),
            accounts.t0_mint.clone(),
            accounts.t1_mint.clone(),
            accounts.bids.clone(),
            accounts.asks.clone(),
            accounts.t0_vault.clone(),
            accounts.t1_vault.clone(),
            accounts.t0_user.clone(),
            accounts.t1_user.clone(),
            accounts.master.clone(),
            accounts.signer.clone(),
            accounts.rent.clone(),
            accounts.token_program.clone(),
            accounts.logger.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SETTLE_FUNDS_IX_ACCOUNTS_LEN]>
for SettleFundsAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; SETTLE_FUNDS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            pair: &arr[0],
            market: &arr[1],
            event_queue: &arr[2],
            dex_user: &arr[3],
            market_user: &arr[4],
            t0_mint: &arr[5],
            t1_mint: &arr[6],
            bids: &arr[7],
            asks: &arr[8],
            t0_vault: &arr[9],
            t1_vault: &arr[10],
            t0_user: &arr[11],
            t1_user: &arr[12],
            master: &arr[13],
            signer: &arr[14],
            rent: &arr[15],
            token_program: &arr[16],
            logger: &arr[17],
        }
    }
}
pub const SETTLE_FUNDS_IX_DISCM: [u8; 8usize] = [238, 64, 163, 96, 75, 171, 16, 33];
#[derive(Clone, Debug, PartialEq)]
pub struct SettleFundsIxData;
impl SettleFundsIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SETTLE_FUNDS_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SETTLE_FUNDS_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn settle_funds_ix_with_program_id(
    program_id: Pubkey,
    keys: SettleFundsKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SETTLE_FUNDS_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: SettleFundsIxData.try_to_vec()?,
    })
}
pub fn settle_funds_ix(keys: SettleFundsKeys) -> std::io::Result<Instruction> {
    settle_funds_ix_with_program_id(DRADEX_PROGRAM_ID, keys)
}
pub fn settle_funds_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SettleFundsAccounts<'_, '_>,
) -> ProgramResult {
    let keys: SettleFundsKeys = accounts.into();
    let ix = settle_funds_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn settle_funds_invoke(accounts: SettleFundsAccounts<'_, '_>) -> ProgramResult {
    settle_funds_invoke_with_program_id(DRADEX_PROGRAM_ID, accounts)
}
pub fn settle_funds_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SettleFundsAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SettleFundsKeys = accounts.into();
    let ix = settle_funds_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn settle_funds_invoke_signed(
    accounts: SettleFundsAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    settle_funds_invoke_signed_with_program_id(DRADEX_PROGRAM_ID, accounts, seeds)
}
pub fn settle_funds_verify_account_keys(
    accounts: SettleFundsAccounts<'_, '_>,
    keys: SettleFundsKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.pair.key, keys.pair),
        (*accounts.market.key, keys.market),
        (*accounts.event_queue.key, keys.event_queue),
        (*accounts.dex_user.key, keys.dex_user),
        (*accounts.market_user.key, keys.market_user),
        (*accounts.t0_mint.key, keys.t0_mint),
        (*accounts.t1_mint.key, keys.t1_mint),
        (*accounts.bids.key, keys.bids),
        (*accounts.asks.key, keys.asks),
        (*accounts.t0_vault.key, keys.t0_vault),
        (*accounts.t1_vault.key, keys.t1_vault),
        (*accounts.t0_user.key, keys.t0_user),
        (*accounts.t1_user.key, keys.t1_user),
        (*accounts.master.key, keys.master),
        (*accounts.signer.key, keys.signer),
        (*accounts.rent.key, keys.rent),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.logger.key, keys.logger),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn settle_funds_verify_writable_privileges<'me, 'info>(
    accounts: SettleFundsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.pair,
        accounts.market,
        accounts.event_queue,
        accounts.market_user,
        accounts.bids,
        accounts.asks,
        accounts.t0_vault,
        accounts.t1_vault,
        accounts.t0_user,
        accounts.t1_user,
        accounts.signer,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn settle_funds_verify_signer_privileges<'me, 'info>(
    accounts: SettleFundsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.signer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn settle_funds_verify_account_privileges<'me, 'info>(
    accounts: SettleFundsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    settle_funds_verify_writable_privileges(accounts)?;
    settle_funds_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CANCEL_ORDER_IX_ACCOUNTS_LEN: usize = 16;
#[derive(Copy, Clone, Debug)]
pub struct CancelOrderAccounts<'me, 'info> {
    pub pair: &'me AccountInfo<'info>,
    pub market: &'me AccountInfo<'info>,
    pub event_queue: &'me AccountInfo<'info>,
    pub dex_user: &'me AccountInfo<'info>,
    pub market_user: &'me AccountInfo<'info>,
    pub bids: &'me AccountInfo<'info>,
    pub asks: &'me AccountInfo<'info>,
    pub t0_vault: &'me AccountInfo<'info>,
    pub t1_vault: &'me AccountInfo<'info>,
    pub t0_user: &'me AccountInfo<'info>,
    pub t1_user: &'me AccountInfo<'info>,
    pub master: &'me AccountInfo<'info>,
    pub signer: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub logger: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CancelOrderKeys {
    pub pair: Pubkey,
    pub market: Pubkey,
    pub event_queue: Pubkey,
    pub dex_user: Pubkey,
    pub market_user: Pubkey,
    pub bids: Pubkey,
    pub asks: Pubkey,
    pub t0_vault: Pubkey,
    pub t1_vault: Pubkey,
    pub t0_user: Pubkey,
    pub t1_user: Pubkey,
    pub master: Pubkey,
    pub signer: Pubkey,
    pub system_program: Pubkey,
    pub token_program: Pubkey,
    pub logger: Pubkey,
}
impl From<CancelOrderAccounts<'_, '_>> for CancelOrderKeys {
    fn from(accounts: CancelOrderAccounts) -> Self {
        Self {
            pair: *accounts.pair.key,
            market: *accounts.market.key,
            event_queue: *accounts.event_queue.key,
            dex_user: *accounts.dex_user.key,
            market_user: *accounts.market_user.key,
            bids: *accounts.bids.key,
            asks: *accounts.asks.key,
            t0_vault: *accounts.t0_vault.key,
            t1_vault: *accounts.t1_vault.key,
            t0_user: *accounts.t0_user.key,
            t1_user: *accounts.t1_user.key,
            master: *accounts.master.key,
            signer: *accounts.signer.key,
            system_program: *accounts.system_program.key,
            token_program: *accounts.token_program.key,
            logger: *accounts.logger.key,
        }
    }
}
impl From<CancelOrderKeys> for [AccountMeta; CANCEL_ORDER_IX_ACCOUNTS_LEN] {
    fn from(keys: CancelOrderKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.pair,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.market,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.event_queue,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.dex_user,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.market_user,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.bids,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.asks,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.t0_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.t1_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.t0_user,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.t1_user,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.master,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.signer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.system_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.logger,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; CANCEL_ORDER_IX_ACCOUNTS_LEN]> for CancelOrderKeys {
    fn from(pubkeys: [Pubkey; CANCEL_ORDER_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            pair: pubkeys[0],
            market: pubkeys[1],
            event_queue: pubkeys[2],
            dex_user: pubkeys[3],
            market_user: pubkeys[4],
            bids: pubkeys[5],
            asks: pubkeys[6],
            t0_vault: pubkeys[7],
            t1_vault: pubkeys[8],
            t0_user: pubkeys[9],
            t1_user: pubkeys[10],
            master: pubkeys[11],
            signer: pubkeys[12],
            system_program: pubkeys[13],
            token_program: pubkeys[14],
            logger: pubkeys[15],
        }
    }
}
impl<'info> From<CancelOrderAccounts<'_, 'info>>
for [AccountInfo<'info>; CANCEL_ORDER_IX_ACCOUNTS_LEN] {
    fn from(accounts: CancelOrderAccounts<'_, 'info>) -> Self {
        [
            accounts.pair.clone(),
            accounts.market.clone(),
            accounts.event_queue.clone(),
            accounts.dex_user.clone(),
            accounts.market_user.clone(),
            accounts.bids.clone(),
            accounts.asks.clone(),
            accounts.t0_vault.clone(),
            accounts.t1_vault.clone(),
            accounts.t0_user.clone(),
            accounts.t1_user.clone(),
            accounts.master.clone(),
            accounts.signer.clone(),
            accounts.system_program.clone(),
            accounts.token_program.clone(),
            accounts.logger.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CANCEL_ORDER_IX_ACCOUNTS_LEN]>
for CancelOrderAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; CANCEL_ORDER_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            pair: &arr[0],
            market: &arr[1],
            event_queue: &arr[2],
            dex_user: &arr[3],
            market_user: &arr[4],
            bids: &arr[5],
            asks: &arr[6],
            t0_vault: &arr[7],
            t1_vault: &arr[8],
            t0_user: &arr[9],
            t1_user: &arr[10],
            master: &arr[11],
            signer: &arr[12],
            system_program: &arr[13],
            token_program: &arr[14],
            logger: &arr[15],
        }
    }
}
pub const CANCEL_ORDER_IX_DISCM: [u8; 8usize] = [95, 129, 237, 240, 8, 49, 223, 132];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CancelOrderIxArgs {
    pub input: CancelOrderInput,
}
#[derive(Clone, Debug, PartialEq)]
pub struct CancelOrderIxData(pub CancelOrderIxArgs);
impl From<CancelOrderIxArgs> for CancelOrderIxData {
    fn from(args: CancelOrderIxArgs) -> Self {
        Self(args)
    }
}
impl CancelOrderIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CANCEL_ORDER_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let input = if reader.is_empty() {
            Default::default()
        } else {
            <CancelOrderInput>::deserialize(&mut reader)?
        };
        Ok(Self(CancelOrderIxArgs { input }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CANCEL_ORDER_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.input, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn cancel_order_ix_with_program_id(
    program_id: Pubkey,
    keys: CancelOrderKeys,
    args: CancelOrderIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CANCEL_ORDER_IX_ACCOUNTS_LEN] = keys.into();
    let data: CancelOrderIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn cancel_order_ix(
    keys: CancelOrderKeys,
    args: CancelOrderIxArgs,
) -> std::io::Result<Instruction> {
    cancel_order_ix_with_program_id(DRADEX_PROGRAM_ID, keys, args)
}
pub fn cancel_order_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CancelOrderAccounts<'_, '_>,
    args: CancelOrderIxArgs,
) -> ProgramResult {
    let keys: CancelOrderKeys = accounts.into();
    let ix = cancel_order_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn cancel_order_invoke(
    accounts: CancelOrderAccounts<'_, '_>,
    args: CancelOrderIxArgs,
) -> ProgramResult {
    cancel_order_invoke_with_program_id(DRADEX_PROGRAM_ID, accounts, args)
}
pub fn cancel_order_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CancelOrderAccounts<'_, '_>,
    args: CancelOrderIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CancelOrderKeys = accounts.into();
    let ix = cancel_order_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn cancel_order_invoke_signed(
    accounts: CancelOrderAccounts<'_, '_>,
    args: CancelOrderIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    cancel_order_invoke_signed_with_program_id(DRADEX_PROGRAM_ID, accounts, args, seeds)
}
pub fn cancel_order_verify_account_keys(
    accounts: CancelOrderAccounts<'_, '_>,
    keys: CancelOrderKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.pair.key, keys.pair),
        (*accounts.market.key, keys.market),
        (*accounts.event_queue.key, keys.event_queue),
        (*accounts.dex_user.key, keys.dex_user),
        (*accounts.market_user.key, keys.market_user),
        (*accounts.bids.key, keys.bids),
        (*accounts.asks.key, keys.asks),
        (*accounts.t0_vault.key, keys.t0_vault),
        (*accounts.t1_vault.key, keys.t1_vault),
        (*accounts.t0_user.key, keys.t0_user),
        (*accounts.t1_user.key, keys.t1_user),
        (*accounts.master.key, keys.master),
        (*accounts.signer.key, keys.signer),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.logger.key, keys.logger),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn cancel_order_verify_writable_privileges<'me, 'info>(
    accounts: CancelOrderAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.pair,
        accounts.market,
        accounts.event_queue,
        accounts.market_user,
        accounts.bids,
        accounts.asks,
        accounts.t0_vault,
        accounts.t1_vault,
        accounts.t0_user,
        accounts.t1_user,
        accounts.signer,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn cancel_order_verify_signer_privileges<'me, 'info>(
    accounts: CancelOrderAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.signer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn cancel_order_verify_account_privileges<'me, 'info>(
    accounts: CancelOrderAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    cancel_order_verify_writable_privileges(accounts)?;
    cancel_order_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const DAO_SET_FUND_MANAGER_IX_ACCOUNTS_LEN: usize = 6;
#[derive(Copy, Clone, Debug)]
pub struct DaoSetFundManagerAccounts<'me, 'info> {
    pub master: &'me AccountInfo<'info>,
    pub dao_config: &'me AccountInfo<'info>,
    pub fund_manager: &'me AccountInfo<'info>,
    pub signer: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct DaoSetFundManagerKeys {
    pub master: Pubkey,
    pub dao_config: Pubkey,
    pub fund_manager: Pubkey,
    pub signer: Pubkey,
    pub authority: Pubkey,
    pub system_program: Pubkey,
}
impl From<DaoSetFundManagerAccounts<'_, '_>> for DaoSetFundManagerKeys {
    fn from(accounts: DaoSetFundManagerAccounts) -> Self {
        Self {
            master: *accounts.master.key,
            dao_config: *accounts.dao_config.key,
            fund_manager: *accounts.fund_manager.key,
            signer: *accounts.signer.key,
            authority: *accounts.authority.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<DaoSetFundManagerKeys>
for [AccountMeta; DAO_SET_FUND_MANAGER_IX_ACCOUNTS_LEN] {
    fn from(keys: DaoSetFundManagerKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.master,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.dao_config,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.fund_manager,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.signer,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.authority,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.system_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; DAO_SET_FUND_MANAGER_IX_ACCOUNTS_LEN]> for DaoSetFundManagerKeys {
    fn from(pubkeys: [Pubkey; DAO_SET_FUND_MANAGER_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            master: pubkeys[0],
            dao_config: pubkeys[1],
            fund_manager: pubkeys[2],
            signer: pubkeys[3],
            authority: pubkeys[4],
            system_program: pubkeys[5],
        }
    }
}
impl<'info> From<DaoSetFundManagerAccounts<'_, 'info>>
for [AccountInfo<'info>; DAO_SET_FUND_MANAGER_IX_ACCOUNTS_LEN] {
    fn from(accounts: DaoSetFundManagerAccounts<'_, 'info>) -> Self {
        [
            accounts.master.clone(),
            accounts.dao_config.clone(),
            accounts.fund_manager.clone(),
            accounts.signer.clone(),
            accounts.authority.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; DAO_SET_FUND_MANAGER_IX_ACCOUNTS_LEN]>
for DaoSetFundManagerAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; DAO_SET_FUND_MANAGER_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            master: &arr[0],
            dao_config: &arr[1],
            fund_manager: &arr[2],
            signer: &arr[3],
            authority: &arr[4],
            system_program: &arr[5],
        }
    }
}
pub const DAO_SET_FUND_MANAGER_IX_DISCM: [u8; 8usize] = [
    176, 157, 0, 226, 74, 191, 133, 102,
];
#[derive(Clone, Debug, PartialEq)]
pub struct DaoSetFundManagerIxData;
impl DaoSetFundManagerIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != DAO_SET_FUND_MANAGER_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&DAO_SET_FUND_MANAGER_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn dao_set_fund_manager_ix_with_program_id(
    program_id: Pubkey,
    keys: DaoSetFundManagerKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; DAO_SET_FUND_MANAGER_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: DaoSetFundManagerIxData.try_to_vec()?,
    })
}
pub fn dao_set_fund_manager_ix(
    keys: DaoSetFundManagerKeys,
) -> std::io::Result<Instruction> {
    dao_set_fund_manager_ix_with_program_id(DRADEX_PROGRAM_ID, keys)
}
pub fn dao_set_fund_manager_invoke_with_program_id(
    program_id: Pubkey,
    accounts: DaoSetFundManagerAccounts<'_, '_>,
) -> ProgramResult {
    let keys: DaoSetFundManagerKeys = accounts.into();
    let ix = dao_set_fund_manager_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn dao_set_fund_manager_invoke(
    accounts: DaoSetFundManagerAccounts<'_, '_>,
) -> ProgramResult {
    dao_set_fund_manager_invoke_with_program_id(DRADEX_PROGRAM_ID, accounts)
}
pub fn dao_set_fund_manager_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: DaoSetFundManagerAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: DaoSetFundManagerKeys = accounts.into();
    let ix = dao_set_fund_manager_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn dao_set_fund_manager_invoke_signed(
    accounts: DaoSetFundManagerAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    dao_set_fund_manager_invoke_signed_with_program_id(
        DRADEX_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn dao_set_fund_manager_verify_account_keys(
    accounts: DaoSetFundManagerAccounts<'_, '_>,
    keys: DaoSetFundManagerKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.master.key, keys.master),
        (*accounts.dao_config.key, keys.dao_config),
        (*accounts.fund_manager.key, keys.fund_manager),
        (*accounts.signer.key, keys.signer),
        (*accounts.authority.key, keys.authority),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn dao_set_fund_manager_verify_writable_privileges<'me, 'info>(
    accounts: DaoSetFundManagerAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.master, accounts.dao_config] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn dao_set_fund_manager_verify_signer_privileges<'me, 'info>(
    accounts: DaoSetFundManagerAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.signer, accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn dao_set_fund_manager_verify_account_privileges<'me, 'info>(
    accounts: DaoSetFundManagerAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    dao_set_fund_manager_verify_writable_privileges(accounts)?;
    dao_set_fund_manager_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const DAO_CLAIM_REVENUE_IX_ACCOUNTS_LEN: usize = 10;
#[derive(Copy, Clone, Debug)]
pub struct DaoClaimRevenueAccounts<'me, 'info> {
    pub token_mint: &'me AccountInfo<'info>,
    pub market: &'me AccountInfo<'info>,
    pub market_vault: &'me AccountInfo<'info>,
    pub dao_config: &'me AccountInfo<'info>,
    pub token_user: &'me AccountInfo<'info>,
    pub master: &'me AccountInfo<'info>,
    pub signer: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub logger: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct DaoClaimRevenueKeys {
    pub token_mint: Pubkey,
    pub market: Pubkey,
    pub market_vault: Pubkey,
    pub dao_config: Pubkey,
    pub token_user: Pubkey,
    pub master: Pubkey,
    pub signer: Pubkey,
    pub system_program: Pubkey,
    pub token_program: Pubkey,
    pub logger: Pubkey,
}
impl From<DaoClaimRevenueAccounts<'_, '_>> for DaoClaimRevenueKeys {
    fn from(accounts: DaoClaimRevenueAccounts) -> Self {
        Self {
            token_mint: *accounts.token_mint.key,
            market: *accounts.market.key,
            market_vault: *accounts.market_vault.key,
            dao_config: *accounts.dao_config.key,
            token_user: *accounts.token_user.key,
            master: *accounts.master.key,
            signer: *accounts.signer.key,
            system_program: *accounts.system_program.key,
            token_program: *accounts.token_program.key,
            logger: *accounts.logger.key,
        }
    }
}
impl From<DaoClaimRevenueKeys> for [AccountMeta; DAO_CLAIM_REVENUE_IX_ACCOUNTS_LEN] {
    fn from(keys: DaoClaimRevenueKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.token_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.market,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.market_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.dao_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_user,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.master,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.signer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.system_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.logger,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; DAO_CLAIM_REVENUE_IX_ACCOUNTS_LEN]> for DaoClaimRevenueKeys {
    fn from(pubkeys: [Pubkey; DAO_CLAIM_REVENUE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            token_mint: pubkeys[0],
            market: pubkeys[1],
            market_vault: pubkeys[2],
            dao_config: pubkeys[3],
            token_user: pubkeys[4],
            master: pubkeys[5],
            signer: pubkeys[6],
            system_program: pubkeys[7],
            token_program: pubkeys[8],
            logger: pubkeys[9],
        }
    }
}
impl<'info> From<DaoClaimRevenueAccounts<'_, 'info>>
for [AccountInfo<'info>; DAO_CLAIM_REVENUE_IX_ACCOUNTS_LEN] {
    fn from(accounts: DaoClaimRevenueAccounts<'_, 'info>) -> Self {
        [
            accounts.token_mint.clone(),
            accounts.market.clone(),
            accounts.market_vault.clone(),
            accounts.dao_config.clone(),
            accounts.token_user.clone(),
            accounts.master.clone(),
            accounts.signer.clone(),
            accounts.system_program.clone(),
            accounts.token_program.clone(),
            accounts.logger.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; DAO_CLAIM_REVENUE_IX_ACCOUNTS_LEN]>
for DaoClaimRevenueAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; DAO_CLAIM_REVENUE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            token_mint: &arr[0],
            market: &arr[1],
            market_vault: &arr[2],
            dao_config: &arr[3],
            token_user: &arr[4],
            master: &arr[5],
            signer: &arr[6],
            system_program: &arr[7],
            token_program: &arr[8],
            logger: &arr[9],
        }
    }
}
pub const DAO_CLAIM_REVENUE_IX_DISCM: [u8; 8usize] = [
    180, 231, 204, 145, 15, 153, 189, 58,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DaoClaimRevenueIxArgs {
    pub token_index: u8,
}
#[derive(Clone, Debug, PartialEq)]
pub struct DaoClaimRevenueIxData(pub DaoClaimRevenueIxArgs);
impl From<DaoClaimRevenueIxArgs> for DaoClaimRevenueIxData {
    fn from(args: DaoClaimRevenueIxArgs) -> Self {
        Self(args)
    }
}
impl DaoClaimRevenueIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != DAO_CLAIM_REVENUE_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let token_index: u8 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(DaoClaimRevenueIxArgs {
                token_index,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&DAO_CLAIM_REVENUE_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.token_index, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn dao_claim_revenue_ix_with_program_id(
    program_id: Pubkey,
    keys: DaoClaimRevenueKeys,
    args: DaoClaimRevenueIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; DAO_CLAIM_REVENUE_IX_ACCOUNTS_LEN] = keys.into();
    let data: DaoClaimRevenueIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn dao_claim_revenue_ix(
    keys: DaoClaimRevenueKeys,
    args: DaoClaimRevenueIxArgs,
) -> std::io::Result<Instruction> {
    dao_claim_revenue_ix_with_program_id(DRADEX_PROGRAM_ID, keys, args)
}
pub fn dao_claim_revenue_invoke_with_program_id(
    program_id: Pubkey,
    accounts: DaoClaimRevenueAccounts<'_, '_>,
    args: DaoClaimRevenueIxArgs,
) -> ProgramResult {
    let keys: DaoClaimRevenueKeys = accounts.into();
    let ix = dao_claim_revenue_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn dao_claim_revenue_invoke(
    accounts: DaoClaimRevenueAccounts<'_, '_>,
    args: DaoClaimRevenueIxArgs,
) -> ProgramResult {
    dao_claim_revenue_invoke_with_program_id(DRADEX_PROGRAM_ID, accounts, args)
}
pub fn dao_claim_revenue_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: DaoClaimRevenueAccounts<'_, '_>,
    args: DaoClaimRevenueIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: DaoClaimRevenueKeys = accounts.into();
    let ix = dao_claim_revenue_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn dao_claim_revenue_invoke_signed(
    accounts: DaoClaimRevenueAccounts<'_, '_>,
    args: DaoClaimRevenueIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    dao_claim_revenue_invoke_signed_with_program_id(
        DRADEX_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn dao_claim_revenue_verify_account_keys(
    accounts: DaoClaimRevenueAccounts<'_, '_>,
    keys: DaoClaimRevenueKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.token_mint.key, keys.token_mint),
        (*accounts.market.key, keys.market),
        (*accounts.market_vault.key, keys.market_vault),
        (*accounts.dao_config.key, keys.dao_config),
        (*accounts.token_user.key, keys.token_user),
        (*accounts.master.key, keys.master),
        (*accounts.signer.key, keys.signer),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.logger.key, keys.logger),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn dao_claim_revenue_verify_writable_privileges<'me, 'info>(
    accounts: DaoClaimRevenueAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.market_vault,
        accounts.token_user,
        accounts.master,
        accounts.signer,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn dao_claim_revenue_verify_signer_privileges<'me, 'info>(
    accounts: DaoClaimRevenueAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.signer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn dao_claim_revenue_verify_account_privileges<'me, 'info>(
    accounts: DaoClaimRevenueAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    dao_claim_revenue_verify_writable_privileges(accounts)?;
    dao_claim_revenue_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CONSUME_EVENTS_IX_ACCOUNTS_LEN: usize = 6;
#[derive(Copy, Clone, Debug)]
pub struct ConsumeEventsAccounts<'me, 'info> {
    pub pair: &'me AccountInfo<'info>,
    pub market: &'me AccountInfo<'info>,
    pub event_queue: &'me AccountInfo<'info>,
    pub signer: &'me AccountInfo<'info>,
    pub master: &'me AccountInfo<'info>,
    pub logger: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ConsumeEventsKeys {
    pub pair: Pubkey,
    pub market: Pubkey,
    pub event_queue: Pubkey,
    pub signer: Pubkey,
    pub master: Pubkey,
    pub logger: Pubkey,
}
impl From<ConsumeEventsAccounts<'_, '_>> for ConsumeEventsKeys {
    fn from(accounts: ConsumeEventsAccounts) -> Self {
        Self {
            pair: *accounts.pair.key,
            market: *accounts.market.key,
            event_queue: *accounts.event_queue.key,
            signer: *accounts.signer.key,
            master: *accounts.master.key,
            logger: *accounts.logger.key,
        }
    }
}
impl From<ConsumeEventsKeys> for [AccountMeta; CONSUME_EVENTS_IX_ACCOUNTS_LEN] {
    fn from(keys: ConsumeEventsKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.pair,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.market,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.event_queue,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.signer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.master,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.logger,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; CONSUME_EVENTS_IX_ACCOUNTS_LEN]> for ConsumeEventsKeys {
    fn from(pubkeys: [Pubkey; CONSUME_EVENTS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            pair: pubkeys[0],
            market: pubkeys[1],
            event_queue: pubkeys[2],
            signer: pubkeys[3],
            master: pubkeys[4],
            logger: pubkeys[5],
        }
    }
}
impl<'info> From<ConsumeEventsAccounts<'_, 'info>>
for [AccountInfo<'info>; CONSUME_EVENTS_IX_ACCOUNTS_LEN] {
    fn from(accounts: ConsumeEventsAccounts<'_, 'info>) -> Self {
        [
            accounts.pair.clone(),
            accounts.market.clone(),
            accounts.event_queue.clone(),
            accounts.signer.clone(),
            accounts.master.clone(),
            accounts.logger.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CONSUME_EVENTS_IX_ACCOUNTS_LEN]>
for ConsumeEventsAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; CONSUME_EVENTS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            pair: &arr[0],
            market: &arr[1],
            event_queue: &arr[2],
            signer: &arr[3],
            master: &arr[4],
            logger: &arr[5],
        }
    }
}
pub const CONSUME_EVENTS_IX_DISCM: [u8; 8usize] = [221, 145, 177, 52, 31, 47, 63, 201];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ConsumeEventsIxArgs {
    pub input: ConsumeEventsInput,
}
#[derive(Clone, Debug, PartialEq)]
pub struct ConsumeEventsIxData(pub ConsumeEventsIxArgs);
impl From<ConsumeEventsIxArgs> for ConsumeEventsIxData {
    fn from(args: ConsumeEventsIxArgs) -> Self {
        Self(args)
    }
}
impl ConsumeEventsIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CONSUME_EVENTS_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let input = if reader.is_empty() {
            Default::default()
        } else {
            <ConsumeEventsInput>::deserialize(&mut reader)?
        };
        Ok(Self(ConsumeEventsIxArgs { input }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CONSUME_EVENTS_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.input, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn consume_events_ix_with_program_id(
    program_id: Pubkey,
    keys: ConsumeEventsKeys,
    args: ConsumeEventsIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CONSUME_EVENTS_IX_ACCOUNTS_LEN] = keys.into();
    let data: ConsumeEventsIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn consume_events_ix(
    keys: ConsumeEventsKeys,
    args: ConsumeEventsIxArgs,
) -> std::io::Result<Instruction> {
    consume_events_ix_with_program_id(DRADEX_PROGRAM_ID, keys, args)
}
pub fn consume_events_invoke_with_program_id(
    program_id: Pubkey,
    accounts: ConsumeEventsAccounts<'_, '_>,
    args: ConsumeEventsIxArgs,
) -> ProgramResult {
    let keys: ConsumeEventsKeys = accounts.into();
    let ix = consume_events_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn consume_events_invoke(
    accounts: ConsumeEventsAccounts<'_, '_>,
    args: ConsumeEventsIxArgs,
) -> ProgramResult {
    consume_events_invoke_with_program_id(DRADEX_PROGRAM_ID, accounts, args)
}
pub fn consume_events_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: ConsumeEventsAccounts<'_, '_>,
    args: ConsumeEventsIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: ConsumeEventsKeys = accounts.into();
    let ix = consume_events_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn consume_events_invoke_signed(
    accounts: ConsumeEventsAccounts<'_, '_>,
    args: ConsumeEventsIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    consume_events_invoke_signed_with_program_id(
        DRADEX_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn consume_events_verify_account_keys(
    accounts: ConsumeEventsAccounts<'_, '_>,
    keys: ConsumeEventsKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.pair.key, keys.pair),
        (*accounts.market.key, keys.market),
        (*accounts.event_queue.key, keys.event_queue),
        (*accounts.signer.key, keys.signer),
        (*accounts.master.key, keys.master),
        (*accounts.logger.key, keys.logger),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn consume_events_verify_writable_privileges<'me, 'info>(
    accounts: ConsumeEventsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.pair,
        accounts.event_queue,
        accounts.signer,
        accounts.master,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn consume_events_verify_signer_privileges<'me, 'info>(
    accounts: ConsumeEventsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.signer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn consume_events_verify_account_privileges<'me, 'info>(
    accounts: ConsumeEventsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    consume_events_verify_writable_privileges(accounts)?;
    consume_events_verify_signer_privileges(accounts)?;
    Ok(())
}
